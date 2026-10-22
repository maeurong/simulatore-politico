mod comune;
use comune::{con_ingresso, dati_base, file_base, imposta, riscrivi, sostituisci};
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
    let gap = 2.0;
    let d = DatiGioco::da_testi(&con_ingresso(file_base(), "output_gap", gap)).unwrap();
    let s = d.scenario();
    let p = Partita::nuova(d.clone(), 1);
    let e = p.economia();
    assert_eq!(e.inflazione[0], 4.2);
    assert_eq!(e.tasso_bce, 2.5);
    assert_eq!(
        e.pil_potenziale_reale,
        s.pil_trimestrale.valore / (1.0 + gap / 100.0)
    );
    assert_eq!(e.output_gap, [gap, gap]);
    assert_eq!(e.livello_prezzi, 1.0);
    assert_eq!(e.inflazione[1], s.inflazione_t3.valore);
    assert_eq!(e.inflazione[2], s.inflazione_t2.valore);
    assert!(
        e.inflazione[3..].iter().all(|&x| x == 2.0),
        "{:?}",
        e.inflazione
    );
    assert!(e.prezzi_settori.iter().all(|p| (p - 1.0).abs() < 1e-9));
    assert_eq!(e.area_euro.inflazione_resto, e.inflazione[0]);
    let pesata: f64 = e
        .regioni
        .iter()
        .zip(d.regioni())
        .map(|(r, g)| r.disoccupazione * g.forze_lavoro)
        .sum();
    let tot: f64 = d.regioni().iter().map(|g| g.forze_lavoro).sum();
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
fn leggi_chiede_i_file_per_nome() {
    let mut chiesti = Vec::new();
    let _ = FileDati::leggi(|nome| {
        chiesti.push(nome.to_string());
        Ok::<_, ()>(String::new())
    });
    assert_eq!(chiesti, FileDati::NOMI);
}

#[test]
fn forze_di_lavoro_non_positive_rifiutate() {
    let e = errore(sostituisci(
        file_base(),
        "regioni",
        "forze_lavoro = 1975.6",
        "forze_lavoro = 0.0",
    ));
    assert!(matches!(e, Errore::StrutturaIncoerente(_)), "{e}");
}

#[test]
fn okun_non_finito_rifiutato() {
    let e = errore(sostituisci(
        file_base(),
        "regioni",
        "okun = -0.239",
        "okun = nan",
    ));
    assert!(matches!(e, Errore::StrutturaIncoerente(_)), "{e}");
}

/// Il file modificato è rifiutato con `StrutturaIncoerente` che nomina `campo`.
fn incoerente_su(f: FileDati, campo: &str) {
    match errore(f) {
        Errore::StrutturaIncoerente(m) => assert!(m.contains(campo), "{m:?} senza {campo:?}"),
        e => panic!("atteso StrutturaIncoerente su {campo}, trovato {e}"),
    }
}

#[test]
fn numeri_della_struttura_non_finiti_o_negativi_rifiutati() {
    let b = file_base;
    incoerente_su(
        imposta(b(), "economia", "valore_aggiunto = [", "nan"),
        "valore_aggiunto del settore A",
    );
    incoerente_su(
        imposta(b(), "economia", "importazioni = [", "inf"),
        "importazioni del settore A",
    );
    incoerente_su(imposta(b(), "economia", "a = [\n  [", "-0.01"), "a[A][A]");
    incoerente_su(
        imposta(b(), "economia", "consumi = [", "nan"),
        "consumi del settore A",
    );
}

#[test]
fn numeri_delle_regioni_non_finiti_o_fuori_intervallo_rifiutati() {
    let b = file_base;
    incoerente_su(
        imposta(b(), "regioni", "disoccupazione_2025 = ", "nan"),
        "disoccupazione_2025 di piemonte",
    );
    incoerente_su(
        imposta(b(), "regioni", "disoccupazione_2025 = ", "101.0"),
        "disoccupazione_2025 di piemonte",
    );
    incoerente_su(
        imposta(b(), "regioni", "mix = { A = ", "nan"),
        "mix di piemonte per il settore A",
    );
}

#[test]
fn okun_positivo_o_nullo_rifiutato() {
    for v in ["0.1", "0.0"] {
        incoerente_su(
            imposta(file_base(), "regioni", "okun = ", v),
            "okun di piemonte",
        );
    }
}

#[test]
fn numeri_dei_dicasteri_non_finiti_o_negativi_rifiutati() {
    let b = file_base;
    incoerente_su(
        imposta(b(), "dicasteri", "spesa_pil = ", "nan"),
        "spesa_pil del dicastero economia_finanze",
    );
    incoerente_su(
        imposta(b(), "dicasteri", "salari_pil = ", "inf"),
        "salari_pil del dicastero economia_finanze",
    );
    incoerente_su(
        imposta(b(), "dicasteri", "acquisti_pil = ", "-1.0"),
        "acquisti_pil del dicastero economia_finanze",
    );
    incoerente_su(
        imposta(b(), "dicasteri", "settori = { A = ", "nan"),
        "quota del dicastero",
    );
}

#[test]
fn settore_senza_domanda_ne_legami_rifiutato() {
    let uno = "[1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]";
    let mut f = riscrivi(
        file_base(),
        "economia",
        "  [",
        "  [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],",
    );
    for campo in ["consumi", "investimenti", "esportazioni", "spesa_pubblica"] {
        f = riscrivi(
            f,
            "economia",
            &format!("{campo} = ["),
            &format!("{campo} = {uno}"),
        );
    }
    let f = riscrivi(f, "dicasteri", "settori = ", "settori = { A = 1.0 }");
    incoerente_su(
        f,
        "il settore BTE non ha produzione di riferimento positiva",
    );
}

#[test]
fn tetti_di_spesa_netta_vuoti_rifiutati() {
    let e = errore(sostituisci(
        file_base(),
        "scenario",
        "tetti_spesa_netta = { 2026 = 1.6, 2027 = 1.9, 2028 = 1.7, 2029 = 1.5 }",
        "tetti_spesa_netta = {}",
    ));
    assert!(matches!(e, Errore::StrutturaIncoerente(_)), "{e}");
}
