mod comune;
use comune::{imposta, presenta, senza_shock};
use sim_core::dati::{DatiGioco, FileDati};
use sim_core::parlamento::{CausaIntesa, StatoEmendamento};
use sim_core::{Comando, Errore, Partita};

fn con(f: FileDati, nome: &str, valore: &str) -> FileDati {
    imposta(f, "manopole", &format!("[{nome}]\nvalore = "), valore)
}

fn al_voto(gradino: (&str, &str)) -> Partita {
    let mut p = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
    presenta(&mut p, &[gradino]).unwrap();
    p.chiudi_turno();
    p
}

fn accogli(p: &mut Partita, accogli: bool) -> Result<(), Errore> {
    let id = p.vista().calendario[0].id;
    p.comanda(Comando::Emendamento {
        misura: id,
        partito: "centrodestra".into(),
        accogli,
    })
}

#[test]
fn alleato_lontano_chiede_un_emendamento_a_meta_strada() {
    let p = al_voto(("eta_pensionabile", "62"));
    let v = p.vista();
    let e = &v.calendario[0].emendamenti;
    assert_eq!(e.len(), 1);
    assert_eq!(
        (e[0].partito.as_str(), e[0].testo.as_str(), e[0].stato),
        (
            "centrodestra",
            "Età pensionabile: 65 anni",
            StatoEmendamento::Aperto
        )
    );
    assert!(e[0].motivo.starts_with("distante"));
}

#[test]
fn emendamento_accolto_attenua_e_compatta() {
    let mut p = al_voto(("eta_pensionabile", "62"));
    accogli(&mut p, true).unwrap();
    let v = p.vista();
    assert!(
        v.calendario[0]
            .linee
            .iter()
            .any(|l| l.partito == "centrodestra" && l.si)
    );
    p.chiudi_turno();
    assert!(p.vista().esiti[0].approvata);
    assert_eq!(p.in_vigore("eta_pensionabile"), Some("65"));
}

#[test]
fn emendamento_ignorato_vale_respinto_e_costa_intesa() {
    let mut p = al_voto(("eta_pensionabile", "62"));
    p.chiudi_turno();
    let a = &p.vista().alleati[0];
    assert!(
        a.variazioni
            .iter()
            .any(|x| x.causa == CausaIntesa::EmendamentoRespinto)
    );
}

#[test]
fn regime_emendato_arriva_dopo_quattro_turni() {
    let mut p = al_voto(("licenziamento", "reintegro"));
    assert!(
        p.vista().calendario[0].emendamenti[0]
            .testo
            .ends_with("in vigore fra 4 turni")
    );
    accogli(&mut p, true).unwrap();
    p.chiudi_turno();
    assert!(p.vista().esiti[0].approvata);
    assert_eq!(p.in_vigore("licenziamento"), Some("tutele_crescenti"));
    for _ in 0..4 {
        p.chiudi_turno();
    }
    assert_eq!(p.in_vigore("licenziamento"), Some("reintegro"));
}

#[test]
fn fiducia_vota_nel_turno_senza_franchi_tiratori() {
    let mut p = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
    presenta(&mut p, &[("eta_pensionabile", "68")]).unwrap();
    let id = p.vista().calendario[0].id;
    p.comanda(Comando::PoniFiducia { misura: id }).unwrap();
    let maggioranza: u32 = p
        .vista()
        .seggi
        .iter()
        .filter(|s| s.partito == "destra" || s.partito == "centrodestra")
        .map(|s| s.camera)
        .sum();
    p.chiudi_turno();
    let v = p.vista();
    assert!(v.esiti[0].fiducia && v.esiti[0].approvata);
    assert_eq!(v.esiti[0].camera.si, maggioranza);
    assert!(
        v.alleati[0]
            .variazioni
            .iter()
            .any(|x| x.causa == CausaIntesa::Fiducia)
    );
}

#[test]
fn alleato_in_ultimatum_e_lontano_vota_no_alla_fiducia() {
    let mut p = Partita::nuova(
        DatiGioco::da_testi(&con(senza_shock(), "intesa_base", "30.0")).unwrap(),
        1,
    );
    presenta(&mut p, &[("eta_pensionabile", "62")]).unwrap();
    let v = p.vista();
    assert_eq!(v.calendario[0].alleati_contro_fiducia, ["centrodestra"]);
    p.comanda(Comando::PoniFiducia {
        misura: v.calendario[0].id,
    })
    .unwrap();
    p.chiudi_turno();
    let e = &p.vista().esiti[0];
    assert!(e.linee.iter().any(|l| l.partito == "centrodestra" && !l.si));
    // Voto a blocchi senza franchi tiratori: votano sì solo i seggi della destra.
    let destra = v.seggi.iter().find(|s| s.partito == "destra").unwrap();
    assert_eq!((e.camera.si, e.senato.si), (destra.camera, destra.senato));
    assert_eq!(
        e.approvata,
        destra.camera * 2 > 400 && destra.senato * 2 > 200
    );
    assert_eq!(e.fiducia_respinta, !e.approvata);
}

#[test]
fn niente_fiducia_su_una_norma_costituzionale() {
    let mut p = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
    presenta(&mut p, &[("numero_parlamentari", "630_315")]).unwrap();
    let id = p.vista().calendario[0].id;
    let e = p.comanda(Comando::PoniFiducia { misura: id }).unwrap_err();
    assert!(
        e.to_string()
            .contains("niente fiducia su una norma costituzionale"),
        "{e}"
    );
}

#[test]
fn salvare_a_meta_turno_conserva_l_aula() {
    let mut p = al_voto(("eta_pensionabile", "62"));
    accogli(&mut p, true).unwrap();
    presenta(&mut p, &[("ticket", "alto")]).unwrap();
    let id = p.vista().calendario[1].id;
    p.comanda(Comando::PoniFiducia { misura: id }).unwrap();
    let mut q = Partita::carica(&p.salva().unwrap()).unwrap();
    p.chiudi_turno();
    q.chiudi_turno();
    assert_eq!(p.salva().unwrap(), q.salva().unwrap());
}

#[test]
fn comandi_dell_aula_non_ammessi_rifiutati_col_nome() {
    let mut p = al_voto(("eta_pensionabile", "62"));
    let id = p.vista().calendario[0].id;
    let prima = p.salva().unwrap();
    for (c, nome) in [
        (
            Comando::Emendamento {
                misura: 99,
                partito: "centrodestra".into(),
                accogli: true,
            },
            "misura sconosciuta: 99",
        ),
        (
            Comando::Emendamento {
                misura: id,
                partito: "sinistra".into(),
                accogli: true,
            },
            "nessun emendamento di sinistra",
        ),
        (
            Comando::PoniFiducia { misura: 99 },
            "misura sconosciuta: 99",
        ),
    ] {
        let e = p.comanda(c).unwrap_err();
        assert!(
            matches!(e, Errore::Comando(_)) && e.to_string().contains(nome),
            "{e}"
        );
    }
    assert_eq!(p.salva().unwrap(), prima);
    accogli(&mut p, true).unwrap();
    assert!(
        accogli(&mut p, false)
            .unwrap_err()
            .to_string()
            .contains("emendamento già deciso: centrodestra")
    );
}
