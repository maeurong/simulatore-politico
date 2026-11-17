// Il registro letto dalle chiusure: la UI filtra, collega e nomina, non decide (spec:410, piano 11f letture 5 e 10).
import { periodo } from "$lib/edicola/edicola";
import { REGIONI } from "$lib/mappa/regioni";
import { SEZIONE_DI } from "$lib/esito/esito";
import { elenco, t, type Chiave } from "$lib/testi/t";
import type { Sezione } from "$lib/ossatura";
import type { Vista } from "$lib/tipi/Vista";
import type { VoceEsito } from "$lib/tipi/VoceEsito";

export type Turno = { turno: number; voci: VoceEsito[]; critiche: number; grave: boolean; voto: boolean };
/** Un turno che il guscio non ha reso: resta in elenco, col suo guasto. */
export type Guasto = { turno: number; guasto: string };
export type Soggetto = { tipo: "partito" | "dicastero" | "regione" | "disposizione" | "bilancio"; id: string };
export type Filtro = { criticita: "tutte" | "notevoli" | "critiche"; sezione: Sezione | null; soggetto: Soggetto | null };

type Campi = Pick<VoceEsito, "partito" | "dicastero" | "regione" | "misura" | "disposizioni">;

export const turnoDa = (turno: number, voci: VoceEsito[]): Turno => ({
  turno,
  voci,
  critiche: voci.filter((v) => v.criticita === "critica").length,
  grave: voci.some((v) => v.tono === "grave"),
  voto: voci.some((v) => v.tipo === "risultato_del_voto"),
});

// Una misura senza disposizioni è la legge di bilancio (lettura 5).
export const tocca = (o: Campi, s: Soggetto): boolean => {
  switch (s.tipo) {
    case "partito": return o.partito === s.id;
    case "dicastero": return o.dicastero === s.id;
    case "regione": return o.regione === s.id;
    case "disposizione": return o.disposizioni.includes(s.id);
    case "bilancio": return o.misura !== null && String(o.misura) === s.id && o.disposizioni.length === 0;
  }
};

const SOGLIA = { tutte: 0, notevoli: 1, critiche: 2 } as const;
const RANGO = { ordinaria: 0, notevole: 1, critica: 2 } as const;

/** La voce passa il filtro; il soggetto vale anche se sta solo in un dettaglio della voce fusa. */
export const passa = (v: VoceEsito, f: Filtro): boolean =>
  RANGO[v.criticita] >= SOGLIA[f.criticita] &&
  (f.sezione === null || SEZIONE_DI[v.tipo] === f.sezione) &&
  (f.soggetto === null || tocca(v, f.soggetto) || v.dettagli.some((d) => tocca(d, f.soggetto!)));

const soggettiDi = (o: Campi): Soggetto[] => [
  ...(o.partito ? [{ tipo: "partito", id: o.partito } as const] : []),
  ...(o.dicastero ? [{ tipo: "dicastero", id: o.dicastero } as const] : []),
  ...(o.regione ? [{ tipo: "regione", id: o.regione } as const] : []),
  ...o.disposizioni.map((id) => ({ tipo: "disposizione", id }) as const),
  ...(o.misura !== null && o.disposizioni.length === 0 ? [{ tipo: "bilancio", id: String(o.misura) } as const] : []),
];

const ORDINE = ["partito", "dicastero", "regione", "disposizione", "bilancio"] as const;

/** Il trimestre del turno: la Vista sa il suo, e ogni turno è un trimestre. */
export function periodoDel(turno: number, vista: Vista): { anno: number; trimestre: 1 | 2 | 3 | 4 } {
  const i = vista.data.anno * 4 + vista.data.trimestre - 1 - (vista.turno - turno);
  return { anno: Math.floor(i / 4), trimestre: ((i % 4) + 1) as 1 | 2 | 3 | 4 };
}
const annoDel = (turno: number, vista: Vista): number => periodoDel(turno, vista).anno;

/** I soggetti che i turni nominano, coi nomi della Vista; un id che la Vista non ha resta col suo id. */
export function soggetti(turni: Turno[], vista: Vista): { soggetto: Soggetto; nome: string }[] {
  const trovati = new Map<string, { soggetto: Soggetto; nome: string }>();
  const nomi = {
    partito: (id: string) => vista.partiti.find((p) => p.id === id)?.nome,
    dicastero: (id: string) => vista.dicasteri.find((d) => d.id === id)?.nome,
    regione: (id: string) => REGIONI.find((r) => r.id === id)?.nome,
    disposizione: (id: string) => vista.disposizioni.find((d) => d.id === id)?.nome,
  };
  for (const t of turni) {
    for (const v of t.voci) {
      for (const o of [v, ...v.dettagli]) {
        for (const s of soggettiDi(o)) {
          const nome = s.tipo === "bilancio" ? `Legge di bilancio · ${annoDel(t.turno, vista)}` : (nomi[s.tipo](s.id) ?? s.id);
          if (!trovati.has(`${s.tipo}:${s.id}`)) trovati.set(`${s.tipo}:${s.id}`, { soggetto: s, nome });
        }
      }
    }
  }
  return [...trovati.values()].sort((a, b) => ORDINE.indexOf(a.soggetto.tipo) - ORDINE.indexOf(b.soggetto.tipo) || a.nome.localeCompare(b.nome, "it"));
}

