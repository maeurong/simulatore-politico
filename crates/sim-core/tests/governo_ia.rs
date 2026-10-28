#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

mod comune;
use comune::{file_base, fondata, imposta};
use serde_json::json;
use sim_core::legislatura::{Assegnazione, Fase, Fondazione, Incarico};
use sim_core::parlamento::{CausaIntesa, Decreto, TipoReferendum, ValoreProposto};
use sim_core::vista::VoceIncarico;
use sim_core::{Comando, Partita};

/// Proporzionale puro (`dati/base/elezioni.toml:307`): nessun premio, coalizioni facoltative.
fn proporzionale(f: sim_core::dati::FileDati, posizione: f64) -> Partita {
    let f = imposta(
        f,
        "scenario",
        "legge_elettorale = \"",
        "proporzionale_puro\"",
    );
    Partita::fonda(
        sim_core::dati::DatiGioco::da_testi(&f).unwrap(),
        7,
        &Fondazione {
            nome: "Partito nuovo".into(),
            posizione,
        },
    )
    .unwrap()
}

#[test]
fn chi_perde_il_voto_va_all_opposizione() {
    let mut p = fondata(1.5); // da solo all'estrema sinistra
    p.chiudi_turno(); // voto
    p.chiudi_turno(); // trattativa
    assert_eq!(p.legislatura().fase, Fase::Ordinaria);
    assert!(!p.governa());
    assert!(!p.governo().al_governo(p.giocatore()));
    let e = p
        .comanda(Comando::Ferma {
            dicastero: "sanita".into(),
        })
        .unwrap_err();
    assert!(e.to_string().contains("all'opposizione"), "{e}");
}

#[test]
fn senza_premio_si_allarga() {
    let mut p = proporzionale(file_base(), 3.5);
    p.chiudi_turno();
    if p.vista().incarico == Some(VoceIncarico::Giocatore) {
        let fuori: Vec<String> = p
            .disponibilita()
            .into_iter()
            .filter(|v| v.disponibile)
            .map(|v| v.partito)
            .collect();
        for x in fuori {
            p.comanda(Comando::Allarga { partiti: vec![x] }).unwrap();
        }
    }
    p.chiudi_turno();
    // Un governo c'è, del giocatore o del Capo dello Stato, oppure le Camere sono sciolte.
    let l = p.legislatura();
    assert!(l.fase == Fase::Ordinaria || l.fase == Fase::Sciolta);
}

/// In trattativa, Camere alla pari con l'`incarico` dato: il giocatore con metà esatta di
/// entrambe le Camere, gli altri con l'altra metà e la destra senza seggi. Nessuna maggioranza,
/// né con lui né senza; i seggi sommano ai totali, 400 e 200.
fn alla_pari(p: &Partita, incarico: serde_json::Value) -> Partita {
    comune::edita(p, |j| {
        assert_eq!(j["legislatura"]["totali"], json!([400, 200]));
        j["legislatura"]["incarico"] = incarico;
        j["parlamento"]["seggi_camera"] = json!([40, 40, 30, 30, 30, 0, 30, 200]);
        j["parlamento"]["seggi_senato"] = json!([20, 20, 15, 15, 15, 0, 15, 100]);
    })
    .unwrap()
}

/// Al Capo dello Stato, con le Camere alla pari, nessuna maggioranza contigua senza il
/// giocatore: si sciolgono dopo il voto del turno zero e di nuovo dopo il voto a Camere sciolte.
#[test]
fn due_scioglimenti_di_fila() {
    let mut p = fondata(1.5);
    p.chiudi_turno(); // voto del turno zero
    for voti in [1, 2] {
        assert_eq!(p.legislatura().fase, Fase::Trattativa);
        assert_eq!(p.vista().elezioni.len(), voti);
        p = alla_pari(&p, serde_json::Value::Null);
        p.chiudi_turno(); // trattativa
        assert_eq!(p.legislatura().fase, Fase::Sciolta);
        p = Partita::carica(&p.salva().unwrap()).unwrap();
        p.chiudi_turno(); // voto a Camere sciolte
    }
    assert_eq!(p.vista().elezioni.len(), 3);
    assert!(Partita::carica(&p.salva().unwrap()).is_ok());
}

