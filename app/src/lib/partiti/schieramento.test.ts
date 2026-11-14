import { describe, expect, it } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import type { VocePartito } from "$lib/tipi/VocePartito";
import { coloreDi } from "$lib/partiti";
import { TETTO_PARTITI, avvisi, cassaBreve, indoleBreve, inOrdine, perFila, seggiDi } from "./schieramento";
import zeroJson from "../../../../.scratch/v1-piano-11-ui/viste/01-turno-zero.json";
import governoJson from "../../../../.scratch/v1-piano-11-ui/viste/03-governo-ordinario.json";
import opposizioneJson from "../../../../.scratch/v1-piano-11-ui/viste/06-opposizione-governo-ia.json";
import cruscottoJson from "../../../../.scratch/v1-piano-11-ui/viste/17-cruscotto.json";
import ereditaJson from "../../../../.scratch/v1-piano-11-ui/viste/30-eredita.json";

const zero = zeroJson as unknown as Vista;
const governo = governoJson as unknown as Vista;
const opposizione = opposizioneJson as unknown as Vista;
const cruscotto = cruscottoJson as unknown as Record<string, { vista: Vista }>;
const ultimatum = cruscotto.ultimatum.vista;
const dieci = cruscotto.crisi_provvisorio.vista;
const eredita = (ereditaJson as unknown as { vista: Vista }).vista;

describe("inOrdine", () => {
  it("il giocatore (4,5) sta fra Centro e Centrodestra", () => {
    expect(inOrdine(zero).map((p) => p.id)).toEqual(["estrema_sinistra", "sinistra", "centrosinistra", "centro", "giocatore", "centrodestra", "destra", "estrema_destra"]);
  });
  it("a parità d'ascissa tiene l'ordine della Vista", () => {
    const [a, b] = [governo.partiti[0], governo.partiti[1]];
    const v = { ...governo, partiti: [{ ...b, posizione_dichiarata: 3 }, { ...a, posizione_dichiarata: 3 }] } as Vista;
    expect(inOrdine(v).map((p) => p.id)).toEqual([b.id, a.id]);
  });
});

describe("perFila", () => {
  it("quattro fino a otto partiti, cinque oltre", () => {
    expect(perFila(8)).toBe(4);
    expect(perFila(9)).toBe(5);
    expect(perFila(10)).toBe(5);
  });
});

describe("indoleBreve", () => {
  it("ogni regola dà una coppia diversa", () => {
    const base = zero.partiti[0];
    const coppie = (["sticker", "aggregator", "hunter", "governator"] as const).map((regola) => indoleBreve({ ...base, regola } as VocePartito).join("|"));
    expect(new Set(coppie).size).toBe(4);
  });
});

describe("avvisi", () => {
  const di = (v: Vista, id: string) => avvisi(v, v.partiti.find((p) => p.id === id)!);
  it("sulla 01 Centrodestra e Destra sono il governo uscente", () => {
    expect(di(zero, "centrodestra")).toContain("avviso.governo_uscente");
    expect(di(zero, "destra")).toContain("avviso.governo_uscente");
    expect(di(zero, "sinistra")).not.toContain("avviso.governo_uscente");
  });
  it("in ultimatum la Sinistra ha l'ultimatum", () => {
    expect(di(ultimatum, "sinistra")).toContain("avviso.ultimatum");
    expect(di(ultimatum, "centro")).not.toContain("avviso.ultimatum");
  });
  it("all'opposizione l'alleato in ultimatum col governo ha il suo avviso", () => {
    const f = opposizione.alleati_avversari.find((x) => x.fascia === "ultimatum");
    if (f) expect(di(opposizione, f.partito)).toContain("avviso.ultimatum_governo");
  });
  it("sulla 30 l'Estrema sinistra è sciolta", () => {
    expect(di(eredita, "estrema_sinistra")).toContain("avviso.sciolto");
  });
  it("con dieci partiti, i due senza colore sono nati in partita", () => {
    const v = inOrdine(dieci);
    expect(v).toHaveLength(TETTO_PARTITI);
    const senza = v.filter((p) => coloreDi(p.id) === null);
    expect(senza).toHaveLength(2);
    for (const p of senza) expect(avvisi(dieci, p)).toContain("avviso.nato");
    expect(avvisi(dieci, v.find((p) => coloreDi(p.id) !== null)!)).not.toContain("avviso.nato");
  });
  it("senza seggi all'ultimo voto: Camera e Senato a zero in elezioni[0]", () => {
    const voti = { partito: "sinistra", quota: 0.01, camera: 0, senato: 0 };
    const v = { ...governo, elezioni: [{ ...governo.elezioni[0], voti: [voti] }] } as Vista;
    expect(di(v, "sinistra")).toContain("avviso.senza_seggi");
    expect(di(governo, "giocatore")).not.toContain("avviso.senza_seggi");
  });
  it("l'ordine: ultimatum prima di sciolto prima di senza seggi", () => {
    const p = ultimatum.partiti.find((x) => x.id === "sinistra")!;
    const v = { ...ultimatum, partiti: ultimatum.partiti.map((x) => (x === p ? { ...x, sciolto: true } : x)) } as Vista;
    const a = avvisi(v, v.partiti.find((x) => x.id === "sinistra")!);
    expect(a.indexOf("avviso.ultimatum")).toBeLessThan(a.indexOf("avviso.sciolto"));
  });
  it("all'opposizione il partito di governo con più seggi guida il governo", () => {
    const guida = opposizione.partiti.filter((p) => avvisi(opposizione, p).includes("avviso.guida"));
    expect(guida).toHaveLength(1);
  });
});

describe("seggiDi", () => {
  it("a Camere sciolte i seggi in carica non ci sono", () => {
    expect(seggiDi(zero, "destra").carica).toBeNull();
    expect(seggiDi(zero, "sinistra").oggi).not.toBeNull();
  });
  it("in carica le voci della Vista; un partito senza riga dà null", () => {
    expect(seggiDi(governo, "giocatore").carica).toEqual(governo.seggi.find((s) => s.partito === "giocatore"));
    expect(seggiDi(governo, "fantasma")).toEqual({ carica: null, oggi: null });
  });
});

describe("cassaBreve", () => {
  it("miliardi, milioni e migliaia, con lo spazio non separabile prima di €", () => {
    expect(cassaBreve(1_200_000_000)).toBe("1,2 mld\u00a0€");
    expect(cassaBreve(2_500_000)).toBe("2,5 mln\u00a0€");
    expect(cassaBreve(340_000)).toBe("340 mila\u00a0€");
  });
});
