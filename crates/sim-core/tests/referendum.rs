mod comune;
use comune::{
    al_governo, all_opposizione, chiudi_col_bilancio, chiudi_senza_opposizione_ia, con_arrivo,
    con_seggi, dati_base, edita, imposta, partito, presenta, senza_shock, sostituisci,
};
use sim_core::dati::{DatiGioco, FileDati};
use sim_core::parlamento::{FormaMisura, TipoReferendum};
use sim_core::{Comando, Errore, Partita};

fn con(f: FileDati, nome: &str, valore: &str) -> FileDati {
    imposta(f, "manopole", &format!("[{nome}]\nvalore = "), valore)
}

/// Le tutele crescenti all'estremo destro e il reintegro al centro: chi sta sotto 5,5 preferisce
/// l'abrogazione, cioè quasi tutto l'elettorato.
fn abrogazione_che_vince(astensione: &str) -> FileDati {
    let f = sostituisci(
        senza_shock(),
        "disposizioni",
        "nome = \"tutele crescenti\", posizione = 4.0",
        "nome = \"tutele crescenti\", posizione = 7.0",
    );
    let f = sostituisci(
        f,
        "disposizioni",
        "nome = \"reintegro\", posizione = 2.0",
        "nome = \"reintegro\", posizione = 4.0",
    );
    con(f, "astensione_abrogativo", astensione)
}

fn al_voto(f: FileDati) -> Partita {
    let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    chiudi_senza_opposizione_ia(&mut p);
    p.promuovi_referendum("sinistra", "licenziamento").unwrap();
    // Turno 1 (gennaio-marzo 2027): firme in chiusura, voto ad aprile-giugno 2028 (turno 6).
    for _ in 0..6 {
        chiudi_col_bilancio(&mut p);
    }
    p
}

#[test]
fn abrogazione_riporta_al_valore_dopo_l_abrogazione_e_blocca_il_gradino() {
    let mut p = al_voto(abrogazione_che_vince("-5.0"));
    let r = &p.vista().esiti_referendum[0];
    assert_eq!(r.tipo, TipoReferendum::Abrogativo);
    // Il titolo è il bersaglio, il gradino abrogato.
    assert_eq!(r.nome, "Licenziamento: tutele crescenti");
    assert!(r.quorum && r.approvato);
    assert_eq!(p.in_vigore("licenziamento"), Some("reintegro"));
    let v = p.vista();
    let l = v
        .disposizioni
        .iter()
        .find(|d| d.id == "licenziamento")
        .unwrap();
    assert_eq!(l.legge, "referendum del 2028");
    assert!(
        l.gradini
            .iter()
            .any(|g| g.id == "tutele_crescenti" && g.bloccato)
    );
    let e = presenta(&mut p, &[("licenziamento", "tutele_crescenti")]).unwrap_err();
    assert!(
        e.to_string()
            .contains("Licenziamento: il gradino «tutele crescenti» è bloccato dal referendum"),
        "{e}"
    );
}

#[test]
fn senza_quorum_paga_il_promotore() {
    let valence = |p: &Partita| {
        p.partiti()
            .iter()
            .find(|x| x.id == "sinistra")
            .unwrap()
            .valence
    };
    let base = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
    let p = al_voto(abrogazione_che_vince("5.0"));
    assert!(!p.vista().esiti_referendum[0].quorum);
    assert_eq!(p.in_vigore("licenziamento"), Some("tutele_crescenti"));
    assert!(valence(&p) < valence(&base));
}

#[test]
fn invito_all_astensione_abbassa_l_affluenza() {
    let votanti = |invito: bool| {
        let mut p = Partita::nuova(
            DatiGioco::da_testi(&abrogazione_che_vince("0.0")).unwrap(),
            1,
        );
        chiudi_senza_opposizione_ia(&mut p);
        p.promuovi_referendum("sinistra", "licenziamento").unwrap();
        for t in 0..6 {
            if invito && t == 5 {
                p.comanda(Comando::InvitaAstensione).unwrap();
            }
            chiudi_col_bilancio(&mut p);
        }
        p.vista().esiti_referendum[0].votanti
    };
    assert!(votanti(true) < votanti(false));
}

