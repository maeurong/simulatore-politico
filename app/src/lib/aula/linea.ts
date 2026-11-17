import { t, numero } from "$lib/testi/t";
import type { CostoLinea } from "$lib/tipi/CostoLinea";
import type { Vista } from "$lib/tipi/Vista";

/** `CostoLinea.percepita` è la posizione dopo il voto, non lo spostamento: si dice da dove parte il giocatore e dove arriva;
 *  uguali alla cifra mostrata (due decimali), «resta». */
export function costoDellaLinea(v: Vista, c: CostoLinea): string {
  const da = numero(v.partiti.find((p) => p.id === v.giocatore)?.posizione_percepita ?? NaN, 2);
  const a = numero(c.percepita, 2);
  const franchi = { c: numero(c.franchi_camera, 1), s: numero(c.franchi_senato, 1) };
  return t(da === a ? "aula.costo_linea_resta" : "aula.costo_linea", { da, a, ...franchi });
}
