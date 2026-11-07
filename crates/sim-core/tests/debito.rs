//! La crisi del debito («Aste scoperte») e il programma del MES (piano 9, Task 9a).

mod comune;

use comune::{
    chiudi_col_bilancio, con_ingresso, dati_base, edita, file_base, imposta, presenta, sostituisci,
};
use serde_json::{Value, json};
use sim_core::dati::{DatiGioco, FileDati};
use sim_core::parlamento::{CausaIntesa, Decreto, FormaMisura, ValoreProposto};
use sim_core::{Comando, Partita};

fn json_di(p: &Partita) -> Value {
    serde_json::from_str(&p.salva().unwrap()).unwrap()
}

fn aste(p: &Partita) -> Vec<Value> {
    let k = dati_base().evento("aste_scoperte").unwrap();
    json_di(p)["eventi"]["aperti"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|a| a["evento"] == k)
        .cloned()
        .collect()
}

/// PDE inadempiente (niente TPI) e un premio di sfiducia che porta lo spread oltre la soglia:
/// dopo una chiusura le Aste scoperte sono aperte. Col programma del MES il premio non si vede.
fn dati_con_premio() -> FileDati {
    sostituisci(
        con_ingresso(
            con_ingresso(file_base(), "debito_pil", 160.0),
            "premio_sfiducia",
            400.0,
        ),
        "scenario",
        "pde = \"sospesa\"",
        "pde = \"inadempiente\"",
    )
}

fn con_premio() -> Partita {
    Partita::nuova(DatiGioco::da_testi(&dati_con_premio()).unwrap(), 1)
}

/// Le Aste scoperte appena nate.
fn in_crisi() -> Partita {
    let mut p = con_premio();
    chiudi_col_bilancio(&mut p);
    assert_eq!(aste(&p).len(), 1, "spread {}", p.economia().spread);
    p
}

/// Il programma del MES appena partito: la crisi chiusa col default.
fn col_mes() -> Partita {
    let mut p = in_crisi();
    chiudi_col_bilancio(&mut p);
    p
}

fn durata() -> u32 {
    dati_base().manopola("durata_mes") as u32
}

#[test]
fn spread_oltre_soglia_apre_le_aste_scoperte() {
    let p = in_crisi();
    assert!(p.economia().spread >= 550.0);
    let a = &aste(&p)[0];
    assert_eq!(a["causa"]["tipo"], "spread");
    assert_eq!(a["causa"]["soglia"], 550);
    assert_eq!(a["tono"], "grave");
    let v = p.vista();
    let e = v.eventi.iter().find(|e| e.nome == "Aste scoperte").unwrap();
    assert!(e.causa.contains("soglia 550 pb"), "{}", e.causa);
}

#[test]
fn il_mes_e_il_default() {
    let prima = in_crisi();
    let turno = prima.vista().turno;
    let alleati = prima.alleati();
    assert!(!alleati.is_empty());
    // Il TPI non c'è (PDE inadempiente) e il premio c'è: col programma vale zero lo stesso.
    assert!(prima.economia().premio_sfiducia > 0.0);
    let mut p = prima;
    chiudi_col_bilancio(&mut p);
    assert_eq!(p.in_vigore("eta_pensionabile"), Some("70"));
    assert_eq!(
        p.in_vigore("indicizzazione_pensioni"),
        Some("bloccata_sopra_soglia")
    );
    assert_eq!(p.economia().programma_mes, Some(turno + durata()));
    assert_eq!(p.economia().premio_sfiducia, 0.0);
    assert!(p.economia().regole_ue.pde.e_inadempiente());
    for j in alleati {
        let v: Vec<_> = (p.intesa().variazioni.iter())
            .filter(|v| v.partito == j && v.causa == CausaIntesa::ProgrammaMes)
            .collect();
        assert_eq!(v.len(), 1, "alleato {j}");
        assert!((v[0].delta + 10.0).abs() < 1e-9, "{}", v[0].delta);
    }
}

