#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

mod comune;
use comune::{dati_base, edita, file_base, imposta, indice, voci};
use sim_core::dati::DatiGioco;
use sim_core::esito::{Criticita, TipoVoce};
use sim_core::ministri::Causa;
use sim_core::vista::{Vista, VoceMinistero};
use sim_core::{Comando, Errore, Partita};

fn ministero<'a>(v: &'a Vista, id: &str) -> &'a VoceMinistero {
    v.ministeri.iter().find(|m| m.dicastero == id).unwrap()
}

fn dimissioni(dicastero: &str) -> Comando {
    Comando::ChiediDimissioni {
        dicastero: dicastero.into(),
    }
}

#[test]
fn dieci_ministri_al_turno_zero() {
    let v = Partita::nuova(dati_base(), 1).vista();
    assert_eq!(v.ministeri.len(), 10);
    for m in &v.ministeri {
        let x = m.ministro.as_ref().unwrap();
        assert!((1..=5).contains(&x.capacita));
        assert!((1.0..=7.0).contains(&x.posizione));
        assert_eq!(x.soddisfazione, 60.0);
        assert!(!x.rodaggio && !x.minaccia);
    }
    assert_eq!(
        ministero(&v, "esteri_difesa")
            .ministro
            .as_ref()
            .unwrap()
            .partito,
        "centrodestra"
    );
    assert_eq!(
        ministero(&v, "sanita").ministro.as_ref().unwrap().obiettivo,
        105.0
    );
    assert_eq!(
        ministero(&v, "giustizia")
            .ministro
            .as_ref()
            .unwrap()
            .obiettivo,
        95.0
    );
}

#[test]
fn stesso_seme_stessi_ministri() {
    let a = Partita::nuova(dati_base(), 4).vista().ministeri;
    assert_eq!(a, Partita::nuova(dati_base(), 4).vista().ministeri);
    assert_ne!(a, Partita::nuova(dati_base(), 5).vista().ministeri);
}

#[test]
fn dimissioni_chieste_portano_interim_rosa_e_rodaggio() {
    let mut p = Partita::nuova(dati_base(), 1);
    p.comanda(dimissioni("sanita")).unwrap();
    p.chiudi_turno();
    let v = p.vista();
    let m = ministero(&v, "sanita");
    assert!(m.ministro.is_none());
    assert_eq!(m.rosa.len(), 3);
    assert_eq!(m.causa_dimissioni, Some(Causa::DimissioniChieste));
    assert!(m.rosa.iter().all(|x| x.partito == "destra"));
    assert!(m.rosa.windows(2).all(|w| w[0].capacita >= w[1].capacita));
    assert!(!m.dimissioni_chieste && m.scelta.is_none());
    // Fatti 9 e 87: chieste dal giocatore, notevole e non critica.
    let esce = voci(&p, TipoVoce::MinistroEsce);
    assert_eq!(esce.len(), 1, "{esce:?}");
    assert_eq!(esce[0].dicastero.as_deref(), Some("sanita"));
    assert!(
        esce[0]
            .fatto
            .ends_with(" lascia il dicastero Sanità, che va in interim"),
        "{}",
        esce[0].fatto
    );
    assert_eq!(
        esce[0].causa,
        "le dimissioni chieste dal Presidente del Consiglio"
    );
    assert_eq!(esce[0].criticita, Criticita::Notevole);
    let scelto = m.rosa[1].nome.clone();
    p.comanda(Comando::ScegliSuccessore {
        dicastero: "sanita".into(),
        indice: 1,
    })
    .unwrap();
    assert_eq!(ministero(&p.vista(), "sanita").scelta, Some(1));
    p.chiudi_turno();
    let x = ministero(&p.vista(), "sanita").ministro.clone().unwrap();
    assert_eq!(x.nome, scelto);
    assert!(x.rodaggio);
    assert_eq!(ministero(&p.vista(), "sanita").causa_dimissioni, None);
    p.chiudi_turno();
    assert!(
        !ministero(&p.vista(), "sanita")
            .ministro
            .as_ref()
            .unwrap()
            .rodaggio
    );
}

