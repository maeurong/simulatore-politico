// La sezione Paese: letture della Vista per la mappa, la lista e il sondaggio (la UI legge, il core decide).
import { periodoDi } from "$lib/aula/aula";
import { ultimoVince } from "$lib/editor/editor.svelte";
import { testoRifiuto, type ErroreGuscio } from "$lib/guscio/chiamate";
import type { Segno } from "$lib/mappa/Mappa.svelte";
import { REGIONI } from "$lib/mappa/regioni";
import { quotaIncrocio, quotaVotanti } from "$lib/partiti";
import { numero, t, type Chiave } from "$lib/testi/t";
import type { Condizione } from "$lib/tipi/Condizione";
import type { ElettoreCampione } from "$lib/tipi/ElettoreCampione";
import type { Fascia } from "$lib/tipi/Fascia";
import type { Incrocio } from "$lib/tipi/Incrocio";
import type { Sondaggio } from "$lib/tipi/Sondaggio";
import type { Vista } from "$lib/tipi/Vista";
import type { VoceDisposizione } from "$lib/tipi/VoceDisposizione";

export const LIVELLI = ["consenso", "disoccupazione", "reddito", "affluenza", "pericolosita", "voto"] as const;
export type Livello = (typeof LIVELLI)[number];

/** null = il livello ha i dati; altrimenti la chiave del perché manca (solo il voto, prima del primo voto). */
export const livelloManca = (l: Livello, v: Vista): Chiave | null => (l === "voto" && !v.elezioni.length ? "paese.voto_nessuno" : null);

export type Pericolo = "totale" | "sisma" | "idro" | "incendi";

/** Il valore di un livello per id di regione. */
export function valoriDi(v: Vista, l: "disoccupazione" | "reddito" | "pericolosita", tipo: Pericolo = "totale"): Record<string, number> {
  return Object.fromEntries(v.regioni.map((r) => [r.id, l === "pericolosita" ? r.pericolosita[tipo] : r[l]]));
}

/** Cinque classi: la pericolosità a passi fissi di 0,2, gli altri livelli a passi uguali fra minimo e massimo (minimo = massimo: una classe sola). */
export function classi(valori: number[], l: Livello): { soglie: number[]; classe: (x: number) => 0 | 1 | 2 | 3 | 4 } {
  const min = Math.min(...valori);
  const max = Math.max(...valori);
  const soglie = l === "pericolosita" ? [0.2, 0.4, 0.6, 0.8] : max > min ? [1, 2, 3, 4].map((k) => min + ((max - min) * k) / 5) : [];
  return { soglie, classe: (x) => soglie.filter((s) => x >= s).length as 0 | 1 | 2 | 3 | 4 };
}

/** Il voto dell'ultima elezione per id di regione; null senza elezioni. */
export function votoPerRegione(v: Vista): Record<string, { primo: string; voti: number[]; senato: number[] }> | null {
  const e = v.elezioni.at(-1);
  return e ? Object.fromEntries(e.regioni.map((r) => [r.regione, { primo: r.primo, voti: r.voti, senato: r.seggi_senato }])) : null;
}

const terzo = (x: number): Fascia => (x < 1 / 3 ? "bassa" : x < 2 / 3 ? "media" : "alta");

/** La scheda del territorio: macroarea, disoccupazione, reddito e la pericolosità per tipo in terzi. */
export function territorioDi(v: Vista, regione: string) {
  const r = v.regioni.find((x) => x.id === regione);
  if (!r) return null;
  const { sisma, idro, incendi, totale } = r.pericolosita;
  return { macroarea: r.macroarea, disoccupazione: r.disoccupazione, reddito: r.reddito, pericolosita: { totale: terzo(totale), sisma: terzo(sisma), idro: terzo(idro), incendi: terzo(incendi) } as Record<Pericolo, Fascia> };
}

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

/** «± 2,8 punti»: il margine dichiarato (una frazione) in punti percentuali. */
export const margineTesto = (m: number): string => t("paese.margine_punti", { m: numero(m * 100, 1) });

