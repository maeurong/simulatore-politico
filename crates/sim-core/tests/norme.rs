mod comune;
use comune::{imposta, presenta, senza_shock, sostituisci};
use sim_core::dati::{DatiGioco, FileDati};
use sim_core::parlamento::{CausaIntesa, TipoReferendum};
use sim_core::vista::SeggiPartito;
use sim_core::{Comando, Partita};

fn con(f: FileDati, nome: &str, valore: &str) -> FileDati {
    imposta(f, "manopole", &format!("[{nome}]\nvalore = "), valore)
}

fn partita(f: FileDati) -> Partita {
    Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1)
}

#[test]
fn legge_elettorale_linea_sui_seggi_attesi() {
    let mut p = partita(con(
        con(senza_shock(), "malus_opposizione", "0.0"),
        "peso_intesa_linea",
        "0.0",
    ));
    presenta(&mut p, &[("soglia_lista", "0")]).unwrap();
    let v = p.vista();
    let m = &v.calendario[0];
    for l in m.linee.iter().filter(|l| l.partito != "destra") {
        let s = m.seggi.iter().find(|s| s.partito == l.partito).unwrap();
        assert_eq!(l.si, s.camera + s.senato > 0, "{}", l.partito);
    }
}

#[test]
fn legge_elettorale_non_valida_rifiutata_col_motivo() {
    let mut p = partita(senza_shock());
    let e = presenta(&mut p, &[("coalizioni", "no")]).unwrap_err();
    assert!(
        e.to_string()
            .contains("legge elettorale non valida: premio alla coalizione senza coalizioni"),
        "{e}"
    );
}

#[test]
fn alleato_che_perde_seggi_paga_intesa_anche_se_vota_si() {
    let mut p = partita(senza_shock());
    // Sul sondaggio del turno zero il premio va al centrosinistra: senza premio il centrodestra
    // guadagna; senza soglia di lista perde un seggio alla Camera.
    presenta(&mut p, &[("soglia_lista", "0")]).unwrap();
    let v = p.vista();
    let perde = v.calendario[0]
        .seggi
        .iter()
        .find(|s| s.partito == "centrodestra")
        .unwrap();
    assert!(perde.camera + perde.senato < 0);
    p.comanda(Comando::PoniFiducia {
        misura: v.calendario[0].id,
    })
    .unwrap();
    p.chiudi_turno();
    assert!(p.vista().esiti[0].approvata);
    assert!(
        p.vista().alleati[0]
            .variazioni
            .iter()
            .any(|x| x.causa == CausaIntesa::SeggiPersi)
    );
}

#[test]
fn voto_finale_segreto_solo_alla_camera() {
    let mut p = partita(con(senza_shock(), "ft_base", "0.2"));
    presenta(&mut p, &[("soglia_lista", "400")]).unwrap();
    let v = p.vista();
    p.comanda(Comando::PoniFiducia {
        misura: v.calendario[0].id,
    })
    .unwrap();
    let maggioranza = |f: fn(&SeggiPartito) -> u32| {
        v.seggi
            .iter()
            .filter(|s| s.partito == "destra" || s.partito == "centrodestra")
            .map(f)
            .sum::<u32>()
    };
    p.chiudi_turno();
    let e = &p.vista().esiti[0];
    assert_eq!(e.senato.si, maggioranza(|s| s.senato));
    assert_ne!(e.camera.si, maggioranza(|s| s.camera));
}

#[test]
fn bocciatura_della_legge_elettorale_non_e_una_fiducia_respinta() {
    let mut p = partita(con(con(senza_shock(), "ft_base", "0.45"), "ft_max", "0.5"));
    presenta(&mut p, &[("soglia_lista", "400")]).unwrap();
    let id = p.vista().calendario[0].id;
    p.comanda(Comando::PoniFiducia { misura: id }).unwrap();
    p.chiudi_turno();
    let e = &p.vista().esiti[0];
    assert!(!e.approvata);
    assert!(!e.fiducia_respinta);
}

