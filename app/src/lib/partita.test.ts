import { beforeEach, describe, expect, it, vi } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import type { PuntoSerie } from "$lib/tipi/PuntoSerie";
import { t } from "$lib/testi/t";

const { guscio, goto } = vi.hoisted(() => ({
  guscio: { chiudiTurno: vi.fn(), riprendi: vi.fn(), fondaPartita: vi.fn(), comanda: vi.fn(), serie: vi.fn() },
  goto: vi.fn(),
}));
vi.mock("$app/navigation", () => ({ goto }));
vi.mock("$lib/guscio/chiamate", async (originale) => ({ ...(await originale<object>()), guscio }));

import { partita } from "./partita.svelte";
import { bozza } from "./bilancio/bozza.svelte";

const prima = { turno: 3, finita: false, fase: "ordinaria", crisi: false, governa: true } as Vista;
const dopo = { ...prima, turno: 4, fase: "trattativa" } as Vista;
const serieVecchia = [{ turno: 2 }] as PuntoSerie[];
const serieNuova = [{ turno: 3 }] as PuntoSerie[];

beforeEach(() => {
  vi.clearAllMocks();
  partita.vista = prima;
  partita.serie = serieVecchia;
  partita.guasto = null;
  guscio.serie.mockResolvedValue(serieNuova);
});

