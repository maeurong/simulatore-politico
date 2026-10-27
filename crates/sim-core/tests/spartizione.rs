#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

mod comune;
use comune::{dati_base, edita, fondata};
use sim_core::legislatura::{Assegnazione, Fase, Incarico};
use sim_core::parlamento::CausaIntesa;
use sim_core::{Comando, Partita};

/// Al posto 4,5 con centrodestra e destra la coalizione prende il premio col seme 7 (sonda del
/// Task 9: al 6,5 nessun partito è disponibile, Task 7).
fn vinta() -> Partita {
    let mut p = fondata(4.5);
    let alleati: Vec<String> = (p.disponibilita().into_iter())
        .filter(|v| {
            v.disponibile
                && ["destra", "centrodestra", "estrema_destra"].contains(&v.partito.as_str())
        })
        .map(|v| v.partito)
        .collect();
    p.comanda(Comando::DichiaraCoalizione {
        partiti: alleati,
        cessioni: vec![],
    })
    .unwrap();
    p.chiudi_turno();
    assert_eq!(
        p.legislatura().incarico,
        Some(Incarico::Giocatore),
        "{:?}",
        p.vista().elezioni[0]
    );
    p
}

fn tutto_a(p: &Partita, partito: Option<&str>) -> Vec<Assegnazione> {
    (p.vista().dicasteri.iter())
        .map(|d| Assegnazione {
            dicastero: d.id.clone(),
            partito: partito.map(Into::into),
        })
        .collect()
}

#[test]
fn la_richiesta_segue_gamson() {
    let p = vinta();
    let r = p.richieste();
    assert!(!r.is_empty());
    let pesi = 14.0; // 3 + 2 + 2 + 7 · 1
    for v in &r {
        let j = p.partiti().iter().position(|x| x.id == v.partito).unwrap();
        let s = |x: &[u32]| f64::from(x[j]);
        let pl = p.parlamento();
        let maggioranza: f64 = (p.legislatura().coalizioni[0].iter())
            .map(|&k| f64::from(pl.seggi_camera[k] + pl.seggi_senato[k]))
            .sum();
        let attesa =
            (0.07 + 0.79 * (s(&pl.seggi_camera) + s(&pl.seggi_senato)) / maggioranza) * pesi;
        assert!((v.richiesta - attesa).abs() < 1e-9, "{v:?}");
    }
}

#[test]
fn l_avarizia_costa_intesa_la_generosita_la_alza() {
    let mut avara = vinta();
    avara
        .comanda(Comando::ProponiSpartizione {
            dicasteri: tutto_a(&avara, Some("giocatore")),
        })
        .unwrap();
    // Senza dicasteri l'offerta è zero positivo, non -0,0 da una somma vuota.
    let offerte: Vec<f64> = avara.richieste().iter().map(|r| r.offerta).collect();
    assert!(!offerte.is_empty());
    assert!(
        offerte.iter().all(|o| *o == 0.0 && o.is_sign_positive()),
        "{offerte:?}"
    );
    avara.chiudi_turno();
    let mut di_default = vinta();
    di_default.chiudi_turno();
    let alleati = avara.alleati();
    assert!(!alleati.is_empty());
    let base = dati_base().manopola("intesa_base");
    for &j in &alleati {
        assert!(avara.intesa().valori[j] < di_default.intesa().valori[j]);
        // Senza dicasteri l'alleato resta in maggioranza.
        assert!(avara.governo().al_governo(j) && !avara.governo().dicasteri.contains(&j));
        // La variazione registrata è quella applicata: da intesa_base, mai sotto lo zero.
        let v = (avara.intesa().variazioni.iter())
            .find(|v| v.partito == j && v.causa == CausaIntesa::Spartizione)
            .unwrap();
        assert!(
            (v.delta - (avara.intesa().valori[j] - base)).abs() < 1e-9,
            "{v:?}"
        );
        assert!(v.delta >= -base, "{v:?}");
    }
    assert_eq!(avara.legislatura().fase, Fase::Ordinaria);
    assert!(avara.governa());
}

