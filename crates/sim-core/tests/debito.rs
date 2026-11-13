//! La crisi del debito («Aste scoperte») e il programma del MES (piano 9, Task 9a).

mod comune;

use comune::{
    chiudi_col_bilancio, con_ingresso, dati_base, edita, file_base, imposta, presenta, sostituisci,
    voci,
};
use serde_json::{Value, json};
use sim_core::dati::{DatiGioco, FileDati};
use sim_core::esito::{Criticita, TipoVoce};
use sim_core::eventi::{Famiglia, Tono};
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

/// `p` col programma del MES fino a `fine` e la condizionalità ai gradini del memorandum, come la
/// mette la risposta all'evento.
fn con_mes(p: &Partita, fine: u32) -> Partita {
    let d = dati_base();
    edita(p, |j| {
        j["economia"]["programma_mes"] = json!(fine);
        for c in d.condizionalita() {
            let k = d.disposizione(&c.disposizione).unwrap();
            let g = (d.disposizioni()[k].gradini.iter())
                .position(|x| x.id == c.gradino)
                .unwrap();
            j["disposizioni"]["voci"][k]["in_vigore"] = json!(g);
            j["disposizioni"]["voci"][k]["in_arrivo"] = json!([]);
        }
    })
    .unwrap()
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
    let crisi = voci(&p, TipoVoce::CrisiDelDebito);
    assert_eq!(crisi.len(), 1, "{:?}", v.esito);
    assert_eq!(
        crisi[0].fatto,
        format!("Spread a {} pb: si aprono le «Aste scoperte»", v.spread_pb)
    );
    assert_eq!(
        crisi[0].causa,
        "lo spread ha raggiunto la soglia di crisi di 550 pb, senza programma del MES né \
         ristrutturazione in corso"
    );
    assert_eq!((crisi[0].tono, crisi[0].precedenza), (Tono::Grave, 2));
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
    let risolto = voci(&p, TipoVoce::EventoRisolto);
    let aste = (risolto.iter())
        .find(|v| v.fatto.starts_with("Aste scoperte"))
        .unwrap();
    assert_eq!(aste.tono, Tono::Grave);
    assert_eq!(aste.famiglia, Some(Famiglia::Debito));
    let mes = voci(&p, TipoVoce::MesAvviato);
    assert_eq!(mes.len(), 1, "{mes:?}");
    assert_eq!(
        (mes[0].criticita, mes[0].precedenza),
        (Criticita::Critica, 2)
    );
    assert_eq!(mes[0].tono, Tono::Grave);
    assert!(
        mes[0].fatto.contains("Età pensionabile")
            && mes[0]
                .fatto
                .ends_with("ogni alleato perde 10,0 punti d'intesa"),
        "{}",
        mes[0].fatto
    );
    assert!(
        mes[0]
            .causa
            .starts_with("la scelta nelle «Aste scoperte», spread a "),
        "{}",
        mes[0].causa
    );
}

#[test]
fn non_si_riapre_col_programma_in_corso() {
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
    // Spread oltre la soglia: la chiusura che risolve le Aste fa partire il programma, e non ne
    // apre altre.
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
            "Età pensionabile: vincolo del memorandum del MES, fino al turno {fine}"
        )),
        "{e}"
    );
    // La stessa disposizione allo stesso gradino del memorandum non è toccata dal vincolo: la
    // rifiuta solo perché è già in vigore.
    let e = presenta(&mut p, &[("eta_pensionabile", "70")]).unwrap_err();
    assert!(
        e.to_string().ends_with("Età pensionabile è già a 70 anni"),
        "{e}"
    );
    // Finito il programma si cambia come ogni altra.
    let turno = p.vista().turno;
    let mut p = edita(&p, |j| j["economia"]["programma_mes"] = json!(turno + 1)).unwrap();
    chiudi_col_bilancio(&mut p);
    chiudi_col_bilancio(&mut p);
    assert_eq!(p.economia().programma_mes, None);
    // Un gradino che nessuna legge ha in esame: il memorandum non blocca più.
    presenta(&mut p, &[("indicizzazione_pensioni", "piena")]).unwrap();
}

