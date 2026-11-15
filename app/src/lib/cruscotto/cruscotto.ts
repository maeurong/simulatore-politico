// Cruscotto del turno zero e riquadro dell'evento: le sole cifre che la UI calcola sono conteggi di voci della Vista.
import { nomeDi } from "$lib/partiti";
import { statoDi } from "$lib/ossatura";
import type { Vista } from "$lib/tipi/Vista";
import type { VoceEvento } from "$lib/tipi/VoceEvento";
import type { VoceOpzione } from "$lib/tipi/VoceOpzione";

/** Una sola opzione: niente da scegliere, la chiusura la prende. */
export const opzioneUnica = (e: VoceEvento): boolean => e.opzioni.length === 1;

/** L'opzione col campo default. */
export const defaultDi = (e: VoceEvento): VoceOpzione | null => e.opzioni.find((o) => o.default) ?? null;

/** Gli eventi con più di un'opzione e senza scelta: le scelte che la chiusura lascia al default (decisione 4). */
export const scelteAlDefault = (v: Vista): number => v.eventi.filter((e) => !opzioneUnica(e) && e.scelta === null).length;

/** Chi può rispondere: il giocatore che governa, fuori dalla trattativa. */
export function risponde(v: Vista): boolean {
  const s = statoDi(v);
  return s === "governo" || s === "crisi";
}

/** Le due righe del turno del voto: lo stato della coalizione e della campagna, in parole. */
export function righeDelVoto(v: Vista) {
  return {
    coalizione: {
      dichiarati: v.coalizione.map((id) => nomeDi(v, id)),
      disponibili: v.disponibilita.filter((d) => d.disponibile).map((d) => nomeDi(v, d.partito)),
    },
    campagna: { spese: v.campagna.spese.length, euro: v.campagna.spese.reduce((a, s) => a + s.euro, 0) },
  };
}
