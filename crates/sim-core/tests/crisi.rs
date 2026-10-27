mod comune;
use comune::{con_seggi, dati_base, edita, partito, presenta};
use sim_core::legislatura::Fase;
use sim_core::ministri::Causa;
use sim_core::parlamento::FormaMisura;
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
    let e = presenta(&mut p, &[("tetto_contante", "1000")]).unwrap_err();
    assert!(e.to_string().contains("ordinaria amministrazione"), "{e}");
    assert_eq!(p.salva().unwrap(), prima);
    p.comanda(Comando::Ferma {
        dicastero: "sanita".into(),
    })
    .unwrap();
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

/// Il centro (3) entra con la Sanità (3), poi esce: destra e centrodestra reggono.
fn col_centro_che_esce() -> Partita {
    let p = Partita::nuova(dati_base(), 1);
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
    p.chiudi_turno();
    assert!(!p.governo().al_governo(3));
    assert_ne!(p.governo().dicasteri[3], 3);
    assert!(p.ministri().dicasteri[3].ministro.is_none());
    assert_eq!(p.legislatura().crisi, None);
    assert!(p.governa());
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
