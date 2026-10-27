mod comune;
use comune::{calo_oltre_il_ritorno, con_seggi, dati_base, edita, in_trattativa, indice, partito};
use serde_json::json;
use sim_core::legislatura::Assegnazione;
use sim_core::parlamento::CausaIntesa;
use sim_core::partiti::Partito;
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

/// La partita di prova con la soddisfazione dei due ministri del centrodestra (Ambiente ed
/// Energia, Esteri e Difesa) a `x`, chiusa di un turno.
fn ministri_del_centrodestra_a(x: f64) -> Partita {
    let p = Partita::nuova(dati_base(), 1);
    let mut p = edita(&p, |j| {
        for k in [6, 9] {
            j["ministri"]["dicasteri"][k]["ministro"]["soddisfazione"] = json!(x);
        }
    })
    .unwrap();
    p.chiudi_turno();
    p
}

#[test]
fn ministri_scontenti_costano_intesa() {
    let p = ministri_del_centrodestra_a(30.0);
    let v = variazioni(&p, partito("centrodestra"), CausaIntesa::MinistriScontenti);
    assert!(v.len() == 1 && v[0] < 0.0, "{v:?}");
    // Alla soglia della minaccia, nessuna variazione.
    let minaccia = dati_base().manopola("soglia_minaccia");
    let p = ministri_del_centrodestra_a(minaccia);
    let v = variazioni(&p, partito("centrodestra"), CausaIntesa::MinistriScontenti);
    assert!(v.is_empty(), "{v:?}");
}

/// Sotto la soglia delle dimissioni i ministri escono nel turno, ma contano ancora nella media.
#[test]
fn il_ministro_che_si_dimette_conta_nella_media() {
    let p = ministri_del_centrodestra_a(10.0);
    assert!(p.ministri().dicasteri[6].ministro.is_none());
    let v = variazioni(&p, partito("centrodestra"), CausaIntesa::MinistriScontenti);
    assert_eq!(v.len(), 1, "{v:?}");
}

#[test]
fn la_quota_sotto_la_richiesta_pesa_ogni_turno() {
    // Centrodestra e destra alla pari: la richiesta del centrodestra supera i suoi 3 punti.
    let p = Partita::nuova(dati_base(), 1);
    let mut p = con_seggi(
        &p,
        &[20, 40, 30, 10, 150, 150, 0],
        &[10, 20, 15, 5, 75, 75, 0],
    );
    let dati = dati_base();
    let richiesta =
        (dati.manopola("gamson_intercetta") + dati.manopola("gamson_pendenza") * 0.5) * 14.0;
    let offerta = 3.0; // Ambiente ed Energia 1, Esteri e Difesa 2
    p.chiudi_turno();
    let v = variazioni(&p, partito("centrodestra"), CausaIntesa::QuotaScoperta);
    let atteso = -dati.manopola("costo_quota_scoperta") * (richiesta - offerta);
    assert!(
        v.len() == 1 && (v[0] - atteso).abs() < 1e-9,
        "{v:?} {atteso}"
    );
}

/// Il giocatore al governo con la quota al voto dell'alleato `j` (il primo) portata a `quota`
/// (`None`: quella vera più cinque punti), chiuso di un turno; rende anche la quota usata.
fn al_governo_con_quota(quota: Option<f64>) -> (Partita, usize, f64) {
    let p = comune::al_governo();
    let j = p.alleati()[0];
    let mut usata = 0.0;
    let mut p = edita(&p, |x| {
        let voti = &mut x["legislatura"]["elezioni"][0]["voti"][j]["quota"];
        usata = quota.unwrap_or_else(|| voti.as_f64().unwrap() + 0.05);
        *voti = json!(usata);
    })
    .unwrap();
    p.chiudi_turno();
    (p, j, usata)
}

