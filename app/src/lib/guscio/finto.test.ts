import { describe, expect, it } from "vitest";
import { rispondi, vistaDi } from "./finto";
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
