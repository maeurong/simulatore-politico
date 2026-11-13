import { describe, expect, it } from "vitest";
import { dataRelativa, elenco, hash, numero, percentuale, segnato, t, variante } from "./t";

const ORA = new Date((1794000000 + 10 * 3600) * 1000);

describe("numeri", () => {
  it("numero: punto delle migliaia anche a quattro cifre, meno vero", () => {
    expect(numero(1234)).toBe("1.234");
    expect(numero(-4.25, 1)).toBe("−4,3");
    expect(numero(0.5, 0)).toBe("1");
  });
  it("percentuale", () => {
    expect(percentuale(0.192387, 1)).toBe("19,2%");
  });
  it("segnato", () => {
    expect(segnato(14)).toBe("+14");
    expect(segnato(-121)).toBe("−121");
    expect(segnato(0)).toBe("0");
  });
  it("segnato coi decimali", () => {
    expect(segnato(2.5, 1)).toBe("+2,5");
    expect(segnato(-2.5, 1)).toBe("−2,5");
    expect(segnato(0.04, 1)).toBe("0,0");
  });
  it("numero: un negativo che arrotonda a zero non porta il meno", () => {
    expect(numero(-0.4)).toBe("0");
  });
  it("percentuale: un negativo che arrotonda a zero non porta il meno", () => {
    expect(percentuale(-0.0004, 1)).toBe("0,0%");
  });
});

describe("dataRelativa", () => {
  it.each([
    [1794000000, "ieri alle 22:20"],
    [1793913600, "giovedì alle 22:20"],
    [1793823600, "mercoledì alle 21:20"],
    [1793568000, "domenica alle 22:20"],
    [1793222400, "il 28 ottobre alle 22:20"],
    [1794000000 + 9 * 3600, "oggi alle 07:20"],
  ])("%i", (sec, atteso) => {
    expect(dataRelativa(sec, ORA)).toBe(atteso);
  });
});

describe("t", () => {
  it("sostituisce i segnaposto", () => {
    expect(t("segnaposto.arriva", { sezione: "Aula", fetta: "11b" })).toBe("Aula: arriva con la fetta 11b.");
  });
  it("un segnaposto mancante resta com'è, senza lanciare", () => {
    expect(t("segnaposto.arriva", { sezione: "Aula" })).toBe("Aula: arriva con la fetta {fetta}.");
  });
  it("sceglie il plurale su n", () => {
    expect(t("cornice.turni_al_voto", { n: 1 })).toBe("1 turno al voto");
    expect(t("cornice.turni_al_voto", { n: 2 })).toBe("2 turni al voto");
  });
});

describe("hash e variante", () => {
  it("FNV-1a a 32 bit, intero non negativo", () => {
    expect(hash("")).toBe(0x811c9dc5);
    expect(hash("a")).toBe(0xe40c292c);
  });
  it("variante è stabile e sta nell'intervallo", () => {
    const v = variante(4, "a", 3);
    expect(variante(4, "a", 3)).toBe(v);
    expect(v).toBeGreaterThanOrEqual(0);
    expect(v).toBeLessThan(4);
  });
});

describe("elenco", () => {
  it("come elenco() del core: «A», «A e B», «A, B e C»; il separatore solo fra le prime", () => {
    expect(elenco([], ", ")).toBe("");
    expect(elenco(["A"], ", ")).toBe("A");
    expect(elenco(["A", "B"], ", ")).toBe("A e B");
    expect(elenco(["4,5", "5,5", "6,5"], "; ")).toBe("4,5; 5,5 e 6,5");
  });
});
