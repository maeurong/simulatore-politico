import { describe, expect, it } from "vitest";
import motti from "./pungolo.json";
import { mottoDi, titoloFine } from "./pungolo";
import v30 from "../../../../.scratch/v1-piano-11-ui/viste/30-eredita.json";
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

describe("titoli della fine", () => {
  const titoli = motti.titoli_fine;

  it("sedici titoli", () => {
    expect(titoli).toHaveLength(16);
  });

  it("nessuna cifra e nessun nome di partito dei dati", () => {
    const nomi = v30.vista.partiti.map((p) => p.nome.toLowerCase());
    for (const x of titoli) {
      expect(x).not.toMatch(/\d/);
      for (const n of nomi) expect(x.toLowerCase()).not.toContain(n);
    }
  });

  it("titoloFine è deterministico, sta fra i sedici e dipende dalla partita", () => {
    const x = titoloFine("Concordia", "Italia di oggi", { anno: 2026, trimestre: 4 });
    expect(titoli).toContain(x);
    expect(titoloFine("Concordia", "Italia di oggi", { anno: 2026, trimestre: 4 })).toBe(x);
    const visti = new Set(Array.from({ length: 40 }, (_, i) => titoloFine(`P${i}`, "Italia di oggi", { anno: 2026, trimestre: 4 })));
    expect(visti.size).toBeGreaterThan(4);
  });
});
