mod comune;
use comune::{
    all_opposizione, chiudi_senza_opposizione_ia, dati_base, edita, file_base, indice, presenta,
    sostituisci,
};
use sim_core::ministri::Causa;
use sim_core::parlamento::{FormaMisura, ValoreProposto};
use sim_core::vista::{Anteprima, SeggiPartito, VoceMinistro, VoceMisura};
use sim_core::{Comando, DatiGioco, Errore, Partita, Trimestre};

fn valori(v: &[(&str, &str)]) -> Vec<ValoreProposto> {
    v.iter()
        .map(|(d, g)| ValoreProposto {
            disposizione: (*d).into(),
            gradino: (*g).into(),
        })
        .collect()
}

/// La misura in calendario con la disposizione `id`.
fn in_calendario(p: &Partita, id: &str) -> VoceMisura {
    (p.vista().calendario.into_iter())
        .find(|m| m.valori.iter().any(|v| v.disposizione == id))
        .unwrap()
}

/// Costo, effetti, leve e gruppi della misura in calendario sono quelli dell'anteprima.
fn stima_uguale(m: &VoceMisura, a: &Anteprima) {
    assert_eq!(
        (m.costo_annuo, &m.effetti, &m.leve, &m.gruppi),
        (a.costo_annuo, &a.effetti, &a.leve, &a.gruppi)
    );
}

fn ministro(p: &Partita, dicastero: &str) -> VoceMinistro {
    (p.vista().ministeri.into_iter())
        .find(|m| m.dicastero == dicastero)
        .and_then(|m| m.ministro)
        .unwrap()
}

/// `p` col ministro del `dicastero` a `capacita`, fuori dal rodaggio.
fn con_capacita(p: &Partita, dicastero: &str, capacita: u8) -> Partita {
    let i = indice(dicastero);
    edita(p, |j| {
        let m = &mut j["ministri"]["dicasteri"][i]["ministro"];
        m["capacita"] = capacita.into();
        m["turni_in_carica"] = 1.into();
    })
    .unwrap()
}

/// Al governo la misura in calendario porta la stima dell'anteprima della stessa bozza, chiamata
/// nello stesso turno prima di presentarla: stessa capacità del ministro, stessa chiave. Col seme 1
/// il ministro della Sanità ha capacità 5, e la stima neutra del ticket alto è un'altra.
#[test]
fn la_misura_in_calendario_porta_l_anteprima() {
    let mut p = Partita::nuova(dati_base(), 1);
    let chi = ministro(&p, "sanita");
    assert_eq!((chi.capacita, chi.rodaggio), (5, false));
    let legge = valori(&[("ticket", "alto")]);
    let a = p.anteprima(&legge).unwrap();
    // La capacità neutra è 3: se desse la stessa stima, il test non distinguerebbe le due.
    let neutra = con_capacita(&p, "sanita", 3).anteprima(&legge).unwrap();
    assert_ne!(neutra.effetti, a.effetti);
    p.comanda(Comando::PresentaLegge { valori: legge }).unwrap();
    let m = in_calendario(&p, "ticket");
    assert!(!m.effetti.is_empty() && m.costo_annuo != 0, "{m:?}");
    stima_uguale(&m, &a);
}

/// La stima ha la chiave del turno di presentazione: fra un turno e l'altro, con lo stesso
/// ministro fuori dal rodaggio, gli effetti non cambiano; due viste di fila danno la stessa misura
/// e non toccano la partita.
#[test]
fn la_stima_in_calendario_non_si_ricampiona() {
    // Col seme 5, dopo una chiusura, il ministro dello Sviluppo economico ha capacità 1: una stima
    // ricampionata al turno dopo sarebbe un'altra.
    let mut p = Partita::nuova(dati_base(), 5);
    p.chiudi_turno();
    let chi = ministro(&p, "sviluppo_economico");
    assert_eq!((chi.capacita, chi.rodaggio), (1, false));
    let legge = valori(&[("incentivo_investimenti", "10"), ("credito_zes", "0_75")]);
    p.comanda(Comando::PresentaLegge {
        valori: legge.clone(),
    })
    .unwrap();
    let prima = p.salva().unwrap();
    let m = in_calendario(&p, "credito_zes");
    assert_eq!(in_calendario(&p, "credito_zes"), m);
    assert_eq!(p.salva().unwrap(), prima);
    p.chiudi_turno();
    assert_eq!(ministro(&p, "sviluppo_economico").nome, chi.nome);
    let dopo = in_calendario(&p, "credito_zes");
    assert!(!m.effetti.is_empty());
    assert_eq!(dopo.effetti, m.effetti);
    // L'anteprima rifiuta la misura in esame: tolta dal calendario, la stessa legge al turno nuovo
    // ha la chiave nuova, e una stima diversa. Se fosse uguale, il test non proverebbe nulla.
    let id = m.id;
    let senza = edita(&p, |j| {
        (j["parlamento"]["misure"].as_array_mut().unwrap()).retain(|x| x["id"] != id);
    })
    .unwrap();
    assert_ne!(senza.anteprima(&legge).unwrap().effetti, m.effetti);
}

