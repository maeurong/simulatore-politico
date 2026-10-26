mod comune;
use comune::{file_base, imposta, presenta, senza_shock};
use sim_core::dati::{DatiGioco, FileDati};
use sim_core::ministri::Causa;
use sim_core::parlamento::FormaMisura;
use sim_core::{Comando, Partita};

fn con(f: FileDati, nome: &str, valore: &str) -> FileDati {
    imposta(f, "manopole", &format!("[{nome}]\nvalore = "), valore)
}

fn partita(f: FileDati, seme: u64) -> Partita {
    Partita::nuova(DatiGioco::da_testi(&f).unwrap(), seme)
}

#[test]
fn legge_approvata_alla_fine_del_turno_dopo() {
    let mut p = partita(senza_shock(), 1);
    presenta(&mut p, &[("eta_pensionabile", "68")]).unwrap();
    p.chiudi_turno();
    assert_eq!(p.in_vigore("eta_pensionabile"), Some("67"));
    assert!(p.vista().esiti.is_empty());
    p.chiudi_turno();
    let v = p.vista();
    let e = &v.esiti[0];
    assert_eq!(e.forma, FormaMisura::Riforma);
    assert!(e.approvata && e.camera.si + e.camera.no == 400 && e.senato.si + e.senato.no == 200);
    assert_eq!(p.in_vigore("eta_pensionabile"), Some("68"));
    let d = v
        .disposizioni
        .iter()
        .find(|d| d.id == "eta_pensionabile")
        .unwrap();
    assert_eq!(d.precedente, "67");
    assert!(
        d.legge.starts_with("L. ") && d.legge.ends_with("/2027"),
        "{}",
        d.legge
    );
    assert!(v.calendario.is_empty());
}

#[test]
fn linee_seguono_la_distanza_senza_intesa_ne_malus() {
    let f = con(
        con(file_base(), "malus_opposizione", "0.0"),
        "peso_intesa_linea",
        "0.0",
    );
    let mut p = partita(f, 1);
    presenta(&mut p, &[("eta_pensionabile", "62")]).unwrap();
    let v = p.vista();
    let si: Vec<&str> = v.calendario[0]
        .linee
        .iter()
        .filter(|l| l.si)
        .map(|l| l.partito.as_str())
        .collect();
    assert_eq!(
        si,
        ["estrema_sinistra", "sinistra", "centrosinistra", "destra"]
    );
}

#[test]
fn conteggio_dentro_la_forbice() {
    let mut dentro = 0;
    for seme in 1..=20 {
        let mut p = partita(senza_shock(), seme);
        presenta(&mut p, &[("eta_pensionabile", "68")]).unwrap();
        p.chiudi_turno();
        let [basso, alto] = p.vista().calendario[0].forbice.camera;
        p.chiudi_turno();
        let si = p.vista().esiti[0].camera.si;
        dentro += usize::from((basso..=alto).contains(&si));
    }
    assert!(dentro >= 17, "{dentro}");
}

#[test]
fn stesso_seme_stesso_voto() {
    let voto = |seme| {
        let mut p = partita(senza_shock(), seme);
        presenta(&mut p, &[("eta_pensionabile", "68")]).unwrap();
        p.chiudi_turno();
        p.chiudi_turno();
        p.vista().esiti[0].clone()
    };
    assert_eq!(voto(4), voto(4));
}

#[test]
fn legge_respinta_si_ripresenta_dopo_due_turni() {
    let mut p = partita(
        con(con(senza_shock(), "ft_base", "0.9"), "ft_max", "1.0"),
        1,
    );
    presenta(&mut p, &[("eta_pensionabile", "68")]).unwrap();
    p.chiudi_turno();
    p.chiudi_turno();
    let v = p.vista();
    assert!(!v.esiti[0].approvata);
    let lw = v
        .ministeri
        .iter()
        .find(|m| m.dicastero == "lavoro_welfare")
        .unwrap();
    assert!(
        lw.ministro
            .as_ref()
            .unwrap()
            .variazioni
            .iter()
            .any(|x| x.causa == Causa::LeggeRespinta)
    );
    let e = presenta(&mut p, &[("eta_pensionabile", "69")]).unwrap_err();
    assert!(
        e.to_string()
            .contains("eta_pensionabile respinta: si ripresenta da"),
        "{e}"
    );
    p.chiudi_turno();
    p.chiudi_turno();
    presenta(&mut p, &[("eta_pensionabile", "69")]).unwrap();
}

#[test]
fn votare_sposta_la_posizione_percepita() {
    let mut p = partita(senza_shock(), 1);
    presenta(&mut p, &[("eta_pensionabile", "62")]).unwrap();
    p.chiudi_turno();
    p.chiudi_turno();
    let percepita = |id: &str| {
        p.partiti()
            .iter()
            .find(|x| x.id == id)
            .unwrap()
            .posizione_percepita
    };
    assert!(percepita("estrema_sinistra") > 1.0 && percepita("destra") < 6.0);
    assert_eq!(
        (percepita("centrosinistra"), percepita("centro")),
        (3.0, 4.0)
    );
}

#[test]
fn legge_approvata_cancella_il_valore_in_arrivo() {
    let mut p = partita(senza_shock(), 1);
    presenta(&mut p, &[("nucleare", "consentito")]).unwrap();
    p.chiudi_turno();
    p.chiudi_turno();
    assert!(p.vista().esiti[0].approvata);
    let v = p.vista();
    let n = v.disposizioni.iter().find(|d| d.id == "nucleare").unwrap();
    assert!(n.in_arrivo.is_empty() && n.in_vigore == "consentito");
}

#[test]
fn dimissioni_del_proponente_non_fanno_cadere_la_legge() {
    let mut p = partita(senza_shock(), 1);
    presenta(&mut p, &[("eta_pensionabile", "68")]).unwrap();
    p.comanda(Comando::ChiediDimissioni {
        dicastero: "lavoro_welfare".into(),
    })
    .unwrap();
    p.chiudi_turno();
    assert_eq!(p.vista().calendario.len(), 1);
    p.chiudi_turno();
    assert_eq!(p.vista().esiti.len(), 1);
}
