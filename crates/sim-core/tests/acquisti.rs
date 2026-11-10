mod comune;
use comune::{all_opposizione, chiudi_col_bilancio, dati_base, edita};
use sim_core::elettorato::Condizione;
use sim_core::informazione::Acquisto;
use sim_core::legislatura::{Bersaglio, Leva, Spesa};
use sim_core::parlamento::ValoreProposto;
use sim_core::{Comando, Partita};

const PREZZO: i64 = 15_000;
const SONDAGGIO: Comando = Comando::Compra {
    acquisto: Acquisto::SondaggioPrivato,
};

fn errore(p: &mut Partita) -> String {
    p.comanda(SONDAGGIO).unwrap_err().to_string()
}

fn con_cassa(p: &Partita, euro: i64) -> Partita {
    let g = p.giocatore();
    edita(p, |j| j["partiti"][g]["cassa"] = euro.into()).unwrap()
}

#[test]
fn il_sondaggio_privato_costa_e_sbaglia_meno() {
    let (mut privato, mut pubblico, mut scarto) = (0.0, 0.0, vec![0.0; 8]);
    for seme in 0..200 {
        let mut p = Partita::nuova(dati_base(), seme);
        let g = p.giocatore();
        let cassa = p.partiti()[g].cassa;
        let vero = p.intenzioni_vere().nazionali;
        p.comanda(SONDAGGIO).unwrap();
        assert_eq!(p.partiti()[g].cassa, cassa - PREZZO);
        let v = p.vista();
        let errore = |q: &[f64]| {
            (q.iter().zip(&vero))
                .map(|(a, b)| (a - b).abs())
                .sum::<f64>()
        };
        let ist = &v.sondaggio.istituti;
        let priv_ = v.acquisti.sondaggio_privato.unwrap();
        assert_eq!(priv_.istituti.len(), 1);
        privato += errore(&priv_.istituti[0].quote);
        pubblico += ist.iter().map(|r| errore(&r.quote)).sum::<f64>() / ist.len() as f64;
        for (j, s) in scarto.iter_mut().enumerate() {
            *s += priv_.istituti[0].quote[j]
                - ist.iter().map(|r| r.quote[j]).sum::<f64>() / ist.len() as f64;
        }
    }
    assert!(privato < pubblico, "{privato} {pubblico}");
    // Il bias comune è lo stesso: la differenza media fra i due non ha segno, su ogni alternativa.
    assert!(
        scarto.iter().all(|s| (s / 200.0).abs() < 0.005),
        "{scarto:?}"
    );
}

#[test]
fn acquisto_rifiutato_non_cambia_nulla() {
    let mut p = Partita::nuova(dati_base(), 1);
    p.comanda(SONDAGGIO).unwrap();
    let prima = p.salva().unwrap();
    assert_eq!(
        errore(&mut p),
        "comando non ammesso: già comprato questo turno: sondaggio privato"
    );
    assert_eq!(p.salva().unwrap(), prima);

    let mut p = con_cassa(&Partita::nuova(dati_base(), 1), PREZZO - 1);
    let prima = p.salva().unwrap();
    assert!(errore(&mut p).ends_with("cassa insufficiente: servono 15.000 euro"));
    assert_eq!(p.salva().unwrap(), prima);

    let mut p = Partita::nuova(dati_base(), 1);
    p.abbandona().unwrap();
    let prima = p.salva().unwrap();
    assert!(errore(&mut p).ends_with("La partita è finita"));
    assert_eq!(p.salva().unwrap(), prima);
}

