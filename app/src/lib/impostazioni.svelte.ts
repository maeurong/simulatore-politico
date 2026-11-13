import { t } from "$lib/testi/t";

export const GRADINI = [1, 1.1, 1.25, 1.4, 1.6, 1.8, 2] as const;
const VELOCITA = ["1", "2", "istantaneo"] as const;
const MOVIMENTO = ["sistema", "acceso", "spento"] as const;
export type Impostazioni = {
  scala: (typeof GRADINI)[number];
  velocita: (typeof VELOCITA)[number];
  movimento: (typeof MOVIMENTO)[number];
};
export const PARTENZA: Impostazioni = { scala: 1, velocita: "1", movimento: "sistema" };
/** Valore: JSON { versione: 1, ...Impostazioni }. Sviluppo e rilascio hanno depositi separati da Tauri: niente prefissi. */
export const CHIAVE = "impostazioni";

const tra = <T>(elenco: readonly T[], v: unknown): v is T => elenco.includes(v as T);

/** Assente, illeggibile, versione ≠ 1 o un valore fuori elenco: i valori di partenza. */
export function leggi(testo: string | null): Impostazioni {
  try {
    const o = JSON.parse(testo ?? "");
    if (o?.versione === 1 && tra(GRADINI, o.scala) && tra(VELOCITA, o.velocita) && tra(MOVIMENTO, o.movimento)) {
      return { scala: o.scala, velocita: o.velocita, movimento: o.movimento };
    }
  } catch {
    // illeggibile: vale come assente
  }
  return { ...PARTENZA };
}

export function scrivi(i: Impostazioni): string {
  return JSON.stringify({ versione: 1, scala: i.scala, velocita: i.velocita, movimento: i.movimento });
}

/** Il gradino più alto ≤ min(scelta, larghezza/1024), mai sotto 1. */
export function scalaApplicata(scelta: number, larghezza: number): number {
  const tetto = Math.min(scelta, larghezza / 1024);
  return GRADINI.findLast((g) => g <= tetto + 1e-9) ?? 1;
}

/** Meno di 1150 px utili. */
export function formaStretta(larghezza: number, applicata: number): boolean {
  return larghezza / applicata < 1150;
}

export function fattori(i: Impostazioni, riduciSistema: boolean): { k: 1 | 0.5 | 0; muove: boolean } {
  const k = ({ "1": 1, "2": 0.5, istantaneo: 0 } as const)[i.velocita];
  return { k, muove: i.movimento === "acceso" || (i.movimento === "sistema" && !riduciSistema) };
}

/** Il gradino accanto, fermo agli estremi. */
export function passo(scelta: number, verso: 1 | -1): Impostazioni["scala"] {
  const i = GRADINI.indexOf(scelta as Impostazioni["scala"]);
  return GRADINI[Math.min(GRADINI.length - 1, Math.max(0, i + verso))];
}

/** 1,25× */
export const volte = (x: number) => `${String(x).replace(".", ",")}×`;

// --- stato: nel browser si legge e si applica all'import, prima del primo disegno ---

const browser = typeof window !== "undefined";

function deposito(): string | null {
  if (!browser) return null;
  try {
    return localStorage.getItem(CHIAVE);
  } catch {
    return null; // deposito bloccato: si parte dai valori di partenza
  }
}

export const impostazioni = $state<Impostazioni>(leggi(deposito()));
/** Ciò che l'app non sceglie: la larghezza della finestra e «Riduci movimento» di macOS. */
export const sistema = $state({ larghezza: 1024, riduce: false });
/** Il testo dell'avviso breve delle scorciatoie; vuoto quando non c'è. */
export const avvisoScala = $state({ testo: "" });

export function applica() {
  if (!browser) return;
  const a = scalaApplicata(impostazioni.scala, sistema.larghezza);
  const { k, muove } = fattori(impostazioni, sistema.riduce);
  const radice = document.documentElement;
  radice.style.zoom = String(a);
  radice.style.setProperty("--k", String(k));
  if (muove) delete radice.dataset.movimento;
  else radice.dataset.movimento = "spento";
  radice.classList.toggle("stretta", formaStretta(sistema.larghezza, a));
}

export function scegli(nuove: Partial<Impostazioni>) {
  Object.assign(impostazioni, nuove);
  try {
    localStorage.setItem(CHIAVE, scrivi(impostazioni));
  } catch {
    // deposito bloccato: la scelta vale fino alla chiusura
  }
  applica();
}

let spegni: ReturnType<typeof setTimeout> | undefined;

/** Cmd+ (= o +), Cmd− e Cmd0, anche con Ctrl. Vero se il tasto era suo. */
export function scorciatoia(e: KeyboardEvent): boolean {
  if (!(e.metaKey || e.ctrlKey) || e.altKey || e.repeat) return false;
  const verso = e.key === "+" || e.key === "=" ? 1 : e.key === "-" ? -1 : e.key === "0" ? 0 : null;
  if (verso === null) return false;
  e.preventDefault();
  const scala = verso === 0 ? 1 : passo(impostazioni.scala, verso);
  scegli({ scala });
  const applicata = scalaApplicata(scala, sistema.larghezza);
  avvisoScala.testo =
    applicata < scala
      ? t("impostazioni.avviso_scala_tetto", { scala: volte(scala), applicata: volte(applicata) })
      : t("impostazioni.avviso_scala", { scala: volte(scala) });
  clearTimeout(spegni);
  spegni = setTimeout(() => (avvisoScala.testo = ""), 1500);
  return true;
}

if (browser) {
  const riduci = matchMedia("(prefers-reduced-motion: reduce)");
  sistema.larghezza = innerWidth;
  sistema.riduce = riduci.matches;
  addEventListener("resize", () => {
    sistema.larghezza = innerWidth;
    applica();
  });
  riduci.addEventListener("change", (e) => {
    sistema.riduce = e.matches;
    applica();
  });
  applica();
}