#[test]
fn una_crisi_del_debito_alla_volta() {
    // Soglia bassa: lo spread dei fondamentali sta oltre la soglia anche col premio a zero.
    let f = imposta(
        dati_con_premio(),
        "manopole",
        "[soglia_crisi_debito]\nvalore = ",
        "300.0",
    );
    let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    chiudi_col_bilancio(&mut p);
    assert_eq!(aste(&p).len(), 1);
    // Evento ancora aperto, spread oltre la soglia: la chiusura che lo risolve non ne apre un altro.
    chiudi_col_bilancio(&mut p);
    assert!(p.economia().programma_mes.is_some());
    assert!(p.economia().spread >= 300.0, "{}", p.economia().spread);
    assert!(aste(&p).is_empty());
    // Programma in corso: niente evento nuovo.
    chiudi_col_bilancio(&mut p);
    assert!(aste(&p).is_empty());
    // Programma scaduto e spread ancora oltre la soglia: la crisi si riapre.
    let fine = p.economia().programma_mes.unwrap();
    let mut p = edita(&p, |j| {
        j["economia"]["programma_mes"] = json!(p.vista().turno)
    })
    .unwrap();
    assert!(fine > p.vista().turno);
    chiudi_col_bilancio(&mut p);
    assert_eq!(p.economia().programma_mes, None);
    assert_eq!(aste(&p).len(), 1);
}

#[test]
fn un_salvataggio_con_la_crisi_aperta_e_il_programma_in_corso_e_rifiutato() {
    let p = in_crisi();
    let turno = p.vista().turno;
    let e = edita(&p, |j| j["economia"]["programma_mes"] = json!(turno + 1)).unwrap_err();
    assert!(e.to_string().contains("Aste scoperte aperte"), "{e}");
}

fn decreto_su(disposizione: &str, gradino: &str) -> Comando {
    Comando::EmanaDecreto {
        decreto: Decreto::Disposizioni {
            valori: vec![ValoreProposto {
                disposizione: disposizione.into(),
                gradino: gradino.into(),
            }],
        },
    }
}

#[test]
fn con_la_crisi_aperta_il_decreto_apre_le_riforme() {
    let mut p = in_crisi();
    // Una disposizione ordinaria di un dicastero qualsiasi.
    let d = dati_base();
    let k = d.disposizione("cittadinanza").unwrap();
    let altro = &d.disposizioni()[k]
        .gradini
        .iter()
        .find(|g| Some(g.id.as_str()) != p.in_vigore("cittadinanza"));
    let gradino = altro.unwrap().id.clone();
    p.comanda(decreto_su("cittadinanza", &gradino)).unwrap();
    // La materia esclusa resta rifiutata.
    let e = d
        .disposizioni()
        .iter()
        .find(|x| x.norma != sim_core::disposizioni::Norma::Ordinaria)
        .unwrap();
    let r = p.comanda(decreto_su(&e.id, &e.gradini[0].id));
    assert!(
        r.unwrap_err().to_string().contains("materia esclusa"),
        "elettorale o costituzionale"
    );
}

#[test]
fn il_memorandum_vincola_le_pensioni() {
    let mut p = col_mes();
    let fine = p.economia().programma_mes.unwrap();
    let e = presenta(&mut p, &[("eta_pensionabile", "67")]).unwrap_err();
    assert!(
        e.to_string().contains(&format!(
            "Età pensionabile: vincolata dal memorandum del MES fino al turno {fine}"
        )),
        "{e}"
    );
    // La stessa disposizione allo stesso gradino del memorandum non è toccata dal vincolo.
    assert!(
        !presenta(&mut p, &[("eta_pensionabile", "70")])
            .unwrap_err()
            .to_string()
            .contains("memorandum")
    );
    // Finito il programma si cambia come ogni altra.
    let turno = p.vista().turno;
    let mut p = edita(&p, |j| j["economia"]["programma_mes"] = json!(turno + 1)).unwrap();
    chiudi_col_bilancio(&mut p);
    chiudi_col_bilancio(&mut p);
    assert_eq!(p.economia().programma_mes, None);
    // Una gradino che nessuna legge ha in esame: il memorandum non blocca più.
    presenta(&mut p, &[("indicizzazione_pensioni", "piena")]).unwrap();
}

