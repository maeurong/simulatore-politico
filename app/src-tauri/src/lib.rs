//! Guscio Tauri: legge i file di dati, sceglie il seme dall'orologio, tiene la partita in corso,
//! la salva nell'archivio e passa la `Vista` alla UI.

mod archivio;

use std::path::Path;
use std::sync::{Mutex, MutexGuard, PoisonError};
use std::time::{SystemTime, UNIX_EPOCH};

use archivio::Archivio;
use sim_core::legislatura::{Eredita, Fine, Fondazione};
use sim_core::parlamento::{Decreto, ValoreProposto};
use sim_core::riepilogo::{RiepilogoScenario, riepilogo};
use sim_core::salvataggi::{VoceAlbo, VocePartita};
use sim_core::vista::Anteprima;
use sim_core::{Comando, DatiGioco, FileDati, Partita, SuMisura, Vista};
use tauri::{Manager, RunEvent, State};

/// La partita in corso; `id` è `None` per la partita di prova, che non si salva.
struct InCorso {
    id: Option<String>,
    partita: Partita,
}

/// Ogni chiamata all'`Archivio` avviene tenendo questo `Mutex`: bozza, snapshot e uscita non si
/// sovrappongono.
type Stato = Mutex<Option<InCorso>>;

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

fn blocca(stato: &Stato) -> Result<MutexGuard<'_, Option<InCorso>>, String> {
    stato.lock().map_err(|_| ERRORE_STATO.to_owned())
}

/// Il lock anche avvelenato, per chi tocca solo il disco o sostituisce lo stato per intero.
fn blocca_comunque(stato: &Stato) -> MutexGuard<'_, Option<InCorso>> {
    stato.lock().unwrap_or_else(PoisonError::into_inner)
}

/// I dati di base, o una loro copia con lo scenario su misura.
fn dati_di(base: &DatiGioco, scenario: Option<&SuMisura>) -> Result<DatiGioco, String> {
    match scenario {
        Some(s) => base.con_su_misura(s).map_err(|e| e.to_string()),
        None => Ok(base.clone()),
    }
}

/// La partita di prova, senza salvataggi.
fn avvia(stato: &Stato, dati: DatiGioco, seme: u64) -> Result<Vista, String> {
    let partita = Partita::nuova(dati, seme);
    let vista = partita.vista();
    *blocca_comunque(stato) = Some(InCorso { id: None, partita });
    stato.clear_poison();
    Ok(vista)
}

/// La partita vera, salvata con l'id del seme in esadecimale.
fn fonda(
    stato: &Stato,
    archivio: &Archivio,
    base: &DatiGioco,
    scenario: Option<&SuMisura>,
    fondazione: &Fondazione,
    seme: u64,
) -> Result<Vista, String> {
    let partita =
        Partita::fonda(dati_di(base, scenario)?, seme, fondazione).map_err(|e| e.to_string())?;
    let id = format!("{seme:016x}");
    let vista = partita.vista();
    let mut guardia = blocca_comunque(stato);
    archivio.crea(&partita, &id)?;
    *guardia = Some(InCorso {
        id: Some(id),
        partita,
    });
    stato.clear_poison();
    Ok(vista)
}

fn apri_partita(stato: &Stato, archivio: &Archivio, id: String) -> Result<Vista, String> {
    let mut guardia = blocca_comunque(stato);
    let partita = archivio.riprendi(&id)?;
    let vista = partita.vista();
    *guardia = Some(InCorso {
        id: Some(id),
        partita,
    });
    stato.clear_poison();
    Ok(vista)
}

/// Chiude il turno; con un id, lo snapshot della nuova apertura. Lo snapshot si scrive anche se la
/// chiusura finisce la partita; se era già finita, no. Se lo snapshot non si scrive, errore e
/// partita in memoria invariata. Il disco può restare un turno avanti, se il guasto arriva dopo
/// il file del turno: la chiusura ripetuta lo riscrive.
fn chiudi(stato: &Stato, archivio: &Archivio) -> Result<Vista, String> {
    let mut guardia = blocca(stato)?;
    let c = guardia.as_mut().ok_or(NESSUNA)?;
    let gia_finita = c.partita.finita();
    let mut nuova = c.partita.clone();
    nuova.chiudi_turno();
    if let Some(id) = &c.id
        && !gia_finita
    {
        archivio.snapshot(id, &nuova)?;
    }
    c.partita = nuova;
    Ok(c.partita.vista())
}

