import { beforeEach, describe, expect, it, vi } from "vitest";
import type { Bilancio } from "$lib/tipi/Bilancio";
import type { RiepilogoBilancio } from "$lib/tipi/RiepilogoBilancio";

const { guscio } = vi.hoisted(() => ({
  guscio: { bozzaBilancio: vi.fn(), indicizza: vi.fn(), riepilogoBilancio: vi.fn() },
}));
vi.mock("$lib/guscio/chiamate", async (originale) => ({ ...(await originale<object>()), guscio }));

import { bozza } from "./bozza.svelte";

/** Una promessa che il test risolve o rifiuta quando vuole. */
function comandata<T>() {
  let risolvi!: (x: T) => void, rifiuta!: (e: unknown) => void;
  const p = new Promise<T>((a, b) => ((risolvi = a), (rifiuta = b)));
  return { p, risolvi, rifiuta };
}
const svuota = () => new Promise((r) => setTimeout(r, 0));

const base = { anno: 2028, dicasteri: [], aliquote: { irpef: [], iva: 0.22, ires: 0.24, contributi: 0.33 }, fondo_riserva: 0 } as Bilancio;
// con un dicastero e due soglie: per annulla
const dic = (id: string, spesa: number, livello: number) => ({ id, spesa, salari: spesa / 4, acquisti: spesa / 4, margine: spesa / 100, livello_prezzi: livello });
const sca = (fino_a: number | null, aliquota: number, livello: number) => ({ fino_a, aliquota, livello_prezzi: livello });
const piena = {
  anno: 2028, fondo_riserva: 0,
  dicasteri: [dic("sanita", 1000, 1), dic("interno", 400, 1)],
  aliquote: { irpef: [sca(28000, 0.23, 1), sca(null, 0.43, 1)], iva: 0.22, ires: 0.24, contributi: 0.33 },
} as Bilancio;
const indicizzataTutta = {
  ...piena,
  dicasteri: [dic("sanita", 1030, 1.03), dic("interno", 412, 1.03)],
  aliquote: { ...piena.aliquote, irpef: [sca(28840, 0.23, 1.03), sca(null, 0.43, 1.03)] },
} as Bilancio;
const conIva = (iva: number) => ({ ...base, aliquote: { ...base.aliquote, iva } });
const riep = (crescita: number) => ({ costo_spesa: 0, costo_entrate: 0, crescita_spesa_netta: crescita, tetto: 1.7, perdita_reale: [] }) as RiepilogoBilancio;

let turno = 100;
async function apriCon(r: RiepilogoBilancio = riep(0.7)) {
  guscio.bozzaBilancio.mockResolvedValue(base);
  guscio.indicizza.mockResolvedValue(base);
  guscio.riepilogoBilancio.mockResolvedValue(r);
  await bozza.apri(++turno);
  await svuota();
}

beforeEach(() => vi.clearAllMocks());

