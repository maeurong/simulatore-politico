#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

mod comune;
use comune::{
    chiudi_col_bilancio, dati_base, ferma_tutti, file_base, imposta, riscrivi, senza_shock,
    sostituisci,
};
use sim_core::caso::Caso;
use sim_core::dati::DatiGioco;
use sim_core::economia::Economia;
use sim_core::elettorato::{Condizione, condizione_di};
use sim_core::grandezze::Grandezze;
use sim_core::{Errore, Partita};

#[test]
fn grandezze_base_si_caricano() {
    let p = Partita::nuova(dati_base(), 1);
    let v = p.vista();
    assert_eq!(v.grandezze.len(), 21);
    let stock = v
        .grandezze
        .iter()
        .find(|g| g.id == "stock_infrastrutture")
        .unwrap();
    assert_eq!(stock.valori, vec![100.0; 20]);
    assert_eq!(p.grandezza("qualita_servizio_sanitario"), Some(100.0));
}

#[test]
fn indicatore_di_servizio_senza_grandezza_rifiutato_col_nome() {
    let f = sostituisci(
        file_base(),
        "grandezze",
        "id = \"qualita_scuola\"",
        "id = \"qualita_scuole\"",
    );
    let e = DatiGioco::da_testi(&f).unwrap_err();
    assert!(matches!(e, Errore::Grandezze(_)));
    assert!(e.to_string().contains("qualita_scuola"), "{e}");
}

#[test]
fn grandezza_richiesta_di_forma_sbagliata_rifiutata() {
    let f = sostituisci(
        file_base(),
        "grandezze",
        "regionale = true",
        "regionale = false",
    );
    let e = DatiGioco::da_testi(&f).unwrap_err();
    assert!(e.to_string().contains("stock_infrastrutture"), "{e}");
}

#[test]
fn grandezza_regionale_di_un_dicastero_senza_bersaglio_rifiutata() {
    let f = sostituisci(
        file_base(),
        "grandezze",
        "dicasteri = [\"infrastrutture_trasporti\"]\n\n[[grandezza]]\nid = \"investimenti_privati\"",
        "dicasteri = [\"sanita\"]\n\n[[grandezza]]\nid = \"investimenti_privati\"",
    );
    let e = DatiGioco::da_testi(&f).unwrap_err();
    assert!(e.to_string().contains("sanita"), "{e}");
}

#[test]
fn servizio_segue_il_funzionamento_reale() {
    let fermo = imposta(
        senza_shock(),
        "manopole",
        "[velocita_servizio]\nvalore = ",
        "0.0",
    );
    let mut a = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
    let mut b = Partita::nuova(DatiGioco::da_testi(&fermo).unwrap(), 1);
    let mut serie = Vec::new();
    for _ in 0..10 {
        ferma_tutti(&mut a);
        ferma_tutti(&mut b);
        a.chiudi_turno();
        b.chiudi_turno();
        serie.push(a.grandezza("qualita_servizio_sanitario").unwrap());
        assert_eq!(b.grandezza("qualita_servizio_sanitario"), Some(100.0));
    }
    // Dal 2028 il bilancio resta quello nominale del 2027: l'inflazione lo erode.
    assert!(serie[9] < serie[5], "{serie:?}");
}

#[test]
fn servizio_in_calo_pesa_sul_partito_del_dicastero() {
    let base = imposta(senza_shock(), "scenario", "sanita = ", "\"centrodestra\"");
    let fermo = imposta(
        base.clone(),
        "manopole",
        "[velocita_servizio]\nvalore = ",
        "0.0",
    );
    let giudizio = |f| {
        let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
        for _ in 0..10 {
            ferma_tutti(&mut p);
            p.chiudi_turno();
        }
        let j = p
            .partiti()
            .iter()
            .position(|x| x.id == "centrodestra")
            .unwrap();
        let g = &p.elettorato().gruppi;
        let pensionati: Vec<f64> = (0..g.len())
            .filter(|&i| condizione_di(i) == Condizione::Pensionato)
            .map(|i| g[i].giudizio[j])
            .collect();
        pensionati.iter().sum::<f64>() / pensionati.len() as f64
    };
    assert!(giudizio(base) < giudizio(fermo));
}

#[test]
fn indicatori_pubblicati_con_un_turno_di_ritardo() {
    let mut p = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
    for _ in 0..3 {
        p.chiudi_turno();
    }
    let g = p.grandezze();
    let v = p.vista();
    for (i, voce) in v.grandezze.iter().enumerate() {
        assert_eq!(voce.valori, g.prima[i]);
    }
    assert!(
        g.valori != g.prima,
        "nessuna grandezza si è mossa nell'ultimo turno"
    );
}

#[test]
fn salvataggio_con_grandezze_corte_illeggibile() {
    let mut p = Partita::nuova(dati_base(), 1);
    p.chiudi_turno();
    let mut json: serde_json::Value = serde_json::from_str(&p.salva().unwrap()).unwrap();
    json["grandezze"]["valori"][11] = serde_json::json!([100.0]);
    let e = Partita::carica(&json.to_string()).unwrap_err();
    assert!(matches!(e, Errore::SalvataggioIlleggibile(_)), "{e}");
}