#[test]
fn il_calo_nei_sondaggi_al_governo_costa_intesa() {
    let (p, j, quota) = al_governo_con_quota(None);
    // La quota fra i votanti nel sondaggio del turno, media degli istituti.
    let s = &p.informazione().sondaggio;
    let sondaggio = (s.istituti.iter())
        .map(|r| r.quote[j] / (1.0 - r.quote.last().unwrap()))
        .sum::<f64>()
        / s.istituti.len() as f64;
    assert!(quota > sondaggio, "{quota} {sondaggio}");
    let atteso = -dati_base().manopola("costo_calo_sondaggi") * (quota - sondaggio) * 100.0;
    let v = variazioni(&p, j, CausaIntesa::SondaggiInCalo);
    assert!(
        v.len() == 1 && (v[0] - atteso).abs() < 1e-9,
        "{v:?} {atteso}"
    );
    // Sopra la quota del voto, nessun calo.
    let (p, j, _) = al_governo_con_quota(Some(0.0));
    assert!(variazioni(&p, j, CausaIntesa::SondaggiInCalo).is_empty());
}

#[test]
fn dimissioni_a_un_tecnico_alleato_non_costano() {
    let p = Partita::nuova(dati_base(), 1);
    let i = indice("ambiente_energia");
    let mut p = edita(&p, |j| {
        j["ministri"]["dicasteri"][i]["ministro"]["tecnico"] = json!(true)
    })
    .unwrap();
    p.comanda(Comando::ChiediDimissioni {
        dicastero: "ambiente_energia".into(),
    })
    .unwrap();
    let cd = partito("centrodestra");
    assert!(variazioni(&p, cd, CausaIntesa::DimissioniChieste).is_empty());
}

