//! Gioca una partita senza finestra: chiude i turni senza decisioni e stampa la vista finale e lo stato vero di economia ed elettorato.

use std::path::Path;
use std::process::ExitCode;

use sim_core::legislatura::Fondazione;
use sim_core::{DatiGioco, FileDati, Partita};

mod tara;

struct Argomenti {
    dati: String,
    seme: u64,
    turni: u32,
    /// Con una posizione, la partita vera fondata lì; senza, la partita di prova dal governo uscente.
    posizione: Option<f64>,
}

fn leggi_argomenti(mut args: impl Iterator<Item = String>) -> Result<Argomenti, String> {
    let (mut dati, mut seme, mut turni, mut posizione) = (None, None, None, None);
    while let Some(nome) = args.next() {
        let valore = args
            .next()
            .ok_or_else(|| format!("manca il valore per {nome}"))?;
        match nome.as_str() {
            "--dati" => dati = Some(valore),
            "--seme" => {
                seme = Some(
                    valore
                        .parse()
                        .map_err(|_| format!("seme non valido: {valore}"))?,
                );
            }
            "--turni" => {
                turni = Some(
                    valore
                        .parse()
                        .map_err(|_| format!("numero di turni non valido: {valore}"))?,
                );
            }
            "--posizione" => {
                posizione = Some(
                    valore
                        .parse()
                        .map_err(|_| format!("posizione non valida: {valore}"))?,
                );
            }
            _ => return Err(format!("argomento sconosciuto: {nome}")),
        }
    }
    Ok(Argomenti {
        dati: dati.ok_or("manca --dati <cartella>")?,
        seme: seme.ok_or("manca --seme <u64>")?,
        turni: turni.ok_or("manca --turni <u32>")?,
        posizione,
    })
}

pub(crate) fn leggi(cartella: &Path, nome: &str) -> Result<String, String> {
    let percorso = cartella.join(nome);
    std::fs::read_to_string(&percorso)
        .map_err(|e| format!("impossibile leggere {}: {e}", percorso.display()))
}

fn esegui() -> Result<String, String> {
    let mut args = std::env::args().skip(1).peekable();
    if args.next_if(|a| a == "tara").is_some() {
        return tara::esegui(args);
    }
    let argomenti = leggi_argomenti(args)?;
    let cartella = Path::new(&argomenti.dati);
    let file = FileDati::leggi(|nome| leggi(cartella, nome))?;
    let dati = DatiGioco::da_testi(&file).map_err(|e| e.to_string())?;
    let mut partita = match argomenti.posizione {
        Some(posizione) => {
            let f = Fondazione {
                nome: "Partito del giocatore".into(),
                posizione,
            };
            Partita::fonda(dati, argomenti.seme, &f).map_err(|e| e.to_string())?
        }
        None => Partita::nuova(dati, argomenti.seme),
    };
    for _ in 0..argomenti.turni {
        partita.chiudi_turno();
    }
    let ids: Vec<&str> = partita.partiti().iter().map(|p| p.id.as_str()).collect();
    let uscita = serde_json::json!({
        "vista": partita.vista(),
        "economia": partita.economia(),
        "elettorato": { "partiti": ids, "nazionali": partita.intenzioni_vere().nazionali },
    });
    serde_json::to_string_pretty(&uscita).map_err(|e| e.to_string())
}

fn main() -> ExitCode {
    match esegui() {
        Ok(json) => {
            println!("{json}");
            ExitCode::SUCCESS
        }
        Err(messaggio) => {
            eprintln!("errore: {messaggio}");
            ExitCode::from(1)
        }
    }
}
