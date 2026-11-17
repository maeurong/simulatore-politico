// Geometria della tavola per poteri: svgB di 37-scelte-11e-schema.prototype.html con riga · pettine · punta (decisione 1 di Mario, 17/11/2026).
// Quattro colonne fisse; le etichette nella riga di chi muove; i fili verso il Presidente su due sbarre ai lati del governo, una punta per sbarra.
import { t, type Chiave } from "$lib/testi/t";
import type { Filo, Nodo } from "./schema";

export type Riga = {
  nodo: Nodo; nome: string; sotto: string; x: number; y: number; w: number; h: number; scura: boolean;
  etichetta: { testo: string; contrario: boolean } | null;
  /** I tratti dei fili che la riga muove. */
  uscenti: Tratto[];
};
/** `x`, `y`: la metà del tratto, dove sta il tondo. */
export type Tratto = { filo: Filo; d: string; x: number; y: number };
export type Sbarra = { d: string; punta: boolean };
/** Il filo o la riga aperti nel dettaglio a lato. */
export type Scelta = { tipo: "filo" | "riga"; id: string } | null;
export type Tavola = { altezza: number; colonne: number[]; cw: number; righe: Riga[]; tratti: Tratto[]; sbarre: Sbarra[] };

/** Sopra le righe: i titoli delle colonne. */
export const TESTA = 26;
/** Altezza della riga: quella del prototipo (fino a 50), mai sotto 42; sotto, la tavola si allunga e scorre. */
export const RIGA_MIN = 42;
const RIGA_MAX = 50;
const PDC = "pdc";
const SCURI: Nodo["tipo"][] = ["presidente", "camera", "mercati"];

export const nomeNodo = (n: Nodo) => (n.nome.startsWith("schema.") ? t(n.nome as Chiave) : n.nome);

/** Larghezza del testo in px. */
export type Misura = (testo: string, px: number, grassetto: boolean) => number;
/** La stima del prototipo, per i test; la pagina misura col canvas (Tavola.svelte). */
export const stima: Misura = (s, px, grassetto) => s.length * px * (grassetto ? 0.6 : 0.54);
function corto(s: string, px: number, max: number, grassetto: boolean, misura: Misura): string {
  if (misura(s, px, grassetto) <= max) return s;
  let n = s.length - 1;
  while (n > 1 && misura(`${s.slice(0, n).trimEnd()}…`, px, grassetto) > max) n--;
  return `${s.slice(0, n).trimEnd()}…`;
}
const r1 = (x: number) => Math.round(x * 10) / 10;

