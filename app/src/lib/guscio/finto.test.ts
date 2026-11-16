import { describe, expect, it, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import { rispondi, trovaFile, vistaDi } from "./finto";
import governo from "../../../../.scratch/v1-piano-11-ui/viste/03-governo-ordinario.json";
import turnoZero from "../../../../.scratch/v1-piano-11-ui/viste/01-turno-zero.json";
import edicola from "../../../../.scratch/v1-piano-11-ui/viste/13-edicola.json";
import editorJson from "../../../../.scratch/v1-piano-11-ui/viste/14-editor.json";
import fondazione from "../../../../.scratch/v1-piano-11-ui/viste/15-fondazione.json";
import cruscottoJson from "../../../../.scratch/v1-piano-11-ui/viste/17-cruscotto.json";
import paese from "../../../../.scratch/v1-piano-11-ui/viste/23-paese.json";
import campagna from "../../../../.scratch/v1-piano-11-ui/viste/26-campagna.json";
import trattativa from "../../../../.scratch/v1-piano-11-ui/viste/27-trattativa.json";
import governoJson from "../../../../.scratch/v1-piano-11-ui/viste/18-governo.json";
import aulaJson from "../../../../.scratch/v1-piano-11-ui/viste/19-aula.json";
import anteprimeJson from "../../../../.scratch/v1-piano-11-ui/viste/20-anteprima.json";
import contiJson from "../../../../.scratch/v1-piano-11-ui/viste/21-conti.json";
import registroJson from "../../../../.scratch/v1-piano-11-ui/viste/25-registro.json";

/* eslint-disable @typescript-eslint/no-explicit-any */
const editor: any = editorJson;
const cruscotto: any = cruscottoJson;
const registro: any = registroJson;
const c = {
  campione: registro[0],
  turnoZero,
  edicola,
  editor,
  fondazione,
  paese,
  campagna,
  trattativa,
  stato: "zero",
};

describe("vistaDi", () => {
  it("campione con turno", () => expect(vistaDi(turnoZero)?.turno).toBe(0));
  it("stato del cruscotto", () => expect(vistaDi(cruscotto["ultimatum"])).not.toBeNull());
  it("voce del registro: ha un turno suo ma la Vista è in .prima", () => {
    const voce = (registroJson as unknown as { prima: { giocatore: string } }[])[0];
    expect(vistaDi(voce)).toBe(voce.prima);
  });
});

describe("rispondi", () => {
  it("chiudi_turno dà .dopo", () => {
    expect((rispondi("chiudi_turno", {}, c) as { turno: number }).turno).toBe(1);
  });
  it("riepilogo di null dà la base", () => {
    const r = rispondi("riepilogo_scenario", { scenario: null }, c) as { spread_pb: number };
    expect(r.spread_pb).toBe(editor.riepiloghi[0].riepilogo.spread_pb);
  });
  it("scenario con errore del campione dà rifiuto", () => {
    const inc = editor.riepiloghi.find((r: { errore?: string }) => r.errore);
    expect(() => rispondi("riepilogo_scenario", { scenario: inc.scenario }, c)).toThrow(
      expect.objectContaining({ specie: "rifiuto", testo: inc.errore }),
    );
  });
  it("un cursore solo dà il passo più vicino della griglia", () => {
    const s = structuredClone(editor.riepiloghi[0].scenario);
    s.cursori.debito_pil = 111;
    const r = rispondi("riepilogo_scenario", { scenario: s }, c);
    expect(r).toEqual(editor.griglia.debito_pil[0].riepilogo);
  });
  it("comando ignoto è rifiuto", () => {
    expect(() => rispondi("anteprima_sfiducia", {}, c)).toThrow(
      expect.objectContaining({ specie: "rifiuto", testo: expect.stringContaining("non nel campione") }),
    );
  });
});

describe("rispondi: il resto dei passacarte", () => {
  it("carica_scenario dà lo scenario col nome chiesto", () => {
    const nome = editor.riepiloghi[1].scenario.nome;
    expect(rispondi("carica_scenario", { nome }, c)).toEqual(editor.riepiloghi[1].scenario);
  });
  it("carica_scenario con nome ignoto è rifiuto", () => {
    expect(() => rispondi("carica_scenario", { nome: "Nessuno" }, c)).toThrow(
      expect.objectContaining({ specie: "rifiuto" }),
    );
  });
  it("catalogo_scenario: esattamente le cinque chiavi, i partiti dalla fondazione", () => {
    const cat = rispondi("catalogo_scenario", {}, c) as Record<string, unknown>;
    expect(Object.keys(cat).sort()).toEqual(["cursori", "disposizioni", "legge_di_base", "leggi_elettorali", "partiti"]);
    expect(cat.partiti).toEqual((fondazione as { partiti: unknown }).partiti);
  });
  it("anteprima_fondazione: orientamento 1 dà l'elettorato a destra, il resto l'oggi", () => {
    const anteprime = (fondazione as { anteprime: Record<string, unknown> }).anteprime;
    expect(rispondi("anteprima_fondazione", { scenario: { cursori: { orientamento: 1 } } }, c)).toEqual(
      anteprime["elettorato a destra, seme 4"],
    );
    expect(rispondi("anteprima_fondazione", { scenario: null }, c)).toEqual(anteprime["oggi, seme 4"]);
  });
  it("fonda_partita dà la Vista del turno zero", () => {
    expect((rispondi("fonda_partita", {}, c) as { turno: number }).turno).toBe(0);
  });
  it("partite e albo vengono dall'edicola", () => {
    expect(rispondi("partite", {}, c)).toEqual((edicola as { partite: unknown }).partite);
    expect(rispondi("albo", {}, c)).toEqual((edicola as { albo: unknown }).albo);
  });
});

describe("campione che manca", () => {
  const rifiutoDi = (comando: string) =>
    expect.objectContaining({ specie: "rifiuto", testo: `non nel campione: ${comando}` });
  it("chiudi_turno senza .dopo è rifiuto", () => {
    expect(() => rispondi("chiudi_turno", {}, { ...c, campione: governo })).toThrow(rifiutoDi("chiudi_turno"));
  });
  it.each(["chiudi_turno", "comanda", "riprendi"])("%s con campione nullo è rifiuto", (cmd) => {
    expect(() => rispondi(cmd, {}, { ...c, campione: null })).toThrow(rifiutoDi(cmd));
  });
  it("serie con campione nullo dà []", () => {
    expect(rispondi("serie", {}, { ...c, campione: null })).toEqual([]);
  });
});

describe("trovaFile", () => {
  const nomi = ["04-governo-bilancio.bozza.json", "04-governo-bilancio.json", "17-cruscotto.json"];
  it("nome esatto prima del prefisso", () => {
    expect(trovaFile(nomi, "04-governo-bilancio")).toBe("04-governo-bilancio.json");
    expect(trovaFile(nomi, "04-governo-bilancio.bozza")).toBe("04-governo-bilancio.bozza.json");
  });
  it("prefisso, il più corto vince", () => {
    expect(trovaFile(nomi, "17")).toBe("17-cruscotto.json");
    expect(trovaFile(nomi, "04")).toBe("04-governo-bilancio.json");
  });
});

describe("rispondi: la fetta 11b", () => {
  const rifiuta = (testo: string) => expect.objectContaining({ specie: "rifiuto", testo });
  const comanda = (comando: unknown, campione: unknown, stato = "zero") =>
    rispondi("comanda", { comando }, { ...c, campione, stato });
  const zeroCampagna = (campagna as unknown as Record<string, unknown>)["zero"];
  const piano = (campagna as unknown as { piano: { ok: unknown[] } }).piano.ok;
  const spese = (piano as { bersaglio: unknown; leva: unknown; euro: number }[]).map(({ bersaglio, leva, euro }) => ({ bersaglio, leva, euro }));

  it("anteprima_campagna: le spese del piano danno le sue tre voci", () => {
    expect(rispondi("anteprima_campagna", { spese }, c)).toEqual(piano);
  });
  it("anteprima_campagna: spese non giocate, la voce dello stesso bersaglio con l'importo più vicino", () => {
    const r = rispondi("anteprima_campagna", { spese: [{ ...spese[0], euro: 290000 }] }, c) as { bersaglio: unknown; euro: number }[];
    expect(r).toHaveLength(1);
    expect(r[0].bersaglio).toEqual(spese[0].bersaglio);
    // importi dell'anteprima per i pensionati: 25k 50k 100k 200k 400k 500k 800k
    expect(r[0].euro).toBe(200000);
  });
  it("anteprima_campagna: un bersaglio mai giocato è rifiuto", () => {
    expect(() => rispondi("anteprima_campagna", { spese: [{ bersaglio: { regione: 99 }, leva: "mobilitare", euro: 1 }] }, c)).toThrow(
      rifiuta("non nel campione: anteprima_campagna"),
    );
  });

  it("elettori_campione: abruzzo|autonomo|alta dà tre elettori, il primo Fabio Caruso", () => {
    const r = rispondi("elettori_campione", { regione: "abruzzo", condizione: "autonomo", fascia: "alta" }, c) as { nome: string }[];
    expect(r).toHaveLength(3);
    expect(r[0].nome).toBe("Fabio Caruso");
  });
  it("elettori_campione fuori dal 23 legge lo stato governo", () => {
    const r = rispondi("elettori_campione", { regione: "abruzzo", condizione: "autonomo", fascia: "alta" }, { ...c, campione: governo, stato: null });
    expect(r).toEqual((paese as unknown as Record<string, { campione: Record<string, unknown> }>).governo.campione["abruzzo|autonomo|alta"]);
  });

  it("dichiara_coalizione: la Vista con coalizione e disponibilita della prova", () => {
    const v = comanda({ tipo: "dichiara_coalizione", partiti: ["centrodestra", "destra"], cessioni: [] }, zeroCampagna) as { coalizione: string[]; disponibilita: unknown };
    expect(v.coalizione).toEqual(["centrodestra", "destra"]);
    const prova = (campagna as unknown as { prove: { zero: { coalizioni: Record<string, { ok: unknown[] }> } } }).prove.zero.coalizioni["centrodestra+destra"];
    expect(v.disponibilita).toEqual(prova.ok[0]);
  });
  it("dichiara_coalizione: nessun partito è la chiave vuota", () => {
    expect((comanda({ tipo: "dichiara_coalizione", partiti: [], cessioni: [] }, zeroCampagna) as { coalizione: string[] }).coalizione).toEqual([]);
  });
  it("dichiara_coalizione: il rifiuto della prova", () => {
    expect(() => comanda({ tipo: "dichiara_coalizione", partiti: ["centro"], cessioni: [] }, zeroCampagna)).toThrow(
      rifiuta("comando non ammesso: Centro non entra: posizione troppo lontana dalla tua"),
    );
  });

  it("compra sondaggio_privato: la Vista col sondaggio della prova", () => {
    const v = comanda({ tipo: "compra", acquisto: "sondaggio_privato" }, zeroCampagna) as { acquisti: { sondaggio_privato: unknown } };
    const prova = (campagna as unknown as { prove: { zero: { privato: { ok: unknown } } } }).prove.zero.privato.ok;
    expect(v.acquisti.sondaggio_privato).toEqual(prova);
  });

  it("campagna: i rifiuti del campione, uno per caso", () => {
    const rif = (spese: unknown[]) => () => comanda({ tipo: "campagna", spese }, zeroCampagna);
    expect(rif([...spese, spese[0]])).toThrow(rifiuta("comando non ammesso: Al massimo 3 bersagli"));
    expect(rif([{ ...spese[0], euro: 0 }])).toThrow(rifiuta("comando non ammesso: La spesa deve essere maggiore di zero"));
    expect(rif([spese[0], spese[0]])).toThrow(rifiuta("comando non ammesso: Bersaglio ripetuto: pensionati"));
    expect(rif([{ ...spese[0], euro: 5_000_000 }])).toThrow(rifiuta("comando non ammesso: Spesa oltre la cassa, che è di 1.000.000 euro"));
    expect(rif(spese)).toThrow(rifiuta("non nel campione: comanda"));
  });
  it("riposiziona: la posizione del campione, o il suo rifiuto", () => {
    expect(Array.isArray((comanda({ tipo: "riposiziona", posizione: 4 }, zeroCampagna) as { partiti: unknown }).partiti)).toBe(true);
    expect(() => comanda({ tipo: "riposiziona", posizione: 5 }, zeroCampagna)).toThrow(rifiuta("comando non ammesso: Non si scavalca Centrodestra"));
  });
  it("prometti: tre promesse sono il rifiuto del campione", () => {
    expect(() => comanda({ tipo: "prometti", promesse: [["a", "x"], ["b", "y"], ["c", "z"]] }, zeroCampagna)).toThrow(
      rifiuta("comando non ammesso: Al massimo 2 promesse"),
    );
  });

  const t27 = trattativa as unknown as Record<string, Record<string, unknown>>;
  it("allarga a tutti e chiedi_guida: la Vista dopo del campione", () => {
    expect(comanda({ tipo: "allarga", partiti: ["a", "b"] }, t27["senza_maggioranza"])).toEqual((t27["senza_maggioranza"]["allarga_tutti"] as { vista: unknown }).vista);
    expect(comanda({ tipo: "chiedi_guida" }, t27["guida"])).toEqual(t27["guida"]["dopo_chiedi"]);
    expect(comanda({ tipo: "rifiuta_ribaltone" }, t27["guida"])).toEqual(t27["guida"]["dopo_rifiuta"]);
  });
  it("allarga a uno: il suo rifiuto o la sua Vista", () => {
    expect(() => comanda({ tipo: "allarga", partiti: ["estrema_sinistra"] }, t27["senza_maggioranza"])).toThrow(
      rifiuta("comando non ammesso: Estrema sinistra non entra: posizione troppo lontana dalla tua"),
    );
  });
  it("proponi_spartizione: la spartizione giocata dà le richieste dopo il comando", () => {
    const prima = (t27["premio"]["proposte"] as { spartizione: unknown; richieste: unknown }[])[1];
    const v = comanda({ tipo: "proponi_spartizione", dicasteri: prima.spartizione }, t27["premio"]) as { richieste: unknown };
    expect(v.richieste).toEqual(prima.richieste);
  });

  it("da una Vista non-27 il ripiego è il 27: la Vista dopo che porta", () => {
    expect(comanda({ tipo: "allarga", partiti: ["a", "b"] }, governo)).toEqual((t27["senza_maggioranza"]["allarga_tutti"] as { vista: unknown }).vista);
    expect(comanda({ tipo: "chiedi_guida" }, governo)).toEqual(t27["guida"]["dopo_chiedi"]);
    const giocata = (t27["premio"]["proposte"] as { spartizione: unknown; richieste: unknown }[])[1];
    const v = comanda({ tipo: "proponi_spartizione", dicasteri: giocata.spartizione }, governo) as { richieste: unknown };
    expect(v.richieste).toEqual(giocata.richieste);
  });
  it("da una Vista non-27 un comando che il 27 non ha giocato resta rifiuto", () => {
    expect(() => comanda({ tipo: "allarga", partiti: ["inventato"] }, governo)).toThrow(rifiuta("non nel campione: comanda"));
    expect(() => comanda({ tipo: "proponi_spartizione", dicasteri: [] }, governo)).toThrow(rifiuta("non nel campione: comanda"));
  });
  it("un comando non giocato resta un rifiuto", () => {
    expect(() => comanda({ tipo: "dimettiti" }, zeroCampagna)).toThrow(rifiuta("non nel campione: comanda"));
  });
});

describe("rispondi: la fetta 11c", () => {
  const rifiuta = (testo: string) => expect.objectContaining({ specie: "rifiuto", testo });
  const gov: any = governoJson;
  const aula: any = aulaJson;
  const ante: any = anteprimeJson;
  const cc = { ...c, campione: governo, cruscotto, governo: gov, aula, anteprime: ante, stato: null };
  const comanda = (comando: unknown, campione: unknown = governo) =>
    rispondi("comanda", { comando }, { ...cc, campione });

  it("costo_dirotta: il numero del campione, sotto atti o regioni", () => {
    const d = gov.costo_dirotta_03.sanita;
    expect(rispondi("costo_dirotta", { dicastero: "sanita", atto: "sanita_case_comunita", regione: null }, cc)).toBe(d.atti.sanita_case_comunita);
    expect(() => rispondi("costo_dirotta", { dicastero: "sanita", atto: null, regione: "abruzzo" }, cc)).toThrow(rifiuta(d.regioni.abruzzo.rifiuto));
    expect(() => rispondi("costo_dirotta", { dicastero: "sanita", atto: null, regione: null }, cc)).toThrow(rifiuta("non nel campione: costo_dirotta"));
  });
  it("menu_elettorale: vuoto dà menu_03, la proporzionale il suo, il resto è rifiuto", () => {
    expect(rispondi("menu_elettorale", { valori: [] }, cc)).toHaveLength(aula.menu_03.length);
    const prop = aula.piena.vista.calendario[1].valori;
    expect(rispondi("menu_elettorale", { valori: prop }, cc)).toEqual(aula.menu_proporzionale);
    expect(() => rispondi("menu_elettorale", { valori: [{ disposizione: "x", gradino: "y" }] }, cc)).toThrow(rifiuta("non nel campione: menu_elettorale"));
  });
  it("anteprima_proposta: gli stessi valori danno l'anteprima, altri un rifiuto", () => {
    expect(rispondi("anteprima_proposta", { valori: aula.anteprima_proposta.valori }, cc)).toEqual(aula.anteprima_proposta.ok);
    expect(() => rispondi("anteprima_proposta", { valori: [] }, cc)).toThrow(rifiuta("non nel campione: anteprima_proposta"));
  });
  it("anteprima: la voce con gli stessi valori, o il suo rifiuto", () => {
    expect(rispondi("anteprima", { valori: ante.anteprime.riforma.valori }, cc)).toEqual(ante.anteprime.riforma.anteprima);
    const r = ante.anteprime.territoriale_decontribuzione;
    expect(() => rispondi("anteprima", { valori: r.valori }, cc)).toThrow(rifiuta(r.rifiuto));
    expect(rispondi("anteprima", { valori: ante.gradini[0].valori }, cc)).toEqual(ante.gradini[0].anteprima);
  });
  it("anteprima_accogli: infrastrutture dal 17", () => {
    expect(rispondi("anteprima_accogli", { dicastero: "infrastrutture_trasporti" }, cc)).toEqual(cruscotto.anteprima_accogli_03);
    expect(() => rispondi("anteprima_accogli", { dicastero: "sanita" }, cc)).toThrow(rifiuta("non nel campione: anteprima_accogli"));
  });
  it("anteprima_decreto: disposizioni con gli stessi valori", () => {
    expect(rispondi("anteprima_decreto", { decreto: { tipo: "disposizioni", valori: ante.decreto.valori } }, cc)).toEqual(ante.decreto.anteprima);
    expect(() => rispondi("anteprima_decreto", { decreto: { tipo: "fondi", da: "a", a: "b", importo: 1 } }, cc)).toThrow(rifiuta("non nel campione: anteprima_decreto"));
  });
  it("ferma su Lavoro: l'agenda di Lavoro fermata", () => {
    const v: any = comanda({ tipo: "ferma", dicastero: "lavoro_welfare" });
    expect(v).toEqual(gov.scelte.vista);
    const lavoro = v.ministeri.find((m: any) => m.dicastero === "lavoro_welfare");
    expect(lavoro.agenda.stato).toBe("fermata");
  });
  it("dirotta e chiedi_dimissioni giocati: la Vista delle scelte; non giocati: rifiuto", () => {
    expect(comanda({ tipo: "dirotta", dicastero: "sanita", atto: "sanita_case_comunita", regione: null })).toEqual(gov.scelte.vista);
    expect(comanda({ tipo: "dirotta", dicastero: "sviluppo_economico", atto: null, regione: "calabria" })).toEqual(gov.scelte.vista);
    expect(comanda({ tipo: "chiedi_dimissioni", dicastero: "ambiente_energia" })).toEqual(gov.scelte.vista);
    expect(() => comanda({ tipo: "ferma", dicastero: "sanita" })).toThrow(rifiuta("non nel campione: comanda"));
  });
  it("scegli_successore: la Vista del campione", () => {
    expect(comanda({ tipo: "scegli_successore", dicastero: "ambiente_energia", indice: 0 })).toEqual(gov.successore_scelto.vista);
  });
  it("dimettiti: dimissioni vere", () => {
    expect((comanda({ tipo: "dimettiti" }) as any).dimissioni).toBe(true);
  });
  it("rinvia su Ambiente e rispondi_evento debito: scelte_fatte", () => {
    expect(comanda({ tipo: "rinvia", dicastero: "ambiente_energia" })).toEqual(cruscotto.scelte_fatte.vista);
    expect(comanda({ tipo: "rispondi_evento", evento: 0, opzione: "debito" })).toEqual(cruscotto.scelte_fatte.vista);
    expect(() => comanda({ tipo: "rinvia", dicastero: "sanita" })).toThrow(rifiuta("non nel campione: comanda"));
  });
  it("presenta_legge: una delle tre misure dà la piena, la seconda costituzionale il rifiuto", () => {
    const cal = aula.piena.vista.calendario;
    expect(comanda({ tipo: "presenta_legge", valori: cal[3].valori })).toEqual(aula.piena.vista);
    expect(comanda({ tipo: "presenta_legge", valori: cal[2].valori })).toEqual(aula.piena.vista);
    expect(() => comanda({ tipo: "presenta_legge", valori: cal[2].valori }, aula.piena)).toThrow(rifiuta(aula.rifiuto_calendario_costituzionale));
    expect(aula.rifiuto_calendario_costituzionale).toContain("calendario pieno");
    expect(() => comanda({ tipo: "presenta_legge", valori: [] })).toThrow(rifiuta("non nel campione: comanda"));
  });
  it("presenta_proposta e linea: la Vista dell'opposizione", () => {
    expect(comanda({ tipo: "presenta_proposta", valori: [] })).toEqual(aula.opposizione_proposta.vista);
    expect(comanda({ tipo: "linea", misura: 1, si: false })).toEqual(aula.opposizione_proposta.vista);
  });
  it("compra il test della misura: l'esito della voce con gli stessi valori, il secondo è rifiuto", () => {
    const valori = ante.anteprime.riforma.valori;
    const v: any = comanda({ tipo: "compra", acquisto: { test_misura: { valori } } });
    expect(v.acquisti.test_misura).toEqual(ante.test.riforma.acquisti.test_misura);
    expect(() => comanda({ tipo: "compra", acquisto: { test_misura: { valori } } }, v)).toThrow(rifiuta(ante.test.riforma.rifiuto_ripetuto));
    expect(() => comanda({ tipo: "compra", acquisto: { test_misura: { valori: [] } } })).toThrow(rifiuta("non nel campione: comanda"));
  });
});

describe("il 21 dei Conti", () => {
  const conti: any = contiJson;
  const s = conti.studi_03;
  const cc = { ...c, campione: s, conti, stato: "studi_03" };
  it("lo stato si serve come prima: senza centro studi", () => {
    expect((vistaDi(s) as any).acquisti.centro_studi).toBeNull();
  });
  it("compra centro_studi dà la vista col centro studi; il secondo compra è rifiutato", () => {
    const v: any = rispondi("comanda", { comando: { tipo: "compra", acquisto: "centro_studi" } }, cc);
    expect(v.acquisti.centro_studi).not.toBeNull();
    expect(() => rispondi("comanda", { comando: { tipo: "compra", acquisto: "centro_studi" } }, { ...cc, campione: v })).toThrow(
      expect.objectContaining({ specie: "rifiuto", testo: "comando non ammesso: già comprato questo turno: centro studi" }),
    );
  });
  it("serie dà quella dello stato", () => {
    expect(rispondi("serie", {}, cc)).toEqual(s.serie);
  });
  it("installaFinto: il secondo compra è rifiutato senza passare la Vista", async () => {
    vi.stubGlobal("window", globalThis);
    const { installaFinto } = await import("./finto");
    await installaFinto(new URL("http://localhost/?vista=21-conti.json&stato=studi_03"));
    const compra = () => invoke<any>("comanda", { comando: { tipo: "compra", acquisto: "centro_studi" } });
    expect((await compra()).acquisti.centro_studi).not.toBeNull();
    await expect(compra()).rejects.toMatchObject({ testo: "comando non ammesso: già comprato questo turno: centro studi" });
    vi.unstubAllGlobals();
  });
});