/// Il comando; con un id, la bozza se è accettato. Un comando rifiutato non la scrive. Se la
/// bozza non si scrive, errore e partita in memoria invariata.
fn esegui(stato: &Stato, archivio: &Archivio, comando: Comando) -> Result<Vista, String> {
    let mut guardia = blocca(stato)?;
    let c = guardia.as_mut().ok_or(NESSUNA)?;
    let mut nuova = c.partita.clone();
    nuova.comanda(comando).map_err(|e| e.to_string())?;
    if let Some(id) = &c.id {
        archivio.bozza(id, &nuova)?;
    }
    c.partita = nuova;
    Ok(c.partita.vista())
}

/// Se il riavvolgimento fallisce, la partita in memoria torna quella su disco, se si riapre.
fn torna_a(stato: &Stato, archivio: &Archivio, turno: u32) -> Result<Vista, String> {
    let mut guardia = blocca(stato)?;
    let c = guardia.as_mut().ok_or(NESSUNA)?;
    let id =
        c.id.as_deref()
            .ok_or("la partita di prova non ha salvataggi a cui tornare")?;
    match archivio.riavvolgi(id, turno) {
        Ok(p) => c.partita = p,
        Err(e) => {
            if let Ok(p) = archivio.riprendi(id) {
                c.partita = p;
            }
            return Err(e);
        }
    }
    Ok(c.partita.vista())
}

/// La voce dell'albo della partita finita, poi la sua cartella cancellata.
fn nell_albo(archivio: &Archivio, id: &str, partita: &Partita) -> Result<(), String> {
    let voce = partita
        .voce_albo()
        .ok_or("nessuna voce d'albo: la partita non è finita")?;
    archivio.chiudi_nell_albo(id, &voce)
}

/// L'abbandono: l'eredità subito definitiva, nell'albo con un id, poi lo stato vuoto. Ripetibile
/// se l'albo non si scrive: la partita già abbandonata non si abbandona di nuovo.
fn lascia(stato: &Stato, archivio: &Archivio) -> Result<Eredita, String> {
    let mut guardia = blocca(stato)?;
    let c = guardia.as_mut().ok_or(NESSUNA)?;
    let eredita = match c.partita.eredita() {
        Some(e) if e.fine == Fine::Abbandono => e,
        Some(_) => return Err(SENZA_SEGGI.into()),
        None => c.partita.abbandona().map_err(|e| e.to_string())?,
    };
    if let Some(id) = &c.id {
        nell_albo(archivio, id, &c.partita)?;
    }
    *guardia = None;
    stato.clear_poison();
    Ok(eredita)
}

/// Conferma l'eredità in bozza della partita finita senza seggi: nell'albo con un id, poi lo
/// stato vuoto.
fn conferma(stato: &Stato, archivio: &Archivio) -> Result<(), String> {
    let mut guardia = blocca(stato)?;
    let c = guardia.as_ref().ok_or(NESSUNA)?;
    if !c.partita.eredita().is_some_and(|e| e.bozza) {
        return Err("nessuna eredità in bozza da confermare".into());
    }
    if let Some(id) = &c.id {
        nell_albo(archivio, id, &c.partita)?;
    }
    *guardia = None;
    stato.clear_poison();
    Ok(())
}

/// All'uscita, la bozza della partita in corso, se ha un id e non è finita. Col lock avvelenato
/// niente bozza: dopo un panic lo stato può essere a metà, e ogni comando accettato ha già scritto
/// la sua. Un errore non blocca l'uscita.
fn alla_chiusura(stato: &Stato, archivio: &Archivio) {
    let Ok(guardia) = stato.lock() else {
        eprintln!(
            "bozza non salvata all'uscita: lo stato della partita è rimasto a metà dopo un errore interno"
        );
        return;
    };
    if let Some(InCorso {
        id: Some(id),
        partita,
    }) = guardia.as_ref()
        && !partita.finita()
        && let Err(e) = archivio.bozza(id, partita)
    {
        eprintln!("bozza non salvata all'uscita: {e}");
    }
}

// async: il comando gira fuori dal thread principale della finestra, che resta reattiva mentre il motore chiude il turno.
#[tauri::command]
async fn nuova_partita(
    base: State<'_, DatiGioco>,
    stato: State<'_, Stato>,
) -> Result<Vista, String> {
    avvia(&stato, base.inner().clone(), seme_da_orologio()?)
}

// async: il comando gira fuori dal thread principale della finestra, che resta reattiva mentre il motore chiude il turno.
#[tauri::command]
async fn chiudi_turno(
    stato: State<'_, Stato>,
    archivio: State<'_, Archivio>,
) -> Result<Vista, String> {
    chiudi(&stato, &archivio)
}

