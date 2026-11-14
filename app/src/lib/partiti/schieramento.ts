// Partiti, forma A · Schieramento (24-partiti.prototype.html): l'ordine dell'asse, le file, gli avvisi, i seggi.
import { coloreDi } from "$lib/partiti";
import { t, type Chiave } from "$lib/testi/t";
import type { CausaIntesa } from "$lib/tipi/CausaIntesa";
import type { SeggiPartito } from "$lib/tipi/SeggiPartito";
import type { Vista } from "$lib/tipi/Vista";
import type { VoceAlleato } from "$lib/tipi/VoceAlleato";
import type { VoceFascia } from "$lib/tipi/VoceFascia";
import type { VocePartito } from "$lib/tipi/VocePartito";
import type { VotiPartito } from "$lib/tipi/VotiPartito";

export const TETTO_PARTITI = 10;

/** I partiti nell'ordine dell'asse: per posizione dichiarata, a parità l'ordine della Vista. */
export const inOrdine = (v: Vista): VocePartito[] =>
  v.partiti.map((p, i) => ({ p, i })).sort((a, b) => a.p.posizione_dichiarata - b.p.posizione_dichiarata || a.i - b.i).map((x) => x.p);

/** Schede per fila: quattro fino a otto partiti, cinque oltre. */
export const perFila = (n: number): 4 | 5 => (n > 8 ? 5 : 4);

/** L'indole in breve: vocazione · regola («mista · insegue i voti»). */
export const indoleBreve = (p: VocePartito): [Chiave, Chiave] => [`partiti.voc_${p.vocazione}`, `partiti.reg_${p.regola}`];

/** Gli avvisi di una scheda, nell'ordine: ultimatum, sciolto, senza seggi all'ultimo voto, governo uscente o al governo, nato in partita. Chi guida un governo IA non è nella Vista: arriva con la 11g. */
export function avvisi(v: Vista, p: VocePartito): Chiave[] {
  const a: Chiave[] = [];
  if (v.alleati.some((x) => x.partito === p.id && x.ultimatum)) a.push("avviso.ultimatum");
  if (v.alleati_avversari.some((x) => x.partito === p.id && x.fascia === "ultimatum")) a.push("avviso.ultimatum_governo");
  if (p.sciolto) a.push("avviso.sciolto");
  const voto = v.elezioni[0]?.voti.find((x) => x.partito === p.id);
  if (!p.sciolto && voto && voto.camera + voto.senato === 0) a.push("avviso.senza_seggi");
  if (p.al_governo && v.fase === "sciolta") a.push("avviso.governo_uscente");
  else if (p.al_governo && !v.governa && p.id !== v.giocatore) a.push("avviso.al_governo");
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

/** Una chiave di testo per ogni causa: un valore nuovo del core fa fallire `check` finché non ne ha una. */
export const CAUSE: Record<CausaIntesa, Chiave> = {
  norma_lontana: "intesa.causa_norma_lontana",
  emendamento_respinto: "intesa.causa_emendamento_respinto",
  fiducia: "intesa.causa_fiducia",
  seggi_persi: "intesa.causa_seggi_persi",
  spartizione: "intesa.causa_spartizione",
  ministri_scontenti: "intesa.causa_ministri_scontenti",
  dimissioni_chieste: "intesa.causa_dimissioni_chieste",
  quota_scoperta: "intesa.causa_quota_scoperta",
  sondaggi_in_calo: "intesa.causa_sondaggi_in_calo",
  rottura_impegno: "intesa.causa_rottura_impegno",
  sfiducia_respinta: "intesa.causa_sfiducia_respinta",
  promessa: "intesa.causa_promessa",
  programma_mes: "intesa.causa_programma_mes",
};

/** La frase sulla distanza: verso (sinistra, destra, ferma) e punti fra percepita e dichiarata. */
export function distanza(p: VocePartito): { verso: "sinistra" | "destra" | "ferma"; punti: number } {
  const d = p.posizione_percepita - p.posizione_dichiarata;
  // sotto mezzo centesimo il testo direbbe «0,00 punti»: è ferma
  return Math.abs(d) < 0.005 ? { verso: "ferma", punti: 0 } : { verso: d > 0 ? "destra" : "sinistra", punti: Math.abs(d) };
}

/** L'intesa con il giocatore: in cifre se è un alleato, in fascia se è un alleato del governo avversario, null altrimenti. */
export function intesaDi(v: Vista, id: string): { alleato: VoceAlleato } | { fascia: VoceFascia } | null {
  const alleato = v.alleati.find((a) => a.partito === id);
  if (alleato) return { alleato };
  const fascia = v.alleati_avversari.find((f) => f.partito === id);
  return fascia ? { fascia } : null;
}

/** Le mosse del partito nell'ultima chiusura, nell'ordine della Vista. Le Viste campione anteriori allo schema 18 non hanno `esito`. */
export const mosseDi = (v: Vista, id: string): string[] => (v.esito ?? []).filter((e) => e.partito === id).map((e) => e.fatto);

/** L'ultimo voto del partito: quota e seggi; null se non c'era o non si è ancora votato. */
export const ultimoVoto = (v: Vista, id: string): VotiPartito | null => v.elezioni[0]?.voti.find((x) => x.partito === id) ?? null;
