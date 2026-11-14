mod comune;

use comune::{al_governo, all_opposizione, chiudi_col_bilancio, dati_base, edita, fondata, voci};
use sim_core::dati::DatiGioco;
use sim_core::esito::{Criticita, MASSIMO_IN_EVIDENZA, TipoVoce};
use sim_core::legislatura::Fondazione;
use sim_core::{Errore, Partita};

#[test]
fn al_turno_zero_l_esito_e_vuoto() {
    assert!(Partita::nuova(dati_base(), 7).vista().esito.is_empty());
    assert!(fondata(4.5).vista().esito.is_empty());
}

#[test]
fn nessuna_causa_vuota_mai_piu_di_sette() {
    for mut p in [al_governo(), all_opposizione()] {
        for turno in 0..8 {
            chiudi_col_bilancio(&mut p);
            let esito = p.vista().esito;
            assert!(
                esito.iter().all(|v| !v.causa.is_empty()),
                "turno {turno}: {esito:?}"
            );
            assert!(
                (esito.iter()).all(|v| !(v.in_evidenza && v.criticita == Criticita::Ordinaria)),
                "turno {turno}: {esito:?}"
            );
            let in_evidenza = esito.iter().filter(|v| v.in_evidenza).count();
            assert!(
                in_evidenza <= MASSIMO_IN_EVIDENZA,
                "turno {turno}: {esito:?}"
            );
        }
    }
}

/// All'opposizione il catalogo dice «–»: soddisfazione dei ministri e intesa degli alleati del
/// governo avversario non sono voci.
#[test]
fn all_opposizione_niente_soddisfazione_ne_intesa() {
    let (mut p, mut q) = (all_opposizione(), al_governo());
    let mut al_governo_ne_ha = false;
    for turno in 0..6 {
        chiudi_col_bilancio(&mut p);
        chiudi_col_bilancio(&mut q);
        assert!(!p.governo().al_governo(p.giocatore()));
        let sono_vietate = |v: &sim_core::esito::VoceEsito| {
            matches!(
                v.tipo,
                TipoVoce::SoddisfazioneVariata | TipoVoce::IntesaVariata
            )
        };
        let vietate: Vec<_> = (p.vista().esito.into_iter()).filter(sono_vietate).collect();
        assert!(vietate.is_empty(), "turno {turno}: {vietate:?}");
        al_governo_ne_ha |= q.vista().esito.iter().any(sono_vietate);
    }
    // Controllo positivo: negli stessi turni, al governo, quelle voci nascono.
    assert!(al_governo_ne_ha, "l'assenza all'opposizione sarebbe vacua");
}

#[test]
fn stesso_seme_stesso_esito() {
    let (mut a, mut b) = (all_opposizione(), all_opposizione());
    let mut voci = 0;
    for _ in 0..6 {
        chiudi_col_bilancio(&mut a);
        chiudi_col_bilancio(&mut b);
        assert_eq!(a.vista().esito, b.vista().esito);
        voci += a.vista().esito.len();
    }
    assert!(
        voci > 0,
        "sei chiusure senza una voce: il confronto non prova nulla"
    );
}

#[test]
fn l_esito_sopravvive_al_salvataggio() {
    let mut p = all_opposizione();
    // Una chiusura con voci, per salvare un esito non vuoto.
    for _ in 0..8 {
        if !p.vista().esito.is_empty() {
            break;
        }
        chiudi_col_bilancio(&mut p);
    }
    assert!(!p.vista().esito.is_empty());
    let mut q = Partita::carica(&p.salva().unwrap()).unwrap();
    assert_eq!(q.vista().esito, p.vista().esito);
    chiudi_col_bilancio(&mut p);
    chiudi_col_bilancio(&mut q);
    assert_eq!(q.vista().esito, p.vista().esito);
}

#[test]
fn l_esito_manomesso_e_rifiutato() {
    let p = all_opposizione();
    assert!(!p.vista().esito.is_empty());
    let voce = serde_json::to_value(&p.vista().esito[0]).unwrap();
    let errore =
        |esito: serde_json::Value| edita(&p, |j| j["esito"] = esito).unwrap_err().to_string();
    let mut senza_causa = voce.clone();
    senza_causa["causa"] = "".into();
    assert!(errore(serde_json::json!([senza_causa])).contains("senza causa"));
    let mut in_evidenza = voce.clone();
    in_evidenza["in_evidenza"] = true.into();
    let otto = vec![in_evidenza; MASSIMO_IN_EVIDENZA + 1];
    assert!(errore(serde_json::json!(otto)).contains("in evidenza"));
    let mut sconosciuto = voce;
    sconosciuto["partito"] = "fantasma".into();
    assert!(errore(serde_json::json!([sconosciuto])).contains("fantasma"));
}

