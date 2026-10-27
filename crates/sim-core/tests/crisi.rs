mod comune;
use comune::{con_seggi, dati_base, edita, partito, presenta};
use sim_core::legislatura::Fase;
use sim_core::ministri::Causa;
use sim_core::parlamento::{Decreto, FormaMisura, ValoreProposto};
use sim_core::{Comando, Partita};

/// Destra 100 e centrodestra 101 alla Camera (201 su 400), 50 e 51 al Senato: senza il
/// centrodestra la destra resta sola; l'estrema destra (110, 55) basta a rifare la maggioranza.
const CRISI_C: [u32; 7] = [10, 40, 29, 10, 101, 100, 110];
const CRISI_S: [u32; 7] = [5, 20, 14, 5, 51, 50, 55];

fn in_crisi() -> Partita {
    let p = con_seggi(&Partita::nuova(dati_base(), 1), &CRISI_C, &CRISI_S);
    let cd = partito("centrodestra");
    let mut p = edita(&p, |j| j["intesa"]["valori"][cd] = serde_json::json!(10.0)).unwrap();
    p.chiudi_turno();
    assert_eq!(p.legislatura().crisi, Some(1));
    p
}

#[test]
fn un_alleato_sotto_25_esce_e_apre_la_crisi() {
    let p = in_crisi();
    let cd = partito("centrodestra");
    assert!(!p.governo().al_governo(cd));
    for k in [6, 9] {
        assert_eq!(p.governo().dicasteri[k], p.governo().presidente);
        assert!(p.ministri().dicasteri[k].ministro.is_none());
        let causa = p.ministri().dicasteri[k].causa_dimissioni;
        assert_eq!(causa, Some(Causa::UscitaDalGoverno));
    }
    assert_eq!(p.intesa().ultimatum[cd], None);
    assert!(p.vista().crisi);
}

#[test]
fn in_crisi_solo_ordinaria_amministrazione() {
    let mut p = in_crisi();
    let prima = p.salva().unwrap();
    let valori = vec![ValoreProposto {
        disposizione: "tetto_contante".into(),
        gradino: "1000".into(),
    }];
    let decreto = Decreto::Fondi {
        da: "sanita".into(),
        a: "interno".into(),
        importo: 1,
    };
    let rifiutati = [
        Comando::PresentaLegge {
            valori: valori.clone(),
        },
        Comando::PresentaBilancio {
            bilancio: p.economia().bilancio.clone(),
        },
        Comando::EmanaDecreto {
            decreto: decreto.clone(),
        },
        Comando::PoniFiducia { misura: 0 },
        Comando::Accogli {
            dicastero: "sanita".into(),
        },
    ];
    let mut errori: Vec<String> = Vec::new();
    for c in rifiutati {
        errori.push(p.comanda(c).unwrap_err().to_string());
    }
    errori.push(p.anteprima(&valori).unwrap_err().to_string());
    errori.push(p.anteprima_decreto(&decreto).unwrap_err().to_string());
    errori.push(p.anteprima_accogli("sanita").unwrap_err().to_string());
    for e in &errori {
        assert!(e.contains("ordinaria amministrazione"), "{e}");
    }
    assert_eq!(p.salva().unwrap(), prima);
    // Ammessi: il controllo sui ministri e le dimissioni.
    let con_proposta = (p.vista().ministeri.into_iter())
        .find(|m| m.proposta.is_some())
        .unwrap();
    p.comanda(Comando::Respingi {
        dicastero: con_proposta.dicastero,
    })
    .unwrap();
    p.comanda(Comando::Dimettiti).unwrap();
}

/// Ticket 09, riga 45: la legge col voto nel turno di crisi resta ferma un turno, senza decadere.
#[test]
fn in_crisi_le_leggi_slittano() {
    let p = con_seggi(&Partita::nuova(dati_base(), 1), &CRISI_C, &CRISI_S);
    let cd = partito("centrodestra");
    let mut p = edita(&p, |j| j["intesa"]["valori"][cd] = serde_json::json!(10.0)).unwrap();
    presenta(&mut p, &[("tetto_contante", "1000")]).unwrap();
    let id = p.parlamento().misure.last().unwrap().id;
    assert_eq!(p.parlamento().misure.last().unwrap().voto, 1);
    p.chiudi_turno();
    assert_eq!(p.legislatura().crisi, Some(1));
    p.comanda(Comando::Allarga {
        partiti: vec!["estrema_destra".into()],
    })
    .unwrap();
    p.chiudi_turno();
    assert!(!p.parlamento().esiti.iter().any(|e| e.misura == id));
    let m = p.parlamento().misure.iter().find(|m| m.id == id).unwrap();
    assert_eq!(m.voto, 2);
}

#[test]
fn crisi_senza_allargamento_finisce_in_sfiducia() {
    let mut p = in_crisi();
    p.chiudi_turno();
    assert!((p.vista().esiti.iter()).any(|e| e.forma == FormaMisura::Sfiducia && e.approvata));
    assert!(!p.governa());
    assert_ne!(p.legislatura().fase, Fase::Ordinaria);
}

