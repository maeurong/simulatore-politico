mod comune;

use comune::{
    al_governo, all_opposizione, dati_base, edita, file_base, file_vero, fondata, presenta,
    sostituisci,
};
use sim_core::elettorato::Genere;
use sim_core::legislatura::{Assegnazione, Fondazione};
use sim_core::parlamento::CausaIntesa;
use sim_core::vista::{Vista, VoceIncarico};
use sim_core::{Comando, DatiGioco, Errore, Partita, VERSIONE_SCHEMA};

/// «Seme N a X» dei ticket: i dati veri, la fondazione a `posizione`.
fn del_ticket(seme: u64, posizione: f64) -> Partita {
    let f = Fondazione {
        nome: "Partito del giocatore".into(),
        posizione,
    };
    Partita::fonda(DatiGioco::da_testi(&file_vero()).unwrap(), seme, &f).unwrap()
}

/// La 02 delle Viste: seme 4 a 4,5, coalizione coi disponibili fra destra, centrodestra ed
/// estrema destra, chiuso il turno zero; trattativa con l'incarico al giocatore.
fn trattativa_02() -> Partita {
    let mut p = del_ticket(4, 4.5);
    let partiti = (p.disponibilita().into_iter())
        .filter(|v| {
            v.disponibile
                && ["destra", "centrodestra", "estrema_destra"].contains(&v.partito.as_str())
        })
        .map(|v| v.partito)
        .collect();
    p.comanda(Comando::DichiaraCoalizione {
        partiti,
        cessioni: vec![],
    })
    .unwrap();
    p.chiudi_turno();
    assert_eq!(p.vista().incarico, Some(VoceIncarico::Giocatore));
    p
}

/// Il 03 delle Viste: la 02 chiusa, al turno 2; il giocatore governa.
fn governo_03() -> Partita {
    let mut p = trattativa_02();
    p.chiudi_turno();
    assert!(p.governa() && p.vista().turno == 2);
    p
}

/// `ProponiSpartizione` sulla spartizione in corso, con un tecnico su `sanita`.
fn col_tecnico_su_sanita(p: &Partita) -> Partita {
    let mut q = p.clone();
    let dicasteri = (q.vista().spartizione.iter())
        .map(|s| Assegnazione {
            dicastero: s.dicastero.clone(),
            partito: s.partito.clone().filter(|_| s.dicastero != "sanita"),
        })
        .collect();
    q.comanda(Comando::ProponiSpartizione { dicasteri })
        .unwrap();
    q
}

/// Media pesata sulle forze di lavoro, nell'ordine dei dati.
fn media_forze_lavoro(valori: impl Iterator<Item = f64>) -> f64 {
    let dati = dati_base();
    let (somma, pesi) = (dati.regioni().iter())
        .zip(valori)
        .fold((0.0, 0.0), |(s, p), (r, u)| {
            (s + r.forze_lavoro * u, p + r.forze_lavoro)
        });
    somma / pesi
}

#[test]
fn venti_regioni_nell_ordine_dei_dati() {
    let dati = dati_base();
    let v = Partita::nuova(dati_base(), 7).vista();
    assert_eq!(v.regioni.len(), 20);
    for (voce, r) in v.regioni.iter().zip(dati.regioni()) {
        assert_eq!(
            (&voce.id, &voce.nome, voce.macroarea),
            (&r.id, &r.nome, r.macroarea)
        );
        assert!(
            voce.reddito.is_finite() && voce.reddito > 0.0,
            "{}: {}",
            voce.id,
            voce.reddito
        );
    }
}

