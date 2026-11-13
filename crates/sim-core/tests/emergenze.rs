//! Emergenze naturali: rischio per regione, prevenzione, fondo di riserva e
//! decreto a debito.

mod comune;

use comune::{
    chiudi_col_bilancio, edita, file_base, file_vero, riscrivi, scandalo_certo, senza_emergenze,
    sostituisci, voci,
};
use serde_json::{Value, json};
use sim_core::Partita;
use sim_core::dati::{DatiGioco, FileDati};
use sim_core::esito::{Criticita, TipoVoce};
use sim_core::eventi::{Famiglia, Tono};

const SISMICA: &str = "Emergenza sismica";

/// I dati senza alcun rischio: nessun sisma (nemmeno il minimo), idro e siccità a zero.
fn senza_rischio() -> FileDati {
    senza_emergenze(file_base())
}

/// I dati dove ogni regione ha un sisma certo a ogni trimestre.
fn sisma_certo() -> FileDati {
    riscrivi(
        senza_rischio(),
        "regioni",
        "pericolosita = ",
        "pericolosita = { sisma = 1.0, idro = 0.05, incendi = 0.0 }",
    )
}

fn nuova(f: &FileDati, seme: u64) -> Partita {
    Partita::nuova(DatiGioco::da_testi(f).unwrap(), seme)
}

fn json_di(p: &Partita) -> Value {
    serde_json::from_str(&p.salva().unwrap()).unwrap()
}

fn indice_evento(id: &str) -> usize {
    DatiGioco::da_testi(&file_base())
        .unwrap()
        .evento(id)
        .unwrap()
}

fn stock() -> usize {
    DatiGioco::da_testi(&file_base())
        .unwrap()
        .grandezza("stock_infrastrutture")
        .unwrap()
}

/// Gli effetti sullo stock di infrastrutture, nel salvataggio.
fn effetti_sullo_stock(j: &Value) -> Vec<Value> {
    let k = stock();
    j["grandezze"]["effetti"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|e| e["grandezza"] == k)
        .cloned()
        .collect()
}

#[test]
fn gli_incendi_cadono_d_estate() {
    let f = file_vero();
    let mut visti = 0;
    for seme in 1..=300 {
        let mut p = nuova(&f, seme);
        for _ in 0..8 {
            chiudi_col_bilancio(&mut p);
            let v = p.vista();
            for e in v.eventi.iter().filter(|e| e.nome == "Emergenza incendi") {
                visti += 1;
                assert_eq!(v.data.trimestre, 3, "seme {seme}: {e:?}");
            }
        }
    }
    assert!(visti > 0, "nessun incendio in 300 partite");
}

/// Il danno sullo stock della prima emergenza nata alla prima chiusura, con lo stock di ogni
/// regione portato a `punti` sopra il livello iniziale.
fn danno_con_stock_piu(punti: f64) -> f64 {
    let p = nuova(&sisma_certo(), 3);
    let mut p = edita(&p, |j| {
        if punti > 0.0 {
            j["grandezze"]["effetti"]
                .as_array_mut()
                .unwrap()
                .push(json!({
                    "grandezza": stock(),
                    "regione": null,
                    "dicastero": null,
                    "disposizione": null,
                    "intensita": punti,
                    "inizio": 0,
                    "forma": "livello",
                    "durata": 1.0,
                }));
        }
    })
    .unwrap();
    chiudi_col_bilancio(&mut p);
    let nuovi: Vec<f64> = effetti_sullo_stock(&json_di(&p))
        .iter()
        .filter_map(|e| e["intensita"].as_f64())
        .filter(|x| *x < 0.0)
        .collect();
    assert_eq!(nuovi.len(), 1, "{nuovi:?}");
    nuovi[0]
}

#[test]
fn la_prevenzione_riduce_il_danno() {
    let senza = danno_con_stock_piu(0.0);
    let con = danno_con_stock_piu(100.0);
    assert!(senza < 0.0);
    // Stock a 100 = nessuna prevenzione, a 200 = piena: il sisma tiene il (1 - 0,85) del danno.
    assert!((con - senza * 0.15).abs() < 1e-9, "{con} contro {senza}");
}

