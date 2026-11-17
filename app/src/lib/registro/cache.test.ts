import { beforeEach, describe, expect, it, vi } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import type { VoceEsito } from "$lib/tipi/VoceEsito";

const { guscio } = vi.hoisted(() => ({ guscio: { registro: vi.fn() } }));
vi.mock("$lib/guscio/chiamate", async (originale) => ({ ...(await originale<object>()), guscio }));

import { registroCache } from "./cache.svelte";

const vista = (turno: number) => ({ turno }) as Vista;
const voci = (n: number) => [{ tipo: "sondaggio", criticita: "ordinaria", tono: "neutro", dettagli: [], disposizioni: [], fatto: `t${n}` }] as unknown as VoceEsito[];

beforeEach(() => {
  registroCache.svuota();
  guscio.registro.mockReset();
});

describe("registroCache", () => {
  it("chiede un turno alla volta, dal più vecchio, e li tiene", async () => {
    let inCorso = 0, massimo = 0;
    guscio.registro.mockImplementation(async (t: number) => {
      massimo = Math.max(massimo, ++inCorso);
      await Promise.resolve();
      inCorso--;
      return voci(t);
    });
    await registroCache.carica(vista(3));
    expect(guscio.registro.mock.calls.map((c) => c[0])).toEqual([0, 1, 2]);
    expect(massimo).toBe(1);
    expect(registroCache.turni.map((t) => t.turno)).toEqual([0, 1, 2]);
  });

  it("un turno che fallisce entra come guasto e gli altri restano leggibili", async () => {
    guscio.registro.mockImplementation(async (t: number) => {
      if (t === 1) throw { specie: "guasto", testo: "disco" };
      return voci(t);
    });
    await registroCache.carica(vista(3));
    expect(registroCache.turni[1]).toEqual({ turno: 1, guasto: "disco" });
    expect("voci" in registroCache.turni[2]).toBe(true);
  });

  it("una seconda carica chiede solo i turni nuovi e i guasti", async () => {
    guscio.registro.mockImplementation(async (t: number) => {
      if (t === 0 && guscio.registro.mock.calls.length === 1) throw { specie: "guasto", testo: "x" };
      return voci(t);
    });
    await registroCache.carica(vista(2));
    guscio.registro.mockClear();
    await registroCache.carica(vista(3));
    expect(guscio.registro.mock.calls.map((c) => c[0])).toEqual([0, 2]);
  });

  it("due aperture insieme condividono la corsa", async () => {
    guscio.registro.mockImplementation(async (t: number) => voci(t));
    await Promise.all([registroCache.carica(vista(2)), registroCache.carica(vista(2))]);
    expect(guscio.registro).toHaveBeenCalledTimes(2);
  });

  it("svuota durante il volo: la risposta in ritardo non entra nella cache nuova", async () => {
    let sblocca!: (v: VoceEsito[]) => void;
    guscio.registro.mockImplementationOnce(() => new Promise((r) => (sblocca = r)));
    const vecchia = registroCache.carica(vista(2));
    registroCache.svuota();
    guscio.registro.mockImplementation(async (t: number) => voci(t + 100));
    await registroCache.carica(vista(1));
    sblocca(voci(0));
    await vecchia;
    expect(registroCache.turni).toHaveLength(1);
    expect((registroCache.turni[0] as { voci: VoceEsito[] }).voci[0].fatto).toBe("t100");
    expect(guscio.registro).toHaveBeenCalledTimes(2);
  });
});
