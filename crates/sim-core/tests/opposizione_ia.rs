mod comune;
use comune::*;
use sim_core::comando::Comando;
use sim_core::parlamento::CausaIntesa::NormaLontana;
use sim_core::parlamento::{FormaMisura, TipoReferendum};
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

/// Il posto riservato è uno per turno: la proposta IA del turno prima, che si vota ora, non toglie
/// il posto al giocatore all'opposizione; una seconda proposta nello stesso turno sì.
#[test]
fn la_proposta_ia_al_voto_non_occupa_il_posto_del_turno() {
    let mut p = all_opposizione();
    for _ in 0..8 {
        if p.vista().posto_riservato {
            break;
        }
        p.chiudi_turno();
    }
    assert!(
        p.vista().posto_riservato,
        "il posto non arriva al giocatore"
    );
    let (g, turno) = (p.giocatore(), p.vista().turno);
    assert!(
        (p.parlamento().misure.iter())
            .any(|m| m.promotore.is_some_and(|j| j != g) && m.presentata + 1 == turno),
        "nessuna proposta IA del turno prima in esame"
    );
    let proposta = |d: &str, g: &str| Comando::PresentaProposta {
        valori: vec![sim_core::parlamento::ValoreProposto {
            disposizione: d.into(),
            gradino: g.into(),
        }],
    };
    p.comanda(proposta("licenziamento", "reintegro")).unwrap();
    let e = (p.comanda(proposta("tetto_contante", "1000"))).unwrap_err();
    assert!(e.to_string().contains("già occupato"), "{e}");
}

/// La riforma a 62 anni presentata dal giocatore che governa, riscritta come proposta della
/// sinistra, con la linea `si` del giocatore e seggi dove passa anche col suo no: gli alleati che
/// pagano la norma lontana nella chiusura del voto. La proposta IA già in esame si toglie, così la
/// proposta è l'unica norma al voto.
fn norma_lontana_della_proposta(si: bool) -> Vec<usize> {
    let mut p = al_governo();
    togli_opposizione_ia(&mut p);
    presenta(&mut p, &[("eta_pensionabile", "62")]).unwrap();
    let id = p.parlamento().misure.last().unwrap().id;
    let sinistra = partito("sinistra");
    let p = edita(&p, |j| {
        for m in j["parlamento"]["misure"].as_array_mut().unwrap() {
            if m["id"] == id {
                m["promotore"] = sinistra.into();
                m["proponente"] = serde_json::Value::Null;
            }
        }
    })
    .unwrap();
    // Estrema sinistra e sinistra, che ci guadagnano, hanno la maggioranza in entrambe le Camere.
    let mut p = con_seggi(
        &p,
        &[120, 140, 20, 15, 30, 40, 15, 20],
        &[60, 70, 10, 8, 15, 20, 7, 10],
    );
    p.comanda(Comando::Linea {
        misura: id,
        si: Some(si),
    })
    .unwrap();
    let mut turno = p.vista().turno;
    for _ in 0..3 {
        if p.parlamento().misure.iter().all(|m| m.id != id) {
            break;
        }
        turno = p.vista().turno;
        chiudi_col_bilancio(&mut p);
    }
    let esito = (p.parlamento().esiti.iter()).find(|e| e.misura == id);
    assert!(esito.is_some_and(|e| e.approvata), "{si}: {esito:?}");
    (p.intesa().variazioni.iter())
        .filter(|x| x.turno == turno && x.causa == NormaLontana)
        .map(|x| x.partito)
        .collect()
}

/// Scelta 15 del piano 8a: la proposta dell'opposizione approvata costa la norma lontana agli
/// alleati lontani solo col sì del governo del giocatore, che la fa passare.
#[test]
fn la_proposta_dell_opposizione_costa_la_norma_lontana_solo_col_si_del_governo() {
    // Centrodestra (5,0) e destra (6,0) distano almeno 3 dai 62 anni (2,0).
    let alleati = ["centrodestra", "destra"].map(partito);
    let col_si = norma_lontana_della_proposta(true);
    assert!(alleati.iter().all(|a| col_si.contains(a)), "{col_si:?}");
    assert_eq!(norma_lontana_della_proposta(false), Vec::<usize>::new());
}

/// `senza_shock()` con `astensione_abrogativo` a `astensione`.
fn con_astensione(astensione: &str) -> sim_core::dati::FileDati {
    imposta(
        senza_shock(),
        "manopole",
        "[astensione_abrogativo]\nvalore = ",
        astensione,
    )
}

