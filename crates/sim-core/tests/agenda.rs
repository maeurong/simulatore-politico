#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

mod comune;
use comune::{dati_base, edita, ferma_tranne, ferma_tutti, indice, senza_shock};
use serde_json::{Value, json};
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

/// Il comando è rifiutato con `Errore::Comando` che contiene `nome`, e la partita resta identica.
fn rifiuta(p: &mut Partita, c: Comando, nome: &str) {
    let prima = p.salva().unwrap();
    let e = p.comanda(c.clone()).unwrap_err();
    assert!(
        matches!(e, Errore::Comando(_)) && e.to_string().contains(nome),
        "{c:?}: {e}"
    );
    assert_eq!(p.salva().unwrap(), prima, "{c:?}");
}

fn dirotta(d: &str, atto: Option<&str>, regione: Option<&str>) -> Comando {
    Comando::Dirotta {
        dicastero: d.into(),
        atto: atto.map(Into::into),
        regione: regione.map(Into::into),
    }
}

#[test]
fn rifiuti_di_ferma_dirotta_e_ferma_cantiere_col_nome_e_partita_identica() {
    const IT: &str = "infrastrutture_trasporti";
    let mut p = Partita::nuova(dati_base(), 1);
    let d = dati_base();
    let di_giustizia = (d.atti().iter())
        .find(|a| a.dicastero == "giustizia")
        .unwrap()
        .id
        .clone();
    rifiuta(
        &mut p,
        dirotta("sanita", Some(&di_giustizia), None),
        &di_giustizia,
    );
    let v = p.vista();
    let regione = ministero(&v, IT).agenda.as_ref().unwrap().regione.clone();
    rifiuta(
        &mut p,
        dirotta(IT, None, regione.as_deref()),
        &format!("niente da dirottare: {IT}"),
    );
    let (caro, di) = (v.ministeri.iter())
        .find_map(|m| {
            let a = m.atti.iter().find(|a| !a.pagabile)?;
            Some((a.id.clone(), m.dicastero.clone()))
        })
        .unwrap();
    rifiuta(
        &mut p,
        dirotta(&di, Some(&caro), None),
        &format!("margine insufficiente per {caro}"),
    );
    p.comanda(ferma("sanita")).unwrap();
    rifiuta(&mut p, ferma("sanita"), "agenda già modificata: sanita");
    rifiuta(
        &mut p,
        dirotta("sanita", Some("sanita_case_comunita"), None),
        "agenda già modificata: sanita",
    );
    p.comanda(Comando::ChiediDimissioni {
        dicastero: "giustizia".into(),
    })
    .unwrap();
    p.chiudi_turno();
    rifiuta(&mut p, ferma("giustizia"), "nessuna agenda: giustizia");
    rifiuta(
        &mut p,
        dirotta("giustizia", Some(&di_giustizia), None),
        "nessuna agenda: giustizia",
    );
    rifiuta(
        &mut p,
        Comando::FermaCantiere {
            dicastero: "giustizia".into(),
            atto: di_giustizia.clone(),
        },
        &format!("nessun cantiere {di_giustizia} in giustizia"),
    );
}

/// Con le dimissioni chieste il ministro uscente non prende più comandi sull'agenda né sulle
/// proposte.
#[test]
fn dimissioni_chieste_bloccano_i_comandi_del_dicastero() {
    let mut p = Partita::nuova(dati_base(), 1);
    p.comanda(Comando::ChiediDimissioni {
        dicastero: "sanita".into(),
    })
    .unwrap();
    assert!(ministero(&p.vista(), "sanita").dimissioni_chieste);
    let s = || "sanita".to_string();
    for c in [
        Comando::Ferma { dicastero: s() },
        dirotta("sanita", Some("sanita_case_comunita"), None),
        Comando::FermaCantiere {
            dicastero: s(),
            atto: "sanita_case_comunita".into(),
        },
        Comando::Accogli { dicastero: s() },
        Comando::Rinvia { dicastero: s() },
        Comando::Respingi { dicastero: s() },
    ] {
        rifiuta(&mut p, c, "dimissioni chieste: sanita");
    }
}

