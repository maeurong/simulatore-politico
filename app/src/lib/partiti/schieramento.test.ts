import { describe, expect, it } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import type { FasciaIntesa } from "$lib/tipi/FasciaIntesa";
import type { VoceEsito } from "$lib/tipi/VoceEsito";
import type { VocePartito } from "$lib/tipi/VocePartito";
import { coloreDi } from "$lib/partiti";
import { t } from "$lib/testi/t";
import { CAUSE, TETTO_PARTITI, avvisi, staPerSciogliersi, caselleFascia, cassaBreve, distanza, fraseDistanza, indoleBreve, inOrdine, intesaDi, mosseDi, perFila, seggiDi, ultimoVoto } from "./schieramento";
import zeroJson from "../../../../.scratch/v1-piano-11-ui/viste/01-turno-zero.json";
import governoJson from "../../../../.scratch/v1-piano-11-ui/viste/03-governo-ordinario.json";
import opposizioneJson from "../../../../.scratch/v1-piano-11-ui/viste/06-opposizione-governo-ia.json";
import trattativaJson from "../../../../.scratch/v1-piano-11-ui/viste/02-trattativa-incarico.json";
import crisiJson from "../../../../.scratch/v1-piano-11-ui/viste/05-crisi.json";
import cruscottoJson from "../../../../.scratch/v1-piano-11-ui/viste/17-cruscotto.json";
import ereditaJson from "../../../../.scratch/v1-piano-11-ui/viste/30-eredita.json";