/// Fatto 81: lo sciolto resta in `partiti` con `sciolto = true`, quindi la voce che lo nomina passa
/// il controllo di `carica` e il suo id si ritrova.
#[test]
fn partito_sciolto_e_salvataggio_ricaricabile() {
    let f = comune::sostituisci_valence(comune::file_base(), "estrema_sinistra", -3.0);
    let fondazione = Fondazione {
        nome: "Partito nuovo".into(),
        posizione: 4.5,
    };
    let p = Partita::fonda(DatiGioco::da_testi(&f).unwrap(), 7, &fondazione).unwrap();
    let mut p = edita(&p, |j| j["partiti"][0]["senza_seggi"] = 1.into()).unwrap();
    p.chiudi_turno();
    let v = voci(&p, TipoVoce::PartitoSciolto);
    assert_eq!(v.len(), 1, "{:?}", p.vista().esito);
    assert_eq!(v[0].partito.as_deref(), Some("estrema_sinistra"));
    let q = Partita::carica(&p.salva().unwrap()).unwrap();
    assert_eq!(q.vista().esito, p.vista().esito);
    assert!(q.partiti()[0].id == "estrema_sinistra" && q.partiti()[0].sciolto);
}

#[test]
fn schema_17_rifiutato() {
    let p = Partita::nuova(dati_base(), 7);
    let mut json: serde_json::Value = serde_json::from_str(&p.salva().unwrap()).unwrap();
    json["versione_schema"] = 17.into();
    assert_eq!(
        Partita::carica(&json.to_string()),
        Err(Errore::SchemaDiverso {
            trovata: 17,
            attesa: 18
        })
    );
}

/// Non compila se `TipoVoce` guadagna una variante: allora serve una riga in `ELENCO`.
fn nell_elenco(t: TipoVoce) {
    use TipoVoce::*;
    match t {
        EmergenzaLieve
        | EventoRisolto
        | MesAvviato
        | Ristrutturazione
        | MargineEnergia
        | PropostaRespinta
        | PropostaDecaduta
        | EmendamentoRespinto
        | AttoEseguito
        | AttoSaltato
        | CantiereAvviato
        | CantiereFermo
        | CantiereFinito
        | LeggiSlittate
        | LeggeApprovata
        | LeggeRespinta
        | DecretoConvertito
        | DecretoDecaduto
        | BilancioApprovato
        | BilancioRicondotto
        | BilancioRespinto
        | FiduciaRespinta
        | SfiduciaApprovata
        | SfiduciaRespinta
        | SfiduciaScartata
        | ValoreRinviato
        | ValoreNonApplicato
        | MisuraDecaduta
        | ConfermativoInAttesa
        | ConfermativoSenzaVoto
        | ReferendumVotato
        | ReferendumDecaduto
        | ReferendumChiesto
        | FirmeRaccolte
        | Scissione
        | MinacciaDiScissione
        | MinacciaRientrata
        | PromessaVerificata
        | PromesseScadute
        | PromessaFatta
        | SpesaDiCampagna
        | CampagnaTagliata
        | MesFinito
        | Spread
        | ControlloUe
        | ProceduraUe
        | AmmendaUe
        | RistrutturazioneFinita
        | RedditoDeluso
        | Sondaggio
        | PrimaStima
        | Revisione
        | SituazioneAccesa
        | SituazioneSpenta
        | CrisiDelDebito
        | EntrateDiCassa
        | RisultatoDelVoto
        | Incarico
        | FineDellaPartita
        | PartitoSciolto
        | SoddisfazioneVariata
        | MinacciaDiDimissioni
        | MinistroEsce
        | IntesaVariata
        | UltimatumPosto
        | UltimatumRitirato
        | CambioDiLeader
        | FrangiaEsce
        | AlleatoEsce
        | Rimpasto
        | CrisiAperta
        | CoalizioneAllargata
        | GovernoCaduto
        | CamereSciolte
        | GuidaRifiutata
        | GovernoFormato
        | ProgrammaDichiarato
        | RotturaDellImpegno
        | EventoNato
        | ShockAssorbito
        | ValoreInVigore
        | ArrivoScartato
        | PosizioneSpostata
        | PartitoDiProtesta
        | CoalizioneIa
        | EmendamentoChiesto
        | MisuraPresentata
        | PropostaAccolta
        | EmendamentoDeciso
        | FiduciaPosta
        | SfiduciaPresentata
        | GovernoDimissionario => {}
    }
}

