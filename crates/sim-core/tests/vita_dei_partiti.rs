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
    // centrosinistra (2), con intesa; le coalizioni di prima non le contengono. Al turno della
    // campagna si vota con `coalizioni_ia`: lì la stessa coalizione. Insieme stanno sotto la
    // soglia di coalizione: nessun premio, l'estrema sinistra resta senza seggi.
    let mut p = edita(&p, |j| {
        j["partiti"][0]["senza_seggi"] = 1.into();
        for chiave in ["coalizioni", "coalizioni_ia"] {
            let c = j["legislatura"][chiave].as_array_mut().unwrap();
            for k in c.iter_mut() {
                k.as_array_mut()
                    .unwrap()
                    .retain(|x| ![0, 2].contains(&x.as_u64().unwrap()));
            }
            c.retain(|k| !k.as_array().unwrap().is_empty());
            c.push(serde_json::json!([0, 2]));
        }
        j["intesa"]["valori"][0] = 60.0.into();
    })
    .unwrap();
    chiudi_col_bilancio(&mut p);
    let coalizioni = &p.legislatura().elezioni[0].coalizioni;
    let premessa = ["estrema_sinistra", "centrosinistra"]
        .map(String::from)
        .to_vec();
    assert!(coalizioni.contains(&premessa), "{coalizioni:?}");
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
    destra_al_centro_da(file_base())
}

fn destra_al_centro_da(f: sim_core::dati::FileDati) -> Partita {
    let f = comune::sostituisci(f, "scenario", "posizione = 6.0", "posizione = 4.5");
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

const CENTRODESTRA: usize = 4;
const USCITA: &str =
    "Esce dalla maggioranza: nei sondaggi il voto gli vale più che restare al governo";

/// `p` con la percepita del partito `j` spostata di `d` dalla dichiarata.
fn scivolata(p: &Partita, j: usize, d: f64) -> Partita {
    let x = p.partiti()[j].posizione_dichiarata + d;
    edita(p, |v| v["partiti"][j]["posizione_percepita"] = x.into()).unwrap()
}

fn righe(p: &Partita, testo: &str) -> usize {
    (p.vista().mosse.iter())
        .filter(|m| m.testo.contains(testo))
        .count()
}

fn virgola(x: f64) -> String {
    format!("{x:.1}").replace('.', ",")
}

/// La partita di prova con `margine_voto_anticipato` a −100: ogni frangia di governo vuole uscire.
fn frangia_che_vuole_uscire() -> Partita {
    let prefisso = "[margine_voto_anticipato]\nvalore = 0.5\nmin = ";
    let f = comune::imposta(file_base(), "manopole", prefisso, "-100.0");
    let f = comune::imposta(
        f,
        "manopole",
        "[margine_voto_anticipato]\nvalore = ",
        "-100.0",
    );
    Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1)
}