/** Il margine della media degli istituti: la media dei loro margini dichiarati. */
export const margineMedio = (s: Sondaggio): number => (s.istituti.length ? s.istituti.reduce((a, r) => a + r.margine, 0) / s.istituti.length : 0);

const senzaAccento = (n: string) => n.replace("’", "'");

/** Cantieri, atti in agenda ed emergenze aperte e opere finite, per regione, col testo per esteso. */
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
  for (const o of v.opere) aggiungi(o.regione, "opera", `${o.atto} · ${trimestreDelTurno(v, o.turno)}`);
  return fuori;
}

/** Il trimestre di un turno passato: il turno corrente è `v.data`, un turno è un trimestre. */
function trimestreDelTurno(v: Vista, turno: number): string {
  const n = v.data.anno * 4 + v.data.trimestre - 1 - (v.turno - turno);
  return periodoDi(v, { anno: Math.floor(n / 4), trimestre: (n % 4) + 1 });
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

export type QuotaPartito = { id: string; nome: string; quota: number };

/** Una quota per partito attivo, dal più al meno votato. Un partito senza riga nel sondaggio (nato dopo) non c'è e non azzera gli altri. */
function quotePerPartito(v: Vista, leggi: (j: number) => number | null): QuotaPartito[] {
  return v.partiti
    .map((p, j) => ({ id: p.id, nome: p.nome, quota: leggi(j), sciolto: p.sciolto }))
    .filter((p): p is typeof p & { quota: number } => p.quota !== null && !p.sciolto)
    .map(({ id, nome, quota }) => ({ id, nome, quota }))
    .sort((a, b) => b.quota - a.quota);
}

/** La media degli istituti: una quota sui votanti per partito (dal più al meno votato) e l'astensione a parte. */
export function media(v: Vista, s: Sondaggio): { partiti: QuotaPartito[]; astensione: number } {
  const partiti = quotePerPartito(v, (j) => quotaVotanti(s, j));
  const astensione = s.istituti.length ? s.istituti.reduce((a, r) => a + r.quote[r.quote.length - 1], 0) / s.istituti.length : 0;
  return { partiti, astensione };
}

export const CONDIZIONI: readonly Condizione[] = ["dipendente_privato", "dipendente_pubblico", "autonomo", "pensionato", "disoccupato_inattivo", "studente"];
export const FASCE: readonly Fascia[] = ["bassa", "media", "alta"];

/** Le disposizioni territoriali che valgono nella regione: quelle col perimetro che la contiene. */
export const disposizioniDi = (v: Vista, regione: string): VoceDisposizione[] => v.disposizioni.filter((d) => d.perimetro.includes(regione));

/** Il sondaggio della regione: una quota per partito fra i votanti, dal sondaggio letto, dal più al meno votato. */
export function sondaggioDi(v: Vista, regione: string): QuotaPartito[] {
  const r = sondaggioLetto(v).sondaggio.regioni.find((x) => x.id === regione);
  return r ? quoteDi(v, r) : [];
}

/** Le quote di un incrocio (una regione, una condizione), come nel sondaggio della regione. */
export const quoteDi = (v: Vista, i: Incrocio): QuotaPartito[] => quotePerPartito(v, (j) => quotaIncrocio(i, j));

export type EsitoElettori = { elettori: ElettoreCampione[] } | { rifiuto: string } | { guasto: string };

/** Gli elettori della casella: una lettura in volo per volta, e si mostra solo l'ultima casella chiesta (o niente dopo `annulla`). Il turno è nella chiave: il core estrae a ogni turno elettori nuovi. */
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
    chiedi(regione: string, condizione: Condizione, fascia: Fascia, turno: number) {
      ultima = `${turno}|${regione}|${condizione}|${fascia}`;
      chiedi({ regione, condizione, fascia, chiave: ultima });
    },
    annulla() {
      ultima = "";
    },
  };
}
