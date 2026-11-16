// Il bilancio in composizione, solo in memoria: resta finché il turno non cambia.
import { ultimoVince } from "$lib/editor/editor.svelte";
import { comeErrore, guscio, testoRifiuto, type ErroreGuscio } from "$lib/guscio/chiamate";
import type { Bilancio } from "$lib/tipi/Bilancio";
import type { RiepilogoBilancio } from "$lib/tipi/RiepilogoBilancio";
import type { Voce } from "$lib/tipi/Voce";
import { annullata, indicizzata } from "./bilancio";

const chiave = (v: Voce) => (v.tipo === "dicastero" ? v.id : v.tipo);
const voci = (b: Bilancio): Voce[] => [{ tipo: "tutto" }, { tipo: "scaglioni" }, ...b.dicasteri.map((d) => ({ tipo: "dicastero" as const, id: d.id }))];
const testo = (e: unknown) => testoRifiuto(comeErrore(e));

class Bozza {
  // raw: i bilanci si sostituiscono interi e si confrontano per identità
  base = $state.raw<Bilancio | null>(null);
  corrente = $state.raw<Bilancio | null>(null);
  riepilogo = $state.raw<RiepilogoBilancio | null>(null);
  rifiuto = $state<string | null>(null);
  inVolo = $state(false);
  /** «tutto», «scaglioni», id del dicastero: il riepilogo della bozza indicizzata su quella voce, o il rifiuto del core (lettura 1). */
  costi = $state.raw<Record<string, RiepilogoBilancio | string>>({});
  #indicizzati = $state.raw<Record<string, Bilancio>>({});
  #turno: number | null = null;
  // avanza a ogni apri e chiudi: una risposta di un giro vecchio si scarta, anche se il turno torna uguale
  #giro = 0;
  /** Il bilancio dell'ultimo riepilogo chiesto: solo il suo esito si mostra. */
  #chiesto: Bilancio | null = null;

  #chiedi = ultimoVince(
    (b: Bilancio) => guscio.riepilogoBilancio(b),
    (r: RiepilogoBilancio | ErroreGuscio, b: Bilancio) => {
      if (b !== this.#chiesto) return;
      this.inVolo = false;
      if ("specie" in r) {
        this.rifiuto = testoRifiuto(r);
        this.riepilogo = null;
      } else {
        this.rifiuto = null;
        this.riepilogo = r;
      }
    },
  );

  #riepiloga(b: Bilancio) {
    this.#chiesto = b;
    this.inVolo = true;
    this.#chiedi(b);
  }

  #svuota(turno: number | null) {
    this.#turno = turno;
    this.#giro++;
    this.base = this.corrente = this.riepilogo = this.#chiesto = null;
    this.rifiuto = null;
    this.inVolo = false;
    this.costi = this.#indicizzati = {};
  }

  /** Fuori dalla partita: la prossima apre la sua bozza anche allo stesso turno. */
  chiudi() {
    this.#svuota(null);
  }

  /** Nuovo turno = bozza nuova; lo stesso turno tiene il bilancio in memoria. */
  async apri(turno: number) {
    if (turno === this.#turno) return;
    this.#svuota(turno);
    const giro = this.#giro;
    // in volo anche la bozza: finché non arriva la pagina non dice che manca
    this.inVolo = true;
    let base: Bilancio | null;
    try {
      base = await guscio.bozzaBilancio();
    } catch (e) {
      // il turno torna null: il prossimo apri ritenta
      if (giro === this.#giro) [this.rifiuto, this.inVolo, this.#turno] = [testo(e), false, null];
      return;
    }
    if (giro !== this.#giro) return;
    if (!base) return void (this.inVolo = false);
    this.base = base;
    this.#riepiloga(base);
    const esiti = await Promise.all(
      voci(base).map(async (v) => {
        try {
          const b = await guscio.indicizza(base, v);
          return [chiave(v), b, await guscio.riepilogoBilancio(b)] as const;
        } catch (e) {
          return [chiave(v), null, testo(e)] as const;
        }
      }),
    );
    if (giro !== this.#giro) return;
    this.costi = Object.fromEntries(esiti.map(([k, , r]) => [k, r]));
    this.#indicizzati = Object.fromEntries(esiti.flatMap(([k, b]) => (b ? [[k, b]] : [])));
  }

  /** Il bilancio in memoria cambia; il riepilogo lo chiede `ultimoVince`. */
  cambia(b: Bilancio) {
    this.corrente = b;
    this.#riepiloga(b);
  }

  /** Indicizza la voce sul bilancio in memoria; se intanto è cambiato, riparte da quello nuovo. */
  async indicizza(voce: Voce) {
    const giro = this.#giro;
    for (;;) {
      const da = this.corrente ?? this.base;
      if (!da) return;
      this.inVolo = true;
      let b: Bilancio;
      try {
        b = await guscio.indicizza(da, voce);
      } catch (e) {
        if (giro !== this.#giro) return;
        this.inVolo = false;
        this.rifiuto = testo(e);
        this.riepilogo = null;
        return;
      }
      if (giro !== this.#giro) return;
      if ((this.corrente ?? this.base) === da) return this.cambia(b);
    }
  }

  /** Riporta la voce alle cifre della bozza; le aliquote restano quelle ritoccate. */
  annulla(voce: Voce) {
    const da = this.corrente ?? this.base;
    if (da && this.base) this.cambia(annullata(da, this.base, voce));
  }

  /** La voce è già indicizzata nel bilancio in memoria: al posto del costo si legge «indicizzato». */
  indicizzato(voce: Voce): boolean {
    const da = this.corrente ?? this.base, ind = this.#indicizzati[chiave(voce)];
    return !!da && !!ind && indicizzata(da, ind, voce);
  }
}

export const bozza = new Bozza();
