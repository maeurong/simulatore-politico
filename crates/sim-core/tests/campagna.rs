mod comune;
use comune::{chiudi_col_bilancio, dati_base, edita, fondata};
use sim_core::partiti::Regola;
use sim_core::{Comando, Partita};

fn al_turno(p: &mut Partita, t: u32) {
    while p.vista().turno < t {
        chiudi_col_bilancio(p);
    }
}

fn riposiziona(p: &mut Partita, posizione: f64) -> Result<(), String> {
    (p.comanda(Comando::Riposiziona { posizione })).map_err(|e| e.to_string())
}

/// Il comando rifiutato col messaggio `atteso` non cambia nulla.
fn rifiutato(p: &mut Partita, posizione: f64, atteso: &str) {
    let prima = p.salva().unwrap();
    let e = riposiziona(p, posizione).unwrap_err();
    assert!(e.contains(atteso), "{e}");
    assert_eq!(p.salva().unwrap(), prima);
}

/// La partita di prova vota al turno 19: la campagna si apre nella chiusura del 18.
#[test]
fn i_partiti_ia_si_muovono_nel_turno_prima_del_voto() {
    let mut p = Partita::nuova(dati_base(), 1);
    al_turno(&mut p, 18);
    // Niente in aula al turno 18: la percepita non scivola coi voti, si muove solo con la
    // campagna.
    let mut p = edita(&p, |j| j["parlamento"]["misure"] = serde_json::json!([])).unwrap();
    let prima = p.partiti().to_vec();
    chiudi_col_bilancio(&mut p);
    assert_eq!(p.vista().turno, 19);
    let mut mossi = 0;
    for (a, b) in prima.iter().zip(p.partiti()) {
        let d = b.posizione_dichiarata - a.posizione_dichiarata;
        assert!(d.abs() <= 0.5 + 1e-12, "{}: {d}", a.id);
        let scarto = b.posizione_percepita - a.posizione_percepita - d / 2.0;
        assert!(scarto.abs() < 1e-12, "{}: {scarto}", a.id);
        if a.regola == Regola::Sticker || a.id == "destra" {
            assert_eq!(d, 0.0, "{}", a.id);
        }
        if d != 0.0 {
            mossi += 1;
            assert!((b.ultimo_passo - d).abs() < 1e-12, "{}", a.id);
            let riga = p
                .vista()
                .mosse
                .iter()
                .any(|m| m.partito == a.id && m.testo.starts_with("Sposta la posizione da "));
            assert!(riga, "{}: {:?}", a.id, p.vista().mosse);
        }
    }
    assert!(mossi > 0);
    // Hunter senza due voti alle spalle: verso il centro.
    let centrodestra = &p.partiti()[4];
    assert_eq!(centrodestra.posizione_dichiarata, 4.5);
    let riga = "Sposta la posizione da 5,0 a 4,5: alla prima legislatura va verso il centro";
    assert!(p.vista().mosse.iter().any(|m| m.testo == riga));
}

#[test]
fn niente_movimenti_al_turno_zero() {
    let p = fondata(1.5);
    let dati = dati_base();
    for (p, d) in p.partiti().iter().zip(&dati.scenario().partiti) {
        assert_eq!(p.posizione_dichiarata, d.posizione, "{}", p.id);
        assert_eq!(p.posizione_percepita, d.posizione, "{}", p.id);
    }
    assert!(p.vista().mosse.is_empty());
}

#[test]
fn riposizionamento_del_giocatore() {
    let mut p = Partita::nuova(dati_base(), 1);
    let g = p.giocatore();
    al_turno(&mut p, 18);
    rifiutato(
        &mut p,
        6.5,
        "Il riposizionamento si decide solo nel turno della campagna",
    );
    chiudi_col_bilancio(&mut p);
    assert_eq!(p.partiti()[g].posizione_dichiarata, 6.0);
    rifiutato(
        &mut p,
        6.6,
        "Al massimo mezzo punto dalla posizione dichiarata",
    );
    rifiutato(&mut p, 7.4, "Posizione fuori da 1-7");
    rifiutato(&mut p, f64::NAN, "Posizione fuori da 1-7");
    let vicino = edita(&p, |j| j["partiti"][6]["posizione_dichiarata"] = 6.3.into()).unwrap();
    let mut vicino = vicino;
    rifiutato(&mut vicino, 6.5, "Non si scavalca Estrema destra");
    riposiziona(&mut vicino, 6.3).unwrap();

    let percepita = p.partiti()[g].posizione_percepita;
    riposiziona(&mut p, 6.5).unwrap();
    assert_eq!(p.partiti()[g].posizione_dichiarata, 6.5);
    assert!((p.partiti()[g].posizione_percepita - percepita - 0.25).abs() < 1e-12);
    rifiutato(&mut p, 6.4, "Già riposizionato in questa campagna");

    // Al voto la campagna si chiude: il turno dopo non è più di campagna.
    chiudi_col_bilancio(&mut p);
    rifiutato(
        &mut p,
        6.4,
        "Il riposizionamento si decide solo nel turno della campagna",
    );
    assert!(!p.legislatura().riposizionato);

    let mut f = fondata(1.5);
    riposiziona(&mut f, 1.8).unwrap();
    let g = f.giocatore();
    assert_eq!(f.partiti()[g].posizione_dichiarata, 1.8);
}
