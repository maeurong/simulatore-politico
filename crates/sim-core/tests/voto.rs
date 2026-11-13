mod comune;
use comune::{chiudi_senza_opposizione_ia, file_base, imposta, presenta, senza_shock, voci};
use sim_core::dati::{DatiGioco, FileDati};
use sim_core::esito::{Criticita, TipoVoce};
use sim_core::ministri::Causa;
use sim_core::parlamento::{Conteggio, FormaMisura};
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
    chiudi_senza_opposizione_ia(&mut p);
    assert_eq!(p.in_vigore("eta_pensionabile"), Some("67"));
    assert!(p.vista().esiti.is_empty());
    chiudi_senza_opposizione_ia(&mut p);
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
    let approvate = voci(&p, TipoVoce::LeggeApprovata);
    assert_eq!(approvate.len(), 1, "{approvate:?}");
    let a = &approvate[0];
    assert_eq!(
        (a.misura, a.criticita),
        (Some(e.misura), Criticita::Notevole)
    );
    assert_eq!(a.disposizioni, ["eta_pensionabile"]);
    assert!(
        a.fatto
            .starts_with("Approvata «Età pensionabile: 68 anni»; Camera "),
        "{}",
        a.fatto
    );
    assert!(a.causa.starts_with("per il sì "), "{}", a.causa);
}

/// `c` dice i franchi del ramo: almeno lo scarto dei sì dai seggi con la linea del sì, e della
/// stessa parità (ogni franco sposta un voto da una parte all'altra).
fn franchi_coerenti(c: &Conteggio, seggi_del_si: u32) {
    let scarto = c.si.abs_diff(seggi_del_si);
    assert!(
        c.franchi >= scarto && (c.franchi - scarto).is_multiple_of(2),
        "{c:?} {seggi_del_si}"
    );
}

#[test]
fn i_franchi_tiratori_si_contano() {
    let mut p = partita(senza_shock(), 1);
    presenta(&mut p, &[("eta_pensionabile", "68")]).unwrap();
    chiudi_senza_opposizione_ia(&mut p);
    let v = p.vista();
    let linea = &v.calendario[0].linee;
    let del_si = |ramo: fn(&sim_core::vista::SeggiPartito) -> u32| -> u32 {
        (v.seggi.iter())
            .filter(|s| linea.iter().any(|l| l.si && l.partito == s.partito))
            .map(ramo)
            .sum()
    };
    let (camera, senato) = (del_si(|s| s.camera), del_si(|s| s.senato));
    chiudi_senza_opposizione_ia(&mut p);
    let e = p.vista().esiti[0].clone();
    assert!(e.camera.franchi > 0 && e.senato.franchi > 0, "{e:?}");
    franchi_coerenti(&e.camera, camera);
    franchi_coerenti(&e.senato, senato);
    let a = &voci(&p, TipoVoce::LeggeApprovata)[0];
    let ramo = |nome: &str, c: &Conteggio| {
        format!(
            "{nome} {} sì e {} no, {} franchi tiratori",
            c.si, c.no, c.franchi
        )
    };
    assert!(a.fatto.contains(&ramo("Camera", &e.camera)), "{}", a.fatto);
    assert!(a.fatto.contains(&ramo("Senato", &e.senato)), "{}", a.fatto);
}

#[test]
fn sotto_fiducia_zero_franchi() {
    let f = con(con(senza_shock(), "ft_base", "0.9"), "ft_max", "1.0");
    let mut p = partita(f, 1);
    presenta(&mut p, &[("eta_pensionabile", "68")]).unwrap();
    chiudi_senza_opposizione_ia(&mut p);
    let id = p.vista().calendario[0].id;
    p.comanda(Comando::PoniFiducia { misura: id }).unwrap();
    chiudi_senza_opposizione_ia(&mut p);
    let e = p.vista().esiti[0].clone();
    assert!(e.fiducia && e.approvata, "{e:?}");
    assert_eq!((e.camera.franchi, e.senato.franchi), (0, 0));
    let a = &voci(&p, TipoVoce::LeggeApprovata)[0];
    assert!(a.fatto.contains("» con la fiducia; Camera "), "{}", a.fatto);
    assert!(!a.fatto.contains("franchi"), "{}", a.fatto);
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
            .contains("Età pensionabile: legge respinta, si ripresenta da "),
        "{e}"
    );
    let respinte = voci(&p, TipoVoce::LeggeRespinta);
    assert_eq!(respinte.len(), 1, "{respinte:?}");
    let r = &respinte[0];
    assert_eq!((r.criticita, r.precedenza), (Criticita::Critica, 2));
    assert!(
        r.fatto
            .starts_with("Respinta «Età pensionabile: 68 anni»; Camera ")
            && r.fatto
                .contains("; disposizioni bloccate per 2 turni; valence -0,05 a "),
        "{}",
        r.fatto
    );
    p.chiudi_turno();
    p.chiudi_turno();
    presenta(&mut p, &[("eta_pensionabile", "69")]).unwrap();
}