/// L'esposizione non anticipa nulla: in media le disoccupazioni regionali della Vista sono la
/// riveduta già pubblica, non la disoccupazione vera di oggi.
#[test]
fn la_disoccupazione_regionale_e_in_ritardo() {
    let dati = dati_base();
    let mut p = Partita::nuova(dati_base(), 7);
    // Al turno zero la copia è uguale alla corrente.
    let correnti = p.economia().regioni.iter().map(|r| r.disoccupazione);
    assert!(
        (p.vista().regioni.iter().map(|r| r.disoccupazione)).eq(correnti),
        "turno zero"
    );
    p.chiudi_turno();
    p.chiudi_turno();
    let v = p.vista();
    let media = media_forze_lavoro(v.regioni.iter().map(|r| r.disoccupazione));
    let riveduta = v.statistica.revisione.expect("revisione").riveduta;
    assert!(
        (media - riveduta.disoccupazione).abs() < 1e-9,
        "media {media}, riveduta {}",
        riveduta.disoccupazione
    );
    let corrente = p.economia().disoccupazione_nazionale(&dati);
    assert!(
        (media - corrente).abs() > 1e-9,
        "media {media} uguale alla corrente {corrente}"
    );
}

#[test]
fn le_quote_dei_gruppi_sommano_a_uno() {
    let mut p = Partita::nuova(dati_base(), 7);
    for turno in 0..3 {
        for r in p.vista().regioni {
            assert_eq!(r.gruppi.len(), 18, "turno {turno}, {}", r.id);
            let somma: f64 = r.gruppi.iter().sum();
            assert!(
                (somma - 1.0).abs() < 1e-9,
                "turno {turno}, {}: {somma}",
                r.id
            );
        }
        p.chiudi_turno();
    }
}

#[test]
fn gli_indici_di_pericolosita() {
    let regioni = Partita::nuova(dati_base(), 7).vista().regioni;
    let indici = |f: fn(&sim_core::vista::IndiciPericolosita) -> f64| {
        regioni
            .iter()
            .map(|r| f(&r.pericolosita))
            .collect::<Vec<_>>()
    };
    for (nome, valori) in [
        ("sisma", indici(|i| i.sisma)),
        ("idro", indici(|i| i.idro)),
        ("incendi", indici(|i| i.incendi)),
        ("totale", indici(|i| i.totale)),
    ] {
        assert!(
            valori.iter().all(|x| (0.0..=1.0).contains(x)),
            "{nome}: {valori:?}"
        );
        assert!(
            valori.iter().any(|x| (x - 1.0).abs() < 1e-12),
            "{nome}: nessuna regione a 1, {valori:?}"
        );
    }
}

#[test]
fn schema_18_rifiutato() {
    let p = Partita::nuova(dati_base(), 7);
    let mut json: serde_json::Value = serde_json::from_str(&p.salva().unwrap()).unwrap();
    json["versione_schema"] = 18.into();
    assert_eq!(
        Partita::carica(&json.to_string()),
        Err(Errore::SchemaDiverso {
            trovata: 18,
            attesa: VERSIONE_SCHEMA
        })
    );
    assert_eq!(VERSIONE_SCHEMA, 19);
}

#[test]
fn disoccupazione_regionale_manomessa_rifiutata() {
    let p = Partita::nuova(dati_base(), 7);
    let errore = edita(&p, |j| {
        j["economia"]["regioni"][0]["disoccupazione_prima"] = (-1.0).into();
    })
    .unwrap_err();
    assert!(
        errore.to_string().contains("disoccupazione_prima"),
        "{errore}"
    );
    let errore = edita(&p, |j| {
        let r = j["economia"]["regioni"][0].as_object_mut().unwrap();
        r.remove("disoccupazione_prima");
    })
    .unwrap_err();
    assert!(
        errore.to_string().contains("disoccupazione_prima"),
        "{errore}"
    );
}

#[test]
fn margini_e_voti_regionali_manomessi_rifiutati() {
    let p = trattativa_02();
    type Guasto = fn(&mut serde_json::Value);
    let guasti: [(&str, Guasto); 4] = [
        ("istituti: margine", |j| {
            j["informazione"]["sondaggio"]["istituti"][0]["margine"] = (-0.01).into();
        }),
        ("condizioni: forma o valori", |j| {
            j["informazione"]["sondaggio"]["condizioni"][0]["margine"] = (-0.01).into();
        }),
        ("voto regionale fuori da 0-1", |j| {
            j["legislatura"]["elezioni"][0]["regioni"][0]["voti"][0] = 1.5.into();
        }),
        ("voto regionale fuori da 0-1", |j| {
            j["legislatura"]["elezioni"][0]["regioni"][0]["voti"][0] = (-0.1).into();
        }),
    ];
    let accolti: Vec<_> = (guasti.into_iter())
        .map(|(atteso, guasto)| (atteso, edita(&p, guasto).err().map(|e| e.to_string())))
        .filter(|(atteso, e)| !e.as_deref().is_some_and(|e| e.contains(atteso)))
        .collect();
    assert!(accolti.is_empty(), "{accolti:?}");
}