/// Un atto già in cantiere nel dicastero non torna in agenda e non si dirotta su di lui.
#[test]
fn niente_cantiere_doppio_dello_stesso_atto() {
    const CASE: &str = "sanita_case_comunita";
    for seme in 1..=8 {
        let mut p = Partita::nuova(dati_base(), seme);
        p.chiudi_turno();
        let annunciato = ministero(&p.vista(), "sanita")
            .agenda
            .as_ref()
            .unwrap()
            .atto
            .clone();
        if annunciato != CASE {
            p.comanda(dirotta("sanita", Some(CASE), None)).unwrap();
        }
        p.chiudi_turno();
        let v = p.vista();
        let m = ministero(&v, "sanita");
        assert!(m.cantieri.iter().any(|c| c.atto == CASE), "seme {seme}");
        assert!(
            !m.agenda.as_ref().is_some_and(|a| a.atto == CASE),
            "seme {seme}"
        );
        if m.agenda.is_some() {
            rifiuta(
                &mut p,
                dirotta("sanita", Some(CASE), None),
                &format!("cantiere già in corso: {CASE}"),
            );
        }
    }
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

/// Tetto agli euro di un salvataggio (`LIMITE_EURO`).
const LIMITE: i64 = 1_000_000_000_000_000;

type Modifica = Box<dyn Fn(&mut Value)>;

/// Turno 2 col seme 7: un cantiere di case di comunità nella Sanità, agende annunciate.
fn con_cantiere() -> Partita {
    let mut p = Partita::nuova(dati_base(), 7);
    p.chiudi_turno();
    p.comanda(dirotta("sanita", Some("sanita_case_comunita"), None))
        .unwrap();
    p.chiudi_turno();
    p
}

#[test]
fn salvataggi_corrotti_rifiutati_senza_panico() {
    let p = con_cantiere();
    let (s, it) = (indice("sanita"), indice("infrastrutture_trasporti"));
    let n_regioni = dati_base().regioni().len();
    assert!(edita(&p, |_| ()).is_ok());
    let righe: Vec<(&str, Modifica)> = vec![
        (
            "rata enorme",
            Box::new(move |j| {
                j["ministri"]["dicasteri"][s]["cantieri"][0]["rata"] = json!(i64::MAX);
            }),
        ),
        (
            "speso oltre il tetto",
            Box::new(move |j| {
                j["ministri"]["dicasteri"][s]["cantieri"][0]["speso"] = json!(LIMITE + 1);
            }),
        ),
        (
            "somma delle rate oltre il tetto",
            Box::new(move |j| {
                let c = &mut j["ministri"]["dicasteri"][s]["cantieri"];
                c[0]["rata"] = json!(LIMITE);
                let doppio = c[0].clone();
                c.as_array_mut().unwrap().push(doppio);
            }),
        ),
        (
            "81 cantieri",
            Box::new(move |j| {
                let c = &mut j["ministri"]["dicasteri"][s]["cantieri"];
                let uno = c[0].clone();
                *c = Value::Array(vec![uno; 81]);
            }),
        ),
        (
            "turni_mancanti 0",
            Box::new(move |j| {
                j["ministri"]["dicasteri"][s]["cantieri"][0]["turni_mancanti"] = json!(0);
            }),
        ),
        (
            "regione in un dicastero senza bersaglio",
            Box::new(move |j| {
                j["ministri"]["dicasteri"][s]["cantieri"][0]["regione"] = json!(0);
            }),
        ),
        (
            "regione oltre le regioni",
            Box::new(move |j| {
                j["ministri"]["dicasteri"][it]["agenda"]["regione"] = json!(n_regioni);
            }),
        ),
        (
            "attaccamenti corti",
            Box::new(move |j| {
                j["ministri"]["dicasteri"][it]["agenda"]["attaccamenti"] = json!([1.0]);
            }),
        ),
        (
            "agenda senza ministro",
            Box::new(move |j| {
                j["ministri"]["dicasteri"][it]["ministro"] = Value::Null;
            }),
        ),
        (
            "margine residuo negativo",
            Box::new(move |j| {
                j["ministri"]["dicasteri"][s]["margine_residuo"] = json!(-1);
            }),
        ),
        (
            "margine residuo oltre il tetto",
            Box::new(move |j| {
                j["ministri"]["dicasteri"][s]["margine_residuo"] = json!(LIMITE + 1);
            }),
        ),
        (
            "causa delle dimissioni con un ministro",
            Box::new(move |j| {
                j["ministri"]["dicasteri"][it]["causa_dimissioni"] = json!("dimissioni_chieste");
            }),
        ),
        (
            "dimissioni chieste in interim",
            Box::new(move |j| {
                let d = &mut j["ministri"]["dicasteri"][it];
                d["ministro"] = Value::Null;
                d["agenda"] = Value::Null;
                d["proposta"] = Value::Null;
                d["dimissioni_chieste"] = json!(true);
            }),
        ),
        (
            "rosa con un ministro in carica",
            Box::new(move |j| {
                let d = &mut j["ministri"]["dicasteri"][it];
                d["rosa"] = json!([d["ministro"], d["ministro"], d["ministro"]]);
            }),
        ),
        (
            "effetto oltre l'intervallo della grandezza",
            Box::new(move |j| {
                let e = json!({"grandezza": 0, "regione": null, "dicastero": 0, "intensita": 1e9,
                "inizio": 0, "forma": "impulso", "durata": 2.0});
                j["grandezze"]["effetti"].as_array_mut().unwrap().push(e);
            }),
        ),
        (
            "decreti negativi",
            Box::new(|j| j["economia"]["decreti"] = json!(-1)),
        ),
        (
            "decreti oltre il tetto",
            Box::new(|j| j["economia"]["decreti"] = json!(LIMITE + 1)),
        ),
        (
            "livello dei prezzi nullo",
            Box::new(|j| j["economia"]["livello_prezzi"] = json!(0.0)),
        ),
        (
            "livello dei prezzi negativo",
            Box::new(|j| {
                j["economia"]["livello_prezzi"] = json!(-1.0);
            }),
        ),
        (
            "livello dei prezzi oltre 1e6",
            Box::new(|j| {
                j["economia"]["livello_prezzi"] = json!(1e7);
            }),
        ),
        (
            "riscossione nulla",
            Box::new(|j| j["misure"]["riscossione"] = json!(0.0)),
        ),
    ];
    for (caso, modifica) in righe {
        let e = edita(&p, |j| modifica(j)).err();
        assert!(
            matches!(e, Some(Errore::SalvataggioIlleggibile(_))),
            "{caso}: {e:?}"
        );
    }
}

/// Dirotta sulla regione ricalcola il motivo con la regione nuova.
#[test]
fn dirotta_di_regione_ricalcola_il_motivo() {
    const IT: &str = "infrastrutture_trasporti";
    let d = dati_base();
    let nome = |id: &str| {
        d.regioni()
            .iter()
            .find(|r| r.id == id)
            .unwrap()
            .nome
            .clone()
    };
    let mut p = Partita::nuova(dati_base(), 11);
    let scelta = ministero(&p.vista(), IT)
        .agenda
        .as_ref()
        .unwrap()
        .regione
        .clone()
        .unwrap();
    let altra = if scelta == "calabria" {
        "lombardia"
    } else {
        "calabria"
    };
    p.comanda(dirotta(IT, None, Some(altra))).unwrap();
    let motivo = ministero(&p.vista(), IT)
        .agenda
        .as_ref()
        .unwrap()
        .motivo
        .clone();
    assert!(!motivo.contains(&nome(&scelta)), "{motivo}");
    let coda = |perche: &str| format!("; {}: {perche}", nome(altra));
    assert!(
        motivo.ends_with(&coda("ci votano")) || motivo.ends_with(&coda("ne ha bisogno")),
        "{motivo}"
    );
    assert_eq!(motivo.matches("; ").count(), 1, "{motivo}");
}

/// FermaCantiere costa l'attaccamento del ministro in carica all'atto, non quello di chi
/// l'aveva avviato.
#[test]
fn ferma_cantiere_costa_l_attaccamento_del_ministro_in_carica() {
    const CASE: &str = "sanita_case_comunita";
    let s = indice("sanita");
    let mut p = con_cantiere();
    p.comanda(Comando::ChiediDimissioni {
        dicastero: "sanita".into(),
    })
    .unwrap();
    p.chiudi_turno();
    p.chiudi_turno();
    let v = p.vista();
    let m = ministero(&v, "sanita");
    assert!(m.ministro.as_ref().unwrap().rodaggio);
    assert!(m.cantieri.iter().any(|c| c.atto == CASE));
    let d = dati_base();
    let pos = d
        .atti_del_dicastero(s)
        .position(|(_, a)| a.id == CASE)
        .unwrap();
    let agenda = p.ministri().dicasteri[s].agenda.clone().unwrap();
    let atteso = -d.manopola("costo_fermare") * agenda.attaccamenti[pos];
    let prima = m.ministro.as_ref().unwrap().soddisfazione;
    p.comanda(Comando::FermaCantiere {
        dicastero: "sanita".into(),
        atto: CASE.into(),
    })
    .unwrap();
    let dopo = ministero(&p.vista(), "sanita").ministro.clone().unwrap();
    assert!(
        (dopo.soddisfazione - (prima + atteso).clamp(0.0, 100.0)).abs() < 1e-9,
        "{prima} {atteso} {}",
        dopo.soddisfazione
    );
}