#[test]
fn firme_in_chiusura_e_invito_una_volta() {
    let mut p = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
    p.chiudi_turno();
    p.promuovi_referendum("sinistra", "licenziamento").unwrap();
    let r = &p.vista().referendum[0];
    assert_eq!((r.tipo, r.firme), (TipoReferendum::Abrogativo, false));
    assert_eq!((r.voto.anno, r.voto.trimestre), (2028, 2));
    let prima = p.salva().unwrap();
    for (e, atteso) in [
        (
            p.comanda(Comando::InvitaAstensione),
            "nessun referendum da disertare",
        ),
        (
            p.promuovi_referendum("centrosinistra", "nucleare"),
            "un referendum è già in corso",
        ),
    ] {
        let e = e.unwrap_err();
        assert!(e.to_string().contains(atteso), "{e}");
    }
    assert_eq!(p.salva().unwrap(), prima);
    p.chiudi_turno();
    assert!(p.vista().referendum[0].firme);
    p.comanda(Comando::InvitaAstensione).unwrap();
    assert!(p.vista().referendum[0].invito_astensione);
    let e = p.comanda(Comando::InvitaAstensione).unwrap_err();
    assert!(e.to_string().contains("astensione già chiesta"), "{e}");
}

#[test]
fn reddito_minimo_abrogato_diventa_non_istituito() {
    // Categoriale all'estremo destro: il «non istituito» (5,5) è più vicino a chi sta sotto 6,25.
    let f = sostituisci(
        senza_shock(),
        "disposizioni",
        "nome = \"categoriale\", posizione = 4.0",
        "nome = \"categoriale\", posizione = 7.0",
    );
    let mut p = Partita::nuova(
        DatiGioco::da_testi(&con(f, "astensione_abrogativo", "-5.0")).unwrap(),
        1,
    );
    chiudi_senza_opposizione_ia(&mut p);
    p.promuovi_referendum("estrema_destra", "reddito_minimo")
        .unwrap();
    for _ in 0..6 {
        p.chiudi_turno();
    }
    assert!(p.vista().esiti_referendum[0].approvato);
    assert_eq!(p.in_vigore("reddito_minimo"), Some("non_istituito"));
}

#[test]
fn abrogazione_cancella_il_valore_in_arrivo() {
    let f = con_arrivo(
        abrogazione_che_vince("-5.0"),
        "licenziamento",
        "solo_indennizzo",
        2029,
        1,
    );
    let mut p = al_voto(f);
    assert!(p.vista().esiti_referendum[0].approvato);
    // Dal turno 7 al 10: l'arrivo di gennaio-marzo 2029 (turno 9) non c'è più.
    for _ in 0..3 {
        p.chiudi_turno();
    }
    assert_eq!(p.in_vigore("licenziamento"), Some("reintegro"));
}

#[test]
fn arrivo_in_corso_rende_ammesso_il_quesito() {
    // Nucleare: vigente e precedente «vietato», ma «consentito» in arrivo a ottobre 2027.
    let mut p = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
    p.chiudi_turno();
    p.promuovi_referendum("sinistra", "nucleare").unwrap();
}

#[test]
fn promozioni_non_ammesse_rifiutate_col_nome() {
    let mut p = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
    let prima = p.salva().unwrap();
    for ((promotore, disposizione), nome) in [
        (("ignoto", "licenziamento"), "partito sconosciuto: ignoto"),
        (
            ("destra", "licenziamento"),
            "il promotore è al governo: Destra",
        ),
        (("sinistra", "ignota"), "disposizione sconosciuta: ignota"),
        (
            ("sinistra", "accise"),
            "Accise sui carburanti non è referendabile",
        ),
        (
            ("sinistra", "eta_pensionabile"),
            "niente da abrogare: Età pensionabile",
        ),
        (("sinistra", "licenziamento"), "fuori finestra"),
    ] {
        let e = p.promuovi_referendum(promotore, disposizione).unwrap_err();
        assert!(
            matches!(e, Errore::Comando(_)) && e.to_string().contains(nome),
            "{e}"
        );
    }
    assert_eq!(p.salva().unwrap(), prima);
    assert!(p.comanda(Comando::InvitaAstensione).is_err());
}

