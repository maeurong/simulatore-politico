#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

mod comune;
use comune::{dati_base, edita, senza_shock};
use serde_json::json;
use sim_core::dati::DatiGioco;
use sim_core::economia::{Bilancio, RiepilogoBilancio, Voce};
use sim_core::elettorato::{Fascia, fascia_di};
use sim_core::ministri::Causa;
use sim_core::{Comando, Errore, Partita};

fn perdita(r: &RiepilogoBilancio, voce: &str) -> f64 {
    r.perdita_reale
        .iter()
        .find(|x| x.voce == voce)
        .unwrap()
        .perdita
}

/// Nessun ministro ha una variazione `BudgetTagliato` nella vista.
fn nessun_taglio(p: &Partita) -> bool {
    (p.vista().ministeri.iter())
        .flat_map(|m| m.ministro.iter().flat_map(|x| &x.variazioni))
        .all(|v| v.causa != Causa::BudgetTagliato)
}

/// Al turno 4 (ottobre-dicembre 2027) la bozza del 2028 è aperta.
fn al_bilancio_2028(seme: u64) -> Partita {
    let mut p = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), seme);
    for _ in 0..4 {
        p.chiudi_turno();
    }
    p
}

fn presenta(p: &mut Partita, b: Bilancio) -> Result<(), Errore> {
    p.comanda(Comando::PresentaBilancio { bilancio: b })
}

type Guasto = Box<dyn Fn(&mut Bilancio)>;

#[test]
fn bozza_solo_in_ottobre_dicembre_senza_bilancio_approvato() {
    assert!(Partita::nuova(dati_base(), 1).bozza_bilancio().is_none());
    let p = al_bilancio_2028(1);
    let b = p.bozza_bilancio().unwrap();
    assert_eq!(b.anno, 2028);
    assert_eq!(b.dicasteri, p.economia().bilancio.dicasteri);
}

#[test]
fn bilancio_fotocopia_e_un_taglio_reale_e_indicizzare_lo_annulla() {
    let p = al_bilancio_2028(1);
    let bozza = p.bozza_bilancio().unwrap();
    let r = p.riepilogo_bilancio(&bozza).unwrap();
    assert_eq!((r.costo_spesa, r.costo_entrate), (0, 0));
    assert!(r.perdita_reale.iter().all(|x| x.perdita > 0.0), "{r:?}");
    let tutto = p.indicizza(&bozza, &Voce::Tutto).unwrap();
    let r = p.riepilogo_bilancio(&tutto).unwrap();
    assert!(r.costo_spesa > 0);
    assert!(
        r.perdita_reale.iter().all(|x| x.perdita.abs() < 1e-9),
        "{r:?}"
    );
}

#[test]
fn bilancio_presentato_vale_dall_anno_dopo() {
    let mut p = al_bilancio_2028(2);
    let mut b = p.bozza_bilancio().unwrap();
    b.dicasteri[3].margine *= 2;
    let margine = b.dicasteri[3].margine;
    presenta(&mut p, b).unwrap();
    p.chiudi_turno();
    let v = p.vista();
    assert!(!v.bilancio.esercizio_provvisorio);
    assert_eq!(v.bilancio.anno, 2028);
    assert_eq!(v.ministeri[3].margine_residuo, margine);
}

#[test]
fn bilancio_non_presentato_porta_l_esercizio_provvisorio() {
    let mut p = al_bilancio_2028(3);
    p.chiudi_turno();
    let v = p.vista();
    assert!(v.bilancio.esercizio_provvisorio && v.bilancio.presentabile);
    assert!(
        v.ministeri.iter().all(|m| m.agenda.is_none()
            && m.motivo_senza_agenda.as_deref() == Some("esercizio provvisorio"))
    );
    let b = p.bozza_bilancio().unwrap();
    assert_eq!(b.anno, 2028);
    presenta(&mut p, b).unwrap();
    p.chiudi_turno();
    assert!(!p.vista().bilancio.esercizio_provvisorio);
    assert_eq!(p.economia().bilancio.anno, 2028);
}

#[test]
fn esercizio_provvisorio_rifiuta_i_fondi_e_l_approvato_lascia_tre_quarti_del_margine() {
    let mut p = al_bilancio_2028(3);
    p.chiudi_turno();
    let prima = p.salva().unwrap();
    let e = p
        .comanda(Comando::Accogli {
            dicastero: "sanita".into(),
        })
        .unwrap_err();
    assert!(
        e.to_string()
            .contains("esercizio provvisorio: nessuna misura che costi denaro")
    );
    assert_eq!(p.salva().unwrap(), prima);
    assert!(p.vista().ministeri.iter().all(|m| m.proposta.is_none()));
    let mut b = p.bozza_bilancio().unwrap();
    b.dicasteri[3].margine *= 2;
    let margine = b.dicasteri[3].margine;
    presenta(&mut p, b).unwrap();
    p.chiudi_turno();
    assert_eq!(p.vista().ministeri[3].margine_residuo, margine / 4 * 3);
}

