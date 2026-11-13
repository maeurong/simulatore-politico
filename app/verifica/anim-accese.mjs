// Controllo al contrario del movimento: col movimento acceso un'animazione parte davvero?
// Se sì, lo zero che la batteria conta col movimento spento non è a vuoto.
// Uso: node verifica/anim-accese.mjs <percorso> <testo del pulsante> [<testo> ...]
//   es. node verifica/anim-accese.mjs /scenario Impostazioni 'Prova ›'
// Più testi si cliccano in fila, per arrivare a un pulsante che sta dietro un altro: si misura dopo l'ultimo.
// Esce con 1 se col movimento acceso non parte nessuna animazione, con 2 se un pulsante non c'è.
import { BASE, attendi, avviaChrome, controlla } from "./comune.mjs";

const [percorso, ...pulsanti] = process.argv.slice(2);
if (!percorso?.startsWith("/") || !pulsanti.length) { console.error("Uso: node verifica/anim-accese.mjs <percorso> <testo del pulsante> [<testo> ...]"); process.exit(2); }
await controlla([percorso]);
const { cmd, val, chiudi } = await avviaChrome();
await cmd("Page.enable"); await cmd("Runtime.enable");
await cmd("Emulation.setDeviceMetricsOverride", { width: 1440, height: 900, deviceScaleFactor: 1, mobile: false });
for (const movimento of ["acceso", "spento"]) {
  const id = (await cmd("Page.addScriptToEvaluateOnNewDocument", { source: `localStorage.setItem("impostazioni", ${JSON.stringify(JSON.stringify({ versione: 1, scala: 1, velocita: "1", movimento }))})` })).result.identifier;
  await cmd("Page.navigate", { url: BASE + percorso });
  let massimo = 0;
  for (let i = 0; i < 40; i++) { await attendi(50); massimo = Math.max(massimo, Number(await val("document.getAnimations().length"))); }
  await attendi(1200);
  for (const pulsante of pulsanti) {
    const cliccato = await val(`(() => { const b = [...document.querySelectorAll('button')].find((b) => b.textContent.includes(${JSON.stringify(pulsante)})); b?.click(); return !!b })()`);
    if (!cliccato) { console.error(`Su ${percorso} non c'è un pulsante con «${pulsante}».`); process.exit(2); }
    if (pulsante !== pulsanti.at(-1)) await attendi(300);
  }
  const tipi = new Set();
  for (let i = 0; i < 40; i++) {
    await attendi(25);
    // di un el.animate() si dicono le proprietà animate: spento, una rotazione diventa una dissolvenza, e il conto resta uguale
    const a = JSON.parse(await val("JSON.stringify(document.getAnimations().map(x => x.transitionProperty || x.animationName || 'waapi:' + [...new Set((x.effect?.getKeyframes?.() ?? []).flatMap(k => Object.keys(k)))].filter(p => !['offset', 'computedOffset', 'easing', 'composite'].includes(p)).join('+')))"));
    massimo = Math.max(massimo, a.length); a.forEach((t) => tipi.add(t));
  }
  console.log(`${movimento}: animazioni max=${massimo} tipi=[${[...tipi].join(",")}] data-movimento=${await val("document.documentElement.dataset.movimento ?? '-'")} --k=${(await val("getComputedStyle(document.documentElement).getPropertyValue('--k')")).trim()} dopo il clic: ${await val("location.pathname")}`);
  if (movimento === "acceso" && massimo === 0) { console.log("← NESSUNA ANIMAZIONE col movimento acceso: qui lo zero della batteria non prova niente"); process.exitCode = 1; }
  await cmd("Page.removeScriptToEvaluateOnNewDocument", { identifier: id });
}
chiudi();
