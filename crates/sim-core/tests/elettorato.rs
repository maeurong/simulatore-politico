#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

mod comune;
use comune::{dati_base, file_base, imposta, riscrivi, senza_shock, sostituisci};
use sim_core::elettorato::{Condizione, Fascia, indice_gruppo};
use sim_core::{DatiGioco, Errore, FileDati, Partita};

/// Messaggio dell'errore, dopo aver verificato che la variante e' quella attesa.
fn rifiuto(f: FileDati, variante: fn(&Errore) -> bool) -> String {
    let e = DatiGioco::da_testi(&f).unwrap_err();
    assert!(variante(&e), "variante sbagliata: {e}");
    e.to_string()
}

fn elettorato(f: FileDati) -> String {
    rifiuto(f, |e| matches!(e, Errore::Elettorato(_)))
}

fn partiti(f: FileDati) -> String {
    rifiuto(f, |e| matches!(e, Errore::Partiti(_)))
}

#[test]
fn dati_base_hanno_360_gruppi_in_ordine() {
    let d = dati_base();
    let e = d.elettorato();
    assert_eq!(e.gruppi.len(), 360);
    for (r, reg) in d.regioni().iter().enumerate() {
        let somma: f64 = (0..18).map(|k| e.gruppi[r * 18 + k].quota).sum();
        assert!((somma - 1.0).abs() < 1e-6, "{}", reg.id);
        assert_eq!(e.gruppi[r * 18].regione, reg.id);
    }
    assert_eq!(
        e.gruppi[indice_gruppo(0, Condizione::Pensionato, Fascia::Alta)].fascia,
        Fascia::Alta
    );
}

#[test]
fn gruppo_di_regione_sconosciuta_rifiutato_col_nome() {
    let m = elettorato(imposta(
        file_base(),
        "elettorato",
        "regione = \"",
        "molisse\"",
    ));
    assert!(m.contains("molisse"), "{m}");
}

#[test]
fn istogramma_che_non_somma_a_uno_rifiutato_col_gruppo() {
    let m = elettorato(imposta(file_base(), "elettorato", "posizione = [", "0.9"));
    assert!(
        m.contains("dipendente_privato") && m.contains("bassa"),
        "{m}"
    );
}

#[test]
fn interessi_con_indicatore_sconosciuto_rifiutati_col_nome() {
    let f = sostituisci(
        file_base(),
        "elettorato",
        "qualita_trasporti = 0.10",
        "trasporto = 0.10",
    );
    let m = elettorato(f);
    assert!(m.contains("trasporto"), "{m}");
}

#[test]
fn pesi_degli_interessi_che_non_sommano_a_uno_rifiutati() {
    let f = sostituisci(
        file_base(),
        "elettorato",
        "disoccupazione = 0.25",
        "disoccupazione = 0.35",
    );
    let m = elettorato(f);
    assert!(m.contains("dipendente_privato"), "{m}");
}

#[test]
fn partito_sconosciuto_nel_governo_rifiutato_col_nome() {
    let f = sostituisci(
        file_base(),
        "scenario",
        "presidente = \"destra\"",
        "presidente = \"destrra\"",
    );
    let m = partiti(f);
    assert!(m.contains("destrra"), "{m}");
}

#[test]
fn dicastero_senza_partito_rifiutato_col_nome() {
    let f = sostituisci(file_base(), "scenario", "sanita = \"destra\", ", "");
    let m = partiti(f);
    assert!(m.contains("sanita"), "{m}");
}

#[test]
fn partito_in_due_coalizioni_rifiutato_col_nome() {
    let f = sostituisci(
        file_base(),
        "scenario",
        "[\"sinistra\", \"centrosinistra\"]",
        "[\"sinistra\", \"centrosinistra\", \"destra\"]",
    );
    let m = partiti(f);
    assert!(m.contains("destra"), "{m}");
}

#[test]
fn valence_fuori_intervallo_rifiutata_col_nome() {
    let f = imposta(
        file_base(),
        "scenario",
        "id = \"centro\"\nnome = \"Centro\"\nposizione = 4.0\n\n[partito.valence]\nvalore = ",
        "9.0",
    );
    let e = DatiGioco::da_testi(&f).unwrap_err();
    assert!(
        matches!(&e, Errore::ScenarioFuoriIntervallo { nome, .. } if nome == "valence di centro"),
        "{e}"
    );
}

#[test]
fn partita_nuova_ha_il_governo_uscente() {
    let p = Partita::nuova(dati_base(), 1);
    let pres = &p.partiti()[p.governo().presidente];
    assert_eq!(pres.id, "destra");
    assert_eq!(p.partiti().len(), 7);
    assert!(
        p.elettorato()
            .gruppi
            .iter()
            .all(|g| g.giudizio.iter().all(|&x| x == 0.0))
    );
}

