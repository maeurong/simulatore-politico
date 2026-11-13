mod comune;
use comune::{
    all_opposizione, chiudi_col_bilancio, chiudi_senza_opposizione_ia, con_seggi, dati_base, edita,
    ferma_tutti, partito, presenta, voci,
};
use sim_core::esito::{Criticita, TipoVoce, VoceEsito};
use sim_core::legislatura::{Fase, Incarico};
use sim_core::parlamento::{Decreto, FormaMisura};
use sim_core::{Comando, Partita};

/// La destra e il centrodestra in minoranza: 150 su 400, 75 su 200. Le prove chiudono con
/// `chiudi_senza_opposizione_ia`: l'opposizione IA, in maggioranza, presenterebbe la sfiducia in
/// apertura, e queste prove vogliono il governo di minoranza in piedi fino all'esercizio
/// provvisorio.
const MINORANZA_C: [u32; 7] = [40, 80, 60, 30, 50, 100, 40];
const MINORANZA_S: [u32; 7] = [20, 40, 30, 15, 25, 50, 20];

#[test]
fn dimettersi_porta_al_ribaltone_o_al_voto() {
    let mut p = Partita::nuova(dati_base(), 1);
    p.comanda(Comando::Dimettiti).unwrap();
    assert!(p.vista().dimissioni);
    p.chiudi_turno();
    let l = p.legislatura();
    match l.fase {
        Fase::Trattativa => {
            let Some(Incarico::Ia { coalizione }) = &l.incarico else {
                panic!("{:?}", l.incarico)
            };
            assert!(!coalizione.contains(&p.giocatore()));
            let v = voci(&p, TipoVoce::Incarico);
            assert_eq!(v.len(), 1, "{v:?}");
            assert_eq!(v[0].fatto, "L'incarico passa al Capo dello Stato");
            assert_eq!(
                v[0].causa,
                "la maggioranza contigua trovata dopo la caduta non ti comprende"
            );
            p.chiudi_turno();
            assert!(!p.governa() && p.legislatura().fase == Fase::Ordinaria);
        }
        Fase::Sciolta => assert!(p.legislatura().turno_di_voto(p.vista().turno)),
        Fase::Ordinaria => panic!("il governo non è caduto"),
    }
}

#[test]
fn fiducia_respinta_fa_dimettere() {
    let mut p = con_seggi(&Partita::nuova(dati_base(), 1), &MINORANZA_C, &MINORANZA_S);
    presenta(&mut p, &[("tetto_contante", "1000")]).unwrap();
    let id = p.vista().calendario[0].id;
    p.comanda(Comando::PoniFiducia { misura: id }).unwrap();
    p.chiudi_turno();
    assert!(p.vista().esiti.iter().any(|e| e.fiducia_respinta));
    assert!(!p.governa());
    assert_ne!(p.legislatura().fase, Fase::Ordinaria);
    let v = voci(&p, TipoVoce::FiduciaRespinta);
    assert_eq!(v.len(), 1, "{v:?}");
    assert_eq!((v[0].criticita, v[0].precedenza), (Criticita::Critica, 3));
    assert_eq!(v[0].misura, Some(id));
    assert!(
        v[0].fatto.starts_with("Respinta la fiducia su «")
            && v[0].fatto.ends_with("; il governo è dimissionario"),
        "{}",
        v[0].fatto
    );
    assert!(voci(&p, TipoVoce::LeggeRespinta).is_empty());
}