#[test]
fn norma_costituzionale_in_due_deliberazioni() {
    let mut p = partita(senza_shock());
    presenta(&mut p, &[("numero_parlamentari", "300_150")]).unwrap();
    p.chiudi_turno();
    let v = p.vista();
    assert_eq!(v.esiti[0].deliberazione, 1);
    assert!(v.esiti[0].approvata);
    assert_eq!(v.calendario[0].deliberazione, 2);
    p.chiudi_turno();
    assert_eq!(p.vista().esiti[0].deliberazione, 2);
}

/// Il vigente all'estremo destro e il nuovo al centro: tutti tranne l'estrema destra preferiscono il nuovo.
fn costituzionale_condivisa() -> FileDati {
    let f = sostituisci(
        senza_shock(),
        "disposizioni",
        "nome = \"400 deputati e 200 senatori\", posizione = 4.5",
        "nome = \"400 deputati e 200 senatori\", posizione = 7.0",
    );
    let f = sostituisci(
        f,
        "disposizioni",
        "nome = \"630 deputati e 315 senatori\", posizione = 3.0",
        "nome = \"630 deputati e 315 senatori\", posizione = 4.0",
    );
    con(
        con(con(f, "malus_opposizione", "0.0"), "ft_base", "0.0"),
        "ft_distanza",
        "0.0",
    )
}

#[test]
fn con_i_due_terzi_niente_referendum() {
    let mut p = partita(costituzionale_condivisa());
    presenta(&mut p, &[("numero_parlamentari", "630_315")]).unwrap();
    p.chiudi_turno();
    p.chiudi_turno();
    assert_eq!(p.in_vigore("numero_parlamentari"), Some("630_315"));
    assert!(p.vista().referendum.is_empty());
    let e = p.proiezione_seggi().unwrap();
    assert_eq!(e.camera.iter().map(|s| s.totale()).sum::<u32>(), 630);
    assert_eq!(e.senato.iter().map(|s| s.totale()).sum::<u32>(), 315);
}

#[test]
fn senza_i_due_terzi_si_vota_il_referendum_confermativo() {
    let f = con(
        con(senza_shock(), "ft_base", "0.0"),
        "malus_opposizione",
        "5.0",
    );
    let mut p = partita(f);
    presenta(&mut p, &[("numero_parlamentari", "300_150")]).unwrap();
    p.chiudi_turno();
    p.chiudi_turno();
    let v = p.vista();
    assert!(v.esiti[0].approvata);
    assert_eq!(v.referendum[0].tipo, TipoReferendum::Confermativo);
    assert_eq!(p.in_vigore("numero_parlamentari"), Some("400_200"));
    p.chiudi_turno();
    let r = &p.vista().esiti_referendum[0];
    assert_eq!(
        p.in_vigore("numero_parlamentari") == Some("300_150"),
        r.si > r.no
    );
}

/// Ognuna valida da sola: approvata la prima, la seconda darebbe il premio alla lista con le
/// coalizioni attive.
#[test]
fn elettorale_che_diventerebbe_non_valida_cade_senza_voto() {
    let f = sostituisci(
        senza_shock(),
        "scenario",
        "legge_elettorale = \"legge_180\"",
        "legge_elettorale = \"proporzionale_3\"",
    );
    let mut p = partita(f);
    presenta(&mut p, &[("coalizioni", "si")]).unwrap();
    presenta(&mut p, &[("premio", "lista")]).unwrap();
    let id = p.vista().calendario[0].id;
    p.comanda(Comando::PoniFiducia { misura: id }).unwrap();
    p.chiudi_turno();
    assert_eq!(p.in_vigore("coalizioni"), Some("si"));
    p.chiudi_turno();
    assert_eq!(p.in_vigore("premio"), Some("nessuno"));
    assert!(p.vista().calendario.is_empty());
    assert!(p.salva().is_ok());
}
