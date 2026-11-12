mod comune;
use comune::{
    all_opposizione, chiudi_col_bilancio, dati_base, edita, ferma_tutti, senza_emergenze,
    senza_shock, sostituisci, voci,
};
use sim_core::dati::DatiGioco;
use sim_core::economia::Economia;
use sim_core::elettorato::{Condizione, condizione_di};
use sim_core::esito::TipoVoce;
use sim_core::informazione::{Acquisto, Stima, VistaCentroStudi};
use sim_core::legislatura::{Bersaglio, Leva, Spesa};
use sim_core::parlamento::{FormaMisura, ValoreProposto};
use sim_core::vista::Fascia3;
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
    let (mut privato, mut pubblico) = (0.0, 0.0);
    let (mut scarto_privato, mut scarto_pubblico) = (vec![Vec::new(); 8], vec![Vec::new(); 8]);
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
        for j in 0..8 {
            scarto_privato[j].push(priv_.istituti[0].quote[j] - vero[j]);
            let media = ist.iter().map(|r| r.quote[j]).sum::<f64>() / ist.len() as f64;
            scarto_pubblico[j].push(media - vero[j]);
        }
    }
    assert!(privato < pubblico, "{privato} {pubblico}");
    // Il bias comune è lo stesso: su ogni alternativa l'errore del privato segue quello del
    // pubblico da un seme all'altro.
    for j in 0..8 {
        let r = correlazione(&scarto_privato[j], &scarto_pubblico[j]);
        assert!(r > 0.5, "{j}: r = {r}");
    }
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
    assert!(errore(&mut p).ends_with(
        "cassa insufficiente per il sondaggio privato: servono 15.000 euro, ce ne sono 14.999"
    ));
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
        (voci(&p, TipoVoce::SpesaDiCampagna).into_iter())
            .find(|v| v.partito.as_ref() == Some(&id))
            .map(|v| v.fatto)
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
        let quote = &mut j["acquisti"]["sondaggio_privato"]["istituti"][0]["quote"];
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

/// La condizione più grande ha la sigma pari a `soglia_reazione_poco`: con la reazione vera in
/// Molto, la fascia stimata è Molto in almeno 190 semi su 200 (con la taratura attuale la vera
/// sta a circa cinque sigma dalla soglia del Medio: una deriva di `errore_test` lo fa cadere).
#[test]
fn il_test_dice_la_fascia_della_condizione_piu_grande() {
    let valori = v("ticket", "nessuno");
    let medio = dati_base().manopola("soglia_reazione_medio");
    let mut molto = 0;
    for seme in 0..200 {
        let mut p = Partita::nuova(dati_base(), seme);
        let vere = p.reazioni_vere(&valori).unwrap();
        let grande = (0..6)
            .max_by(|&a, &b| vere[a].1.total_cmp(&vere[b].1))
            .unwrap();
        assert!(vere[grande].0.abs() >= medio, "{seme}: {vere:?}");
        p.comanda(test_di(&valori)).unwrap();
        let t = p.vista().acquisti.test_misura.unwrap();
        molto += u32::from(t.reazioni[grande].fascia == Fascia3::Molto);
    }
    assert!(molto >= 190, "{molto}");
}

