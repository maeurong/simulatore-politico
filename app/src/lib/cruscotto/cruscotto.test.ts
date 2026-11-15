import { describe, expect, it } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import { defaultDi, fondoBasta, opzioneUnica, ordineDelGiorno, righeDelVoto, risponde, scelteAlDefault } from "./cruscotto";
import turnoZeroJson from "../../../../.scratch/v1-piano-11-ui/viste/01-turno-zero.json";
import trattativaJson from "../../../../.scratch/v1-piano-11-ui/viste/02-trattativa-incarico.json";
import governoJson from "../../../../.scratch/v1-piano-11-ui/viste/03-governo-ordinario.json";
import crisiJson from "../../../../.scratch/v1-piano-11-ui/viste/05-crisi.json";
import opposizioneJson from "../../../../.scratch/v1-piano-11-ui/viste/06-opposizione-governo-ia.json";
import cruscottoJson from "../../../../.scratch/v1-piano-11-ui/viste/17-cruscotto.json";
import aulaJson from "../../../../.scratch/v1-piano-11-ui/viste/19-aula.json";
import campagnaJson from "../../../../.scratch/v1-piano-11-ui/viste/26-campagna.json";

const turnoZero = turnoZeroJson as unknown as Vista;
const trattativa = trattativaJson as unknown as Vista;
const governo = governoJson as unknown as Vista;
const crisi = crisiJson as unknown as Vista;
const opposizione = opposizioneJson as unknown as Vista;
const sceltaFatta = (cruscottoJson as unknown as { scelte_fatte: { vista: Vista } }).scelte_fatte.vista;
const stati = cruscottoJson as unknown as Record<"ultimatum" | "esercizio_provvisorio" | "crisi_provvisorio", { vista: Vista }>;
const dopoVoto = (aulaJson as unknown as { dopo_voto: { vista: Vista } }).dopo_voto.vista;
const dichiarata = (campagnaJson as unknown as { dichiarata: Vista }).dichiarata;

describe("scelteAlDefault", () => {
  it("gli eventi senza scelta più le voci al default dell'ordine del giorno", () => {
    expect(scelteAlDefault(governo)).toBe(4);
    expect(scelteAlDefault(sceltaFatta)).toBe(2);
    expect(scelteAlDefault(stati.ultimatum.vista)).toBe(1);
    expect(scelteAlDefault(stati.esercizio_provvisorio.vista)).toBe(1);
  });
  it("la crisi e i sei interim; lo scandalo a opzione unica non conta", () => {
    expect(scelteAlDefault(crisi)).toBe(7);
  });
  it("in esercizio provvisorio la crisi non conta, il bilancio nemmeno", () => {
    expect(scelteAlDefault(stati.crisi_provvisorio.vista)).toBe(6);
  });
  it("al turno zero le proposte del governo uscente non contano", () => {
    expect(scelteAlDefault(turnoZero)).toBe(0);
  });
});

describe("ordineDelGiorno", () => {
  it("la 03: tre proposte nell'ordine dei dicasteri, ciascuna col suo default", () => {
    const p = ordineDelGiorno(governo).filter((x) => x.gruppo === "proposte");
    expect(p.map((x) => x.dicastero)).toEqual(["infrastrutture_trasporti", "ambiente_energia", "esteri_difesa"]);
    expect(p.every((x) => x.default === "senza scelta resta in sospeso al più un turno, poi vale respinta" && x.alDefault && !x.fatta)).toBe(true);
  });
  it("una proposta rinviata non è al default e ha la sua riga", () => {
    const r = ordineDelGiorno(sceltaFatta).find((x) => x.dicastero === "ambiente_energia");
    expect(r).toMatchObject({ gruppo: "proposte", alDefault: false, fatta: true });
  });
  it("dopo il voto: due emendamenti aperti, il testo si vota com'è", () => {
    const e = ordineDelGiorno(dopoVoto).filter((x) => x.gruppo === "emendamenti");
    expect(e).toHaveLength(2);
    expect(e.map((x) => x.default)).toEqual(Array(2).fill("resta aperto, si vota il testo com’è"));
  });
  it("l'ultimatum della Sinistra sta in «Da tenere d'occhio»", () => {
    const o = ordineDelGiorno(stati.ultimatum.vista).filter((x) => x.gruppo === "occhio");
    expect(o.map((x) => x.partito)).toEqual(["sinistra"]);
    expect(o[0].azioni).toEqual({ rimando: "partiti" });
  });
  it("il bilancio da presentare sta in «Da decidere» con il rimando; nella crisi non c'è", () => {
    expect(ordineDelGiorno(stati.esercizio_provvisorio.vista)[0]).toMatchObject({ gruppo: "da_decidere", azioni: { rimando: "bilancio" }, urgente: true });
    expect(ordineDelGiorno(stati.crisi_provvisorio.vista).some((x) => x.azioni === "successore")).toBe(true);
    expect(ordineDelGiorno(stati.crisi_provvisorio.vista).some((x) => typeof x.azioni === "object")).toBe(false);
  });
  it("i sei interim della crisi: una voce ciascuno, senza ministro", () => {
    expect(ordineDelGiorno(crisi).filter((x) => x.azioni === "successore")).toHaveLength(6);
  });
  it("fuori dal governo niente", () => {
    expect(ordineDelGiorno(turnoZero)).toEqual([]);
    expect(ordineDelGiorno(opposizione)).toEqual([]);
  });
});

describe("fondoBasta", () => {
  it("falso sulla 03: fondo 0, evento da un miliardo", () => {
    expect(fondoBasta(governo, governo.eventi[0])).toBe(false);
  });
  it("vero se il fondo copre il costo", () => {
    expect(fondoBasta({ ...governo, bilancio: { ...governo.bilancio, fondo_residuo: 2e9 } }, governo.eventi[0])).toBe(true);
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
