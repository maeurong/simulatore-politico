mod comune;
use comune::*;
use sim_core::comando::Comando;
use sim_core::parlamento::CausaIntesa::NormaLontana;
use sim_core::parlamento::FormaMisura;
use sim_core::partita::Partita;

/// Il giocatore (7) governa con centrodestra (4) e destra (5). Opposizione: 175 su 400 alla Camera
/// e 84 su 200 al Senato; col centrodestra (30 e 16) 205 alla Camera, 100 al Senato.
fn seggi_al_limite(p: Partita) -> Partita {
    con_seggi(
        &p,
        &[0, 115, 22, 15, 30, 110, 23, 85],
        &[0, 56, 10, 5, 16, 57, 13, 43],
    )
}

/// Il giocatore governa col centrodestra; seggi scritti con `con_seggi` perché l'opposizione da
/// sola non abbia la metà in nessuna Camera e, col centrodestra, la superi alla Camera. Con
/// `ultimatum` il centrodestra è in ultimatum: vota sì sulla sfiducia ai ministri della destra.
fn al_governo_coi_seggi(ultimatum: bool) -> Partita {
    let p = seggi_al_limite(al_governo());
    let cd = partito("centrodestra");
    edita(&p, |j| {
        if ultimatum {
            j["intesa"]["valori"][cd] = 30.0.into();
            j["intesa"]["ultimatum"][cd] = "spartizione".into();
        }
    })
    .unwrap()
}

#[test]
fn l_opposizione_ia_sfiducia_solo_quando_passerebbe() {
    let mut p = al_governo_coi_seggi(true);
    chiudi_col_bilancio(&mut p);
    let (dicastero, promotore) = p.parlamento().sfiducia.expect("almeno l'individuale passa");
    assert_ne!(promotore, p.giocatore());
    let testo = match dicastero {
        Some(i) => {
            let m = p.ministri().dicasteri[i].ministro.as_ref().unwrap();
            assert_ne!(m.partito, partito("centrodestra"));
            format!(
                "Presenta la sfiducia a {}, {}: Centrodestra in ultimatum la voterebbe",
                m.nome,
                dati_base().dicasteri()[i].nome
            )
        }
        None => "Presenta la sfiducia al governo: Centrodestra in ultimatum la voterebbe".into(),
    };
    let promotore = &p.partiti()[promotore].id;
    assert!(
        (p.vista().mosse.iter()).any(|m| m.testo == testo && &m.partito == promotore),
        "{:?}",
        p.vista().mosse
    );
    let mut q = al_governo_coi_seggi(false);
    chiudi_col_bilancio(&mut q);
    assert!(
        q.parlamento().sfiducia.is_none(),
        "nessun alleato in ultimatum: niente sfiducia di bandiera"
    );
}

#[test]
fn una_sola_sfiducia_per_turno() {
    let p = all_opposizione();
    let mut p = edita(&p, |j| {
        j["parlamento"]["sfiducia"] = serde_json::json!([null, 0])
    })
    .unwrap();
    let e = p
        .comanda(Comando::Sfiducia { dicastero: None })
        .unwrap_err();
    assert!(e.to_string().contains("già presentata"), "{e}");
    let sfiducie = |p: &Partita| {
        (p.vista().esiti.iter())
            .filter(|e| e.forma == FormaMisura::Sfiducia)
            .count()
    };
    p.chiudi_turno();
    assert_eq!(sfiducie(&p), 1);
    // Il centrodestra esce (intesa sotto `soglia_uscita`): destra e giocatore, 195 su 400, sono
    // in crisi. La destra in ultimatum voterebbe la sfiducia ai ministri del giocatore, ma nel
    // turno di crisi la sfiducia automatica è già sul tavolo: l'opposizione IA non ne presenta
    // un'altra.
    let p = seggi_al_limite(al_governo());
    let (cd, destra) = (partito("centrodestra"), partito("destra"));
    let mut p = edita(&p, |j| {
        j["intesa"]["valori"][cd] = 10.0.into();
        j["intesa"]["valori"][destra] = 30.0.into();
        j["intesa"]["ultimatum"][destra] = "spartizione".into();
    })
    .unwrap();
    chiudi_col_bilancio(&mut p);
    assert_eq!(p.legislatura().crisi, Some(p.vista().turno));
    assert!(p.alleati().contains(&destra));
    assert!(p.parlamento().sfiducia.is_none());
    // Senza maggioranza, in chiusura la sola sfiducia automatica.
    p.chiudi_turno();
    assert_eq!(sfiducie(&p), 1);
}

