// Tauri doesn't have a Node.js server to do proper SSR
// so we use adapter-static with a fallback to index.html to put the site in SPA mode
// See: https://svelte.dev/docs/kit/single-page-apps
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
import "$lib/impostazioni.svelte"; // all'import applica la scala, prima del primo disegno
import { partita } from "$lib/partita.svelte";

export const ssr = false;

// una volta sola, all'avvio: load non legge l'URL e non si rilancia a ogni navigazione.
// import() dentro DEV: nel build il ramo cade e il finto non entra nel rilascio.
// Il finto risponde a riprendi con la Vista del campione: Vista e serie passano da #esegui, guasto compreso
const finto = import.meta.env.DEV
  ? import("$lib/guscio/finto").then(async ({ installaFinto }) => {
      if (await installaFinto(new URL(location.href))) await partita.riprendi("finto");
    })
  : null;

// contatore dei tagli: solo in sviluppo e solo con ?tagli=1
if (import.meta.env.DEV && new URL(location.href).searchParams.get("tagli") === "1") {
  import("$lib/sviluppo/tagli").then((m) => m.avvia());
}

export const load = async () => {
  await finto;
};