#[test]
fn taglio_al_dicastero_costa_soddisfazione() {
    let soddisfazione = |taglio: bool| {
        let mut p = al_bilancio_2028(4);
        let mut b = p.bozza_bilancio().unwrap();
        if taglio {
            b.dicasteri[3].salari = b.dicasteri[3].salari * 9 / 10;
        }
        presenta(&mut p, b).unwrap();
        p.chiudi_turno();
        p.vista().ministeri[3]
            .ministro
            .clone()
            .unwrap()
            .soddisfazione
    };
    assert!(soddisfazione(true) < soddisfazione(false));
}

#[test]
fn scaglione_di_mezzo_piu_basso_aiuta_la_fascia_alta() {
    let imposte = |aliquota: f64| {
        let mut p = al_bilancio_2028(5);
        let mut b = p.bozza_bilancio().unwrap();
        b.aliquote.irpef[1].aliquota = aliquota;
        presenta(&mut p, b).unwrap();
        p.chiudi_turno();
        p.chiudi_turno();
        let g = &p.elettorato().gruppi;
        let media = |f: Fascia| {
            let x: Vec<f64> = (0..g.len())
                .filter(|&i| fascia_di(i) == f)
                .map(|i| g[i].reddito.imposte)
                .collect();
            x.iter().sum::<f64>() / x.len() as f64
        };
        (media(Fascia::Alta), media(Fascia::Bassa))
    };
    let (alta_33, bassa_33) = imposte(0.33);
    let (alta_28, bassa_28) = imposte(0.28);
    assert!(alta_28 / alta_33 < bassa_28 / bassa_33);
}

#[test]
fn margine_zero_nessuna_agenda() {
    let mut p = al_bilancio_2028(6);
    let mut b = p.bozza_bilancio().unwrap();
    for d in &mut b.dicasteri {
        d.margine = 0;
    }
    presenta(&mut p, b).unwrap();
    p.chiudi_turno();
    let v = p.vista();
    assert!(v.ministeri.iter().all(
        |m| m.agenda.is_none() && m.motivo_senza_agenda.as_deref() == Some("margine esaurito")
    ));
    p.chiudi_turno();
}

#[test]
fn iva_tagliata_pesa_sul_tetto_ue() {
    let p = al_bilancio_2028(7);
    let bozza = p.bozza_bilancio().unwrap();
    let mut b = bozza.clone();
    b.aliquote.iva = 0.20;
    let r = p.riepilogo_bilancio(&b).unwrap();
    assert!(r.costo_entrate > 0);
    assert!(r.crescita_spesa_netta > p.riepilogo_bilancio(&bozza).unwrap().crescita_spesa_netta);
}

#[test]
fn bilancio_malformato_rifiutato_col_nome() {
    let mut p = al_bilancio_2028(8);
    let prima = p.salva().unwrap();
    let bozza = p.bozza_bilancio().unwrap();
    let guasti: Vec<(Guasto, &str)> = vec![
        (
            Box::new(|b| b.aliquote.irpef[1].fino_a = Some(10_000)),
            "scaglioni",
        ),
        (Box::new(|b| b.aliquote.iva = 1.2), "iva"),
        (Box::new(|b| b.dicasteri[8].acquisti = -1), "giustizia"),
        (Box::new(|b| b.dicasteri.swap(0, 1)), "dicasteri"),
        (Box::new(|b| b.anno = 2030), "2030"),
    ];
    for (guasta, nome) in guasti {
        let mut b = bozza.clone();
        guasta(&mut b);
        let e = presenta(&mut p, b).unwrap_err();
        assert!(
            matches!(e, Errore::Bilancio(_)) && e.to_string().contains(nome),
            "{e}"
        );
    }
    let mut fuori = Partita::nuova(dati_base(), 8);
    let e = presenta(&mut fuori, bozza).unwrap_err();
    assert!(e.to_string().contains("fuori stagione"), "{e}");
    assert_eq!(p.salva().unwrap(), prima);
}

