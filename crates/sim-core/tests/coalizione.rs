#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

mod comune;
use comune::fondata;
use sim_core::legislatura::Cessione;
use sim_core::{Comando, Partita};

fn dichiara(p: &mut Partita, partiti: &[&str]) -> Result<(), sim_core::Errore> {
    p.comanda(Comando::DichiaraCoalizione {
        partiti: partiti.iter().map(|s| (*s).into()).collect(),
        cessioni: Vec::new(),
    })
}

// Variante del brief: al posto 6,5 l'estrema destra non è mai disponibile (corre da sola con i
// suoi seggi, entrando paga la distanza e non guadagna né seggi né maggioranza); al posto 5,5 il
// centrodestra lascia la coalizione con la destra, più lontana, ed entra.
#[test]
fn un_vicino_disponibile_entra_un_lontano_no() {
    let mut p = fondata(5.5);
    let v = p.disponibilita();
    let vicino = v.iter().find(|x| x.partito == "centrodestra").unwrap();
    assert!(vicino.disponibile, "{vicino:?}");
    dichiara(&mut p, &["centrodestra"]).unwrap();
    let prima = p.salva().unwrap();
    let e = dichiara(&mut p, &["estrema_sinistra"]).unwrap_err();
    assert!(e.to_string().contains("Estrema sinistra"), "{e}");
    assert_eq!(p.salva().unwrap(), prima);
}

/// La partita vera al posto 5,5 col `valore_guida` dato.
fn fondata_con_guida(valore: &str) -> Partita {
    let f = comune::imposta(
        comune::file_base(),
        "manopole",
        "[valore_guida]\nvalore = ",
        valore,
    );
    Partita::fonda(
        sim_core::dati::DatiGioco::da_testi(&f).unwrap(),
        7,
        &sim_core::legislatura::Fondazione {
            nome: "Partito nuovo".into(),
            posto: 5.5,
        },
    )
    .unwrap()
}

#[test]
fn la_guida_ceduta_trattiene_il_grande() {
    // Destra guida centrodestra e destra. Col valore della guida al massimo non entra, e la causa
    // è la guida; centrodestra, che non guida, non cambia. Senza valore della guida la causa
    // della destra non è più la guida.
    let v = fondata_con_guida("10.0").disponibilita();
    let voce = |id: &str| v.iter().find(|x| x.partito == id).unwrap();
    assert!(!voce("destra").disponibile);
    assert!(
        voce("destra").causa.contains("guiderebbe"),
        "{:?}",
        voce("destra")
    );
    let base = fondata(5.5).disponibilita();
    let prima = base.iter().find(|x| x.partito == "centrodestra").unwrap();
    assert_eq!(voce("centrodestra").disponibile, prima.disponibile);
    let senza = fondata_con_guida("0.0").disponibilita();
    let destra = senza.iter().find(|x| x.partito == "destra").unwrap();
    assert!(
        destra.disponibile || !destra.causa.contains("guiderebbe"),
        "{destra:?}"
    );
}

#[test]
fn cedere_posti_attira_un_partito() {
    let mut p = fondata(4.5);
    let cessioni = |n| {
        vec![Cessione {
            partito: "centro".into(),
            seggi: n,
        }]
    };
    let rifiuto = p.comanda(Comando::DichiaraCoalizione {
        partiti: vec!["centro".into()],
        cessioni: cessioni(0),
    });
    assert!(rifiuto.is_err());
    p.comanda(Comando::DichiaraCoalizione {
        partiti: vec!["centro".into()],
        cessioni: cessioni(60), // ne bastano 49 col seme 7
    })
    .unwrap();
    assert_eq!(p.vista().coalizione, vec!["centro".to_string()]);
}

