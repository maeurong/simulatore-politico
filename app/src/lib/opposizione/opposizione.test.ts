import { describe, expect, it } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import type { AnteprimaSfiducia } from "$lib/tipi/AnteprimaSfiducia";
import opposizione from "../../../../.scratch/v1-piano-11-ui/viste/28-opposizione.json";
import { leve, maggioranzaAvversaria, misureDelGoverno, programma, siMancanti } from "./opposizione";

/* eslint-disable @typescript-eslint/no-explicit-any */
const stati = opposizione as any;
const vista = (x: unknown) => x as Vista;
const di = (v: Vista, tipo: string) => leve(v).find((l) => l.tipo === tipo)!;

describe("leve", () => {
  const prima = vista(stati.sfiducia.prima);
  it("sfiducia al governo: libera, poi presentata dopo il comando", () => {
    expect(di(prima, "sfiducia_governo")).toMatchObject({ stato: "libera", chi: null });
    expect(di(vista(stati.sfiducia.dopo_comando), "sfiducia_governo")).toMatchObject({ stato: "presentata", chi: null });
  });
  it("presentata la sfiducia al governo, quella a un ministro è spenta col suo motivo", () => {
    const l = di(vista(stati.sfiducia.dopo_comando), "sfiducia_ministro");
    expect(l.stato).toBe("spenta");
    expect(l.motivo).toBe("opposizione.sfiducia_gia_presentata");
  });
  it("sfiducia di un altro partito: altrui, con chi", () => {
    const v = { ...prima, sfiducia: { dicastero: null, partito: "sinistra" } };
    expect(di(v, "sfiducia_governo")).toMatchObject({ stato: "altrui", chi: "sinistra" });
  });
  it("sfiducia a un ministro presentata: tocca la leva del ministro, non quella del governo", () => {
    const v = { ...prima, sfiducia: { dicastero: "giustizia", partito: "giocatore" } };
    expect(di(v, "sfiducia_ministro").stato).toBe("presentata");
    expect(di(v, "sfiducia_governo").stato).toBe("spenta");
  });
  it("posto riservato: di un altro è altrui col nome giusto", () => {
    expect(di(vista(stati.grave.vista), "posto")).toMatchObject({ stato: "altrui", chi: "nuovo_8" });
    expect(di(vista(stati.intesa_varia.vista), "posto")).toMatchObject({ stato: "altrui", chi: "estrema_destra" });
  });
  it("posto riservato: mio libero, poi presentata", () => {
    expect(di(vista(stati.posto_mio.prima), "posto")).toMatchObject({ stato: "libera", chi: null });
    expect(di(vista(stati.posto_mio.vista), "posto")).toMatchObject({ stato: "presentata", chi: null });
  });
  it("referendum abrogativo: libero, poi presentato dal giocatore", () => {
    expect(di(prima, "abrogativo").stato).toBe("libera");
    expect(di(vista(stati.referendum.dopo_comando), "abrogativo").stato).toBe("presentata");
  });
  it("referendum abrogativo di un altro promotore: altrui, con chi (il core ne ammette uno alla volta)", () => {
    const r = { tipo: "abrogativo", nome: "x", promotore: "sinistra", firme: false, invito_astensione: false, richiesto: false, scelta: false, voto: prima.voto };
    expect(di({ ...prima, referendum: [r] } as unknown as Vista, "abrogativo")).toMatchObject({ stato: "altrui", chi: "sinistra" });
  });
  it("posto: senza posti riservati la leva è spenta e senza motivo", () => {
    expect(di({ ...prima, posto_riservato: false, posto_riservato_di: null, calendario: [] } as unknown as Vista, "posto")).toMatchObject({ stato: "spenta", motivo: null });
  });
  it("confermativo: solo se richiesto o richiedibile", () => {
    expect(leve(prima).some((l) => l.tipo === "confermativo")).toBe(false);
    const r = (x: object) => ({ ...prima, referendum: [{ tipo: "confermativo", nome: "x", promotore: null, firme: true, invito_astensione: false, richiesto: false, scelta: false, voto: prima.voto, ...x }] }) as unknown as Vista;
    expect(leve(r({})).some((l) => l.tipo === "confermativo")).toBe(false);
    expect(di(r({ scelta: true }), "confermativo").stato).toBe("libera");
    expect(di(r({ richiesto: true }), "confermativo").stato).toBe("presentata");
  });
});

describe("misureDelGoverno", () => {
  it("lo stato linea: nessuna misura del giocatore, nell'ordine del calendario", () => {
    const v = vista(stati.linea.vista);
    const m = misureDelGoverno(v);
    expect(m.length).toBeGreaterThan(0);
    expect(m.every((x) => x.promotore !== v.giocatore)).toBe(true);
    expect(m.map((x) => x.id)).toEqual(v.calendario.filter((x) => x.promotore !== v.giocatore).map((x) => x.id));
  });
  it("lo stato posto_mio: la proposta del giocatore resta fuori", () => {
    const v = vista(stati.posto_mio.vista);
    expect(v.calendario.some((x) => x.promotore === v.giocatore)).toBe(true);
    expect(misureDelGoverno(v).length).toBe(v.calendario.length - 1);
  });
});

describe("programma e maggioranza avversaria", () => {
  const v = vista(stati.intesa_varia.vista);
  it("il programma è quello della Vista", () => expect(programma(v)).toBe(v.programma));
  it("maggioranza: chi guida senza fascia, gli alleati con la loro", () => {
    const m = maggioranzaAvversaria(v);
    const governo = v.partiti.filter((p) => p.al_governo).map((p) => p.id);
    expect(m.map((x) => x.partito).sort()).toEqual([...governo].sort());
    const guida = m.filter((x) => x.fascia === null);
    expect(guida).toHaveLength(1);
    expect(v.alleati_avversari.some((a) => a.partito === guida[0].partito)).toBe(false);
    const alleato = m.find((x) => x.partito === "sinistra")!;
    expect(alleato.fascia).toBe("ultimatum");
    const s = v.seggi.find((x) => x.partito === "sinistra")!;
    expect(alleato.seggi).toEqual([s.camera, s.senato]);
  });
});

describe("siMancanti", () => {
  it("al turno 2 il governo non passa: metà più uno della Camera meno i sì", () => {
    const v = vista(stati.sfiducia.prima);
    const a = stati.sfiducia.anteprime.governo.ok as AnteprimaSfiducia;
    const camera = v.seggi.reduce((s, x) => s + x.camera, 0);
    const si = ["estrema_sinistra", "estrema_destra", "giocatore"].reduce((s, id) => s + v.seggi.find((x) => x.partito === id)!.camera, 0);
    expect(a.passa).toBe(false);
    expect(siMancanti(v, a)).toBe(Math.floor(camera / 2) + 1 - si);
    expect(siMancanti(v, a)).toBe(201 - si);
  });
  it("la sfiducia che passa: zero", () => {
    expect(siMancanti(vista(stati.sfiducia_che_passa.vista), stati.sfiducia_che_passa.anteprime.governo.ok)).toBe(0);
  });
});
