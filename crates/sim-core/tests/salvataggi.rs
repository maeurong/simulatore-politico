mod comune;
use comune::{dati_base, edita, fondata};
use sim_core::salvataggi::{FORMATO_ALBO, VoceAlbo};
use sim_core::{Errore, Partita, SuMisura, VERSIONE_SCHEMA};

#[test]
fn un_punto_per_apertura() {
    let mut p = fondata(3.5);
    assert_eq!(p.serie().len(), 1);
    for _ in 0..3 {
        p.chiudi_turno();
    }
    let turni: Vec<u32> = p.serie().iter().map(|s| s.turno).collect();
    assert_eq!(turni, [0, 1, 2, 3]);
    assert_eq!(p.serie().last().unwrap().data, p.vista().data);
}

#[test]
fn senza_versione_e_illeggibile() {
    let p = fondata(3.5);
    for versione in [
        None,
        Some(serde_json::json!("11")),
        Some(serde_json::json!(11.5)),
    ] {
        let mut j: serde_json::Value = serde_json::from_str(&p.salva().unwrap()).unwrap();
        match versione {
            None => {
                j.as_object_mut().unwrap().remove("versione_schema");
            }
            Some(v) => j["versione_schema"] = v,
        }
        let e = Partita::carica(&j.to_string()).unwrap_err();
        assert!(
            matches!(&e, Errore::SalvataggioIlleggibile(m) if m.contains("versione")),
            "{e:?}"
        );
    }
}

#[test]
fn lo_schema_zero_resta_schema_diverso() {
    let p = fondata(3.5);
    let mut j: serde_json::Value = serde_json::from_str(&p.salva().unwrap()).unwrap();
    j["versione_schema"] = 0.into();
    let e = Partita::carica(&j.to_string()).unwrap_err();
    assert!(matches!(
        e,
        Errore::SchemaDiverso {
            trovata: 0,
            attesa: VERSIONE_SCHEMA
        }
    ));
}

#[test]
fn la_testata_regge_un_salvataggio() {
    let mut p = fondata(3.5);
    p.chiudi_turno();
    let t = p.testata();
    assert_eq!(t.nome, "Partito nuovo");
    assert_eq!(t.scenario, "Italia di oggi");
    assert_eq!(
        (t.turno, t.versione_schema, t.finita),
        (1, VERSIONE_SCHEMA, false)
    );
    assert_eq!(Partita::carica(&p.salva().unwrap()).unwrap().testata(), t);
}

#[test]
fn la_testata_porta_il_nome_dello_scenario_su_misura() {
    let base = dati_base();
    let mut s = SuMisura::base(&base);
    s.nome = "Italia indebitata".into();
    let p = Partita::nuova(base.con_su_misura(&s).unwrap(), 1);
    assert_eq!(p.testata().scenario, "Italia indebitata");
    assert_eq!(
        Partita::carica(&p.salva().unwrap()).unwrap().testata(),
        p.testata()
    );
}

#[test]
fn a_camere_sciolte_manca_tutta_la_maggioranza() {
    let p = fondata(3.5);
    let leve = p.disposizioni().leve(&dati_base());
    let meta_piu_uno = |seggi: f64| -(seggi.round() as i32 / 2 + 1);
    let s = &p.serie()[0];
    assert_eq!(s.maggioranza_camera, meta_piu_uno(leve.seggi_camera));
    assert_eq!(s.maggioranza_senato, meta_piu_uno(leve.seggi_senato));
    assert_eq!(s.spread_pb, p.economia().spread.round() as i32);
}

#[test]
fn il_consenso_e_la_media_degli_istituti() {
    let p = fondata(3.5);
    let g = p.giocatore();
    let istituti = &p.informazione().sondaggio.istituti;
    let atteso = (istituti.iter())
        .map(|r| r.quote[g] / (1.0 - r.quote.last().unwrap()))
        .sum::<f64>()
        / istituti.len() as f64;
    assert!((p.serie()[0].consenso - atteso).abs() < 1e-12);
    assert!(atteso > 0.0);
}

type Guasto = fn(&mut serde_json::Value);