/// La partita dopo una chiusura (fuori dal quarto trimestre, dove il fondo si azzera), con
/// `n` emergenze sismiche aperte nella regione 0, scelta `scelta` e fondo di `fondo` euro.
fn con_emergenze(n: usize, scelta: Option<usize>, fondo: i64) -> Partita {
    let mut p = nuova(&senza_rischio(), 5);
    chiudi_col_bilancio(&mut p);
    let evento = indice_evento("emergenza_sismica");
    let turno = p.vista().turno;
    edita(&p, |j| {
        j["economia"]["bilancio"]["fondo_riserva"] = fondo.into();
        if !j["economia"]["bilancio_approvato"].is_null() {
            j["economia"]["bilancio_approvato"]["fondo_riserva"] = fondo.into();
        }
        j["eventi"]["aperti"] = Value::Array(
            (0..n)
                .map(|_| {
                    json!({
                        "evento": evento,
                        "turno": turno,
                        "causa": { "tipo": "rischio", "livello": "molto" },
                        "regione": 0,
                        "bersaglio": null,
                        "tono": "ordinario",
                        "costo": 1_000_000_000_i64,
                        "scelta": scelta,
                    })
                })
                .collect(),
        );
    })
    .unwrap()
}

#[test]
fn il_fondo_paga_e_non_va_sotto_zero() {
    let mut p = con_emergenze(2, None, 50_000_000);
    chiudi_col_bilancio(&mut p);
    let j = json_di(&p);
    // Il fondo ha pagato il residuo, e la seconda emergenza non ha trovato niente.
    assert_eq!(j["economia"]["fondo_speso"], 50_000_000);
    // Ripristino del 5% (50 milioni su 1 miliardo) della sola prima: 0,05 * 1 mld * 2 punti.
    let ripristino: f64 = effetti_sullo_stock(&j)
        .iter()
        .filter_map(|e| e["intensita"].as_f64())
        .sum();
    assert!((ripristino - 0.1).abs() < 1e-9, "{ripristino}");
    let v = voci(&p, TipoVoce::EventoRisolto);
    let sismiche: Vec<_> = (v.iter())
        .filter(|v| v.fatto.starts_with(SISMICA))
        .map(|v| v.fatto.as_str())
        .collect();
    assert_eq!(sismiche.len(), 2, "{v:?}");
    assert!(
        sismiche[0].contains("«Attingere al fondo di riserva» per default")
            && sismiche[0].contains("; 50.000.000 euro dal fondo di riserva (5% del costo)"),
        "{}",
        sismiche[0]
    );
    assert!(
        sismiche[1].contains("; 0 euro dal fondo di riserva (0% del costo)"),
        "{}",
        sismiche[1]
    );
}

#[test]
fn il_fondo_di_un_anno_nuovo_riparte() {
    let mut p = con_emergenze(1, None, 50_000_000);
    chiudi_col_bilancio(&mut p);
    assert_eq!(json_di(&p)["economia"]["fondo_speso"], 50_000_000);
    // Il turno dopo è il quarto trimestre: la sua chiusura azzera l'anno.
    while p.vista().data.trimestre != 4 {
        chiudi_col_bilancio(&mut p);
    }
    chiudi_col_bilancio(&mut p);
    assert_eq!(json_di(&p)["economia"]["fondo_speso"], 0);
}

#[test]
fn il_decreto_a_debito_alza_il_disavanzo() {
    let costo = 1_000_000_000_i64;
    let disavanzo = |mut p: Partita| {
        chiudi_col_bilancio(&mut p);
        let j = json_di(&p);
        (
            j["economia"]["disavanzo_trimestre"].as_i64().unwrap(),
            j["economia"]["fondo_speso"].as_i64().unwrap(),
        )
    };
    let (senza, _) = disavanzo(con_emergenze(0, None, 300_000_000));
    // L'opzione 1 è «Finanziare con un decreto a debito».
    let (con, speso) = disavanzo(con_emergenze(1, Some(1), 300_000_000));
    assert!(
        (con - senza - costo).abs() <= 1,
        "{con} contro {senza} + {costo}"
    );
    assert_eq!(speso, 0, "il fondo non si tocca");
}

#[test]
fn con_vittime_il_tono_e_grave() {
    let (mut gravi, mut ordinarie) = (0, 0);
    for seme in 1..=100 {
        let mut p = nuova(&sisma_certo(), seme);
        chiudi_col_bilancio(&mut p);
        let sismica = indice_evento("emergenza_sismica");
        let costo = json_di(&p)["eventi"]["aperti"]
            .as_array()
            .unwrap()
            .iter()
            .find(|a| a["evento"] == sismica)
            .unwrap()["costo"]
            .as_i64()
            .unwrap();
        let v = p.vista();
        let e = v
            .eventi
            .iter()
            .find(|e| e.famiglia == Famiglia::Emergenza)
            .unwrap();
        assert_eq!(e.nome, SISMICA);
        // Lo stock è a 100: nessuna prevenzione, 5 miliardi sono il livello con vittime.
        assert_eq!(e.tono == Tono::Grave, costo == 5_000_000_000, "seme {seme}");
        // La voce dell'evento risolto porta il tono dell'evento.
        let tono = e.tono;
        chiudi_col_bilancio(&mut p);
        let risolta = (voci(&p, TipoVoce::EventoRisolto).into_iter())
            .find(|v| v.fatto.starts_with(SISMICA))
            .unwrap();
        assert_eq!(risolta.tono, tono, "seme {seme}");
        if tono == Tono::Grave {
            gravi += 1;
        } else {
            ordinarie += 1;
        }
    }
    assert!(gravi > 0 && ordinarie > 0, "{gravi} gravi, {ordinarie} no");
}

