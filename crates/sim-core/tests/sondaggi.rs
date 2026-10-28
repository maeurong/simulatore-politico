#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

mod comune;
use comune::{chiudi_col_bilancio, dati_base, file_base, imposta, riscrivi, senza_shock};
use sim_core::caso::Caso;
use sim_core::elettorato::{Condizione, Fascia, PUNTI};
use sim_core::informazione::{ElettoreCampione, Motivo, Sondaggio, genera};
use sim_core::{DatiGioco, Errore, FileDati, Partita, Vista};

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
        chiudi_col_bilancio(&mut p);
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
fn incrocio_con_sottocampione_nullo_valido() {
    // Campione 100 per istituto e una Valle d'Aosta da 1000 elettori: 400 · quota < 0,5,
    // il sottocampione arrotondato sarebbe zero e scatta il minimo di 1.
    let f = riscrivi(file_base(), "elettorato", "campione = ", "campione = 100");
    let f = imposta(
        f,
        "elettorato",
        "id = \"valle_d_aosta\"\nelettori = ",
        "1000",
    );
    let mut p = giocata(&f, 4);
    for _ in 0..5 {
        let v = p.vista();
        let vda = v.sondaggio.regioni.iter().find(|i| i.id == "valle_d_aosta");
        let vda = vda.unwrap();
        assert_eq!(vda.campione, 1);
        for i in v.sondaggio.condizioni.iter().chain(&v.sondaggio.fasce) {
            assert!(i.campione >= 1);
        }
        for i in v.sondaggio.regioni.iter().chain(&v.sondaggio.condizioni) {
            assert!(somma_uno(&i.quote), "{}", i.id);
            assert!(somma_uno(&i.posizione) && i.posizione.len() == PUNTI);
            assert!(i.approvazione.is_finite());
        }
        p.chiudi_turno();
    }
}

#[test]
fn quote_dell_istituto_sono_vero_piu_bias_piu_effetto() {
    // Campioni da un miliardo: l'errore di campionamento scende sotto 1e-4 e restano
    // bias comune ed effetto d'istituto.
    let f = riscrivi(
        file_base(),
        "elettorato",
        "campione = ",
        "campione = 1000000000",
    );
    let f = con_manopola(f, "sigma_istituto", "0.02");
    let f = con_manopola(f, "sigma_bias_comune", "0.02");
    let mut p = giocata(&f, 4);
    p.chiudi_turno();
    let vero = p.intenzioni_vere().nazionali;
    let inf = p.informazione();
    let h = &inf.effetti_istituto;
    assert!((h[0][0] - h[1][0]).abs() > 1e-3, "effetti uguali");
    for (k, r) in inf.sondaggio.istituti.iter().enumerate() {
        let grezzo: Vec<f64> = (0..vero.len())
            .map(|j| (vero[j] + inf.bias[j] + h[k][j]).max(0.0))
            .collect();
        let tot: f64 = grezzo.iter().sum();
        for (q, g) in r.quote.iter().zip(&grezzo) {
            assert!((q - g / tot).abs() < 5e-4, "istituto {k}: {q} {}", g / tot);
        }
    }
}

#[test]
fn campioni_al_massimo_non_vanno_in_panico() {
    // I dati viaggiano nel salvataggio: quattro campioni da u32::MAX non devono traboccare.
    let f = riscrivi(
        file_base(),
        "elettorato",
        "campione = ",
        "campione = 4294967295",
    );
    let mut p = giocata(&f, 4);
    p.chiudi_turno();
    assert!(
        p.vista()
            .sondaggio
            .regioni
            .iter()
            .all(|i| somma_uno(&i.quote))
    );
}

