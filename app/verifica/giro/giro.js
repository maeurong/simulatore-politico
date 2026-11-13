// Telecomando di sola verifica: gira dentro la WebView vera dell'app, prende i comandi dal server Vite del giro
// (vite.giro.config.mjs) e risponde con l'esito. Nessun file dell'app lo importa: lo aggancia il plugin, in memoria.
const avvisi = [];
for (const k of ["warn", "error"]) { const o = console[k].bind(console); console[k] = (...a) => { avvisi.push(k + ": " + a.map((x) => (x && x.testo) || String(x)).join(" ").slice(0, 300)); o(...a); }; }
addEventListener("securitypolicyviolation", (e) => avvisi.push(`csp: ${e.violatedDirective} ← ${e.blockedURI}`));
addEventListener("error", (e) => avvisi.push("errore: " + e.message));
addEventListener("unhandledrejection", (e) => avvisi.push("rigetto: " + (e.reason?.testo ?? e.reason)));
const trova = (c) => c.sel ? document.querySelectorAll(c.sel)[c.n ?? 0] : [...document.querySelectorAll(c.tra ?? "button, a, input, select, label, [role=radio]")].filter((e) => (e.textContent || "").trim().includes(c.testo))[c.n ?? 0];
let dopo = null; // vai e ricarica navigano solo dopo aver risposto: altrimenti la risposta si perde con la pagina
const passi = {
  // con ?tagli=1 il contatore conta con un ritardo: stato aspetta il numero, fino a 5 s
  stato: async () => { if (/[?&]tagli=1/.test(location.search)) for (let i = 0; i < 50 && typeof window.__tagli !== "number"; i++) await new Promise((r) => setTimeout(r, 100)); return { url: location.pathname + location.search, innerWidth, innerHeight, clientWidth: document.documentElement.clientWidth, zoom: document.documentElement.style.zoom, classi: document.documentElement.className, movimento: document.documentElement.dataset.movimento ?? null, k: getComputedStyle(document.documentElement).getPropertyValue("--k"), fonts: [...document.fonts].map((f) => `${f.family} ${f.style} ${f.status}`), fraunces: document.fonts.check('700 23px Fraunces'), atkinson: document.fonts.check('400 14px "Atkinson Hyperlegible Next"'), corsivo: document.fonts.check('italic 400 14px "Atkinson Hyperlegible Next"'), riduce: matchMedia("(prefers-reduced-motion: reduce)").matches, dpr: devicePixelRatio, animazioni: document.getAnimations().length, tagli: window.__tagli ?? null, ls: localStorage.getItem("impostazioni"), tauri: "__TAURI_INTERNALS__" in window, ua: navigator.userAgent, avvisi: avvisi.splice(0) }; },
  testo: (c) => ((c.sel || c.testo ? trova(c)?.innerText : document.body.innerText) ?? "NON TROVATO").replace(/\n{2,}/g, "\n").slice(0, c.max ?? 1500),
  clicca: (c) => { const e = trova(c); if (!e) return "NON TROVATO"; e.scrollIntoView({ block: "center" }); e.click(); return (e.textContent || e.name || e.tagName).trim().slice(0, 60); },
  scrivi: (c) => { const e = trova(c); if (!e) return "NON TROVATO"; const p = e.tagName === "SELECT" ? HTMLSelectElement.prototype : HTMLInputElement.prototype; Object.getOwnPropertyDescriptor(p, "value").set.call(e, c.valore); e.dispatchEvent(new Event("input", { bubbles: true })); e.dispatchEvent(new Event("change", { bubbles: true })); return e.value; },
  tasto: (c) => { const e = new KeyboardEvent("keydown", { key: c.key, code: c.code, metaKey: !!c.meta, bubbles: true, cancelable: true }); (c.sel ? trova(c) : document.activeElement ?? document.body).dispatchEvent(e); return "prevenuto=" + e.defaultPrevented; },
  // senza sel né testo legge chi ha il fuoco adesso; con un bersaglio che non c'è non descrive l'elemento di prima
  fuoco: (c) => { if (c.sel || c.testo) { const e = trova(c); if (!e) return "NON TROVATO"; e.focus(); } const a = document.activeElement; return a ? a.tagName + " «" + (a.textContent || a.value || "").trim().slice(0, 40) + "»" : null; },
  rett: (c) => { const e = trova(c); if (!e) return "NON TROVATO"; const r = e.getBoundingClientRect(); return { x: r.x, y: r.y, w: r.width, h: r.height }; },
  elenco: (c) => [...document.querySelectorAll(c.sel)].map((e) => (e.innerText || e.value || e.getAttribute("aria-label") || "").trim().replace(/\s+/g, " ").slice(0, c.max ?? 90)),
  stile: (c) => { const e = trova(c); if (!e) return "NON TROVATO"; const s = getComputedStyle(e); return Object.fromEntries((c.prop ?? ["fontFamily", "fontSize"]).map((p) => [p, s[p]])); },
  // il localStorage delle impostazioni è lo stesso dell'app di sviluppo di Mario: a fine giro si rimette il valore letto da «stato» all'inizio
  rimetti: (c) => { if (c.ls == null) localStorage.removeItem("impostazioni"); else localStorage.setItem("impostazioni", typeof c.ls === "string" ? c.ls : JSON.stringify(c.ls)); return localStorage.getItem("impostazioni"); },
  ricarica: () => { dopo = () => location.reload(); return "ricarico"; },
  // solo percorsi dell'app: un indirizzo esterno o un javascript: non passa
  vai: (c) => { if (!/^\/(?!\/)/.test(c.url ?? "")) return "RIFIUTATO: solo percorsi che cominciano con /"; dopo = () => (location.href = c.url); return "vado"; },
};
(async () => { for (;;) { try { const r = await fetch("/__comando", { method: "POST" }); if (r.status === 200) { const c = await r.json(); let esito; try { esito = await passi[c.passo](c); } catch (e) { esito = "ERRORE: " + e; } await new Promise((r) => setTimeout(r, c.attesa ?? 500)); await fetch("/__diario", { method: "POST", body: JSON.stringify({ id: c.id, passo: c.passo, esito, url: location.pathname }) }); if (dopo) return dopo(); } } catch {} await new Promise((r) => setTimeout(r, 200)); } })();
