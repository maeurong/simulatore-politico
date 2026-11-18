// Il kit delle caricature: le parti del profilo «Linea rifinita» (prototipo 32, variante 5), una per leva.
// File generato da uno script di sviluppo che converte il pennino a punta piatta del prototipo (centinaia di quadrilateri
// per faccia) in un solo poligono per parte; lo script non sta nel repo. `pen` è in decimi di unità della tela 64 × 64
// (si disegna sotto scale(.1)); `d` è il tracciato di centro, in unità.
// Lista nera (spec 11, caricature): nessuna parte è capelli, occhiali, barbe, baffi, collane o fiocchi.
import type { Leve } from "./caricatura";

// dalla radice del naso (42,26) alla base (43,34); `tip` è la punta, per l'ombra sotto il naso
const NASI = [
  { d: "L61 34", tip: [61,34] as [number, number], pen: "M412 365l37-27 5-1 86-5-43-19-56-39-26-11 10-6 26 10 84 20 92 40-38 26-151 9-23 16z" },
  { d: "Q56 20 60 30Q61 38 53 36.5", tip: [60,32] as [number, number], pen: "M411 363l38-26 117 17 13 0 2-8 1-14 0-14-2-7-2-6-7-12-8-12-10-10-11-8-12-6-14-4-18-4-17-1-18 1-16 3-32 11 10-6 12-6 37-13 32-9 24-3 17 0 7 1 20 7 17 11 14 16 13 24 2 14 0 14-3 13-7 13-10 9-38 26-6 3-14 3-21 0-91-14-15 11z" },
  { d: "L46 25.5C59 21 63 40 50 38.5", tip: [59,33] as [number, number], pen: "M411 363l38-26 69 34 6 1 14 1 15-3 3-6 4-13 0-21-5-20-11-18-15-15-12-7-18-8-11-3-18-2-55 6 10-6 36-6 32-9 16-3 16-2 17 2 13 5 17 11 10 11 10 17 6 21 1 13-3 14-5 13-4 6-10 9-37 27-12 7-20 5-14 1-14-1-56-28-9 7z" },
  { d: "Q50 35 61 23L54 35.5", tip: [61,23] as [number, number], pen: "M411 364l38-26 6-1 82 4 45-82-56 39-19 8-17 3-12-1-11-3-11-6-11-8-30-28 10-6 20 19 21 9 15 3 16 2 23-3 16-4 26-11 28-27 37-26-68 123-38 26-87-4-19 14z" },
  { d: "Q55 23 62 41L50 37.5", tip: [62,41] as [number, number], pen: "M412 363l38-26 72 26 60 17-17-29-20-28-15-16-35-30-17-9-23-8-12-1-28 4 10-6 13-3 38-3 24 1 23 5 22 7 13 8 15 14 18 21 16 23 16 31 14 32-1 3-38 26-114-33-59-21-11 8z" },
  { d: "L52 22.5L61 33L49 36.5", tip: [61,33] as [number, number], pen: "M411 364l38-27 60 15 72-21-80-93-59 16-27 9 10-6 26-9 85-35 6 3 86 100-37 27-6 2-113 33-44-11-13 10z" },
];
// nessun mento sporgente: la testa calva con la mascella in fuori è la caricatura di un dittatore
const MENTI = [
  { d: "Q43 42 40 45", pen: "M334 494l2-4 35-25 18-11 19-24 13-25-1-7-8-33 37-27 5-1 7-1-10 8 7 28 0 7-5 12-11 18-22 27-53 36 4 29-1 38 5 28-10 6-5-27-12-30z" },
  { d: "L45 41L44 46", pen: "M335 492l37-25 53-13 5-26 0-7-9-19-9-34 1-5 37-26 14-1-13 9 8 31 9 19 0 6-8 42-4 4-38 27-50 11 2 15 0 39 5 28-10 6-4-27-12-30z" },
  { d: "L44.6 41Q48.5 42.6 45 45.2Q48 47.4 43.6 49", pen: "M336 492l29-20 5-2 24 0 47 5-10-10 11-9 4-6 2-6-2-7-4-5-12-8-4-5-14-53 41-29 7-1-10 7 14 50 4 5 11 7 5 6 1 6-2 7-4 5-10 10 10 9 2 7-2 6-4 6-38 26-18 10-50-5 1 41 5 28-10 6-6-33-9-25z" },
];
const FRONTI = [
  { d: "Q38 6 41 11L43 23", pen: "M266 86l37-27 21-3 21-1 21 2 20 5 19 9 15 14 9 11 2 14 3 113 0 7-9 27-10 6 9-26 1-7-30-87-4-20-8-11-10-10-12-8-13-6-13-4-14-2-21 0z" },
  { d: "Q35 5 38 9L44 23", pen: "M266 86l38-27 40-9 14 1 14 3 12 7 10 10 5 5 19 45 25 100 1 7-19 29-10 6 20-29-51-80-23-51-5-6-16-13-12-5-7-2-21 1-20 5z" },
];
// la nuca: `b` è l'ascissa della curva (13 corta, 18 lunga)
const NUCHE = [
  { b: 13, pen: "M137 295l1-21 3-27 9-42 7-19 9-19 10-18 9-12 14-15 10-9 55-39 13-6 20-7 13-3 21-2-38 26-27 4-14 3-16 7-18 21-18 30-10 25-9 35-4 27-2 28 0 28 4 34 7 35 9 27 9 18 14 24 6 37 4 67 7 35-10 6-7-34-21-56-15-30-9-10-7-12-12-26-8-27-5-27-3-28z" },
  { b: 18, pen: "M171 350l3-63 7-55 6-28 8-26 7-20 10-19 11-17 14-16 48-35 19-10 13-4 14-1-38 26-14 1-12 4-7 8-14 24-11 25-9 27-7 34-5 35-4 34-1 36 1 41 4 49 10 57 4 75 7 35-10 6-7-34-21-55-16-30-4-21-3-27z" },
];
const SPALLE = { d: "M2.4 64C3.4 56.4 10 53.4 21 53.2M38 53.2C50 53.2 57.6 57 60.6 64", pen: "M18 617l1-5 8-11 14-16 45-32 18-11 33-11 42-8 12 0 11 5 12 1-8 6-16 2-30 8-50 8-20 6-20 7-9 11-23 36-9 17-1 7-8 6 1-7zM376 535l8-6 16-1 39-6 21 2 27 5 28 7 19 7 19 10 18 11 16 13 10 10 4 5 5 31 4 14-8 6-3-6-5-6-27-15-14-15-16-13-18-11-19-10-20-7-27-7-48-7-17-10z" };
const ORECCHIO = { d: "M28.6 27.2C23.2 26.6 22.4 36.2 28.2 36", pen: "M231 329l1-14 2-6 6-13 4-5 32-23 13 2-6 4-7 0-13 5-8 15-2 13 1 14 4 13 4 6 10 9 6 8 7 1-6 4-17 1-10 0-7-4-8-10-3-6z" };

