#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

mod comune;
use comune::{chiudi_col_bilancio, dati_base, edita, fondata};
use sim_core::Partita;

fn casse(p: &Partita) -> Vec<i64> {
    p.vista().partiti.iter().map(|x| x.cassa).collect()
}

#[test]
fn il_partito_fondato_parte_con_la_dote() {
    let p = fondata(4.5).vista();
    let g = p.partiti.iter().find(|p| p.id == "giocatore").unwrap();
    assert_eq!(g.cassa, 1_000_000);
}

/// La partita di prova vota ai turni 19 e 39, non al turno zero: l'ultimo voto c'è dopo il 19.
#[test]
fn le_entrate_seguono_voti_e_intenzioni() {
    let mut p = Partita::nuova(dati_base(), 1);
    while p.legislatura().elezioni.is_empty() {
        chiudi_col_bilancio(&mut p);
    }
    let prima = casse(&p);
    chiudi_col_bilancio(&mut p);
    let delta: Vec<i64> = casse(&p).iter().zip(&prima).map(|(x, c)| x - c).collect();
    assert!(delta.iter().all(|&d| d > 0));
    // Chi ha preso più voti incassa più di chi ne ha presi meno.
    let voti = &p.legislatura().elezioni[0].voti;
    let ids: Vec<String> = p.vista().partiti.into_iter().map(|x| x.id).collect();
    let quota = |j: usize| voti.iter().find(|v| v.partito == ids[j]).unwrap().quota;
    let primo = (0..ids.len())
        .max_by(|&a, &b| quota(a).total_cmp(&quota(b)))
        .unwrap();
    let ultimo = (0..ids.len())
        .min_by(|&a, &b| quota(a).total_cmp(&quota(b)))
        .unwrap();
    assert!(delta[primo] > delta[ultimo]);
}

/// Le quote fra i votanti sommano a 1: le entrate del turno zero valgono
/// `(due_per_mille_turno + donazioni_turno) · quota_libera_entrate`, la cassa iniziale venti volte.
#[test]
fn le_casse_iniziali_dello_scenario_sono_venti_turni() {
    let c = casse(&Partita::nuova(dati_base(), 1));
    assert!(c.iter().all(|&x| x > 0 && x % 20 == 0));
    let attese = (8_146_050.0 + 5_500_000.0) * 0.1 * 20.0;
    // Ogni partito arrotonda al singolo euro prima dei venti turni.
    let scarto = (c.iter().sum::<i64>() as f64 - attese).abs();
    assert!(scarto <= 7.0 * 20.0, "scarto {scarto}");
}

#[test]
fn nella_partita_vera_i_partiti_dello_scenario_hanno_cassa_e_il_fondato_la_dote() {
    let c = casse(&fondata(4.5));
    assert_eq!(c.len(), 8);
    assert!(c[..7].iter().all(|&x| x > 0));
    assert_eq!(c[7], 1_000_000);
}

/// Dopo il primo voto l'estrema sinistra, che ha preso voti, si scioglie (seggi spostati alla
/// sinistra): il 2 per mille sulla sua quota non entra, la cassa resta quella.
#[test]
fn lo_sciolto_non_incassa() {
    let mut p = Partita::nuova(dati_base(), 1);
    while p.legislatura().elezioni.is_empty() {
        chiudi_col_bilancio(&mut p);
    }
    assert!(p.legislatura().elezioni[0].voti[0].quota > 0.0);
    let mut p = edita(&p, |j| {
        j["partiti"][0]["sciolto"] = true.into();
        for ramo in ["seggi_camera", "seggi_senato"] {
            let v = &mut j["parlamento"][ramo];
            let x = v[0].as_u64().unwrap();
            v[0] = 0.into();
            v[1] = (v[1].as_u64().unwrap() + x).into();
        }
    })
    .unwrap();
    let prima = casse(&p);
    chiudi_col_bilancio(&mut p);
    let dopo = casse(&p);
    assert_eq!(dopo[0], prima[0]);
    assert!(dopo[1] > prima[1]);
}

/// Prima del primo voto il 2 per mille va sull'intenzione vera, come la cassa iniziale: le quote
/// sommano a 1, quindi la prima chiusura porta in tutto le entrate piene di un turno.
#[test]
fn prima_del_primo_voto_il_due_per_mille_va_sull_intenzione() {
    let mut p = Partita::nuova(dati_base(), 1);
    assert!(p.legislatura().elezioni.is_empty());
    let prima: i64 = casse(&p).iter().sum();
    chiudi_col_bilancio(&mut p);
    assert!(p.legislatura().elezioni.is_empty());
    let entrate = casse(&p).iter().sum::<i64>() - prima;
    let attese = (8_146_050.0 + 5_500_000.0) * 0.1;
    assert!((entrate as f64 - attese).abs() <= 7.0, "{entrate}");
}