/// Per ogni tipo, nell'ordine dell'enum, il test che ne fa nascere la voce (`file::test`;
/// `partita` e `partita/ia` sono i `mod tests` del core). Elenco tenuto a mano: serve alla
/// review, non prova da solo che il test nominato asserisca quel tipo.
const ELENCO: &[(TipoVoce, &str)] = &[
    (
        TipoVoce::EmergenzaLieve,
        "emergenze::una_lieve_non_apre_evento_la_paga_il_fondo_e_non_va_sotto_zero",
    ),
    (
        TipoVoce::EventoRisolto,
        "emergenze::il_fondo_paga_e_non_va_sotto_zero",
    ),
    (TipoVoce::MesAvviato, "debito::il_mes_e_il_default"),
    (
        TipoVoce::Ristrutturazione,
        "debito::la_ristrutturazione_taglia_il_debito",
    ),
    (
        TipoVoce::MargineEnergia,
        "shock::l_opzione_clausola_ricorda_l_anno_della_chiusura",
    ),
    (
        TipoVoce::PropostaRespinta,
        "proposte::proposta_respinta_per_silenzio",
    ),
    (
        TipoVoce::PropostaDecaduta,
        "proposte::la_riforma_gia_in_vigore_decade_senza_costo",
    ),
    (
        TipoVoce::EmendamentoRespinto,
        "emendamenti::emendamento_ignorato_vale_respinto_e_costa_intesa",
    ),
    (
        TipoVoce::AttoEseguito,
        "agenda::atto_eseguito_lascia_la_voce",
    ),
    (
        TipoVoce::AttoSaltato,
        "agenda::avvio_cantiere_paga_una_sola_rata_e_le_rate_precedono_l_agenda",
    ),
    (
        TipoVoce::CantiereAvviato,
        "agenda::avvio_cantiere_paga_una_sola_rata_e_le_rate_precedono_l_agenda",
    ),
    (
        TipoVoce::CantiereFermo,
        "agenda::cantiere_senza_fondi_resta_fermo_senza_perdere_quanto_speso",
    ),
    (
        TipoVoce::CantiereFinito,
        "agenda::cantiere_finito_lascia_la_voce",
    ),
    (
        TipoVoce::LeggiSlittate,
        "bilancio_in_aula::fiducia_persa_nello_slittamento_si_rimborsa",
    ),
    (
        TipoVoce::LeggeApprovata,
        "voto::legge_approvata_alla_fine_del_turno_dopo",
    ),
    (
        TipoVoce::LeggeRespinta,
        "voto::legge_respinta_si_ripresenta_dopo_due_turni",
    ),
    (
        TipoVoce::DecretoConvertito,
        "decreti::accogli_apre_una_conversione_a_fine_turno",
    ),
    (
        TipoVoce::DecretoDecaduto,
        "decreti::spostamento_decaduto_torna_indietro_e_non_si_ripete",
    ),
    (
        TipoVoce::BilancioApprovato,
        "bilancio_in_aula::bilancio_si_vota_e_passa_con_la_maggioranza",
    ),
    (
        TipoVoce::BilancioRicondotto,
        "debito::bilancio_ricondotto_lascia_la_voce",
    ),
    (
        TipoVoce::BilancioRespinto,
        "bilancio_in_aula::bilancio_respinto_porta_l_esercizio_provvisorio",
    ),
    (
        TipoVoce::FiduciaRespinta,
        "caduta::fiducia_respinta_fa_dimettere",
    ),
    (
        TipoVoce::SfiduciaApprovata,
        "opposizione::la_sfiducia_approvata_fa_cadere_il_governo",
    ),
    (
        TipoVoce::SfiduciaRespinta,
        "opposizione::la_sfiducia_respinta_ricompatta_e_costa_valence",
    ),
    (
        TipoVoce::SfiduciaScartata,
        "opposizione::la_sfiducia_scartata_lascia_la_voce",
    ),
    (
        TipoVoce::ValoreRinviato,
        "emendamenti::regime_emendato_arriva_dopo_quattro_turni",
    ),
    (
        TipoVoce::ValoreNonApplicato,
        "debito::valore_non_applicato_per_il_memorandum",
    ),
    (
        TipoVoce::MisuraDecaduta,
        "caduta::alla_caduta_il_calendario_decade",
    ),
    (
        TipoVoce::ConfermativoInAttesa,
        "norme::senza_i_due_terzi_si_vota_il_referendum_confermativo",
    ),
    (
        TipoVoce::ConfermativoSenzaVoto,
        "referendum::il_giocatore_rinuncia_al_confermativo",
    ),
    (
        TipoVoce::ReferendumVotato,
        "referendum::abrogazione_riporta_al_valore_dopo_l_abrogazione_e_blocca_il_gradino",
    ),
    (
        TipoVoce::ReferendumDecaduto,
        "referendum::un_valore_gia_abrogato_prima_del_voto_fa_decadere_il_quesito",
    ),
    (
        TipoVoce::ReferendumChiesto,
        "opposizione_ia::l_opposizione_ia_chiede_solo_referendum_vincenti",
    ),
    (
        TipoVoce::FirmeRaccolte,
        "referendum::firme_in_chiusura_e_invito_una_volta",
    ),
    (
        TipoVoce::Scissione,
        "vita_dei_partiti::a_un_punto_il_partito_si_scinde",
    ),
    (
        TipoVoce::MinacciaDiScissione,
        "vita_dei_partiti::a_mezzo_punto_solo_la_minaccia",
    ),
    (
        TipoVoce::MinacciaRientrata,
        "vita_dei_partiti::a_mezzo_punto_solo_la_minaccia",
    ),
    (
        TipoVoce::PromessaVerificata,
        "promesse::promessa_mantenuta_da_valence",
    ),
    (
        TipoVoce::PromesseScadute,
        "promesse::senza_quattro_turni_la_promessa_non_conta",
    ),
    (
        TipoVoce::PromessaFatta,
        "promesse::al_turno_zero_promettono_il_presidente_e_gli_altri_partiti_ia",
    ),
    (TipoVoce::SpesaDiCampagna, "campagna::spese_al_turno_zero"),
    (
        TipoVoce::CampagnaTagliata,
        "acquisti::campagna_tagliata_dagli_acquisti",
    ),
    (
        TipoVoce::MesFinito,
        "debito::il_programma_dura_durata_mes_chiusure",
    ),
    (
        TipoVoce::Spread,
        "sondaggi::turno_quieto_ha_prima_stima_e_sondaggio",
    ),
    (
        TipoVoce::ControlloUe,
        "conti::il_conto_di_controllo_non_scende_sotto_zero",
    ),
    (
        TipoVoce::ProceduraUe,
        "conti::pde_aperta_senza_deviazione_va_sospesa",
    ),
    (
        TipoVoce::AmmendaUe,
        "conti::la_catena_delle_sanzioni_si_attraversa",
    ),
    (
        TipoVoce::RistrutturazioneFinita,
        "debito::la_perdita_di_reddito_finisce_dentro_la_partita",
    ),
    (
        TipoVoce::RedditoDeluso,
        "partita::reddito_deluso_sopra_lo_scarto",
    ),
    (
        TipoVoce::Sondaggio,
        "sondaggi::turno_quieto_ha_prima_stima_e_sondaggio",
    ),
    (
        TipoVoce::PrimaStima,
        "sondaggi::turno_quieto_ha_prima_stima_e_sondaggio",
    ),
    (
        TipoVoce::Revisione,
        "sondaggi::turno_quieto_ha_prima_stima_e_sondaggio",
    ),
    (
        TipoVoce::SituazioneAccesa,
        "partita::situazione_accesa_tace_se_nasce_il_suo_evento",
    ),
    (
        TipoVoce::SituazioneSpenta,
        "situazioni::situazione_spenta_lascia_la_voce",
    ),
    (
        TipoVoce::CrisiDelDebito,
        "debito::spread_oltre_soglia_apre_le_aste_scoperte",
    ),
    (
        TipoVoce::EntrateDiCassa,
        "cassa::le_entrate_seguono_voti_e_intenzioni",
    ),
    (
        TipoVoce::RisultatoDelVoto,
        "voto_turno_zero::il_voto_del_turno_zero_elegge_il_parlamento",
    ),
    (
        TipoVoce::Incarico,
        "ribaltone::primo_nel_ribaltone_ottieni_l_incarico",
    ),
    (
        TipoVoce::FineDellaPartita,
        "voto_turno_zero::senza_seggi_la_partita_finisce",
    ),
    (
        TipoVoce::PartitoSciolto,
        "esito::partito_sciolto_e_salvataggio_ricaricabile",
    ),
    (
        TipoVoce::SoddisfazioneVariata,
        "bilancio::ingresso_in_provvisorio_costa_la_perdita_del_bilancio_che_continua",
    ),
    (
        TipoVoce::MinacciaDiDimissioni,
        "ministri::minaccia_nata_nella_chiusura",
    ),
    (
        TipoVoce::MinistroEsce,
        "ministri::dimissioni_chieste_portano_interim_rosa_e_rodaggio",
    ),
    (
        TipoVoce::IntesaVariata,
        "intesa::ministri_scontenti_costano_intesa",
    ),
    (
        TipoVoce::UltimatumPosto,
        "intesa::l_ultimatum_resta_con_la_sua_causa",
    ),
    (
        TipoVoce::UltimatumRitirato,
        "intesa::ultimatum_si_ritira_alla_soglia_e_non_si_riscrive",
    ),
    (
        TipoVoce::CambioDiLeader,
        "partiti_ia::chi_perde_il_governo_cambia_leader",
    ),
    (
        TipoVoce::FrangiaEsce,
        "vita_dei_partiti::la_frangia_esce_quando_le_conviene",
    ),
    (
        TipoVoce::AlleatoEsce,
        "crisi::alleato_che_esce_lascia_la_voce",
    ),
    (
        TipoVoce::Rimpasto,
        "crisi::rimpasto_se_la_maggioranza_regge",
    ),
    (
        TipoVoce::CrisiAperta,
        "crisi::un_alleato_sotto_25_esce_e_apre_la_crisi",
    ),
    (
        TipoVoce::CoalizioneAllargata,
        "crisi::allargare_in_crisi_salva_il_governo",
    ),
    (
        TipoVoce::GovernoCaduto,
        "governo_ia::senza_vicini_il_motivo_resta_nessun_partito",
    ),
    (
        TipoVoce::CamereSciolte,
        "caduta::senza_maggioranza_contigua_il_capo_dello_stato_scioglie",
    ),
    (
        TipoVoce::GuidaRifiutata,
        "ribaltone::chi_guida_la_propria_coalizione_non_cede",
    ),
    (
        TipoVoce::GovernoFormato,
        "coalizione::incarico_e_governo_del_giocatore",
    ),
    (
        TipoVoce::ProgrammaDichiarato,
        "governo_ia::il_governo_ia_nasce_col_programma_pubblico",
    ),
    (
        TipoVoce::RotturaDellImpegno,
        "intesa::allargare_all_altro_campo_rompe_l_impegno",
    ),
    (
        TipoVoce::EventoNato,
        "eventi::evento_nato_porta_la_famiglia_e_la_causa",
    ),
    (
        TipoVoce::ShockAssorbito,
        "shock::shock_escluso_voce_senza_effetti",
    ),
    (
        TipoVoce::ValoreInVigore,
        "stato_disposizioni::valore_in_arrivo_arriva_al_suo_turno",
    ),
    (
        TipoVoce::ArrivoScartato,
        "stato_disposizioni::arrivo_elettorale_scartato_lascia_la_voce",
    ),
    (
        TipoVoce::PosizioneSpostata,
        "campagna::i_partiti_ia_si_muovono_nel_turno_prima_del_voto",
    ),
    (
        TipoVoce::PartitoDiProtesta,
        "vita_dei_partiti::nasce_la_protesta_dove_l_asse_e_scoperto",
    ),
    (
        TipoVoce::CoalizioneIa,
        "partita/ia::le_coalizioni_ia_al_turno_19",
    ),
    (
        TipoVoce::EmendamentoChiesto,
        "emendamenti::alleato_lontano_chiede_un_emendamento_a_meta_strada",
    ),
    (
        TipoVoce::MisuraPresentata,
        "governo_ia::il_programma_va_in_aula_in_ordine",
    ),
    (
        TipoVoce::PropostaAccolta,
        "governo_ia::il_governo_ia_accoglie_la_proposta_del_dicastero",
    ),
    (
        TipoVoce::EmendamentoDeciso,
        "partita::il_governo_ia_accoglie_l_emendamento_dell_alleato_sotto_40",
    ),
    (
        TipoVoce::FiduciaPosta,
        "partita::il_governo_ia_pone_la_fiducia_se_la_forbice_e_in_bilico",
    ),
    (
        TipoVoce::SfiduciaPresentata,
        "opposizione_ia::l_opposizione_ia_sfiducia_solo_quando_passerebbe",
    ),
    (
        TipoVoce::GovernoDimissionario,
        "governo_ia::il_governo_avversario_si_dimette_col_motivo",
    ),
];