#[test]
fn la_serie_si_controlla_al_caricamento() {
    let mut p = fondata(3.5);
    p.chiudi_turno();
    p.chiudi_turno();
    let guasti: [(&str, Guasto); 3] = [
        ("vuota", |j| j["serie"] = serde_json::json!([])),
        ("buco", |j| {
            j["serie"].as_array_mut().unwrap().remove(1);
        }),
        ("ripetuto", |j| j["serie"][2]["turno"] = 1.into()),
    ];
    for (caso, f) in guasti {
        let e = edita(&p, f).unwrap_err();
        assert!(
            matches!(&e, Errore::SalvataggioIlleggibile(m) if m.contains("serie")),
            "{caso}: {e:?}"
        );
    }
}

#[test]
fn la_voce_dell_albo_regge_un_formato_futuro() {
    let mut p = fondata(3.5);
    p.chiudi_turno();
    p.abbandona().unwrap();
    let v = p.voce_albo().unwrap();
    let mut j = serde_json::to_value(&v).unwrap();
    j["campo_del_futuro"] = serde_json::json!({"a": 1});
    assert_eq!(serde_json::from_value::<VoceAlbo>(j).unwrap(), v);
}

#[test]
fn l_albo_porta_elezioni_e_serie() {
    let mut p = fondata(3.5);
    for _ in 0..60 {
        if !p.legislatura().elezioni.is_empty() {
            break;
        }
        p.chiudi_turno();
    }
    assert!(!p.legislatura().elezioni.is_empty());
    p.chiudi_turno();
    p.abbandona().unwrap();
    let v = p.voce_albo().unwrap();
    let e = p.eredita().unwrap();
    assert_eq!(v.formato_albo, FORMATO_ALBO);
    assert_eq!(v.esito, "Abbandono");
    assert_eq!(v.nome, p.testata().nome);
    assert_eq!(v.scenario, p.testata().scenario);
    assert_eq!(v.turni, p.testata().turno);
    // Un punto per apertura giocata, turno zero compreso; l'abbandono non chiude.
    assert_eq!(v.serie.len(), v.turni as usize + 1);
    assert_eq!(v.serie.as_slice(), p.serie());
    assert_eq!(v.inizio, v.serie[0].data);
    assert_eq!(v.fine, p.testata().data);
    assert_eq!(v.elezioni.len(), p.legislatura().elezioni.len());
    let (el, es) = (&v.elezioni[0], &p.legislatura().elezioni[0]);
    assert_eq!(el.data, es.data);
    assert_eq!(el.voti.len(), es.voti.len());
    let mio = &p.partiti()[p.giocatore()];
    let suo = es.voti.iter().find(|x| x.partito == mio.id);
    assert_eq!(
        el.seggi_giocatore,
        suo.map_or((0, 0), |x| (x.camera, x.senato))
    );
    assert!(el.voti.iter().any(|(n, _)| *n == mio.nome));
    assert_eq!(v.anni_al_governo, e.anni_al_governo);
    assert_eq!(v.leggi_in_vigore, e.leggi_in_vigore);
    assert_eq!(v.riavvolgimenti, e.riavvolgimenti);
    assert_eq!(v.indicatori.len(), e.indicatori.len());
    for (a, b) in v.indicatori.iter().zip(&e.indicatori) {
        assert_eq!(
            (&a.id, &a.nome, a.turno_zero, a.fine),
            (&b.id, &b.nome, b.turno_zero, b.fine)
        );
    }
}

#[test]
fn l_albo_di_una_partita_senza_seggi_non_ha_il_punto_della_chiusura() {
    // Stessa fondazione di `eredita_di_chi_non_ha_mai_governato` (tests/eredita.rs).
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
    let v = p.voce_albo().unwrap();
    assert_eq!(v.esito, "Senza seggi");
    assert_eq!(v.turni, 1);
    assert_eq!(v.serie.len(), v.turni as usize);
}

#[test]
fn nessuna_voce_a_partita_in_corso() {
    assert!(fondata(3.5).voce_albo().is_none());
}

#[test]
fn i_riavvolgimenti_reggono_un_salvataggio() {
    let mut p = fondata(3.5);
    assert_eq!(p.riavvolgimenti(), 0);
    p.imposta_riavvolgimenti(2);
    assert_eq!(
        Partita::carica(&p.salva().unwrap())
            .unwrap()
            .riavvolgimenti(),
        2
    );
}
