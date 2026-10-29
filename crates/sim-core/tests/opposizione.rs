mod comune;
use comune::{all_opposizione, con_seggi, dati_base, edita, fondata};
use sim_core::legislatura::Fase;
use sim_core::ministri::Causa;
use sim_core::parlamento::{CausaIntesa, FormaMisura, ValoreProposto};
use sim_core::vista::FasciaIntesa;
use sim_core::{Comando, Partita};

fn al_posto_riservato(p: &mut Partita) {
    for _ in 0..8 {
        if p.vista().posto_riservato {
            return;
        }
        p.chiudi_turno();
    }
    panic!("il posto riservato non arriva mai al giocatore");
}

/// Chiude finché la misura `id` esce dal calendario, togliendo prima di ogni chiusura le riforme e
/// le elettorali del governo IA: il voto di `id` resta l'unico con posizione. Col seme 7 il posto
/// riservato arriva al giocatore in luglio-settembre 2027: il voto cade nel turno del bilancio e
/// slitta di un turno, come ogni legge (ticket 09, riga 45).
fn al_voto_da_sola(p: &mut Partita, id: u32) {
    let mut n = 0;
    while p.vista().calendario.iter().any(|m| m.id == id) {
        assert!(n < 3, "la misura {id} non arriva al voto");
        *p = edita(p, |j| {
            let misure = j["parlamento"]["misure"].as_array_mut().unwrap();
            misure.retain(|m| {
                !(m["promotore"].is_null()
                    && (m["forma"] == "riforma" || m["forma"] == "elettorale"))
            });
        })
        .unwrap();
        p.chiudi_turno();
        n += 1;
    }
}

/// La proposta del giocatore in calendario, fra le leggi del governo IA: il suo id.
fn proposta_del_giocatore(p: &Partita) -> u32 {
    (p.vista().calendario.iter())
        .find(|m| m.promotore.as_deref() == Some("giocatore"))
        .unwrap()
        .id
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

/// Fuori dalla fase ordinaria, e a partita finita, i comandi dell'opposizione e l'anteprima della
/// sfiducia sono rifiutati col motivo, e la partita resta identica.
#[test]
fn comandi_dell_opposizione_rifiutati_fuori_dalla_fase_ordinaria() {
    let mut sciolta = fondata(1.5);
    assert_eq!(sciolta.legislatura().fase, Fase::Sciolta);
    let mut trattativa = fondata(1.5);
    trattativa.chiudi_turno();
    assert_eq!(trattativa.legislatura().fase, Fase::Trattativa);
    let mut finita = all_opposizione();
    finita.abbandona().unwrap();
    for (p, motivo) in [
        (&mut sciolta, "solo all'opposizione: Camere sciolte"),
        (&mut trattativa, "solo all'opposizione: trattativa in corso"),
        (&mut finita, "La partita è finita"),
    ] {
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
            rifiutato(p, c, motivo);
        }
        let e = p.anteprima_sfiducia(None).unwrap_err();
        assert!(e.to_string().contains(motivo), "{e}");
    }
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
    assert!(!a.passa); // il governo IA ha la maggioranza
    assert!(!e.approvata); // ciò che l'anteprima mostrava
    let g = p.giocatore();
    let dati = dati_base();
    let atteso =
        dati.manopola("valence_sfiducia_respinta") * (1.0 - dati.manopola("ritorno_valence"));
    let persa = gemella.partiti()[g].valence - p.partiti()[g].valence;
    assert!((persa - atteso).abs() < 1e-9, "{persa}");
}

/// L'opposizione con 270 seggi su 400 alla Camera (giocatore 240, estrema destra 30).
fn opposizione_in_maggioranza() -> Partita {
    let p = all_opposizione();
    let senato = p.parlamento().seggi_senato.clone();
    con_seggi(&p, &[0, 50, 10, 10, 10, 50, 30, 240], &senato)
}

#[test]
fn la_sfiducia_approvata_fa_cadere_il_governo() {
    let mut p = opposizione_in_maggioranza();
    assert!(p.anteprima_sfiducia(None).unwrap().passa);
    p.comanda(Comando::Sfiducia { dicastero: None }).unwrap();
    p.chiudi_turno();
    let e = (p.vista().esiti.into_iter())
        .find(|e| e.forma == FormaMisura::Sfiducia)
        .unwrap();
    assert!(e.approvata);
    assert_ne!(p.legislatura().fase, Fase::Ordinaria); // trattativa o Camere sciolte
}