/// Al turno del voto, con la campagna pari alla cassa: l'acquisto la riduce di 15.000.
#[test]
fn l_acquisto_toglie_alla_campagna() {
    let spesa = |cassa: i64| {
        let mut p = Partita::nuova(dati_base(), 1);
        while p.vista().turno < 19 {
            chiudi_col_bilancio(&mut p);
        }
        let mut p = con_cassa(&p, cassa);
        let spese = vec![Spesa {
            bersaglio: Bersaglio::Regione(0),
            leva: Leva::Persuadere,
            euro: cassa,
        }];
        p.comanda(Comando::Campagna { spese }).unwrap();
        p.comanda(SONDAGGIO).unwrap();
        chiudi_col_bilancio(&mut p);
        let id = p.vista().giocatore;
        (p.vista().mosse.into_iter())
            .find(|m| m.partito == id && m.testo.starts_with("Spende "))
            .map(|m| m.testo)
    };
    let riga = spesa(3_000_000).unwrap();
    assert!(riga.starts_with("Spende 2.985.000 euro"), "{riga}");
    assert_eq!(spesa(PREZZO), None);
}

#[test]
fn gli_acquisti_finiscono_col_turno() {
    let mut p = Partita::nuova(dati_base(), 1);
    p.comanda(SONDAGGIO).unwrap();
    assert!(p.vista().acquisti.sondaggio_privato.is_some());
    chiudi_col_bilancio(&mut p);
    assert!(p.vista().acquisti.sondaggio_privato.is_none());
    p.comanda(SONDAGGIO).unwrap();
}

#[test]
fn comprare_non_sposta_le_estrazioni() {
    let chiusa = |compra: bool| {
        let mut p = Partita::nuova(dati_base(), 1);
        if compra {
            p.comanda(SONDAGGIO).unwrap();
        }
        chiudi_col_bilancio(&mut p);
        let v = p.vista();
        (v.sondaggio, v.statistica.prima_stima)
    };
    assert_eq!(chiusa(false), chiusa(true));
}

#[test]
fn due_acquisti_dello_stesso_tipo_nel_salvataggio_sono_rifiutati() {
    let mut p = Partita::nuova(dati_base(), 1);
    p.comanda(SONDAGGIO).unwrap();
    let e = edita(&p, |j| {
        let primo = j["acquisti"][0].clone();
        j["acquisti"].as_array_mut().unwrap().push(primo);
    })
    .unwrap_err();
    assert!(
        e.to_string()
            .contains("acquisti: due acquisti dello stesso tipo nel turno"),
        "{e}"
    );
}

/// Gli incroci del privato pescano dal campione di un solo istituto (4 volte il suo), non da
/// quello di tutti.
#[test]
fn gli_incroci_del_privato_hanno_il_campione_di_un_istituto() {
    let mut p = Partita::nuova(dati_base(), 1);
    p.comanda(SONDAGGIO).unwrap();
    let v = p.vista();
    let privato = v.acquisti.sondaggio_privato.unwrap();
    let campione = privato.istituti[0].campione;
    assert_eq!(campione, 4 * v.sondaggio.istituti[0].campione);
    let regioni: u32 = privato.regioni.iter().map(|r| r.campione).sum();
    assert!(
        regioni.abs_diff(campione) as usize <= privato.regioni.len(),
        "{regioni} {campione}"
    );
}

#[test]
fn un_sondaggio_privato_malformato_nel_salvataggio_e_rifiutato() {
    let mut p = Partita::nuova(dati_base(), 1);
    p.comanda(SONDAGGIO).unwrap();
    let e = edita(&p, |j| {
        let quote = &mut j["acquisti"][0]["SondaggioPrivato"]["istituti"][0]["quote"];
        quote.as_array_mut().unwrap().pop();
    })
    .unwrap_err();
    assert!(e.to_string().contains("acquisti.privato.istituti"), "{e}");
}

const PREZZO_TEST: i64 = 20_000;

fn v(d: &str, g: &str) -> Vec<ValoreProposto> {
    vec![ValoreProposto {
        disposizione: d.into(),
        gradino: g.into(),
    }]
}

fn test_di(valori: &[ValoreProposto]) -> Comando {
    Comando::Compra {
        acquisto: Acquisto::TestMisura {
            valori: valori.to_vec(),
        },
    }
}

fn segno(x: f64) -> i8 {
    if x > 0.0 {
        1
    } else if x < 0.0 {
        -1
    } else {
        0
    }
}

