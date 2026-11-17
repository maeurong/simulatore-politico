import { describe, expect, it } from "vitest";
import { render } from "svelte/server";
import { t } from "$lib/testi/t";
import type { Fermata } from "$lib/tipi/Fermata";
import type { Vista } from "$lib/tipi/Vista";
import Banda from "./Banda.svelte";

const vista = { calendario: [], partiti: [] } as unknown as Vista;
const html = (corsa: { chiusure: number; da: number; fermate: Fermata[] }) =>
  render(Banda, { props: { vista, corsa } }).body;

describe("Banda", () => {
  it("con le fermate le dice, e il link al registro sul più vecchio dei saltati", () => {
    const h = html({ chiusure: 3, da: 16, fermate: [{ tipo: "crisi" }] });
    expect(h).toContain(t("prima.ferma_crisi"));
    expect(h).toContain("Ti ferma qui");
    expect(h).toContain('href="/partita/registro?turno=16"');
  });
  it("senza fermate (guasto a metà corsa, partita finita) dice solo le chiusure, niente frase spezzata", () => {
    const h = html({ chiusure: 2, da: 4, fermate: [] });
    expect(h).toContain(t("prima.banda_senza_fermata", { n: 2, da: 4 }));
    expect(h).not.toContain("Ti ferma qui");
  });
  it("una sola chiusura: niente link, non c'è niente di saltato", () => {
    expect(html({ chiusure: 1, da: 4, fermate: [] })).not.toContain("href=");
  });
});
