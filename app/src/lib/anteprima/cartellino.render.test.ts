import { describe, expect, it, vi } from "vitest";
import { render } from "svelte/server";
import type { Vista } from "$lib/tipi/Vista";
import aula from "../../../../.scratch/v1-piano-11-ui/viste/19-aula.json";

vi.mock("$app/navigation", () => ({ goto: vi.fn() }));

import Cartellino from "./Cartellino.svelte";

const piena = (aula as unknown as Record<string, { vista: Vista }>).piena.vista;
const m = piena.calendario.find((x) => x.id === 3)!;
const prova = (vista: Vista, valori = m.valori) => render(Cartellino, { props: { vista, misura: m, valori, manda: async () => true, rifiuto: null } }).body;

describe("Cartellino, compra il test", () => {
  it("con le disposizioni c'è, abilitato", () => {
    const body = prova(piena);
    expect(body).toContain("Compra il test");
    expect(body).not.toContain("già comprato");
  });
  it("senza disposizioni (proposta di fondi, bilancio, decreto di fondi) non c'è: il core lo rifiuterebbe come «legge vuota»", () => {
    expect(prova(piena, [])).not.toContain("Compra il test");
  });
  it("con il test di un'altra misura già comprato nel turno è spento col motivo", () => {
    const altro = { ...piena, acquisti: { ...piena.acquisti, test_misura: { misura: "altra", reazioni: [] } } };
    const body = prova(altro);
    expect(body).toContain("Compra il test");
    expect(body).toContain("già comprato un test in questo turno");
    expect(body).toContain('aria-disabled="true"');
  });
});
