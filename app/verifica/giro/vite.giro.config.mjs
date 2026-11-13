// Configurazione Vite del solo giro nell'app vera: quella del progetto più il telecomando.
// Il plugin fa da coda di comandi fra cmd.mjs e giro.js, e aggiunge in coda a +layout.ts, in memoria, l'import di giro.js:
// nessun file dell'app cambia, e «npm run dev» e «npm run build» non lo vedono.
import base from "../../vite.config.js";
import { appendFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import { uscita } from "../comune.mjs";

const diario = join(uscita("giro"), "diario.jsonl");
mkdirSync(uscita("giro"), { recursive: true });
const coda = []; const esiti = new Map();
const corpo = (req) => new Promise((r) => { let s = ""; req.on("data", (d) => (s += d)); req.on("end", () => r(s)); });
const telecomando = {
  name: "telecomando-verifica",
  apply: "serve", // mai in un build, nemmeno lanciato a mano con questa configurazione
  configureServer(server) {
    server.middlewares.use(async (req, res, next) => {
      const u = req.url.split("?")[0];
      if (!["/__coda", "/__comando", "/__diario", "/__esito"].includes(u)) return next();
      const rifiuta = (codice) => { res.statusCode = codice; res.end(); };
      // Chi può pilotare l'app: solo questo Mac, e non una pagina qualunque aperta nel browser.
      // Passa chi non ha origine (cmd.mjs) o ha quella del server; e tutto ciò che cambia la coda è POST,
      // che da un'altra origine porta sempre Origin: un <img> o un GET in no-cors non svuota la coda.
      if (!/^(::ffff:)?127\.|^::1$/.test(req.socket.remoteAddress ?? "")) return rifiuta(403);
      const origine = req.headers.origin;
      if (origine && origine !== `http://${req.headers.host}`) return rifiuta(403);
      if (u !== "/__esito" && req.method !== "POST") return rifiuta(405);
      try {
        if (u === "/__coda") { coda.push(JSON.parse(await corpo(req))); res.end("ok"); }
        else if (u === "/__comando") { const c = coda.shift(); if (!c) rifiuta(204); else { res.setHeader("content-type", "application/json"); res.end(JSON.stringify(c)); } }
        else if (u === "/__diario") { const t = await corpo(req); const o = JSON.parse(t); esiti.set(String(o.id), t); appendFileSync(diario, t + "\n"); res.end("ok"); }
        else { const id = new URL(req.url, "http://x").searchParams.get("id"); if (esiti.has(id)) res.end(esiti.get(id)); else rifiuta(204); }
      } catch { rifiuta(400); } // un corpo che non è JSON non deve far cadere il server
    });
  },
  transform(codice, id) {
    // /verifica/giro/giro.js: relativo alla radice di Vite, cioè app/
    if (id.endsWith("/src/routes/+layout.ts")) return codice + '\nif (typeof window !== "undefined" && "__TAURI_INTERNALS__" in window) import(/* @vite-ignore */ "/verifica/giro/giro.js");\n';
  },
};
export default (env) => { const c = typeof base === "function" ? base(env) : base; return { ...c, plugins: [...c.plugins, telecomando] }; };
