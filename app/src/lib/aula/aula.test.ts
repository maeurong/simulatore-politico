import { describe, expect, it } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import type { VoceMisura } from "$lib/tipi/VoceMisura";
import aula from "../../../../.scratch/v1-piano-11-ui/viste/19-aula.json";
import bilancio from "../../../../.scratch/v1-piano-11-ui/viste/04-governo-bilancio.json";
import crisi from "../../../../.scratch/v1-piano-11-ui/viste/05-crisi.json";
import { calendario, fiduciaInCrisi, fiduciaPonibile, quandoMisura, rifiutoPer, riportaFuoco, lineaScelta, slitta, ultimiVoti, votaOra } from "./aula";

const stati = aula as unknown as Record<string, { vista: Vista }>;
const piena = stati.piena.vista;
const per = (v: Vista, id: number): VoceMisura => v.calendario.find((m) => m.id === id)!;
const nomi = (v: Vista) => calendario(v).map((p) => `${p.tipo}:${p.misura?.id ?? "-"}`);

describe("calendario", () => {
  it("piena: un posto per i decreti, tre ordinarie (due occupate), la costituzionale, l'opposizione col Centro", () => {
    expect(nomi(piena)).toEqual(["decreti:-", "ordinaria:1", "ordinaria:3", "ordinaria:-", "costituzionale:2", "opposizione:0"]);
    expect(calendario(piena).flatMap((p) => (p.tipo === "ordinaria" ? [[p.numero, p.di]] : []))).toEqual([[1, 3], [2, 3], [3, 3]]);
  });
  it("il bilancio occupa il calendario: un posto solo", () => {
    expect(calendario(bilancio as unknown as Vista).map((p) => p.tipo)).toEqual(["bilancio"]);
  });
  it("opposizione col posto riservato libero: il posto dell'opposizione è libero e i conti sono quelli di Vista.posti", () => {
    const v = stati.opposizione_libero.vista;
    expect(calendario(v).find((p) => p.tipo === "opposizione")!.misura).toBeNull();
    expect(calendario(v).filter((p) => p.tipo === "ordinaria").every((p) => p.misura)).toBe(true);
    expect(calendario(v).find((p) => p.tipo === "decreti")!.misura!.forma).toBe("conversione");
  });
  it("i posti si contano da Vista.posti: più posti che misure restano liberi", () => {
    const v = { ...piena, posti: { ...piena.posti, ordinari: 4, ordinari_occupati: 2 } };
    expect(calendario(v).filter((p) => p.tipo === "ordinaria").map((p) => p.misura?.id ?? null)).toEqual([1, 3, null, null]);
  });
  it("ogni ordinaria della Vista compare in un posto, anche se ordinari_occupati ne conta meno", () => {
    const v = { ...piena, posti: { ...piena.posti, ordinari: 3, ordinari_occupati: 1 } };
    const ordinarie = piena.calendario.filter((m) => (m.forma === "riforma" || m.forma === "elettorale") && m.promotore === null).map((m) => m.id);
    expect(ordinarie.length).toBeGreaterThan(1);
    expect(calendario(v).flatMap((p) => (p.tipo === "ordinaria" && p.misura ? [p.misura.id] : []))).toEqual(ordinarie);
  });
  it("calendario pieno: nessun posto libero fra le ordinarie", () => {
    const v = { ...piena, posti: { ...piena.posti, ordinari: 2, ordinari_occupati: 2 } };
    expect(calendario(v).filter((p) => p.tipo === "ordinaria").map((p) => p.misura?.id ?? null)).toEqual([1, 3]);
  });
});

