import { invoke } from "@tauri-apps/api/core";
import type { Anteprima } from "$lib/tipi/Anteprima";
import type { AnteprimaSfiducia } from "$lib/tipi/AnteprimaSfiducia";
import type { Bilancio } from "$lib/tipi/Bilancio";
import type { AnteprimaFondazione } from "$lib/tipi/AnteprimaFondazione";
import type { CatalogoScenario } from "$lib/tipi/CatalogoScenario";
import type { Comando } from "$lib/tipi/Comando";
import type { Condizione } from "$lib/tipi/Condizione";
import type { Decreto } from "$lib/tipi/Decreto";
import type { ElettoreCampione } from "$lib/tipi/ElettoreCampione";
import type { Eredita } from "$lib/tipi/Eredita";
import type { Fascia } from "$lib/tipi/Fascia";
import type { Fondazione } from "$lib/tipi/Fondazione";
import type { PuntoSerie } from "$lib/tipi/PuntoSerie";
import type { RiepilogoBilancio } from "$lib/tipi/RiepilogoBilancio";
import type { RiepilogoScenario } from "$lib/tipi/RiepilogoScenario";
import type { Spesa } from "$lib/tipi/Spesa";
import type { SuMisura } from "$lib/tipi/SuMisura";
import type { ValoreProposto } from "$lib/tipi/ValoreProposto";
import type { Vista } from "$lib/tipi/Vista";
import type { Voce } from "$lib/tipi/Voce";
import type { VoceAlbo } from "$lib/tipi/VoceAlbo";
import type { VoceAnteprimaCampagna } from "$lib/tipi/VoceAnteprimaCampagna";
import type { VoceEsito } from "$lib/tipi/VoceEsito";
import type { VoceMenu } from "$lib/tipi/VoceMenu";
import type { VocePartita } from "$lib/tipi/VocePartita";

export type ErroreGuscio = { specie: "rifiuto" | "guasto"; testo: string };

export function comeErrore(e: unknown): ErroreGuscio {
  if (typeof e === "object" && e !== null) {
    const { specie, testo } = e as Record<string, unknown>;
    if ((specie === "rifiuto" || specie === "guasto") && typeof testo === "string") {
      return { specie, testo };
    }
  }
  return { specie: "guasto", testo: String(e) };
}

/** Il testo del core senza «comando non ammesso: » in testa. */
export function testoRifiuto(e: ErroreGuscio): string {
  return e.testo.replace(/^comando non ammesso: /, "");
}

async function chiama<T>(comando: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(comando, args);
  } catch (e) {
    throw comeErrore(e);
  }
}

export const guscio = {
  chiudiTurno: () => chiama<Vista>("chiudi_turno"),
  partite: () => chiama<VocePartita[]>("partite"),
  fondaPartita: (scenario: SuMisura | null, fondazione: Fondazione) =>
    chiama<Vista>("fonda_partita", { scenario, fondazione }),
  riprendi: (id: string) => chiama<Vista>("riprendi", { id }),
  comanda: (comando: Comando) => chiama<Vista>("comanda", { comando }),
  riavvolgi: (turno: number) => chiama<Vista>("riavvolgi", { turno }),
  registro: (turno: number) => chiama<VoceEsito[]>("registro", { turno }),
  abbandona: () => chiama<Eredita>("abbandona"),
  confermaEredita: () => chiama<void>("conferma_eredita"),
  albo: () => chiama<VoceAlbo[]>("albo"),
  scenari: () => chiama<string[]>("scenari"),
  salvaScenario: (scenario: SuMisura) => chiama<void>("salva_scenario", { scenario }),
  caricaScenario: (nome: string) => chiama<SuMisura>("carica_scenario", { nome }),
  riepilogoScenario: (scenario: SuMisura | null) =>
    chiama<RiepilogoScenario>("riepilogo_scenario", { scenario }),
  anteprima: (valori: ValoreProposto[]) => chiama<Anteprima>("anteprima", { valori }),
  anteprimaDecreto: (decreto: Decreto) => chiama<Anteprima>("anteprima_decreto", { decreto }),
  anteprimaAccogli: (dicastero: string) => chiama<Anteprima>("anteprima_accogli", { dicastero }),
  anteprimaSfiducia: (dicastero: string | null) =>
    chiama<AnteprimaSfiducia>("anteprima_sfiducia", { dicastero }),
  costoDirotta: (dicastero: string, atto: string | null, regione: string | null) =>
    chiama<number>("costo_dirotta", { dicastero, atto, regione }),
  menuElettorale: (valori: ValoreProposto[]) => chiama<VoceMenu[]>("menu_elettorale", { valori }),
  anteprimaProposta: (valori: ValoreProposto[]) => chiama<Anteprima>("anteprima_proposta", { valori }),
  serie: () => chiama<PuntoSerie[]>("serie"),
  anteprimaFondazione: (scenario: SuMisura | null) =>
    chiama<AnteprimaFondazione>("anteprima_fondazione", { scenario }),
  catalogoScenario: () => chiama<CatalogoScenario>("catalogo_scenario"),
  anteprimaCampagna: (spese: Spesa[]) => chiama<VoceAnteprimaCampagna[]>("anteprima_campagna", { spese }),
  elettoriCampione: (regione: string, condizione: Condizione, fascia: Fascia) =>
    chiama<ElettoreCampione[]>("elettori_campione", { regione, condizione, fascia }),
  bozzaBilancio: () => chiama<Bilancio | null>("bozza_bilancio"),
  indicizza: (bilancio: Bilancio, voce: Voce) => chiama<Bilancio>("indicizza", { bilancio, voce }),
  riepilogoBilancio: (bilancio: Bilancio) => chiama<RiepilogoBilancio>("riepilogo_bilancio", { bilancio }),
  riepilogoPresentato: () => chiama<RiepilogoBilancio | null>("riepilogo_presentato"),
};
