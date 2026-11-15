import { describe, expect, it } from "vitest";
import { VARIANTI, type Leve } from "./caricatura";
import { render } from "svelte/server";
import Caricatura from "./Caricatura.svelte";
import { FANTASIE, OGGETTI, profilo } from "./kit";
import fonte from "./kit.ts?raw";

const chiavi = Object.keys(VARIANTI) as (keyof Leve)[];

describe("kit", () => {
  it("ha una fantasia per leva colletto e il file sta sotto i 60 KB", () => {
    expect(FANTASIE).toHaveLength(6);
    expect(FANTASIE).toHaveLength(VARIANTI.colletto);
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

  it("ogni coordinata della sagoma sta nella tela 64 × 64 e ogni pennino si chiude, dentro la tela in decimi", () => {
    for (let naso = 0; naso < VARIANTI.naso; naso++)
      for (let mento = 0; mento < VARIANTI.mento; mento++)
        for (let fronte = 0; fronte < VARIANTI.fronte; fronte++)
          for (let nuca = 0; nuca < VARIANTI.nuca; nuca++) {
            const p = profilo({ naso, mento, fronte, nuca, occhio: 0, colletto: 0 });
            // sagoma: solo comandi assoluti, ogni numero è una coordinata della tela
            const nums = p.sagoma.match(/-?\d+(\.\d+)?/g)!.map(Number);
            expect(nums.every((n) => n >= 0 && n <= 64)).toBe(true);
            // pennino: sottotracciati «M x y l dx dy … z»; il punto corrente, sommando i relativi, resta nella tela in decimi
            const sotto = p.pennino.split(/(?=M)/);
            for (const s of sotto) {
              expect(s).toMatch(/^M\d+ \d+l[-\d ]+z$/);
              const [x0, y0, ...rel] = (s.match(/-?\d+/g) ?? []).map(Number);
              let x = x0, y = y0;
              for (let i = 0; i < rel.length; i += 2) {
                x += rel[i];
                y += rel[i + 1];
                // y fino a 650: le spalle sbordano di 3 decimi sotto la tela (643 misurato), tagliate dal bordo dell'svg
                expect(x >= 0 && x <= 640 && y >= 0 && y <= 650).toBe(true);
              }
            }
          }
  });

  it("naso e mento cambiano il filo e il pennino", () => {
    const base = { naso: 0, mento: 0, fronte: 0, nuca: 0, occhio: 0, colletto: 0 };
    expect(profilo({ ...base, naso: 1 }).pennino).not.toBe(profilo(base).pennino);
    expect(profilo({ ...base, mento: 2 }).filo).not.toBe(profilo(base).filo);
  });

  it("il colletto riempie con la fantasia della leva, quello del tecnico col grigio", () => {
    const base = { naso: 0, mento: 0, fronte: 0, nuca: 0, occhio: 0 };
    const colletto = (ruolo: "ministro" | "tecnico", c: number) =>
      render(Caricatura, { props: { ruolo, partito: "Centro", nome: "", leve: { ...base, colletto: c }, misura: 62 } }).body.match(/<path class="coll[^"]*"[^>]*>/)![0];
    FANTASIE.forEach((f, c) => expect(colletto("ministro", c)).toContain(`url(#fantasia-${f})`));
    expect(colletto("tecnico", 2)).toContain("grigio");
    expect(colletto("tecnico", 2)).not.toContain("url(#fantasia");
  });

  it("l'oggetto del dicastero c'è solo con un id noto, e mai sul leader", () => {
    const corpo = (props: Record<string, unknown>) => render(Caricatura, { props: { ruolo: "ministro", partito: "Centro", nome: "", misura: 62, ...props } as never }).body;
    expect(corpo({ dicastero: "sanita" })).toContain('class="ogg"');
    expect(corpo({})).not.toContain("ogg");
    expect(corpo({ dicastero: "inesistente" })).not.toContain("ogg");
    expect(corpo({ dicastero: "toString" })).not.toContain("ogg");
    expect(corpo({ dicastero: "sanita", ruolo: "leader" })).not.toContain("ogg");
    expect(corpo({ dicastero: "sanita", fondo: "rosso" })).toMatch(/class="car[^"]* rosso/);
  });

  it("ogni dicastero ha il suo oggetto", () => {
    expect(Object.keys(OGGETTI)).toHaveLength(10);
  });
});
