import { describe, expect, it } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import { figurine, obiettivo, posizioni } from "./governo";
import ordinarioJson from "../../../../.scratch/v1-piano-11-ui/viste/03-governo-ordinario.json";
import crisiJson from "../../../../.scratch/v1-piano-11-ui/viste/05-crisi.json";
import governoJson from "../../../../.scratch/v1-piano-11-ui/viste/18-governo.json";

const ordinario = ordinarioJson as unknown as Vista;
const crisi = crisiJson as unknown as Vista;
const stati = governoJson as unknown as Record<string, { vista: Vista }>;
const di = (v: Vista, dicastero: string) => figurine(v).find((f) => f.dicastero === dicastero)!;

describe("figurine", () => {
  it("dieci, nell'ordine dei dicasteri", () => {
    expect(figurine(ordinario).map((f) => f.dicastero)).toEqual(ordinario.dicasteri.map((d) => d.id));
    expect(figurine(ordinario).map((f) => f.indice)).toEqual([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    expect(di(ordinario, "sanita").nomeDicastero).toBe("Sanità");
  });
  it("l'evidenza è quella dell'agenda: distanza, evento, vuota", () => {
    const ev = (d: string) => di(ordinario, d).evidenza;
    expect(ev("economia_finanze")).toEqual(["distanza"]);
    expect(ev("sviluppo_economico")).toEqual(["distanza"]);
    expect(ev("ambiente_energia")).toEqual(["evento"]);
    const altri = figurine(ordinario).filter((f) => !["economia_finanze", "sviluppo_economico", "ambiente_energia"].includes(f.dicastero));
    expect(altri.every((f) => f.evidenza.length === 0)).toBe(true);
  });
  it("senza agenda dice perché", () => {
    expect(di(ordinario, "infrastrutture_trasporti").senzaAgenda).toBe("margine esaurito");
    expect(di(ordinario, "esteri_difesa").senzaAgenda).toBe("margine esaurito");
    expect(di(ordinario, "sanita").senzaAgenda).toBeNull();
    expect(di(ordinario, "sanita").agenda).not.toBeNull();
  });
  it("primo turno di governo: tutti in rodaggio", () => {
    expect(figurine(stati.scelte.vista).every((f) => f.stati.includes("rodaggio"))).toBe(true);
    expect(figurine(stati.scontenti.vista).some((f) => f.stati.includes("rodaggio"))).toBe(false);
  });
  it("scelte: Lavoro perde 11,6 per l'atto fermato, Ambiente ha chiesto le dimissioni", () => {
    const lavoro = di(stati.scelte.vista, "lavoro_welfare");
    expect(lavoro.freccia).toBeCloseTo(-11.6, 1);
    expect(lavoro.voto).toBe(48);
    expect(di(stati.scelte.vista, "ambiente_energia").stati).toContain("dimissioni_chieste");
    expect(di(stati.scelte.vista, "sanita").stati).not.toContain("dimissioni_chieste");
  });
  it("scontenti: sei carte in preavviso", () => {
    expect(figurine(stati.scontenti.vista).filter((f) => f.stati.includes("preavviso"))).toHaveLength(6);
  });
  it("tecnico, interim", () => {
    expect(di(stati.tecnico.vista, "giustizia").stati).toContain("tecnico");
    const amb = di(stati.interim.vista, "ambiente_energia");
    expect(amb.stati).toContain("interim");
    expect(amb.ministro).toBeNull();
    expect(amb.voto).toBeNull();
    expect(amb.freccia).toBe(0);
  });
  it("crisi: sei carte in interim", () => {
    expect(figurine(crisi).filter((f) => f.stati.includes("interim"))).toHaveLength(6);
  });
});

describe("obiettivo", () => {
  it("l'indicatore di servizio contro l'obiettivo del ministro", () => {
    const o = obiettivo(ordinario, "sanita");
    expect(o.nome).toBe("Qualità del servizio sanitario");
    expect(o.valore).toBeCloseTo(99.7, 1);
    expect(o.obiettivo).toBeCloseTo(104.7, 1);
    expect(o.verso).toBe("sale");
  });
  it("in interim l'obiettivo manca", () => {
    expect(obiettivo(stati.interim.vista, "ambiente_energia").obiettivo).toBeNull();
  });
});

describe("posizioni", () => {
  it("la tua è quella percepita del tuo partito", () => {
    const m = ordinario.ministeri[0].ministro!;
    const p = posizioni(ordinario, m);
    expect(p.ministro).toBe(m.posizione);
    expect(p.tua).toBe(ordinario.partiti.find((x) => x.id === ordinario.giocatore)!.posizione_percepita);
  });
});
