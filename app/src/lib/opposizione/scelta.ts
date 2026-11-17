/** La riga dell'ordine del giorno aperta nel dettaglio a lato; `id` distingue il referendum (abrogativo, confermativo) e la misura. */
export type Scelta = { tipo: "sfiducia" | "ministro" | "referendum" | "misura"; id: string | null };
