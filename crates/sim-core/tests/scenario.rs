mod comune;
use comune::{con_ingresso, dati_base, file_base, sostituisci};
use sim_core::{DatiGioco, Errore, FileDati, Partita};

fn errore(f: FileDati) -> Errore {
    DatiGioco::da_testi(&f).unwrap_err()
}

#[test]
fn dati_base_hanno_dieci_settori_e_venti_regioni() {
    let d = dati_base();
    assert_eq!(d.struttura().settori.len(), 10);
    assert_eq!(d.regioni().len(), 20);
}

#[test]
fn turno_zero_riproduce_i_valori_di_partenza() {
    let d = dati_base();
    let p = Partita::nuova(d.clone(), 1);
    let e = p.economia();
    assert_eq!(e.inflazione[0], 4.2);
    assert_eq!(e.tasso_bce, 2.5);
    let pesata: f64 = e
        .regioni
        .iter()
        .zip(d.regioni())
        .map(|(r, g)| r.disoccupazione * f64::from(g.forze_lavoro))
        .sum();
    let tot: f64 = d.regioni().iter().map(|g| f64::from(g.forze_lavoro)).sum();
    assert!((pesata / tot - 5.6).abs() < 1e-9);
}

#[test]
fn colonna_non_produttiva_rifiutata_col_nome() {
    let testo = file_base().economia;
    let riga = testo
        .lines()
        .find(|l| l.starts_with("importazioni = ["))
        .unwrap()
        .to_string();
    let primo = riga["importazioni = [".len()..]
        .split(',')
        .next()
        .unwrap()
        .trim()
        .to_string();
    let nuova = riga.replacen(&primo, "0.99", 1);
    let e = errore(sostituisci(file_base(), "economia", &riga, &nuova));
    assert_eq!(e, Errore::SettoreNonProduttivo { id: "A".into() });
}

#[test]
fn settore_sconosciuto_nel_dicastero_rifiutato_col_nome() {
    let e = errore(sostituisci(file_base(), "dicasteri", "BTE", "BTX"));
    assert_eq!(
        e.to_string(),
        "settore sconosciuto: BTX (in infrastrutture_trasporti)"
    );
}

#[test]
fn mix_regionale_che_non_somma_a_uno_rifiutato() {
    let e = errore(sostituisci(file_base(), "regioni", "A = 0.", "A = 9."));
    assert!(matches!(e, Errore::QuoteNonUnitarie { .. }), "{e}");
}

#[test]
fn ingresso_dello_scenario_fuori_intervallo_rifiutato_col_nome() {
    let e = errore(con_ingresso(file_base(), "debito_pil", 250.0));
    assert!(
        e.to_string()
            .starts_with("valore dello scenario fuori intervallo: debito_pil = 250"),
        "{e}"
    );
}

#[test]
fn manopola_mancante_rifiutata_col_nome() {
    let e = errore(sostituisci(
        file_base(),
        "manopole",
        "[okun_nazionale]",
        "[okun_nazionalx]",
    ));
    assert_eq!(
        e,
        Errore::ManopolaMancante {
            nome: "okun_nazionale".into()
        }
    );
}

#[test]
fn leggi_chiede_i_cinque_file_per_nome() {
    let mut chiesti = Vec::new();
    let _ = FileDati::leggi(|nome| {
        chiesti.push(nome.to_string());
        Ok::<_, ()>(String::new())
    });
    assert_eq!(chiesti, FileDati::NOMI);
}