#[test]
fn al_massimo_un_puntuale_per_turno() {
    let f = sisma_certo();
    for seme in 1..=100 {
        let mut p = nuova(&f, seme);
        for turno in 0..20 {
            chiudi_col_bilancio(&mut p);
            let emergenze = (p.vista().eventi.iter())
                .filter(|e| e.famiglia == Famiglia::Emergenza)
                .count();
            // Zero solo se tutte le estrazioni sono uscite lievi: la lieve non apre un evento.
            let lievi = json_di(&p)["eventi"]["lievi"].as_array().unwrap().len();
            assert_eq!(emergenze + lievi, 1, "seme {seme}, turno {turno}");
        }
    }
}

/// Solo il Piemonte ha un sisma certo; il resto del paese non rischia nulla.
fn un_sisma_certo() -> FileDati {
    sostituisci(
        senza_rischio(),
        "regioni",
        "pericolosita = { sisma = 0.0,",
        "pericolosita = { sisma = 1.0,",
    )
}

fn emergenze_aperte(p: &Partita) -> usize {
    (p.vista().eventi.iter())
        .filter(|e| e.famiglia == Famiglia::Emergenza)
        .count()
}

fn con_fondo(p: &Partita, fondo: i64) -> Partita {
    edita(p, |j| {
        j["economia"]["bilancio"]["fondo_riserva"] = fondo.into();
        if !j["economia"]["bilancio_approvato"].is_null() {
            j["economia"]["bilancio_approvato"]["fondo_riserva"] = fondo.into();
        }
    })
    .unwrap()
}

#[test]
fn una_lieve_non_apre_evento_la_paga_il_fondo_e_non_va_sotto_zero() {
    let fondo = 50_000_000_i64;
    let speso = |p: &Partita| json_di(p)["economia"]["fondo_speso"].as_i64().unwrap();
    for seme in 1..=60 {
        let mut p = con_fondo(&nuova(&un_sisma_certo(), seme), fondo);
        chiudi_col_bilancio(&mut p);
        // Un solo candidato, certo: senza evento aperto ha vinto una lieve.
        if emergenze_aperte(&p) != 0 {
            continue;
        }
        // La lieve paga alla chiusura dopo (la spesa cade lì): il fondo paga il residuo, non il costo.
        assert_eq!(speso(&p), 0);
        chiudi_col_bilancio(&mut p);
        assert_eq!(speso(&p), fondo);
        let v = voci(&p, TipoVoce::EmergenzaLieve);
        assert_eq!(v.len(), 1, "{v:?}");
        assert_eq!(v[0].regione.as_deref(), Some("piemonte"));
        assert_eq!(v[0].criticita, Criticita::Ordinaria);
        assert_eq!(
            v[0].fatto,
            "Emergenza sismica lieve in Piemonte, 50.000.000 euro dal fondo di riserva (50% del costo)"
        );
        // Il Piemonte è la sola regione col sisma: la più esposta.
        assert_eq!(v[0].causa, "pericolosità sismica alta");
        // Nello stesso anno le altre emergenze non trovano più nulla.
        chiudi_col_bilancio(&mut p);
        assert_eq!(speso(&p), fondo, "seme {seme}");
        // Con il fondo pieno la lieve costa 100 milioni, interi.
        let mut q = nuova(&un_sisma_certo(), seme);
        chiudi_col_bilancio(&mut q);
        chiudi_col_bilancio(&mut q);
        assert_eq!(speso(&q), 100_000_000);
        return;
    }
    panic!("nessuna lieve in 60 semi");
}

#[test]
fn una_lieve_elide_il_danno_anche_dopo_il_salvataggio() {
    for seme in 1..=60 {
        let mut p = con_fondo(&nuova(&un_sisma_certo(), seme), 5_000_000_000);
        chiudi_col_bilancio(&mut p);
        // Un solo candidato, certo: senza evento aperto ha vinto una lieve, in attesa nel fondo.
        if emergenze_aperte(&p) != 0 {
            continue;
        }
        let mut q = Partita::carica(&p.salva().unwrap()).unwrap();
        assert_eq!(p, q);
        chiudi_col_bilancio(&mut p);
        chiudi_col_bilancio(&mut q);
        assert_eq!(p, q);
        let nati: Vec<f64> = effetti_sullo_stock(&json_di(&p))
            .iter()
            .filter(|e| e["inizio"] == 0)
            .filter_map(|e| e["intensita"].as_f64())
            .collect();
        assert_eq!(nati.len(), 2, "danno e ripristino: {nati:?}");
        assert!(nati.iter().sum::<f64>().abs() < 1e-9, "{nati:?}");
        return;
    }
    panic!("nessuna lieve in 60 semi");
}

