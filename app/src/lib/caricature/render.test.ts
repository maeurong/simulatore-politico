import { describe, expect, it } from "vitest";
import { render } from "svelte/server";
import Caricatura from "./Caricatura.svelte";
import { OGGETTI } from "./kit";
import { VARIANTI, leveDi, type Leve, type Ruolo } from "./caricatura";

const html = (ruolo: Ruolo, leve: Leve, fondo?: "giallo" | "rosso", dicastero?: string, genere?: "donna" | "uomo") =>
  render(Caricatura, { props: { ruolo, partito: "Centro", nome: "Giorgio Valli", leve, misura: 84, fondo, dicastero, genere } }).body;

const pesante = Object.keys(OGGETTI).sort((a, b) => OGGETTI[b].length - OGGETTI[a].length)[0];

describe("Caricatura renderizzata", () => {
  // i tratti della donna (ciglia, labbra, orecchino, cappello) pesano circa 1 KB: tetto proprio. Quello dell'uomo passa da 3 a 3,2 KB:
  // i tre blocchi `{#if donna}` lasciano nell'HTML del server i loro commenti (~55 byte), il disegno è lo stesso.
  it.each([["uomo", 3.2], ["donna", 4]] as const)("una faccia %s pesa meno di %f KB su ogni combinazione di leve, per ogni ruolo e fondo", (genere, kb) => {
    let max = 0;
    for (let naso = 0; naso < VARIANTI.naso; naso++)
      for (let mento = 0; mento < VARIANTI.mento; mento++)
        for (let fronte = 0; fronte < VARIANTI.fronte; fronte++)
          for (let nuca = 0; nuca < VARIANTI.nuca; nuca++)
            for (let occhio = 0; occhio < VARIANTI.occhio; occhio++)
              for (let colletto = 0; colletto < VARIANTI.colletto; colletto++)
                for (const ruolo of ["leader", "tecnico", "ministro"] as const) {
                  max = Math.max(max, html(ruolo, { naso, mento, fronte, nuca, occhio, colletto, orecchino: true, cappello: true }, "rosso", pesante, genere).length);
                }
    expect(max).toBeLessThan(kb * 1024);
  });

  const base = leveDi("ministro", "Centro", "Giorgio Valli");
  it("la donna ha labbra rosse e il naso scalato; uomo e assente restano com'erano", () => {
    const d = html("ministro", base, undefined, undefined, "donna");
    expect(d).toContain('class="rosso lc"');
    expect(d).toContain("scale(.085)");
    const u = html("ministro", base, undefined, undefined, "uomo");
    expect(u).not.toContain('class="rosso lc"');
    expect(u).not.toContain("scale(.085)");
    expect(html("ministro", base)).toBe(u);
  });

  it("orecchino e cappello vengono dai bit, solo nelle donne", () => {
    const con = html("ministro", { ...base, orecchino: true, cappello: true }, undefined, undefined, "donna");
    const senza = html("ministro", { ...base, orecchino: false, cappello: false }, undefined, undefined, "donna");
    expect(con).toContain('class="giallo ink-l"');
    expect(senza).not.toContain('class="giallo ink-l"');
    expect(con.length).toBeGreaterThan(senza.length + 300);
    expect(html("ministro", { ...base, orecchino: true, cappello: true }, undefined, undefined, "uomo")).toBe(html("ministro", { ...base, orecchino: false, cappello: false }, undefined, undefined, "uomo"));
  });
});
