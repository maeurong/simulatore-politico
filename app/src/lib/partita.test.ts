import { beforeEach, describe, expect, it, vi } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import type { PuntoSerie } from "$lib/tipi/PuntoSerie";
import { t } from "$lib/testi/t";

const { guscio, goto } = vi.hoisted(() => ({
  guscio: { chiudiTurno: vi.fn(), riprendi: vi.fn(), fondaPartita: vi.fn(), comanda: vi.fn(), serie: vi.fn(), riavvolgi: vi.fn(), partite: vi.fn(), confermaEredita: vi.fn() },
  goto: vi.fn(),
}));
vi.mock("$app/navigation", () => ({ goto }));
vi.mock("$lib/guscio/chiamate", async (originale) => ({ ...(await originale<object>()), guscio }));

import { partita } from "./partita.svelte";
import { rivelazione } from "./esito/rivelazione.svelte";
import { impostazioni } from "./impostazioni.svelte";
import { bozza } from "./bilancio/bozza.svelte";
import { registroCache } from "./registro/cache.svelte";
import type { Fondazione } from "$lib/tipi/Fondazione";

const voce = (in_evidenza: boolean) => ({ in_evidenza }) as Vista["esito"][number];
const prima = { turno: 3, finita: false, fase: "ordinaria", crisi: false, governa: true, esito: [] } as unknown as Vista;
const dopo = { ...prima, turno: 4, fase: "trattativa", esito: [voce(true), voce(false), voce(true)] } as Vista;
const serieVecchia = [{ turno: 2 }] as PuntoSerie[];
const serieNuova = [{ turno: 3 }] as PuntoSerie[];

beforeEach(() => {
  vi.clearAllMocks();
  partita.vista = prima;
  partita.serie = serieVecchia;
  partita.guasto = null;
  rivelazione.chiudi();
  impostazioni.velocita = "1";
  guscio.serie.mockResolvedValue(serieNuova);
});