#[test]
fn sfiducia_a_un_ministro_lo_manda_in_interim() {
    let mut p = opposizione_in_maggioranza();
    let (i, presidente) = (comune::indice("sanita"), p.governo().presidente);
    assert!(p.ministri().dicasteri[i].ministro.is_some());
    p.comanda(Comando::Sfiducia {
        dicastero: Some("sanita".into()),
    })
    .unwrap();
    p.chiudi_turno();
    let st = &p.ministri().dicasteri[i];
    assert!(st.ministro.is_none());
    assert_eq!(st.causa_dimissioni, Some(Causa::Sfiduciato));
    assert_eq!(p.legislatura().fase, Fase::Ordinaria);
    assert_eq!(p.governo().presidente, presidente);
}

#[test]
fn una_sfiducia_per_turno_e_la_linea_su_una_misura_vera() {
    let mut p = all_opposizione();
    p.comanda(Comando::Sfiducia { dicastero: None }).unwrap();
    let sfiducia = Comando::Sfiducia {
        dicastero: Some("sanita".into()),
    };
    rifiutato(
        &mut p,
        sfiducia,
        "una sfiducia è già presentata in questo turno",
    );
    let linea = Comando::Linea {
        misura: 99,
        si: Some(true),
    };
    rifiutato(&mut p, linea, "misura sconosciuta: 99");
}

/// Le firme bastano da un decimo esatto della Camera (40 su 400), non da uno in meno.
#[test]
fn le_firme_bastano_a_un_decimo_esatto() {
    let p = all_opposizione();
    let senato = p.parlamento().seggi_senato.clone();
    for (opposizione, firme) in [(40, true), (39, false)] {
        let camera = [0, 125, 26, 18, 33, 198 - opposizione, 0, opposizione];
        let p = con_seggi(&p, &camera, &senato);
        assert_eq!(
            p.anteprima_sfiducia(None).unwrap().firme,
            firme,
            "{opposizione}"
        );
    }
}

#[test]
fn sfiducia_e_linea_sopravvivono_al_salvataggio() {
    let mut p = all_opposizione();
    al_posto_riservato(&mut p);
    p.comanda(Comando::PresentaProposta {
        valori: valori(&[("licenziamento", "reintegro")]),
    })
    .unwrap();
    let id = proposta_del_giocatore(&p);
    p.comanda(Comando::Linea {
        misura: id,
        si: Some(false),
    })
    .unwrap();
    p.comanda(Comando::Sfiducia { dicastero: None }).unwrap();
    let salvato = p.salva().unwrap();
    let ricaricata = Partita::carica(&salvato).unwrap();
    assert_eq!(ricaricata.salva().unwrap(), salvato);
    let v = ricaricata.vista();
    let m = v.calendario.iter().find(|m| m.id == id).unwrap();
    assert_eq!(m.linea_giocatore, Some(false));
    assert_eq!(
        ricaricata.parlamento().sfiducia,
        Some((None, p.giocatore()))
    );
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
    assert!(
        e.to_string().contains("il posto riservato è già occupato"),
        "{e}"
    );
    assert!(!p.vista().posto_riservato); // occupato dalla proposta
    let id = proposta_del_giocatore(&p);
    let pdc = p.governo().presidente;
    // Respinta, non tocca la valence del Presidente del Consiglio oltre il ritorno verso la base
    // e le leggi e i decreti del governo IA respinti nelle stesse chiusure.
    let d = dati_base();
    let r = d.manopola("ritorno_valence");
    let base = p.partiti()[pdc].valence_base;
    let mut attesa = p.partiti()[pdc].valence;
    let mut chiudi = |p: &mut Partita| {
        let data = p.vista().data;
        p.chiudi_turno();
        let perso: f64 = (p.vista().esiti.iter())
            .filter(|e| e.data == data && e.misura != id && !e.approvata)
            .filter(|e| e.decaduta.is_none())
            .map(|e| match e.forma {
                FormaMisura::Conversione => d.manopola("valence_decreto_decaduto"),
                _ => d.manopola("valence_legge_respinta"),
            })
            .sum();
        attesa = base + (attesa - perso - base) * (1.0 - r);
    };
    // All'apertura del turno del voto gli alleati lontani (centro, centrodestra, destra) non
    // chiedono emendamenti sulla proposta dell'opposizione.
    chiudi(&mut p);
    let vista = p.vista();
    let m = vista.calendario.iter().find(|m| m.id == id).unwrap();
    assert_eq!(m.voto, vista.data);
    assert!(m.emendamenti.is_empty(), "{:?}", m.emendamenti);
    for _ in 0..3 {
        if p.vista().calendario.iter().all(|m| m.id != id) {
            break;
        }
        chiudi(&mut p);
    }
    assert!(p.vista().calendario.iter().all(|m| m.id != id));
    let e = (p.vista().esiti.into_iter())
        .find(|e| e.misura == id)
        .unwrap();
    assert!(e.nome.contains("Licenziament"), "{}", e.nome);
    assert!(!e.approvata, "col seme 7 la proposta è respinta: {e:?}");
    let valence = p.partiti()[pdc].valence;
    assert!((valence - attesa).abs() < 1e-9, "{valence} {attesa}");
}

