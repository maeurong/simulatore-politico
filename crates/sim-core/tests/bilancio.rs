#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

mod comune;
use comune::{dati_base, senza_shock};
use sim_core::dati::DatiGioco;
use sim_core::economia::{Bilancio, Voce};
use sim_core::elettorato::{Fascia, fascia_di};
use sim_core::{Comando, Errore, Partita};

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