#[test]
fn successore_non_scelto_e_il_primo_della_rosa() {
    let mut p = Partita::nuova(dati_base(), 2);
    p.comanda(dimissioni("interno")).unwrap();
    p.chiudi_turno();
    let primo = ministero(&p.vista(), "interno").rosa[0].nome.clone();
    p.chiudi_turno();
    assert_eq!(
        ministero(&p.vista(), "interno")
            .ministro
            .as_ref()
            .unwrap()
            .nome,
        primo
    );
}

#[test]
fn ministri_sotto_soglia_minacciano_e_si_dimettono_a_fine_turno() {
    let f = imposta(
        file_base(),
        "manopole",
        "[soddisfazione_base]\nvalore = ",
        "30.0",
    );
    let f = imposta(f, "manopole", "[soglia_dimissioni]\nvalore = ", "35.0");
    let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    assert!(
        p.vista()
            .ministeri
            .iter()
            .all(|m| m.ministro.as_ref().unwrap().minaccia)
    );
    p.chiudi_turno();
    assert!(
        p.vista()
            .ministeri
            .iter()
            .all(|m| m.ministro.is_none() && m.rosa.len() == 3 && m.causa_dimissioni.is_some())
    );
    // Fatto 87: dieci uscite per soddisfazione, critiche e fuse in una voce.
    let esce = voci(&p, TipoVoce::MinistroEsce);
    assert_eq!(esce.len(), 10, "{esce:?}");
    for v in &esce {
        assert_eq!((v.criticita, v.precedenza), (Criticita::Critica, 2));
        assert!(
            v.causa
                .starts_with("la soddisfazione è scesa sotto la soglia di dimissioni di 35,0"),
            "{}",
            v.causa
        );
    }
    // Già sotto la minaccia prima della chiusura, ed escono: nessuna minaccia nuova.
    assert!(voci(&p, TipoVoce::MinacciaDiDimissioni).is_empty());
}

/// Fatto 88: con la base a 30 tutti minacciano già; la Sanità, a 41, scende sotto i 40 nella
/// chiusura ed è l'unica voce.
#[test]
fn minaccia_nata_nella_chiusura() {
    let f = imposta(
        file_base(),
        "manopole",
        "[soddisfazione_base]\nvalore = ",
        "30.0",
    );
    let f = imposta(f, "manopole", "[soglia_dimissioni]\nvalore = ", "15.0");
    let p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    let i = indice("sanita");
    let mut p = edita(&p, |j| {
        j["ministri"]["dicasteri"][i]["ministro"]["soddisfazione"] = serde_json::json!(41.0)
    })
    .unwrap();
    p.chiudi_turno();
    let minacce = voci(&p, TipoVoce::MinacciaDiDimissioni);
    assert_eq!(minacce.len(), 1, "{minacce:?}");
    let v = &minacce[0];
    assert_eq!(v.dicastero.as_deref(), Some("sanita"));
    assert!(
        v.fatto.ends_with(" (Sanità) minaccia le dimissioni"),
        "{}",
        v.fatto
    );
    assert!(
        v.causa
            .starts_with("la soddisfazione è scesa sotto la soglia di minaccia di 40,0"),
        "{}",
        v.causa
    );
    assert_eq!((v.criticita, v.precedenza), (Criticita::Critica, 2));
}

#[test]
fn comandi_non_ammessi_rifiutati_col_nome() {
    let mut p = Partita::nuova(dati_base(), 1);
    let prima = p.salva().unwrap();
    for (c, nome) in [
        (dimissioni("sanitá"), "sanitá"),
        (
            Comando::ScegliSuccessore {
                dicastero: "sanita".into(),
                indice: 0,
            },
            "Sanità",
        ),
    ] {
        let e = p.comanda(c).unwrap_err();
        assert!(
            matches!(e, Errore::Comando(_)) && e.to_string().contains(nome),
            "{e}"
        );
    }
    p.comanda(dimissioni("sanita")).unwrap();
    p.chiudi_turno();
    let e = p
        .comanda(Comando::ScegliSuccessore {
            dicastero: "sanita".into(),
            indice: 3,
        })
        .unwrap_err();
    assert!(e.to_string().contains("Sanità"), "{e}");
    let mut q = Partita::carica(&prima).unwrap();
    assert!(q.comanda(dimissioni("sanitá")).is_err());
    assert_eq!(q.salva().unwrap(), prima);
}

