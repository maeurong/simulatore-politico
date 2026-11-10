mod comune;
use comune::{
    all_opposizione, chiudi_col_bilancio, dati_base, edita, ferma_tutti, senza_emergenze,
    senza_shock,
};
use sim_core::dati::DatiGioco;
use sim_core::economia::Economia;
use sim_core::elettorato::{Condizione, condizione_di};
use sim_core::informazione::{Acquisto, Stima, VistaCentroStudi};
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

/// L'oracolo della reazione vera: la partita vera senza shock né derive, con la bozza messa in
/// vigore prima della chiusura (come `entra_in_vigore` nell'aula dello stesso turno), chiusa
/// `orizzonte_test` volte contro una gemella senza bozza; Δ della somma dei giudizi sui partiti
/// di governo, per condizione, pesata con gli elettori del turno zero.
fn oracolo(d: &str, g: &str) -> ([f64; 6], [(f64, f64); 6]) {
    let dati = DatiGioco::da_testi(&senza_shock()).unwrap();
    let (k, gamma) = (
        dati.disposizione(d).unwrap(),
        dati.manopola("peso_giudizio"),
    );
    let gradino = dati.disposizioni()[k].gradino(g).unwrap();
    let turni = dati.manopola("orizzonte_test") as usize;
    let p = Partita::nuova(dati.clone(), 1);
    let vere = p.reazioni_vere(&v(d, g)).unwrap();
    let chiusa = |con: bool| {
        let mut q = edita(&p, |j| {
            if con {
                j["disposizioni"]["voci"][k]["in_vigore"] = gradino.into();
                j["disposizioni"]["voci"][k]["in_arrivo"] = serde_json::json!([]);
            }
        })
        .unwrap();
        for _ in 0..turni {
            ferma_tutti(&mut q);
            chiudi_col_bilancio(&mut q);
        }
        assert_eq!(q.governo(), p.governo());
        q
    };
    let (con, senza) = (chiusa(true), chiusa(false));
    let governo: Vec<usize> = (0..p.partiti().len())
        .filter(|&j| p.governo().al_governo(j))
        .collect();
    let mut somme = [(0.0, 0.0); 6];
    let coppie = (con.elettorato().gruppi.iter()).zip(&senza.elettorato().gruppi);
    for (i, (a, b)) in coppie.enumerate() {
        let w = p.elettorato().elettori(dati.elettorato(), i);
        let s = &mut somme[condizione_di(i) as usize];
        s.0 += w;
        s.1 += w * governo
            .iter()
            .map(|&j| a.giudizio[j] - b.giudizio[j])
            .sum::<f64>();
    }
    (somme.map(|(w, x)| gamma * x / w), vere)
}

#[test]
fn la_reazione_vera_coincide_con_la_partita_chiusa() {
    for (d, g) in [
        ("regime_pensioni", "retributivo"),
        ("indicizzazione_pensioni", "piena"),
        ("indicizzazione_pensioni", "bloccata_sopra_soglia"),
    ] {
        let (attesa, vere) = oracolo(d, g);
        for (a, (vera, _)) in attesa.iter().zip(vere) {
            assert!((a - vera).abs() <= 1e-5, "{d} {g}: {attesa:?} {vere:?}");
        }
    }
}

fn rifiuto(p: &mut Partita, valori: &[ValoreProposto]) -> String {
    p.comanda(test_di(valori)).unwrap_err().to_string()
}

#[test]
fn il_test_rifiuta_doppio_e_senza_cassa_prima_della_bozza() {
    let valori = v("regime_pensioni", "retributivo");
    let mut p = Partita::nuova(dati_base(), 1);
    p.comanda(test_di(&valori)).unwrap();
    let prima = p.salva().unwrap();
    assert_eq!(
        rifiuto(&mut p, &valori),
        "comando non ammesso: già comprato questo turno: test di una misura"
    );
    assert_eq!(p.salva().unwrap(), prima);

    let mut p = con_cassa(&Partita::nuova(dati_base(), 1), PREZZO_TEST - 1);
    let prima = p.salva().unwrap();
    for valori in [valori, v("eta_pensionabile", "67")] {
        let e = rifiuto(&mut p, &valori);
        assert!(
            e.ends_with("cassa insufficiente: servono 20.000 euro"),
            "{e}"
        );
    }
    assert_eq!(p.salva().unwrap(), prima);
}