/// Il centrodestra, al governo col giocatore, a un punto: la frangia nasce in chiusura col 12%
/// dei seggi (somme invariate) e della cassa (più le entrate del turno, che toccano a tutti e
/// due), nella coalizione senza dicasteri; l'origine riparte con la percepita sulla dichiarata.
/// Col margine di base la frangia resta.
#[test]
fn a_un_punto_il_partito_si_scinde() {
    let p = Partita::nuova(dati_base(), 1);
    let (camera, senato) = (
        p.parlamento().seggi_camera.clone(),
        p.parlamento().seggi_senato.clone(),
    );
    let cassa = p.partiti()[CENTRODESTRA].cassa;
    let x = p.partiti()[CENTRODESTRA].posizione_dichiarata + 1.0;
    let mut p = scivolata(&p, CENTRODESTRA, 1.0);
    assert!(p.governo().al_governo(CENTRODESTRA));
    chiudi_col_bilancio(&mut p);
    assert_eq!(p.partiti().len(), 8);
    let (o, f) = (&p.partiti()[CENTRODESTRA], &p.partiti()[7]);
    assert_eq!(f.id, "nuovo_7");
    assert_eq!((f.posizione_dichiarata, f.posizione_percepita), (x, x));
    assert_eq!(o.posizione_percepita, o.posizione_dichiarata);
    assert_eq!((f.vocazione, f.regola), (o.vocazione, o.regola));
    assert!(!f.leader.is_empty() && f.leader != o.leader);
    let quota = |n: u32| (f64::from(n) * 0.12).round() as u32;
    let pl = p.parlamento();
    let (c, s) = (pl.seggi_camera[7], pl.seggi_senato[7]);
    assert!(c > 0 && s > 0);
    assert_eq!(c, quota(camera[CENTRODESTRA]));
    assert_eq!(s, quota(senato[CENTRODESTRA]));
    assert_eq!(pl.seggi_camera[CENTRODESTRA] + c, camera[CENTRODESTRA]);
    assert_eq!(pl.seggi_senato[CENTRODESTRA] + s, senato[CENTRODESTRA]);
    assert_eq!(
        pl.seggi_camera.iter().sum::<u32>(),
        camera.iter().sum::<u32>()
    );
    assert_eq!(
        pl.seggi_senato.iter().sum::<u32>(),
        senato.iter().sum::<u32>()
    );
    assert!(f.cassa >= (cassa as f64 * 0.12).round() as i64);
    assert!(o.cassa > f.cassa);
    assert!(p.governo().coalizione.contains(&7));
    assert!(!p.governo().dicasteri.contains(&7));
    assert_eq!(p.legislatura().frange, vec![7]);
    let riga = format!(
        "Scissione: nasce {} con {c} {} e {s} {}, sulla posizione {}",
        f.nome,
        if c == 1 { "deputato" } else { "deputati" },
        if s == 1 { "senatore" } else { "senatori" },
        virgola(x)
    );
    assert!(
        (p.vista().mosse.iter()).any(|m| m.partito == "centrodestra" && m.testo == riga),
        "{:?}",
        p.vista().mosse
    );
    assert_eq!(righe(&p, USCITA), 0);
}

/// A 0,6 una riga di minaccia, nessuna nascita; il turno dopo, ancora sopra 0,5, nessuna riga
/// ripetuta; tornata sotto, la minaccia si spegne.
#[test]
fn a_mezzo_punto_solo_la_minaccia() {
    let p = Partita::nuova(dati_base(), 1);
    let d = p.partiti()[CENTRODESTRA].posizione_dichiarata;
    let mut p = scivolata(&p, CENTRODESTRA, 0.6);
    chiudi_col_bilancio(&mut p);
    assert_eq!(p.partiti().len(), 7);
    let distanza = p.partiti()[CENTRODESTRA].posizione_percepita - d;
    let riga = format!(
        "Minaccia di scissione: la posizione percepita dista {} punti dalla dichiarata",
        virgola(distanza)
    );
    assert!(
        (p.vista().mosse.iter()).any(|m| m.partito == "centrodestra" && m.testo == riga),
        "{:?}",
        p.vista().mosse
    );
    assert!(p.partiti()[CENTRODESTRA].minaccia);
    assert_eq!(Partita::carica(&p.salva().unwrap()).unwrap(), p);
    chiudi_col_bilancio(&mut p);
    assert!(p.partiti()[CENTRODESTRA].posizione_percepita - d >= 0.5);
    assert_eq!(righe(&p, "Minaccia di scissione"), 0);
    assert_eq!(p.partiti().len(), 7);
    let mut p = scivolata(&p, CENTRODESTRA, 0.0);
    chiudi_col_bilancio(&mut p);
    assert!(!p.partiti()[CENTRODESTRA].minaccia);
}

