import { describe, expect, it } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import { defaultDi, opzioneUnica, righeDelVoto, risponde, scelteAlDefault } from "./cruscotto";
import turnoZeroJson from "../../../../.scratch/v1-piano-11-ui/viste/01-turno-zero.json";
import trattativaJson from "../../../../.scratch/v1-piano-11-ui/viste/02-trattativa-incarico.json";
import governoJson from "../../../../.scratch/v1-piano-11-ui/viste/03-governo-ordinario.json";
import crisiJson from "../../../../.scratch/v1-piano-11-ui/viste/05-crisi.json";
import opposizioneJson from "../../../../.scratch/v1-piano-11-ui/viste/06-opposizione-governo-ia.json";
import cruscottoJson from "../../../../.scratch/v1-piano-11-ui/viste/17-cruscotto.json";
import campagnaJson from "../../../../.scratch/v1-piano-11-ui/viste/26-campagna.json";

const turnoZero = turnoZeroJson as unknown as Vista;
const trattativa = trattativaJson as unknown as Vista;
const governo = governoJson as unknown as Vista;
const crisi = crisiJson as unknown as Vista;
const opposizione = opposizioneJson as unknown as Vista;
const sceltaFatta = (cruscottoJson as unknown as { scelte_fatte: { vista: Vista } }).scelte_fatte.vista;
const dichiarata = (campagnaJson as unknown as { dichiarata: Vista }).dichiarata;

describe("scelteAlDefault", () => {
  it("conta solo gli eventi con più opzioni e senza scelta", () => {
    expect(scelteAlDefault(turnoZero)).toBe(0);
    expect(scelteAlDefault(governo)).toBe(1);
    expect(scelteAlDefault(sceltaFatta)).toBe(0);
    expect(scelteAlDefault(crisi)).toBe(0);
  });
});

describe("defaultDi e opzioneUnica", () => {
  it("l'opzione col campo default", () => {
    expect(defaultDi(governo.eventi[0])?.id).toBe("fondo");
    expect(defaultDi({ ...governo.eventi[0], opzioni: [] })).toBeNull();
  });
  it("opzione unica: lo scandalo della crisi sì, l'emergenza no", () => {
    expect(opzioneUnica(crisi.eventi[0])).toBe(true);
    expect(opzioneUnica(governo.eventi[0])).toBe(false);
  });
});

describe("risponde", () => {
  it("il giocatore che governa, non in trattativa né all'opposizione", () => {
    expect(risponde(governo)).toBe(true);
    expect(risponde(trattativa)).toBe(false);
    expect(risponde(opposizione)).toBe(false);
  });
});

describe("righeDelVoto", () => {
  it("al turno zero: nessun dichiarato, due disponibili, nessuna spesa", () => {
    const r = righeDelVoto(turnoZero);
    expect(r.coalizione.dichiarati).toEqual([]);
    expect(r.coalizione.disponibili).toEqual(["Centrodestra", "Destra"]);
    expect(r.campagna).toEqual({ spese: 0, euro: 0 });
  });
  it("coalizione dichiarata: i due nomi", () => {
    expect(righeDelVoto(dichiarata).coalizione.dichiarati).toEqual(["Centrodestra", "Destra"]);
  });
  it("le spese sommano gli euro", () => {
    const spesa = { bersaglio: { tipo: "regione", nome: "Lazio" }, leva: "comizi", euro: 200000 };
    const v = { ...turnoZero, campagna: { attiva: true, spese: [spesa, spesa, spesa] } } as unknown as Vista;
    expect(righeDelVoto(v).campagna).toEqual({ spese: 3, euro: 600000 });
  });
});
