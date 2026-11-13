mod comune;
use comune::*;
use sim_core::Partita;
use sim_core::esito::TipoVoce;
use sim_core::partiti::{Regola, Vocazione};

#[test]
fn ogni_partito_ha_la_sua_regola() {
    let v = fondata(4.5).vista();
    let regola = |id: &str| v.partiti.iter().find(|p| p.id == id).unwrap().regola;
    let attese = [
        ("estrema_sinistra", Regola::Sticker),
        ("sinistra", Regola::Aggregator),
        ("centrosinistra", Regola::Hunter),
        ("centro", Regola::Governator),
        ("centrodestra", Regola::Hunter),
        ("destra", Regola::Aggregator),
        ("estrema_destra", Regola::Sticker),
        ("giocatore", Regola::Sticker),
    ];
    for (id, r) in attese {
        assert_eq!(regola(id), r, "{id}");
    }
    assert!(v.partiti.iter().all(|p| !p.sciolto));
}

#[test]
fn carica_rifiuta_un_partito_in_piu_non_valido() {
    let p = fondata(4.5);
    let e = edita(&p, |j| {
        let mut intruso = j["partiti"][0].clone();
        intruso["id"] = "intruso".into();
        j["partiti"].as_array_mut().unwrap().push(intruso);
    })
    .unwrap_err();
    assert!(
        e.to_string().contains("partito in più non valido: intruso"),
        "{e}"
    );
    let e = edita(&p, |j| {
        let g = j["giocatore"].as_u64().unwrap() as usize;
        j["partiti"][g]["sciolto"] = true.into();
    })
    .unwrap_err();
    assert!(
        e.to_string()
            .contains("il partito del giocatore non può essere sciolto"),
        "{e}"
    );
}

/// Gli altri rifiuti dell'elenco dei partiti, sulla partita fondata (sette dello scenario più il
/// giocatore all'indice 7).
#[test]
fn carica_rifiuta_un_elenco_di_partiti_storto() {
    let p = fondata(4.5);
    let rifiuto = |f: &dyn Fn(&mut serde_json::Value)| edita(&p, f).unwrap_err().to_string();
    let e = rifiuto(&|j| {
        let v = j["partiti"].as_array_mut().unwrap();
        v.truncate(6);
    });
    assert!(e.contains("partiti ha 6 voci, ne servono almeno 7"), "{e}");
    let e = rifiuto(&|j| j["partiti"].as_array_mut().unwrap().swap(0, 1));
    assert!(
        e.contains("partito dello scenario mancante o fuori ordine: estrema_sinistra"),
        "{e}"
    );
    let e = rifiuto(&|j| {
        let copia = j["partiti"][0].clone();
        j["partiti"].as_array_mut().unwrap().push(copia);
    });
    assert!(e.contains("partito ripetuto: estrema_sinistra"), "{e}");
    let e = rifiuto(&|j| j["giocatore"] = 0.into());
    assert!(
        e.contains("giocatore fuori dai partiti o diverso dal partito fondato"),
        "{e}"
    );
    let e = rifiuto(&|j| {
        let mut nuovo = j["partiti"][0].clone();
        nuovo["id"] = "nuovo_9".into();
        j["partiti"].as_array_mut().unwrap().push(nuovo);
    });
    assert!(e.contains("partito in più non valido: nuovo_9"), "{e}");
}

#[test]
fn ogni_partito_ha_vocazione_e_leader() {
    let v = fondata(4.5).vista();
    let voce = |id: &str| v.partiti.iter().find(|p| p.id == id).unwrap().clone();
    assert_eq!(voce("estrema_sinistra").vocazione, Vocazione::Identitaria);
    assert_eq!(voce("estrema_destra").vocazione, Vocazione::Identitaria);
    assert_eq!(voce("centro").vocazione, Vocazione::Governista);
    assert_eq!(voce("sinistra").vocazione, Vocazione::Mista);
    assert_eq!(voce("destra").leader, "Davide Ranieri");
    assert_eq!(voce("giocatore").vocazione, Vocazione::Mista);
    assert_eq!(voce("giocatore").leader, "");
}