#[test]
fn gradino_abrogato_non_torna_con_un_secondo_referendum() {
    // Dopo l'abrogazione il precedente è «tutele crescenti», bloccato: un secondo quesito lo
    // riporterebbe in vigore.
    let mut p = al_voto(abrogazione_che_vince("-5.0"));
    assert!(p.vista().esiti_referendum[0].approvato);
    let e = p
        .promuovi_referendum("sinistra", "licenziamento")
        .unwrap_err();
    assert!(
        e.to_string().contains(
            "Licenziamento: il gradino che tornerebbe in vigore è già stato abrogato da un \
             referendum"
        ),
        "{e}"
    );
}

/// Il Rosatellum senza coalizioni: abrogare le coalizioni di oggi le toglie.
fn coalizioni_abrogabili() -> FileDati {
    sostituisci(
        senza_shock(),
        "scenario",
        "coalizioni = true",
        "coalizioni = false",
    )
}

/// Porta il premio di maggioranza della partita al gradino `gradino`, passando dal salvataggio.
fn con_premio(p: &Partita, gradino: &str) -> Partita {
    let dati = DatiGioco::da_testi(&coalizioni_abrogabili()).unwrap();
    let k = dati.disposizione("premio").unwrap();
    let g = dati.disposizioni()[k].gradino(gradino).unwrap();
    edita(p, |j| j["disposizioni"]["voci"][k]["in_vigore"] = g.into()).unwrap()
}

#[test]
fn quesito_elettorale_inammissibile_col_nome() {
    // Senza coalizioni resterebbe il premio alla coalizione della legge in vigore.
    let mut p = Partita::nuova(DatiGioco::da_testi(&coalizioni_abrogabili()).unwrap(), 1);
    p.chiudi_turno();
    let prima = p.salva().unwrap();
    let e = p.promuovi_referendum("sinistra", "coalizioni").unwrap_err();
    assert!(
        e.to_string().contains(
            "quesito inammissibile su Coalizioni: premio alla coalizione senza coalizioni"
        ),
        "{e}"
    );
    assert_eq!(p.salva().unwrap(), prima);
}

#[test]
fn elettorale_resa_non_valida_prima_del_voto_decade() {
    let mut p = Partita::nuova(DatiGioco::da_testi(&coalizioni_abrogabili()).unwrap(), 1);
    p.chiudi_turno();
    let mut p = con_premio(&p, "nessuno");
    p.promuovi_referendum("sinistra", "coalizioni").unwrap();
    // Il premio alla coalizione torna prima del voto: senza coalizioni non sarebbe applicabile.
    let mut p = con_premio(&p, "coalizione");
    for _ in 0..6 {
        p.chiudi_turno();
    }
    let r = &p.vista().esiti_referendum[0];
    assert!(!r.approvato && r.votanti == 0, "{r:?}");
    let d = DatiGioco::da_testi(&coalizioni_abrogabili()).unwrap();
    let x = &d.disposizioni()[d.disposizione("coalizioni").unwrap()];
    let vigente = &x.gradini[x.gradino("si").unwrap()].nome;
    assert_eq!(r.nome, format!("{}: {vigente}", x.nome));
    let motivo = r.decaduto.as_deref().unwrap_or_default();
    assert!(motivo.starts_with("Decaduto senza voto"), "{motivo}");
    assert_eq!(p.in_vigore("coalizioni"), Some("si"));
    assert!(Partita::carica(&p.salva().unwrap()).is_ok());
}

