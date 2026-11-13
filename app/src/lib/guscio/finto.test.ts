import { describe, expect, it } from "vitest";
import { rispondi, trovaFile, vistaDi } from "./finto";
import governo from "../../../../.scratch/v1-piano-11-ui/viste/03-governo-ordinario.json";
import turnoZero from "../../../../.scratch/v1-piano-11-ui/viste/01-turno-zero.json";
import edicola from "../../../../.scratch/v1-piano-11-ui/viste/13-edicola.json";
import editorJson from "../../../../.scratch/v1-piano-11-ui/viste/14-editor.json";
import fondazione from "../../../../.scratch/v1-piano-11-ui/viste/15-fondazione.json";
import cruscottoJson from "../../../../.scratch/v1-piano-11-ui/viste/17-cruscotto.json";
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
