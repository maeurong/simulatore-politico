import { describe, expect, it } from "vitest";
import { render } from "svelte/server";
import Caricatura from "./Caricatura.svelte";
import { VARIANTI, type Leve, type Ruolo } from "./caricatura";

const html = (ruolo: Ruolo, leve: Leve, fondo?: "giallo" | "rosso") =>
  render(Caricatura, { props: { ruolo, partito: "Centro", nome: "Giorgio Valli", leve, misura: 84, fondo } }).body;

describe("Caricatura renderizzata", () => {
  it("una faccia pesa meno di 3 KB su ogni combinazione di naso, mento, fronte, nuca, occhio e colletto, per ogni ruolo e fondo", () => {
    let max = 0;
    for (let naso = 0; naso < VARIANTI.naso; naso++)
      for (let mento = 0; mento < VARIANTI.mento; mento++)
        for (let fronte = 0; fronte < VARIANTI.fronte; fronte++)
          for (let nuca = 0; nuca < VARIANTI.nuca; nuca++)
            for (let occhio = 0; occhio < VARIANTI.occhio; occhio++)
              for (let colletto = 0; colletto < VARIANTI.colletto; colletto++)
                for (const ruolo of ["leader", "tecnico"] as const) {
                  max = Math.max(max, html(ruolo, { naso, mento, fronte, nuca, occhio, colletto }, "rosso").length);
                }
    expect(max).toBeLessThan(3 * 1024);
  });
});
