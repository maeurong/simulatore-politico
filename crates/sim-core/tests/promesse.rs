mod comune;
use comune::{chiudi_col_bilancio, dati_base, edita};
use sim_core::parlamento::CausaIntesa;
use sim_core::{Comando, Partita};

/// La partita di prova al turno 19, quello del voto (scadenza 19): la campagna elettorale.
fn in_campagna() -> Partita {
    let mut p = Partita::nuova(dati_base(), 1);
    while p.vista().turno < 19 {
        chiudi_col_bilancio(&mut p);
    }
    p
}

fn copia(p: &Partita) -> Partita {
    Partita::carica(&p.salva().unwrap()).unwrap()
}

fn prometti(p: &mut Partita, promesse: &[(&str, &str)]) -> Result<(), String> {
    let promesse = (promesse.iter())
        .map(|(d, g)| ((*d).to_string(), (*g).to_string()))
        .collect();
    (p.comanda(Comando::Prometti { promesse })).map_err(|e| e.to_string())
}

/// Indice della disposizione `id` e il suo gradino in vigore in `p`.
fn in_vigore(p: &Partita, id: &str) -> (usize, usize) {
    let d = dati_base().disposizione(id).unwrap();
    (d, p.disposizioni().voci[d].in_vigore)
}

/// «Disposizione: gradino», il nome di una promessa.
fn nome(d: usize, g: usize) -> String {
    let dati = dati_base();
    let x = &dati.disposizioni()[d];
    format!("{}: {}", x.nome, x.gradini[g].nome)
}

/// Un gradino della disposizione `d` diverso dal vigente in `p`.
fn altro_gradino(p: &Partita, d: usize) -> usize {
    usize::from(p.disposizioni().voci[d].in_vigore == 0)
}

/// Il giocatore di `p` con la promessa in sospeso `(d, g)` e `turni` turni da presidente prima
/// della chiusura del voto, che ne aggiunge uno; chiude il turno. Rende la valence del giocatore
/// dopo la chiusura e le mosse del suo partito.
fn al_voto_con(p: &Partita, promesse: &[(usize, usize)], turni: u32) -> (f64, Vec<String>) {
    let g = p.giocatore();
    let mut p = edita(p, |j| {
        j["partiti"][g]["promesse"] = serde_json::json!(promesse);
        j["partiti"][g]["turni_da_presidente"] = turni.into();
    })
    .unwrap();
    p.chiudi_turno();
    let id = p.partiti()[g].id.clone();
    let mosse = (p.vista().mosse.into_iter())
        .filter(|m| m.partito == id)
        .map(|m| m.testo)
        .collect();
    (p.partiti()[g].valence, mosse)
}

/// Mantenuta: il gradino promesso è in vigore al voto dopo. Con 3 turni chiusi da presidente la
/// chiusura del voto fa il quarto: la promessa conta, e la valence sale di
/// `valence_promessa_mantenuta`, meno il ritorno del turno.
#[test]
fn promessa_mantenuta_da_valence() {
    let dati = dati_base();
    let p = in_campagna();
    let (d, g) = in_vigore(&p, "eta_pensionabile");
    let (senza, _) = al_voto_con(&p, &[], 3);
    let (con, mosse) = al_voto_con(&p, &[(d, g)], 3);
    let atteso =
        dati.manopola("valence_promessa_mantenuta") * (1.0 - dati.manopola("ritorno_valence"));
    assert!((con - senza - atteso).abs() < 1e-12, "{con} {senza}");
    assert!(atteso > 0.0);
    assert!(
        mosse.contains(&format!("Promessa mantenuta: {}", nome(d, g))),
        "{mosse:?}"
    );
}

#[test]
fn promessa_mancata_toglie_di_piu() {
    let dati = dati_base();
    let p = in_campagna();
    let (d, _) = in_vigore(&p, "eta_pensionabile");
    let g = altro_gradino(&p, d);
    let (senza, _) = al_voto_con(&p, &[], 3);
    let (con, mosse) = al_voto_con(&p, &[(d, g)], 3);
    let mancata = dati.manopola("valence_promessa_mancata");
    let atteso = -mancata * (1.0 - dati.manopola("ritorno_valence"));
    assert!((con - senza - atteso).abs() < 1e-12, "{con} {senza}");
    assert!(mancata > dati.manopola("valence_promessa_mantenuta"));
    assert!(
        mosse.contains(&format!("Promessa mancata: {}", nome(d, g))),
        "{mosse:?}"
    );
}

/// Con 2 turni chiusi da presidente, la chiusura del voto fa il terzo: sotto i 4 la promessa
/// esce senza effetto e senza riga.
#[test]
fn senza_quattro_turni_la_promessa_non_conta() {
    let p = in_campagna();
    let (d, _) = in_vigore(&p, "eta_pensionabile");
    let g = altro_gradino(&p, d);
    let (senza, _) = al_voto_con(&p, &[], 2);
    let (con, mosse) = al_voto_con(&p, &[(d, g)], 2);
    assert_eq!(con, senza);
    assert!(
        !mosse.iter().any(|m| m.starts_with("Promessa")),
        "{mosse:?}"
    );
}

