// L'edicola (ticket 13, forma A · Banco): ordine, azioni, verso degli indicatori, punti dei grafici.
import { t, type Chiave } from "$lib/testi/t";
import type { Trimestre } from "$lib/tipi/Trimestre";
import type { VoceAlbo } from "$lib/tipi/VoceAlbo";
import type { VocePartita } from "$lib/tipi/VocePartita";

/** Per ultimo_salvataggio decrescente. */
export function inOrdine(p: VocePartita[]): VocePartita[] {
  return [...p].sort((a, b) => b.indice.ultimo_salvataggio - a.indice.ultimo_salvataggio);
}

/** "riprendi" | "eredita" (finita: «Apri l'eredità ›», senza «Abbandona») | "nessuna" (non apribile) */
export function azioneDi(p: VocePartita): "riprendi" | "eredita" | "nessuna" {
  if (!p.apribile) return "nessuna";
  return p.indice.testata.finita ? "eredita" : "riprendi";
}

const SALGONO_MALE = ["debito_pil", "spread", "disoccupazione"];
/** Il colore della freccia di un indicatore d'albo: debito_pil, spread e disoccupazione salgono male; inflazione senza colore. */
export function maleSe(id: string): "sale" | null {
  return SALGONO_MALE.includes(id) ? "sale" : null;
}

/** «aprile–giugno 2028», in minuscolo dentro la frase. */
export const periodo = (d: Trimestre) => t(`edicola.trimestre_${d.trimestre}` as Chiave, { anno: d.anno });

/** Gli anni al governo sono a trimestri: «5 anni e 3 mesi al governo». */
export function anniAlGoverno(a: number): string {
  if (!a) return t("edicola.mai_al_governo");
  // si arrotonda il totale in mesi, poi si divide: 1,99 anni sono «2 anni», non «1 anno e 12 mesi»
  const totale = Math.round(a * 12);
  const n = Math.floor(totale / 12);
  const m = totale % 12;
  const anni = n ? t("edicola.anni", { n }) : "";
  const mesi = m ? t("edicola.mesi", { n: m }) : "";
  const durata = anni && mesi ? t("edicola.anni_e_mesi", { anni, mesi }) : anni || mesi;
  return t("edicola.al_governo", { durata });
}

const ESITI: Record<string, Chiave> = { Abbandono: "edicola.esito_abbandono", "Senza seggi": "edicola.esito_senza_seggi" };
/** Il testo del bollo: i due esiti del formato v1 dai testi, un esito ignoto com'è. */
export const esitoTesto = (esito: string) => (Object.hasOwn(ESITI, esito) ? t(ESITI[esito]) : esito);

const quando = (d: Trimestre) => d.anno * 4 + d.trimestre;
/** La voce d'albo che l'abbandono di `nome` ha appena scritto: quella che in `prima` non c'era. La voce non ha un id,
 *  quindi il confronto è sul contenuto, contando i doppioni. Se le voci nuove non sono esattamente una, ricade sulla
 *  fine più recente fra quelle col suo nome. */
export function voceNuova(prima: VoceAlbo[], dopo: VoceAlbo[], nome: string): VoceAlbo | null {
  const conte = new Map<string, number>();
  for (const a of prima) conte.set(JSON.stringify(a), (conte.get(JSON.stringify(a)) ?? 0) + 1);
  const nuove = dopo.filter((a) => {
    const k = JSON.stringify(a);
    const c = conte.get(k) ?? 0;
    if (c) conte.set(k, c - 1);
    return !c;
  });
  if (nuove.length === 1) return nuove[0];
  return dopo.filter((a) => a.nome === nome).reduce<VoceAlbo | null>((m, a) => (m && quando(m.fine) >= quando(a.fine) ? m : a), null);
}

/** Le coordinate in [0, larghezza] × [0, altezza], il massimo in alto; con zero lo zero entra nell'intervallo.
 *  Senza punti o con un punto solo niente NaN: un intervallo nullo vale 1, un punto solo sta a sinistra. */
export function tracce(linee: number[][], larghezza: number, altezza: number, zero = false) {
  const tutti = linee.flat().concat(zero ? [0] : []);
  const lo = tutti.length ? Math.min(...tutti) : 0;
  const hi = tutti.length ? Math.max(...tutti) : 0;
  const ampiezza = hi - lo || 1;
  const n = Math.max(0, ...linee.map((l) => l.length));
  return {
    n,
    lo,
    hi,
    x: (i: number) => (n < 2 ? 0 : (larghezza * i) / (n - 1)),
    y: (v: number) => altezza * (1 - (v - lo) / ampiezza),
  };
}
