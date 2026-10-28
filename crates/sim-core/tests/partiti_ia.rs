mod comune;
use comune::*;
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
