// Il cartellino dell'Anteprima oscilla solo ai tre gesti (apertura, cambio di gradino, acquisto del test) e non a ogni Vista nuova.
// Uso: VERIFICA_BASE=<server> node verifica/cartellino.mjs     Esce con 1 se un conto non torna.
// Conta le animazioni `.cart` partite: apertura 1, test comprato 1, rifiuto del core (comando senza effetto) 0, la scelta resta dopo
// una Vista nuova dello stesso turno; movimento spento, velocità istantanea e tono grave 0; peso della molla: a 2× dura la metà;
// al picco dell'oscillazione il foglio resta dentro la colonna (`.lato` taglia fuori dal padding-box), a quattro finestre/scale e quattro forme.
import { BASE, attendi, avviaChrome } from "./comune.mjs";

const AULA = "/partita/aula?vista=19-aula.json&stato=piena";
const { cmd, val, chiudi } = await avviaChrome();
await cmd("Page.enable"); await cmd("Runtime.enable");
await cmd("Emulation.setDeviceMetricsOverride", { width: 1440, height: 900, deviceScaleFactor: 1, mobile: false });
let male = false;
const conta = (nome, ottenuto, atteso) => { const ok = JSON.stringify(ottenuto) === JSON.stringify(atteso); if (!ok) male = true; console.log(`${ok ? "ok " : "MALE"} ${nome}: ${JSON.stringify(ottenuto)}${ok ? "" : ` (atteso ${JSON.stringify(atteso)})`}`); };
const click = (finder, testo) => val(`[...document.querySelectorAll(${JSON.stringify(finder)})].find((b) => b.innerText.includes(${JSON.stringify(testo)})).click()`);
const durate = () => val(`JSON.stringify(window.__osc)`).then(JSON.parse);
async function apri(percorso, imp, misura) {
  await cmd("Page.navigate", { url: BASE + "/scenario" }); await attendi(500);
  await val(`localStorage.setItem("impostazioni", ${JSON.stringify(JSON.stringify({ versione: 1, scala: 1, velocita: "1", movimento: "acceso", ...imp }))})`);
  await cmd("Page.navigate", { url: BASE + percorso }); await attendi(2000);
  await val(`window.__osc = []; { const o = Element.prototype.animate; Element.prototype.animate = function (k, op) { if (this.classList.contains("cart")) window.__osc.push(op.duration); return o.call(this, k, op); } } 1`);
  await click(".posto", misura); await attendi(200);
  await click(".lato button", "Anteprima"); await attendi(300);
}
await apri(AULA, {}, "Salario");
conta("apertura", await durate(), [1200]);
await click(".cart button", "Compra"); await attendi(600);
conta("test comprato", await durate(), [1200, 1200]);
conta("dopo la Vista nuova la misura scelta e il cartellino restano", await val(`!!document.querySelector(".cart") && document.querySelector(".cart .tit, .cart h4").innerText`), "Salario minimo legale: 11 €/h");
await apri(AULA, {}, "Sgravio");
await click(".cart button", "Compra"); await attendi(600);
conta("rifiuto del core: nessuna oscillazione in più", await durate(), [1200]);
await apri(AULA, { velocita: "2" }, "Salario");
conta("velocità 2×", await durate(), [600]);
await apri(AULA, { movimento: "spento" }, "Salario");
conta("movimento spento", await durate(), []);
await apri(AULA, { velocita: "istantaneo" }, "Salario");
conta("velocità istantanea", await durate(), []);
await cmd("Page.navigate", { url: BASE + "/scenario" }); await attendi(500);
await val(`localStorage.setItem("impostazioni", '{"versione":1,"scala":1,"velocita":"1","movimento":"acceso"}')`);
await cmd("Page.navigate", { url: BASE + "/partita/aula?vista=08-tono-grave.json" }); await attendi(2000);
await val(`window.__osc = []; { const o = Element.prototype.animate; Element.prototype.animate = function (k, op) { if (this.classList.contains("cart")) window.__osc.push(op.duration); return o.call(this, k, op); } } 1`);
await click(".lato button", "Anteprima"); await attendi(400);
conta("tono grave", [await val(`!!document.querySelector(".cart")`), await durate()], [true, []]);
// Fotogramma al picco: animazione in pausa, currentTime ogni 25 ms; sforamento = quanto il bordo del foglio supera il padding-box di `.lato` (≤ 0 = dentro).
const OPP = "/partita/aula?vista=19-aula.json&stato=opposizione_libero";
for (const [w, h, sc] of [[1280, 800, 1], [1280, 800, 1.25], [1440, 900, 1], [1440, 900, 1.4]]) {
  await cmd("Emulation.setDeviceMetricsOverride", { width: Math.round(w / sc), height: Math.round(h / sc), deviceScaleFactor: sc, mobile: false });
  for (const [percorso, misura, forma] of [[AULA, "Salario", "riforma"], [AULA, "Coalizioni", "elettorale"], [AULA, "Numero dei", "costituzionale"], [OPP, "Decreto", "decreto"]]) {
    await apri(percorso, { scala: sc }, misura);
    const sforo = await val(`(async () => {
      const c = document.querySelector(".cart"), l = document.querySelector(".lato"), a = c?.getAnimations()[0];
      if (!a) return null;
      a.pause();
      const cs = getComputedStyle(l), sx = l.getBoundingClientRect().left + parseFloat(cs.borderLeftWidth), dx = sx + l.clientWidth;
      let peggio = -Infinity;
      for (let t = 0; t <= 1200; t += 25) { a.currentTime = t; await new Promise((r) => requestAnimationFrame(r)); const b = c.getBoundingClientRect(); peggio = Math.max(peggio, sx - b.left, b.right - dx); }
      return +peggio.toFixed(1);
    })()`);
    conta(`picco dentro la colonna, ${forma} ${w}x${h} x${sc} (sforamento ${sforo} px)`, sforo !== null && sforo <= 0, true);
  }
}
chiudi();
process.exit(male ? 1 : 0);
