// L'ossatura dentro la partita: stato del giocatore, casa, sezioni barrate (ticket 07, spec:278-290).
import type { Vista } from "$lib/tipi/Vista";
import { t, type Chiave } from "$lib/testi/t";

export type StatoGiocatore = "turno_zero" | "trattativa" | "governo" | "crisi" | "opposizione" | "finita";
export const SEZIONI = ["prima-pagina", "cruscotto", "governo", "aula", "conti", "bilancio", "paese", "partiti", "campagna", "registro"] as const;
export type Sezione = (typeof SEZIONI)[number];
/** Ciò che /partita/[sezione] apre: le dieci sezioni, le due case che non sono il cruscotto, lo schema. */
export const PAGINE = [...SEZIONI, "trattativa", "eredita", "schema"] as const;
export type Pagina = (typeof PAGINE)[number];

/** Il nome nel sommario («Prima pagina», «Eredità», «Schema dello Stato»). */
export const etichetta = (p: Pagina) => t(`sommario.${p.replace("-", "_")}` as Chiave);

/** In quest'ordine: finita → turno === 0 → fase "trattativa" → crisi del proprio governo → governa → opposizione. */
export function statoDi(v: Vista): StatoGiocatore {
  if (v.finita) return "finita";
  if (v.turno === 0) return "turno_zero";
  if (v.fase === "trattativa") return "trattativa";
  // Vista.crisi è vera anche per la crisi di un governo IA: per chi sta all'opposizione resta opposizione
  if (v.crisi && v.governa) return "crisi";
  return v.governa ? "governo" : "opposizione";
}

export function casa(v: Vista): Sezione | "trattativa" | "eredita" {
  const s = statoDi(v);
  return s === "finita" ? "eredita" : s === "trattativa" ? "trattativa" : "cruscotto";
}

const CASE: readonly string[] = ["cruscotto", "trattativa", "eredita"];

/** Si apre: un nome noto, non barrato e, fra cruscotto, trattativa ed eredità, solo la casa di questo stato. */
export function apribile(p: string, v: Vista): p is Pagina {
  if (!(PAGINE as readonly string[]).includes(p)) return false;
  if (CASE.includes(p)) return p === casa(v);
  return p === "schema" || barrata(p as Sezione, v) === null;
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
    // aperta nel quarto trimestre e ogni volta che la Vista dice che il bilancio si può presentare (anche in esercizio provvisorio)
    case "bilancio":
      if (v.finita) return "barrata.bilancio_finita";
      if (!v.governa) return "barrata.bilancio_opposizione";
      return v.data.trimestre === 4 || v.bilancio.presentabile ? null : "barrata.bilancio_stagione";
    case "campagna": return !v.finita && v.turni_al_voto === 0 ? null : "barrata.campagna";
    default: return null;
  }
}

const FETTA: Record<Exclude<Sezione, "cruscotto" | "campagna" | "paese" | "partiti" | "governo" | "aula" | "conti" | "bilancio"> | "eredita" | "schema", string | null> = {
  "prima-pagina": "11f", registro: "11f",
  eredita: null, schema: "11e",
};
const CRUSCOTTO: Record<StatoGiocatore, string | null> = {
  turno_zero: null, trattativa: "11b", governo: null, crisi: null, opposizione: null, finita: null,
};

/** La fetta che costruisce la sezione in questo stato; null = la sezione è già costruita. */
export function fettaDi(s: Sezione | "trattativa" | "eredita" | "schema", stato: StatoGiocatore): string | null {
  return s === "cruscotto" ? CRUSCOTTO[stato] : s === "campagna" || s === "paese" || s === "partiti" || s === "governo" || s === "aula" || s === "conti" || s === "bilancio" || s === "trattativa" ? null : FETTA[s];
}