describe("partita", () => {
  it("chiudiTurno: Vista e serie nuove, la rivelazione parte da quelle di prima, poi la prima pagina", async () => {
    guscio.chiudiTurno.mockResolvedValue(dopo);
    await partita.chiudiTurno();
    expect(partita.vista).toBe(dopo);
    expect(partita.serie).toBe(serieNuova);
    expect(rivelazione.prima).toBe(prima);
    expect(rivelazione.serie_prima).toBe(serieVecchia);
    expect(rivelazione.fine).toBe(2);
    expect(rivelazione.ferma).toBe(false);
    expect(goto).toHaveBeenCalledWith("/partita/prima-pagina", { keepFocus: true });
    expect(partita.occupato).toBe(false);
  });

  it("chiudiTurno a velocità istantanea: la pagina è già composta, nessuna rivelazione aperta", async () => {
    impostazioni.velocita = "istantaneo";
    guscio.chiudiTurno.mockResolvedValue(dopo);
    await partita.chiudiTurno();
    expect(rivelazione.prima).toBeNull();
    expect(rivelazione.ferma).toBe(true);
    expect(goto).toHaveBeenCalledWith("/partita/prima-pagina", { keepFocus: true });
  });

  it("a velocità istantanea una Vista di prima rimasta da una chiusura precedente si chiude", async () => {
    rivelazione.inizia({ turno: 1 } as Vista, [], 4);
    impostazioni.velocita = "istantaneo";
    guscio.chiudiTurno.mockResolvedValue(dopo);
    await partita.chiudiTurno();
    expect(rivelazione.prima).toBeNull();
  });

  it("chiudiTurno con la partita che finisce apre comunque la prima pagina (lettura 2)", async () => {
    guscio.chiudiTurno.mockResolvedValue({ ...dopo, finita: true } as Vista);
    await partita.chiudiTurno();
    expect(goto).toHaveBeenCalledWith("/partita/prima-pagina", { keepFocus: true });
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
    expect(rivelazione.prima).toBeNull();
    expect(partita.occupato).toBe(false);
  });

  describe("avanti", () => {
    const aperta = (turno: number) => ({ ...prima, turno, fermate: [] }) as unknown as Vista;
    const ferma = { ...prima, turno: 6, fermate: [{ tipo: "posto_riservato" }], esito: [voce(true)] } as unknown as Vista;

    it("chiude finché la Vista nuova ha fermate vuote: si ferma alla prima non vuota e conta le chiusure", async () => {
      guscio.chiudiTurno.mockResolvedValueOnce(aperta(4)).mockResolvedValueOnce(aperta(5)).mockResolvedValueOnce(ferma);
      await partita.avanti();
      expect(guscio.chiudiTurno).toHaveBeenCalledTimes(3);
      expect(partita.vista).toBe(ferma);
      expect(rivelazione.corsa).toEqual({ chiusure: 3, da: 3, fermate: ferma.fermate });
      expect(goto).toHaveBeenCalledWith("/partita/prima-pagina", { keepFocus: true });
      expect(partita.occupato).toBe(false);
    });

    it("la serie si chiede una volta sola, e la rivelazione parte dalla Vista prima dell'ultima chiusura con la serie di fine corsa", async () => {
      const penultima = aperta(5);
      const fine = [{ turno: 3 }, { turno: 4 }, { turno: 5 }] as PuntoSerie[];
      guscio.serie.mockResolvedValue(fine);
      guscio.chiudiTurno.mockResolvedValueOnce(aperta(4)).mockResolvedValueOnce(penultima).mockResolvedValueOnce(ferma);
      await partita.avanti();
      expect(guscio.serie).toHaveBeenCalledTimes(1);
      expect(rivelazione.prima).toBe(penultima);
      expect(rivelazione.serie_prima).toBe(fine);
      expect(rivelazione.serie_prima.some((x) => x.turno === penultima.turno - 1)).toBe(true);
      expect(rivelazione.fine).toBe(1);
      expect(partita.serie).toBe(fine);
    });

    it("una sola chiusura: corsa di uno, la rivelazione parte dalla Vista di prima", async () => {
      guscio.chiudiTurno.mockResolvedValueOnce(ferma);
      await partita.avanti();
      expect(rivelazione.corsa).toEqual({ chiusure: 1, da: 3, fermate: ferma.fermate });
      expect(rivelazione.prima).toBe(prima);
    });

    it("un guasto alla seconda chiusura lascia la Vista della prima, corsa di uno e il guasto detto", async () => {
      const prima_chiusura = aperta(4);
      guscio.chiudiTurno.mockResolvedValueOnce(prima_chiusura).mockRejectedValueOnce({ specie: "guasto", testo: "disco pieno" });
      await partita.avanti();
      expect(partita.vista).toBe(prima_chiusura);
      expect(rivelazione.corsa).toEqual({ chiusure: 1, da: 3, fermate: [] });
      expect(partita.guasto).toBe(`disco pieno ${t("cornice.corsa_interrotta", { n: 1 })}`);
      expect(goto).toHaveBeenCalledWith("/partita/prima-pagina", { keepFocus: true });
      expect(partita.occupato).toBe(false);
    });

    it("un guasto alla prima chiusura lascia tutto com'era e non apre niente", async () => {
      guscio.chiudiTurno.mockRejectedValueOnce({ specie: "guasto", testo: "disco pieno" });
      await partita.avanti();
      expect(partita.vista).toBe(prima);
      expect(rivelazione.corsa).toBeNull();
      expect(goto).not.toHaveBeenCalled();
    });

    it("uscire all'edicola a metà corsa ferma le chiusure: la partita lasciata non si salva più e la vista resta null", async () => {
      guscio.chiudiTurno
        .mockImplementationOnce(async () => {
          partita.esci();
          return aperta(4);
        })
        .mockResolvedValueOnce(aperta(5))
        .mockResolvedValueOnce(ferma);
      await partita.avanti();
      expect(guscio.chiudiTurno).toHaveBeenCalledTimes(1);
      expect(partita.vista).toBeNull();
      expect(partita.occupato).toBe(false);
      expect(rivelazione.corsa).toBeNull();
      expect(partita.guasto).toBeNull();
    });

    it("una chiusura normale dopo la corsa la cancella", async () => {
      guscio.chiudiTurno.mockResolvedValueOnce(ferma);
      await partita.avanti();
      guscio.chiudiTurno.mockResolvedValueOnce(dopo);
      await partita.chiudiTurno();
      expect(rivelazione.corsa).toBeNull();
    });
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
    expect(goto).toHaveBeenCalledWith("/partita/prima-pagina", { keepFocus: true });
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
    expect(rivelazione.prima).toBeNull();
    expect(partita.occupato).toBe(false);
  });

  it("esci chiude la rivelazione aperta", () => {
    rivelazione.inizia(prima, serieVecchia, 3);
    partita.esci();
    expect(rivelazione.prima).toBeNull();
    expect(rivelazione.ferma).toBe(true);
  });

  it("riprendi, fonda e riavvolgi riusciti chiudono la rivelazione aperta", async () => {
    guscio.riprendi.mockResolvedValue(dopo);
    rivelazione.inizia(prima, serieVecchia, 3);
    await partita.riprendi("x");
    expect(rivelazione.prima).toBeNull();
    guscio.fondaPartita.mockResolvedValue(dopo);
    rivelazione.inizia(prima, serieVecchia, 3);
    await partita.fonda(null, {} as never);
    expect(rivelazione.prima).toBeNull();
    guscio.riavvolgi.mockResolvedValue(dopo);
    rivelazione.inizia(prima, serieVecchia, 3);
    await partita.riavvolgi(2);
    expect(rivelazione.prima).toBeNull();
  });

  it("goto che fallisce: la rivelazione si chiude e l'errore risale", async () => {
    guscio.chiudiTurno.mockResolvedValue(dopo);
    goto.mockRejectedValueOnce(new Error("rotta"));
    await expect(partita.chiudiTurno()).rejects.toThrow("rotta");
    expect(rivelazione.prima).toBeNull();
    expect(rivelazione.ferma).toBe(true);
  });

  it("goto annullato (non si approda sulla prima pagina): la rivelazione si chiude", async () => {
    vi.stubGlobal("location", { pathname: "/partita/cruscotto" });
    guscio.chiudiTurno.mockResolvedValue(dopo);
    await partita.chiudiTurno();
    expect(rivelazione.prima).toBeNull();
    vi.stubGlobal("location", { pathname: "/partita/prima-pagina" });
    await partita.chiudiTurno();
    expect(rivelazione.prima).toBe(dopo);
    vi.unstubAllGlobals();
  });

  describe("eredità", () => {
    const giocatore = { id: "mio", nome: "Noi" };
    const bozzaVista = { ...prima, finita: true, giocatore: "mio", partiti: [giocatore] } as unknown as Vista;
    const indice = (id: string, nome: string, ultimo: number) => ({ indice: { id, ultimo_salvataggio: ultimo, testata: { nome } } });
    beforeEach(() => {
      partita.esci();
      partita.vista = bozzaVista;
    });
    // la cache com'è quando #esegui chiede la serie: la Vista nuova è già assegnata, e l'effect del Registro l'ha già vista
    const turniAllaSerie = () => {
      const visti: unknown[] = [];
      guscio.serie.mockImplementation(async () => (visti.push(registroCache.turni), serieNuova));
      return visti;
    };

    it("riavvolgi: la Vista del turno e la serie nuova", async () => {
      guscio.riavvolgi.mockResolvedValue(dopo);
      expect(await partita.riavvolgi(20)).toBe(dopo);
      expect(guscio.riavvolgi).toHaveBeenCalledWith(20);
      expect(partita.vista).toBe(dopo);
      expect(guscio.riprendi).not.toHaveBeenCalled();
    });

    it("riavvolgi con un guasto: riprende dal disco la partita più recente del giocatore e il guasto resta scritto", async () => {
      const disco = { ...bozzaVista, turno: 21 } as Vista;
      guscio.riavvolgi.mockRejectedValue({ specie: "guasto", testo: "disco pieno" });
      guscio.partite.mockResolvedValue([indice("a", "Noi", 5), indice("b", "Noi", 9), indice("c", "Altri", 99)]);
      guscio.riprendi.mockResolvedValue(disco);
      expect(await partita.riavvolgi(20)).toBeNull();
      expect(guscio.riprendi).toHaveBeenCalledWith("b");
      expect(partita.vista).toBe(disco);
      expect(partita.guasto).toBe(`disco pieno ${t("cornice.rimasta_com_era")}`);
      expect(partita.occupato).toBe(false);
    });

    it("riavvolgi con un guasto: la partita ripresa con riprendi si ritrova per id, non per nome", async () => {
      guscio.riprendi.mockResolvedValue(bozzaVista);
      await partita.riprendi("a");
      guscio.riavvolgi.mockRejectedValue({ specie: "guasto", testo: "no" });
      guscio.partite.mockResolvedValue([indice("a", "Noi", 5), indice("b", "Noi", 9)]);
      await partita.riavvolgi(20);
      expect(guscio.riprendi).toHaveBeenLastCalledWith("a");
    });

    it("riavvolgi due volte di fila, anche nella finestra dopo il guasto: parte una volta sola", async () => {
      guscio.riavvolgi.mockRejectedValue({ specie: "guasto", testo: "no" });
      let sblocca!: (v: unknown[]) => void;
      guscio.partite.mockReturnValue(new Promise((r) => (sblocca = r)));
      const prima = partita.riavvolgi(20);
      await vi.waitFor(() => expect(guscio.partite).toHaveBeenCalled());
      expect(await partita.riavvolgi(20)).toBeNull();
      sblocca([]);
      await prima;
      expect(guscio.riavvolgi).toHaveBeenCalledTimes(1);
      expect(await partita.riavvolgi(20)).toBeNull();
      expect(guscio.riavvolgi).toHaveBeenCalledTimes(2);
    });

    it("riavvolgi e riprendi falliscono insieme: il guasto mostrato è quello di riavvolgi", async () => {
      guscio.riavvolgi.mockRejectedValue({ specie: "guasto", testo: "disco pieno" });
      guscio.partite.mockResolvedValue([indice("b", "Noi", 9)]);
      guscio.riprendi.mockRejectedValue({ specie: "guasto", testo: "file illeggibile" });
      expect(await partita.riavvolgi(20)).toBeNull();
      expect(guscio.riprendi).toHaveBeenCalledWith("b");
      expect(partita.guasto).toBe(`disco pieno ${t("cornice.rimasta_com_era")}`);
    });

    it("fonda, riprendi, riavvolgi riuscito ed esci svuotano la cache del registro", async () => {
      const svuota = vi.spyOn(registroCache, "svuota");
      guscio.fondaPartita.mockResolvedValue(dopo);
      await partita.fonda(null, {} as Fondazione);
      guscio.riprendi.mockResolvedValue(dopo);
      await partita.riprendi("a");
      guscio.riavvolgi.mockResolvedValue(dopo);
      await partita.riavvolgi(2);
      partita.esci();
      expect(svuota).toHaveBeenCalledTimes(4);
      svuota.mockRestore();
    });

    it("riavvolgi che fallisce col riallineamento riuscito: il registro letto resta in cache", async () => {
      const letti = [{ turno: 0, guasto: "x" }];
      registroCache.turni = letti;
      guscio.riavvolgi.mockRejectedValue({ specie: "guasto", testo: "disco pieno" });
      guscio.partite.mockResolvedValue([indice("b", "Noi", 9)]);
      guscio.riprendi.mockResolvedValue(bozzaVista);
      const allaSerie = turniAllaSerie();
      await partita.riavvolgi(3);
      expect(guscio.riprendi).toHaveBeenCalledWith("b");
      expect(allaSerie).toEqual([letti]);
      expect(registroCache.turni).toBe(letti);
    });

    it("riavvolgi che fallisce a metà e il disco riapre a un turno più vecchio: la cache del registro si svuota", async () => {
      registroCache.turni = [{ turno: 0, guasto: "x" }];
      guscio.riavvolgi.mockRejectedValue({ specie: "guasto", testo: "disco pieno" });
      guscio.partite.mockResolvedValue([indice("b", "Noi", 9)]);
      guscio.riprendi.mockResolvedValue({ ...bozzaVista, turno: bozzaVista.turno - 2 } as Vista);
      const allaSerie = turniAllaSerie();
      await partita.riavvolgi(3);
      // già vuota quando la Vista nuova è in vista: l'effect del Registro che ricarica la riempie, e niente la azzera dopo
      expect(allaSerie).toEqual([[]]);
      expect(registroCache.turni).toEqual([]);
    });

    it("riprendi, fonda e riavvolgi riuscito svuotano il registro prima che la Vista nuova sia in vista", async () => {
      const allaSerie = turniAllaSerie();
      const riempi = () => (registroCache.turni = [{ turno: 0, guasto: "x" }]);
      guscio.riprendi.mockResolvedValue(dopo);
      riempi();
      await partita.riprendi("a");
      guscio.fondaPartita.mockResolvedValue(dopo);
      riempi();
      await partita.fonda(null, {} as Fondazione);
      guscio.riavvolgi.mockResolvedValue(dopo);
      riempi();
      await partita.riavvolgi(2);
      expect(allaSerie).toEqual([[], [], []]);
    });

    it("riavvolgi rifiutato: riprende comunque dal disco e rilancia il rifiuto", async () => {
      const rifiuto = { specie: "rifiuto", testo: "turno non riavvolgibile" };
      guscio.riavvolgi.mockRejectedValue(rifiuto);
      guscio.partite.mockResolvedValue([indice("b", "Noi", 9)]);
      guscio.riprendi.mockResolvedValue(bozzaVista);
      await expect(partita.riavvolgi(3)).rejects.toEqual(rifiuto);
      expect(guscio.riprendi).toHaveBeenCalledWith("b");
    });

    it("confermaEredita: vero a conferma riuscita; un guasto la lascia in bozza e lo dice", async () => {
      guscio.confermaEredita.mockResolvedValueOnce(null);
      expect(await partita.confermaEredita()).toBe(true);
      guscio.confermaEredita.mockRejectedValueOnce({ specie: "guasto", testo: "scrittura fallita" });
      expect(await partita.confermaEredita()).toBe(false);
      expect(partita.guasto).toBe(`scrittura fallita ${t("cornice.rimasta_com_era")}`);
      expect(partita.occupato).toBe(false);
    });
  });
});