#[test]
fn leader_vuoto_rifiutato_col_nome() {
    let f = sostituisci(
        file_base(),
        "scenario",
        "leader = \"Giorgio Valli\"",
        "leader = \"\"",
    );
    let e = sim_core::dati::DatiGioco::da_testi(&f).unwrap_err();
    assert!(e.to_string().contains("leader vuoto: centro"), "{e}");
}

/// Dopo il voto del turno zero, in trattativa con l'incarico IA a estrema sinistra, sinistra e
/// centrosinistra: alla formazione la destra, partito del Presidente del Consiglio uscente, è
/// fuori dal governo. `forma_governo` prende la coalizione dell'incarico IA senza contare i seggi.
fn sinistra_al_governo() -> Partita {
    let mut p = fondata(4.5);
    p.chiudi_turno(); // voto
    edita(&p, |j| {
        j["legislatura"]["incarico"] = serde_json::json!({ "Ia": { "coalizione": [0, 1, 2] } });
    })
    .unwrap()
}

#[test]
fn chi_perde_il_governo_cambia_leader() {
    let mut p = sinistra_al_governo();
    let destra = partito("destra");
    assert_eq!(p.governo().presidente, destra);
    let prima = p.partiti()[destra].clone();
    p.chiudi_turno(); // trattativa: governa la sinistra
    let dopo = &p.partiti()[destra];
    assert_ne!(dopo.leader, prima.leader);
    let luna = dati_base().manopola("luna_di_miele_leader");
    assert!((dopo.valence - (dopo.valence_base + luna)).abs() < 1e-9);
    let leader = &p.partiti()[destra].leader;
    let fatto = format!("Nuovo leader: {leader}");
    let causa = "la perdita del governo";
    assert!(ha_voce(
        &p,
        TipoVoce::CambioDiLeader,
        "destra",
        &fatto,
        causa
    ));
}

#[test]
fn il_giocatore_non_cambia_mai_leader() {
    let p = all_opposizione();
    let g = p.giocatore();
    let turno = p.vista().turno;
    // Il giocatore aveva il 90% al voto precedente, e il voto è questo turno: cala oltre soglia.
    let mut p = edita(&p, |j| {
        j["legislatura"]["scadenza"] = serde_json::json!(turno);
        j["legislatura"]["elezioni"][0]["voti"][g]["quota"] = serde_json::json!(0.9);
    })
    .unwrap();
    for _ in 0..24 {
        p.chiudi_turno();
        assert_eq!(p.partiti()[g].leader, "");
        // Minaccia e scissione toccano anche il partito del giocatore (piano 8b), il leader no.
        assert!(
            (voci(&p, TipoVoce::CambioDiLeader).iter())
                .all(|v| v.partito.as_deref() != Some("giocatore")),
            "{:?}",
            p.vista().esito
        );
    }
    // Precondizione: il calo c'è stato (il voto è avvenuto con la quota di prima a 0,9).
    assert!(p.legislatura().elezioni.len() >= 2);
}

#[test]
fn una_sconfitta_un_solo_cambio() {
    let destra = partito("destra");
    // Il voto: la destra, al governo, cala oltre soglia rispetto a un voto precedente al 90%.
    let mut probe = fondata(4.5);
    probe.chiudi_turno();
    let mut esito = serde_json::to_value(&probe.legislatura().elezioni[0]).unwrap();
    esito["voti"][destra]["quota"] = serde_json::json!(0.9);
    let mut p = edita(&fondata(4.5), |j| {
        j["legislatura"]["elezioni"] = serde_json::json!([esito]);
    })
    .unwrap();
    assert_eq!(p.governo().presidente, destra);
    let cambi = |p: &Partita| {
        (voci(p, TipoVoce::CambioDiLeader).iter())
            .filter(|v| v.partito.as_deref() == Some("destra"))
            .count()
    };
    p.chiudi_turno(); // voto: calo
    assert_eq!(cambi(&p), 1);
    let dopo_il_voto = p.partiti()[destra].leader.clone();
    let mut p = edita(&p, |j| {
        j["legislatura"]["incarico"] = serde_json::json!({ "Ia": { "coalizione": [0, 1, 2] } });
    })
    .unwrap();
    p.chiudi_turno(); // trattativa: la destra resta fuori dal governo
    assert!(!p.governo().al_governo(destra));
    assert_eq!(cambi(&p), 0);
    assert_eq!(p.partiti()[destra].leader, dopo_il_voto);
}

