#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

mod comune;
use comune::{dati_base, file_base};
use sim_core::{DatiGioco, Errore, FileDati, Partita, Trimestre, VERSIONE_SCHEMA};

fn giocata(seme: u64, turni: u32) -> Partita {
    let mut p = Partita::nuova(dati_base(), seme);
    for _ in 0..turni {
        p.chiudi_turno();
    }
    p
}

#[test]
fn turno_zero_e_ottobre_dicembre_2026() {
    let v = Partita::nuova(dati_base(), 1).vista();
    assert_eq!(v.turno, 0);
    assert_eq!(
        v.data,
        Trimestre {
            anno: 2026,
            trimestre: 4
        }
    );
    assert_eq!(v.dicasteri.len(), 10);
    assert!((79..=89).contains(&v.spread_pb), "{}", v.spread_pb);
}

#[test]
fn chiudere_il_turno_avanza_di_un_trimestre() {
    assert_eq!(
        giocata(1, 1).vista().data,
        Trimestre {
            anno: 2027,
            trimestre: 1
        }
    );
    let v = giocata(1, 8).vista();
    assert_eq!(v.turno, 8);
    assert_eq!(
        v.data,
        Trimestre {
            anno: 2028,
            trimestre: 4
        }
    );
}

#[test]
fn stesso_seme_stessa_partita() {
    assert_eq!(
        giocata(42, 8).salva().unwrap(),
        giocata(42, 8).salva().unwrap()
    );
}

#[test]
fn salvare_e_ricaricare_non_cambia_il_seguito() {
    let mut diretta = giocata(42, 4);
    let mut ricaricata = Partita::carica(&diretta.salva().unwrap()).unwrap();
    assert_eq!(ricaricata, diretta);
    for _ in 0..4 {
        diretta.chiudi_turno();
        ricaricata.chiudi_turno();
    }
    assert_eq!(ricaricata.salva().unwrap(), diretta.salva().unwrap());
}

// Primo test che estrae dal Caso prima e dopo salva/carica (rimando della PR #15).
#[test]
fn estrazioni_identiche_dopo_salva_carica() {
    let mut diretta = giocata(42, 3);
    let mut ricaricata = Partita::carica(&diretta.salva().unwrap()).unwrap();
    for _ in 0..5 {
        diretta.chiudi_turno();
        ricaricata.chiudi_turno();
    }
    assert_eq!(ricaricata.economia(), diretta.economia());
    assert_eq!(ricaricata.salva().unwrap(), diretta.salva().unwrap());
    assert_ne!(
        giocata(42, 8).economia(),
        giocata(43, 8).economia(),
        "il caso deve essere stato estratto"
    );
}

#[test]
fn i_dati_viaggiano_nel_salvataggio() {
    let p = Partita::carica(&giocata(3, 1).salva().unwrap()).unwrap();
    assert!(p.vista().dicasteri.iter().any(|d| d.nome == "Sanità"));
}

#[test]
fn schema_diverso_rifiutato() {
    let mut json: serde_json::Value =
        serde_json::from_str(&giocata(1, 1).salva().unwrap()).unwrap();
    json["versione_schema"] = 99.into();
    let e = Partita::carica(&json.to_string()).unwrap_err();
    assert_eq!(
        e,
        Errore::SchemaDiverso {
            trovata: 99,
            attesa: VERSIONE_SCHEMA
        }
    );
    assert_eq!(
        e.to_string(),
        format!(
            "salvataggio di una versione diversa del gioco: schema 99, atteso {VERSIONE_SCHEMA}"
        )
    );
}

#[test]
fn salvataggio_troncato_non_va_in_panico() {
    let s = giocata(1, 2).salva().unwrap();
    let e = Partita::carica(&s[..s.floor_char_boundary(s.len() / 2)]).unwrap_err();
    assert!(matches!(e, Errore::SalvataggioIlleggibile(_)));
}

fn salvataggio_modificato(f: impl FnOnce(&mut serde_json::Value)) -> String {
    let mut json: serde_json::Value =
        serde_json::from_str(&giocata(1, 1).salva().unwrap()).unwrap();
    f(&mut json);
    json.to_string()
}

#[test]
fn trimestre_fuori_intervallo_rifiutato() {
    let s = salvataggio_modificato(|j| j["data"]["trimestre"] = 255.into());
    let e = Partita::carica(&s).unwrap_err();
    assert!(matches!(&e, Errore::SalvataggioIlleggibile(m) if m.contains("255")));
}

