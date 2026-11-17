// La partita in corso, condivisa fra cornice e sezioni.
import { goto } from "$app/navigation";
import { comeErrore, guscio } from "$lib/guscio/chiamate";
import { ordineRivelazione } from "$lib/esito/esito";
import { rivelazione } from "$lib/esito/rivelazione.svelte";
import { registroCache } from "$lib/registro/cache.svelte";
import { fattori, impostazioni, sistema } from "$lib/impostazioni.svelte";
import { bozza } from "$lib/bilancio/bozza.svelte";
import { t } from "$lib/testi/t";
import type { Comando } from "$lib/tipi/Comando";
import type { Fondazione } from "$lib/tipi/Fondazione";
import type { IndicePartita } from "$lib/tipi/IndicePartita";
import type { PuntoSerie } from "$lib/tipi/PuntoSerie";
import type { SuMisura } from "$lib/tipi/SuMisura";
import type { Vista } from "$lib/tipi/Vista";

// come `prima_di_sostituire`: l'effect del Registro che vede il turno nuovo deve ricaricare su una cache già vuota, non essere azzerato dopo
const svuotaRegistro = () => registroCache.svuota();

class Partita {
  // raw: la Vista si sostituisce intera, mai si modifica sul posto
  vista = $state.raw<Vista | null>(null);
  serie = $state.raw<PuntoSerie[]>([]);
  guasto = $state<string | null>(null);
  occupato = $state(false);
  // esci() lo fa avanzare: una risposta arrivata dopo l'uscita appartiene alla partita lasciata e si scarta
  #giro = 0;
  // l'id lo sa solo chi ha ripreso la partita; chi l'ha fondata la ritrova da indice()
  #id: string | null = null;
  // copre anche la finestra fra il guasto di guscio.riavvolgi e il riallineamento, in cui `occupato` è spento
  #riavvolgendo = false;

  /**
   * La Vista nuova, o null davanti a un guasto del comando (la partita resta com'era). Riuscito il comando la Vista
   * nuova vale comunque: se poi la serie si guasta resta quella di prima e il guasto lo dice. Il rifiuto si rilancia.
   * Se nel frattempo si è usciti all'edicola torna null e non tocca niente.
   */
  async #esegui(chiamata: () => Promise<Vista>, prima_di_sostituire?: (nuova: Vista) => void): Promise<Vista | null> {
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
      // prima di sostituire la Vista, nello stesso giro: la pagina che la mostra non si compone, per un istante, già rivelata
      prima_di_sostituire?.(vista);
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

