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

/** Il default accanto a «Chiudi il turno», secondo l'incarico. */
export const defaultDellaChiusura = (v: Vista): Chiave | null =>
  v.incarico === "giocatore" ? "cornice.senza_proposta" : v.incarico === "guida" ? "cornice.senza_scelta_guida" : null;