#[test]
fn primo_voto_nessun_cambio() {
    let mut p = fondata(4.5);
    let prima: Vec<_> = p.partiti().iter().map(|x| x.leader.clone()).collect();
    p.chiudi_turno();
    assert_eq!(p.legislatura().elezioni.len(), 1);
    // Al turno zero i partiti IA promettono anche senza seggi (R28); le casse IA di `fondata` sono
    // vuote, quindi nessuno spende: oltre le promesse, nessuna voce dei partiti IA. Restano fuori
    // gli eventi del caso, risolti e lievi, gli atti dei ministri del governo uscente e le voci di
    // ogni chiusura: spread, sondaggio, prima stima, revisione, entrate di cassa.
    let esito = p.vista().esito;
    let fuori = |t: TipoVoce| {
        matches!(
            t,
            TipoVoce::EventoRisolto
                | TipoVoce::EmergenzaLieve
                | TipoVoce::AttoEseguito
                | TipoVoce::AttoSaltato
                | TipoVoce::CantiereAvviato
                | TipoVoce::CantiereFermo
                | TipoVoce::CantiereFinito
                | TipoVoce::Spread
                | TipoVoce::Sondaggio
                | TipoVoce::PrimaStima
                | TipoVoce::Revisione
                | TipoVoce::EntrateDiCassa
        )
    };
    assert!(
        (esito.iter())
            .filter(|v| !fuori(v.tipo))
            .all(|v| v.tipo == TipoVoce::PromessaFatta),
        "{esito:?}"
    );
    let dopo: Vec<_> = p.partiti().iter().map(|x| x.leader.clone()).collect();
    assert_eq!(prima, dopo);
}

#[test]
fn calo_sotto_soglia_non_cambia_leader() {
    let mut p = sinistra_al_governo();
    p.chiudi_turno(); // governo della sinistra
    let destra = partito("destra");
    let turno = p.vista().turno;
    let soglia = dati_base().manopola("calo_cambio_leader");
    let voto = |p: &Partita, prima: Option<f64>| {
        edita(p, |j| {
            j["legislatura"]["scadenza"] = serde_json::json!(turno);
            if let Some(q) = prima {
                j["legislatura"]["elezioni"][0]["voti"][destra]["quota"] = serde_json::json!(q);
            }
        })
        .unwrap()
    };
    // Sonda: la quota che la destra prende al voto, poi un voto precedente di poco sotto soglia.
    let mut sonda = voto(&p, None);
    sonda.chiudi_turno();
    let quota = sonda.legislatura().elezioni[0].voti[destra].quota;
    let mut p = voto(&p, Some(quota + soglia - 0.01));
    let prima: Vec<_> = p.partiti().iter().map(|x| x.leader.clone()).collect();
    p.chiudi_turno();
    let dopo: Vec<_> = p.partiti().iter().map(|x| x.leader.clone()).collect();
    assert_eq!(prima, dopo);
    assert!(
        (voci(&p, TipoVoce::CambioDiLeader).iter()).all(|v| !v.causa.starts_with("il calo di"))
    );
}

