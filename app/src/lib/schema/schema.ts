// Schema dello Stato (spec:402): nodi e fili ricavati dalla Vista, nessun campo nuovo (docs/ricerca/sintesi/vista-per-schermata.md:62).
// Il riferimento per capi, etichette e frasi è modello() in 29-schema.prototype.html.
import type { Vista } from "$lib/tipi/Vista";
import type { VoceMinistero } from "$lib/tipi/VoceMinistero";
import type { VoceEffetto } from "$lib/tipi/VoceEffetto";
import type { CausaIntesa } from "$lib/tipi/CausaIntesa";
import { euro } from "$lib/governo/governo";
import { CAUSE } from "$lib/partiti/schieramento";
import { nomeDi } from "$lib/partiti";
import { periodoDi } from "$lib/aula/aula";
import type { Sezione } from "$lib/ossatura";
import { elenco, numero, segnato, t, type Chiave } from "$lib/testi/t";

export type Colonna = 0 | 1 | 2 | 3;
export type TipoNodo = "partito" | "camera" | "presidente" | "ministero" | "paese" | "situazione" | "mercati";
export type TipoFilo =
  | "maggioranza" | "agenda" | "cantiere" | "proposta" | "minaccia" | "dimissioni" | "intesa"
  | "misura" | "legge" | "posto" | "sfiducia" | "evento" | "situazione" | "spread";

export type Nodo = { id: string; colonna: Colonna; tipo: TipoNodo; nome: Chiave | string; sotto: string; spento: boolean; tuo: boolean; alGoverno: boolean };
export type Filo = {
  id: string; da: string; a: string; tipo: TipoFilo; etichetta: string; frase: string;
  contrario: boolean; ritardo: boolean; cantiere: boolean; sezione: Sezione; fonte: string;
};

/** Dentro la colonna: partiti e Parlamento come vengono, Presidente prima dei ministeri, Mercati prima del Paese, situazioni in fondo. */
const ORDINE: Record<TipoNodo, number> = { partito: 0, camera: 0, presidente: 0, ministero: 1, mercati: 0, paese: 1, situazione: 2 };
const PDC = "pdc";

const minuscolo = (s: string) => s.toLowerCase();
const turni = (n: number) => t("schema.turni_mancanti", { n });
const quando = (e: VoceEffetto) => (e.ritardo > 0 ? t("schema.fra_turni", { n: e.ritardo }) : t("schema.subito"));
const verbo = (e: VoceEffetto) => t(e.migliora ? "schema.migliora" : "schema.peggiora");
const fraTrimestri = (voto: { anno: number; trimestre: number }, oggi: { anno: number; trimestre: number }) => (voto.anno - oggi.anno) * 4 + voto.trimestre - oggi.trimestre;
const causaMinistro = (c: string) => t(`governo.causa_${c}` as Chiave);

/** La guida non è un campo della Vista: fuori dal governo è il partito di governo senza fascia d'intesa, al turno zero quello con più ministri. */
function guidaDi(v: Vista): string | undefined {
  if (v.governa) return v.giocatore;
  const governo = v.partiti.filter((p) => p.al_governo).map((p) => p.id);
  if (v.alleati_avversari.length) return governo.find((id) => !v.alleati_avversari.some((a) => a.partito === id));
  const conta = new Map<string, number>();
  for (const m of v.ministeri) if (m.ministro) conta.set(m.ministro.partito, (conta.get(m.ministro.partito) ?? 0) + 1);
  return [...conta].sort((a, b) => b[1] - a[1])[0]?.[0];
}

