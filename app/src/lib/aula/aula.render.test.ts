import { describe, expect, it, vi } from "vitest";
import { render } from "svelte/server";
import type { Vista } from "$lib/tipi/Vista";
import aula from "../../../../.scratch/v1-piano-11-ui/viste/19-aula.json";
import bilancio from "../../../../.scratch/v1-piano-11-ui/viste/04-governo-bilancio.json";
import crisi from "../../../../.scratch/v1-piano-11-ui/viste/05-crisi.json";
import grave from "../../../../.scratch/v1-piano-11-ui/viste/08-tono-grave.json";

vi.mock("$app/navigation", () => ({ goto: vi.fn() }));

import Aula from "./Aula.svelte";
import Emiciclo from "../componenti/Emiciclo.svelte";

const stati = aula as unknown as Record<string, { vista: Vista }>;
const viste: [string, Vista][] = [
  ["04", bilancio as unknown as Vista],
  ["05-crisi", crisi as unknown as Vista],
  ["08-tono-grave", grave as unknown as Vista],
  ...["piena", "dopo_voto", "seconda_deliberazione", "dopo_elettorale", "opposizione_libero", "opposizione_proposta"].map((k): [string, Vista] => [`19 ${k}`, stati[k].vista]),
];

describe("Aula, render", () => {
  it.each(viste)("%s: non lancia, ha il calendario e il dettaglio", (_, vista) => {
    const { body } = render(Aula, { props: { vista } });
    expect(body).toContain("posti");
    expect(body).toContain("emi");
  });
  it("il bilancio è un posto solo", () => {
    const { body } = render(Aula, { props: { vista: bilancio as unknown as Vista } });
    expect(body.match(/class="posto/g)).toHaveLength(1);
  });
  it("opposizione: il posto riservato libero dice di portare una proposta", () => {
    const { body } = render(Aula, { props: { vista: stati.opposizione_libero.vista } });
    expect(body).toContain("libero: porta una proposta");
  });
  it("la fiducia si offre sul salario minimo, mai sullo sgravio del Centro", () => {
    const v = stati.piena.vista;
    const sgravio = { ...v, calendario: v.calendario.filter((m) => m.id === 0) };
    expect(render(Aula, { props: { vista: sgravio } }).body).not.toContain("Poni la fiducia");
    const salario = { ...v, calendario: v.calendario.filter((m) => m.id === 3) };
    expect(render(Aula, { props: { vista: salario } }).body).toContain("Poni la fiducia");
  });
});

describe("Misura, emendamenti", () => {
  const dopo = stati.dopo_voto.vista;
  const conStato = (stato: "aperto" | "accolto") => ({
    ...dopo,
    calendario: dopo.calendario.filter((m) => m.id === 3).map((m) => ({ ...m, emendamenti: m.emendamenti.map((e) => ({ ...e, stato })) })),
  });
  it("aperti: Accogli e Respingi e il default; accolti: solo il segno, niente pulsanti", () => {
    const aperti = render(Aula, { props: { vista: conStato("aperto") } }).body;
    expect(aperti.match(/>Accogli</g)).toHaveLength(2);
    const nomi = [...aperti.matchAll(/aria-label="(Accogli|Respingi): emendamento di ([^"]+)"/g)].map((x) => x[0]);
    expect(nomi).toHaveLength(4);
    expect(new Set(nomi).size).toBe(4);
    expect(aperti).toContain("Senza scelta: resta aperto");
    const accolti = render(Aula, { props: { vista: conStato("accolto") } }).body;
    expect(accolti).not.toContain("Accogli");
    expect(accolti).toContain("accolto");
  });
});

describe("Misura, linea", () => {
  const v = stati.opposizione_proposta.vista;
  const con = (si: boolean | null) => ({ ...v, calendario: [{ ...v.calendario[0], linea_giocatore: si }] });
  it("«Come da posizione (x)» solo senza scelta del giocatore: la linea nella Vista riflette la scelta, non il default", () => {
    expect(render(Aula, { props: { vista: con(null) } }).body).toMatch(/Come da posizione \(/);
    const scelta = render(Aula, { props: { vista: con(true) } }).body;
    expect(scelta).toContain("Come da posizione");
    expect(scelta).not.toMatch(/Come da posizione \(/);
  });
});

describe("Emiciclo", () => {
  it("il nome accessibile dice la frase della didascalia e i seggi sono tanti quanto il totale", () => {
    const { body } = render(Emiciclo, { props: { totale: 400, si: [179, 193], maggioranza: 201, ramo: "Camera" } });
    expect(body).toContain('aria-label="Camera · 179–193 sì · ne servono 201"');
    expect(body.match(/<circle/g)).toHaveLength(400);
  });
});

describe("Emiciclo, forbice degenere", () => {
  const fig = (si: [number, number]) => render(Emiciclo, { props: { ramo: "Camera", totale: 400, si, maggioranza: 201 } }).body;
  it("con i due valori uguali ne scrive uno solo, anche nel nome accessibile", () => {
    const b = fig([205, 205]);
    expect(b).not.toContain("205–205");
    expect(b.match(/Camera · 205 sì · ne servono 201/g)).toHaveLength(2);
  });
  it("con una forbice vera scrive i due estremi", () => {
    expect(fig([190, 215])).toContain("Camera · 190–215 sì");
  });
});

describe("InAula, nome accessibile della forbice", () => {
  it("nessun segnaposto rimasto: la forbice si legge con i suoi valori", () => {
    const { body } = render(Aula, { props: { vista: stati.piena.vista } });
    expect(body).not.toContain("{si}");
    expect(body).toMatch(/aria-label="Camera · [\d.]+(–[\d.]+)? sì · ne servono/);
  });
});
