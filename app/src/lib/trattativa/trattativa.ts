// Trattativa: la bozza della spartizione e il risultato del voto in testa (ticket 27, storie 84-87).
import { maggioranza } from "$lib/componenti/seggi";
import { coalizioneDi, nomeDi } from "$lib/partiti";
import type { Chiave } from "$lib/testi/t";
import type { Comando } from "$lib/tipi/Comando";
import type { Vista } from "$lib/tipi/Vista";
import type { VotiPartito } from "$lib/tipi/VotiPartito";

/** Peso a pallini di un dicastero: 3 Economia, 2 Interno ed Esteri, 1 gli altri (spec:379). La 11g lo legge dal core. */
export function pesoDi(dicastero: string): 1 | 2 | 3 {
  return dicastero === "economia_finanze" ? 3 : dicastero === "interno" || dicastero === "esteri_difesa" ? 2 : 1;
}

/** I dicasteri che la bozza dà a `a` (il giocatore, un alleato, null il tecnico): quanti e quanti punti di peso. */
export function quota(v: Vista, b: Bozza, a: string | null) {
  const mie = v.dicasteri.filter((d) => b[d.id] === a);
  return { n: mie.length, punti: mie.reduce((s, d) => s + pesoDi(d.id), 0) };
}

/** A chi si può dare un dicastero: il giocatore, il tecnico (null), poi gli alleati di Vista.richieste. */
export const aChi = (v: Vista): (string | null)[] => ["giocatore", null, ...v.richieste.map((r) => r.partito)];

/** La bozza: un'assegnazione per dicastero, `undefined` finché il giocatore non sceglie. */
export type Bozza = Record<string, string | null | undefined>;

export const completa = (v: Vista, b: Bozza): boolean => v.dicasteri.every((d) => b[d.id] !== undefined);

/** Il comando, solo a bozza completa: una voce per dicastero, nell'ordine di Vista.dicasteri. */
export function proponi(v: Vista, b: Bozza): Comando | null {
  if (!completa(v, b)) return null;
  return { tipo: "proponi_spartizione", dicasteri: v.dicasteri.map((d) => ({ dicastero: d.id, partito: b[d.id] ?? null })) };
}

const ramo = (voti: VotiPartito[], miei: Set<string>, r: "camera" | "senato") => ({
  seggi: voti.filter((x) => miei.has(x.partito)).reduce((a, x) => a + x[r], 0),
  soglia: maggioranza(voti.reduce((a, x) => a + x[r], 0)),
});

/** Il risultato dell'ultimo voto letto per la testa della pagina. */
export function risultato(v: Vista) {
  const e = v.elezioni.at(-1);
  if (!e) return null;
  const miei = new Set(coalizioneDi(v));
  const pos = (id: string) => v.partiti.find((p) => p.id === id)?.posizione_dichiarata ?? 0;
  return {
    affluenza: e.affluenza,
    premio: e.premio ? e.premio.split("+").map((id) => nomeDi(v, id)) : null,
    voti: e.voti.map((x) => ({ ...x, nome: nomeDi(v, x.partito), miei: miei.has(x.partito) })).sort((a, b) => pos(a.partito) - pos(b.partito)),
    camera: ramo(e.voti, miei, "camera"),
    senato: ramo(e.voti, miei, "senato"),
  };
}

/** Per alleato: richiesta, offerta e scarto in punti, dalla Vista. */
export const richiesteDi = (v: Vista) =>
  v.richieste.map((r) => ({ id: r.partito, nome: nomeDi(v, r.partito), richiesta: r.richiesta, offerta: r.offerta, scarto: r.offerta - r.richiesta }));

/** La coalizione ha la metà più uno in tutte e due le Camere. */
export function haLaMaggioranza(v: Vista): boolean {
  const r = risultato(v);
  return !!r && r.camera.seggi >= r.camera.soglia && r.senato.seggi >= r.senato.soglia;
}

/** I partiti fuori dalla coalizione: seggi dell'ultimo voto, disponibilità e causa del core. */
export const fuori = (v: Vista) =>
  v.disponibilita.map((d) => {
    const x = v.elezioni.at(-1)?.voti.find((y) => y.partito === d.partito);
    return { id: d.partito, nome: nomeDi(v, d.partito), camera: x?.camera ?? 0, senato: x?.senato ?? 0, disponibile: d.disponibile, causa: d.causa };
  });

export const allarga = (partiti: string[]): Comando => ({ tipo: "allarga", partiti });

/** Il default accanto a «Chiudi il turno», secondo l'incarico. */
export const defaultDellaChiusura = (v: Vista): Chiave | null =>
  v.incarico === "giocatore" ? "cornice.senza_proposta" : v.incarico === "guida" ? "cornice.senza_scelta_guida" : null;

export type EsitoManda = "ok" | "occupato" | "guasto" | { rifiuto: string };

