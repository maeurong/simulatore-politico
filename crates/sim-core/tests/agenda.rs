#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

mod comune;
use comune::{
    dati_base, edita, ferma_tranne, ferma_tutti, file_base, imposta, indice, senza_shock,
};
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
            "Sanità",
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
        "niente da dirottare: Infrastrutture e Trasporti",
    );
    let (caro, nome, di) = (v.ministeri.iter())
        .find_map(|m| {
            let a = m.atti.iter().find(|a| !a.pagabile)?;
            Some((a.id.clone(), a.nome.clone(), m.dicastero.clone()))
        })
        .unwrap();
    rifiuta(
        &mut p,
        dirotta(&di, Some(&caro), None),
        &format!("margine insufficiente per {nome}"),
    );
    p.comanda(ferma("sanita")).unwrap();
    rifiuta(&mut p, ferma("sanita"), "agenda già modificata: Sanità");
    rifiuta(
        &mut p,
        dirotta("sanita", Some("sanita_case_comunita"), None),
        "agenda già modificata: Sanità",
    );
    p.comanda(Comando::ChiediDimissioni {
        dicastero: "giustizia".into(),
    })
    .unwrap();
    p.chiudi_turno();
    rifiuta(&mut p, ferma("giustizia"), "nessuna agenda: Giustizia");
    rifiuta(
        &mut p,
        dirotta("giustizia", Some(&di_giustizia), None),
        "nessuna agenda: Giustizia",
    );
    rifiuta(
        &mut p,
        Comando::FermaCantiere {
            dicastero: "giustizia".into(),
            atto: di_giustizia.clone(),
        },
        &format!("nessun cantiere {di_giustizia} in Giustizia"),
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
        rifiuta(&mut p, c, "dimissioni chieste: Sanità");
    }
}

