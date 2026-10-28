mod comune;
use comune::{dati_base, edita, fondata, presenta};
use sim_core::legislatura::Fine;
use sim_core::{Comando, Partita};

#[test]
fn abbandonare_produce_subito_l_eredita() {
    let mut p = Partita::nuova(dati_base(), 1);
    for _ in 0..4 {
        p.chiudi_turno();
    }
    let e = p.abbandona().unwrap();
    assert_eq!(e.fine, Fine::Abbandono);
    assert!(!e.bozza);
    assert!((e.anni_al_governo - 1.0).abs() < 1e-12);
    assert!(p.finita());
    assert_eq!(p.vista().eredita, Some(e));
    assert!(p.abbandona().is_err());
    assert!(
        p.comanda(Comando::Ferma {
            dicastero: "sanita".into()
        })
        .is_err()
    );
}

#[test]
fn le_leggi_del_giocatore_in_vigore() {
    let mut p = Partita::nuova(dati_base(), 1);
    presenta(&mut p, &[("tetto_contante", "1000")]).unwrap();
    let id = p.vista().calendario[0].id;
    p.comanda(Comando::PoniFiducia { misura: id }).unwrap();
    p.chiudi_turno();
    assert!(
        p.vista()
            .esiti
            .iter()
            .any(|e| e.misura == id && e.approvata)
    );
    let nome = p
        .vista()
        .disposizioni
        .iter()
        .find(|d| d.id == "tetto_contante")
        .unwrap()
        .nome
        .clone();
    assert_eq!(p.abbandona().unwrap().leggi_in_vigore, vec![nome]);
}

#[test]
fn eredita_di_chi_non_ha_mai_governato() {
    // Come `senza_seggi_la_partita_finisce` del 7a (tests/voto_turno_zero.rs).
    let f = comune::imposta(
        comune::file_base(),
        "manopole",
        "[valence_partito_nuovo]\nvalore = ",
        "-3.0",
    );
    let f = comune::imposta(f, "manopole", "[luna_di_miele]\nvalore = ", "0.0");
    let mut p = Partita::fonda(
        sim_core::dati::DatiGioco::da_testi(&f).unwrap(),
        7,
        &sim_core::legislatura::Fondazione {
            nome: "Partito nuovo".into(),
            posizione: 1.5,
        },
    )
    .unwrap();
    p.chiudi_turno();
    let e = p.eredita().unwrap();
    assert_eq!(e.fine, Fine::SenzaSeggi);
    assert!(e.bozza);
    assert_eq!(e.anni_al_governo, 0.0);
    assert!(e.leggi_in_vigore.is_empty());
    assert_eq!(e.indicatori.len(), 4);
    assert!(
        e.indicatori
            .iter()
            .all(|i| i.turno_zero.is_finite() && i.fine.is_finite())
    );
    assert!(p.abbandona().is_err());
}

/// Il confermativo di una legge del giocatore, approvato, la mette fra le sue leggi.
#[test]
fn il_confermativo_approvato_entra_nell_eredita() {
    let dati = dati_base();
    let k = dati.disposizione("salario_minimo").unwrap();
    let g = dati.disposizioni()[k].gradino("7").unwrap();
    let mut p = edita(&Partita::nuova(dati_base(), 1), |j| {
        j["parlamento"]["confermativi"] = serde_json::json!([{
            "tipo": "confermativo",
            "misura": 99,
            "valori": [[k, g]],
            "promotore": null,
            "del_giocatore": true,
            "voto": 0,
            "firme": true,
            "invito_astensione": false,
        }]);
    })
    .unwrap();
    p.chiudi_turno();
    assert!(p.vista().esiti_referendum[0].approvato);
    let nome = dati.disposizioni()[k].nome.clone();
    assert_eq!(p.abbandona().unwrap().leggi_in_vigore, vec![nome]);
}

#[test]
fn i_turni_senza_governo_non_contano_negli_anni() {
    let mut p = fondata(1.5);
    for _ in 0..3 {
        p.chiudi_turno(); // Camere sciolte, trattativa, opposizione
    }
    assert_eq!(p.abbandona().unwrap().anni_al_governo, 0.0);
}

/// Scelta 17: contano i turni da Presidente del Consiglio, anche l'interim dopo le dimissioni.
#[test]
fn l_interim_conta_negli_anni_al_governo() {
    let mut p = Partita::nuova(dati_base(), 1);
    p.comanda(Comando::Dimettiti).unwrap();
    p.chiudi_turno();
    assert!(!p.governa());
    assert_eq!(p.governo().presidente, p.giocatore());
    p.chiudi_turno();
    assert!((p.abbandona().unwrap().anni_al_governo - 0.5).abs() < 1e-12);
}

#[test]
fn salvataggi_editati_rifiutati_col_motivo() {
    let p = Partita::nuova(dati_base(), 1);
    let rifiutato = |f: &dyn Fn(&mut serde_json::Value), frammento: &str| {
        let e = edita(&p, f).err().unwrap().to_string();
        assert!(e.contains(frammento), "{e}");
    };
    rifiutato(&|j| j["turni_al_governo"] = 2.into(), "turni_al_governo");
    rifiutato(
        &|j| j["parlamento"]["sfiducia"] = serde_json::json!([99, 0]),
        "la sfiducia fuori dai dicasteri",
    );
    rifiutato(
        &|j| j["economia"]["pil_trimestrale"] = 0.0.into(),
        "pil_trimestrale",
    );
}

#[test]
fn al_turno_zero_gli_indicatori_non_cambiano() {
    let mut p = Partita::nuova(dati_base(), 1);
    let e = p.abbandona().unwrap();
    assert!(
        e.indicatori.iter().all(|i| i.turno_zero == i.fine),
        "{:?}",
        e.indicatori
    );
}
