#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

mod comune;
use sim_core::su_misura::SuMisura;

#[test]
fn la_base_non_cambia_i_dati() {
    let base = comune::dati_base();
    let d = base.con_su_misura(&SuMisura::base(&base)).unwrap();
    assert_eq!(d.scenario(), base.scenario());
    assert_eq!(d.nome_scenario(), "Italia di oggi");
    assert_eq!(base.nome_scenario(), "Italia di oggi");
    assert!(base.su_misura().is_none());
    assert_eq!(d.su_misura(), Some(&SuMisura::base(&base)));
}

#[test]
fn andata_e_ritorno_del_file() {
    let base = comune::dati_base();
    let mut s = SuMisura::base(&base);
    s.nome = "Italia indebitata".into();
    s.cursori.debito_pil = 150.0;
    s.sostituzioni.insert("costo_medio_debito".into(), 3.2);
    let testo = s.in_toml(&base);
    assert!(!testo.contains("ricchezza"), "solo differenze: {testo}");
    assert!(!testo.contains("inflazione"), "solo differenze: {testo}");
    assert_eq!(SuMisura::da_toml(&testo, &base).unwrap(), s);
}

#[test]
fn andata_e_ritorno_con_nomi_da_virgolettare() {
    let base = comune::dati_base();
    let mut s = SuMisura::base(&base);
    s.nome = "L'Italia \"di domani\"".into();
    s.cursori.ricchezza = -10.0;
    s.cursori.legge_elettorale = "proporzionale_3".into();
    s.disposizioni.insert("cuneo_fiscale".into(), "0".into());
    s.sostituzioni.insert("valence di destra".into(), 0.5);
    let testo = s.in_toml(&base);
    assert_eq!(SuMisura::da_toml(&testo, &base).unwrap(), s);
}

#[test]
fn file_vuoto_e_la_base_col_suo_nome() {
    let base = comune::dati_base();
    let s = SuMisura::da_toml("nome = \"Altra Italia\"\n", &base).unwrap();
    assert_eq!(s.nome, "Altra Italia");
    assert_eq!(s.cursori, SuMisura::base(&base).cursori);
    assert!(s.disposizioni.is_empty() && s.sostituzioni.is_empty());
}

#[test]
fn sostituzione_e_cursore_assoluto_arrivano_allo_scenario() {
    let base = comune::dati_base();
    let mut s = SuMisura::base(&base);
    s.nome = "Italia indebitata".into();
    s.cursori.debito_pil = 150.0;
    s.sostituzioni.insert("costo_medio_debito".into(), 3.2);
    let d = base.con_su_misura(&s).unwrap();
    assert_eq!(d.scenario().debito_pil.valore, 150.0);
    assert_eq!(d.scenario().costo_medio_debito.valore, 3.2);
    assert_eq!(d.scenario().debito_pil.min, base.scenario().debito_pil.min);
    assert_eq!(d.nome_scenario(), "Italia indebitata");
}

#[test]
fn sostituzione_fuori_intervallo_rifiutata_col_nome() {
    let base = comune::dati_base();
    let mut s = SuMisura::base(&base);
    s.sostituzioni.insert("costo_medio_debito".into(), 99.0);
    let e = base.con_su_misura(&s).unwrap_err().to_string();
    assert!(e.contains("costo_medio_debito"), "{e}");
}

