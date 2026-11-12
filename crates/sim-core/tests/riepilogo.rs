#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

mod comune;
use comune::{chiudi_col_bilancio, con_ingresso, dati_base, file_base, file_vero, sostituisci};
use sim_core::dati::{DatiGioco, FileDati};
use sim_core::disposizioni::Norma;
use sim_core::legislatura::{Fondazione, posizioni};
use sim_core::riepilogo::{Avviso, RiepilogoScenario, catalogo, riepilogo};
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
        assert!(
            p.quota_coalizione > 0.42,
            "{}: {}",
            p.posizione,
            p.quota_coalizione
        );
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

fn riepilogo_di(f: FileDati) -> RiepilogoScenario {
    riepilogo(&DatiGioco::da_testi(&f).unwrap())
}

#[test]
fn caro_energia_acceso_al_turno_zero_avvisa() {
    let r = riepilogo_di(file_vero());
    assert_eq!(
        r.avvisi,
        [Avviso::SituazioniAccese {
            nomi: vec!["Caro energia".into()]
        }]
    );
    let json = serde_json::to_value(&r.avvisi[0]).unwrap();
    assert_eq!(json["tipo"], "situazioni_accese");
    assert!(
        riepilogo_di(con_ingresso(file_vero(), "energia", 9.0))
            .avvisi
            .is_empty()
    );
}

#[test]
fn il_debito_al_massimo_accende_l_avviso_di_crisi() {
    let r = riepilogo_di(con_ingresso(file_base(), "debito_pil", 160.0));
    let [
        Avviso::CrisiDelDebito {
            spread_pb,
            soglia_pb,
        },
    ] = r.avvisi.as_slice()
    else {
        panic!("un avviso solo: {:?}", r.avvisi);
    };
    assert_eq!(*soglia_pb, 550);
    assert!(spread_pb > soglia_pb, "spread {spread_pb}");
    assert_eq!(*spread_pb, r.spread_pb);
    let json = serde_json::to_value(&r.avvisi[0]).unwrap();
    assert_eq!(json["tipo"], "crisi_del_debito");
}

/// L'avviso c'è se e solo se la prima chiusura apre davvero le Aste scoperte. Vale lontano dalla
/// soglia (debito 110 e 160): vicino, la chiusura ricalcola lo spread col ministro vero.
#[test]
fn l_avviso_di_crisi_coincide_con_le_aste_della_prima_chiusura() {
    let aste = dati_base().evento("aste_scoperte").unwrap();
    for debito in [110.0, 160.0] {
        let f = con_ingresso(file_base(), "debito_pil", debito);
        let avviso = riepilogo_di(f.clone())
            .avvisi
            .iter()
            .any(|a| matches!(a, Avviso::CrisiDelDebito { .. }));
        let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
        chiudi_col_bilancio(&mut p);
        let json: serde_json::Value = serde_json::from_str(&p.salva().unwrap()).unwrap();
        let aperte = json["eventi"]["aperti"]
            .as_array()
            .unwrap()
            .iter()
            .any(|e| e["evento"] == aste);
        assert_eq!(avviso, aperte, "debito {debito}");
    }
}

#[test]
fn gli_avvisi_di_stato_seguono_l_ordine() {
    let r = riepilogo_di(con_ingresso(file_vero(), "debito_pil", 160.0));
    assert!(matches!(
        r.avvisi.as_slice(),
        [
            Avviso::SituazioniAccese { .. },
            Avviso::CrisiDelDebito { .. }
        ]
    ));
}

