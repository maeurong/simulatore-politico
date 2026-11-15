// Campagna: i comandi della coalizione e le righe che la mostrano (ticket 26, storie 62-63).
import { comeErrore, testoRifiuto } from "$lib/guscio/chiamate";
import { ultimoVince } from "$lib/editor/editor.svelte";
import { idDi } from "$lib/mappa/mappa";
import { REGIONI } from "$lib/mappa/regioni";
import { nomeDi, quotaIncrocio } from "$lib/partiti";
import { t } from "$lib/testi/t";
import type { Bersaglio } from "$lib/tipi/Bersaglio";
import type { Cessione } from "$lib/tipi/Cessione";
import type { Comando } from "$lib/tipi/Comando";
import type { Leva } from "$lib/tipi/Leva";
import type { Spesa } from "$lib/tipi/Spesa";
import type { Vista } from "$lib/tipi/Vista";
import type { VocePromessa } from "$lib/tipi/VocePromessa";
import type { VoceAnteprimaCampagna } from "$lib/tipi/VoceAnteprimaCampagna";

export const PASSO_CESSIONE = 5;

/** Il comando che dichiara la coalizione di oggi con un partito in più o in meno; le cessioni degli usciti cadono. */
export function dichiara(v: Vista, cessioni: Cessione[], partito: string, entra: boolean): Comando {
  const partiti = entra ? [...new Set([...v.coalizione, partito])] : v.coalizione.filter((id) => id !== partito);
  return { tipo: "dichiara_coalizione", partiti, cessioni: cessioni.filter((c) => partiti.includes(c.partito)) };
}

/** Il comando con la cessione a `partito` portata a `seggi`; 0 la toglie. */
export function cedi(v: Vista, cessioni: Cessione[], partito: string, seggi: number): Comando {
  const altre = cessioni.filter((c) => c.partito !== partito);
  if (seggi <= 0) return { tipo: "dichiara_coalizione", partiti: v.coalizione, cessioni: altre };
  return {
    tipo: "dichiara_coalizione",
    partiti: [...new Set([...v.coalizione, partito])],
    cessioni: [...altre, { partito, seggi }],
  };
}

/** I seggi attesi del giocatore alla Camera sulla proiezione pubblica; null senza proiezione. */
export const seggiAttesi = (v: Vista): number | null => v.proiezione?.find((s) => s.partito === v.giocatore)?.camera ?? null;

/** Una riga per partito IA, nell'ordine di Vista.partiti: dichiarato, disponibile, causa. */
export function righeCoalizione(v: Vista) {
  return v.partiti
    .filter((p) => p.id !== v.giocatore)
    .map((p) => {
      const d = v.disponibilita.find((x) => x.partito === p.id);
      return { id: p.id, nome: nomeDi(v, p.id), dichiarato: v.coalizione.includes(p.id), disponibile: d?.disponibile ?? false, causa: d?.causa ?? "" };
    });
}

export const LEVE: readonly Leva[] = ["mobilitare", "persuadere", "convincere"];
export const MASSIMO_SPESE = 3;

/** Cassa del giocatore (già netta del sondaggio), prezzo del sondaggio se comprato in questo turno, somma delle spese, resto. */
export function conto(v: Vista, spese: Spesa[]): { cassa: number; sondaggio: number; spese: number; resto: number } {
  const cassa = v.partiti.find((p) => p.id === v.giocatore)?.cassa ?? 0;
  const sondaggio = v.acquisti.sondaggio_privato ? (v.acquisti.prezzi.find((p) => p.acquisto === "sondaggio_privato")?.euro ?? 0) : 0;
  const somma = spese.reduce((a, s) => a + s.euro, 0);
  return { cassa, sondaggio, spese: somma, resto: cassa - somma };
}

/** Cosa farebbe la chiusura con questa cassa: le spese si pagano in ordine finché la cassa basta (partita.rs:2301-2303). */
export function taglio(spese: Spesa[], cassa: number): { indice: number; paga: number; esito: "intera" | "scende" | "cade" }[] {
  let resto = cassa;
  return spese.map((s, indice) => {
    const paga = Math.max(0, Math.min(s.euro, resto));
    resto -= paga;
    return { indice, paga, esito: paga === s.euro ? "intera" : paga === 0 ? "cade" : "scende" };
  });
}

/** Il nome del bersaglio: la condizione in plurale, la regione dal suo id nella Vista. */
export const nomeBersaglio = (v: Vista, b: Bersaglio): string =>
  "regione" in b ? (REGIONI.find((r) => r.id === idDi(v, b.regione))?.nome ?? String(b.regione)) : t(`condizione.${b.condizione}`);

