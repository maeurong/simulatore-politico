// Un foglio appeso a uno spillo: oscilla a un gesto, con una molla smorzata (docs/ricerca/estetica/grafica-italiana-giocosa.md:211:
// circa 1,5 Hz, ferma in circa 1,2 s), e poi sta fermo. Mai in loop.
const DURATA = 1200, FREQUENZA = 1.5, SMORZAMENTO = 2.9, AMPIEZZA = 4.5, PASSI = 24;
// il fondo del foglio, che ruota attorno allo spillo (20 px sopra il bordo), non si sposta di più di questo, e la colonna (`.lato` ha overflow-x
// nascosto) taglierebbe oltre. La prima mezza oscillazione (rotate positivo) porta il fondo a sinistra, dove ci sono il margine di `.cart`
// (16 px, Cartellino.svelte) e il padding di `.lato` (--canale); la seconda, a destra, è il 38 % della prima e sta nel margine.
const SPOSTAMENTO_MAX = 32, SPILLO = 20;
// la molla smorzata tocca il massimo a tan(ωs) = ω/k, a una frazione dell'ampiezza: è il picco che si vede, e quello che il margine deve contenere
const OMEGA = 2 * Math.PI * FREQUENZA, S_PICCO = Math.atan2(OMEGA, SMORZAMENTO) / OMEGA;
const PICCO = Math.exp(-SMORZAMENTO * S_PICCO) * Math.sin(OMEGA * S_PICCO);

/** L'ampiezza in gradi per un foglio alto `h` px: 4,5° del prototipo, ridotta solo se il picco sposterebbe il fondo oltre il margine. */
export const ampiezzaPer = (h: number): number =>
  h > 0 ? Math.min(AMPIEZZA, (Math.asin(Math.min(1, SPOSTAMENTO_MAX / (h + SPILLO))) * 180) / Math.PI / PICCO) : AMPIEZZA;

/** Un'oscillazione smorzata attorno allo spillo; `k` è la durata relativa (`fattori`, 0 = movimento spento); null nel tono grave o con k = 0. */
export function oscilla(el: HTMLElement, k: number): Animation | null {
  if (k <= 0 || (typeof document !== "undefined" && document.documentElement.classList.contains("grave"))) return null;
  const ampiezza = ampiezzaPer(el.offsetHeight);
  const s = (i: number) => (i / PASSI) * (DURATA / 1000);
  const passi = Array.from({ length: PASSI + 1 }, (_, i) => ({
    transform: `rotate(${i === PASSI ? 0 : +(ampiezza * Math.exp(-SMORZAMENTO * s(i)) * Math.sin(2 * Math.PI * FREQUENZA * s(i))).toFixed(2)}deg)`,
  }));
  return el.animate(passi, { duration: DURATA * k, easing: "linear" });
}
