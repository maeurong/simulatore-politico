// Manda i comandi al telecomando, in ordine, e stampa gli esiti.
// Uso: npm run verifica:comando -- '<json del comando>' ['<json>' ...]
//   es. npm run verifica:comando -- '{"passo":"stato"}' '{"passo":"clicca","testo":"Nuova partita"}'
// Esce con 1 se un passo non ha risposta, è rifiutato, non trova l'elemento o va in errore.
// Un comando: { passo, sel | testo [, tra, n], attesa (ms dopo il passo, 500 se manca), ... }; i passi stanno in giro.js.
import { BASE } from "../comune.mjs";

let n = Date.now();
for (const a of process.argv.slice(2)) {
  const c = { id: ++n, ...JSON.parse(a) };
  const accolto = await fetch(BASE + "/__coda", { method: "POST", body: JSON.stringify(c) });
  // la guardia del telecomando ha detto no (403, 405): inutile aspettare 30 secondi una risposta che non verrà
  if (!accolto.ok) { console.error(`${c.passo}: il telecomando ha rifiutato il comando (stato ${accolto.status}) su ${BASE}`); process.exit(1); }
  let esito = null;
  for (let i = 0; i < 150 && esito === null; i++) { const r = await fetch(BASE + "/__esito?id=" + c.id); if (r.status === 200) esito = await r.json(); else await new Promise((r) => setTimeout(r, 200)); }
  if (!esito || /^(ERRORE|RIFIUTATO|NON TROVATO)/.test(esito.esito)) process.exitCode = 1;
  console.log(`${c.passo} ${JSON.stringify({ ...c, id: undefined, passo: undefined, attesa: undefined })} → [${esito?.url ?? "?"}] ${esito ? (typeof esito.esito === "string" ? esito.esito : JSON.stringify(esito.esito)) : "NESSUNA RISPOSTA"}`);
}
