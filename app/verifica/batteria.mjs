// Batteria in Chrome headless via CDP, senza dipendenze.
// Uso: npm run verifica:chrome -- <nome>=<percorso> [<nome>=<percorso> ...]
//   es. 'cornice-01=/partita/cruscotto?vista=01-turno-zero.json'
// Per ogni pagina: 2 finestre × 2 scale → tagli, forma stretta, errori di console, screenshot;
// poi movimento (spento; sistema + reduce) e giro di Tab a 1440×900.
// Esce con 1 se una misura non vale: tagli diversi da zero, contatore muto, scala non applicata, pagina finita altrove;
// o se il movimento spento (o «Come il sistema» sotto il movimento ridotto) anima altro che opacity, o il Tab non raggiunge un elemento.
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { BASE, FINESTRE, altrove, attendi, avviaChrome, nonDissolvenze, pagine, REGISTRA_ANIMAZIONI, uscita } from "./comune.mjs";

const elenco = await pagine("npm run verifica:chrome --");
const out = uscita("chrome");
mkdirSync(out, { recursive: true });
let errori = [];
const { cmd, val, chiudi } = await avviaChrome((d) => {
  if (d.method === "Runtime.exceptionThrown") errori.push("eccezione: " + (d.params.exceptionDetails.exception?.description ?? d.params.exceptionDetails.text).split("\n")[0]);
  if (d.method === "Runtime.consoleAPICalled" && d.params.type === "error") errori.push("console.error: " + d.params.args.map((a) => a.value ?? a.description).join(" ").slice(0, 200));
  if (d.method === "Log.entryAdded" && d.params.entry.level === "error") errori.push("log: " + d.params.entry.text.slice(0, 200));
});
await cmd("Page.enable"); await cmd("Runtime.enable"); await cmd("Log.enable");
await cmd("Page.addScriptToEvaluateOnNewDocument", { source: REGISTRA_ANIMAZIONI });

let scriptId = null;
async function impostazioni(i) {
  if (scriptId) await cmd("Page.removeScriptToEvaluateOnNewDocument", { identifier: scriptId });
  const testo = JSON.stringify({ versione: 1, velocita: "1", movimento: "sistema", ...i });
  scriptId = (await cmd("Page.addScriptToEvaluateOnNewDocument", { source: `try { localStorage.setItem("impostazioni", ${JSON.stringify(testo)}) } catch {}` })).result.identifier;
}
const finestra = (w, h) => cmd("Emulation.setDeviceMetricsOverride", { width: w, height: h, deviceScaleFactor: 1, mobile: false });
async function apri(percorso) {
  errori = [];
  const u = BASE + percorso + (percorso.includes("?") ? "&" : "?") + "tagli=1";
  await cmd("Page.navigate", { url: u }); await attendi(1800);
  await val("document.fonts.ready.then(() => 1)"); await attendi(400);
  // il contatore ricalcola con un ritardo: aspetta che il numero ci sia
  for (let i = 0; i < 30 && (await val("typeof window.__tagli")) !== "number"; i++) await attendi(100);
  await attendi(250);
}
const foto = async (nome) => writeFileSync(join(out, nome), Buffer.from((await cmd("Page.captureScreenshot", { format: "png" })).result.data, "base64"));
const tasto = async (key, code, kc, mod = 0) => {
  for (const type of ["keyDown", "keyUp"]) await cmd("Input.dispatchKeyEvent", { type, key, code, windowsVirtualKeyCode: kc, modifiers: mod, ...(key === "Enter" && type === "keyDown" ? { text: "\r" } : {}) });
  await attendi(60);
};
const stato = () => val(`JSON.stringify({ tagli: window.__tagli ?? null, stretta: document.documentElement.className, zoom: document.documentElement.style.zoom, scorreDiLato: document.documentElement.scrollWidth > document.documentElement.clientWidth + 1, url: location.pathname + location.search, percorso: location.pathname })`);

