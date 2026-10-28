mod comune;
use comune::{chiudi_col_bilancio, con_seggi, dati_base, edita, partito, presenta};
use sim_core::legislatura::{Fase, Incarico};
use sim_core::{Comando, Partita};

/// La destra e il centrodestra in minoranza: 150 su 400, 75 su 200.
const MINORANZA_C: [u32; 7] = [40, 80, 60, 30, 50, 100, 40];
const MINORANZA_S: [u32; 7] = [20, 40, 30, 15, 25, 50, 20];

#[test]
fn dimettersi_porta_al_ribaltone_o_al_voto() {
    let mut p = Partita::nuova(dati_base(), 1);
    p.comanda(Comando::Dimettiti).unwrap();
    assert!(p.vista().dimissioni);
    p.chiudi_turno();
    let l = p.legislatura();
    match l.fase {
        Fase::Trattativa => {
            let Some(Incarico::Ia { coalizione }) = &l.incarico else {
                panic!("{:?}", l.incarico)
            };
            assert!(!coalizione.contains(&p.giocatore()));
            p.chiudi_turno();
            assert!(!p.governa() && p.legislatura().fase == Fase::Ordinaria);
        }
        Fase::Sciolta => assert!(p.legislatura().turno_di_voto(p.vista().turno)),
        Fase::Ordinaria => panic!("il governo non è caduto"),
    }
}

#[test]
fn fiducia_respinta_fa_dimettere() {
    let mut p = con_seggi(&Partita::nuova(dati_base(), 1), &MINORANZA_C, &MINORANZA_S);
    presenta(&mut p, &[("tetto_contante", "1000")]).unwrap();
    let id = p.vista().calendario[0].id;
    p.comanda(Comando::PoniFiducia { misura: id }).unwrap();
    p.chiudi_turno();
    assert!(p.vista().esiti.iter().any(|e| e.fiducia_respinta));
    assert!(!p.governa());
    assert_ne!(p.legislatura().fase, Fase::Ordinaria);
}

#[test]
fn bilancio_mancato_anche_in_esercizio_provvisorio() {
    let mut p = con_seggi(&Partita::nuova(dati_base(), 1), &MINORANZA_C, &MINORANZA_S);
    for _ in 0..4 {
        p.chiudi_turno(); // fino a ottobre-dicembre 2027
    }
    let b = p.bozza_bilancio().unwrap();
    p.comanda(Comando::PresentaBilancio { bilancio: b })
        .unwrap();
    p.chiudi_turno(); // respinto: esercizio provvisorio nel 2028
    assert!(p.governa());
    let b = p.bozza_bilancio().unwrap();
    p.comanda(Comando::PresentaBilancio { bilancio: b })
        .unwrap();
    p.chiudi_turno(); // respinto di nuovo: dimissioni
    assert!(!p.governa());
}

#[test]
fn dimissioni_nel_turno_del_voto() {
    let mut p = Partita::nuova(dati_base(), 1);
    while p.vista().turno < 19 {
        // Senza il bilancio il governo cadrebbe nell'esercizio provvisorio del 2028.
        if let Some(bilancio) = p.bozza_bilancio() {
            p.comanda(Comando::PresentaBilancio { bilancio }).unwrap();
        }
        p.chiudi_turno();
    }
    p.comanda(Comando::Dimettiti).unwrap();
    p.chiudi_turno(); // si vota
    let l = p.legislatura();
    assert_eq!(l.elezioni.len(), 1);
    assert_eq!(l.fase, Fase::Trattativa);
    assert!(!l.dimissioni);
}

/// Un governo IA (presiede il centrodestra) cade a `scadenza − 1`: il giocatore, primo nella
/// contigua, ha l'incarico col ribaltone. Il turno dopo è quello del voto: vince il voto, la
/// maggioranza del ribaltone si cancella e il salvataggio si ricarica.
#[test]
fn ribaltone_alla_vigilia_del_voto() {
    let mut p = Partita::nuova(dati_base(), 1);
    while p.vista().turno < 18 {
        chiudi_col_bilancio(&mut p);
    }
    let mut p = edita(&p, |j| {
        j["governo"]["presidente"] = partito("centrodestra").into();
        j["legislatura"]["dimissioni"] = true.into();
    })
    .unwrap();
    p.chiudi_turno();
    let l = p.legislatura();
    assert_eq!(l.incarico, Some(Incarico::Giocatore));
    assert!(l.maggioranza.is_some());
    assert!(l.turno_di_voto(p.vista().turno));
    p.chiudi_turno(); // si vota
    let l = p.legislatura();
    assert_eq!(l.elezioni.len(), 1);
    assert_eq!(l.fase, Fase::Trattativa);
    assert_eq!(l.maggioranza, None);
    assert!(l.allargati.is_empty());
    assert!(Partita::carica(&p.salva().unwrap()).is_ok());
}

/// La destra del giocatore tiene metà dei seggi in entrambe le Camere: senza di lei nessuna
/// maggioranza, e il Capo dello Stato scioglie. Col governo IA la contigua con tutti c'è sempre
/// (la fila intera ha tutti i seggi): lo scioglimento si vede solo dopo il governo del giocatore.
#[test]
fn senza_maggioranza_contigua_il_capo_dello_stato_scioglie() {
    let c = [20, 60, 40, 20, 40, 200, 20];
    let s = [10, 30, 20, 10, 20, 100, 10];
    let mut p = con_seggi(&Partita::nuova(dati_base(), 1), &c, &s);
    p.comanda(Comando::Dimettiti).unwrap();
    p.chiudi_turno();
    assert_eq!(p.legislatura().fase, Fase::Sciolta);
    assert_eq!(p.legislatura().incarico, None);
}

/// iter-legislativo-italiano:80: la legge del governo caduto decade senza voto, come a fine
/// legislatura, e non costa al successore.
#[test]
fn alla_caduta_il_calendario_decade() {
    let mut p = Partita::nuova(dati_base(), 1);
    presenta(&mut p, &[("tetto_contante", "1000")]).unwrap();
    let id = p.vista().calendario[0].id;
    p.comanda(Comando::Dimettiti).unwrap();
    p.chiudi_turno();
    assert!(!p.governa());
    assert!(p.vista().calendario.is_empty());
    let e = (p.vista().esiti.into_iter())
        .find(|e| e.misura == id)
        .unwrap();
    assert_eq!(
        e.decaduta.as_deref(),
        Some("Decaduta per la caduta del governo")
    );
    assert!(!e.approvata);
}
