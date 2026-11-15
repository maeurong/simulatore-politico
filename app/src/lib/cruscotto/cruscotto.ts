// Cruscotto del turno zero, ordine del giorno al governo e riquadro dell'evento: le sole cifre che la UI calcola sono conteggi di voci della Vista.
import { nomeDi } from "$lib/partiti";
import { CAUSE } from "$lib/partiti/schieramento";
import { statoDi, type Sezione } from "$lib/ossatura";
import { t, numero, type Chiave } from "$lib/testi/t";
import { votaOra } from "$lib/aula/aula";
import type { Vista } from "$lib/tipi/Vista";
import type { VoceEvento } from "$lib/tipi/VoceEvento";
import type { VoceAgenda } from "$lib/tipi/VoceAgenda";
import type { VoceMinistro } from "$lib/tipi/VoceMinistro";
import type { VoceOpzione } from "$lib/tipi/VoceOpzione";

/** Una sola opzione: niente da scegliere, la chiusura la prende. */
export const opzioneUnica = (e: VoceEvento): boolean => e.opzioni.length === 1;

/** L'opzione col campo default. */
export const defaultDi = (e: VoceEvento): VoceOpzione | null => e.opzioni.find((o) => o.default) ?? null;

export type Gruppo = "da_decidere" | "proposte" | "emendamenti" | "si_vota" | "occhio";
export type VoceOdg = {
  chiave: string; gruppo: Gruppo; titolo: string; sotto: string;
  /** il default scritto sulla riga; vuoto dove non c'è una scelta da lasciare */
  default: string;
  /** conta fra le scelte al default (lettura 1) */
  alDefault: boolean;
  urgente: boolean; fatta: boolean;
  azioni: "proposta" | "emendamento" | "successore" | "fiducia" | { rimando: Sezione };
  dicastero?: string; misura?: number; partito?: string;
};

/** Le voci per gruppo, nell'ordine dei gruppi (quello in cui sono spinte) e, dentro, della Vista. Solo chi governa ha un ordine del giorno. */
export function ordineDelGiorno(v: Vista): VoceOdg[] {
  if (!v.governa) return [];
  const voci: VoceOdg[] = [];
  const dicastero = (id: string) => v.dicasteri.find((d) => d.id === id)?.nome ?? id;
  const prov = v.bilancio.esercizio_provvisorio;
  if (v.bilancio.presentabile && !v.bilancio.presentato && !v.crisi) {
    voci.push({
      chiave: "bilancio", gruppo: "da_decidere", titolo: t("cruscotto.presenta_bilancio"),
      sotto: t(prov ? "cruscotto.bilancio_sotto_provvisorio" : "cruscotto.bilancio_sotto"),
      default: t(prov ? "cruscotto.bilancio_default_provvisorio" : "cruscotto.bilancio_default"),
      alDefault: true, urgente: prov, fatta: false, azioni: { rimando: "bilancio" },
    });
  }
  for (const m of v.ministeri) {
    if (m.ministro !== null || !m.rosa.length) continue;
    const causa = m.causa_dimissioni ? ` · ${t(`governo.causa_${m.causa_dimissioni}` as Chiave)}` : "";
    voci.push({
      chiave: `suc:${m.dicastero}`, gruppo: "da_decidere", titolo: t("cruscotto.successore", { dicastero: dicastero(m.dicastero) }),
      sotto: `${t("governo.stato_interim")}${causa}`,
      default: m.scelta === null ? t("cruscotto.successore_default", { nome: m.rosa[0].nome }) : "",
      alDefault: m.scelta === null, urgente: false, fatta: m.scelta !== null, azioni: "successore", dicastero: m.dicastero,
    });
  }
  for (const m of v.ministeri) {
    const p = m.proposta;
    if (!p) continue;
    const euro = p.importo ? ` · ${numero(p.importo)}\u00A0€` : "";
    voci.push({
      chiave: `pro:${m.dicastero}`, gruppo: "proposte", titolo: p.nome,
      sotto: `${dicastero(m.dicastero)} · ${m.ministro?.nome ?? t("governo.senza_ministro")}${euro}`,
      default: t(p.rinviata ? "cruscotto.proposta_default_rinviata" : "cruscotto.proposta_default"),
      alDefault: !p.rinviata, urgente: false, fatta: p.rinviata, azioni: "proposta", dicastero: m.dicastero,
    });
  }
  for (const m of v.calendario) {
    for (const e of m.emendamenti.filter((x) => x.stato === "aperto")) {
      voci.push({
        chiave: `em:${m.id}:${e.partito}`, gruppo: "emendamenti", titolo: t("cruscotto.emendamento_su", { partito: nomeDi(v, e.partito), misura: m.nome }),
        sotto: e.testo, default: t("cruscotto.emendamento_default"), alDefault: true, urgente: false, fatta: false,
        azioni: "emendamento", misura: m.id, partito: e.partito,
      });
    }
  }
  for (const m of v.calendario.filter((x) => votaOra(v, x))) {
    const f = m.forbice;
    voci.push({
      chiave: `voto:${m.id}`, gruppo: "si_vota", titolo: m.nome,
      sotto: t("cruscotto.forbice", { c0: f.camera[0], c1: f.camera[1], mc: f.maggioranza_camera, s0: f.senato[0], s1: f.senato[1], ms: f.maggioranza_senato }),
      default: "", alDefault: false, urgente: f.in_bilico, fatta: m.fiducia, azioni: "fiducia", misura: m.id,
    });
  }
  for (const a of v.alleati.filter((x) => x.ultimatum)) {
    const causa = a.causa_ultimatum ? `. ${t("partiti.s_ultimatum_per", { causa: t(CAUSE[a.causa_ultimatum]) })}` : "";
    voci.push({
      chiave: `ult:${a.partito}`, gruppo: "occhio", titolo: t("cruscotto.ultimatum", { partito: nomeDi(v, a.partito) }),
      sotto: `${t("cruscotto.ultimatum_sotto", { n: numero(a.intesa), soglia: numero(v.soglie.uscita) })}${causa}`,
      default: "", alDefault: false, urgente: true, fatta: false, azioni: { rimando: "partiti" }, partito: a.partito,
    });
  }
  return voci;
}