/** Le sei fantasie del colletto, in inchiostro su carta (`Fantasie.svelte`): l'indice è la leva colletto. Il tecnico lo ha sempre grigio. */
export const FANTASIE = ["pieno", "vuoto", "righe", "pois", "quadretti", "tratteggio"] as const;

// naso all'85% verso la radice (42,26) nelle donne (prototipo 40, scheda 1): le coordinate si scalano a mano, il poligono del naso si disegna a parte con `NASO_T`
const K = 0.85;
const scala = (s: string) => s.replace(/(-?\d+(?:\.\d+)?) (-?\d+(?:\.\d+)?)/g, (_, x, y) => `${+(42 + (x - 42) * K).toFixed(2)} ${+(26 + (y - 26) * K).toFixed(2)}`);

/** I tratti delle donne (prototipo 40, tratti A, cappello «i»): ciglia (una per forma d'occhio), labbra, orecchino, cappello e nastro. */
export const TRATTI = {
  ciglia: ["M36.2 21.4L34.4 18.9M38 20.4L37 17.6M39.8 20.6L39.6 17.8", "M36.3 20.6L34.6 18.2M38 19.9L37.2 17.1M39.7 20.2L39.6 17.4"],
  labbra: "M41.8 39Q45.4 37.8 46.9 39.3Q45.5 40.5 46.7 41.5Q45.6 43.3 42 42.7Z",
  bocca: "M46 40.5L41.2 40.8",
  orecchino: { filo: "M27.4 35.8L27.4 37.6", gemma: "M27.4 37.2L31.2 42.2L27.4 47.2L23.6 42.2Z" },
  cappello: ["M3.6 19.6Q2.6 16.6 8 14.6Q26 8.6 43.4 5.4Q46.4 5.2 45 7.4Q28 13.2 7.4 20.8Q4.6 21.6 3.6 19.6Z", "M16.6 13.2Q15.6 3 27.4 1.4Q38.6 0 40.2 7.8Q28 9.6 16.6 13.2Z"],
  nastro: "M17.4 11.2Q28 7.8 39.6 6.2",
  naso: "translate(6.3 3.9) scale(.085)",
};

