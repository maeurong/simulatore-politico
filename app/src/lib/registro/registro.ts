// Il registro letto dalle chiusure: la UI filtra, collega e nomina, non decide (spec:410, piano 11f letture 5 e 10).
import { REGIONI } from "$lib/mappa/regioni";
import { SEZIONE_DI } from "$lib/esito/esito";
import type { Sezione } from "$lib/ossatura";
import type { Vista } from "$lib/tipi/Vista";
import type { VoceEsito } from "$lib/tipi/VoceEsito";

export type Turno = { turno: number; voci: VoceEsito[]; critiche: number; grave: boolean; voto: boolean };
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
const tocca = (o: Campi, s: Soggetto): boolean => {
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

/** L'anno del trimestre chiuso dal turno: la Vista sa il suo, e ogni turno è un trimestre. */
const annoDel = (turno: number, vista: Vista): number =>
  Math.floor((vista.data.anno * 4 + vista.data.trimestre - 1 - (vista.turno - turno)) / 4);

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

/** Ciò che il riavvolgimento al turno `n` cancella, fino all'ultima chiusura: per il testo della conferma. */
export function cancella(n: number, ultimo: number, turni: Turno[]): { chiusure: number; voti: number[]; governi: number[] } {
  const R = turni.filter((t) => t.turno >= n && t.turno <= ultimo);
  return {
    chiusure: ultimo - n + 1,
    voti: R.filter((t) => t.voto).map((t) => t.turno),
    governi: R.filter((t) => t.voci.some((v) => v.tipo === "governo_formato")).map((t) => t.turno),
  };
}
