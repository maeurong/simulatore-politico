#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

mod comune;
use comune::{dati_base, edita, fondata};
use sim_core::legislatura::{Fase, Fondazione, posizioni};
use sim_core::{Comando, Errore, Partita};

#[test]
fn sei_posizioni_fra_i_sette_partiti() {
    assert_eq!(posizioni(&dati_base()), vec![1.5, 2.5, 3.5, 4.5, 5.5, 6.5]);
}

#[test]
fn la_partita_vera_comincia_a_camere_sciolte() {
    let p = fondata(4.5);
    assert_eq!(p.partiti().len(), 8);
    assert_eq!(p.giocatore(), 7);
    assert_eq!(p.partiti()[7].id, "giocatore");
    assert_eq!(p.legislatura().fase, Fase::Sciolta);
    assert!(!p.governa());
    assert_eq!(p.parlamento().seggi_camera, vec![0; 8]);
    assert_eq!(p.governo().presidente, 5); // destra, il governo uscente
    let dati = dati_base();
    let atteso = dati.manopola("valence_partito_nuovo") + dati.manopola("luna_di_miele");
    assert!((p.partiti()[7].valence - atteso).abs() < 1e-12);
    assert!(p.vista().turni_al_voto == 0);
}

#[test]
fn fondazione_non_valida_rifiutata_col_motivo() {
    let lungo = "x".repeat(41);
    for (nome, posizione, motivo) in [
        ("  ", 4.5, "nome"),
        (lungo.as_str(), 4.5, "più lungo di 40 caratteri"),
        ("Partito", 4.0, "posizione 4 non ammessa"),
        (
            "Partito",
            0.5,
            "posizione 0,5 non ammessa: scegli fra 1,5; 2,5;",
        ),
        ("Partito", f64::NAN, "posizione non valida"),
    ] {
        let f = Fondazione {
            nome: nome.into(),
            posizione,
        };
        match Partita::fonda(dati_base(), 7, &f) {
            Err(Errore::Fondazione(m)) => assert!(m.contains(motivo), "{m}"),
            altro => panic!("{nome:?} {posizione}: {altro:?}"),
        }
    }
    let f = Fondazione {
        nome: "x".repeat(40),
        posizione: 4.5,
    };
    assert!(Partita::fonda(dati_base(), 7, &f).is_ok());
}

#[test]
fn comandi_di_governo_rifiutati_a_chi_non_governa() {
    let mut p = fondata(4.5);
    let prima = p.salva().unwrap();
    let e = p
        .comanda(Comando::Ferma {
            dicastero: "sanita".into(),
        })
        .unwrap_err();
    assert!(e.to_string().contains("non governi: Camere sciolte"), "{e}");
    assert_eq!(p.salva().unwrap(), prima);
}

#[test]
fn l_anteprima_della_fondazione_e_il_sondaggio_della_partita() {
    let a = Partita::anteprima_fondazione(&dati_base(), 7);
    assert_eq!(a.posizioni.len(), 6);
    let p = fondata(4.5);
    let v = p.vista();
    let q = &a.posizioni[3];
    assert_eq!(q.posizione, 4.5);
    let atteso = v
        .sondaggio
        .istituti
        .iter()
        .map(|r| r.quote[7] / (1.0 - r.quote[8]))
        .sum::<f64>()
        / v.sondaggio.istituti.len() as f64;
    assert!((q.quota_attesa - atteso).abs() < 1e-12);
    assert!((a.istogramma.iter().sum::<f64>() - 1.0).abs() < 1e-9);
}

#[test]
fn salvataggio_della_partita_vera_si_ricarica() {
    let p = fondata(2.5);
    let q = Partita::carica(&p.salva().unwrap()).unwrap();
    assert_eq!(p, q);
    // Un giocatore fuori dai partiti è rifiutato.
    assert!(edita(&p, |j| j["giocatore"] = serde_json::json!(99)).is_err());
}

/// Quote vere fra i votanti, una per partito, alla partita appena fondata.
fn quote_vere(p: &Partita) -> Vec<f64> {
    let v = p.intenzioni_vere().nazionali;
    let votanti = 1.0 - v[v.len() - 1];
    v[..v.len() - 1].iter().map(|q| q / votanti).collect()
}

#[test]
fn luna_di_miele_al_quindici_per_cento_della_posizione_media() {
    let media = posizioni(&dati_base())
        .iter()
        .map(|&x| quote_vere(&fondata(x))[7])
        .sum::<f64>()
        / 6.0;
    assert!((media - 0.15).abs() < 0.005, "{media}");
}

#[test]
fn vincibile_da_ogni_posizione() {
    // Esiste un blocco di al più tre partiti contigui sull'asse, col giocatore, sopra il 42% dei
    // votanti. Il tetto serve: senza, il blocco di tutti gli otto partiti fa 1 e il test non dice nulla.
    for x in posizioni(&dati_base()) {
        let p = fondata(x);
        let q = quote_vere(&p);
        let mut ordine: Vec<usize> = (0..q.len()).collect();
        ordine.sort_by(|&a, &b| {
            p.partiti()[a]
                .posizione_dichiarata
                .total_cmp(&p.partiti()[b].posizione_dichiarata)
        });
        let g = ordine.iter().position(|&j| j == 7).unwrap();
        let migliore = (0..=g)
            .flat_map(|i| (g..ordine.len()).map(move |k| (i, k)))
            .filter(|(i, k)| k - i < 3)
            .map(|(i, k)| ordine[i..=k].iter().map(|&j| q[j]).sum::<f64>())
            .fold(0.0, f64::max);
        assert!(migliore > 0.42, "posizione {x}: {migliore}");
    }
}
