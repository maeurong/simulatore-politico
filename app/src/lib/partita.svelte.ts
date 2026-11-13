// La partita in corso, condivisa fra cornice e sezioni.
import { goto } from "$app/navigation";
import { comeErrore, guscio } from "$lib/guscio/chiamate";
import { casa } from "$lib/ossatura";
import { t } from "$lib/testi/t";
import type { Fondazione } from "$lib/tipi/Fondazione";
import type { PuntoSerie } from "$lib/tipi/PuntoSerie";
import type { SuMisura } from "$lib/tipi/SuMisura";
import type { Vista } from "$lib/tipi/Vista";

class Partita {
  // raw: la Vista si sostituisce intera, mai si modifica sul posto
  vista = $state.raw<Vista | null>(null);
  serie = $state.raw<PuntoSerie[]>([]);
  guasto = $state<string | null>(null);
  occupato = $state(false);
  // esci() lo fa avanzare: una risposta arrivata dopo l'uscita appartiene alla partita lasciata e si scarta
  #giro = 0;

  /**
   * La Vista nuova, o null davanti a un guasto del comando (la partita resta com'era). Riuscito il comando la Vista
   * nuova vale comunque: se poi la serie si guasta resta quella di prima e il guasto lo dice. Il rifiuto si rilancia.
   * Se nel frattempo si è usciti all'edicola torna null e non tocca niente.
   */
  async #esegui(chiamata: () => Promise<Vista>): Promise<Vista | null> {
    if (this.occupato) return null;
    this.occupato = true;
    const giro = this.#giro;
    const scartato = () => giro !== this.#giro;
    try {
      let vista: Vista;
      try {
        vista = await chiamata();
      } catch (e) {
        if (scartato()) return null;
        const errore = comeErrore(e);
        if (errore.specie === "rifiuto") throw errore;
        this.guasto = `${errore.testo} ${t("cornice.rimasta_com_era")}`;
        return null;
      }
      if (scartato()) return null;
      this.vista = vista;
      this.guasto = null;
      try {
        const serie = await guscio.serie();
        if (scartato()) return null;
        this.serie = serie;
      } catch (e) {
        if (scartato()) return null;
        this.guasto = `${comeErrore(e).testo} ${t("cornice.grafici_non_aggiornati")}`;
      }
      return vista;
    } finally {
      this.occupato = false;
    }
  }

  fonda = (scenario: SuMisura | null, fondazione: Fondazione) => this.#esegui(() => guscio.fondaPartita(scenario, fondazione));
  riprendi = (id: string) => this.#esegui(() => guscio.riprendi(id));

  /** Fino a 11f il turno nuovo si apre sulla sua casa (spec:290). keepFocus: il pulsante resta nella cornice, e il fuoco con lui. */
  chiudiTurno = async () => {
    const vista = await this.#esegui(guscio.chiudiTurno);
    if (vista) await goto(`/partita/${casa(vista)}`, { keepFocus: true });
  };

  /** Verso l'edicola: il guscio non ha niente da fare. */
  esci = () => {
    this.#giro++;
    this.vista = null;
    this.serie = [];
    this.guasto = null;
  };
}

export const partita = new Partita();
