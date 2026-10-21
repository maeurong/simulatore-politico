#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

mod comune;
use comune::file_base;
use sim_core::dati::{DatiGioco, FileDati, Ruolo};
use sim_core::errore::Errore;

fn con_dicasteri(testo: &str) -> FileDati {
    FileDati {
        dicasteri: testo.into(),
        ..file_base()
    }
}

#[test]
fn dati_base_si_caricano() {
    let d = DatiGioco::da_testi(&file_base()).unwrap();
    assert_eq!(d.dicasteri.len(), 10);
    assert!(d.dicasteri.iter().any(|x| x.nome == "Sanità"));
    assert_eq!(
        d.dicasteri.iter().filter(|x| x.regione_bersaglio).count(),
        2
    );
    assert_eq!(d.manopole["soglia_minaccia"].valore, 40.0);
}

#[test]
fn ruolo_orfano_rifiutato_col_nome() {
    let testo = file_base()
        .dicasteri
        .replace("\"colpa_disoccupazione_trasferimenti\"", "");
    let e = DatiGioco::da_testi(&con_dicasteri(&testo)).unwrap_err();
    assert_eq!(
        e,
        Errore::RuoloOrfano {
            ruolo: Ruolo::ColpaDisoccupazioneTrasferimenti
        }
    );
    assert_eq!(
        e.to_string(),
        "ruolo senza dicastero: colpa_disoccupazione_trasferimenti"
    );
}

#[test]
fn ruolo_doppio_rifiutato_col_nome() {
    let testo = format!(
        "{}\n[[dicastero]]\nid = \"bis\"\nnome = \"Bis\"\nindicatore_servizio = \"x\"\npeso_spartizione = 1\nregione_bersaglio = false\nruoli = [\"bilancio_e_credibilita\"]\n",
        file_base().dicasteri
    );
    let e = DatiGioco::da_testi(&con_dicasteri(&testo)).unwrap_err();
    assert!(
        e.to_string()
            .starts_with("ruolo assegnato a due dicasteri: bilancio_e_credibilita")
    );
}

#[test]
fn indicatore_servizio_doppio_rifiutato() {
    let testo = format!(
        "{}\n[[dicastero]]\nid = \"bis\"\nnome = \"Bis\"\nindicatore_servizio = \"sicurezza\"\npeso_spartizione = 1\nregione_bersaglio = false\nruoli = []\n",
        file_base().dicasteri
    );
    let e = DatiGioco::da_testi(&con_dicasteri(&testo)).unwrap_err();
    assert!(matches!(
        &e,
        Errore::IndicatoreDoppio { indicatore, secondo, .. }
            if indicatore == "sicurezza" && secondo == "bis"
    ));
    assert!(
        e.to_string()
            .starts_with("indicatore di servizio assegnato a due dicasteri: sicurezza")
    );
}

#[test]
fn ruoli_tutti_coprono_ogni_variante() {
    // Il match senza `_` non compila se l'enum cresce: va aggiornato insieme a TUTTI.
    fn nella_lista(r: Ruolo) -> bool {
        match r {
            Ruolo::ColpaDisoccupazioneTrasferimenti
            | Ruolo::ColpaImposteImpiegoPubblico
            | Ruolo::BilancioECredibilita => Ruolo::TUTTI.contains(&r),
        }
    }
    for r in [
        Ruolo::ColpaDisoccupazioneTrasferimenti,
        Ruolo::ColpaImposteImpiegoPubblico,
        Ruolo::BilancioECredibilita,
    ] {
        assert!(nella_lista(r));
    }
}

#[test]
fn ruolo_sconosciuto_rifiutato_col_nome() {
    let testo = file_base()
        .dicasteri
        .replace("\"bilancio_e_credibilita\"", "\"bilancio_e_credibilta\"");
    let e = DatiGioco::da_testi(&con_dicasteri(&testo)).unwrap_err();
    assert!(matches!(e, Errore::Toml { .. }));
    assert!(e.to_string().contains("bilancio_e_credibilta"));
}

#[test]
fn dicastero_ripetuto_rifiutato() {
    let base = file_base().dicasteri;
    let primo = base.split("[[dicastero]]").nth(1).unwrap();
    let testo = format!("{base}\n[[dicastero]]{primo}");
    let e = DatiGioco::da_testi(&con_dicasteri(&testo)).unwrap_err();
    assert_eq!(
        e,
        Errore::DicasteroDoppio {
            id: "economia_finanze".into()
        }
    );
}

#[test]
fn manopola_fuori_intervallo_rifiutata_col_nome() {
    let manopole = file_base()
        .manopole
        .replacen("valore = 40.0", "valore = 90.0", 1);
    let e = DatiGioco::da_testi(&FileDati {
        manopole,
        ..file_base()
    })
    .unwrap_err();
    assert!(
        e.to_string()
            .starts_with("manopola fuori intervallo: soglia_minaccia = 90")
    );
}

#[test]
fn manopola_non_finita_rifiutata() {
    let manopole = file_base()
        .manopole
        .replacen("valore = 40.0", "valore = nan", 1);
    let e = DatiGioco::da_testi(&FileDati {
        manopole,
        ..file_base()
    })
    .unwrap_err();
    assert!(matches!(e, Errore::FuoriIntervallo { ref nome, .. } if nome == "soglia_minaccia"));
}

#[test]
fn manopola_con_massimo_infinito_rifiutata() {
    // valore dentro l'intervallo: a rifiutarla e' solo is_finite.
    let manopole = file_base().manopole.replacen("max = 50.0", "max = inf", 1);
    let e = DatiGioco::da_testi(&FileDati {
        manopole,
        ..file_base()
    })
    .unwrap_err();
    assert!(matches!(e, Errore::FuoriIntervallo { ref nome, .. } if nome == "soglia_minaccia"));
}

#[test]
fn manopola_con_intervallo_rovesciato_rifiutata() {
    let manopole = file_base().manopole.replacen("min = 30.0", "min = 60.0", 1);
    let e = DatiGioco::da_testi(&FileDati {
        manopole,
        ..file_base()
    })
    .unwrap_err();
    assert!(matches!(e, Errore::FuoriIntervallo { ref nome, .. } if nome == "soglia_minaccia"));
}
