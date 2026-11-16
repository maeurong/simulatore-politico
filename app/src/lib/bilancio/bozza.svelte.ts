// Il bilancio in composizione, solo in memoria: resta finché il turno non cambia.
import { ultimoVince } from "$lib/editor/editor.svelte";
import { comeErrore, guscio, testoRifiuto, type ErroreGuscio } from "$lib/guscio/chiamate";
import type { Bilancio } from "$lib/tipi/Bilancio";
import type { RiepilogoBilancio } from "$lib/tipi/RiepilogoBilancio";
import type { Voce } from "$lib/tipi/Voce";
import { allarga, annullata, indicizzata } from "./bilancio";

const chiave = (v: Voce) => (v.tipo === "dicastero" ? v.id : v.tipo);
const voci = (b: Bilancio): Voce[] => [{ tipo: "tutto" }, { tipo: "scaglioni" }, ...b.dicasteri.map((d) => ({ tipo: "dicastero" as const, id: d.id }))];
const testo = (e: unknown) => testoRifiuto(comeErrore(e));
const VIE = ["funzionamento", "disavanzo"] as const;
export type Via = (typeof VIE)[number];

class Bozza {
  // raw: i bilanci si sostituiscono interi e si confrontano per identità
  base = $state.raw<Bilancio | null>(null);
  corrente = $state.raw<Bilancio | null>(null);
  riepilogo = $state.raw<RiepilogoBilancio | null>(null);
  rifiuto = $state<string | null>(null);
  /** Il rifiuto è un guasto (il core non ha risposto), non un no del core: «Presenta» lo dice diverso. */
  guasto = $state(false);
  /** «Modifica e ripresenta»: la pagina torna alla bozza fino alla nuova presentazione. */
  riaperta = $state(false);
  // raw: il rifiuto di «Presenta» vale per il bilancio che l'ha avuto, confrontato per identità
  #presentaRifiutata = $state.raw<{ testo: string; bilancio: Bilancio } | null>(null);
  /** I passi del margine aggiunti, per dicastero e per via: il − toglie solo questi (lettura 2). */
  passi = $state.raw<Record<string, Record<Via, number>>>({});
  /** Il rifiuto di «Indicizza», per voce: sta accanto alla voce, non copre il giogo. */
  #rifiutiVoce = $state.raw<Record<string, string>>({});
  // la bozza o un «Indicizza» in volo; il riepilogo a parte, perché ultimoVince ne lascia tornare solo l'ultimo
  #volo = $state(0);
  #aspetta = $state(false);
  /** Qualcosa che cambia il conto è ancora in volo: «Presenta» aspetta. */
  get inVolo() {
    return this.#volo > 0 || this.#aspetta;
  }
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
      this.#aspetta = false;
      if ("specie" in r) {
        this.rifiuto = testoRifiuto(r);
        this.guasto = r.specie === "guasto";
        this.riepilogo = null;
      } else {
        this.rifiuto = null;
        this.guasto = false;
        this.riepilogo = r;
      }
    },
  );

  #riepiloga(b: Bilancio) {
    this.#chiesto = b;
    this.#aspetta = true;
    this.#chiedi(b);
  }

  #svuota(turno: number | null) {
    this.#turno = turno;
    this.#giro++;
    this.base = this.corrente = this.riepilogo = this.#chiesto = null;
    this.rifiuto = null;
    this.guasto = this.riaperta = false;
    this.#presentaRifiutata = null;
    this.#volo = 0;
    this.#aspetta = false;
    this.costi = this.#indicizzati = this.#rifiutiVoce = {};
    this.passi = {};
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
    this.#volo++;
    let base: Bilancio | null;
    try {
      base = await guscio.bozzaBilancio();
    } catch (e) {
      // il turno torna null: il prossimo apri ritenta
      if (giro === this.#giro) [this.rifiuto, this.guasto, this.#turno] = [testo(e), comeErrore(e).specie === "guasto", null];
      return;
    } finally {
      if (giro === this.#giro) this.#volo--;
    }
    if (giro !== this.#giro || !base) return;
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
    this.#rifiutiVoce = {};
    this.#riepiloga(b);
  }

  /** «Modifica e ripresenta ›»: il bilancio in aula, emendamenti accolti compresi, torna in memoria (lettura 4). */
  riapri(inAula: Bilancio) {
    this.riaperta = true;
    this.cambia(inAula);
  }

  /** Il rifiuto dell'ultimo «Presenta», finché il bilancio in memoria è quello presentato. */
  get rifiutoPresenta(): string | null {
    const r = this.#presentaRifiutata;
    return r && r.bilancio === (this.corrente ?? this.base) ? r.testo : null;
  }

  /** Presenta il bilancio in memoria con `comanda` (partita.comanda): riuscito chiude la riapertura, rifiutato tiene il testo. */
  async presenta(comanda: (b: Bilancio) => Promise<unknown>) {
    const b = this.corrente ?? this.base, giro = this.#giro;
    if (!b) return;
    this.#presentaRifiutata = null;
    try {
      // null: guasto o comando già in corso, la presentazione non è avvenuta
      if ((await comanda(b)) !== null && giro === this.#giro) this.riaperta = false;
    } catch (e) {
      if (giro === this.#giro) this.#presentaRifiutata = { testo: testo(e), bilancio: b };
    }
  }

  /** Indicizza la voce sul bilancio in memoria; se intanto è cambiato, riparte da quello nuovo. */
  async indicizza(voce: Voce) {
    const giro = this.#giro;
    for (;;) {
      const da = this.corrente ?? this.base;
      if (!da) return;
      this.#volo++;
      let b: Bilancio;
      try {
        b = await guscio.indicizza(da, voce);
      } catch (e) {
        if (giro === this.#giro) this.#rifiutiVoce = { ...this.#rifiutiVoce, [chiave(voce)]: testo(e) };
        return;
      } finally {
        if (giro === this.#giro) this.#volo--;
      }
      if (giro !== this.#giro) return;
      if ((this.corrente ?? this.base) === da) return this.cambia(b);
    }
  }

  /** Il rifiuto dell'ultimo «Indicizza» su questa voce, finché il bilancio non cambia. */
  rifiutoDi(voce: Voce): string | undefined {
    return this.#rifiutiVoce[chiave(voce)];
  }

  /** Un passo del margine in più o in meno; a zero passi il − non fa niente. */
  allarga(dicastero: string, via: Via, verso: 1 | -1) {
    const da = this.corrente ?? this.base, n = this.passi[dicastero] ?? { funzionamento: 0, disavanzo: 0 };
    if (!da || !this.base || n[via] + verso < 0) return;
    this.passi = { ...this.passi, [dicastero]: { ...n, [via]: n[via] + verso } };
    this.cambia(allarga(da, this.base, dicastero, via, verso));
  }

  /** Riporta la voce alle cifre della bozza; le aliquote restano quelle ritoccate, i passi del margine contati si riapplicano. */
  annulla(voce: Voce) {
    const da = this.corrente ?? this.base, base = this.base;
    if (!da || !base) return;
    let b = annullata(da, base, voce);
    if (voce.tipo !== "scaglioni")
      for (const [id, n] of Object.entries(this.passi)) if (voce.tipo === "tutto" || id === voce.id) for (const via of VIE) b = allarga(b, base, id, via, n[via]);
    this.cambia(b);
  }

  /** La voce è già indicizzata nel bilancio in memoria: al posto del costo si legge «indicizzato». */
  indicizzato(voce: Voce): boolean {
    const da = this.corrente ?? this.base, ind = this.#indicizzati[chiave(voce)];
    return !!da && !!ind && indicizzata(da, ind, voce);
  }
}

export const bozza = new Bozza();