#[test]
fn contatori_al_limite_non_vanno_in_panico() {
    let s = salvataggio_modificato(|j| {
        j["turno"] = u32::MAX.into();
        j["data"] = serde_json::json!({ "anno": 2200, "trimestre": 4 });
        // A quel turno ogni valore in arrivo dei dati sarebbe già scaduto.
        for v in j["disposizioni"]["voci"].as_array_mut().unwrap() {
            v["in_arrivo"] = serde_json::json!([]);
        }
    });
    let mut p = Partita::carica(&s).unwrap();
    p.chiudi_turno();
    assert_eq!(p.vista().turno, u32::MAX);
}

#[test]
fn f64_identici_attraverso_salva_carica() {
    let file = file_base();
    let manopole = file
        .manopole
        .replacen("valore = 40.0", "valore = 94.52706955539223", 1)
        .replacen("min = 30.0", "min = 0.0", 1)
        .replacen("max = 50.0", "max = 100.0", 1);
    let dati = DatiGioco::da_testi(&FileDati { manopole, ..file }).unwrap();
    let p = Partita::nuova(dati, 1);
    let r = Partita::carica(&p.salva().unwrap()).unwrap();
    assert_eq!(r, p);
}

#[test]
fn manopola_nan_rifiutata_al_caricamento() {
    let file = file_base();
    let manopole = file.manopole.replacen("valore = 40.0", "valore = nan", 1);
    let e = DatiGioco::da_testi(&FileDati { manopole, ..file }).unwrap_err();
    assert!(matches!(e, Errore::FuoriIntervallo { ref nome, .. } if nome == "soglia_minaccia"));
}

#[test]
fn il_salvataggio_non_ripete_il_seme() {
    let json: serde_json::Value = serde_json::from_str(&giocata(5, 1).salva().unwrap()).unwrap();
    assert!(json.get("seme").is_none());
    assert_eq!(json["caso"]["seme"], 5);
}

#[test]
fn dati_incoerenti_nel_salvataggio_rifiutati() {
    let s = salvataggio_modificato(|j| {
        for d in j["dati"]["dicasteri"].as_array_mut().unwrap() {
            d["ruoli"] = serde_json::json!([]);
        }
    });
    let e = Partita::carica(&s).unwrap_err();
    assert!(matches!(e, Errore::RuoloOrfano { .. }));
}

#[test]
fn versione_assente_vale_zero() {
    let s = salvataggio_modificato(|j| {
        j.as_object_mut().unwrap().remove("versione_schema");
    });
    let e = Partita::carica(&s).unwrap_err();
    assert_eq!(
        e,
        Errore::SchemaDiverso {
            trovata: 0,
            attesa: VERSIONE_SCHEMA
        }
    );
}

#[test]
fn versione_non_intera_rifiutata() {
    for sbagliata in [serde_json::json!("1"), serde_json::json!(1.5)] {
        let s = salvataggio_modificato(|j| j["versione_schema"] = sbagliata.clone());
        let e = Partita::carica(&s).unwrap_err();
        assert!(
            matches!(e, Errore::SchemaDiverso { trovata: 0, .. }),
            "{sbagliata}"
        );
    }
}

#[test]
fn economia_che_non_combacia_coi_dati_rifiutata() {
    let rifiutata = |campo: &str, f: fn(&mut Vec<serde_json::Value>)| {
        let s = salvataggio_modificato(|j| f(j["economia"][campo].as_array_mut().unwrap()));
        Partita::carica(&s).unwrap_err()
    };
    let e = rifiutata("regioni", |v| {
        v.pop();
    });
    assert!(matches!(&e, Errore::SalvataggioIlleggibile(m) if m.contains("economia.regioni")));
    let e = rifiutata("regioni", |v| v.swap(0, 1));
    assert!(matches!(&e, Errore::SalvataggioIlleggibile(m) if m.contains("economia.regioni")));
    let e = rifiutata("gap_settori", |v| {
        v.pop();
    });
    assert!(matches!(&e, Errore::SalvataggioIlleggibile(m) if m.contains("gap_settori")));
    let e = rifiutata("prezzi_settori", |v| {
        v.pop();
    });
    assert!(matches!(&e, Errore::SalvataggioIlleggibile(m) if m.contains("prezzi_settori")));
}