#[test]
fn la_minaccia_sta_nel_partito() {
    let p = Partita::nuova(dati_base(), 7);
    assert!(p.vista().partiti.iter().all(|x| !x.minaccia));
    let p = edita(&p, |j| j["partiti"][1]["minaccia"] = true.into()).unwrap();
    let minacce: Vec<bool> = p.vista().partiti.iter().map(|x| x.minaccia).collect();
    assert!(minacce[1], "{minacce:?}");
    assert_eq!(minacce.iter().filter(|&&m| m).count(), 1, "{minacce:?}");
}

#[test]
fn vista_completa_al_turno_zero() {
    let nuova = Partita::nuova(dati_base(), 7);
    let v = nuova.vista();
    assert!(
        v.elezioni.iter().all(|e| e.regioni.is_empty()),
        "nessun voto"
    );
    let r = &v.regole_ue;
    assert_eq!(r.procedura, nuova.economia().regole_ue.pde);
    assert!(!r.coesione_sospesa);
    assert_eq!(r.margine_energia, None);
    // Con la clausola nazionale finita nel 2025 il margine per l'energia del 2026 conta.
    let f = sostituisci(
        file_base(),
        "scenario",
        "clausola_nazionale_fino_a = 2028",
        "clausola_nazionale_fino_a = 2025",
    );
    let corta = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 7);
    assert_eq!(corta.vista().regole_ue.margine_energia, None);
    let margine = edita(&corta, |j| {
        j["economia"]["regole_ue"]["clausola_energia_fino_a"] = 2026.into();
    })
    .unwrap();
    assert_eq!(margine.vista().regole_ue.margine_energia, Some(2026));
    for (nome, p) in [
        ("al governo", al_governo()),
        ("all'opposizione", all_opposizione()),
    ] {
        let v = p.vista();
        assert!(!v.elezioni.is_empty(), "{nome}");
        for e in &v.elezioni {
            assert_eq!(e.regioni.len(), v.regioni.len(), "{nome}: {:?}", e.data);
        }
        assert_eq!(
            (v.debito.mes, v.debito.ristrutturazione),
            (None, None),
            "{nome}"
        );
        let anno = v.regole_ue.proiezione.expect("proiezione").anno;
        let senza = edita(&p, |j| {
            let conto = j["economia"]["regole_ue"]["spesa_netta"]
                .as_object_mut()
                .unwrap();
            assert!(conto.remove(&anno.to_string()).is_some(), "{nome}: {anno}");
        })
        .unwrap();
        assert_eq!(senza.vista().regole_ue.proiezione, None, "{nome}");
    }
}

#[test]
fn l_indicatore_e_una_grandezza_della_vista() {
    let v = Partita::nuova(dati_base(), 7).vista();
    assert_eq!(v.dicasteri.len(), 10);
    for d in &v.dicasteri {
        assert!(
            v.grandezze.iter().any(|g| g.id == d.indicatore),
            "{}: {}",
            d.id,
            d.indicatore
        );
    }
    let economia = v.dicasteri.iter().find(|d| d.id == "economia_finanze");
    assert_eq!(
        economia.map(|d| d.indicatore.as_str()),
        Some("efficienza_riscossione")
    );
}

