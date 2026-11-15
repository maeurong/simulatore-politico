// Governo: le dieci carte dell'album, l'obiettivo del ministro e la sua posizione accanto alla tua (storie 98-103).
import type { MotivoEvidenza } from "$lib/tipi/MotivoEvidenza";
import type { Vista } from "$lib/tipi/Vista";
import type { VoceAgenda } from "$lib/tipi/VoceAgenda";
import type { VoceMinistro } from "$lib/tipi/VoceMinistro";
import { numero } from "$lib/testi/t";

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
