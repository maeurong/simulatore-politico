import { describe, expect, it } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import type { PuntoSerie } from "$lib/tipi/PuntoSerie";
import { caselle, tracciatiDi, type Scala } from "./tracciati";
import turnoZeroJson from "../../../.scratch/v1-piano-11-ui/viste/01-turno-zero.json";
import governoJson from "../../../.scratch/v1-piano-11-ui/viste/03-governo-ordinario.json";
import crisiJson from "../../../.scratch/v1-piano-11-ui/viste/05-crisi.json";
import opposizioneJson from "../../../.scratch/v1-piano-11-ui/viste/06-opposizione-governo-ia.json";
import trattativaJson from "../../../.scratch/v1-piano-11-ui/viste/02-trattativa-incarico.json";
import finitaJson from "../../../.scratch/v1-piano-11-ui/viste/07-partita-finita-eredita.json";
import registroJson from "../../../.scratch/v1-piano-11-ui/viste/25-registro.json";
import contiJson from "../../../.scratch/v1-piano-11-ui/viste/21-conti.json";

/* eslint-disable @typescript-eslint/no-explicit-any */
const vista = (x: unknown) => x as Vista;
const turnoZero = vista(turnoZeroJson);
const governo = vista(governoJson);
const trattativa = vista(trattativaJson);
const crisi = vista(crisiJson);
const opposizione = vista(opposizioneJson);
const finita = vista(finitaJson);
const dichiarata = vista((registroJson as any)[0].prima);
const spreadAlto = (contiJson as any).spread_alto as { vista: Vista; serie: PuntoSerie[] };

const premio: Scala = { stacco: 42, passo: 4, rovesciata: false };
const lista: Scala = { stacco: 3, passo: 1, rovesciata: false };
const margine: Scala = { stacco: 0, passo: 3, rovesciata: false };
const spread: Scala = { stacco: 550, passo: 75, rovesciata: true };

describe("caselle", () => {
  it("sulla soglia: fuori dal pericolo dove la regola salva", () => {
    expect(caselle(42, premio).piene).toBe(4);
    expect(caselle(41.99, premio).piene).toBe(3);
    expect(caselle(0, margine).piene).toBe(4);
    expect(caselle(-1, margine).piene).toBe(3);
    expect(caselle(3, lista).piene).toBe(4);
  });
  it("spread rovesciato: a 550 la regola è già scattata", () => {
    expect(caselle(550, spread).piene).toBe(3);
    expect(caselle(549, spread).piene).toBe(4);
  });
  it("fuori scala", () => {
    expect(caselle(13.4, lista)).toEqual({ piene: 10, sotto: false, sopra: true });
    expect(caselle(-121, margine)).toEqual({ piene: 0, sotto: true, sopra: false });
  });
});

