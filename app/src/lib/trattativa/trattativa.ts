// Trattativa: la spartizione in corso (dalla Vista) e il risultato del voto in testa (ticket 27, storie 84-87).
import { maggioranza } from "$lib/componenti/seggi";
import { coalizioneDi, nomeDi } from "$lib/partiti";
import { t, type Chiave } from "$lib/testi/t";
import type { Comando } from "$lib/tipi/Comando";
import type { Vista } from "$lib/tipi/Vista";
import type { VotiPartito } from "$lib/tipi/VotiPartito";

/** I dicasteri della spartizione in corso che sono di `a` (il giocatore, un alleato, null il tecnico): quanti e quanti punti di peso. */
export function quota(v: Vista, a: string | null) {
  const mie = v.spartizione.filter((d) => d.partito === a);
  return { n: mie.length, punti: mie.reduce((s, d) => s + d.peso, 0) };
}

/** A chi si può dare un dicastero: il giocatore, il tecnico (null), poi gli alleati di Vista.richieste. */
export const aChi = (v: Vista): (string | null)[] => ["giocatore", null, ...v.richieste.map((r) => r.partito)];

/** Il comando: la spartizione intera della Vista, con quel dicastero dato ad `a`. */
export const proponi = (v: Vista, dicastero: string, a: string | null): Comando => ({
  tipo: "proponi_spartizione",
  dicasteri: v.spartizione.map((d) => ({ dicastero: d.dicastero, partito: d.dicastero === dicastero ? a : d.partito })),
});

/** Il comando che rimette ogni dicastero al suo `di_default`. */
export const predefinita = (v: Vista): Comando => ({
  tipo: "proponi_spartizione",
  dicasteri: v.spartizione.map((d) => ({ dicastero: d.dicastero, partito: d.di_default })),
});

/** Almeno un dicastero non è dove lo mette il default. */
export const cambiata = (v: Vista): boolean => v.spartizione.some((d) => d.partito !== d.di_default);

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

/** Per alleato: richiesta, offerta, scarto in punti e intesa di partenza; `stato` dice se l'intesa è sotto l'ultimatum o l'uscita. */
export const richiesteDi = (v: Vista) =>
  v.richieste.map((r) => ({
    id: r.partito,
    nome: nomeDi(v, r.partito),
    richiesta: r.richiesta,
    offerta: r.offerta,
    scarto: r.offerta - r.richiesta,
    intesa: r.intesa_partenza,
    stato: (r.intesa_partenza < v.soglie.uscita ? "uscita" : r.intesa_partenza < v.soglie.ultimatum ? "ultimatum" : "ok") as "ok" | "ultimatum" | "uscita",
  }));

/** La coalizione del ribaltone nell'ordine del core, coi seggi dell'ultimo voto e il primo segnato; null fuori dal ribaltone. */
export function ribaltoneDi(v: Vista): { id: string; nome: string; camera: number; senato: number; primo: boolean }[] | null {
  if (!v.ribaltone) return null;
  const voti = v.elezioni.at(-1)?.voti ?? [];
  return v.ribaltone.coalizione.map((id) => {
    const x = voti.find((y) => y.partito === id);
    return { id, nome: id === "giocatore" ? t("trattativa.tu") : nomeDi(v, id), camera: x?.camera ?? 0, senato: x?.senato ?? 0, primo: id === v.ribaltone!.primo };
  });
}

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
 * Un comando del ribaltone. Il fuoco passa alla testa solo mentre il comando è in volo e fino a che la pagina nuova ha montato
 * (`dopo`); chi lo consuma (l'onMount di Trattativa) lo azzera. Riuscito senza rimontaggio, o guasto, o rifiuto, il flag torna
 * spento e nessun montaggio futuro lo trova.
 */
export async function mandaRibaltone(tipo: "chiedi_guida" | "rifiuta_ribaltone", manda: (c: Comando) => Promise<EsitoManda>, occupato: () => boolean, dopo: () => Promise<void>): Promise<EsitoManda> {
  if (occupato()) return "occupato";
  passaggio.fuoco = true;
  try {
    return await manda({ tipo });
  } finally {
    try {
      await dopo();
    } finally {
      passaggio.fuoco = false;
    }
  }
}