#[test]
fn il_mes_limita_la_legge_di_bilancio() {
    let mut p = Partita::nuova(dati_base(), 7);
    for _ in 0..4 {
        p.chiudi_turno();
    }
    let mut p = edita(&p, |j| {
        j["economia"]["programma_mes"] = json!(p.vista().turno + 5)
    })
    .unwrap();
    let bozza = p.bozza_bilancio().unwrap();
    let mut b = bozza.clone();
    for d in &mut b.dicasteri {
        d.acquisti += d.spesa;
    }
    let r = p.riepilogo_bilancio(&b).unwrap();
    assert!(r.crescita_spesa_netta > r.tetto, "{r:?}");
    let prima = p.salva().unwrap();
    let e = p
        .comanda(Comando::PresentaBilancio { bilancio: b })
        .unwrap_err();
    assert!(e.to_string().contains("programma del MES"), "{e}");
    assert_eq!(p.salva().unwrap(), prima);
    // La bozza, nel tetto, passa.
    let ok = p.riepilogo_bilancio(&bozza).unwrap();
    assert!(ok.crescita_spesa_netta <= ok.tetto, "{ok:?}");
    p.comanda(Comando::PresentaBilancio { bilancio: bozza })
        .unwrap();
}

#[test]
fn una_legge_in_esame_prima_del_mes_non_muove_la_condizionalita() {
    let mut p = con_premio();
    chiudi_col_bilancio(&mut p);
    assert_eq!(aste(&p).len(), 1);
    // In esame prima del MES: due disposizioni, una vincolata e una no.
    presenta(
        &mut p,
        &[("eta_pensionabile", "66"), ("pensione_minima", "piu_20")],
    )
    .unwrap();
    chiudi_col_bilancio(&mut p);
    assert!(p.economia().programma_mes.is_some());
    // Nel turno del bilancio le leggi slittano: la votazione è alla chiusura dopo.
    chiudi_col_bilancio(&mut p);
    assert!(p.economia().programma_mes.is_some());
    assert!(
        p.vista()
            .esiti
            .iter()
            .any(|e| e.forma == FormaMisura::Riforma && e.approvata),
        "la legge doveva passare: {:?}",
        p.vista().esiti
    );
    assert_eq!(p.in_vigore("eta_pensionabile"), Some("70"));
    assert_eq!(p.in_vigore("pensione_minima"), Some("piu_20"));
}

fn al_bilancio_2028(tetto: Option<f64>) -> Partita {
    let mut f = sim_core_senza_shock();
    if let Some(t) = tetto {
        f = sostituisci(f, "scenario", "2028 = 1.7", &format!("2028 = {t}"));
    }
    let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    for _ in 0..4 {
        comune::chiudi_senza_opposizione_ia(&mut p);
    }
    let b = p.bozza_bilancio().unwrap();
    p.comanda(Comando::PresentaBilancio { bilancio: b })
        .unwrap();
    p
}

fn sim_core_senza_shock() -> FileDati {
    comune::senza_shock()
}

fn emenda(p: &mut Partita) -> Result<(), sim_core::Errore> {
    let misura = (p.vista().calendario.into_iter())
        .find(|m| m.forma == FormaMisura::Bilancio)
        .unwrap()
        .id;
    p.comanda(Comando::Emendamento {
        misura,
        partito: "centrodestra".into(),
        accogli: true,
    })
}

#[test]
fn il_mes_limita_anche_l_emendamento_al_bilancio() {
    // Sonda: la crescita della bozza e quella dopo l'emendamento accolto, senza programma.
    let mut sonda = al_bilancio_2028(None);
    let c0 = sonda.riepilogo_presentato().unwrap().crescita_spesa_netta;
    emenda(&mut sonda).unwrap();
    let c1 = sonda.riepilogo_presentato().unwrap().crescita_spesa_netta;
    assert!(c1 > c0, "{c0} {c1}");
    // Tetto fra le due: la bozza sta dentro, l'emendato no.
    let tetto = (c0 + c1) / 2.0;
    let mut libero = al_bilancio_2028(Some(tetto));
    emenda(&mut libero).unwrap();
    let p = al_bilancio_2028(Some(tetto));
    let turno = p.vista().turno;
    let mut p = edita(&p, |j| j["economia"]["programma_mes"] = json!(turno + 5)).unwrap();
    let prima = p.salva().unwrap();
    let e = emenda(&mut p).unwrap_err();
    assert!(e.to_string().contains("programma del MES"), "{e}");
    assert_eq!(p.salva().unwrap(), prima);
}
