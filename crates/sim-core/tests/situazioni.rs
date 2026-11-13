//! Le sei situazioni a soglia: accese al turno zero, aperte dalla chiusura, con isteresi.
//! `due_trimestri_in_calo_accendono_la_recessione` sta in `eventi/situazioni.rs`: `aggiorna` e'
//! pura e le innovazioni non sono nello stato.

mod comune;

use comune::{al_governo, chiudi_col_bilancio, dati_base, edita, file_vero, sostituisci, voci};
use serde_json::json;
use sim_core::esito::{Criticita, TipoVoce, VoceEsito};
use sim_core::{DatiGioco, Partita};

fn accesa(p: &Partita, misura: &str) -> bool {
    p.situazioni()
        .iter()
        .any(|s| s.misura == misura && s.accesa)
}

/// Gli eventi aperti delle carceri: l'economia rumorosa puo' aprirne altri (la recessione).
fn eventi_carceri(p: &Partita) -> usize {
    (p.vista().eventi.iter())
        .filter(|e| e.nome == "Emergenza carceri")
        .count()
}

/// Il valore ufficiale della situazione su `misura`, a una cifra decimale con la virgola.
fn ufficiale(p: &Partita, misura: &str) -> String {
    let s = (p.situazioni().into_iter())
        .find(|s| s.misura == misura)
        .unwrap();
    format!("{:.1}", s.valore).replace('.', ",")
}

/// Le voci di `tipo` sulle carceri.
fn delle_carceri(p: &Partita, tipo: TipoVoce) -> Vec<VoceEsito> {
    (voci(p, tipo).into_iter())
        .filter(|v| v.fatto.contains("«Sovraffollamento delle carceri»"))
        .collect()
}

fn indice(misura: &str) -> usize {
    dati_base().grandezza(misura).unwrap()
}

/// `p` con la grandezza `misura` portata a `v` in ogni regione, nel valore e nella base.
fn con_valore(p: &Partita, misura: &str, v: f64) -> Partita {
    let k = indice(misura);
    edita(p, |j| {
        for campo in ["valori", "basi"] {
            for x in j["grandezze"][campo][k].as_array_mut().unwrap() {
                *x = json!(v);
            }
        }
    })
    .unwrap()
}

#[test]
fn la_crisi_energetica_e_accesa_al_turno_zero_senza_evento() {
    let p = Partita::nuova(DatiGioco::da_testi(&file_vero()).unwrap(), 1);
    assert!(accesa(&p, "energia"));
    assert!(p.vista().eventi.is_empty());
    assert_eq!(p.vista().situazioni.len(), 6);
}

#[test]
fn a_riposo_nessuna_situazione_e_accesa() {
    let p = Partita::nuova(dati_base(), 1);
    assert!(p.situazioni().iter().all(|s| !s.accesa));
}

#[test]
fn le_carceri_oltre_la_soglia_aprono_un_evento() {
    let mut p = con_valore(&al_governo(), "sovraffollamento_carceri", 118.0);
    assert_eq!(eventi_carceri(&p), 0);
    chiudi_col_bilancio(&mut p);
    assert!(accesa(&p, "sovraffollamento_carceri"));
    let v = p.vista();
    let e = (v.eventi.iter().find(|e| e.nome == "Emergenza carceri")).unwrap();
    assert!(e.causa.starts_with("Sovraffollamento"), "{}", e.causa);
    assert!(e.causa.ends_with("(soglia 117,0)"));
    let accese = delle_carceri(&p, TipoVoce::SituazioneAccesa);
    let fatto = format!(
        "Si accende la situazione «Sovraffollamento delle carceri»: sovraffollamento delle \
         carceri a {}",
        ufficiale(&p, "sovraffollamento_carceri")
    );
    assert_eq!(accese.len(), 1, "{:?}", p.vista().esito);
    assert_eq!(accese[0].fatto, fatto);
    assert_eq!(
        accese[0].causa,
        "oltrepassata la soglia di accensione di 117,0"
    );
    assert_eq!(accese[0].criticita, Criticita::Critica);
}

