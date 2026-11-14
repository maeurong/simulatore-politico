import { describe, expect, it } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import { aChi, completa, defaultDellaChiusura, pesoDi, proponi, risultato, type Bozza } from "./trattativa";
import turnoZeroJson from "../../../../.scratch/v1-piano-11-ui/viste/01-turno-zero.json";
import trattativaJson from "../../../../.scratch/v1-piano-11-ui/viste/02-trattativa-incarico.json";
import campioneJson from "../../../../.scratch/v1-piano-11-ui/viste/27-trattativa.json";

const turnoZero = turnoZeroJson as unknown as Vista;
const v02 = trattativaJson as unknown as Vista;
const campione = campioneJson as unknown as { premio: { pesi: number[] }; guida: { vista: Vista }; capo_dello_stato: { vista: Vista } };
const ids = v02.dicasteri.map((d) => d.id);
const piena = (b: Bozza = {}): Bozza => Object.fromEntries(ids.map((id) => [id, id in b ? b[id] : "giocatore"]));

describe("pesoDi", () => {
  it("sui dieci dicasteri della 02 dà i pesi del campione", () => {
    expect(ids.map(pesoDi)).toEqual(campione.premio.pesi);
    expect(ids.map(pesoDi)).toEqual([3, 1, 1, 1, 1, 1, 1, 2, 1, 2]);
  });
});

describe("aChi", () => {
  it("il giocatore, il tecnico, poi gli alleati delle richieste", () => {
    expect(aChi(v02)).toEqual(["giocatore", null, "centrodestra", "destra"]);
  });
});

describe("completa e proponi", () => {
  const nove = Object.fromEntries(ids.slice(1).map((id) => [id, "giocatore"]));
  it("nove dicasteri assegnati non bastano, dieci sì (anche col tecnico)", () => {
    expect(completa(v02, {})).toBe(false);
    expect(completa(v02, nove)).toBe(false);
    expect(completa(v02, { ...nove, [ids[0]]: null })).toBe(true);
  });
  it("proponi è null a bozza incompleta", () => {
    expect(proponi(v02, nove)).toBeNull();
  });
  it("a bozza completa ha dieci assegnazioni nell'ordine della Vista, e il tecnico è partito null", () => {
    const c = proponi(v02, piena({ [ids[1]]: null, [ids[0]]: "destra" }));
    expect(c).toMatchObject({ tipo: "proponi_spartizione" });
    const d = (c as { dicasteri: { dicastero: string; partito: string | null }[] }).dicasteri;
    expect(d.map((x) => x.dicastero)).toEqual(ids);
    expect(d).toHaveLength(10);
    expect(d[0].partito).toBe("destra");
    expect(d[1].partito).toBeNull();
  });
});

describe("risultato", () => {
  it("02: affluenza, premio di tre membri, seggi e soglie della coalizione", () => {
    const r = risultato(v02)!;
    expect(r.affluenza).toBeCloseTo(0.714, 3);
    expect(r.premio).toHaveLength(3);
    expect(r.camera).toEqual({ seggi: 224, soglia: 201 });
    expect(r.senato).toEqual({ seggi: 115, soglia: 101 });
  });
  it("le quote portano il nome e quali sono della coalizione", () => {
    const r = risultato(v02)!;
    expect(r.voti.filter((x) => x.miei).map((x) => x.partito).sort()).toEqual(["centrodestra", "destra", "giocatore"]);
    expect(r.voti.every((x) => x.nome.length > 0)).toBe(true);
  });
  it("senza voto letto è null", () => {
    expect(risultato(turnoZero)).toBeNull();
  });
});

describe("defaultDellaChiusura", () => {
  it("la spartizione sulla 02, la guida sulla guida, niente sul capo dello Stato", () => {
    expect(defaultDellaChiusura(v02)).toBe("cornice.senza_proposta");
    expect(defaultDellaChiusura(campione.guida.vista)).toBe("cornice.senza_scelta_guida");
    expect(defaultDellaChiusura(campione.capo_dello_stato.vista)).toBeNull();
  });
});
