// I tre tracciati della cornice: scale e stati dal ticket 31 (.scratch/v1-piano-11-ui/issues/31-scale-dei-tracciati.md).
import type { Vista } from "$lib/tipi/Vista";
import type { PuntoSerie } from "$lib/tipi/PuntoSerie";
import type { SeggiPartito } from "$lib/tipi/SeggiPartito";
import type { AnteprimaSfiducia } from "$lib/tipi/AnteprimaSfiducia";
import { siMancanti } from "$lib/opposizione/opposizione";
import { numero, percentuale, segnato, type Chiave } from "$lib/testi/t";
import { statoDi } from "$lib/ossatura";
import { coalizioneDi, quotaVotanti } from "$lib/partiti";
import { ordineRivelazione, scattaDopo } from "$lib/esito/esito";
import type { rivelazione } from "$lib/esito/rivelazione.svelte";

export type Scala = { stacco: number; passo: number; rovesciata: boolean };

/** Casella i copre [da+i·passo, da+(i+1)·passo), con da = stacco − 3·passo (rovesciata: da = stacco + 3·passo). */
export function caselle(v: number, s: Scala): { piene: number; sotto: boolean; sopra: boolean } {
  const grezze = s.rovesciata
    ? Math.ceil((s.stacco + 3 * s.passo - v) / s.passo)
    : Math.floor((v - (s.stacco - 3 * s.passo)) / s.passo) + 1;
  return { piene: Math.max(0, Math.min(10, grezze)), sotto: grezze < 1, sopra: grezze > 10 };
}

export type Tracciato = {
  nome: Chiave;
  valore: number | null; // null → caselle tratteggiate
  testo: string; // il numero da mostrare ("19,2%", "+14", "86")
  ramo: "Camera" | "Senato" | null; // solo per maggioranza e seggi attesi
  scala: Scala | null; // null → nessuna casella da riempire
  rosso: boolean;
  grigio: boolean; // partita finita
  rimando: string | null; // la fetta che riempie il posto vuoto: nessun tracciato ne ha più una
  prima: number | null; // il valore al turno prima, dalla serie: la variazione sotto il numero
  era: string | null; // durante la rivelazione, dopo lo scatto: il testo del valore di prima
  scatta: boolean; // durante la rivelazione: il valore è appena cambiato
};
type Base = Omit<Tracciato, "era" | "scatta">;

export const MARGINE: Scala = { stacco: 0, passo: 3, rovesciata: false };
// «per farlo cadere»: meno sì mancano, più caselle; 15 seggi a casella, vuota da 150 in su
const MANCANTI: Scala = { stacco: 105, passo: 15, rovesciata: true };
// lo stacco dello spread è la soglia della crisi del debito, letta dalla Vista
const spread = (v: Vista): Scala => ({ stacco: v.soglie.crisi_debito_pb, passo: 75, rovesciata: true });

const disposizione = (v: Vista, id: string) => v.disposizioni.find((d) => d.id === id)?.in_vigore;
/** In centesimi di punto: "4200" → 42. */
const punti = (v: Vista, id: string) => Number(disposizione(v, id)) / 100;

/** NaN senza istituti o col partito fuori da `partiti`: il consenso lo legge come dato mancante. */
function quota(v: Vista, partito: string): number {
  return quotaVotanti(v.sondaggio, v.partiti.findIndex((p) => p.id === partito)) ?? NaN;
}

/** Il consenso: senza dato nel sondaggio, caselle tratteggiate e nessun rimando (manca il dato, non una fetta). */
function consenso(nome: Chiave, q: number, scala: Scala, rosso: boolean, prima: PuntoSerie | undefined): Base {
  const c = { nome, ramo: null, scala, grigio: false, rimando: null, prima: prima ? prima.consenso * 100 : null };
  return Number.isFinite(q) ? { ...c, valore: q * 100, testo: percentuale(q), rosso } : { ...c, valore: null, testo: "—", rosso: false };
}

/** Seggi della coalizione meno la metà più uno del ramo (voto.rs:106); null se il ramo non ha seggi. */
export function margini(righe: SeggiPartito[], coalizione: string[]) {
  const ramo = (r: "camera" | "senato") => {
    const totale = righe.reduce((s, x) => s + x[r], 0);
    const nostri = righe.filter((x) => coalizione.includes(x.partito)).reduce((s, x) => s + x[r], 0);
    return totale === 0 ? null : nostri - (Math.floor(totale / 2) + 1);
  };
  return { camera: ramo("camera"), senato: ramo("senato") };
}

export const sottoLoStacco = (v: number, s: Scala) => caselle(v, s).piene <= 3;

function maggioranza(nome: Chiave, righe: SeggiPartito[], coalizione: string[], prima: PuntoSerie | undefined): Base {
  const { camera, senato } = margini(righe, coalizione);
  if (camera === null || senato === null) {
    return { nome, valore: null, testo: "—", ramo: null, scala: null, rosso: false, grigio: false, rimando: null, prima: null };
  }
  const ramo = camera <= senato ? "Camera" : "Senato";
  const valore = Math.min(camera, senato);
  return {
    nome, valore, testo: segnato(valore), ramo, scala: MARGINE, rosso: sottoLoStacco(valore, MARGINE), grigio: false, rimando: null,
    prima: prima ? (ramo === "Camera" ? prima.margine_camera : prima.margine_senato) : null,
  };
}

