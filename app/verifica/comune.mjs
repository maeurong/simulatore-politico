// Pezzi comuni agli strumenti di verifica: indirizzo del server, cartella d'uscita, Chrome headless via CDP.
// Nessuna dipendenza: solo Node.
import { spawn } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

/** Il server di sviluppo: la porta è quella di vite.config.js, si cambia con VERIFICA_BASE. */
export const BASE = process.env.VERIFICA_BASE ?? "http://127.0.0.1:1420";
/** Dove finiscono screenshot ed esiti: verifica/uscita/<strumento>, fuori da git. VERIFICA_USCITA cambia la radice, non la sottocartella. */
export const uscita = (strumento) => join(process.env.VERIFICA_USCITA ?? join(import.meta.dirname, "uscita"), strumento);
/** Finestre e scale della batteria: la minima e quella di partenza, ciascuna a scala 1 e alla massima che regge. */
export const FINESTRE = [[1280, 800, [1, 1.25]], [1440, 900, [1, 1.4]]];
export const attendi = (ms) => new Promise((r) => setTimeout(r, ms));
export const esci = (testo) => { console.error(testo); process.exit(2); };
/** Sotto carico (una compilazione di cargo, per esempio) Chrome e wk non rispondono in tempo: il sospetto più probabile. */
export const CARICO = "Il sospetto più probabile è la macchina sotto carico, per esempio una compilazione di cargo: riprova a macchina scarica.";
/** Il nome di una pagina finisce nel nome dei file: solo lettere, cifre, - e _, così non esce da `uscita`. */
export const nomeValido = (nome) => /^[A-Za-z0-9_-]+$/.test(nome);
/** Col movimento spento restano solo le dissolvenze (spec, storia 55): le proprietà animate diverse da opacity. */
export const nonDissolvenze = (proprieta) => [...new Set(proprieta)].filter((p) => p !== "opacity");
/** Legge il registro dopo l'ultima animazione (transitionrun e animationstart partono al frame dopo il clic): null se lo script iniettato non ha girato. */
export async function leggiRegistro(val) {
  await attendi(250);
  return JSON.parse(await val("JSON.stringify(window.__anim ?? null)"));
}
/** Registra ogni animazione che parte, con le proprietà che anima: window.__anim è un elenco di elenchi. Va iniettato a ogni nuovo documento. */
export const REGISTRA_ANIMAZIONI = `(() => {
  window.__anim = [];
  const chiavi = (a) => [...new Set((a?.effect?.getKeyframes?.() ?? []).flatMap((k) => Object.keys(k)))].filter((p) => !["offset", "computedOffset", "easing", "composite"].includes(p));
  addEventListener("transitionrun", (e) => window.__anim.push([e.propertyName]), true);
  addEventListener("animationstart", (e) => { const a = e.target.getAnimations?.({ subtree: true }).find((x) => x.animationName === e.animationName); window.__anim.push(a ? chiavi(a) : ["@" + e.animationName]); }, true);
  const animate = Element.prototype.animate;
  Element.prototype.animate = function (...args) { const a = animate.apply(this, args); window.__anim.push(chiavi(a)); return a; };
})()`;

/** Il server risponde, è quello di questo checkout e ogni percorso esiste: altrimenti esce dicendo perché. */
export async function controlla(percorsi) {
  const prendi = (u) => fetch(BASE + u).catch(() => esci(`Il server di sviluppo non risponde su ${BASE}: lancia «npm run dev» in un altro terminale.`));
  // /@fs serve solo i file sotto la radice del server: quello di un altro worktree, sulla stessa porta, questo file non lo dà
  if (!(await prendi("/@fs" + import.meta.filename)).ok) esci(`Su ${BASE} risponde un server che non è di questo checkout: si misurerebbe un altro codice.`);
  for (const p of percorsi) if (!(await prendi(p)).ok) esci(`La pagina ${p} non c'è: il server risponde con un errore.`);
}