describe("tracciatiDi", () => {
  it("turno zero: partito, seggi attesi, spread", () => {
    const [consenso, seggi, conti] = tracciatiDi(turnoZero, []);
    expect(consenso.testo).toBe("19,2%");
    expect(caselle(consenso.valore!, consenso.scala!)).toEqual({ piene: 10, sotto: false, sopra: true });
    expect(consenso.rosso).toBe(false);
    expect(seggi.valore).toBe(-121);
    expect(seggi.ramo).toBe("Camera");
    expect(seggi.rosso).toBe(true);
    expect(conti.valore).toBe(84);
    expect(caselle(conti.valore!, conti.scala!).piene).toBe(10);
    expect(conti.rosso).toBe(false);
  });

  it("turno zero con la coalizione dichiarata: seggi attesi sul Senato", () => {
    const seggi = tracciatiDi(dichiarata, [])[1];
    expect(seggi.valore).toBe(15);
    expect(seggi.ramo).toBe("Senato");
    expect(caselle(seggi.valore!, seggi.scala!).piene).toBe(9);
    expect(seggi.rosso).toBe(false);
  });

  it("al governo: consenso della coalizione e maggioranza", () => {
    const [consenso, maggioranza] = tracciatiDi(governo, []);
    expect(consenso.valore).toBeCloseTo(56.68, 2);
    expect(consenso.testo).toBe("56,7%");
    expect(caselle(consenso.valore!, consenso.scala!).piene).toBe(7);
    expect(consenso.rosso).toBe(false);
    expect(maggioranza.valore).toBe(14);
    expect(maggioranza.testo).toBe("+14");
    expect(maggioranza.ramo).toBe("Senato");
    expect(caselle(maggioranza.valore!, maggioranza.scala!).piene).toBe(8);
  });

  it("in crisi: rosso il consenso dalla proiezione, rossa la maggioranza", () => {
    const [consenso, maggioranza] = tracciatiDi(crisi, []);
    expect(consenso.testo).toBe("20,8%");
    expect(consenso.rosso).toBe(true);
    expect(maggioranza.valore).toBe(-97);
    expect(maggioranza.ramo).toBe("Camera");
    expect(maggioranza.rosso).toBe(true);
  });

  it("senza proiezione il consenso al governo non si accende", () => {
    expect(tracciatiDi({ ...crisi, proiezione: null }, [])[0].rosso).toBe(false);
  });

  it("partita finita: tutti grigi, nessuno rosso", () => {
    const tre = tracciatiDi(finita, []);
    expect(tre.every((x) => x.grigio)).toBe(true);
    expect(tre.some((x) => x.rosso)).toBe(false);
    const [consenso, maggioranza] = tre;
    expect(consenso.nome).toBe("tracciato.consenso_coalizione");
    expect(consenso.valore).toBeCloseTo(48.67, 2);
    expect(consenso.testo).toBe("48,7%");
    expect(caselle(consenso.valore!, consenso.scala!).piene).toBe(5);
    expect(maggioranza.nome).toBe("tracciato.maggioranza");
    expect(maggioranza.valore).toBe(14);
    expect(maggioranza.ramo).toBe("Senato");
    expect(caselle(maggioranza.valore!, maggioranza.scala!).piene).toBe(8);
  });

  it("partita finita senza alleati: l'ultimo stato è l'opposizione", () => {
    const secondo = tracciatiDi({ ...opposizione, finita: true }, [])[1];
    expect(secondo.nome).toBe("tracciato.per_farlo_cadere");
    expect(secondo.valore).toBeNull();
    expect(secondo.grigio).toBe(true);
  });

  it("lo spread ha il valore e rimanda all'11g per lo stacco", () => {
    const conti = tracciatiDi(governo, [])[2];
    expect(conti.valore).toBe(86);
    expect(conti.rimando).toBe("11g");
    expect(caselle(conti.valore!, conti.scala!).piene).toBe(10);
  });

  // la serie dà consenso e margini della coalizione: col partito solo o senza incarico non sono la stessa grandezza
  const punto = (turno: number): PuntoSerie => ({ turno, data: { anno: 2027, trimestre: 1 }, consenso: 0.5, margine_camera: 10, margine_senato: 5, spread_pb: 80 });
  it("all'opposizione il consenso del partito non ha variazione", () => {
    const [consenso, , conti] = tracciatiDi(opposizione, [punto(1), punto(2)]);
    expect(consenso.nome).toBe("tracciato.consenso_partito");
    expect(consenso.prima).toBeNull();
    expect(conti.prima).toBe(80);
  });

  it("in trattativa senza incarico consenso e maggioranza non hanno variazione; con l'incarico sì", () => {
    const [consenso, maggioranza, conti] = tracciatiDi({ ...trattativa, richieste: [] }, [punto(0), punto(1)]);
    expect(consenso.prima).toBeNull();
    expect(maggioranza.prima).toBeNull();
    expect(conti.prima).toBe(80);
    const [conIncarico] = tracciatiDi(trattativa, [punto(0), punto(1)]);
    expect(conIncarico.prima).toBe(50);
  });

  it("all'opposizione: «per farlo cadere» rimanda all'11e", () => {
    const secondo = tracciatiDi(opposizione, [])[1];
    expect(secondo.valore).toBeNull();
    expect(secondo.rimando).toBe("11e");
  });

  it("la variazione viene dal punto della serie del turno prima", () => {
    expect(tracciatiDi(spreadAlto.vista, spreadAlto.serie)[2].prima).toBe(196);
    expect(tracciatiDi(governo, [])[2].prima).toBeNull();
  });

  it("in trattativa la coalizione viene dalle richieste", () => {
    const [consenso, maggioranza] = tracciatiDi(trattativa, []);
    expect(consenso.testo).toBe("50,9%");
    expect(caselle(consenso.valore!, consenso.scala!).piene).toBe(6);
    expect(consenso.rosso).toBe(false);
    expect(maggioranza.valore).toBe(14);
    expect(maggioranza.ramo).toBe("Senato");
    expect(caselle(maggioranza.valore!, maggioranza.scala!).piene).toBe(8);
    expect(maggioranza.rosso).toBe(false);
  });

  it("senza premio lo stacco del consenso della coalizione è 50", () => {
    const disposizioni = governo.disposizioni.map((d) => (d.id === "premio" ? { ...d, in_vigore: "nessuno" } : d));
    const consenso = tracciatiDi({ ...governo, disposizioni }, [])[0];
    expect(consenso.scala?.stacco).toBe(50);
    expect(caselle(consenso.valore!, consenso.scala!).piene).toBe(5);
  });

  it("senza istituti il consenso manca: tratteggiato, senza rimando", () => {
    const consenso = tracciatiDi({ ...governo, sondaggio: { ...governo.sondaggio, istituti: [] } }, [])[0];
    expect(consenso.valore).toBeNull();
    expect(consenso.rimando).toBeNull();
    expect(consenso.rosso).toBe(false);
  });

  it("col partito fuori dal sondaggio il consenso manca", () => {
    const consenso = tracciatiDi({ ...opposizione, partiti: opposizione.partiti.filter((p) => p.id !== opposizione.giocatore) }, [])[0];
    expect(consenso.valore).toBeNull();
    expect(consenso.rimando).toBeNull();
    expect(consenso.rosso).toBe(false);
  });
});
