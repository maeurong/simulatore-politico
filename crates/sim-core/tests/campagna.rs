mod comune;
use comune::{chiudi_col_bilancio, dati_base, edita, fondata};
use sim_core::elettorato::{Condizione, condizione_di};
use sim_core::legislatura::{Bersaglio, Intensita, Leva, Spesa};
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
    for (id, riga) in [
        (
            "centrodestra",
            "Sposta la posizione da 5,0 a 4,5: alla prima legislatura va verso il centro",
        ),
        (
            "sinistra",
            "Sposta la posizione da 2,0 a 2,5: verso i suoi elettori",
        ),
        (
            "centro",
            "Sposta la posizione da 4,0 a 4,5: verso la posizione del governo",
        ),
    ] {
        let mosse = &p.vista().mosse;
        assert!(
            mosse.iter().any(|m| m.partito == id && m.testo == riga),
            "{mosse:?}"
        );
    }
}

/// Gli indici dei partiti attivi nella fila per (dichiarata, indice).
fn fila(p: &Partita) -> Vec<usize> {
    let mut f: Vec<(f64, usize)> = (p.partiti().iter().enumerate())
        .filter(|(_, x)| !x.sciolto)
        .map(|(j, x)| (x.posizione_dichiarata, j))
        .collect();
    f.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
    f.into_iter().map(|(_, j)| j).collect()
}

/// La partita di prova senza decisioni, come `sim-cli --seme 1 --turni 100`: fra due chiusure i
/// partiti attivi in tutte e due restano nello stesso ordine della fila. Chi nasce (protesta,
/// frangia) entra dove sta, chi si scioglie esce: nessuno scavalca.
#[test]
fn nessuno_scavalca_in_cento_turni() {
    let mut p = Partita::nuova(dati_base(), 1);
    let mut prima = fila(&p);
    while p.vista().turno < 100 {
        p.chiudi_turno();
        let ora = fila(&p);
        let comuni = |a: &[usize], b: &[usize]| -> Vec<usize> {
            a.iter().copied().filter(|j| b.contains(j)).collect()
        };
        let t = p.vista().turno;
        assert_eq!(comuni(&prima, &ora), comuni(&ora, &prima), "turno {t}");
        prima = ora;
    }
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
        "Il riposizionamento si decide solo in campagna elettorale",
    );
    chiudi_col_bilancio(&mut p);
    assert_eq!(p.partiti()[g].posizione_dichiarata, 6.0);
    rifiutato(
        &mut p,
        6.6,
        "Al massimo 0,5 punti dalla posizione dichiarata",
    );
    rifiutato(&mut p, 7.4, "Posizione fuori da 1-7");
    rifiutato(&mut p, f64::NAN, "Posizione fuori da 1-7");
    let vicino = edita(&p, |j| j["partiti"][6]["posizione_dichiarata"] = 6.3.into()).unwrap();
    let mut vicino = vicino;
    rifiutato(&mut vicino, 6.5, "Non si scavalca Estrema destra");
    riposiziona(&mut vicino, 6.3).unwrap();
    // Alla stessa posizione la fila segue l'indice: l'estrema destra (6) dopo la destra (5) non si
    // lascia indietro, il centrodestra (4) non si raggiunge da sotto.
    let mut pari = edita(&p, |j| j["partiti"][6]["posizione_dichiarata"] = 6.0.into()).unwrap();
    rifiutato(&mut pari, 6.2, "Non si scavalca Estrema destra");
    riposiziona(&mut pari, 5.8).unwrap();
    let mut sopra = edita(&p, |j| j["partiti"][4]["posizione_dichiarata"] = 6.3.into()).unwrap();
    rifiutato(&mut sopra, 6.3, "Non si scavalca Centrodestra");

    let percepita = p.partiti()[g].posizione_percepita;
    riposiziona(&mut p, 6.5).unwrap();
    assert_eq!(p.partiti()[g].posizione_dichiarata, 6.5);
    assert!((p.partiti()[g].posizione_percepita - percepita - 0.25).abs() < 1e-12);
    rifiutato(
        &mut p,
        6.4,
        "Già riposizionato in questa campagna elettorale",
    );

    // Al voto la campagna si chiude: il turno dopo non è più di campagna.
    chiudi_col_bilancio(&mut p);
    rifiutato(
        &mut p,
        6.4,
        "Il riposizionamento si decide solo in campagna elettorale",
    );
    assert!(!p.legislatura().riposizionato);

    // Verso sinistra il vicino ferma uguale; sciolto non conta.
    let vicino = |sciolto: bool| {
        edita(&fondata(1.5), |j| {
            j["partiti"][0]["posizione_dichiarata"] = 1.3.into();
            j["partiti"][0]["sciolto"] = sciolto.into();
        })
        .unwrap()
    };
    rifiutato(&mut vicino(false), 1.2, "Non si scavalca Estrema sinistra");
    riposiziona(&mut vicino(true), 1.2).unwrap();

    let mut f = fondata(1.5);
    riposiziona(&mut f, 1.8).unwrap();
    let g = f.giocatore();
    assert_eq!(f.partiti()[g].posizione_dichiarata, 1.8);
}

