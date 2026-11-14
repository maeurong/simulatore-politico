// Partiti, forma A · Schieramento (24-partiti.prototype.html): l'ordine dell'asse, le file, gli avvisi, i seggi.
import { coloreDi } from "$lib/partiti";
import { t, type Chiave } from "$lib/testi/t";
import type { SeggiPartito } from "$lib/tipi/SeggiPartito";
import type { Vista } from "$lib/tipi/Vista";
import type { VocePartito } from "$lib/tipi/VocePartito";

export const TETTO_PARTITI = 10;

/** I partiti nell'ordine dell'asse: per posizione dichiarata, a parità l'ordine della Vista. */
export const inOrdine = (v: Vista): VocePartito[] =>
  v.partiti.map((p, i) => ({ p, i })).sort((a, b) => a.p.posizione_dichiarata - b.p.posizione_dichiarata || a.i - b.i).map((x) => x.p);

/** Schede per fila: quattro fino a otto partiti, cinque oltre. */
export const perFila = (n: number): 4 | 5 => (n > 8 ? 5 : 4);

/** L'indole in breve: vocazione · regola («mista · insegue i voti»). */
export const indoleBreve = (p: VocePartito): [Chiave, Chiave] => [`partiti.voc_${p.vocazione}`, `partiti.reg_${p.regola}`];

/** Gli avvisi di una scheda, nell'ordine: ultimatum, sciolto, senza seggi all'ultimo voto, governo uscente, guida il governo, nato in partita. */
export function avvisi(v: Vista, p: VocePartito): Chiave[] {
  const a: Chiave[] = [];
  if (v.alleati.some((x) => x.partito === p.id && x.ultimatum)) a.push("avviso.ultimatum");
  if (v.alleati_avversari.some((x) => x.partito === p.id && x.fascia === "ultimatum")) a.push("avviso.ultimatum_governo");
  if (p.sciolto) a.push("avviso.sciolto");
  const voto = v.elezioni[0]?.voti.find((x) => x.partito === p.id);
  if (!p.sciolto && voto && voto.camera + voto.senato === 0) a.push("avviso.senza_seggi");
  if (p.al_governo && v.fase === "sciolta") a.push("avviso.governo_uscente");
  else if (p.al_governo && !v.governa && p.id !== v.giocatore) {
    const camera = (id: string) => v.seggi.find((s) => s.partito === id)?.camera ?? 0;
    const capo = v.partiti.filter((x) => x.al_governo && x.id !== v.giocatore).reduce((m, x) => (camera(x.id) > camera(m.id) ? x : m));
    if (capo.id === p.id) a.push("avviso.guida");
  }
  if (coloreDi(p.id) === null) a.push("avviso.nato");
  return a;
}

/** Seggi in carica e proiezione di un partito; null a Camere sciolte o senza proiezione. */
export function seggiDi(v: Vista, id: string): { carica: SeggiPartito | null; oggi: SeggiPartito | null } {
  const riga = (l: SeggiPartito[] | null) => l?.find((s) => s.partito === id) ?? null;
  return { carica: v.fase === "sciolta" ? null : riga(v.seggi), oggi: riga(v.proiezione) };
}

/** La cassa in poche cifre: «2,5 mln €», «340 mila €». */
export function cassaBreve(euro: number): string {
  const x = (n: number) => n.toLocaleString("it-IT", { maximumFractionDigits: 1 });
  return euro >= 1e9 ? t("partiti.cassa_mld", { x: x(euro / 1e9) }) : euro >= 1e6 ? t("partiti.cassa_mln", { x: x(euro / 1e6) }) : t("partiti.cassa_mila", { x: x(Math.round(euro / 1e3)) });
}
