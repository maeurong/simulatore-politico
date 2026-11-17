import motti from "./pungolo.json";
import { variante } from "./t";
import type { Trimestre } from "$lib/tipi/Trimestre";

/** Il motto del ministro: hash(nome, dicastero) mod n sui motti del dicastero (spec:319); null se il dicastero non ne ha. */
export function mottoDi(nome: string, dicastero: string): string | null {
  const m = (motti.motti as Record<string, string[]>)[dicastero];
  return m?.length ? m[variante(m.length, nome, dicastero)] : null;
}

/** Il titolo della fine senza seggi: hash(nome, scenario, trimestre d'inizio) mod 16 — li hanno la bozza e l'albo, quindi la stessa partita ha lo stesso titolo. */
export function titoloFine(nome: string, scenario: string, inizio: Trimestre): string {
  return motti.titoli_fine[variante(motti.titoli_fine.length, nome, scenario, inizio.anno, inizio.trimestre)];
}