#[test]
fn i_tecnici_stanno_nella_quota_del_giocatore() {
    let mut di_partito = vinta();
    di_partito
        .comanda(Comando::ProponiSpartizione {
            dicasteri: tutto_a(&di_partito, Some("giocatore")),
        })
        .unwrap();
    di_partito.chiudi_turno();
    let mut p = vinta();
    p.comanda(Comando::ProponiSpartizione {
        dicasteri: tutto_a(&p, None),
    })
    .unwrap();
    p.chiudi_turno();
    let g = p.giocatore();
    assert!(p.legislatura().incarico.is_none() && p.legislatura().proposta.is_none());
    assert!(p.governo().dicasteri.iter().all(|&k| k == g));
    assert!((p.ministri().dicasteri.iter()).all(|s| s.ministro.as_ref().unwrap().tecnico));
    // Scelta 7: stesse estrazioni del ministro di partito, posizione ancorata a 4 invece che al
    // partito del giocatore, capacità da 3 a 5.
    let ancora = di_partito.partiti()[g].posizione_dichiarata;
    let coppie = p
        .ministri()
        .dicasteri
        .iter()
        .zip(&di_partito.ministri().dicasteri);
    for (t, m) in coppie {
        let (t, m) = (t.ministro.as_ref().unwrap(), m.ministro.as_ref().unwrap());
        assert!((3..=5).contains(&t.capacita), "{t:?}");
        let dentro = |x: f64| x > 1.0 && x < 7.0;
        if dentro(t.posizione) && dentro(m.posizione) {
            assert!((t.posizione - 4.0 - (m.posizione - ancora)).abs() < 1e-9);
        }
    }
    let dopo = (p.comanda(Comando::ProponiSpartizione {
        dicasteri: tutto_a(&p, None),
    }))
    .unwrap_err();
    assert!(dopo.to_string().contains("trattativa"), "{dopo}");
}

#[test]
fn proposte_non_valide_rifiutate_col_nome() {
    let mut p = vinta();
    let prima = p.salva().unwrap();
    let mut a = tutto_a(&p, Some("giocatore"));
    a.pop();
    let ultimo = p.vista().dicasteri.last().unwrap().nome.clone();
    let e = (p.comanda(Comando::ProponiSpartizione { dicasteri: a })).unwrap_err();
    assert!(e.to_string().contains(&ultimo), "{e}");
    let a = tutto_a(&p, Some("sinistra"));
    let e = (p.comanda(Comando::ProponiSpartizione { dicasteri: a })).unwrap_err();
    assert!(e.to_string().contains("Sinistra"), "{e}");
    let mut a = tutto_a(&p, Some("giocatore"));
    a[1] = a[0].clone();
    let e = (p.comanda(Comando::ProponiSpartizione { dicasteri: a })).unwrap_err();
    assert!(e.to_string().contains("Economia e Finanze ripetuto"), "{e}");
    assert_eq!(p.salva().unwrap(), prima);
}

#[test]
fn chi_entra_al_governo_e_giudicato_da_zero() {
    let p = vinta();
    let g = p.giocatore();
    let destra = p.partiti().iter().position(|x| x.id == "destra").unwrap();
    assert!(p.governo().al_governo(destra));
    // Un giudizio residuo sul partito del giocatore, come da un governo passato, e su chi resta.
    let mut p = edita(&p, |j| {
        for x in j["elettorato"]["gruppi"].as_array_mut().unwrap() {
            x["giudizio"][g] = serde_json::json!(1.0);
            x["giudizio"][destra] = serde_json::json!(1.0);
        }
    })
    .unwrap();
    p.chiudi_turno();
    let json: serde_json::Value = serde_json::from_str(&p.salva().unwrap()).unwrap();
    assert!(
        (json["elettorato"]["gruppi"].as_array().unwrap().iter())
            .all(|x| x["giudizio"][g].as_f64().unwrap() == 0.0)
    );
    assert!(
        (json["elettorato"]["gruppi"].as_array().unwrap().iter())
            .all(|x| x["giudizio"][destra].as_f64().unwrap() != 0.0)
    );
}

#[test]
fn salvare_a_meta_trattativa() {
    let mut a = vinta();
    a.comanda(Comando::ProponiSpartizione {
        dicasteri: tutto_a(&a, Some("giocatore")),
    })
    .unwrap();
    let mut b = Partita::carica(&a.salva().unwrap()).unwrap();
    a.chiudi_turno();
    b.chiudi_turno();
    assert_eq!(a.salva().unwrap(), b.salva().unwrap());
}