/// Al voto le promesse della campagna restano in sospeso fino al voto dopo, e il contatore dei
/// turni da presidente si azzera; nella vista le promesse portano i nomi del catalogo.
#[test]
fn al_voto_la_promessa_resta_in_sospeso() {
    let mut p = in_campagna();
    let g = p.giocatore();
    let (d, _) = in_vigore(&p, "eta_pensionabile");
    let gr = altro_gradino(&p, d);
    let dati = dati_base();
    let x = &dati.disposizioni()[d];
    let id_g = x.gradini[gr].id.clone();
    prometti(&mut p, &[("eta_pensionabile", &id_g)]).unwrap();
    assert_eq!(p.partiti()[g].promesse_campagna, vec![(d, gr)]);
    let vista = p.vista();
    let voce = &vista.partiti[g].promesse;
    assert_eq!(voce.len(), 1);
    assert_eq!(voce[0].disposizione, x.nome);
    assert_eq!(voce[0].gradino, x.gradini[gr].nome);
    assert!(!voce[0].in_sospeso);
    p.chiudi_turno();
    assert!(p.partiti()[g].promesse_campagna.is_empty());
    assert_eq!(p.partiti()[g].promesse, vec![(d, gr)]);
    assert!(p.vista().partiti[g].promesse[0].in_sospeso);
    assert!(p.partiti().iter().all(|x| x.turni_da_presidente == 0));
    let pc = p.governo().presidente;
    p.chiudi_turno();
    assert_eq!(p.partiti()[pc].turni_da_presidente, 1);
}

/// Il gradino della disposizione `d` diverso dal vigente che fa guadagnare di più (`verso` = 1) o
/// di meno (`verso` = −1) distanza al partito in `x`; il guadagno col segno di `verso`.
fn gradino_per(p: &Partita, d: usize, x: f64, verso: f64) -> Option<(usize, f64)> {
    let dati = dati_base();
    let dato = &dati.disposizioni()[d];
    let vigente = p.disposizioni().voci[d].in_vigore;
    let v = dato.posizione(vigente)?;
    (0..dato.gradini.len())
        .filter(|&g| g != vigente)
        .filter_map(|g| Some((g, verso * ((x - v).abs() - (x - dato.posizione(g)?).abs()))))
        .max_by(|a, b| a.1.total_cmp(&b.1))
}

/// La promessa più lontana e la più vicina al partito in `x`, fra tutte le disposizioni: (id
/// della disposizione, id del gradino, guadagno).
fn estrema(p: &Partita, x: f64, verso: f64) -> (String, String, f64) {
    let dati = dati_base();
    let (d, g, s) = (0..dati.disposizioni().len())
        .filter_map(|d| gradino_per(p, d, x, verso).map(|(g, s)| (d, g, s)))
        .max_by(|a, b| a.2.total_cmp(&b.2))
        .unwrap();
    let dato = &dati.disposizioni()[d];
    (dato.id.clone(), dato.gradini[g].id.clone(), verso * s)
}

/// Al governo, nel turno del voto, con l'intesa a 50 (la fedeltà non pesa sulla linea): la
/// promessa che allontana il centrodestra dalla sua posizione gli toglie `intesa_promessa` per il
/// peso delle politiche, con la causa `Promessa`; quella che lo avvicina no.
#[test]
fn la_promessa_costa_intesa_agli_alleati_con_linea_no() {
    let dati = dati_base();
    let p = in_campagna();
    assert!(p.governa());
    let cd = comune::partito("centrodestra");
    assert!(p.alleati().contains(&cd));
    let p = edita(&p, |j| j["intesa"]["valori"][cd] = 50.0.into()).unwrap();
    let x = p.partiti()[cd].posizione_dichiarata;
    let prima = p.intesa().valori[cd];
    let della_promessa = |p: &Partita| -> Vec<f64> {
        (p.intesa().variazioni.iter())
            .filter(|v| v.partito == cd && v.causa == CausaIntesa::Promessa)
            .map(|v| v.delta)
            .collect()
    };
    let (d, g, guadagno) = estrema(&p, x, -1.0);
    assert!(guadagno < 0.0, "{d} {g} {guadagno}");
    let mut lontana = copia(&p);
    prometti(&mut lontana, &[(&d, &g)]).unwrap();
    let costo =
        dati.manopola("intesa_promessa") * lontana.partiti()[cd].vocazione.peso_politiche(&dati);
    assert!(costo > 0.0);
    assert_eq!(della_promessa(&lontana), vec![-costo]);
    assert!((lontana.intesa().valori[cd] - (prima - costo)).abs() < 1e-12);
    let (d, g, guadagno) = estrema(&p, x, 1.0);
    assert!(guadagno > 0.0, "{d} {g} {guadagno}");
    let mut vicina = copia(&p);
    prometti(&mut vicina, &[(&d, &g)]).unwrap();
    assert!(della_promessa(&vicina).is_empty());
    assert_eq!(vicina.intesa().valori[cd], prima);
}