fn riferimento() -> i64 {
    dati_base().manopola("spesa_riferimento").round() as i64
}

/// Le tre regioni con più elettori.
fn piu_popolose() -> Vec<usize> {
    let d = dati_base();
    let mut r: Vec<(u64, usize)> = (d.elettorato().regioni.iter().enumerate())
        .map(|(i, x)| (x.elettori, i))
        .collect();
    r.sort_by(|a, b| b.cmp(a));
    r.into_iter().take(3).map(|(_, i)| i).collect()
}

/// Un terzo della spesa di riferimento su ciascuna delle tre regioni più popolose.
fn sulle_tre(leva: Leva) -> Vec<Spesa> {
    (piu_popolose().into_iter())
        .map(|r| Spesa {
            bersaglio: Bersaglio::Regione(r),
            leva,
            euro: riferimento() / 3,
        })
        .collect()
}

/// La partita di prova (seme 1) al turno 19, la campagna, con la cassa del giocatore alla spesa
/// di riferimento. Nel Task 8 i partiti IA non fanno campagna.
fn in_campagna() -> Partita {
    let mut p = Partita::nuova(dati_base(), 1);
    al_turno(&mut p, 19);
    let g = p.giocatore();
    edita(&p, |j| j["partiti"][g]["cassa"] = riferimento().into()).unwrap()
}

/// La quota del giocatore fra i votanti al voto che chiude il turno di `base`, con la campagna
/// `spese`.
fn quota_al_voto(base: &Partita, spese: &[Spesa]) -> f64 {
    let mut p = edita(base, |_| {}).unwrap();
    let spese = spese.to_vec();
    p.comanda(Comando::Campagna { spese }).unwrap();
    chiudi_col_bilancio(&mut p);
    let id = p.partiti()[p.giocatore()].id.clone();
    let esito = &p.vista().elezioni[0];
    esito.voti.iter().find(|v| v.partito == id).unwrap().quota
}

#[test]
fn la_campagna_a_cassa_piena_vale_da_mezzo_a_un_punto_e_mezzo() {
    let base = in_campagna();
    let guadagno = quota_al_voto(&base, &sulle_tre(Leva::Persuadere)) - quota_al_voto(&base, &[]);
    assert!((0.005..=0.015).contains(&guadagno), "{guadagno}");
}

#[test]
fn le_leve_a_pari_spesa() {
    let base = in_campagna();
    let senza = quota_al_voto(&base, &[]);
    let [p, m, c] = [Leva::Persuadere, Leva::Mobilitare, Leva::Convincere]
        .map(|l| quota_al_voto(&base, &sulle_tre(l)) - senza);
    assert!(p >= m && m > c && c > 0.0, "{p} {m} {c}");
}

/// Massa dei pensionati sul punto di indice `k`, sommata sui loro 60 gruppi.
fn massa_pensionati(p: &Partita, k: usize) -> f64 {
    (p.elettorato().gruppi.iter().enumerate())
        .filter(|(i, _)| condizione_di(*i) == Condizione::Pensionato)
        .map(|(_, g)| g.posizione[k])
        .sum()
}

#[test]
fn convincere_dura_oltre_il_voto() {
    let base = in_campagna();
    let g = base.giocatore();
    // La destra dichiara 6: il punto di indice 5.
    assert_eq!(base.partiti()[g].posizione_dichiarata, 6.0);
    let spese = vec![Spesa {
        bersaglio: Bersaglio::Condizione(Condizione::Pensionato),
        leva: Leva::Convincere,
        euro: riferimento(),
    }];
    let mut con = edita(&base, |_| {}).unwrap();
    let mut senza = edita(&base, |_| {}).unwrap();
    con.comanda(Comando::Campagna { spese }).unwrap();
    let mut scarti = Vec::new();
    for _ in 0..2 {
        chiudi_col_bilancio(&mut con);
        chiudi_col_bilancio(&mut senza);
        scarti.push(massa_pensionati(&con, 5) - massa_pensionati(&senza, 5));
    }
    // Dopo il voto la campagna è vuota: nessuna spinta nuova, l'ancora riassorbe.
    assert!(con.partiti()[g].campagna.is_empty());
    assert!(
        scarti[0] > 0.0 && scarti[1] > 0.0 && scarti[1] < scarti[0],
        "{scarti:?}"
    );
}

/// La campagna rifiutata col messaggio `atteso`, anche in anteprima, non cambia nulla.
fn campagna_rifiutata(p: &mut Partita, spese: &[Spesa], atteso: &str) {
    let prima = p.salva().unwrap();
    let e = p.anteprima_campagna(spese).unwrap_err().to_string();
    assert!(e.contains(atteso), "{e}");
    let spese = spese.to_vec();
    let e = p
        .comanda(Comando::Campagna { spese })
        .unwrap_err()
        .to_string();
    assert!(e.contains(atteso), "{e}");
    assert_eq!(p.salva().unwrap(), prima);
}