describe("calendario, più misure che posti", () => {
  it("due ordinarie su un posto: tutte e due compaiono, il posto in più non è libero", () => {
    const v = { ...piena, posti: { ...piena.posti, ordinari: 1, ordinari_occupati: 1 } };
    const o = calendario(v).filter((p) => p.tipo === "ordinaria");
    expect(o.map((p) => p.misura?.id ?? null)).toEqual([1, 3]);
  });
  it("costituzionali: una per posto, la seconda non sparisce; posti in più liberi", () => {
    const cost = piena.calendario.find((m) => m.id === 2)!;
    const due = { ...piena, calendario: [...piena.calendario, { ...cost, id: 9, nome: "Altra costituzionale" }] };
    const un = { ...due, posti: { ...due.posti, costituzionali: 1, costituzionali_occupati: 1 } };
    expect(calendario(un).filter((p) => p.tipo === "costituzionale").map((p) => p.misura?.id ?? null)).toEqual([2, 9]);
    const tre = { ...piena, posti: { ...piena.posti, costituzionali: 3, costituzionali_occupati: 0 } };
    expect(calendario(tre).filter((p) => p.tipo === "costituzionale").map((p) => p.misura?.id ?? null)).toEqual([2, null, null]);
  });
});

describe("votaOra", () => {
  it("vero sulla costituzionale di piena (prima deliberazione, 2027T2), falso sulle altre", () => {
    expect(piena.data).toMatchObject({ anno: 2027, trimestre: 2 });
    expect(per(piena, 2).deliberazione).toBe(1);
    expect(piena.calendario.filter((m) => votaOra(piena, m)).map((m) => m.id)).toEqual([2]);
  });
});

describe("fiduciaPonibile", () => {
  it("falso sullo sgravio del Centro, vero sul salario minimo", () => {
    expect(fiduciaPonibile(piena, per(piena, 0))).toBe(false);
    expect(fiduciaPonibile(piena, per(piena, 3))).toBe(true);
  });
  it("nel turno del bilancio non sulle riforme del governo, sì sulle conversioni; mai se già posta", () => {
    const b = bilancio as unknown as Vista;
    const riforma = piena.calendario.find((m) => m.id === 3)!;
    expect(fiduciaPonibile(piena, riforma)).toBe(true);
    expect(fiduciaPonibile({ ...piena, calendario: [...piena.calendario, b.calendario.find((m) => m.forma === "bilancio")!] }, riforma)).toBe(false);
    const decreto = { ...riforma, forma: "conversione" as const };
    expect(fiduciaPonibile({ ...piena, calendario: [...piena.calendario, b.calendario.find((m) => m.forma === "bilancio")!] }, decreto)).toBe(true);
    expect(fiduciaPonibile(piena, { ...riforma, fiducia: true })).toBe(false);
  });
  it("falso su ogni misura della crisi, del bilancio, della costituzionale e dell'opposizione", () => {
    const c = crisi as unknown as Vista;
    expect(c.calendario.length).toBeGreaterThan(0);
    expect(c.calendario.some((m) => fiduciaPonibile(c, m))).toBe(false);
    const b = bilancio as unknown as Vista;
    expect(b.calendario.filter((m) => m.forma === "bilancio").some((m) => fiduciaPonibile(b, m))).toBe(false);
    expect(fiduciaPonibile(piena, per(piena, 2))).toBe(false);
    expect(stati.opposizione_proposta.vista.calendario.some((m) => fiduciaPonibile(stati.opposizione_proposta.vista, m))).toBe(false);
  });
});

describe("lineaScelta", () => {
  it("al governo, vero sullo sgravio del Centro e falso sulle misure del governo", () => {
    expect(lineaScelta(piena, per(piena, 0))).toBe(true);
    expect(lineaScelta(piena, per(piena, 3))).toBe(false);
  });
  it("all'opposizione, vero su ogni misura", () => {
    const v = stati.opposizione_proposta.vista;
    expect(v.calendario.every((m) => lineaScelta(v, m))).toBe(true);
  });
});

describe("ultimiVoti", () => {
  it("il primo è la costituzionale approvata, Camera 219 sì e 181 no", () => {
    const e = ultimiVoti(stati.dopo_voto.vista)[0];
    expect(e).toMatchObject({ forma: "costituzionale", approvata: true, camera: { si: 219, no: 181 } });
  });
  it("dal più recente, anche se la Vista li desse in disordine", () => {
    const v = stati.dopo_voto.vista;
    const [a, b] = [v.esiti[0], { ...v.esiti[0], data: { anno: 2030, trimestre: 1 }, nome: "nuovo" }];
    expect(ultimiVoti({ ...v, esiti: [a, b] })[0].nome).toBe("nuovo");
  });
});