/// Un atto già in cantiere nel dicastero non torna in agenda e non si dirotta su di lui.
#[test]
fn niente_cantiere_doppio_dello_stesso_atto() {
    const CASE: &str = "sanita_case_comunita";
    let nome = (dati_base().atti().iter().find(|a| a.id == CASE))
        .unwrap()
        .nome
        .clone();
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
                &format!("cantiere già in corso: {nome}"),
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
            Box::new(|j| j["economia"]["decreti"][0] = json!(-1)),
        ),
        (
            "decreti oltre il tetto",
            Box::new(|j| j["economia"]["decreti"][0] = json!(LIMITE + 1)),
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

const CASE: &str = "sanita_case_comunita";

/// Ferma senza costo di soddisfazione: le agende si fermano a ogni turno senza che i ministri si
/// dimettano.
fn fermare_gratis() -> DatiGioco {
    let f = imposta(file_base(), "manopole", "[costo_fermare]\nvalore = ", "0.0");
    DatiGioco::da_testi(&f).unwrap()
}

/// Turno 1 col seme 7, agende ferme dal turno zero: la Sanità avvia le case di comunità, le altre
/// agende sono ferme. Rende la partita al turno 2.
fn cantiere_solo() -> Partita {
    let mut p = Partita::nuova(fermare_gratis(), 7);
    ferma_tutti(&mut p);
    p.chiudi_turno();
    if ministero(&p.vista(), "sanita")
        .agenda
        .as_ref()
        .unwrap()
        .atto
        != CASE
    {
        p.comanda(dirotta("sanita", Some(CASE), None)).unwrap();
    }
    ferma_tranne(&mut p, "sanita");
    p.chiudi_turno();
    p
}

fn effetti_di(p: &Partita, dicastero: &str) -> Vec<sim_core::grandezze::EffettoAttivo> {
    let i = indice(dicastero);
    (p.grandezze().effetti.iter())
        .filter(|e| e.dicastero == Some(i))
        .cloned()
        .collect()
}

fn cantiere(p: &Partita, dicastero: &str) -> Option<sim_core::vista::VoceCantiere> {
    ministero(&p.vista(), dicastero).cantieri.first().cloned()
}

fn residuo(p: &Partita, dicastero: &str) -> i64 {
    ministero(&p.vista(), dicastero).margine_residuo
}

#[test]
fn cantiere_finito_registra_effetti_una_volta_e_sparisce() {
    let d = dati_base();
    let atto = d.atti().iter().find(|a| a.id == CASE).unwrap();
    let mut p = cantiere_solo();
    assert_eq!(cantiere(&p, "sanita").unwrap().turni_mancanti, 3);
    let c = f64::from(
        ministero(&p.vista(), "sanita")
            .ministro
            .as_ref()
            .unwrap()
            .capacita,
    );
    while cantiere(&p, "sanita").is_some() {
        assert!(effetti_di(&p, "sanita").is_empty());
        assert!(ministero(&p.vista(), "sanita").ministro.is_some());
        assert!(Partita::carica(&p.salva().unwrap()).is_ok());
        ferma_tutti(&mut p);
        p.chiudi_turno();
    }
    // Con i prezzi sopra 1 la rata supera un quarto del margine annuo: l'ultima aspetta gennaio.
    let fine = p.vista().turno - 1;
    let effetti = effetti_di(&p, "sanita");
    assert_eq!(effetti.len(), atto.effetti.len());
    for (e, scritto) in effetti.iter().zip(&atto.effetti) {
        let ritardo = (f64::from(scritto.ritardo) * 3.0 / c).round() as u32;
        assert_eq!(e.inizio, fine + ritardo);
    }
    ferma_tutti(&mut p);
    p.chiudi_turno();
    assert_eq!(effetti_di(&p, "sanita").len(), atto.effetti.len());
}

#[test]
fn avvio_cantiere_paga_una_sola_rata_e_le_rate_precedono_l_agenda() {
    let s = indice("sanita");
    let mut p = Partita::nuova(fermare_gratis(), 7);
    ferma_tutti(&mut p);
    p.chiudi_turno();
    if ministero(&p.vista(), "sanita")
        .agenda
        .as_ref()
        .unwrap()
        .atto
        != CASE
    {
        p.comanda(dirotta("sanita", Some(CASE), None)).unwrap();
    }
    ferma_tranne(&mut p, "sanita");
    let prima = residuo(&p, "sanita");
    p.chiudi_turno();
    let f = cantiere(&p, "sanita").unwrap();
    assert_eq!((f.speso, f.turni_mancanti), (f.rata, 3));
    assert_eq!(residuo(&p, "sanita"), prima - f.rata);
    // Margine pari alla rata, agenda annunciata: paga la rata, l'agenda non si esegue.
    let mut q = edita(&p, |j| {
        j["ministri"]["dicasteri"][s]["margine_residuo"] = json!(f.rata);
    })
    .unwrap();
    assert!(ministero(&q.vista(), "sanita").agenda.is_some());
    ferma_tranne(&mut q, "sanita");
    q.chiudi_turno();
    let v = q.vista();
    let m = ministero(&v, "sanita");
    assert_eq!((m.margine_residuo, m.cantieri.len()), (0, 1));
    assert_eq!(m.cantieri[0].turni_mancanti, 2);
    assert!(effetti_di(&q, "sanita").is_empty());
}

/// Il cantiere del turno 2 con il margine residuo editato a `margine`, agende ferme; chiude il turno.
fn con_residuo(margine: i64) -> (Partita, sim_core::vista::VoceCantiere) {
    let s = indice("sanita");
    let p = cantiere_solo();
    let f = cantiere(&p, "sanita").unwrap();
    let mut q = edita(&p, |j| {
        j["ministri"]["dicasteri"][s]["margine_residuo"] = json!(margine + f.rata);
    })
    .unwrap();
    ferma_tutti(&mut q);
    q.chiudi_turno();
    (q, f)
}

#[test]
fn cantiere_senza_fondi_resta_fermo_senza_perdere_quanto_speso() {
    let (q, prima) = con_residuo(-1);
    let dopo = cantiere(&q, "sanita").unwrap();
    assert_eq!(dopo, prima);
    assert_eq!(residuo(&q, "sanita"), prima.rata - 1);
}

#[test]
fn cantiere_con_residuo_pari_alla_rata_avanza() {
    let (q, prima) = con_residuo(0);
    let dopo = cantiere(&q, "sanita").unwrap();
    assert_eq!(dopo.turni_mancanti, prima.turni_mancanti - 1);
    assert_eq!(dopo.speso, prima.speso + prima.rata);
    assert_eq!(residuo(&q, "sanita"), 0);
}

#[test]
fn margine_si_azzera_solo_a_gennaio_e_non_si_accumula() {
    // Il governo IA del ribaltone (turno 6) resta fino a gennaio 2029: `margine_voto_anticipato`
    // a 4 lo tiene lontano dal voto anticipato.
    let f = imposta(file_base(), "manopole", "[costo_fermare]\nvalore = ", "0.0");
    let f = imposta(f, "manopole", "[margine_voto_anticipato]\nvalore = ", "4.0");
    let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    let mut r = Vec::new();
    for turno in 0..=9 {
        r.push(residuo(&p, "sanita"));
        if turno == 2 {
            ferma_tranne(&mut p, "sanita");
        } else {
            ferma_tutti(&mut p);
        }
        p.chiudi_turno();
    }
    let spesa = r[2] - r[3];
    assert!(spesa > 0, "{r:?}");
    assert!(r[1] > r[0], "{r:?}");
    assert_eq!((r[2], r[4]), (r[1], r[3]), "{r:?}");
    assert_eq!(r[5], r[1], "{r:?}");
    assert!(r[5..9].iter().all(|x| *x == r[1]), "{r:?}");
    // A gennaio 2029 governa l'IA, che ha indicizzato il bilancio: il margine riparte dal suo.
    let v = p.vista();
    assert_eq!(v.bilancio.anno, 2029);
    let sanita = (v.bilancio.dicasteri.iter()).find(|d| d.id == "sanita");
    assert_eq!(Some(r[9]), sanita.map(|d| d.margine), "{r:?}");
}

#[test]
fn salva_e_carica_a_ogni_turno_per_12_turni() {
    let mut p = Partita::nuova(dati_base(), 13);
    for _ in 0..12 {
        let mut q = Partita::carica(&p.salva().unwrap()).unwrap();
        p.chiudi_turno();
        q.chiudi_turno();
        assert_eq!(p.salva().unwrap(), q.salva().unwrap());
    }
}

#[test]
fn effetto_regionale_muove_solo_la_regione_bersaglio() {
    const IT: &str = "infrastrutture_trasporti";
    const ATTO: &str = "infrastrutture_trasporti_commissari";
    let d = dati_base();
    let calabria = d.regioni().iter().position(|r| r.id == "calabria").unwrap();
    let k = d.grandezza("domanda_regionale").unwrap();
    let (mut a, mut b) = coppia(3);
    for p in [&mut a, &mut b] {
        ferma_tutti(p);
        p.chiudi_turno();
    }
    let ag = ministero(&a.vista(), IT).agenda.clone().unwrap();
    let atto = (ag.atto != ATTO).then_some(ATTO);
    let regione = (ag.regione.as_deref() != Some("calabria")).then_some("calabria");
    if atto.is_some() || regione.is_some() {
        a.comanda(dirotta(IT, atto, regione)).unwrap();
    }
    ferma_tranne(&mut a, IT);
    ferma_tutti(&mut b);
    let mut mosso = false;
    for _ in 0..4 {
        a.chiudi_turno();
        b.chiudi_turno();
        let (va, vb) = (&a.grandezze().valori[k], &b.grandezze().valori[k]);
        for (r, (x, y)) in va.iter().zip(vb).enumerate() {
            if r == calabria {
                mosso |= x != y;
            } else {
                assert_eq!(x, y, "regione {r}");
            }
        }
        ferma_tutti(&mut a);
        ferma_tutti(&mut b);
    }
    assert!(mosso);
}

#[test]
fn ferma_e_dirotta_costano_in_proporzione_alla_perdita_di_attaccamento() {
    const IT: &str = "infrastrutture_trasporti";
    let d = dati_base();
    let (s, it) = (indice("sanita"), indice(IT));
    let (fermare, regione) = (
        d.manopola("costo_fermare"),
        d.manopola("costo_dirottare_regione"),
    );
    let mut p = Partita::nuova(dati_base(), 5);
    p.chiudi_turno();
    let attaccamenti = [1.0, 0.5, 0.2, -0.5];
    let ag = p.ministri().dicasteri[it].agenda.clone().unwrap();
    let (scelta, altra) = (
        ag.regione.unwrap(),
        (ag.regione.unwrap() + 1) % d.regioni().len(),
    );
    let p = edita(&p, |j| {
        j["ministri"]["dicasteri"][s]["agenda"]["attaccamenti"] = json!(attaccamenti);
        let mut bersaglio = vec![0.0; d.regioni().len()];
        bersaglio[scelta] = 3.0;
        bersaglio[altra] = 1.0;
        j["ministri"]["dicasteri"][it]["agenda"]["bersaglio"] = json!(bersaglio);
    })
    .unwrap();
    let soddisfazione = |p: &Partita, id: &str| {
        ministero(&p.vista(), id)
            .ministro
            .as_ref()
            .unwrap()
            .soddisfazione
    };
    let v = p.vista();
    let m = ministero(&v, "sanita");
    let annunciato = m.agenda.as_ref().unwrap().atto.clone();
    let pos = |id: &str| {
        d.atti_del_dicastero(s)
            .position(|(_, a)| a.id == id)
            .unwrap()
    };
    let a_ann = attaccamenti[pos(&annunciato)];
    let prima = soddisfazione(&p, "sanita");
    let mut fermata = Partita::carica(&p.salva().unwrap()).unwrap();
    fermata.comanda(ferma("sanita")).unwrap();
    let atteso = (prima - fermare * a_ann).clamp(0.0, 100.0);
    assert!((soddisfazione(&fermata, "sanita") - atteso).abs() < 1e-9);
    for x in m.atti.iter().filter(|x| x.id != annunciato && x.pagabile) {
        let mut q = Partita::carica(&p.salva().unwrap()).unwrap();
        q.comanda(dirotta("sanita", Some(&x.id), None)).unwrap();
        let perdita = (a_ann - attaccamenti[pos(&x.id)]).max(0.0);
        let atteso = (prima - fermare * perdita).clamp(0.0, 100.0);
        assert!(
            (soddisfazione(&q, "sanita") - atteso).abs() < 1e-9,
            "{}",
            x.id
        );
    }
    let prima = soddisfazione(&p, IT);
    let mut q = Partita::carica(&p.salva().unwrap()).unwrap();
    let altra_id = d.regioni()[altra].id.clone();
    q.comanda(dirotta(IT, None, Some(&altra_id))).unwrap();
    let atteso = (prima - regione * 2.0).clamp(0.0, 100.0);
    assert!((soddisfazione(&q, IT) - atteso).abs() < 1e-9);
}

#[test]
fn successore_in_rodaggio_esegue_con_capacita_ridotta() {
    const LISTE: &str = "sanita_liste_straordinario";
    let d = dati_base();
    let atto = d.atti().iter().find(|a| a.id == LISTE).unwrap();
    let mut p = Partita::nuova(dati_base(), 1);
    p.comanda(Comando::ChiediDimissioni {
        dicastero: "sanita".into(),
    })
    .unwrap();
    ferma_tutti(&mut p);
    p.chiudi_turno();
    ferma_tutti(&mut p);
    p.chiudi_turno();
    let v = p.vista();
    let m = ministero(&v, "sanita");
    let x = m.ministro.as_ref().unwrap();
    assert!(x.rodaggio && x.capacita > 1, "{x:?}");
    if m.agenda.as_ref().unwrap().atto != LISTE {
        p.comanda(dirotta("sanita", Some(LISTE), None)).unwrap();
    }
    ferma_tranne(&mut p, "sanita");
    let turno = v.turno;
    let c = (f64::from(x.capacita) - d.manopola("riduzione_rodaggio")).max(1.0);
    p.chiudi_turno();
    let effetti = effetti_di(&p, "sanita");
    assert_eq!(effetti.len(), atto.effetti.len());
    for (e, scritto) in effetti.iter().zip(&atto.effetti) {
        assert!((e.intensita - scritto.intensita * c / 3.0).abs() < 1e-12);
        let ritardo = (f64::from(scritto.ritardo) * 3.0 / c).round() as u32;
        assert_eq!(e.inizio, turno + ritardo);
    }
}

#[test]
fn il_costo_degli_atti_segue_l_indice_pubblicato_non_il_livello_vero() {
    let mut p = Partita::nuova(dati_base(), 1);
    p.chiudi_turno();
    let costo = |p: &Partita| {
        ministero(&p.vista(), "sanita")
            .agenda
            .as_ref()
            .unwrap()
            .costo
    };
    let q = edita(&p, |j| {
        let s = &mut j["informazione"]["statistica"]["indice_prezzi"];
        *s = json!(s.as_f64().unwrap() * 1.1);
    })
    .unwrap();
    assert_eq!(q.economia().livello_prezzi, p.economia().livello_prezzi);
    let rapporto = costo(&q) as f64 / costo(&p) as f64;
    assert!((rapporto - 1.1).abs() < 1e-6, "{rapporto}");
}

/// Convenzioni col privato a costo trascurabile e Privato accreditato a "ridotto": il margine
/// basta, a negare il dirottamento resta solo il prerequisito.
#[test]
fn dirottare_su_un_atto_senza_prerequisito_e_rifiutato_e_la_partita_resta_identica() {
    let costo_basso = comune::sostituisci(
        file_base(),
        "atti",
        "costo = 1.0\neffetti = [\n  { grandezza = \"qualita_servizio_sanitario\", intensita = 8.0, ritardo = 1, forma = \"impulso\", durata = 1.5",
        "costo = 0.1\neffetti = [\n  { grandezza = \"qualita_servizio_sanitario\", intensita = 8.0, ritardo = 1, forma = \"impulso\", durata = 1.5",
    );
    let f = comune::sostituisci(
        costo_basso,
        "disposizioni",
        "partenza = \"attuale\"\nlegge = \"L. 199/2025, art. 1 c. 400\"",
        "partenza = \"ridotto\"\nlegge = \"L. 199/2025, art. 1 c. 400\"",
    );
    let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    assert!(ministero(&p.vista(), "sanita").agenda.is_some());
    rifiuta(
        &mut p,
        dirotta("sanita", Some("sanita_convenzioni_privato"), None),
        "sanita_convenzioni_privato: serve Privato accreditato: attuale o ampio",
    );
}