/// Sciolte dopo il voto del turno zero, al turno 2.
fn sciolta() -> Partita {
    let mut p = fondata(1.5);
    p.chiudi_turno();
    let mut p = alla_pari(&p, serde_json::Value::Null);
    p.chiudi_turno();
    assert_eq!(p.legislatura().fase, Fase::Sciolta);
    p
}

/// A Camere sciolte nessun abrogativo si chiede (in trattativa nemmeno: tests/debiti_7a.rs).
#[test]
fn nessun_referendum_a_camere_sciolte() {
    let mut p = sciolta();
    let prima = p.salva().unwrap();
    let e = (p.promuovi_referendum("sinistra", "licenziamento")).unwrap_err();
    assert!(
        e.to_string().contains("Nessun referendum a Camere sciolte"),
        "{e}"
    );
    assert_eq!(p.salva().unwrap(), prima);
}

/// Un confermativo col voto nel turno sciolto si vota in quel turno, non slitta.
#[test]
fn a_camere_sciolte_il_confermativo_si_vota() {
    let p = sciolta();
    let durata = comune::dati_base()
        .disposizione("durata_legislatura")
        .unwrap();
    let mut p = comune::edita(&p, |j| {
        j["parlamento"]["confermativi"] = json!([{
            "tipo": "confermativo",
            "misura": 99,
            "valori": [[durata, 0]],
            "promotore": null,
            "voto": 2,
            "firme": true,
            "invito_astensione": false,
            "del_giocatore": false,
        }]);
    })
    .unwrap();
    assert_eq!(p.vista().turno, 2);
    p.chiudi_turno();
    assert!(p.parlamento().confermativi.is_empty());
    assert!((p.vista().esiti_referendum.iter()).any(|r| r.tipo == TipoReferendum::Confermativo));
}

/// Sotto un governo IA la vista non mostra alleati.
#[test]
fn sotto_un_governo_ia_nessun_alleato() {
    let mut p = fondata(1.5);
    p.chiudi_turno();
    p.chiudi_turno();
    assert!(!p.governa() && p.legislatura().fase == Fase::Ordinaria);
    assert!(p.vista().alleati.is_empty());
}

/// Sinistra e destra al minimo, proporzionale puro, posizione 3,5: col seme 7 il giocatore è
/// primo alla Camera senza maggioranza (176 su 400, 90 su 200), quindi ha l'incarico.
fn primo_senza_maggioranza() -> Partita {
    let mut f = file_base();
    for id in ["sinistra", "destra"] {
        f = comune::sostituisci_valence(f, id, -3.0);
    }
    let mut p = proporzionale(f, 3.5);
    p.chiudi_turno();
    assert_eq!(p.vista().incarico, Some(VoceIncarico::Giocatore));
    p
}

#[test]
fn allargata_alla_maggioranza_la_coalizione_governa() {
    let mut p = primo_senza_maggioranza();
    let disponibile = |p: &Partita, id: &str| {
        (p.vista().disponibilita.iter())
            .find(|v| v.partito == id)
            .map(|v| v.disponibile)
    };
    assert_eq!(disponibile(&p, "centrodestra"), Some(true));
    p.comanda(Comando::Allarga {
        partiti: vec!["centrodestra".into()],
    })
    .unwrap();
    assert_eq!(p.legislatura().allargati, vec![4]);
    assert_eq!(disponibile(&p, "centrodestra"), None);
    assert!(p.richieste().iter().any(|r| r.partito == "centrodestra"));
    assert!(Partita::carica(&p.salva().unwrap()).is_ok());
    p.chiudi_turno();
    assert!(p.governa());
    assert!(p.governo().al_governo(4));
    assert!(p.legislatura().allargati.is_empty());
}

/// Ogni dicastero al giocatore.
fn tutto_a_me(p: &Partita) -> Vec<Assegnazione> {
    (p.vista().dicasteri.into_iter())
        .map(|d| Assegnazione {
            dicastero: d.id,
            partito: Some("giocatore".into()),
        })
        .collect()
}

/// Una proposta fatta prima di allargare decade: l'allargato entra con la spartizione di default,
/// non con zero dicasteri e l'intesa già sotto l'ultimatum.
#[test]
fn allargare_annulla_la_proposta() {
    let mut p = primo_senza_maggioranza();
    let dicasteri = tutto_a_me(&p);
    p.comanda(Comando::ProponiSpartizione { dicasteri })
        .unwrap();
    p.comanda(Comando::Allarga {
        partiti: vec!["centrodestra".into()],
    })
    .unwrap();
    assert!(p.legislatura().proposta.is_none());
    p.chiudi_turno();
    let cd = (p.vista().alleati.into_iter())
        .find(|a| a.partito == "centrodestra")
        .unwrap();
    assert!(!cd.ultimatum, "{cd:?}");
    assert!(p.governo().dicasteri.contains(&4));
}

