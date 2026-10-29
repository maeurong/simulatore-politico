#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

mod comune;
use comune::{chiudi_col_bilancio, dati_base, edita, file_base, partito};
use sim_core::Partita;
use sim_core::dati::DatiGioco;
use sim_core::legislatura::Fine;

const RIGA: &str = "Si scioglie dopo 2 elezioni senza seggi";

fn al_turno(p: &mut Partita, t: u32) {
    while p.vista().turno < t {
        chiudi_col_bilancio(p);
    }
}

/// La partita di prova con l'estrema sinistra a valence −3 (il minimo): zero seggi. Vota ai
/// turni 19 e 39.
fn senza_estrema_sinistra() -> Partita {
    let f = comune::sostituisci_valence(file_base(), "estrema_sinistra", -3.0);
    Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1)
}

#[test]
fn due_elezioni_senza_seggi_sciolgono() {
    let mut p = senza_estrema_sinistra();
    al_turno(&mut p, 19);
    chiudi_col_bilancio(&mut p);
    assert_eq!(p.partiti()[0].senza_seggi, 1);
    assert!(!p.partiti()[0].sciolto);
    assert!(p.vista().mosse.iter().all(|m| m.testo != RIGA));
    al_turno(&mut p, 39);
    chiudi_col_bilancio(&mut p);
    assert!(p.partiti()[0].sciolto);
    assert!(
        (p.vista().mosse.iter()).any(|m| m.testo == RIGA && m.partito == "estrema_sinistra"),
        "{:?}",
        p.vista().mosse
    );
    // Il turno dopo: quota zero nel sondaggio, il salvataggio si ricarica.
    chiudi_col_bilancio(&mut p);
    let ist = &p.informazione().sondaggio.istituti;
    assert!(ist.iter().all(|r| r.quote[0] == 0.0));
    assert_eq!(Partita::carica(&p.salva().unwrap()).unwrap(), p);
}

#[test]
fn una_elezione_con_seggi_azzera_il_conto() {
    let mut p = Partita::nuova(dati_base(), 1);
    al_turno(&mut p, 19);
    let sinistra = partito("sinistra");
    let mut p = edita(&p, |j| j["partiti"][sinistra]["senza_seggi"] = 1.into()).unwrap();
    chiudi_col_bilancio(&mut p);
    assert!(p.parlamento().seggi_camera[sinistra] > 0);
    assert_eq!(p.partiti()[sinistra].senza_seggi, 0);
    assert!(!p.partiti()[sinistra].sciolto);
}

#[test]
fn lo_sciolto_esce_dalle_coalizioni() {
    let mut p = senza_estrema_sinistra();
    al_turno(&mut p, 19);
    // Estrema sinistra (0, senza seggi, a un voto dallo scioglimento) in coalizione col
    // centrosinistra (2), con intesa; le coalizioni di prima non le contengono. Insieme stanno
    // sotto la soglia di coalizione: nessun premio, l'estrema sinistra resta senza seggi.
    let mut p = edita(&p, |j| {
        j["partiti"][0]["senza_seggi"] = 1.into();
        let c = j["legislatura"]["coalizioni"].as_array_mut().unwrap();
        for k in c.iter_mut() {
            k.as_array_mut()
                .unwrap()
                .retain(|x| ![0, 2].contains(&x.as_u64().unwrap()));
        }
        c.push(serde_json::json!([0, 2]));
        j["intesa"]["valori"][0] = 60.0.into();
    })
    .unwrap();
    chiudi_col_bilancio(&mut p);
    assert!(p.partiti()[0].sciolto);
    let l = p.legislatura();
    assert!(l.coalizioni.iter().flatten().all(|&j| j != 0 && j != 2));
    assert!(l.coalizioni.iter().all(|c| c.len() >= 2));
    assert_eq!(Partita::carica(&p.salva().unwrap()).unwrap(), p);
    // Il voto dopo, con lo sciolto presente: quota e seggi zero, il riparto riesce, si ricarica.
    al_turno(&mut p, 39);
    chiudi_col_bilancio(&mut p);
    assert_eq!(p.legislatura().elezioni.len(), 2);
    let esito = &p.legislatura().elezioni[0];
    let v = &esito.voti[0];
    assert_eq!((v.quota, v.camera, v.senato), (0.0, 0, 0));
    assert!(esito.voti.iter().map(|v| v.camera).sum::<u32>() > 0);
    assert_eq!(Partita::carica(&p.salva().unwrap()).unwrap(), p);
}

