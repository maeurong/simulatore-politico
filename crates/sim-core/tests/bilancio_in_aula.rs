mod comune;
use comune::{edita, imposta, presenta, senza_shock};
use sim_core::dati::{DatiGioco, FileDati};
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