#[test]
fn campagna_rifiutata_non_cambia_nulla() {
    let spesa = |r: usize, euro: i64| Spesa {
        bersaglio: Bersaglio::Regione(r),
        leva: Leva::Persuadere,
        euro,
    };
    let mut p = Partita::nuova(dati_base(), 1);
    al_turno(&mut p, 18);
    let fuori = "Si spende solo in campagna elettorale";
    campagna_rifiutata(&mut p, &[spesa(0, 1)], fuori);
    assert!(!p.vista().campagna.attiva);

    let mut p = in_campagna();
    assert!(p.vista().campagna.attiva);
    // La campagna già scelta resta dopo i rifiuti.
    p.comanda(Comando::Campagna {
        spese: vec![spesa(0, 1000)],
    })
    .unwrap();
    let quattro: Vec<Spesa> = (0..4).map(|r| spesa(r, 1000)).collect();
    campagna_rifiutata(&mut p, &quattro, "Al massimo 3 bersagli");
    let nome = dati_base().regioni()[3].nome.clone();
    let ripetuto = format!("Bersaglio ripetuto: {nome}");
    campagna_rifiutata(&mut p, &[spesa(3, 1000), spesa(3, 1000)], &ripetuto);
    let zero = [spesa(0, 1000), spesa(1, 0)];
    campagna_rifiutata(&mut p, &zero, "Spesa non positiva");
    campagna_rifiutata(&mut p, &[spesa(0, -5)], "Spesa non positiva");
    let oltre = [spesa(0, riferimento()), spesa(1, 1)];
    campagna_rifiutata(&mut p, &oltre, "Spesa oltre la cassa: 3.000.000 euro");
    campagna_rifiutata(&mut p, &[spesa(20, 1000)], "Regione sconosciuta");
    assert_eq!(p.vista().campagna.spese, vec![spesa(0, 1000)]);
    // Sostituisce la campagna già scelta; vuota la annulla.
    p.comanda(Comando::Campagna { spese: vec![] }).unwrap();
    assert!(p.vista().campagna.spese.is_empty());
}

#[test]
fn anteprima_a_fasce() {
    let p = in_campagna();
    let prima = p.salva().unwrap();
    let r = riferimento();
    for (euro, intensita) in [
        (r / 16, Intensita::Poco),
        (r / 4, Intensita::Medio),
        (r, Intensita::Molto),
    ] {
        let spesa = Spesa {
            bersaglio: Bersaglio::Condizione(Condizione::Studente),
            leva: Leva::Mobilitare,
            euro,
        };
        let voci = p.anteprima_campagna(std::slice::from_ref(&spesa)).unwrap();
        assert_eq!(voci.len(), 1);
        let v = &voci[0];
        assert_eq!(
            (&v.bersaglio, v.leva, v.euro),
            (&spesa.bersaglio, spesa.leva, euro)
        );
        assert_eq!(v.intensita, intensita, "{euro}");
        assert!(
            !v.causa.is_empty() && !v.causa.contains('\n'),
            "{}",
            v.causa
        );
    }
    assert_eq!(p.salva().unwrap(), prima);
}

#[test]
fn spese_al_turno_zero() {
    let mut p = fondata(1.5);
    let g = p.giocatore();
    let dote = dati_base().manopola("dote_partito_nuovo").round() as i64;
    assert_eq!(p.partiti()[g].cassa, dote);
    p.comanda(Comando::Campagna {
        spese: vec![Spesa {
            bersaglio: Bersaglio::Condizione(Condizione::Studente),
            leva: Leva::Persuadere,
            euro: 400_000,
        }],
    })
    .unwrap();
    p.chiudi_turno();
    let v = p.vista();
    let id = &v.giocatore;
    let riga = "Spende 400.000 euro: persuade studenti";
    assert!(
        (v.mosse.iter()).any(|m| &m.partito == id && m.testo == riga),
        "{:?}",
        v.mosse
    );
    assert!(p.partiti()[g].campagna.is_empty());
    // Prima del primo voto le entrate leggono la quota vera fra i votanti da tutte e due le parti:
    // la quota al voto la ripete, a meno dell'arrotondamento dei voti.
    let d = dati_base();
    let quota = (v.elezioni[0].voti.iter())
        .find(|x| &x.partito == id)
        .unwrap()
        .quota;
    let entrate = (d.manopola("due_per_mille_turno") + d.manopola("donazioni_turno"))
        * quota
        * d.manopola("quota_libera_entrate");
    let atteso = dote - 400_000 + entrate.round() as i64;
    let cassa = p.partiti()[g].cassa;
    assert!((cassa - atteso).abs() <= 2, "{cassa} {atteso}");
}