  fonda = async (scenario: SuMisura | null, fondazione: Fondazione) => {
    const vista = await this.#esegui(() => guscio.fondaPartita(scenario, fondazione), svuotaRegistro);
    if (vista) {
      this.#id = null;
      rivelazione.chiudi();
    }
    return vista;
  };
  riprendi = (id: string) => this.#riprendi(id, svuotaRegistro);
  // il registro lo svuota `prima_di_sostituire`; il riallineamento di riavvolgi lo svuota solo se il turno è cambiato. Allo stesso turno
  // i turni letti valgono ancora e il Registro resta montato col rifiuto; a un turno diverso (cancellazioni interrotte a metà) si svuota,
  // e il Registro ricarica da sé su vista.turno
  async #riprendi(id: string, prima_di_sostituire: (nuova: Vista) => void) {
    const vista = await this.#esegui(() => guscio.riprendi(id), prima_di_sostituire);
    if (vista) {
      this.#id = id;
      rivelazione.chiudi();
    }
    return vista;
  }
  /** Come fonda e riprendi: Vista nuova, o null davanti a un guasto; il rifiuto si rilancia al chiamante. */
  comanda = (comando: Comando) => this.#esegui(() => guscio.comanda(comando));

  /**
   * Il turno nuovo si apre sulla prima pagina, anche se la partita è finita (lettura 2); la rivelazione parte dalla Vista e dalla serie
   * di prima, e a velocità istantanea non parte. keepFocus: il pulsante resta nella cornice, e il fuoco con lui.
   */
  chiudiTurno = async () => {
    const [prima, serie] = [this.vista, this.serie];
    const k = fattori(impostazioni, sistema.riduce).k;
    const vista = await this.#esegui(guscio.chiudiTurno, (nuova) => {
      // a velocità istantanea la pagina è già composta (spec:407): una Vista di prima di una chiusura precedente non deve restare
      if (prima && k > 0) rivelazione.inizia(prima, serie, ordineRivelazione(nuova.esito).length);
      else rivelazione.chiudi();
    });
    if (vista) await this.#apriPrimaPagina();
  };

  async #apriPrimaPagina() {
    try {
      await goto("/partita/prima-pagina", { keepFocus: true });
    } catch (e) {
      rivelazione.chiudi();
      throw e;
    }
    // goto annullato: PrimaPagina non si monta e nessuno chiuderebbe la rivelazione
    if (typeof location !== "undefined" && !location.pathname.endsWith("/prima-pagina")) rivelazione.chiudi();
  }

  /**
   * Chiude il turno finché la Vista nuova non ha una fermata; la serie si chiede una volta, alla fine. Un guasto dopo la prima chiusura
   * ferma la corsa: vale la Vista dell'ultima chiusura riuscita, e il guasto lo dice.
   */
  avanti = async () => {
    const [prima, serie] = [this.vista, this.serie];
    if (!prima) return;
    const k = fattori(impostazioni, sistema.riduce).k;
    let precedente = prima;
    let chiusure = 0;
    let guastoDellaCorsa: string | null = null;
    const giro = this.#giro;
    const vista = await this.#esegui(
      async () => {
        let v = await guscio.chiudiTurno();
        chiusure = 1;
        while (!v.fermate.length && !v.finita && giro === this.#giro) {
          try {
            const n = await guscio.chiudiTurno();
            [precedente, v] = [v, n];
            chiusure++;
          } catch (e) {
            guastoDellaCorsa = comeErrore(e).testo;
            break;
          }
        }
        return v;
      },
      (nuova) => {
        if (k > 0) rivelazione.inizia(precedente, serie, ordineRivelazione(nuova.esito).length);
        else rivelazione.chiudi();
        rivelazione.corsa = { chiusure, da: prima.turno, fermate: nuova.fermate };
      },
    );
    if (!vista) return;
    // `tracciatiDi(prima)` cerca il punto del turno prima della sua Vista: lo ha la serie di fine corsa, non quella d'inizio
    if (k > 0) rivelazione.serie_prima = this.serie;
    if (guastoDellaCorsa) this.guasto = `${guastoDellaCorsa} ${t("cornice.corsa_interrotta", { n: chiusure })}`;
    await this.#apriPrimaPagina();
  };

  /** L'indice della partita in corso: per id se è stata ripresa, altrimenti la più recente col nome del giocatore. */
  async indice(): Promise<IndicePartita | null> {
    const v = this.vista;
    if (!v) return null;
    const nome = v.partiti.find((p) => p.id === v.giocatore)?.nome;
    const mie = (await guscio.partite()).map((x) => x.indice).filter((i) => (this.#id ? i.id === this.#id : i.testata.nome === nome));
    return mie.sort((a, b) => b.ultimo_salvataggio - a.ultimo_salvataggio)[0] ?? null;
  }

  /**
   * Un riavvolgimento che non riesce lascia la memoria del guscio com'è sul disco: la Vista di qui si riallinea con
   * riprendi, e il guasto resta scritto. Il rifiuto si rilancia dopo il riallineamento.
   */
  riavvolgi = async (turno: number): Promise<Vista | null> => {
    if (this.occupato || this.#riavvolgendo) return null;
    this.#riavvolgendo = true;
    try {
      const giro = this.#giro;
      const turnoPrima = this.vista?.turno;
      let rifiuto: unknown = null;
      let vista: Vista | null = null;
      try {
        vista = await this.#esegui(() => guscio.riavvolgi(turno), svuotaRegistro);
      } catch (e) {
        rifiuto = e;
      }
      if (vista) rivelazione.chiudi();
      if (vista || giro !== this.#giro) return vista;
      const guasto = this.guasto;
      const id = (await this.indice().catch(() => null))?.id;
      if (id) {
        await this.#riprendi(id, (nuova) => {
          if (nuova.turno !== turnoPrima) registroCache.svuota();
        }).catch(() => null);
        // il guasto di riavvolgi vale più di quello (o del silenzio) del riallineamento, salvo che nel frattempo si sia usciti
        if (guasto && giro === this.#giro) this.guasto = guasto;
      }
      if (rifiuto) throw rifiuto;
      return null;
    } finally {
      this.#riavvolgendo = false;
    }
  };

  /** Vero a conferma scritta: la partita esce dalle in corso e l'albo ha la voce. Il guasto la lascia in bozza; il rifiuto si rilancia. */
  confermaEredita = async (): Promise<boolean> => {
    if (this.occupato) return false;
    this.occupato = true;
    try {
      await guscio.confermaEredita();
      this.guasto = null;
      return true;
    } catch (e) {
      const errore = comeErrore(e);
      if (errore.specie === "rifiuto") throw errore;
      this.guasto = `${errore.testo} ${t("cornice.rimasta_com_era")}`;
      return false;
    } finally {
      this.occupato = false;
    }
  };

  /** Verso l'edicola: il guscio non ha niente da fare. */
  esci = () => {
    this.#giro++;
    rivelazione.chiudi();
    registroCache.svuota();
    this.#id = null;
    this.vista = null;
    this.serie = [];
    this.guasto = null;
    bozza.chiudi();
  };
}

export const partita = new Partita();
