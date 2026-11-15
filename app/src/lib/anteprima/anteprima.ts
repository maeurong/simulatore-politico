// L'anteprima di una misura: ciò che il cartellino legge dalla Vista (storie 113-117). Le cifre e le fasce sono del core.
import { elenco, numero, t, type Chiave } from "$lib/testi/t";
import { euro } from "$lib/governo/governo";
import { inOrdine } from "$lib/partiti/schieramento";
import type { EffettoStimato as Effetto } from "$lib/tipi/EffettoStimato";
import type { VoceLinea } from "$lib/tipi/VoceLinea";
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

/** «+275 mln €», «−710 mln €», «Saldo zero»: la spesa col suo segno, uguale nella bozza e nel cartellino. */
export function spesa(costoAnnuo: number): string {
  return costoAnnuo === 0 ? t("anteprima.saldo_zero") : `${costoAnnuo > 0 ? "+" : "−"}${euro(Math.abs(costoAnnuo))}`;
}

/** «migliora · molto»: il verso e la fascia di un effetto; la fascia passa da una chiave, non dall'enum del core. */
export function verso(e: Pick<Effetto, "migliora" | "fascia">): string {
  return t(e.migliora ? "anteprima.migliora" : "anteprima.peggiora", { fascia: t(`anteprima.fascia_${e.fascia}` as Chiave) });
}

/** Il nome della grandezza senza l'unità fra parentesi. */
export function nomeGrandezza(v: Vista, id: string): string {
  return (v.grandezze.find((g) => g.id === id)?.nome ?? id).replace(/ \(.*\)$/, "");
}

/** «Salario: migliora · molto, subito». */
export function rigaEffetto(v: Vista, e: Effetto): string {
  return t("anteprima.effetto_riga", { nome: nomeGrandezza(v, e.grandezza), verso: verso(e), quando: quando(e.ritardo) });
}

/** I partiti che votano sì (o no) secondo le linee, nell'ordine della Vista; chi non ha linea conta fra i no. */
export function votano(v: Vista, linee: VoceLinea[], si: boolean): string {
  return elenco(inOrdine(v).filter((p) => (linee.find((l) => l.partito === p.id)?.si ?? false) === si).map((p) => p.nome), ", ");
}

/** «205» se la forbice è un punto, «220–235» altrimenti. */
export function forbice([da, a]: [number, number]): string {
  return da === a ? numero(da) : `${numero(da)}–${numero(a)}`;
}
