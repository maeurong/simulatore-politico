import { describe, expect, it } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import { PAGINE, SEZIONI, apribile, barrata, casa, etichetta, fettaDi, statoDi, type Sezione } from "./ossatura";
import turnoZeroJson from "../../../.scratch/v1-piano-11-ui/viste/01-turno-zero.json";
import trattativaJson from "../../../.scratch/v1-piano-11-ui/viste/02-trattativa-incarico.json";
import governoJson from "../../../.scratch/v1-piano-11-ui/viste/03-governo-ordinario.json";
import crisiJson from "../../../.scratch/v1-piano-11-ui/viste/05-crisi.json";
import opposizioneJson from "../../../.scratch/v1-piano-11-ui/viste/06-opposizione-governo-ia.json";
import cruscottoJson from "../../../.scratch/v1-piano-11-ui/viste/17-cruscotto.json";
import finitaJson from "../../../.scratch/v1-piano-11-ui/viste/07-partita-finita-eredita.json";
import bilancioJson from "../../../.scratch/v1-piano-11-ui/viste/22-bilancio.json";
import ereditaJson from "../../../.scratch/v1-piano-11-ui/viste/30-eredita.json";

const vista = (x: unknown) => x as Vista;
const turnoZero = vista(turnoZeroJson);
const trattativa = vista(trattativaJson);
const governo = vista(governoJson);
const crisi = vista(crisiJson);
const opposizione = vista(opposizioneJson);
const finita = vista(finitaJson);

const provvisorio = (cruscottoJson as unknown as { esercizio_provvisorio: { vista: Vista } }).esercizio_provvisorio.vista;

describe("bilancio in esercizio provvisorio", () => {
  it("il bilancio presentabile fuori dal quarto trimestre apre la sezione", () => {
    expect(provvisorio.data.trimestre).not.toBe(4);
    expect(provvisorio.bilancio.presentabile).toBe(true);
    expect(barrata("bilancio", provvisorio)).toBeNull();
    expect(apribile("bilancio", provvisorio)).toBe(true);
  });
  it("non presentabile e fuori dal quarto trimestre resta barrata", () => {
    expect(barrata("bilancio", { ...provvisorio, bilancio: { ...provvisorio.bilancio, presentabile: false } })).toBe("barrata.bilancio_stagione");
  });
});

const barrate = (v: Vista) => SEZIONI.filter((s) => barrata(s, v) !== null);

describe("statoDi", () => {
  it("un campione per stato", () => {
    expect(statoDi(turnoZero)).toBe("turno_zero");
    expect(statoDi(trattativa)).toBe("trattativa");
    expect(statoDi(governo)).toBe("governo");
    expect(statoDi(crisi)).toBe("crisi");
    expect(statoDi(opposizione)).toBe("opposizione");
    expect(statoDi(finita)).toBe("finita");
  });
  it("la crisi di un governo IA, col giocatore all'opposizione, resta opposizione", () => {
    expect(statoDi({ ...opposizione, crisi: true })).toBe("opposizione");
    expect(statoDi(crisi)).toBe("crisi");
  });
});

describe("barrata", () => {
  it("al turno zero: Prima pagina, Governo, Aula, Bilancio e Registro; Campagna accesa", () => {
    expect(barrate(turnoZero)).toEqual<Sezione[]>(["prima-pagina", "governo", "aula", "bilancio", "registro"]);
    expect(barrata("campagna", turnoZero)).toBeNull();
    expect(barrata("aula", turnoZero)).toBe("barrata.aula_sciolta");
  });

  it("all'opposizione: Governo, e Bilancio anche fuori stagione", () => {
    expect(barrata("governo", opposizione)).toBe("barrata.governo");
    expect(barrata("bilancio", opposizione)).toBe("barrata.bilancio_opposizione");
    expect(barrata("bilancio", { ...opposizione, data: { anno: 2027, trimestre: 4 } })).toBe("barrata.bilancio_opposizione");
  });

  it("al governo il Bilancio si accende in ottobre-dicembre", () => {
    expect(barrata("bilancio", governo)).toBe("barrata.bilancio_stagione");
    expect(barrata("bilancio", { ...governo, data: { anno: 2027, trimestre: 4 } })).toBeNull();
    expect(barrata("bilancio", (bilancioJson as unknown as { bozza_04: { vista: Vista } }).bozza_04.vista)).toBeNull();
  });

  it("in trattativa: Aula e Governo barrati", () => {
    expect(barrata("aula", trattativa)).toBe("barrata.aula_trattativa");
    expect(barrata("governo", trattativa)).toBe("barrata.governo");
  });

  it("a partita finita il Bilancio è barrato «Partita finita», prima di ogni altro motivo", () => {
    expect(barrata("bilancio", finita)).toBe("barrata.bilancio_finita");
    expect(barrata("bilancio", vista(ereditaJson.vista))).toBe("barrata.bilancio_finita");
  });

  it("a partita finita: Governo, Aula, Bilancio e Campagna; Registro senza bozza", () => {
    expect(barrate(finita)).toEqual<Sezione[]>(["governo", "aula", "bilancio", "campagna", "registro"]);
    expect(barrata("aula", finita)).toBe("barrata.aula_finita");
    expect(finita.eredita?.bozza).toBe(false);
    expect(barrata("registro", { ...finita, eredita: { ...finita.eredita!, bozza: true } })).toBeNull();
  });

  it("Conti, Paese e Partiti mai barrate", () => {
    for (const v of [turnoZero, trattativa, governo, crisi, opposizione, finita]) {
      expect(["conti", "paese", "partiti"].map((s) => barrata(s as Sezione, v))).toEqual([null, null, null]);
    }
  });
});

