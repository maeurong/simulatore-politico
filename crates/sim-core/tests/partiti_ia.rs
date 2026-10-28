mod comune;
use comune::*;
use sim_core::Partita;
use sim_core::partiti::Vocazione;

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
    assert!(
        p.vista()
            .mosse
            .iter()
            .any(|m| m.partito == "destra" && m.testo.contains("la perdita del governo"))
    );
}

#[test]
fn il_giocatore_non_cambia_mai_leader() {
    let mut p = all_opposizione();
    let g = p.giocatore();
    for _ in 0..24 {
        p.chiudi_turno();
        assert_eq!(p.partiti()[g].leader, "");
    }
}

#[test]
fn le_mosse_si_azzerano_a_ogni_chiusura() {
    let mut p = sinistra_al_governo();
    p.chiudi_turno();
    assert!(!p.vista().mosse.is_empty());
    p.chiudi_turno();
    assert!(
        p.vista()
            .mosse
            .iter()
            .all(|m| !m.testo.contains("la perdita del governo"))
    );
}

#[test]
fn chi_resta_alleato_non_cambia_leader() {
    let mut p = vinta(); // la destra, se disponibile, entra nella coalizione del giocatore
    let destra = partito("destra");
    let prima = p.partiti()[destra].leader.clone();
    p.chiudi_turno();
    if p.governo().al_governo(destra) {
        assert_eq!(p.partiti()[destra].leader, prima);
    }
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
    assert_ne!(p.partiti()[destra].leader, prima);
    assert!(p.vista().mosse.iter().any(|m| m.partito == "destra"
        && m.testo.contains("il calo di ")
        && m.testo.contains(" punti al voto")));
}