describe("rifiutoPer", () => {
  it("il rifiuto di una misura non compare sotto un'altra, né in un'altra Vista", () => {
    const r = { testo: "no", vista: piena, misura: 3 };
    expect(rifiutoPer(r, piena, 3)).toBe(r);
    expect(rifiutoPer(r, piena, 0)).toBeNull();
    expect(rifiutoPer(r, { ...piena }, 3)).toBeNull();
    expect(rifiutoPer(null, piena, 3)).toBeNull();
  });
});

describe("riportaFuoco", () => {
  const prova = (da: { isConnected: boolean } | null, prossimo: { focus: (o?: unknown) => void } | null) => {
    const chiamate: string[] = [];
    let selettore = "";
    const lato = { querySelector: (q: string) => ((selettore = q), prossimo), focus: () => chiamate.push("lato") } as unknown as HTMLElement;
    riportaFuoco(lato, da as unknown as Element);
    return { chiamate, selettore };
  };
  it("bottone smontato: al prossimo emendamento aperto e abilitato senza scroll, se no al dettaglio", () => {
    const c: unknown[] = [];
    const r = prova({ isConnected: false }, { focus: (o) => c.push(o) });
    expect(c).toEqual([{ preventScroll: true }]);
    expect(r.selettore).toContain(":not([aria-disabled=\"true\"])");
    expect(prova({ isConnected: false }, null).chiamate).toEqual(["lato"]);
  });
  it("bottone ancora montato (linea Sì/No col clic): non si sposta il fuoco", () => {
    const c: unknown[] = [];
    expect(prova({ isConnected: true }, { focus: () => c.push(1) }).chiamate).toEqual([]);
    expect(c).toEqual([]);
  });
  it("fuoco su body o fuori dal dettaglio (da nullo): non si sposta", () => {
    const c: unknown[] = [];
    expect(prova(null, { focus: () => c.push(1) }).chiamate).toEqual([]);
    expect(c).toEqual([]);
  });
});

describe("slitta", () => {
  const b = bilancio as unknown as Vista, c = crisi as unknown as Vista;
  it("04, turno del bilancio: la riforma col voto adesso slitta, quella col voto dopo e il bilancio no", () => {
    expect(b.calendario.map((m) => [m.id, slitta(b, m)])).toEqual([[1, true], [2, false], [3, false]]);
  });
  it("05, crisi: la riforma col voto adesso slitta, quella col voto dopo no", () => {
    expect(c.calendario.map((m) => [m.id, slitta(c, m)])).toEqual([[74, true], [76, false]]);
  });
  it("un turno ordinario non fa slittare niente", () => {
    expect(piena.calendario.some((m) => slitta(piena, m))).toBe(false);
  });
  it("quandoMisura: la legge che slitta dice il periodo del turno dopo, le altre il loro voto", () => {
    expect(quandoMisura(b, per(b, 1))).toBe("slitta a fine gennaio–marzo 2028");
    expect(quandoMisura(c, per(c, 74))).toBe("slitta a fine luglio–settembre 2035");
    expect(quandoMisura(b, per(b, 2))).toBe("si vota a fine gennaio–marzo 2028");
    expect(quandoMisura(piena, per(piena, 2))).toBe("si vota a fine turno");
  });
});

describe("fiduciaInCrisi", () => {
  it("in crisi la fiducia sul salario minimo c'è ma non si può porre; fuori dalla crisi non è un caso", () => {
    const c = { ...(crisi as unknown as Vista), calendario: piena.calendario.filter((m) => m.id === 3) };
    expect(c.calendario.map((m) => [fiduciaInCrisi(c, m), fiduciaPonibile(c, m)])).toEqual([[true, false]]);
    expect(fiduciaInCrisi(piena, per(piena, 3))).toBe(false);
  });
  it("le proposte dell'opposizione non hanno fiducia nemmeno spenta", () => {
    const c = crisi as unknown as Vista;
    expect(c.calendario.some((m) => fiduciaInCrisi(c, m))).toBe(false);
  });
});