#[test]
fn salva_e_carica_con_l_abrogativo_in_corso_non_cambiano_l_esito() {
    let gioca = |ricarica: bool| {
        let rileggi = |p: Partita| {
            if ricarica {
                Partita::carica(&p.salva().unwrap()).unwrap()
            } else {
                p
            }
        };
        let f = abrogazione_che_vince("0.0");
        let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
        chiudi_senza_opposizione_ia(&mut p);
        p.promuovi_referendum("sinistra", "licenziamento").unwrap();
        let mut p = rileggi(p);
        p.chiudi_turno();
        let mut p = rileggi(p);
        p.comanda(Comando::InvitaAstensione).unwrap();
        let mut p = rileggi(p);
        for _ in 0..5 {
            p.chiudi_turno();
        }
        p
    };
    let (sempre, ricaricata) = (gioca(false), gioca(true));
    assert_eq!(sempre.vista().esiti_referendum.len(), 1);
    assert_eq!(
        sempre.vista().esiti_referendum,
        ricaricata.vista().esiti_referendum
    );
    assert_eq!(sempre.salva().unwrap(), ricaricata.salva().unwrap());
}

/// La disposizione col quesito in corso è in esame: nessuna legge la tocca prima del voto.
#[test]
fn abrogativo_in_corso_mette_la_disposizione_in_esame() {
    let mut p = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
    p.chiudi_turno();
    p.promuovi_referendum("sinistra", "licenziamento").unwrap();
    let prima = p.salva().unwrap();
    let e = presenta(&mut p, &[("licenziamento", "reintegro")]).unwrap_err();
    assert!(
        matches!(e, Errore::Comando(_)) && e.to_string().contains("Licenziamento è già in esame"),
        "{e}"
    );
    assert_eq!(p.salva().unwrap(), prima);
}

/// La disposizione in una legge in calendario è in esame: niente quesito sopra (iter-legislativo-italiano:131-137).
#[test]
fn legge_in_calendario_blocca_l_abrogativo() {
    let mut p = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
    p.chiudi_turno();
    presenta(&mut p, &[("licenziamento", "reintegro")]).unwrap();
    let prima = p.salva().unwrap();
    let e = p
        .promuovi_referendum("sinistra", "licenziamento")
        .unwrap_err();
    assert!(
        matches!(e, Errore::Comando(_)) && e.to_string().contains("Licenziamento è già in esame"),
        "{e}"
    );
    assert_eq!(p.salva().unwrap(), prima);
}

/// Un abrogativo col voto oltre l'anno e mezzo, senza firme e col voto che non è quello del
/// turno, o non richiesto, è un salvataggio manomesso.
#[test]
fn abrogativo_manomesso_al_caricamento_rifiutato() {
    let mut p = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
    p.chiudi_turno();
    p.promuovi_referendum("sinistra", "licenziamento").unwrap();
    assert!(Partita::carica(&p.salva().unwrap()).is_ok());
    let fuori = "un referendum fuori dalle disposizioni, dai gradini o dai partiti";
    let abrogativo = "l'abrogativo non è richiesto, oppure è senza firme e il suo voto non cade nel turno chiesto";
    for (voto, firme, richiesto, atteso) in [
        (1000, true, true, fuori),
        (7, false, true, abrogativo),
        (6, false, false, abrogativo),
    ] {
        let e = edita(&p, |j| {
            j["parlamento"]["referendum"]["voto"] = voto.into();
            j["parlamento"]["referendum"]["firme"] = firme.into();
            j["parlamento"]["referendum"]["richiesto"] = richiesto.into();
        })
        .err();
        assert!(
            matches!(&e, Some(Errore::SalvataggioIlleggibile(m)) if m.contains(atteso)),
            "{voto} {firme} {richiesto}: {e:?}"
        );
    }
}

fn al_voto_su(f: FileDati, disposizione: &str) -> Partita {
    let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    chiudi_senza_opposizione_ia(&mut p);
    p.promuovi_referendum("sinistra", disposizione).unwrap();
    for _ in 0..6 {
        chiudi_col_bilancio(&mut p);
    }
    p
}

fn affluenza(p: &Partita) -> f64 {
    let e = &p.vista().esiti_referendum[0];
    e.votanti as f64 / e.aventi_diritto as f64
}

#[test]
fn l_abrogativo_passa_il_quorum_solo_sui_temi_che_toccano_molti() {
    // Il regime delle pensioni tocca quattro condizioni su sei.
    assert!(
        al_voto_su(senza_shock(), "regime_pensioni")
            .vista()
            .esiti_referendum[0]
            .quorum
    );
    for d in ["licenziamento", "tetto_contante", "nucleare"] {
        let p = al_voto_su(senza_shock(), d);
        assert!(
            !p.vista().esiti_referendum[0].quorum,
            "{d}: {}",
            affluenza(&p)
        );
    }
}