const esiti = [];
for (const [nome, percorso] of elenco) {
  // passo 1: finestre e scale
  for (const [w, h, scale] of FINESTRE) {
    for (const scala of scale) {
      await impostazioni({ scala }); await finestra(w, h); await apri(percorso);
      const s = JSON.parse(await stato());
      await foto(`${nome}-${w}x${h}-x${scala}.png`);
      // una misura vale se la pagina è quella chiesta, la scala è applicata e il contatore dà zero
      const guasto = altrove(percorso, s.percorso) ? `ALTROVE: ${s.url}` : Number(s.zoom || 1) !== scala ? "SCALA NON APPLICATA" : s.tagli !== 0 ? "TAGLI" : null;
      esiti.push({ pagina: nome, finestra: `${w}x${h}`, scala, ...s, guasto, errori: [...errori] });
    }
  }
  // passo 2: movimento, spento e poi «Come il sistema» col movimento ridotto. Conta ogni animazione partita dopo il caricamento e ogni clic.
  for (const [etichetta, imp, riduci] of [["spento", { scala: 1, movimento: "spento" }, false], ["sistema+reduce", { scala: 1, movimento: "sistema" }, true]]) {
    await cmd("Emulation.setEmulatedMedia", { features: [{ name: "prefers-reduced-motion", value: riduci ? "reduce" : "no-preference" }] });
    await impostazioni(imp); await finestra(1440, 900); await apri(percorso);
    const quanti = Number(await val(`document.querySelectorAll('button, a[href], [role="radio"], input, select').length`));
    for (let i = 0; i < Math.min(quanti, 40); i++) {
      await val(`(() => { const e = document.querySelectorAll('button, a[href], [role="radio"], input, select')[${i}]; if (!e || e.getAttribute('aria-disabled') === 'true' || e.closest('[data-tagli-ignora]')) return; if (e.tagName === 'A') { e.focus(); return } e.click() })()`);
      await attendi(40);
    }
    const partite = JSON.parse(await val("JSON.stringify(window.__anim ?? [])"));
    const tipi = [...new Set(partite.flat())];
    esiti.push({ pagina: nome, movimento: etichetta, animazioni: partite.length, tipi, guasto: nonDissolvenze(tipi).length ? "MOVIMENTO" : null, errori: [...errori] });
  }
  await cmd("Emulation.setEmulatedMedia", { features: [{ name: "prefers-reduced-motion", value: "no-preference" }] });
  // passo 3: tastiera. Ogni comando visibile si raggiunge col Tab.
  await impostazioni({ scala: 1 }); await finestra(1440, 900); await apri(percorso);
  const attesi = JSON.parse(await val(`JSON.stringify([...document.querySelectorAll('button, a[href], input, select, textarea, [tabindex]')].filter(e => e.tabIndex >= 0 && !e.closest('[data-tagli-ignora]') && e.getClientRects().length && getComputedStyle(e).visibility !== 'hidden' && !(e.type === 'radio' && !e.checked && (document.querySelector('input[type=radio][name="' + e.name + '"]:checked') || document.querySelector('input[type=radio][name="' + e.name + '"]') !== e))).map((e, i) => { e.dataset.bt = i; return (e.getAttribute('aria-label') || e.textContent.trim() || e.name || e.tagName).slice(0, 40) }))`));
  const visti = new Set();
  await val("document.activeElement?.blur()");
  for (let i = 0; i < attesi.length + 15; i++) {
    await tasto("Tab", "Tab", 9);
    const k = await val("document.activeElement?.dataset?.bt ?? null");
    if (k !== null) visti.add(Number(k));
  }
  const mancanti = attesi.map((t, i) => (visti.has(i) ? null : `${i}:${t}`)).filter(Boolean);
  esiti.push({ pagina: nome, tastiera: `${visti.size}/${attesi.length}`, nonRaggiunti: mancanti, guasto: mancanti.length ? "TASTIERA" : null, errori: [...errori] });
}
writeFileSync(join(out, "esiti.json"), JSON.stringify(esiti, null, 1));
const riga = (e) => (e.finestra ? `${e.pagina} ${e.finestra} x${e.scala}: tagli=${e.tagli} classi=[${e.stretta}] zoom=${e.zoom || 1} lato=${e.scorreDiLato} err=${e.errori.length}`
  : e.movimento ? `${e.pagina} movimento ${e.movimento}: animazioni=${e.animazioni} tipi=[${e.tipi}] err=${e.errori.length}`
  : `${e.pagina} tastiera ${e.tastiera}${e.nonRaggiunti.length ? " NON RAGGIUNTI: " + e.nonRaggiunti.join(" | ") : ""} err=${e.errori.length}`) + (e.guasto ? " ← " + e.guasto : "");
console.log(esiti.map(riga).join("\n"));
const tutti = [...new Set(esiti.flatMap((e) => e.errori))]; if (tutti.length) console.log("ERRORI:\n" + tutti.join("\n"));
console.log(`screenshot ed esiti.json in ${out}`);
if (esiti.some((e) => e.guasto)) process.exitCode = 1;
chiudi();