#[test]
fn salvataggio_porta_esito_e_leader_nuovo() {
    let mut p = sinistra_al_governo();
    p.chiudi_turno(); // la destra cambia leader
    assert!(!p.vista().esito.is_empty());
    let mut q = Partita::carica(&p.salva().unwrap()).unwrap();
    assert_eq!(q.vista().esito, p.vista().esito);
    assert_eq!(q.salva().unwrap(), p.salva().unwrap());
    p.chiudi_turno();
    q.chiudi_turno();
    assert_eq!(q.salva().unwrap(), p.salva().unwrap());
}

#[test]
fn salvataggio_con_leader_o_esito_sbagliati_rifiutato_col_nome() {
    let p = sinistra_al_governo();
    let destra = partito("destra");
    let ko = |f: fn(&mut serde_json::Value, usize), cosa: &str| {
        let e = edita(&p, |j| f(j, destra)).unwrap_err();
        assert!(e.to_string().contains(cosa), "{e}");
    };
    ko(|j, d| j["partiti"][d]["leader"] = "  ".into(), "destra");
    ko(
        |j, _| {
            let g = j["giocatore"].as_u64().unwrap() as usize;
            j["partiti"][g]["leader"] = "Mario Rossi".into();
        },
        "giocatore",
    );
    ko(
        |j, _| {
            let mut v = j["esito"][0].clone();
            v["partito"] = "fantasma".into();
            j["esito"] = serde_json::json!([v]);
        },
        "fantasma",
    );
}

#[test]
fn l_esito_si_azzera_a_ogni_chiusura() {
    let mut p = sinistra_al_governo();
    p.chiudi_turno();
    assert!(!p.vista().esito.is_empty());
    p.chiudi_turno();
    assert!(
        (voci(&p, TipoVoce::CambioDiLeader).iter()).all(|v| v.causa != "la perdita del governo")
    );
}

#[test]
fn chi_resta_alleato_non_cambia_leader() {
    let destra = partito("destra");
    let mut p = edita(&sinistra_al_governo(), |j| {
        j["legislatura"]["incarico"] =
            serde_json::json!({ "Ia": { "coalizione": [0, 1, 2, destra] } });
    })
    .unwrap();
    assert_eq!(p.governo().presidente, destra); // uscente
    let prima = p.partiti()[destra].leader.clone();
    p.chiudi_turno();
    // Precondizione: alleata del governo nuovo, ma senza la presidenza.
    assert!(p.governo().al_governo(destra));
    assert_ne!(p.governo().presidente, destra);
    assert_eq!(p.partiti()[destra].leader, prima);
    assert!(
        (p.vista().esito.iter()).all(|v| v.partito.as_deref() != Some("destra")),
        "{:?}",
        p.vista().esito
    );
}

#[test]
fn il_calo_al_voto_cambia_leader() {
    let mut p = sinistra_al_governo();
    p.chiudi_turno(); // governo della sinistra
    let destra = partito("destra");
    let turno = p.vista().turno;
    // La destra aveva il 90% al voto precedente, e il voto è questo turno.
    let mut p = edita(&p, |j| {
        j["legislatura"]["scadenza"] = serde_json::json!(turno);
        j["legislatura"]["elezioni"][0]["voti"][destra]["quota"] = serde_json::json!(0.9);
    })
    .unwrap();
    let prima = p.partiti()[destra].leader.clone();
    p.chiudi_turno();
    let dopo = &p.partiti()[destra];
    assert_ne!(dopo.leader, prima);
    let luna = dati_base().manopola("luna_di_miele_leader");
    assert!((dopo.valence - (dopo.valence_base + luna)).abs() < 1e-9);
    let quota = p.legislatura().elezioni[0].voti[destra].quota;
    let punti = format!("{:.1}", (0.9 - quota) * 100.0).replace('.', ",");
    let fatto = format!("Nuovo leader: {}", dopo.leader);
    let causa = format!("il calo di {punti} punti percentuali al voto");
    let testi: Vec<_> = (voci(&p, TipoVoce::CambioDiLeader).into_iter())
        .filter(|v| v.partito.as_deref() == Some("destra"))
        .map(|v| (v.fatto, v.causa))
        .collect();
    assert_eq!(testi, [(fatto, causa)]);
}