#[test]
fn il_prezzo_delle_dimissioni_e_quello_del_comando() {
    let mut chiusa = col_tecnico_su_sanita(&trattativa_02());
    chiusa.chiudi_turno();
    let somma = |v: &Vista| v.alleati.iter().map(|a| a.intesa).sum::<f64>();
    for (nome, p) in [("03", governo_03()), ("02 col tecnico, chiusa", chiusa)] {
        let v = p.vista();
        assert!(p.governa(), "{nome}");
        let (mut alleati, mut gratis) = (0, 0);
        for m in &v.ministeri {
            let x = m.ministro.as_ref().unwrap();
            let mut q = p.clone();
            (q.comanda(Comando::ChiediDimissioni {
                dicastero: m.dicastero.clone(),
            }))
            .unwrap();
            let perso = somma(&v) - somma(&q.vista());
            let atteso = if x.tecnico || x.partito == v.giocatore {
                gratis += 1;
                0.0
            } else {
                alleati += 1;
                10.0
            };
            assert!(
                (m.prezzo_dimissioni - perso).abs() < 1e-9 && (perso - atteso).abs() < 1e-9,
                "{nome}, {}: prezzo {}, perso {perso}, atteso {atteso}",
                m.dicastero,
                m.prezzo_dimissioni
            );
        }
        assert!(alleati > 0 && gratis > 0, "{nome}: {alleati} {gratis}");
        let sanita = v
            .ministeri
            .iter()
            .find(|m| m.dicastero == "sanita")
            .unwrap();
        let tecnico = sanita.ministro.as_ref().unwrap().tecnico;
        assert_eq!(tecnico, nome != "03", "{nome}");
    }
    // Sotto un governo IA il comando si rifiuta: nessun prezzo.
    let mut ia = del_ticket(1, 1.5);
    ia.chiudi_turno();
    ia.chiudi_turno();
    assert!(!ia.governa() && ia.vista().ministeri.iter().any(|m| m.ministro.is_some()));
    assert!(
        ia.vista()
            .ministeri
            .iter()
            .all(|m| m.prezzo_dimissioni == 0.0)
    );
    // Dimissioni già chieste: il comando si rifiuta su quel dicastero, gli altri restano.
    let mut p = governo_03();
    let v = p.vista();
    let m = (v.ministeri.iter())
        .find(|m| m.prezzo_dimissioni > 0.0)
        .unwrap();
    let altri = (v.ministeri.iter()).filter(|x| x.dicastero != m.dicastero);
    let altri: Vec<_> = altri
        .map(|x| (x.dicastero.clone(), x.prezzo_dimissioni))
        .collect();
    let dimissioni = Comando::ChiediDimissioni {
        dicastero: m.dicastero.clone(),
    };
    p.comanda(dimissioni.clone()).unwrap();
    assert!(p.clone().comanda(dimissioni).is_err());
    let dopo = p.vista();
    for x in &dopo.ministeri {
        let atteso = (altri.iter())
            .find(|(d, _)| *d == x.dicastero)
            .map_or(0.0, |a| a.1);
        assert_eq!(x.prezzo_dimissioni, atteso, "{}", x.dicastero);
    }
}

#[test]
fn le_soglie_sono_le_manopole() {
    let s = Partita::nuova(dati_base(), 7).vista().soglie;
    assert_eq!(
        (
            s.minaccia,
            s.dimissioni,
            s.ultimatum,
            s.uscita,
            s.crisi_debito_pb
        ),
        (40.0, 25.0, 40.0, 25.0, 550.0)
    );
    // Dimissioni e uscita valgono 25 entrambe: una manopola sola le separa.
    let f = sostituisci(
        file_base(),
        "manopole",
        "[soglia_dimissioni]\nvalore = 25.0",
        "[soglia_dimissioni]\nvalore = 20.0",
    );
    let s = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 7)
        .vista()
        .soglie;
    assert_eq!((s.dimissioni, s.uscita), (20.0, 25.0));
    let f = sostituisci(
        file_base(),
        "manopole",
        "[soglia_crisi_debito]\nvalore = 550.0",
        "[soglia_crisi_debito]\nvalore = 400.0",
    );
    let f = sostituisci(
        f,
        "manopole",
        "[soglia_ultimatum]\nvalore = 40.0",
        "[soglia_ultimatum]\nvalore = 35.0",
    );
    let s = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 7)
        .vista()
        .soglie;
    assert_eq!((s.ultimatum, s.crisi_debito_pb), (35.0, 400.0));
    assert_eq!((s.minaccia, s.uscita), (40.0, 25.0));
}