const crisi = crisiJson as unknown as Vista;
const zero = zeroJson as unknown as Vista;
const trattativa = trattativaJson as unknown as Vista;
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
  it("all'opposizione l'alleato in ultimatum col governo ha il suo avviso (Vista costruita: la 06 non ne ha)", () => {
    expect(opposizione.alleati_avversari.some((x) => x.fascia === "ultimatum")).toBe(false);
    const v = { ...opposizione, alleati_avversari: [{ partito: "sinistra", fascia: "ultimatum", causa_ultimatum: null }] } as Vista;
    expect(di(v, "sinistra")).toEqual(["avviso.ultimatum_governo", "avviso.al_governo"]);
    expect(di(v, "centro")).toEqual(["avviso.al_governo"]);
  });
  it("sulla 05 l'Estrema sinistra in minaccia ha l'avviso, il Centro no; sciolta non lo ha", () => {
    expect(crisi.partiti.find((p) => p.id === "estrema_sinistra")!.minaccia).toBe(true);
    expect(di(crisi, "estrema_sinistra")).toContain("avviso.minaccia_scissione");
    expect(di(crisi, "centro")).not.toContain("avviso.minaccia_scissione");
    const v = { ...crisi, partiti: crisi.partiti.map((p) => (p.id === "estrema_sinistra" ? { ...p, sciolto: true } : p)) } as Vista;
    expect(di(v, "estrema_sinistra")).not.toContain("avviso.minaccia_scissione");
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
  it("senza seggi, al governo e nato insieme, in quest'ordine", () => {
    const voti = { partito: "sinistra", quota: 0.01, camera: 0, senato: 0 };
    const v = {
      ...opposizione,
      partiti: opposizione.partiti.map((p) => (p.id === "sinistra" ? { ...p, id: "nuovo_9" } : p)),
      elezioni: [{ ...opposizione.elezioni[0], voti: [{ ...voti, partito: "nuovo_9" }] }],
    } as Vista;
    expect(di(v, "nuovo_9")).toEqual(["avviso.senza_seggi", "avviso.al_governo", "avviso.nato"]);
  });
  it("tutti gli avvisi insieme, nell'ordine del brief: ultimatum al governo, sciolto, al governo, nato", () => {
    const v = {
      ...opposizione,
      alleati_avversari: [{ partito: "nuovo_9", fascia: "ultimatum", causa_ultimatum: null }],
      partiti: opposizione.partiti.map((p) => (p.id === "sinistra" ? { ...p, id: "nuovo_9", sciolto: true } : p)),
    } as Vista;
    expect(di(v, "nuovo_9")).toEqual(["avviso.ultimatum_governo", "avviso.sciolto", "avviso.al_governo", "avviso.nato"]);
  });
  it("l'ultimatum dell'alleato del giocatore viene prima dello sciolto", () => {
    const v = {
      ...ultimatum,
      partiti: ultimatum.partiti.map((p) => (p.id === "sinistra" ? { ...p, sciolto: true } : p)),
    } as Vista;
    expect(di(v, "sinistra")).toEqual(["avviso.ultimatum", "avviso.sciolto"]);
  });
  it("«al governo» non va agli alleati del giocatore: sulla 30 centrodestra e nuovo_8 governano ma sono alleati", () => {
    expect(eredita.governa).toBe(false);
    for (const id of ["centrodestra", "nuovo_8", "giocatore"]) {
      expect(eredita.partiti.find((p) => p.id === id)!.al_governo).toBe(true);
      expect(di(eredita, id)).not.toContain("avviso.al_governo");
    }
  });
  it("sulla 03 gli alleati al governo non hanno «al governo»", () => {
    for (const id of ["centrodestra", "destra"]) expect(di(governo, id)).not.toContain("avviso.al_governo");
  });
  it("il chip «senza seggi» non va al giocatore nemmeno senza seggi", () => {
    const voti = governo.elezioni[0].voti.map((x) => (x.partito === "giocatore" ? { ...x, camera: 0, senato: 0 } : x));
    const v = { ...governo, elezioni: [{ ...governo.elezioni[0], voti }] } as Vista;
    expect(di(v, "giocatore")).not.toContain("avviso.senza_seggi");
    expect(staPerSciogliersi(v, v.partiti.find((p) => p.id === "giocatore")!)).toBe(false);
  });
  it("all'opposizione ogni partito di governo ha «al governo»; gli altri no, né il giocatore", () => {
    const [dentro, fuori] = [opposizione.partiti.filter((p) => p.al_governo), opposizione.partiti.filter((p) => !p.al_governo)];
    expect(dentro.length).toBeGreaterThan(0);
    for (const p of dentro) expect(avvisi(opposizione, p)).toContain("avviso.al_governo");
    for (const p of fuori) expect(avvisi(opposizione, p)).not.toContain("avviso.al_governo");
  });
  it("il giocatore che governa non ha «al governo» su di sé, e a Camere sciolte vale «governo uscente»", () => {
    expect(avvisi(governo, governo.partiti.find((p) => p.id === governo.giocatore)!)).not.toContain("avviso.al_governo");
    const uscente = zero.partiti.find((p) => p.id === "destra")!;
    expect(avvisi(zero, uscente)).toEqual(["avviso.governo_uscente"]);
  });
});