/** Le pagine dagli argomenti, nella forma nome=percorso. Senza pagine valide esce con l'uso. */
export async function pagine(uso) {
  const voci = process.argv.slice(2).map((v) => [v.slice(0, Math.max(v.indexOf("="), 0)), v.slice(v.indexOf("=") + 1)]);
  if (!voci.length || voci.some(([nome, percorso]) => !nomeValido(nome) || !percorso.startsWith("/"))) {
    esci(`Uso: ${uso} <nome>=<percorso> [<nome>=<percorso> ...]\n  il nome ha solo lettere, cifre, - e _\n  il percorso comincia con /\n  es. 'edicola=/?vista=13-edicola.json' scenario=/scenario`);
  }
  await controlla(voci.map(([, percorso]) => percorso));
  return voci;
}

/** La pagina è rimasta dove l'abbiamo mandata? Una Vista che non esiste rimanda all'edicola, e lì i tagli sono zero. */
export const altrove = (percorso, arrivo) => new URL(percorso, BASE).pathname !== arrivo;

/** Lancia Chrome headless e apre il canale CDP sulla prima pagina. `suEvento` riceve ogni messaggio. */
export async function avviaChrome(suEvento = () => {}) {
  const profilo = mkdtempSync(join(tmpdir(), "verifica-chrome-"));
  // porta 0: la sceglie il sistema, e Chrome la scrive in DevToolsActivePort dentro il profilo
  const chrome = spawn(process.env.CHROME ?? "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome", [
    "--headless=new", "--remote-debugging-port=0", `--user-data-dir=${profilo}`, "--no-first-run", "--hide-scrollbars", "about:blank",
  ], { stdio: "ignore" });
  chrome.on("error", (e) => esci(`Chrome non parte (${e.message}): indica l'eseguibile con CHROME=<percorso>.`));
  // comunque finisca lo script, Chrome non resta acceso e il profilo non resta su disco; la pulizia non cambia il codice d'uscita
  process.on("exit", () => {
    chrome.kill("SIGKILL");
    try { rmSync(profilo, { recursive: true, force: true, maxRetries: 5, retryDelay: 100 }); } catch {}
  });
  process.on("SIGINT", () => process.exit(130));
  let pagina;
  // 30 secondi d'orologio, fetch compresa: sotto carico una fetch può non tornare
  const limite = Date.now() + 30_000;
  while (!pagina && Date.now() < limite) {
    try {
      const porta = readFileSync(join(profilo, "DevToolsActivePort"), "utf8").split("\n")[0];
      pagina = (await (await fetch(`http://127.0.0.1:${porta}/json/list`, { signal: AbortSignal.timeout(Math.max(1, limite - Date.now())) })).json()).find((x) => x.type === "page");
    } catch {}
    if (!pagina) await attendi(100);
  }
  if (!pagina) esci(`Chrome non ha aperto il canale di debug entro 30 secondi. ${CARICO}`);
  const ws = new WebSocket(pagina.webSocketDebuggerUrl);
  ws.onerror = () => esci("Il canale di debug di Chrome è caduto.");
  await new Promise((r) => {
    const tetto = setTimeout(() => esci(`Il canale di debug di Chrome non si è aperto entro 30 secondi. ${CARICO}`), 30_000);
    ws.onopen = () => { clearTimeout(tetto); r(); };
  });
  let n = 0; const attese = new Map();
  ws.onmessage = (m) => {
    const d = JSON.parse(m.data);
    if (d.id && attese.has(d.id)) { attese.get(d.id)(d); attese.delete(d.id); }
    suEvento(d);
  };
  const cmd = (method, params = {}) => new Promise((r) => {
    const id = ++n;
    // un comando senza risposta non lascia lo script appeso: esce dicendo quale
    const tetto = setTimeout(() => esci(`Chrome non risponde a ${method} da 30 secondi. ${CARICO}`), 30_000);
    attese.set(id, (d) => { clearTimeout(tetto); r(d); });
    ws.send(JSON.stringify({ id, method, params }));
  });
  const val = async (e) => (await cmd("Runtime.evaluate", { expression: e, returnByValue: true, awaitPromise: true })).result?.result?.value;
  const chiudi = () => { ws.close(); chrome.kill(); };
  return { cmd, val, chiudi };
}
