mod comune;
use comune::{file_base, fondata, imposta};
use sim_core::legislatura::{Fase, Fondazione};
use sim_core::parlamento::{Decreto, ValoreProposto};
use sim_core::{Comando, Partita};

/// Proporzionale puro (`dati/base/elezioni.toml:307`): nessun premio, coalizioni facoltative.
fn proporzionale(f: sim_core::dati::FileDati, posto: f64) -> Partita {
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
            posto,
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
    if p.vista().incarico.as_deref() == Some("giocatore") {
        let fuori: Vec<String> = p
            .disponibilita()
            .into_iter()
            .filter(|v| v.disponibile)
            .map(|v| v.partito)
            .collect();
        for x in fuori {
            p.comanda(Comando::Allarga { partito: x }).unwrap();
        }
    }
    p.chiudi_turno();
    // Un governo c'è, del giocatore o del Capo dello Stato, oppure le Camere sono sciolte.
    let l = p.legislatura();
    assert!(l.fase == Fase::Ordinaria || l.fase == Fase::Sciolta);
}

#[test]
fn due_scioglimenti_di_fila() {
    // Proporzionale puro coi dati di base: il giocatore a 3,5 divide la fila, a sinistra e a
    // destra di lui nessuna maggioranza contigua, e da solo non è primo alla Camera. Col seme 7
    // le Camere si sciolgono a ogni trattativa. (Con le valence di sinistra, destra e
    // centrosinistra al minimo, come voleva il piano, il giocatore prende da solo la
    // maggioranza: 203 su 400 e 106 su 200.)
    let mut p = proporzionale(file_base(), 3.5);
    for _ in 0..6 {
        p.chiudi_turno();
    }
    assert!(p.vista().elezioni.len() >= 2);
    assert_eq!(p.legislatura().fase, Fase::Sciolta);
    assert!(Partita::carica(&p.salva().unwrap()).is_ok());
}

/// Sinistra e destra al minimo, proporzionale puro, posto 3,5: col seme 7 il giocatore è primo
/// alla Camera senza maggioranza (176 su 400, 90 su 200), quindi ha l'incarico.
fn primo_senza_maggioranza() -> Partita {
    let mut f = file_base();
    for id in ["sinistra", "destra"] {
        f = comune::sostituisci_valence(f, id, -3.0);
    }
    let mut p = proporzionale(f, 3.5);
    p.chiudi_turno();
    assert_eq!(p.vista().incarico.as_deref(), Some("giocatore"));
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
        partito: "centrodestra".into(),
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
    let prima = p.salva().unwrap();
    for (id, testo) in [
        (
            "estrema_sinistra",
            "Estrema sinistra non entra: troppo lontano dalla tua posizione",
        ),
        ("ignoto", "Partito sconosciuto: ignoto"),
        ("giocatore", "Partito nuovo è già nella coalizione"),
    ] {
        let e = p
            .comanda(Comando::Allarga { partito: id.into() })
            .unwrap_err();
        assert!(e.to_string().contains(testo), "{e}");
    }
    assert_eq!(p.salva().unwrap(), prima);
    let allarga = |p: &mut Partita| {
        p.comanda(Comando::Allarga {
            partito: "centro".into(),
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
    // Destra, 3 seggi, non porta la maggioranza e non è disponibile: entra dal salvataggio.
    let p = primo_senza_maggioranza();
    let mut p = comune::edita(&p, |j| {
        j["legislatura"]["allargati"] = serde_json::json!([5])
    })
    .unwrap();
    p.chiudi_turno();
    let l = p.legislatura();
    assert_eq!(l.fase, Fase::Sciolta);
    assert!(l.allargati.is_empty() && l.incarico.is_none() && l.dichiarazione.is_none());
    assert!(!p.governa());
    assert_eq!(p.vista().turni_al_voto, 0);
    let e = p
        .comanda(Comando::Allarga {
            partito: "centro".into(),
        })
        .unwrap_err();
    assert!(
        e.to_string().contains("solo in trattativa, con l'incarico"),
        "{e}"
    );
    p.chiudi_turno();
    assert_eq!(p.vista().elezioni.len(), 2);
    assert_eq!(p.legislatura().fase, Fase::Trattativa);
}

#[test]
fn senza_incarico_decide_il_capo_dello_stato() {
    let mut p = fondata(1.5);
    p.chiudi_turno();
    let v = p.vista();
    assert_eq!(v.incarico.as_deref(), Some("Capo dello Stato"));
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
}
