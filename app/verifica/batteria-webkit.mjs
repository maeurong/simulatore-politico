// Batteria nel WebKit di sistema, il motore dell'app: le pagine, le finestre e le scale di batteria.mjs,
// caricate da wk.swift in una WKWebView fuori schermo. Misura tagli, forma stretta e scorrimento di lato,
// e scatta una fotografia; movimento e tastiera restano a batteria.mjs.
// Uso: npm run verifica:webkit -- <nome>=<percorso> [<nome>=<percorso> ...]
// Esce con 1 se una misura non vale: tagli diversi da zero, contatore muto, scala non applicata, pagina finita altrove.
// Al primo uso compila wk.swift con swiftc.
import { execFileSync } from "node:child_process";
import { mkdirSync, statSync } from "node:fs";
import { join } from "node:path";
import { BASE, CARICO, FINESTRE, altrove, esci, pagine, uscita } from "./comune.mjs";

const elenco = await pagine("npm run verifica:webkit --");
const out = uscita("webkit");
mkdirSync(out, { recursive: true });
const sorgente = join(import.meta.dirname, "wk.swift");
const wk = join(import.meta.dirname, "uscita", "wk");
const eta = (f) => { try { return statSync(f).mtimeMs; } catch { return 0; } };
if (eta(wk) < eta(sorgente)) {
  console.error("compilo wk.swift…");
  mkdirSync(join(import.meta.dirname, "uscita"), { recursive: true });
  execFileSync("swiftc", ["-O", sorgente, "-o", wk], { stdio: "inherit" });
}
// t: tagli, s: classi della radice (stretta, grave), z: scala applicata, dove: percorso d'arrivo, lato: scorre di lato, chi: i primi elementi tagliati
const ESPR = `(async () => { for (let i = 0; i < 50 && typeof window.__tagli !== "number"; i++) await new Promise((r) => setTimeout(r, 100)); const h = document.documentElement; return { t: window.__tagli ?? null, s: h.className, z: Number(h.style.zoom || 1), dove: location.pathname, lato: h.scrollWidth > h.clientWidth + 1, chi: [...document.querySelectorAll(".taglio")].slice(0, 8).map((e) => e.tagName + "." + e.className.replace(/ ?taglio/, "") + " «" + e.textContent.trim().slice(0, 30) + "»") }; })()`;

const TETTO = 60_000;
let male = false;
for (const [nome, percorso] of elenco) {
  for (const [w, h, scale] of FINESTRE) {
    for (const scala of scale) {
      const url = BASE + percorso + (percorso.includes("?") ? "&" : "?") + "tagli=1";
      const env = { ...process.env, WK_FOTO: join(out, `${nome}-${w}x${h}-x${scala}.png`) };
      let esito;
      try { esito = execFileSync(wk, [url, String(w), String(h), String(scala), ESPR], { env, encoding: "utf8", timeout: TETTO }).trim(); }
      catch (e) {
        esci(`wk ${nome} ${w}x${h} x${scala}: ` + (e.code === "ETIMEDOUT" ? `non ha risposto in ${TETTO / 1000} s. ${CARICO}`
          : `uscito con ${e.status ?? e.signal ?? e.code}: ${String(e.stderr || e.message).trim().slice(0, 300)}`));
      }
      // una misura vale se la pagina è quella chiesta, la scala è applicata e il contatore dà zero; un esito illeggibile non vale
      let guasto = "ILLEGGIBILE";
      try { const e = JSON.parse(esito); guasto = altrove(percorso, e.dove) ? "ALTROVE" : e.z !== scala ? "SCALA NON APPLICATA" : e.t !== 0 ? "TAGLI" : null; } catch {}
      console.log(`${nome} ${w}x${h} x${scala} ${esito}${guasto ? " ← " + guasto : ""}`);
      if (guasto) male = true;
    }
  }
}
console.log(`fotografie in ${out}`);
if (male) process.exitCode = 1;