#[test]
fn senza_gruppi_toccati_l_affluenza_dell_abrogativo_resta_bassa() {
    let p = al_voto_su(senza_shock(), "nucleare");
    assert!(affluenza(&p) < 0.35, "{}", affluenza(&p));
}

#[test]
fn abrogazione_approvata_costa_a_tutta_la_coalizione() {
    let prima = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
    let p = al_voto(abrogazione_che_vince("-5.0"));
    let persa = |id: &str| {
        let v = |q: &Partita| q.partiti().iter().find(|x| x.id == id).unwrap().valence;
        v(&prima) - v(&p)
    };
    let dati = DatiGioco::da_testi(&senza_shock()).unwrap();
    let pieno =
        dati.manopola("valence_referendum_perso") * (1.0 - dati.manopola("ritorno_valence"));
    for id in ["destra", "centrodestra"] {
        assert!((persa(id) - pieno).abs() < 1e-9, "{id}: {}", persa(id));
    }
    assert!(persa("sinistra").abs() < 1e-9);
}

#[test]
fn sull_abrogativo_elettorale_gli_elettori_votano_i_seggi() {
    // Il premio alla coalizione è abrogabile: chi ne guadagna seggi è votato dalla maggioranza.
    // Il ripescaggio no: nel sondaggio iniziale nessuna lista di coalizione resta sotto soglia,
    // i seggi non cambiano e il quesito finisce pari.
    let p = al_voto_su(
        con(senza_shock(), "astensione_abrogativo", "-5.0"),
        "premio",
    );
    let e = &p.vista().esiti_referendum[0];
    // il premio sposta i seggi, quindi sì e no non finiscono pari
    assert!(e.no > e.si, "{e:?}");
}

#[test]
fn un_valore_gia_abrogato_prima_del_voto_fa_decadere_il_quesito() {
    // Il reintegro entra in vigore al turno 3, prima del voto (turno 6); in esame la
    // disposizione non accetta leggi, quindi l'arrivo sta nei dati.
    let f = con_arrivo(senza_shock(), "licenziamento", "reintegro", 2027, 3);
    let p = al_voto_su(f, "licenziamento");
    assert_eq!(p.in_vigore("licenziamento"), Some("reintegro"));
    let e = &p.vista().esiti_referendum[0];
    assert_eq!(
        e.decaduto.as_deref(),
        Some("Decaduto senza voto: la disposizione ha già il valore che il quesito le darebbe")
    );
    assert!(!e.quorum && !e.approvato);
}

#[test]
fn un_arrivo_pendente_non_fa_decadere_il_quesito_sul_valore_gia_in_vigore() {
    // Partenza e precedente coincidono: il quesito ammesso colpisce l'arrivo, non il vigente.
    let f = sostituisci(
        senza_shock(),
        "disposizioni",
        "partenza = \"tutele_crescenti\"\nlegge = \"D.Lgs. 23/2015, art. 3\"",
        "partenza = \"reintegro\"\nlegge = \"D.Lgs. 23/2015, art. 3\"",
    );
    let f = con_arrivo(f, "licenziamento", "solo_indennizzo", 2029, 4);
    let p = al_voto_su(con(f, "astensione_abrogativo", "-5.0"), "licenziamento");
    let e = &p.vista().esiti_referendum[0];
    assert!(e.decaduto.is_none(), "{e:?}");
    assert!(e.si + e.no > 0, "{e:?}");
}