/** Le parti disegnate dalle leve: tracciati e poligoni da stendere nella tela 64 × 64. Con `donna` il naso è all'85% e il suo poligono sta a parte in `naso`. */
export function profilo(l: Leve, donna = false) {
  const n = NASI[l.naso], m = MENTI[l.mento], f = FRONTI[l.fronte], u = NUCHE[l.nuca];
  const nd = donna ? scala(n.d) : n.d;
  const tip = donna ? scala(`${n.tip[0]} ${n.tip[1]}`) : `${n.tip[0]} ${n.tip[1]}`;
  const dietro = `M23 57L21 47L20 44C${u.b} 36 ${u.b} 9 30 7${f.d}L42 26`;
  const davanti = `M42 26${nd}L43 35L44 39${m.d}L35 48L36 51L37 57`;
  return {
    /** la figura, da campire di carta e da usare come ritaglio per le ombre */
    sagoma: `M20 44C${u.b} 36 ${u.b} 9 30 7${f.d}L42 26${davanti.slice(6).replace(/L37 57$/, "")}L37 61L23 61L21 47Z`,
    /** il filo di centro: tiene vive le code assottigliate del pennino */
    filo: dietro + davanti + SPALLE.d + ORECCHIO.d,
    /** il pennino: un solo poligono per tutte le pennellate, in decimi */
    pennino: u.pen + f.pen + (donna ? "" : n.pen) + m.pen + SPALLE.pen + ORECCHIO.pen,
    /** il poligono del naso, solo nelle donne: stesso inchiostro, con la trasformazione `TRATTI.naso` */
    naso: donna ? n.pen : "",
    /** le ombre in terra chiara, da ritagliare sulla sagoma: sotto la mascella, sotto il naso, l'orecchio */
    ombre: `M28 37.5L39 45L38 58L21 58ZM45 31L${tip}L44 35.4Z${ORECCHIO.d}Z`,
  };
}

/**
 * L'oggetto del dicastero, in alto a destra della faccia (prototipo 18-governo, riquadro 16 × 16, solo la stampa piena): senza classe riempie con l'inchiostro, `od` col fondo; `sf`/`sd` tracciano senza riempire; lo spessore è quello del prototipo, 1,5 (ereditato dal `<g>`) dove non lo dicono. Chiavi: id di `Vista.dicasteri`.
 */
export const OGGETTI: Record<string, string> = {
  economia_finanze: '<circle cx="8" cy="8" r="7"/><path class="sd" d="M11.2 5A4 4 0 1 0 11.2 11M3.4 7h5.8M3.4 9.4h5.8" stroke-width="1.7"/>',
  sviluppo_economico: '<path class="sf" d="M5 6V2.6h6V6" stroke-width="2"/><path d="M1 5.5h14v9.5H1z"/><path class="sd" d="M1.6 9.8h12.8"/>',
  lavoro_welfare: '<path class="sf" d="M2.6 13.4L9.6 6.4" stroke-width="3.6"/><circle class="sf" cx="11.4" cy="4.6" r="3.3" stroke-width="2.6"/>',
  sanita: '<path d="M1 1h14v14H1z"/><path class="sd" d="M8 3.4v9.2M3.4 8h9.2" stroke-width="3.2"/>',
  istruzione_ricerca: '<path d="M1.5 14.5L3 9.4L11.6 .8L15.2 4.4L6.6 13Z"/><path class="od" d="M1.5 14.5L3 9.4L6.6 13Z"/>',
  infrastrutture_trasporti: '<path d="M4.4 13L7 1.4h2L11.6 13Z"/><path class="od" d="M5.7 7.4h4.6l.6 2.8H5.1Z"/><path d="M1.2 12.6h13.6v2.8H1.2z"/>',
  ambiente_energia: '<path d="M1.5 14.5C1.5 6 7 1.5 14.5 1.5C14.5 10 9 14.5 1.5 14.5Z"/><path class="sd" d="M3.2 12.8L10.8 5.2" stroke-width="1.6"/>',
  interno: '<path d="M8 .6l2.2 4.7 5.1.5-3.9 3.5 1.2 5L8 11.8l-4.6 2.5 1.2-5L.7 5.8l5.1-.5Z"/>',
  giustizia: '<path d="M1.4 5.6L6 1l5 5-4.6 4.6Z"/><path class="sf" d="M8.4 8.4l5.4 5.4" stroke-width="2.8"/><path d="M.6 13.2h8.4v2.4H.6z"/>',
  esteri_difesa: '<circle cx="8" cy="8" r="7"/><ellipse class="sd s14" cx="8" cy="8" rx="3" ry="6.4"/><path class="sd s14" d="M1.6 8h12.8"/>',
};
