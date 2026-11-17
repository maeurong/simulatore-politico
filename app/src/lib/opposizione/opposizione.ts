// Il cruscotto all'opposizione: le leve del giocatore, le misure e il programma del governo avversario, la sua maggioranza (storie 85-92).
import type { Chiave } from "$lib/testi/t";
import type { AnteprimaSfiducia } from "$lib/tipi/AnteprimaSfiducia";
import type { FasciaIntesa } from "$lib/tipi/FasciaIntesa";
import type { Vista } from "$lib/tipi/Vista";
import type { VoceMisura } from "$lib/tipi/VoceMisura";
import type { VoceProgramma } from "$lib/tipi/VoceProgramma";

export type Leva = {
  tipo: "sfiducia_governo" | "sfiducia_ministro" | "posto" | "abrogativo" | "confermativo";
  stato: "libera" | "presentata" | "altrui" | "spenta";
  motivo: Chiave | null;
  chi: string | null;
};

const leva = (tipo: Leva["tipo"], stato: Leva["stato"], chi: string | null = null, motivo: Chiave | null = null): Leva => ({ tipo, stato, motivo, chi });

/** Una sfiducia a turno, al governo o a un ministro: chi la presenta occupa tutte e due le leve. */
function sfiducia(v: Vista, tipo: "sfiducia_governo" | "sfiducia_ministro"): Leva {
  const s = v.sfiducia;
  if (!s) return leva(tipo, "libera");
  if ((s.dicastero === null) !== (tipo === "sfiducia_governo")) return leva(tipo, "spenta", null, "opposizione.sfiducia_gia_presentata");
  return s.partito === v.giocatore ? leva(tipo, "presentata") : leva(tipo, "altrui", s.partito);
}

function posto(v: Vista): Leva {
  if (v.posto_riservato) return leva("posto", "libera");
  if (v.calendario.some((m) => m.promotore === v.giocatore)) return leva("posto", "presentata");
  return v.posto_riservato_di ? leva("posto", "altrui", v.posto_riservato_di) : leva("posto", "spenta");
}

export function leve(v: Vista): Leva[] {
  const conf = v.referendum.find((r) => r.tipo === "confermativo" && (r.richiesto || r.scelta));
  return [
    sfiducia(v, "sfiducia_governo"),
    sfiducia(v, "sfiducia_ministro"),
    posto(v),
    leva("abrogativo", v.referendum.some((r) => r.tipo === "abrogativo" && r.promotore === v.giocatore) ? "presentata" : "libera"),
    ...(conf ? [leva("confermativo", conf.scelta ? "libera" : "presentata")] : []),
  ];
}

/** Il calendario senza le proposte del giocatore, nell'ordine del voto della Vista. */
export const misureDelGoverno = (v: Vista): VoceMisura[] => v.calendario.filter((m) => m.promotore !== v.giocatore);

export const programma = (v: Vista): VoceProgramma[] => v.programma;

/** I partiti al governo (Camera, Senato) con la fascia d'intesa; chi guida non ne ha. */
export function maggioranzaAvversaria(v: Vista): { partito: string; seggi: [number, number]; fascia: FasciaIntesa | null }[] {
  return v.partiti.filter((p) => p.al_governo).map(({ id }) => {
    const s = v.seggi.find((x) => x.partito === id);
    return { partito: id, seggi: [s?.camera ?? 0, s?.senato ?? 0], fascia: v.alleati_avversari.find((a) => a.partito === id)?.fascia ?? null };
  });
}

/** I sì che mancano alla Camera per la sfiducia: metà più uno meno i seggi di chi la voterebbe. */
export function siMancanti(v: Vista, a: AnteprimaSfiducia): number {
  if (a.passa) return 0;
  const totale = v.seggi.reduce((s, x) => s + x.camera, 0);
  const si = a.linee.filter((l) => l.si).reduce((s, l) => s + (v.seggi.find((x) => x.partito === l.partito)?.camera ?? 0), 0);
  return Math.max(0, Math.floor(totale / 2) + 1 - si);
}
