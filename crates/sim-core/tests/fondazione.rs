#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

mod comune;
use comune::{dati_base, edita, fondata};
use sim_core::legislatura::{Fase, Fondazione, posti};
use sim_core::{Comando, Errore, Partita};

#[test]
fn sei_posti_fra_i_sette_partiti() {
    assert_eq!(posti(&dati_base()), vec![1.5, 2.5, 3.5, 4.5, 5.5, 6.5]);
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
    for (nome, posto, motivo) in [
        ("  ", 4.5, "nome"),
        ("Partito", 4.0, "4"),
        ("Partito", 0.5, "0.5"),
        ("Partito", f64::NAN, "posto"),
    ] {
        let f = Fondazione {
            nome: nome.into(),
            posto,
        };
        match Partita::fonda(dati_base(), 7, &f) {
            Err(Errore::Fondazione(m)) => assert!(m.contains(motivo), "{m}"),
            altro => panic!("{nome:?} {posto}: {altro:?}"),
        }
    }
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
    assert!(e.to_string().contains("Camere sciolte"), "{e}");
    assert_eq!(p.salva().unwrap(), prima);
}

#[test]
fn l_anteprima_della_fondazione_e_il_sondaggio_della_partita() {
    let a = Partita::anteprima_fondazione(&dati_base(), 7);
    assert_eq!(a.posti.len(), 6);
    let p = fondata(4.5);
    let v = p.vista();
    let q = &a.posti[3];
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
