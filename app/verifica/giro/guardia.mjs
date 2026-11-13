// Chi può pilotare l'app dal telecomando: solo questo Mac, e non una pagina qualunque aperta nel browser.
// Funzione pura, senza import: la chiama il plugin di vite.giro.config.mjs e la prova guardia.test.mjs.

/**
 * Il codice con cui rifiutare la richiesta (403 o 405), oppure null se passa.
 * `u` è il percorso senza query, `porta` quella del server del giro.
 */
export function rifiuto({ remoteAddress, host, origin, method }, u, porta) {
  // solo connessioni dal loopback: con TAURI_DEV_HOST il server ascolta anche sulla rete
  if (!/^(::ffff:)?127\.|^::1$/.test(remoteAddress ?? "")) return 403;
  // l'Host dev'essere uno dei nomi locali del server: un nome estraneo che punta qui è DNS rebinding
  if (![`127.0.0.1:${porta}`, `localhost:${porta}`, `[::1]:${porta}`].includes(host)) return 403;
  // passa chi non ha origine (cmd.mjs) o ha quella del server; «null» è l'origine di una pagina isolata
  if (origin && origin !== `http://${host}`) return 403;
  // tutto ciò che cambia la coda è POST, che da un'altra origine porta sempre Origin:
  // un <img> o un GET in no-cors non svuota la coda
  if (u !== "/__esito" && method !== "POST") return 405;
  return null;
}
