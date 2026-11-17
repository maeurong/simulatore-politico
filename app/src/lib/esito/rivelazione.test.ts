import { beforeEach, describe, expect, it } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import { rivelazione } from "./rivelazione.svelte";

const prima = { turno: 3 } as Vista;

beforeEach(() => rivelazione.chiudi());

describe("rivelazione", () => {
  it("senza prima la pagina è ferma", () => {
    expect(rivelazione.prima).toBeNull();
    expect(rivelazione.ferma).toBe(true);
  });

  it("inizia poi avanza porta passo da 0 a 1", () => {
    rivelazione.inizia(prima, [], 3);
    expect(rivelazione.prima).toBe(prima);
    expect(rivelazione.ferma).toBe(false);
    expect(rivelazione.passo).toBe(0);
    rivelazione.avanza();
    expect(rivelazione.passo).toBe(1);
  });

  it("avanza si ferma sull'ultima voce", () => {
    rivelazione.inizia(prima, [], 2);
    rivelazione.avanza();
    expect(rivelazione.ferma).toBe(false);
    rivelazione.avanza();
    expect(rivelazione.ferma).toBe(true);
    rivelazione.avanza();
    expect(rivelazione.passo).toBe(2);
  });

  it("senza voci in evidenza nasce già ferma", () => {
    rivelazione.inizia(prima, [], 0);
    expect(rivelazione.ferma).toBe(true);
  });

  it("salta porta passo alla fine e ferma a vero", () => {
    rivelazione.inizia(prima, [], 5);
    rivelazione.avanza();
    rivelazione.salta();
    expect(rivelazione.passo).toBe(5);
    expect(rivelazione.ferma).toBe(true);
  });

  it("chiudi azzera prima e la serie, e ferma", () => {
    rivelazione.inizia(prima, [{ turno: 3 } as never], 5);
    rivelazione.chiudi();
    expect(rivelazione.prima).toBeNull();
    expect(rivelazione.serie_prima).toEqual([]);
    expect(rivelazione.ferma).toBe(true);
  });
});