#[test]
fn un_abrogativo_su_un_gradino_spostato_dalla_legge_vota_quel_gradino() {
    // Il solo indennizzo all'estremo destro: l'abrogazione verso il reintegro vince.
    let f = sostituisci(
        abrogazione_che_vince("-5.0"),
        "disposizioni",
        "nome = \"solo indennizzo\", posizione = 5.5",
        "nome = \"solo indennizzo\", posizione = 7.0",
    );
    let mut p = al_voto_su(
        con_arrivo(f, "licenziamento", "solo_indennizzo", 2027, 3),
        "licenziamento",
    );
    let e = &p.vista().esiti_referendum[0];
    assert!(e.decaduto.is_none(), "{e:?}");
    assert_eq!(e.nome, "Licenziamento: solo indennizzo");
    assert!(e.approvato, "{e:?}");
    assert_eq!(p.in_vigore("licenziamento"), Some("reintegro"));
    let err = presenta(&mut p, &[("licenziamento", "solo_indennizzo")]).unwrap_err();
    assert!(err.to_string().contains("bloccato dal referendum"), "{err}");
}

/// Seggi di un ramo da `totale`: `quote` (partito, seggi) scritte a mano, il resto al partito del
/// Presidente del Consiglio, nessuno agli altri.
fn seggi_a(p: &Partita, quote: &[(usize, u32)], totale: u32) -> Vec<u32> {
    let pdc = p.governo().presidente;
    let scritti: u32 = quote.iter().map(|q| q.1).sum();
    (0..p.partiti().len())
        .map(|j| match quote.iter().find(|q| q.0 == j) {
            Some(&(_, n)) => n,
            None if j == pdc => totale - scritti,
            None => 0,
        })
        .collect()
}

/// Da `all_opposizione()`: la costituzionale passa senza i due terzi col solo partito del
/// giocatore contro, oltre un quinto (140 su 400, 70 su 200). Il confermativo aspetta il voto.
fn costituzionale_senza_due_terzi_all_opposizione() -> Partita {
    let p = all_opposizione();
    let g = p.giocatore();
    let p = costituzionale_al_voto(&p, Some(false), &[(g, 140)], &[(g, 70)]);
    assert_eq!(
        p.parlamento().confermativi.len(),
        1,
        "{:?}",
        p.vista().esiti
    );
    p
}

/// Da `all_opposizione()`, storia 97: il giocatore vota sì (90 su 400, 45 su 200), il
/// centrosinistra al governo vota no (guadagno −1: 3,0 contro 4,5 → 5,5) e toglie i due terzi (150
/// e 75); fuori dal governo nessuno è contro, quindi niente quinto contro. Col giocatore il quinto
/// ci sarebbe: il confermativo aspetta la sua richiesta.
fn costituzionale_col_si_del_giocatore() -> Partita {
    let p = all_opposizione();
    let (g, cs) = (p.giocatore(), partito("centrosinistra"));
    let p = costituzionale_al_voto(&p, Some(true), &[(g, 90), (cs, 150)], &[(g, 45), (cs, 75)]);
    assert_eq!(
        p.parlamento().confermativi.len(),
        1,
        "{:?}",
        p.vista().esiti
    );
    p
}

/// Da `p`: i seggi `camera` e `senato` (`seggi_a` su 400 e 200), poi la costituzionale del governo
/// (300 deputati e 150 senatori) in seconda deliberazione col voto nel turno e la linea `linea`
/// del giocatore (`None` = quella di default), scritta con `edita`; poi la chiusura.
fn costituzionale_al_voto(
    p: &Partita,
    linea: Option<bool>,
    camera: &[(usize, u32)],
    senato: &[(usize, u32)],
) -> Partita {
    let p = con_seggi(p, &seggi_a(p, camera, 400), &seggi_a(p, senato, 200));
    let dati = dati_base();
    let k = dati.disposizione("numero_parlamentari").unwrap();
    let g = dati.disposizioni()[k].gradino("300_150").unwrap();
    let mut p = edita(&p, |j| {
        let (id, turno) = (j["parlamento"]["prossima"].clone(), j["turno"].clone());
        j["parlamento"]["prossima"] = (id.as_u64().unwrap() + 1).into();
        let misura = serde_json::json!({
            "id": id, "forma": "costituzionale", "valori": [[k, g]], "proponente": null,
            "promotore": null, "linea_giocatore": linea, "presentata": turno, "voto": turno,
            "deliberazione": 2, "fiducia": false, "emendamenti": [],
            "emendamenti_aperti": false, "fondi": null,
        });
        (j["parlamento"]["misure"].as_array_mut().unwrap()).push(misura);
    })
    .unwrap();
    p.chiudi_turno();
    p
}