#[test]
fn allargare_in_crisi_salva_il_governo() {
    let mut p = in_crisi();
    let ed = partito("estrema_destra");
    let disponibile = p
        .vista()
        .disponibilita
        .into_iter()
        .find(|v| v.partito == "estrema_destra");
    assert!(disponibile.is_some_and(|v| v.disponibile));
    p.comanda(Comando::Allarga {
        partiti: vec!["estrema_destra".into()],
    })
    .unwrap();
    p.chiudi_turno();
    assert!(p.governa());
    assert!(p.governo().al_governo(ed));
    assert_eq!(p.legislatura().crisi, None);
    assert!(p.legislatura().allargati.is_empty());
    // Il rimpasto dà dicasteri al nuovo alleato, in interim con una rosa del suo partito.
    assert!(p.governo().dicasteri.contains(&ed));
    for (k, &j) in p.governo().dicasteri.iter().enumerate() {
        let st = &p.ministri().dicasteri[k];
        if j == ed {
            assert!(st.ministro.is_none());
            assert!(!st.rosa.is_empty() && st.rosa.iter().all(|x| x.partito == ed));
            let causa = st.causa_dimissioni;
            assert!(matches!(
                causa,
                Some(Causa::UscitaDalGoverno | Causa::Rimpasto)
            ));
        }
    }
}

/// L'allargato entra come alla formazione: giudizio da zero, intesa alla base. L'estrema destra
/// ha corso da sola: nessuna rottura.
#[test]
fn chi_entra_in_crisi_riparte_da_zero() {
    let ed = partito("estrema_destra");
    let mut p = edita(&in_crisi(), |j| {
        j["elettorato"]["gruppi"][0]["giudizio"][ed] = serde_json::json!(-0.5);
        j["intesa"]["valori"][ed] = serde_json::json!(30.0);
    })
    .unwrap();
    p.comanda(Comando::Allarga {
        partiti: vec!["estrema_destra".into()],
    })
    .unwrap();
    p.chiudi_turno();
    assert!(p.governo().al_governo(ed));
    assert!(p.elettorato().gruppi.iter().all(|g| g.giudizio[ed] == 0.0));
    assert_eq!(p.intesa().valori[ed], dati_base().manopola("intesa_base"));
    assert_eq!(p.intesa().ultimatum[ed], None);
}

/// Il centro (3) entra con la Sanità (3), poi esce: destra e centrodestra reggono (201 su 400,
/// 101 su 200), alla pari di seggi, e il centrodestra chiede più dei suoi due dicasteri.
fn col_centro_che_esce() -> Partita {
    let p = con_seggi(&Partita::nuova(dati_base(), 1), &CRISI_C, &CRISI_S);
    edita(&p, |j| {
        j["governo"]["dicasteri"][3] = serde_json::json!(3);
        j["governo"]["coalizione"]
            .as_array_mut()
            .unwrap()
            .push(serde_json::json!(3));
        j["ministri"]["dicasteri"][3]["ministro"]["partito"] = serde_json::json!(3);
        j["intesa"]["valori"][3] = serde_json::json!(10.0);
    })
    .unwrap()
}

#[test]
fn rimpasto_se_la_maggioranza_regge() {
    let mut p = col_centro_che_esce();
    let prima = p.governo().dicasteri.clone();
    p.chiudi_turno();
    assert!(!p.governo().al_governo(3));
    assert_ne!(p.governo().dicasteri[3], 3);
    assert_eq!(p.legislatura().crisi, None);
    assert!(p.governa());
    // Il centrodestra, che resta, tiene Ambiente ed Energia ed Esteri e Difesa.
    let cd = partito("centrodestra");
    assert_eq!(
        (p.governo().dicasteri[6], p.governo().dicasteri[9]),
        (cd, cd)
    );
    let causa = |k: usize| p.ministri().dicasteri[k].causa_dimissioni;
    assert!(p.ministri().dicasteri[3].ministro.is_none());
    assert_eq!(causa(3), Some(Causa::UscitaDalGoverno));
    let cambiati: Vec<usize> = (0..prima.len())
        .filter(|&k| k != 3 && prima[k] != p.governo().dicasteri[k])
        .collect();
    assert_eq!(cambiati, [1, 2, 4, 8]);
    for k in cambiati {
        assert!(p.ministri().dicasteri[k].ministro.is_none());
        assert_eq!(causa(k), Some(Causa::Rimpasto), "dicastero {k}");
    }
}

#[test]
fn rimpasto_con_interim_si_ricarica() {
    // Come sopra, con le dimissioni chieste a un ministro della destra nello stesso turno.
    let mut p = col_centro_che_esce();
    p.comanda(Comando::ChiediDimissioni {
        dicastero: "giustizia".into(),
    })
    .unwrap();
    p.chiudi_turno();
    let q = Partita::carica(&p.salva().unwrap()).unwrap();
    assert_eq!(p, q);
    // La Giustizia (8), già in interim, passa al centrodestra: resta la causa di prima, e la rosa
    // è del partito nuovo.
    let cd = partito("centrodestra");
    let st = &p.ministri().dicasteri[8];
    assert_eq!(p.governo().dicasteri[8], cd);
    assert_eq!(st.causa_dimissioni, Some(Causa::DimissioniChieste));
    assert!(!st.rosa.is_empty() && st.rosa.iter().all(|x| x.partito == cd));
}

#[test]
fn due_cadute_nello_stesso_turno() {
    let mut p = in_crisi();
    p.comanda(Comando::Dimettiti).unwrap();
    p.chiudi_turno();
    assert!(
        !p.vista()
            .esiti
            .iter()
            .any(|e| e.forma == FormaMisura::Sfiducia)
    );
    assert!(!p.governa());
    assert!(Partita::carica(&p.salva().unwrap()).is_ok());
}

#[test]
fn salvare_a_meta_crisi() {
    let mut a = in_crisi();
    a.comanda(Comando::Allarga {
        partiti: vec!["estrema_destra".into()],
    })
    .unwrap();
    let mut b = Partita::carica(&a.salva().unwrap()).unwrap();
    a.chiudi_turno();
    b.chiudi_turno();
    assert_eq!(a.salva().unwrap(), b.salva().unwrap());
}
