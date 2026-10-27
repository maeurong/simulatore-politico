mod comune;
use comune::{file_base, presenta, sanita_cara};
use sim_core::dati::DatiGioco;
use sim_core::{Comando, Partita};

/// Partita con decreto (accogli), legge, referendum abrogativo e bilancio in aula: un comando per
/// turno, dal turno `da` al turno `a` escluso. Stessi comandi a ogni chiamata.
fn gioca(mut p: Partita, da: usize, a: usize) -> Partita {
    for t in da..a {
        match t {
            0 => p.chiudi_turno(),
            1 => {
                p.promuovi_referendum("sinistra", "licenziamento").unwrap();
                p.comanda(Comando::Accogli {
                    dicastero: "sanita".into(),
                })
                .unwrap();
                presenta(&mut p, &[("ticket", "alto")]).unwrap();
                p.chiudi_turno();
            }
            4 => {
                let bilancio = p.bozza_bilancio().unwrap();
                p.comanda(Comando::PresentaBilancio { bilancio }).unwrap();
                p.chiudi_turno();
            }
            _ => p.chiudi_turno(),
        }
    }
    p
}

fn nuova() -> Partita {
    Partita::nuova(DatiGioco::da_testi(&sanita_cara(file_base())).unwrap(), 7)
}

#[test]
fn stesso_seme_stessi_comandi_stessa_partita() {
    let p = gioca(nuova(), 0, 7);
    assert!(
        !p.vista().esiti_referendum.is_empty(),
        "referendum non votato"
    );
    let a = gioca(p, 7, 9).salva().unwrap();
    let b = gioca(nuova(), 0, 9).salva().unwrap();
    assert_eq!(a, b);
}

#[test]
fn salvare_e_ricaricare_a_meta_non_cambia_il_seguito() {
    let intera = gioca(nuova(), 0, 9).salva().unwrap();
    let meta = gioca(nuova(), 0, 5).salva().unwrap();
    let ripresa = gioca(Partita::carica(&meta).unwrap(), 5, 9);
    assert_eq!(ripresa.salva().unwrap(), intera);
}