/// La proposta del giocatore all'opposizione non ha ministro: la stima in calendario è quella
/// neutra di `anteprima_proposta`.
#[test]
fn la_proposta_d_opposizione_ha_la_stima_neutra() {
    let mut p = all_opposizione();
    for _ in 0..8 {
        if p.vista().posto_riservato {
            break;
        }
        chiudi_senza_opposizione_ia(&mut p);
    }
    // Due disposizioni: sulla prima una stima di capacità 5 sarebbe un'altra, sulla seconda una
    // di capacità 1.
    let legge = valori(&[("incentivo_investimenti", "20"), ("credito_zes", "0")]);
    let a = p.anteprima_proposta(&legge).unwrap();
    p.comanda(Comando::PresentaProposta { valori: legge })
        .unwrap();
    let m = in_calendario(&p, "credito_zes");
    assert_eq!(m.promotore.as_deref(), Some("giocatore"));
    assert!(!m.effetti.is_empty(), "{m:?}");
    stima_uguale(&m, &a);
    // La stessa misura, data al ministro dello Sviluppo economico di capacità 1 e poi 5, ha
    // un'altra stima: se una delle due fosse la neutra, il test non la distinguerebbe.
    let sviluppo = indice("sviluppo_economico");
    let col_ministro = edita(&p, |j| {
        let misure = j["parlamento"]["misure"].as_array_mut().unwrap();
        for x in misure.iter_mut().filter(|x| x["id"] == m.id) {
            x["proponente"] = sviluppo.into();
        }
    })
    .unwrap();
    for capacita in [1, 5] {
        let q = con_capacita(&col_ministro, "sviluppo_economico", capacita);
        assert_ne!(
            in_calendario(&q, "credito_zes").effetti,
            m.effetti,
            "{capacita}"
        );
    }
}

#[test]
fn parlamento_uscente_con_la_maggioranza() {
    let v = Partita::nuova(dati_base(), 1).vista();
    let somma = |f: fn(&SeggiPartito) -> u32| v.seggi.iter().map(f).sum::<u32>();
    assert_eq!((somma(|s| s.camera), somma(|s| s.senato)), (400, 200));
    let governo = |f: fn(&SeggiPartito) -> u32| {
        v.seggi
            .iter()
            .filter(|s| s.partito == "destra" || s.partito == "centrodestra")
            .map(f)
            .sum::<u32>()
    };
    assert!(governo(|s| s.camera) > 200 && governo(|s| s.senato) > 100);
}

#[test]
fn parlamento_uscente_incoerente_rifiutato_col_nome() {
    for (da, a, nome) in [
        (
            "estrema_sinistra = 9,",
            "ignoto = 9,",
            "camera: partito sconosciuto: ignoto",
        ),
        (
            "centro = 7,",
            "centro = -7,",
            "senato: seggi negativi a centro",
        ),
        ("destra = 189,", "destra = 190,", "camera: 401 seggi"),
        (
            "destra = 189,",
            "destra = 9223372036854775807,",
            "camera: 9223372036854775807 seggi a destra, più dei 400 del ramo",
        ),
    ] {
        let f = sostituisci(file_base(), "scenario", da, a);
        let e = DatiGioco::da_testi(&f).unwrap_err();
        assert!(
            matches!(e, Errore::Partiti(_)) && e.to_string().contains(nome),
            "{e}"
        );
    }
}

#[test]
fn legge_presentata_occupa_il_calendario_fino_al_voto() {
    let mut p = Partita::nuova(dati_base(), 1);
    presenta(&mut p, &[("eta_pensionabile", "68")]).unwrap();
    let v = p.vista();
    let m = &v.calendario[0];
    assert_eq!(
        (m.forma, m.voto, m.proponente.as_deref()),
        (
            FormaMisura::Riforma,
            Trimestre {
                anno: 2027,
                trimestre: 1
            },
            Some("lavoro_welfare")
        )
    );
    assert_eq!(m.nome, "Età pensionabile: 68 anni");
    presenta(&mut p, &[("ticket", "alto")]).unwrap();
    presenta(&mut p, &[("obbligo_scolastico", "18")]).unwrap();
    let e = presenta(&mut p, &[("leva_militare", "obbligatoria")]).unwrap_err();
    assert!(e.to_string().contains("calendario pieno"), "{e}");
    presenta(&mut p, &[("numero_parlamentari", "630_315")]).unwrap();
    assert_eq!(p.vista().calendario[3].forma, FormaMisura::Costituzionale);
}

