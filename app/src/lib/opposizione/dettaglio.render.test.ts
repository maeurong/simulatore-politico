import { describe, expect, it } from "vitest";
import { render } from "svelte/server";
import type { Vista } from "$lib/tipi/Vista";
import type { AnteprimaSfiducia } from "$lib/tipi/AnteprimaSfiducia";
import opposizione from "../../../../.scratch/v1-piano-11-ui/viste/28-opposizione.json";
import DettaglioReferendum from "./DettaglioReferendum.svelte";
import DettaglioSfiducia from "./DettaglioSfiducia.svelte";

const stati = opposizione as unknown as Record<string, { prima?: Vista; vista?: Vista; dopo_comando?: Vista; anteprime?: { governo: { ok: AnteprimaSfiducia } } }>;
const prima = stati.sfiducia.prima!;
const anteprima = stati.sfiducia.anteprime!.governo.ok;
const base = { manda: async () => true, rifiuto: null };
const sf = (vista: Vista, a: AnteprimaSfiducia | null | undefined, dicastero: string | null = null) => render(DettaglioSfiducia, { props: { vista, dicastero, anteprima: a, ...base } }).body;

describe("DettaglioSfiducia", () => {
  it("libera: firme a parole, esito, costo se respinta, «Presenta» e nessun numero di firme", () => {
    const body = sf(prima, anteprima);
    expect(body).toContain("le firme bastano");
    expect(body).toContain("non passa");
    expect(body).toContain("perdi credibilità e gli alleati del governo si stringono");
    expect(body).toContain("Presenta la sfiducia");
    expect(body).not.toContain("presentata: si vota");
  });
  it("il 28 «passa» dice passa", () => {
    const v = stati.sfiducia_che_passa.vista!;
    const a = (stati.sfiducia_che_passa as unknown as { anteprime: { governo: { ok: AnteprimaSfiducia } } }).anteprime.governo.ok;
    expect(sf(v, a)).toContain("passa</b>");
  });
  it("presentata da me: il chip e niente bottone", () => {
    const body = sf(stati.sfiducia.dopo_comando as unknown as Vista, anteprima);
    expect(body).toContain("presentata: si vota alla chiusura");
    expect(body).not.toContain("Presenta la sfiducia");
  });
  it("presentata da un altro partito: bottone spento col motivo", () => {
    const body = sf({ ...prima, sfiducia: { dicastero: null, partito: "sinistra" } }, anteprima);
    expect(body).toContain("Presentata da");
    expect(body).toContain('aria-disabled="true"');
  });
  it("una sfiducia già presentata altrove occupa il posto di questa", () => {
    const body = sf({ ...prima, sfiducia: { dicastero: "giustizia", partito: prima.giocatore } }, anteprima);
    expect(body).toContain("Una sola sfiducia a turno");
  });
  it("anteprima non arrivata o guasta: lo dice, e «Presenta» resta offerto", () => {
    expect(sf(prima, undefined)).toContain("Anteprima in arrivo");
    const guasta = sf(prima, null);
    expect(guasta).toContain("Anteprima non disponibile");
    expect(guasta).toContain("Presenta la sfiducia");
  });
  it("firme che non bastano: bottone spento", () => {
    expect(sf(prima, { ...anteprima, firme: false })).toContain("le firme non bastano");
  });
  it("a un ministro: nome e partito del ministro", () => {
    const m = prima.ministeri.find((x) => x.dicastero === "giustizia")!.ministro!;
    expect(sf(prima, anteprima, "giustizia")).toContain(m.nome);
  });
});

describe("DettaglioReferendum", () => {
  const ref = stati.referendum.prima!;
  const abr = (vista: Vista, rifiuto: { testo: string; chiave: string } | null = null) => render(DettaglioReferendum, { props: { vista, quale: "abrogativo", manda: async () => true, rifiuto } }).body;
  it("abrogativo: «Promuovi» solo dove il core ammette, il motivo del core dove no, i non referendabili in elenco", () => {
    const body = abr(ref);
    const ammesse = ref.disposizioni.filter((d) => d.referendabile && d.rifiuto_referendum === null).length;
    expect(body.match(/>Promuovi</g)).toHaveLength(ammesse);
    expect(body).toContain("niente da abrogare: Ticket sanitario");
    expect(body).toContain("non referendabile");
    expect(body).toContain("Regime forfettario degli autonomi");
  });
  it("il rifiuto del comando sta sotto la sua disposizione e non sotto le altre", () => {
    const body = abr(ref, { testo: "RIFIUTO-X", chiave: "ref:tetto_contante" });
    expect(body.match(/RIFIUTO-X/g)).toHaveLength(1);
    expect(body.indexOf("RIFIUTO-X")).toBeGreaterThan(body.indexOf("Tetto"));
  });
  it("con un referendum in corso lo dice e promosso da chi", () => {
    const body = abr(stati.referendum.dopo_comando!);
    expect(body).toContain("In corso · promosso da");
  });
  const confVista = (scelta: boolean, richiesto: boolean): Vista => ({
    ...ref,
    referendum: [{ tipo: "confermativo", nome: "Numero dei parlamentari", voto: { anno: 2027, trimestre: 2 }, promotore: null, firme: true, invito_astensione: false, richiesto, scelta }],
    esiti: [{ forma: "costituzionale", deliberazione: 2, nome: "Numero dei parlamentari", decaduta: null, misura: 7, data: { anno: 2026, trimestre: 4 } } as unknown as Vista["esiti"][number]],
  });
  const conf = (v: Vista) => render(DettaglioReferendum, { props: { vista: v, quale: "confermativo", ...base } }).body;
  it("confermativo da decidere: i due bottoni, senza conferma", () => {
    const body = conf(confVista(true, false));
    expect(body).toContain("Chiedi il referendum");
    expect(body).toContain("Non chiederlo");
    expect(body).not.toContain("Annulla");
  });
  it("confermativo già deciso: niente bottoni; senza confermativo: il testo vuoto", () => {
    expect(conf(confVista(false, true))).not.toContain("Chiedi il referendum");
    expect(conf(ref)).toContain("Nessuna norma costituzionale da confermare");
  });
});
