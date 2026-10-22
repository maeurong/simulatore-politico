#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

mod comune;
use comune::{dati_base, file_base, imposta, sostituisci};
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
