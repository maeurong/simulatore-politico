mod comune;
use comune::*;
use sim_core::comando::Comando;
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
