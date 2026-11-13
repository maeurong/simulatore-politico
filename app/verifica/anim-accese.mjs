// Controllo al contrario del movimento: col movimento acceso un'animazione parte davvero?
// Legge lo stesso registro della batteria (comune.mjs). Se sì, lo zero che la batteria conta col movimento spento non è a vuoto.
// Uso: node verifica/anim-accese.mjs <percorso> <testo del pulsante> [<testo> ...]
//   es. node verifica/anim-accese.mjs /scenario Impostazioni 'Prova ›'
// Più testi si cliccano in fila, per arrivare a un pulsante che sta dietro un altro: si misura dopo l'ultimo.
// Esce con 1 se col movimento acceso non parte nessuna animazione, con 2 se un pulsante non c'è o il registro è muto.
import { BASE, REGISTRA_ANIMAZIONI, attendi, avviaChrome, controlla, leggiRegistro } from "./comune.mjs";

const [percorso, ...pulsanti] = process.argv.slice(2);
if (!percorso?.startsWith("/") || !pulsanti.length) { console.error("Uso: node verifica/anim-accese.mjs <percorso> <testo del pulsante> [<testo> ...]"); process.exit(2); }
await controlla([percorso]);
const { cmd, val, chiudi } = await avviaChrome();
await cmd("Page.enable"); await cmd("Runtime.enable");
await cmd("Page.addScriptToEvaluateOnNewDocument", { source: REGISTRA_ANIMAZIONI });
await cmd("Emulation.setDeviceMetricsOverride", { width: 1440, height: 900, deviceScaleFactor: 1, mobile: false });
for (const movimento of ["acceso", "spento"]) {
  const id = (await cmd("Page.addScriptToEvaluateOnNewDocument", { source: `localStorage.setItem("impostazioni", ${JSON.stringify(JSON.stringify({ versione: 1, scala: 1, velocita: "1", movimento }))})` })).result.identifier;
  await cmd("Page.navigate", { url: BASE + percorso });
  await attendi(3200);
  for (const pulsante of pulsanti) {
    const cliccato = await val(`(() => { const b = [...document.querySelectorAll('button')].find((b) => b.textContent.includes(${JSON.stringify(pulsante)})); b?.click(); return !!b })()`);
    if (!cliccato) { console.error(`Su ${percorso} non c'è un pulsante con «${pulsante}».`); process.exit(2); }
    if (pulsante !== pulsanti.at(-1)) await attendi(300);
  }
  const partite = await leggiRegistro(val);
  if (!partite) { console.error("Il registro delle animazioni è muto: lo script iniettato non ha girato."); process.exit(2); }
  const tipi = new Set(partite.flat());
  console.log(`${movimento}: animazioni=${partite.length} tipi=[${[...tipi].join(",")}] data-movimento=${await val("document.documentElement.dataset.movimento ?? '-'")} --k=${(await val("getComputedStyle(document.documentElement).getPropertyValue('--k')")).trim()} dopo il clic: ${await val("location.pathname")}`);
  if (movimento === "acceso" && partite.length === 0) { console.log("← NESSUNA ANIMAZIONE col movimento acceso: qui lo zero della batteria non prova niente"); process.exitCode = 1; }
  await cmd("Page.removeScriptToEvaluateOnNewDocument", { identifier: id });
}
chiudi();
