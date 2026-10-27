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

#[test]
fn la_guida_ceduta_trattiene_il_grande() {
    // Destra guida centrodestra e destra. Col valore della guida al massimo non entra, e la causa
    // è la guida; centrodestra, che non guida, non cambia.
    let f = comune::imposta(
        comune::file_base(),
        "manopole",
        "[valore_guida]\nvalore = ",
        "10.0",
    );
    let p = Partita::fonda(
        sim_core::dati::DatiGioco::da_testi(&f).unwrap(),
        7,
        &sim_core::legislatura::Fondazione {
            nome: "Partito nuovo".into(),
            posto: 5.5,
        },
    )
    .unwrap();
    let v = p.disponibilita();
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
    if rifiuto.is_err() {
        p.comanda(Comando::DichiaraCoalizione {
            partiti: vec!["centro".into()],
            cessioni: cessioni(60), // ne bastano 49 col seme 7
        })
        .unwrap();
    }
    assert_eq!(p.vista().coalizione, vec!["centro".to_string()]);
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