describe("partita", () => {
  it("chiudiTurno: Vista e serie nuove, poi la casa del turno nuovo", async () => {
    guscio.chiudiTurno.mockResolvedValue(dopo);
    await partita.chiudiTurno();
    expect(partita.vista).toBe(dopo);
    expect(partita.serie).toBe(serieNuova);
    expect(goto).toHaveBeenCalledWith("/partita/trattativa", { keepFocus: true });
    expect(partita.occupato).toBe(false);
  });

  it("fonda passa scenario e fondazione al guscio e prende la Vista", async () => {
    const zero = { ...prima, turno: 0, fase: "sciolta", governa: false } as Vista;
    guscio.fondaPartita.mockResolvedValue(zero);
    const fondazione = { nome: "Noi" } as never;
    expect(await partita.fonda(null, fondazione)).toBe(zero);
    expect(guscio.fondaPartita).toHaveBeenCalledWith(null, fondazione);
    expect(partita.vista).toBe(zero);
  });

  it("comanda: la Vista nuova; un rifiuto si rilancia e la Vista resta com'era", async () => {
    guscio.comanda.mockResolvedValueOnce(dopo);
    expect(await partita.comanda({ tipo: "dimettiti" })).toBe(dopo);
    expect(partita.vista).toBe(dopo);
    partita.vista = prima;
    const rifiuto = { specie: "rifiuto", testo: "comando non ammesso: no" };
    guscio.comanda.mockRejectedValueOnce(rifiuto);
    await expect(partita.comanda({ tipo: "dimettiti" })).rejects.toEqual(rifiuto);
    expect(partita.vista).toBe(prima);
    expect(partita.occupato).toBe(false);
  });

  it("due comanda sovrapposti: una chiamata sola", async () => {
    guscio.comanda.mockResolvedValue(dopo);
    const [a, b] = await Promise.all([partita.comanda({ tipo: "dimettiti" }), partita.comanda({ tipo: "dimettiti" })]);
    expect(guscio.comanda).toHaveBeenCalledTimes(1);
    expect([a, b]).toEqual([dopo, null]);
  });

  it("un guasto del comando lascia la Vista com'era e lo dice", async () => {
    guscio.chiudiTurno.mockRejectedValue({ specie: "guasto", testo: "disco pieno" });
    await partita.chiudiTurno();
    expect(partita.vista).toBe(prima);
    expect(partita.guasto).toBe(`disco pieno ${t("cornice.rimasta_com_era")}`);
    expect(goto).not.toHaveBeenCalled();
    expect(partita.occupato).toBe(false);
  });

  it("esci: via Vista, serie e guasto, per l'edicola", () => {
    partita.guasto = "vecchio";
    partita.esci();
    expect(partita.vista).toBeNull();
    expect(partita.serie).toEqual([]);
    expect(partita.guasto).toBeNull();
  });

  it("esci chiude anche la bozza del bilancio: un'altra partita allo stesso turno non la ritrova", () => {
    const chiudi = vi.spyOn(bozza, "chiudi");
    partita.esci();
    expect(chiudi).toHaveBeenCalledOnce();
    chiudi.mockRestore();
  });

  it("comando riuscito e serie guasta: la Vista nuova resta, la serie è quella di prima", async () => {
    guscio.chiudiTurno.mockResolvedValue(dopo);
    guscio.serie.mockRejectedValue({ specie: "guasto", testo: "serie illeggibile" });
    await partita.chiudiTurno();
    expect(partita.vista).toBe(dopo);
    expect(partita.serie).toBe(serieVecchia);
    expect(partita.guasto).toBe(`serie illeggibile ${t("cornice.grafici_non_aggiornati")}`);
    expect(goto).toHaveBeenCalledWith("/partita/trattativa", { keepFocus: true });
    expect(partita.occupato).toBe(false);
  });

  it("un rifiuto torna a chi ha chiamato e libera occupato", async () => {
    guscio.riprendi.mockRejectedValue({ specie: "rifiuto", testo: "no" });
    await expect(partita.riprendi("x")).rejects.toEqual({ specie: "rifiuto", testo: "no" });
    expect(partita.vista).toBe(prima);
    expect(partita.guasto).toBeNull();
    expect(partita.occupato).toBe(false);
  });

  it("occupato durante la chiamata; la seconda chiamata in volo è ignorata", async () => {
    let rispondi!: (v: Vista) => void;
    guscio.chiudiTurno.mockReturnValue(new Promise<Vista>((r) => (rispondi = r)));
    const prima = partita.chiudiTurno();
    expect(partita.occupato).toBe(true);
    await partita.chiudiTurno();
    expect(guscio.chiudiTurno).toHaveBeenCalledTimes(1);
    rispondi(dopo);
    await prima;
    expect(partita.occupato).toBe(false);
    expect(partita.vista).toBe(dopo);
  });

  it("comanda: con il primo ancora in volo il secondo dà null e il guscio è chiamato una volta", async () => {
    let rispondi!: (v: Vista) => void;
    guscio.comanda.mockReturnValue(new Promise<Vista>((r) => (rispondi = r)));
    const comando = { tipo: "ferma", dicastero: "sanita" } as never;
    const primo = partita.comanda(comando);
    expect(await partita.comanda(comando)).toBeNull();
    expect(guscio.comanda).toHaveBeenCalledTimes(1);
    rispondi(dopo);
    expect(await primo).toBe(dopo);
  });

  it("esci con un comando in volo: la risposta che arriva dopo si scarta", async () => {
    let rispondi!: (v: Vista) => void;
    guscio.chiudiTurno.mockReturnValue(new Promise<Vista>((r) => (rispondi = r)));
    const inVolo = partita.chiudiTurno();
    partita.esci();
    rispondi(dopo);
    await inVolo;
    expect(partita.vista).toBeNull();
    expect(partita.serie).toEqual([]);
    expect(partita.guasto).toBeNull();
    expect(goto).not.toHaveBeenCalled();
    expect(partita.occupato).toBe(false);
  });

  it("esci con un comando in volo, poi rifiutato: null, niente throw né guasto", async () => {
    let rifiuta!: (e: unknown) => void;
    guscio.comanda.mockReturnValue(new Promise<Vista>((_, r) => (rifiuta = r)));
    const inVolo = partita.comanda({ tipo: "dimettiti" });
    partita.esci();
    rifiuta({ specie: "rifiuto", testo: "comando non ammesso: no" });
    expect(await inVolo).toBeNull();
    expect(partita.guasto).toBeNull();
    expect(partita.occupato).toBe(false);
  });

  it("esci con la serie in volo, comando già riuscito: la serie si scarta", async () => {
    let rispondi!: (s: PuntoSerie[]) => void;
    guscio.chiudiTurno.mockResolvedValue(dopo);
    guscio.serie.mockReturnValue(new Promise<PuntoSerie[]>((r) => (rispondi = r)));
    const inVolo = partita.chiudiTurno();
    await vi.waitFor(() => expect(guscio.serie).toHaveBeenCalled());
    partita.esci();
    rispondi(serieNuova);
    await inVolo;
    expect(partita.vista).toBeNull();
    expect(partita.serie).toEqual([]);
    expect(goto).not.toHaveBeenCalled();
    expect(partita.occupato).toBe(false);
  });
});