fn confermativo(p: &Partita, chiedi: bool) -> Comando {
    let misura = p.parlamento().confermativi[0].misura.unwrap();
    Comando::Confermativo { misura, chiedi }
}

#[test]
fn il_giocatore_rinuncia_al_confermativo() {
    let mut p = costituzionale_senza_due_terzi_all_opposizione();
    p.comanda(confermativo(&p, false)).unwrap();
    assert!(!p.vista().referendum[0].richiesto);
    p.chiudi_turno();
    assert!(p.parlamento().confermativi.is_empty());
    assert!(p.vista().esiti_referendum.is_empty());
    assert_eq!(p.in_vigore("numero_parlamentari"), Some("300_150"));
}

/// Il giocatore (90 su 400, 45 su 200) e l'estrema sinistra (90 e 45), che perde 1 dalla legge,
/// votano no: l'estrema sinistra da sola fa un quinto, e la rinuncia del giocatore non toglie il
/// confermativo.
#[test]
fn la_rinuncia_non_toglie_il_quinto_degli_altri() {
    let p = all_opposizione();
    let (g, es) = (p.giocatore(), partito("estrema_sinistra"));
    let mut p = costituzionale_al_voto(&p, Some(false), &[(g, 90), (es, 90)], &[(g, 45), (es, 45)]);
    assert_eq!(
        p.parlamento().confermativi.len(),
        1,
        "{:?}",
        p.vista().esiti
    );
    p.comanda(confermativo(&p, false)).unwrap();
    assert!(p.vista().referendum[0].richiesto);
}

/// Review Focus 5: il confermativo non richiesto, la sfiducia dell'opposizione IA presentata e
/// l'abrogativo IA in corso tornano uguali da `carica(salva())`.
#[test]
fn salva_e_carica_col_confermativo_non_richiesto_la_sfiducia_e_l_abrogativo_ia() {
    let p = costituzionale_col_si_del_giocatore();
    assert!(!p.parlamento().confermativi[0].richiesto);
    let k = dati_base().disposizione("licenziamento").unwrap();
    let ed = partito("estrema_destra");
    let p = edita(&p, |j| {
        j["parlamento"]["sfiducia"] = serde_json::json!([null, ed]);
        let voto = j["turno"].as_u64().unwrap() + 1;
        j["parlamento"]["referendum"] = serde_json::json!({
            "tipo": "abrogativo", "misura": null, "valori": [[k, 0]], "promotore": ed,
            "del_giocatore": false, "voto": voto, "firme": true, "invito_astensione": false,
            "richiesto": true,
        });
    })
    .unwrap();
    assert_eq!(p.parlamento().sfiducia, Some((None, ed)));
    assert_eq!(Partita::carica(&p.salva().unwrap()).unwrap(), p);
}

#[test]
fn il_giocatore_ci_ripensa_e_chiede_il_confermativo() {
    let mut p = costituzionale_senza_due_terzi_all_opposizione();
    p.comanda(confermativo(&p, false)).unwrap();
    p.comanda(confermativo(&p, true)).unwrap();
    assert!(p.vista().referendum[0].richiesto);
    p.chiudi_turno();
    assert_eq!(
        p.vista().esiti_referendum[0].tipo,
        TipoReferendum::Confermativo
    );
}

#[test]
fn senza_scelta_vale_la_regola_dell_08() {
    let mut p = costituzionale_senza_due_terzi_all_opposizione();
    let v = p.vista();
    assert!(v.referendum[0].richiesto && v.referendum[0].scelta);
    p.chiudi_turno();
    assert!(p.parlamento().confermativi.is_empty());
    let r = &p.vista().esiti_referendum[0];
    assert_eq!(r.tipo, TipoReferendum::Confermativo);
    assert_eq!(
        p.in_vigore("numero_parlamentari") == Some("300_150"),
        r.approvato
    );
}

