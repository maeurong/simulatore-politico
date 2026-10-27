#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

mod comune;
use comune::{edita, fondata};
use sim_core::legislatura::{Cessione, Fase};
use sim_core::{Comando, Partita};

fn dichiara(p: &mut Partita, alleati: &[&str]) {
    dichiara_con(p, alleati, Vec::new());
}

fn dichiara_con(p: &mut Partita, alleati: &[&str], cessioni: Vec<Cessione>) {
    p.comanda(Comando::DichiaraCoalizione {
        partiti: alleati.iter().map(|s| (*s).into()).collect(),
        cessioni,
    })
    .unwrap();
}

fn votata(posto: f64, alleati: &[&str]) -> Partita {
    let mut p = fondata(posto);
    dichiara(&mut p, alleati);
    p.chiudi_turno();
    p
}

#[test]
fn le_cessioni_passano_seggi_dopo_il_riparto() {
    let k = 10;
    let senza = votata(5.5, &["centrodestra"]);
    let mut con = fondata(5.5);
    let cessione = Cessione {
        partito: "centrodestra".into(),
        seggi: k,
    };
    dichiara_con(&mut con, &["centrodestra"], vec![cessione]);
    con.chiudi_turno();
    let (g, m) = (7, 4);
    assert_eq!(con.partiti()[m].id, "centrodestra");
    let (a, b) = (senza.parlamento(), con.parlamento());
    assert_eq!(b.seggi_camera[g], a.seggi_camera[g] - k);
    assert_eq!(b.seggi_camera[m], a.seggi_camera[m] + k);
    assert_eq!(b.seggi_senato[g], a.seggi_senato[g] - k / 2);
    assert_eq!(b.seggi_senato[m], a.seggi_senato[m] + k / 2);
    assert_eq!(con.legislatura().totali, senza.legislatura().totali);
    let e = &con.vista().elezioni[0];
    let camera: Vec<u32> = e.voti.iter().map(|v| v.camera).collect();
    let senato: Vec<u32> = e.voti.iter().map(|v| v.senato).collect();
    assert_eq!(
        (camera, senato),
        (b.seggi_camera.clone(), b.seggi_senato.clone())
    );
}

// Variante del brief: al posto 6,5 nessun partito è disponibile (Task 7); al posto 5,5 il
// centrodestra entra.
#[test]
fn il_voto_del_turno_zero_elegge_il_parlamento() {
    let p = votata(5.5, &["centrodestra"]);
    let l = p.legislatura();
    assert_eq!(l.fase, Fase::Trattativa);
    assert_eq!((l.inizio, l.scadenza), (1, 20));
    assert_eq!(p.parlamento().seggi_camera.iter().sum::<u32>(), 400);
    assert_eq!(p.parlamento().seggi_senato.iter().sum::<u32>(), 200);
    assert!(p.parlamento().seggi_camera[7] > 0);
    let e = &p.vista().elezioni[0];
    assert_eq!(e.voti.len(), 8);
    assert!((e.voti.iter().map(|v| v.quota).sum::<f64>() - 1.0).abs() < 1e-9);
    assert_eq!(
        e.coalizioni[0],
        vec!["giocatore".to_string(), "centrodestra".into()]
    );
    assert!(0.0 < e.affluenza && e.affluenza < 1.0, "{}", e.affluenza);
    let astensione = fondata(5.5).intenzioni_vere().nazionali[8];
    assert!(
        (e.affluenza - (1.0 - astensione)).abs() < 0.02,
        "{}",
        e.affluenza
    );
}

#[test]
fn chiudere_senza_dichiarare_vota_con_la_coalizione_ereditata() {
    let mut p = fondata(5.5);
    p.chiudi_turno();
    assert_eq!(
        p.vista().elezioni[0].coalizioni[0],
        vec!["giocatore".to_string()]
    );
}

#[test]
fn secondo_voto_riparte_pulito() {
    let p = votata(5.5, &["centrodestra"]);
    let turno = p.vista().turno;
    let mut p = edita(&p, |j| {
        j["legislatura"]["fase"] = "ordinaria".into();
        j["legislatura"]["scadenza"] = turno.into();
    })
    .unwrap();
    dichiara(&mut p, &[]);
    p.chiudi_turno();
    let l = p.legislatura();
    assert!(l.dichiarazione.is_none());
    assert_eq!(l.elezioni.len(), 2);
    let data = |i: usize| (l.elezioni[i].data.anno, l.elezioni[i].data.trimestre);
    assert!(data(0) > data(1));
    let ids: Vec<Vec<String>> = (l.coalizioni.iter())
        .map(|c| c.iter().map(|&j| p.partiti()[j].id.clone()).collect())
        .collect();
    assert_eq!(ids, l.elezioni[0].coalizioni);
    assert_eq!(l.inizio, turno + 1);
}

#[test]
fn a_camere_sciolte_l_aula_non_vota() {
    // Partita di prova con una legge in calendario per la chiusura del turno 1, poi Camere
    // sciolte: la legge non si vota, decade col voto delle Camere.
    let mut p = Partita::nuova(comune::dati_base(), 1);
    comune::presenta(&mut p, &[("eta_pensionabile", "68")]).unwrap();
    p.chiudi_turno();
    assert_eq!(p.vista().calendario.len(), 1);
    let mut p = edita(&p, |j| {
        j["legislatura"]["fase"] = "sciolta".into();
        j["parlamento"]["seggi_camera"] = serde_json::json!(vec![0; 7]);
        j["parlamento"]["seggi_senato"] = serde_json::json!(vec![0; 7]);
        j["legislatura"]["totali"] = serde_json::json!([0, 0]);
    })
    .unwrap();
    p.chiudi_turno();
    let esiti = p.vista().esiti;
    assert!(
        esiti
            .iter()
            .all(|e| e.decaduta.is_some() && e.linee.is_empty()),
        "{esiti:?}"
    );
}

