//! Guscio Tauri: legge i file di dati, sceglie il seme dall'orologio, tiene la partita in corso,
//! la salva nell'archivio e passa la `Vista` alla UI.

mod archivio;
mod errore;

use std::path::Path;
use std::sync::{Mutex, MutexGuard, PoisonError};
use std::time::{SystemTime, UNIX_EPOCH};

use archivio::Archivio;
use errore::{ErroreGuscio, guasto, rifiuto};
use sim_core::elettorato::{Condizione, Fascia};
use sim_core::esito::VoceEsito;
use sim_core::informazione::ElettoreCampione;
use sim_core::legislatura::{
    AnteprimaFondazione, Eredita, Fine, Fondazione, Spesa, VoceAnteprimaCampagna,
};
use sim_core::parlamento::{Decreto, ValoreProposto};
use sim_core::riepilogo::{CatalogoScenario, RiepilogoScenario, catalogo, riepilogo};
use sim_core::salvataggi::{PuntoSerie, VoceAlbo, VocePartita};
use sim_core::vista::{Anteprima, VoceMenu};
use sim_core::{Comando, DatiGioco, FileDati, Partita, SuMisura, Vista};
use tauri::{Manager, RunEvent, State};

/// La partita in corso.
struct InCorso {
    id: String,
    partita: Partita,
}

/// Ogni chiamata all'`Archivio` avviene tenendo questo `Mutex`: bozza, snapshot e uscita non si
/// sovrappongono.
type Stato = Mutex<Option<InCorso>>;

/// Il seme scelto dall'anteprima della fondazione: la fondazione lo usa e lo azzera solo se riesce,
/// così la partita fondata è quella che l'anteprima ha mostrato.
type SemeFondazione = Mutex<Option<u64>>;

const ERRORE_STATO: &str = "stato della partita non disponibile";
const NESSUNA: &str = "nessuna partita in corso";
const SENZA_SEGGI: &str =
    "la partita è finita senza seggi: conferma l'eredità per chiuderla nell'albo";

fn leggi(cartella: &Path, nome: &str) -> Result<String, String> {
    let percorso = cartella.join(nome);
    std::fs::read_to_string(&percorso)
        .map_err(|e| format!("impossibile leggere {}: {e}", percorso.display()))
}

fn carica_dati(cartella: &Path) -> Result<DatiGioco, String> {
    let file = FileDati::leggi(|nome| leggi(cartella, nome))?;
    DatiGioco::da_testi(&file).map_err(|e| e.to_string())
}

fn seme_da_orologio() -> Result<u64, String> {
    let nanosecondi = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("orologio di sistema non valido: {e}"))?
        .as_nanos();
    u64::try_from(nanosecondi).map_err(|e| format!("seme fuori intervallo: {e}"))
}

/// Il seme dell'anteprima, se c'è, senza toglierlo; altrimenti uno nuovo dall'orologio.
fn seme_per_fondare(seme: &SemeFondazione) -> Result<u64, ErroreGuscio> {
    let tenuto = *blocca_seme(seme);
    Ok(match tenuto {
        Some(s) => s,
        None => seme_da_orologio()?,
    })
}

/// L'anteprima con un seme nuovo, che resta tenuto per la fondazione.
fn anteprima_di_fondazione(
    base: &DatiGioco,
    scenario: Option<&SuMisura>,
    seme: &SemeFondazione,
) -> Result<AnteprimaFondazione, ErroreGuscio> {
    // Il lock sta per tutto il calcolo: l'ordine dei semi segue quello delle anteprime.
    let mut tenuto = blocca_seme(seme);
    let dati = dati_di(base, scenario)?;
    let nuovo = seme_da_orologio()?;
    let risultato = Partita::anteprima_fondazione(&dati, nuovo);
    *tenuto = Some(nuovo);
    Ok(risultato)
}

fn blocca(stato: &Stato) -> Result<MutexGuard<'_, Option<InCorso>>, ErroreGuscio> {
    stato.lock().map_err(|_| guasto(ERRORE_STATO))
}

/// Il lock anche avvelenato, per chi tocca solo il disco o sostituisce lo stato per intero.
fn blocca_comunque(stato: &Stato) -> MutexGuard<'_, Option<InCorso>> {
    stato.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Il lock del seme anche avvelenato: un `Option<u64>` è valido dopo un panico.
fn blocca_seme(seme: &SemeFondazione) -> MutexGuard<'_, Option<u64>> {
    seme.lock().unwrap_or_else(PoisonError::into_inner)
}

/// I dati di base, o una loro copia con lo scenario su misura.
fn dati_di(base: &DatiGioco, scenario: Option<&SuMisura>) -> Result<DatiGioco, ErroreGuscio> {
    match scenario {
        Some(s) => base.con_su_misura(s).map_err(rifiuto),
        None => Ok(base.clone()),
    }
}

/// La partita vera, salvata con l'id del seme in esadecimale.
fn fonda(
    stato: &Stato,
    archivio: &Archivio,
    base: &DatiGioco,
    scenario: Option<&SuMisura>,
    fondazione: &Fondazione,
    seme: u64,
) -> Result<Vista, ErroreGuscio> {
    let partita = Partita::fonda(dati_di(base, scenario)?, seme, fondazione).map_err(rifiuto)?;
    let id = format!("{seme:016x}");
    let vista = partita.vista();
    let mut guardia = blocca_comunque(stato);
    archivio.crea(&partita, &id)?;
    *guardia = Some(InCorso { id, partita });
    stato.clear_poison();
    Ok(vista)
}

/// Fonda col seme tenuto; solo a fondazione riuscita lo azzera, se è ancora quello usato (un'anteprima
/// arrivata nel frattempo ha messo il suo). L'azzeramento sta fuori da `fonda`: i due lock non si
/// tengono insieme.
fn fonda_col_seme(
    stato: &Stato,
    archivio: &Archivio,
    base: &DatiGioco,
    scenario: Option<&SuMisura>,
    fondazione: &Fondazione,
    seme: &SemeFondazione,
) -> Result<Vista, ErroreGuscio> {
    let usato = seme_per_fondare(seme)?;
    let vista = fonda(stato, archivio, base, scenario, fondazione, usato)?;
    let mut tenuto = blocca_seme(seme);
    if *tenuto == Some(usato) {
        *tenuto = None;
    }
    Ok(vista)
}

fn apri_partita(stato: &Stato, archivio: &Archivio, id: String) -> Result<Vista, ErroreGuscio> {
    let mut guardia = blocca_comunque(stato);
    let partita = archivio.riprendi(&id)?;
    let vista = partita.vista();
    *guardia = Some(InCorso { id, partita });
    stato.clear_poison();
    Ok(vista)
}

/// Chiude il turno e scrive lo snapshot della nuova apertura. Lo snapshot si scrive anche se la
/// chiusura finisce la partita; se era già finita, no. Se lo snapshot non si scrive, errore e
/// partita in memoria invariata. Il disco può restare un turno avanti, se il guasto arriva dopo
/// il file del turno: la chiusura ripetuta lo riscrive.
fn chiudi(stato: &Stato, archivio: &Archivio) -> Result<Vista, ErroreGuscio> {
    let mut guardia = blocca(stato)?;
    let c = guardia.as_mut().ok_or(NESSUNA)?;
    let gia_finita = c.partita.finita();
    let mut nuova = c.partita.clone();
    nuova.chiudi_turno();
    if !gia_finita {
        archivio.snapshot(&c.id, &nuova)?;
    }
    c.partita = nuova;
    Ok(c.partita.vista())
}

