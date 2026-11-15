// I referendum dell'Aula (storia 110): la misura a cui il confermativo risponde e come si legge un esito. Cifre e flag sono del core.
import type { EsitoVoto } from "$lib/tipi/EsitoVoto";
import type { EsitoReferendum } from "$lib/tipi/EsitoReferendum";
import type { Vista } from "$lib/tipi/Vista";
import type { VistaReferendum } from "$lib/tipi/VistaReferendum";

/** La costituzionale della seconda deliberazione senza i due terzi a cui il confermativo risponde: il core la ritrova da `misura`, la Vista
 *  non la porta; si ricava dagli esiti per nome. Nessun esito che torni = nessuna scelta da offrire. */
export const misuraDelConfermativo = (v: Vista, r: VistaReferendum): number | null =>
  v.esiti
    .filter((e) => e.forma === "costituzionale" && e.deliberazione === 2 && e.nome === r.nome && e.decaduta === null)
    .reduce<EsitoVoto | null>((p, e) => (!p || e.data.anno * 4 + e.data.trimestre > p.data.anno * 4 + p.data.trimestre ? e : p), null)?.misura ?? null;

/** Solo l'abrogativo ha il quorum. */
export const esitoDi = (e: EsitoReferendum): "decaduto" | "senza_quorum" | "approvato" | "respinto" =>
  e.decaduto !== null ? "decaduto" : e.tipo === "abrogativo" && !e.quorum ? "senza_quorum" : e.approvato ? "approvato" : "respinto";
