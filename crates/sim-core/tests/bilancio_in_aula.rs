mod comune;
use comune::{edita, imposta, presenta, senza_shock};
use sim_core::dati::{DatiGioco, FileDati};
use sim_core::economia::Voce;
use sim_core::parlamento::{CausaIntesa, FormaMisura, StatoEmendamento};
use sim_core::{Comando, Errore, Partita, Trimestre};

fn al_turno(f: FileDati, turni: usize) -> Partita {
    let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    for _ in 0..turni {
        p.chiudi_turno();
    }
    p
}

fn presenta_bozza(p: &mut Partita) {
    let b = p.bozza_bilancio().unwrap();
    p.comanda(Comando::PresentaBilancio { bilancio: b })
        .unwrap();
}

#[test]
fn bilancio_si_vota_e_passa_con_la_maggioranza() {
    let mut p = al_turno(senza_shock(), 4);
    presenta_bozza(&mut p);
    assert!(
        p.vista()
            .calendario
            .iter()
            .any(|m| m.forma == FormaMisura::Bilancio)
    );
    p.chiudi_turno();
    let e = p
        .vista()
        .esiti
        .into_iter()
        .find(|e| e.forma == FormaMisura::Bilancio)
        .unwrap();
    assert!(e.approvata);
    assert!(!p.vista().bilancio.esercizio_provvisorio);
}

/// Franchi tiratori al 90%: la maggioranza vota quasi tutta no, l'opposizione quasi tutta sì.
fn franchi_al_90() -> FileDati {
    imposta(
        imposta(senza_shock(), "manopole", "[ft_base]\nvalore = ", "0.9"),
        "manopole",
        "[ft_max]\nvalore = ",
        "1.0",
    )
}

#[test]
fn bilancio_respinto_porta_l_esercizio_provvisorio() {
    let mut p = al_turno(franchi_al_90(), 4);
    presenta_bozza(&mut p);
    p.chiudi_turno();
    assert!(p.vista().bilancio.esercizio_provvisorio);
}

#[test]
fn fiducia_sul_bilancio_senza_franchi_tiratori() {
    // Con `ft_base` 0,4 i franchi tiratori dell'opposizione compensano quelli della maggioranza
    // e il bilancio passerebbe anche senza questione di fiducia: serve il 90%.
    let mut p = al_turno(franchi_al_90(), 4);
    presenta_bozza(&mut p);
    let id = p
        .vista()
        .calendario
        .iter()
        .find(|m| m.forma == FormaMisura::Bilancio)
        .unwrap()
        .id;
    p.comanda(Comando::PoniFiducia { misura: id }).unwrap();
    p.chiudi_turno();
    assert!(!p.vista().bilancio.esercizio_provvisorio);
}

#[test]
fn leggi_in_scadenza_slittano_nel_turno_del_bilancio() {
    let mut p = al_turno(senza_shock(), 3);
    presenta(&mut p, &[("eta_pensionabile", "68")]).unwrap();
    p.chiudi_turno();
    presenta_bozza(&mut p);
    let id = p
        .vista()
        .calendario
        .iter()
        .find(|m| m.forma == FormaMisura::Riforma)
        .unwrap()
        .id;
    let prima = p.salva().unwrap();
    let e = p.comanda(Comando::PoniFiducia { misura: id }).unwrap_err();
    assert!(
        e.to_string().contains("il bilancio occupa il calendario"),
        "{e}"
    );
    assert_eq!(p.salva().unwrap(), prima);
    presenta(&mut p, &[("ticket", "alto")]).unwrap();
    p.chiudi_turno();
    let v = p.vista();
    assert!(v.esiti.iter().all(|e| e.forma != FormaMisura::Riforma));
    let m = v.calendario.iter().find(|m| m.id == id).unwrap();
    assert_eq!(
        m.voto,
        Trimestre {
            anno: 2028,
            trimestre: 1
        }
    );
}

