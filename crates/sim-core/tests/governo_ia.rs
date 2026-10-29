#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

mod comune;
use comune::{all_opposizione, edita, file_base, fondata, imposta};
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
            "richiesto": true,
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
    // Ottobre-dicembre 2027: il bilancio del 2028 lo presenta il governo IA in apertura, e la
    // chiusura lo vota.
    assert_eq!(p.vista().data.trimestre, 4);
    assert!(p.turno_del_bilancio());
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
    // In minoranza il governo IA pone la fiducia sulle conversioni già in apertura: costa a parte.
    for j in p.alleati() {
        let fiducia: f64 = (p.intesa().variazioni.iter())
            .filter(|v| v.partito == j && v.causa == CausaIntesa::Fiducia)
            .map(|v| v.delta)
            .sum();
        assert!(
            (p.intesa().valori[j] - fiducia - attesa).abs() < 1e-9,
            "{j}"
        );
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

/// Dopo la nascita il governo IA porta in aula le prime voci del programma, fino a tre; le
/// riforme accolte dai ministri vengono dopo e non contano.
#[test]
fn il_programma_va_in_aula_in_ordine() {
    let p = all_opposizione();
    let programma = &p.governo().programma;
    let in_aula: Vec<usize> = (p.parlamento().misure.iter())
        .filter(|m| m.promotore.is_none() && m.valori.len() == 1)
        .map(|m| m.valori[0].0)
        .filter(|k| programma.iter().any(|v| v.0 == *k))
        .collect();
    let attese: Vec<usize> = (p.governo().programma.iter())
        .map(|v| v.0)
        .take(in_aula.len())
        .collect();
    assert!(!in_aula.is_empty() && in_aula.len() <= 3);
    assert_eq!(in_aula, attese);
    assert!(
        p.vista()
            .mosse
            .iter()
            .any(|m| m.testo.starts_with("Porta in aula: "))
    );
}

/// Il programma ridotto alla prima voce, scritta al gradino già in vigore: il governo la scarta
/// prima di presentarla (non è «non ancora in vigore»), e non porta nulla in aula né lascia righe.
#[test]
fn programma_esaurito_non_presenta_nulla() {
    let p = all_opposizione();
    let k = p.governo().programma[0].0;
    let vigente = p.disposizioni().voci[k].in_vigore;
    let mut p = edita(&p, |j| {
        j["governo"]["programma"] = serde_json::json!([[k, vigente]]);
        // Fuori la sola legge della voce: le conversioni dei decreti accolti restano.
        let misure: Vec<serde_json::Value> = (j["parlamento"]["misure"].as_array().unwrap())
            .iter()
            .filter(|m| m["valori"].as_array().unwrap().iter().all(|v| v[0] != k))
            .cloned()
            .collect();
        j["parlamento"]["misure"] = serde_json::json!(misure);
    })
    .unwrap();
    p.chiudi_turno();
    assert!(
        p.vista()
            .mosse
            .iter()
            .all(|m| !m.testo.starts_with("Porta in aula"))
    );
    assert!((p.parlamento().misure.iter()).all(|m| m.valori.iter().all(|v| v.0 != k)));
}

#[test]
fn il_bilancio_ia_sta_nel_tetto_senza_ultimatum() {
    // Avanza al primo ottobre-dicembre sotto il governo IA; all'apertura del turno prima niente
    // ultimatum e intesa piena, così la chiusura non ne pone.
    let mut p = all_opposizione();
    while p.vista().data.trimestre != 3 {
        p.chiudi_turno();
    }
    let n = p.partiti().len();
    let mut p = edita(&p, |j| {
        j["intesa"]["ultimatum"] = json!(vec![serde_json::Value::Null; n]);
        j["intesa"]["valori"] = json!(vec![100.0; n]);
    })
    .unwrap();
    p.chiudi_turno();
    assert_eq!(p.vista().data.trimestre, 4);
    assert!(p.intesa().ultimatum.iter().all(Option::is_none));
    let r = p
        .riepilogo_presentato()
        .expect("bilancio IA presentato in apertura");
    assert!(r.crescita_spesa_netta <= r.tetto + 1e-6, "{r:?}");
    assert!(
        p.vista()
            .mosse
            .iter()
            .any(|m| m.testo.starts_with("Bilancio "))
    );
}

#[test]
fn lo_stato_ia_sopravvive_al_salvataggio() {
    let mut p = all_opposizione();
    p.chiudi_turno();
    let q = Partita::carica(&p.salva().unwrap()).unwrap();
    assert_eq!(p, q);
    let fuori = edita(&p, |j| {
        j["governo"]["programma"] = serde_json::json!([[999, 0]])
    });
    assert!(fuori.is_err());
}

/// Il primo alleato del governo senza il quale la coalizione perde la maggioranza in una Camera.
fn decisivo(p: &Partita) -> Option<usize> {
    let (pl, g) = (p.parlamento(), p.governo());
    let oltre = |seggi: &[u32], k: &[usize]| {
        2 * k.iter().map(|&j| seggi[j]).sum::<u32>() > seggi.iter().sum::<u32>()
    };
    (g.coalizione.iter().copied())
        .filter(|&a| a != g.presidente)
        .find(|&a| {
            let senza: Vec<usize> = g.coalizione.iter().copied().filter(|&j| j != a).collect();
            !(oltre(&pl.seggi_camera, &senza) && oltre(&pl.seggi_senato, &senza))
        })
}

/// L'alleato del governo IA senza il quale la coalizione perde la maggioranza esce con `intesa`,
/// sotto 25: il turno dopo è di crisi.
fn in_crisi(p: Partita, intesa: f64) -> Partita {
    let alleato = decisivo(&p).expect("un alleato decisivo nel governo IA");
    let mut p = edita(&p, |j| j["intesa"]["valori"][alleato] = intesa.into()).unwrap();
    p.chiudi_turno();
    assert!(p.vista().crisi);
    p
}

const SI_DIMETTE: &str =
    "Si dimette: nessun partito contiguo gli ridà la maggioranza con un guadagno";
const SI_DIMETTE_ROTTURA: &str =
    "Si dimette: il solo partito contiguo che gli ridà la maggioranza ha rotto con lui";
const SI_DIMETTE_IN_PROVVISORIO: &str =
    "Si dimette: in esercizio provvisorio non allarga la coalizione";

#[test]
fn in_crisi_il_governo_ia_non_riprende_chi_e_uscito() {
    // 24: col ritorno dell'intesa l'uscente risale sopra 25 prima della crisi, non sopra 40.
    for intesa in [10.0, 24.0] {
        let p = all_opposizione();
        let uscente = decisivo(&p).unwrap();
        let p = in_crisi(p, intesa);
        let l = p.legislatura();
        assert!(!l.allargati.contains(&uscente), "{intesa}");
        assert!(!l.allargati.contains(&p.giocatore()));
        // L'uscente (Destra) è l'unico vicino con seggi e ridarebbe la maggioranza: ha rotto.
        assert!(l.dimissioni);
        let mosse = &p.vista().mosse;
        assert!(mosse.iter().any(|m| m.testo == SI_DIMETTE_ROTTURA));
        assert!(mosse.iter().all(|m| m.testo != SI_DIMETTE));
    }
}

/// Il giocatore tiene quasi tutti i seggi: nemmeno col vicino la coalizione IA ha la maggioranza.
#[test]
fn senza_vicini_il_motivo_resta_nessun_partito() {
    let p = all_opposizione();
    let (n, g) = (p.partiti().len(), p.giocatore());
    let ripartisci = |totale: u32, altri: u32| -> Vec<u32> {
        let resto = totale - altri * (n as u32 - 1);
        (0..n).map(|j| if j == g { resto } else { altri }).collect()
    };
    let p = comune::con_seggi(&p, &ripartisci(400, 10), &ripartisci(200, 5));
    let p = edita(&p, |j| j["intesa"]["valori"] = json!(vec![100.0; n])).unwrap();
    let p = in_crisi(p, 10.0);
    assert!(p.legislatura().dimissioni);
    let mosse = &p.vista().mosse;
    assert!(mosse.iter().any(|m| m.testo == SI_DIMETTE));
    assert!(mosse.iter().all(|m| m.testo != SI_DIMETTE_ROTTURA));
}

/// L'estrema sinistra (0) con seggi tolti al giocatore, l'intesa alla base e vocazione governista
/// (con la sua, identitaria, la distanza le toglierebbe il guadagno): uscita Destra, è il vicino
/// che ridà la maggioranza al governo IA e ci guadagna.
fn con_vicino(p: &Partita) -> Partita {
    let camera = [10, 125, 26, 18, 33, 125, 30, 33];
    let senato = [5, 69, 10, 7, 13, 65, 13, 18];
    let p = comune::con_seggi(p, &camera, &senato);
    edita(&p, |j| {
        j["intesa"]["valori"][0] = 60.0.into();
        j["partiti"][0]["vocazione"] = "governista".into();
    })
    .unwrap()
}

#[test]
fn il_governo_ia_in_crisi_allarga_ma_non_in_esercizio_provvisorio() {
    let mut p = all_opposizione();
    while p.vista().data.trimestre != 3 {
        p.chiudi_turno();
    }
    // La crisi cade in ottobre-dicembre: ordinaria amministrazione, niente bilancio; allarga.
    let mut q = in_crisi(con_vicino(&p), 10.0);
    assert_eq!(q.vista().data.trimestre, 4);
    assert!(q.economia().bilancio_presentato.is_none());
    assert_eq!(q.legislatura().allargati, [0]);
    q.chiudi_turno();
    assert!(q.salva().is_ok());
    // Ottobre-dicembre senza bilancio: la crisi cade a gennaio, in esercizio provvisorio, e il
    // governo cade comunque a fine turno; niente allargamento.
    p.chiudi_turno();
    assert_eq!(p.vista().data.trimestre, 4);
    let p = edita(&p, |j| {
        j["economia"]["bilancio_presentato"] = json!(null);
        let misure = j["parlamento"]["misure"].as_array_mut().unwrap();
        misure.retain(|m| m["forma"] != "bilancio");
    })
    .unwrap();
    let p = in_crisi(con_vicino(&p), 10.0);
    assert_eq!(p.vista().data.trimestre, 1);
    assert!(p.economia().bilancio_presentato.is_none());
    let l = p.legislatura();
    assert!(l.allargati.is_empty() && l.dimissioni);
    assert!(
        p.vista()
            .mosse
            .iter()
            .any(|m| m.testo == SI_DIMETTE_IN_PROVVISORIO)
    );
}

#[test]
fn col_voto_anticipato_il_capo_dello_stato_esclude_chi_si_dimette() {
    let p = all_opposizione();
    let pdc = p.governo().presidente;
    let mut p = edita(&p, |j| {
        j["legislatura"]["dimissioni"] = true.into();
        j["legislatura"]["voto_anticipato"] = true.into();
    })
    .unwrap();
    p.chiudi_turno();
    assert!(!p.legislatura().voto_anticipato);
    match &p.legislatura().incarico {
        Some(Incarico::Ia { coalizione } | Incarico::Guida { coalizione }) => {
            assert!(!coalizione.contains(&pdc));
        }
        Some(Incarico::Giocatore) => {
            assert!(!p.legislatura().maggioranza.as_ref().unwrap().contains(&pdc));
        }
        None => assert_eq!(p.legislatura().fase, Fase::Sciolta),
    }
}

/// Le guardie di `carica` sullo stato del governo IA. In crisi (turno 2, aprile-giugno) gli
/// allargati validi sono un solo partito fuori dalla coalizione e non il giocatore.
#[test]
fn carica_rifiuta_lo_stato_ia_non_valido() {
    let p = all_opposizione();
    let k = p.governo().programma[0].0;
    assert!(edita(&p, |j| j["governo"]["programma"] = json!([[k, 999]])).is_err());
    let g = comune::al_governo();
    assert!(edita(&g, |j| j["governo"]["programma"] = json!([[k, 0]])).is_err());
    let fuori: Vec<usize> = (0..p.partiti().len())
        .filter(|j| !p.governo().coalizione.contains(j) && *j != p.giocatore())
        .collect();
    let in_crisi = |allargati: serde_json::Value| {
        edita(&p, |j| {
            j["legislatura"]["crisi"] = j["turno"].clone();
            j["legislatura"]["allargati"] = allargati;
        })
    };
    assert!(in_crisi(json!([fuori[0]])).is_ok());
    assert!(in_crisi(json!([p.giocatore()])).is_err());
    assert!(in_crisi(json!([p.governo().coalizione[0]])).is_err());
    assert!(in_crisi(json!([fuori[0], fuori[1]])).is_err());
}

/// Con la Sanità che chiede fondi, il governo IA accoglie la proposta in apertura.
#[test]
fn il_governo_ia_accoglie_la_proposta_del_dicastero() {
    let dati = sim_core::dati::DatiGioco::da_testi(&comune::sanita_cara(file_base())).unwrap();
    let f = Fondazione {
        nome: "Partito nuovo".into(),
        posizione: 1.5,
    };
    let mut p = Partita::fonda(dati, 7, &f).unwrap();
    p.chiudi_turno(); // voto
    p.chiudi_turno(); // trattativa: governo IA
    assert!(!p.governa());
    let v = p.vista();
    let sanita = v
        .ministeri
        .iter()
        .find(|m| m.dicastero == "sanita")
        .unwrap();
    assert!(sanita.proposta.is_none());
    assert!(
        v.mosse
            .iter()
            .any(|m| m.testo == "Accoglie la proposta del dicastero Sanità"),
        "{:?}",
        v.mosse
    );
}

/// Ottobre-dicembre senza bilancio, fuori dalla crisi: a gennaio, in esercizio provvisorio, il
/// governo IA lo presenta in apertura.
#[test]
fn il_governo_ia_presenta_il_bilancio_in_esercizio_provvisorio() {
    let mut p = all_opposizione();
    while p.vista().data.trimestre != 4 {
        p.chiudi_turno();
    }
    let n = p.partiti().len();
    let mut p = edita(&p, |j| {
        j["economia"]["bilancio_presentato"] = json!(null);
        let misure = j["parlamento"]["misure"].as_array_mut().unwrap();
        misure.retain(|m| m["forma"] != "bilancio");
        j["intesa"]["ultimatum"] = json!(vec![serde_json::Value::Null; n]);
        j["intesa"]["valori"] = json!(vec![100.0; n]);
    })
    .unwrap();
    p.chiudi_turno();
    let v = p.vista();
    assert_eq!(v.data.trimestre, 1);
    assert!(v.bilancio.esercizio_provvisorio && !v.crisi);
    assert!(p.economia().bilancio_presentato.is_some());
    assert!(v.mosse.iter().any(|m| m.testo.starts_with("Bilancio ")));
}

#[test]
fn voto_anticipato_solo_con_le_dimissioni() {
    let p = all_opposizione();
    let e = edita(&p, |j| j["legislatura"]["voto_anticipato"] = true.into());
    assert!(e.is_err());
}