/// Accesa, resta accesa fra le due soglie senza voci, poi si spegne sotto lo spegnimento con la
/// voce che dice valore e soglia.
#[test]
fn situazione_spenta_lascia_la_voce() {
    let mut p = con_valore(&al_governo(), "sovraffollamento_carceri", 118.0);
    chiudi_col_bilancio(&mut p);
    let mut p = con_valore(&p, "sovraffollamento_carceri", 113.0);
    chiudi_col_bilancio(&mut p);
    assert!(accesa(&p, "sovraffollamento_carceri"));
    assert!(delle_carceri(&p, TipoVoce::SituazioneAccesa).is_empty());
    assert!(delle_carceri(&p, TipoVoce::SituazioneSpenta).is_empty());
    let mut p = con_valore(&p, "sovraffollamento_carceri", 105.0);
    chiudi_col_bilancio(&mut p);
    assert!(!accesa(&p, "sovraffollamento_carceri"));
    let spente = delle_carceri(&p, TipoVoce::SituazioneSpenta);
    let fatto = format!(
        "Si spegne la situazione «Sovraffollamento delle carceri»: sovraffollamento delle \
         carceri a {}",
        ufficiale(&p, "sovraffollamento_carceri")
    );
    assert_eq!(spente.len(), 1, "{:?}", p.vista().esito);
    assert_eq!(spente[0].fatto, fatto);
    assert_eq!(
        spente[0].causa,
        "raggiunta la soglia di spegnimento di 110,0"
    );
    assert_eq!(spente[0].criticita, Criticita::Notevole);
}

#[test]
fn la_causa_porta_il_valore_ufficiale_non_il_vero() {
    let mut p = con_valore(&al_governo(), "sovraffollamento_carceri", 118.0);
    chiudi_col_bilancio(&mut p);
    let ufficiale = (p.situazioni().into_iter())
        .find(|s| s.misura == "sovraffollamento_carceri")
        .unwrap()
        .valore;
    let k = indice("sovraffollamento_carceri");
    let j: serde_json::Value = serde_json::from_str(&p.salva().unwrap()).unwrap();
    let vero = j["grandezze"]["valori"][k][0].as_f64().unwrap();
    assert_ne!(format!("{ufficiale:.1}"), format!("{vero:.1}"));
    let v = p.vista();
    let e = (v.eventi.iter().find(|e| e.nome == "Emergenza carceri")).unwrap();
    let testo = format!("{ufficiale:.1}").replace('.', ",");
    assert!(e.causa.contains(&format!(": {testo} ")), "{}", e.causa);
}

#[test]
fn l_evento_aperto_dalla_situazione_innesca_la_proposta_del_ministro() {
    let d = dati_base();
    let i = d
        .dicasteri()
        .iter()
        .position(|x| x.id == "giustizia")
        .unwrap();
    let mut p = edita(
        &con_valore(&al_governo(), "sovraffollamento_carceri", 118.0),
        |j| {
            j["ministri"]["dicasteri"][i]["proposta"] = serde_json::Value::Null;
        },
    )
    .unwrap();
    chiudi_col_bilancio(&mut p);
    let proposta = (p.vista().ministeri.into_iter())
        .find(|m| m.dicastero == "giustizia")
        .and_then(|m| m.proposta)
        .expect("proposta del ministro della Giustizia");
    assert!(
        proposta.motivo.contains("risposta a Emergenza carceri"),
        "{}",
        proposta.motivo
    );
}

#[test]
fn la_situazione_non_lampeggia() {
    let mut p = con_valore(&al_governo(), "sovraffollamento_carceri", 118.0);
    chiudi_col_bilancio(&mut p);
    assert_eq!(eventi_carceri(&p), 1);
    // Fra le due soglie resta accesa, senza un secondo evento.
    for v in [113.0, 118.0, 113.0] {
        let mut q = con_valore(&p, "sovraffollamento_carceri", v);
        chiudi_col_bilancio(&mut q);
        assert!(accesa(&q, "sovraffollamento_carceri"), "a {v}");
        assert_eq!(eventi_carceri(&q), 0, "a {v}");
        p = q;
    }
    // Sotto lo spegnimento si spegne; oltre l'accensione di nuovo, un evento nuovo.
    let mut p = con_valore(&p, "sovraffollamento_carceri", 105.0);
    chiudi_col_bilancio(&mut p);
    assert!(!accesa(&p, "sovraffollamento_carceri"));
    // 125 e non 118: l'accensione è a 117 e il rumore della deriva, che dipende dal seme e dalle
    // estrazioni, può portare il valore ufficiale sotto.
    let mut p = con_valore(&p, "sovraffollamento_carceri", 125.0);
    chiudi_col_bilancio(&mut p);
    assert_eq!(eventi_carceri(&p), 1);
}

