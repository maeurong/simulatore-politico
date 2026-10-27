mod comune;
use comune::{dati_base, edita, in_trattativa, partito};
use sim_core::parlamento::CausaIntesa;
use sim_core::{Comando, Partita};

fn variazioni(p: &Partita, j: usize, c: CausaIntesa) -> Vec<f64> {
    (p.intesa().variazioni.iter())
        .filter(|v| v.partito == j && v.causa == c)
        .map(|v| v.delta)
        .collect()
}

#[test]
fn dimissioni_chieste_a_un_alleato_costano_intesa() {
    let mut p = Partita::nuova(dati_base(), 1);
    let cd = partito("centrodestra");
    p.comanda(Comando::ChiediDimissioni {
        dicastero: "ambiente_energia".into(),
    })
    .unwrap();
    let costo = dati_base().manopola("costo_dimissioni_chieste");
    assert_eq!(
        variazioni(&p, cd, CausaIntesa::DimissioniChieste),
        vec![-costo]
    );
    // Un ministro del partito del Presidente del Consiglio: nessun alleato paga.
    p.comanda(Comando::ChiediDimissioni {
        dicastero: "sanita".into(),
    })
    .unwrap();
    assert_eq!(variazioni(&p, cd, CausaIntesa::DimissioniChieste).len(), 1);
}

#[test]
fn ministri_scontenti_costano_intesa() {
    let p = Partita::nuova(dati_base(), 1);
    let mut p = edita(&p, |j| {
        for k in [6, 9] {
            j["ministri"]["dicasteri"][k]["ministro"]["soddisfazione"] = serde_json::json!(30.0);
        }
    })
    .unwrap();
    p.chiudi_turno();
    let v = variazioni(&p, partito("centrodestra"), CausaIntesa::MinistriScontenti);
    assert!(v.len() == 1 && v[0] < 0.0, "{v:?}");
}

#[test]
fn la_quota_sotto_la_richiesta_pesa_ogni_turno() {
    let mut p = Partita::nuova(dati_base(), 1);
    let cd = partito("centrodestra");
    let pl = p.parlamento();
    let s = |j: usize| f64::from(pl.seggi_camera[j] + pl.seggi_senato[j]);
    let dati = dati_base();
    let richiesta = (dati.manopola("gamson_intercetta")
        + dati.manopola("gamson_pendenza") * s(cd) / (s(cd) + s(partito("destra"))))
        * 14.0;
    let offerta = 3.0; // Ambiente ed Energia 1, Esteri e Difesa 2
    p.chiudi_turno();
    let v = variazioni(&p, cd, CausaIntesa::QuotaScoperta);
    if richiesta > offerta {
        let atteso = -dati.manopola("costo_quota_scoperta") * (richiesta - offerta);
        assert!(
            v.len() == 1 && (v[0] - atteso).abs() < 1e-9,
            "{v:?} {atteso}"
        );
    } else {
        assert!(v.is_empty());
    }
}

#[test]
fn il_calo_nei_sondaggi_al_governo_costa_intesa() {
    let p = comune::al_governo();
    let j = p.alleati()[0];
    // Una quota al voto più alta di cinque punti: l'alleato risulta in calo.
    let mut p = edita(&p, |x| {
        let q = x["legislatura"]["elezioni"][0]["voti"][j]["quota"]
            .as_f64()
            .unwrap();
        x["legislatura"]["elezioni"][0]["voti"][j]["quota"] = serde_json::json!(q + 0.05);
    })
    .unwrap();
    p.chiudi_turno();
    let v = variazioni(&p, j, CausaIntesa::SondaggiInCalo);
    assert!(v.len() == 1 && v[0] < 0.0, "{v:?}");
}

#[test]
fn l_ultimatum_resta_con_la_sua_causa() {
    let p = Partita::nuova(dati_base(), 1);
    let cd = partito("centrodestra");
    let mut p = edita(&p, |j| j["intesa"]["valori"][cd] = serde_json::json!(45.0)).unwrap();
    p.comanda(Comando::ChiediDimissioni {
        dicastero: "ambiente_energia".into(),
    })
    .unwrap();
    p.chiudi_turno();
    let v = p.vista();
    let a = v
        .alleati
        .iter()
        .find(|a| a.partito == "centrodestra")
        .unwrap();
    assert!(a.ultimatum);
    assert_eq!(a.causa_ultimatum, Some(CausaIntesa::DimissioniChieste));
    assert!(p.governo().al_governo(cd)); // sopra 25: non esce
}

/// Il giocatore (7) dichiarato col centro (3); 7 + 3 + centrosinistra (2) hanno la maggioranza
/// (Camera 210 su 400, Senato 105 su 200), 7 + 3 da soli no.
const CAMERA: [u32; 8] = [10, 60, 80, 30, 50, 50, 20, 100];
const SENATO: [u32; 8] = [5, 30, 40, 15, 25, 25, 10, 50];

#[test]
fn allargare_all_altro_campo_rompe_l_impegno() {
    let coalizioni = serde_json::json!([[7, 3], [1, 2], [4, 5]]);
    let mut gemella = in_trattativa(coalizioni.clone(), &CAMERA, &SENATO);
    let mut p = in_trattativa(coalizioni, &CAMERA, &SENATO);
    p.comanda(Comando::Allarga {
        partiti: vec!["centrosinistra".into()],
    })
    .unwrap();
    p.chiudi_turno();
    gemella.chiudi_turno();
    assert!(p.governa());
    let rotture = |j| variazioni(&p, j, CausaIntesa::RotturaImpegno);
    let costo = dati_base().manopola("intesa_rottura_impegno");
    assert_eq!(rotture(2), vec![-costo]);
    assert_eq!(rotture(3), vec![-costo]);
    let calo = gemella.partiti()[7].valence - p.partiti()[7].valence;
    let atteso = dati_base().manopola("valence_rottura_impegno");
    assert!((calo - atteso).abs() < 1e-9, "{calo} {atteso}");
}