/// Il test non entra in calendario: la legge che il governo ha in esame si testa.
#[test]
fn l_opposizione_testa_la_legge_del_governo_in_esame() {
    let mut p = all_opposizione();
    let legge = (p.vista().calendario.into_iter())
        .find(|m| m.promotore.is_none() && m.forma == FormaMisura::Riforma)
        .unwrap();
    p.comanda(test_di(&legge.valori)).unwrap();
    assert!(p.vista().acquisti.test_misura.is_some());
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

/// Con la cassa pari al prezzo l'acquisto passa e la cassa va a zero, per i tre tipi.
#[test]
fn con_la_cassa_pari_al_prezzo_si_compra() {
    let test = test_di(&v("regime_pensioni", "retributivo"));
    for (acquisto, prezzo) in [
        (SONDAGGIO, PREZZO),
        (test, PREZZO_TEST),
        (CENTRO_STUDI, PREZZO_CENTRO_STUDI),
    ] {
        let mut p = con_cassa(&Partita::nuova(dati_base(), 1), prezzo);
        p.comanda(acquisto).unwrap();
        assert_eq!(p.partiti()[p.giocatore()].cassa, 0, "{prezzo}");
    }
}

/// La quota di una condizione è la sua parte degli elettori di ora.
#[test]
fn la_quota_del_test_e_la_parte_degli_elettori() {
    let p = Partita::nuova(dati_base(), 1);
    let d = dati_base();
    let mut elettori = [0.0; 6];
    for i in 0..p.elettorato().gruppi.len() {
        elettori[condizione_di(i) as usize] += p.elettorato().elettori(d.elettorato(), i);
    }
    let totale: f64 = elettori.iter().sum();
    let vere = p
        .reazioni_vere(&v("regime_pensioni", "retributivo"))
        .unwrap();
    for (w, (_, quota)) in elettori.iter().zip(vere) {
        assert!((w / totale - quota).abs() <= 1e-9, "{elettori:?} {vere:?}");
    }
    let somma: f64 = vere.iter().map(|(_, q)| q).sum();
    assert!((somma - 1.0).abs() <= 1e-9, "{somma}");
}

#[test]
fn l_opposizione_compra_il_sondaggio_privato_e_il_centro_studi() {
    let mut p = all_opposizione();
    let g = p.giocatore();
    let cassa = p.partiti()[g].cassa;
    p.comanda(SONDAGGIO).unwrap();
    assert_eq!(p.partiti()[g].cassa, cassa - PREZZO);
    centro_studi(&mut p);
    let a = p.vista().acquisti;
    assert!(a.sondaggio_privato.is_some() && a.centro_studi.is_some());
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
        j["acquisti"]["test_misura"]["reazioni"][0]["segno"] = 2.into();
    })
    .unwrap_err();
    assert!(e.to_string().contains("acquisti.test.segno"), "{e}");
    let e = edita(&p, |j| {
        let r = &mut j["acquisti"]["test_misura"]["reazioni"];
        r.as_array_mut().unwrap().swap(0, 1);
    })
    .unwrap_err();
    assert!(e.to_string().contains("acquisti.test.reazioni"), "{e}");
}

/// L'oracolo della reazione vera: la partita di `dati` con la bozza messa in vigore prima della
/// chiusura (come `entra_in_vigore` nell'aula dello stesso turno), chiusa `orizzonte_test` volte
/// contro una gemella senza bozza; per condizione, `peso_giudizio` per il Δ del giudizio su ogni
/// partito di governo, pesato con gli elettori del turno zero. Con le reazioni vere.
fn oracolo(dati: DatiGioco, d: &str, g: &str) -> ([Vec<f64>; 6], [(f64, f64); 6]) {
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
    let mut somme: [(f64, Vec<f64>); 6] = std::array::from_fn(|_| (0.0, vec![0.0; governo.len()]));
    let coppie = (con.elettorato().gruppi.iter()).zip(&senza.elettorato().gruppi);
    for (i, (a, b)) in coppie.enumerate() {
        let w = p.elettorato().elettori(dati.elettorato(), i);
        let s = &mut somme[condizione_di(i) as usize];
        s.0 += w;
        for (x, &j) in s.1.iter_mut().zip(&governo) {
            *x += w * (a.giudizio[j] - b.giudizio[j]);
        }
    }
    (
        somme.map(|(w, x)| x.iter().map(|x| gamma * x / w).collect()),
        vere,
    )
}

#[test]
fn la_reazione_vera_coincide_con_la_partita_chiusa() {
    for (d, g) in [
        ("regime_pensioni", "retributivo"),
        ("indicizzazione_pensioni", "piena"),
        ("indicizzazione_pensioni", "bloccata_sopra_soglia"),
    ] {
        let (attesa, vere) = oracolo(DatiGioco::da_testi(&senza_shock()).unwrap(), d, g);
        for (a, (vera, _)) in attesa.iter().zip(vere) {
            let a: f64 = a.iter().sum();
            assert!((a - vera).abs() <= 1e-5, "{d} {g}: {attesa:?} {vere:?}");
        }
    }
}

