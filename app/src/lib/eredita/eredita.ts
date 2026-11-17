// L'adattatore dell'eredità: una forma sola per la bozza (Vista) e per l'albo (VoceAlbo). Copia, non calcola.
import { periodo } from "$lib/edicola/edicola";
import { titoloFine } from "$lib/testi/pungolo";
import { percentuale, t } from "$lib/testi/t";
import type { ElezioneAlbo } from "$lib/tipi/ElezioneAlbo";
import type { IndicatoreAlbo } from "$lib/tipi/IndicatoreAlbo";
import type { PuntoSerie } from "$lib/tipi/PuntoSerie";
import type { Trimestre } from "$lib/tipi/Trimestre";
import type { Vista } from "$lib/tipi/Vista";
import type { VoceAlbo } from "$lib/tipi/VoceAlbo";

export type Edizione = {
  nome: string;
  scenario: string;
  inizio: Trimestre;
  fine: Trimestre;
  turni: number;
  esito: "senza_seggi" | "abbandono";
  anni_al_governo: number;
  riavvolgimenti: number;
  leggi: { nome: string; legge: string | null }[];
  indicatori: IndicatoreAlbo[];
  serie: PuntoSerie[];
  elezioni: ElezioneAlbo[];
  bozza: boolean;
};

export function daVista(v: Vista, serie: PuntoSerie[], nome: string, scenario: string, inizio: Trimestre): Edizione {
  const e = v.eredita;
  if (!e) throw new Error("la Vista non ha l'eredità: la partita non è finita");
  const etichette = new Map(v.disposizioni.map((d) => [d.nome, d.legge]));
  const nomi = new Map(v.partiti.map((p) => [p.id, p.nome]));
  return {
    nome, scenario, inizio, serie,
    fine: v.data,
    turni: v.turno,
    esito: e.fine,
    anni_al_governo: e.anni_al_governo,
    riavvolgimenti: e.riavvolgimenti,
    leggi: e.leggi_in_vigore.map((l) => ({ nome: l, legge: etichette.get(l) ?? null })),
    indicatori: e.indicatori,
    elezioni: v.elezioni.map((x) => {
      const mio = x.voti.find((p) => p.partito === v.giocatore);
      return {
        data: x.data,
        affluenza: x.affluenza,
        voti: x.voti.map((p): [string, number] => [nomi.get(p.partito) ?? p.partito, p.quota]),
        seggi_giocatore: [mio?.camera ?? 0, mio?.senato ?? 0],
      };
    }),
    bozza: e.bozza,
  };
}

export function daAlbo(a: VoceAlbo): Edizione {
  return {
    nome: a.nome, scenario: a.scenario, inizio: a.inizio, fine: a.fine, turni: a.turni,
    esito: a.esito === "Abbandono" ? "abbandono" : "senza_seggi",
    anni_al_governo: a.anni_al_governo,
    riavvolgimenti: a.riavvolgimenti,
    leggi: a.leggi_in_vigore.map((nome) => ({ nome, legge: null })),
    indicatori: a.indicatori,
    serie: a.serie,
    elezioni: a.elezioni,
    bozza: false,
  };
}

/** Senza seggi: ultimo voto e quota del giocatore (l'albo e la Vista tengono la più recente per prima); abbandono: dove ha lasciato. */
export function occhiello(e: Edizione): string {
  if (e.esito === "abbandono") return t("eredita.occhiello_abbandono", { fine: periodo(e.fine) });
  const ultima = e.elezioni[0];
  const quota = ultima?.voti.find(([n]) => n === e.nome)?.[1];
  return t("eredita.occhiello_senza_seggi", {
    voto: periodo(ultima?.data ?? e.fine),
    quota: quota === undefined ? "—" : percentuale(quota),
  });
}

/** Senza seggi lo scherzo scelto con l'hash; abbandono la testa seria col nome del partito. */
export function titolo(e: Edizione): string {
  return e.esito === "abbandono" ? t("eredita.titolo_abbandono", { nome: e.nome }) : titoloFine(e.nome, e.scenario, e.inizio);
}