describe("bozza", () => {
  it("apri chiede la bozza, il suo riepilogo e i costi di tutto, delle soglie e di ogni dicastero", async () => {
    await apriCon();
    expect(bozza.base).toBe(base);
    expect(bozza.corrente).toBeNull();
    expect(bozza.riepilogo?.crescita_spesa_netta).toBe(0.7);
    expect(Object.keys(bozza.costi).sort()).toEqual(["scaglioni", "tutto"]);
  });

  it("tre cambia di fila: due richieste, la prima e l'ultima; vince l'ultima; inVolo finché l'ultima non torna", async () => {
    await apriCon();
    guscio.riepilogoBilancio.mockReset();
    const prima = comandata<RiepilogoBilancio>(), ultima = comandata<RiepilogoBilancio>();
    guscio.riepilogoBilancio.mockReturnValueOnce(prima.p).mockReturnValueOnce(ultima.p);
    bozza.cambia(conIva(0.23));
    bozza.cambia(conIva(0.24));
    bozza.cambia(conIva(0.25));
    expect(guscio.riepilogoBilancio).toHaveBeenCalledTimes(1);
    expect(bozza.inVolo).toBe(true);
    prima.risolvi(riep(1));
    await svuota();
    expect(guscio.riepilogoBilancio).toHaveBeenCalledTimes(2);
    expect(guscio.riepilogoBilancio).toHaveBeenLastCalledWith(conIva(0.25));
    expect(bozza.inVolo).toBe(true);
    expect(bozza.riepilogo?.crescita_spesa_netta).not.toBe(1);
    ultima.risolvi(riep(2));
    await svuota();
    expect(bozza.inVolo).toBe(false);
    expect(bozza.riepilogo?.crescita_spesa_netta).toBe(2);
    expect(bozza.corrente).toEqual(conIva(0.25));
  });

  it("un rifiuto va in rifiuto e svuota il riepilogo; il riepilogo dopo lo toglie", async () => {
    await apriCon();
    guscio.riepilogoBilancio.mockRejectedValueOnce({ specie: "rifiuto", testo: "legge di bilancio non ammessa: iva: aliquota fuori da 0-0,9" });
    bozza.cambia(conIva(0.95));
    await svuota();
    expect(bozza.rifiuto).toBe("legge di bilancio non ammessa: iva: aliquota fuori da 0-0,9");
    expect(bozza.riepilogo).toBeNull();
    expect(bozza.inVolo).toBe(false);
    bozza.cambia(conIva(0.22));
    await svuota();
    expect(bozza.rifiuto).toBeNull();
    expect(bozza.riepilogo).not.toBeNull();
  });

  it("apri con lo stesso turno tiene il bilancio in memoria; con un turno nuovo rifà base e svuota corrente", async () => {
    await apriCon();
    bozza.cambia(conIva(0.23));
    await svuota();
    await bozza.apri(turno);
    expect(bozza.corrente).toEqual(conIva(0.23));
    expect(guscio.bozzaBilancio).toHaveBeenCalledTimes(1);
    const nuova = { ...base, anno: 2029 };
    guscio.bozzaBilancio.mockResolvedValue(nuova);
    await bozza.apri(++turno);
    expect(guscio.bozzaBilancio).toHaveBeenCalledTimes(2);
    expect(bozza.base).toBe(nuova);
    expect(bozza.corrente).toBeNull();
  });

  it("indicizza parte dal bilancio in memoria e ne chiede il riepilogo", async () => {
    await apriCon();
    const indicizzato = conIva(0.3);
    guscio.indicizza.mockResolvedValueOnce(indicizzato);
    await bozza.indicizza({ tipo: "tutto" });
    await svuota();
    expect(guscio.indicizza).toHaveBeenLastCalledWith(base, { tipo: "tutto" });
    expect(bozza.corrente).toBe(indicizzato);
    expect(guscio.riepilogoBilancio).toHaveBeenLastCalledWith(indicizzato);
  });

  it("due indicizza di fila: la seconda parte dal risultato della prima", async () => {
    await apriCon();
    const a = comandata<Bilancio>(), b = comandata<Bilancio>(), c = conIva(0.4);
    guscio.indicizza.mockReturnValueOnce(a.p).mockReturnValueOnce(b.p).mockResolvedValueOnce(c);
    const uno = bozza.indicizza({ tipo: "scaglioni" });
    const due = bozza.indicizza({ tipo: "tutto" });
    a.risolvi(conIva(0.3));
    await uno;
    b.risolvi(conIva(0.35));
    await due;
    expect(guscio.indicizza).toHaveBeenLastCalledWith(conIva(0.3), { tipo: "tutto" });
    expect(bozza.corrente).toBe(c);
  });

  it("annulla la Sanità indicizzata: corrente torna uguale alla base", async () => {
    guscio.bozzaBilancio.mockResolvedValue(piena);
    guscio.riepilogoBilancio.mockResolvedValue(riep(0.7));
    guscio.indicizza.mockResolvedValue(piena);
    await bozza.apri(++turno);
    await svuota();
    guscio.indicizza.mockResolvedValueOnce({ ...piena, dicasteri: [indicizzataTutta.dicasteri[0], piena.dicasteri[1]] });
    await bozza.indicizza({ tipo: "dicastero", id: "sanita" });
    expect(bozza.corrente).not.toEqual(piena);
    bozza.annulla({ tipo: "dicastero", id: "sanita" });
    expect(bozza.corrente).toEqual(piena);
    // la richiesta dell'indicizza è in volo: quella dell'annulla parte quando torna (ultimoVince)
    await svuota();
    expect(guscio.riepilogoBilancio).toHaveBeenLastCalledWith(piena);
  });

  it("annulla tutto riporta dicasteri e soglie; un'aliquota ritoccata prima resta", async () => {
    guscio.bozzaBilancio.mockResolvedValue(piena);
    guscio.riepilogoBilancio.mockResolvedValue(riep(0.7));
    guscio.indicizza.mockResolvedValue(piena);
    await bozza.apri(++turno);
    await svuota();
    const ritoccata = { ...indicizzataTutta, aliquote: { ...indicizzataTutta.aliquote, iva: 0.23, irpef: [{ ...indicizzataTutta.aliquote.irpef[0], aliquota: 0.24 }, indicizzataTutta.aliquote.irpef[1]] } };
    bozza.cambia(ritoccata);
    bozza.annulla({ tipo: "tutto" });
    expect(bozza.corrente!.dicasteri).toEqual(piena.dicasteri);
    expect(bozza.corrente!.aliquote.iva).toBe(0.23);
    expect(bozza.corrente!.aliquote.irpef).toEqual([{ ...piena.aliquote.irpef[0], aliquota: 0.24 }, piena.aliquote.irpef[1]]);
  });

  it("annulla le soglie lascia i dicasteri indicizzati", async () => {
    guscio.bozzaBilancio.mockResolvedValue(piena);
    guscio.riepilogoBilancio.mockResolvedValue(riep(0.7));
    guscio.indicizza.mockResolvedValue(piena);
    await bozza.apri(++turno);
    await svuota();
    bozza.cambia(indicizzataTutta);
    bozza.annulla({ tipo: "scaglioni" });
    expect(bozza.corrente!.dicasteri).toEqual(indicizzataTutta.dicasteri);
    expect(bozza.corrente!.aliquote.irpef).toEqual(piena.aliquote.irpef);
  });

  it("chiudi: lo stesso turno di un'altra partita richiede la bozza", async () => {
    await apriCon();
    bozza.cambia(conIva(0.23));
    bozza.chiudi();
    expect(bozza.base).toBeNull();
    expect(bozza.corrente).toBeNull();
    await bozza.apri(turno);
    expect(guscio.bozzaBilancio).toHaveBeenCalledTimes(2);
    expect(bozza.corrente).toBeNull();
  });

  it("un guasto della bozza: il prossimo apri dello stesso turno ritenta", async () => {
    guscio.bozzaBilancio.mockRejectedValueOnce({ specie: "guasto", testo: "disco pieno" });
    await bozza.apri(++turno);
    expect(bozza.rifiuto).toBe("disco pieno");
    guscio.bozzaBilancio.mockResolvedValue(base);
    guscio.riepilogoBilancio.mockResolvedValue(riep(0.7));
    await bozza.apri(turno);
    expect(guscio.bozzaBilancio).toHaveBeenCalledTimes(2);
    expect(bozza.base).toBe(base);
    expect(bozza.rifiuto).toBeNull();
  });
});
