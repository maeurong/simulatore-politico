//! Guscio Tauri: legge i file di dati, sceglie il seme dall'orologio e passa la `Vista` alla UI.

use std::path::Path;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use sim_core::{DatiGioco, FileDati, Partita, Vista};
use tauri::{AppHandle, Manager, State};

type Stato = Mutex<Option<Partita>>;

const ERRORE_STATO: &str = "stato della partita non disponibile";

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

fn avvia(stato: &Stato, dati: DatiGioco, seme: u64) -> Result<Vista, String> {
    let partita = Partita::nuova(dati, seme);
    let vista = partita.vista();
    // Lo stato viene sostituito per intero: un lock avvelenato si recupera.
    *stato
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(partita);
    Ok(vista)
}

fn chiudi(stato: &Stato) -> Result<Vista, String> {
    let mut guardia = stato.lock().map_err(|_| ERRORE_STATO)?;
    let partita = guardia.as_mut().ok_or("nessuna partita in corso")?;
    partita.chiudi_turno();
    Ok(partita.vista())
}

#[tauri::command]
async fn nuova_partita(app: AppHandle, stato: State<'_, Stato>) -> Result<Vista, String> {
    let risorse = app
        .path()
        .resource_dir()
        .map_err(|e| format!("cartella delle risorse non trovata: {e}"))?;
    let dati = carica_dati(&risorse.join("dati/base"))?;
    avvia(&stato, dati, seme_da_orologio()?)
}

#[tauri::command]
async fn chiudi_turno(stato: State<'_, Stato>) -> Result<Vista, String> {
    chiudi(&stato)
}

pub fn run() -> tauri::Result<()> {
    tauri::Builder::default()
        .manage(Stato::default())
        .invoke_handler(tauri::generate_handler![nuova_partita, chiudi_turno])
        .run(tauri::generate_context!())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dati_base() -> Result<DatiGioco, String> {
        carica_dati(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../dati/base"))
    }

    #[test]
    fn chiudi_senza_partita_da_errore() {
        let stato = Stato::default();
        assert_eq!(chiudi(&stato), Err("nessuna partita in corso".to_string()));
    }

    #[test]
    fn avvia_e_chiudi_avanzano_il_turno() -> Result<(), String> {
        let stato = Stato::default();
        assert_eq!(avvia(&stato, dati_base()?, 1)?.turno, 0);
        let vista = chiudi(&stato)?;
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
            .unwrap_or_default();
        assert!(errore.contains("/non/esiste/dicasteri.toml"), "{errore}");
    }
}
