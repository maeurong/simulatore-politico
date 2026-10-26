mod comune;
use comune::{dati_base, ferma_tranne, ferma_tutti, imposta, senza_shock};
use sim_core::dati::{DatiGioco, FileDati};
use sim_core::{Comando, Partita};

/// 6 condizioni × 3 fasce: i gruppi di una regione sono contigui (`indice_gruppo`).
const PER_REGIONE: usize = 18;

/// Due partite uguali al turno 1: nella prima `sviluppo_economico` fa un atto rapido sulla
/// domanda della Calabria, nella seconda nessun atto; tutte le altre agende fermate in entrambe.
fn prepara(f: FileDati) -> (Partita, Partita) {
    let d = DatiGioco::da_testi(&f).unwrap();
    let (mut a, mut b) = (Partita::nuova(d.clone(), 3), Partita::nuova(d, 3));
    a.chiudi_turno();
    b.chiudi_turno();
    let v = a.vista();
    let se = v
        .ministeri
        .iter()
        .find(|m| m.dicastero == "sviluppo_economico")
        .unwrap();
    let annunciata = se.agenda.as_ref().unwrap();
    let rapido = se
        .atti
        .iter()
        .find(|x| {
            x.turni == 1
                && x.pagabile
                && x.effetti.iter().any(|e| e.grandezza == "domanda_regionale")
        })
        .unwrap()
        .id
        .clone();
    let atto = (annunciata.atto != rapido).then_some(rapido);
    let regione =
        (annunciata.regione.as_deref() != Some("calabria")).then(|| "calabria".to_string());
    if atto.is_some() || regione.is_some() {
        a.comanda(Comando::Dirotta {
            dicastero: "sviluppo_economico".into(),
            atto,
            regione,
        })
        .unwrap();
    }
    ferma_tranne(&mut a, "sviluppo_economico");
    ferma_tutti(&mut b);
    (a, b)
}

fn u(p: &Partita, id: &str) -> f64 {
    p.economia()
        .regioni
        .iter()
        .find(|r| r.id == id)
        .unwrap()
        .disoccupazione
}

#[test]
fn atto_regionale_abbassa_la_disoccupazione_della_regione_bersaglio() {
    let (mut a, mut b) = prepara(senza_shock());
    let (mut calabria, mut lombardia) = (0.0, 0.0);
    for _ in 0..10 {
        a.chiudi_turno();
        b.chiudi_turno();
        calabria += u(&a, "calabria") - u(&b, "calabria");
        lombardia += u(&a, "lombardia") - u(&b, "lombardia");
        ferma_tutti(&mut a);
        ferma_tutti(&mut b);
    }
    assert!(calabria < 0.0);
    assert!(calabria.abs() > 10.0 * lombardia.abs());
}

#[test]
fn merito_dell_atto_al_partito_del_dicastero() {
    let f = imposta(
        senza_shock(),
        "scenario",
        "sviluppo_economico = ",
        "\"centrodestra\"",
    );
    let f = imposta(f, "manopole", "[quota_colpa_presidente]\nvalore = ", "0.0");
    let (mut a, mut b) = prepara(f);
    let j = a
        .partiti()
        .iter()
        .position(|x| x.id == "centrodestra")
        .unwrap();
    let regioni = dati_base();
    let r = |id: &str| regioni.regioni().iter().position(|x| x.id == id).unwrap();
    let media = |p: &Partita, reg: usize| {
        let g = &p.elettorato().gruppi[reg * PER_REGIONE..(reg + 1) * PER_REGIONE];
        g.iter().map(|x| x.giudizio[j]).sum::<f64>() / g.len() as f64
    };
    // Si guarda il massimo e non la fine: il giudizio pesa le perdite più dei guadagni, e un
    // impulso che svanisce chiude in negativo. L'effetto nazionale si cancella nella differenza.
    let mut picco = f64::NEG_INFINITY;
    for _ in 0..6 {
        a.chiudi_turno();
        b.chiudi_turno();
        let d = |reg: usize| media(&a, reg) - media(&b, reg);
        picco = picco.max(d(r("calabria")) - d(r("lombardia")));
        ferma_tutti(&mut a);
        ferma_tutti(&mut b);
    }
    assert!(picco > 0.0, "{picco}");
}

#[test]
fn riscossione_in_calo_toglie_gettito() {
    let ferma = imposta(
        senza_shock(),
        "manopole",
        "[velocita_servizio]\nvalore = ",
        "0.0",
    );
    let gettito = |f: FileDati| {
        let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
        for _ in 0..10 {
            ferma_tutti(&mut p);
            p.chiudi_turno();
        }
        (p.misure().riscossione, p.economia().conti.entrate.iva)
    };
    let (ra, ia) = gettito(senza_shock());
    let (rb, ib) = gettito(ferma);
    assert_eq!(rb, 1.0);
    assert!(ra < 1.0 && ia < ib);
}

#[test]
fn senza_ministro_dell_economia_lo_spread_sale() {
    let d = DatiGioco::da_testi(&senza_shock()).unwrap();
    let (mut a, mut b) = (Partita::nuova(d.clone(), 2), Partita::nuova(d, 2));
    a.comanda(Comando::ChiediDimissioni {
        dicastero: "economia_finanze".into(),
    })
    .unwrap();
    for _ in 0..2 {
        ferma_tutti(&mut a);
        ferma_tutti(&mut b);
        a.chiudi_turno();
        b.chiudi_turno();
    }
    assert_eq!(a.misure().capacita_economia, 1.0);
    assert!(a.economia().spread >= b.economia().spread);
}
