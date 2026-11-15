import { describe, expect, it } from "vitest";
import { render } from "svelte/server";
import type { Vista } from "$lib/tipi/Vista";
import aula from "../../../../.scratch/v1-piano-11-ui/viste/19-aula.json";
import opposizione from "../../../../.scratch/v1-piano-11-ui/viste/28-opposizione.json";
import crisi from "../../../../.scratch/v1-piano-11-ui/viste/05-crisi.json";
import Referendum from "./Referendum.svelte";
import ordinario from "../../../../.scratch/v1-piano-11-ui/viste/03-governo-ordinario.json";
import Disposizioni from "./Disposizioni.svelte";

const prop = { onapri: () => {}, oncambia: () => {}, manda: async () => true, rifiuto: null };
const opp = (opposizione as unknown as { referendum: { prima: Vista } }).referendum.prima;

describe("Disposizioni, render", () => {
  it("ogni «Cambia ›» ha il nome accessibile della sua disposizione", () => {
    const vista = ordinario as unknown as Vista;
    const { body } = render(Disposizioni, { props: { vista, bozza: [], aperta: null, onapri: () => {}, oncambia: () => {}, manda: async () => true, rifiuto: null } });
    const attese = vista.disposizioni.filter((d) => d.norma !== "elettorale");
    for (const d of attese) expect(body).toContain(`aria-label="Cambia: ${d.nome}"`);
    expect(body.match(/aria-label="Cambia: /g)).toHaveLength(attese.length);
  });

  it("all'opposizione la riga aperta offre «Promuovi» con la conferma se il core ammette, il testo senza bottone se no", () => {
    const aperta = (id: string) => render(Disposizioni, { props: { vista: opp, bozza: [], aperta: id, ...prop } }).body;
    expect(aperta("tetto_contante")).toContain("Promuovi un referendum abrogativo");
    const rifiutata = aperta("ticket");
    expect(rifiutata).toContain("niente da abrogare: Ticket sanitario");
    expect(rifiutata).not.toContain("Promuovi un referendum abrogativo");
  });
  it("al governo la riga aperta non offre il referendum", () => {
    const v = (aula as unknown as Record<string, { vista: Vista }>).piena.vista;
    expect(render(Disposizioni, { props: { vista: v, bozza: [], aperta: "salario_minimo", ...prop } }).body).not.toContain("abrogativo");
  });
});

describe("Referendum, render", () => {
  it("al governo l'abrogativo in attesa ha «Invita all'astensione»; l'opposizione no", () => {
    const g = crisi as unknown as Vista;
    expect(render(Referendum, { props: { vista: g, manda: async () => true, rifiuto: null } }).body).toContain("Invita all’astensione");
    const o = { ...g, governa: false };
    expect(render(Referendum, { props: { vista: o, manda: async () => true, rifiuto: null } }).body).not.toContain("Invita all’astensione");
  });
  it("senza referendum dice che non ce ne sono", () => {
    const { body } = render(Referendum, { props: { vista: opp, manda: async () => true, rifiuto: null } });
    expect(body).toContain("Nessun referendum in attesa.");
    expect(body).toContain("Nessun referendum votato.");
  });
});
