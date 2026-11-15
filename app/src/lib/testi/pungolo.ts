import motti from "./pungolo.json";
import { variante } from "./t";

/** Il motto del ministro: hash(nome, dicastero) mod n sui motti del dicastero (spec:319); null se il dicastero non ne ha. */
export function mottoDi(nome: string, dicastero: string): string | null {
  const m = (motti.motti as Record<string, string[]>)[dicastero];
  return m?.length ? m[variante(m.length, nome, dicastero)] : null;
}