#[test]
fn il_test_indovina_il_segno_dei_gruppi_grandi() {
    let valori = v("regime_pensioni", "retributivo");
    let (mut giusti, mut quote) = ([0; 6], [0.0; 6]);
    for seme in 0..200 {
        let mut p = Partita::nuova(dati_base(), seme);
        let vere = p.reazioni_vere(&valori).unwrap();
        assert_ne!(vere[Condizione::Pensionato as usize].0, 0.0);
        p.comanda(test_di(&valori)).unwrap();
        let t = p.vista().acquisti.test_misura.unwrap();
        assert_eq!(t.reazioni.len(), 6);
        for (k, (r, (vera, quota))) in t.reazioni.iter().zip(vere).enumerate() {
            giusti[k] += u32::from(r.segno == segno(vera));
            quote[k] = quota;
        }
    }
    let pensionato = Condizione::Pensionato as usize;
    let piccola = (0..6)
        .min_by(|&a, &b| quote[a].total_cmp(&quote[b]))
        .unwrap();
    assert!(giusti[pensionato] >= 180, "{giusti:?}");
    assert!(giusti[piccola] < giusti[pensionato], "{giusti:?} {quote:?}");
}

#[test]
fn test_su_misura_non_ammessa_rifiutato_senza_pagare() {
    let mut p = Partita::nuova(dati_base(), 1);
    let g = p.giocatore();
    let (cassa, prima) = (p.partiti()[g].cassa, p.salva().unwrap());
    let e = p
        .comanda(test_di(&v("eta_pensionabile", "67")))
        .unwrap_err();
    assert!(
        e.to_string().contains("Età pensionabile è già a 67 anni"),
        "{e}"
    );
    assert_eq!(p.partiti()[g].cassa, cassa);
    assert_eq!(p.salva().unwrap(), prima);
}

#[test]
fn il_test_lo_compra_anche_l_opposizione() {
    let mut p = all_opposizione();
    let g = p.giocatore();
    let cassa = p.partiti()[g].cassa;
    p.comanda(test_di(&v("regime_pensioni", "retributivo")))
        .unwrap();
    assert_eq!(p.partiti()[g].cassa, cassa - PREZZO_TEST);
    let t = p.vista().acquisti.test_misura.unwrap();
    assert_eq!(
        t.reazioni.iter().map(|r| r.condizione).collect::<Vec<_>>(),
        Condizione::TUTTE
    );
}

#[test]
fn il_test_non_tocca_la_partita() {
    let json = |p: &Partita| -> serde_json::Value {
        let mut j: serde_json::Value = serde_json::from_str(&p.salva().unwrap()).unwrap();
        j["acquisti"] = serde_json::Value::Null;
        j["partiti"][p.giocatore()]["cassa"] = 0.into();
        j
    };
    let chiusa = |compra: bool| {
        let mut p = Partita::nuova(dati_base(), 1);
        if compra {
            let prima = json(&p);
            p.comanda(test_di(&v("regime_pensioni", "retributivo")))
                .unwrap();
            assert_eq!(json(&p), prima);
        }
        chiudi_col_bilancio(&mut p);
        let v = p.vista();
        (v.sondaggio, v.statistica.prima_stima)
    };
    assert_eq!(chiusa(false), chiusa(true));
}

#[test]
fn un_test_manomesso_nel_salvataggio_e_rifiutato() {
    let mut p = Partita::nuova(dati_base(), 1);
    p.comanda(test_di(&v("regime_pensioni", "retributivo")))
        .unwrap();
    let e = edita(&p, |j| {
        j["acquisti"][0]["TestMisura"]["reazioni"][0]["segno"] = 2.into();
    })
    .unwrap_err();
    assert!(e.to_string().contains("acquisti.test.segno"), "{e}");
    let e = edita(&p, |j| {
        let r = &mut j["acquisti"][0]["TestMisura"]["reazioni"];
        r.as_array_mut().unwrap().swap(0, 1);
    })
    .unwrap_err();
    assert!(e.to_string().contains("acquisti.test.reazioni"), "{e}");
}