/// La mozione IA presentata in apertura, l'ultimatum ritirato prima della chiusura: respinta, il
/// promotore perde `valence_sfiducia_respinta` (meno il ritorno del turno) rispetto alla gemella
/// senza mozione.
#[test]
fn la_sfiducia_ia_respinta_costa_valence_al_promotore() {
    let mut p = al_governo_coi_seggi(true);
    chiudi_col_bilancio(&mut p);
    let (_, promotore) = p.parlamento().sfiducia.unwrap();
    let cd = partito("centrodestra");
    let senza_ultimatum = |j: &mut serde_json::Value| {
        j["intesa"]["valori"][cd] = 60.0.into();
        j["intesa"]["ultimatum"][cd] = serde_json::Value::Null;
    };
    let mut p = edita(&p, senza_ultimatum).unwrap();
    let mut gemella = edita(&p, |j| {
        j["parlamento"]["sfiducia"] = serde_json::Value::Null;
    })
    .unwrap();
    chiudi_col_bilancio(&mut p);
    chiudi_col_bilancio(&mut gemella);
    let e = (p.vista().esiti.into_iter())
        .find(|e| e.forma == FormaMisura::Sfiducia)
        .unwrap();
    assert!(!e.approvata);
    let dati = dati_base();
    let atteso =
        dati.manopola("valence_sfiducia_respinta") * (1.0 - dati.manopola("ritorno_valence"));
    let persa = gemella.partiti()[promotore].valence - p.partiti()[promotore].valence;
    assert!((persa - atteso).abs() < 1e-9, "{persa}");
}

#[test]
fn la_sfiducia_ia_presentata_passa_se_nulla_cambia() {
    let mut p = al_governo_coi_seggi(true);
    chiudi_col_bilancio(&mut p);
    let (dicastero, _) = p.parlamento().sfiducia.unwrap();
    p.chiudi_turno();
    let e = (p.vista().esiti.into_iter())
        .find(|e| e.forma == FormaMisura::Sfiducia)
        .unwrap();
    assert!(e.approvata, "{e:?}");
    if let Some(i) = dicastero {
        assert!(
            e.nome.ends_with(&dati_base().dicasteri()[i].nome),
            "{}",
            e.nome
        );
    }
}

#[test]
fn sotto_un_governo_ia_l_opposizione_ia_sfiducia() {
    // Governo 1-5: 220 su 400 e 110 su 200; opposizione 180 e 90, firme IA (0 e 6) 100. Col
    // centrodestra (40 e 20) l'opposizione passa a 220 alla Camera. Intesa a 38: sotto
    // `soglia_ultimatum`, ma lontana da `soglia_uscita` anche dopo le pressioni del turno.
    let p = con_seggi(
        &all_opposizione(),
        &[40, 80, 20, 20, 40, 60, 60, 80],
        &[10, 40, 10, 10, 20, 30, 30, 50],
    );
    let cd = partito("centrodestra");
    let mut p = edita(&p, |j| {
        j["intesa"]["valori"][cd] = 38.0.into();
        j["intesa"]["ultimatum"][cd] = "spartizione".into();
    })
    .unwrap();
    p.chiudi_turno();
    let (dicastero, promotore) = p.parlamento().sfiducia.expect("la sfiducia IA passa");
    assert_eq!(promotore, partito("estrema_destra"));
    if let Some(i) = dicastero {
        let m = p.ministri().dicasteri[i].ministro.as_ref().unwrap();
        assert_ne!(m.partito, cd);
    }
    p.chiudi_turno();
    let e = (p.vista().esiti.into_iter())
        .find(|e| e.forma == FormaMisura::Sfiducia)
        .unwrap();
    let giocatore = &p.partiti()[p.giocatore()].id;
    assert!(e.linee.iter().any(|v| &v.partito == giocatore && v.si));
}

