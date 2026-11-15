import { describe, expect, it } from "vitest";
import { coalizioneDi, coloreDi, iniziali, nomeDi, quotaIncrocio, quotaVotanti } from "./partiti";
import { percentuale } from "$lib/testi/t";
import type { Vista } from "$lib/tipi/Vista";
import zero from "../../../.scratch/v1-piano-11-ui/viste/01-turno-zero.json";
import incarico from "../../../.scratch/v1-piano-11-ui/viste/02-trattativa-incarico.json";
import campagna from "../../../.scratch/v1-piano-11-ui/viste/26-campagna.json";
import governo from "../../../.scratch/v1-piano-11-ui/viste/03-governo-ordinario.json";
import cruscotto from "../../../.scratch/v1-piano-11-ui/viste/17-cruscotto.json";

const v = (x: unknown) => x as Vista;
const dieci = v((cruscotto as unknown as Record<string, { vista: unknown }>)["crisi_provvisorio"].vista);

describe("quotaVotanti", () => {
  it("il giocatore al turno zero: 19,2%, come il tracciato", () => {
    const j = v(zero).partiti.findIndex((p) => p.id === v(zero).giocatore);
    expect(percentuale(quotaVotanti(v(zero).sondaggio, j)!)).toBe("19,2%");
  });
  it("con dieci partiti: un numero per ognuno, null oltre l'ultimo e senza istituti", () => {
    dieci.partiti.forEach((_, j) => expect(quotaVotanti(dieci.sondaggio, j)).toEqual(expect.any(Number)));
    expect(quotaVotanti(dieci.sondaggio, dieci.partiti.length)).toBeNull();
    expect(quotaVotanti(dieci.sondaggio, -1)).toBeNull();
    expect(quotaVotanti({ ...dieci.sondaggio, istituti: [] }, 0)).toBeNull();
  });
});

describe("quotaIncrocio", () => {
  it("quote[j] / (1 − astensione); null fuori riga", () => {
    const i = { id: "x", campione: 1, quote: [0.3, 0.3, 0.4], posizione: [], approvazione: 0, margine: 0 };
    expect(quotaIncrocio(i, 0)).toBeCloseTo(0.5);
    expect(quotaIncrocio(i, 2)).toBeNull();
  });
});

describe("coalizioneDi", () => {
  it("turno zero senza dichiarazione: il solo giocatore", () => expect(coalizioneDi(v(zero))).toEqual(["giocatore"]));
  it("dichiarata: giocatore, centrodestra, destra", () =>
    expect(coalizioneDi(v(campagna["dichiarata"]))).toEqual(["giocatore", "centrodestra", "destra"]));
  it("in trattativa: le richieste degli alleati", () =>
    expect(coalizioneDi(v(incarico))).toEqual(["giocatore", "centrodestra", "destra"]));
  it("al governo: giocatore e alleati, la coalizione del voto non conta", () =>
    expect(coalizioneDi(v({ ...governo, coalizione: ["sinistra"] }))).toEqual(["giocatore", "centrodestra", "destra"]));
  it("in trattativa: anche la coalizione del voto", () =>
    expect(coalizioneDi(v({ ...incarico, coalizione: ["sinistra"] }))).toEqual(["giocatore", "sinistra", "centrodestra", "destra"]));
  it("al turno zero: le richieste e gli alleati non contano", () =>
    expect(coalizioneDi(v({ ...zero, coalizione: ["destra"], alleati: [{ partito: "centro" }] }))).toEqual(["giocatore", "destra"]));
});

describe("colori e nomi", () => {
  it("un partito dello scenario ha il colore, un nato in partita no", () => {
    expect(coloreDi("destra")).not.toBeNull();
    expect(coloreDi("nuovo_8")).toBeNull();
  });
  it("nomeDi: il nome del partito, il giocatore compreso", () => {
    expect(nomeDi(v(zero), "giocatore")).toBe("Partito del giocatore");
    expect(nomeDi(v(zero), "sconosciuto")).toBe("sconosciuto");
  });
});

describe("iniziali", () => {
  it("le iniziali del nome", () => expect(iniziali("Luca Benedetti")).toBe("LB"));
  it("nome vuoto: niente", () => expect(iniziali("")).toBe(""));
});
