import { describe, expect, it } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import type { VoceMenu } from "$lib/tipi/VoceMenu";
import aula from "../../../../.scratch/v1-piano-11-ui/viste/19-aula.json";
import editor from "../../../../.scratch/v1-piano-11-ui/viste/14-editor.json";
import ordinario from "../../../../.scratch/v1-piano-11-ui/viste/03-governo-ordinario.json";
import { bozzaDiLegge, conLegge, eLegge, spenti } from "./elettorale";

const stati = aula as unknown as Record<string, { vista: Vista }> & { menu_03: VoceMenu[] };
const v03 = ((ordinario as unknown as { vista?: Vista }).vista ?? ordinario) as unknown as Vista;
const leggi = (editor as unknown as { leggi_elettorali: Record<string, Record<string, string>> }).leggi_elettorali;

describe("bozzaDiLegge", () => {
  it("la legge in vigore sulla 03 non cambia niente", () => {
    expect(bozzaDiLegge(v03, leggi.legge_180)).toEqual([]);
  });
  it("la proporzionale al 3% porta i sette valori della misura 1, nell'ordine delle disposizioni", () => {
    expect(bozzaDiLegge(v03, leggi.proporzionale_3)).toEqual(stati.piena.vista.calendario[1].valori);
  });
});

describe("conLegge, eLegge", () => {
  it("sostituisce solo la parte elettorale della bozza", () => {
    const salario = { disposizione: "salario_minimo", gradino: "11" };
    const b = conLegge(v03, [salario, { disposizione: "tetto", gradino: "nessuno" }], leggi.proporzionale_3);
    expect(b).toEqual([salario, ...bozzaDiLegge(v03, leggi.proporzionale_3)]);
  });
  it("una legge è attiva se bozza e vigore la compongono tutta", () => {
    expect(eLegge(v03, [], leggi.legge_180)).toBe(true);
    expect(eLegge(v03, [], leggi.proporzionale_3)).toBe(false);
    expect(eLegge(v03, bozzaDiLegge(v03, leggi.proporzionale_3), leggi.proporzionale_3)).toBe(true);
  });
});

describe("spenti", () => {
  it("ogni voce non ammessa ha il suo motivo, nessuna ammessa compare", () => {
    const menu = stati.menu_03;
    const s = spenti(menu);
    const no = menu.filter((m) => !m.ammesso);
    expect(no.length).toBeGreaterThan(0);
    expect(Object.keys(s)).toHaveLength(no.length);
    for (const m of no) expect(s[`${m.disposizione}|${m.gradino}`]).toBe(m.motivo);
    for (const m of menu.filter((x) => x.ammesso)) expect(s).not.toHaveProperty(`${m.disposizione}|${m.gradino}`);
  });
  it("un motivo mancante non lascia il gradino aperto", () => {
    expect(spenti([{ disposizione: "a", gradino: "1", ammesso: false, motivo: null }])).toEqual({ "a|1": "non ammesso" });
  });
});