/// Col lavoro al centrodestra l'alleato prende la colpa delle pensioni: la reazione vera è la
/// somma su tutti i partiti di governo, non sul solo presidente.
#[test]
fn la_reazione_vera_somma_anche_gli_alleati() {
    let f = sostituisci(
        senza_shock(),
        "scenario",
        "lavoro_welfare = \"destra\"",
        "lavoro_welfare = \"centrodestra\"",
    );
    let dati = DatiGioco::da_testi(&f).unwrap();
    let p = Partita::nuova(dati.clone(), 1);
    let governo: Vec<usize> = (0..p.partiti().len())
        .filter(|&j| p.governo().al_governo(j))
        .collect();
    let alleato = (p.partiti().iter())
        .position(|x| x.id == "centrodestra")
        .unwrap();
    let k = governo.iter().position(|&j| j == alleato).unwrap();
    assert_ne!(p.governo().presidente, alleato);
    let (attesa, vere) = oracolo(dati, "regime_pensioni", "retributivo");
    let pensionato = Condizione::Pensionato as usize;
    assert!(attesa[pensionato][k].abs() > 1e-3, "{attesa:?}");
    for (a, (vera, _)) in attesa.iter().zip(vere) {
        let a: f64 = a.iter().sum();
        assert!((a - vera).abs() <= 1e-5, "{attesa:?} {vere:?}");
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
            e.ends_with(
                "cassa insufficiente per il test di una misura: servono 20.000 euro, ce ne sono \
                 19.999"
            ),
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

/// Il test non entra in calendario, quindi la crisi non lo ferma (l'anteprima sì).
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

fn media(x: &[f64]) -> f64 {
    x.iter().sum::<f64>() / x.len() as f64
}

fn deviazione(x: &[f64]) -> f64 {
    let m = media(x);
    (x.iter().map(|v| (v - m).powi(2)).sum::<f64>() / x.len() as f64).sqrt()
}

fn correlazione(x: &[f64], y: &[f64]) -> f64 {
    let (mx, my) = (media(x), media(y));
    let c = (x.iter().zip(y))
        .map(|(a, b)| (a - mx) * (b - my))
        .sum::<f64>()
        / x.len() as f64;
    c / (deviazione(x) * deviazione(y))
}

/// Su 200 semi, per campo, l'errore della stima è fra 0,5 e 0,7 volte quello della prima
/// stima, e non lo segue (|r| < 0,3). Con la stima presa come frazione dell'errore della prima,
/// r = 1 e il vero si ricava da tutte e due.
#[test]
fn la_stima_del_centro_studi_e_piu_vicina() {
    let (mut prima, mut centro) = (vec![Vec::new(); 4], vec![Vec::new(); 4]);
    for seme in 0..200 {
        let mut p = Partita::nuova(dati_base(), seme);
        chiudi_col_bilancio(&mut p);
        let j: serde_json::Value = serde_json::from_str(&p.salva().unwrap()).unwrap();
        let stima = |k: &str| -> [f64; 4] {
            campi(&serde_json::from_value(j["informazione"]["statistica"][k].clone()).unwrap())
        };
        let (vera, ps) = (stima("vera"), stima("prima_stima"));
        let c = campi(&centro_studi(&mut p).stima);
        for k in 0..4 {
            prima[k].push(ps[k] - vera[k]);
            centro[k].push(c[k] - vera[k]);
        }
    }
    for k in 0..4 {
        let rapporto = deviazione(&centro[k]) / deviazione(&prima[k]);
        let r = correlazione(&centro[k], &prima[k]);
        assert!((0.5..=0.7).contains(&rapporto), "{k}: {rapporto}");
        assert!(r.abs() < 0.3, "{k}: r = {r}");
    }
}

/// Come la prima stima: l'inflazione annua compone la stima sua del trimestre coi tre prima.
#[test]
fn il_centro_studi_compone_l_inflazione_annua_dalla_sua_stima() {
    let mut p = Partita::nuova(dati_base(), 1);
    chiudi_col_bilancio(&mut p);
    let vera = p.informazione().statistica.vera.inflazione;
    let c = centro_studi(&mut p).stima;
    assert_ne!(c.inflazione, vera);
    let attesa = p.economia().inflazione_annua_con(c.inflazione);
    assert!((c.inflazione_annua - attesa).abs() < 1e-12);
}

/// Al turno zero la prima stima è il vero, e il centro studi non ci aggiunge errore.
#[test]
fn al_turno_zero_la_stima_del_centro_studi_e_il_vero() {
    let mut p = Partita::nuova(dati_base(), 1);
    let j: serde_json::Value = serde_json::from_str(&p.salva().unwrap()).unwrap();
    let vera: Stima =
        serde_json::from_value(j["informazione"]["statistica"]["vera"].clone()).unwrap();
    assert_eq!(centro_studi(&mut p).stima, vera);
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
        let (r1, r4) = (
            dati.manopola(&format!("rmse_{nome}_1")),
            dati.manopola(&format!("rmse_{nome}_4")),
        );
        assert_eq!((s[0], s[3]), (r1, r4), "{nome}");
        for h in [2, 3] {
            let retta = r1 + (r4 - r1) * f64::from(h - 1) / 3.0;
            assert!((s[h as usize - 1] - retta).abs() <= 1e-12, "{nome}: {s:?}");
        }
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

/// PIL reale come la statistica: nominale sul livello dei prezzi.
fn reale(e: &Economia) -> f64 {
    e.pil_trimestrale / e.livello_prezzi
}

/// I valori veri del trimestre chiuso per ultimo, con `storia[c]` l'economia dopo `c` chiusure.
/// Il trimestre di un anno prima è quello di quattro chiusure fa; prima del turno zero è uno dei
/// quattro piatti al conto e ai prezzi del turno zero, con i prezzi della storia d'inflazione del
/// turno zero. Debito sul PIL degli ultimi quattro trimestri.
fn veri(dati: &DatiGioco, storia: &[Economia]) -> [f64; 4] {
    let (c, e, zero) = (storia.len() - 1, &storia[storia.len() - 1], &storia[0]);
    let prima = &storia[c.saturating_sub(4)];
    let prezzi_prima = prima.livello_prezzi
        / (zero.inflazione[..4usize.saturating_sub(c)].iter())
            .map(|i| 1.0 + i / 400.0)
            .product::<f64>();
    let n = e.storico.len();
    let pil_anno: f64 = e.storico[n - 4..].iter().map(|t| t.pil as f64).sum();
    [
        100.0 * (reale(e) / reale(prima) - 1.0),
        e.disoccupazione_nazionale(dati),
        100.0 * (e.livello_prezzi / prezzi_prima - 1.0),
        100.0 * e.debito as f64 / pil_anno,
    ]
}

/// L'oracolo della previsione: senza shock, derive, emergenze né atti dei ministri, il centro del
/// primo turno è il valore della chiusura vera, con l'acquisto ai turni 0-4 (il trimestre di un
/// anno prima prima del turno zero, poi dopo la prima chiusura). Tolleranza 1e-9: sopra l'ordine
/// delle operazioni del debito (2,8e-14) e l'arrotondamento a euro del PIL di `storico` (~1e-10
/// punti). La previsione del PIL sta anche entro una sigma dall'oracolo.
#[test]
fn senza_shock_la_previsione_e_esatta() {
    let dati = DatiGioco::da_testi(&senza_emergenze(senza_shock())).unwrap();
    let mut p = Partita::nuova(dati.clone(), 1);
    let mut storia = vec![p.economia().clone()];
    for k in 0..=4 {
        ferma_tutti(&mut p);
        let voce = centro_studi(&mut p).previsione.remove(0);
        let data = p.vista().data;
        chiudi_col_bilancio(&mut p);
        storia.push(p.economia().clone());
        assert_eq!(voce.data, data);
        let centri = [
            voce.pil.centro,
            voce.disoccupazione.centro,
            voce.inflazione.centro,
            voce.debito.centro,
        ];
        let veri = veri(&dati, &storia);
        assert!(
            (voce.pil.centro - veri[0]).abs() <= voce.pil.sigma,
            "turno {k}: {centri:?} {veri:?}"
        );
        for (i, (c, v)) in centri.iter().zip(veri).enumerate() {
            assert!((c - v).abs() <= 1e-9, "turno {k}, {i}: {centri:?} {veri:?}");
        }
    }
}

#[test]
fn un_centro_studi_manomesso_nel_salvataggio_e_rifiutato() {
    let mut p = Partita::nuova(dati_base(), 1);
    centro_studi(&mut p);
    let rifiuto = |f: &dyn Fn(&mut serde_json::Value)| {
        edita(&p, |j| f(&mut j["acquisti"]["centro_studi"]))
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