/// La fiducia respinta sulla conversione: una voce sola, coi fondi tornati.
#[test]
fn fiducia_respinta_sul_decreto() {
    let mut p = con_seggi(&Partita::nuova(dati_base(), 1), &MINORANZA_C, &MINORANZA_S);
    let decreto = Decreto::Fondi {
        da: "giustizia".into(),
        a: "sanita".into(),
        importo: 1_000_000,
    };
    p.comanda(Comando::EmanaDecreto { decreto }).unwrap();
    ferma_tutti(&mut p);
    let id = (p.vista().calendario.iter())
        .find(|m| m.forma == FormaMisura::Conversione)
        .unwrap()
        .id;
    p.comanda(Comando::PoniFiducia { misura: id }).unwrap();
    p.chiudi_turno();
    let v = voci(&p, TipoVoce::FiduciaRespinta);
    assert_eq!(v.len(), 1, "{v:?}");
    assert!(
        v[0].fatto
            .starts_with("Respinta la fiducia su «Decreto: fondi da Giustizia a Sanità»; Camera ")
            && v[0]
                .fatto
                .contains("; tornano 1.000.000 euro a Giustizia; ")
            && v[0].fatto.ends_with("; il governo è dimissionario"),
        "{}",
        v[0].fatto
    );
    assert!(voci(&p, TipoVoce::DecretoDecaduto).is_empty());
}

/// La fiducia respinta sul bilancio di ottobre: il bilancio presentato cade, e da gennaio si va in
/// esercizio provvisorio.
#[test]
fn fiducia_respinta_sul_bilancio() {
    let mut p = con_seggi(&Partita::nuova(dati_base(), 1), &MINORANZA_C, &MINORANZA_S);
    for _ in 0..4 {
        chiudi_senza_opposizione_ia(&mut p); // fino a ottobre-dicembre 2027
    }
    let b = p.bozza_bilancio().unwrap();
    p.comanda(Comando::PresentaBilancio { bilancio: b })
        .unwrap();
    let id = (p.vista().calendario.iter())
        .find(|m| m.forma == FormaMisura::Bilancio)
        .unwrap()
        .id;
    p.comanda(Comando::PoniFiducia { misura: id }).unwrap();
    p.chiudi_turno();
    let v = voci(&p, TipoVoce::FiduciaRespinta);
    assert_eq!(v.len(), 1, "{v:?}");
    assert!(
        v[0].fatto.starts_with("Respinta la fiducia su «")
            && v[0].fatto.ends_with(
                "; il bilancio presentato cade, esercizio provvisorio da gennaio; il governo è \
                 dimissionario"
            ),
        "{}",
        v[0].fatto
    );
    assert!(voci(&p, TipoVoce::BilancioRespinto).is_empty());
}

#[test]
fn bilancio_mancato_anche_in_esercizio_provvisorio() {
    let mut p = con_seggi(&Partita::nuova(dati_base(), 1), &MINORANZA_C, &MINORANZA_S);
    for _ in 0..4 {
        chiudi_senza_opposizione_ia(&mut p); // fino a ottobre-dicembre 2027
    }
    let b = p.bozza_bilancio().unwrap();
    p.comanda(Comando::PresentaBilancio { bilancio: b })
        .unwrap();
    chiudi_senza_opposizione_ia(&mut p); // respinto: esercizio provvisorio nel 2028
    assert!(p.governa());
    let b = p.bozza_bilancio().unwrap();
    p.comanda(Comando::PresentaBilancio { bilancio: b })
        .unwrap();
    p.chiudi_turno(); // respinto di nuovo: dimissioni
    assert!(!p.governa());
}

#[test]
fn dimissioni_nel_turno_del_voto() {
    let mut p = Partita::nuova(dati_base(), 1);
    while p.vista().turno < 19 {
        // Senza il bilancio il governo cadrebbe nell'esercizio provvisorio del 2028.
        if let Some(bilancio) = p.bozza_bilancio() {
            p.comanda(Comando::PresentaBilancio { bilancio }).unwrap();
        }
        p.chiudi_turno();
    }
    p.comanda(Comando::Dimettiti).unwrap();
    p.chiudi_turno(); // si vota
    let l = p.legislatura();
    assert_eq!(l.elezioni.len(), 1);
    assert_eq!(l.fase, Fase::Trattativa);
    assert!(l.dimissioni.is_none());
}