#[test]
fn il_test_rifiuta_cio_che_il_mes_vincola() {
    let dati = dati_base();
    let c = &dati.condizionalita()[0];
    let k = dati.disposizione(&c.disposizione).unwrap();
    let x = &dati.disposizioni()[k];
    let vincolato = x.gradino(&c.gradino).unwrap();
    let altro = &x.gradini[(vincolato + 1) % x.gradini.len()].id;
    // Tutta la condizionalità ai gradini del memorandum, come la mette la risposta all'evento.
    let mut p = edita(&Partita::nuova(dati_base(), 1), |j| {
        j["economia"]["programma_mes"] = 8.into();
        for c in dati.condizionalita() {
            let k = dati.disposizione(&c.disposizione).unwrap();
            let g = dati.disposizioni()[k].gradino(&c.gradino).unwrap();
            j["disposizioni"]["voci"][k]["in_vigore"] = g.into();
            j["disposizioni"]["voci"][k]["in_arrivo"] = serde_json::json!([]);
        }
    })
    .unwrap();
    let prima = p.salva().unwrap();
    let e = rifiuto(&mut p, &v(&x.id, altro));
    let atteso = format!(
        "{}: vincolo del memorandum del MES, fino al turno 8",
        x.nome
    );
    assert!(e.ends_with(&atteso), "{e}");
    assert_eq!(p.salva().unwrap(), prima);
}

/// Ruling 64: il test non entra in calendario, quindi la crisi non lo ferma (l'anteprima sì).
#[test]
fn il_test_si_compra_anche_in_crisi() {
    let valori = v("regime_pensioni", "retributivo");
    let p = Partita::nuova(dati_base(), 1);
    let mut p = edita(&p, |j| j["legislatura"]["crisi"] = 0.into()).unwrap();
    assert!(p.anteprima(&valori).is_err());
    p.comanda(test_di(&valori)).unwrap();
    assert!(p.vista().acquisti.test_misura.is_some());
}

const PREZZO_CENTRO_STUDI: i64 = 150_000;
const CENTRO_STUDI: Comando = Comando::Compra {
    acquisto: Acquisto::CentroStudi,
};

fn centro_studi(p: &mut Partita) -> VistaCentroStudi {
    let g = p.giocatore();
    let cassa = p.partiti()[g].cassa;
    p.comanda(CENTRO_STUDI).unwrap();
    assert_eq!(p.partiti()[g].cassa, cassa - PREZZO_CENTRO_STUDI);
    p.vista().acquisti.centro_studi.unwrap()
}

fn campi(s: &Stima) -> [f64; 4] {
    [s.crescita_pil, s.disoccupazione, s.inflazione, s.disavanzo]
}

#[test]
fn la_stima_del_centro_studi_e_piu_vicina() {
    let mut p = Partita::nuova(dati_base(), 1);
    chiudi_col_bilancio(&mut p);
    let j: serde_json::Value = serde_json::from_str(&p.salva().unwrap()).unwrap();
    let stima = |k: &str| -> Stima {
        serde_json::from_value(j["informazione"]["statistica"][k].clone()).unwrap()
    };
    let (vera, prima) = (campi(&stima("vera")), campi(&stima("prima_stima")));
    let centro = campi(&centro_studi(&mut p).stima);
    for k in 0..4 {
        let (errore, residuo) = (prima[k] - vera[k], centro[k] - vera[k]);
        assert!(errore.abs() > 1e-6, "{k}: prima stima senza errore");
        assert!(
            (residuo - 0.6 * errore).abs() <= 1e-9,
            "{k}: {residuo} contro {errore}"
        );
    }
}