/// Una coalizione IA con la maggioranza al voto governa anche se non è contigua: estrema sinistra
/// ed estrema destra, con 250 seggi su 400 e 125 su 200.
#[test]
fn l_incarico_ia_del_voto_vince_sulla_contigua() {
    let mut p = fondata(1.5);
    p.chiudi_turno();
    let mut p = comune::edita(&p, |j| {
        j["legislatura"]["incarico"] = serde_json::json!({ "Ia": { "coalizione": [0, 6] } });
        j["parlamento"]["seggi_camera"] = serde_json::json!([150, 20, 20, 20, 20, 20, 100, 50]);
        j["parlamento"]["seggi_senato"] = serde_json::json!([75, 10, 10, 10, 10, 10, 50, 25]);
    })
    .unwrap();
    assert_eq!(
        p.legislatura().incarico,
        Some(Incarico::Ia {
            coalizione: vec![0, 6]
        })
    );
    p.chiudi_turno();
    assert_eq!(p.legislatura().fase, Fase::Ordinaria);
    assert_eq!(p.governo().coalizione, vec![0, 6]);
}

/// Coi seggi veri del voto, non quelli proiettati, `P` vale 1 alla maggioranza esatta, non la
/// metà della logistica: il centrodestra, a 1,5 dal giocatore, entra se con lui la coalizione ha
/// 201 seggi su 400 e 101 su 200; con un seggio in meno alla Camera no. I seggi tolti al
/// giocatore vanno all'estrema destra.
#[test]
fn alla_maggioranza_esatta_governare_vale_uno() {
    for (camera, disponibile) in [(201, true), (200, false)] {
        let p = primo_senza_maggioranza();
        let p = comune::edita(&p, |j| {
            for (ramo, soglia) in [("seggi_camera", camera), ("seggi_senato", 101)] {
                let s = &mut j["parlamento"][ramo];
                let (giocatore, cd) = (s[7].as_u64().unwrap(), s[4].as_u64().unwrap());
                let tolti = giocatore - (soglia - cd);
                s[7] = (soglia - cd).into();
                s[6] = (s[6].as_u64().unwrap() + tolti).into();
            }
        })
        .unwrap();
        let v = p.disponibilita();
        let cd = v.iter().find(|v| v.partito == "centrodestra").unwrap();
        assert_eq!(cd.disponibile, disponibile, "{camera}: {cd:?}");
    }
}

#[test]
fn allargamenti_rifiutati_lasciano_la_partita_identica() {
    let mut p = primo_senza_maggioranza();
    let dicasteri = tutto_a_me(&p);
    p.comanda(Comando::ProponiSpartizione { dicasteri })
        .unwrap();
    let prima = p.salva().unwrap();
    for (id, testo) in [
        (
            "estrema_sinistra",
            "Estrema sinistra non entra: posizione troppo lontana dalla tua",
        ),
        ("ignoto", "Partito sconosciuto: ignoto"),
        ("giocatore", "Partito nuovo è già nella coalizione"),
    ] {
        let e = p
            .comanda(Comando::Allarga {
                partiti: vec![id.into()],
            })
            .unwrap_err();
        assert!(e.to_string().contains(testo), "{e}");
    }
    assert_eq!(p.salva().unwrap(), prima);
    assert!(p.legislatura().proposta.is_some());
    let allarga = |p: &mut Partita| {
        p.comanda(Comando::Allarga {
            partiti: vec!["centro".into()],
        })
    };
    allarga(&mut p).unwrap();
    let e = allarga(&mut p).unwrap_err();
    assert!(
        e.to_string().contains("Centro è già nella coalizione"),
        "{e}"
    );
}