#[test]
fn ultimatum_si_ritira_alla_soglia_e_non_si_riscrive() {
    let p = Partita::nuova(dati_base(), 1);
    let cd = partito("centrodestra");
    let mut p = edita(&p, |j| {
        j["intesa"]["valori"][cd] = json!(35.0);
        j["intesa"]["ultimatum"][cd] = json!("fiducia");
    })
    .unwrap();
    p.chiudi_turno();
    assert_eq!(p.intesa().ultimatum[cd], Some(CausaIntesa::Fiducia));
    let mut p = edita(&p, |j| j["intesa"]["valori"][cd] = json!(41.0)).unwrap();
    p.chiudi_turno();
    assert_eq!(p.intesa().ultimatum[cd], None);
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

/// Allarga al centrosinistra (2, che ha corso col 1) e chiude la trattativa: rende la partita e
/// i partiti prima della chiusura.
fn allarga_al_centrosinistra(
    coalizioni: serde_json::Value,
    camera: &[u32],
    senato: &[u32],
) -> (Partita, Vec<Partito>) {
    let mut p = in_trattativa(coalizioni, camera, senato);
    p.comanda(Comando::Allarga {
        partiti: vec!["centrosinistra".into()],
    })
    .unwrap();
    let prima = p.partiti().to_vec();
    p.chiudi_turno();
    assert!(p.governa());
    (p, prima)
}

#[test]
fn allargare_all_altro_campo_rompe_l_impegno() {
    let (p, prima) = allarga_al_centrosinistra(
        serde_json::json!([[7, 3], [1, 2], [4, 5]]),
        &CAMERA,
        &SENATO,
    );
    let rotture = |j| variazioni(&p, j, CausaIntesa::RotturaImpegno);
    let costo = dati_base().manopola("intesa_rottura_impegno");
    assert_eq!(rotture(2), vec![-costo]);
    assert_eq!(rotture(3), vec![-costo]);
    // Il Presidente del Consiglio non ha intesa: paga solo la valence.
    assert!(rotture(7).is_empty());
    let atteso = dati_base().manopola("valence_rottura_impegno");
    for j in [2, 3, 7] {
        let calo = calo_oltre_il_ritorno(&prima, &p, j);
        assert!((calo - atteso).abs() < 1e-9, "{j}: {calo} {atteso}");
    }
}

/// Il giocatore che ha corso da solo paga la rottura se governa con chi ha corso contro di lui. I
/// seggi del centro passano al centrosinistra: 7 + 2 hanno la maggioranza (210 su 400, 105 su 200).
#[test]
fn il_giocatore_da_solo_che_allarga_all_altro_campo_rompe() {
    let (p, prima) = allarga_al_centrosinistra(
        serde_json::json!([[7], [1, 2], [4, 5]]),
        &[10, 60, 110, 0, 50, 50, 20, 100],
        &[5, 30, 55, 0, 25, 25, 10, 50],
    );
    let calo = calo_oltre_il_ritorno(&prima, &p, 7);
    let atteso = dati_base().manopola("valence_rottura_impegno");
    assert!((calo - atteso).abs() < 1e-9, "{calo} {atteso}");
}

/// Il giocatore (7) ha corso col centrodestra (4), alleato del governo uscente, e insieme hanno la
/// maggioranza (Camera 201 su 400, Senato 101 su 200).
const CAMERA_CD: [u32; 8] = [10, 50, 40, 29, 110, 50, 20, 91];
const SENATO_CD: [u32; 8] = [5, 25, 20, 14, 55, 25, 10, 46];

/// In trattativa col centrodestra a 30 e in ultimatum per la fiducia.
fn col_centrodestra_in_ultimatum() -> Partita {
    let p = in_trattativa(json!([[7, 4]]), &CAMERA_CD, &SENATO_CD);
    edita(&p, |j| {
        j["intesa"]["valori"][4] = json!(30.0);
        j["intesa"]["ultimatum"][4] = json!("fiducia");
    })
    .unwrap()
}

#[test]
fn la_formazione_azzera_l_ultimatum() {
    let mut p = col_centrodestra_in_ultimatum();
    p.chiudi_turno();
    assert!(p.governa() && p.alleati() == vec![4]);
    assert_eq!(p.intesa().ultimatum[4], None);
}

/// Ogni dicastero al giocatore: il centrodestra entra sotto la soglia e pone l'ultimatum subito,
/// con la causa della spartizione al posto di quella di prima.
#[test]
fn la_spartizione_avara_pone_l_ultimatum_alla_formazione() {
    let mut p = col_centrodestra_in_ultimatum();
    let dicasteri = (p.vista().dicasteri.into_iter())
        .map(|d| Assegnazione {
            dicastero: d.id,
            partito: Some("giocatore".into()),
        })
        .collect();
    p.comanda(Comando::ProponiSpartizione { dicasteri })
        .unwrap();
    p.chiudi_turno();
    assert!(p.governa() && p.alleati() == vec![4]);
    assert!(p.intesa().valori[4] < dati_base().manopola("soglia_ultimatum"));
    assert_eq!(p.intesa().ultimatum[4], Some(CausaIntesa::Spartizione));
}

#[test]
fn in_trattativa_nessuna_pressione_sull_intesa() {
    let p = in_trattativa(json!([[7, 4]]), &CAMERA_CD, &SENATO_CD);
    assert!(!p.alleati().is_empty());
    // Ministri tutti scontenti: in fase ordinaria gli alleati del governo uscente pagherebbero.
    let mut p = edita(&p, |j| {
        for st in j["ministri"]["dicasteri"].as_array_mut().unwrap() {
            if !st["ministro"].is_null() {
                st["ministro"]["soddisfazione"] = json!(30.0);
            }
        }
    })
    .unwrap();
    p.chiudi_turno();
    let pressioni = [
        CausaIntesa::MinistriScontenti,
        CausaIntesa::QuotaScoperta,
        CausaIntesa::SondaggiInCalo,
    ];
    let v: Vec<_> = (p.intesa().variazioni.iter())
        .filter(|v| pressioni.contains(&v.causa))
        .collect();
    assert!(v.is_empty(), "{v:?}");
}

/// Una variazione oltre il turno della partita, o una quota del voto fuori da 0-1 (darebbe un
/// `SondaggiInCalo` sproporzionato): il salvataggio è illeggibile.
#[test]
fn carica_rifiuta_variazioni_future_e_quote_fuori_da_0_1() {
    let p = comune::al_governo();
    let dopo = p.vista().turno + 1;
    let e = edita(
        &p,
        |j| {
            j["intesa"]["variazioni"] =
                json!([{ "partito": 0, "causa": "fiducia", "delta": -1.0, "turno": dopo }])
        },
    )
    .unwrap_err();
    assert!(e.to_string().contains("oltre il turno"), "{e}");
    for q in [1.5, -0.1] {
        let e = edita(&p, |j| {
            j["legislatura"]["elezioni"][0]["voti"][0]["quota"] = json!(q)
        })
        .unwrap_err();
        assert!(e.to_string().contains("quota"), "{q}: {e}");
    }
}