/// `Vista.posti` conta come `presenta`: col posto ordinario libero la legge entra, coi posti tutti
/// occupati `presenta` rifiuta; lo stesso per il posto costituzionale.
#[test]
fn i_posti_contati_come_li_conta_presenta() {
    let mut p = Partita::nuova(dati_base(), 1);
    let posti = |p: &Partita| {
        let v = p.vista().posti;
        (
            v.ordinari,
            v.ordinari_occupati,
            v.costituzionali,
            v.costituzionali_occupati,
        )
    };
    assert_eq!(posti(&p), (3, 0, 1, 0));
    presenta(&mut p, &[("eta_pensionabile", "68")]).unwrap();
    presenta(&mut p, &[("soglia_lista", "0")]).unwrap();
    assert_eq!(posti(&p), (3, 2, 1, 0));
    presenta(&mut p, &[("obbligo_scolastico", "18")]).unwrap();
    assert_eq!(posti(&p), (3, 3, 1, 0));
    let e = presenta(&mut p, &[("leva_militare", "obbligatoria")]).unwrap_err();
    assert!(e.to_string().contains("calendario pieno"), "{e}");
    presenta(&mut p, &[("numero_parlamentari", "630_315")]).unwrap();
    assert_eq!(posti(&p), (3, 3, 1, 1));
    let e = presenta(&mut p, &[("durata_legislatura", "4")]).unwrap_err();
    assert!(e.to_string().contains("calendario pieno"), "{e}");
}

#[test]
fn leggi_non_ammesse_rifiutate_col_nome() {
    let mut p = Partita::nuova(dati_base(), 1);
    let prima = p.salva().unwrap();
    for (valori, nome) in [
        (
            vec![("eta_pensionabile", "67")],
            "Età pensionabile è già a 67 anni",
        ),
        (
            vec![("eta_pensionabile", "71")],
            "Età pensionabile: gradino sconosciuto «71»",
        ),
        (
            vec![("eta_pension", "68")],
            "disposizione sconosciuta: eta_pension",
        ),
        (
            vec![("eta_pensionabile", "68"), ("ticket", "alto")],
            "le disposizioni di una legge devono stare nello stesso dicastero: «Ticket sanitario» sta altrove",
        ),
        (
            vec![("eta_pensionabile", "68"), ("eta_pensionabile", "69")],
            "disposizione ripetuta: Età pensionabile",
        ),
        (
            vec![("eta_pensionabile", "68"), ("soglia_lista", "0")],
            "norme di forma diversa",
        ),
        (vec![], "legge vuota"),
    ] {
        let e = presenta(&mut p, &valori).unwrap_err();
        assert!(
            matches!(e, Errore::Comando(_)) && e.to_string().contains(nome),
            "{e}"
        );
    }
    assert_eq!(p.salva().unwrap(), prima);
    presenta(&mut p, &[("eta_pensionabile", "68")]).unwrap();
    let e = presenta(&mut p, &[("eta_pensionabile", "69")]).unwrap_err();
    assert!(
        e.to_string().contains("Età pensionabile è già in esame"),
        "{e}"
    );
}

#[test]
fn firma_contro_la_posizione_costa_soddisfazione() {
    let s = |gradino: &str| {
        let mut p = Partita::nuova(dati_base(), 2);
        presenta(&mut p, &[("eta_pensionabile", gradino)]).unwrap();
        let v = p.vista();
        let m = v
            .ministeri
            .iter()
            .find(|m| m.dicastero == "lavoro_welfare")
            .unwrap()
            .ministro
            .clone()
            .unwrap();
        (
            m.soddisfazione,
            m.variazioni
                .iter()
                .any(|x| x.causa == Causa::NormaControPosizione),
        )
    };
    let (lontana, causa) = s("62");
    assert!(causa && lontana < s("68").0);
}

#[test]
fn alleati_con_l_intesa_nella_vista() {
    let v = Partita::nuova(dati_base(), 1).vista();
    assert_eq!(v.alleati.len(), 1);
    let a = &v.alleati[0];
    assert_eq!(
        (a.partito.as_str(), a.intesa, a.ultimatum),
        ("centrodestra", 60.0, false)
    );
}

#[test]
fn legge_presentata_sopravvive_al_salvataggio() {
    let mut p = Partita::nuova(dati_base(), 1);
    presenta(&mut p, &[("eta_pensionabile", "68")]).unwrap();
    let mut q = Partita::carica(&p.salva().unwrap()).unwrap();
    assert_eq!(
        (q.vista(), q.salva().unwrap()),
        (p.vista(), p.salva().unwrap())
    );
    p.chiudi_turno();
    q.chiudi_turno();
    assert_eq!(
        (q.vista(), q.salva().unwrap()),
        (p.vista(), p.salva().unwrap())
    );
}

#[test]
fn parlamento_e_intesa_manomessi_rifiutati() {
    let p = Partita::nuova(dati_base(), 1);
    let piu_uno = p.parlamento().seggi_camera[0] + 1;
    for (punto, valore, nome) in [
        (
            "/parlamento/seggi_camera/0",
            serde_json::json!(piu_uno),
            "parlamento: seggi",
        ),
        ("/intesa/valori/0", serde_json::json!(150.0), "intesa:"),
        // In u32 la somma trabocca e torna 400.
        (
            "/parlamento/seggi_camera",
            serde_json::json!([u32::MAX, 1, 400, 0, 0, 0, 0]),
            "parlamento: seggi",
        ),
    ] {
        let e = edita(&p, |j| *j.pointer_mut(punto).unwrap() = valore).unwrap_err();
        assert!(
            matches!(e, Errore::SalvataggioIlleggibile(_)) && e.to_string().contains(nome),
            "{e}"
        );
    }
}