#[test]
fn senza_maggioranza_le_camere_si_sciolgono() {
    // Camere alla pari con l'incarico al giocatore. Destra, senza seggi, non porta la
    // maggioranza e non è disponibile: entra dal salvataggio.
    let p = alla_pari(&primo_senza_maggioranza(), json!("Giocatore"));
    let mut p = comune::edita(&p, |j| {
        j["legislatura"]["allargati"] = serde_json::json!([5])
    })
    .unwrap();
    let dicasteri = tutto_a_me(&p);
    p.comanda(Comando::ProponiSpartizione { dicasteri })
        .unwrap();
    p.chiudi_turno();
    let l = p.legislatura();
    assert_eq!(l.fase, Fase::Sciolta);
    assert!(l.allargati.is_empty() && l.incarico.is_none() && l.dichiarazione.is_none());
    assert!(l.proposta.is_none());
    assert!(Partita::carica(&p.salva().unwrap()).is_ok());
    assert!(!p.governa());
    assert_eq!(p.vista().turni_al_voto, 0);
    for (id, nome) in [("centro", "Centro"), ("ignoto", "ignoto")] {
        let e = p
            .comanda(Comando::Allarga {
                partiti: vec![id.into()],
            })
            .unwrap_err();
        let atteso = format!("{nome}: allargamento solo in trattativa con l'incarico");
        assert!(e.to_string().contains(&atteso), "{e}");
    }
    p.chiudi_turno();
    assert_eq!(p.vista().elezioni.len(), 2);
    assert_eq!(p.legislatura().fase, Fase::Trattativa);
}

#[test]
fn senza_incarico_decide_il_capo_dello_stato() {
    let mut p = fondata(1.5);
    p.chiudi_turno();
    let v = p.vista();
    assert_eq!(v.incarico, Some(VoceIncarico::CapoDelloStato));
    assert!(v.disponibilita.is_empty() && v.richieste.is_empty());
    p.chiudi_turno();
    assert_eq!(p.vista().incarico, None);
    let c = &p.governo().coalizione;
    assert!(!c.is_empty() && !c.contains(&p.giocatore()));
}

#[test]
fn carica_rifiuta_allargati_non_validi() {
    let p = primo_senza_maggioranza();
    for v in [vec![99], vec![5, 5], vec![7]] {
        let r = comune::edita(&p, |j| j["legislatura"]["allargati"] = serde_json::json!(v));
        assert!(r.is_err(), "{v:?}");
    }
    let mut q = fondata(1.5);
    q.chiudi_turno();
    q.chiudi_turno();
    let r = comune::edita(&q, |j| {
        j["legislatura"]["allargati"] = serde_json::json!([3])
    });
    assert!(r.is_err());
}

#[test]
fn il_governo_ia_presenta_il_bilancio() {
    let mut p = fondata(1.5);
    for _ in 0..4 {
        p.chiudi_turno(); // voto, trattativa, aprile-giugno, luglio-settembre 2027
    }
    // Ottobre-dicembre 2027: il bilancio del 2028 lo presenta il governo IA.
    p.chiudi_turno();
    assert!(
        p.vista()
            .esiti
            .iter()
            .any(|e| e.forma == sim_core::parlamento::FormaMisura::Bilancio)
    );
}

/// Rimando del Task 5: all'opposizione le anteprime rifiutano come i comandi.
#[test]
fn all_opposizione_le_anteprime_rifiutano_come_i_comandi() {
    let mut p = fondata(1.5);
    p.chiudi_turno();
    p.chiudi_turno();
    assert!(!p.governa() && p.legislatura().fase == Fase::Ordinaria);
    let valori = vec![ValoreProposto {
        disposizione: "privato_accreditato".into(),
        gradino: "ampio".into(),
    }];
    let decreto = Decreto::Fondi {
        da: "sanita".into(),
        a: "istruzione".into(),
        importo: 1,
    };
    let comando = |p: &mut Partita, c| p.comanda(c).unwrap_err().to_string();
    let legge = comando(
        &mut p,
        Comando::PresentaLegge {
            valori: valori.clone(),
        },
    );
    assert!(legge.contains("all'opposizione"), "{legge}");
    let e = p.anteprima(&valori).unwrap_err().to_string();
    assert_eq!(e, legge);
    let e = p.anteprima_decreto(&decreto).unwrap_err().to_string();
    assert_eq!(
        e,
        comando(
            &mut p,
            Comando::EmanaDecreto {
                decreto: decreto.clone()
            }
        )
    );
    let e = p.anteprima_accogli("sanita").unwrap_err().to_string();
    assert_eq!(
        e,
        comando(
            &mut p,
            Comando::Accogli {
                dicastero: "sanita".into()
            }
        )
    );
    let bilancio = p.economia().bilancio.clone();
    let e = p.riepilogo_bilancio(&bilancio).unwrap_err().to_string();
    assert_eq!(e, comando(&mut p, Comando::PresentaBilancio { bilancio }));
}

