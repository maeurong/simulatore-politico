// Strumento di sola verifica (DEV, ?tagli=1): conta gli elementi che escono dal loro riquadro
// o il cui contenuto è più largo di loro, li contorna di magenta. Nel rilascio non entra:
// +layout.ts lo carica con import() dentro import.meta.env.DEV.

const IGNORA = "[data-tagli-ignora]";

function scorre(el: Element): boolean {
  for (let a = el.parentElement; a; a = a.parentElement) {
    if (/auto|scroll/.test(getComputedStyle(a).overflowY)) return true;
  }
  return false;
}

export function tagli(radice: HTMLElement): number {
  // ponytail: finestra visibile come riquadro; la scala (zoom sulla radice) è già nelle unità di getBoundingClientRect, innerWidth/Height le condividono
  const L = window.innerWidth;
  const A = window.innerHeight;
  let n = 0;
  radice.querySelectorAll(`.taglio`).forEach((e) => e.classList.remove("taglio"));
  for (const el of radice.querySelectorAll("*")) {
    if (el.closest("svg") || el.closest(IGNORA)) continue;
    const b = el.getBoundingClientRect();
    const cs = getComputedStyle(el);
    // ponytail: .perche del sommario è ritagliato a 1x1 finché non prende il fuoco; chiuso non si conta, aperto sì
    if (cs.clipPath !== "none" && b.width <= 2 && b.height <= 2) continue;
    const fuori = b.width > 0 && (b.right > L + 1 || (b.bottom > A + 1 && !scorre(el)));
    // il <select> nativo di WebKit dichiara sotto zoom un contenuto 2 px più largo della scatola senza tagliare nulla (1440×900, scala 1,4)
    const stretto =
      el.tagName !== "SELECT" && el.clientWidth > 0 && el.scrollWidth > el.clientWidth + 1 && cs.display !== "inline" &&
      !/ellipsis/.test(cs.textOverflow) && (cs as CSSStyleDeclaration & { webkitLineClamp: string }).webkitLineClamp === "none";
    if (fuori || stretto) {
      n++;
      el.classList.add("taglio");
    }
  }
  return n;
}

/** Badge fisso, stile iniettato, rilancio su mutazioni e resize. Il numero sta anche in window.__tagli. */
export function avvia() {
  const st = document.createElement("style");
  st.textContent = `.taglio{outline:2px solid magenta!important;outline-offset:-2px}
#tagli-badge{position:fixed;left:8px;bottom:8px;z-index:99999;font:700 12px monospace;color:#fff;background:magenta;padding:2px 6px;pointer-events:none}`;
  document.head.append(st);
  const badge = document.createElement("div");
  badge.id = "tagli-badge";
  badge.setAttribute("data-tagli-ignora", "");
  document.body.append(badge);
  let t = 0;
  const w = window as unknown as { __tagli?: number };
  let ultimo = -1;
  const lancia = () => {
    const n = tagli(document.body);
    badge.textContent = `tagli: ${n}`;
    w.__tagli = n;
    if (n !== ultimo) console.info(`tagli: ${n}`);
    ultimo = n;
  };
  const rinvia = () => {
    w.__tagli = undefined; // chi misura aspetta un numero: quello vecchio non vale più
    badge.textContent = "tagli: …";
    clearTimeout(t);
    t = window.setTimeout(lancia, 150);
  };
  addEventListener("resize", rinvia);
  // ponytail: childList/subtree copre navigazione e cambi di stato; le classi .taglio sono attributi e non riattivano il giro
  new MutationObserver((m) => m.some((x) => x.target !== badge && !badge.contains(x.target)) && rinvia()).observe(document.body, { childList: true, subtree: true });
  rinvia();
}
