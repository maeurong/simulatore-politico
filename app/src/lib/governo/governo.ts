// Governo: le dieci carte dell'album, l'obiettivo del ministro e la sua posizione accanto alla tua (storie 98-103).
import type { MotivoEvidenza } from "$lib/tipi/MotivoEvidenza";
import type { Vista } from "$lib/tipi/Vista";
import type { VoceAgenda } from "$lib/tipi/VoceAgenda";
import type { VoceAtto } from "$lib/tipi/VoceAtto";
import type { VoceMinistero } from "$lib/tipi/VoceMinistero";
import type { VoceMinistro } from "$lib/tipi/VoceMinistro";
import { comeErrore, testoRifiuto } from "$lib/guscio/chiamate";
import { nomeDi } from "$lib/partiti";
import { numero, segnato, t } from "$lib/testi/t";

export type StatoCarta = "preavviso" | "rodaggio" | "tecnico" | "dimissioni_chieste" | "interim";
export type Figurina = {
  indice: number; dicastero: string; nomeDicastero: string;
  ministro: VoceMinistro | null; stati: StatoCarta[];
  /** soddisfazione arrotondata */
  voto: number | null;
  /** somma delle variazioni del ministro (gli ultimi due turni, comandi compresi) */
  freccia: number;
  /** agenda.evidenza; vuota = niente filo rosso */
  evidenza: MotivoEvidenza[];
  agenda: VoceAgenda | null; senzaAgenda: string | null;
};

/** Le dieci carte nell'ordine di Vista.dicasteri. */
export function figurine(v: Vista): Figurina[] {
  return v.dicasteri.map((d, i) => {
    const m = v.ministeri.find((x) => x.dicastero === d.id)!;
    const x = m.ministro;
    const stati: StatoCarta[] = [];
    if (!x) stati.push("interim");
    else {
      if (m.dimissioni_chieste) stati.push("dimissioni_chieste");
      else if (x.minaccia) stati.push("preavviso");
      if (x.rodaggio) stati.push("rodaggio");
      if (x.tecnico) stati.push("tecnico");
    }
    return {
      indice: i + 1, dicastero: d.id, nomeDicastero: d.nome, ministro: x, stati,
      voto: x ? Math.round(x.soddisfazione) : null,
      freccia: x ? x.variazioni.reduce((s, y) => s + y.delta, 0) : 0,
      evidenza: m.agenda?.evidenza ?? [], agenda: m.agenda, senzaAgenda: m.motivo_senza_agenda,
    };
  });
}

/** L'indicatore di servizio del dicastero (VoceDicastero.indicatore in Vista.grandezze) contro l'obiettivo del ministro. */
export function obiettivo(v: Vista, dicastero: string): { nome: string; valore: number | null; obiettivo: number | null; verso: string } {
  const g = v.grandezze.find((x) => x.id === v.dicasteri.find((d) => d.id === dicastero)?.indicatore);
  const m = v.ministeri.find((x) => x.dicastero === dicastero);
  return { nome: g?.nome ?? "", valore: g?.valori[0] ?? null, obiettivo: m?.ministro?.obiettivo ?? null, verso: g?.verso ?? "sale" };
}

/** La posizione del ministro accanto a quella del giocatore (posizione_percepita del suo partito). */
export function posizioni(v: Vista, m: VoceMinistro): { ministro: number; tua: number } {
  return { ministro: m.posizione, tua: v.partiti.find((p) => p.id === v.giocatore)?.posizione_percepita ?? 0 };
}

/** «2,5 mld €», «12 mln €», «850.000 €». */
export function euro(c: number): string {
  if (c >= 1e9) return `${numero(c / 1e9, 1)} mld €`;
  if (c >= 1e6) return `${numero(c / 1e6)} mln €`;
  return `${numero(c)} €`;
}

/** «Ferma −18», «Ferma +3», «Ferma» a zero (lettura 2). */
export function etichettaFerma(costo: number): string {
  const r = Math.round(costo);
  return r === 0 ? t("governo.ferma") : `${t("governo.ferma")} ${segnato(r)}`;
}

/** Gli atti di Dirotta: gli altri tre del dicastero. */
export const attiDirotta = (m: VoceMinistero): VoceAtto[] => m.atti.filter((a) => a.id !== m.agenda?.atto);

/** La regione bersaglio si sceglie solo per Sviluppo economico e Infrastrutture (spec:387). */
export const conRegione = (m: VoceMinistero): boolean => m.dicastero === "sviluppo_economico" || m.dicastero === "infrastrutture_trasporti";

type Costi = Record<string, number | string>;
/** I costi di Dirotta, uno per atto o per regione, in sequenza; un rifiuto resta come testo. `vivo` falso ferma il giro (una nuova apertura annulla la vecchia). */
export async function costiDirotta(
  chiedi: (atto: string | null, regione: string | null) => Promise<number>, atti: string[], regioni: string[], vivo: () => boolean = () => true,
): Promise<{ atti: Costi; regioni: Costi }> {
  const una = async (a: string | null, r: string | null) => chiedi(a, r).catch((e) => testoRifiuto(comeErrore(e)));
  const fuori = { atti: {} as Costi, regioni: {} as Costi };
  for (const a of atti) if (vivo()) fuori.atti[a] = await una(a, null);
  for (const r of regioni) if (vivo()) fuori.regioni[r] = await una(null, r);
  return fuori;
}

/** Il prezzo delle dimissioni chieste, in parole: gratis col tecnico, niente col tuo partito, N punti d'intesa all'alleato. */
export function prezzoDimissioni(v: Vista, m: VoceMinistero): string {
  if (m.ministro?.tecnico) return t("governo.dimissioni_tecnico");
  if (m.prezzo_dimissioni <= 0 || !m.ministro) return t("governo.dimissioni_tuo");
  return t("governo.dimissioni_alleato", { n: numero(m.prezzo_dimissioni), partito: nomeDi(v, m.ministro.partito) });
}

/** La molla del prototipo 18 come curva `linear()` su una sola Web Animation (movimento-e-scala-in-wkwebview.md:45): sottosmorzata, supera 1 di circa il 10% e si assesta. */
export function molla(punti = 40): string {
  const z = 0.6, w = 8.3, wd = w * Math.sqrt(1 - z * z);
  const x = (s: number) => 1 - Math.exp(-z * w * s) * (Math.cos(wd * s) + ((z * w) / wd) * Math.sin(wd * s));
  const v = Array.from({ length: punti - 1 }, (_, i) => +x((i + 1) / punti).toFixed(3));
  return `linear(0, ${v.join(", ")}, 1)`;
}
