// I registri dei turni chiusi, in memoria fino a riavvolgi, riprendi, fonda o esci (piano 11f, lettura 10).
import { comeErrore, guscio } from "$lib/guscio/chiamate";
import type { Vista } from "$lib/tipi/Vista";
import { turnoDa, type Turno } from "./registro";

type Guasto = { turno: number; guasto: string };

class RegistroCache {
  // raw: l'elenco si sostituisce intero, mai si modifica sul posto
  turni = $state.raw<(Turno | Guasto)[]>([]);
  // svuota() lo fa avanzare: la risposta di un giro vecchio non entra nella cache nuova
  #giro = 0;
  #volo: Promise<void> | null = null;

  /** Un turno alla volta, dal più vecchio; i turni già letti non si richiedono, quelli guasti sì. Una chiamata in volo si riusa. */
  carica(vista: Vista): Promise<void> {
    return (this.#volo ??= this.#leggi(vista));
  }

  async #leggi(vista: Vista) {
    const giro = this.#giro;
    try {
      for (let t = 0; t < vista.turno; t++) {
        if ("voci" in (this.turni[t] ?? {})) continue;
        let voce: Turno | Guasto;
        try {
          voce = turnoDa(t, await guscio.registro(t));
        } catch (e) {
          voce = { turno: t, guasto: comeErrore(e).testo };
        }
        if (giro !== this.#giro) return;
        const nuovi = [...this.turni];
        nuovi[t] = voce;
        this.turni = nuovi;
      }
    } finally {
      if (giro === this.#giro) this.#volo = null;
    }
  }

  svuota() {
    this.#giro++;
    this.#volo = null;
    this.turni = [];
  }
}

export const registroCache = new RegistroCache();