export function tracciatiDi(v: Vista, serie: PuntoSerie[], sfiducia?: AnteprimaSfiducia | null): [Tracciato, Tracciato, Tracciato] {
  const prima = serie.find((p) => p.turno === v.turno - 1);
  // a partita finita valgono le regole dell'ultimo stato, che si legge dagli alleati: governa è sempre falso
  const stato = v.finita ? (v.alleati.length ? "governo" : "opposizione") : statoDi(v);
  const turnoZero = stato === "turno_zero";
  const alGoverno = stato === "trattativa" || stato === "crisi" || stato === "governo";
  const coalizione = coalizioneDi(v);

  // la serie è della coalizione: senza incarico, in trattativa, la coalizione qui è il solo giocatore e non si confronta
  const primaCoalizione = stato === "trattativa" && v.richieste.length === 0 ? undefined : prima;
  let primo: Base;
  if (alGoverno) {
    const stacco = disposizione(v, "premio") === "nessuno" ? 50 : punti(v, "soglia_premio");
    const p = v.proiezione && margini(v.proiezione, coalizione);
    primo = consenso("tracciato.consenso_coalizione", coalizione.reduce((s, x) => s + quota(v, x), 0),
      { stacco, passo: 4, rovesciata: false }, !!p && ((p.camera ?? 0) < 0 || (p.senato ?? 0) < 0), primaCoalizione);
  } else {
    const scala = { stacco: punti(v, "soglia_lista"), passo: 1, rovesciata: false };
    const q = quota(v, v.giocatore);
    // il partito da solo non è la coalizione della serie: nessuna variazione
    primo = consenso("tracciato.consenso_partito", q, scala, sottoLoStacco(q * 100, scala), undefined);
  }

  const secondo: Base = turnoZero
    ? maggioranza("tracciato.seggi_attesi", v.proiezione ?? [], coalizione, prima)
    : alGoverno
      ? maggioranza("tracciato.maggioranza", v.seggi, coalizione, primaCoalizione)
      : sfiducia
        ? { nome: "tracciato.per_farlo_cadere", valore: siMancanti(v, sfiducia), testo: numero(siMancanti(v, sfiducia)), ramo: null, scala: MANCANTI, rosso: false, grigio: false, rimando: null, prima: null }
        : { nome: "tracciato.per_farlo_cadere", valore: null, testo: "—", ramo: null, scala: null, rosso: false, grigio: false, rimando: null, prima: null };

  const terzo: Base = {
    nome: "tracciato.conti", valore: v.spread_pb, testo: numero(v.spread_pb), ramo: null, scala: spread(v),
    rosso: false, grigio: false, rimando: null, prima: prima ? prima.spread_pb : null,
  };

  const tre = [primo, secondo, terzo].map((x) => ({ ...x, era: null, scatta: false })) as [Tracciato, Tracciato, Tracciato];
  return v.finita ? (tre.map((x) => ({ ...x, rosso: false, grigio: true, prima: null })) as typeof tre) : tre;
}

const MOSSI = ["consenso", "margine", "spread"] as const;

/**
 * I tre tracciati durante la rivelazione (piano 11f, lettura 6): ciascuno resta quello di `prima` finché la voce
 * che lo muove non è passata, poi è quello di `v` con `era`. Quello che nessuna voce muove scatta alla fine.
 */
export function tracciatiRivelati(v: Vista, serie: PuntoSerie[], r: typeof rivelazione, sfiducia?: AnteprimaSfiducia | null): [Tracciato, Tracciato, Tracciato] {
  const nuovi = tracciatiDi(v, serie, sfiducia);
  if (!r.prima) return nuovi;
  // l'anteprima della sfiducia è della Vista nuova: sulla Vista di prima il tracciato è senza dato, e si mostra subito quello nuovo
  const vecchi = tracciatiDi(r.prima, r.serie_prima);
  const evidenza = ordineRivelazione(v.esito);
  const passato = (tr: (typeof MOSSI)[number]) => {
    if (r.ferma) return true;
    const i = scattaDopo(v.esito, tr);
    return i !== null && r.passo >= Math.max(1, evidenza.filter((k) => k <= i).length);
  };
  // una grandezza diversa (turno zero → governo, governo ↔ opposizione) o senza valore di prima non si confronta: niente «era», niente lampo
  return nuovi.map((n, i) =>
    !passato(MOSSI[i]) && vecchi[i].valore !== null ? vecchi[i]
    : vecchi[i].nome !== n.nome || vecchi[i].valore === null ? n
    : { ...n, era: vecchi[i].testo, scatta: n.testo !== vecchi[i].testo },
  ) as [Tracciato, Tracciato, Tracciato];
}