/// Il sorgente del file di test nominato in `ELENCO` (`partita` e `partita/ia` sono i `mod tests`
/// del core); `None` per un file che qui manca.
fn sorgente(file: &str) -> Option<&'static str> {
    Some(match file {
        "acquisti" => include_str!("acquisti.rs"),
        "agenda" => include_str!("agenda.rs"),
        "bilancio" => include_str!("bilancio.rs"),
        "bilancio_in_aula" => include_str!("bilancio_in_aula.rs"),
        "caduta" => include_str!("caduta.rs"),
        "campagna" => include_str!("campagna.rs"),
        "cassa" => include_str!("cassa.rs"),
        "coalizione" => include_str!("coalizione.rs"),
        "conti" => include_str!("conti.rs"),
        "crisi" => include_str!("crisi.rs"),
        "debito" => include_str!("debito.rs"),
        "decreti" => include_str!("decreti.rs"),
        "emendamenti" => include_str!("emendamenti.rs"),
        "emergenze" => include_str!("emergenze.rs"),
        "esito" => include_str!("esito.rs"),
        "eventi" => include_str!("eventi.rs"),
        "governo_ia" => include_str!("governo_ia.rs"),
        "intesa" => include_str!("intesa.rs"),
        "ministri" => include_str!("ministri.rs"),
        "norme" => include_str!("norme.rs"),
        "opposizione" => include_str!("opposizione.rs"),
        "opposizione_ia" => include_str!("opposizione_ia.rs"),
        "partita" => include_str!("../src/partita.rs"),
        "partita/ia" => include_str!("../src/partita/ia.rs"),
        "partiti_ia" => include_str!("partiti_ia.rs"),
        "promesse" => include_str!("promesse.rs"),
        "proposte" => include_str!("proposte.rs"),
        "referendum" => include_str!("referendum.rs"),
        "ribaltone" => include_str!("ribaltone.rs"),
        "shock" => include_str!("shock.rs"),
        "situazioni" => include_str!("situazioni.rs"),
        "sondaggi" => include_str!("sondaggi.rs"),
        "stato_disposizioni" => include_str!("stato_disposizioni.rs"),
        "vita_dei_partiti" => include_str!("vita_dei_partiti.rs"),
        "voto" => include_str!("voto.rs"),
        "voto_turno_zero" => include_str!("voto_turno_zero.rs"),
        _ => return None,
    })
}