#[test]
fn senza_posizioni_gli_avvisi_vengono_dalla_partita_del_governo_uscente() {
    let mut f = con_ingresso(file_vero(), "debito_pil", 160.0);
    f.scenario = f
        .scenario
        .lines()
        .map(|l| {
            if l.starts_with("posizione = ") {
                "posizione = 4.0"
            } else {
                l
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    let r = riepilogo_di(f);
    assert!(r.posizioni.is_empty());
    assert!(matches!(
        r.avvisi.as_slice(),
        [
            Avviso::SituazioniAccese { .. },
            Avviso::CrisiDelDebito { .. }
        ]
    ));
}

#[test]
fn le_situazioni_accese_stanno_nell_ordine_dei_dati() {
    // «scende»: si accende sotto `accensione`, quindi alzarla sopra il valore di oggi la accende.
    let f = sostituisci(
        file_vero(),
        "eventi",
        "accensione = 85.0\nspegnimento = 92.0",
        "accensione = 110.0\nspegnimento = 115.0",
    );
    let r = riepilogo_di(f);
    let [a @ Avviso::SituazioniAccese { nomi }] = r.avvisi.as_slice() else {
        panic!("un avviso solo: {:?}", r.avvisi);
    };
    assert_eq!(nomi, &["Criminalità", "Caro energia"]);
    assert_eq!(
        a.testo(),
        "Già accese al turno zero: Criminalità e Caro energia. Pesano finché non rientrano."
    );
}

#[test]
fn il_catalogo_della_base() {
    let c = catalogo(&dati_base()).unwrap();
    let nomi: Vec<_> = c.cursori.iter().map(|x| x.nome.as_str()).collect();
    assert_eq!(
        nomi,
        [
            "debito_pil",
            "disoccupazione",
            "inflazione",
            "ricchezza",
            "eta_mediana",
            "orientamento"
        ]
    );
    assert_eq!(
        (c.cursori[0].valore, c.cursori[0].min, c.cursori[0].max),
        (136.7, 110.0, 160.0)
    );
    let leggi: Vec<_> = c.leggi_elettorali.keys().map(String::as_str).collect();
    assert_eq!(
        leggi,
        ["legge_180", "proporzionale_3", "proporzionale_puro"]
    );
    assert_eq!(c.leggi_elettorali["legge_180"].len(), 9);
    assert_eq!(c.legge_di_base, "legge_180");
    assert_eq!(c.disposizioni.len(), 46);
    assert_eq!(
        c.disposizioni
            .iter()
            .filter(|d| d.norma == Norma::Ordinaria)
            .count(),
        35
    );
    assert_eq!(c.partiti.len(), 7);
    assert_eq!(
        (c.partiti[0].id.as_str(), c.partiti[0].posizione),
        ("estrema_sinistra", 1.0)
    );
    let forfettario = c
        .disposizioni
        .iter()
        .find(|d| d.id == "forfettario")
        .unwrap();
    assert_eq!(forfettario.dicastero.as_deref(), Some("Economia e Finanze"));
    assert_eq!(forfettario.partenza, "85000");
    assert_eq!(forfettario.gradini.len(), 4);
}

#[test]
fn i_gradini_di_ogni_legge_coincidono_coi_dati_che_ne_fa_il_core() {
    let dati = dati_base();
    let c = catalogo(&dati).unwrap();
    assert_eq!(c.leggi_elettorali.len(), 3);
    for (legge, gradini) in &c.leggi_elettorali {
        let mut s = SuMisura::base(&dati);
        s.cursori.legge_elettorale.clone_from(legge);
        let x = dati.con_su_misura(&s).unwrap();
        let attesi: Vec<_> = x
            .disposizioni()
            .iter()
            .filter(|d| d.norma == Norma::Elettorale)
            .map(|d| (d.id.clone(), d.partenza.clone()))
            .collect();
        assert_eq!(attesi.len(), 9, "{legge}");
        let trovati: Vec<_> = gradini
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        let mut attesi = attesi;
        attesi.sort();
        assert_eq!(trovati, attesi, "{legge}");
    }
}

/// Oracolo: `.scratch/v1-piano-11-ui/viste/14-editor.json`, `leggi_elettorali`.
#[test]
fn due_leggi_elettorali_hanno_gradini_diversi() {
    let c = catalogo(&dati_base()).unwrap();
    let gradino = |legge: &str, id: &str| c.leggi_elettorali[legge][id].as_str();
    let attesi = [
        ("premio", "coalizione", "nessuno"),
        ("soglia_lista", "300", "0"),
        ("tetto", "5500_5650", "nessuno"),
    ];
    for (id, legge_180, puro) in attesi {
        assert_eq!(gradino("legge_180", id), legge_180, "{id}");
        assert_eq!(gradino("proporzionale_puro", id), puro, "{id}");
    }
}