describe("casa", () => {
  it("cruscotto, trattativa o eredità", () => {
    expect(casa(turnoZero)).toBe("cruscotto");
    expect(casa(trattativa)).toBe("trattativa");
    expect(casa(governo)).toBe("cruscotto");
    expect(casa(finita)).toBe("eredita");
  });
});

describe("fettaDi", () => {
  it("il cruscotto cambia fetta con lo stato", () => {
    expect(fettaDi("cruscotto", "turno_zero")).toBeNull();
    expect(fettaDi("cruscotto", "trattativa")).toBe("11b");
    expect(fettaDi("cruscotto", "governo")).toBeNull();
    expect(fettaDi("cruscotto", "crisi")).toBeNull();
    expect(fettaDi("cruscotto", "opposizione")).toBeNull();
    expect(fettaDi("cruscotto", "finita")).toBeNull();
  });
  it("le altre sezioni hanno una fetta sola", () => {
    expect(fettaDi("registro", "governo")).toBe("11f");
    expect(fettaDi("bilancio", "governo")).toBeNull();
    expect(fettaDi("campagna", "turno_zero")).toBeNull();
    expect(fettaDi("paese", "governo")).toBeNull();
    expect(fettaDi("partiti", "governo")).toBeNull();
    expect(fettaDi("governo", "governo")).toBeNull();
    expect(fettaDi("aula", "governo")).toBeNull();
    expect(fettaDi("trattativa", "trattativa")).toBeNull();
    expect(fettaDi("partiti", "turno_zero")).toBeNull();
    expect(fettaDi("paese", "turno_zero")).toBeNull();
    expect(fettaDi("eredita", "finita")).toBeNull();
    expect(fettaDi("schema", "governo")).toBe("11e");
  });
  it("i conti sono costruiti in ogni stato", () => {
    for (const s of ["turno_zero", "trattativa", "governo", "crisi", "opposizione", "finita"] as const) expect(fettaDi("conti", s)).toBeNull();
  });
});

describe("etichetta", () => {
  it("ogni pagina ha l'etichetta del sommario", () => {
    expect(PAGINE.map(etichetta)).toEqual([
      "Prima pagina", "Cruscotto", "Governo", "Aula", "Conti", "Bilancio", "Paese", "Partiti", "Campagna", "Registro",
      "Trattativa", "Eredità", "Schema dello Stato",
    ]);
  });
});

describe("apribile", () => {
  it("una sezione barrata o un nome ignoto no", () => {
    expect(apribile("aula", turnoZero)).toBe(false);
    expect(apribile("pippo", governo)).toBe(false);
    expect(apribile("conti", turnoZero)).toBe(true);
    expect(apribile("schema", turnoZero)).toBe(true);
  });
  it("delle tre case solo quella dello stato", () => {
    expect(["cruscotto", "trattativa", "eredita"].filter((p) => apribile(p, governo))).toEqual(["cruscotto"]);
    expect(["cruscotto", "trattativa", "eredita"].filter((p) => apribile(p, trattativa))).toEqual(["trattativa"]);
    expect(["cruscotto", "trattativa", "eredita"].filter((p) => apribile(p, finita))).toEqual(["eredita"]);
  });
});

// Ogni Vista dei campioni, anche le annidate (17-cruscotto, 25-registro, …): l'oracolo è la tabella della spec
// (righe 278-290 e 300-311) letta sui campi grezzi, non barrata().
const campioni = import.meta.glob("../../../.scratch/v1-piano-11-ui/viste/*.json", { eager: true, import: "default" });
function trovaViste(o: unknown, percorso: string, fuori: [string, Vista][], profondita = 0) {
  if (!o || typeof o !== "object" || profondita > 5) return;
  if ("giocatore" in o && "turno" in o && "finita" in o) return void fuori.push([percorso, o as Vista]);
  for (const [k, x] of Object.entries(o)) trovaViste(x, `${percorso}/${k}`, fuori, profondita + 1);
}
const tutte: [string, Vista][] = [];
for (const [file, json] of Object.entries(campioni)) trovaViste(json, file.split("/").pop()!, tutte);

function barrateAttese(v: Vista): Sezione[] {
  const attese: Sezione[] = [];
  const zero = v.turno === 0;
  if (zero) attese.push("prima-pagina");
  if (!v.governa) attese.push("governo");
  if (v.finita || v.fase === "sciolta" || v.fase === "trattativa") attese.push("aula");
  if (v.finita || !v.governa || (v.data.trimestre !== 4 && !v.bilancio.presentabile)) attese.push("bilancio");
  if (v.finita || v.turni_al_voto !== 0) attese.push("campagna");
  if (v.finita ? !v.eredita?.bozza : zero) attese.push("registro");
  return SEZIONI.filter((s) => attese.includes(s));
}

describe("ogni Vista campione", () => {
  it("ne trova, anche annidate", () => {
    expect(tutte.length).toBeGreaterThan(100);
  });
  it.each(tutte)("%s: barrate come dice la spec", (_, v) => {
    expect(barrate(v)).toEqual(barrateAttese(v));
  });
  it.each(tutte)("%s: la casa è eredità a partita finita, trattativa in trattativa, altrimenti cruscotto", (_, v) => {
    expect(casa(v)).toBe(v.finita ? "eredita" : v.fase === "trattativa" ? "trattativa" : "cruscotto");
  });
});
