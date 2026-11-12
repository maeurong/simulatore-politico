import { describe, expect, it } from "vitest";
import { numero } from "$lib/testi/t";
import { freccia } from "./NumeroFreccia.svelte";

const uno = (x: number) => numero(x, 1);

describe("freccia", () => {
  it("senza turno prima non c'è variazione", () => {
    expect(freccia(56.5, undefined, uno, "scende")).toBeNull();
  });
  it("il verso viene dal segno, il giudizio da cosa è male", () => {
    expect(freccia(56.5, 50.9, uno, "scende")).toEqual({ verso: "sale", giudizio: "bene", testo: "5,6" });
    expect(freccia(51, 53.7, uno, "scende")).toEqual({ verso: "scende", giudizio: "male", testo: "2,7" });
    expect(freccia(86, 83, numero, "sale")).toEqual({ verso: "sale", giudizio: "male", testo: "3" });
  });
  it("male null: freccia senza colore", () => {
    expect(freccia(24, 20, numero, null)).toEqual({ verso: "sale", giudizio: null, testo: "4" });
  });
  it("formatoVariazione scrive la variazione, formato resta al valore", () => {
    expect(freccia(51, 53.7, numero, "scende", uno)).toEqual({ verso: "scende", giudizio: "male", testo: "2,7" });
    expect(freccia(51, 53.7, numero, "scende")).toEqual({ verso: "scende", giudizio: "male", testo: "3" });
    expect(freccia(60.04, 60, numero, "scende", (x) => numero(x, 2))).toEqual({ verso: "sale", giudizio: "bene", testo: "0,04" });
  });
  it("ciò che si arrotonda a zero è pari", () => {
    expect(freccia(60.04, 60, uno, "scende")).toEqual({ verso: "pari", giudizio: null, testo: "" });
  });
});