#[test]
fn la_spartizione_in_corso() {
    let dati = DatiGioco::da_testi(&file_vero()).unwrap();
    let p = trattativa_02();
    let v = p.vista();
    assert_eq!(v.spartizione.len(), 10);
    for (s, d) in v.spartizione.iter().zip(dati.dicasteri()) {
        assert_eq!((&s.dicastero, s.peso), (&d.id, d.peso_spartizione));
        assert_eq!(s.partito.as_deref(), Some(s.di_default.as_str()), "{s:?}");
    }
    let tecnico = col_tecnico_su_sanita(&p).vista().spartizione;
    for (a, b) in tecnico.iter().zip(&v.spartizione) {
        if a.dicastero == "sanita" {
            assert_eq!((&a.partito, &a.di_default), (&None, &b.di_default));
        } else {
            assert_eq!(a, b);
        }
    }
    assert!(governo_03().vista().spartizione.is_empty());
    assert!(all_opposizione().vista().spartizione.is_empty());
}

#[test]
fn l_intesa_di_partenza_e_quella_della_chiusura() {
    let p = trattativa_02();
    for (nome, q, destra) in [
        ("default", p.clone(), 62.1),
        ("tecnico su sanita", col_tecnico_su_sanita(&p), 52.1),
    ] {
        let richieste = q.vista().richieste;
        let mut chiusa = q.clone();
        chiusa.chiudi_turno();
        let alleati = chiusa.vista().alleati;
        assert_eq!(richieste.len(), alleati.len(), "{nome}");
        for r in &richieste {
            let a = alleati.iter().find(|a| a.partito == r.partito).unwrap();
            assert!(
                (r.intesa_partenza - a.intesa).abs() < 1e-9,
                "{nome}, {}: {} contro {}",
                r.partito,
                r.intesa_partenza,
                a.intesa
            );
        }
        let d = richieste.iter().find(|r| r.partito == "destra").unwrap();
        assert!(
            (d.intesa_partenza - destra).abs() < 0.05,
            "{nome}: {}",
            d.intesa_partenza
        );
    }
}

#[test]
fn l_intesa_di_partenza_paga_la_rottura_dell_impegno() {
    let p = trattativa_02();
    let rotta = |q: &Partita| {
        (q.intesa().variazioni.iter())
            .filter(|v| v.causa == CausaIntesa::RotturaImpegno)
            .count()
    };
    let mut provati = Vec::new();
    let allargata = (p.disponibilita().into_iter())
        .filter(|d| d.disponibile)
        .find_map(|d| {
            let mut q = p.clone();
            q.comanda(Comando::Allarga {
                partiti: vec![d.partito.clone()],
            })
            .ok()?;
            let mut chiusa = q.clone();
            chiusa.chiudi_turno();
            provati.push(d.partito);
            (chiusa.governa() && rotta(&chiusa) > rotta(&q)).then_some((q, chiusa))
        });
    let Some((q, chiusa)) = allargata else {
        panic!("nessun allargamento rompe l'impegno: {provati:?}");
    };
    let richieste = q.vista().richieste;
    let alleati = chiusa.vista().alleati;
    assert_eq!(richieste.len(), alleati.len());
    for r in &richieste {
        let a = alleati.iter().find(|a| a.partito == r.partito).unwrap();
        assert!(
            (r.intesa_partenza - a.intesa).abs() < 1e-9,
            "{}: {} contro {}",
            r.partito,
            r.intesa_partenza,
            a.intesa
        );
    }
}