/// Un governo IA (presiede il centrodestra) cade a `scadenza − 1`: il giocatore, primo nella
/// contigua, ha l'incarico col ribaltone. Il turno dopo è quello del voto: vince il voto, la
/// maggioranza del ribaltone si cancella e il salvataggio si ricarica.
#[test]
fn ribaltone_alla_vigilia_del_voto() {
    let mut p = Partita::nuova(dati_base(), 1);
    while p.vista().turno < 18 {
        chiudi_col_bilancio(&mut p);
    }
    let mut p = edita(&p, |j| {
        j["governo"]["presidente"] = partito("centrodestra").into();
        j["legislatura"]["dimissioni"] = "sfiducia".into();
    })
    .unwrap();
    p.chiudi_turno();
    let l = p.legislatura();
    assert_eq!(l.incarico, Some(Incarico::Giocatore));
    assert!(l.maggioranza.is_some());
    assert!(l.turno_di_voto(p.vista().turno));
    p.chiudi_turno(); // si vota
    let l = p.legislatura();
    assert_eq!(l.elezioni.len(), 1);
    assert_eq!(l.fase, Fase::Trattativa);
    assert_eq!(l.maggioranza, None);
    assert!(l.allargati.is_empty());
    assert!(Partita::carica(&p.salva().unwrap()).is_ok());
}

/// La destra del giocatore tiene metà dei seggi in entrambe le Camere: senza di lei nessuna
/// maggioranza, e il Capo dello Stato scioglie. Col governo IA la contigua con tutti c'è sempre
/// (la fila intera ha tutti i seggi): lo scioglimento si vede solo dopo il governo del giocatore.
#[test]
fn senza_maggioranza_contigua_il_capo_dello_stato_scioglie() {
    let c = [20, 60, 40, 20, 40, 200, 20];
    let s = [10, 30, 20, 10, 20, 100, 10];
    let mut p = con_seggi(&Partita::nuova(dati_base(), 1), &c, &s);
    p.comanda(Comando::Dimettiti).unwrap();
    p.chiudi_turno();
    assert_eq!(p.legislatura().fase, Fase::Sciolta);
    assert_eq!(p.legislatura().incarico, None);
    let v = voci(&p, TipoVoce::CamereSciolte);
    assert_eq!(v.len(), 1, "{:?}", p.vista().esito);
    assert_eq!(v[0].fatto, "Camere sciolte, si vota a gennaio 2027");
    assert_eq!(
        v[0].causa,
        "il Capo dello Stato non trova una maggioranza contigua senza il partito del Presidente \
         del Consiglio uscente"
    );
    assert_eq!((v[0].criticita, v[0].precedenza), (Criticita::Critica, 3));
    assert!(voci(&p, TipoVoce::Incarico).is_empty());
}

/// iter-legislativo-italiano:80: la legge del governo caduto decade senza voto, come a fine
/// legislatura, e non costa al successore.
#[test]
fn alla_caduta_il_calendario_decade() {
    let mut p = Partita::nuova(dati_base(), 1);
    presenta(&mut p, &[("tetto_contante", "1000")]).unwrap();
    let id = p.vista().calendario[0].id;
    p.comanda(Comando::Dimettiti).unwrap();
    p.chiudi_turno();
    assert!(!p.governa());
    assert!(p.vista().calendario.is_empty());
    let e = (p.vista().esiti.into_iter())
        .find(|e| e.misura == id)
        .unwrap();
    assert_eq!(
        e.decaduta.as_deref(),
        Some("Decaduta per la caduta del governo")
    );
    assert!(!e.approvata);
    let d = voci(&p, TipoVoce::MisuraDecaduta);
    assert_eq!(d.len(), 1, "{d:?}");
    assert_eq!(
        (d[0].misura, d[0].criticita),
        (Some(id), Criticita::Notevole)
    );
    assert_eq!(d[0].disposizioni, ["tetto_contante"]);
    assert_eq!(d[0].causa, "la caduta del governo");
}

/// La partita col governo di minoranza a gennaio-marzo 2028, in esercizio provvisorio senza
/// bilancio: alla chiusura il governo cade.
fn in_esercizio_provvisorio() -> Partita {
    let mut p = con_seggi(&Partita::nuova(dati_base(), 1), &MINORANZA_C, &MINORANZA_S);
    for _ in 0..5 {
        chiudi_senza_opposizione_ia(&mut p); // fino a gennaio-marzo 2028, senza bilancio
    }
    assert!(p.governa() && p.vista().bilancio.esercizio_provvisorio);
    p
}