/// La partita vera: il giocatore resta allo stesso indice e senza leader, la frangia è IA.
#[test]
fn si_scinde_anche_il_partito_del_giocatore() {
    let mut p = comune::fondata(4.5);
    p.chiudi_turno(); // voto
    let g = p.giocatore();
    let n = p.partiti().len();
    let mut p = scivolata(&p, g, -1.0);
    p.chiudi_turno();
    assert_eq!(p.giocatore(), g);
    assert_eq!(p.partiti()[g].id, "giocatore");
    assert!(p.partiti()[g].leader.is_empty());
    assert_eq!(p.partiti()[g].posizione_percepita, 4.5);
    let f = &p.partiti()[n];
    assert_eq!(f.id, format!("nuovo_{n}"));
    assert!(!f.leader.is_empty());
    assert_eq!(f.posizione_dichiarata, 3.5);
    assert!(
        (p.vista().mosse.iter())
            .any(|m| m.partito == "giocatore" && m.testo.starts_with("Scissione")),
        "{:?}",
        p.vista().mosse
    );
    assert_eq!(Partita::carica(&p.salva().unwrap()).unwrap(), p);
}

/// La frangia del centrodestra appena nata, con `margine_voto_anticipato` a −100: nella chiusura in
/// cui nasce non esce (vita-dei-partiti:61-67).
fn frangia_appena_nata() -> Partita {
    let mut p = scivolata(&frangia_che_vuole_uscire(), CENTRODESTRA, 1.0);
    chiudi_col_bilancio(&mut p);
    assert_eq!(p.partiti().len(), 8);
    assert_eq!(righe(&p, USCITA), 0);
    assert!(p.governo().coalizione.contains(&7));
    p
}

/// Nata in una chiusura, esce in quella dopo. Senza dicasteri da lasciare, nessun rimpasto: i
/// dicasteri restano dove sono.
#[test]
fn la_frangia_esce_quando_le_conviene() {
    let mut p = frangia_appena_nata();
    let dicasteri = p.governo().dicasteri.clone();
    chiudi_col_bilancio(&mut p);
    assert!(!p.governo().coalizione.contains(&7));
    if p.legislatura().crisi.is_none() {
        assert_eq!(p.governo().dicasteri, dicasteri);
    }
    assert!(
        (p.vista().mosse.iter()).any(|m| m.partito == "nuovo_7" && m.testo == USCITA),
        "{:?}",
        p.vista().mosse
    );
    // Rimpasto con la maggioranza, crisi il turno dopo senza.
    let pl = p.parlamento();
    let seggi = |v: &[u32]| -> u32 { p.governo().coalizione.iter().map(|&j| v[j]).sum() };
    let maggioranza = 2 * seggi(&pl.seggi_camera) > pl.seggi_camera.iter().sum()
        && 2 * seggi(&pl.seggi_senato) > pl.seggi_senato.iter().sum();
    let crisi = p.legislatura().crisi == Some(p.vista().turno);
    assert_ne!(maggioranza, crisi);
    assert_eq!(Partita::carica(&p.salva().unwrap()).unwrap(), p);
    // Fuori dalla coalizione non esce più.
    chiudi_col_bilancio(&mut p);
    assert_eq!(righe(&p, USCITA), 0);
}

/// La frangia nata che vuole uscire: in crisi, con le dimissioni, in trattativa e nel turno del
/// voto resta.
#[test]
fn la_frangia_non_esce_fuori_finestra() {
    let nata = frangia_appena_nata();
    let t = nata.vista().turno;
    for (chiave, valore) in [
        ("crisi", serde_json::json!(t)),
        ("dimissioni", true.into()),
        ("fase", "trattativa".into()),
    ] {
        let mut p = edita(&nata, |j| j["legislatura"][chiave] = valore).unwrap();
        chiudi_col_bilancio(&mut p);
        assert_eq!(righe(&p, USCITA), 0, "{chiave}");
    }
    // Nata nella chiusura del 18, al voto del 19 resta; dopo il voto non è più una frangia.
    let mut p = frangia_che_vuole_uscire();
    al_turno(&mut p, 18);
    let mut p = scivolata(&p, CENTRODESTRA, 1.5);
    chiudi_col_bilancio(&mut p);
    assert_eq!(p.partiti().len(), 8);
    assert_eq!(p.legislatura().frange, vec![7]);
    chiudi_col_bilancio(&mut p);
    assert_eq!(righe(&p, USCITA), 0);
    assert!(p.legislatura().frange.is_empty());
}

