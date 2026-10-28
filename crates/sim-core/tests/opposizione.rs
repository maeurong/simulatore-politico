mod comune;
use comune::{all_opposizione, dati_base, fondata};
use sim_core::parlamento::{FormaMisura, ValoreProposto};
use sim_core::{Comando, Partita};

fn al_posto_riservato(p: &mut Partita) {
    for _ in 0..8 {
        if p.vista().posto_opposizione {
            return;
        }
        p.chiudi_turno();
    }
    panic!("il posto riservato non arriva mai al giocatore");
}

/// Chiude finché la misura `id` esce dal calendario e rende le chiusure. Col seme 7 il posto
/// riservato arriva al giocatore in luglio-settembre 2027: il voto cade nel turno del bilancio e
/// slitta di un turno, come ogni legge (ticket 09, riga 45).
fn al_voto(p: &mut Partita, id: u32) -> i32 {
    let mut n = 0;
    while p.vista().calendario.iter().any(|m| m.id == id) {
        assert!(n < 3, "la misura {id} non arriva al voto");
        p.chiudi_turno();
        n += 1;
    }
    n
}

fn valori(v: &[(&str, &str)]) -> Vec<ValoreProposto> {
    v.iter()
        .map(|(d, g)| ValoreProposto {
            disposizione: (*d).into(),
            gradino: (*g).into(),
        })
        .collect()
}

#[test]
fn comandi_dell_opposizione_rifiutati_a_chi_governa() {
    for mut p in [Partita::nuova(dati_base(), 1), fondata(4.5)] {
        let prima = p.salva().unwrap();
        for c in [
            Comando::Sfiducia { dicastero: None },
            Comando::PresentaProposta {
                valori: valori(&[("licenziamento", "reintegro")]),
            },
            Comando::PromuoviReferendum {
                disposizione: "licenziamento".into(),
            },
            Comando::Linea {
                misura: 0,
                si: Some(false),
            },
        ] {
            let e = p.comanda(c).unwrap_err();
            assert!(e.to_string().contains("opposizione"), "{e}");
        }
        assert_eq!(p.salva().unwrap(), prima);
    }
    let mut p = all_opposizione();
    let e = p.comanda(Comando::Dimettiti).unwrap_err();
    assert!(e.to_string().contains("opposizione"), "{e}");
}

#[test]
fn la_sfiducia_respinta_ricompatta_e_costa_valence() {
    let mut p = all_opposizione();
    let a = p.anteprima_sfiducia(None).unwrap();
    assert!(a.firme);
    let mut gemella = Partita::carica(&p.salva().unwrap()).unwrap();
    p.comanda(Comando::Sfiducia { dicastero: None }).unwrap();
    p.chiudi_turno();
    gemella.chiudi_turno();
    let e = (p.vista().esiti.into_iter())
        .find(|e| e.forma == FormaMisura::Sfiducia)
        .unwrap();
    assert_eq!(e.approvata, a.passa); // ciò che l'anteprima mostrava
    if !a.passa {
        let g = p.giocatore();
        let dati = dati_base();
        let atteso =
            dati.manopola("valence_sfiducia_respinta") * (1.0 - dati.manopola("ritorno_valence"));
        let persa = gemella.partiti()[g].valence - p.partiti()[g].valence;
        assert!((persa - atteso).abs() < 1e-9, "{persa}");
    }
}

#[test]
fn la_proposta_dell_opposizione_va_al_voto_senza_costi() {
    let mut p = all_opposizione();
    al_posto_riservato(&mut p);
    p.comanda(Comando::PresentaProposta {
        valori: valori(&[("licenziamento", "reintegro")]),
    })
    .unwrap();
    let e = p
        .comanda(Comando::PresentaProposta {
            valori: valori(&[("tetto_contante", "1000")]),
        })
        .unwrap_err();
    assert!(e.to_string().contains("posto riservato"), "{e}");
    let m = &p.vista().calendario[0];
    assert_eq!(m.promotore.as_deref(), Some("giocatore"));
    assert!(m.emendamenti.is_empty());
    let pdc = p.governo().presidente;
    let v = p.partiti()[pdc].valence;
    let n = al_voto(&mut p, m.id);
    assert!(
        p.vista()
            .esiti
            .iter()
            .any(|e| e.nome.contains("Licenziament")),
        "{:?}",
        p.vista().esiti
    );
    // Respinta o approvata, la proposta non tocca la valence del Presidente del Consiglio
    // oltre il ritorno verso la base.
    let r = dati_base().manopola("ritorno_valence");
    let base = p.partiti()[pdc].valence_base;
    let attesa = base + (v - base) * (1.0 - r).powi(n);
    assert!((p.partiti()[pdc].valence - attesa).abs() < 1e-9);
}

#[test]
fn la_linea_scelta_si_vede_e_si_vota() {
    let mut p = all_opposizione();
    al_posto_riservato(&mut p);
    p.comanda(Comando::PresentaProposta {
        valori: valori(&[("licenziamento", "reintegro")]),
    })
    .unwrap();
    let id = p.vista().calendario[0].id;
    p.comanda(Comando::Linea {
        misura: id,
        si: Some(false),
    })
    .unwrap();
    assert_eq!(p.vista().calendario[0].linea_giocatore, Some(false));
    al_voto(&mut p, id);
    let e = (p.vista().esiti.into_iter())
        .find(|e| e.misura == id)
        .unwrap();
    assert!(
        !e.linee
            .iter()
            .find(|l| l.partito == "giocatore")
            .unwrap()
            .si
    );
}

#[test]
fn il_giocatore_promuove_il_referendum() {
    let mut p = all_opposizione(); // turno 2, aprile-giugno 2027: fuori dai sei mesi
    p.comanda(Comando::PromuoviReferendum {
        disposizione: "licenziamento".into(),
    })
    .unwrap();
    assert_eq!(
        p.vista().referendum[0].promotore.as_deref(),
        Some("giocatore")
    );
}

#[test]
fn l_intesa_avversaria_si_vede_in_fasce() {
    let p = all_opposizione();
    let v = p.vista();
    assert_eq!(v.alleati_avversari.len(), p.alleati().len());
    assert!(v.alleati.is_empty()); // nessuna cifra
}
