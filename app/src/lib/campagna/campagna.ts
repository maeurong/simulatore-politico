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

export type EsitoAnteprima = { voci: VoceAnteprimaCampagna[] } | { rifiuto: string };

/**
 * La bozza delle spese in lettura 5: ogni modifica chiede l'anteprima (una in volo, vince l'ultima); all'uscita dal campo
 * il comando parte solo se l'anteprima dell'ultima bozza è riuscita, e se è ancora in volo parte quando arriva.
 */
export function sessioneSpese(
  anteprima: (spese: Spesa[]) => Promise<VoceAnteprimaCampagna[]>,
  manda: (spese: Spesa[]) => void,
  mostra: (esito: EsitoAnteprima) => void,
) {
  let giro = 0;
  let bozza: Spesa[] = [];
  let sporca = false;
  let riuscita: boolean | null = null;
  let alloArrivo = false;
  const chiedi = ultimoVince(
    (a: { spese: Spesa[]; giro: number }) => anteprima(a.spese),
    (r, a) => {
      if (a.giro !== giro) return;
      riuscita = Array.isArray(r);
      mostra(Array.isArray(r) ? { voci: r } : { rifiuto: testoRifiuto(comeErrore(r)) });
      if (alloArrivo && riuscita) manda(a.spese);
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
      chiedi({ spese, giro: ++giro });
    },
    esci() {
      if (!sporca) return;
      sporca = false;
      if (riuscita === null) alloArrivo = true;
      else if (riuscita) manda(bozza);
    },
  };
}
