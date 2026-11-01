#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

mod comune;
use comune::dati_base;
use sim_core::legislatura::{Fondazione, posizioni};
use sim_core::riepilogo::{Avviso, riepilogo};
use sim_core::{Partita, SuMisura};

#[test]
fn il_riepilogo_della_base_e_vincibile_ovunque() {
    let dati = dati_base();
    let r = riepilogo(&dati);
    assert_eq!(r.posizioni.len(), 6);
    assert!(
        r.avvisi.is_empty(),
        "spec riga 285: vincibilità della base {r:#?}"
    );
    assert_eq!(r.spread_pb, SPREAD_DELLA_BASE);
    assert_eq!(r.spread_pb, Partita::nuova(dati, 1).vista().spread_pb);
    assert_eq!(r.saldo_disposizioni_pil, 0.0);
    for p in &r.posizioni {
        assert!(p.coalizione.iter().any(|id| id == "giocatore"));
        assert!((1..=3).contains(&p.coalizione.len()));
        assert!(p.quota_coalizione >= p.quota_attesa);
    }
}

/// Lo spread della partita di prova al turno 0, `Partita::nuova(dati_base(), 1).vista().spread_pb`:
/// 84 pb, misurato a 7bf8c06 col diff del Task 5.
const SPREAD_DELLA_BASE: i32 = 84;

#[test]
fn il_riepilogo_non_ha_rumore() {
    let dati = dati_base();
    let r = riepilogo(&dati);
    assert_eq!(r, riepilogo(&dati));
    for (x, voce) in posizioni(&dati).into_iter().zip(&r.posizioni) {
        let f = Fondazione {
            nome: "Prova".into(),
            posizione: x,
        };
        // Stesse intenzioni vere con qualunque seme: il seme tocca solo informazione e ministri.
        let p = Partita::fonda(dati.clone(), 99, &f).unwrap();
        let n = p.intenzioni_vere().nazionali;
        let votanti = 1.0 - n.last().unwrap();
        let atteso = n[p.giocatore()] / votanti;
        assert_eq!(voce.posizione, x);
        assert!((voce.quota_attesa - atteso).abs() < 1e-12, "{x}");
    }
}

#[test]
fn un_elettorato_spostato_accende_l_avviso() {
    let base = dati_base();
    let mut s = SuMisura::base(&base);
    s.cursori.orientamento = -1.0;
    s.cursori.legge_elettorale = "proporzionale_puro".into();
    let r = riepilogo(&base.con_su_misura(&s).unwrap());
    let [Avviso::SenzaMaggioranza { posizioni }] = r.avvisi.as_slice() else {
        panic!("un avviso solo: {:?}", r.avvisi);
    };
    let senza: Vec<f64> = (r.posizioni.iter())
        .filter(|p| !p.maggioranza)
        .map(|p| p.posizione)
        .collect();
    assert!(!senza.is_empty());
    assert_eq!(posizioni, &senza);
    let json = serde_json::to_value(&r.avvisi[0]).unwrap();
    assert_eq!(json["tipo"], "senza_maggioranza");
}

#[test]
fn le_disposizioni_dell_editor_entrano_nel_saldo() {
    let base = dati_base();
    let mut s = SuMisura::base(&base);
    s.disposizioni.insert("cuneo_fiscale".into(), "0".into());
    let r = riepilogo(&base.con_su_misura(&s).unwrap());
    assert_ne!(r.saldo_disposizioni_pil, 0.0);
}
