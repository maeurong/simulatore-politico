#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

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
    // Senza dichiarazione la destra corre ancora col centrodestra.
    let indice = |id: &str| p.partiti().iter().position(|x| x.id == id).unwrap();
    let (destra, centrodestra) = (indice("destra"), indice("centrodestra"));
    assert!(
        (p.legislatura().coalizioni.iter())
            .any(|c| c.contains(&destra) && c.contains(&centrodestra)),
        "{:?}",
        p.legislatura().coalizioni
    );
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

/// Partita di prova, voti alla scadenza 19 e 39: a ogni turno il salvataggio riletto è identico;
/// la legislatura nuova parte dal turno dopo il voto e scade venti turni dopo.
#[test]
fn due_legislature_di_fila_con_salvataggio_a_ogni_turno() {
    let mut p = Partita::nuova(dati_base(), 1);
    let mut fasi = Vec::new();
    let mut voti = Vec::new();
    while p.vista().turno < 41 {
        let turno = p.vista().turno;
        assert_eq!(
            Partita::carica(&p.salva().unwrap()).unwrap(),
            p,
            "turno {turno}"
        );
        fasi.push(p.legislatura().fase);
        let (elezioni, al_voto) = (p.vista().elezioni.len(), p.vista().turni_al_voto);
        assert_eq!(al_voto, p.legislatura().scadenza - turno, "turno {turno}");
        p.chiudi_turno();
        if p.vista().elezioni.len() > elezioni {
            assert_eq!(al_voto, 0, "turno {turno}");
            assert_eq!(p.vista().elezioni.len(), elezioni + 1);
            let l = p.legislatura();
            assert_eq!((l.inizio, l.scadenza), (turno + 1, turno + 20));
            voti.push(turno);
        }
    }
    assert_eq!(voti, [19, 39]);
    let attese: Vec<Fase> = (0..41)
        .map(|t| match t {
            20 | 40 => Fase::Trattativa,
            _ => Fase::Ordinaria,
        })
        .collect();
    assert_eq!(fasi, attese);
}

/// Un salvataggio coi totali da miliardi, e seggi che li sommano, non si carica: oltre il
/// gradino più alto di `numero_parlamentari` nel catalogo.
#[test]
fn carica_rifiuta_i_seggi_oltre_il_catalogo() {
    let p = Partita::nuova(dati_base(), 1);
    let r = edita(&p, |j| {
        j["legislatura"]["totali"] = serde_json::json!([3_000_000_000_u32, 3_000_000_000_u32]);
        for ramo in ["seggi_camera", "seggi_senato"] {
            let seggi = j["parlamento"][ramo].as_array_mut().unwrap();
            for x in seggi.iter_mut() {
                *x = 0.into();
            }
            seggi[0] = 3_000_000_000_u32.into();
        }
    });
    let e = r.unwrap_err().to_string();
    assert!(e.contains("oltre i seggi del catalogo"), "{e}");
}