/// Subito, nel turno del voto: la promessa che porta una disposizione verso il centro (4) alza la
/// quota del giocatore nelle intenzioni vere, perché i gruppi con la posizione media al centro
/// sono più vicini al gradino promesso che al vigente; una promessa su una disposizione senza
/// posizione (l'elettorale) non sposta nulla.
#[test]
fn la_promessa_persuade_chi_ne_guadagna() {
    let p = in_campagna();
    let g = p.giocatore();
    let quota = |p: &Partita| p.intenzioni_vere().nazionali[g];
    let senza = quota(&p);
    let (d, gr, guadagno) = estrema(&p, 4.0, 1.0);
    assert!(guadagno > 0.0);
    let mut con = copia(&p);
    prometti(&mut con, &[(&d, &gr)]).unwrap();
    assert!(quota(&con) > senza, "{} {senza}", quota(&con));
    let dati = dati_base();
    let (k, elettorale) = (dati.disposizioni().iter().enumerate())
        .find(|(_, x)| x.gradini.iter().all(|g| g.posizione.is_none()))
        .unwrap();
    let gr = altro_gradino(&p, k);
    let mut senza_posizione = copia(&p);
    prometti(
        &mut senza_posizione,
        &[(&elettorale.id, &elettorale.gradini[gr].id)],
    )
    .unwrap();
    assert_eq!(quota(&senza_posizione), senza);
}

/// Il comando rifiutato col messaggio `atteso` non cambia nulla.
fn rifiutato(p: &mut Partita, promesse: &[(&str, &str)], atteso: &str) {
    let prima = p.salva().unwrap();
    let e = prometti(p, promesse).unwrap_err();
    assert_eq!(e, format!("comando non ammesso: {atteso}"));
    assert_eq!(p.salva().unwrap(), prima);
}

#[test]
fn promessa_rifiutata_non_cambia_nulla() {
    let dati = dati_base();
    let mut p = Partita::nuova(dati_base(), 1);
    while p.vista().turno < 18 {
        chiudi_col_bilancio(&mut p);
    }
    let (d, vigente) = in_vigore(&p, "eta_pensionabile");
    let x = &dati.disposizioni()[d];
    let altro = x.gradini[altro_gradino(&p, d)].id.clone();
    let e = "eta_pensionabile";
    rifiutato(
        &mut p,
        &[(e, &altro)],
        "Si promette solo in campagna elettorale",
    );
    chiudi_col_bilancio(&mut p);
    let y = &dati.disposizioni()[0];
    let z = &dati.disposizioni()[1];
    let tre = [
        (e, altro.as_str()),
        (y.id.as_str(), y.gradini[0].id.as_str()),
        (z.id.as_str(), z.gradini[0].id.as_str()),
    ];
    rifiutato(&mut p, &tre, "Al massimo 2 promesse");
    rifiutato(
        &mut p,
        &[("nessuna", "x")],
        "Disposizione sconosciuta: nessuna",
    );
    rifiutato(&mut p, &[(e, "nessuno")], "Gradino sconosciuto: nessuno");
    let (_, vigente_ora) = in_vigore(&p, e);
    assert_eq!(vigente_ora, vigente);
    rifiutato(
        &mut p,
        &[(e, &x.gradini[vigente].id)],
        &format!("Già in vigore: {}", nome(d, vigente)),
    );
    let g2 = altro_gradino(&p, d);
    rifiutato(
        &mut p,
        &[(e, &altro), (e, &altro)],
        &format!("Promessa ripetuta: {}", nome(d, g2)),
    );
    prometti(&mut p, &[(e, &altro)]).unwrap();
    rifiutato(
        &mut p,
        &[(e, &altro)],
        "Promesse già fatte in questa campagna",
    );
}

/// `carica` rifiuta più di 2 promesse per lista e gli indici fuori dal catalogo.
#[test]
fn carica_rifiuta_promesse_storte() {
    let p = Partita::nuova(dati_base(), 1);
    let n = dati_base().disposizioni().len();
    for (campo, v) in [
        ("promesse", serde_json::json!([[0, 0], [1, 0], [2, 0]])),
        ("promesse_campagna", serde_json::json!([[n, 0]])),
        ("promesse", serde_json::json!([[0, 99]])),
    ] {
        let e = edita(&p, |j| j["partiti"][0][campo] = v.clone()).unwrap_err();
        assert!(e.to_string().contains("più di 2 promesse"), "{e}");
    }
    let due = serde_json::json!([[0, 0], [1, 0]]);
    assert!(edita(&p, |j| j["partiti"][0]["promesse"] = due).is_ok());
}
