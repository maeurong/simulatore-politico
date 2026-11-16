import { beforeEach, describe, expect, it, vi } from "vitest";
import type { Vista } from "$lib/tipi/Vista";

const { guscio, goto } = vi.hoisted(() => ({
  guscio: { chiudiTurno: vi.fn(), riprendi: vi.fn(), fondaPartita: vi.fn(), comanda: vi.fn(), serie: vi.fn() },
  goto: vi.fn(),
}));
vi.mock("$app/navigation", () => ({ goto }));
vi.mock("$lib/guscio/chiamate", async (originale) => ({ ...(await originale<object>()), guscio }));

import { partita } from "./partita.svelte";
import { acquisto } from "./acquisto.svelte";

const prima = { turno: 3, finita: false, fase: "ordinaria", crisi: false, governa: true } as Vista;
const dopo = { ...prima, turno: 4 } as Vista;
const rifiuto = { specie: "rifiuto", testo: "comando non ammesso: già comprato" };

beforeEach(() => {
  vi.clearAllMocks();
  partita.vista = prima;
  partita.guasto = null;
  partita.occupato = false;
  guscio.serie.mockResolvedValue([]);
});

describe("acquisto", () => {
  it("compra manda il comando compra/sondaggio_privato", async () => {
    guscio.comanda.mockResolvedValue(dopo);
    await acquisto("sondaggio_privato").compra();
    expect(guscio.comanda).toHaveBeenCalledWith({ tipo: "compra", acquisto: "sondaggio_privato" });
    expect(partita.vista).toBe(dopo);
  });

  it("il rifiuto del core si legge senza il prefisso", async () => {
    guscio.comanda.mockRejectedValue(rifiuto);
    const a = acquisto("sondaggio_privato");
    await a.compra();
    expect(a.rifiuto).toBe("già comprato");
  });

  it("il rifiuto vale per la Vista in cui è nato: con una Vista nuova scompare", async () => {
    guscio.comanda.mockRejectedValue(rifiuto);
    const a = acquisto("sondaggio_privato");
    await a.compra();
    partita.vista = dopo;
    expect(a.rifiuto).toBeNull();
  });

  it("occupato: il clic non fa niente, nemmeno cancella il rifiuto che si sta leggendo", async () => {
    guscio.comanda.mockRejectedValueOnce(rifiuto);
    const a = acquisto("sondaggio_privato");
    await a.compra();
    partita.occupato = true;
    await a.compra();
    expect(guscio.comanda).toHaveBeenCalledTimes(1);
    expect(a.rifiuto).toBe("già comprato");
  });

  it("un nuovo compra azzera il rifiuto di prima", async () => {
    guscio.comanda.mockRejectedValueOnce(rifiuto);
    const a = acquisto("sondaggio_privato");
    await a.compra();
    expect(a.rifiuto).toBe("già comprato");
    guscio.comanda.mockResolvedValueOnce(dopo);
    await a.compra();
    expect(a.rifiuto).toBeNull();
  });

  it("centro_studi: manda il comando compra/centro_studi; due compra di fila ne mandano uno solo", async () => {
    guscio.comanda.mockResolvedValue(dopo);
    const a = acquisto("centro_studi");
    await Promise.all([a.compra(), a.compra()]);
    expect(guscio.comanda).toHaveBeenCalledTimes(1);
    expect(guscio.comanda).toHaveBeenCalledWith({ tipo: "compra", acquisto: "centro_studi" });
  });
});
