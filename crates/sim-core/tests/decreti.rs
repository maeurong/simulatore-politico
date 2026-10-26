mod comune;
use comune::{dati_base, edita, ferma_tranne, ferma_tutti, file_base, imposta};
use sim_core::dati::{DatiGioco, FileDati};
use sim_core::ministri::Causa;
use sim_core::parlamento::{Decreto, FormaMisura, ValoreProposto};
use sim_core::{Comando, Errore, Partita};

fn sanita_cara() -> FileDati {
    comune::sanita_cara(file_base())
}

fn accolta(f: FileDati) -> (Partita, i64) {
    let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    let importo = p.vista().ministeri[3].proposta.as_ref().unwrap().importo;
    p.comanda(Comando::Accogli {
        dicastero: "sanita".into(),
    })
    .unwrap();
    (p, importo)
}

fn sposta(da: &str, a: &str, importo: i64) -> Comando {
    Comando::EmanaDecreto {
        decreto: Decreto::Fondi {
            da: da.into(),
            a: a.into(),
            importo,
        },
    }
}

#[test]
fn accogli_apre_una_conversione_a_fine_turno() {
    let (mut p, importo) = accolta(sanita_cara());
    let v = p.vista();
    let m = v
        .calendario
        .iter()
        .find(|m| m.forma == FormaMisura::Conversione)
        .unwrap();
    assert_eq!(m.proponente.as_deref(), Some("sanita"));
    assert_eq!(p.economia().decreti[3], importo);
    p.chiudi_turno();
    assert!(
        p.vista()
            .esiti
            .iter()
            .any(|e| e.forma == FormaMisura::Conversione)
    );
    assert_eq!(p.economia().decreti.iter().sum::<i64>(), 0);
}

#[test]
fn ferma_dopo_accogli_ritira_il_decreto() {
    let prima = Partita::nuova(DatiGioco::da_testi(&sanita_cara()).unwrap(), 1)
        .vista()
        .ministeri[3]
        .margine_residuo;
    let (mut p, _) = accolta(sanita_cara());
    p.comanda(Comando::Ferma {
        dicastero: "sanita".into(),
    })
    .unwrap();
    assert_eq!(p.economia().decreti[3], 0);
    assert_eq!(p.vista().ministeri[3].margine_residuo, prima);
    assert!(
        p.vista()
            .calendario
            .iter()
            .all(|m| m.forma != FormaMisura::Conversione)
    );
}

#[test]
fn dimissioni_dopo_accogli_ritirano_il_decreto() {
    let prima = Partita::nuova(DatiGioco::da_testi(&sanita_cara()).unwrap(), 1)
        .vista()
        .ministeri[3]
        .margine_residuo;
    let (mut p, _) = accolta(sanita_cara());
    p.comanda(Comando::ChiediDimissioni {
        dicastero: "sanita".into(),
    })
    .unwrap();
    assert_eq!(p.economia().decreti[3], 0);
    assert_eq!(p.vista().ministeri[3].margine_residuo, prima);
    assert!(
        p.vista()
            .calendario
            .iter()
            .all(|m| m.forma != FormaMisura::Conversione)
    );
}

#[test]
fn dirotta_dopo_accogli_tiene_il_decreto() {
    let (mut p, importo) = accolta(sanita_cara());
    let v = p.vista();
    let m = &v.ministeri[3];
    // I quattro atti costano tutti 9 quote: il margine accolto per uno paga anche gli altri.
    let altro = m
        .atti
        .iter()
        .find(|a| a.pagabile && a.id != m.agenda.as_ref().unwrap().atto)
        .expect("un altro atto pagabile");
    p.comanda(Comando::Dirotta {
        dicastero: "sanita".into(),
        atto: Some(altro.id.clone()),
        regione: None,
    })
    .unwrap();
    assert_eq!(p.economia().decreti[3], importo);
    assert!(
        p.vista()
            .calendario
            .iter()
            .any(|m| m.forma == FormaMisura::Conversione)
    );
}

#[test]
fn decreto_accolto_entra_nella_domanda_del_dicastero() {
    let (mut a, importo) = accolta(sanita_cara());
    let mut b = Partita::nuova(DatiGioco::da_testi(&sanita_cara()).unwrap(), 1);
    b.comanda(Comando::Respingi {
        dicastero: "sanita".into(),
    })
    .unwrap();
    // Fermare la Sanità in `a` ritirerebbe il decreto; in `b` non ha agenda.
    ferma_tranne(&mut a, "sanita");
    ferma_tranne(&mut b, "sanita");
    assert_eq!(a.economia().decreti[3], importo);
    a.chiudi_turno();
    b.chiudi_turno();
    let d = dati_base();
    let st = d.struttura();
    // La Sanità non ha un vettore proprio (`dati/base/dicasteri.toml`): usa la spesa pubblica nazionale.
    let quote: Vec<f64> = match &d.dicasteri()[3].settori {
        Some(m) => st
            .settori
            .iter()
            .map(|s| m.get(&s.id).copied().unwrap_or(0.0))
            .collect(),
        None => st.spesa_pubblica.clone(),
    };
    let k = (0..quote.len())
        .max_by(|&x, &y| quote[x].total_cmp(&quote[y]))
        .unwrap();
    // La sola componente settoriale: l'impulso fiscale del decreto alza già l'output gap.
    let settoriale = |p: &Partita| p.economia().gap_settori[k] - p.economia().output_gap[0];
    assert!(settoriale(&a) > settoriale(&b));
}

