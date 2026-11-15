import { partita } from "$lib/partita.svelte";
import { comeErrore, testoRifiuto } from "$lib/guscio/chiamate";
import type { Vista } from "$lib/tipi/Vista";

/** L'acquisto del sondaggio privato, diretto e senza conferma. Il rifiuto del core vale per la Vista in cui è nato. */
export function acquistoSondaggio() {
  // raw: la Vista si confronta per identità
  let rifiuto = $state.raw<{ testo: string; vista: Vista | null } | null>(null);
  return {
    get rifiuto(): string | null {
      return rifiuto && rifiuto.vista === partita.vista ? rifiuto.testo : null;
    },
    async compra() {
      if (partita.occupato) return;
      rifiuto = null;
      try {
        await partita.comanda({ tipo: "compra", acquisto: "sondaggio_privato" });
      } catch (e) {
        rifiuto = { testo: testoRifiuto(comeErrore(e)), vista: partita.vista };
      }
    },
  };
}
