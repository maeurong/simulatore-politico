mod comune;
use comune::{con_arrivo, dati_base, edita, file_base, sostituisci};
use sim_core::dati::DatiGioco;
use sim_core::disposizioni::Disposizioni;
use sim_core::{Errore, Partita};

#[test]
fn disposizioni_al_turno_zero_nella_vista() {
    let p = Partita::nuova(dati_base(), 1);
    let v = p.vista();
    assert_eq!(v.disposizioni.len(), 46);
    let e = v
        .disposizioni
        .iter()
        .find(|d| d.id == "eta_pensionabile")
        .unwrap();
    assert_eq!(
        (
            e.in_vigore.as_str(),
            e.legge.as_str(),
            e.precedente.as_str()
        ),
        ("67", "L. 199/2025, art. 1 c. 185", "67")
    );
    assert_eq!(p.in_vigore("nucleare"), Some("vietato"));
    let n = v.disposizioni.iter().find(|d| d.id == "nucleare").unwrap();
    assert_eq!(n.in_arrivo[0].gradino, "consentito");
}

#[test]
fn valore_in_arrivo_arriva_al_suo_turno() {
    let mut p = Partita::nuova(dati_base(), 1);
    for _ in 0..3 {
        p.chiudi_turno();
    }
    assert_eq!(p.in_vigore("nucleare"), Some("vietato"));
    p.chiudi_turno();
    assert_eq!(p.in_vigore("nucleare"), Some("consentito"));
    let v = p.vista();
    let n = v.disposizioni.iter().find(|d| d.id == "nucleare").unwrap();
    assert_eq!(
        (n.precedente.as_str(), n.legge.as_str()),
        ("vietato", "L. 169/2026, art. 1")
    );
    assert!(n.in_arrivo.is_empty());
}

#[test]
fn proiezione_con_la_legge_in_vigore() {
    let f = sostituisci(
        file_base(),
        "scenario",
        "legge_elettorale = \"legge_180\"",
        "legge_elettorale = \"proporzionale_puro\"",
    );
    let d = DatiGioco::da_testi(&f).unwrap();
    let p = Partita::nuova(d.clone(), 1);
    assert_eq!(
        p.legge_elettorale(),
        d.elezioni().impostazioni["proporzionale_puro"]
    );
    assert!(p.proiezione_seggi().unwrap().premio.is_none());
}

#[test]
fn arrivo_scritto_nei_dati_cambia_la_disposizione() {
    let f = con_arrivo(file_base(), "eta_pensionabile", "70", 2027, 1);
    let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    p.chiudi_turno();
    assert_eq!(p.in_vigore("eta_pensionabile"), Some("70"));
}

#[test]
fn arrivo_programmato_sostituisce_gli_altri_e_porta_la_sua_legge() {
    // Credito ZES: nei dati `1` al turno 1 e `0_75` al turno 5; l'arrivo programmato li sostituisce.
    let d = dati_base();
    let k = d.disposizione("credito_zes").unwrap();
    let g = |id: &str| d.disposizioni()[k].gradino(id).unwrap();
    let mut s = Disposizioni::iniziali(&d);
    s.programma(k, g("0"), 3, "L. 7/2027");
    assert_eq!(s.voci[k].in_arrivo.len(), 1);
    s.controlla(&d).unwrap();
    s.arrivi(&d, 2);
    assert_eq!(s.voci[k].in_vigore, g("2_3"));
    s.arrivi(&d, 3);
    assert_eq!(
        (s.voci[k].in_vigore, s.voci[k].legge.as_str()),
        (g("0"), "L. 7/2027")
    );
    assert!(s.voci[k].in_arrivo.is_empty());
}

#[test]
fn un_arrivo_non_cancella_quelli_dopo() {
    // Decontribuzione: `15` a gennaio 2029 (turno 9), `0` a gennaio 2030 (turno 13).
    let d = dati_base();
    let k = d.disposizione("decontribuzione").unwrap();
    let g = |id: &str| d.disposizioni()[k].gradino(id).unwrap();
    let mut s = Disposizioni::iniziali(&d);
    s.arrivi(&d, 9);
    assert_eq!(
        (s.voci[k].in_vigore, s.voci[k].in_arrivo.len()),
        (g("15"), 1)
    );
    s.arrivi(&d, 13);
    assert_eq!(
        (s.voci[k].in_vigore, s.voci[k].precedente),
        (g("0"), g("15"))
    );
}

#[test]
fn atto_senza_prerequisito_non_va_in_agenda() {
    let f = sostituisci(
        file_base(),
        "disposizioni",
        "partenza = \"attuale\"\nlegge = \"L. 199/2025, art. 1 c. 400\"",
        "partenza = \"ridotto\"\nlegge = \"L. 199/2025, art. 1 c. 400\"",
    );
    for seme in 1..=5 {
        let v = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), seme).vista();
        let m = v
            .ministeri
            .iter()
            .find(|m| m.dicastero == "sanita")
            .unwrap();
        let c = m
            .atti
            .iter()
            .find(|a| a.id == "sanita_convenzioni_privato")
            .unwrap();
        assert!(!c.pagabile);
        assert_eq!(
            c.prerequisito_mancante.as_deref(),
            Some("serve Privato accreditato: attuale o ampio")
        );
        assert_ne!(
            m.agenda.as_ref().map(|a| a.atto.as_str()),
            Some("sanita_convenzioni_privato")
        );
    }
}

#[test]
fn salvataggio_con_disposizioni_guaste_illeggibile() {
    let p = Partita::nuova(dati_base(), 1);
    for guasto in [
        Box::new(|j: &mut serde_json::Value| {
            j["disposizioni"]["voci"].as_array_mut().unwrap().pop();
        }) as Box<dyn Fn(&mut serde_json::Value)>,
        Box::new(|j: &mut serde_json::Value| {
            j["disposizioni"]["voci"][0]["in_vigore"] = serde_json::json!(99)
        }),
    ] {
        let e = edita(&p, |j| guasto(j)).unwrap_err();
        assert!(matches!(e, Errore::SalvataggioIlleggibile(_)), "{e}");
    }
}

/// Dopo il primo arrivo del credito ZES (2027 T1) resta il secondo (2028 T1) e il precedente è
/// il gradino di partenza: salvare e ricaricare conserva tutto, e i due turni seguenti coincidono.
#[test]
fn salva_e_carica_dopo_un_arrivo_scattato_chiude_il_turno_in_modo_identico() {
    let mut p = Partita::nuova(dati_base(), 1);
    p.chiudi_turno();
    let z = |p: &Partita| {
        let v = p.vista();
        v.disposizioni
            .into_iter()
            .find(|d| d.id == "credito_zes")
            .unwrap()
    };
    let d = z(&p);
    assert_eq!(
        (
            d.in_vigore.as_str(),
            d.precedente.as_str(),
            d.in_arrivo.len()
        ),
        ("1", "2_3", 1)
    );
    let mut q = Partita::carica(&p.salva().unwrap()).unwrap();
    assert_eq!(z(&q), d);
    assert_eq!(q.salva().unwrap(), p.salva().unwrap());
    for _ in 0..2 {
        p.chiudi_turno();
        q.chiudi_turno();
        assert_eq!(q.salva().unwrap(), p.salva().unwrap());
    }
}
