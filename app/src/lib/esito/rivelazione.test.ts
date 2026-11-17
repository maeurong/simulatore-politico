import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
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
    // la Vista di prima resta: i tracciati scattano comunque, e «era X» si legge finché non si esce dalla pagina
    expect(rivelazione.prima).toBe(prima);
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

describe("ritmo", () => {
  beforeEach(() => vi.useFakeTimers());
  afterEach(() => vi.useRealTimers());

  it("avanza una voce ogni 1100 ms × k e non supera fine", () => {
    rivelazione.inizia(prima, [], 2);
    const stop = rivelazione.ritmo(0.5);
    vi.advanceTimersByTime(549);
    expect(rivelazione.passo).toBe(0);
    vi.advanceTimersByTime(1);
    expect(rivelazione.passo).toBe(1);
    vi.advanceTimersByTime(5000);
    expect(rivelazione.passo).toBe(2);
    expect(rivelazione.ferma).toBe(true);
    stop();
  });

  it("il cleanup ferma il timer", () => {
    rivelazione.inizia(prima, [], 5);
    const stop = rivelazione.ritmo(1);
    vi.advanceTimersByTime(1100);
    stop();
    vi.advanceTimersByTime(10000);
    expect(rivelazione.passo).toBe(1);
  });

  it("uscire a metà (cleanup del ritmo, poi quello della pagina) lascia tutto al valore nuovo e ferma", () => {
    rivelazione.inizia(prima, [{ turno: 3 } as never], 5);
    const pulizie = [rivelazione.ritmo(1), () => rivelazione.chiudi()];
    vi.advanceTimersByTime(3300);
    expect(rivelazione.passo).toBe(3);
    pulizie.forEach((f) => f());
    vi.advanceTimersByTime(10000);
    expect(rivelazione.prima).toBeNull();
    expect(rivelazione.ferma).toBe(true);
    expect(rivelazione.passo).toBe(5);
  });
});
