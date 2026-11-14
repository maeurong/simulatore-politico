mod comune;
use comune::{caduto, calo_oltre_il_ritorno, dati_base, edita, partito, voci};
use serde_json::json;
use sim_core::esito::{Criticita, TipoVoce, VoceEsito};
use sim_core::legislatura::{Fase, Incarico};
use sim_core::{Comando, Partita};

/// L'unica voce di `tipo` nell'esito.
fn una(p: &Partita, tipo: TipoVoce) -> VoceEsito {
    let v = voci(p, tipo);
    assert_eq!(v.len(), 1, "{:?}", p.vista().esito);
    v[0].clone()
}

// Ordine sull'asse: 0 (1), 1 (2), 2 (3), 3 (4), giocatore 7 (4,5), 4 (5), 5 (6), 6 (7).
/// Nessuna coppia ha la maggioranza; fra le terne vince (7, 4, 5), la più grande (240 e 120):
/// il giocatore è il primo.
const PRIMO_C: [u32; 8] = [10, 40, 40, 50, 40, 80, 20, 120];
const PRIMO_S: [u32; 8] = [5, 20, 20, 25, 20, 40, 10, 60];
/// Fra le terne pareggiano (2, 3, 7) e (3, 7, 4), 315 seggi: vince la più a sinistra, (2, 3, 7),
/// dove il centro (3) ha più seggi del giocatore.
const SECONDO_C: [u32; 8] = [10, 40, 40, 120, 40, 80, 20, 50];
const SECONDO_S: [u32; 8] = [5, 20, 20, 60, 20, 40, 10, 25];

#[test]
fn primo_nel_ribaltone_ottieni_l_incarico() {
    let mut p = caduto(&PRIMO_C, &PRIMO_S);
    assert_eq!(p.legislatura().fase, Fase::Trattativa);
    assert_eq!(p.legislatura().incarico, Some(Incarico::Giocatore));
    assert!(p.richieste().iter().any(|r| r.partito == "centrodestra"));
    let v = una(&p, TipoVoce::Incarico);
    assert_eq!(v.fatto, "Incarico di formare il governo a Partito nuovo");
    assert_eq!(
        v.causa,
        "sei il primo per seggi della maggioranza contigua trovata dopo la caduta"
    );
    assert_eq!((v.criticita, v.precedenza), (Criticita::Critica, 3));
    p.chiudi_turno();
    assert!(p.governa());
    let v = una(&p, TipoVoce::GovernoFormato);
    assert!(
        v.fatto
            .starts_with("Nasce il governo di Partito nuovo per ribaltone, con "),
        "{}",
        v.fatto
    );
    assert_eq!(
        v.causa,
        "la tua coalizione ha la maggioranza in entrambe le Camere"
    );
}

#[test]
fn secondo_nel_ribaltone_chiedi_la_guida() {
    let mut p = caduto(&SECONDO_C, &SECONDO_S);
    assert!(matches!(
        p.legislatura().incarico,
        Some(Incarico::Guida { .. })
    ));
    let v = una(&p, TipoVoce::Incarico);
    assert_eq!(v.fatto, "Ribaltone possibile, con la guida a Centro");
    assert_eq!(
        v.causa,
        "nella maggioranza contigua trovata dopo la caduta Centro ha più seggi di te"
    );
    p.comanda(Comando::ChiediGuida).unwrap();
    assert_eq!(p.legislatura().incarico, Some(Incarico::Giocatore));
    p.chiudi_turno();
    assert!(p.governa());
}

#[test]
fn senza_scelta_il_default_chiede_la_guida() {
    let mut a = caduto(&SECONDO_C, &SECONDO_S);
    let mut b = caduto(&SECONDO_C, &SECONDO_S);
    // Il default si valuta alla chiusura, dopo il motore; `b` lo valuta a inizio turno: col seme
    // del test le due valutazioni coincidono.
    let accetta = b.comanda(Comando::ChiediGuida).is_ok();
    a.chiudi_turno();
    assert_eq!(a.governa(), accetta);
}

#[test]
fn rifiutare_il_ribaltone_lascia_il_giocatore_fuori() {
    let mut p = caduto(&SECONDO_C, &SECONDO_S);
    p.comanda(Comando::RifiutaRibaltone).unwrap();
    p.chiudi_turno();
    assert!(!p.governa());
    assert!(!p.governo().al_governo(p.giocatore()));
    assert_eq!(p.legislatura().fase, Fase::Ordinaria);
}