#[test]
fn a_partita_finita_la_vista_non_offre_cio_che_il_comando_rifiuta() {
    // Al governo, con una proposta dell'opposizione in calendario: la linea si sceglie.
    let mut p = governo_03();
    presenta(&mut p, &[("salario_minimo", "9")]).unwrap();
    let id = p.parlamento().misure.last().unwrap().id;
    let sinistra = (p.partiti().iter()).position(|x| x.id == "sinistra");
    let mut p = edita(&p, |j| {
        for m in j["parlamento"]["misure"].as_array_mut().unwrap() {
            if m["id"] == id {
                m["promotore"] = sinistra.into();
                m["proponente"] = serde_json::Value::Null;
            }
        }
    })
    .unwrap();
    let costo = |p: &Partita| {
        (p.vista().calendario.into_iter())
            .find(|m| m.id == id)
            .and_then(|m| m.costo_linea)
    };
    assert!(costo(&p).is_some());
    p.abbandona().unwrap();
    assert_eq!(costo(&p), None);
    let linea = Comando::Linea {
        misura: id,
        si: Some(true),
    };
    assert!(p.clone().comanda(linea).is_err());

    // All'opposizione col posto riservato di un partito IA.
    let mut p = del_ticket(1, 1.5);
    p.chiudi_turno();
    p.chiudi_turno();
    assert!(p.vista().posto_riservato_di.is_some());
    p.abbandona().unwrap();
    let v = p.vista();
    assert_eq!(v.posto_riservato_di, None);
    for d in &v.disposizioni {
        let comando = (p.clone())
            .comanda(Comando::PromuoviReferendum {
                disposizione: d.id.clone(),
            })
            .err()
            .map(|e| e.to_string());
        assert_eq!(d.rifiuto_referendum, comando, "{}", d.id);
        assert_eq!(
            comando.as_deref(),
            Some("comando non ammesso: La partita è finita")
        );
    }

    // In campagna elettorale, al turno zero: restare sulla dichiarata è ammesso.
    let mut p = del_ticket(4, 4.5);
    assert!(p.anteprima_riposizionamento(4.5).is_ok());
    p.abbandona().unwrap();
    assert!(p.anteprima_riposizionamento(4.5).is_err());
    assert!(
        p.clone()
            .comanda(Comando::Riposiziona { posizione: 4.5 })
            .is_err()
    );
}

#[test]
fn il_ribaltone_nella_vista() {
    let mut p = del_ticket(2, 1.5);
    for _ in 0..5 {
        assert_eq!(p.vista().ribaltone, None, "turno {}", p.vista().turno);
        p.chiudi_turno();
    }
    let v = p.vista();
    assert_eq!((v.turno, v.incarico), (5, Some(VoceIncarico::Guida)));
    let r = v.ribaltone.unwrap();
    let mut coalizione = r.coalizione.clone();
    coalizione.sort();
    assert_eq!(
        coalizione,
        ["centro", "centrosinistra", "giocatore", "sinistra"]
    );
    assert_eq!(r.primo.as_deref(), Some("sinistra"));
    let mut accolta = p.clone();
    accolta.comanda(Comando::ChiediGuida).unwrap();
    assert_eq!(accolta.vista().ribaltone, None);
    assert_eq!(trattativa_02().vista().ribaltone, None);
    assert_eq!(governo_03().vista().ribaltone, None);
}

#[test]
fn generi_lunghi_quanto_i_nomi() {
    let corti = sostituisci(
        file_base(),
        "elettorato",
        "generi = [\"donna\", ",
        "generi = [",
    );
    let Err(e) = DatiGioco::da_testi(&corti) else {
        panic!("un genere in meno accettato");
    };
    assert!(e.to_string().contains("generi"), "{e}");
    let senza = sostituisci(file_base(), "scenario", "genere_leader = \"donna\"\n", "");
    let Err(e) = DatiGioco::da_testi(&senza) else {
        panic!("un partito senza genere_leader accettato");
    };
    assert!(e.to_string().contains("genere_leader"), "{e}");
}

