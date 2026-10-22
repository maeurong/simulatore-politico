#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

mod comune;
use comune::{dati_base, file_base, imposta, senza_shock};
use sim_core::elettorato::PUNTI;
use sim_core::{DatiGioco, FileDati, Partita};

fn con_manopola(f: FileDati, nome: &str, valore: &str) -> FileDati {
    imposta(f, "manopole", &format!("[{nome}]\nvalore = "), valore)
}

fn giocata(f: &FileDati, seme: u64) -> Partita {
    Partita::nuova(DatiGioco::da_testi(f).unwrap(), seme)
}

fn somma_uno(v: &[f64]) -> bool {
    (v.iter().sum::<f64>() - 1.0).abs() < 1e-12 && v.iter().all(|x| x.is_finite() && *x >= 0.0)
}

#[test]
fn la_vista_non_contiene_lo_stato_vero() {
    let mut p = Partita::nuova(dati_base(), 4);
    p.chiudi_turno();
    let json = serde_json::to_string(&p.vista()).unwrap();
    for chiave in [
        "\"bias\"",
        "\"effetti_istituto\"",
        "\"vera\"",
        "\"voto_precedente\"",
        "\"giudizio\"",
        "\"per_gruppo\"",
    ] {
        assert!(!json.contains(chiave), "{chiave}");
    }
}

#[test]
fn la_vista_mostra_i_partiti_con_il_governo() {
    let p = Partita::nuova(dati_base(), 4);
    let partiti = p.vista().partiti;
    assert_eq!(partiti.len(), p.partiti().len());
    for (i, v) in partiti.iter().enumerate() {
        assert_eq!(v.id, p.partiti()[i].id);
        assert_eq!(v.al_governo, p.governo().al_governo(i));
        assert_eq!(v.valence, p.partiti()[i].valence);
    }
    assert!(partiti.iter().any(|v| v.al_governo));
}

#[test]
fn la_prima_stima_sbaglia_e_la_revisione_e_esatta() {
    let mut p = Partita::nuova(dati_base(), 4);
    p.chiudi_turno();
    let s = p.vista().statistica;
    let vera = p.informazione().statistica.vera.clone();
    assert_ne!(s.prima_stima.crescita_pil, vera.crescita_pil);
    p.chiudi_turno();
    let r = p.vista().statistica.revisione.unwrap();
    assert_eq!(r.riveduta, vera);
    assert_eq!(r.prima_stima, s.prima_stima);
}

#[test]
fn senza_errore_la_prima_stima_e_esatta() {
    let mut f = senza_shock();
    for n in ["pil", "disoccupazione", "inflazione", "disavanzo"] {
        f = con_manopola(f, &format!("errore_stima_{n}"), "0.0");
    }
    let mut p = giocata(&f, 4);
    for _ in 0..3 {
        p.chiudi_turno();
        let s = &p.informazione().statistica;
        assert_eq!(s.prima_stima, s.vera);
    }
}

#[test]
fn la_statistica_segue_l_economia() {
    let d = dati_base();
    let mut p = Partita::nuova(dati_base(), 4);
    assert_eq!(p.informazione().statistica.vera.crescita_pil, 0.0);
    let r0 = p.economia().pil_trimestrale / p.economia().livello_prezzi;
    p.chiudi_turno();
    let e = p.economia();
    let vera = &p.informazione().statistica.vera;
    let crescita = 400.0 * (e.pil_trimestrale / e.livello_prezzi / r0 - 1.0);
    assert!((vera.crescita_pil - crescita).abs() < 1e-9);
    assert_eq!(vera.inflazione, e.inflazione[0]);
    assert!((vera.disoccupazione - e.disoccupazione_nazionale(&d)).abs() < 1e-12);
    let disavanzo = 100.0 * e.disavanzo_trimestre as f64 / (e.pil_trimestrale * 1e9);
    assert!((vera.disavanzo - disavanzo).abs() < 1e-12);
}