#[test]
fn il_giocatore_non_si_scioglie() {
    let f = comune::imposta(
        file_base(),
        "manopole",
        "[valence_partito_nuovo]\nvalore = ",
        "-3.0",
    );
    let f = comune::imposta(f, "manopole", "[luna_di_miele]\nvalore = ", "0.0");
    let fondazione = sim_core::legislatura::Fondazione {
        nome: "Partito nuovo".into(),
        posizione: 1.5,
    };
    let mut p = Partita::fonda(DatiGioco::da_testi(&f).unwrap(), 7, &fondazione).unwrap();
    p.chiudi_turno();
    assert!(p.finita());
    assert_eq!(p.eredita().unwrap().fine, Fine::SenzaSeggi);
    let g = p.giocatore();
    assert!(!p.partiti()[g].sciolto);
    assert_eq!(p.partiti()[g].senza_seggi, 0);
}

const PROTESTA: &str = "partito di protesta";

/// La partita di prova con la destra a 4,5 e l'estrema destra a 5: il tratto 6-7 resta scoperto.
/// `soglia_protesta` all'1% (minimo compreso), perché la quota del punto 7 non conti.
fn destra_al_centro() -> Partita {
    let f = comune::sostituisci(
        file_base(),
        "scenario",
        "posizione = 6.0",
        "posizione = 4.5",
    );
    let f = comune::sostituisci(f, "scenario", "posizione = 7.0", "posizione = 5.0");
    let prefisso = "[soglia_protesta]\nvalore = 0.1\nmin = ";
    let f = comune::imposta(f, "manopole", prefisso, "0.01");
    let f = comune::imposta(f, "manopole", "[soglia_protesta]\nvalore = ", "0.01");
    Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1)
}

#[test]
fn nasce_la_protesta_dove_l_asse_e_scoperto() {
    let mut p = destra_al_centro();
    al_turno(&mut p, 18);
    assert_eq!(p.partiti().len(), 7);
    chiudi_col_bilancio(&mut p); // apertura del turno 19, la campagna
    let nuovo = p.partiti().last().unwrap();
    assert_eq!(nuovo.id, "nuovo_7");
    assert!(
        nuovo.posizione_dichiarata >= 6.0,
        "{}",
        nuovo.posizione_dichiarata
    );
    assert_eq!(nuovo.posizione_percepita, nuovo.posizione_dichiarata);
    assert_eq!(nuovo.regola, sim_core::partiti::Regola::Hunter);
    assert_eq!(nuovo.vocazione, sim_core::partiti::Vocazione::Identitaria);
    assert_eq!(nuovo.cassa, 1_000_000);
    assert!(!nuovo.leader.is_empty());
    assert_eq!(p.parlamento().seggi_camera[7], 0);
    assert_eq!(p.parlamento().seggi_senato[7], 0);
    assert!(p.legislatura().protesta);
    let riga = format!(
        "Nasce {}, partito di protesta in posizione {}: elettori lontani dai partiti e astensione alta",
        nuovo.nome, nuovo.posizione_dichiarata
    );
    assert!(
        (p.vista().mosse.iter()).any(|m| m.partito == "nuovo_7" && m.testo == riga),
        "{:?}",
        p.vista().mosse
    );
    assert_eq!(Partita::carica(&p.salva().unwrap()).unwrap(), p);
    chiudi_col_bilancio(&mut p); // il voto
    let esito = &p.legislatura().elezioni[0];
    let voti = esito.voti.iter().find(|v| v.partito == "nuovo_7").unwrap();
    assert!(voti.quota > 0.0);
    assert!(!p.legislatura().protesta);
}

#[test]
fn una_sola_protesta_a_legislatura() {
    let mut p = destra_al_centro();
    al_turno(&mut p, 18);
    let mut p = edita(&p, |j| j["legislatura"]["protesta"] = true.into()).unwrap();
    chiudi_col_bilancio(&mut p);
    assert_eq!(p.partiti().len(), 7);
    assert!(p.vista().mosse.iter().all(|m| !m.testo.contains(PROTESTA)));
}
