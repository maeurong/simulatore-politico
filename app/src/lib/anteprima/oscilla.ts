// Un foglio appeso a uno spillo: oscilla a un gesto, con una molla smorzata (docs/ricerca/estetica/grafica-italiana-giocosa.md:211:
// circa 1,5 Hz, ferma in circa 1,2 s), e poi sta fermo. Mai in loop.
const DURATA = 1200, FREQUENZA = 1.5, SMORZAMENTO = 2.9, AMPIEZZA = 4.5, PASSI = 24;
// il fondo del foglio, che ruota attorno allo spillo, non si sposta di più di questo: la colonna lo taglierebbe (`.lato` ha overflow-x nascosto)
const SPOSTAMENTO_MAX = 9;

/** Un'oscillazione smorzata attorno allo spillo; `k` è la durata relativa (`fattori`, 0 = movimento spento); null nel tono grave o con k = 0. */
export function oscilla(el: HTMLElement, k: number): Animation | null {
  if (k <= 0 || (typeof document !== "undefined" && document.documentElement.classList.contains("grave"))) return null;
  const ampiezza = el.offsetHeight > 0 ? Math.min(AMPIEZZA, (Math.asin(Math.min(1, SPOSTAMENTO_MAX / el.offsetHeight)) * 180) / Math.PI) : AMPIEZZA;
  const s = (i: number) => (i / PASSI) * (DURATA / 1000);
  const passi = Array.from({ length: PASSI + 1 }, (_, i) => ({
    transform: `rotate(${i === PASSI ? 0 : +(ampiezza * Math.exp(-SMORZAMENTO * s(i)) * Math.sin(2 * Math.PI * FREQUENZA * s(i))).toFixed(2)}deg)`,
  }));
  return el.animate(passi, { duration: DURATA * k, easing: "linear" });
}
