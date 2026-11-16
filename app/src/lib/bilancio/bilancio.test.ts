import { describe, expect, it } from "vitest";
import type { Bilancio } from "$lib/tipi/Bilancio";
import type { RiepilogoBilancio } from "$lib/tipi/RiepilogoBilancio";
import bilancioJson from "../../../../.scratch/v1-piano-11-ui/viste/22-bilancio.json";
import { allarga, distanzaTetto, euroSegnato, indicizzata, passoMargine, ritocca, tagliato } from "./bilancio";

/* eslint-disable @typescript-eslint/no-explicit-any */
const b04: any = (bilancioJson as any).bozza_04;
const bozza: Bilancio = b04.bozza;
const va = b04.varianti;
const sanita = (b: Bilancio) => b.dicasteri.find((d) => d.id === "sanita")!;

describe("passoMargine", () => {
  it("mezzo punto della spesa della Sanità, arrotondato", () => expect(passoMargine(sanita(bozza))).toBe(776_917_230));
});

describe("allarga", () => {
  it("un passo dal funzionamento è la variante del campione", () =>
    expect(allarga(bozza, bozza, "sanita", "funzionamento", 1)).toEqual(va.margine_passo.funzionamento.bilancio));
  it("un passo in disavanzo è la variante del campione", () =>
    expect(allarga(bozza, bozza, "sanita", "disavanzo", 1)).toEqual(va.margine_passo.disavanzo.bilancio));
  it("zero passi è la bozza", () => expect(allarga(bozza, bozza, "sanita", "funzionamento", 0)).toEqual(bozza));
  it("un passo indietro toglie quello aggiunto", () => {
    const uno = allarga(bozza, bozza, "sanita", "disavanzo", 1);
    expect(allarga(uno, bozza, "sanita", "disavanzo", -1)).toEqual(bozza);
  });
});

describe("ritocca", () => {
  it("Iva +1 porta a 0,23 ed è la variante del campione", () => {
    const b = ritocca(bozza, { tipo: "iva" }, 1);
    expect(b.aliquote.iva).toBe(0.23);
    expect(b).toEqual(va.aliquote_passo.iva.piu.bilancio);
  });
  it.each([
    [{ tipo: "irpef", indice: 0 } as const, "irpef_0"],
    [{ tipo: "irpef", indice: 2 } as const, "irpef_2"],
    [{ tipo: "contributi" } as const, "contributi"],
    [{ tipo: "ires" } as const, "ires"],
  ])("%o −1 e +1 sono le varianti del campione", (imposta, chiave) => {
    expect(ritocca(bozza, imposta, -1)).toEqual(va.aliquote_passo[chiave].meno.bilancio);
    expect(ritocca(bozza, imposta, 1)).toEqual(va.aliquote_passo[chiave].piu.bilancio);
  });
  it("non limita: oltre 0,9 resta da dire al core", () => {
    let b = bozza;
    for (let i = 0; i < 70; i++) b = ritocca(b, { tipo: "iva" }, 1);
    expect(b.aliquote.iva).toBe(0.92);
  });
});

describe("tagliato", () => {
  it("un passo dal funzionamento: acquisti −776.917.230, salari 0", () =>
    expect(tagliato(sanita(va.margine_passo.funzionamento.bilancio), sanita(bozza))).toEqual({ salari: 0, acquisti: -776_917_230 }));
  it("uguale alla bozza: null", () => expect(tagliato(sanita(bozza), sanita(bozza))).toBeNull());
});

describe("distanzaTetto", () => {
  it("la bozza sta dentro di 1,0 punti", () => expect(distanzaTetto(b04.riepilogo_bozza.riepilogo)).toBe("dentro il tetto: restano 1,0 punti"));
  it("oltre il tetto", () => {
    const r: RiepilogoBilancio = { ...b04.riepilogo_bozza.riepilogo, crescita_spesa_netta: 2.1 };
    expect(distanzaTetto(r)).toBe("oltre il tetto di 0,4 punti: il controllo di aprile-giugno lo rileva");
  });
});

describe("indicizzata", () => {
  const tutto: Bilancio = va.tutto.bilancio;
  const dicSanita = va.per_dicastero.find((p: any) => p.id === "sanita").bilancio;
  it("la bozza non è indicizzata su niente", () => {
    expect(indicizzata(bozza, tutto, { tipo: "tutto" })).toBe(false);
    expect(indicizzata(bozza, va.scaglioni.bilancio, { tipo: "scaglioni" })).toBe(false);
    expect(indicizzata(bozza, dicSanita, { tipo: "dicastero", id: "sanita" })).toBe(false);
  });
  it("indicizzata per tutto lo è anche per le soglie e per la Sanità", () => {
    expect(indicizzata(tutto, tutto, { tipo: "tutto" })).toBe(true);
    expect(indicizzata(tutto, va.scaglioni.bilancio, { tipo: "scaglioni" })).toBe(true);
    expect(indicizzata(tutto, dicSanita, { tipo: "dicastero", id: "sanita" })).toBe(true);
  });
  it("la Sanità sola non è tutto", () => {
    expect(indicizzata(dicSanita, dicSanita, { tipo: "dicastero", id: "sanita" })).toBe(true);
    expect(indicizzata(dicSanita, tutto, { tipo: "tutto" })).toBe(false);
  });
});

describe("euroSegnato", () => {
  it("col segno, lo zero con ±", () => {
    expect(euroSegnato(6_197_708_677)).toBe("+6,2 mld €");
    expect(euroSegnato(-13_302_199_636)).toBe("−13,3 mld €");
    expect(euroSegnato(0)).toBe("±0 €");
  });
});