#[test]
fn la_linea_scelta_si_vede_e_si_vota() {
    let mut p = all_opposizione();
    al_posto_riservato(&mut p);
    p.comanda(Comando::PresentaProposta {
        valori: valori(&[("licenziamento", "reintegro")]),
    })
    .unwrap();
    let id = proposta_del_giocatore(&p);
    p.comanda(Comando::Linea {
        misura: id,
        si: Some(false),
    })
    .unwrap();
    let v = p.vista();
    let m = v.calendario.iter().find(|m| m.id == id).unwrap();
    assert_eq!(m.linea_giocatore, Some(false));
    let g = p.giocatore();
    let k = dati_base().disposizione("licenziamento").unwrap();
    let vigente = p.disposizioni().posizione(&dati_base(), k).unwrap();
    let prima = p.partiti()[g].posizione_percepita;
    al_voto_da_sola(&mut p, id);
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
    // Col no la percepita scivola verso la posizione vigente (tutele crescenti), non verso il
    // reintegro: è l'unico voto con posizione di queste chiusure.
    let s = dati_base().manopola("scivolamento_percepita");
    let attesa = prima + s * (vigente - prima);
    let dopo = p.partiti()[g].posizione_percepita;
    assert!(
        (dopo - attesa).abs() < 1e-9,
        "{prima} → {dopo}, attesa {attesa}"
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
    let (a, b) = (p.alleati()[0], p.alleati()[1]);
    let p = edita(&p, |j| {
        j["intesa"]["valori"][a] = 60.0.into();
        j["intesa"]["ultimatum"][b] = "norma_lontana".into();
    })
    .unwrap();
    let id = |j: usize| p.partiti()[j].id.clone();
    let fasce = p.vista().alleati_avversari;
    let fascia = |j: usize| fasce.iter().find(|f| f.partito == id(j)).unwrap();
    assert_eq!(fascia(a).fascia, FasciaIntesa::Solida);
    assert_eq!(fascia(b).fascia, FasciaIntesa::Ultimatum);
    assert_eq!(fascia(b).causa_ultimatum, Some(CausaIntesa::NormaLontana));
    // Gli altri alleati stanno a 50, sotto soglia_incrinata.
    let c = p.alleati()[2];
    assert_eq!(fascia(c).fascia, FasciaIntesa::Incrinata);
    assert_eq!(fascia(c).causa_ultimatum, None);
}

/// Storia 90: chi non governa non vede le agende dei ministri prima dell'esecuzione.
#[test]
fn senza_governo_le_agende_dei_ministri_non_si_vedono() {
    let senza_agende = |p: &Partita| p.vista().ministeri.iter().all(|m| m.agenda.is_none());
    let mut p = fondata(1.5); // Camere sciolte
    assert!(senza_agende(&p));
    p.chiudi_turno(); // trattativa
    assert!(senza_agende(&p));
    assert!(senza_agende(&all_opposizione()));
    let p = Partita::nuova(dati_base(), 1);
    assert!(p.vista().ministeri.iter().any(|m| m.agenda.is_some()));
}

#[test]
fn le_fasce_solo_in_fase_ordinaria() {
    let mut p = fondata(1.5);
    assert_eq!(p.legislatura().fase, Fase::Sciolta);
    assert!(p.vista().alleati_avversari.is_empty());
    p.chiudi_turno();
    assert_eq!(p.legislatura().fase, Fase::Trattativa);
    assert_ne!(p.governo().presidente, p.giocatore());
    assert!(p.vista().alleati_avversari.is_empty());
}

/// `c` è rifiutato con `frammento` nel messaggio, e la partita resta identica.
fn rifiutato(p: &mut Partita, c: Comando, frammento: &str) {
    let prima = p.salva().unwrap();
    let e = p.comanda(c).unwrap_err();
    assert!(e.to_string().contains(frammento), "{e}");
    assert_eq!(p.salva().unwrap(), prima);
}

#[test]
fn sfiducia_rifiutata_col_motivo() {
    let mut p = all_opposizione();
    let sfiducia = |d: &str| Comando::Sfiducia {
        dicastero: Some(d.into()),
    };
    rifiutato(&mut p, sfiducia("inesistente"), "dicastero sconosciuto");
    // L'anteprima non tocca nulla.
    let prima = p.salva().unwrap();
    p.anteprima_sfiducia(None).unwrap();
    assert_eq!(p.salva().unwrap(), prima);
    let mut p = edita(&p, |j| {
        let d = &mut j["ministri"]["dicasteri"][0];
        for campo in ["ministro", "agenda", "proposta"] {
            d[campo] = serde_json::Value::Null;
        }
    })
    .unwrap();
    let interim = dati_base().dicasteri()[0].clone();
    assert!(p.anteprima_sfiducia(Some(&interim.id)).is_err());
    rifiutato(&mut p, sfiducia(&interim.id), "è in interim");
}

#[test]
fn senza_firme_ne_posto_all_opposizione() {
    let p = all_opposizione();
    // L'opposizione (estrema sinistra, estrema destra, giocatore) senza seggi alla Camera.
    let camera = [0, 145, 26, 18, 33, 178, 0, 0];
    let mut p = con_seggi(&p, &camera, &p.parlamento().seggi_senato.clone());
    assert!(!p.anteprima_sfiducia(None).unwrap().firme);
    let sfiducia = Comando::Sfiducia { dicastero: None };
    rifiutato(
        &mut p,
        sfiducia,
        "servono 40 firme, un decimo della Camera: ne hai 0",
    );
    let proposta = Comando::PresentaProposta {
        valori: valori(&[("licenziamento", "reintegro")]),
    };
    rifiutato(&mut p, proposta, "nessun posto riservato");
}

#[test]
fn il_posto_riservato_ha_le_sue_regole() {
    // Turno 2: la fila è [estrema destra, giocatore] (l'estrema sinistra non ha seggi alla Camera),
    // e il posto va all'estrema destra.
    let mut p = all_opposizione();
    assert!(!p.vista().posto_riservato);
    let proposta = |d: &str, g: &str| Comando::PresentaProposta {
        valori: valori(&[(d, g)]),
    };
    rifiutato(
        &mut p,
        proposta("licenziamento", "reintegro"),
        "il posto riservato di questo turno è di",
    );
    al_posto_riservato(&mut p);
    rifiutato(
        &mut p,
        proposta("durata_legislatura", "4"),
        "una norma costituzionale non entra nel posto riservato",
    );
}

/// La proposta dell'opposizione non è del governo caduto: alla caduta senza voto non decade.
#[test]
fn la_proposta_dell_opposizione_sopravvive_alla_caduta() {
    let mut p = all_opposizione();
    al_posto_riservato(&mut p);
    p.comanda(Comando::PresentaProposta {
        valori: valori(&[("licenziamento", "reintegro")]),
    })
    .unwrap();
    let id = proposta_del_giocatore(&p);
    let mut p = edita(&p, |j| j["legislatura"]["dimissioni"] = true.into()).unwrap();
    p.chiudi_turno();
    assert_ne!(p.legislatura().fase, Fase::Ordinaria);
    assert!(p.vista().calendario.iter().any(|m| m.id == id));
    assert!(!(p.vista().esiti.iter()).any(|e| e.misura == id));
}

/// In crisi sotto un governo IA, la sfiducia del giocatore respinta prende il posto di quella
/// automatica: un solo voto, un solo ricompattamento, e la crisi si riapre.
#[test]
fn in_crisi_la_sfiducia_del_giocatore_sostituisce_quella_automatica() {
    // Governo e opposizione pari alla Camera (200 e 200), il governo avanti al Senato (101).
    let camera = [0, 100, 20, 20, 20, 40, 0, 200];
    let senato = [0, 50, 15, 15, 11, 10, 0, 99];
    let p = con_seggi(&all_opposizione(), &camera, &senato);
    let turno = p.vista().turno;
    let mut p = edita(&p, |j| j["legislatura"]["crisi"] = turno.into()).unwrap();
    assert!(!p.anteprima_sfiducia(None).unwrap().passa);
    let alleati = p.alleati();
    p.comanda(Comando::Sfiducia { dicastero: None }).unwrap();
    p.chiudi_turno();
    let sfiducie = (p.vista().esiti.iter())
        .filter(|e| e.forma == FormaMisura::Sfiducia)
        .count();
    assert_eq!(sfiducie, 1);
    for j in alleati {
        let ricompattati = (p.intesa().variazioni.iter())
            .filter(|v| v.partito == j && v.causa == CausaIntesa::SfiduciaRespinta)
            .count();
        assert_eq!(ricompattati, 1, "{j}");
    }
    assert_eq!(p.legislatura().crisi, Some(turno + 1));
}