#[test]
fn la_forchetta_si_allarga() {
    let dati = dati_base();
    let mut p = all_opposizione();
    let previsione = centro_studi(&mut p).previsione;
    assert_eq!(previsione.len(), 4);
    let sigme = |nome: &str, f: fn(&sim_core::informazione::VocePrevisione) -> f64| {
        let s: Vec<f64> = previsione.iter().map(f).collect();
        assert!(s.windows(2).all(|w| w[0] < w[1]), "{nome}: {s:?}");
        assert_eq!(s[0], dati.manopola(&format!("rmse_{nome}_1")), "{nome}");
        assert_eq!(s[3], dati.manopola(&format!("rmse_{nome}_4")), "{nome}");
    };
    sigme("pil", |v| v.pil.sigma);
    sigme("disoccupazione", |v| v.disoccupazione.sigma);
    sigme("inflazione", |v| v.inflazione.sigma);
    sigme("debito", |v| v.debito.sigma);
}

#[test]
fn la_previsione_non_tocca_la_partita() {
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
            centro_studi(&mut p);
            assert_eq!(json(&p), prima);
        }
        chiudi_col_bilancio(&mut p);
        json(&p)
    };
    assert_eq!(chiusa(false), chiusa(true));
}

/// I valori veri del trimestre appena chiuso, con le definizioni della previsione ricalcolate qui
/// dalle serie dell'economia: PIL reale e prezzi sul trimestre dello stesso anno prima, debito sul
/// PIL degli ultimi quattro trimestri.
fn veri(dati: &DatiGioco, e: &Economia) -> [f64; 4] {
    let n = e.storico.len();
    let prezzi: f64 = e.inflazione[..4].iter().map(|i| 1.0 + i / 400.0).product();
    let pil = e.storico[n - 1].pil as f64 / e.storico[n - 5].pil as f64 / prezzi;
    let pil_anno: f64 = e.storico[n - 4..].iter().map(|t| t.pil as f64).sum();
    [
        100.0 * (pil - 1.0),
        e.disoccupazione_nazionale(dati),
        100.0 * (prezzi - 1.0),
        100.0 * e.debito as f64 / pil_anno,
    ]
}

/// L'oracolo della previsione: senza shock, derive, emergenze né atti dei ministri, il centro del
/// primo turno è il valore della chiusura vera. Tolleranza 1e-9: stesse operazioni nello stesso
/// ordine sullo stesso stato, la differenza attesa è zero.
#[test]
fn senza_shock_la_previsione_e_esatta() {
    let dati = DatiGioco::da_testi(&senza_emergenze(senza_shock())).unwrap();
    let mut p = Partita::nuova(dati.clone(), 1);
    ferma_tutti(&mut p);
    let voce = centro_studi(&mut p).previsione.remove(0);
    let data = p.vista().data;
    chiudi_col_bilancio(&mut p);
    assert_eq!(voce.data, data);
    let centri = [
        voce.pil.centro,
        voce.disoccupazione.centro,
        voce.inflazione.centro,
        voce.debito.centro,
    ];
    let veri = veri(&dati, p.economia());
    for (k, (c, v)) in centri.iter().zip(veri).enumerate() {
        assert!((c - v).abs() <= 1e-9, "{k}: {centri:?} {veri:?}");
    }
}

#[test]
fn un_centro_studi_manomesso_nel_salvataggio_e_rifiutato() {
    let mut p = Partita::nuova(dati_base(), 1);
    centro_studi(&mut p);
    let rifiuto = |f: &dyn Fn(&mut serde_json::Value)| {
        edita(&p, |j| f(&mut j["acquisti"][0]["CentroStudi"]))
            .unwrap_err()
            .to_string()
    };
    let e = rifiuto(&|c| {
        c["previsione"].as_array_mut().unwrap().pop();
    });
    assert!(
        e.contains("acquisti.centro_studi.previsione: servono 4 voci"),
        "{e}"
    );
    let e = rifiuto(&|c| {
        let prima = c["previsione"][0]["data"].clone();
        c["previsione"][2]["data"] = prima;
    });
    assert!(e.contains("acquisti.centro_studi.date:"), "{e}");
    let e = rifiuto(&|c| c["previsione"][1]["debito"]["sigma"] = (-0.1).into());
    assert!(e.contains("acquisti.centro_studi.sigma: negativa"), "{e}");
}