/// L'opposizione IA ha da sola la maggioranza alla Camera (213 su 400), nessun alleato in
/// ultimatum: la sfiducia al governo passerebbe coi soli voti dell'opposizione, e passa.
#[test]
fn la_sfiducia_al_governo_passa_coi_voti_dell_opposizione() {
    let mut p = con_seggi(
        &al_governo(),
        &[0, 140, 30, 20, 30, 80, 23, 77],
        &[0, 56, 10, 5, 16, 57, 13, 43],
    );
    chiudi_col_bilancio(&mut p);
    assert_eq!(p.parlamento().sfiducia, Some((None, partito("sinistra"))));
    let riga = "Presenta la sfiducia al governo: passerebbe coi voti dell'opposizione";
    assert!(
        (p.vista().mosse.iter()).any(|m| m.testo == riga && m.partito == "sinistra"),
        "{:?}",
        p.vista().mosse
    );
    chiudi_col_bilancio(&mut p);
    let e = (p.vista().esiti.into_iter())
        .find(|e| e.forma == FormaMisura::Sfiducia)
        .unwrap();
    assert!(
        e.approvata && e.nome == "Mozione di sfiducia al governo",
        "{e:?}"
    );
}

/// Sotto il governo IA 1-5, la destra in ultimatum ha più seggi di tutti (150 su 400): nella
/// maggioranza contigua dopo la caduta guiderebbe lei, e vota la sfiducia al governo.
#[test]
fn la_sfiducia_al_governo_nomina_l_alleato_in_ultimatum() {
    let p = con_seggi(
        &all_opposizione(),
        &[20, 60, 20, 20, 40, 150, 40, 50],
        &[10, 30, 10, 10, 20, 75, 20, 25],
    );
    let destra = partito("destra");
    let mut p = edita(&p, |j| {
        j["intesa"]["valori"][destra] = 38.0.into();
        j["intesa"]["ultimatum"][destra] = "spartizione".into();
    })
    .unwrap();
    p.chiudi_turno();
    let ed = partito("estrema_destra");
    assert_eq!(p.parlamento().sfiducia, Some((None, ed)));
    let riga = "Presenta la sfiducia al governo: Destra in ultimatum la voterebbe";
    assert!(
        (p.vista().mosse.iter()).any(|m| m.testo == riga && m.partito == "estrema_destra"),
        "{:?}",
        p.vista().mosse
    );
}

/// Nel turno del voto (19) il governo cade comunque al voto: l'opposizione IA non presenta né la
/// sfiducia, anche col centrodestra in ultimatum, né la proposta nel posto riservato, che
/// decadrebbe senza voto.
#[test]
fn nel_turno_del_voto_l_opposizione_ia_non_sfiducia_ne_propone() {
    let mut p = Partita::nuova(dati_base(), 1);
    while p.vista().turno < 18 {
        chiudi_col_bilancio(&mut p);
    }
    let cd = partito("centrodestra");
    let mut p = edita(&p, |j| {
        j["intesa"]["valori"][cd] = 38.0.into();
        j["intesa"]["ultimatum"][cd] = "spartizione".into();
        j["parlamento"]["sfiducia"] = serde_json::Value::Null;
    })
    .unwrap();
    chiudi_col_bilancio(&mut p);
    assert!(p.legislatura().turno_di_voto(p.vista().turno));
    assert!(p.alleati().contains(&cd));
    assert!(p.parlamento().sfiducia.is_none());
    let turno = p.vista().turno;
    assert!(
        (p.parlamento().misure.iter()).all(|m| m.promotore.is_none() || m.presentata != turno),
        "{:?}",
        p.vista().mosse
    );
}