/** Il fuoco che la pagina smontata da un comando lascia alla testa di quella nuova (Ribaltone → Trattativa). */
export const passaggio = { fuoco: false };

/**
 * Un comando del ribaltone. Il fuoco passa alla testa solo mentre il comando è in volo e fino a che la pagina nuova ha
 * montato (`dopo`): riuscito senza rimontaggio, o guasto, o rifiuto, il flag torna spento e nessun montaggio futuro lo trova.
 */
export async function mandaRibaltone(tipo: "chiedi_guida" | "rifiuta_ribaltone", manda: (c: Comando) => Promise<EsitoManda>, occupato: () => boolean, dopo: () => Promise<void>): Promise<EsitoManda> {
  if (occupato()) return "occupato";
  passaggio.fuoco = true;
  try {
    return await manda({ tipo });
  } finally {
    await dopo();
    passaggio.fuoco = false;
  }
}
export type StatoSpartizione = {
  bozza: Bozza;
  /** sotto la riga che il giocatore ha cliccato */
  rifiuto: { dicastero: string; testo: string } | null;
  /** «partita» solo se l'ultimo comando è riuscito; «pendente» finché un comando è in volo o aspetta di partire */
  esito: "pendente" | "partita" | null;
};

/**
 * La bozza della spartizione: ogni scelta a bozza completa manda il comando. La bozza resta l'ultima che il core ha
 * accettato: un rifiuto o un guasto riportano il dicastero cliccato alla scelta accettata. Un clic con un comando in volo
 * (`manda` dà «occupato») resta pendente e `riprova()`, da chiamare quando la partita si libera, lo manda una volta sola.
 */
export function sessioneSpartizione(vista: () => Vista, manda: (c: Comando) => Promise<EsitoManda>, mostra: (s: StatoSpartizione) => void) {
  let bozza: Bozza = {};
  let accettata: Bozza = {};
  let rifiuto: StatoSpartizione["rifiuto"] = null;
  let esito: "pendente" | "partita" | null = null;
  let pendente = false;
  let ultimo = "";
  // svuota() lo fa avanzare: la risposta di un invio partito prima appartiene a una bozza che non c'è più e si scarta
  let giro = 0;
  const dai = () => mostra({ bozza, rifiuto, esito: pendente ? "pendente" : esito });
  // le scelte che non stanno più in aChi (un alleato è uscito) non valgono
  const pulisci = (b: Bozza): Bozza => {
    const valide = new Set(aChi(vista()));
    return Object.fromEntries(Object.entries(b).filter(([, x]) => x !== undefined && valide.has(x)));
  };
  const indietro = (inviata: Bozza, dicastero: string) => {
    if (bozza[dicastero] !== inviata[dicastero]) return;
    const { [dicastero]: _, ...resto } = bozza;
    bozza = accettata[dicastero] === undefined ? resto : { ...resto, [dicastero]: accettata[dicastero] };
  };

  async function invia() {
    pendente = false;
    const inviata = bozza;
    const dicastero = ultimo;
    const mio = giro;
    const comando = proponi(vista(), inviata);
    if (!comando) {
      esito = null;
      return dai();
    }
    esito = "pendente";
    dai();
    const r = await manda(comando);
    if (mio !== giro) return;
    if (r === "occupato") pendente = true;
    else if (r === "ok") {
      accettata = pulisci(inviata);
      esito = bozza === inviata ? "partita" : pendente ? "pendente" : null;
    } else {
      indietro(inviata, dicastero);
      if (typeof r === "object") rifiuto = { dicastero, testo: r.rifiuto };
      esito = null;
    }
    dai();
  }

  return {
    scegli(dicastero: string, a: string | null) {
      if (bozza[dicastero] === a) return;
      rifiuto = null;
      ultimo = dicastero;
      bozza = { ...bozza, [dicastero]: a };
      return invia();
    },
    riprova() {
      if (!pendente) return;
      rifiuto = null;
      return invia();
    },
    /** «Torna alla spartizione di default»: la bozza riparte da zero; la proposta già fatta resta nel core finché non ne parte un'altra. */
    svuota() {
      giro++;
      bozza = {};
      accettata = {};
      rifiuto = null;
      esito = null;
      pendente = false;
      dai();
    },
    /** La Vista è cambiata: le scelte che non stanno più in aChi (un alleato è uscito) si scartano, senza mandare niente. */
    aggiorna() {
      const nuova = pulisci(bozza);
      accettata = pulisci(accettata);
      if (Object.keys(nuova).length === Object.keys(bozza).filter((k) => bozza[k] !== undefined).length) return;
      bozza = nuova;
      rifiuto = null;
      esito = null;
      pendente = false;
      dai();
    },
  };
}

export type Sessione = ReturnType<typeof sessioneSpartizione>;
/** Ciò che la pagina tiene della sessione: lo stato e la Vista della partita in cui è nato (il rifiuto vale solo per quella). */
export type Stato = { s: StatoSpartizione; vista: Vista | null };