#[test]
fn dichiarazione_rifiutata_col_motivo() {
    let cessione = |partito: &str, seggi| Cessione {
        partito: partito.into(),
        seggi,
    };
    let casi = [
        (vec!["giocatore"], vec![], "la coalizione la guidi tu"),
        (vec!["centrodestra", "centrodestra"], vec![], "ripetuto"),
        (
            vec!["centrodestra"],
            vec![cessione("destra", 1)],
            "non è nella coalizione",
        ),
        (
            vec!["centrodestra"],
            vec![cessione("centrodestra", u32::MAX)],
            "Cessioni",
        ),
        (vec!["inesistente"], vec![], "Partito sconosciuto"),
    ];
    let mut p = fondata(5.5);
    let prima = p.salva().unwrap();
    for (partiti, cessioni, motivo) in casi {
        let e = p
            .comanda(Comando::DichiaraCoalizione {
                partiti: partiti.iter().map(|s| (*s).into()).collect(),
                cessioni,
            })
            .unwrap_err();
        assert!(e.to_string().contains(motivo), "{partiti:?}: {e}");
        assert_eq!(p.salva().unwrap(), prima, "{partiti:?}");
    }
}

#[test]
fn l_intesa_bassa_pesa_sull_alleato_in_carica() {
    // Partita di prova: il giocatore (destra) governa con centrodestra (4); la scadenza portata al
    // turno zero lo mette nel turno del voto.
    let p = Partita::nuova(comune::dati_base(), 1);
    assert!(p.governa());
    assert_eq!(p.alleati(), vec![4]);
    let causa = |intesa: f64| {
        let q = comune::edita(&p, |j| {
            j["legislatura"]["scadenza"] = 0.into();
            j["intesa"]["valori"][4] = intesa.into();
        })
        .unwrap();
        let v = q.disponibilita();
        v.iter()
            .find(|x| x.partito == "centrodestra")
            .unwrap()
            .causa
            .clone()
    };
    assert_ne!(causa(100.0), "intesa bassa con te");
    assert_eq!(causa(0.0), "intesa bassa con te");
}

#[test]
fn fuori_dal_turno_di_voto_non_si_dichiara() {
    let mut p = Partita::nuova(comune::dati_base(), 1); // partita di prova, scadenza 19
    let e = dichiara(&mut p, &[]).unwrap_err();
    assert!(e.to_string().contains("turno del voto"), "{e}");
}

#[test]
fn salvare_con_la_coalizione_dichiarata() {
    let mut a = fondata(5.5);
    dichiara(&mut a, &["centrodestra"]).unwrap();
    let mut b = Partita::carica(&a.salva().unwrap()).unwrap();
    a.chiudi_turno();
    b.chiudi_turno();
    assert_eq!(a.salva().unwrap(), b.salva().unwrap());
}

fn cedi_a_centrodestra(p: &mut Partita, seggi: u32) -> Result<(), sim_core::Errore> {
    p.comanda(Comando::DichiaraCoalizione {
        partiti: vec!["centrodestra".into()],
        cessioni: vec![Cessione {
            partito: "centrodestra".into(),
            seggi,
        }],
    })
}

/// I seggi attesi del giocatore alla Camera, sul sondaggio, con la coalizione dichiarata.
fn attesi_camera(p: &Partita) -> u32 {
    p.vista().proiezione.unwrap()[7].camera
}

#[test]
fn cedere_tutti_i_seggi_attesi_rifiutato() {
    let mut p = fondata(5.5);
    dichiara(&mut p, &["centrodestra"]).unwrap();
    let attesi = attesi_camera(&p);
    let prima = p.salva().unwrap();
    let e = cedi_a_centrodestra(&mut p, attesi).unwrap_err();
    assert!(e.to_string().contains("Cessioni"), "{e}");
    assert_eq!(p.salva().unwrap(), prima);
}

#[test]
fn cedere_i_seggi_attesi_meno_uno_lascia_un_seggio() {
    let mut p = fondata(5.5);
    dichiara(&mut p, &["centrodestra"]).unwrap();
    let attesi = attesi_camera(&p);
    let mut senza = Partita::carica(&p.salva().unwrap()).unwrap();
    senza.chiudi_turno();
    cedi_a_centrodestra(&mut p, attesi - 1).unwrap();
    p.chiudi_turno();
    assert!(!p.finita());
    let (a, b) = (senza.parlamento(), p.parlamento());
    for (prima, dopo) in [
        (a.seggi_camera[7], b.seggi_camera[7]),
        (a.seggi_senato[7], b.seggi_senato[7]),
    ] {
        assert!(prima == 0 || dopo >= 1, "{prima} -> {dopo}");
    }
}