#[test]
fn grandezze_restano_nei_limiti_dopo_200_turni() {
    let dati = dati_base();
    let mut p = Partita::nuova(dati.clone(), 9);
    for _ in 0..200 {
        p.chiudi_turno();
    }
    for (d, riga) in dati.grandezze().iter().zip(&p.grandezze().valori) {
        assert!(
            riga.iter()
                .all(|x| x.is_finite() && (d.min..=d.max).contains(x)),
            "{}",
            d.id
        );
    }
    p.salva().unwrap();
}

#[test]
fn le_carceri_crescono_da_sole() {
    let mut p = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
    for _ in 0..20 {
        chiudi_col_bilancio(&mut p);
    }
    let v = p.grandezza("sovraffollamento_carceri").unwrap();
    // 1,009^20 ~ 1,196 (senza rumore, quindi la banda non dipende da dove cade il flusso degli
    // eventi, che le emergenze consumano).
    let atteso = 100.0 * 1.009_f64.powi(20);
    assert!((v - atteso).abs() < 1.0, "{v} contro {atteso}");
}

#[test]
fn gli_sbarchi_tornano_verso_cento() {
    let dati = dati_base();
    let g = dati.grandezza("immigrazione_irregolare").unwrap();
    let e = Economia::iniziale(&dati);
    let mut finali: Vec<f64> = (0..200)
        .map(|seme| {
            let (mut gr, mut caso) = (Grandezze::iniziali(&dati), Caso::nuovo(seme));
            for turno in 0..40 {
                gr.avanza(&dati, Some(&mut caso), &e, turno);
            }
            gr.nazionale(&dati, g)
        })
        .collect();
    finali.sort_by(f64::total_cmp);
    let mediana = finali[100];
    assert!(mediana > 60.0 && mediana < 160.0, "{mediana}");
}

#[test]
fn senza_caso_la_deriva_non_ha_rumore() {
    let dati = dati_base();
    let g = dati.grandezza("sovraffollamento_carceri").unwrap();
    let e = Economia::iniziale(&dati);
    let mut gr = Grandezze::iniziali(&dati);
    for turno in 0..4 {
        gr.avanza(&dati, None, &e, turno);
    }
    let v = gr.nazionale(&dati, g);
    assert!((v - 100.0 * 1.009_f64.powi(4)).abs() < 1e-9, "{v}");
}

/// Righe `[[deriva]]` con tasso e sigma a 0 contro nessuna riga: le estrazioni degli Eventi
/// avvengono comunque, ma il sondaggio (flusso Sondaggi) non cambia.
#[test]
fn la_deriva_non_tocca_gli_altri_flussi() {
    let con = riscrivi(
        riscrivi(file_base(), "grandezze", "tasso = ", "tasso = 0.0"),
        "grandezze",
        "sigma = ",
        "sigma = 0.0",
    );
    let mut senza = file_base();
    let fine = senza.grandezze.find("[[deriva]]").unwrap();
    senza.grandezze = format!("deriva = []\n{}", &senza.grandezze[..fine]);
    let sondaggio = |f: &_| {
        let mut p = Partita::nuova(DatiGioco::da_testi(f).unwrap(), 3);
        chiudi_col_bilancio(&mut p);
        serde_json::to_string(&p.vista().sondaggio).unwrap()
    };
    assert_eq!(sondaggio(&con), sondaggio(&senza));
}

#[test]
fn una_deriva_su_una_grandezza_non_fissa_e_rifiutata_col_nome() {
    let f = format!(
        "{}\n[[deriva]]\ngrandezza = \"qualita_scuola\"\ntasso = 0.0\nsigma = 0.0\nritorno = 0.0\nlogaritmica = false\n",
        file_base().grandezze
    );
    let mut file = file_base();
    file.grandezze = f;
    let e = DatiGioco::da_testi(&file).unwrap_err();
    assert!(matches!(e, Errore::Grandezze(_)));
    assert!(e.to_string().contains("qualita_scuola"), "{e}");
}

#[test]
fn una_deriva_con_sigma_negativa_o_ritorno_fuori_da_zero_uno_e_rifiutata() {
    for (da, a) in [
        ("sigma = 0.0075", "sigma = -0.1"),
        ("ritorno = 0.2", "ritorno = 1.5"),
    ] {
        let f = sostituisci(file_base(), "grandezze", da, a);
        let e = DatiGioco::da_testi(&f).unwrap_err();
        assert!(matches!(e, Errore::Grandezze(_)), "{e}");
    }
}

#[test]
fn una_deriva_logaritmica_su_un_minimo_non_positivo_e_rifiutata_col_nome() {
    let f = sostituisci(
        file_base(),
        "grandezze",
        "min = 1.0\nmax = 600.0",
        "min = 0.0\nmax = 600.0",
    );
    let e = DatiGioco::da_testi(&f).unwrap_err();
    assert!(
        matches!(&e, Errore::Grandezze(m) if m.contains("immigrazione_irregolare")
            && m.contains("logaritmica")),
        "{e}"
    );
}