#[test]
fn gli_sbarchi_spingono_a_destra() {
    let k = dati_base()
        .situazioni()
        .iter()
        .position(|s| s.id == "sbarchi")
        .unwrap();
    let base = al_governo();
    let mut accesi = edita(&base, |j| j["eventi"]["accese"][k] = json!(true)).unwrap();
    let mut spenti = edita(&base, |_| {}).unwrap();
    chiudi_col_bilancio(&mut accesi);
    chiudi_col_bilancio(&mut spenti);
    let media = |p: &Partita, i: usize| -> f64 {
        (p.elettorato().gruppi[i].posizione.iter().enumerate())
            .map(|(h, m)| (h + 1) as f64 * m)
            .sum()
    };
    let n = spenti.elettorato().gruppi.len();
    assert!((0..n).all(|i| media(&accesi, i) > media(&spenti, i)));
}

#[test]
fn la_spinta_conserva_la_massa() {
    let mut e = al_governo().elettorato().clone();
    let prima: Vec<f64> = e.gruppi.iter().map(|g| g.posizione.iter().sum()).collect();
    e.spingi_a_destra(0.1);
    for (g, p) in e.gruppi.iter().zip(prima) {
        assert!((g.posizione.iter().sum::<f64>() - p).abs() < 1e-12);
    }
}

#[test]
fn la_spinta_non_fa_due_passi() {
    let mut e = al_governo().elettorato().clone();
    for g in &mut e.gruppi {
        g.posizione = [0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    }
    e.spingi_a_destra(0.1);
    for g in &e.gruppi {
        assert!((g.posizione[1] - 0.9).abs() < 1e-12);
        assert!((g.posizione[2] - 0.1).abs() < 1e-12);
        assert!(g.posizione[3].abs() < 1e-12);
    }
}

#[test]
fn la_spinta_di_due_situazioni_si_somma() {
    let d = dati_base();
    let k = |id: &str| d.situazioni().iter().position(|s| s.id == id).unwrap();
    let (sbarchi, criminalita) = (k("sbarchi"), k("criminalita"));
    let base = al_governo();
    let mut una = edita(&base, |j| {
        j["dati"]["manopole"]["spinta_situazione"]["valore"] = json!(0.004);
        j["eventi"]["accese"][sbarchi] = json!(true);
    })
    .unwrap();
    let mut due = edita(&base, |j| {
        j["dati"]["manopole"]["spinta_situazione"]["valore"] = json!(0.002);
        j["eventi"]["accese"][sbarchi] = json!(true);
        j["eventi"]["accese"][criminalita] = json!(true);
    })
    .unwrap();
    chiudi_col_bilancio(&mut una);
    chiudi_col_bilancio(&mut due);
    for (a, b) in una.elettorato().gruppi.iter().zip(&due.elettorato().gruppi) {
        for (x, y) in a.posizione.iter().zip(&b.posizione) {
            assert!((x - y).abs() < 1e-12, "{x} {y}");
        }
    }
}

#[test]
fn l_energia_accesa_al_turno_zero_non_apre_evento_alla_prima_chiusura() {
    let mut p = Partita::nuova(DatiGioco::da_testi(&file_vero()).unwrap(), 1);
    chiudi_col_bilancio(&mut p);
    let aperti: Vec<_> = p.vista().eventi.into_iter().map(|e| e.nome).collect();
    assert!(
        !aperti.contains(&"Rincari dell'energia".to_string()),
        "{aperti:?}"
    );
}

#[test]
fn accensione_e_spegnimento_uguali_sono_rifiutati() {
    let f = sostituisci(
        file_vero(),
        "eventi",
        "accensione = 117.0\nspegnimento = 110.0",
        "accensione = 117.0\nspegnimento = 117.0",
    );
    let e = DatiGioco::da_testi(&f).unwrap_err();
    assert!(
        e.to_string().contains("Sovraffollamento delle carceri"),
        "{e}"
    );
}
