import { describe, expect, it } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import { PASSO_CESSIONE, cedi, dichiara, righeCoalizione, seggiAttesi } from "./campagna";
import turnoZeroJson from "../../../../.scratch/v1-piano-11-ui/viste/01-turno-zero.json";
import campagnaJson from "../../../../.scratch/v1-piano-11-ui/viste/26-campagna.json";

const turnoZero = turnoZeroJson as unknown as Vista;
const dichiarata = (campagnaJson as unknown as { dichiarata: Vista }).dichiarata;

describe("dichiara", () => {
  it("aggiunge il partito alla coalizione di oggi", () => {
    expect(dichiara(turnoZero, [], "centrodestra", true)).toEqual({ tipo: "dichiara_coalizione", partiti: ["centrodestra"], cessioni: [] });
  });
  it("toglie il partito e fa cadere le sue cessioni", () => {
    expect(dichiara(dichiarata, [{ partito: "destra", seggi: 10 }], "destra", false)).toEqual({
      tipo: "dichiara_coalizione", partiti: ["centrodestra"], cessioni: [],
    });
  });
  it("tiene le cessioni degli altri", () => {
    const c = dichiara(dichiarata, [{ partito: "destra", seggi: 10 }], "centrodestra", false);
    expect(c).toEqual({ tipo: "dichiara_coalizione", partiti: ["destra"], cessioni: [{ partito: "destra", seggi: 10 }] });
  });
});

describe("cedi", () => {
  it("aggiunge la cessione, e il partito che cede entra nei partiti", () => {
    expect(cedi(turnoZero, [], "centro", 10)).toEqual({ tipo: "dichiara_coalizione", partiti: ["centro"], cessioni: [{ partito: "centro", seggi: 10 }] });
  });
  it("cambia la cessione che c'è", () => {
    const c = cedi(dichiarata, [{ partito: "destra", seggi: 5 }], "destra", 10);
    expect(c).toMatchObject({ partiti: ["centrodestra", "destra"], cessioni: [{ partito: "destra", seggi: 10 }] });
  });
  it("0 la toglie, e non porta dentro chi non c'era", () => {
    expect(cedi(turnoZero, [{ partito: "centro", seggi: 10 }], "centro", 0)).toEqual({ tipo: "dichiara_coalizione", partiti: [], cessioni: [] });
  });
  it("il passo è cinque seggi", () => {
    expect(PASSO_CESSIONE).toBe(5);
  });
});

describe("righeCoalizione", () => {
  it("al turno zero: sette righe, Centrodestra e Destra disponibili, il Centro no con la causa del core", () => {
    const r = righeCoalizione(turnoZero);
    expect(r).toHaveLength(7);
    expect(r.filter((x) => x.disponibile).map((x) => x.id)).toEqual(["centrodestra", "destra"]);
    expect(r.find((x) => x.id === "centro")).toEqual({ id: "centro", nome: "Centro", dichiarato: false, disponibile: false, causa: "posizione troppo lontana dalla tua" });
    expect(r.every((x) => !x.dichiarato)).toBe(true);
  });
  it("coalizione dichiarata: due righe dichiarate", () => {
    expect(righeCoalizione(dichiarata).filter((x) => x.dichiarato).map((x) => x.id)).toEqual(["centrodestra", "destra"]);
  });
});

describe("seggiAttesi", () => {
  it("i seggi alla Camera del giocatore sul sondaggio pubblico", () => {
    expect(seggiAttesi(turnoZero)).toBe(80);
  });
  it("senza proiezione, null", () => {
    expect(seggiAttesi({ ...turnoZero, proiezione: null })).toBeNull();
  });
});
