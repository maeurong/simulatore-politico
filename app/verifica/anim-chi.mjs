// Quale animazione parte col movimento spento o ridotto? Serve quando la batteria segna «animazioni max» sopra zero.
// Uso: node verifica/anim-chi.mjs <percorso> <spento|sistema>
//   es. node verifica/anim-chi.mjs /scenario spento
import { BASE, attendi, avviaChrome, controlla } from "./comune.mjs";

const [percorso, modo] = process.argv.slice(2);
if (!percorso?.startsWith("/") || !["spento", "sistema"].includes(modo)) { console.error("Uso: node verifica/anim-chi.mjs <percorso> <spento|sistema>"); process.exit(2); }
await controlla([percorso]);
const { cmd, val, chiudi } = await avviaChrome();
await cmd("Page.enable"); await cmd("Runtime.enable");
await cmd("Emulation.setDeviceMetricsOverride", { width: 1440, height: 900, deviceScaleFactor: 1, mobile: false });
if (modo === "sistema") await cmd("Emulation.setEmulatedMedia", { features: [{ name: "prefers-reduced-motion", value: "reduce" }] });
await cmd("Page.addScriptToEvaluateOnNewDocument", { source: `localStorage.setItem("impostazioni", ${JSON.stringify(JSON.stringify({ versione: 1, scala: 1, velocita: "1", movimento: modo }))})` });
await cmd("Page.navigate", { url: BASE + percorso }); await attendi(2500);
const viste = new Set();
const guarda = async (dove) => { const a = JSON.parse(await val(`JSON.stringify(document.getAnimations().map(x => ({ p: x.transitionProperty || x.animationName || 'waapi', t: (x.effect?.target?.tagName || '?') + '.' + (x.effect?.target?.className || '') + ' «' + (x.effect?.target?.textContent || '').trim().slice(0, 30) + '»', pe: x.effect?.pseudoElement || '', d: x.effect?.getTiming?.().duration })))`)); a.forEach((x) => viste.add(dove + " → " + JSON.stringify(x))); };
await guarda("al carico");
const quanti = Number(await val(`document.querySelectorAll('button, a[href], [role="radio"], input, select').length`));
for (let i = 0; i < Math.min(quanti, 40); i++) {
  const chi = await val(`(() => { const e = document.querySelectorAll('button, a[href], [role="radio"], input, select')[${i}]; if (!e || e.getAttribute('aria-disabled') === 'true' || e.closest('[data-tagli-ignora]')) return null; const d = e.tagName + ' «' + (e.textContent || e.name || e.type || '').trim().slice(0, 25) + '»'; if (e.tagName === 'A') { e.focus(); return d } e.click(); return d })()`);
  for (let k = 0; k < 3; k++) { await attendi(15); await guarda("dopo " + chi); }
}
console.log("data-movimento:", await val("document.documentElement.dataset.movimento ?? '-'"));
console.log([...viste].join("\n") || "nessuna animazione");
chiudi();