/// Le tutele crescenti all'estremo destro e il reintegro al centro, come in tests/referendum.rs:
/// l'abrogazione del licenziamento (7,0 → 4,0) piace a quasi tutti.
fn col_licenziamento(f: sim_core::dati::FileDati) -> sim_core::dati::FileDati {
    let f = sostituisci(
        f,
        "disposizioni",
        "nome = \"tutele crescenti\", posizione = 4.0",
        "nome = \"tutele crescenti\", posizione = 7.0",
    );
    sostituisci(
        f,
        "disposizioni",
        "nome = \"reintegro\", posizione = 2.0",
        "nome = \"reintegro\", posizione = 4.0",
    )
}

/// Il reddito categoriale a 6,5: l'abrogazione verso il «non istituito» (5,5) vince, ma con un
/// margine più stretto del licenziamento, perché le due posizioni sono vicine.
fn col_reddito(f: sim_core::dati::FileDati) -> sim_core::dati::FileDati {
    sostituisci(
        f,
        "disposizioni",
        "nome = \"categoriale\", posizione = 4.0",
        "nome = \"categoriale\", posizione = 6.5",
    )
}

fn partita_di_prova(f: &sim_core::dati::FileDati) -> Partita {
    Partita::nuova(sim_core::dati::DatiGioco::da_testi(f).unwrap(), 1)
}

fn licenziamento_abrogabile(astensione: &str) -> Partita {
    partita_di_prova(&col_licenziamento(con_astensione(astensione)))
}

/// Chiude il turno e toglie le mosse IA dell'apertura: la proposta IA sul licenziamento lo
/// metterebbe in esame.
fn chiudi_e_togli(p: &mut Partita) {
    chiudi_col_bilancio(p);
    togli_opposizione_ia(p);
}

/// La posizione dichiarata dei partiti `posizioni` (id, posizione), scritta con `edita`.
fn con_posizioni(p: &Partita, posizioni: &[(&str, f64)]) -> Partita {
    edita(p, |j| {
        for &(id, x) in posizioni {
            j["partiti"][partito(id)]["posizione_dichiarata"] = x.into();
        }
    })
    .unwrap()
}

/// L'abrogativo in corso: id della disposizione e del promotore.
fn abrogativo(p: &Partita) -> Option<(String, String)> {
    let r = p.parlamento().referendum.as_ref()?;
    let d = dati_base().disposizioni()[r.valori[0].0].id.clone();
    Some((d, p.partiti()[r.promotore?].id.clone()))
}

fn chiede(p: &Partita) -> bool {
    (p.vista().mosse.iter()).any(|m| m.testo.starts_with("Chiede il referendum"))
}

#[test]
fn l_opposizione_ia_chiede_solo_referendum_vincenti() {
    let mut p = licenziamento_abrogabile("-5.0");
    // Ottobre-dicembre: fuori finestra. La proposta IA sul licenziamento lo metterebbe in esame.
    chiudi_col_bilancio(&mut p);
    togli_opposizione_ia(&mut p);
    assert!(p.parlamento().referendum.is_none());
    chiudi_col_bilancio(&mut p);
    let r = (p.parlamento().referendum.clone()).expect("un abrogativo vincente c'è");
    let promotore = r.promotore.unwrap();
    assert_ne!(promotore, p.giocatore());
    assert!(!p.governo().al_governo(promotore));
    assert!(r.firme && r.richiesto);
    let riga = (p.vista().mosse.into_iter())
        .find(|m| m.testo.starts_with("Chiede il referendum su "))
        .expect("la riga della mossa");
    assert_eq!(riga.partito, p.partiti()[promotore].id);
    assert!(
        riga.testo.ends_with("% e il quorum") && riga.testo.contains(','),
        "{}",
        riga.testo
    );
    // Il giocatore ha la precedenza: col referendum chiesto nel turno l'IA non ne chiede altri.
    let mut q = licenziamento_abrogabile("-5.0");
    chiudi_col_bilancio(&mut q);
    q.promuovi_referendum("sinistra", "nucleare").unwrap();
    chiudi_col_bilancio(&mut q);
    let r = q.parlamento().referendum.as_ref().unwrap();
    assert_eq!(q.partiti()[r.promotore.unwrap()].id, "sinistra");
    assert!(!(q.vista().mosse.iter()).any(|m| m.testo.starts_with("Chiede il referendum")));
}