#[test]
fn chiedere_senza_un_quinto_e_rifiutato() {
    let p = costituzionale_senza_due_terzi_all_opposizione();
    // Il partito del giocatore scende sotto un quinto: 70 su 400, 35 su 200.
    let g = p.giocatore();
    let mut p = con_seggi(
        &p,
        &seggi_a(&p, &[(g, 70)], 400),
        &seggi_a(&p, &[(g, 35)], 200),
    );
    let prima = p.salva().unwrap();
    let ignota = Comando::Confermativo {
        misura: 9999,
        chiedi: true,
    };
    for (c, atteso) in [
        (
            confermativo(&p, true),
            "per chiedere il referendum serve un quinto dei seggi di una Camera: il tuo partito \
             più i partiti contrari alla misura",
        ),
        (ignota, "nessun confermativo da chiedere su questa misura"),
    ] {
        let e = p.comanda(c).unwrap_err();
        assert!(
            matches!(e, Errore::Comando(_)) && e.to_string().contains(atteso),
            "{e}"
        );
    }
    assert_eq!(p.salva().unwrap(), prima);
}

#[test]
fn col_si_del_giocatore_il_confermativo_aspetta_la_sua_richiesta() {
    let mut p = costituzionale_col_si_del_giocatore();
    let v = p.vista();
    assert!(!v.referendum[0].richiesto && v.referendum[0].scelta);
    assert_eq!(p.in_vigore("numero_parlamentari"), Some("400_200"));
    // Senza richiesta: niente voto, niente esito, la legge entra in vigore.
    p.chiudi_turno();
    assert!(p.parlamento().confermativi.is_empty());
    assert!(p.vista().esiti_referendum.is_empty());
    assert_eq!(p.in_vigore("numero_parlamentari"), Some("300_150"));
    // Con la richiesta si vota.
    let mut q = costituzionale_col_si_del_giocatore();
    q.comanda(confermativo(&q, true)).unwrap();
    assert!(q.vista().referendum[0].richiesto);
    q.chiudi_turno();
    assert_eq!(
        q.vista().esiti_referendum[0].tipo,
        TipoReferendum::Confermativo
    );
}

/// Il giocatore governa col centrodestra, che con l'intesa a 0 vota no (guadagno 0 meno la
/// fedeltà) e toglie i due terzi; fuori dal governo nessuno ha seggi: niente confermativo, la
/// legge entra subito in vigore.
#[test]
fn al_governo_senza_quinto_contro_la_costituzionale_entra_subito_in_vigore() {
    let cd = partito("centrodestra");
    let p = edita(&al_governo(), |j| j["intesa"]["valori"][cd] = 0.0.into()).unwrap();
    let p = costituzionale_al_voto(&p, None, &[(cd, 150)], &[(cd, 75)]);
    let e = (p.parlamento().esiti.iter())
        .find(|e| e.forma == FormaMisura::Costituzionale)
        .unwrap();
    assert!(e.approvata && e.camera.si * 3 < 800, "{e:?}");
    assert!(p.parlamento().confermativi.is_empty());
    assert_eq!(p.in_vigore("numero_parlamentari"), Some("300_150"));
}

#[test]
fn al_governo_il_confermativo_e_rifiutato_dalla_guardia() {
    let mut p = Partita::nuova(dati_base(), 1);
    let c = Comando::Confermativo {
        misura: 0,
        chiedi: true,
    };
    let e = p.comanda(c).unwrap_err();
    assert!(
        e.to_string().contains("solo all'opposizione: governi tu"),
        "{e}"
    );
}

/// Sull'abrogativo il giocatore all'opposizione non ha nulla da scegliere.
#[test]
fn sull_abrogativo_nessuna_scelta() {
    let p = all_opposizione();
    let k = dati_base().disposizione("licenziamento").unwrap();
    let p = edita(&p, |j| {
        let voto = j["turno"].as_u64().unwrap() + 1;
        j["parlamento"]["referendum"] = serde_json::json!({
            "tipo": "abrogativo", "misura": null, "valori": [[k, 0]], "promotore": 0,
            "del_giocatore": false, "voto": voto, "firme": true, "invito_astensione": false,
            "richiesto": true,
        });
    })
    .unwrap();
    let r = &p.vista().referendum[0];
    assert_eq!(r.tipo, TipoReferendum::Abrogativo);
    assert!(!r.scelta);
}
