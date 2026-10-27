mod comune;
use comune::{con_arrivo, edita, imposta, presenta, senza_shock, sostituisci};
use sim_core::dati::{DatiGioco, FileDati};
use sim_core::parlamento::TipoReferendum;
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
    p.chiudi_turno();
    p.promuovi_referendum("sinistra", "licenziamento").unwrap();
    // Turno 1 (gennaio-marzo 2027): firme in chiusura, voto ad aprile-giugno 2028 (turno 6).
    for _ in 0..6 {
        p.chiudi_turno();
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
        p.chiudi_turno();
        p.promuovi_referendum("sinistra", "licenziamento").unwrap();
        for t in 0..6 {
            if invito && t == 5 {
                p.comanda(Comando::InvitaAstensione).unwrap();
            }
            p.chiudi_turno();
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
    p.chiudi_turno();
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
        p.chiudi_turno();
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

/// Un abrogativo col voto oltre l'anno e mezzo, o senza firme e col voto che non è quello del
/// turno, è un salvataggio manomesso.
#[test]
fn abrogativo_manomesso_al_caricamento_rifiutato() {
    let mut p = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
    p.chiudi_turno();
    p.promuovi_referendum("sinistra", "licenziamento").unwrap();
    assert!(Partita::carica(&p.salva().unwrap()).is_ok());
    for (voto, firme) in [(1000, true), (7, false)] {
        let e = edita(&p, |j| {
            j["parlamento"]["referendum"]["voto"] = voto.into();
            j["parlamento"]["referendum"]["firme"] = firme.into();
        })
        .err();
        assert!(
            matches!(e, Some(Errore::SalvataggioIlleggibile(_))),
            "{voto} {firme}: {e:?}"
        );
    }
}

fn al_voto_su(f: FileDati, disposizione: &str) -> Partita {
    let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    p.chiudi_turno();
    p.promuovi_referendum("sinistra", disposizione).unwrap();
    for _ in 0..6 {
        p.chiudi_turno();
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
