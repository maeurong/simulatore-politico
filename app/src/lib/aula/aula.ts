// Aula: i posti del calendario, chi può porre la fiducia o scegliere la linea, gli ultimi voti (storie 104-106, 111).
import { t, type Chiave } from "$lib/testi/t";
import type { EsitoVoto } from "$lib/tipi/EsitoVoto";
import type { Vista } from "$lib/tipi/Vista";
import type { VoceMisura } from "$lib/tipi/VoceMisura";

export type Posto =
  | { tipo: "bilancio"; misura: VoceMisura }
  | { tipo: "decreti" | "ordinaria" | "costituzionale" | "opposizione"; misura: VoceMisura | null; numero?: number; di?: number };

/** I posti del calendario: decreti in precedenza, poi le ordinarie da Vista.posti, la costituzionale, l'opposizione;
 *  nel turno del bilancio un posto solo. Le misure riempiono i posti nell'ordine di Vista.calendario. */
export function calendario(v: Vista): Posto[] {
  const bilancio = v.calendario.find((m) => m.forma === "bilancio");
  if (bilancio) return [{ tipo: "bilancio", misura: bilancio }];
  // il posto riservato lo porta chi ha un promotore: fuori dai conti ordinari (VistaPosti)
  const libere = v.calendario.filter((m) => m.promotore === null);
  const decreti = libere.filter((m) => m.forma === "conversione");
  const ordinarie = libere.filter((m) => m.forma === "riforma" || m.forma === "elettorale");
  const costituzionali = libere.filter((m) => m.forma === "costituzionale");
  // le misure in calendario contano più dei conteggi: una che eccede i posti ne prende uno in più, non sparisce
  const ordinari = Math.max(v.posti.ordinari, ordinarie.length), cos = Math.max(v.posti.costituzionali, costituzionali.length);
  return [
    ...(decreti.length ? decreti.map((misura) => ({ tipo: "decreti" as const, misura })) : [{ tipo: "decreti" as const, misura: null }]),
    ...Array.from({ length: ordinari }, (_, i) => ({ tipo: "ordinaria" as const, misura: ordinarie[i] ?? null, numero: i + 1, di: ordinari })),
    ...Array.from({ length: cos }, (_, i) => ({ tipo: "costituzionale" as const, misura: costituzionali[i] ?? null })),
    { tipo: "opposizione", misura: v.calendario.find((m) => m.promotore !== null) ?? null },
  ];
}

/** Si vota alla chiusura di questo turno. */
export const votaOra = (v: Vista, m: VoceMisura): boolean => m.voto.anno === v.data.anno && m.voto.trimestre === v.data.trimestre;

/** La fiducia si pone solo sulle misure del governo, fuori dalla crisi, non due volte; non sulla costituzionale né sul bilancio, e
 *  nel turno del bilancio solo sulle conversioni (parlamento/emendamenti.rs:247-277). */
export const fiduciaPonibile = (v: Vista, m: VoceMisura): boolean => {
  if (!v.governa || v.crisi || m.promotore !== null || m.fiducia) return false;
  if (m.forma === "conversione") return true;
  return (m.forma === "riforma" || m.forma === "elettorale") && !v.calendario.some((x) => x.forma === "bilancio");
};

/** La linea del giocatore si sceglie all'opposizione su ogni misura, al governo sulle proposte dell'opposizione. */
export const lineaScelta = (v: Vista, m: VoceMisura): boolean => !v.governa || (m.promotore !== null && m.promotore !== "giocatore");

/** Gli ultimi voti, dal più recente. */
export const ultimiVoti = (v: Vista): EsitoVoto[] =>
  v.esiti.map((e, i) => ({ e, i })).sort((a, b) => b.e.data.anno - a.e.data.anno || b.e.data.trimestre - a.e.data.trimestre || a.i - b.i).map((x) => x.e);

/** «si vota a fine turno» o «si vota a fine luglio–settembre 2027»; con `breve` senza «si vota a». */
export function quando(v: Vista, d: { anno: number; trimestre: number }, breve = false): string {
  const adesso = d.anno === v.data.anno && d.trimestre === v.data.trimestre;
  const periodo = t(`cornice.trimestre_${d.trimestre}` as Chiave, { anno: d.anno }).toLowerCase();
  return t(adesso ? (breve ? "aula.fine_turno" : "cornice.si_vota_a_fine_turno") : breve ? "aula.fine_periodo" : "aula.si_vota_a", { periodo });
}

/** Chi porta la misura: il posto riservato, il ministro del dicastero, il governo, o il giocatore. */
export function chiLaPorta(v: Vista, m: VoceMisura, nomeDi: (id: string) => string): string {
  if (m.promotore) return m.promotore === "giocatore" ? t("aula.tua_proposta") : t("aula.proposta_di", { partito: nomeDi(m.promotore) });
  if (m.proponente) return t("aula.del_ministro", { dicastero: v.dicasteri.find((d) => d.id === m.proponente)?.nome ?? m.proponente });
  return t(m.forma === "bilancio" || !v.governa ? "aula.del_governo" : "aula.presentata_da_te");
}

/** La forma in parole; la costituzionale dice a che deliberazione è. */
export const formaDi = (m: { forma: string; deliberazione: number }): string =>
  m.forma === "costituzionale" ? t("aula.forma_costituzionale", { n: m.deliberazione || 1 }) : t(`aula.forma_${m.forma}` as Chiave);

/** Il rifiuto vale per la Vista e per la misura in cui è nato: cambiando l'una o l'altra non resta appeso a un'altra misura. */
export const rifiutoPer = <R extends { vista: Vista | null; misura: number | null }>(r: R | null, v: Vista, misura: number | null): R | null =>
  r && r.vista === v && r.misura === misura ? r : null;

/** `da` è l'elemento col fuoco prima del comando, se stava nel dettaglio. Se è sparito col suo effetto il fuoco passa al primo emendamento
 *  aperto e abilitato, o al dettaglio; se è ancora montato, o il fuoco stava altrove, non si tocca niente. */
export function riportaFuoco(lato: HTMLElement, da: Element | null): void {
  if (!da || da.isConnected) return;
  (lato.querySelector<HTMLElement>('.az button:not([disabled]):not([aria-disabled="true"])') ?? lato).focus({ preventScroll: true });
}
