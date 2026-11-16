// Il Bilancio: le modifiche che la UI scrive sul bilancio in memoria; costi, tetto e perdita reale li dice il core.
import { euro } from "$lib/governo/governo";
import { comeErrore, guscio, testoRifiuto } from "$lib/guscio/chiamate";
import { numero, t } from "$lib/testi/t";
import type { Bilancio } from "$lib/tipi/Bilancio";
import type { BudgetDicastero } from "$lib/tipi/BudgetDicastero";
import type { RiepilogoBilancio } from "$lib/tipi/RiepilogoBilancio";
import type { Vista } from "$lib/tipi/Vista";
import type { Voce } from "$lib/tipi/Voce";
import type { VoceEmendamento } from "$lib/tipi/VoceEmendamento";
import type { VoceMisura } from "$lib/tipi/VoceMisura";

export type Imposta = { tipo: "irpef"; indice: number } | { tipo: "iva" | "ires" | "contributi" };

// al centesimo: 0,22 + 0,01 deve dare 0,23, la stessa cifra che il core scrive
const punto = (a: number, verso: 1 | -1) => Math.round((a + verso * 0.01) * 100) / 100;

/** Un punto in più o in meno sull'aliquota (lettura 3); non limita: i limiti li dice il core. */
export function ritocca(b: Bilancio, imposta: Imposta, verso: 1 | -1): Bilancio {
  const a = b.aliquote;
  if (imposta.tipo !== "irpef") return { ...b, aliquote: { ...a, [imposta.tipo]: punto(a[imposta.tipo], verso) } };
  const irpef = a.irpef.map((s, i) => (i === imposta.indice ? { ...s, aliquota: punto(s.aliquota, verso) } : s));
  return { ...b, aliquote: { ...a, irpef } };
}

/** Il passo del margine: mezzo punto della spesa della bozza (decisione 3). */
export const passoMargine = (bozza: BudgetDicastero): number => Math.round(bozza.spesa * 0.005);

/**
 * Aggiunge n passi dalla via data al bilancio `b`: dagli acquisti (lettura 2) o in disavanzo (sulla spesa); n < 0 li toglie.
 * Lavora a scarti su `b`: della bozza prende solo il passo. Quanti passi sono già dentro lo conta `bozza.passi`.
 */
export function allarga(b: Bilancio, bozza: Bilancio, dicastero: string, via: "funzionamento" | "disavanzo", passi: number): Bilancio {
  const z = bozza.dicasteri.find((d) => d.id === dicastero);
  if (!z || passi === 0) return b;
  const x = passi * passoMargine(z);
  const dicasteri = b.dicasteri.map((d) =>
    d.id !== dicastero ? d : via === "funzionamento" ? { ...d, margine: d.margine + x, acquisti: d.acquisti - x } : { ...d, margine: d.margine + x, spesa: d.spesa + x },
  );
  return { ...b, dicasteri };
}

/** Il funzionamento tagliato, sotto la barra: salari e acquisti del bilancio in memoria contro la bozza (spec:395); null se uguali. */
export function tagliato(b: BudgetDicastero, bozza: BudgetDicastero): { salari: number; acquisti: number } | null {
  const salari = b.salari - bozza.salari, acquisti = b.acquisti - bozza.acquisti;
  return salari || acquisti ? { salari, acquisti } : null;
}

/** «dentro il tetto: restano 1,0 punti» o «oltre il tetto di 0,4 punti: il controllo di aprile-giugno lo rileva». */
export function distanzaTetto(r: RiepilogoBilancio): string {
  const d = r.tetto - r.crescita_spesa_netta;
  return t(d >= 0 ? "bilancio.dentro_tetto" : "bilancio.oltre_tetto", { n: numero(Math.abs(d), 1) });
}

/**
 * La voce è già indicizzata in `b` (lettura 1): le sue cifre stanno al livello dei prezzi di `indicizzato`, la bozza che
 * il core ha indicizzato sulla stessa voce. Il confronto è sul livello, così un margine allargato dopo non lo cambia.
 */
export function indicizzata(b: Bilancio, indicizzato: Bilancio, voce: Voce): boolean {
  const dicasteri = voce.tipo === "scaglioni" ? [] : b.dicasteri.filter((d) => voce.tipo === "tutto" || d.id === voce.id);
  const soglie = voce.tipo === "dicastero" ? [] : b.aliquote.irpef.map((s, i) => [s, indicizzato.aliquote.irpef[i]] as const).filter(([s]) => s.fino_a !== null);
  return (
    dicasteri.every((d) => d.livello_prezzi === indicizzato.dicasteri.find((x) => x.id === d.id)?.livello_prezzi) &&
    soglie.every(([s, x]) => s.livello_prezzi === x?.livello_prezzi)
  );
}

