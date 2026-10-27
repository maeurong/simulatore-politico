mod comune;
use comune::{dati_base, edita, presenta, senza_shock};
use sim_core::Partita;
use sim_core::dati::DatiGioco;
use sim_core::legislatura::Fase;
use sim_core::parlamento::TipoReferendum;

fn al_turno(p: &mut Partita, t: u32) {
    while p.vista().turno < t {
        p.chiudi_turno();
    }
}

#[test]
fn a_fine_legislatura_le_leggi_in_esame_decadono_e_i_gradini_si_sbloccano() {
    let mut p = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
    al_turno(&mut p, 18);
    let dati = dati_base();
    let k = dati.disposizione("licenziamento").unwrap();
    let durata = dati.disposizione("durata_legislatura").unwrap();
    // Un gradino bloccato a mano, come dopo un referendum; una legge respinta che aspetta di
    // ripresentarsi; un confermativo che si vota dopo la scadenza.
    let mut p = edita(&p, |j| {
        j["disposizioni"]["voci"][k]["bloccati"] = serde_json::json!([0]);
        j["parlamento"]["respinte"] = serde_json::json!([[k, 40]]);
        j["parlamento"]["confermativi"] = serde_json::json!([{
            "tipo": "confermativo",
            "misura": 99,
            "valori": [[durata, 0]],
            "promotore": null,
            "voto": 24,
            "firme": false,
            "invito_astensione": false,
        }]);
    })
    .unwrap();
    al_turno(&mut p, 19);
    presenta(&mut p, &[("tetto_contante", "1000")]).unwrap();
    p.chiudi_turno(); // voto
    let v = p.vista();
    assert!(v.calendario.is_empty());
    assert!(v.esiti.iter().any(|e| {
        e.decaduta.as_deref() == Some("Decaduta per la fine della legislatura") && !e.approvata
    }));
    assert!(
        v.disposizioni
            .iter()
            .all(|d| d.gradini.iter().all(|g| !g.bloccato))
    );
    assert!(p.parlamento().respinte.is_empty());
    assert_eq!(p.parlamento().confermativi.len(), 1);
    assert_eq!(p.in_vigore("tetto_contante"), Some("5000"));
    assert_eq!(p.legislatura().fase, Fase::Trattativa);
}

#[test]
fn la_legge_decaduta_non_costa_valence() {
    let valence = |presenta_al_voto: bool| {
        let mut p = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
        al_turno(&mut p, 19);
        if presenta_al_voto {
            presenta(&mut p, &[("tetto_contante", "1000")]).unwrap();
        }
        p.chiudi_turno(); // voto
        p.partiti().iter().map(|x| x.valence).collect::<Vec<_>>()
    };
    assert_eq!(valence(true), valence(false));
}

#[test]
fn il_voto_sospende_l_abrogativo_indetto() {
    let mut p = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
    al_turno(&mut p, 15);
    p.promuovi_referendum("sinistra", "licenziamento").unwrap();
    p.chiudi_turno(); // firme
    // Il voto spostato oltre la scadenza: indetto e non ancora votato al voto delle Camere.
    let mut p = edita(&p, |j| {
        j["parlamento"]["referendum"]["voto"] = serde_json::json!(20);
    })
    .unwrap();
    al_turno(&mut p, 19);
    p.chiudi_turno(); // voto
    assert!(p.parlamento().referendum.is_none());
    let r = &p.vista().esiti_referendum[0];
    assert_eq!(r.tipo, TipoReferendum::Abrogativo);
    assert_eq!(r.nome, "Licenziamento: tutele crescenti");
    assert_eq!(
        r.decaduto.as_deref(),
        Some("Sospeso per lo scioglimento delle Camere")
    );
    assert!(!r.approvato);
    assert_eq!(p.in_vigore("licenziamento"), Some("tutele_crescenti"));
}

#[test]
fn nessun_referendum_nell_ultimo_anno() {
    let mut p = Partita::nuova(dati_base(), 1);
    // Turno 16 fuori finestra, turno 17 (gennaio-marzo 2031) dentro: il blocco vale su entrambi.
    for t in [16, 17] {
        al_turno(&mut p, t);
        let prima = p.salva().unwrap();
        let e = p
            .promuovi_referendum("sinistra", "licenziamento")
            .unwrap_err();
        assert!(
            e.to_string()
                .contains("Nessuna richiesta di referendum nell'ultimo anno della legislatura"),
            "{e}"
        );
        assert_eq!(p.salva().unwrap(), prima);
    }
}

#[test]
fn la_durata_cambiata_vale_dalla_legislatura_dopo() {
    // `durata_legislatura` a 4 anni dal turno 3: la scadenza in corso resta 19, la prossima è
    // 19 + 4 · 4 = 35.
    let f = comune::con_arrivo(comune::file_base(), "durata_legislatura", "4", 2027, 3);
    let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    al_turno(&mut p, 4);
    assert_eq!(p.in_vigore("durata_legislatura"), Some("4"));
    assert_eq!(p.legislatura().scadenza, 19);
    al_turno(&mut p, 20);
    assert_eq!(p.legislatura().scadenza, 35);
}
