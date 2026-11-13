import type { ErroreGuscio } from "./chiamate";
import type { Vista } from "$lib/tipi/Vista";

// ponytail: tutto `any` — il campione è JSON non tipato, il finto è solo per sviluppo.
/* eslint-disable @typescript-eslint/no-explicit-any */
type Campioni = {
  campione: any; turnoZero?: any; edicola: any; editor: any; fondazione: any;
  paese: any; campagna: any; trattativa: any;
  /** ?stato= dell'URL: sceglie la prova del 26 e lo stato del 23 */
  stato: string | null;
};

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

/** {ok} → ok, {rifiuto} → il rifiuto del core, niente → non nel campione. */
function esito(voce: any, comando: string): any {
  presente(voce, comando);
  if (voce.rifiuto != null) throw rifiuto(voce.rifiuto);
  return voce.ok;
}

const forma = (s: any) => JSON.stringify({ bersaglio: s.bersaglio, leva: s.leva, euro: s.euro });

function anteprimaCampagna(spese: any[], campagna: any): unknown {
  const esatta = [...campagna.anteprime, campagna.piano].find(
    (e) => e.ok?.length === spese.length && e.ok.every((v: any, i: number) => forma(v) === forma(spese[i])),
  );
  if (esatta) return esatta.ok;
  const voci: any[] = campagna.anteprime.flatMap((e: any) => e.ok ?? []);
  return spese.map((s) => {
    const stessi = voci.filter((v) => uguali(v.bersaglio, s.bersaglio));
    if (!stessi.length) throw rifiuto("non nel campione: anteprima_campagna");
    return stessi.reduce((m, v) => (Math.abs(v.euro - s.euro) < Math.abs(m.euro - s.euro) ? v : m));
  });
}

/** La Vista dopo un comando della fetta, dal campione; undefined se il campione non l'ha giocato. */
function dopoComando(cmd: any, c: Campioni): unknown {
  const { campione, campagna, paese, stato } = c;
  const v = vistaDi(campione);
  // la trattativa sta nel campione in uso se è un 27, altrimenti nello stato del 27 che porta la voce
  const di27 = (voce: string): any => (campione?.[voce] ? campione : Object.values(c.trattativa ?? {}).find((s: any) => s?.[voce]));
  switch (cmd.tipo) {
    case "dichiara_coalizione": {
      const ordine = (id: string) => v?.partiti.findIndex((p) => p.id === id) ?? 0;
      const chiave = [...cmd.partiti].sort((a: string, b: string) => ordine(a) - ordine(b)).join("+");
      const [disponibilita] = esito(campagna.prove?.[stato ?? ""]?.coalizioni?.[chiave], "comanda");
      return { ...v, coalizione: cmd.partiti, disponibilita };
    }
    case "compra": {
      if (cmd.acquisto !== "sondaggio_privato") return undefined;
      const prova = campagna.prove?.[stato ?? ""]?.privato;
      const sondaggio = prova ? esito(prova, "comanda") : paese[stato ?? ""]?.vista?.acquisti?.sondaggio_privato;
      return { ...v, acquisti: { ...v?.acquisti, sondaggio_privato: presente(sondaggio, "comanda") } };
    }
    case "campagna": {
      const spese: any[] = cmd.spese;
      const cassa = v?.partiti.find((p) => p.id === v.giocatore)?.cassa ?? Infinity;
      const rifiuti = campagna.rifiuti_campagna;
      if (spese.length > 3) return esito(rifiuti.quattro, "comanda");
      if (spese.some((s) => !(s.euro > 0))) return esito(rifiuti.zero_euro, "comanda");
      if (new Set(spese.map((s) => JSON.stringify(s.bersaglio))).size < spese.length) return esito(rifiuti.ripetuto, "comanda");
      if (spese.reduce((t, s) => t + s.euro, 0) > cassa) return esito(rifiuti.oltre_cassa, "comanda");
      return undefined;
    }
    case "riposiziona": return { ...v, partiti: esito(campagna.riposiziona?.[String(cmd.posizione)], "comanda") };
    case "prometti": {
      const voce = campagna.promesse?.[{ 2: "due", 3: "tre" }[cmd.promesse.length as 2 | 3] ?? ""];
      return voce ? { ...v, partiti: esito(voce, "comanda") } : undefined;
    }
    case "proponi_spartizione": {
      const t = di27("proposte");
      const proposta = t?.proposte.find((p: any) => uguali(p.spartizione, cmd.dicasteri));
      return proposta ? { ...vistaDi(t), richieste: (esito(proposta.esito, "comanda"), proposta.richieste) } : undefined;
    }
    case "allarga": {
      const tutti = cmd.partiti.length > 1;
      const t = di27(tutti ? "allarga_tutti" : "allarga_uno");
      const voce = tutti ? t?.allarga_tutti : t?.allarga_uno.find((e: any) => e.partito === cmd.partiti[0]);
      if (!voce) return undefined;
      esito(voce.esito, "comanda");
      return tutti ? voce.vista : voce.dopo;
    }
    case "chiedi_guida": return di27("dopo_chiedi")?.dopo_chiedi;
    case "rifiuta_ribaltone": return di27("dopo_rifiuta")?.dopo_rifiuta;
    default: return undefined;
  }
}

export function rispondi(comando: string, args: Record<string, unknown>, c: Campioni): unknown {
  const { campione, edicola, editor, fondazione } = c;
  switch (comando) {
    case "chiudi_turno": return presente(campione?.dopo, comando);
    case "comanda": return dopoComando(args.comando ?? {}, c) ?? presente(campione?.dopo_comando, comando);
    case "anteprima_campagna": return anteprimaCampagna(args.spese as any[], c.campagna);
    case "elettori_campione": {
      // il campione del 23 porta il suo stato; fuori dal 23 vale quello del governo
      const gruppi = campione?.campione ?? c.paese.governo.campione;
      return presente(gruppi[`${args.regione}|${args.condizione}|${args.fascia}`], comando);
    }
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
    paese: await carica("23-"),
    campagna: await carica("26-"),
    trattativa: await carica("27-"),
    stato,
  };
  const { mockIPC } = await import("@tauri-apps/api/mocks");
  mockIPC((cmd, args) => rispondi(cmd, (args ?? {}) as Record<string, unknown>, c));
  return vistaDi(campione);
}