/// Scelta 16 del piano: nel turno del posto riservato a un partito IA, la proposta-imbarazzo che
/// almeno un partito di governo voterebbe.
#[test]
fn la_proposta_imbarazzo_piace_alla_maggioranza() {
    let mut p = al_governo();
    // Avanza finché il posto riservato tocca a un partito IA (col seme 7 già al turno 2): si
    // guarda prima di chiudere, perché la chiusura svuota le mosse.
    let mut proposta = None;
    for _ in 0..8 {
        proposta = (p.parlamento().misure.iter())
            .find(|m| m.promotore.is_some_and(|j| j != p.giocatore()))
            .cloned();
        if proposta.is_some() {
            break;
        }
        chiudi_col_bilancio(&mut p);
    }
    let m = proposta.expect("un turno del posto riservato a un partito IA");
    let voce = p
        .vista()
        .calendario
        .into_iter()
        .find(|v| v.id == m.id)
        .unwrap();
    let inizio = format!("Porta nel posto riservato {}; ", voce.nome);
    assert!(
        (p.vista().mosse.iter()).any(|x| x.testo.starts_with(&inizio)),
        "{:?}",
        p.vista().mosse
    );
    // Almeno un partito di governo la voterebbe.
    let indice = |id: &str| p.partiti().iter().position(|x| x.id == id).unwrap();
    assert!((voce.linee.iter()).any(|l| l.si && p.governo().al_governo(indice(&l.partito))));
}

#[test]
fn il_giocatore_al_governo_sceglie_la_linea_solo_sulle_proposte_dell_opposizione() {
    let mut p = al_governo();
    presenta(&mut p, &[("eta_pensionabile", "70")]).unwrap();
    let id = p.parlamento().misure.last().unwrap().id;
    let e = p
        .comanda(Comando::Linea {
            misura: id,
            si: Some(false),
        })
        .unwrap_err();
    assert!(
        e.to_string()
            .contains("solo sulle proposte dell'opposizione"),
        "{e}"
    );
}

/// Scelta 15 del piano 8a: la proposta dell'opposizione approvata col sì del giocatore che governa
/// costa intesa agli alleati lontani, con la stessa norma lontana delle leggi del governo.
#[test]
fn la_proposta_dell_opposizione_approvata_costa_la_norma_lontana() {
    let mut p = al_governo();
    presenta(&mut p, &[("eta_pensionabile", "62")]).unwrap();
    let id = p.parlamento().misure.last().unwrap().id;
    let sinistra = partito("sinistra");
    // La legge diventa una proposta della sinistra; la maggioranza la fanno estrema sinistra e
    // sinistra, che ci guadagnano, col giocatore che sceglie il sì.
    let p = edita(&p, |j| {
        for m in j["parlamento"]["misure"].as_array_mut().unwrap() {
            if m["id"] == id {
                m["promotore"] = sinistra.into();
                m["proponente"] = serde_json::Value::Null;
            }
        }
    })
    .unwrap();
    let mut p = con_seggi(
        &p,
        &[60, 100, 20, 15, 30, 40, 15, 120],
        &[30, 50, 10, 8, 15, 20, 7, 60],
    );
    p.comanda(Comando::Linea {
        misura: id,
        si: Some(true),
    })
    .unwrap();
    for _ in 0..3 {
        if p.parlamento().misure.iter().all(|m| m.id != id) {
            break;
        }
        chiudi_col_bilancio(&mut p);
    }
    let esito = (p.parlamento().esiti.iter()).find(|e| e.misura == id);
    assert!(esito.is_some_and(|e| e.approvata), "{esito:?}");
    // Centrodestra (5,0) e destra (6,0) distano almeno 3 dai 62 anni (2,0).
    for a in ["centrodestra", "destra"].map(partito) {
        assert!(p.alleati().contains(&a));
        assert!(
            (p.intesa().variazioni.iter()).any(|x| x.partito == a && x.causa == NormaLontana),
            "{a}: {:?}",
            p.intesa().variazioni
        );
    }
}