#[test]
fn il_mes_limita_la_legge_di_bilancio() {
    let mut p = Partita::nuova(dati_base(), 7);
    for _ in 0..4 {
        p.chiudi_turno();
    }
    let turno = p.vista().turno;
    let mut p = con_mes(&p, turno + 5);
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

/// Una legge su due disposizioni, una vincolata e una no, presentata prima del MES e votata col
/// programma in corso.
fn legge_votata_col_mes() -> Partita {
    let mut p = con_premio();
    chiudi_col_bilancio(&mut p);
    assert_eq!(aste(&p).len(), 1);
    presenta(
        &mut p,
        &[("eta_pensionabile", "66"), ("pensione_minima", "piu_20")],
    )
    .unwrap();
    chiudi_col_bilancio(&mut p);
    assert!(p.economia().programma_mes.is_some());
    // Nel turno del bilancio le leggi slittano: la votazione è alla chiusura dopo.
    chiudi_col_bilancio(&mut p);
    p
}

#[test]
fn valore_non_applicato_per_il_memorandum() {
    let p = legge_votata_col_mes();
    let v = voci(&p, TipoVoce::ValoreNonApplicato);
    assert_eq!(v.len(), 1, "{v:?}");
    assert_eq!(v[0].criticita, Criticita::Notevole);
    assert_eq!(v[0].disposizioni, ["eta_pensionabile", "pensione_minima"]);
    assert_eq!(
        v[0].fatto,
        "Età pensionabile resta a 70 anni, non a 66 anni, nonostante la legge approvata"
    );
    assert!(
        v[0].causa
            .starts_with("il vincolo del memorandum del MES, fino a "),
        "{}",
        v[0].causa
    );
}

#[test]
fn una_legge_in_esame_prima_del_mes_non_muove_la_condizionalita() {
    let p = legge_votata_col_mes();
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
    let mut p = con_mes(&p, turno + 5);
    let prima = p.salva().unwrap();
    let e = emenda(&mut p).unwrap_err();
    assert!(e.to_string().contains("programma del MES"), "{e}");
    assert_eq!(p.salva().unwrap(), prima);
}

/// Le Aste scoperte risolte con la ristrutturazione: la chiusura che le applica.
fn ristrutturata() -> Partita {
    let mut p = in_crisi();
    let k = (p.vista().eventi.iter())
        .position(|e| e.nome == "Aste scoperte")
        .unwrap();
    p.comanda(Comando::RispondiEvento {
        evento: k,
        opzione: "ristrutturazione".into(),
    })
    .unwrap();
    chiudi_col_bilancio(&mut p);
    p
}

fn premio_dopo(p: &Partita) -> f64 {
    p.economia().ristrutturazione.unwrap().premio
}

#[test]
fn la_ristrutturazione_taglia_il_debito() {
    let prima = in_crisi().economia().debito as f64;
    let r = ristrutturata();
    let e = r.economia();
    let rapporto = e.debito as f64 / prima;
    assert!((0.45..0.55).contains(&rapporto), "{rapporto}");
    // Il premio entra intero nello spread del turno, poi decade di un passo.
    assert!(e.spread >= 300.0, "{}", e.spread);
    assert!((premio_dopo(&r) - 300.0 * 0.5_f64.powf(0.125)).abs() < 1e-9);
    let v = voci(&r, TipoVoce::Ristrutturazione);
    assert_eq!(v.len(), 1, "{v:?}");
    assert_eq!(v[0].criticita, Criticita::Critica);
    assert!(
        v[0].fatto.contains("premio di 300 pb sullo spread")
            && v[0]
                .fatto
                .contains("fasce media e alta giù del 5% per 8 turni"),
        "{}",
        v[0].fatto
    );
    assert_eq!(v[0].causa, "la scelta nelle «Aste scoperte»");
}

#[test]
fn la_ristrutturazione_tiene_i_conti() {
    let p = ristrutturata();
    let e = p.economia();
    assert_eq!(e.sfc.stock.stato, -e.debito);
    assert_eq!(e.residuo_stock(), 0);
    let r = Partita::carica(&p.salva().unwrap()).unwrap();
    assert_eq!(r.economia().sfc.stock.stato, -r.economia().debito);
}

#[test]
fn il_premio_si_dimezza() {
    let mut p = ristrutturata();
    let inizio = premio_dopo(&p);
    for _ in 0..8 {
        chiudi_col_bilancio(&mut p);
    }
    assert!((premio_dopo(&p) - inizio / 2.0).abs() < 1e-9);
}

/// Una partita ordinaria (PDE in regola, il TPI vale) con un premio di sfiducia di 400 pb e,
/// se `premio`, una ristrutturazione di quel premio: dopo una chiusura, il premio di sfiducia.
fn sfiducia_con(premio: Option<f64>) -> f64 {
    let p = Partita::nuova(dati_base(), 1);
    let mut p = edita(&p, |j| {
        j["economia"]["premio_sfiducia"] = json!(400.0);
        if let Some(premio) = premio {
            j["economia"]["ristrutturazione"] = json!({ "turno": 0, "premio": premio });
        }
    })
    .unwrap();
    chiudi_col_bilancio(&mut p);
    p.economia().premio_sfiducia
}

#[test]
fn la_ristrutturazione_esce_dal_tpi() {
    assert_eq!(sfiducia_con(None), 0.0);
    assert_eq!(sfiducia_con(Some(40.0)), 0.0);
    // A 50 esatti il TPI vale ancora: esce solo oltre.
    assert_eq!(sfiducia_con(Some(50.0)), 0.0);
    assert!(sfiducia_con(Some(100.0)) > 100.0);
}

#[test]
fn niente_crisi_durante_la_ristrutturazione() {
    let p = con_premio();
    let mut p = edita(&p, |j| {
        j["economia"]["ristrutturazione"] = json!({ "turno": 0, "premio": 100.0 })
    })
    .unwrap();
    chiudi_col_bilancio(&mut p);
    assert!(p.economia().spread >= 550.0, "{}", p.economia().spread);
    assert!(aste(&p).is_empty());
}

fn rifiutato(p: &Partita, f: impl FnOnce(&mut Value), messaggio: &str) {
    let e = edita(p, f).unwrap_err();
    assert!(e.to_string().contains(messaggio), "{e}");
}

#[test]
fn un_salvataggio_con_la_ristrutturazione_manomessa_e_rifiutato() {
    let p = ristrutturata();
    let turno = p.vista().turno;
    rifiutato(
        &p,
        |j| j["economia"]["ristrutturazione"]["premio"] = json!(-1.0),
        "economia.ristrutturazione.premio",
    );
    rifiutato(
        &p,
        |j| j["economia"]["ristrutturazione"]["premio"] = json!(300.5),
        "economia.ristrutturazione.premio",
    );
    rifiutato(
        &p,
        |j| j["economia"]["ristrutturazione"]["turno"] = json!(turno + 1),
        "economia.ristrutturazione.turno",
    );
    rifiutato(
        &p,
        |j| j["economia"]["programma_mes"] = json!(turno + 1),
        "programma del MES e ristrutturazione insieme",
    );
}

#[test]
fn la_ristrutturazione_taglia_il_reddito_dei_detentori() {
    let d = dati_base();
    let p = ristrutturata();
    let senza = p.economia().clone();
    let r = senza.ristrutturazione.unwrap();
    let con = senza.clone();
    let senza = sim_core::economia::Economia {
        ristrutturazione: None,
        ..senza
    };
    let (perdita, turni) = (
        d.manopola("perdita_ristrutturazione"),
        d.manopola("turni_ristrutturazione") as u32,
    );
    let primari = |e: &sim_core::economia::Economia, turno: u32| {
        let mut el = p.elettorato().clone();
        el.aggiorna_redditi(&d, e, p.misure(), turno);
        el.gruppi
            .iter()
            .map(|g| g.reddito.primario)
            .collect::<Vec<_>>()
    };
    let base = primari(&senza, r.turno);
    let dentro = primari(&con, r.turno);
    let fuori = primari(&con, r.turno + turni);
    for (k, g) in d.elettorato().gruppi.iter().enumerate() {
        let k_atteso = if g.fascia == sim_core::elettorato::Fascia::Bassa {
            1.0
        } else {
            1.0 - perdita
        };
        assert!(
            (dentro[k] - base[k] * k_atteso).abs() < 1e-6 * base[k].max(1.0),
            "{k}"
        );
        assert_eq!(fuori[k], base[k], "{k}");
    }
}

#[test]
fn un_salvataggio_con_ristrutturazione_e_aste_aperte_e_rifiutato() {
    let p = in_crisi();
    rifiutato(
        &p,
        |j| j["economia"]["ristrutturazione"] = json!({ "turno": 0, "premio": 100.0 }),
        "Aste scoperte aperte con una ristrutturazione in corso",
    );
}

#[test]
fn il_memorandum_vincola_anche_l_anteprima_e_non_copre_i_gradini_sconosciuti() {
    let mut p = col_mes();
    let valori = |g: &str| {
        vec![ValoreProposto {
            disposizione: "eta_pensionabile".into(),
            gradino: g.into(),
        }]
    };
    let e = p.anteprima(&valori("67")).unwrap_err();
    assert!(e.to_string().contains("vincolo del memorandum"), "{e}");
    let e = presenta(&mut p, &[("eta_pensionabile", "inesistente")]).unwrap_err();
    assert!(!e.to_string().contains("memorandum"), "{e}");
    assert!(
        e.to_string()
            .contains("Età pensionabile: gradino sconosciuto «inesistente»"),
        "{e}"
    );
}

#[test]
fn il_programma_dura_durata_mes_chiusure() {
    let mut p = col_mes();
    let fine = p.economia().programma_mes.unwrap();
    for _ in 1..durata() {
        chiudi_col_bilancio(&mut p);
        assert_eq!(p.economia().programma_mes, Some(fine));
    }
    // Al turno `fine` il programma non è più in corso: il memorandum non vincola più (la legge
    // può cadere per altro, per esempio un'altra già in esame).
    assert_eq!(p.vista().turno, fine);
    if let Err(e) = presenta(&mut p, &[("eta_pensionabile", "67")]) {
        assert!(!e.to_string().contains("memorandum"), "{e}");
    }
    assert!(voci(&p, TipoVoce::MesFinito).is_empty());
    chiudi_col_bilancio(&mut p);
    assert_eq!(p.economia().programma_mes, None);
    let finito = voci(&p, TipoVoce::MesFinito);
    assert_eq!(finito.len(), 1, "{:?}", p.vista().esito);
    assert_eq!(
        finito[0].fatto,
        "Finito il programma del MES: cadono i vincoli del memorandum"
    );
    assert_eq!(finito[0].causa, "la durata del programma, 8 trimestri");
}

fn eta_pensionabile() -> (usize, usize) {
    let d = dati_base();
    let k = d.disposizione("eta_pensionabile").unwrap();
    let g = (d.disposizioni()[k].gradini.iter())
        .position(|g| g.id == "67")
        .unwrap();
    (k, g)
}

#[test]
fn l_abrogativo_sulla_condizionalita_decade_col_mes() {
    let p = col_mes();
    let (k, g) = eta_pensionabile();
    let promotore = comune::partito("estrema_destra");
    let mut p = edita(&p, |j| {
        let voto = j["turno"].as_u64().unwrap() + 1;
        j["parlamento"]["referendum"] = json!({
            "tipo": "abrogativo", "misura": null, "valori": [[k, g]], "promotore": promotore,
            "del_giocatore": false, "voto": voto, "firme": true, "invito_astensione": false,
            "richiesto": true,
        });
    })
    .unwrap();
    let fine = p.economia().programma_mes.unwrap();
    chiudi_col_bilancio(&mut p);
    chiudi_col_bilancio(&mut p);
    let esito = &p.parlamento().esiti_referendum[0];
    assert_eq!(
        esito.decaduto.as_deref(),
        Some(
            format!(
                "Decaduto senza voto: la disposizione è vincolata dal memorandum del MES fino al \
                 turno {fine}"
            )
            .as_str()
        )
    );
    assert_eq!(p.in_vigore("eta_pensionabile"), Some("70"));
}

/// Il bilancio della partita col seme 7 al quarto trimestre del 2027, con gli acquisti dei
/// dicasteri raddoppiati: oltre il tetto.
fn al_bilancio_oltre_il_tetto() -> (Partita, sim_core::economia::Bilancio) {
    let mut p = Partita::nuova(dati_base(), 7);
    for _ in 0..4 {
        p.chiudi_turno();
    }
    let mut b = p.bozza_bilancio().unwrap();
    for d in &mut b.dicasteri {
        d.acquisti += d.spesa;
    }
    (p, b)
}

/// Il bilancio oltre il tetto presentato, poi il MES chiesto per default nella stessa chiusura:
/// la partita chiusa, la sonda col programma sullo stato di prima, il limite e la crescita del
/// presentato.
fn ricondotto() -> (Partita, Partita, f64, f64, sim_core::economia::Bilancio) {
    let (p, b) = al_bilancio_oltre_il_tetto();
    let turno = p.vista().turno;
    // La sonda: il limite del programma e il riepilogo di un bilancio, sullo stesso stato.
    let sonda = con_mes(&p, turno + durata());
    let bozza = sonda
        .riepilogo_bilancio(&sonda.bozza_bilancio().unwrap())
        .unwrap();
    let limite = bozza.tetto.max(bozza.crescita_spesa_netta);
    let crescita = sonda.riepilogo_bilancio(&b).unwrap().crescita_spesa_netta;
    assert!(crescita > limite);
    let mut p = p;
    p.comanda(Comando::PresentaBilancio { bilancio: b })
        .unwrap();
    let presentato = p.economia().bilancio_presentato.clone().unwrap();
    // Le Aste aperte: alla chiusura il default chiede il MES, prima dell'aula.
    let k = dati_base().evento("aste_scoperte").unwrap();
    let mut p = edita(&p, |j| {
        j["eventi"]["aperti"] = json!([{
            "evento": k, "turno": turno,
            "causa": { "tipo": "spread", "pb": 600, "soglia": 550 },
            "regione": null, "bersaglio": null, "tono": "grave", "costo": 0, "scelta": null,
        }]);
    })
    .unwrap();
    let presidente = p.governo().presidente;
    p.chiudi_turno();
    assert_eq!(p.governo().presidente, presidente);
    (p, sonda, limite, crescita, presentato)
}

#[test]
fn il_bilancio_presentato_prima_del_mes_entra_ricondotto() {
    let (p, sonda, limite, _, presentato) = ricondotto();
    assert!(p.economia().programma_mes.is_some());
    let approvato = p.economia().bilancio_approvato.clone().unwrap();
    assert_ne!(approvato, presentato);
    let r = sonda.riepilogo_bilancio(&approvato).unwrap();
    assert!(r.crescita_spesa_netta <= limite + 1e-6, "{r:?} {limite}");
    assert!(!p.vista().bilancio.esercizio_provvisorio);
}

#[test]
fn bilancio_ricondotto_lascia_la_voce() {
    let (p, _, limite, crescita, _) = ricondotto();
    let virgola = |x: f64| format!("{x:.1}").replace('.', ",");
    let tipi: Vec<TipoVoce> = (p.vista().esito.iter())
        .map(|v| v.tipo)
        .filter(|t| {
            matches!(
                t,
                TipoVoce::BilancioApprovato | TipoVoce::BilancioRicondotto
            )
        })
        .collect();
    assert_eq!(
        tipi,
        [TipoVoce::BilancioApprovato, TipoVoce::BilancioRicondotto]
    );
    let v = voci(&p, TipoVoce::BilancioRicondotto);
    assert_eq!((v[0].criticita, v[0].precedenza), (Criticita::Critica, 2));
    assert!(v[0].disposizioni.is_empty() && v[0].misura.is_some());
    assert!(
        v[0].fatto.starts_with(&format!(
            "Legge di bilancio ricondotta al tetto del MES: la spesa netta presentata cresceva \
             del {}%, il limite è del {}%; dei budget presentati resta il ",
            virgola(crescita),
            virgola(limite)
        )) && v[0].fatto.ends_with("% dello scarto dalla bozza"),
        "{}",
        v[0].fatto
    );
    assert!(
        v[0].causa
            .starts_with("il programma del MES, cominciato dopo la presentazione, finisce a "),
        "{}",
        v[0].causa
    );
}

#[test]
fn un_bilancio_al_tetto_esatto_passa() {
    let mut sonda = al_bilancio_2028(None);
    emenda(&mut sonda).unwrap();
    let c1 = sonda.riepilogo_presentato().unwrap().crescita_spesa_netta;
    let p = al_bilancio_2028(Some(c1));
    let turno = p.vista().turno;
    let mut p = con_mes(&p, turno + 5);
    emenda(&mut p).unwrap();
    let r = p.riepilogo_presentato().unwrap();
    assert_eq!(r.crescita_spesa_netta, r.tetto);
}

#[test]
fn il_governo_ia_col_mes_e_un_ultimatum_approva_il_bilancio() {
    // Tetto del 2028 sotto la crescita della bozza: il limite del programma è la bozza stessa. I
    // budget dell'alleato in ultimatum fermi a metà dei prezzi di oggi: indicizzarli lo
    // sforerebbe.
    let f = sostituisci(file_base(), "scenario", "2028 = 1.7", "2028 = -5.0");
    let fondazione = sim_core::legislatura::Fondazione {
        nome: "Partito nuovo".into(),
        posizione: 1.5,
    };
    let mut p = Partita::fonda(DatiGioco::da_testi(&f).unwrap(), 7, &fondazione).unwrap();
    p.chiudi_turno();
    p.chiudi_turno();
    assert!(!p.governa());
    while p.vista().data.trimestre != 3 {
        p.chiudi_turno();
    }
    let g = p.governo().clone();
    let alleato = (g.coalizione.iter().copied())
        .find(|&j| j != g.presidente && g.dicasteri.contains(&j))
        .expect("un alleato con dicasteri");
    let n = p.partiti().len();
    let turno = p.vista().turno;
    let p = con_mes(&p, turno + 5);
    let mut p = edita(&p, |j| {
        let mut ultimatum = vec![Value::Null; n];
        ultimatum[alleato] = json!("norma_lontana");
        let mut valori = vec![json!(100.0); n];
        valori[alleato] = json!(35.0);
        j["intesa"]["ultimatum"] = json!(ultimatum);
        j["intesa"]["valori"] = json!(valori);
        for (k, _) in g
            .dicasteri
            .iter()
            .enumerate()
            .filter(|&(_, &x)| x == alleato)
        {
            j["economia"]["bilancio"]["dicasteri"][k]["livello_prezzi"] = json!(0.5);
        }
    })
    .unwrap();
    p.chiudi_turno();
    assert_eq!(p.vista().data.trimestre, 4);
    assert!(p.intesa().ultimatum[alleato].is_some());
    assert!(
        p.economia().bilancio_presentato.is_some(),
        "{:?}",
        p.vista().esito
    );
    p.chiudi_turno();
    assert!(p.economia().bilancio_approvato.is_some());
    assert!(!p.vista().bilancio.esercizio_provvisorio);
    assert_eq!(p.governo().presidente, g.presidente);
}

#[test]
fn salva_e_ricarica_non_cambiano_l_esito() {
    let mut in_crisi_risposta = in_crisi();
    let k = (in_crisi_risposta.vista().eventi.iter())
        .position(|e| e.nome == "Aste scoperte")
        .unwrap();
    in_crisi_risposta
        .comanda(Comando::RispondiEvento {
            evento: k,
            opzione: "ristrutturazione".into(),
        })
        .unwrap();
    let nuova = Partita::nuova(dati_base(), 3);
    let turno = nuova.vista().turno;
    // Il bordo del controllo: il programma finisce a turno più `durata_mes`.
    let mes_al_bordo = con_mes(&nuova, turno + durata());
    let mut con_shock = Partita::nuova(dati_base(), 3);
    chiudi_col_bilancio(&mut con_shock);
    chiudi_col_bilancio(&mut con_shock);
    for p in [
        in_crisi_risposta,
        col_mes(),
        ristrutturata(),
        mes_al_bordo,
        con_shock,
    ] {
        let mut q = Partita::carica(&p.salva().unwrap()).unwrap();
        let mut p = p;
        for _ in 0..3 {
            chiudi_col_bilancio(&mut p);
            chiudi_col_bilancio(&mut q);
        }
        assert_eq!(p.salva().unwrap(), q.salva().unwrap());
    }
}

#[test]
fn il_premio_della_ristrutturazione_entra_nello_spread() {
    let p = Partita::nuova(dati_base(), 1);
    let mut con = edita(&p, |j| {
        j["economia"]["ristrutturazione"] = json!({ "turno": 0, "premio": 200.0 })
    })
    .unwrap();
    let mut senza = p;
    chiudi_col_bilancio(&mut con);
    chiudi_col_bilancio(&mut senza);
    let d = con.economia().spread - senza.economia().spread;
    assert!((d - 200.0).abs() < 1e-6, "{d}");
}

#[test]
fn dopo_la_ristrutturazione_la_crisi_non_si_riapre() {
    // Soglia bassa: lo spread resta oltre anche col debito tagliato.
    let f = imposta(
        dati_con_premio(),
        "manopole",
        "[soglia_crisi_debito]\nvalore = ",
        "300.0",
    );
    let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    chiudi_col_bilancio(&mut p);
    let k = (p.vista().eventi.iter())
        .position(|e| e.nome == "Aste scoperte")
        .unwrap();
    p.comanda(Comando::RispondiEvento {
        evento: k,
        opzione: "ristrutturazione".into(),
    })
    .unwrap();
    for _ in 0..2 {
        chiudi_col_bilancio(&mut p);
        assert!(p.economia().ristrutturazione.is_some());
        assert!(p.economia().spread >= 300.0, "{}", p.economia().spread);
        assert!(aste(&p).is_empty());
    }
}

/// Col premio a 1 pb, sotto la soglia dopo il primo dimezzamento, la ristrutturazione resta per
/// tutta la perdita dei redditi: le fasce media e alta perdono `perdita_ristrutturazione` per
/// `turni_ristrutturazione` chiusure, poi non più.
#[test]
fn la_perdita_di_reddito_finisce_dentro_la_partita() {
    let d = dati_base();
    let f = comune::senza_emergenze(comune::senza_shock());
    let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    chiudi_col_bilancio(&mut p);
    let turno = p.vista().turno;
    let mut con = edita(&p, |j| {
        j["economia"]["ristrutturazione"] = json!({ "turno": turno, "premio": 1.0 })
    })
    .unwrap();
    let mut senza = p;
    let (perdita, turni) = (
        d.manopola("perdita_ristrutturazione"),
        d.manopola("turni_ristrutturazione") as u32,
    );
    let alta = (d.elettorato().gruppi.iter())
        .position(|g| g.fascia != sim_core::elettorato::Fascia::Bassa)
        .unwrap();
    for i in 0..turni + 2 {
        chiudi_col_bilancio(&mut con);
        chiudi_col_bilancio(&mut senza);
        let primario = |p: &Partita| p.elettorato().gruppi[alta].reddito.primario;
        let rapporto = primario(&con) / primario(&senza);
        let atteso = if i < turni { 1.0 - perdita } else { 1.0 };
        assert!((rapporto - atteso).abs() < 0.005, "{i}: {rapporto}");
        assert_eq!(
            con.economia().ristrutturazione.is_some(),
            i + 1 < turni,
            "{i}"
        );
        let finita = voci(&con, TipoVoce::RistrutturazioneFinita);
        if i + 1 == turni {
            assert_eq!(finita.len(), 1, "{i}: {:?}", con.vista().esito);
            assert_eq!(
                finita[0].fatto,
                "Finita la ristrutturazione del debito: il premio sullo spread è rientrato e i \
                 redditi delle fasce media e alta non calano più"
            );
            assert_eq!(
                finita[0].causa,
                "premio sotto 1 punto base e 8 turni di perdita dei redditi trascorsi"
            );
        } else {
            assert!(finita.is_empty(), "{i}");
        }
    }
}
