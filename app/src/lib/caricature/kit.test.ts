import { describe, expect, it } from "vitest";
import { VARIANTI, type Leve } from "./caricatura";
import { COLLETTI, profilo } from "./kit";
import fonte from "./kit.ts?raw";

const chiavi = Object.keys(VARIANTI) as (keyof Leve)[];

describe("kit", () => {
  it("ha un colletto per leva e il file sta sotto i 60 KB", () => {
    expect(COLLETTI).toHaveLength(VARIANTI.colletto);
    expect(fonte.length).toBeLessThan(60 * 1024);
  });

  it("ogni combinazione di naso, mento, fronte e nuca dà una sagoma chiusa e un pennino non vuoto", () => {
    const viste = new Set<string>();
    for (let naso = 0; naso < VARIANTI.naso; naso++)
      for (let mento = 0; mento < VARIANTI.mento; mento++)
        for (let fronte = 0; fronte < VARIANTI.fronte; fronte++)
          for (let nuca = 0; nuca < VARIANTI.nuca; nuca++) {
            const p = profilo({ naso, mento, fronte, nuca, occhio: 0, colletto: 0 });
            expect(p.sagoma).toMatch(/^M20 44C.*Z$/);
            expect(p.pennino).toMatch(/^M\d+ \d+l/);
            expect(p.pennino).not.toMatch(/NaN|undefined/);
            viste.add(p.sagoma);
          }
    expect(viste.size).toBe(chiavi.slice(0, 4).reduce((n, k) => n * VARIANTI[k], 1));
  });

  it("naso e mento cambiano il filo e il pennino", () => {
    const base = { naso: 0, mento: 0, fronte: 0, nuca: 0, occhio: 0, colletto: 0 };
    expect(profilo({ ...base, naso: 1 }).pennino).not.toBe(profilo(base).pennino);
    expect(profilo({ ...base, mento: 2 }).filo).not.toBe(profilo(base).filo);
  });
});