#[tauri::command]
async fn partite(
    stato: State<'_, Stato>,
    archivio: State<'_, Archivio>,
) -> Result<Vec<VocePartita>, String> {
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
) -> Result<Vista, String> {
    let seme = seme_da_orologio()?;
    fonda(
        &stato,
        &archivio,
        &base,
        scenario.as_ref(),
        &fondazione,
        seme,
    )
}

#[tauri::command]
async fn riprendi(
    id: String,
    stato: State<'_, Stato>,
    archivio: State<'_, Archivio>,
) -> Result<Vista, String> {
    apri_partita(&stato, &archivio, id)
}

#[tauri::command]
async fn comanda(
    comando: Comando,
    stato: State<'_, Stato>,
    archivio: State<'_, Archivio>,
) -> Result<Vista, String> {
    esegui(&stato, &archivio, comando)
}

#[tauri::command]
async fn riavvolgi(
    turno: u32,
    stato: State<'_, Stato>,
    archivio: State<'_, Archivio>,
) -> Result<Vista, String> {
    torna_a(&stato, &archivio, turno)
}

#[tauri::command]
async fn abbandona(
    stato: State<'_, Stato>,
    archivio: State<'_, Archivio>,
) -> Result<Eredita, String> {
    lascia(&stato, &archivio)
}

#[tauri::command]
async fn conferma_eredita(
    stato: State<'_, Stato>,
    archivio: State<'_, Archivio>,
) -> Result<(), String> {
    conferma(&stato, &archivio)
}

#[tauri::command]
async fn albo(
    stato: State<'_, Stato>,
    archivio: State<'_, Archivio>,
) -> Result<Vec<VoceAlbo>, String> {
    let _guardia = blocca_comunque(&stato);
    Ok(archivio.albo())
}

#[tauri::command]
async fn scenari(
    stato: State<'_, Stato>,
    archivio: State<'_, Archivio>,
) -> Result<Vec<String>, String> {
    let _guardia = blocca_comunque(&stato);
    Ok(archivio.scenari())
}

#[tauri::command]
async fn salva_scenario(
    scenario: SuMisura,
    base: State<'_, DatiGioco>,
    stato: State<'_, Stato>,
    archivio: State<'_, Archivio>,
) -> Result<(), String> {
    let _guardia = blocca_comunque(&stato);
    archivio.salva_scenario(&scenario, &base)
}

#[tauri::command]
async fn carica_scenario(
    nome: String,
    base: State<'_, DatiGioco>,
    stato: State<'_, Stato>,
    archivio: State<'_, Archivio>,
) -> Result<SuMisura, String> {
    let _guardia = blocca_comunque(&stato);
    archivio.carica_scenario(&nome, &base)
}

// async: sei partite costruite, una per posizione, fuori dal thread principale.
#[tauri::command]
async fn riepilogo_scenario(
    scenario: Option<SuMisura>,
    base: State<'_, DatiGioco>,
) -> Result<RiepilogoScenario, String> {
    Ok(riepilogo(&dati_di(&base, scenario.as_ref())?))
}

/// Esegue `f` sulla partita in corso, in sola lettura.
fn sulla_partita<T>(
    stato: &Stato,
    f: impl FnOnce(&Partita) -> Result<T, sim_core::Errore>,
) -> Result<T, String> {
    let guardia = blocca(stato)?;
    let c = guardia.as_ref().ok_or(NESSUNA)?;
    f(&c.partita).map_err(|e| e.to_string())
}

// async: le anteprime prendono il Mutex che tiene `chiudi_turno`; fuori dal thread principale non lo bloccano.
#[tauri::command]
async fn anteprima(
    valori: Vec<ValoreProposto>,
    stato: State<'_, Stato>,
) -> Result<Anteprima, String> {
    sulla_partita(&stato, |p| p.anteprima(&valori))
}

#[tauri::command]
async fn anteprima_decreto(decreto: Decreto, stato: State<'_, Stato>) -> Result<Anteprima, String> {
    sulla_partita(&stato, |p| p.anteprima_decreto(&decreto))
}

#[tauri::command]
async fn anteprima_accogli(
    dicastero: String,
    stato: State<'_, Stato>,
) -> Result<Anteprima, String> {
    sulla_partita(&stato, |p| p.anteprima_accogli(&dicastero))
}

