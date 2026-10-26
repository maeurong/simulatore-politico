#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

mod comune;
use comune::{dati_base, edita, senza_shock};
use serde_json::json;
use sim_core::dati::DatiGioco;
use sim_core::economia::{Aliquote, Bilancio, RiepilogoBilancio, Voce, irpef};
use sim_core::elettorato::{Condizione, Fascia, fascia_di};
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
    b.dicasteri[3].spesa += 1_000_000_000;
    assert_eq!(perdita(&p.riepilogo_bilancio(&b).unwrap(), "sanita"), 0.0);
}

#[test]
fn alzare_la_prima_soglia_tocca_la_perdita_del_primo_scaglione_e_non_del_secondo() {
    let p = al_bilancio_2028(1);
    let bozza = p.bozza_bilancio().unwrap();
    let prima = p.riepilogo_bilancio(&bozza).unwrap();
    let mut b = bozza.clone();
    b.aliquote.irpef[0].fino_a = b.aliquote.irpef[0].fino_a.map(|t| t + 200);
    let dopo = p.riepilogo_bilancio(&b).unwrap();
    assert!(perdita(&dopo, "scaglione 1") < perdita(&prima, "scaglione 1"));
    assert_eq!(
        perdita(&dopo, "scaglione 2"),
        perdita(&prima, "scaglione 2")
    );
    let n = bozza.aliquote.irpef.len() - 1;
    assert_eq!(p.vista().bilancio.perdita_reale_scaglioni.len(), n);
    assert!(
        !dopo
            .perdita_reale
            .iter()
            .any(|x| x.voce == format!("scaglione {}", n + 1))
    );
}

#[test]
fn indicizzare_un_dicastero_o_gli_scaglioni_tocca_solo_quelli() {
    let p = al_bilancio_2028(1);
    let indice = p.informazione().statistica.indice_prezzi;
    assert_ne!(indice, p.economia().livello_prezzi);
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
    assert!(b.aliquote.irpef.iter().all(|s| s.livello_prezzi == indice));
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
    let guasti: [fn(&mut serde_json::Value); 5] = [
        |b| b["dicasteri"][0]["livello_prezzi"] = json!(0.0),
        |b| b["fondo_riserva"] = json!(-1),
        |b| b["dicasteri"][0]["livello_prezzi"] = json!(1e7),
        |b| b["dicasteri"][0]["margine"] = json!(-1),
        |b| b["aliquote"]["irpef"][1]["fino_a"] = json!(2_000_000_000_000_000_i64),
    ];
    for guasta in guasti {
        let r = edita(&p, |j| guasta(&mut j["economia"]["bilancio_presentato"]));
        assert!(matches!(r, Err(Errore::SalvataggioIlleggibile(_))), "{r:?}");
    }
}

/// Crescita % della spesa netta del 2028 sul 2027 come la legge il controllo UE, a fine 2028.
fn crescita_vera_2028(mut p: Partita) -> f64 {
    for _ in 0..5 {
        p.chiudi_turno();
    }
    let sn = &p.economia().regole_ue.spesa_netta;
    100.0 * (sn[&2028] as f64 / sn[&2027] as f64 - 1.0)
}

#[test]
fn il_riepilogo_prevede_la_crescita_della_spesa_netta_del_controllo_ue() {
    let p = al_bilancio_2028(1);
    let bozza = p.bozza_bilancio().unwrap();
    for b in [bozza.clone(), p.indicizza(&bozza, &Voce::Tutto).unwrap()] {
        let prevista = p.riepilogo_bilancio(&b).unwrap().crescita_spesa_netta;
        let mut q = edita(&p, |_| {}).unwrap();
        presenta(&mut q, b).unwrap();
        let vera = crescita_vera_2028(q);
        assert!((prevista - vera).abs() < 0.5, "{prevista} {vera}");
    }
}

fn presentato(p: &Partita) -> &Bilancio {
    p.economia().bilancio_presentato.as_ref().unwrap()
}

#[test]
fn indicizza_tutto_indicizza_la_spesa_intera_e_il_resto_oltre_il_tetto_si_limita() {
    let mut p = al_bilancio_2028(1);
    let indice = p.informazione().statistica.indice_prezzi;
    let bozza = p.bozza_bilancio().unwrap();
    let tutto = p.indicizza(&bozza, &Voce::Tutto).unwrap();
    presenta(&mut p, tutto.clone()).unwrap();
    for (d, z) in presentato(&p).dicasteri.iter().zip(&bozza.dicasteri) {
        let attesa = (z.spesa as f64 * indice / z.livello_prezzi).round() as i64;
        assert!(
            (d.spesa - attesa).abs() <= 3,
            "{} {} {attesa}",
            d.id,
            d.spesa
        );
    }
    let spese = |b: &Bilancio| b.dicasteri.iter().map(|d| d.spesa).collect::<Vec<_>>();
    let indicizzate = spese(presentato(&p));
    let mut oltre = tutto;
    oltre.dicasteri[3].spesa += 1_000_000_000_000;
    presenta(&mut p, oltre).unwrap();
    assert_eq!(spese(presentato(&p)), indicizzate);
}