/// L'emendamento aperto slitta con la legge: il default scatta nella chiusura del nuovo voto.
#[test]
fn emendamenti_aperti_slittano_con_la_legge() {
    let mut p = al_turno(senza_shock(), 3);
    presenta(&mut p, &[("eta_pensionabile", "62")]).unwrap();
    p.chiudi_turno();
    presenta_bozza(&mut p);
    let stati = |p: &Partita| -> Vec<StatoEmendamento> {
        let v = p.vista();
        let m = v
            .calendario
            .iter()
            .find(|m| m.forma == FormaMisura::Riforma);
        m.map_or(Vec::new(), |m| {
            m.emendamenti.iter().map(|e| e.stato).collect()
        })
    };
    assert_eq!(stati(&p), [StatoEmendamento::Aperto]);
    p.chiudi_turno();
    assert_eq!(stati(&p), [StatoEmendamento::Aperto]);
    p.chiudi_turno();
    assert!(
        p.vista()
            .esiti
            .iter()
            .any(|e| e.forma == FormaMisura::Riforma)
    );
    let respinti: Vec<u32> = (p.intesa().variazioni.iter())
        .filter(|x| x.causa == CausaIntesa::EmendamentoRespinto)
        .map(|x| x.turno)
        .collect();
    assert_eq!(respinti, [5]);
}

#[test]
fn senza_bilancio_presentato_le_leggi_si_votano() {
    // Turno 4 (ottobre-dicembre 2027) e turno 5 (esercizio provvisorio), nessun bilancio
    // presentato. Al turno 0 il bilancio 2027 è già approvato: niente bozza.
    let mut p = al_turno(senza_shock(), 4);
    assert!(p.bozza_bilancio().is_some() && !p.turno_del_bilancio());
    presenta(&mut p, &[("eta_pensionabile", "68")]).unwrap();
    presenta(&mut p, &[("ticket", "alto")]).unwrap();
    let id = p.vista().calendario[0].id;
    p.comanda(Comando::PoniFiducia { misura: id }).unwrap();
    p.chiudi_turno();
    assert_eq!(p.in_vigore("eta_pensionabile"), Some("68"));
    assert!(p.vista().bilancio.esercizio_provvisorio && !p.turno_del_bilancio());
    p.chiudi_turno();
    assert!(p.vista().calendario.is_empty());
}

#[test]
fn approvato_il_bilancio_le_leggi_slittano_lo_stesso() {
    let mut p = al_turno(senza_shock(), 3);
    presenta(&mut p, &[("eta_pensionabile", "68")]).unwrap();
    p.chiudi_turno();
    let id = p
        .vista()
        .calendario
        .iter()
        .find(|m| m.forma == FormaMisura::Riforma)
        .unwrap()
        .id;
    // Fiducia posta prima di presentare il bilancio: ammessa, e persa con lo slittamento.
    p.comanda(Comando::PoniFiducia { misura: id }).unwrap();
    presenta_bozza(&mut p);
    p.chiudi_turno();
    let v = p.vista();
    assert!(
        v.esiti
            .iter()
            .any(|e| e.forma == FormaMisura::Bilancio && e.approvata)
    );
    assert!(v.esiti.iter().all(|e| e.forma != FormaMisura::Riforma));
    let m = v.calendario.iter().find(|m| m.id == id).unwrap();
    assert_eq!(
        (m.voto, m.fiducia),
        (
            Trimestre {
                anno: 2028,
                trimestre: 1
            },
            false
        )
    );
}

#[test]
fn bilancio_presentato_senza_misura_rifiutato_al_caricamento() {
    let mut p = al_turno(senza_shock(), 4);
    presenta_bozza(&mut p);
    let r = edita(&p, |j| j["parlamento"]["misure"] = serde_json::json!([]));
    assert!(matches!(r, Err(Errore::SalvataggioIlleggibile(_))), "{r:?}");
}

/// La riforma in scadenza al turno 4, con o senza la questione di fiducia, col bilancio presentato
/// dopo; chiuso il turno del bilancio.
fn slittata(fiducia: bool) -> Partita {
    let mut p = al_turno(senza_shock(), 3);
    presenta(&mut p, &[("eta_pensionabile", "68")]).unwrap();
    p.chiudi_turno();
    if fiducia {
        let id = p.vista().calendario[0].id;
        p.comanda(Comando::PoniFiducia { misura: id }).unwrap();
    }
    presenta_bozza(&mut p);
    p.chiudi_turno();
    p
}

#[test]
fn fiducia_persa_nello_slittamento_si_rimborsa() {
    let (con, senza) = (slittata(true), slittata(false));
    for j in con.alleati() {
        let fiducia: Vec<f64> = (con.intesa().variazioni.iter())
            .filter(|v| v.partito == j && v.causa == CausaIntesa::Fiducia && v.turno == 4)
            .map(|v| v.delta)
            .collect();
        assert_eq!(fiducia.len(), 2, "{fiducia:?}");
        assert!(fiducia.iter().sum::<f64>().abs() < 1e-9, "{fiducia:?}");
        let (a, b) = (con.intesa().valori[j], senza.intesa().valori[j]);
        assert!((a - b).abs() < 1e-9, "{a} contro {b}");
    }
}

