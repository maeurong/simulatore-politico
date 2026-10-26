mod comune;
use comune::{dati_base, file_base, imposta};
use sim_core::dati::DatiGioco;
use sim_core::vista::{Vista, VoceMinistero};
use sim_core::{Comando, Errore, Partita};

fn ministero<'a>(v: &'a Vista, id: &str) -> &'a VoceMinistero {
    v.ministeri.iter().find(|m| m.dicastero == id).unwrap()
}

fn dimissioni(dicastero: &str) -> Comando {
    Comando::ChiediDimissioni {
        dicastero: dicastero.into(),
    }
}

#[test]
fn dieci_ministri_al_turno_zero() {
    let v = Partita::nuova(dati_base(), 1).vista();
    assert_eq!(v.ministeri.len(), 10);
    for m in &v.ministeri {
        let x = m.ministro.as_ref().unwrap();
        assert!((1..=5).contains(&x.capacita));
        assert!((1.0..=7.0).contains(&x.posizione));
        assert_eq!(x.soddisfazione, 60.0);
        assert!(!x.rodaggio && !x.minaccia);
    }
    assert_eq!(
        ministero(&v, "esteri_difesa")
            .ministro
            .as_ref()
            .unwrap()
            .partito,
        "centrodestra"
    );
    assert_eq!(
        ministero(&v, "sanita").ministro.as_ref().unwrap().obiettivo,
        105.0
    );
    assert_eq!(
        ministero(&v, "giustizia")
            .ministro
            .as_ref()
            .unwrap()
            .obiettivo,
        95.0
    );
}

#[test]
fn stesso_seme_stessi_ministri() {
    let a = Partita::nuova(dati_base(), 4).vista().ministeri;
    assert_eq!(a, Partita::nuova(dati_base(), 4).vista().ministeri);
    assert_ne!(a, Partita::nuova(dati_base(), 5).vista().ministeri);
}

#[test]
fn dimissioni_chieste_portano_interim_rosa_e_rodaggio() {
    let mut p = Partita::nuova(dati_base(), 1);
    p.comanda(dimissioni("sanita")).unwrap();
    p.chiudi_turno();
    let v = p.vista();
    let m = ministero(&v, "sanita");
    assert!(m.ministro.is_none());
    assert_eq!(m.rosa.len(), 3);
    assert!(m.rosa.iter().all(|x| x.partito == "destra"));
    assert!(m.rosa.windows(2).all(|w| w[0].capacita >= w[1].capacita));
    let scelto = m.rosa[1].nome.clone();
    p.comanda(Comando::ScegliSuccessore {
        dicastero: "sanita".into(),
        indice: 1,
    })
    .unwrap();
    p.chiudi_turno();
    let x = ministero(&p.vista(), "sanita").ministro.clone().unwrap();
    assert_eq!(x.nome, scelto);
    assert!(x.rodaggio);
    p.chiudi_turno();
    assert!(
        !ministero(&p.vista(), "sanita")
            .ministro
            .as_ref()
            .unwrap()
            .rodaggio
    );
}

#[test]
fn successore_non_scelto_e_il_primo_della_rosa() {
    let mut p = Partita::nuova(dati_base(), 2);
    p.comanda(dimissioni("interno")).unwrap();
    p.chiudi_turno();
    let primo = ministero(&p.vista(), "interno").rosa[0].nome.clone();
    p.chiudi_turno();
    assert_eq!(
        ministero(&p.vista(), "interno")
            .ministro
            .as_ref()
            .unwrap()
            .nome,
        primo
    );
}

#[test]
fn ministri_sotto_soglia_minacciano_e_si_dimettono_a_fine_turno() {
    let f = imposta(
        file_base(),
        "manopole",
        "[soddisfazione_base]\nvalore = ",
        "30.0",
    );
    let f = imposta(f, "manopole", "[soglia_dimissioni]\nvalore = ", "35.0");
    let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    assert!(
        p.vista()
            .ministeri
            .iter()
            .all(|m| m.ministro.as_ref().unwrap().minaccia)
    );
    p.chiudi_turno();
    assert!(
        p.vista()
            .ministeri
            .iter()
            .all(|m| m.ministro.is_none() && m.rosa.len() == 3)
    );
}

#[test]
fn comandi_non_ammessi_rifiutati_col_nome() {
    let mut p = Partita::nuova(dati_base(), 1);
    let prima = p.salva().unwrap();
    for (c, nome) in [
        (dimissioni("sanitá"), "sanitá"),
        (
            Comando::ScegliSuccessore {
                dicastero: "sanita".into(),
                indice: 0,
            },
            "sanita",
        ),
    ] {
        let e = p.comanda(c).unwrap_err();
        assert!(
            matches!(e, Errore::Comando(_)) && e.to_string().contains(nome),
            "{e}"
        );
    }
    p.comanda(dimissioni("sanita")).unwrap();
    p.chiudi_turno();
    let e = p
        .comanda(Comando::ScegliSuccessore {
            dicastero: "sanita".into(),
            indice: 3,
        })
        .unwrap_err();
    assert!(e.to_string().contains("sanita"), "{e}");
    let mut q = Partita::carica(&prima).unwrap();
    assert!(q.comanda(dimissioni("sanitá")).is_err());
    assert_eq!(q.salva().unwrap(), prima);
}

#[test]
fn salvataggio_con_ministri_corti_illeggibile() {
    let p = Partita::nuova(dati_base(), 1);
    let mut json: serde_json::Value = serde_json::from_str(&p.salva().unwrap()).unwrap();
    json["ministri"]["dicasteri"].as_array_mut().unwrap().pop();
    let e = Partita::carica(&json.to_string()).unwrap_err();
    assert!(matches!(e, Errore::SalvataggioIlleggibile(_)), "{e}");
}