fn senza_shock_dati() -> DatiGioco {
    DatiGioco::da_testi(&senza_shock()).unwrap()
}

/// Due partite gemelle in ottobre-dicembre 2027: `a` presenta la bozza, `b` la bozza ritoccata.
fn coppia_presentata(seme: u64, ritocco: impl Fn(&mut Bilancio)) -> (Partita, Partita) {
    let mut a = al_bilancio_2028(seme);
    let mut b = edita(&a, |_| {}).unwrap();
    let bozza = a.bozza_bilancio().unwrap();
    let mut x = bozza.clone();
    ritocco(&mut x);
    presenta(&mut a, bozza).unwrap();
    presenta(&mut b, x).unwrap();
    (a, b)
}

/// Le variazioni `BudgetTagliato` della sanità registrate al turno `turno`.
fn tagli_sanita(p: &Partita, turno: u32) -> Vec<f64> {
    (p.ministri().dicasteri[3].variazioni.iter())
        .filter(|v| v.causa == Causa::BudgetTagliato && v.turno == turno)
        .map(|v| v.delta)
        .collect()
}

#[test]
fn iva_presentata_in_dicembre_da_gettito_esatto_a_gennaio() {
    let e = senza_shock_dati().manopola("elasticita_imponibile_iva");
    let (mut a, mut b) = coppia_presentata(1, |x| x.aliquote.iva = 0.20);
    for _ in 0..2 {
        a.chiudi_turno();
        b.chiudi_turno();
    }
    let (ca, cb) = (&a.economia().conti, &b.economia().conti);
    let atteso = 0.20 / 0.22 * (0.80_f64 / 0.78).powf(e);
    let rapporto = cb.entrate.iva as f64 / ca.entrate.iva as f64;
    assert!((rapporto - atteso).abs() < 1e-6, "{rapporto} {atteso}");
    assert_eq!(
        (cb.entrate.ires, cb.entrate.contributi),
        (ca.entrate.ires, ca.entrate.contributi)
    );
    assert!((cb.entrate_discrezionali - (cb.entrate.iva - ca.entrate.iva)).abs() <= 1);
    assert_eq!(ca.entrate_discrezionali, 0);
}

#[test]
fn il_gettito_nuovo_non_anticipa_a_dicembre() {
    let (mut a, mut b) = coppia_presentata(1, |x| x.aliquote.iva = 0.20);
    a.chiudi_turno();
    b.chiudi_turno();
    assert_eq!(a.economia().conti, b.economia().conti);
    assert_eq!(a.misure(), b.misure());
}

#[test]
fn irpef_presentata_da_imposte_di_gruppo_esatte() {
    let d = senza_shock_dati();
    let (_, mut b) = coppia_presentata(1, |x| x.aliquote.irpef[2].aliquota = 0.60);
    b.chiudi_turno();
    b.chiudi_turno();
    let s = &b.economia().bilancio.aliquote;
    assert_eq!(s.irpef[2].aliquota, 0.60);
    let tasso = |a: &Aliquote, l: f64| irpef(&a.irpef, l) / l + a.contributi;
    let de = d.elettorato();
    let lordo = |k: usize| {
        let r = &b.elettorato().gruppi[k].reddito;
        r.primario + r.trasferimenti
    };
    let k = (0..de.gruppi.len())
        .find(|&k| de.gruppi[k].condizione != Condizione::Autonomo && lordo(k) > 50_000.0)
        .unwrap();
    let dato = &de.gruppi[k];
    let r = tasso(s, lordo(k)) / tasso(&d.scenario().aliquote, dato.primario + dato.trasferimenti);
    let atteso = (de.aliquote[&dato.fascia] * lordo(k) * r).min(lordo(k));
    let imposte = b.elettorato().gruppi[k].reddito.imposte;
    assert!(
        (imposte - atteso).abs() <= 1e-9 * atteso,
        "{imposte} {atteso}"
    );
}