#[test]
fn redditi_al_turno_zero_dai_dati() {
    let d = dati_base();
    let p = Partita::nuova(d.clone(), 1);
    let i = indice_gruppo(3, Condizione::Autonomo, Fascia::Alta);
    let g = &d.elettorato().gruppi[i];
    let lordo = g.primario + g.trasferimenti;
    let atteso = lordo - d.elettorato().aliquote[&Fascia::Alta] * lordo;
    assert!((p.elettorato().gruppi[i].reddito.reale() - atteso).abs() < 1e-6);
}

#[test]
fn bilancio_fermo_erode_il_reddito_dei_dipendenti_pubblici() {
    // 4 turni senza shock: salari pubblici fermi in nominale fino al bilancio di gennaio,
    // salari privati che seguono il PIL nominale.
    let mut p = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
    let pubblico = indice_gruppo(11, Condizione::DipendentePubblico, Fascia::Media);
    let privato = indice_gruppo(11, Condizione::DipendentePrivato, Fascia::Media);
    let reale = |p: &Partita, i: usize| p.elettorato().gruppi[i].reddito.reale();
    let (pu0, pr0) = (reale(&p, pubblico), reale(&p, privato));
    for _ in 0..4 {
        p.chiudi_turno();
    }
    assert!(reale(&p, pubblico) / pu0 < reale(&p, privato) / pr0);
}

#[test]
fn gini_nullo_con_redditi_uguali() {
    let f = riscrivi(
        riscrivi(
            riscrivi(
                file_base(),
                "elettorato",
                "primario = ",
                "primario = 1000.0",
            ),
            "elettorato",
            "trasferimenti = ",
            "trasferimenti = 0.0",
        ),
        "elettorato",
        "aliquote = ",
        "aliquote = { bassa = 0.2, media = 0.2, alta = 0.2 }",
    );
    let d = DatiGioco::da_testi(&f).unwrap();
    assert!(Partita::nuova(d.clone(), 1).elettorato().gini(&d).abs() < 1e-12);
    let base = dati_base();
    let g = Partita::nuova(base.clone(), 1).elettorato().gini(&base);
    assert!(g > 0.1 && g < 0.6, "{g}");
}

#[test]
fn pressione_estrema_e_base_nulla_lasciano_redditi_finiti_e_non_negativi() {
    let d = dati_base();
    let p = Partita::nuova(d.clone(), 1);
    let mut el = p.elettorato().clone();
    el.base.pressione *= 1e-6;
    el.base.salari_pubblici = 0.0;
    el.base.pensione = 0.0;
    el.aggiorna_redditi(&d, p.economia());
    for g in &el.gruppi {
        assert!(g.reddito.reale().is_finite() && g.reddito.netto() >= 0.0);
    }
}

#[test]
fn quote_di_regione_sommano_a_uno_dopo_40_turni() {
    let mut p = Partita::nuova(dati_base(), 7);
    for _ in 0..40 {
        p.chiudi_turno();
    }
    for r in 0..20 {
        let s: f64 = (0..18)
            .map(|k| p.elettorato().gruppi[r * 18 + k].quota)
            .sum();
        assert!((s - 1.0).abs() < 1e-9, "regione {r}: {s}");
    }
}

fn quota_disoccupati(p: &Partita, r: usize) -> f64 {
    Fascia::TUTTE
        .iter()
        .map(|&f| p.elettorato().gruppi[indice_gruppo(r, Condizione::DisoccupatoInattivo, f)].quota)
        .sum()
}

#[test]
fn la_disoccupazione_sposta_dipendenti_fra_i_disoccupati() {
    let mut p = Partita::nuova(dati_base(), 5);
    let u0: Vec<f64> = p
        .economia()
        .regioni
        .iter()
        .map(|r| r.disoccupazione)
        .collect();
    let q0: Vec<f64> = (0..20).map(|r| quota_disoccupati(&p, r)).collect();
    p.chiudi_turno();
    let mut mossi = 0;
    for r in 0..20 {
        let du = p.economia().regioni[r].disoccupazione - u0[r];
        if du.abs() > 1e-9 {
            mossi += 1;
            let dq = quota_disoccupati(&p, r) - q0[r];
            assert!(dq * du > 0.0, "regione {r}: du {du}, dq {dq}");
        }
    }
    assert!(mossi > 0, "nessuna regione ha cambiato disoccupazione");
}

#[test]
fn i_pensionati_cambiano_solo_a_gennaio() {
    let d = dati_base();
    let mut p = Partita::nuova(dati_base(), 3);
    let pensionati = |p: &Partita| -> f64 {
        (0..360)
            .filter(|i| i % 18 / 3 == Condizione::Pensionato as usize)
            .map(|i| {
                p.elettorato().gruppi[i].quota * d.elettorato().regioni[i / 18].elettori as f64
            })
            .sum()
    };
    let t0 = pensionati(&p);
    p.chiudi_turno();
    assert!((pensionati(&p) - t0).abs() < 1e-6 * t0);
    p.chiudi_turno();
    assert!((pensionati(&p) - t0).abs() > 1e-6 * t0);
}