#[test]
fn il_genere_non_cambia_i_nomi() {
    let dati = dati_base();
    let el = dati.elettorato();
    // Il genere del nome nel bacino; `None` per un nome che non ci sta.
    let del_bacino = |nome: &str| {
        let n = nome.split(' ').next()?;
        let k = el.nomi.iter().position(|x| x == n)?;
        el.generi.get(k).copied()
    };
    let v = governo_03().vista();
    let nomi: Vec<&str> = (v.ministeri.iter())
        .map(|m| m.ministro.as_ref().unwrap().nome.as_str())
        .collect();
    assert_eq!(
        nomi,
        [
            "Simone Gallo",
            "Marco Bianchi",
            "Simone Ricci",
            "Sara Ferraro",
            "Chiara Caruso",
            "Filippo Ricci",
            "Luca Romano",
            "Filippo Barbieri",
            "Simone Bianchi",
            "Giulia Rossi",
        ]
    );
    let mut donne = 0;
    for x in v
        .ministeri
        .iter()
        .flat_map(|m| m.ministro.iter().chain(&m.rosa))
    {
        assert_eq!(Some(x.genere), del_bacino(&x.nome), "{}", x.nome);
        donne += usize::from(x.genere == Genere::Donna);
    }
    assert!(donne > 0);
    let leader = |v: &Vista, id: &str| {
        let p = v.partiti.iter().find(|p| p.id == id).unwrap();
        (p.leader.clone(), p.genere_leader)
    };
    assert_eq!(
        leader(&v, "centrodestra"),
        ("Silvia Moretti".into(), Some(Genere::Donna))
    );
    assert_eq!(leader(&v, "giocatore"), (String::new(), None));
    // Il leader nuovo della Destra, estratto in partita, prende il genere col nome.
    let mut p = del_ticket(2, 1.5);
    for _ in 0..5 {
        p.chiudi_turno();
    }
    let v = p.vista();
    assert_eq!(
        leader(&v, "destra"),
        ("Anna Barbieri".into(), Some(Genere::Donna))
    );
    for x in v.partiti.iter().filter(|x| !x.leader.is_empty()) {
        if let Some(g) = del_bacino(&x.leader) {
            assert_eq!(x.genere_leader, Some(g), "{}", x.leader);
        }
    }
}

#[test]
fn redditi_fasce_manomesse_rifiutate() {
    let p = Partita::nuova(dati_base(), 7);
    type Guasto = fn(&mut serde_json::Value);
    let guasti: [(&str, Guasto); 3] = [
        ("due voci", |j| {
            j["elettorato"]["redditi_fasce"]
                .as_array_mut()
                .unwrap()
                .pop();
        }),
        ("ordine invertito", |j| {
            j["elettorato"]["redditi_fasce"]
                .as_array_mut()
                .unwrap()
                .reverse();
        }),
        ("campo tolto", |j| {
            j["elettorato"]
                .as_object_mut()
                .unwrap()
                .remove("redditi_fasce");
        }),
    ];
    for (nome, guasto) in guasti {
        let e = edita(&p, guasto).expect_err(nome);
        assert!(
            matches!(e, Errore::SalvataggioIlleggibile(_)),
            "{nome}: {e}"
        );
    }
    let e = edita(&p, |j| {
        j["elettorato"]["redditi_fasce"]
            .as_array_mut()
            .unwrap()
            .pop();
    })
    .unwrap_err();
    assert!(e.to_string().contains("redditi_fasce"), "{e}");
}

#[test]
fn opere_manomesse_rifiutate() {
    let p = Partita::nuova(dati_base(), 7);
    let opera = |dicastero: i64, regione: i64, turno: i64, euro: i64| {
        serde_json::json!([{
            "dicastero": dicastero, "atto": "x", "regione": regione,
            "turno": turno, "cantiere": false, "euro": euro
        }])
    };
    let valida = opera(0, 0, 0, 0);
    let q = edita(&p, |j| j["opere"] = valida).unwrap();
    assert_eq!(q.vista().opere.len(), 1);
    for (nome, guasto) in [
        ("dicastero", opera(99, 0, 0, 0)),
        ("regione", opera(0, 99, 0, 0)),
        ("turno nel futuro", opera(0, 0, 5, 0)),
        ("spesa negativa", opera(0, 0, 0, -1)),
    ] {
        let e = edita(&p, |j| j["opere"] = guasto).expect_err(nome);
        assert!(e.to_string().contains("opere"), "{nome}: {e}");
    }
    let e = edita(&p, |j| {
        j.as_object_mut().unwrap().remove("opere");
    })
    .unwrap_err();
    assert!(matches!(e, Errore::SalvataggioIlleggibile(_)), "{e}");
}