#[test]
fn salva_carica_fra_presentazione_e_aula_non_cambia_il_futuro() {
    let mut p = al_bilancio_2028(1);
    let mut b = p.bozza_bilancio().unwrap();
    b.dicasteri[3].margine *= 2;
    b.aliquote.iva = 0.20;
    presenta(&mut p, b).unwrap();
    let mut q = Partita::carica(&p.salva().unwrap()).unwrap();
    assert!(q.vista().bilancio.presentato);
    for _ in 0..3 {
        p.chiudi_turno();
        q.chiudi_turno();
        assert_eq!(p.salva().unwrap(), q.salva().unwrap());
    }
    assert_eq!(q.economia().bilancio.aliquote.iva, 0.20);
}

#[test]
fn provvisorio_poi_secondo_anno_poi_terzo() {
    let mut p = al_bilancio_2028(3);
    p.chiudi_turno();
    assert!(p.vista().bilancio.esercizio_provvisorio);
    let mut b = p.bozza_bilancio().unwrap();
    b.aliquote.iva = 0.20;
    b.dicasteri[3].margine *= 2;
    let margine = b.dicasteri[3].margine;
    presenta(&mut p, b).unwrap();
    p.chiudi_turno();
    let v = p.vista();
    assert_eq!(
        (v.bilancio.anno, v.ministeri[3].margine_residuo),
        (2028, margine / 4 * 3)
    );
    assert!(!v.bilancio.presentabile);
    p.chiudi_turno();
    assert!(!p.vista().bilancio.presentabile);
    p.chiudi_turno();
    let bozza = p.bozza_bilancio().unwrap();
    assert_eq!((bozza.anno, bozza.aliquote.iva), (2029, 0.20));
    presenta(&mut p, bozza).unwrap();
    p.chiudi_turno();
    let v = p.vista();
    assert!(!v.bilancio.esercizio_provvisorio);
    assert_eq!(v.bilancio.anno, 2029);
    p.chiudi_turno();
    assert_eq!(p.economia().bilancio.aliquote.iva, 0.20);
    assert_eq!(p.economia().bilancio.anno, 2029);
}

#[test]
fn due_gennai_provvisori_di_fila_senza_presentare() {
    let mut p = al_bilancio_2028(5);
    p.chiudi_turno();
    assert!(p.vista().bilancio.esercizio_provvisorio);
    for _ in 0..4 {
        p.chiudi_turno();
    }
    let v = p.vista();
    assert_eq!((v.data.anno, v.data.trimestre), (2029, 1));
    assert!(v.bilancio.esercizio_provvisorio);
    let b = p.bozza_bilancio().unwrap();
    assert_eq!(b.anno, 2029);
    presenta(&mut p, b).unwrap();
    assert!(Partita::carica(&p.salva().unwrap()).is_ok());
    p.chiudi_turno();
    assert!(!p.vista().bilancio.esercizio_provvisorio);
}

#[test]
fn taglio_budget_costa_meno_costo_per_perdita_reale() {
    let costo = senza_shock_dati().manopola("costo_taglio_budget");
    let mut p = al_bilancio_2028(4);
    let mut b = p.bozza_bilancio().unwrap();
    b.dicasteri[3].salari = b.dicasteri[3].salari * 9 / 10;
    let attesa = perdita(&p.riepilogo_bilancio(&b).unwrap(), "sanita");
    presenta(&mut p, b).unwrap();
    let turno = p.vista().turno;
    p.chiudi_turno();
    let tagli = tagli_sanita(&p, turno);
    assert_eq!(tagli.len(), 1, "{tagli:?}");
    assert!(
        (tagli[0] + costo * attesa).abs() < 1e-9,
        "{tagli:?} {attesa}"
    );
}

#[test]
fn ingresso_in_provvisorio_costa_la_perdita_del_bilancio_che_continua() {
    let costo = senza_shock_dati().manopola("costo_taglio_budget");
    let mut p = al_bilancio_2028(4);
    p.chiudi_turno();
    let v = p.vista();
    let continua = v.bilancio.dicasteri[3].perdita_reale;
    assert!(continua > 0.0);
    let tagli = tagli_sanita(&p, v.turno);
    assert_eq!(tagli.len(), 1, "{tagli:?}");
    assert!((tagli[0] + costo * continua).abs() < 1e-9);
    // Approvare nello stesso gennaio costa di nuovo, sulla perdita del bilancio nuovo (Ruling).
    let b = p.bozza_bilancio().unwrap();
    let nuova = perdita(&p.riepilogo_bilancio(&b).unwrap(), "sanita");
    presenta(&mut p, b).unwrap();
    p.chiudi_turno();
    let tagli = tagli_sanita(&p, v.turno);
    assert_eq!(tagli.len(), 2, "{tagli:?}");
    assert!((tagli[1] + costo * nuova).abs() < 1e-9);
}