#[test]
fn proiezione_segue_il_sondaggio() {
    let p = Partita::nuova(dati_base(), 1);
    let n = p.partiti().len();
    let k = p.partiti().iter().position(|x| x.id == "centro").unwrap();
    let mut j: serde_json::Value = serde_json::from_str(&p.salva().unwrap()).unwrap();
    for inc in j["informazione"]["sondaggio"]["regioni"]
        .as_array_mut()
        .unwrap()
    {
        let q = inc["quote"].as_array_mut().unwrap();
        q[k] = 0.0.into();
        let tot: f64 = q.iter().map(|x| x.as_f64().unwrap()).sum();
        for x in q.iter_mut() {
            *x = (x.as_f64().unwrap() / tot).into();
        }
    }
    let p = Partita::carica(&j.to_string()).unwrap();
    let v = p.vista();
    let seggi = v.proiezione.unwrap();
    assert_eq!((seggi[k].camera, seggi[k].senato), (0, 0));
    let d = dati_base();
    let voti = |i: usize| -> f64 {
        (v.sondaggio.regioni.iter().zip(&d.elettorato().regioni))
            .map(|(r, e)| r.quote[i] * e.elettori as f64)
            .sum()
    };
    let altri = || (0..n).filter(|&i| i != k);
    let piu = altri()
        .max_by(|a, b| voti(*a).total_cmp(&voti(*b)))
        .unwrap();
    let meno = altri()
        .min_by(|a, b| voti(*a).total_cmp(&voti(*b)))
        .unwrap();
    assert!(seggi[piu].camera > seggi[meno].camera);
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

#[test]
fn elettori_campione_non_toccano_la_partita() {
    let mut a = Partita::nuova(dati_base(), 9);
    let mut b = Partita::nuova(dati_base(), 9);
    let chiedi = |p: &Partita| {
        p.elettori_campione("campania", Condizione::Pensionato, Fascia::Bassa)
            .unwrap()
    };
    let primi = chiedi(&a);
    assert_eq!(primi, chiedi(&a));
    assert_eq!(primi.len(), 3);
    a.chiudi_turno();
    b.chiudi_turno();
    assert_eq!(a.salva().unwrap(), b.salva().unwrap());
}

#[test]
fn nessun_elettore_vota_un_partito_che_il_sondaggio_da_a_zero() {
    let dati = dati_base();
    let mut v = Partita::nuova(dati_base(), 3).vista();
    let centro = v.partiti.iter().position(|x| x.id == "centro").unwrap();
    let nome = v.partiti[centro].id.clone();
    let s = &mut v.sondaggio;
    let quote = s.istituti.iter_mut().map(|r| &mut r.quote);
    let incroci = s
        .condizioni
        .iter_mut()
        .chain(&mut s.fasce)
        .chain(&mut s.regioni)
        .map(|i| &mut i.quote);
    for q in quote.chain(incroci) {
        q[centro] = 0.0;
        let tot: f64 = q.iter().sum();
        q.iter_mut().for_each(|x| *x /= tot);
    }
    let caso = Caso::nuovo(3);
    for chiave in 0..50 {
        let mut rng = caso.estemporaneo(chiave);
        let gente = genera(
            &v.sondaggio,
            &v.partiti,
            &dati,
            0,
            Condizione::Autonomo,
            Fascia::Media,
            &mut rng,
        );
        assert!(gente.iter().all(|e| e.voto.as_ref() != Some(&nome)));
    }
}

/// Ogni vettore di quote del sondaggio (istituti e incroci).
fn quote_mut(s: &mut Sondaggio) -> impl Iterator<Item = &mut Vec<f64>> {
    let incroci = (s
        .condizioni
        .iter_mut()
        .chain(&mut s.fasce)
        .chain(&mut s.regioni))
    .map(|i| &mut i.quote);
    s.istituti.iter_mut().map(|r| &mut r.quote).chain(incroci)
}

fn tre(v: &Vista, c: Condizione) -> Vec<ElettoreCampione> {
    let mut rng = Caso::nuovo(3).estemporaneo(1);
    genera(
        &v.sondaggio,
        &v.partiti,
        &dati_base(),
        0,
        c,
        Fascia::Media,
        &mut rng,
    )
}

#[test]
fn casi_degeneri_del_campione() {
    let mut v = Partita::nuova(dati_base(), 3).vista();
    // Stima a somma zero: l'incrocio della condizione è tutto a zero, si ripiega sulla regione.
    v.sondaggio.condizioni[Condizione::Autonomo as usize]
        .quote
        .fill(0.0);
    let regione = &mut v.sondaggio.regioni[0].quote;
    regione.fill(0.0);
    regione[0] = 1.0;
    let primo = v.partiti[0].id.clone();
    // Nessun istogramma: la posizione cade sul punto di mezzo.
    let incroci = (v.sondaggio.condizioni.iter_mut())
        .chain(&mut v.sondaggio.fasce)
        .chain(&mut v.sondaggio.regioni);
    incroci.for_each(|i| i.posizione.fill(0.0));
    let gente = tre(&v, Condizione::Autonomo);
    assert_eq!(gente.len(), 3);
    assert!(gente.iter().all(|e| e.posizione == 4), "{gente:?}");
    assert!(
        gente.iter().all(|e| e.voto.as_ref() == Some(&primo)),
        "{gente:?}"
    );
}

/// Vista in cui il sondaggio dà tutto al primo partito e nessuna approvazione.
fn vista_tutto_al_primo() -> Vista {
    let mut v = Partita::nuova(dati_base(), 3).vista();
    for q in quote_mut(&mut v.sondaggio) {
        let n = q.len();
        *q = (0..n).map(|j| f64::from(u8::from(j == 0))).collect();
    }
    let s = &mut v.sondaggio;
    for i in s
        .condizioni
        .iter_mut()
        .chain(&mut s.fasce)
        .chain(&mut s.regioni)
    {
        i.approvazione = 0.0;
        i.posizione = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0];
    }
    for p in &mut v.partiti {
        (p.posizione_percepita, p.valence, p.al_governo) = (4.0, 0.0, false);
    }
    v
}

