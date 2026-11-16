import { describe, expect, it } from "vitest";
import motti from "./pungolo.json";
import { mottoDi } from "./pungolo";
import v03 from "../../../../.scratch/v1-piano-11-ui/viste/03-governo-ordinario.json";

const ID = v03.dicasteri.map((d) => d.id);

describe("pungolo", () => {
  it("quaranta motti, quattro per ciascuno dei dieci dicasteri, nell'ordine della Vista", () => {
    expect(Object.keys(motti.motti)).toEqual(ID);
    expect(ID).toHaveLength(10);
    for (const id of ID) expect((motti.motti as Record<string, string[]>)[id]).toHaveLength(4);
  });

  it("nessun motto contiene una cifra", () => {
    for (const m of Object.values(motti.motti).flat()) expect(m).not.toMatch(/\d/);
  });

  it("mottoDi è deterministico e sta fra i motti del dicastero", () => {
    const m = mottoDi("Simone Gallo", "economia_finanze");
    expect(motti.motti.economia_finanze).toContain(m);
    expect(mottoDi("Simone Gallo", "economia_finanze")).toBe(m);
  });

  it("un dicastero senza motti dà null", () => {
    expect(mottoDi("Simone Gallo", "nessuno")).toBeNull();
  });
});
