// La sezione Paese: letture della Vista per la mappa, la lista e il sondaggio (la UI legge, il core decide).
import type { Segno } from "$lib/mappa/Mappa.svelte";
import { REGIONI } from "$lib/mappa/regioni";
import { quotaIncrocio, quotaVotanti } from "$lib/partiti";
import type { Sondaggio } from "$lib/tipi/Sondaggio";
import type { Vista } from "$lib/tipi/Vista";

export const LIVELLI = ["consenso", "disoccupazione", "reddito", "affluenza", "pericolosita", "voto"] as const;
export type Livello = (typeof LIVELLI)[number];

/** null = il livello ha i dati; altrimenti la fetta che li porta («11g»). In 11b: consenso e affluenza. */
export const livelloManca = (l: Livello): string | null => (l === "consenso" || l === "affluenza" ? null : "11g");

/** Il sondaggio che la pagina legge: il privato se comprato, altrimenti il pubblico. */
export function sondaggioLetto(v: Vista): { sondaggio: Sondaggio; privato: boolean } {
  const privato = v.acquisti.sondaggio_privato;
  return privato ? { sondaggio: privato, privato: true } : { sondaggio: v.sondaggio, privato: false };
}

/** Consenso per regione di una coalizione o di un partito solo: somma delle quote fra i votanti. La mappa legge il sondaggio pubblico (il privato è per incroci e scheda). */
export function consensoPerRegione(v: Vista, chi: string[]): Record<string, number | null> {
  const posti = chi.map((id) => v.partiti.findIndex((p) => p.id === id));
  return Object.fromEntries(
    v.sondaggio.regioni.map((r) => {
      // un partito senza riga nel sondaggio (nato in partita) non azzera gli altri
      const quote = posti.map((j) => quotaIncrocio(r, j)).filter((q): q is number => q !== null);
      return [r.id, quote.length ? quote.reduce((a, q) => a + q, 0) : null];
    }),
  );
}

/** Uno meno l'astensione del sondaggio della regione. */
export function affluenzaPerRegione(v: Vista): Record<string, number | null> {
  return Object.fromEntries(v.sondaggio.regioni.map((r) => [r.id, r.quote.length ? 1 - r.quote[r.quote.length - 1] : null]));
}

const senzaAccento = (n: string) => n.replace("’", "'");

/** Cantieri, atti in agenda ed emergenze aperte, per id di regione. Le opere finite arrivano in 11g. */
export function segniPerRegione(v: Vista): Record<string, Segno[]> {
  const fuori: Record<string, Segno[]> = {};
  const aggiungi = (id: string | null | undefined, s: Segno) => id && (fuori[id] ??= []).push(s);
  for (const m of v.ministeri) {
    aggiungi(m.agenda?.regione, "atto");
    for (const c of m.cantieri) aggiungi(c.regione, "cantiere");
  }
  for (const e of v.eventi.filter((e) => e.famiglia === "emergenza")) aggiungi(REGIONI.find((r) => e.regione && senzaAccento(r.nome) === senzaAccento(e.regione))?.id, "emergenza");
  return fuori;
}

/** La lista delle venti regioni, per nome o per valore (decrescente, i senza dato in fondo). */
export function lista(valori: Record<string, number | null>, ordine: "nome" | "valore"): { id: string; nome: string; valore: number | null }[] {
  const righe = REGIONI.map((r) => ({ id: r.id, nome: r.nome, valore: valori[r.id] ?? null }));
  return righe.sort((a, b) =>
    ordine === "nome" ? a.nome.localeCompare(b.nome, "it") : a.valore === b.valore ? 0 : a.valore === null ? 1 : b.valore === null ? -1 : b.valore - a.valore,
  );
}

/** La media degli istituti: una quota sui votanti per partito (dal più al meno votato) e l'astensione a parte. */
export function media(v: Vista, s: Sondaggio): { partiti: { id: string; nome: string; quota: number }[]; astensione: number } {
  const partiti = v.partiti
    .map((p, j) => ({ id: p.id, nome: p.nome, quota: quotaVotanti(s, j), sciolto: p.sciolto }))
    .filter((p): p is typeof p & { quota: number } => p.quota !== null && !p.sciolto)
    .map(({ id, nome, quota }) => ({ id, nome, quota }))
    .sort((a, b) => b.quota - a.quota);
  const astensione = s.istituti.length ? s.istituti.reduce((a, r) => a + r.quote[r.quote.length - 1], 0) / s.istituti.length : 0;
  return { partiti, astensione };
}
