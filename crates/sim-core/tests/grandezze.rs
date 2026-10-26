#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

mod comune;
use comune::{dati_base, file_base, imposta, senza_shock, sostituisci};
use sim_core::dati::DatiGioco;
use sim_core::elettorato::{Condizione, condizione_di};
use sim_core::{Errore, Partita};

#[test]
fn grandezze_base_si_caricano() {
    let p = Partita::nuova(dati_base(), 1);
    let v = p.vista();
    assert_eq!(v.grandezze.len(), 20);
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
