mod comune;
use comune::{dati_base, file_base, presenta, sostituisci};
use sim_core::ministri::Causa;
use sim_core::parlamento::FormaMisura;
use sim_core::vista::SeggiPartito;
use sim_core::{DatiGioco, Errore, Partita, Trimestre};

#[test]
fn parlamento_uscente_con_la_maggioranza() {
    let v = Partita::nuova(dati_base(), 1).vista();
    let somma = |f: fn(&SeggiPartito) -> u32| v.seggi.iter().map(f).sum::<u32>();
    assert_eq!((somma(|s| s.camera), somma(|s| s.senato)), (400, 200));
    let governo = |f: fn(&SeggiPartito) -> u32| {
        v.seggi
            .iter()
            .filter(|s| s.partito == "destra" || s.partito == "centrodestra")
            .map(f)
            .sum::<u32>()
    };
    assert!(governo(|s| s.camera) > 200 && governo(|s| s.senato) > 100);
}

#[test]
fn parlamento_uscente_incoerente_rifiutato_col_nome() {
    for (da, a, nome) in [
        (
            "estrema_sinistra = 9,",
            "ignoto = 9,",
            "camera: partito sconosciuto: ignoto",
        ),
        (
            "centro = 7,",
            "centro = -7,",
            "senato: seggi negativi a centro",
        ),
        ("destra = 189,", "destra = 190,", "camera: 401 seggi"),
    ] {
        let f = sostituisci(file_base(), "scenario", da, a);
        let e = DatiGioco::da_testi(&f).unwrap_err();
        assert!(
            matches!(e, Errore::Partiti(_)) && e.to_string().contains(nome),
            "{e}"
        );
    }
}

#[test]
fn legge_presentata_occupa_il_calendario_fino_al_voto() {
    let mut p = Partita::nuova(dati_base(), 1);
    presenta(&mut p, &[("eta_pensionabile", "68")]).unwrap();
    let v = p.vista();
    let m = &v.calendario[0];
    assert_eq!(
        (m.forma, m.voto, m.proponente.as_deref()),
        (
            FormaMisura::Riforma,
            Trimestre {
                anno: 2027,
                trimestre: 1
            },
            Some("lavoro_welfare")
        )
    );
    assert_eq!(m.nome, "Età pensionabile: 68 anni");
    presenta(&mut p, &[("ticket", "alto")]).unwrap();
    presenta(&mut p, &[("obbligo_scolastico", "18")]).unwrap();
    let e = presenta(&mut p, &[("leva_militare", "obbligatoria")]).unwrap_err();
    assert!(e.to_string().contains("calendario pieno"), "{e}");
    presenta(&mut p, &[("numero_parlamentari", "630_315")]).unwrap();
    assert_eq!(p.vista().calendario[3].forma, FormaMisura::Costituzionale);
}

#[test]
fn leggi_non_ammesse_rifiutate_col_nome() {
    let mut p = Partita::nuova(dati_base(), 1);
    let prima = p.salva().unwrap();
    for (valori, nome) in [
        (
            vec![("eta_pensionabile", "67")],
            "eta_pensionabile è già a 67 anni",
        ),
        (
            vec![("eta_pensionabile", "71")],
            "gradino sconosciuto: eta_pensionabile 71",
        ),
        (
            vec![("eta_pension", "68")],
            "disposizione sconosciuta: eta_pension",
        ),
        (
            vec![("eta_pensionabile", "68"), ("ticket", "alto")],
            "disposizioni di dicasteri diversi: ticket",
        ),
        (
            vec![("eta_pensionabile", "68"), ("eta_pensionabile", "69")],
            "disposizione ripetuta: eta_pensionabile",
        ),
        (
            vec![("eta_pensionabile", "68"), ("soglia_lista", "0")],
            "norme di forma diversa",
        ),
        (vec![], "legge vuota"),
    ] {
        let e = presenta(&mut p, &valori).unwrap_err();
        assert!(
            matches!(e, Errore::Comando(_)) && e.to_string().contains(nome),
            "{e}"
        );
    }
    assert_eq!(p.salva().unwrap(), prima);
    presenta(&mut p, &[("eta_pensionabile", "68")]).unwrap();
    let e = presenta(&mut p, &[("eta_pensionabile", "69")]).unwrap_err();
    assert!(
        e.to_string().contains("eta_pensionabile è già in esame"),
        "{e}"
    );
}

#[test]
fn firma_contro_la_posizione_costa_soddisfazione() {
    let s = |gradino: &str| {
        let mut p = Partita::nuova(dati_base(), 2);
        presenta(&mut p, &[("eta_pensionabile", gradino)]).unwrap();
        let v = p.vista();
        let m = v
            .ministeri
            .iter()
            .find(|m| m.dicastero == "lavoro_welfare")
            .unwrap()
            .ministro
            .clone()
            .unwrap();
        (
            m.soddisfazione,
            m.variazioni
                .iter()
                .any(|x| x.causa == Causa::NormaControPosizione),
        )
    };
    let (lontana, causa) = s("62");
    assert!(causa && lontana < s("68").0);
}

#[test]
fn alleati_con_l_intesa_nella_vista() {
    let v = Partita::nuova(dati_base(), 1).vista();
    assert_eq!(v.alleati.len(), 1);
    let a = &v.alleati[0];
    assert_eq!(
        (a.partito.as_str(), a.intesa, a.ultimatum),
        ("centrodestra", 60.0, false)
    );
}
