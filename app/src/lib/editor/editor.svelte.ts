// Lo scenario in composizione: vive solo in memoria, lo legge la fondazione.
import { comeErrore, type ErroreGuscio } from "$lib/guscio/chiamate";
import type { CatalogoScenario } from "$lib/tipi/CatalogoScenario";
import type { Cursori } from "$lib/tipi/Cursori";
import type { CursoreCatalogo } from "$lib/tipi/CursoreCatalogo";
import type { RiepilogoScenario } from "$lib/tipi/RiepilogoScenario";
import type { SuMisura } from "$lib/tipi/SuMisura";
import type { VoceCatalogo } from "$lib/tipi/VoceCatalogo";

/** Lo scenario di base dal catalogo: i valori dei cursori, legge_di_base, nessun ritocco. */
export function base(c: CatalogoScenario): SuMisura {
  const cursori = Object.fromEntries(c.cursori.map((x) => [x.nome, x.valore]));
  return { nome: "", cursori: { ...cursori, legge_elettorale: c.legge_di_base } as Cursori, disposizioni: {}, sostituzioni: {} };
}

/** Un ritocco: sull'Italia di oggi (nome "") apre un su misura nuovo senza nome (storia 13). */
export function ritocca(s: SuMisura | null, c: CatalogoScenario, f: (s: SuMisura) => void): SuMisura {
  const n = s ? structuredClone(s) : base(c);
  f(n);
  return n;
}

/** Cambiare legge toglie i ritocchi delle nove disposizioni elettorali (ticket 14:24). */
export function cambiaLegge(s: SuMisura, c: CatalogoScenario, legge: string): SuMisura {
  return ritocca(s, c, (n) => {
    n.cursori.legge_elettorale = legge;
    for (const d of c.disposizioni) if (d.norma === "elettorale") delete n.disposizioni[d.id];
  });
}

/** Il gradino senza ritocco: per le elettorali quello della legge scelta, per le altre quello di partenza. */
export function diPartenza(s: SuMisura | null, c: CatalogoScenario, d: VoceCatalogo): string {
  if (d.norma !== "elettorale") return d.partenza;
  return c.leggi_elettorali[s?.cursori.legge_elettorale ?? c.legge_di_base]?.[d.id] ?? d.partenza;
}

export const gradino = (s: SuMisura | null, c: CatalogoScenario, d: VoceCatalogo) =>
  s?.disposizioni[d.id] ?? diPartenza(s, c, d);

/** Sceglie un gradino; quello senza ritocco toglie la voce, così lo scenario resta il più corto. */
export function scegli(s: SuMisura | null, c: CatalogoScenario, d: VoceCatalogo, g: string): SuMisura {
  return ritocca(s, c, (n) => {
    if (g === diPartenza(n, c, d)) delete n.disposizioni[d.id];
    else n.disposizioni[d.id] = g;
  });
}

/** Il rifiuto del core senza il prefisso che ripete il titolo (14-editor.prototype.html:332). */
export const senzaPrefisso = (testo: string) => testo.replace(/^.*legge elettorale: /, "");

export const alLimite = (v: number, cur: CursoreCatalogo) => v <= cur.min || v >= cur.max;

/** Una richiesta in volo per volta; arrivata quella, parte l'ultima chiesta nel frattempo; l'esito vecchio non sovrascrive il nuovo. */
export function ultimoVince<A, R>(f: (a: A) => Promise<R>, esito: (r: R | ErroreGuscio, a: A) => void): (a: A) => void {
  let inVolo = false;
  let prossima: { a: A } | null = null;
  const parti = (a: A) => {
    inVolo = true;
    let p: Promise<R>;
    try {
      p = f(a);
    } catch (e) {
      // un lancio sincrono passa dalla stessa strada di un rifiuto: inVolo torna falso nel finally
      p = Promise.reject(e);
    }
    p
      .then((r) => esito(r, a), (e) => esito(comeErrore(e), a))
      .finally(() => {
        inVolo = false;
        const p = prossima;
        prossima = null;
        if (p) parti(p.a);
      });
  };
  return (a) => {
    if (inVolo) prossima = { a };
    else parti(a);
  };
}

class Editor {
  // raw: lo scenario si sostituisce intero (ritocca), mai si modifica sul posto
  /** null = l'Italia di oggi */
  scenario = $state.raw<SuMisura | null>(null);
  /** il nome dello scenario salvato da cui si parte; null per l'Italia di oggi e per un su misura nuovo */
  sorgente = $state<string | null>(null);
  /** modifiche non salvate */
  mosso = $state(false);
  /** l'ultimo riepilogo valido: davanti a un rifiuto resta fermo, spento */
  riepilogo = $state.raw<RiepilogoScenario | null>(null);
  /** il testo del core se l'ultimo scenario chiesto non regge */
  rifiuto = $state<string | null>(null);
  /** il testo del guasto dell'ultimo riepilogo chiesto */
  guasto = $state<string | null>(null);
  // azzera() lo fa avanzare: un esito chiesto prima dell'uscita appartiene all'editor lasciato e si scarta
  #giro = 0;
  get giro() {
    return this.#giro;
  }

  /** Un esito di riepilogo_scenario chiesto al giro `giro`. */
  esito(r: RiepilogoScenario | ErroreGuscio, giro: number) {
    if (giro !== this.#giro) return;
    if (!("specie" in r)) {
      this.riepilogo = r;
      this.rifiuto = this.guasto = null;
    } else if (r.specie === "rifiuto") this.rifiuto = r.testo;
    else this.guasto = r.testo;
  }

  /** Si può fondare: l'ultimo riepilogo chiesto non è stato rifiutato. */
  get valido() {
    return this.rifiuto === null;
  }

  azzera() {
    this.scenario = null;
    this.sorgente = null;
    this.mosso = false;
    this.riepilogo = null;
    this.rifiuto = null;
    this.guasto = null;
    this.#giro++;
  }
}

export const editor = new Editor();

/** La coda dei riepiloghi della pagina: ultimoVince che scrive in `editor`, col giro del momento della richiesta. */
export function codaRiepiloghi(f: (s: SuMisura | null) => Promise<RiepilogoScenario>): (s: SuMisura | null) => void {
  const chiedi = ultimoVince((x: { s: SuMisura | null; giro: number }) => f(x.s), (r, x) => editor.esito(r, x.giro));
  return (s) => chiedi({ s, giro: editor.giro });
}