#[test]
fn spostare_fondi_per_decreto_muove_margine_e_bilancio() {
    // Al turno zero il bilancio in vigore è quello del 2026 (`Economia.bilancio`).
    let mut p = Partita::nuova(dati_base(), 1);
    let (margine, spesa) = (
        p.economia().bilancio.dicasteri[8].margine,
        p.economia().bilancio.dicasteri[3].spesa,
    );
    p.comanda(sposta("giustizia", "sanita", 1_000_000)).unwrap();
    assert_eq!(
        p.economia().bilancio.dicasteri[8].margine,
        margine - 1_000_000
    );
    assert_eq!(p.economia().bilancio.dicasteri[3].spesa, spesa + 1_000_000);
    assert!(
        p.vista()
            .calendario
            .iter()
            .any(|m| m.forma == FormaMisura::Conversione)
    );
}

#[test]
fn decreti_non_ammessi_rifiutati_col_nome() {
    let mut p = Partita::nuova(dati_base(), 1);
    let prima = p.salva().unwrap();
    let disp = |d: &str, g: &str| Comando::EmanaDecreto {
        decreto: Decreto::Disposizioni {
            valori: vec![ValoreProposto {
                disposizione: d.into(),
                gradino: g.into(),
            }],
        },
    };
    // Le disposizioni col nome del catalogo, come i rifiuti dell'aula (PR B); i dicasteri con l'id.
    for (c, nome) in [
        (
            disp("soglia_lista", "0"),
            "materia esclusa dal decreto: Soglia di lista",
        ),
        (
            disp("numero_parlamentari", "630_315"),
            "materia esclusa dal decreto: Numero dei parlamentari",
        ),
        (disp("eta_pensionabile", "68"), "decreto senza innesco"),
        (sposta("sanita", "sanita", 1), "stesso dicastero: sanita"),
        (sposta("sanita", "giustizia", 0), "importo non ammesso: 0"),
        (
            sposta("sanità", "giustizia", 1),
            "dicastero sconosciuto: sanità",
        ),
    ] {
        let e = p.comanda(c).unwrap_err();
        assert!(
            matches!(e, Errore::Comando(_)) && e.to_string().contains(nome),
            "{e}"
        );
    }
    assert_eq!(p.salva().unwrap(), prima);
}

#[test]
fn conversione_respinta_costa_e_non_si_ripete() {
    let f = imposta(
        imposta(sanita_cara(), "manopole", "[ft_base]\nvalore = ", "0.9"),
        "manopole",
        "[ft_max]\nvalore = ",
        "1.0",
    );
    let (mut p, _) = accolta(f);
    // Il bilancio del 2027 è già approvato al turno 0: il turno 1 non è in esercizio provvisorio,
    // dove ogni Accogli è rifiutato.
    assert!(p.bozza_bilancio().is_none());
    p.chiudi_turno();
    assert!(
        p.vista()
            .esiti
            .iter()
            .any(|e| e.forma == FormaMisura::Conversione && !e.approvata)
    );
    // Gli atti restano impagabili anche col margine di un anno: la proposta rinasce sullo stesso atto.
    assert!(p.vista().ministeri[3].proposta.is_some());
    let e = p
        .comanda(Comando::Accogli {
            dicastero: "sanita".into(),
        })
        .unwrap_err();
    assert!(
        e.to_string().contains("decreto decaduto non si ripete"),
        "{e}"
    );
}

/// Lo spostamento decaduto torna indietro (agende ferme: la Sanità ha ancora tutto il margine),
/// la Sanità proponente paga il decreto decaduto e la stessa firma non si ripete.
#[test]
fn spostamento_decaduto_torna_indietro_e_non_si_ripete() {
    let f = imposta(
        imposta(file_base(), "manopole", "[ft_base]\nvalore = ", "0.9"),
        "manopole",
        "[ft_max]\nvalore = ",
        "1.0",
    );
    let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    let margine = p.economia().bilancio.dicasteri[8].margine;
    p.comanda(sposta("giustizia", "sanita", 1_000_000)).unwrap();
    ferma_tutti(&mut p);
    p.chiudi_turno();
    assert!(
        p.vista()
            .esiti
            .iter()
            .any(|e| e.forma == FormaMisura::Conversione && !e.approvata)
    );
    assert_eq!(p.economia().bilancio.dicasteri[8].margine, margine);
    let sanita = p.vista().ministeri[3].ministro.clone().unwrap();
    assert!(
        sanita
            .variazioni
            .iter()
            .any(|v| v.causa == Causa::DecretoDecaduto)
    );
    let e = p
        .comanda(sposta("giustizia", "sanita", 1_000_000))
        .unwrap_err();
    assert!(
        e.to_string()
            .contains("decreto decaduto non si ripete: giustizia → sanita"),
        "{e}"
    );
}

