// Partiti e coalizione: le formule che prima stavano copiate nei tracciati.
import { statoDi } from "$lib/ossatura";
import type { Incrocio } from "$lib/tipi/Incrocio";
import type { Sondaggio } from "$lib/tipi/Sondaggio";
import type { Vista } from "$lib/tipi/Vista";

const OTTO = ["estrema_sinistra", "sinistra", "centrosinistra", "centro", "centrodestra", "destra", "estrema_destra", "giocatore"];
/** id → variabile CSS (fondamenta.css), per gli otto dello scenario. */
export const COLORI: Record<string, string> = Object.fromEntries(OTTO.map((id) => [id, `var(--p-${id})`]));

/** Il colore del partito, o null: un nato in partita va a retino grigio. */
export const coloreDi = (id: string): string | null => COLORI[id] ?? null;

/** Il giocatore: il nome del suo partito. */
export const nomeDi = (v: Vista, id: string): string => v.partiti.find((p) => p.id === id)?.nome ?? id;

/** quote[j] / (1 − astensione); l'astensione è l'ultima quota, quindi j deve stare prima. */
const fraVotanti = (quote: number[], j: number): number | null =>
  j >= 0 && j < quote.length - 1 ? quote[j] / (1 - quote[quote.length - 1]) : null;

/** Media sugli istituti di quote[j] / (1 − astensione); null se il partito non ha riga (docs/ricerca/modelli/sondaggi-ed-errore.md:55). */
export function quotaVotanti(s: Sondaggio, j: number): number | null {
  const q = s.istituti.map((r) => fraVotanti(r.quote, j));
  return q.length && q.every((x) => x !== null) ? q.reduce((a, x) => a! + x!, 0)! / q.length : null;
}

/** La stessa quota dentro un incrocio (una condizione, una regione). */
export const quotaIncrocio = (i: Incrocio, j: number): number | null => fraVotanti(i.quote, j);

/** {giocatore} ∪ coalizione al voto (turno zero), ∪ coalizione e richieste in trattativa, ∪ alleati altrove. */
export function coalizioneDi(v: Vista): string[] {
  const stato = statoDi(v);
  const altri = stato === "turno_zero" ? v.coalizione
    : stato === "trattativa" ? [...v.coalizione, ...v.richieste.map((r) => r.partito)]
    : v.alleati.map((a) => a.partito);
  return [...new Set([v.giocatore, ...altri])];
}

/** «Luca Benedetti» → «LB». */
export const iniziali = (nome: string): string =>
  nome.split(/\s+/).filter(Boolean).map((w) => w[0]).join("");
