#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

mod comune;
use comune::{dati_base, edita, fondata, in_trattativa};
use sim_core::{Comando, Partita};

#[test]
fn il_successore_di_un_tecnico_e_un_tecnico() {
    let p = Partita::nuova(dati_base(), 1);
    let mut p = edita(&p, |j| {
        j["ministri"]["dicasteri"][3]["ministro"]["tecnico"] = serde_json::json!(true)
    })
    .unwrap();
    p.comanda(Comando::ChiediDimissioni {
        dicastero: "sanita".into(),
    })
    .unwrap();
    p.chiudi_turno();
    let st = &p.ministri().dicasteri[3];
    assert!(st.ministro.is_none());
    assert_eq!(st.rosa.len(), 3);
    assert!(st.rosa.iter().all(|x| x.tecnico));
}

#[test]
fn nessun_referendum_nei_sei_mesi_dopo_i_comizi() {
    let mut p = fondata(1.5);
    p.chiudi_turno(); // voto: turno 1 = trattativa, inizio = 1
    let e = p
        .promuovi_referendum("sinistra", "licenziamento")
        .unwrap_err();
    assert!(e.to_string().contains("sei mesi"), "{e}");
    // Al turno dopo l'inizio i sei mesi sono passati (iter-legislativo-italiano:135).
    p.chiudi_turno();
    assert_eq!(p.vista().turno, p.legislatura().inizio + 1);
    let e = p.promuovi_referendum("sinistra", "licenziamento").err();
    assert!(
        e.as_ref()
            .is_none_or(|e| !e.to_string().contains("sei mesi")),
        "{e:?}"
    );
}

/// Partiti 3 (centro, 4,0) e 4 (centrodestra, 5,0) accanto al giocatore (4,5): da soli non
/// bastano, insieme sì (Camera 210 su 400, Senato 105 su 200).
const CAMERA: [u32; 8] = [10, 80, 50, 30, 30, 40, 10, 150];
const SENATO: [u32; 8] = [5, 40, 25, 15, 15, 20, 5, 75];

#[test]
fn due_partner_che_bastano_insieme_entrano_insieme() {
    let mut p = in_trattativa(serde_json::json!([[7]]), &CAMERA, &SENATO);
    let prima = p.salva().unwrap();
    let e = p
        .comanda(Comando::Allarga {
            partiti: vec!["centro".into()],
        })
        .unwrap_err();
    assert!(e.to_string().contains("Centro"), "{e}");
    assert_eq!(p.salva().unwrap(), prima);
    p.comanda(Comando::Allarga {
        partiti: vec!["centro".into(), "centrodestra".into()],
    })
    .unwrap();
    p.chiudi_turno();
    assert!(p.governa());
    assert!(p.governo().al_governo(3) && p.governo().al_governo(4));
}

#[test]
fn allargamenti_rifiutati_lasciano_la_partita_identica() {
    let mut p = in_trattativa(serde_json::json!([[7]]), &CAMERA, &SENATO);
    let prima = p.salva().unwrap();
    for (ids, testo) in [
        (vec![], "allargamento senza partiti"),
        (
            vec!["centro", "centro"],
            "Centro ripetuto nell'allargamento",
        ),
        (
            vec!["centro", "inesistente"],
            "Partito sconosciuto: inesistente",
        ),
        (vec!["giocatore"], "Partito nuovo è già nella coalizione"),
        // Il centro è disponibile e non compare; gli altri tre sì, separati da «; ».
        (
            vec!["centro", "centrodestra", "estrema_sinistra", "sinistra"],
            "comando non ammesso: Centrodestra non entra: posizione troppo lontana dalla tua; \
             Estrema sinistra non entra: posizione troppo lontana dalla tua; \
             Sinistra non entra: posizione troppo lontana dalla tua",
        ),
    ] {
        let partiti = ids.into_iter().map(String::from).collect();
        let e = p.comanda(Comando::Allarga { partiti }).unwrap_err();
        assert!(e.to_string().contains(testo), "{e}");
        assert_eq!(p.salva().unwrap(), prima);
    }
}