/// Il comando, e la bozza se è accettato. Un comando rifiutato non la scrive. Se la
/// bozza non si scrive, errore e partita in memoria invariata.
fn esegui(stato: &Stato, archivio: &Archivio, comando: Comando) -> Result<Vista, ErroreGuscio> {
    let mut guardia = blocca(stato)?;
    let c = guardia.as_mut().ok_or(NESSUNA)?;
    let mut nuova = c.partita.clone();
    nuova.comanda(comando).map_err(rifiuto)?;
    archivio.bozza(&c.id, &nuova)?;
    c.partita = nuova;
    Ok(c.partita.vista())
}

/// Se il riavvolgimento fallisce, la partita in memoria torna quella su disco, se si riapre.
fn torna_a(stato: &Stato, archivio: &Archivio, turno: u32) -> Result<Vista, ErroreGuscio> {
    let mut guardia = blocca(stato)?;
    let c = guardia.as_mut().ok_or(NESSUNA)?;
    let id = &c.id;
    match archivio.riavvolgi(id, turno) {
        Ok(p) => c.partita = p,
        Err(e) => {
            if let Ok(p) = archivio.riprendi(id) {
                c.partita = p;
            }
            return Err(e.into());
        }
    }
    Ok(c.partita.vista())
}

/// Il registro di un turno che la partita in memoria ha chiuso. Dopo un guasto a metà il disco può
/// stare un turno avanti (vedi `chiudi`): quel turno per il giocatore non è chiuso, e si rifiuta
/// prima di toccare il disco.
fn registro_di(
    stato: &Stato,
    archivio: &Archivio,
    turno: u32,
) -> Result<Vec<VoceEsito>, ErroreGuscio> {
    let guardia = blocca(stato)?;
    let c = guardia.as_ref().ok_or(NESSUNA)?;
    let id = c.id.as_str();
    let in_corso = c.partita.testata().turno;
    if turno >= in_corso {
        return Err(rifiuto(format!(
            "nessun registro per il turno {turno}: il turno non è ancora chiuso"
        )));
    }
    archivio.registro(id, turno)
}

/// La voce dell'albo della partita finita, poi la sua cartella cancellata.
fn nell_albo(archivio: &Archivio, id: &str, partita: &Partita) -> Result<(), ErroreGuscio> {
    let voce = partita
        .voce_albo()
        .ok_or("nessuna voce d'albo: la partita non è finita")?;
    Ok(archivio.chiudi_nell_albo(id, &voce)?)
}

/// L'abbandono: l'eredità subito definitiva, nell'albo, poi lo stato vuoto. Ripetibile
/// se l'albo non si scrive: la partita già abbandonata non si abbandona di nuovo.
fn lascia(stato: &Stato, archivio: &Archivio) -> Result<Eredita, ErroreGuscio> {
    let mut guardia = blocca(stato)?;
    let c = guardia.as_mut().ok_or(NESSUNA)?;
    let eredita = match c.partita.eredita() {
        Some(e) if e.fine == Fine::Abbandono => e,
        Some(_) => return Err(rifiuto(SENZA_SEGGI)),
        None => c.partita.abbandona().map_err(rifiuto)?,
    };
    nell_albo(archivio, &c.id, &c.partita)?;
    *guardia = None;
    stato.clear_poison();
    Ok(eredita)
}

/// Conferma l'eredità in bozza della partita finita senza seggi: nell'albo, poi lo
/// stato vuoto.
fn conferma(stato: &Stato, archivio: &Archivio) -> Result<(), ErroreGuscio> {
    let mut guardia = blocca(stato)?;
    let c = guardia.as_ref().ok_or(NESSUNA)?;
    if !c.partita.eredita().is_some_and(|e| e.bozza) {
        return Err(rifiuto("nessuna eredità in bozza da confermare"));
    }
    nell_albo(archivio, &c.id, &c.partita)?;
    *guardia = None;
    stato.clear_poison();
    Ok(())
}

/// All'uscita, la bozza della partita in corso, se non è finita. Col lock avvelenato
/// niente bozza: dopo un panic lo stato può essere a metà, e ogni comando accettato ha già scritto
/// la sua. Un errore non blocca l'uscita.
fn alla_chiusura(stato: &Stato, archivio: &Archivio) {
    let Ok(guardia) = stato.lock() else {
        eprintln!(
            "bozza non salvata all'uscita: lo stato della partita è rimasto a metà dopo un errore interno"
        );
        return;
    };
    if let Some(InCorso { id, partita }) = guardia.as_ref()
        && !partita.finita()
        && let Err(e) = archivio.bozza(id, partita)
    {
        eprintln!("bozza non salvata all'uscita: {e}");
    }
}

// async: il comando gira fuori dal thread principale della finestra, che resta reattiva mentre il motore chiude il turno.
#[tauri::command]
async fn chiudi_turno(
    stato: State<'_, Stato>,
    archivio: State<'_, Archivio>,
) -> Result<Vista, ErroreGuscio> {
    chiudi(&stato, &archivio)
}

#[tauri::command]
async fn partite(
    stato: State<'_, Stato>,
    archivio: State<'_, Archivio>,
) -> Result<Vec<VocePartita>, ErroreGuscio> {
    let _guardia = blocca_comunque(&stato);
    Ok(archivio.elenca())
}

#[tauri::command]
async fn fonda_partita(
    scenario: Option<SuMisura>,
    fondazione: Fondazione,
    base: State<'_, DatiGioco>,
    stato: State<'_, Stato>,
    archivio: State<'_, Archivio>,
    seme: State<'_, SemeFondazione>,
) -> Result<Vista, ErroreGuscio> {
    fonda_col_seme(
        &stato,
        &archivio,
        &base,
        scenario.as_ref(),
        &fondazione,
        &seme,
    )
}

#[tauri::command]
async fn riprendi(
    id: String,
    stato: State<'_, Stato>,
    archivio: State<'_, Archivio>,
) -> Result<Vista, ErroreGuscio> {
    apri_partita(&stato, &archivio, id)
}

#[tauri::command]
async fn comanda(
    comando: Comando,
    stato: State<'_, Stato>,
    archivio: State<'_, Archivio>,
) -> Result<Vista, ErroreGuscio> {
    esegui(&stato, &archivio, comando)
}

#[tauri::command]
async fn riavvolgi(
    turno: u32,
    stato: State<'_, Stato>,
    archivio: State<'_, Archivio>,
) -> Result<Vista, ErroreGuscio> {
    torna_a(&stato, &archivio, turno)
}

#[tauri::command]
async fn registro(
    turno: u32,
    stato: State<'_, Stato>,
    archivio: State<'_, Archivio>,
) -> Result<Vec<VoceEsito>, ErroreGuscio> {
    registro_di(&stato, &archivio, turno)
}

#[tauri::command]
async fn abbandona(
    stato: State<'_, Stato>,
    archivio: State<'_, Archivio>,
) -> Result<Eredita, ErroreGuscio> {
    lascia(&stato, &archivio)
}