#[test]
fn indice_pubblicato_non_livello_vero() {
    let p = al_bilancio_2028(1);
    let q = edita(&p, |j| {
        let s = &mut j["informazione"]["statistica"]["indice_prezzi"];
        *s = json!(s.as_f64().unwrap() * 1.1);
    })
    .unwrap();
    assert_eq!(q.economia().livello_prezzi, p.economia().livello_prezzi);
    let bozza = q.bozza_bilancio().unwrap();
    let salari = |x: &Partita| x.indicizza(&bozza, &Voce::Tutto).unwrap().dicasteri[3].salari;
    let rapporto = (salari(&q) - bozza.dicasteri[3].salari) as f64
        / (salari(&p) - bozza.dicasteri[3].salari) as f64;
    let indice = |x: &Partita| x.informazione().statistica.indice_prezzi;
    let l = bozza.dicasteri[3].livello_prezzi;
    let atteso = (1.1 * indice(&p) - l) / (indice(&p) - l);
    assert!((rapporto - atteso).abs() < 1e-6, "{rapporto} {atteso}");
    let fotocopia = perdita(&q.riepilogo_bilancio(&bozza).unwrap(), "sanita");
    assert!((fotocopia - 100.0 * (1.0 - l / indice(&q))).abs() < 1e-9);
}

#[test]
fn riepilogo_costo_entrate_coincide_coi_conti_del_trimestre_dopo() {
    let mut a = al_bilancio_2028(1);
    a.chiudi_turno();
    assert!(a.vista().bilancio.esercizio_provvisorio);
    let mut b = edita(&a, |_| {}).unwrap();
    let bozza = a.bozza_bilancio().unwrap();
    let mut x = bozza.clone();
    x.aliquote.iva = 0.20;
    let costo = b.riepilogo_bilancio(&x).unwrap().costo_entrate as f64;
    presenta(&mut a, bozza).unwrap();
    presenta(&mut b, x).unwrap();
    a.chiudi_turno();
    b.chiudi_turno();
    let imposte = |p: &Partita| {
        let e = &p.economia().conti.entrate;
        (e.irpef + e.ires + e.iva + e.contributi) as f64
    };
    let realizzato = 4.0 * (imposte(&a) - imposte(&b));
    // Il riepilogo legge l'ultimo trimestre chiuso; il trimestre dopo è cresciuto in nominale.
    assert!(
        (costo / realizzato - 1.0).abs() < 0.06,
        "{costo} {realizzato}"
    );
}

#[test]
fn rifiuti_aggiuntivi_lasciano_la_partita_identica() {
    let zero = Partita::nuova(dati_base(), 1);
    let e = (zero.riepilogo_bilancio(&zero.economia().bilancio)).unwrap_err();
    assert!(e.to_string().contains("fuori stagione"), "{e}");
    let mut p = al_bilancio_2028(1);
    p.chiudi_turno();
    let prima = p.salva().unwrap();
    let bozza = p.bozza_bilancio().unwrap();
    let guasti: Vec<(Guasto, &str)> = vec![
        (Box::new(|b| b.anno = 2027), "anno 2027, atteso 2028"),
        (Box::new(|b| b.fondo_riserva = -1), "fondo_riserva"),
        (Box::new(|b| b.aliquote.ires = 0.95), "ires"),
        (Box::new(|b| b.aliquote.contributi = f64::NAN), "contributi"),
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
    let mut enorme = bozza.clone();
    enorme.dicasteri[0].salari = 1_000_000_000_000_000;
    let e = p.indicizza(&enorme, &Voce::Tutto).unwrap_err();
    assert!(e.to_string().contains("voce negativa o enorme"), "{e}");
    assert_eq!(p.salva().unwrap(), prima);
}

#[test]
fn presentare_in_aprile_giugno_e_fuori_stagione() {
    let mut p = Partita::nuova(senza_shock_dati(), 1);
    p.chiudi_turno();
    p.chiudi_turno();
    assert_eq!(p.vista().data.trimestre, 2);
    let prima = p.salva().unwrap();
    let b = p.economia().bilancio.clone();
    let e = presenta(&mut p, b).unwrap_err();
    assert!(e.to_string().contains("fuori stagione"), "{e}");
    assert_eq!(p.salva().unwrap(), prima);
}

#[test]
fn indice_prezzi_non_positivo_o_non_numerico_nel_salvataggio_rifiutato() {
    let p = Partita::nuova(dati_base(), 1);
    for guasto in [json!(0.0), json!(-1.0), json!(null)] {
        let r = edita(&p, |j| {
            j["informazione"]["statistica"]["indice_prezzi"] = guasto
        });
        assert!(matches!(r, Err(Errore::SalvataggioIlleggibile(_))), "{r:?}");
    }
}