#[test]
fn genere_leader_manomesso_rifiutato() {
    let p = fondata(3.5);
    let e = edita(&p, |j| {
        j["partiti"][0]["genere_leader"] = serde_json::Value::Null
    })
    .expect_err("un leader senza genere");
    assert!(e.to_string().contains("genere_leader"), "{e}");
    let g = p.giocatore();
    let e = edita(&p, |j| j["partiti"][g]["genere_leader"] = "donna".into())
        .expect_err("il giocatore con un genere");
    assert!(e.to_string().contains("non ha un leader"), "{e}");
}

/// Il reddito regionale è quello della chiusura precedente (`reddito_prima`), non quello di oggi:
/// la Vista non anticipa.
#[test]
fn il_reddito_regionale_e_quello_della_chiusura_precedente() {
    let mut p = Partita::nuova(dati_base(), 7);
    p.chiudi_turno();
    p.chiudi_turno();
    let v = p.vista();
    let (mut diversi, mut regioni) = (0, 0);
    for (r, voce) in v.regioni.iter().enumerate() {
        let gruppi = &p.elettorato().gruppi[r * 18..(r + 1) * 18];
        let prima: f64 = gruppi
            .iter()
            .map(|g| g.quota * g.reddito_prima.reale())
            .sum();
        let oggi: f64 = gruppi.iter().map(|g| g.quota * g.reddito.reale()).sum();
        assert!((voce.reddito - prima).abs() < 1e-9, "{}", voce.id);
        diversi += usize::from((prima - oggi).abs() > 1e-6);
        regioni += 1;
    }
    assert!(
        diversi > 0,
        "reddito_prima e reddito coincidono: {regioni} regioni"
    );
}

#[test]
fn le_regole_ue_riportano_lo_stato() {
    let p = Partita::nuova(dati_base(), 7);
    let q = edita(&p, |j| {
        let r = &mut j["economia"]["regole_ue"];
        r["pde"] = "inadempiente".into();
        r["turni_in_stato"] = 3.into();
        r["conto_controllo"] = 1.25.into();
        r["ultima_deviazione"] = 0.5.into();
    })
    .unwrap();
    let r = q.vista().regole_ue;
    assert!(r.coesione_sospesa);
    assert_eq!(
        (r.turni_in_stato, r.conto_controllo, r.ultima_deviazione),
        (3, 1.25, 0.5)
    );
    let aperta = edita(&p, |j| j["economia"]["regole_ue"]["pde"] = "aperta".into()).unwrap();
    assert!(!aperta.vista().regole_ue.coesione_sospesa);
}

/// L'intesa di un alleato sotto il prezzo: si perde solo quella che c'è.
#[test]
fn il_prezzo_delle_dimissioni_si_ferma_all_intesa() {
    let p = governo_03();
    let v = p.vista();
    let m = (v.ministeri.iter())
        .find(|m| m.prezzo_dimissioni > 0.0)
        .unwrap();
    let j = (p.partiti().iter())
        .position(|x| x.id == m.ministro.as_ref().unwrap().partito)
        .unwrap();
    let q = edita(&p, |x| x["intesa"]["valori"][j] = 4.0.into()).unwrap();
    let dopo = q.vista();
    let m = dopo
        .ministeri
        .iter()
        .find(|x| x.dicastero == m.dicastero)
        .unwrap();
    assert!(
        (m.prezzo_dimissioni - 4.0).abs() < 1e-9,
        "{}",
        m.prezzo_dimissioni
    );
    let mut r = q.clone();
    r.comanda(Comando::ChiediDimissioni {
        dicastero: m.dicastero.clone(),
    })
    .unwrap();
    let a = |v: &Vista| v.alleati.iter().map(|a| a.intesa).sum::<f64>();
    assert!((a(&dopo) - a(&r.vista()) - 4.0).abs() < 1e-9);
}