describe("staPerSciogliersi", () => {
  const p = (v: Vista, id: string) => v.partiti.find((x) => x.id === id)!;
  it("senza seggi all'ultimo voto e non sciolto: alla prossima si scioglie (03: Estrema sinistra, 0 · 0)", () => {
    expect(staPerSciogliersi(governo, p(governo, "estrema_sinistra"))).toBe(true);
    expect(staPerSciogliersi(governo, p(governo, "sinistra"))).toBe(false);
  });
  it("già sciolto, o mai votato: no (30: Estrema sinistra sciolta; 01: nessun voto)", () => {
    expect(staPerSciogliersi(eredita, p(eredita, "estrema_sinistra"))).toBe(false);
    expect(staPerSciogliersi(zero, p(zero, "estrema_sinistra"))).toBe(false);
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

describe("distanza", () => {
  const con = (dichiarata: number, percepita: number) => ({ ...zero.partiti[0], posizione_dichiarata: dichiarata, posizione_percepita: percepita }) as VocePartito;
  it("percepita a destra della dichiarata: verso destra, la differenza in punti", () => {
    const d = distanza(con(2, 2.76));
    expect(d.verso).toBe("destra");
    expect(d.punti).toBeCloseTo(0.76, 9);
  });
  it("percepita a sinistra: verso sinistra, i punti sono positivi", () => {
    const d = distanza(con(5, 3.5));
    expect(d.verso).toBe("sinistra");
    expect(d.punti).toBeCloseTo(1.5, 9);
  });
  it("le due uguali: ferma, zero", () => {
    expect(distanza(con(3, 3))).toEqual({ verso: "ferma", punti: 0 });
  });
});

describe("intesaDi", () => {
  it("sulla 03 un alleato dà l'intesa in cifre, sulla scala 0-100 della Vista", () => {
    const r = intesaDi(governo, "centrodestra");
    expect(r).toEqual({ alleato: governo.alleati.find((a) => a.partito === "centrodestra") });
    expect(r && "alleato" in r && r.alleato.intesa).toBe(54.86607669616519);
  });
  it("sulla 06 un alleato del governo dà la fascia", () => {
    const f = opposizione.alleati_avversari[0];
    expect(intesaDi(opposizione, f.partito)).toEqual({ fascia: f });
  });
  it("il partito del giocatore e chi non è alleato danno null", () => {
    expect(intesaDi(governo, "giocatore")).toBeNull();
    expect(intesaDi(governo, "sinistra")).toBeNull();
    expect(intesaDi(opposizione, "giocatore")).toBeNull();
  });
});

describe("ultimoVoto", () => {
  it("sulla 01 non si è ancora votato: null", () => {
    expect(ultimoVoto(zero, "destra")).toBeNull();
  });
  it("sulla 02 Destra ha la quota e i seggi del campione (elezioni[0].voti)", () => {
    expect(ultimoVoto(trattativa, "destra")).toEqual({ partito: "destra", quota: 0.24933003381413937, camera: 116, senato: 62 });
  });
  it("un partito senza riga nell'ultimo voto (un nato): null", () => {
    expect(ultimoVoto(trattativa, "nuovo_9")).toBeNull();
  });
});

describe("mosseDi", () => {
  const voce = (partito: string | null, fatto: string): VoceEsito => ({ tipo: "misura_presentata", criticita: "ordinaria", precedenza: 0, in_evidenza: false, tono: "ordinario", fatto, causa: "", partito, dicastero: null, regione: null, misura: null, disposizioni: [], famiglia: null, dettagli: [] });
  it("sulla 03 il Centro ha la sola mossa del posto riservato; un partito senza voci ha []", () => {
    expect(mosseDi(governo, "centro")).toEqual(["Porta nel posto riservato Sgravio contributivo nel Mezzogiorno: 15%"]);
    expect(mosseDi(governo, "sinistra")).toEqual([]);
  });
  it("le voci del partito, nell'ordine dell'esito; quelle senza partito non sono di nessuno", () => {
    const v = { ...governo, esito: [voce("centro", "prima"), voce("destra", "altro"), voce(null, "di nessuno"), voce("centro", "seconda")] } as Vista;
    expect(mosseDi(v, "centro")).toEqual(["prima", "seconda"]);
  });
});

describe("caselleFascia", () => {
  it("solida 3, incrinata 2, in ultimatum 1 su tre", () => {
    const fasce: FasciaIntesa[] = ["solida", "incrinata", "ultimatum"];
    expect(fasce.map(caselleFascia)).toEqual([3, 2, 1]);
  });
});

describe("CAUSE", () => {
  it("ogni causa d'intesa ha la sua chiave di testo, e il testo si legge", () => {
    expect(Object.keys(CAUSE)).toHaveLength(13);
    for (const chiave of Object.values(CAUSE)) expect(typeof t(chiave)).toBe("string");
  });
});

describe("fraseDistanza", () => {
  const c = crisi.campagna;
  const a = (d: number, minaccia = false) => fraseDistanza({ ...crisi.partiti[0], posizione_dichiarata: 3, posizione_percepita: 3 + d, minaccia }, c);
  it("sotto la minaccia, a minaccia, a scissione; la minaccia già scattata, sotto soglia, ha la sua frase", () => {
    expect(a(0.2)).toBe("partiti.s_sotto_minaccia");
    expect(a(-c.minaccia_scissione)).toBe("partiti.s_minaccia");
    expect(a(c.soglia_scissione)).toBe("partiti.s_scissione");
    expect(a(0.2, true)).toBe("partiti.s_minaccia_accesa");
  });
});
