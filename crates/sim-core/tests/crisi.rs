mod comune;
use comune::{chiudi_col_bilancio, con_seggi, dati_base, edita, partito, presenta};
use sim_core::legislatura::Fase;
use sim_core::ministri::Causa;
use sim_core::parlamento::{CausaIntesa, Decreto, FormaMisura, ValoreProposto};
use sim_core::{Comando, Partita};

/// Destra 100 e centrodestra 101 alla Camera (201 su 400), 50 e 51 al Senato: senza il
/// centrodestra la destra resta sola; l'estrema destra (110, 55) basta a rifare la maggioranza.
const CRISI_C: [u32; 7] = [10, 40, 29, 10, 101, 100, 110];
const CRISI_S: [u32; 7] = [5, 20, 14, 5, 51, 50, 55];

/// Come `con_seggi`, con l'estrema destra (identitaria) resa mista: entrare in un governo a un
/// punto di distanza non le conviene più (vocazione, piano 8a), e queste prove vogliono che entri.
fn con_seggi_ed_mista(p: &Partita, camera: &[u32], senato: &[u32]) -> Partita {
    let ed = partito("estrema_destra");
    edita(&con_seggi(p, camera, senato), |j| {
        j["partiti"][ed]["vocazione"] = "mista".into();
    })
    .unwrap()
}