/// La legge respinta e ripresentata: due voci, due misure, le stesse disposizioni.
#[test]
fn la_legge_ripresentata_tiene_le_disposizioni() {
    let mut p = partita(
        con(con(senza_shock(), "ft_base", "0.9"), "ft_max", "1.0"),
        1,
    );
    let mut viste = Vec::new();
    for _ in 0..2 {
        presenta(&mut p, &[("eta_pensionabile", "68")]).unwrap();
        p.chiudi_turno();
        p.chiudi_turno();
        // Del governo: l'opposizione IA può avere la sua proposta respinta nella stessa chiusura.
        let r: Vec<_> = (voci(&p, TipoVoce::LeggeRespinta).into_iter())
            .filter(|v| v.partito.is_none())
            .collect();
        assert_eq!(r.len(), 1, "{r:?}");
        viste.push(r[0].clone());
        p.chiudi_turno();
        p.chiudi_turno();
    }
    assert_ne!(viste[0].misura, viste[1].misura);
    assert_eq!(viste[0].disposizioni, ["eta_pensionabile"]);
    assert_eq!(viste[0].disposizioni, viste[1].disposizioni);
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
    chiudi_senza_opposizione_ia(&mut p);
    assert_eq!(p.vista().calendario.len(), 1);
    chiudi_senza_opposizione_ia(&mut p);
    assert_eq!(p.vista().esiti.len(), 1);
}

/// Salva e ricarica dopo il voto: le due copie restano identiche per tre chiusure.
fn ricaricata_resta_identica(f: FileDati, approvata: bool) {
    let mut p = partita(f, 1);
    presenta(&mut p, &[("eta_pensionabile", "68")]).unwrap();
    p.chiudi_turno();
    p.chiudi_turno();
    assert_eq!(p.vista().esiti[0].approvata, approvata);
    let mut q = Partita::carica(&p.salva().unwrap()).unwrap();
    for _ in 0..3 {
        p.chiudi_turno();
        q.chiudi_turno();
    }
    assert_eq!(p.salva().unwrap(), q.salva().unwrap());
}

#[test]
fn ricaricare_dopo_un_voto_approvato_non_cambia_il_seguito() {
    ricaricata_resta_identica(senza_shock(), true);
}

#[test]
fn ricaricare_dopo_un_voto_respinto_non_cambia_il_seguito() {
    let f = con(con(senza_shock(), "ft_base", "0.9"), "ft_max", "1.0");
    ricaricata_resta_identica(f, false);
}

#[test]
fn legge_respinta_col_proponente_in_interim_non_pesa_sul_successore() {
    let mut p = partita(
        con(con(senza_shock(), "ft_base", "0.9"), "ft_max", "1.0"),
        1,
    );
    presenta(&mut p, &[("eta_pensionabile", "68")]).unwrap();
    p.comanda(Comando::ChiediDimissioni {
        dicastero: "lavoro_welfare".into(),
    })
    .unwrap();
    p.chiudi_turno();
    let ministro = |p: &Partita| {
        let v = p.vista();
        let lw = v
            .ministeri
            .into_iter()
            .find(|m| m.dicastero == "lavoro_welfare");
        lw.unwrap().ministro
    };
    assert!(ministro(&p).is_none());
    p.chiudi_turno();
    assert!(!p.vista().esiti[0].approvata);
    for _ in 0..3 {
        let successore = ministro(&p).expect("successore insediato");
        assert!(
            successore
                .variazioni
                .iter()
                .all(|x| x.causa != Causa::LeggeRespinta)
        );
        p.chiudi_turno();
    }
}
