import { beforeEach, describe, expect, it, vi } from "vitest";
import { render } from "svelte/server";
import type { Vista } from "$lib/tipi/Vista";
import ordinario from "../../../../.scratch/v1-piano-11-ui/viste/03-governo-ordinario.json";
import opposizione from "../../../../.scratch/v1-piano-11-ui/viste/06-opposizione-governo-ia.json";

// In SSR non si clicca: il Pulsante finto tiene le proprietà con cui la Cornice lo monta, e il clic è `onclick()`.
const { pulsanti } = vi.hoisted(() => ({ pulsanti: [] as { forma: string; onclick?: () => void }[] }));
vi.mock("$lib/componenti/Pulsante.svelte", () => ({ default: (_: unknown, props: (typeof pulsanti)[number]) => void pulsanti.push(props) }));

import { partita } from "$lib/partita.svelte";
import Cornice from "./Cornice.svelte";

// `.body` è pigro: i Pulsanti si montano quando lo si legge
const monta = (vista: unknown) => render(Cornice, { props: { vista: vista as Vista, sezione: "cruscotto", impostazioni: false, onimpostazioni: () => {} } }).body;
const avanti = () => pulsanti.filter((p) => p.forma === "bordo");

beforeEach(() => {
  pulsanti.length = 0;
  vi.restoreAllMocks();
});

describe("Cornice, «Avanti fino a novità»", () => {
  it("all'opposizione c'è, e il clic chiama partita.avanti", () => {
    const avantiFn = vi.spyOn(partita, "avanti").mockResolvedValue(undefined);
    monta(opposizione);
    expect(avanti()).toHaveLength(1);
    avanti()[0].onclick?.();
    expect(avantiFn).toHaveBeenCalledOnce();
  });

  it("al governo non c'è", () => {
    monta(ordinario);
    expect(avanti()).toHaveLength(0);
    expect(pulsanti.some((p) => p.forma === "chiude")).toBe(true);
  });
});