/// La questione di fiducia persa nello slittamento non cancella gli emendamenti: si riaprono nel
/// turno del voto e, respinti, costano come senza la fiducia.
#[test]
fn fiducia_persa_nello_slittamento_riapre_gli_emendamenti() {
    let gioca = |fiducia: bool| {
        let mut p = al_turno(senza_shock(), 3);
        presenta(&mut p, &[("eta_pensionabile", "62")]).unwrap();
        p.chiudi_turno();
        if fiducia {
            let id = p.vista().calendario[0].id;
            p.comanda(Comando::PoniFiducia { misura: id }).unwrap();
        }
        presenta_bozza(&mut p);
        p.chiudi_turno();
        p.chiudi_turno();
        p
    };
    let (con, senza) = (gioca(true), gioca(false));
    let respinti = |p: &Partita| {
        (p.intesa().variazioni.iter())
            .filter(|x| x.causa == CausaIntesa::EmendamentoRespinto)
            .count()
    };
    assert!(respinti(&senza) > 0);
    assert_eq!(respinti(&con), respinti(&senza));
    let (c, s) = (&con.intesa().valori, &senza.intesa().valori);
    assert!(
        c.iter().zip(s).all(|(x, y)| (x - y).abs() < 1e-9),
        "{c:?} {s:?}"
    );
}

/// L'emendamento già deciso non si chiede di nuovo quando gli emendamenti si riaprono.
#[test]
fn emendamento_deciso_non_si_richiede_dopo_lo_slittamento() {
    let mut p = al_turno(senza_shock(), 3);
    presenta(&mut p, &[("eta_pensionabile", "62")]).unwrap();
    p.chiudi_turno();
    let id = p.vista().calendario[0].id;
    p.comanda(Comando::Emendamento {
        misura: id,
        partito: "centrodestra".into(),
        accogli: false,
    })
    .unwrap();
    p.comanda(Comando::PoniFiducia { misura: id }).unwrap();
    presenta_bozza(&mut p);
    p.chiudi_turno();
    let v = p.vista();
    let m = v.calendario.iter().find(|m| m.id == id).unwrap();
    let stati: Vec<StatoEmendamento> = m.emendamenti.iter().map(|e| e.stato).collect();
    assert_eq!(stati, [StatoEmendamento::Respinto]);
}

#[test]
fn ripresentare_il_bilancio_tiene_id_e_fiducia() {
    let mut p = al_turno(senza_shock(), 4);
    presenta_bozza(&mut p);
    let bilanci = |p: &Partita| -> Vec<(u32, bool)> {
        (p.vista().calendario.iter())
            .filter(|m| m.forma == FormaMisura::Bilancio)
            .map(|m| (m.id, m.fiducia))
            .collect()
    };
    let id = bilanci(&p)[0].0;
    p.comanda(Comando::PoniFiducia { misura: id }).unwrap();
    presenta_bozza(&mut p);
    presenta_bozza(&mut p);
    assert_eq!(bilanci(&p), [(id, true)]);
}

#[test]
fn niente_fiducia_su_una_legge_elettorale_nel_turno_del_bilancio() {
    let mut p = al_turno(senza_shock(), 4);
    presenta(&mut p, &[("soglia_lista", "0")]).unwrap();
    presenta_bozza(&mut p);
    let id = (p.vista().calendario.iter())
        .find(|m| m.forma == FormaMisura::Elettorale)
        .unwrap()
        .id;
    let prima = p.salva().unwrap();
    let e = p.comanda(Comando::PoniFiducia { misura: id }).unwrap_err();
    assert!(
        e.to_string().contains("il bilancio occupa il calendario"),
        "{e}"
    );
    assert_eq!(p.salva().unwrap(), prima);
}

#[test]
fn norma_costituzionale_slitta_nel_turno_del_bilancio() {
    let mut p = al_turno(senza_shock(), 4);
    presenta(&mut p, &[("numero_parlamentari", "630_315")]).unwrap();
    presenta_bozza(&mut p);
    p.chiudi_turno();
    let v = p.vista();
    assert!(
        v.esiti
            .iter()
            .all(|e| e.forma != FormaMisura::Costituzionale)
    );
    let m = (v.calendario.iter())
        .find(|m| m.forma == FormaMisura::Costituzionale)
        .unwrap();
    assert_eq!(
        (m.voto, m.deliberazione),
        (
            Trimestre {
                anno: 2028,
                trimestre: 1
            },
            1
        )
    );
}