/// `fn nome(` esiste nel sorgente ed è un test: la riga prima porta `#[test]`.
fn e_un_test(sorgente: &str, nome: &str) -> bool {
    let testa = format!("fn {nome}(");
    let righe: Vec<&str> = sorgente.lines().map(str::trim).collect();
    (1..righe.len()).any(|i| righe[i].starts_with(&testa) && righe[i - 1] == "#[test]")
}

#[test]
fn ogni_tipo_e_coperto() {
    let mut visti = [false; ELENCO.len()];
    for (i, (tipo, test)) in ELENCO.iter().enumerate() {
        nell_elenco(*tipo);
        let (file, nome) = test.split_once("::").unwrap();
        let s = sorgente(file).unwrap_or_else(|| panic!("{tipo:?}: file {file} sconosciuto"));
        assert!(e_un_test(s, nome), "{tipo:?}: {test} non è un test");
        // Il cast dà il posto della variante nell'enum; l'elenco lo segue riga per riga.
        let posto = *tipo as usize;
        assert!(
            posto < visti.len(),
            "{tipo:?}: l'elenco ha meno righe dei tipi"
        );
        assert!(!visti[posto], "{tipo:?} compare due volte");
        visti[posto] = true;
        assert_eq!(posto, i, "{tipo:?} fuori dall'ordine dell'enum");
    }
    assert_eq!(
        TipoVoce::GovernoDimissionario as usize + 1,
        ELENCO.len(),
        "l'elenco non ha una riga per ogni tipo"
    );
}