/// Il ribaltone ceduto si forma come dopo il voto: il centrosinistra lascia la sinistra, sua
/// compagna al voto, e paga la rottura dell'impegno.
#[test]
fn la_guida_ceduta_paga_le_rotture() {
    let mut p = caduto(&SECONDO_C, &SECONDO_S);
    p.comanda(Comando::ChiediGuida).unwrap();
    let prima = p.partiti().to_vec();
    p.chiudi_turno();
    let rottura = dati_base().manopola("valence_rottura_impegno");
    let calo = calo_oltre_il_ritorno(&prima, &p, partito("centrosinistra"));
    assert!((calo - rottura).abs() < 1e-9, "{calo}");
    let v = voci(&p, TipoVoce::RotturaDellImpegno);
    let cs = (v.iter())
        .find(|v| v.partito.as_deref() == Some("centrosinistra"))
        .unwrap();
    assert_eq!(
        cs.fatto,
        "Centrosinistra rompe l'impegno pre-elettorale con Sinistra, che resta fuori dal \
         governo; valence -0,30 a Centrosinistra; perde 10,0 punti d'intesa"
    );
    assert_eq!(
        cs.causa,
        "la coalizione dichiarata prima del voto con Sinistra"
    );
    assert_eq!(cs.criticita, Criticita::Notevole);
}

/// La sinistra guida la sua coalizione dell'ultimo voto (sinistra e centrosinistra): nel
/// ribaltone (1, 2, 3, 7), dove è prima per seggi, non cede la guida.
const SINISTRA_C: [u32; 8] = [10, 100, 40, 50, 40, 60, 20, 80];
const SINISTRA_S: [u32; 8] = [5, 50, 20, 25, 20, 30, 10, 40];

#[test]
fn chi_guida_la_propria_coalizione_non_cede() {
    let mut p = caduto(&SINISTRA_C, &SINISTRA_S);
    assert_eq!(
        p.legislatura().incarico,
        Some(Incarico::Guida {
            coalizione: vec![1, 2, 3, 7]
        })
    );
    let prima = p.salva().unwrap();
    let e = p.comanda(Comando::ChiediGuida).unwrap_err().to_string();
    assert!(
        e.contains("Sinistra non ti cede la guida: guiderebbe la propria coalizione"),
        "{e}"
    );
    assert_eq!(p.salva().unwrap(), prima);
    p.chiudi_turno();
    assert!(!p.governa());
    let v = una(&p, TipoVoce::GuidaRifiutata);
    assert_eq!(v.partito.as_deref(), Some("sinistra"));
    assert_eq!(v.fatto, "Sinistra non ti cede la guida del ribaltone");
    let (_, causa) = e.split_once("la guida: ").unwrap();
    assert_eq!(v.causa, causa);
    assert_eq!((v.criticita, v.precedenza), (Criticita::Critica, 3));
}

#[test]
fn senza_guida_aperta_niente_da_chiedere() {
    let mut p = caduto(&PRIMO_C, &PRIMO_S);
    for c in [Comando::ChiediGuida, Comando::RifiutaRibaltone] {
        let e = p.comanda(c).unwrap_err().to_string();
        assert!(e.contains("Ribaltone: nessuna guida da chiedere"), "{e}");
    }
}

/// `carica` rifiuta i campi del 7b incoerenti, ognuno col suo messaggio.
#[test]
fn carica_rifiuta_crisi_e_maggioranza_incoerenti() {
    let p = caduto(&SECONDO_C, &SECONDO_S);
    let turno = p.vista().turno;
    let guida = |c: serde_json::Value| json!({ "Guida": { "coalizione": c } });
    let casi: [(&str, serde_json::Value, &str); 7] = [
        (
            "crisi",
            json!(turno + 1),
            "legislatura.crisi fuori dal turno in corso",
        ),
        (
            "maggioranza",
            json!([2, 3, 7]),
            "legislatura.maggioranza fuori dalla trattativa col giocatore",
        ),
        ("incarico", guida(json!([])), "con una coalizione vuota"),
        (
            "incarico",
            json!({ "Ia": { "coalizione": [2, 2, 3] } }),
            "un partito ripetuto",
        ),
        (
            "incarico",
            guida(json!([2, 3, 3, 7])),
            "un partito ripetuto",
        ),
        (
            "incarico",
            guida(json!([2, 3])),
            "di guida senza il giocatore",
        ),
        (
            "incarico",
            guida(json!([2, 7])),
            "col giocatore primo per seggi",
        ),
    ];
    for (campo, valore, messaggio) in casi {
        let e = edita(&p, |j| j["legislatura"][campo] = valore).unwrap_err();
        assert!(e.to_string().contains(messaggio), "{campo}: {e}");
    }
}
