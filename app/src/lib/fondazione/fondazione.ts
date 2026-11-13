import type { Chiave } from "$lib/testi/t";
import type { PartitoCatalogo } from "$lib/tipi/PartitoCatalogo";
import type { VocePosizione } from "$lib/tipi/VocePosizione";

/** Il limite di `fonda` (sim-core, legislatura/fondazione.rs). */
export const LUNGHEZZA_NOME = 40;

/** Solo spazi = vuoto. */
export function nomeValido(nome: string): boolean {
  return nome.trim() !== "";
}

/** Il motivo per cui «Fonda il partito» è spento, o null: prima il nome, poi il posto. */
export function cosaManca(nome: string, posto: number | null): Chiave | null {
  if (!nomeValido(nome)) return "fondazione.manca_il_nome";
  return posto === null ? "fondazione.manca_il_posto" : null;
}

/** I partiti dei due lati del posto: «fra X e Y». */
export function fra(posto: number, partiti: PartitoCatalogo[]): [PartitoCatalogo, PartitoCatalogo] {
  const ordinati = [...partiti].sort((a, b) => a.posizione - b.posizione);
  return [ordinati.findLast((p) => p.posizione < posto)!, ordinati.find((p) => p.posizione > posto)!];
}

/** «Con chi correre»: i sette, prima i disponibili, ciascuno con la causa del core; ordine dei dati a parità. */
export function conChiCorrere(
  v: VocePosizione,
  partiti: PartitoCatalogo[],
): { partito: PartitoCatalogo; disponibile: boolean; causa: string }[] {
  // ponytail: un id che il catalogo non conosce si salta; il core li prende dallo stesso scenario
  const voci = v.disponibilita.flatMap((d) => {
    const partito = partiti.find((p) => p.id === d.partito);
    return partito ? [{ partito, disponibile: d.disponibile, causa: d.causa }] : [];
  });
  // sort è stabile: a parità resta l'ordine dei dati
  return voci.sort((a, b) => Number(b.disponibile) - Number(a.disponibile));
}
