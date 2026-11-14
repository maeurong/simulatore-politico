// La sezione Paese: letture della Vista per la mappa, la lista e il sondaggio (la UI legge, il core decide).
import { ultimoVince } from "$lib/editor/editor.svelte";
import { testoRifiuto, type ErroreGuscio } from "$lib/guscio/chiamate";
import type { Segno } from "$lib/mappa/Mappa.svelte";
import { REGIONI } from "$lib/mappa/regioni";
import { quotaIncrocio, quotaVotanti } from "$lib/partiti";
import { t } from "$lib/testi/t";
import type { Condizione } from "$lib/tipi/Condizione";
import type { ElettoreCampione } from "$lib/tipi/ElettoreCampione";
import type { Fascia } from "$lib/tipi/Fascia";
import type { Sondaggio } from "$lib/tipi/Sondaggio";
import type { Vista } from "$lib/tipi/Vista";
import type { VoceDisposizione } from "$lib/tipi/VoceDisposizione";

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

/** Cantieri, atti in agenda ed emergenze aperte, per regione, col testo per esteso. Le opere finite arrivano in 11g. */
function vociSegni(v: Vista): { regione: string; segno: Segno; testo: string }[] {
  const fuori: { regione: string; segno: Segno; testo: string }[] = [];
  const aggiungi = (regione: string | null | undefined, segno: Segno, testo: string) => regione && fuori.push({ regione, segno, testo: `${t(`mappa.segno_${segno}`)}: ${testo}` });
  for (const m of v.ministeri) {
    if (m.agenda) aggiungi(m.agenda.regione, "atto", m.agenda.nome);
    for (const c of m.cantieri) aggiungi(c.regione, "cantiere", m.atti.find((a) => a.id === c.atto)?.nome ?? c.atto);
  }
  for (const e of v.eventi.filter((e) => e.famiglia === "emergenza")) {
    aggiungi(REGIONI.find((r) => e.regione && senzaAccento(r.nome) === senzaAccento(e.regione))?.id, "emergenza", `${e.nome} · ${e.causa}`);
  }
  return fuori;
}

/** Cantieri, atti in agenda ed emergenze aperte, per id di regione. */
export function segniPerRegione(v: Vista): Record<string, Segno[]> {
  const fuori: Record<string, Segno[]> = {};
  for (const x of vociSegni(v)) (fuori[x.regione] ??= []).push(x.segno);
  return fuori;
}

/** I segni di una regione col testo per esteso, per la scheda. */
export const segniDi = (v: Vista, regione: string) => vociSegni(v).filter((x) => x.regione === regione);

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

export const CONDIZIONI: readonly Condizione[] = ["dipendente_privato", "dipendente_pubblico", "autonomo", "pensionato", "disoccupato_inattivo", "studente"];
export const FASCE: readonly Fascia[] = ["bassa", "media", "alta"];

/** Le disposizioni territoriali che valgono nella regione: quelle col perimetro che la contiene. */
export const disposizioniDi = (v: Vista, regione: string): VoceDisposizione[] => v.disposizioni.filter((d) => d.perimetro.includes(regione));

/** Il sondaggio della regione: una quota per partito fra i votanti, dal sondaggio letto, dal più al meno votato. */
export function sondaggioDi(v: Vista, regione: string): { id: string; nome: string; quota: number }[] {
  const r = sondaggioLetto(v).sondaggio.regioni.find((x) => x.id === regione);
  if (!r) return [];
  return v.partiti
    .map((p, j) => ({ id: p.id, nome: p.nome, quota: quotaIncrocio(r, j), sciolto: p.sciolto }))
    .filter((p): p is typeof p & { quota: number } => p.quota !== null && !p.sciolto)
    .map(({ id, nome, quota }) => ({ id, nome, quota }))
    .sort((a, b) => b.quota - a.quota);
}

export type EsitoElettori = { elettori: ElettoreCampione[] } | { rifiuto: string } | { guasto: string };

/** Gli elettori della casella: una lettura in volo per volta, e si mostra solo l'ultima casella chiesta (o niente dopo `annulla`). */
export function sessioneElettori(
  leggi: (regione: string, condizione: Condizione, fascia: Fascia) => Promise<ElettoreCampione[]>,
  mostra: (esito: EsitoElettori) => void,
) {
  let ultima = "";
  const chiedi = ultimoVince(
    (a: { regione: string; condizione: Condizione; fascia: Fascia; chiave: string }) => leggi(a.regione, a.condizione, a.fascia),
    (r, a) => {
      if (a.chiave !== ultima) return;
      mostra(Array.isArray(r) ? { elettori: r } : (r as ErroreGuscio).specie === "rifiuto" ? { rifiuto: testoRifiuto(r as ErroreGuscio) } : { guasto: (r as ErroreGuscio).testo });
    },
  );
  return {
    chiedi(regione: string, condizione: Condizione, fascia: Fascia) {
      ultima = `${regione}|${condizione}|${fascia}`;
      chiedi({ regione, condizione, fascia, chiave: ultima });
    },
    annulla() {
      ultima = "";
    },
  };
}
