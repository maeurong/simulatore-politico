import { describe, expect, it } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import { salti, leveDi, chiaveDi } from "$lib/caricature/caricatura";
import { attiDirotta, conRegione, costiDirotta, etichettaFerma, figurine, molla, obiettivo, posizioni, prezzoDimissioni } from "./governo";
import ordinarioJson from "../../../../.scratch/v1-piano-11-ui/viste/03-governo-ordinario.json";
import crisiJson from "../../../../.scratch/v1-piano-11-ui/viste/05-crisi.json";
import governoJson from "../../../../.scratch/v1-piano-11-ui/viste/18-governo.json";

const ordinario = ordinarioJson as unknown as Vista;
const crisi = crisiJson as unknown as Vista;
const stati = governoJson as unknown as Record<string, { vista: Vista }>;
const di = (v: Vista, dicastero: string) => figurine(v).find((f) => f.dicastero === dicastero)!;

describe("figurine", () => {
  it("dieci, nell'ordine dei dicasteri", () => {
    expect(figurine(ordinario).map((f) => f.dicastero)).toEqual(ordinario.dicasteri.map((d) => d.id));
    expect(figurine(ordinario).map((f) => f.indice)).toEqual([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    expect(di(ordinario, "sanita").nomeDicastero).toBe("Sanità");
  });
  it("l'evidenza è quella dell'agenda: distanza, evento, vuota", () => {
    const ev = (d: string) => di(ordinario, d).evidenza;
    expect(ev("economia_finanze")).toEqual(["distanza"]);
    expect(ev("sviluppo_economico")).toEqual(["distanza"]);
    expect(ev("ambiente_energia")).toEqual(["evento"]);
    const altri = figurine(ordinario).filter((f) => !["economia_finanze", "sviluppo_economico", "ambiente_energia"].includes(f.dicastero));
    expect(altri.every((f) => f.evidenza.length === 0)).toBe(true);
  });
  it("senza agenda dice perché", () => {
    expect(di(ordinario, "infrastrutture_trasporti").senzaAgenda).toBe("margine esaurito");
    expect(di(ordinario, "esteri_difesa").senzaAgenda).toBe("margine esaurito");
    expect(di(ordinario, "sanita").senzaAgenda).toBeNull();
    expect(di(ordinario, "sanita").agenda).not.toBeNull();
  });
  it("primo turno di governo: tutti in rodaggio", () => {
    expect(figurine(stati.scelte.vista).every((f) => f.stati.includes("rodaggio"))).toBe(true);
    expect(figurine(stati.scontenti.vista).some((f) => f.stati.includes("rodaggio"))).toBe(false);
  });
  it("scelte: Lavoro perde 11,6 per l'atto fermato, Ambiente ha chiesto le dimissioni", () => {
    const lavoro = di(stati.scelte.vista, "lavoro_welfare");
    expect(lavoro.freccia).toBeCloseTo(-11.6, 1);
    expect(lavoro.voto).toBe(48);
    expect(di(stati.scelte.vista, "ambiente_energia").stati).toContain("dimissioni_chieste");
    expect(di(stati.scelte.vista, "sanita").stati).not.toContain("dimissioni_chieste");
  });
  it("scontenti: sei carte in preavviso", () => {
    expect(figurine(stati.scontenti.vista).filter((f) => f.stati.includes("preavviso"))).toHaveLength(6);
  });
  it("tecnico, interim", () => {
    expect(di(stati.tecnico.vista, "giustizia").stati).toContain("tecnico");
    const amb = di(stati.interim.vista, "ambiente_energia");
    expect(amb.stati).toContain("interim");
    expect(amb.ministro).toBeNull();
    expect(amb.voto).toBeNull();
    expect(amb.freccia).toBe(0);
  });
  it("crisi: sei carte in interim", () => {
    expect(figurine(crisi).filter((f) => f.stati.includes("interim"))).toHaveLength(6);
  });
});

describe("obiettivo", () => {
  it("l'indicatore di servizio contro l'obiettivo del ministro", () => {
    const o = obiettivo(ordinario, "sanita");
    expect(o.nome).toBe("Qualità del servizio sanitario");
    expect(o.valore).toBeCloseTo(99.7, 1);
    expect(o.obiettivo).toBeCloseTo(104.7, 1);
    expect(o.verso).toBe("sale");
  });
  it("in interim l'obiettivo manca", () => {
    expect(obiettivo(stati.interim.vista, "ambiente_energia").obiettivo).toBeNull();
  });
});

describe("posizioni", () => {
  it("la tua è quella percepita del tuo partito", () => {
    const m = ordinario.ministeri[0].ministro!;
    const p = posizioni(ordinario, m);
    expect(p.ministro).toBe(m.posizione);
    expect(p.tua).toBe(ordinario.partiti.find((x) => x.id === ordinario.giocatore)!.posizione_percepita);
  });
});

const min = (v: Vista, d: string) => v.ministeri.find((x) => x.dicastero === d)!;

describe("etichettaFerma", () => {
  it("col segno; a zero niente", () => {
    expect(etichettaFerma(-18.4)).toBe("Ferma −18");
    expect(etichettaFerma(2.6)).toBe("Ferma +3");
    expect(etichettaFerma(0.2)).toBe("Ferma");
    expect(etichettaFerma(-0.2)).toBe("Ferma");
  });
});

describe("dirotta", () => {
  it("gli altri tre atti, senza quello in agenda", () => {
    const m = min(ordinario, "sanita");
    const a = attiDirotta(m);
    expect(a).toHaveLength(3);
    expect(a.map((x) => x.id)).not.toContain(m.agenda!.atto);
  });
  it("la regione solo per Sviluppo economico e Infrastrutture", () => {
    const con = ordinario.ministeri.filter(conRegione).map((m) => m.dicastero);
    expect(con).toEqual(["sviluppo_economico", "infrastrutture_trasporti"]);
    expect(ordinario.ministeri.filter((m) => !conRegione(m))).toHaveLength(8);
  });
  it("i costi in sequenza; un rifiuto resta come testo", async () => {
    const chiamate: string[] = [];
    let aperte = 0;
    const chiedi = async (atto: string | null, regione: string | null) => {
      expect(aperte++).toBe(0);
      chiamate.push(`${atto}/${regione}`);
      await Promise.resolve();
      aperte--;
      if (atto === "b") throw { specie: "rifiuto", testo: "comando non ammesso: niente da dirottare" };
      return atto ? -5 : -2;
    };
    const r = await costiDirotta(chiedi, ["a", "b"], ["lazio"]);
    expect(r).toEqual({ atti: { a: -5, b: "niente da dirottare" }, regioni: { lazio: -2 } });
    expect(chiamate).toEqual(["a/null", "b/null", "null/lazio"]);
  });
  it("una nuova apertura ferma la vecchia", async () => {
    let n = 0;
    const r = await costiDirotta(async () => (n++, -1), ["a", "b", "c"], [], () => n < 1);
    expect(n).toBe(1);
    expect(Object.keys(r.atti)).toEqual(["a"]);
  });
});

describe("prezzoDimissioni", () => {
  it("N punti d'intesa all'alleato, niente col tuo partito, gratis col tecnico", () => {
    expect(prezzoDimissioni(ordinario, min(ordinario, "economia_finanze"))).toMatch(/10 punti d.intesa a Destra/);
    expect(prezzoDimissioni(ordinario, min(ordinario, "interno"))).toMatch(/^niente/i);
    expect(prezzoDimissioni(stati.tecnico.vista, min(stati.tecnico.vista, "giustizia"))).toMatch(/^gratis/i);
  });
});

describe("rosa", () => {
  it("tre candidati, tre coppie naso-colletto distinte", () => {
    const v = stati.interim.vista;
    const rosa = min(v, "ambiente_energia").rosa;
    expect(rosa).toHaveLength(3);
    const c = rosa.map((x) => ({ partito: chiaveDi(v, x.partito), nome: x.nome }));
    const coppie = salti("ministro", c).map((s, i) => { const l = leveDi("ministro", c[i].partito, c[i].nome, s); return `${l.naso}:${l.colletto}`; });
    expect(new Set(coppie).size).toBe(3);
  });
});

describe("molla", () => {
  it("parte da 0, arriva a 1 e supera 1 per strada", () => {
    const m = molla();
    expect(m).toMatch(/^linear\(0,/);
    expect(m).toMatch(/, 1\)$/);
    expect(Math.max(...m.slice(7, -1).split(", ").map(Number))).toBeGreaterThan(1);
  });
});