#[test]
fn la_media_degli_istituti_non_toglie_il_bias_comune() {
    let f = con_manopola(file_base(), "sigma_istituto", "0.0");
    let f = con_manopola(f, "effetto_disegno", "1.0");
    let mut p = giocata(&f, 4);
    let turni = 40;
    // Scarto medio osservato contro quello che il bias dà dopo la rinormalizzazione:
    // (p + b) / Σ(p + b) − p. Senza rinormalizzare sarebbe il bias stesso.
    let mut scarto = vec![0.0; p.partiti().len() + 1];
    let mut atteso = scarto.clone();
    for _ in 0..turni {
        p.chiudi_turno();
        let vero = p.intenzioni_vere().nazionali;
        let bias = &p.informazione().bias;
        let somma: f64 = vero.iter().zip(bias).map(|(v, b)| v + b).sum();
        let ist = &p.informazione().sondaggio.istituti;
        for (j, s) in scarto.iter_mut().enumerate() {
            let media = ist.iter().map(|r| r.quote[j]).sum::<f64>() / ist.len() as f64;
            *s += (media - vero[j]) / f64::from(turni);
            atteso[j] += ((vero[j] + bias[j]) / somma - vero[j]) / f64::from(turni);
        }
    }
    for (s, a) in scarto.iter().zip(&atteso) {
        assert!((s - a).abs() < 0.005, "{s} {a}");
    }
    assert!(scarto.iter().any(|s| s.abs() > 0.005), "{scarto:?}");
}

#[test]
fn l_errore_cresce_nei_sottocampioni_piccoli() {
    let f = con_manopola(senza_shock(), "sigma_bias_comune", "0.0");
    let d = DatiGioco::da_testi(&f).unwrap();
    let d = d.elettorato();
    let mut p = giocata(&f, 4);
    let errore = |p: &Partita, id: &str| {
        let r = d.regioni.iter().position(|r| r.id == id).unwrap();
        let vero = p.intenzioni_vere().per_gruppo;
        let (mut pesi, mut m) = (0.0, vec![0.0; p.partiti().len() + 1]);
        for (i, v) in vero.iter().enumerate().skip(r * 18).take(18) {
            let w = p.elettorato().elettori(d, i);
            pesi += w;
            m.iter_mut().zip(v).for_each(|(a, x)| *a += w * x);
        }
        let inc = p
            .informazione()
            .sondaggio
            .regioni
            .iter()
            .find(|i| i.id == id);
        m.iter()
            .zip(&inc.unwrap().quote)
            .map(|(a, q)| (a / pesi - q).abs())
            .sum::<f64>()
    };
    let (mut vda, mut lombardia) = (0.0, 0.0);
    for _ in 0..30 {
        p.chiudi_turno();
        vda += errore(&p, "valle_d_aosta");
        lombardia += errore(&p, "lombardia");
    }
    assert!(vda > lombardia, "{vda} {lombardia}");
}

#[test]
fn incrocio_della_regione_piu_piccola_valido() {
    let p = Partita::nuova(dati_base(), 4);
    let vda = p
        .vista()
        .sondaggio
        .regioni
        .into_iter()
        .find(|i| i.id == "valle_d_aosta")
        .unwrap();
    assert!(vda.campione >= 1);
    assert!(somma_uno(&vda.quote));
    assert!(somma_uno(&vda.posizione) && vda.posizione.len() == PUNTI);
    assert!(vda.approvazione.is_finite());
}

#[test]
fn quattro_istituti_e_incroci_completi() {
    let p = Partita::nuova(dati_base(), 4);
    let s = p.vista().sondaggio;
    assert_eq!(
        (
            s.istituti.len(),
            s.condizioni.len(),
            s.fasce.len(),
            s.regioni.len()
        ),
        (4, 6, 3, 20)
    );
    let n = p.partiti().len() + 1;
    for r in &s.istituti {
        assert!(r.quote.len() == n && somma_uno(&r.quote));
    }
    for i in s.condizioni.iter().chain(&s.fasce).chain(&s.regioni) {
        assert!(i.quote.len() == n && somma_uno(&i.quote) && somma_uno(&i.posizione));
    }
}
