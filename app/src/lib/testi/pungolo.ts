import motti from "./pungolo.json";
import { hash, variante } from "./t";
import type { TipoFilo } from "$lib/schema/schema";
import type { Famiglia } from "$lib/tipi/Famiglia";
import type { TipoVoce } from "$lib/tipi/TipoVoce";
import type { VoceEsito } from "$lib/tipi/VoceEsito";
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

/** Una chiave per tipo; gli eventi per famiglia (spec:319). Vuota = nessun titolo: l'occhiello sale a titolo. */
export type ChiaveTitolo = Exclude<TipoVoce, "evento_nato" | "evento_risolto"> | `${"evento_nato" | "evento_risolto"}|${Famiglia}`;

// tipato sul JSON: un tipo nuovo del core senza chiave fa fallire `npm run check`
const TITOLI: Record<ChiaveTitolo, string[]> = motti.titoli;
const DIDASCALIE: Partial<Record<ChiaveTitolo, string>> = motti.didascalie;
const BATTUTE: Partial<Record<TipoFilo, string>> = motti.battute;

export function chiaveDi(v: VoceEsito): ChiaveTitolo {
  return (v.tipo === "evento_nato" || v.tipo === "evento_risolto" ? `${v.tipo}|${v.famiglia}` : v.tipo) as ChiaveTitolo;
}

/** (hash(id della partita, chiave) + turno + posto fra le voci dello stesso tipo nella pagina) mod n, senza stato (spec:319); null se la chiave non ha titoli. */
export function titoloDi(id: string, v: VoceEsito, turno: number, posto: number): string | null {
  const chiave = chiaveDi(v);
  const titoli = TITOLI[chiave];
  return titoli?.length ? titoli[(hash(`${id}|${chiave}`) + turno + posto) % titoli.length] : null;
}

export const didascaliaDi = (v: VoceEsito): string | null => DIDASCALIE[chiaveDi(v)] ?? null;

export const battutaDi = (filo: TipoFilo): string | null => BATTUTE[filo] ?? null;