fn in_crisi() -> Partita {
    let p = con_seggi_ed_mista(&Partita::nuova(dati_base(), 1), &CRISI_C, &CRISI_S);
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
    let p = con_seggi_ed_mista(&Partita::nuova(dati_base(), 1), &CRISI_C, &CRISI_S);
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

/// Vocazione (piano 8a): l'estrema destra identitaria, a un punto di distanza, non entra per sole
/// poltrone; resa mista (`con_seggi_ed_mista`) entra.
#[test]
fn identitaria_a_distanza_1_rifiuta_l_allargamento_per_sole_poltrone() {
    let p = con_seggi(&Partita::nuova(dati_base(), 1), &CRISI_C, &CRISI_S);
    let cd = partito("centrodestra");
    let mut p = edita(&p, |j| j["intesa"]["valori"][cd] = serde_json::json!(10.0)).unwrap();
    p.chiudi_turno();
    assert_eq!(p.legislatura().crisi, Some(1));
    let voce = (p.vista().disponibilita.into_iter())
        .find(|v| v.partito == "estrema_destra")
        .unwrap();
    assert!(!voce.disponibile);
    let e = p
        .comanda(Comando::Allarga {
            partiti: vec!["estrema_destra".into()],
        })
        .unwrap_err();
    assert!(e.to_string().contains("Estrema destra non entra"), "{e}");
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
    let p = con_seggi_ed_mista(&Partita::nuova(dati_base(), 1), &CRISI_C, &CRISI_S);
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
    assert!(!p.legislatura().dimissioni);
    assert_eq!(p.legislatura().crisi, None);
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

/// Spec riga 342 e decisione 2 di Mario: in crisi il bilancio non si presenta. Il centrodestra
/// esce in ottobre-dicembre senza bilancio presentato: a gennaio, in esercizio provvisorio, la
/// caduta è certa, l'allargamento inutile e la disponibilità vuota.
#[test]
fn crisi_in_esercizio_provvisorio_senza_allargamento() {
    let mut p = con_seggi_ed_mista(&Partita::nuova(dati_base(), 1), &CRISI_C, &CRISI_S);
    for _ in 0..4 {
        p.chiudi_turno(); // fino a ottobre-dicembre 2027, senza bilancio
    }
    let cd = partito("centrodestra");
    let mut p = edita(&p, |j| j["intesa"]["valori"][cd] = serde_json::json!(10.0)).unwrap();
    p.chiudi_turno();
    assert_eq!(p.legislatura().crisi, Some(p.vista().turno));
    assert!(p.governa());
    assert!(p.vista().disponibilita.is_empty());
    let prima = p.salva().unwrap();
    let e = p
        .comanda(Comando::Allarga {
            partiti: vec!["estrema_destra".into()],
        })
        .unwrap_err()
        .to_string();
    assert!(
        e.contains(
            "Estrema destra: allargamento inutile, senza bilancio in esercizio provvisorio il \
             governo cade a fine turno"
        ),
        "{e}"
    );
    assert_eq!(p.salva().unwrap(), prima);
    p.chiudi_turno();
    assert!(!p.governa());
    assert_ne!(p.legislatura().fase, Fase::Ordinaria);
    assert!(!p.legislatura().dimissioni && p.legislatura().crisi.is_none());
    assert!(!(p.vista().esiti.iter()).any(|e| e.forma == FormaMisura::Sfiducia));
}

/// Destra e centrodestra con 200 seggi su 400 alla Camera e 101 su 200 al Senato: la sfiducia
/// automatica pareggia alla Camera e perde al Senato. Respinta, il governo resta senza
/// maggioranza e la crisi si riapre (spec, «Governo, ministri e coalizione»: niente governo di
/// minoranza).
#[test]
fn sfiducia_pareggiata_riapre_la_crisi() {
    let c = [10, 40, 30, 20, 100, 100, 100];
    let s = [5, 20, 14, 10, 51, 50, 50];
    let p = con_seggi_ed_mista(&Partita::nuova(dati_base(), 1), &c, &s);
    let mut p = edita(&p, |j| j["legislatura"]["crisi"] = 0.into()).unwrap();
    p.chiudi_turno();
    let sfiducia = (p.vista().esiti.into_iter()).find(|e| e.forma == FormaMisura::Sfiducia);
    assert!(sfiducia.is_some_and(|e| !e.approvata));
    assert!(p.governa());
    assert_eq!(p.legislatura().crisi, Some(1));
    p.comanda(Comando::Allarga {
        partiti: vec!["estrema_destra".into()],
    })
    .unwrap();
    p.chiudi_turno();
    assert!(p.governa());
    assert!(p.governo().al_governo(partito("estrema_destra")));
    assert_eq!(p.legislatura().crisi, None);
}

/// Il centro (3), con la Sanità, esce e apre la crisi: destra e centrodestra, rimasti, hanno 181
/// seggi su 400; l'estrema destra (110) rifà la maggioranza.
fn col_centro_in_crisi() -> Partita {
    let c = [10, 40, 39, 20, 81, 100, 110];
    let s = [5, 20, 19, 10, 41, 50, 55];
    let p = con_seggi_ed_mista(&Partita::nuova(dati_base(), 1), &c, &s);
    let mut p = edita(&p, |j| {
        j["governo"]["dicasteri"][3] = serde_json::json!(3);
        j["governo"]["coalizione"]
            .as_array_mut()
            .unwrap()
            .push(serde_json::json!(3));
        j["ministri"]["dicasteri"][3]["ministro"]["partito"] = serde_json::json!(3);
        j["intesa"]["valori"][3] = serde_json::json!(10.0);
    })
    .unwrap();
    p.chiudi_turno();
    assert_eq!(p.legislatura().crisi, Some(1));
    p
}

/// La Sanità del centro è in interim presso il presidente fino al rimpasto: nel turno di crisi il
/// centrodestra, che resta, non paga la quota scoperta.
#[test]
fn in_crisi_niente_quota_scoperta() {
    let mut p = col_centro_in_crisi();
    p.chiudi_turno();
    let cd = partito("centrodestra");
    let quota = (p.intesa().variazioni.iter())
        .any(|v| v.partito == cd && v.turno == 1 && v.causa == CausaIntesa::QuotaScoperta);
    assert!(!quota);
}

/// A fine crisi l'estrema destra entra e l'ultimatum che le soglie pongono al centrodestra (39,9
/// sotto 40) resta, anche se il ritorno lo riporta sopra la soglia nello stesso turno.
#[test]
fn a_fine_crisi_l_ultimatum_degli_alleati_resta() {
    let cd = partito("centrodestra");
    let p = col_centro_in_crisi();
    let mut p = edita(&p, |j| j["intesa"]["valori"][cd] = serde_json::json!(39.9)).unwrap();
    p.comanda(Comando::Allarga {
        partiti: vec!["estrema_destra".into()],
    })
    .unwrap();
    p.chiudi_turno();
    assert!(p.governa() && p.governo().al_governo(partito("estrema_destra")));
    assert!(p.intesa().valori[cd] >= 40.0, "{}", p.intesa().valori[cd]);
    assert!(p.intesa().ultimatum[cd].is_some());
}

/// Decisione 2 di Mario, per chiunque governi: in ottobre-dicembre il governo IA presenta il
/// bilancio, tranne in crisi.
#[test]
fn il_governo_ia_in_crisi_non_presenta_il_bilancio() {
    let mut p = Partita::nuova(dati_base(), 1);
    for _ in 0..4 {
        p.chiudi_turno(); // fino a ottobre-dicembre 2027
    }
    let cd = partito("centrodestra");
    let bilancio = |crisi: Option<u32>| {
        let mut p = edita(&p, |j| {
            j["governo"]["presidente"] = cd.into();
            j["legislatura"]["crisi"] = serde_json::json!(crisi);
        })
        .unwrap();
        p.chiudi_turno();
        (p.vista().esiti.iter()).any(|e| e.forma == FormaMisura::Bilancio)
    };
    assert!(bilancio(None));
    assert!(!bilancio(Some(4)));
}

/// La Sanità del centro uscente è già in interim, con una rosa del centro: dopo il rimpasto resta
/// la causa di prima, la rosa è del nuovo titolare e il salvataggio si ricarica.
#[test]
fn rimpasto_con_rosa_aperta_si_ricarica() {
    let p = col_centro_che_esce();
    let mut p = edita(&p, |j| {
        let st = &mut j["ministri"]["dicasteri"][3];
        let m = st["ministro"].take();
        st["rosa"] = serde_json::json!([m.clone(), m.clone(), m]);
        st["causa_dimissioni"] = "indicatore_in_calo".into();
        st["agenda"] = serde_json::Value::Null;
        st["proposta"] = serde_json::Value::Null;
    })
    .unwrap();
    assert_eq!(p.ministri().dicasteri[3].rosa.len(), 3);
    p.chiudi_turno();
    assert!(!p.governo().al_governo(3));
    let titolare = p.governo().dicasteri[3];
    let st = &p.ministri().dicasteri[3];
    assert!(st.rosa.iter().all(|x| x.partito == titolare));
    assert_eq!(st.causa_dimissioni, Some(Causa::IndicatoreInCalo));
    assert_eq!(Partita::carica(&p.salva().unwrap()).unwrap(), p);
}

/// La partita di prova a `scadenza − 1`, col bilancio presentato a ogni ottobre-dicembre.
fn vigilia_del_voto() -> Partita {
    let mut p = Partita::nuova(dati_base(), 1);
    while p.vista().turno < 18 {
        chiudi_col_bilancio(&mut p);
    }
    p
}

/// Il centrodestra esce alla vigilia del voto e apre la crisi nel turno del voto: lì `Allarga` è
/// rifiutato e vince il voto, senza sfiducia.
#[test]
fn crisi_alla_vigilia_del_voto_vince_il_voto() {
    let cd = partito("centrodestra");
    let p = con_seggi_ed_mista(&vigilia_del_voto(), &CRISI_C, &CRISI_S);
    let mut p = edita(&p, |j| j["intesa"]["valori"][cd] = serde_json::json!(10.0)).unwrap();
    p.chiudi_turno();
    assert!(!p.governo().al_governo(cd));
    assert_eq!(p.legislatura().crisi, Some(19));
    assert!(p.governa() && p.legislatura().turno_di_voto(19));
    let prima = p.salva().unwrap();
    let e = p
        .comanda(Comando::Allarga {
            partiti: vec!["estrema_destra".into()],
        })
        .unwrap_err()
        .to_string();
    assert!(e.contains("o in crisi mentre governi"), "{e}");
    assert_eq!(p.salva().unwrap(), prima);
    p.chiudi_turno(); // si vota
    let l = p.legislatura();
    assert_eq!(l.elezioni.len(), 1);
    assert_eq!(l.crisi, None);
    assert_eq!(l.fase, Fase::Trattativa);
    assert!(!(p.vista().esiti.iter()).any(|e| e.forma == FormaMisura::Sfiducia));
    assert!(Partita::carica(&p.salva().unwrap()).is_ok());
}

/// Nel turno del voto nessuno esce: il centrodestra a intesa 10 resta nel governo uscente.
#[test]
fn nel_turno_del_voto_nessuna_uscita() {
    let cd = partito("centrodestra");
    let mut p = vigilia_del_voto();
    chiudi_col_bilancio(&mut p);
    let p = con_seggi_ed_mista(&p, &CRISI_C, &CRISI_S);
    let mut p = edita(&p, |j| j["intesa"]["valori"][cd] = serde_json::json!(10.0)).unwrap();
    assert!(p.legislatura().turno_di_voto(p.vista().turno));
    p.chiudi_turno(); // si vota
    assert_eq!(p.legislatura().elezioni.len(), 1);
    assert_eq!(p.legislatura().crisi, None);
    assert!(p.governo().al_governo(cd));
}

/// Col governo IA in crisi il giocatore non allarga.
#[test]
fn senza_governare_niente_allargamento_in_crisi() {
    let cd = partito("centrodestra");
    let p = Partita::nuova(dati_base(), 1);
    let mut p = edita(&p, |j| {
        j["governo"]["presidente"] = cd.into();
        j["legislatura"]["crisi"] = 0.into();
    })
    .unwrap();
    let prima = p.salva().unwrap();
    let e = p
        .comanda(Comando::Allarga {
            partiti: vec!["estrema_destra".into()],
        })
        .unwrap_err()
        .to_string();
    assert!(e.contains("o in crisi mentre governi"), "{e}");
    assert_eq!(p.salva().unwrap(), prima);
}
