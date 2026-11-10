mod comune;
use comune::{chiudi_col_bilancio, dati_base, edita};
use sim_core::informazione::Acquisto;
use sim_core::legislatura::{Bersaglio, Leva, Spesa};
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
    let (mut privato, mut pubblico, mut scarto) = (0.0, 0.0, 0.0);
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
        scarto += priv_.istituti[0].quote[0]
            - ist.iter().map(|r| r.quote[0]).sum::<f64>() / ist.len() as f64;
    }
    assert!(privato < pubblico, "{privato} {pubblico}");
    // Il bias comune è lo stesso: la differenza media fra i due non ha segno.
    assert!((scarto / 200.0).abs() < 0.005, "{scarto}");
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
    assert!(errore(&mut p).ends_with("cassa insufficiente: servono 15000 €"));
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