#[test]
fn il_motivo_e_il_termine_che_domina() {
    let c = Condizione::Pensionato;
    // Vicinanza: il primo partito sta sul punto dell'elettore, gli altri lontani.
    let mut v = vista_tutto_al_primo();
    v.partiti
        .iter_mut()
        .skip(1)
        .for_each(|p| p.posizione_percepita = 7.0);
    assert!(tre(&v, c).iter().all(|e| e.motivo == Motivo::Vicinanza));
    // Leader: posizioni uguali, valence alta del primo.
    let mut v = vista_tutto_al_primo();
    v.partiti[0].valence = 2.0;
    assert!(tre(&v, c).iter().all(|e| e.motivo == Motivo::Leader));
    // Governo: posizioni e valence uguali, il primo governa e l'approvazione è alta ovunque.
    let mut v = vista_tutto_al_primo();
    v.partiti[0].al_governo = true;
    let s = &mut v.sondaggio;
    for i in s
        .condizioni
        .iter_mut()
        .chain(&mut s.fasce)
        .chain(&mut s.regioni)
    {
        i.approvazione = 1.0;
    }
    assert!(tre(&v, c).iter().all(|e| e.motivo == Motivo::Governo));
}

#[test]
fn proiezione_assente_se_il_riparto_rifiuta() {
    let p = Partita::nuova(dati_base(), 1);
    let mut j: serde_json::Value = serde_json::from_str(&p.salva().unwrap()).unwrap();
    for inc in j["informazione"]["sondaggio"]["regioni"]
        .as_array_mut()
        .unwrap()
    {
        inc["quote"][0] = 1e30.into();
    }
    let v = Partita::carica(&j.to_string()).unwrap().vista();
    assert!(v.proiezione.is_none());
}

#[test]
fn chi_si_astiene_ha_motivo_protesta() {
    let dati = dati_base();
    let v = Partita::nuova(dati_base(), 3).vista();
    let caso = Caso::nuovo(3);
    let (mut astenuti, mut votanti) = (0, 0);
    for chiave in 0..70 {
        let mut rng = caso.estemporaneo(chiave);
        for e in genera(
            &v.sondaggio,
            &v.partiti,
            &dati,
            2,
            Condizione::Studente,
            Fascia::Bassa,
            &mut rng,
        ) {
            assert_eq!(e.voto.is_none(), e.motivo == Motivo::Protesta, "{e:?}");
            assert!((1..=7).contains(&e.posizione));
            assert!((18..=25).contains(&e.eta));
            if e.voto.is_none() {
                astenuti += 1;
            } else {
                votanti += 1;
            }
        }
    }
    assert!(astenuti > 0 && votanti > 0, "{astenuti} {votanti}");
}

#[test]
fn regione_sconosciuta_rifiutata_col_nome() {
    let p = Partita::nuova(dati_base(), 1);
    let Err(Errore::Elettorato(m)) =
        p.elettori_campione("padania", Condizione::Studente, Fascia::Media)
    else {
        panic!("regione sconosciuta accettata")
    };
    assert!(m.contains("padania"));
}

#[test]
fn la_proiezione_assegna_tutti_i_seggi() {
    let v = Partita::nuova(dati_base(), 1).vista();
    let s = v.proiezione.expect("riparto riuscito");
    assert_eq!(s.iter().map(|x| x.camera).sum::<u32>(), 400);
    assert_eq!(s.iter().map(|x| x.senato).sum::<u32>(), 200);
}
