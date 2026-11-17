import { beforeEach, describe, expect, it, vi } from "vitest";
import { render } from "svelte/server";
import type { Vista } from "$lib/tipi/Vista";
import registroJson from "../../../../.scratch/v1-piano-11-ui/viste/25-registro.json";

// In SSR non si clicca: la Conferma finta tiene le proprietà con cui Riavvolgi la monta, e la conferma è `onconferma()`.
const { conferme, goto } = vi.hoisted(() => ({ conferme: [] as { onconferma: () => Promise<void> }[], goto: vi.fn() }));
vi.mock("$lib/componenti/Conferma.svelte", () => ({ default: (_: unknown, props: (typeof conferme)[number]) => { conferme.push(props); } }));
vi.mock("$app/navigation", () => ({ goto }));

import { partita } from "$lib/partita.svelte";
import Riavvolgi from "./Riavvolgi.svelte";

const vista = (registroJson as { dopo: Vista }[])[8].dopo;

beforeEach(() => {
  conferme.length = 0;
  vi.restoreAllMocks();
  goto.mockReset();
});

describe("Riavvolgi, «Torna al turno N»", () => {
  it("la conferma chiama partita.riavvolgi con il turno scelto", async () => {
    const riavvolgi = vi.spyOn(partita, "riavvolgi").mockResolvedValue(null);
    void render(Riavvolgi, { props: { turno: 3, vista, turni: [] } }).body;
    await conferme[0].onconferma();
    expect(riavvolgi).toHaveBeenCalledExactlyOnceWith(3);
  });
});