#[tauri::command]
async fn conferma_eredita(
    stato: State<'_, Stato>,
    archivio: State<'_, Archivio>,
) -> Result<(), ErroreGuscio> {
    conferma(&stato, &archivio)
}

#[tauri::command]
async fn albo(
    stato: State<'_, Stato>,
    archivio: State<'_, Archivio>,
) -> Result<Vec<VoceAlbo>, ErroreGuscio> {
    let _guardia = blocca_comunque(&stato);
    Ok(archivio.albo())
}

#[tauri::command]
async fn scenari(
    stato: State<'_, Stato>,
    archivio: State<'_, Archivio>,
) -> Result<Vec<String>, ErroreGuscio> {
    let _guardia = blocca_comunque(&stato);
    Ok(archivio.scenari())
}

#[tauri::command]
async fn salva_scenario(
    scenario: SuMisura,
    base: State<'_, DatiGioco>,
    stato: State<'_, Stato>,
    archivio: State<'_, Archivio>,
) -> Result<(), ErroreGuscio> {
    let _guardia = blocca_comunque(&stato);
    archivio.salva_scenario(&scenario, &base)
}

#[tauri::command]
async fn carica_scenario(
    nome: String,
    base: State<'_, DatiGioco>,
    stato: State<'_, Stato>,
    archivio: State<'_, Archivio>,
) -> Result<SuMisura, ErroreGuscio> {
    let _guardia = blocca_comunque(&stato);
    Ok(archivio.carica_scenario(&nome, &base)?)
}

// async: sei partite costruite, una per posizione, fuori dal thread principale.
#[tauri::command]
async fn riepilogo_scenario(
    scenario: Option<SuMisura>,
    base: State<'_, DatiGioco>,
) -> Result<RiepilogoScenario, ErroreGuscio> {
    Ok(riepilogo(&dati_di(&base, scenario.as_ref())?))
}

// async: sei partite costruite, una per posizione, fuori dal thread principale.
#[tauri::command]
async fn anteprima_fondazione(
    scenario: Option<SuMisura>,
    base: State<'_, DatiGioco>,
    seme: State<'_, SemeFondazione>,
) -> Result<AnteprimaFondazione, ErroreGuscio> {
    anteprima_di_fondazione(&base, scenario.as_ref(), &seme)
}

#[tauri::command]
async fn catalogo_scenario(base: State<'_, DatiGioco>) -> Result<CatalogoScenario, ErroreGuscio> {
    catalogo(&base).map_err(guasto)
}

/// Esegue `f` sulla partita in corso, in sola lettura.
fn sulla_partita<T>(
    stato: &Stato,
    f: impl FnOnce(&Partita) -> Result<T, sim_core::Errore>,
) -> Result<T, ErroreGuscio> {
    let guardia = blocca(stato)?;
    let c = guardia.as_ref().ok_or(NESSUNA)?;
    f(&c.partita).map_err(rifiuto)
}

fn serie_in_corso(stato: &Stato) -> Result<Vec<PuntoSerie>, ErroreGuscio> {
    sulla_partita(stato, |p| Ok(p.serie().to_vec()))
}

#[tauri::command]
async fn serie(stato: State<'_, Stato>) -> Result<Vec<PuntoSerie>, ErroreGuscio> {
    serie_in_corso(&stato)
}

// async: le anteprime prendono il Mutex che tiene `chiudi_turno`; fuori dal thread principale non lo bloccano.
#[tauri::command]
async fn anteprima(
    valori: Vec<ValoreProposto>,
    stato: State<'_, Stato>,
) -> Result<Anteprima, ErroreGuscio> {
    sulla_partita(&stato, |p| p.anteprima(&valori))
}

fn anteprima_di_campagna(
    stato: &Stato,
    spese: &[Spesa],
) -> Result<Vec<VoceAnteprimaCampagna>, ErroreGuscio> {
    sulla_partita(stato, |p| p.anteprima_campagna(spese))
}

fn elettori_del_gruppo(
    stato: &Stato,
    regione: &str,
    condizione: Condizione,
    fascia: Fascia,
) -> Result<Vec<ElettoreCampione>, ErroreGuscio> {
    sulla_partita(stato, |p| p.elettori_campione(regione, condizione, fascia))
}

#[tauri::command]
async fn anteprima_campagna(
    spese: Vec<Spesa>,
    stato: State<'_, Stato>,
) -> Result<Vec<VoceAnteprimaCampagna>, ErroreGuscio> {
    anteprima_di_campagna(&stato, &spese)
}

#[tauri::command]
async fn elettori_campione(
    regione: String,
    condizione: Condizione,
    fascia: Fascia,
    stato: State<'_, Stato>,
) -> Result<Vec<ElettoreCampione>, ErroreGuscio> {
    elettori_del_gruppo(&stato, &regione, condizione, fascia)
}

#[tauri::command]
async fn anteprima_decreto(
    decreto: Decreto,
    stato: State<'_, Stato>,
) -> Result<Anteprima, ErroreGuscio> {
    sulla_partita(&stato, |p| p.anteprima_decreto(&decreto))
}

#[tauri::command]
async fn anteprima_accogli(
    dicastero: String,
    stato: State<'_, Stato>,
) -> Result<Anteprima, ErroreGuscio> {
    sulla_partita(&stato, |p| p.anteprima_accogli(&dicastero))
}

fn costo_del_dirotta(
    stato: &Stato,
    dicastero: &str,
    atto: Option<&str>,
    regione: Option<&str>,
) -> Result<f64, ErroreGuscio> {
    sulla_partita(stato, |p| p.costo_dirotta(dicastero, atto, regione))
}

fn menu_del_governo(
    stato: &Stato,
    valori: &[ValoreProposto],
) -> Result<Vec<VoceMenu>, ErroreGuscio> {
    sulla_partita(stato, |p| Ok(p.menu_elettorale(valori)))
}

fn anteprima_della_proposta(
    stato: &Stato,
    valori: &[ValoreProposto],
) -> Result<Anteprima, ErroreGuscio> {
    sulla_partita(stato, |p| p.anteprima_proposta(valori))
}

#[tauri::command]
async fn costo_dirotta(
    dicastero: String,
    atto: Option<String>,
    regione: Option<String>,
    stato: State<'_, Stato>,
) -> Result<f64, ErroreGuscio> {
    costo_del_dirotta(&stato, &dicastero, atto.as_deref(), regione.as_deref())
}

#[tauri::command]
async fn menu_elettorale(
    valori: Vec<ValoreProposto>,
    stato: State<'_, Stato>,
) -> Result<Vec<VoceMenu>, ErroreGuscio> {
    menu_del_governo(&stato, &valori)
}

#[tauri::command]
async fn anteprima_proposta(
    valori: Vec<ValoreProposto>,
    stato: State<'_, Stato>,
) -> Result<Anteprima, ErroreGuscio> {
    anteprima_della_proposta(&stato, &valori)
}

