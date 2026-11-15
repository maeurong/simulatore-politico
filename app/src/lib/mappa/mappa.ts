import type { Vista } from "$lib/tipi/Vista";

/** L'id della regione di indice i nei dati, e il contrario: l'ordine è quello di Vista.sondaggio.regioni. */
export const idDi = (v: Vista, indice: number): string | null => v.sondaggio.regioni[indice]?.id ?? null;
export function indiceDi(v: Vista, id: string): number | null {
  const i = v.sondaggio.regioni.findIndex((r) => r.id === id);
  return i < 0 ? null : i;
}

export function estremi(valori: (number | null)[]): { min: number; max: number; passo: number } | null {
  const x = valori.filter((n): n is number => n !== null);
  if (!x.length) return null;
  const min = Math.min(...x), max = Math.max(...x);
  return { min, max, passo: (max - min) / 5 };
}

/** Cinque classi a passi uguali fra il minimo e il massimo dei valori; null resta null (senza dato). */
export function classi(valori: (number | null)[]): (0 | 1 | 2 | 3 | 4 | null)[] {
  const e = estremi(valori);
  return valori.map((x) => (x === null || !e || !e.passo ? (x === null ? null : 0) : (Math.min(4, Math.floor((x - e.min) / e.passo)) as 0 | 1 | 2 | 3 | 4)));
}

/** La regione accanto nell'ordine della lista, girando agli estremi. */
export function accanto(id: string, ordine: string[], verso: 1 | -1): string {
  const i = ordine.indexOf(id);
  return ordine[(i + verso + ordine.length) % ordine.length];
}