#[test]
fn campo_obbligatorio_mancante_rifiutato() {
    let s = salvataggio_modificato(|j| {
        j.as_object_mut().unwrap().remove("turno");
    });
    let e = Partita::carica(&s).unwrap_err();
    assert!(matches!(&e, Errore::SalvataggioIlleggibile(m) if m.contains("turno")));
}

#[test]
fn carica_rifiuta_stato_incoerente() {
    use serde_json::{Value, json};
    type Guasto = fn(&mut Value);
    let casi: [(&str, Guasto); 6] = [
        ("storico vuoto", |j| j["economia"]["storico"] = json!([])),
        ("stock diverso dal debito", |j| {
            j["economia"]["sfc"]["stock"]["stato"] = 1.into();
        }),
        ("bilancio con id diverso", |j| {
            j["economia"]["bilancio"]["dicasteri"][0]["id"] = "altro".into();
        }),
        ("pensioni vuote", |j| j["economia"]["pensioni"] = json!([])),
        ("anno oltre il 2200", |j| j["data"]["anno"] = 2201.into()),
        ("disavanzo diverso dai conti", |j| {
            j["economia"]["disavanzo_trimestre"] = 1.into();
        }),
    ];
    for (nome, guasto) in casi {
        let e = Partita::carica(&salvataggio_modificato(guasto)).unwrap_err();
        assert!(
            matches!(e, Errore::SalvataggioIlleggibile(_)),
            "{nome}: {e:?}"
        );
    }
}

#[test]
fn pensionati_enormi_non_vanno_in_panico() {
    // Due annate da i64::MAX/2+1: la loro somma trabocca. Carica rifiuta, o il gennaio regge.
    let s = salvataggio_modificato(|j| {
        let pensioni = j["economia"]["pensioni"].as_array_mut().unwrap();
        let mut annata = pensioni[0].clone();
        annata["anno"] = 2027.into();
        annata["pensionati"] = (i64::MAX / 2 + 1).into();
        pensioni.push(annata.clone());
        pensioni.push(annata);
    });
    match Partita::carica(&s) {
        Err(e) => assert!(matches!(e, Errore::SalvataggioIlleggibile(_)), "{e:?}"),
        Ok(mut p) => p.chiudi_turno(),
    }
}

#[test]
fn salvataggio_con_un_gruppo_in_meno_illeggibile() {
    let s = salvataggio_modificato(|j| {
        j["elettorato"]["gruppi"].as_array_mut().unwrap().pop();
    });
    assert!(matches!(
        Partita::carica(&s),
        Err(Errore::SalvataggioIlleggibile(_))
    ));
}

#[test]
fn salvataggio_con_crescite_oltre_16_illeggibile() {
    let s = salvataggio_modificato(|j| {
        j["elettorato"]["gruppi"][3]["crescite"] = serde_json::Value::Array(vec![0.0.into(); 17]);
    });
    assert!(matches!(
        Partita::carica(&s),
        Err(Errore::SalvataggioIlleggibile(_))
    ));
}

#[test]
fn salvataggio_con_giudizio_corto_illeggibile() {
    let s = salvataggio_modificato(|j| {
        j["elettorato"]["gruppi"][5]["giudizio"]
            .as_array_mut()
            .unwrap()
            .pop();
    });
    assert!(matches!(
        Partita::carica(&s),
        Err(Errore::SalvataggioIlleggibile(_))
    ));
}

#[test]
fn salvataggio_con_presidente_fuori_dai_partiti_illeggibile() {
    let s = salvataggio_modificato(|j| j["governo"]["presidente"] = 99.into());
    assert!(matches!(
        Partita::carica(&s),
        Err(Errore::SalvataggioIlleggibile(_))
    ));
}