#[test]
fn su_misura_ostile_rifiutato_col_nome() {
    let base = comune::dati_base();
    let casi = [
        ("[cursori]\npil = 1.0", "`pil`"),
        ("[sostituzioni]\ndebito = 1.0", "debito"),
        ("[sostituzioni]\ndebito_pil = 150.0", "debito_pil"),
        ("[sostituzioni]\nricchezza = 0.0", "ricchezza"),
        ("[cursori]\ndebito_pil = 170.0", "debito_pil"),
        ("[cursori]\ndisoccupazione = 4.0", "disoccupazione"),
        ("[cursori]\ninflazione = -1.0", "inflazione"),
        ("[cursori]\nricchezza = 25.0", "ricchezza"),
        ("[cursori]\neta_mediana = -6.0", "eta_mediana"),
        ("[cursori]\norientamento = nan", "orientamento"),
        (
            "[cursori]\nlegge_elettorale = \"maggioritario\"",
            "maggioritario",
        ),
        (
            "[disposizioni]\nnumero_parlamentari = \"400\"",
            "numero_parlamentari",
        ),
        (
            "[disposizioni]\nnumero_parlamentari = \"400_200\"",
            "numero_parlamentari: le disposizioni costituzionali",
        ),
        ("[disposizioni]\ncuneo_fiscale = \"99\"", "cuneo_fiscale"),
        ("[disposizioni]\ninesistente = \"0\"", "inesistente"),
        ("pil = 1.0", "`pil`"),
    ];
    for (corpo, colpevole) in casi {
        let testo = format!("nome = \"Ostile\"\n{corpo}\n");
        let e = SuMisura::da_toml(&testo, &base).unwrap_err().to_string();
        assert!(e.contains(colpevole), "{corpo} -> {e}");
    }
    let costituzionale = &base.disposizioni()[base.disposizione("numero_parlamentari").unwrap()];
    assert!(
        costituzionale.gradino("400_200").is_some(),
        "il caso costituzionale deve usare un gradino vero"
    );
    let e = SuMisura::da_toml("[cursori]\n", &base)
        .unwrap_err()
        .to_string();
    assert!(e.contains("nome"), "{e}");
}

#[test]
fn errore_su_misura_si_stampa_col_prefisso() {
    let e = sim_core::Errore::SuMisura("x".into());
    assert_eq!(e.to_string(), "scenario su misura non valido: x");
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = "solo dati di base")]
fn la_base_da_dati_gia_su_misura_e_un_errore_di_programma() {
    let base = comune::dati_base();
    let mut s = SuMisura::base(&base);
    s.cursori.debito_pil = 150.0;
    let derivati = base.con_su_misura(&s).unwrap();
    SuMisura::base(&derivati);
}

#[test]
fn la_disoccupazione_sposta_naspi_e_gruppi() {
    use sim_core::Partita;
    use sim_core::elettorato::{Condizione, DatoGruppo, Fascia, indice_gruppo};
    let base = comune::dati_base();
    let mut s = SuMisura::base(&base);
    s.cursori.disoccupazione = 8.0;
    let d = base.con_su_misura(&s).unwrap();
    let attesa = base.scenario().spesa_sussidi.valore * 8.0 / 5.6;
    assert!((d.scenario().spesa_sussidi.valore - attesa).abs() < 1e-9);
    let sussidio = |d| Partita::nuova(d, 1).economia().sussidio_per_disoccupato;
    assert!((sussidio(d.clone()) - sussidio(base.clone())).abs() <= 1);
    let dis = |g: &[DatoGruppo], r| -> f64 {
        Fascia::TUTTE
            .iter()
            .map(|&f| g[indice_gruppo(r, Condizione::DisoccupatoInattivo, f)].quota)
            .sum()
    };
    for r in 0..base.regioni().len() {
        let (prima, dopo) = (
            dis(&base.elettorato().gruppi, r),
            dis(&d.elettorato().gruppi, r),
        );
        assert!(dopo > prima, "regione {r}: {dopo} <= {prima}");
    }
}

#[test]
fn l_inflazione_sposta_il_tasso_bce() {
    let base = comune::dati_base();
    let mut s = SuMisura::base(&base);
    s.cursori.inflazione = 6.0;
    let d = base.con_su_misura(&s).unwrap();
    let attesa = base.scenario().tasso_bce.valore
        + (1.0 + base.manopola("taylor_inflazione"))
            * base.manopola("quota_italia_area_euro")
            * 1.8;
    assert!((d.scenario().tasso_bce.valore - attesa).abs() < 1e-9);
}

#[test]
fn il_debito_muove_interessi_e_spread() {
    use sim_core::economia::Economia;
    let base = comune::dati_base();
    let mut s = SuMisura::base(&base);
    s.cursori.debito_pil = 150.0;
    let d = base.con_su_misura(&s).unwrap();
    let (e0, e1) = (Economia::iniziale(&base), Economia::iniziale(&d));
    assert!(e1.spread > e0.spread);
    assert!(e1.conti.spese.interessi > e0.conti.spese.interessi);
}