/** Gli eventi senza scelta più le voci al default dell'ordine del giorno, più la crisi fuori dall'esercizio provvisorio (lettura 1). */
export const scelteAlDefault = (v: Vista): number =>
  v.eventi.filter((e) => !opzioneUnica(e) && e.scelta === null).length +
  ordineDelGiorno(v).filter((x) => x.alDefault).length +
  (statoDi(v) === "crisi" && !v.bilancio.esercizio_provvisorio ? 1 : 0);

/** Il fondo di riserva rimasto basta a coprire il costo dell'evento. */
export const fondoBasta = (v: Vista, e: VoceEvento): boolean => v.bilancio.fondo_residuo >= e.costo;

/** Chi può rispondere: il giocatore che governa, fuori dalla trattativa. */
export function risponde(v: Vista): boolean {
  const s = statoDi(v);
  return s === "governo" || s === "crisi";
}

/** Le due righe del turno del voto: lo stato della coalizione e della campagna, in parole. */
export function righeDelVoto(v: Vista) {
  return {
    coalizione: {
      dichiarati: v.coalizione.map((id) => nomeDi(v, id)),
      disponibili: v.disponibilita.filter((d) => d.disponibile).map((d) => nomeDi(v, d.partito)),
    },
    campagna: { spese: v.campagna.spese.length, euro: v.campagna.spese.reduce((a, s) => a + s.euro, 0) },
  };
}

/** I ministri in evidenza (agenda in evidenza o ministro in preavviso), nell'ordine della Vista; solo chi governa. */
export const inEvidenza = (v: Vista): { dicastero: string; ministro: VoceMinistro; agenda: VoceAgenda | null; preavviso: boolean }[] =>
  v.governa
    ? v.ministeri.flatMap((m) =>
        m.ministro && (m.agenda?.in_evidenza || m.ministro.minaccia) ? [{ dicastero: m.dicastero, ministro: m.ministro, agenda: m.agenda, preavviso: m.ministro.minaccia }] : [])
    : [];

/** Il riquadro di testa: crisi da allargare, crisi in esercizio provvisorio (caduta certa), dimissioni annunciate, o niente. */
export function testa(v: Vista): "crisi" | "crisi_provvisorio" | "dimissioni" | null {
  if (statoDi(v) === "crisi") return v.bilancio.esercizio_provvisorio ? "crisi_provvisorio" : "crisi";
  return v.governa && v.dimissioni ? "dimissioni" : null;
}
