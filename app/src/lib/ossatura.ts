// L'ossatura dentro la partita: stato del giocatore, casa, sezioni barrate (ticket 07, spec:278-290).
import type { Vista } from "$lib/tipi/Vista";
import type { Chiave } from "$lib/testi/t";

export type StatoGiocatore = "turno_zero" | "trattativa" | "governo" | "crisi" | "opposizione" | "finita";
export const SEZIONI = ["prima-pagina", "cruscotto", "governo", "aula", "conti", "bilancio", "paese", "partiti", "campagna", "registro"] as const;
export type Sezione = (typeof SEZIONI)[number];

/** In quest'ordine: finita → turno === 0 → fase "trattativa" → crisi → governa → opposizione. */
export function statoDi(v: Vista): StatoGiocatore {
  if (v.finita) return "finita";
  if (v.turno === 0) return "turno_zero";
  if (v.fase === "trattativa") return "trattativa";
  if (v.crisi) return "crisi";
  return v.governa ? "governo" : "opposizione";
}

export function casa(v: Vista): Sezione | "trattativa" | "eredita" {
  const s = statoDi(v);
  return s === "finita" ? "eredita" : s === "trattativa" ? "trattativa" : "cruscotto";
}

/** null = accesa; altrimenti il motivo della barrata (07-ossatura.prototype.html:235). */
export function barrata(s: Sezione, v: Vista): Chiave | null {
  const zero = v.turno === 0;
  switch (s) {
    case "prima-pagina": return zero ? "barrata.prima_pagina" : null;
    case "registro":
      if (v.finita) return v.eredita?.bozza ? null : "barrata.registro_finita";
      return zero ? "barrata.registro" : null;
    case "aula":
      if (v.finita) return "barrata.aula_finita";
      if (v.fase === "sciolta") return "barrata.aula_sciolta";
      return v.fase === "trattativa" ? "barrata.aula_trattativa" : null;
    case "governo": return v.governa ? null : "barrata.governo";
    case "bilancio": return !v.finita && v.governa && v.data.trimestre === 4 ? null : "barrata.bilancio";
    case "campagna": return !v.finita && v.turni_al_voto === 0 ? null : "barrata.campagna";
    default: return null;
  }
}

const FETTA: Record<Exclude<Sezione, "cruscotto"> | "trattativa" | "eredita" | "schema", string> = {
  "prima-pagina": "11f", registro: "11f",
  governo: "11c", aula: "11c",
  conti: "11d", bilancio: "11d",
  paese: "11b", partiti: "11b", campagna: "11b", trattativa: "11b",
  eredita: "11e", schema: "11e",
};
const CRUSCOTTO: Record<StatoGiocatore, string> = {
  turno_zero: "11b", trattativa: "11b", governo: "11c", crisi: "11c", opposizione: "11e", finita: "11e",
};

/** La fetta che costruisce la sezione in questo stato; null se è costruita in 11a. */
export function fettaDi(s: Sezione | "trattativa" | "eredita" | "schema", stato: StatoGiocatore): string | null {
  return s === "cruscotto" ? CRUSCOTTO[stato] : FETTA[s];
}