/// Lo stato di `l_opposizione_ia_chiede_solo_referendum_vincenti`, che col quorum chiede il
/// licenziamento, ma con l'astensione alta.
#[test]
fn senza_quorum_atteso_l_opposizione_ia_non_chiede_referendum() {
    let mut p = licenziamento_abrogabile("5.0");
    chiudi_e_togli(&mut p);
    chiudi_col_bilancio(&mut p);
    assert_eq!(abrogativo(&p), None);
    assert!(!chiede(&p));
}

/// Col quorum ma senza le posizioni spostate il sì perde ovunque (sul sondaggio del turno 1 il no
/// vince di 1-10 milioni): nessun referendum. Il nucleare non è referendabile: il suo quesito, che
/// abroga solo l'arrivo, finisce pari senza nessuno che ci guadagni, e nasconderebbe gli altri.
#[test]
fn col_quorum_ma_col_si_perdente_l_opposizione_ia_non_chiede() {
    let f = sostituisci(
        con_astensione("-5.0"),
        "disposizioni",
        "referendabile = true\ngruppi = []\npartenza = \"vietato\"",
        "referendabile = false\ngruppi = []\npartenza = \"vietato\"",
    );
    let mut p = partita_di_prova(&f);
    chiudi_e_togli(&mut p);
    chiudi_col_bilancio(&mut p);
    assert_eq!(abrogativo(&p), None);
    assert!(!chiede(&p));
}

/// Licenziamento (7,0 → 4,0) e reddito (6,5 → 5,5) vincono entrambi: l'IA chiede quello col
/// margine sì − no più ampio, il licenziamento; da solo vince il reddito. Promotore: chi guadagna
/// di più dall'abrogazione del licenziamento, con le posizioni spostate perché i guadagni
/// differiscano: estrema sinistra a 5,0 (+1), sinistra a 2,0 (+3), centrosinistra a 4,5 (+2),
/// centro a 6,0 (−1), estrema destra a 7,0 (−3).
#[test]
fn vince_il_margine_piu_ampio_e_il_promotore_che_guadagna_di_piu() {
    let chiesto = |f: sim_core::dati::FileDati| {
        let mut p = partita_di_prova(&f);
        chiudi_e_togli(&mut p);
        let posizioni = [
            ("estrema_sinistra", 5.0),
            ("centrosinistra", 4.5),
            ("centro", 6.0),
        ];
        let mut p = con_posizioni(&p, &posizioni);
        chiudi_col_bilancio(&mut p);
        abrogativo(&p)
    };
    let entrambi = col_reddito(col_licenziamento(con_astensione("-5.0")));
    assert_eq!(
        chiesto(entrambi),
        Some(("licenziamento".into(), "sinistra".into()))
    );
    let solo_reddito = chiesto(col_reddito(con_astensione("-5.0")));
    assert_eq!(solo_reddito.map(|r| r.0).as_deref(), Some("reddito_minimo"));
}

/// Il licenziamento vince, ma i partiti IA all'opposizione stanno tutti a 6,0 o oltre: nessuno
/// guadagna dall'abrogazione verso il 4,0, quindi niente referendum.
#[test]
fn senza_un_promotore_che_ci_guadagni_niente_referendum() {
    let mut p = licenziamento_abrogabile("-5.0");
    chiudi_e_togli(&mut p);
    let a_destra = ["estrema_sinistra", "sinistra", "centrosinistra", "centro"].map(|id| (id, 6.0));
    let mut p = con_posizioni(&p, &a_destra);
    chiudi_col_bilancio(&mut p);
    assert_eq!(abrogativo(&p), None);
    assert!(!chiede(&p));
}

/// Scelta 8: nel turno in cui si vota l'abrogativo del giocatore lo slot si libera, ma il giocatore
/// non ha ancora avuto il turno per chiederne un altro: l'IA aspetta il turno dopo.
#[test]
fn nel_turno_del_voto_dell_abrogativo_l_ia_non_chiede_il_successivo() {
    let mut p = licenziamento_abrogabile("-5.0");
    chiudi_e_togli(&mut p);
    p.promuovi_referendum("sinistra", "nucleare").unwrap();
    // Turni 1-5: lo slot è occupato. Il voto è nella chiusura del turno 6.
    for _ in 1..6 {
        chiudi_e_togli(&mut p);
    }
    assert_eq!(p.vista().turno, 6);
    chiudi_e_togli(&mut p);
    assert!((p.vista().esiti_referendum.iter()).any(|e| e.tipo == TipoReferendum::Abrogativo));
    assert_eq!(abrogativo(&p), None);
    chiudi_col_bilancio(&mut p);
    assert_eq!(
        abrogativo(&p).map(|r| r.0).as_deref(),
        Some("licenziamento")
    );
}