/// La frangia e il centrodestra sotto `soglia_uscita` escono nella stessa chiusura: una sola
/// crisi, aperta per il turno dopo, e la riga d'uscita solo sulla frangia (l'uscita per l'intesa
/// non ne scrive).
#[test]
fn frangia_e_alleato_escono_insieme() {
    let p = edita(&frangia_appena_nata(), |j| {
        j["intesa"]["valori"][CENTRODESTRA] = 0.0.into();
    })
    .unwrap();
    let mut p = p;
    chiudi_col_bilancio(&mut p);
    let c = &p.governo().coalizione;
    assert!(!c.contains(&7) && !c.contains(&CENTRODESTRA), "{c:?}");
    assert_eq!(p.legislatura().crisi, Some(p.vista().turno));
    let uscite: Vec<String> = (p.vista().mosse.into_iter())
        .filter(|m| m.testo == USCITA)
        .map(|m| m.partito)
        .collect();
    assert_eq!(uscite, ["nuovo_7"]);
}

/// La frangia già in uscita per l'intesa non riceve anche la riga del valore atteso.
#[test]
fn la_frangia_uscita_per_l_intesa_non_ha_la_riga() {
    let p = edita(&frangia_appena_nata(), |j| {
        j["intesa"]["valori"][7] = 0.0.into();
    })
    .unwrap();
    let mut p = p;
    chiudi_col_bilancio(&mut p);
    assert!(!p.governo().coalizione.contains(&7));
    assert_eq!(righe(&p, USCITA), 0);
}

/// Con un solo nome nello scenario, «Orizzonte Comune», dato alla sinistra: la scissione resta
/// minaccia e la protesta non nasce. Con lo stesso nome libero nascono tutte e due.
#[test]
fn senza_nomi_liberi_nessuna_nascita() {
    let un_nome = || {
        let riga = "nomi_partiti = [\"Orizzonte Comune\"]";
        comune::riscrivi(file_base(), "scenario", "nomi_partiti = ", riga)
    };
    let preso = |p: &Partita| edita(p, |j| j["partiti"][1]["nome"] = "Orizzonte Comune".into());
    let scissione = |p: Partita| {
        let mut p = scivolata(&p, CENTRODESTRA, 1.0);
        chiudi_col_bilancio(&mut p);
        p
    };
    let dati = DatiGioco::da_testi(&un_nome()).unwrap();
    let p = scissione(preso(&Partita::nuova(dati, 1)).unwrap());
    assert_eq!(p.partiti().len(), 7);
    assert!(p.partiti()[CENTRODESTRA].minaccia);
    assert_eq!(righe(&p, "Minaccia di scissione"), 1);
    assert_eq!(righe(&p, "Scissione"), 0);
    let dati = DatiGioco::da_testi(&un_nome()).unwrap();
    assert_eq!(scissione(Partita::nuova(dati, 1)).partiti().len(), 8);

    let protesta = |p: Partita| {
        let mut p = p;
        al_turno(&mut p, 19);
        p
    };
    let p = protesta(preso(&destra_al_centro_da(un_nome())).unwrap());
    assert_eq!(p.partiti().len(), 7);
    assert_eq!(righe(&p, PROTESTA), 0);
    let p = protesta(destra_al_centro_da(un_nome()));
    assert_eq!(p.partiti().len(), 8);
}

#[test]
fn la_scissione_sopravvive_al_salvataggio() {
    let mut p = scivolata(&Partita::nuova(dati_base(), 1), CENTRODESTRA, 1.0);
    chiudi_col_bilancio(&mut p);
    assert_eq!(p.partiti().len(), 8);
    assert_eq!(Partita::carica(&p.salva().unwrap()).unwrap(), p);
    let e = edita(&p, |j| j["legislatura"]["frange"] = serde_json::json!([8])).unwrap_err();
    assert!(e.to_string().contains("legislatura.frange"), "{e}");
    let e = edita(&p, |j| {
        j["legislatura"]["frange"] = serde_json::json!([7, 7])
    })
    .unwrap_err();
    assert!(e.to_string().contains("legislatura.frange"), "{e}");
    edita(&p, |j| j["legislatura"]["frange"] = serde_json::json!([7])).unwrap();
}