#[test]
fn gli_alleati_del_governo_ia_partono_dalla_base() {
    let mut p = fondata(1.5);
    p.chiudi_turno(); // voto
    let n = p.partiti().len();
    let mut p = comune::edita(&p, |j| j["intesa"]["valori"] = json!(vec![30.0; n])).unwrap();
    p.chiudi_turno(); // trattativa: governo IA
    let dati = comune::dati_base();
    let base = dati.manopola("intesa_base");
    assert!(!p.alleati().is_empty());
    // Chi rompe l'impegno pre-elettorale parte da `intesa_base − intesa_rottura_impegno`.
    for j in p.alleati() {
        let rompe = (p.intesa().variazioni.iter())
            .any(|v| v.partito == j && v.causa == CausaIntesa::RotturaImpegno);
        let attesa = base
            - if rompe {
                dati.manopola("intesa_rottura_impegno")
            } else {
                0.0
            };
        assert!((p.intesa().valori[j] - attesa).abs() < 1e-9, "{j}");
    }
}

/// Il giocatore (7), che ha corso da solo, non ha la maggioranza; l'incarico va all'IA con
/// centrosinistra (2, che lascia l'1), centro (3, da solo, governa con 2 e 4) e centrodestra (4, che
/// lascia il 5): rompono tutti e tre.
#[test]
fn l_ia_che_rompe_l_impegno_perde_valence_e_intesa() {
    let p = comune::in_trattativa(
        json!([[7], [1, 2], [4, 5]]),
        &[10, 60, 80, 30, 50, 50, 20, 100],
        &[5, 30, 40, 15, 25, 25, 10, 50],
    );
    let mut p = comune::edita(&p, |j| {
        j["legislatura"]["incarico"] = json!({ "Ia": { "coalizione": [2, 3, 4] } })
    })
    .unwrap();
    let prima = p.partiti().to_vec();
    p.chiudi_turno();
    assert_eq!(p.governo().coalizione, vec![2, 3, 4]);
    let dati = comune::dati_base();
    let valence = dati.manopola("valence_rottura_impegno");
    for j in [2, 3, 4] {
        let calo = comune::calo_oltre_il_ritorno(&prima, &p, j);
        assert!((calo - valence).abs() < 1e-9, "{j}: {calo}");
    }
    let attesa = dati.manopola("intesa_base") - dati.manopola("intesa_rottura_impegno");
    assert_eq!(p.alleati().len(), 2);
    for j in p.alleati() {
        assert!((p.intesa().valori[j] - attesa).abs() < 1e-9, "{j}");
    }
}

#[test]
fn il_governo_ia_nasce_col_programma_pubblico() {
    let p = comune::all_opposizione();
    let v = p.vista();
    assert!(!v.programma.is_empty());
    // In ordine di distanza decrescente del vigente dalla posizione della coalizione.
    let g = p.governo();
    let seggi: Vec<f64> = (0..p.partiti().len())
        .map(|j| f64::from(p.parlamento().seggi_camera[j] + p.parlamento().seggi_senato[j]))
        .collect();
    let pos = g
        .coalizione
        .iter()
        .map(|&j| seggi[j] * p.partiti()[j].posizione_dichiarata)
        .sum::<f64>()
        / g.coalizione.iter().map(|&j| seggi[j]).sum::<f64>();
    let dati = comune::dati_base();
    let d = dati.disposizioni();
    let distanze: Vec<f64> = (g.programma.iter())
        .filter_map(|&(k, _)| d[k].posizione(p.disposizioni().voci[k].in_vigore))
        .map(|x| (x - pos).abs())
        .collect();
    assert!(distanze.windows(2).all(|w| w[0] >= w[1]), "{distanze:?}");
    assert!(
        v.mosse
            .iter()
            .any(|m| m.testo.starts_with("Programma di governo"))
    );
}

#[test]
fn col_giocatore_al_governo_nessun_programma() {
    let p = comune::al_governo();
    assert!(p.governo().programma.is_empty());
    assert!(p.vista().programma.is_empty());
}