/// Il decreto accolto in agenda non si sposta: il tetto è il margine residuo senza il decreto.
/// Spostato il resto, Ferma ritira il decreto e il margine resta non negativo.
#[test]
fn spostare_il_margine_del_decreto_accolto_e_rifiutato() {
    let (mut p, importo) = accolta(sanita_cara());
    let residuo = p.vista().ministeri[3].margine_residuo;
    let prima = p.salva().unwrap();
    // Anche un euro oltre il margine senza il decreto: entro il margine annuo, lo ferma solo il
    // decreto in agenda.
    for importo in [residuo, residuo - importo + 1] {
        let e = p
            .comanda(sposta("sanita", "giustizia", importo))
            .unwrap_err();
        assert!(
            e.to_string().contains("margine insufficiente in sanita"),
            "{e}"
        );
    }
    assert_eq!(p.salva().unwrap(), prima);
    p.comanda(sposta("sanita", "giustizia", residuo - importo))
        .unwrap();
    p.comanda(Comando::Ferma {
        dicastero: "sanita".into(),
    })
    .unwrap();
    assert!(p.vista().ministeri[3].margine_residuo >= 0);
    assert!(Partita::carica(&p.salva().unwrap()).is_ok());
}

/// Ritirare la conversione con la questione di fiducia posta rimborsa agli alleati il suo costo
/// d'intesa (scelta 1: si paga solo Ferma).
#[test]
fn ritiro_dopo_la_fiducia_rimborsa_l_intesa() {
    let ferma = |p: &mut Partita| {
        p.comanda(Comando::Ferma {
            dicastero: "sanita".into(),
        })
        .unwrap();
    };
    let (mut senza, _) = accolta(sanita_cara());
    ferma(&mut senza);
    let (mut con, _) = accolta(sanita_cara());
    let id = (con.vista().calendario.iter())
        .find(|m| m.forma == FormaMisura::Conversione)
        .unwrap()
        .id;
    let prima = con.intesa().valori.clone();
    con.comanda(Comando::PoniFiducia { misura: id }).unwrap();
    assert_ne!(con.intesa().valori, prima);
    ferma(&mut con);
    let (c, s) = (&con.intesa().valori, &senza.intesa().valori);
    assert!(
        c.iter().zip(s).all(|(x, y)| (x - y).abs() < 1e-9),
        "{c:?} {s:?}"
    );
}

/// Conversione in calendario ma decreto 0 in agenda ed economia: nessun decreto da convertire.
#[test]
#[ignore = "bug: controlla_decreti (partita.rs:1390) non rifiuta una conversione Accolti con decreto 0"]
fn conversione_orfana_al_caricamento_rifiutata() {
    let (p, _) = accolta(sanita_cara());
    let e = edita(&p, |j| {
        j["ministri"]["dicasteri"][3]["agenda"]["decreto"] = 0.into();
        j["economia"]["decreti"][3] = 0.into();
    })
    .err();
    assert!(
        matches!(e, Some(Errore::SalvataggioIlleggibile(_))),
        "{e:?}"
    );
}

type Modifica = Box<dyn Fn(&mut serde_json::Value)>;

/// Un salvataggio col decreto in agenda incoerente con economia, margine o calendario è illeggibile.
#[test]
fn decreto_incoerente_al_caricamento_rifiutato() {
    let (p, importo) = accolta(sanita_cara());
    assert!(Partita::carica(&p.salva().unwrap()).is_ok());
    let residuo = p.vista().ministeri[3].margine_residuo;
    let righe: Vec<(&str, Modifica)> = vec![
        (
            "decreto in agenda diverso da economia",
            Box::new(move |j| {
                j["ministri"]["dicasteri"][3]["agenda"]["decreto"] = (importo - 1).into();
            }),
        ),
        (
            "economia senza il decreto",
            Box::new(|j| j["economia"]["decreti"][3] = 0.into()),
        ),
        (
            "decreto oltre il margine residuo",
            Box::new(move |j| {
                j["ministri"]["dicasteri"][3]["agenda"]["decreto"] = (residuo + 1).into();
                j["economia"]["decreti"][3] = (residuo + 1).into();
            }),
        ),
        (
            "conversione fuori dal calendario",
            Box::new(|j| j["parlamento"]["misure"] = serde_json::json!([])),
        ),
    ];
    for (caso, modifica) in righe {
        let e = edita(&p, |j| modifica(j)).err();
        assert!(
            matches!(e, Some(Errore::SalvataggioIlleggibile(_))),
            "{caso}: {e:?}"
        );
    }
}