/** Le voci degli altri turni con lo stesso soggetto: nei campi della voce (`stesse`) o solo in una riga di una fusa (`nelleRighe`). */
export function collegate(v: VoceEsito, turno: number, turni: Turno[]): { stesse: [number, VoceEsito][]; nelleRighe: [number, VoceEsito][] } {
  const chiavi = [v, ...v.dettagli].flatMap(soggettiDi);
  const delle = (o: Campi) => chiavi.some((s) => tocca(o, s));
  const altre = turni.filter((t) => t.turno !== turno).flatMap((t) => t.voci.map((y) => [t.turno, y] as [number, VoceEsito]));
  const stesse = altre.filter(([, y]) => delle(y));
  return { stesse, nelleRighe: altre.filter((x) => !stesse.includes(x) && x[1].dettagli.some(delle)) };
}

/** La prima stima del turno prima, che una `revisione` corregge (storia 119). */
export const corregge = (v: VoceEsito, turno: number, turni: Turno[]): VoceEsito | null =>
  v.tipo === "revisione" ? (turni.find((t) => t.turno === turno - 1)?.voci.find((y) => y.tipo === "prima_stima") ?? null) : null;

/** La revisione del turno dopo, che corregge una `prima_stima`. */
export const riveduta = (v: VoceEsito, turno: number, turni: Turno[]): VoceEsito | null =>
  v.tipo === "prima_stima" ? (turni.find((x) => x.turno === turno + 1)?.voci.find((y) => y.tipo === "revisione") ?? null) : null;

/** Ciò che il riavvolgimento al turno `n` cancella, fino all'ultima chiusura: per il testo della conferma. */
export function cancella(n: number, ultimo: number, turni: Turno[]): { chiusure: number; voti: number[]; governi: number[] } {
  const R = turni.filter((t) => t.turno >= n && t.turno <= ultimo);
  return {
    chiusure: ultimo - n + 1,
    voti: R.filter((t) => t.voto).map((t) => t.turno),
    governi: R.filter((t) => t.voci.some((v) => v.tipo === "governo_formato")).map((t) => t.turno),
  };
}

const ordinale = (n: number) => (n <= 5 ? t(`registro.ord_${n}` as Chiave) : `${n}°`);
// «all’8», «all’11», «all’80»…: davanti a un numero che si legge con la vocale
const al = (n: number) => (/^(8\d*|11)$/.test(String(n)) ? "all’" : "al ");

/**
 * La conferma del riavvolgimento al turno `n` (lettura 11): le chiusure che si cancellano, i voti e i governi nati nel mezzo
 * e il numero del riavvolgimento. `riavvolgimenti` null = l'indice non si è letto: la frase sul numero cade.
 */
export function testoConferma(n: number, vista: Vista, turni: (Turno | Guasto)[], riavvolgimenti: number | null): string {
  const ultimo = vista.turno - 1;
  const letti = turni.filter((x): x is Turno => "voci" in x);
  const { chiusure, voti, governi } = cancella(n, ultimo, letti);
  const nel = (d: { anno: number; trimestre: 1 | 2 | 3 | 4 }) => periodo(d);
  const mezzo = [
    ...voti.map((x) => t("registro.conferma_voto", { periodo: nel(periodoDel(x, vista)) })),
    ...governi.map((x) => t("registro.conferma_governo", { periodo: nel(periodoDel(x, vista)) })),
  ];
  const nonLetti = chiusure - letti.filter((x) => x.turno >= n && x.turno <= ultimo).length;
  return [
    t("registro.conferma_apertura", { n, periodo: nel(periodoDel(n, vista)) }),
    t(chiusure === 1 ? "registro.conferma_una" : "registro.conferma_piu", { n, k: chiusure, al: al(ultimo), ultimo, fine: nel(periodoDel(ultimo, vista)) }),
    vista.finita ? t("registro.conferma_eredita") : t("registro.conferma_in_corso", { n: vista.turno }),
    mezzo.length ? t("registro.conferma_in_mezzo", { n: mezzo.length, elenco: elenco(mezzo, ", ") }) : t("registro.conferma_niente_in_mezzo"),
    ...(nonLetti > 0 ? [t("registro.conferma_non_letti", { n: nonLetti })] : []),
    riavvolgimenti === null ? t("registro.conferma_si_conta") : t("registro.conferma_si_conta_n", { n: ordinale(riavvolgimenti + 1) }),
    t("registro.conferma_stesso_esito"),
  ].join(" ");
}
