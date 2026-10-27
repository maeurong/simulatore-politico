#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

mod comune;
use comune::{edita, fondata};
use sim_core::legislatura::{Assegnazione, Fase, Incarico};
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
    avara.chiudi_turno();
    let mut di_default = vinta();
    di_default.chiudi_turno();
    for &j in &avara.alleati() {
        assert!(avara.intesa().valori[j] < di_default.intesa().valori[j]);
    }
    assert_eq!(avara.legislatura().fase, Fase::Ordinaria);
    assert!(avara.governa());
}

#[test]
fn i_tecnici_stanno_nella_quota_del_giocatore() {
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
    // Un giudizio residuo sul partito del giocatore, come da un governo passato.
    let mut p = edita(&p, |j| {
        for x in j["elettorato"]["gruppi"].as_array_mut().unwrap() {
            x["giudizio"][g] = serde_json::json!(1.0);
        }
    })
    .unwrap();
    p.chiudi_turno();
    let json: serde_json::Value = serde_json::from_str(&p.salva().unwrap()).unwrap();
    assert!(
        (json["elettorato"]["gruppi"].as_array().unwrap().iter())
            .all(|x| x["giudizio"][g].as_f64().unwrap() == 0.0)
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
