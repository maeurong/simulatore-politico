import { beforeEach, describe, expect, it, vi } from "vitest";
import type { Vista } from "$lib/tipi/Vista";

const { guscio, goto } = vi.hoisted(() => ({
  guscio: { chiudiTurno: vi.fn(), riprendi: vi.fn(), serie: vi.fn() },
  goto: vi.fn(),
}));
vi.mock("$app/navigation", () => ({ goto }));
vi.mock("$lib/guscio/chiamate", async (originale) => ({ ...(await originale<object>()), guscio }));

import { partita } from "./partita.svelte";

const prima = { turno: 3, finita: false, fase: "ordinaria", crisi: false, governa: true } as Vista;
const dopo = { ...prima, turno: 4, fase: "trattativa" } as Vista;

beforeEach(() => {
  vi.clearAllMocks();
  partita.vista = prima;
  partita.guasto = null;
  guscio.serie.mockResolvedValue([]);
});

describe("partita", () => {
  it("chiudiTurno: Vista e serie nuove, poi la casa del turno nuovo", async () => {
    guscio.chiudiTurno.mockResolvedValue(dopo);
    await partita.chiudiTurno();
    expect(partita.vista).toBe(dopo);
    expect(guscio.serie).toHaveBeenCalled();
    expect(goto).toHaveBeenCalledWith("/partita/trattativa");
    expect(partita.occupato).toBe(false);
  });

  it("un guasto lascia la Vista com'era e si scrive", async () => {
    guscio.chiudiTurno.mockRejectedValue({ specie: "guasto", testo: "disco pieno" });
    await partita.chiudiTurno();
    expect(partita.vista).toBe(prima);
    expect(partita.guasto).toBe("disco pieno");
    expect(goto).not.toHaveBeenCalled();
    expect(partita.occupato).toBe(false);
  });

  it("un rifiuto torna a chi ha chiamato", async () => {
    guscio.riprendi.mockRejectedValue({ specie: "rifiuto", testo: "no" });
    await expect(partita.riprendi("x")).rejects.toEqual({ specie: "rifiuto", testo: "no" });
    expect(partita.vista).toBe(prima);
    expect(partita.guasto).toBeNull();
  });
});