#[test]
fn taglio_nominale_alza_la_perdita_e_l_aumento_oltre_l_indice_la_azzera() {
    let p = al_bilancio_2028(1);
    let bozza = p.bozza_bilancio().unwrap();
    let fotocopia = perdita(&p.riepilogo_bilancio(&bozza).unwrap(), "sanita");
    let mut b = bozza.clone();
    b.dicasteri[3].salari = b.dicasteri[3].salari * 9 / 10;
    let tagliata = perdita(&p.riepilogo_bilancio(&b).unwrap(), "sanita");
    assert!(
        tagliata > fotocopia && fotocopia > 0.0,
        "{tagliata} {fotocopia}"
    );
    let sanita = Voce::Dicastero {
        id: "sanita".into(),
    };
    let mut b = p.indicizza(&bozza, &sanita).unwrap();
    b.dicasteri[3].salari += 1_000_000_000;
    assert_eq!(perdita(&p.riepilogo_bilancio(&b).unwrap(), "sanita"), 0.0);
}

#[test]
fn indicizzare_un_dicastero_o_gli_scaglioni_tocca_solo_quelli() {
    let p = al_bilancio_2028(1);
    let indice = p.informazione().statistica.indice_prezzi;
    let bozza = p.bozza_bilancio().unwrap();
    let sanita = Voce::Dicastero {
        id: "sanita".into(),
    };
    let b = p.indicizza(&bozza, &sanita).unwrap();
    for (i, (d, z)) in b.dicasteri.iter().zip(&bozza.dicasteri).enumerate() {
        assert_eq!(i == 3, d != z, "{}", d.id);
    }
    assert!(b.dicasteri[3].salari > bozza.dicasteri[3].salari);
    assert_eq!(b.dicasteri[3].livello_prezzi, indice);
    assert_eq!(b.aliquote, bozza.aliquote);
    let b = p.indicizza(&bozza, &Voce::Scaglioni).unwrap();
    assert_eq!(b.dicasteri, bozza.dicasteri);
    assert_eq!(b.livello_prezzi_scaglioni, indice);
    let soglie = |x: &Bilancio| {
        x.aliquote
            .irpef
            .iter()
            .map(|s| s.fino_a)
            .collect::<Vec<_>>()
    };
    assert!(
        soglie(&b)
            .iter()
            .zip(soglie(&bozza))
            .all(|(n, v)| n.unwrap_or(0) >= v.unwrap_or(0))
    );
    assert_ne!(soglie(&b), soglie(&bozza));
    let e = p
        .indicizza(
            &bozza,
            &Voce::Dicastero {
                id: "nessuno".into(),
            },
        )
        .unwrap_err();
    assert!(e.to_string().contains("nessuno"), "{e}");
}

#[test]
fn il_bilancio_del_governo_uscente_non_costa_soddisfazione() {
    let mut p = Partita::nuova(dati_base(), 1);
    p.chiudi_turno();
    assert!(nessun_taglio(&p));
}

#[test]
fn bilancio_indicizzato_non_costa_soddisfazione() {
    let mut p = al_bilancio_2028(4);
    let b = p
        .indicizza(&p.bozza_bilancio().unwrap(), &Voce::Tutto)
        .unwrap();
    presenta(&mut p, b).unwrap();
    p.chiudi_turno();
    assert!(nessun_taglio(&p));
}

#[test]
fn ripresentare_sostituisce() {
    let mut p = al_bilancio_2028(2);
    let bozza = p.bozza_bilancio().unwrap();
    let mut b = bozza.clone();
    b.dicasteri[3].margine *= 2;
    presenta(&mut p, b).unwrap();
    presenta(&mut p, bozza.clone()).unwrap();
    assert!(p.vista().bilancio.presentato);
    p.chiudi_turno();
    assert_eq!(
        p.vista().ministeri[3].margine_residuo,
        bozza.dicasteri[3].margine
    );
}

#[test]
fn bilancio_presentato_guasto_rifiutato_al_caricamento() {
    let zero = Partita::nuova(dati_base(), 1);
    let fuori = edita(&zero, |j| {
        let e = &mut j["economia"];
        e["bilancio_presentato"] = e["bilancio"].clone();
    });
    assert!(
        matches!(fuori, Err(Errore::SalvataggioIlleggibile(_))),
        "{fuori:?}"
    );
    let mut p = al_bilancio_2028(9);
    let bozza = p.bozza_bilancio().unwrap();
    presenta(&mut p, bozza).unwrap();
    assert!(edita(&p, |_| {}).is_ok());
    let guasti: [fn(&mut serde_json::Value); 2] = [
        |b| b["dicasteri"][0]["livello_prezzi"] = json!(0.0),
        |b| b["fondo_riserva"] = json!(-1),
    ];
    for guasta in guasti {
        let r = edita(&p, |j| guasta(&mut j["economia"]["bilancio_presentato"]));
        assert!(matches!(r, Err(Errore::SalvataggioIlleggibile(_))), "{r:?}");
    }
}
