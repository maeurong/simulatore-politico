import interfaccia from "./interfaccia.json";

export type Chiave = keyof typeof interfaccia;

const plurali = new Intl.PluralRules("it");

/** {nome} sostituiti; un valore oggetto {one, other} sceglie con Intl.PluralRules("it") su {n}; "many" ricade su "other". */
export function t(chiave: Chiave, valori: Record<string, string | number> = {}): string {
  const v: string | Record<string, string> = interfaccia[chiave];
  const testo = typeof v === "string" ? v : (v[plurali.select(Number(valori.n))] ?? v.other);
  // ponytail: i numeri entrano come String(): un anno non deve diventare «2.027»; chi vuole il punto passa numero()
  return testo.replace(/\{(\w+)\}/g, (segnaposto, nome) => (nome in valori ? String(valori[nome]) : segnaposto));
}

const MENO = "−";
/** Si legge zero: nessuna cifra da 1 a 9. */
const zero = (testo: string) => !/[1-9]/.test(testo);

export function numero(x: number, decimali = 0): string {
  const testo = new Intl.NumberFormat("it-IT", {
    minimumFractionDigits: decimali,
    maximumFractionDigits: decimali,
    useGrouping: "always",
  }).format(x);
  return testo.replace("-", zero(testo) ? "" : MENO);
}

export function percentuale(quota: number, decimali = 1): string {
  return `${numero(quota * 100, decimali)}%`;
}

export function segnato(x: number, decimali = 0): string {
  const assoluto = numero(Math.abs(x), decimali);
  if (zero(assoluto)) return assoluto;
  return (x > 0 ? "+" : MENO) + assoluto;
}

const giorno = (d: Date) => new Date(d.getFullYear(), d.getMonth(), d.getDate());

/** «oggi alle 22:20», «ieri alle …», giorno della settimana fino a sei giorni, poi «il 28 ottobre alle …». */
export function dataRelativa(secondiUnix: number, ora: Date): string {
  const d = new Date(secondiUnix * 1000);
  const giorni = Math.round((giorno(ora).getTime() - giorno(d).getTime()) / 864e5);
  const alle = d.toLocaleTimeString("it-IT", { hour: "2-digit", minute: "2-digit" });
  if (giorni === 0) return `oggi alle ${alle}`;
  if (giorni === 1) return `ieri alle ${alle}`;
  if (giorni < 7) return `${d.toLocaleDateString("it-IT", { weekday: "long" })} alle ${alle}`;
  return `il ${d.toLocaleDateString("it-IT", { day: "numeric", month: "long" })} alle ${alle}`;
}

/** FNV-1a a 32 bit sui byte UTF-8, intero ≥ 0. */
export function hash(testo: string): number {
  let h = 0x811c9dc5;
  for (const byte of new TextEncoder().encode(testo)) h = Math.imul(h ^ byte, 0x01000193) >>> 0;
  return h;
}

export function variante(n: number, ...parti: (string | number)[]): number {
  return hash(parti.join("|")) % n;
}
