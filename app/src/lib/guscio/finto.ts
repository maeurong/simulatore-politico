import type { ErroreGuscio } from "./chiamate";
import type { Vista } from "$lib/tipi/Vista";

// ponytail: tutto `any` — il campione è JSON non tipato, il finto è solo per sviluppo.
/* eslint-disable @typescript-eslint/no-explicit-any */
type Campioni = {
  campione: any; turnoZero?: any; edicola: any; editor: any; fondazione: any;
  paese: any; campagna: any; trattativa: any;
  /** 17-, 18-, 19-, 20-: i campioni della fetta 11c */
  cruscotto?: any; governo?: any; aula?: any; anteprime?: any;
  /** 21-: i Conti; ?stato= ne sceglie uno, servito come `.prima` */
  conti?: any;
  /** 22-: il Bilancio, caricato sempre; varianti nel bozza_04 */
  bilancio?: any;
  /** la Vista del 21 dopo un compra riuscito: da lì il secondo compra è un rifiuto */
  servita?: any;
  /** ?stato= dell'URL: sceglie la prova del 26, lo stato del 23 e quello del 21 */
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
  // lo stato del 21 ha prima e vista: la Vista di partenza è la prima
  if (campione && typeof campione === "object" && "cassa_prima" in campione && campione.prima) return campione.prima;
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
function esito(voce: any, comando: string, campo = "ok"): any {
  presente(voce, comando);
  if (voce.rifiuto != null) throw rifiuto(voce.rifiuto);
  return voce[campo];
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

const sorgente = (s: unknown) => (s == null ? "None" : `Some(${JSON.stringify(s)})`);

/** La mossa nella forma Debug di Rust con cui il campione 18 elenca le sue `mosse`. */
function mossa(cmd: any): string | undefined {
  const d = JSON.stringify(cmd.dicastero);
  switch (cmd.tipo) {
    case "ferma": return `Ferma { dicastero: ${d} }`;
    case "chiedi_dimissioni": return `ChiediDimissioni { dicastero: ${d} }`;
    case "dirotta": return `Dirotta { dicastero: ${d}, atto: ${sorgente(cmd.atto)}, regione: ${sorgente(cmd.regione)} }`;
  }
}

/** Le anteprime del 20 (voci con `valori` e `anteprima` o `rifiuto`) che hanno questi valori. */
function vociAnteprima(valori: unknown, c: Campioni): any {
  const a = c.anteprime;
  return [...Object.values(a?.anteprime ?? {}), ...(a?.gradini ?? [])].find((v: any) => uguali(v.valori, valori));
}

/** Le varianti del 22 con bilancio e riepilogo (o rifiuto), più le due bozze col loro riepilogo. */
function variantiBilancio(bil: any): any[] {
  const b04 = bil?.bozza_04;
  if (!b04) return [];
  const va = b04.varianti;
  return [
    { bilancio: b04.bozza, ...b04.riepilogo_bozza },
    { bilancio: bil.provvisorio?.bozza, ...bil.provvisorio?.riepilogo_bozza },
    va.tutto, va.scaglioni, ...va.per_dicastero,
    ...Object.values(va.margine_passo ?? {}),
    ...Object.values(va.aliquote_passo ?? {}).flatMap((x: any) => [x.meno, x.piu]),
  ].filter((x) => x?.bilancio);
}

function riepilogoBilancio(bilancio: unknown, c: Campioni): unknown {
  const v = vistaDi(c.campione), bil = c.bilancio;
  if (v && !v.governa) return esito(bil?.opposizione?.rifiuto_riepilogo, "riepilogo_bilancio");
  if (v && v.data.trimestre !== 4 && !v.bilancio.presentabile) return esito(bil?.fuori_stagione, "riepilogo_bilancio");
  return esito(variantiBilancio(bil).find((x) => uguali(x.bilancio, bilancio)), "riepilogo_bilancio", "riepilogo");
}

function indicizzaBilancio(voce: any, bil: any): unknown {
  const va = bil?.bozza_04?.varianti;
  const x = voce?.tipo === "dicastero" ? va?.per_dicastero.find((p: any) => p.id === voce.id) : va?.[voce?.tipo];
  return esito(x, "indicizza", "bilancio");
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
      const test = cmd.acquisto?.test_misura;
      if (test) {
        const chiave = Object.keys(c.anteprime?.anteprime ?? {}).find((k) => uguali(c.anteprime.anteprime[k].valori, test.valori));
        const voce = chiave && c.anteprime.test?.[chiave];
        if (!voce) return undefined;
        if (v?.acquisti.test_misura) throw rifiuto(voce.rifiuto_ripetuto);
        return { ...v, acquisti: { ...v?.acquisti, test_misura: voce.acquisti.test_misura } };
      }
      if (cmd.acquisto === "centro_studi") {
        const voce = c.conti?.[stato ?? ""];
        if (!voce) return undefined;
        if ((c.servita ?? v)?.acquisti.centro_studi) throw rifiuto(voce.rifiuto_ripetuto);
        return (c.servita = voce.vista);
      }
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
    case "ferma": case "dirotta": case "chiedi_dimissioni": {
      const scelte = c.governo?.scelte;
      return scelte?.mosse.some((m: any) => m.mossa === mossa(cmd)) ? scelte.vista : undefined;
    }
    case "scegli_successore": return c.governo?.successore_scelto && esito(c.governo.successore_scelto, "comanda", "vista");
    case "dimettiti": return c.cruscotto?.dimissioni?.vista;
    case "rinvia": return cmd.dicastero === "ambiente_energia" ? c.cruscotto?.scelte_fatte?.vista : undefined;
    case "rispondi_evento": return cmd.opzione === "debito" ? c.cruscotto?.scelte_fatte?.vista : undefined;
    case "presenta_legge": {
      const piena = c.aula?.piena?.vista;
      const misura = piena?.calendario.find((m: any) => [1, 2, 3].includes(m.id) && uguali(m.valori, cmd.valori));
      if (!misura) return undefined;
      if (misura.forma === "costituzionale" && v?.calendario?.some((m: any) => m.forma === "costituzionale")) {
        throw rifiuto(c.aula.rifiuto_calendario_costituzionale);
      }
      return piena;
    }
    case "presenta_proposta": case "linea": return c.aula?.opposizione_proposta?.vista;
    case "presenta_bilancio": return uguali(cmd.bilancio, c.bilancio?.bozza_04?.bozza) ? c.bilancio.presentato.vista : undefined;
    case "emendamento": {
      const giocate = [["centrodestra", true], ["destra", false]];
      const giocata = cmd.misura === c.bilancio?.presentato?.misura && giocate.some(([p, a]) => p === cmd.partito && a === cmd.accogli);
      return giocata ? c.bilancio.emendamenti_decisi.vista : undefined;
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
    case "anteprima": return esito(presente(vociAnteprima(args.valori, c), comando), comando, "anteprima");
    case "anteprima_accogli": {
      const voce: any = [{ dicastero: "infrastrutture_trasporti", anteprima: c.cruscotto?.anteprima_accogli_03 }, c.anteprime?.anteprime.accogli]
        .find((e) => e?.dicastero === args.dicastero && e.anteprima);
      return (presente(voce, comando) as any).anteprima;
    }
    case "anteprima_decreto": {
      const d = args.decreto as any;
      const voci: any[] = [c.anteprime?.decreto, c.anteprime?.decreto_grave].filter((x) => d?.tipo === "disposizioni" && x && uguali(x.valori, d.valori));
      return (presente(voci.find((x) => uguali(x.vista, vistaDi(campione))) ?? voci[0], comando) as any).anteprima;
    }
    case "anteprima_proposta": {
      const voce = c.aula?.anteprima_proposta;
      return esito(voce && uguali(voce.valori, args.valori) ? voce : null, comando);
    }
    case "menu_elettorale": {
      const { menu_03, menu_proporzionale, piena } = c.aula ?? {};
      if (uguali(args.valori, [])) return presente(menu_03, comando);
      return presente(uguali(args.valori, piena?.vista.calendario[1]?.valori) ? menu_proporzionale : null, comando);
    }
    case "costo_dirotta": {
      const d = c.governo?.costo_dirotta_03?.[args.dicastero as string];
      const costo = args.atto != null ? d?.atti[args.atto as string] : args.regione != null ? d?.regioni[args.regione as string] : null;
      if (typeof costo === "number") return costo;
      return esito(costo ?? null, comando);
    }
    case "serie": return campione?.serie ?? [];
    case "bozza_bilancio": {
      const bil = c.bilancio;
      return bil?.[c.stato ?? ""]?.bozza ?? bil?.bozza_04?.bozza ?? null;
    }
    case "indicizza": return indicizzaBilancio(args.voce, c.bilancio);
    case "riepilogo_bilancio": return riepilogoBilancio(args.bilancio, c);
    case "riepilogo_presentato": return c.bilancio?.[c.stato ?? ""]?.riepilogo_presentato ?? null;
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
    cruscotto: await carica("17-"),
    governo: await carica("18-"),
    aula: await carica("19-"),
    anteprime: await carica("20-"),
    conti: await carica("21-"),
    bilancio: await carica("22-"),
    stato,
  };
  const { mockIPC } = await import("@tauri-apps/api/mocks");
  mockIPC((cmd, args) => rispondi(cmd, (args ?? {}) as Record<string, unknown>, c));
  return vistaDi(campione);
}