#[test]
fn salvataggio_con_voto_precedente_o_posizione_guasti_illeggibile() {
    type Guasto = fn(&mut serde_json::Value);
    let casi: [(&str, Guasto); 4] = [
        ("voto_precedente corto", |j| {
            j["elettorato"]["gruppi"][2]["voto_precedente"]
                .as_array_mut()
                .unwrap()
                .pop();
        }),
        ("voto_precedente che non somma a 1", |j| {
            j["elettorato"]["gruppi"][2]["voto_precedente"][0] = 5.0.into();
        }),
        ("posizione negativa", |j| {
            j["elettorato"]["gruppi"][2]["posizione"][0] = (-0.5).into();
        }),
        ("posizione che non somma a 1", |j| {
            j["elettorato"]["gruppi"][2]["posizione"] = serde_json::json!(vec![0.0; 7]);
        }),
    ];
    for (nome, guasto) in casi {
        let e = Partita::carica(&salvataggio_modificato(guasto)).unwrap_err();
        assert!(
            matches!(e, Errore::SalvataggioIlleggibile(_)),
            "{nome}: {e:?}"
        );
    }
}

#[test]
fn salvataggio_con_ultima_disoccupazione_corta_illeggibile() {
    let s = salvataggio_modificato(|j| {
        j["elettorato"]["ultima_disoccupazione"]
            .as_array_mut()
            .unwrap()
            .pop();
    });
    let e = Partita::carica(&s).unwrap_err();
    assert!(
        matches!(&e, Errore::SalvataggioIlleggibile(m) if m.contains("ultima_disoccupazione")),
        "{e:?}"
    );
}

#[test]
fn salvataggio_con_elettorato_manomesso_illeggibile() {
    type Guasto = fn(&mut serde_json::Value);
    let casi: [(&str, Guasto); 7] = [
        ("quota negativa", |j| {
            j["elettorato"]["gruppi"][2]["quota"] = (-0.1).into();
        }),
        ("quote di una regione tutte a zero", |j| {
            for g in &mut j["elettorato"]["gruppi"].as_array_mut().unwrap()[..18] {
                g["quota"] = 0.0.into();
            }
        }),
        ("quota enorme", |j| {
            j["elettorato"]["gruppi"][0]["quota"] = 1e6.into();
        }),
        ("ancora tutta a zero", |j| {
            j["elettorato"]["gruppi"][2]["ancora"] = serde_json::json!(vec![0.0; 7]);
        }),
        ("voto_precedente più lungo", |j| {
            j["elettorato"]["gruppi"][2]["voto_precedente"]
                .as_array_mut()
                .unwrap()
                .push(0.0.into());
        }),
        ("reddito con paniere nullo", |j| {
            j["elettorato"]["gruppi"][2]["reddito"]["paniere"] = 0.0.into();
        }),
        ("giudizio null", |j| {
            j["elettorato"]["gruppi"][2]["giudizio"][0] = serde_json::Value::Null;
        }),
    ];
    for (nome, guasto) in casi {
        let e = Partita::carica(&salvataggio_modificato(guasto)).unwrap_err();
        assert!(
            matches!(e, Errore::SalvataggioIlleggibile(_)),
            "{nome}: {e:?}"
        );
    }
}

#[test]
fn salvataggio_con_informazione_guasta_illeggibile() {
    type Guasto = fn(&mut serde_json::Value);
    let casi: [(&str, Guasto); 7] = [
        ("bias corto", |j| {
            j["informazione"]["bias"].as_array_mut().unwrap().pop();
        }),
        ("effetti_istituto corti", |j| {
            j["informazione"]["effetti_istituto"]
                .as_array_mut()
                .unwrap()
                .pop();
        }),
        ("riga di effetti_istituto corta", |j| {
            j["informazione"]["effetti_istituto"][1]
                .as_array_mut()
                .unwrap()
                .pop();
        }),
        ("quote di un istituto corte", |j| {
            j["informazione"]["sondaggio"]["istituti"][0]["quote"]
                .as_array_mut()
                .unwrap()
                .pop();
        }),
        ("regioni del sondaggio in meno", |j| {
            j["informazione"]["sondaggio"]["regioni"]
                .as_array_mut()
                .unwrap()
                .pop();
        }),
        ("posizione di un incrocio corta", |j| {
            j["informazione"]["sondaggio"]["fasce"][2]["posizione"]
                .as_array_mut()
                .unwrap()
                .pop();
        }),
        ("trimestre della statistica fuori da 1-4", |j| {
            j["informazione"]["statistica"]["trimestre"]["trimestre"] = 9.into();
        }),
    ];
    for (nome, guasto) in casi {
        let e = Partita::carica(&salvataggio_modificato(guasto)).unwrap_err();
        assert!(
            matches!(e, Errore::SalvataggioIlleggibile(_)),
            "{nome}: {e:?}"
        );
    }
}
