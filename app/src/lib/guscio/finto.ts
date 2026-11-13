import type { ErroreGuscio } from "./chiamate";
import type { Vista } from "$lib/tipi/Vista";

// ponytail: tutto `any` — il campione è JSON non tipato, il finto è solo per sviluppo.
/* eslint-disable @typescript-eslint/no-explicit-any */
type Campioni = { campione: any; turnoZero?: any; edicola: any; editor: any; fondazione: any };

const rifiuto = (testo: string): ErroreGuscio => ({ specie: "rifiuto", testo });
const uguali = (a: unknown, b: unknown) => JSON.stringify(a) === JSON.stringify(b);

const presente = (v: unknown, comando: string) => {
  if (v == null) throw rifiuto(`non nel campione: ${comando}`);
  return v;
};

export function trovaFile(nomi: string[], vista: string): string | undefined {
  return (
    nomi.find((n) => n === vista || n === `${vista}.json`) ??
    nomi.filter((n) => n.startsWith(vista)).sort((a, b) => a.length - b.length)[0]
  );
}

export function vistaDi(campione: any): Vista | null {
  // "giocatore", non "turno": le voci del registro hanno un turno loro e la Vista in .prima
  if (campione && typeof campione === "object" && "giocatore" in campione) return campione;
  return campione?.vista ?? campione?.prima ?? null;
}

function riepilogo(scenario: any, editor: any): unknown {
  const base = editor.riepiloghi[0];
  if (scenario == null) return base.riepilogo;
  const noto = editor.riepiloghi.find((r: any) => uguali(r.scenario, scenario));
  if (noto) {
    if (noto.errore) throw rifiuto(noto.errore);
    return noto.riepilogo;
  }
  const b = base.scenario;
  const diversi = Object.keys(scenario.cursori).filter((k) => scenario.cursori[k] !== b.cursori[k]);
  if (
    diversi.length === 1 &&
    uguali(scenario.disposizioni, b.disposizioni) &&
    uguali(scenario.sostituzioni, b.sostituzioni)
  ) {
    const k = diversi[0];
    const griglia = editor.griglia[k];
    if (griglia) {
      const v = scenario.cursori[k];
      return griglia.reduce((m: any, p: any) =>
        Math.abs(p.valore - v) < Math.abs(m.valore - v) ? p : m,
      ).riepilogo;
    }
  }
  throw rifiuto("non nel campione: riepilogo_scenario");
}

export function rispondi(comando: string, args: Record<string, unknown>, c: Campioni): unknown {
  const { campione, edicola, editor, fondazione } = c;
  switch (comando) {
    case "chiudi_turno": return presente(campione?.dopo, comando);
    case "comanda": return presente(campione?.dopo_comando, comando);
    case "serie": return campione?.serie ?? [];
    case "riprendi": return presente(vistaDi(campione), comando);
    case "partite": return edicola.partite;
    case "albo": return edicola.albo;
    case "scenari": return editor.scenari_salvati;
    case "carica_scenario": {
      const r = editor.riepiloghi.find((r: any) => r.scenario.nome === args.nome);
      if (!r) throw rifiuto(`non nel campione: carica_scenario ${String(args.nome)}`);
      return r.scenario;
    }
    case "catalogo_scenario": {
      const { cursori, leggi_elettorali, legge_di_base, disposizioni } = editor;
      return { cursori, leggi_elettorali, legge_di_base, disposizioni, partiti: fondazione.partiti };
    }
    case "riepilogo_scenario": return riepilogo(args.scenario, editor);
    case "anteprima_fondazione": {
      const s = args.scenario as any;
      return fondazione.anteprime[s?.cursori?.orientamento === 1 ? "elettorato a destra, seme 4" : "oggi, seme 4"];
    }
    case "fonda_partita": return vistaDi(c.turnoZero);
    case "salva_scenario": return null;
    default: throw rifiuto(`non nel campione: ${comando}`);
  }
}

export async function installaFinto(url: URL): Promise<Vista | null> {
  if (!import.meta.env.DEV || "__TAURI_INTERNALS__" in window) return null;
  const file = import.meta.glob("../../../../.scratch/v1-piano-11-ui/viste/*.json", {
    import: "default",
  });
  const carica = async (prefisso: string) => {
    const nomi = Object.keys(file).map((k) => k.split("/").pop()!);
    const trovato = trovaFile(nomi, prefisso);
    const chiave = Object.keys(file).find((k) => k.endsWith(`/${trovato}`));
    return chiave ? await file[chiave]() : null;
  };
  const nome = url.searchParams.get("vista");
  const stato = url.searchParams.get("stato");
  let campione: any = nome ? await carica(nome) : null;
  if (campione && stato !== null) campione = campione[stato];
  const c: Campioni = {
    campione,
    turnoZero: await carica("01-"),
    // ?edicola=vuota: il primo avvio, nessuna partita e albo vuoto
    edicola: url.searchParams.get("edicola") === "vuota" ? { partite: [], albo: [] } : await carica("13-"),
    editor: await carica("14-"),
    fondazione: await carica("15-"),
  };
  const { mockIPC } = await import("@tauri-apps/api/mocks");
  mockIPC((cmd, args) => rispondi(cmd, (args ?? {}) as Record<string, unknown>, c));
  return vistaDi(campione);
}
