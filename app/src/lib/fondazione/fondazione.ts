import { t, type Chiave } from "$lib/testi/t";
import type { PartitoCatalogo } from "$lib/tipi/PartitoCatalogo";
import type { VocePosizione } from "$lib/tipi/VocePosizione";

/** Il limite di `fonda` (sim-core, legislatura/fondazione.rs). */
export const LUNGHEZZA_NOME = 40;

/** Solo spazi = vuoto. */
export function nomeValido(nome: string): boolean {
  return nome.trim() !== "";
}

/** L'anteprima dello scenario corrente: arrivata, in volo, o finita in un rifiuto o in un guasto. */
export type StatoAnteprima = "pronta" | "in_arrivo" | "fallita";

/**
 * Il motivo per cui «Fonda il partito» è spento, o null. Prima l'anteprima: senza quella dello scenario corrente si
 * fonderebbe una partita mai mostrata (il guscio riusa il seme dell'ultima anteprima). Poi il nome, poi il posto.
 */
export function cosaManca(nome: string, posto: number | null, anteprima: StatoAnteprima): Chiave | null {
  if (anteprima === "in_arrivo") return "fondazione.anteprima_in_arrivo";
  if (anteprima === "fallita") return "fondazione.anteprima_fallita";
  if (!nomeValido(nome)) return "fondazione.manca_il_nome";
  return posto === null ? "fondazione.manca_il_posto" : null;
}

/** I partiti dei due lati del posto; fuori dall'asse un lato manca. */
export function fra(posto: number, partiti: PartitoCatalogo[]): [PartitoCatalogo | undefined, PartitoCatalogo | undefined] {
  const ordinati = [...partiti].sort((a, b) => a.posizione - b.posizione);
  return [ordinati.findLast((p) => p.posizione < posto), ordinati.find((p) => p.posizione > posto)];
}

/** «fra X e Y»; con un lato solo «oltre X»; senza partiti, niente. */
export function fraTesto(posto: number, partiti: PartitoCatalogo[]): string {
  const [a, b] = fra(posto, partiti);
  if (a && b) return t("fondazione.fra", { a: a.nome, b: b.nome });
  const x = a ?? b;
  return x ? t("fondazione.oltre", { a: x.nome }) : "";
}

/** «Con chi correre»: i sette, prima i disponibili, ciascuno con la causa del core; ordine dei dati a parità. */
export function conChiCorrere(
  v: VocePosizione,
  partiti: PartitoCatalogo[],
): { partito: PartitoCatalogo; disponibile: boolean; causa: string }[] {
  // un id che il catalogo non conosce resta, col nome grezzo, come in TabellaPosti
  const voci = v.disponibilita.map((d) => ({
    partito: partiti.find((p) => p.id === d.partito) ?? { id: d.partito, nome: d.partito, posizione: NaN },
    disponibile: d.disponibile,
    causa: d.causa,
  }));
  // sort è stabile: a parità resta l'ordine dei dati
  return voci.sort((a, b) => Number(b.disponibile) - Number(a.disponibile));
}