/** La quota del giocatore fra i votanti del bersaglio, sul sondaggio pubblico e sul privato se comprato. */
export function quotaNelBersaglio(v: Vista, b: Bersaglio): { pubblica: number | null; privata: number | null } {
  const j = v.partiti.findIndex((p) => p.id === v.giocatore);
  const quota = (s: Vista["sondaggio"] | null | undefined) => {
    const i = !s ? undefined : "regione" in b ? s.regioni[b.regione] : s.condizioni.find((c) => c.id === b.condizione);
    return i ? quotaIncrocio(i, j) : null;
  };
  return { pubblica: quota(v.sondaggio), privata: quota(v.acquisti.sondaggio_privato) };
}

/** Le casse dei partiti IA, dalla più ricca, e le loro promesse. */
export const rivali = (v: Vista): { id: string; nome: string; cassa: number; promesse: VocePromessa[] }[] =>
  v.partiti
    .filter((p) => p.id !== v.giocatore && !p.sciolto)
    .map((p) => ({ id: p.id, nome: p.nome, cassa: p.cassa, promesse: p.promesse }))
    .sort((a, b) => b.cassa - a.cassa);

/** La spesa nuova in coda alla bozza: persuadere, 100.000 €. Un bersaglio già preso entra lo stesso: il rifiuto è del core. */
export const aggiungiSpesa = (spese: Spesa[], bersaglio: Bersaglio): Spesa[] => [...spese, { bersaglio, leva: "persuadere", euro: 100_000 }];

export type EsitoAnteprima = { voci: VoceAnteprimaCampagna[] } | { rifiuto: string };

/**
 * La bozza delle spese in lettura 5: ogni modifica chiede l'anteprima (una in volo, vince l'ultima); all'uscita dal campo
 * il comando parte solo se l'anteprima dell'ultima bozza è riuscita, e se è ancora in volo parte quando arriva.
 */
export function sessioneSpese(
  anteprima: (spese: Spesa[]) => Promise<VoceAnteprimaCampagna[]>,
  /** `false`: il comando non è partito (la partita era occupata); il rifiuto del core o il guasto sono un esito, non un `false` */
  manda: (spese: Spesa[]) => Promise<boolean | void> | boolean | void,
  mostra: (esito: EsitoAnteprima) => void,
) {
  let giro = 0;
  let bozza: Spesa[] = [];
  let sporca = false;
  let riuscita: boolean | null = null;
  let alloArrivo = false;
  let pendente = false;
  const parti = (spese: Spesa[]) =>
    void Promise.resolve(manda(spese)).then((ok) => {
      if (ok === false && spese === bozza) sporca = pendente = true;
    });
  const chiedi = ultimoVince(
    (a: { spese: Spesa[]; giro: number }) => anteprima(a.spese),
    (r, a) => {
      if (a.giro !== giro) return;
      riuscita = Array.isArray(r);
      mostra(Array.isArray(r) ? { voci: r } : { rifiuto: testoRifiuto(comeErrore(r)) });
      if (alloArrivo && riuscita) parti(a.spese);
      alloArrivo = false;
    },
  );
  return {
    /** `modificata` falso: la prima lettura all'apertura, che non è una modifica e non fa partire il comando. */
    cambia(spese: Spesa[], modificata = true) {
      bozza = spese;
      sporca ||= modificata;
      riuscita = null;
      alloArrivo = false;
      pendente = false;
      chiedi({ spese, giro: ++giro });
    },
    esci() {
      if (!sporca) return;
      sporca = false;
      if (riuscita === null) alloArrivo = true;
      else if (riuscita) parti(bozza);
    },
    /** Da chiamare quando la partita smette di essere occupata: rimanda la bozza che il comando non ha portato. */
    riprova() {
      if (!pendente) return;
      pendente = false;
      this.esci();
    },
  };
}

export const PASSI = [-0.5, -0.25, 0.25, 0.5] as const;
export const MASSIMO_PROMESSE = 2;

/** Le quattro posizioni raggiungibili dalla dichiarata di oggi. */
export const posizioniVicine = (v: Vista): number[] => {
  const d = v.partiti.find((p) => p.id === v.giocatore)?.posizione_dichiarata ?? 0;
  return PASSI.map((p) => d + p);
};

/** Le promesse della campagna in corso: quelle del giocatore non in sospeso. */
export const promesseFatte = (v: Vista): VocePromessa[] => v.partiti.find((p) => p.id === v.giocatore)?.promesse.filter((p) => !p.in_sospeso) ?? [];

/** Il comando con le coppie (id della disposizione, id del gradino). */
export const prometti = (scelte: { disposizione: string; gradino: string }[]): Comando => ({
  tipo: "prometti",
  promesse: scelte.map((s) => [s.disposizione, s.gradino]),
});
