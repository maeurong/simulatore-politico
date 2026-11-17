// L'anteprima della sfiducia nel dettaglio a lato: quali dicasteri si chiedono, in che ordine, e i conti sui seggi.
import { anteprimaPer } from "$lib/cornice/sfiducia";
import type { AnteprimaSfiducia } from "$lib/tipi/AnteprimaSfiducia";
import type { Vista } from "$lib/tipi/Vista";

/** null = il governo; null come risposta = guasto o rifiuto del guscio. */
export type Anteprime = Map<string | null, AnteprimaSfiducia | null>;

/** Un dicastero in interim non si sfiducia (`sfiducia_ammessa`, partita.rs:4391): non si chiede. */
export const dicasteriConMinistro = (v: Vista): string[] => v.ministeri.filter((m) => m.ministro).map((m) => m.dicastero);

/** Una chiamata alla volta, nell'ordine dato; se la Vista cambia a metà si ferma e non scrive più nulla. */
export async function caricaAnteprime(
  v: Vista, bersagli: (string | null)[], chiedi: (d: string | null) => Promise<AnteprimaSfiducia>,
  corrente: () => Vista | null, scrivi: (d: string | null, a: AnteprimaSfiducia | null) => void,
): Promise<void> {
  for (const d of bersagli) {
    const a = await anteprimaPer(v, () => chiedi(d), corrente);
    if (a === undefined) return;
    scrivi(d, a);
  }
}

/** Su quanti dei dicasteri chiesti la sfiducia passerebbe; null finché una risposta manca o è guasta. */
export function quantiPassano(anteprime: Anteprime, dicasteri: string[]): number | null {
  const risposte = dicasteri.map((d) => anteprime.get(d));
  return risposte.every((a) => a) ? risposte.filter((a) => a!.passa).length : null;
}

/** I sì dei partiti con la linea sì e la metà più uno dei seggi del ramo. */
export function conteggio(v: Vista, a: AnteprimaSfiducia, ramo: "camera" | "senato"): { si: number; serve: number } {
  const totale = v.seggi.reduce((s, x) => s + x[ramo], 0);
  const si = a.linee.filter((l) => l.si).reduce((s, l) => s + (v.seggi.find((x) => x.partito === l.partito)?.[ramo] ?? 0), 0);
  return { si, serve: Math.floor(totale / 2) + 1 };
}
