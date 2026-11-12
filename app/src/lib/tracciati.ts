// I tre tracciati della cornice: scale e stati dal ticket 31 (.scratch/v1-piano-11-ui/issues/31-scale-dei-tracciati.md).
import type { Vista } from "$lib/tipi/Vista";
import type { PuntoSerie } from "$lib/tipi/PuntoSerie";
import type { SeggiPartito } from "$lib/tipi/SeggiPartito";
import { numero, percentuale, segnato, type Chiave } from "$lib/testi/t";

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
  rimando: string | null; // fetta che riempie un tracciato senza dato
  prima: number | null; // il valore al turno prima, dalla serie: la variazione sotto il numero
};

const MARGINE: Scala = { stacco: 0, passo: 3, rovesciata: false };
// ponytail: lo stacco dello spread non ha dato fino a 11g (ticket 31, rimando al core): 550 serve solo alle caselle
const SPREAD: Scala = { stacco: 550, passo: 75, rovesciata: true };

const disposizione = (v: Vista, id: string) => v.disposizioni.find((d) => d.id === id)?.in_vigore;
/** In centesimi di punto: "4200" → 42. */
const punti = (v: Vista, id: string) => Number(disposizione(v, id)) / 100;

/** quota_fra_i_votanti: media sugli istituti di quote[j] / (1 − astensione). */
function quota(v: Vista, partito: string): number {
  const j = v.partiti.findIndex((p) => p.id === partito);
  const istituti = v.sondaggio.istituti;
  return istituti.reduce((s, r) => s + r.quote[j] / (1 - r.quote[r.quote.length - 1]), 0) / istituti.length;
}

/** Seggi della coalizione meno la metà più uno del ramo (voto.rs:106); null se il ramo non ha seggi. */
function margini(righe: SeggiPartito[], coalizione: string[]) {
  const ramo = (r: "camera" | "senato") => {
    const totale = righe.reduce((s, x) => s + x[r], 0);
    const nostri = righe.filter((x) => coalizione.includes(x.partito)).reduce((s, x) => s + x[r], 0);
    return totale === 0 ? null : nostri - (Math.floor(totale / 2) + 1);
  };
  return { camera: ramo("camera"), senato: ramo("senato") };
}

const sottoLoStacco = (v: number, s: Scala) => caselle(v, s).piene <= 3;

function maggioranza(nome: Chiave, righe: SeggiPartito[], coalizione: string[], prima: PuntoSerie | undefined): Tracciato {
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

export function tracciatiDi(v: Vista, serie: PuntoSerie[]): [Tracciato, Tracciato, Tracciato] {
  const prima = serie.find((p) => p.turno === v.turno - 1);
  // a partita finita valgono le regole dell'ultimo stato, nell'ordine di statoDi (piano 11a, Task 10)
  const turnoZero = v.turno === 0;
  const alGoverno = !turnoZero && (v.fase === "trattativa" || v.crisi || v.governa);
  const conDichiarata = turnoZero || v.fase === "trattativa";
  const coalizione = [v.giocatore, ...(conDichiarata ? v.coalizione : v.alleati.map((a) => a.partito))];

  let primo: Tracciato;
  if (alGoverno) {
    const stacco = disposizione(v, "premio") === "nessuno" ? 50 : punti(v, "soglia_premio");
    const scala = { stacco, passo: 4, rovesciata: false };
    const q = coalizione.reduce((s, p) => s + quota(v, p), 0);
    const p = v.proiezione && margini(v.proiezione, coalizione);
    primo = {
      nome: "tracciato.consenso_coalizione", valore: q * 100, testo: percentuale(q), ramo: null, scala,
      rosso: !!p && ((p.camera ?? 0) < 0 || (p.senato ?? 0) < 0), grigio: false, rimando: null,
      prima: prima ? prima.consenso * 100 : null,
    };
  } else {
    const scala = { stacco: punti(v, "soglia_lista"), passo: 1, rovesciata: false };
    const q = quota(v, v.giocatore);
    primo = {
      nome: "tracciato.consenso_partito", valore: q * 100, testo: percentuale(q), ramo: null, scala,
      rosso: sottoLoStacco(q * 100, scala), grigio: false, rimando: null, prima: prima ? prima.consenso * 100 : null,
    };
  }

  const secondo: Tracciato = turnoZero
    ? maggioranza("tracciato.seggi_attesi", v.proiezione ?? [], coalizione, prima)
    : alGoverno
      ? maggioranza("tracciato.maggioranza", v.seggi, coalizione, prima)
      : { nome: "tracciato.per_farlo_cadere", valore: null, testo: "—", ramo: null, scala: null, rosso: false, grigio: false, rimando: "11e", prima: null };

  const terzo: Tracciato = {
    nome: "tracciato.conti", valore: v.spread_pb, testo: numero(v.spread_pb), ramo: null, scala: SPREAD,
    rosso: false, grigio: false, rimando: null, prima: prima ? prima.spread_pb : null,
  };

  const tre: [Tracciato, Tracciato, Tracciato] = [primo, secondo, terzo];
  return v.finita ? (tre.map((x) => ({ ...x, rosso: false, grigio: true, prima: null })) as typeof tre) : tre;
}