pub fn run() -> tauri::Result<()> {
    tauri::Builder::default()
        .manage(Stato::default())
        .manage(SemeFondazione::default())
        // `resource_dir` e `app_data_dir` ci sono solo dopo `build`.
        .setup(|app| {
            app.manage(carica_dati(&app.path().resource_dir()?.join("dati/base"))?);
            let mut radice = app.path().app_data_dir()?;
            if cfg!(debug_assertions) {
                radice.push("sviluppo");
            }
            app.manage(Archivio::nuovo(radice)?);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            chiudi_turno,
            partite,
            fonda_partita,
            riprendi,
            comanda,
            riavvolgi,
            registro,
            abbandona,
            conferma_eredita,
            albo,
            scenari,
            salva_scenario,
            carica_scenario,
            riepilogo_scenario,
            anteprima,
            anteprima_decreto,
            anteprima_accogli,
            anteprima_campagna,
            costo_dirotta,
            menu_elettorale,
            anteprima_proposta,
            elettori_campione,
            serie,
            anteprima_fondazione,
            catalogo_scenario
        ])
        .build(tauri::generate_context!())?
        // Cmd+Q dà solo `RunEvent::Exit` (ricerca `persistenza-dei-salvataggi`, «Chiusura
        // dell'app e bozza del turno»).
        .run(|app, evento| {
            if let RunEvent::Exit = evento
                && let (Some(stato), Some(archivio)) =
                    (app.try_state::<Stato>(), app.try_state::<Archivio>())
            {
                alla_chiusura(&stato, &archivio);
            }
        });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::archivio::tests::{ID, cartella, dati, fondazione};
    use crate::errore::Specie;
    use sim_core::elettorato::{Condizione, Fascia};
    use sim_core::legislatura::{Bersaglio, Leva, Spesa};

    /// Il seme che dà `ID`.
    const SEME: u64 = 0xa1;

    fn campagna(bersagli: usize) -> Comando {
        let spese = (0..bersagli)
            .map(|r| Spesa {
                bersaglio: Bersaglio::Regione(r),
                leva: Leva::Persuadere,
                euro: 1000,
            })
            .collect();
        Comando::Campagna { spese }
    }

    /// Lo stato con una partita vera fondata nell'archivio di `cartella`.
    fn con_partita(cartella: &Path) -> Result<(Stato, Archivio), String> {
        let stato = Stato::default();
        let archivio = Archivio::nuovo(cartella.to_path_buf())?;
        fonda(&stato, &archivio, &dati(), None, &fondazione(), SEME)?;
        Ok((stato, archivio))
    }

    fn nella_partita(stato: &Stato, f: impl FnOnce(&mut Partita)) -> Result<(), String> {
        let mut guardia = blocca_comunque(stato);
        f(&mut guardia.as_mut().ok_or(NESSUNA)?.partita);
        Ok(())
    }

    /// La partita di prova: ha un id, ma nessuna cartella nell'archivio.
    fn avvia(stato: &Stato, dati: DatiGioco, seme: u64) -> Result<Vista, ErroreGuscio> {
        let partita = Partita::nuova(dati, seme);
        let vista = partita.vista();
        *blocca_comunque(stato) = Some(InCorso {
            id: "prova".into(),
            partita,
        });
        Ok(vista)
    }

    /// Avvelena il lock con un panic in un altro thread.
    fn avvelena(stato: &Stato) {
        let _ = std::thread::scope(|s| {
            s.spawn(|| {
                let _guardia = stato.lock();
                panic!("lock avvelenato apposta");
            })
            .join()
        });
        assert!(stato.is_poisoned());
    }

    #[test]
    fn lock_avvelenato_e_un_guasto() -> Result<(), String> {
        let c = cartella("lib-lock-avvelenato");
        let (stato, archivio) = con_partita(&c.0)?;
        avvelena(&stato);
        assert_eq!(chiudi(&stato, &archivio), Err(guasto(ERRORE_STATO)));
        // I comandi che toccano solo il disco prendono il lock anche avvelenato.
        assert!(blocca_comunque(&stato).is_some());

        fonda(&stato, &archivio, &dati(), None, &fondazione(), SEME + 1)?;
        assert!(!stato.is_poisoned());
        assert_eq!(chiudi(&stato, &archivio)?.turno, 1);

        // Anche riprendere una partita salvata ripulisce il lock.
        avvelena(&stato);
        apri_partita(&stato, &archivio, ID.into())?;
        assert!(!stato.is_poisoned());
        assert_eq!(chiudi(&stato, &archivio)?.turno, 1);
        Ok(())
    }

    #[test]
    fn rifiuto_del_core_su_comanda() -> Result<(), String> {
        let c = cartella("lib-rifiuto-del-core");
        let (stato, archivio) = con_partita(&c.0)?;
        let prima = sulla_partita(&stato, |p| Ok(p.vista()))?;
        // Quattro bersagli: oltre i tre che il core ammette.
        let e = esegui(&stato, &archivio, campagna(4)).err();
        assert_eq!(e.map(|e| e.specie), Some(Specie::Rifiuto));
        assert_eq!(sulla_partita(&stato, |p| Ok(p.vista()))?, prima);
        Ok(())
    }

    #[test]
    fn nome_di_scenario_non_valido_e_un_rifiuto() -> Result<(), String> {
        let c = cartella("lib-nome-scenario-non-valido");
        let archivio = Archivio::nuovo(c.0.clone())?;
        let base = dati();
        let mut s = SuMisura::base(&base);
        s.nome = String::new();
        let e = archivio.salva_scenario(&s, &base).err();
        assert_eq!(e.map(|e| e.specie), Some(Specie::Rifiuto));
        Ok(())
    }

    #[test]
    fn riavvolgi_fallito_riallinea_la_memoria_al_disco() -> Result<(), String> {
        let c = cartella("lib-riavvolgi-fallito");
        let (stato, archivio) = con_partita(&c.0)?;
        chiudi(&stato, &archivio)?;
        chiudi(&stato, &archivio)?;
        // `turno-002` diventa una cartella: la sua cancellazione fallisce dopo quella della bozza.
        let turno_due = c.0.join("partite").join(ID).join("turno-002.json.gz");
        std::fs::remove_file(&turno_due).map_err(|e| e.to_string())?;
        std::fs::create_dir(&turno_due).map_err(|e| e.to_string())?;
        std::fs::write(turno_due.join("dentro"), "x").map_err(|e| e.to_string())?;

        let e = torna_a(&stato, &archivio, 0)
            .err()
            .map_or_else(String::new, String::from);
        assert!(
            e.starts_with(&format!(
                "impossibile cancellare partite/{ID}/turno-002.json.gz"
            )),
            "{e}"
        );
        // In memoria c'è ora l'ultimo snapshot leggibile, il turno 1.
        assert_eq!(sulla_partita(&stato, |p| Ok(p.vista()))?.turno, 1);
        Ok(())
    }

    /// Sostituisce `nome` con una cartella non vuota: ogni scrittura su quel file fallisce.
    fn guasta(c: &Path, nome: &str) -> Result<std::path::PathBuf, String> {
        let file = c.join("partite").join(ID).join(nome);
        // Il file può mancare; ogni altro errore emerge da `create_dir`.
        let _ = std::fs::remove_file(&file);
        std::fs::create_dir(&file).map_err(|e| e.to_string())?;
        std::fs::write(file.join("dentro"), "x").map_err(|e| e.to_string())?;
        Ok(file)
    }

    #[test]
    fn chiudi_con_snapshot_fallito_non_avanza() -> Result<(), String> {
        let c = cartella("lib-chiudi-snapshot-fallito");
        let (stato, archivio) = con_partita(&c.0)?;
        let prima = sulla_partita(&stato, |p| Ok(p.clone()))?;
        let guasto = guasta(&c.0, "turno-001.json.gz")?;

        assert!(chiudi(&stato, &archivio).is_err());
        assert!(sulla_partita(&stato, |p| Ok(*p == prima))?);
        assert_eq!(sulla_partita(&stato, |p| Ok(p.vista()))?.turno, 0);

        std::fs::remove_dir_all(&guasto).map_err(|e| e.to_string())?;
        assert_eq!(chiudi(&stato, &archivio)?.turno, 1);
        Ok(())
    }

    #[test]
    fn esegui_con_bozza_fallita_non_applica_il_comando() -> Result<(), String> {
        let c = cartella("lib-esegui-bozza-fallita");
        let (stato, archivio) = con_partita(&c.0)?;
        let prima = sulla_partita(&stato, |p| Ok(p.clone()))?;
        guasta(&c.0, "bozza.json.gz")?;

        // L'errore è della scrittura, non di un comando rifiutato.
        let e = esegui(&stato, &archivio, campagna(1))
            .err()
            .map_or_else(String::new, String::from);
        assert!(e.starts_with("impossibile"), "{e}");
        assert!(sulla_partita(&stato, |p| Ok(*p == prima))?);
        Ok(())
    }

    /// Il guasto dopo il file del turno: su disco resta `turno-001`, la memoria no, e la chiusura
    /// ripetuta lo riscrive.
    #[test]
    fn chiudi_guasta_a_meta_si_ripete() -> Result<(), String> {
        let c = cartella("lib-chiudi-a-meta");
        let (stato, archivio) = con_partita(&c.0)?;
        // La bozza non si cancella: lo snapshot fallisce al secondo passo.
        let guasto = guasta(&c.0, "bozza.json.gz")?;

        let e = chiudi(&stato, &archivio)
            .err()
            .map_or_else(String::new, String::from);
        assert!(e.starts_with("impossibile cancellare"), "{e}");
        assert!(guasto.with_file_name("turno-001.json.gz").is_file());
        assert_eq!(sulla_partita(&stato, |p| Ok(p.vista()))?.turno, 0);

        std::fs::remove_dir_all(&guasto).map_err(|e| e.to_string())?;
        assert_eq!(chiudi(&stato, &archivio)?.turno, 1);
        Ok(())
    }

    #[test]
    fn registro_del_turno_in_corso_rifiutato() -> Result<(), String> {
        let c = cartella("lib-registro-in-corso");
        let (stato, archivio) = con_partita(&c.0)?;
        // Il disco resta un turno avanti: su disco c'è `turno-001`, la memoria è al turno 0.
        let guasto = guasta(&c.0, "bozza.json.gz")?;
        assert!(chiudi(&stato, &archivio).is_err());
        assert!(guasto.with_file_name("turno-001.json.gz").is_file());

        let e = registro_di(&stato, &archivio, 0).err();
        assert_eq!(
            e,
            Some(rifiuto(
                "nessun registro per il turno 0: il turno non è ancora chiuso"
            ))
        );

        std::fs::remove_dir_all(&guasto).map_err(|e| e.to_string())?;
        chiudi(&stato, &archivio)?;
        assert!(registro_di(&stato, &archivio, 0).is_ok());
        assert_eq!(
            registro_di(&stato, &archivio, 1).err().map(|e| e.specie),
            Some(Specie::Rifiuto)
        );
        Ok(())
    }

    #[test]
    fn registro_senza_partita_e_un_guasto() -> Result<(), String> {
        let c = cartella("lib-registro-senza-partita");
        let archivio = Archivio::nuovo(c.0.clone())?;
        assert_eq!(
            registro_di(&Stato::default(), &archivio, 0),
            Err(guasto(NESSUNA))
        );
        Ok(())
    }

    #[test]
    fn senza_partita_e_un_guasto() -> Result<(), String> {
        let c = cartella("lib-chiudi-senza-partita");
        let archivio = Archivio::nuovo(c.0.clone())?;
        let stato = Stato::default();
        assert_eq!(chiudi(&stato, &archivio), Err(guasto(NESSUNA)));
        Ok(())
    }

    #[test]
    fn anteprima_senza_partita_da_errore() {
        let stato = Stato::default();
        let r = sulla_partita(&stato, |p| p.anteprima_accogli("sanita"));
        assert_eq!(r.err(), Some(guasto(NESSUNA)));
    }

    #[test]
    fn anteprima_rifiutata_porta_il_nome_del_dicastero() -> Result<(), String> {
        let stato = Stato::default();
        avvia(&stato, dati(), 1)?;
        let errore = sulla_partita(&stato, |p| p.anteprima_accogli("giustizia"))
            .err()
            .map_or_else(String::new, String::from);
        assert!(errore.contains("Giustizia"), "{errore}");
        Ok(())
    }

    #[test]
    fn chiudi_avanza_il_turno() -> Result<(), String> {
        let c = cartella("lib-chiudi-avanza");
        let (stato, archivio) = con_partita(&c.0)?;
        let vista = chiudi(&stato, &archivio)?;
        assert_eq!(
            (vista.turno, vista.data.trimestre, vista.data.anno),
            (1, 1, 2027)
        );
        assert_eq!(vista.dicasteri.len(), 10);
        Ok(())
    }

    #[test]
    fn cartella_mancante_nomina_il_percorso() {
        let errore = carica_dati(Path::new("/non/esiste"))
            .err()
            .map_or_else(String::new, String::from);
        assert!(errore.contains("/non/esiste/dicasteri.toml"), "{errore}");
    }

    #[test]
    fn fonda_comanda_chiudi_riprendi() -> Result<(), String> {
        let c = cartella("lib-fonda-comanda-chiudi-riprendi");
        let (stato, archivio) = con_partita(&c.0)?;
        let file = |nome: &str| c.0.join("partite").join(ID).join(nome);
        assert!(file("turno-000.json.gz").exists());

        // Un comando rifiutato non scrive la bozza; uno accettato sì.
        let e = esegui(&stato, &archivio, campagna(4))
            .err()
            .map_or_else(String::new, String::from);
        assert!(e.contains("Al massimo 3 bersagli"), "{e}");
        assert!(!file("bozza.json.gz").exists());
        let vista = esegui(&stato, &archivio, campagna(1))?;
        assert!(file("bozza.json.gz").exists());
        assert_eq!(apri_partita(&stato, &archivio, ID.into())?, vista);

        let vista = chiudi(&stato, &archivio)?;
        assert_eq!(vista.turno, 1);
        assert!(!file("bozza.json.gz").exists());
        assert!(file("turno-001.json.gz").exists());
        assert_eq!(apri_partita(&stato, &archivio, ID.into())?, vista);
        assert_eq!(archivio.elenca().len(), 1);

        // Il riavvolgimento torna al turno zero e conta.
        assert_eq!(torna_a(&stato, &archivio, 0)?.turno, 0);
        assert!(!file("turno-001.json.gz").exists());
        Ok(())
    }

    #[test]
    fn scenario_fuori_intervallo_e_un_errore() -> Result<(), String> {
        let c = cartella("lib-scenario-fuori-intervallo");
        let stato = Stato::default();
        let archivio = Archivio::nuovo(c.0.clone())?;
        let base = dati();
        let mut s = SuMisura::base(&base);
        s.cursori.debito_pil = 1e6;
        let e = fonda(&stato, &archivio, &base, Some(&s), &fondazione(), SEME)
            .err()
            .map_or_else(String::new, String::from);
        assert!(e.contains("debito"), "{e}");
        assert!(dati_di(&base, Some(&s)).is_err());
        assert!(blocca(&stato)?.is_none());
        assert!(archivio.elenca().is_empty());
        Ok(())
    }

    #[test]
    fn abbandona_scrive_l_albo_e_libera_lo_stato() -> Result<(), String> {
        let c = cartella("lib-abbandona");
        let (stato, archivio) = con_partita(&c.0)?;
        let e = conferma(&stato, &archivio)
            .err()
            .map_or_else(String::new, String::from);
        assert_eq!(e, "nessuna eredità in bozza da confermare");

        let eredita = lascia(&stato, &archivio)?;
        assert_eq!((eredita.fine, eredita.bozza), (Fine::Abbandono, false));
        assert_eq!(archivio.albo().len(), 1);
        assert!(archivio.elenca().is_empty());
        assert!(blocca(&stato)?.is_none());
        assert_eq!(conferma(&stato, &archivio), Err(guasto(NESSUNA)));
        Ok(())
    }

    #[test]
    fn abbandono_ripetibile_se_l_albo_non_si_scrive() -> Result<(), String> {
        let c = cartella("lib-abbandono-ripetibile");
        let (stato, archivio) = con_partita(&c.0)?;
        // Già abbandonata in memoria, come dopo un albo non scritto.
        nella_partita(&stato, |p| {
            let _ = p.abbandona();
        })?;
        assert_eq!(lascia(&stato, &archivio)?.fine, Fine::Abbandono);
        assert_eq!(archivio.albo().len(), 1);
        assert!(blocca(&stato)?.is_none());
        Ok(())
    }

    #[test]
    fn conferma_dell_eredita_in_bozza() -> Result<(), String> {
        let c = cartella("lib-conferma");
        let (stato, archivio) = con_partita(&c.0)?;
        chiudi(&stato, &archivio)?;
        // Finita senza seggi nella chiusura del turno 1: il punto di serie del turno 1 non c'è.
        let mut guardia = blocca(&stato)?;
        let corrente = guardia.as_mut().ok_or(NESSUNA)?;
        let mut j: serde_json::Value =
            serde_json::from_str(&corrente.partita.salva().map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
        j["fine"] = "senza_seggi".into();
        if let Some(serie) = j["serie"].as_array_mut() {
            serie.pop();
        }
        corrente.partita = Partita::carica(&j.to_string()).map_err(|e| e.to_string())?;
        drop(guardia);

        // L'abbandono rimanda alla conferma.
        assert_eq!(lascia(&stato, &archivio), Err(rifiuto(SENZA_SEGGI)));
        conferma(&stato, &archivio)?;
        let albo = archivio.albo();
        assert_eq!(albo.len(), 1);
        assert_eq!(albo[0].esito, "Senza seggi");
        assert!(archivio.elenca().is_empty());
        assert!(blocca(&stato)?.is_none());
        Ok(())
    }

    #[test]
    fn conferma_rifiuta_la_partita_abbandonata() -> Result<(), String> {
        let c = cartella("lib-conferma-abbandonata");
        let (stato, archivio) = con_partita(&c.0)?;
        // Finita ma con eredità già definitiva: niente da confermare, niente albo.
        nella_partita(&stato, |p| {
            let _ = p.abbandona();
        })?;
        let e = conferma(&stato, &archivio)
            .err()
            .map_or_else(String::new, String::from);
        assert_eq!(e, "nessuna eredità in bozza da confermare");
        assert!(archivio.albo().is_empty());
        assert_eq!(archivio.elenca().len(), 1);
        assert!(blocca(&stato)?.is_some());
        Ok(())
    }

    #[test]
    fn alla_chiusura_scrive_la_bozza_solo_se_serve() -> Result<(), String> {
        let c = cartella("lib-alla-chiusura");
        let bozza = c.0.join("partite").join(ID).join("bozza.json.gz");

        // Una scelta fuori dai comandi del guscio: l'uscita la salva.
        let (stato, archivio) = con_partita(&c.0)?;
        nella_partita(&stato, |p| {
            let _ = p.comanda(campagna(1));
        })?;
        let attesa = sulla_partita(&stato, |p| Ok(p.vista()))?;
        alla_chiusura(&stato, &archivio);
        assert_eq!(archivio.riprendi(ID)?.vista(), attesa);

        // Col lock avvelenato lo stato può essere a metà: niente bozza.
        std::fs::remove_file(&bozza).map_err(|e| e.to_string())?;
        avvelena(&stato);
        alla_chiusura(&stato, &archivio);
        assert!(!bozza.exists());

        // A partita finita la bozza non si scrive.
        stato.clear_poison();
        nella_partita(&stato, |p| {
            let _ = p.abbandona();
        })?;
        alla_chiusura(&stato, &archivio);
        assert!(!bozza.exists());
        Ok(())
    }

    #[test]
    fn riepilogo_della_base_senza_scenario() -> Result<(), String> {
        let base = dati();
        assert_eq!(riepilogo(&dati_di(&base, None)?).posizioni.len(), 6);
        Ok(())
    }

    #[test]
    fn la_fondazione_riusa_il_seme_dell_anteprima() -> Result<(), String> {
        let c = cartella("lib-fondazione-riusa-seme");
        let stato = Stato::default();
        let archivio = Archivio::nuovo(c.0.clone())?;
        let seme = SemeFondazione::new(Some(SEME));
        fonda_col_seme(&stato, &archivio, &dati(), None, &fondazione(), &seme)?;
        let id = blocca(&stato)?.as_ref().map(|c| c.id.clone());
        assert_eq!(id.as_deref(), Some(ID));
        assert_eq!(*seme.lock().map_err(|e| e.to_string())?, None);
        Ok(())
    }

    #[test]
    fn la_fondazione_rifiutata_lascia_il_seme() -> Result<(), String> {
        let c = cartella("lib-fondazione-rifiutata");
        let stato = Stato::default();
        let archivio = Archivio::nuovo(c.0.clone())?;
        let seme = SemeFondazione::new(Some(SEME));
        let vuota = Fondazione {
            nome: String::new(),
            ..fondazione()
        };
        let e = fonda_col_seme(&stato, &archivio, &dati(), None, &vuota, &seme).err();
        assert_eq!(e.map(|e| e.specie), Some(Specie::Rifiuto));
        assert_eq!(*seme.lock().map_err(|e| e.to_string())?, Some(SEME));

        let vista = fonda_col_seme(&stato, &archivio, &dati(), None, &fondazione(), &seme)?;
        assert_eq!(vista.turno, 0);
        let id = blocca(&stato)?.as_ref().map(|c| c.id.clone());
        assert_eq!(id.as_deref(), Some(ID));
        assert_eq!(*seme.lock().map_err(|e| e.to_string())?, None);
        Ok(())
    }

    #[test]
    fn la_fondazione_senza_anteprima_usa_l_orologio() -> Result<(), String> {
        let c = cartella("lib-fondazione-senza-anteprima");
        let stato = Stato::default();
        let archivio = Archivio::nuovo(c.0.clone())?;
        let seme = SemeFondazione::default();
        fonda_col_seme(&stato, &archivio, &dati(), None, &fondazione(), &seme)?;
        let id = blocca(&stato)?.as_ref().map(|c| c.id.clone());
        assert!(id.is_some_and(|i| i != ID));
        assert_eq!(*seme.lock().map_err(|e| e.to_string())?, None);
        Ok(())
    }

    #[test]
    fn l_anteprima_mostra_il_sondaggio_della_partita() -> Result<(), String> {
        let base = dati();
        let seme = SemeFondazione::default();
        let a = anteprima_di_fondazione(&base, None, &seme)?;
        let s = seme_per_fondare(&seme)?;
        let f = fondazione();
        let voce = a
            .posizioni
            .iter()
            .find(|v| (v.posizione - f.posizione).abs() <= 1e-9)
            .ok_or("posizione scelta non in anteprima")?;

        let vista = Partita::fonda(base, s, &f)
            .map_err(|e| e.to_string())?
            .vista();
        let j = vista
            .partiti
            .iter()
            .position(|p| p.id == vista.giocatore)
            .ok_or("giocatore non nei partiti")?;
        let istituti = &vista.sondaggio.istituti;
        let quota = istituti
            .iter()
            .map(|r| r.quote[j] / (1.0 - r.quote.last().copied().unwrap_or(0.0)))
            .sum::<f64>()
            / istituti.len() as f64;
        assert!(
            (voce.quota_attesa - quota).abs() < 1e-12,
            "anteprima {} vista {quota}",
            voce.quota_attesa
        );
        Ok(())
    }

    #[test]
    fn serie_senza_partita_e_un_guasto() {
        assert_eq!(serie_in_corso(&Stato::default()), Err(guasto(NESSUNA)));
    }

    fn spesa(bersaglio: Bersaglio) -> Spesa {
        Spesa {
            bersaglio,
            leva: Leva::Mobilitare,
            euro: 300_000,
        }
    }

    fn bozza_cittadinanza() -> Vec<ValoreProposto> {
        vec![ValoreProposto {
            disposizione: "cittadinanza".into(),
            gradino: "ius_soli_temperato".into(),
        }]
    }

    #[test]
    fn costo_dirotta_senza_partita_e_un_guasto() {
        let r = costo_del_dirotta(&Stato::default(), "sanita", None, None);
        assert_eq!(r, Err(guasto(NESSUNA)));
    }

    #[test]
    fn menu_senza_partita_e_un_guasto() {
        let r = menu_del_governo(&Stato::default(), &[]);
        assert_eq!(r.err(), Some(guasto(NESSUNA)));
    }

    #[test]
    fn anteprima_proposta_senza_partita_e_un_guasto() {
        let r = anteprima_della_proposta(&Stato::default(), &bozza_cittadinanza());
        assert_eq!(r.err(), Some(guasto(NESSUNA)));
    }

    #[test]
    fn costo_dirotta_al_turno_zero_e_un_rifiuto() -> Result<(), String> {
        let c = cartella("lib-costo-dirotta");
        let (stato, _archivio) = con_partita(&c.0)?;
        let atteso = blocca_comunque(&stato)
            .as_ref()
            .ok_or(NESSUNA)?
            .partita
            .costo_dirotta("sanita", None, None)
            .err()
            .ok_or("Dirotta accettata al turno zero")?
            .to_string();
        let e = costo_del_dirotta(&stato, "sanita", None, None)
            .err()
            .ok_or("costo dato al turno zero")?;
        assert_eq!(e.specie, Specie::Rifiuto);
        assert_eq!(e.testo, atteso);
        Ok(())
    }

    /// Una partita fondata a `posizione`, con chi vuole in coalizione, chiusa due volte (voto,
    /// trattativa): al governo se `destra`, altrimenti all'opposizione.
    fn dopo_il_voto(seme: u64, posizione: f64, destra: bool) -> Result<Stato, String> {
        let f = Fondazione {
            nome: "Partito di prova".into(),
            posizione,
        };
        let mut p = Partita::fonda(dati(), seme, &f).map_err(|e| e.to_string())?;
        if destra {
            let partiti = (p.disponibilita().into_iter())
                .filter(|v| {
                    v.disponibile
                        && ["destra", "centrodestra", "estrema_destra"]
                            .contains(&v.partito.as_str())
                })
                .map(|v| v.partito)
                .collect();
            p.comanda(Comando::DichiaraCoalizione {
                partiti,
                cessioni: vec![],
            })
            .map_err(|e| e.to_string())?;
        }
        p.chiudi_turno();
        p.chiudi_turno();
        assert_eq!(p.governa(), destra, "governo non come voluto");
        let stato = Stato::default();
        *blocca_comunque(&stato) = Some(InCorso {
            id: "prova".into(),
            partita: p,
        });
        Ok(stato)
    }

    #[test]
    fn costo_dirotta_al_governo_e_quello_del_core() -> Result<(), String> {
        let stato = dopo_il_voto(4, 4.5, true)?;
        let (p, v) = {
            let g = blocca_comunque(&stato);
            let p = g.as_ref().ok_or(NESSUNA)?.partita.clone();
            let v = p.vista();
            (p, v)
        };
        let (d, atto) = (v.ministeri.iter())
            .flat_map(|m| {
                m.atti
                    .iter()
                    .map(move |a| (m.dicastero.as_str(), a.id.as_str()))
            })
            .find(|&(d, a)| p.costo_dirotta(d, Some(a), None).is_ok())
            .ok_or("nessun atto dirottabile")?;
        let (dr, regione) = (v.ministeri.iter())
            .flat_map(|m| {
                v.sondaggio
                    .regioni
                    .iter()
                    .map(move |r| (m.dicastero.as_str(), r.id.as_str()))
            })
            .find(|&(d, r)| p.costo_dirotta(d, None, Some(r)).is_ok())
            .ok_or("nessuna regione dirottabile")?;
        for (d, a, r) in [(d, Some(atto), None), (dr, None, Some(regione))] {
            let atteso = p.costo_dirotta(d, a, r).map_err(|e| e.to_string())?;
            let letto = costo_del_dirotta(&stato, d, a, r).map_err(|e| e.testo)?;
            assert!(
                (atteso - letto).abs() < 1e-12,
                "{a:?} {r:?}: core {atteso}, guscio {letto}"
            );
        }
        // Scambiati, atto e regione non sono ammessi: il guscio non li può invertire.
        assert!(costo_del_dirotta(&stato, d, Some(regione), None).is_err());
        assert!(costo_del_dirotta(&stato, dr, None, Some(atto)).is_err());
        Ok(())
    }

    #[test]
    fn il_menu_con_valori_e_quello_del_core() -> Result<(), String> {
        let c = cartella("lib-menu-valori");
        let (stato, _archivio) = con_partita(&c.0)?;
        let valori = bozza_cittadinanza();
        let atteso = blocca_comunque(&stato)
            .as_ref()
            .ok_or(NESSUNA)?
            .partita
            .menu_elettorale(&valori);
        assert_eq!(menu_del_governo(&stato, &valori), Ok(atteso));
        Ok(())
    }

    #[test]
    fn l_anteprima_della_proposta_all_opposizione_e_quella_del_core() -> Result<(), String> {
        let stato = dopo_il_voto(7, 1.5, false)?;
        let valori = bozza_cittadinanza();
        let atteso = blocca_comunque(&stato)
            .as_ref()
            .ok_or(NESSUNA)?
            .partita
            .anteprima_proposta(&valori)
            .map_err(|e| e.to_string())?;
        assert_eq!(anteprima_della_proposta(&stato, &valori), Ok(atteso));
        Ok(())
    }

    #[test]
    fn il_menu_e_quello_del_core() -> Result<(), String> {
        let c = cartella("lib-menu");
        let (stato, _archivio) = con_partita(&c.0)?;
        let atteso = blocca_comunque(&stato)
            .as_ref()
            .ok_or(NESSUNA)?
            .partita
            .menu_elettorale(&[]);
        assert!(!atteso.is_empty());
        assert_eq!(menu_del_governo(&stato, &[]), Ok(atteso));
        Ok(())
    }

    #[test]
    fn l_anteprima_della_proposta_al_turno_zero_e_un_rifiuto() -> Result<(), String> {
        let c = cartella("lib-anteprima-proposta");
        let (stato, _archivio) = con_partita(&c.0)?;
        let atteso = blocca_comunque(&stato)
            .as_ref()
            .ok_or(NESSUNA)?
            .partita
            .anteprima_proposta(&bozza_cittadinanza())
            .err()
            .ok_or("proposta ammessa al turno zero")?;
        let e = anteprima_della_proposta(&stato, &bozza_cittadinanza())
            .err()
            .ok_or("anteprima data al turno zero")?;
        assert_eq!(e.specie, Specie::Rifiuto);
        assert_eq!(e.testo, atteso.to_string());
        Ok(())
    }

    #[test]
    fn anteprima_di_campagna_senza_partita_e_un_guasto() {
        let r = anteprima_di_campagna(&Stato::default(), &[]);
        assert_eq!(r.err(), Some(guasto(NESSUNA)));
    }

    #[test]
    fn elettori_senza_partita_e_un_guasto() {
        let r = elettori_del_gruppo(
            &Stato::default(),
            "piemonte",
            Condizione::Pensionato,
            Fascia::Media,
        );
        assert_eq!(r.err(), Some(guasto(NESSUNA)));
    }

    #[test]
    fn quattro_spese_sono_un_rifiuto() -> Result<(), String> {
        let c = cartella("lib-quattro-spese");
        let (stato, _archivio) = con_partita(&c.0)?;
        let spese: Vec<Spesa> = [
            Condizione::DipendentePrivato,
            Condizione::DipendentePubblico,
            Condizione::Autonomo,
            Condizione::Pensionato,
        ]
        .map(|c| spesa(Bersaglio::Condizione(c)))
        .into();
        let e = anteprima_di_campagna(&stato, &spese)
            .err()
            .ok_or("quattro spese accettate")?;
        assert_eq!(e.specie, Specie::Rifiuto);
        assert!(e.testo.contains("Al massimo 3 bersagli"), "{}", e.testo);
        Ok(())
    }

    #[test]
    fn una_spesa_da_la_sua_voce() -> Result<(), String> {
        let c = cartella("lib-una-spesa");
        let (stato, _archivio) = con_partita(&c.0)?;
        let s = spesa(Bersaglio::Condizione(Condizione::Pensionato));
        let voci = anteprima_di_campagna(&stato, std::slice::from_ref(&s))?;
        assert_eq!(voci.len(), 1);
        assert_eq!(
            (voci[0].bersaglio, voci[0].leva, voci[0].euro),
            (s.bersaglio, s.leva, s.euro)
        );
        Ok(())
    }

    #[test]
    fn regione_sconosciuta_e_un_rifiuto() -> Result<(), String> {
        let c = cartella("lib-regione-sconosciuta");
        let (stato, _archivio) = con_partita(&c.0)?;
        let e = elettori_del_gruppo(&stato, "atlantide", Condizione::Pensionato, Fascia::Media)
            .err()
            .ok_or("regione sconosciuta accettata")?;
        assert_eq!(e.specie, Specie::Rifiuto);
        Ok(())
    }

    #[test]
    fn tre_elettori_per_gruppo() -> Result<(), String> {
        let c = cartella("lib-tre-elettori");
        let (stato, _archivio) = con_partita(&c.0)?;
        let chiedi = || {
            elettori_del_gruppo(
                &stato,
                "piemonte",
                Condizione::DipendentePrivato,
                Fascia::Media,
            )
        };
        let primi = chiedi()?;
        assert_eq!(primi.len(), 3);
        assert_eq!(primi, chiedi()?);
        Ok(())
    }

    #[test]
    fn il_seme_avvelenato_non_blocca_l_anteprima() -> Result<(), String> {
        let seme = SemeFondazione::new(Some(SEME));
        let _ = std::thread::scope(|s| {
            s.spawn(|| {
                let _guardia = seme.lock();
                panic!("lock del seme avvelenato apposta");
            })
            .join()
        });
        assert!(seme.is_poisoned());
        anteprima_di_fondazione(&dati(), None, &seme)?;
        assert!(seme_per_fondare(&seme)? != SEME);
        Ok(())
    }

    #[test]
    fn il_seme_tenuto_resta_finche_non_si_azzera() -> Result<(), ErroreGuscio> {
        let seme = SemeFondazione::new(Some(SEME));
        assert_eq!(seme_per_fondare(&seme)?, SEME);
        assert_eq!(seme_per_fondare(&seme)?, SEME);
        let vuoto = SemeFondazione::default();
        // Dall'orologio: nanosecondi dall'epoca, ben oltre ogni seme scelto a mano.
        let dopo = seme_per_fondare(&vuoto)?;
        assert!(dopo > u64::from(u32::MAX), "{dopo}");
        Ok(())
    }

    #[test]
    fn l_anteprima_sovrascrive_il_seme_precedente() -> Result<(), String> {
        let seme = SemeFondazione::new(Some(SEME));
        anteprima_di_fondazione(&dati(), None, &seme)?;
        let tenuto = *seme.lock().map_err(|e| e.to_string())?;
        assert!(tenuto.is_some_and(|s| s != SEME), "{tenuto:?}");
        Ok(())
    }

    #[test]
    fn anteprima_incoerente_e_un_rifiuto_che_lascia_il_seme() -> Result<(), String> {
        let base = dati();
        let mut s = SuMisura::base(&base);
        s.cursori.debito_pil = 1e6;
        let seme = SemeFondazione::new(Some(SEME));
        let e = anteprima_di_fondazione(&base, Some(&s), &seme).err();
        assert_eq!(e.map(|e| e.specie), Some(Specie::Rifiuto));
        assert_eq!(*seme.lock().map_err(|e| e.to_string())?, Some(SEME));
        Ok(())
    }

    #[test]
    fn il_catalogo_dei_dati_di_base_si_costruisce() -> Result<(), String> {
        let c = catalogo(&dati()).map_err(rifiuto)?;
        assert_eq!(c.cursori.len(), 6);
        assert_eq!(c.legge_di_base, "legge_180");
        Ok(())
    }
}