/// La rosa tecnica è del ministro uscito: con un ministro in carica lo stato è incoerente.
#[test]
fn salvataggio_con_rosa_tecnica_e_ministro_illeggibile() {
    let p = Partita::nuova(dati_base(), 1);
    let e = comune::edita(&p, |j| {
        j["ministri"]["dicasteri"][0]["rosa_tecnica"] = serde_json::json!(true)
    })
    .unwrap_err();
    assert!(e.to_string().contains("rosa tecnica"), "{e}");
}

#[test]
fn salvataggio_con_ministri_corti_illeggibile() {
    let p = Partita::nuova(dati_base(), 1);
    let mut json: serde_json::Value = serde_json::from_str(&p.salva().unwrap()).unwrap();
    json["ministri"]["dicasteri"].as_array_mut().unwrap().pop();
    let e = Partita::carica(&json.to_string()).unwrap_err();
    assert!(matches!(e, Errore::SalvataggioIlleggibile(_)), "{e}");
}

/// La rosa fissa l'obiettivo su ciò che il Presidente del Consiglio vede (le grandezze del turno
/// prima), non sul valore vero.
#[test]
fn rosa_fissa_l_obiettivo_sul_valore_visto() {
    let d = dati_base();
    let passo = d.manopola("passo_obiettivo");
    let mut p = Partita::nuova(dati_base(), 1);
    p.comanda(dimissioni("sanita")).unwrap();
    p.chiudi_turno();
    let v = p.vista();
    let visto = (v.grandezze.iter())
        .find(|g| g.id == "qualita_servizio_sanitario")
        .unwrap()
        .valori[0];
    assert_ne!(Some(visto), p.grandezza("qualita_servizio_sanitario"));
    for x in &ministero(&v, "sanita").rosa {
        assert_eq!(x.obiettivo, visto + passo, "{}", x.nome);
    }
}

#[test]
fn i_messaggi_dei_ministri_nominano_il_dicastero() {
    let mut p = Partita::nuova(dati_base(), 1);
    p.comanda(Comando::ChiediDimissioni {
        dicastero: "sanita".into(),
    })
    .unwrap();
    let e = p
        .comanda(Comando::ChiediDimissioni {
            dicastero: "sanita".into(),
        })
        .unwrap_err();
    assert!(e.to_string().contains("Sanità"), "{e}");
    let e = p
        .comanda(Comando::ScegliSuccessore {
            dicastero: "giustizia".into(),
            indice: 0,
        })
        .unwrap_err();
    assert!(
        e.to_string().contains("Giustizia") && !e.to_string().contains("giustizia"),
        "{e}"
    );
}

/// La partita con l'evento `id` (nel catalogo) aperto.
fn con_evento_aperto(p: &Partita, id: &str) -> Partita {
    let evento = dati_base().evento(id).unwrap();
    comune::edita(p, |j| {
        j["eventi"]["aperti"] = serde_json::json!([{
            "evento": evento,
            "turno": p.vista().turno,
            "causa": { "tipo": "soglia", "misura": "sicurezza", "valore": 80.0, "soglia": 85.0 },
            "regione": null,
            "bersaglio": null,
            "tono": "ordinario",
            "costo": 0,
            "scelta": null,
        }]);
    })
    .unwrap()
}

#[test]
fn evento_nel_dicastero_mette_in_evidenza() {
    // L'agenda del dicastero e' in evidenza solo per l'evento aperto nel suo dicastero.
    let evidenza = |p: &Partita, dicastero: &str| {
        let v = p.vista();
        ministero(&v, dicastero)
            .agenda
            .as_ref()
            .map(|a| a.in_evidenza)
    };
    let coppie = [
        ("criminalita", "interno"),
        ("liste_attesa", "sanita"),
        ("carceri", "giustizia"),
        ("crisi_energetica", "ambiente_energia"),
    ];
    let mut provate = 0;
    for (evento, dicastero) in coppie {
        for seme in 1..=20 {
            let p = Partita::nuova(dati_base(), seme);
            if evidenza(&p, dicastero) != Some(false) {
                continue;
            }
            assert_eq!(
                evidenza(&con_evento_aperto(&p, evento), dicastero),
                Some(true)
            );
            let altro = coppie.iter().find(|c| c.1 != dicastero).unwrap().0;
            assert_eq!(
                evidenza(&con_evento_aperto(&p, altro), dicastero),
                Some(false)
            );
            provate += 1;
        }
    }
    assert!(provate > 0);
}
