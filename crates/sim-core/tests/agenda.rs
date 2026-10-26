mod comune;
use comune::{dati_base, ferma_tranne, ferma_tutti, senza_shock};
use sim_core::dati::DatiGioco;
use sim_core::vista::{Vista, VoceMinistero};
use sim_core::{Comando, Errore, Partita};

fn ministero<'a>(v: &'a Vista, id: &str) -> &'a VoceMinistero {
    v.ministeri.iter().find(|m| m.dicastero == id).unwrap()
}

fn ferma(d: &str) -> Comando {
    Comando::Ferma {
        dicastero: d.into(),
    }
}

fn coppia(seme: u64) -> (Partita, Partita) {
    let d = DatiGioco::da_testi(&senza_shock()).unwrap();
    (Partita::nuova(d.clone(), seme), Partita::nuova(d, seme))
}

#[test]
fn ogni_ministro_annuncia_un_atto_che_il_margine_paga() {
    let v = Partita::nuova(dati_base(), 1).vista();
    for m in &v.ministeri {
        let a = m.agenda.as_ref().unwrap();
        assert!(a.costo <= m.margine_residuo && !a.motivo.is_empty());
        let bersaglio =
            ["infrastrutture_trasporti", "sviluppo_economico"].contains(&m.dicastero.as_str());
        assert_eq!(a.regione.is_some(), bersaglio, "{}", m.dicastero);
        assert_eq!(m.atti.len(), 4);
    }
}

#[test]
fn atto_eseguito_muove_la_grandezza_e_spende_il_margine() {
    let (mut a, mut b) = coppia(3);
    a.chiudi_turno();
    b.chiudi_turno();
    let costo = ministero(&a.vista(), "sanita")
        .agenda
        .as_ref()
        .unwrap()
        .costo;
    ferma_tranne(&mut a, "sanita");
    ferma_tutti(&mut b);
    a.chiudi_turno();
    b.chiudi_turno();
    let residuo = |p: &Partita| ministero(&p.vista(), "sanita").margine_residuo;
    assert_eq!(residuo(&a), residuo(&b) - costo);
    let (mut sa, mut sb) = (0.0, 0.0);
    for _ in 0..12 {
        ferma_tutti(&mut a);
        ferma_tutti(&mut b);
        a.chiudi_turno();
        b.chiudi_turno();
        sa += a.grandezza("qualita_servizio_sanitario").unwrap();
        sb += b.grandezza("qualita_servizio_sanitario").unwrap();
    }
    assert!(sa > sb);
}

#[test]
fn fermare_costa_soddisfazione_e_dirottare_su_un_altro_atto_costa_meno() {
    let (mut a, mut b) = coppia(5);
    a.chiudi_turno();
    b.chiudi_turno();
    let v = a.vista();
    let m = ministero(&v, "sanita");
    let annunciato = m.agenda.as_ref().unwrap().atto.clone();
    let altro = m
        .atti
        .iter()
        .find(|x| x.id != annunciato && x.pagabile)
        .unwrap()
        .id
        .clone();
    let prima = m.ministro.as_ref().unwrap().soddisfazione;
    a.comanda(ferma("sanita")).unwrap();
    b.comanda(Comando::Dirotta {
        dicastero: "sanita".into(),
        atto: Some(altro),
        regione: None,
    })
    .unwrap();
    let s = |p: &Partita| {
        ministero(&p.vista(), "sanita")
            .ministro
            .as_ref()
            .unwrap()
            .soddisfazione
    };
    assert!(s(&a) < prima);
    assert!(s(&b) >= s(&a));
    assert!(a.comanda(ferma("sanita")).is_err());
}

#[test]
fn cantiere_fermato_a_meta_perde_quanto_speso() {
    let (mut a, mut b) = coppia(7);
    a.chiudi_turno();
    b.chiudi_turno();
    a.comanda(Comando::Dirotta {
        dicastero: "sanita".into(),
        atto: Some("sanita_case_comunita".into()),
        regione: None,
    })
    .unwrap();
    b.comanda(ferma("sanita")).unwrap();
    a.chiudi_turno();
    b.chiudi_turno();
    let vista = a.vista();
    let c = &ministero(&vista, "sanita").cantieri;
    assert_eq!((c.len(), c[0].turni_mancanti), (1, 3));
    let speso = c[0].speso;
    a.comanda(Comando::FermaCantiere {
        dicastero: "sanita".into(),
        atto: "sanita_case_comunita".into(),
    })
    .unwrap();
    let residuo = |p: &Partita| ministero(&p.vista(), "sanita").margine_residuo;
    assert_eq!(residuo(&a), residuo(&b) - speso);
    for _ in 0..10 {
        ferma_tutti(&mut a);
        ferma_tutti(&mut b);
        a.chiudi_turno();
        b.chiudi_turno();
        assert_eq!(
            a.grandezza("qualita_servizio_sanitario"),
            b.grandezza("qualita_servizio_sanitario")
        );
    }
}

#[test]
fn salvare_a_meta_turno_conserva_le_decisioni() {
    let mut p = Partita::nuova(dati_base(), 11);
    p.comanda(ferma("sanita")).unwrap();
    let scelta = ministero(&p.vista(), "infrastrutture_trasporti")
        .agenda
        .as_ref()
        .unwrap()
        .regione
        .clone();
    let altra = if scelta.as_deref() == Some("calabria") {
        "lombardia"
    } else {
        "calabria"
    };
    p.comanda(Comando::Dirotta {
        dicastero: "infrastrutture_trasporti".into(),
        atto: None,
        regione: Some(altra.into()),
    })
    .unwrap();
    let mut q = Partita::carica(&p.salva().unwrap()).unwrap();
    p.chiudi_turno();
    q.chiudi_turno();
    assert_eq!(p.salva().unwrap(), q.salva().unwrap());
}

#[test]
fn comandi_dell_agenda_non_ammessi_rifiutati_col_nome() {
    let mut p = Partita::nuova(dati_base(), 1);
    let prima = p.salva().unwrap();
    for (c, nome) in [
        (
            Comando::Dirotta {
                dicastero: "sanita".into(),
                atto: None,
                regione: Some("calabria".into()),
            },
            "sanita",
        ),
        (
            Comando::Dirotta {
                dicastero: "sanita".into(),
                atto: Some("giustizia_x".into()),
                regione: None,
            },
            "giustizia_x",
        ),
        (
            Comando::Dirotta {
                dicastero: "sviluppo_economico".into(),
                atto: None,
                regione: Some("padania".into()),
            },
            "padania",
        ),
        (
            Comando::FermaCantiere {
                dicastero: "sanita".into(),
                atto: "sanita_case_comunita".into(),
            },
            "sanita_case_comunita",
        ),
    ] {
        let e = p.comanda(c).unwrap_err();
        assert!(
            matches!(e, Errore::Comando(_)) && e.to_string().contains(nome),
            "{e}"
        );
    }
    assert_eq!(p.salva().unwrap(), prima);
}

#[test]
fn salvataggio_con_agenda_di_un_altro_dicastero_illeggibile() {
    let p = Partita::nuova(dati_base(), 1);
    let mut json: serde_json::Value = serde_json::from_str(&p.salva().unwrap()).unwrap();
    let v = p.vista();
    let i = v
        .ministeri
        .iter()
        .position(|m| m.dicastero == "sanita")
        .unwrap();
    json["ministri"]["dicasteri"][i]["agenda"]["atto"] = serde_json::json!(0);
    let e = Partita::carica(&json.to_string()).unwrap_err();
    assert!(matches!(e, Errore::SalvataggioIlleggibile(_)), "{e}");
}