fn al_bilancio_del_2028() -> Partita {
    let mut p = al_turno(senza_shock(), 4); // ottobre-dicembre 2027
    presenta_bozza(&mut p);
    p
}

fn bilancio(p: &Partita) -> sim_core::vista::VoceMisura {
    (p.vista().calendario.into_iter())
        .find(|m| m.forma == FormaMisura::Bilancio)
        .unwrap()
}

fn emenda(p: &mut Partita, accogli: bool) {
    let misura = bilancio(p).id;
    p.comanda(Comando::Emendamento {
        misura,
        partito: "centrodestra".into(),
        accogli,
    })
    .unwrap();
}

#[test]
fn gli_alleati_chiedono_di_indicizzare_i_loro_dicasteri() {
    let m = bilancio(&al_bilancio_del_2028());
    let e = (m.emendamenti.iter())
        .find(|e| e.partito == "centrodestra")
        .unwrap();
    assert!(e.testo.contains("indicizzato"), "{}", e.testo);
    assert!(e.motivo.contains("% reale"), "{}", e.motivo);
    assert_eq!(e.stato, StatoEmendamento::Aperto);
}

#[test]
fn accolto_l_emendamento_la_voce_e_indicizzata() {
    let mut p = al_bilancio_del_2028();
    let voce = |p: &Partita| {
        let b = p.economia().bilancio_presentato.as_ref().unwrap();
        b.dicasteri
            .iter()
            .find(|d| d.id == "ambiente_energia")
            .unwrap()
            .spesa
    };
    let prima = voce(&p);
    emenda(&mut p, true);
    assert!(voce(&p) > prima);
}

#[test]
fn respinto_l_emendamento_il_bilancio_non_cambia_e_l_alleato_paga() {
    let mut p = al_bilancio_del_2028();
    let prima = p.economia().bilancio_presentato.clone();
    emenda(&mut p, false);
    assert_eq!(p.economia().bilancio_presentato, prima);
    let pagato = (p.intesa().variazioni.iter())
        .filter(|x| x.causa == CausaIntesa::EmendamentoRespinto)
        .count();
    assert_eq!(pagato, 1);
}

#[test]
fn ripresentato_il_bilancio_l_accolto_resta_e_si_riapplica() {
    let mut p = al_bilancio_del_2028();
    emenda(&mut p, true);
    let indicizzato = p.economia().bilancio_presentato.clone();
    presenta_bozza(&mut p);
    assert_eq!(p.economia().bilancio_presentato, indicizzato);
    let m = bilancio(&p);
    let e = m.emendamenti.iter().filter(|e| e.partito == "centrodestra");
    assert_eq!(
        e.map(|e| e.stato).collect::<Vec<_>>(),
        [StatoEmendamento::Accolto]
    );
}

#[test]
fn un_bilancio_indicizzato_non_riceve_emendamenti() {
    let mut p = al_turno(senza_shock(), 4);
    let b = p.bozza_bilancio().unwrap();
    let b = p.indicizza(&b, &Voce::Tutto).unwrap();
    p.comanda(Comando::PresentaBilancio { bilancio: b })
        .unwrap();
    assert!(bilancio(&p).emendamenti.is_empty());
}

#[test]
fn a_fine_turno_l_emendamento_aperto_sul_bilancio_e_respinto() {
    let mut p = al_bilancio_del_2028();
    assert_eq!(bilancio(&p).emendamenti[0].stato, StatoEmendamento::Aperto);
    p.chiudi_turno();
    let pagato = (p.intesa().variazioni.iter())
        .filter(|x| x.causa == CausaIntesa::EmendamentoRespinto)
        .count();
    assert_eq!(pagato, 1);
}

#[test]
fn respinto_l_emendamento_i_franchi_tiratori_dell_alleato_crescono() {
    let mut p = al_bilancio_del_2028();
    let prima = bilancio(&p).forbice;
    emenda(&mut p, false);
    let m = bilancio(&p);
    assert_eq!(m.emendamenti[0].stato, StatoEmendamento::Respinto);
    // `ft_emendamento_respinto` moltiplica i franchi: la forbice dei sì si abbassa.
    assert!(
        m.forbice.camera[0] < prima.camera[0],
        "{:?} {:?}",
        m.forbice,
        prima
    );
}