#[test]
fn il_fondo_parziale_scala_la_valence() {
    let valence = |mut p: Partita| {
        chiudi_col_bilancio(&mut p);
        let j = json_di(&p);
        let g = usize::try_from(j["governo"]["presidente"].as_u64().unwrap()).unwrap();
        j["partiti"][g]["valence"].as_f64().unwrap()
    };
    let senza = valence(con_emergenze(0, None, 300_000_000));
    // Costo di un miliardo: con mezzo miliardo di fondo la risposta vale la metà.
    let pieno = valence(con_emergenze(1, None, 5_000_000_000)) - senza;
    let meta = valence(con_emergenze(1, None, 500_000_000)) - senza;
    assert!(pieno > 0.0, "{pieno}");
    assert!((meta - pieno / 2.0).abs() < 1e-9, "{meta} contro {pieno}");
}

#[test]
fn il_tetto_vale_tra_famiglie() {
    // Un solo sisma certo (il Piemonte) contro uno scandalo certo: lo scandalo (gravità 6-7)
    // batte la lieve (5), la grave e quella con vittime (10 e oltre) battono lo scandalo.
    let f = sostituisci(
        scandalo_certo(file_base()),
        "regioni",
        "pericolosita = { sisma = 0.0,",
        "pericolosita = { sisma = 1.0,",
    );
    let (mut emergenza, mut scandalo) = (0, 0);
    for seme in 1..=40 {
        let mut p = nuova(&f, seme);
        for turno in 0..6 {
            chiudi_col_bilancio(&mut p);
            let eventi = p.vista().eventi;
            let conta = |fam: Famiglia| eventi.iter().filter(|e| e.famiglia == fam).count();
            let lievi = json_di(&p)["eventi"]["lievi"].as_array().unwrap().len();
            let (e, s) = (conta(Famiglia::Emergenza), conta(Famiglia::Scandalo));
            assert_eq!(e + s + lievi, 1, "seme {seme}, turno {turno}");
            emergenza += e;
            scandalo += s;
        }
    }
    assert!(emergenza > 0 && scandalo > 0, "{emergenza} e {scandalo}");
}

#[test]
fn senza_fondo_la_risposta_non_da_valence() {
    let valence = |mut p: Partita| {
        chiudi_col_bilancio(&mut p);
        let j = json_di(&p);
        let g = usize::try_from(j["governo"]["presidente"].as_u64().unwrap()).unwrap();
        j["partiti"][g]["valence"].as_f64().unwrap()
    };
    let senza_evento = valence(con_emergenze(0, None, 300_000_000));
    let fondo_vuoto = valence(con_emergenze(1, None, 0));
    let fondo_pieno = valence(con_emergenze(1, None, 5_000_000_000));
    assert_eq!(fondo_vuoto, senza_evento);
    assert!(fondo_pieno > senza_evento, "{fondo_pieno} {senza_evento}");
}

#[test]
fn il_decreto_a_debito_elide_il_danno_anche_dopo_il_salvataggio() {
    let sismica = indice_evento("emergenza_sismica");
    for seme in 1..=50 {
        let mut p = nuova(&sisma_certo(), seme);
        chiudi_col_bilancio(&mut p);
        let aperti = json_di(&p)["eventi"]["aperti"].as_array().unwrap().clone();
        // Grave o con vittime: le lievi non sono eventi.
        let Some(i) = aperti
            .iter()
            .position(|a| a["evento"] == sismica && a["costo"].as_i64().unwrap() >= 500_000_000)
        else {
            continue;
        };
        let p = edita(&p, |j| j["eventi"]["aperti"][i]["scelta"] = 1.into()).unwrap();
        // Salvataggio e caricamento fra la nascita e la risoluzione.
        let mut p = Partita::carica(&p.salva().unwrap()).unwrap();
        chiudi_col_bilancio(&mut p);
        let nati: Vec<f64> = effetti_sullo_stock(&json_di(&p))
            .iter()
            .filter(|e| e["inizio"] == 0)
            .filter_map(|e| e["intensita"].as_f64())
            .collect();
        assert_eq!(nati.len(), 2, "danno e ripristino: {nati:?}");
        assert!(nati.iter().sum::<f64>().abs() < 1e-9, "{nati:?}");
        return;
    }
    panic!("nessuna emergenza grave in 50 semi");
}