/// La voce `governo_caduto` dell'ultima chiusura, che è una sola, del partito di `presidente`.
fn caduta(p: &Partita, presidente: usize) -> VoceEsito {
    let v = voci(p, TipoVoce::GovernoCaduto);
    assert_eq!(v.len(), 1, "{v:?}");
    let x = &p.partiti()[presidente];
    assert_eq!(v[0].fatto, format!("Cade il governo di {}", x.nome));
    assert_eq!(v[0].partito.as_deref(), Some(x.id.as_str()));
    assert_eq!((v[0].criticita, v[0].precedenza), (Criticita::Critica, 3));
    v[0].clone()
}

/// Review Focus 1: a gennaio in esercizio provvisorio, senza bilancio, la fiducia respinta e il
/// bilancio mancato fanno cadere il governo una volta sola. La fiducia, votata in aula, viene
/// prima dell'esercizio provvisorio: la voce dice lei.
#[test]
fn due_origini_vince_la_prima() {
    let mut p = in_esercizio_provvisorio();
    let pc = p.governo().presidente;
    presenta(&mut p, &[("tetto_contante", "1000")]).unwrap();
    let id = p.vista().calendario[0].id;
    p.comanda(Comando::PoniFiducia { misura: id }).unwrap();
    p.chiudi_turno();
    assert!(p.vista().esiti.iter().any(|e| e.fiducia_respinta));
    assert!(!p.governa());
    assert_ne!(p.legislatura().fase, Fase::Ordinaria);
    assert!(p.legislatura().dimissioni.is_none());
    assert_eq!(caduta(&p, pc).causa, "la fiducia respinta in aula");
}

/// Fatto 100: una caduta per ognuna delle sei origini. Crisi senza uscita e voto anticipato
/// partono dal salvataggio come lo lasciano `crisi_ia` in apertura e `voto_anticipato` in
/// chiusura; i loro punti veri li provano `governo_ia.rs` e i test di `partita/ia.rs`.
#[test]
fn governo_caduto_dice_l_origine() {
    let causa = |mut p: Partita| {
        let pc = p.governo().presidente;
        p.chiudi_turno();
        assert_ne!(p.legislatura().fase, Fase::Ordinaria);
        caduta(&p, pc).causa
    };
    let mut p = Partita::nuova(dati_base(), 1);
    p.comanda(Comando::Dimettiti).unwrap();
    assert_eq!(causa(p), "le dimissioni del Presidente del Consiglio");

    let mut p = con_seggi(&Partita::nuova(dati_base(), 1), &MINORANZA_C, &MINORANZA_S);
    presenta(&mut p, &[("tetto_contante", "1000")]).unwrap();
    let id = p.vista().calendario[0].id;
    p.comanda(Comando::PoniFiducia { misura: id }).unwrap();
    assert_eq!(causa(p), "la fiducia respinta in aula");

    assert_eq!(
        causa(in_esercizio_provvisorio()),
        "l'esercizio provvisorio senza legge di bilancio approvata"
    );

    // L'opposizione con 270 seggi su 400 alla Camera (giocatore 240, estrema destra 30).
    let p = all_opposizione();
    let senato = p.parlamento().seggi_senato.clone();
    let mut p = con_seggi(&p, &[0, 50, 10, 10, 10, 50, 30, 240], &senato);
    p.comanda(Comando::Sfiducia { dicastero: None }).unwrap();
    assert_eq!(causa(p), "la mozione di sfiducia approvata");

    let p = edita(&all_opposizione(), |j| {
        j["legislatura"]["crisi"] = j["turno"].clone();
        j["legislatura"]["dimissioni"] = "crisi_senza_uscita".into();
    })
    .unwrap();
    assert_eq!(causa(p), "la crisi di governo senza uscita");

    let p = edita(&all_opposizione(), |j| {
        j["legislatura"]["dimissioni"] = "voto_anticipato".into();
    })
    .unwrap();
    assert_eq!(
        causa(p),
        "le dimissioni per il voto anticipato: nei sondaggi il voto gli vale più che restare al \
         governo"
    );
}
