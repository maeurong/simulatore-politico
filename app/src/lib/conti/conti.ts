import { numero, segnato, t } from "$lib/testi/t";
import type { Vista } from "$lib/tipi/Vista";
import type { RedditoFascia } from "$lib/tipi/RedditoFascia";
import type { VistaRegoleUe } from "$lib/tipi/VistaRegoleUe";
import type { VoceGrandezza } from "$lib/tipi/VoceGrandezza";
import type { VoceSituazione } from "$lib/tipi/VoceSituazione";

const soglia = (x: number) => numero(x, Number.isInteger(x) ? 0 : 1);
export const quanti = ["", "un", "due", "tre", "quattro"];
const ordinali = ["", "primo", "secondo", "terzo", "quarto"];

/** La frase della distanza (ticket 21; decisione 1 per `oltre`). */
export function distanza(s: VoceSituazione): string {
  // accensione sopra lo spegnimento: la situazione si accende salendo
  const sale = s.accensione > s.spegnimento;
  const mancano = (verso: string, limite: number) => ({ verso, soglia: soglia(limite), n: numero(Math.abs(s.valore - limite), 1) });
  const oltreDa = (verbo: string) =>
    t("conti.oltre_da", {
      verbo,
      da: `${quanti[s.oltre] ?? s.oltre} ${s.oltre === 1 ? "trimestre" : "trimestri"}`,
      al: ordinali[s.trimestri] ?? `${s.trimestri}º`,
    });
  if (s.accesa) {
    if (s.oltre > 0) return oltreDa("si spegne");
    if (sale ? s.valore <= s.spegnimento : s.valore >= s.spegnimento) return t("conti.oltre_soglia");
    return t("conti.accesa", mancano(sale ? "sotto" : "sopra", s.spegnimento));
  }
  if (s.oltre > 0) return oltreDa("si accende");
  if (sale ? s.valore >= s.accensione : s.valore <= s.accensione) return t("conti.oltre_soglia");
  return t("conti.spenta", mancano(sale ? "sopra" : "sotto", s.accensione));
}

/** La procedura in parole e se è rossa (lettura 9). */
export function procedura(u: VistaRegoleUe): { testo: string; male: boolean } {
  return {
    testo: t(`conti.procedura_${u.procedura}`),
    male: u.procedura !== "nessuna" && u.procedura !== "sospesa",
  };
}

/** «0,27 punti sotto il percorso», «sul percorso», «0,60 punti sopra il percorso» (lettura 9). */
export function percorso(deviazione: number): string {
  const n = numero(Math.abs(deviazione), 2);
  if (!/[1-9]/.test(n)) return t("conti.sul_percorso");
  return t(deviazione < 0 ? "conti.sotto_percorso" : "conti.sopra_percorso", { n });
}

/** Le grandezze nazionali col valore; le regionali come «da–a» col rimando a Paese (ticket 21). */
export function grandezza(g: VoceGrandezza): { valore: number } | { da: number; a: number } {
  if (g.valori.length === 1) return { valore: g.valori[0] };
  return { da: Math.min(...g.valori), a: Math.max(...g.valori) };
}

/** La perdita reale scritta: «−0,1%» rossa, «+1,0%» verde se negativa (deflazione), «0,0%» sotto 0,05. */
export function reale(p: number): { testo: string; tono: "male" | "bene" | "neutro" } {
  if (Math.abs(p) < 0.05) return { testo: "0,0%", tono: "neutro" };
  return { testo: `${segnato(-p, 1)}%`, tono: p > 0 ? "male" : "bene" };
}

/** Sotto le attese: la crescita del trimestre minore delle attese. */
export const sottoAttese = (r: RedditoFascia): boolean => r.crescita < r.attesa;

/** La riga scelta c'è ancora nella Vista: una Vista nuova (MES finito, situazione sparita) riporta il lato al centro studi. */
export function esiste(v: Vista, k: string): boolean {
  const [tipo, id] = k.split(":");
  switch (tipo) {
    case "dic": return v.bilancio.dicasteri.some((d) => d.id === id);
    case "irpef": return +id < v.bilancio.aliquote.irpef.length;
    case "sit": return +id < v.situazioni.length;
    case "gr": return v.grandezze.some((g) => g.id === id);
    case "stat": return id in v.statistica.prima_stima;
    case "red": return v.redditi.some((r) => r.fascia === id);
    case "mes": return v.debito.mes !== null;
    case "ristr": return v.debito.ristrutturazione !== null;
    default: return ["fondo", "spread", "grandezze", "debito", "ue"].includes(k);
  }
}