#[test]
fn carica_rifiuta_proposta_e_coalizione_non_valide() {
    let p = vinta();
    let g = p.giocatore();
    let sinistra = p.partiti().iter().position(|x| x.id == "sinistra").unwrap();
    let fuori = edita(&p, |j| {
        j["legislatura"]["proposta"] = serde_json::json!(vec![Some(sinistra); 10]);
    });
    assert!(
        fuori
            .unwrap_err()
            .to_string()
            .contains("legislatura.proposta")
    );
    let corta = edita(&p, |j| {
        j["legislatura"]["proposta"] = serde_json::json!(vec![Some(g); 9]);
    });
    assert!(
        corta
            .unwrap_err()
            .to_string()
            .contains("legislatura.proposta")
    );
    let vuota = edita(&p, |j| {
        j["legislatura"]["incarico"] = serde_json::json!({ "Ia": { "coalizione": [] } });
    });
    let e = vuota.unwrap_err().to_string();
    assert!(e.contains("legislatura.incarico"), "{e}");
    let mut formata = vinta();
    formata.chiudi_turno();
    let fuori_fase = edita(&formata, |j| {
        j["legislatura"]["proposta"] = serde_json::json!(vec![Some(g); 10]);
    });
    let e = fuori_fase.unwrap_err().to_string();
    assert!(e.contains("legislatura.proposta"), "{e}");
    let ripetuta = edita(&p, |j| {
        j["governo"]["coalizione"] = serde_json::json!([g, g]);
    });
    assert!(
        ripetuta
            .unwrap_err()
            .to_string()
            .contains("governo.coalizione")
    );
}

#[test]
fn l_intesa_degli_alleati_riparte_dalla_base() {
    let mut di_default = vinta();
    let g = di_default.giocatore();
    let alleati: Vec<usize> = (di_default.legislatura().coalizioni[0].iter().copied())
        .filter(|&j| j != g)
        .collect();
    let mut bassa = edita(&di_default, |j| {
        for &k in &alleati {
            j["intesa"]["valori"][k] = serde_json::json!(10.0);
        }
    })
    .unwrap();
    di_default.chiudi_turno();
    bassa.chiudi_turno();
    for &k in &alleati {
        assert_eq!(bassa.intesa().valori[k], di_default.intesa().valori[k]);
    }
}

/// Partita di prova: il voto alla scadenza (turno 19) dà l'incarico a una coalizione IA; alla
/// chiusura della trattativa governa il suo primo partito per seggi, il giocatore no.
#[test]
fn con_l_incarico_ia_governa_il_primo_partito_per_seggi() {
    let mut p = Partita::nuova(comune::dati_base(), 7);
    for _ in 0..20 {
        p.chiudi_turno();
    }
    let Some(Incarico::Ia { coalizione }) = p.legislatura().incarico.clone() else {
        panic!("{:?}", p.legislatura().incarico);
    };
    let pl = p.parlamento();
    let primo = *(coalizione.iter())
        .max_by_key(|&&j| pl.seggi_camera[j] + pl.seggi_senato[j])
        .unwrap();
    p.chiudi_turno();
    assert_eq!(p.governo().presidente, primo);
    assert_eq!(p.governo().coalizione, coalizione);
    assert!(!p.governa() && p.legislatura().fase == Fase::Ordinaria);
}

/// Senza proposta l'offerta mostrata è quella della spartizione di default, la stessa che la
/// chiusura applica.
#[test]
fn senza_proposta_l_offerta_e_quella_di_default() {
    let mut p = vinta();
    let richieste = p.richieste();
    p.chiudi_turno();
    let dati = dati_base();
    for v in &richieste {
        let j = p.partiti().iter().position(|x| x.id == v.partito).unwrap();
        let punti: f64 = (dati.dicasteri().iter().zip(&p.governo().dicasteri))
            .filter(|&(_, &k)| k == j)
            .map(|(d, _)| f64::from(d.peso_spartizione))
            .sum();
        assert!(punti > 0.0 && v.offerta == punti, "{v:?} {punti}");
    }
}