/** La voce riportata alle cifre della bozza: i dicasteri interi, delle soglie solo `fino_a` e livello (l'aliquota resta). */
export function annullata(b: Bilancio, bozza: Bilancio, voce: Voce): Bilancio {
  const dicasteri = voce.tipo === "scaglioni" ? b.dicasteri : b.dicasteri.map((d) => (voce.tipo === "tutto" || d.id === voce.id ? (bozza.dicasteri.find((z) => z.id === d.id) ?? d) : d));
  const irpef = voce.tipo === "dicastero" ? b.aliquote.irpef : b.aliquote.irpef.map((s, i) => {
    const z = bozza.aliquote.irpef[i];
    return z ? { ...s, fino_a: z.fino_a, livello_prezzi: z.livello_prezzi } : s;
  });
  return { ...b, dicasteri, aliquote: { ...b.aliquote, irpef } };
}

/** «+6,2 mld €», «−13,3 mld €», «±0 €» (prototipo 22). */
export function euroSegnato(c: number): string {
  const s = euro(Math.abs(c));
  return /[1-9]/.test(s) ? `${c > 0 ? "+" : "−"}${s}` : `±${s}`;
}

/** La riga «se cade»: esercizio provvisorio da gennaio, o le dimissioni se si è già in esercizio provvisorio (storia 128). */
export function seCade(v: Vista): string {
  return v.bilancio.esercizio_provvisorio ? t("bilancio.se_cade_dimissioni") : t("bilancio.se_cade", { anno: v.bilancio.anno });
}

/**
 * «la qualità del servizio sanitario», «l’efficienza della riscossione», «il sostegno alle imprese».
 * shortcut: l'articolo dalla prima parola, giusto per i dieci indicatori dei dicasteri; un indicatore nuovo va provato qui.
 */
export function colArticolo(nome: string): string {
  const n = nome.charAt(0).toLowerCase() + nome.slice(1);
  if (/^[aeiouàèéìòù]/.test(n)) return `l’${n}`;
  return `${/^\S*o\b/.test(n) ? "il" : "la"} ${n}`;
}

/** Sotto la barra: «acquisti −777 mln € sulla bozza · la qualità del servizio sanitario peggiora con ritardo» (spec:395). */
export function rigaTagliato(x: { salari: number; acquisti: number }, indicatore: string): string {
  const parti = [x.salari ? `${t("bilancio.salari")} ${euroSegnato(x.salari)}` : "", x.acquisti ? `${t("bilancio.acquisti")} ${euroSegnato(x.acquisti)}` : ""];
  const riga = t("bilancio.sulla_bozza", { parti: parti.filter(Boolean).join(" · ") });
  return x.salari < 0 || x.acquisti < 0 ? `${riga} · ${t("bilancio.peggiora", { servizio: colArticolo(indicatore) })}` : riga;
}

/** `?dicastero=<id>` dal link «Indicizzalo» dei Conti: la riga da aprire nel lato, se il dicastero c'è. */
export const daUrl = (id: string | null, v: Vista): string | null => (id && v.dicasteri.some((d) => d.id === id) ? `dic:${id}` : null);

/** «Indicizza» solo con una perdita da recuperare di almeno 0,05 (prototipo 22:427). */
export const daIndicizzare = (p: number): boolean => p >= 0.05;

/** Sulla riga delle soglie: con «tutto» niente (l'annulla è in testata, 22:526), poi l'annulla, poi «indicizza» con una perdita sopra 0,05 (22:537). */
export function comandoSoglie(tutto: boolean, soglie: boolean, perdita: number | undefined): "annulla" | "indicizza" | null {
  if (tutto) return null;
  if (soglie) return "annulla";
  return perdita !== undefined && daIndicizzare(perdita) ? "indicizza" : null;
}

/** La misura del bilancio in calendario, se c'è. */
export const misuraBilancio = (v: Vista): VoceMisura | undefined => v.calendario.find((m) => m.forma === "bilancio");

/** Il riepilogo del bilancio in aula (storia 129), o il testo dell'errore. */
export const riepilogoPresentato = (): Promise<RiepilogoBilancio | string | null> => guscio.riepilogoPresentato().catch((e) => testoRifiuto(comeErrore(e)));

/** La forma della presentata: c'è un bilancio in aula e non lo si sta modificando per ripresentarlo; lì «Presenta» non c'è. */
export const presentata = (v: Vista, riaperta: boolean): boolean => v.bilancio.presentato && v.bilancio.in_aula !== null && !riaperta;

/** Il costo «se accogli»: in_aula indicizzato in sequenza sui dicasteri dell'emendamento, poi il riepilogo (lettura 5); un rifiuto resta testo. */
export async function seAccogli(
  chiedi: { indicizza: typeof guscio.indicizza; riepilogo: typeof guscio.riepilogoBilancio },
  inAula: Bilancio,
  e: VoceEmendamento,
): Promise<RiepilogoBilancio | string> {
  try {
    let b = inAula;
    for (const id of e.dicasteri) b = await chiedi.indicizza(b, { tipo: "dicastero", id });
    return await chiedi.riepilogo(b);
  } catch (x) {
    return testoRifiuto(comeErrore(x));
  }
}
