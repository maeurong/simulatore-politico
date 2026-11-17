// La prima pagina letta dall'esito: la UI ordina, conta e sceglie un testo, non decide le sette (spec:250, piano 11f).
import type { Vista } from "$lib/tipi/Vista";
import type { VoceEsito } from "$lib/tipi/VoceEsito";
import type { TipoVoce } from "$lib/tipi/TipoVoce";
import type { Sezione } from "$lib/ossatura";

const RANGO = { critica: 2, notevole: 1, ordinaria: 0 } as const;

/** Gli indici delle voci in evidenza, nell'ordine della chiusura. */
export const ordineRivelazione = (es: VoceEsito[]): number[] => es.flatMap((v, i) => (v.in_evidenza ? [i] : []));

/** La voce di testa fra quelle in evidenza: criticità, poi precedenza più alta, a parità la prima (spec:406). */
export function testa(es: VoceEsito[]): number | null {
  let t: number | null = null;
  for (const i of ordineRivelazione(es)) {
    const [a, b] = [es[i], t === null ? null : es[t]];
    if (!b || RANGO[a.criticita] > RANGO[b.criticita] || (a.criticita === b.criticita && a.precedenza > b.precedenza)) t = i;
  }
  return t;
}

/** Le colonne sono nell'ordine della chiusura, senza la testa; `fuori` le voci non in evidenza, `critFuori` le critiche fra loro. */
export function pagina(es: VoceEsito[]) {
  const t = testa(es);
  const fuori = es.filter((v) => !v.in_evidenza);
  return {
    testa: t,
    colonne: ordineRivelazione(es).filter((i) => i !== t),
    fuori: fuori.length,
    critFuori: fuori.filter((v) => v.criticita === "critica").length,
  };
}

/** Tono grave della pagina (spec:413): una voce grave nell'esito, o un evento grave ancora aperto. */
export const grave = (v: Vista): boolean => v.esito.some((x) => x.tono === "grave") || v.eventi.some((e) => e.tono === "grave");

/**
 * La sezione che decide o legge il soggetto della voce (piano 11f, lettura 7). Base: la tabella RIGA del prototipo 25,
 * con il rimando di evento_risolto e emergenza_lieve ai Conti; gli altri tipi vanno dove si decide il loro soggetto.
 */
export const SEZIONE_DI: Record<TipoVoce, Sezione> = {
  evento_nato: "cruscotto", incarico: "cruscotto", crisi_aperta: "cruscotto", governo_caduto: "cruscotto",
  governo_dimissionario: "cruscotto", camere_sciolte: "cruscotto",
  evento_risolto: "conti", emergenza_lieve: "conti", spread: "conti", prima_stima: "conti", revisione: "conti",
  situazione_accesa: "conti", situazione_spenta: "conti", crisi_del_debito: "conti", shock_assorbito: "conti",
  mes_avviato: "conti", mes_finito: "conti", ristrutturazione: "conti", ristrutturazione_finita: "conti",
  margine_energia: "conti", controllo_ue: "conti", procedura_ue: "conti", ammenda_ue: "conti", valore_non_applicato: "conti",
  proposta_respinta: "governo", proposta_decaduta: "governo", proposta_accolta: "governo", atto_eseguito: "governo",
  atto_saltato: "governo", cantiere_avviato: "governo", cantiere_fermo: "governo", cantiere_finito: "governo",
  soddisfazione_variata: "governo", minaccia_di_dimissioni: "governo", ministro_esce: "governo", governo_formato: "governo",
  rimpasto: "governo",
  emendamento_respinto: "aula", emendamento_chiesto: "aula", emendamento_deciso: "aula", leggi_slittate: "aula",
  legge_approvata: "aula", legge_respinta: "aula", decreto_convertito: "aula", decreto_decaduto: "aula",
  bilancio_approvato: "aula", bilancio_respinto: "aula", fiducia_posta: "aula", fiducia_respinta: "aula",
  sfiducia_presentata: "aula", sfiducia_approvata: "aula", sfiducia_respinta: "aula", sfiducia_scartata: "aula",
  misura_presentata: "aula", misura_decaduta: "aula", valore_in_vigore: "aula", valore_rinviato: "aula",
  arrivo_scartato: "aula", programma_dichiarato: "aula", confermativo_in_attesa: "aula", confermativo_senza_voto: "aula",
  referendum_chiesto: "aula", referendum_votato: "aula", referendum_decaduto: "aula", firme_raccolte: "aula",
  bilancio_ricondotto: "bilancio",
  sondaggio: "paese", reddito_deluso: "paese",
  campagna_tagliata: "campagna",
  entrate_di_cassa: "partiti", intesa_variata: "partiti", ultimatum_posto: "partiti", ultimatum_ritirato: "partiti",
  promessa_fatta: "partiti", promessa_verificata: "partiti", promesse_scadute: "partiti", rottura_dell_impegno: "partiti",
  spesa_di_campagna: "partiti", risultato_del_voto: "partiti", fine_della_partita: "partiti", partito_sciolto: "partiti",
  scissione: "partiti", minaccia_di_scissione: "partiti", minaccia_rientrata: "partiti", cambio_di_leader: "partiti",
  frangia_esce: "partiti", alleato_esce: "partiti", coalizione_allargata: "partiti", guida_rifiutata: "partiti",
  partito_di_protesta: "partiti", coalizione_ia: "partiti", posizione_spostata: "partiti",
};

export type TracciatoMosso = "consenso" | "margine" | "spread";

/** Quale tracciato muove ciascun tipo (spec:407, lettura 6); risultato_del_voto ne muove due. */
export const TRACCIATO_DI: Record<TracciatoMosso, TipoVoce[]> = {
  consenso: ["sondaggio", "risultato_del_voto"],
  margine: ["risultato_del_voto", "governo_formato", "alleato_esce", "frangia_esce", "coalizione_allargata", "scissione", "partito_sciolto", "governo_caduto", "crisi_aperta"],
  spread: ["spread"],
};

/** L'indice della prima voce che muove il tracciato, in evidenza o no; null: nessuna, e il tracciato scatta alla fine. */
export function scattaDopo(es: VoceEsito[], tr: TracciatoMosso): number | null {
  const i = es.findIndex((v) => TRACCIATO_DI[tr].includes(v.tipo));
  return i < 0 ? null : i;
}
