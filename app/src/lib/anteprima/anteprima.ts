// L'anteprima di una misura: ciò che il cartellino legge dalla Vista (storie 113-117). Le cifre e le fasce sono del core.
import { numero, t } from "$lib/testi/t";
import type { Anteprima } from "$lib/tipi/Anteprima";
import type { ValoreProposto } from "$lib/tipi/ValoreProposto";
import type { Vista } from "$lib/tipi/Vista";
import type { VoceMisura } from "$lib/tipi/VoceMisura";

/** Ciò che il cartellino mostra: un'anteprima del core o una misura già in calendario (oggi porta costo, effetti e gruppi). */
export type Misurabile = Anteprima | VoceMisura;

/** «subito», «dopo 4 turni», «dopo 40 turni · 10 anni». */
export function quando(ritardo: number): string {
  if (ritardo <= 0) return t("anteprima.subito");
  const turni = t("anteprima.dopo_turni", { n: ritardo });
  return ritardo >= 8 ? `${turni} · ${t("anteprima.anni", { n: numero(ritardo / 4) })}` : turni;
}

/** Una riga per disposizione: il gradino in vigore e quello della misura. Una disposizione che la Vista non conosce non fa riga. */
export function cosaCambia(v: Vista, valori: ValoreProposto[]): { disposizione: string; ora: string; con: string }[] {
  return valori.flatMap((x) => {
    const d = v.disposizioni.find((y) => y.id === x.disposizione);
    if (!d) return [];
    const nome = (id: string) => d.gradini.find((g) => g.id === id)?.nome ?? id;
    return [{ disposizione: d.nome, ora: nome(d.in_vigore), con: nome(x.gradino) }];
  });
}

/** Chi stima: il ministro del dicastero proponente con la sua capacità; null = stima neutra (storia 114). */
export function stimatore(v: Vista, proponente: string | null): { nome: string; dicastero: string; capacita: number } | null {
  const m = proponente ? v.ministeri.find((x) => x.dicastero === proponente)?.ministro : null;
  if (!m) return null;
  return { nome: m.nome, dicastero: v.dicasteri.find((d) => d.id === proponente)?.nome ?? proponente!, capacita: m.capacita };
}

/** Le regioni (id) dove valgono le disposizioni della misura; vuoto = tutto il Paese. */
export function perimetro(v: Vista, valori: ValoreProposto[]): string[] {
  return [...new Set(valori.flatMap((x) => v.disposizioni.find((d) => d.id === x.disposizione)?.perimetro ?? []))];
}