#[test]
fn dopo_il_voto_l_intenzione_parte_dal_voto() {
    let p = votata(4.5, &[]);
    let v = p.intenzioni_vere();
    let e = &p.vista().elezioni[0];
    // Nessun turno è passato dal voto: l'intenzione del turno dopo è vicina al voto.
    assert!((v.nazionali[7] / (1.0 - v.nazionali[8]) - e.voti[7].quota).abs() < 0.02);
}

#[test]
fn senza_seggi_la_partita_finisce() {
    // Luna di miele azzerata e valence del partito nuovo al minimo: sotto il 3% da solo.
    let f = comune::imposta(
        comune::file_base(),
        "manopole",
        "[valence_partito_nuovo]\nvalore = ",
        "-3.0",
    );
    let f = comune::imposta(f, "manopole", "[luna_di_miele]\nvalore = ", "0.0");
    let mut p = Partita::fonda(
        sim_core::dati::DatiGioco::da_testi(&f).unwrap(),
        7,
        &sim_core::legislatura::Fondazione {
            nome: "Partito nuovo".into(),
            posto: 1.5,
        },
    )
    .unwrap();
    // Senza seggi attesi la coalizione da solo, senza cessioni, si dichiara.
    dichiara(&mut p, &[]);
    p.chiudi_turno();
    assert!(p.finita());
    assert!(p.vista().disponibilita.is_empty());
    let prima = p.salva().unwrap();
    assert!(Partita::carica(&prima).is_ok());
    p.chiudi_turno();
    assert_eq!(p.salva().unwrap(), prima);
    assert!(
        p.comanda(Comando::DichiaraCoalizione {
            partiti: vec![],
            cessioni: vec![]
        })
        .is_err()
    );
}

#[test]
fn seggi_che_non_tornano_col_parlamento_eletto_rifiutati() {
    let rifiutata = |p: &Partita, campo: &str, chiave: &str, valore: u32| {
        edita(p, |j| j[campo][chiave][0] = valore.into()).is_err()
    };
    let p = votata(5.5, &["centrodestra"]);
    assert!(edita(&p, |_| {}).is_ok());
    assert!(rifiutata(&p, "parlamento", "seggi_camera", 999));
    assert!(rifiutata(&p, "legislatura", "totali", 401));
    // A Camere sciolte i seggi sono zero, anche se sommano ai seggi del ramo.
    assert!(rifiutata(&fondata(5.5), "parlamento", "seggi_camera", 400));
}

#[test]
fn salvataggio_incoerente_rifiutato() {
    use serde_json::{Value, json};
    let p = votata(5.5, &["centrodestra"]);
    assert_eq!(p.legislatura().coalizioni[0], vec![7, 4]);
    assert!(edita(&p, |_| {}).is_ok());
    let dichiarazione = |partiti: Value, cessioni: Value| {
        vec![(
            "/legislatura/dichiarazione",
            json!({"partiti": partiti, "cessioni": cessioni}),
        )]
    };
    let casi = [
        // Inizio oltre la scadenza (20), turno 1 entro: lo coglie solo il controllo sull'inizio.
        (
            "scadenza prima dell'inizio",
            vec![("/legislatura/inizio", json!(21))],
        ),
        (
            "coalizione con 99",
            vec![("/legislatura/coalizioni/0/0", json!(99))],
        ),
        (
            "incarico con 99",
            vec![("/legislatura/incarico", json!({"Ia": {"coalizione": [99]}}))],
        ),
        (
            "dichiarazione con 99",
            dichiarazione(json!([99]), json!([])),
        ),
        (
            "dichiarazione con un duplicato",
            dichiarazione(json!([4, 4]), json!([])),
        ),
        (
            "cessione a un non membro",
            dichiarazione(json!([4]), json!([[3, 1]])),
        ),
        (
            "cessione ripetuta",
            dichiarazione(json!([4]), json!([[4, 1], [4, 1]])),
        ),
        (
            "coalizioni non disgiunte",
            vec![("/legislatura/coalizioni", json!([[7, 4], [4, 5]]))],
        ),
        (
            "trattativa con Camere a zero",
            vec![
                ("/parlamento/seggi_camera", json!(vec![0; 8])),
                ("/parlamento/seggi_senato", json!(vec![0; 8])),
                ("/legislatura/totali", json!([0, 0])),
            ],
        ),
        (
            "turno oltre la scadenza",
            vec![
                ("/legislatura/inizio", json!(0)),
                ("/legislatura/scadenza", json!(0)),
            ],
        ),
    ];
    let accettati: Vec<&str> = (casi.iter())
        .filter(|(_, modifiche)| {
            let modifica = |j: &mut Value| {
                for (dove, v) in modifiche {
                    *j.pointer_mut(dove).unwrap() = v.clone();
                }
            };
            edita(&p, modifica).is_ok()
        })
        .map(|(nome, _)| *nome)
        .collect();
    assert!(accettati.is_empty(), "{accettati:?}");
}

// Decisione 2 del controller: nel turno del voto la Vista mostra le coalizioni con cui si vota.
#[test]
fn nel_turno_del_voto_proiezione_e_coalizione_sono_quelle_al_voto() {
    let mut p = fondata(5.5);
    let prima = p.vista().proiezione;
    dichiara(&mut p, &["centrodestra"]);
    let v = p.vista();
    assert_eq!(v.coalizione, vec!["centrodestra".to_string()]);
    assert_ne!(v.proiezione, prima);
}