export function tavola(nodi: Nodo[], fili: Filo[], W: number, H: number, misura: Misura = stima): Tavola {
  const gap = W * 0.055, cw = (W - gap * 3) / 4, X = (c: number) => c * (cw + gap);
  const pos = new Map<string, { x: number; y: number; h: number; c: number }>();
  let altezza = H;
  for (const c of [0, 1, 2, 3]) {
    const L = nodi.filter((n) => n.colonna === c);
    const rh = Math.max(RIGA_MIN, Math.min(RIGA_MAX, (H - TESTA - 4) / Math.max(L.length, 1)));
    L.forEach((n, k) => pos.set(n.id, { x: X(c), y: TESTA + rh * k + rh / 2, h: Math.min(rh - 6, 38), c }));
    altezza = Math.max(altezza, TESTA + rh * L.length + 4);
  }

  const p = pos.get(PDC);
  const sbX = [X(2) - gap * 0.4, X(2) + cw + gap * 0.4];
  const sbarra = [0, 1].map(() => ({ usata: false, punta: false, su: p?.y ?? 0, giu: p?.y ?? 0 }));
  const tratti: Tratto[] = [];
  const uscenti = new Map<string, Tratto[]>();
  for (const f of fili) {
    const A = pos.get(f.da), B = pos.get(f.a);
    if (!A || !B) continue;
    let d: string, mx: number, my: number;
    if (p && (f.a === PDC || (f.da === PDC && B.c === 2))) {
      // pettine: dritto sulla sbarra del suo lato, all'altezza della riga dell'altro capo
      const altro = f.a === PDC ? A : B, lato = altro.c <= 2 ? 0 : 1, sx = sbX[lato], Y = altro.y, s = sbarra[lato];
      s.usata = true; s.punta ||= f.a === PDC; s.su = Math.min(s.su, Y); s.giu = Math.max(s.giu, Y);
      const xo = altro.c === 2 ? X(2) : altro.c < 2 ? X(altro.c) + cw : X(altro.c);
      d = f.a === PDC ? `M${r1(xo)},${r1(Y)} L${r1(sx)},${r1(Y)}` : `M${r1(sx)},${r1(Y)} L${r1(xo)},${r1(Y)}`;
      // dai Partiti il pettine scavalca il Parlamento: il tondo sta nel primo canale, non sopra una riga
      mx = altro.c === 0 ? xo + gap / 2 : (xo + sx) / 2; my = Y;
    } else if (A.c === B.c) {
      // stessa colonna (Camera → Senato): un'ansa nel canale a sinistra
      const x = A.x - 2, ansa = 14 + Math.abs(B.y - A.y) * 0.08;
      d = `M${r1(x)},${r1(A.y)} C${r1(x - ansa)},${r1(A.y)} ${r1(x - ansa)},${r1(B.y)} ${r1(x)},${r1(B.y)}`;
      mx = x - ansa * 0.75; my = (A.y + B.y) / 2;
    } else {
      const xa = A.c < B.c ? A.x + cw : A.x, xb = A.c < B.c ? B.x : B.x + cw, k = (xb - xa) * 0.5;
      d = `M${r1(xa)},${r1(A.y)} C${r1(xa + k)},${r1(A.y)} ${r1(xb - k)},${r1(B.y)} ${r1(xb)},${r1(B.y)}`;
      // scavalcando una colonna, il tondo nel primo canale (lì la curva è ancora all'altezza di chi muove)
      [mx, my] = Math.abs(B.c - A.c) > 1 ? [xa + (A.c < B.c ? gap : -gap) / 2, A.y] : [(xa + xb) / 2, (A.y + B.y) / 2];
    }
    const tr = { filo: f, d, x: r1(mx), y: r1(my) };
    tratti.push(tr);
    uscenti.set(f.da, [...(uscenti.get(f.da) ?? []), tr]);
  }

  const sbarre: Sbarra[] = [];
  sbarra.forEach((s, lato) => {
    if (!s.usata || !p) return;
    const sx = r1(sbX[lato]), entra = r1(lato === 0 ? X(2) : X(2) + cw);
    sbarre.push({ d: `M${sx},${r1(s.su)} L${sx},${r1(s.giu)} M${sx},${r1(p.y)} L${entra},${r1(p.y)}`, punta: s.punta });
  });

  const righe: Riga[] = nodi.map((n) => {
    const { x, y, h } = pos.get(n.id)!;
    const U = uscenti.get(n.id) ?? [];
    const etichetta = U.length ? { testo: corto(U[0].filo.etichetta, 12, cw * 0.55, true, misura) + (U.length > 1 ? ` +${U.length - 1}` : ""), contrario: U[0].filo.contrario } : null;
    const spazio = etichetta ? misura(etichetta.testo, 12, true) + 10 : 0;
    return { nodo: n, nome: corto(nomeNodo(n), 13, cw - 18, true, misura), sotto: corto(n.sotto, 12, cw - 18 - spazio, false, misura), x, y, w: cw, h, scura: SCURI.includes(n.tipo), etichetta, uscenti: U };
  });

  return { altezza, colonne: [0, 1, 2, 3].map(X), cw, righe, tratti, sbarre };
}

/** Un tondo su un filo: il numero, il centro, rosso se il filo è contrario. */
export type Tondo = { n: number; x: number; y: number; m: boolean; filo: string };
/** I tondi dei fili, raggruppati per punto: fili con la stessa geometria (le leggi Camera → Senato) accodano i numeri invece di coprirsi. */
export function tondiSuiFili(tratti: Tratto[], numeri: Map<string, number[]>): Tondo[] {
  const punti = new Map<string, { x: number; y: number; voci: { n: number; m: boolean; filo: string }[] }>();
  for (const { filo: f, x, y } of tratti) {
    for (const n of numeri.get(f.id) ?? []) {
      const k = `${x},${y}`;
      if (!punti.has(k)) punti.set(k, { x, y, voci: [] });
      punti.get(k)!.voci.push({ n, m: f.contrario, filo: f.id });
    }
  }
  return [...punti.values()].flatMap((p) => p.voci.sort((a, b) => a.n - b.n).map((v, i) => ({ ...v, x: p.x - ((p.voci.length - 1) * 19) / 2 + i * 19, y: p.y })));
}

const inPrima = (numeri: number[]) => (numeri.length ? t("schema.prima_aria", { n: numeri.join(", ") }) : "");
/** Il nome accessibile della riga: i fili si contano su tutto il trimestre, anche a filtro acceso. */
export const nomeRiga = (n: Nodo, tutti: Filo[], numeri: number[]) =>
  t("schema.nome_riga", { nome: nomeNodo(n), sotto: n.sotto, fili: t("schema.n_fili", { n: tutti.filter((f) => f.da === n.id || f.a === n.id).length }) }) + inPrima(numeri);
export const nomeFilo = (f: Filo, numeri: number[]) => f.frase + inPrima(numeri);