export function schema(v: Vista): { nodi: Nodo[]; fili: Filo[] } {
  const sciolta = v.fase === "sciolta";
  const seggi = (id: string, r: "camera" | "senato") => v.seggi.find((s) => s.partito === id)?.[r] ?? 0;
  const totale = (r: "camera" | "senato") => v.seggi.reduce((a, s) => a + s[r], 0);
  const governo = v.partiti.filter((p) => p.al_governo).map((p) => p.id);
  const guida = guidaDi(v);
  const nomeGuida = guida ? nomeDi(v, guida) : "";
  const nomeGrandezza = (id: string) => v.grandezze.find((g) => g.id === id)?.nome ?? id.replaceAll("_", " ");

  const nodi = new Map<string, Nodo>();
  const aggiungi = (id: string, colonna: Colonna, tipo: TipoNodo, nome: Chiave | string, sotto: string, extra: Partial<Nodo> = {}) =>
    nodi.set(id, { id, colonna, tipo, nome, sotto, spento: false, tuo: false, alGoverno: false, ...extra });

  for (const p of v.partiti) {
    if (p.sciolto) continue;
    aggiungi(`p:${p.id}`, 0, "partito", p.nome,
      sciolta ? t("schema.sotto_campagna") : t("schema.sotto_partito", { camera: seggi(p.id, "camera"), senato: seggi(p.id, "senato") }),
      { tuo: p.id === v.giocatore, alGoverno: p.al_governo });
  }
  for (const r of ["camera", "senato"] as const) {
    aggiungi(r, 1, "camera", r === "camera" ? "schema.camera" : "schema.senato",
      sciolta ? t(r === "camera" ? "schema.sotto_sciolta" : "schema.sotto_sciolto") : t("schema.sotto_seggi", { n: totale(r) }), { spento: sciolta });
  }
  aggiungi(PDC, 2, "presidente", "schema.presidente",
    sciolta ? t("schema.sotto_uscente", { nome: nomeGuida }) : v.governa ? t("schema.sotto_tu") : t("schema.sotto_guida", { nome: nomeGuida }));
  for (const m of v.ministeri) {
    const mi = m.ministro;
    aggiungi(`d:${m.dicastero}`, 2, "ministero", v.dicasteri.find((d) => d.id === m.dicastero)?.nome ?? m.dicastero,
      mi ? t("schema.sotto_ministro", { cognome: mi.nome.split(" ").at(-1)!, partito: nomeDi(v, mi.partito) }) : t("schema.sotto_interim"));
  }
  aggiungi("mer", 3, "mercati", "schema.mercati", t("schema.sotto_spread", { n: v.spread_pb }));

  // i nodi del Paese nascono dal filo che li tocca; senza riga nella Vista il filo non si disegna
  const grandezza = (id: string): string | null => {
    if (!v.grandezze.some((g) => g.id === id)) return null;
    if (!nodi.has(`g:${id}`)) aggiungi(`g:${id}`, 3, "paese", nomeGrandezza(id), t("schema.sotto_grandezza"));
    return `g:${id}`;
  };
  const regione = (id: string | undefined): string | null => {
    const r = v.regioni.find((x) => x.id === id);
    if (!r) return null;
    if (!nodi.has(`r:${r.id}`)) aggiungi(`r:${r.id}`, 3, "paese", r.nome, t("schema.sotto_regione"));
    return `r:${r.id}`;
  };
  const situazione = (nome: string): string => {
    if (!nodi.has(`s:${nome}`)) aggiungi(`s:${nome}`, 3, "situazione", nome, t("schema.sotto_situazione"));
    return `s:${nome}`;
  };

  const fili: Filo[] = [];
  const filo = (f: Partial<Filo> & Pick<Filo, "id" | "da" | "a" | "tipo" | "etichetta" | "frase" | "sezione" | "fonte">) => {
    fili.push({ contrario: false, ritardo: false, cantiere: false, ...f });
  };
  const partito = (id: string) => `p:${id}`;

  // maggioranza
  if (!sciolta) {
    for (const r of ["camera", "senato"] as const) {
      const sg = governo.reduce((a, id) => a + seggi(id, r), 0);
      const margine = segnato(Math.round(sg - totale(r) / 2));
      filo({ id: `magg:${r}`, da: PDC, a: r, tipo: "maggioranza", etichetta: t("schema.et_maggioranza", { margine }), sezione: "aula", fonte: "seggi, partiti[].al_governo",
        frase: t("schema.frase_maggioranza", { seggi: sg, tot: totale(r), dove: t(r === "camera" ? "schema.alla_camera" : "schema.al_senato"), margine }),
        contrario: sg - totale(r) / 2 < 3 });
    }
  } else {
    filo({ id: "sciolte", da: PDC, a: "camera", tipo: "maggioranza", etichetta: t("schema.et_si_vota"), frase: t("schema.frase_si_vota"), sezione: "campagna", fonte: "fase" });
  }

  // ministeri
  const atto = (m: VoceMinistero, id: string) => m.atti.find((a) => a.id === id);
  for (const m of v.ministeri) {
    const d = `d:${m.dicastero}`, mi = m.ministro, a = m.agenda;
    if (a) {
      const e = atto(m, a.atto)?.effetti[0];
      const bersaglio = a.regione ? regione(a.regione) : e ? grandezza(e.grandezza) : null;
      if (bersaglio) {
        const effetti = (atto(m, a.atto)?.effetti ?? []).map((x) => t("schema.effetto", { verbo: verbo(x), grandezza: minuscolo(nomeGrandezza(x.grandezza)), quando: quando(x) }));
        const grandezzaE = e ? minuscolo(nomeGrandezza(e.grandezza)) : "";
        filo({ id: `ag:${m.dicastero}`, da: d, a: bersaglio, tipo: "agenda", sezione: "governo", fonte: "ministeri[].agenda, atti[].effetti",
          etichetta: a.regione ? `${e ? (e.migliora ? "▲ " : "▼ ") : ""}${grandezzaE}`.trim() || t("schema.et_agenda") : `${e!.migliora ? "▲" : "▼"} ${quando(e!)}`,
          frase: t("schema.frase_agenda", { nome: a.nome, regione: a.regione ? t("schema.in_regione", { regione: v.regioni.find((r) => r.id === a.regione)?.nome ?? a.regione }) : "",
            effetti: elenco(effetti, "; "), costo: euro(a.costo), stato: t(`schema.stato_${a.stato}` as Chiave) }),
          contrario: !!e && !e.migliora, ritardo: !!e && e.ritardo > 0 });
      }
    }
    m.cantieri.forEach((c, i) => {
      const at = atto(m, c.atto), e = at?.effetti[0];
      const bersaglio = c.regione ? regione(c.regione) : e ? grandezza(e.grandezza) : null;
      if (!bersaglio) return;
      filo({ id: `cant:${m.dicastero}:${i}`, da: d, a: bersaglio, tipo: "cantiere", cantiere: true, sezione: "governo", fonte: "ministeri[].cantieri",
        etichetta: t("schema.et_cantiere", { n: c.turni_mancanti }),
        frase: t("schema.frase_cantiere", { nome: at?.nome ?? c.atto, turni: turni(c.turni_mancanti),
          regione: c.regione ? t("schema.in_regione", { regione: v.regioni.find((r) => r.id === c.regione)?.nome ?? c.regione }) : "",
          poi: e ? t("schema.cantiere_poi", { verbo: verbo(e), grandezza: minuscolo(nomeGrandezza(e.grandezza)) }) : "" }) });
    });
    if (m.proposta) {
      const p = m.proposta, respinta = mi?.causa_principale === "proposta_respinta";
      const chi = mi ? mi.nome : t("schema.chi_ministero");
      // la proposta di riforma non chiede fondi: importo 0
      const fondi = p.importo > 0;
      filo({ id: `prop:${m.dicastero}`, da: d, a: PDC, tipo: "proposta", sezione: "cruscotto", fonte: "ministeri[].proposta", contrario: respinta,
        etichetta: fondi ? t("schema.et_proposta", { importo: euro(p.importo) }) : t("schema.et_riforma"),
        frase: t(fondi ? "schema.frase_proposta" : "schema.frase_riforma", { chi, importo: euro(p.importo), nome: p.nome, motivo: p.motivo, respinta: respinta ? t("schema.proposta_respinta") : "" }) });
    }
    if (mi?.minaccia) {
      filo({ id: `min:${m.dicastero}`, da: d, a: PDC, tipo: "minaccia", sezione: "governo", fonte: "ministeri[].ministro.minaccia", contrario: true,
        etichetta: t("schema.et_minaccia"),
        frase: t("schema.frase_minaccia", { nome: mi.nome, n: numero(mi.soddisfazione, 1), causa: mi.causa_principale ? `, ${causaMinistro(mi.causa_principale)}` : "" }) });
    }
    if (m.dimissioni_chieste) {
      filo({ id: `dim:${m.dicastero}`, da: PDC, a: d, tipo: "dimissioni", sezione: "governo", fonte: "ministeri[].dimissioni_chieste", contrario: true,
        etichetta: t("schema.et_dimissioni"), frase: t("schema.frase_dimissioni", { nome: mi?.nome ?? (nodi.get(d)!.nome as string) }) });
    }
  }

  // alleati: in cifre al governo, in fasce all'opposizione
  const causaIntesa = (c: CausaIntesa) => t(CAUSE[c]);
  for (const al of v.alleati) {
    const ultima = al.variazioni.at(-1);
    filo({ id: `al:${al.partito}`, da: partito(al.partito), a: PDC, tipo: "intesa", sezione: "partiti", fonte: "alleati",
      etichetta: t(al.ultimatum ? "schema.et_intesa_ultimatum" : "schema.et_intesa", { n: Math.round(al.intesa) }),
      frase: t("schema.frase_intesa", { partito: nomeDi(v, al.partito), n: numero(al.intesa, 1),
        ultimatum: al.ultimatum && al.causa_ultimatum ? t("schema.intesa_ultimatum", { causa: causaIntesa(al.causa_ultimatum) }) : "",
        pesa: al.causa_principale ? t("schema.intesa_pesa", { causa: causaIntesa(al.causa_principale) }) : "",
        ultimo: al.variazioni.length ? t("schema.intesa_ultimo", { elenco: elenco(al.variazioni.map((x) => t("schema.intesa_variazione", { causa: causaIntesa(x.causa), delta: segnato(+x.delta.toFixed(1), 1) })), ", ") }) : "" }),
      contrario: al.ultimatum || (!!ultima && ultima.delta < 0) });
  }
  for (const al of v.alleati_avversari) {
    filo({ id: `al:${al.partito}`, da: partito(al.partito), a: PDC, tipo: "intesa", sezione: "cruscotto", fonte: "alleati_avversari",
      etichetta: t(`schema.fascia_${al.fascia}` as Chiave),
      frase: t("schema.frase_fascia", { partito: nomeDi(v, al.partito), guida: nomeGuida, fascia: t(`schema.fascia_${al.fascia}` as Chiave),
        causa: al.causa_ultimatum ? t("schema.fascia_causa", { causa: causaIntesa(al.causa_ultimatum) }) : "" }),
      contrario: al.fascia === "ultimatum" });
  }

  // aula
  for (const c of v.calendario) {
    const da = c.promotore ? partito(c.promotore) : c.proponente ? `d:${c.proponente}` : c.forma === "bilancio" ? "d:economia_finanze" : PDC;
    const fra = fraTrimestri(c.voto, v.data), f = c.forbice;
    const origine = c.promotore ? t("schema.origine_posto", { partito: nomeDi(v, c.promotore) }) : c.proponente ? t("schema.origine_dicastero", { dicastero: nodi.get(`d:${c.proponente}`)?.nome as string ?? c.proponente }) : "";
    filo({ id: `cal:${c.id}`, da, a: "camera", tipo: "misura", sezione: "aula", fonte: "calendario",
      etichetta: fra <= 0 ? t("schema.et_al_voto") : t("schema.et_in_aula", { periodo: t(`schema.tr_${c.voto.trimestre}` as Chiave) }),
      frase: t("schema.frase_misura", { nome: c.nome, origine, quando: fra <= 0 ? t("schema.quando_fine_turno") : t("schema.quando_in", { periodo: periodoDi(v, c.voto) }),
        c0: f.camera[0], c1: f.camera[1], cm: f.maggioranza_camera, s0: f.senato[0], s1: f.senato[1], sm: f.maggioranza_senato, bilico: f.in_bilico ? t("schema.in_bilico") : "" }),
      contrario: f.in_bilico, ritardo: fra > 0 });
  }
  // gli esiti del trimestre appena chiuso; una misura decaduta non ha avuto un voto
  const prima = v.data.trimestre === 1 ? { anno: v.data.anno - 1, trimestre: 4 } : { anno: v.data.anno, trimestre: v.data.trimestre - 1 };
  for (const e of v.esiti.filter((x) => x.data.anno === prima.anno && x.data.trimestre === prima.trimestre && x.decaduta === null)) {
    filo({ id: `es:${e.misura}`, da: "camera", a: "senato", tipo: "legge", sezione: "aula", fonte: "esiti (trimestre chiuso)", contrario: !e.approvata,
      etichetta: t(e.approvata ? "schema.et_approvata" : "schema.et_respinta"),
      frase: t(e.approvata ? "schema.frase_approvata" : "schema.frase_respinta", { nome: e.nome, cs: e.camera.si, cn: e.camera.no, ss: e.senato.si, sn: e.senato.no }) });
  }
  // il posto riservato: del giocatore solo se libero (altrimenti lo dice già la misura), altrui sempre
  if (v.posto_riservato_di && (v.posto_riservato_di !== v.giocatore || v.posto_riservato)) {
    const tuo = v.posto_riservato_di === v.giocatore;
    filo({ id: "posto", da: partito(v.posto_riservato_di), a: "camera", tipo: "posto", sezione: "aula", fonte: "posto_riservato_di", ritardo: tuo,
      etichetta: t(tuo ? "schema.et_posto_tuo" : "schema.et_posto"),
      frase: t(tuo ? "schema.frase_posto_tuo" : "schema.frase_posto", { partito: nomeDi(v, v.posto_riservato_di) }) });
  }
  if (v.sfiducia) {
    const { dicastero: bersaglio, partito: chi } = v.sfiducia;
    const ministro = bersaglio ? v.ministeri.find((m) => m.dicastero === bersaglio)?.ministro?.nome ?? (nodi.get(`d:${bersaglio}`)?.nome as string | undefined) ?? bersaglio : "";
    filo({ id: "sfiducia", da: partito(chi), a: bersaglio ? `d:${bersaglio}` : PDC, tipo: "sfiducia", sezione: "cruscotto", fonte: "sfiducia", contrario: true,
      etichetta: t("schema.et_sfiducia"),
      frase: bersaglio ? t("schema.frase_sfiducia_ministro", { partito: nomeDi(v, chi), nome: ministro }) : t("schema.frase_sfiducia_governo", { partito: nomeDi(v, chi) }) });
  }

  // paese
  for (const e of v.eventi) {
    const reg = v.regioni.find((r) => r.nome === e.regione)?.id;
    const da = e.regione ? regione(reg) : situazione(e.nome);
    const predefinita = e.opzioni.find((o) => o.default)?.nome ?? "";
    filo({ id: `ev:${e.nome}`, da: da ?? "", a: PDC, tipo: "evento", sezione: "cruscotto", fonte: "eventi", contrario: true,
      etichetta: e.costo ? euro(e.costo) : t("schema.et_si_decide"),
      frase: t("schema.frase_evento", { nome: e.nome, regione: e.regione ? t("schema.in_regione", { regione: e.regione }) : "", costo: e.costo ? t("schema.evento_costo", { costo: euro(e.costo) }) : "",
        causa: e.causa, chi: t(v.governa ? "schema.chi_tu" : "schema.chi_governo"), default: minuscolo(predefinita) }) });
  }
  for (const s of v.situazioni) {
    if (!s.accesa || v.eventi.some((e) => e.nome === s.nome)) continue;
    filo({ id: `sit:${s.nome}`, da: situazione(s.nome), a: PDC, tipo: "situazione", sezione: "conti", fonte: "situazioni", contrario: true,
      etichetta: t("schema.et_situazione"), frase: t("schema.frase_situazione", { nome: s.nome }) });
  }
  filo({ id: "spread", da: "mer", a: "d:economia_finanze", tipo: "spread", sezione: "conti", fonte: "spread_pb", contrario: v.spread_pb > 350,
    etichetta: t("schema.et_spread", { n: v.spread_pb }), frase: t("schema.frase_spread", { n: v.spread_pb }) });

  // un filo il cui capo non ha una riga non si disegna (punto da sorvegliare 5)
  return {
    nodi: [...nodi.values()].sort((a, b) => a.colonna - b.colonna || ORDINE[a.tipo] - ORDINE[b.tipo]),
    fili: fili.filter((f) => nodi.has(f.da) && nodi.has(f.a)),
  };
}
