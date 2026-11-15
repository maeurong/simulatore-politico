// La legge in composizione: i valori scelti nelle Disposizioni, la sua forma, il bottone e il comando (storie 107-108).
// La UI non ripete le regole sulle miscele (un dicastero per legge, la costituzionale da sola): le dice il core col suo rifiuto.
import { t, type Chiave } from "$lib/testi/t";
import type { Comando } from "$lib/tipi/Comando";
import type { ValoreProposto } from "$lib/tipi/ValoreProposto";
import type { Vista } from "$lib/tipi/Vista";
import type { VoceDisposizione } from "$lib/tipi/VoceDisposizione";
import type { VoceGradino } from "$lib/tipi/VoceGradino";
import type { VoceMisura } from "$lib/tipi/VoceMisura";

export type Bozza = ValoreProposto[];

/** Mette o toglie un gradino: il gradino in vigore, o già in bozza, toglie la disposizione; un altro prende il posto del precedente. */
export function cambia(v: Vista, b: Bozza, disposizione: string, gradino: string): Bozza {
  const resto = b.filter((x) => x.disposizione !== disposizione);
  const ora = b.find((x) => x.disposizione === disposizione)?.gradino;
  const vigente = v.disposizioni.find((d) => d.id === disposizione)?.in_vigore;
  return gradino === vigente || gradino === ora ? resto : [...resto, { disposizione, gradino }];
}

/** La forma dalla norma delle disposizioni (lettura 3); null a bozza vuota. */
export function forma(v: Vista, b: Bozza): "riforma" | "elettorale" | "costituzionale" | null {
  const norme = b.map((x) => v.disposizioni.find((d) => d.id === x.disposizione)?.norma);
  if (!norme.length) return null;
  return norme.includes("costituzionale") ? "costituzionale" : norme.includes("elettorale") ? "elettorale" : "riforma";
}

/** Il bottone della bozza; null a bozza vuota. */
export function etichettaBozza(v: Vista, b: Bozza): Chiave | null {
  const f = forma(v, b);
  if (!f) return null;
  if (!v.governa) return "aula.porta_nel_posto";
  return f === "costituzionale" ? "aula.proponi_costituzionale" : f === "elettorale" ? "aula.presenta_elettorale" : "aula.presenta_riforma";
}

export const comandoBozza = (v: Vista, b: Bozza): Comando => ({ tipo: v.governa ? "presenta_legge" : "presenta_proposta", valori: b });

/** Lo stato di un gradino nella riga aperta. */
export function statoGradino(d: VoceDisposizione, b: Bozza, g: VoceGradino): "in_vigore" | "in_bozza" | "in_arrivo" | "bloccato" | "libero" {
  if (g.bloccato) return "bloccato";
  if (b.some((x) => x.disposizione === d.id && x.gradino === g.id)) return "in_bozza";
  if (d.in_vigore === g.id) return "in_vigore";
  return d.in_arrivo.some((a) => a.gradino === g.id) ? "in_arrivo" : "libero";
}

/** Le disposizioni ordinarie e costituzionali per dicastero, nell'ordine di Vista.dicasteri; le senza dicastero in «Altre», la Costituzione in fondo. */
export function perDicastero(v: Vista): { dicastero: string | null; nome: string; voci: VoceDisposizione[] }[] {
  const ammesse = v.disposizioni.filter((d) => d.norma !== "elettorale");
  const gruppo = (dicastero: string | null, nome: string, voci: VoceDisposizione[]) => ({ dicastero, nome, voci });
  return [
    ...v.dicasteri.map((c) => gruppo(c.id, c.nome, ammesse.filter((d) => d.norma === "ordinaria" && d.dicastero === c.id))),
    gruppo(null, t("aula.disp_altre"), ammesse.filter((d) => d.norma === "ordinaria" && (d.dicastero === null || !v.dicasteri.some((c) => c.id === d.dicastero)))),
    gruppo(null, t("aula.disp_costituzione"), ammesse.filter((d) => d.norma === "costituzionale")),
  ].filter((g) => g.voci.length);
}

/** La misura in calendario che il testo del rifiuto nomina, se c'è. */
export const misuraNominata = (v: Vista, testo: string): VoceMisura | null => v.calendario.find((m) => testo.includes(m.nome)) ?? null;