pub fn run() -> tauri::Result<()> {
    tauri::Builder::default()
        .manage(Stato::default())
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
            nuova_partita,
            chiudi_turno,
            partite,
            fonda_partita,
            riprendi,
            comanda,
            riavvolgi,
            abbandona,
            conferma_eredita,
            albo,
            scenari,
            salva_scenario,
            carica_scenario,
            riepilogo_scenario,
            anteprima,
            anteprima_decreto,
            anteprima_accogli
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
    fn lock_avvelenato_ripulito_da_una_partita_nuova() -> Result<(), String> {
        let c = cartella("lib-lock-avvelenato");
        let (stato, archivio) = con_partita(&c.0)?;
        avvelena(&stato);
        assert_eq!(chiudi(&stato, &archivio), Err(ERRORE_STATO.to_string()));
        // I comandi che toccano solo il disco prendono il lock anche avvelenato.
        assert!(blocca_comunque(&stato).is_some());

        avvia(&stato, dati(), 1)?;
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

        let e = torna_a(&stato, &archivio, 0).err().unwrap_or_default();
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
            .unwrap_or_default();
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

        let e = chiudi(&stato, &archivio).err().unwrap_or_default();
        assert!(e.starts_with("impossibile cancellare"), "{e}");
        assert!(guasto.with_file_name("turno-001.json.gz").is_file());
        assert_eq!(sulla_partita(&stato, |p| Ok(p.vista()))?.turno, 0);

        std::fs::remove_dir_all(&guasto).map_err(|e| e.to_string())?;
        assert_eq!(chiudi(&stato, &archivio)?.turno, 1);
        Ok(())
    }

    #[test]
    fn chiudi_senza_partita_da_errore() -> Result<(), String> {
        let c = cartella("lib-chiudi-senza-partita");
        let archivio = Archivio::nuovo(c.0.clone())?;
        let stato = Stato::default();
        assert_eq!(chiudi(&stato, &archivio), Err(NESSUNA.to_string()));
        Ok(())
    }

    #[test]
    fn anteprima_senza_partita_da_errore() {
        let stato = Stato::default();
        let r = sulla_partita(&stato, |p| p.anteprima_accogli("sanita"));
        assert_eq!(r.err(), Some(NESSUNA.to_string()));
    }

    #[test]
    fn anteprima_rifiutata_porta_il_nome_del_dicastero() -> Result<(), String> {
        let stato = Stato::default();
        avvia(&stato, dati(), 1)?;
        let errore = sulla_partita(&stato, |p| p.anteprima_accogli("giustizia"))
            .err()
            .unwrap_or_default();
        assert!(errore.contains("Giustizia"), "{errore}");
        Ok(())
    }

    #[test]
    fn avvia_e_chiudi_avanzano_il_turno() -> Result<(), String> {
        let c = cartella("lib-avvia-e-chiudi");
        let archivio = Archivio::nuovo(c.0.clone())?;
        let stato = Stato::default();
        assert_eq!(avvia(&stato, dati(), 1)?.turno, 0);
        let vista = chiudi(&stato, &archivio)?;
        assert_eq!(
            (vista.turno, vista.data.trimestre, vista.data.anno),
            (1, 1, 2027)
        );
        assert_eq!(vista.dicasteri.len(), 10);
        // La partita di prova non si salva e non si riavvolge.
        assert!(archivio.elenca().is_empty());
        let e = torna_a(&stato, &archivio, 0).err().unwrap_or_default();
        assert!(e.contains("partita di prova"), "{e}");
        Ok(())
    }

    #[test]
    fn cartella_mancante_nomina_il_percorso() {
        let errore = carica_dati(Path::new("/non/esiste"))
            .err()
            .unwrap_or_default();
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
            .unwrap_or_default();
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
            .unwrap_or_default();
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
        let e = conferma(&stato, &archivio).err().unwrap_or_default();
        assert_eq!(e, "nessuna eredità in bozza da confermare");

        let eredita = lascia(&stato, &archivio)?;
        assert_eq!((eredita.fine, eredita.bozza), (Fine::Abbandono, false));
        assert_eq!(archivio.albo().len(), 1);
        assert!(archivio.elenca().is_empty());
        assert!(blocca(&stato)?.is_none());
        assert_eq!(conferma(&stato, &archivio), Err(NESSUNA.to_string()));

        // La partita di prova si abbandona senza albo.
        avvia(&stato, dati(), 1)?;
        lascia(&stato, &archivio)?;
        assert_eq!(archivio.albo().len(), 1);
        assert!(blocca(&stato)?.is_none());
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
        assert_eq!(lascia(&stato, &archivio), Err(SENZA_SEGGI.to_string()));
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
        let e = conferma(&stato, &archivio).err().unwrap_or_default();
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

        // La partita di prova non si salva.
        let stato = Stato::default();
        let archivio = Archivio::nuovo(c.0.clone())?;
        avvia(&stato, dati(), 1)?;
        alla_chiusura(&stato, &archivio);
        assert!(archivio.elenca().is_empty());

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
}
