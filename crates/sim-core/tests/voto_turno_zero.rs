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
    p.chiudi_turno();
    assert!(p.finita());
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
