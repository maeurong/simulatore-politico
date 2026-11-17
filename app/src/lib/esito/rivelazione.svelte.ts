// La rivelazione della prima pagina e la corsa di «avanti», condivise fra cornice e sezioni (piano 11f, lettura 4).
import type { Fermata } from "$lib/tipi/Fermata";
import type { PuntoSerie } from "$lib/tipi/PuntoSerie";
import type { Vista } from "$lib/tipi/Vista";

class Rivelazione {
  // raw: la Vista di prima si sostituisce intera
  prima = $state.raw<Vista | null>(null);
  serie_prima = $state.raw<PuntoSerie[]>([]);
  /** Quante voci in evidenza sono rivelate. */
  passo = $state(0);
  /** Senza `prima` la pagina è ferma (Riprendi). */
  ferma = $state(true);
  /** Quante voci in evidenza ha la pagina: la rivelazione finisce qui. Prima di inizia() è infinito: chiudi() senza inizia lascia passo infinito. */
  fine = $state(Infinity);
  /** La corsa di «avanti» che ha aperto la pagina; ogni inizia() e ogni chiudi() la cancellano, e chi la scrive lo fa dopo. */
  corsa = $state.raw<{ chiusure: number; da: number; fermate: Fermata[] } | null>(null);

  inizia(prima: Vista, serie: PuntoSerie[], fine: number) {
    this.corsa = null;
    this.prima = prima;
    this.serie_prima = serie;
    this.fine = fine;
    this.passo = 0;
    this.ferma = fine <= 0;
  }

  avanza() {
    if (this.ferma) return;
    this.passo++;
    if (this.passo >= this.fine) this.ferma = true;
  }

  salta() {
    this.passo = this.fine;
    this.ferma = true;
  }

  /** Una voce ogni 1100 ms × k; il valore di ritorno ferma il timer. */
  ritmo(k: number): () => void {
    const timer = setInterval(() => this.avanza(), 1100 * k);
    return () => clearInterval(timer);
  }

  /** Tutto al valore nuovo: i tracciati tornano a leggere la Vista corrente. */
  chiudi() {
    this.corsa = null;
    this.salta();
    this.prima = null;
    this.serie_prima = [];
  }
}

export const rivelazione = new Rivelazione();
