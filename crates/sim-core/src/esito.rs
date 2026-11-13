//! La voce dell'esito: ciò che una chiusura di turno ha prodotto, con la sua criticità, e la
//! scelta delle sette in evidenza (spec «esito e Vista completa»).

use crate::eventi::{Famiglia, Tono};
use serde::{Deserialize, Serialize};
#[cfg(test)]
use ts_rs::TS;

/// Quante voci l'esito porta in evidenza al massimo.
pub const MASSIMO_IN_EVIDENZA: usize = 7;

/// Un tipo per ogni riga del catalogo della spec: un fatto nuovo della chiusura prende un tipo
/// nuovo, e `grado` non compila finché non gli dà una criticità.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
#[serde(rename_all = "snake_case")]
pub enum TipoVoce {
    EmergenzaLieve,
    EventoRisolto,
    MesAvviato,
    Ristrutturazione,
    MargineEnergia,
    PropostaRespinta,
    PropostaDecaduta,
    EmendamentoRespinto,
    AttoEseguito,
    AttoSaltato,
    CantiereAvviato,
    CantiereFermo,
    CantiereFinito,
    LeggiSlittate,
    LeggeApprovata,
    LeggeRespinta,
    DecretoConvertito,
    DecretoDecaduto,
    BilancioApprovato,
    BilancioRicondotto,
    BilancioRespinto,
    FiduciaRespinta,
    SfiduciaApprovata,
    SfiduciaRespinta,
    SfiduciaScartata,
    ValoreRinviato,
    ValoreNonApplicato,
    MisuraDecaduta,
    ConfermativoInAttesa,
    ConfermativoSenzaVoto,
    ReferendumVotato,
    ReferendumDecaduto,
    ReferendumChiesto,
    FirmeRaccolte,
    Scissione,
    MinacciaDiScissione,
    MinacciaRientrata,
    PromessaVerificata,
    PromesseScadute,
    PromessaFatta,
    SpesaDiCampagna,
    CampagnaTagliata,
    MesFinito,
    Spread,
    ControlloUe,
    ProceduraUe,
    AmmendaUe,
    RistrutturazioneFinita,
    RedditoDeluso,
    Sondaggio,
    PrimaStima,
    Revisione,
    SituazioneAccesa,
    SituazioneSpenta,
    CrisiDelDebito,
    EntrateDiCassa,
    RisultatoDelVoto,
    Incarico,
    FineDellaPartita,
    PartitoSciolto,
    SoddisfazioneVariata,
    MinacciaDiDimissioni,
    MinistroEsce,
    IntesaVariata,
    UltimatumPosto,
    UltimatumRitirato,
    CambioDiLeader,
    FrangiaEsce,
    AlleatoEsce,
    Rimpasto,
    CrisiAperta,
    CoalizioneAllargata,
    GovernoCaduto,
    CamereSciolte,
    GuidaRifiutata,
    GovernoFormato,
    ProgrammaDichiarato,
    RotturaDellImpegno,
    EventoNato,
    ShockAssorbito,
    ValoreInVigore,
    ArrivoScartato,
    PosizioneSpostata,
    PartitoDiProtesta,
    CoalizioneIa,
    EmendamentoChiesto,
    MisuraPresentata,
    PropostaAccolta,
    EmendamentoDeciso,
    FiduciaPosta,
    SfiduciaPresentata,
    GovernoDimissionario,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
#[serde(rename_all = "snake_case")]
pub enum Criticita {
    Critica,
    Notevole,
    Ordinaria,
}

/// La criticità del catalogo: `C3`, `C2`, `C1` sono le critiche e il numero è la precedenza.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Grado {
    C3,
    C2,
    C1,
    N,
    O,
}

impl Grado {
    fn criticita(self) -> Criticita {
        match self {
            Grado::C3 | Grado::C2 | Grado::C1 => Criticita::Critica,
            Grado::N => Criticita::Notevole,
            Grado::O => Criticita::Ordinaria,
        }
    }

    fn precedenza(self) -> u8 {
        match self {
            Grado::C3 => 3,
            Grado::C2 => 2,
            Grado::C1 => 1,
            Grado::N | Grado::O => 0,
        }
    }
}

/// Il soggetto di una voce fusa, copiato così com'è dalla voce confluita.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct Dettaglio {
    pub testo: String,
    pub partito: Option<String>,
    pub dicastero: Option<String>,
    pub regione: Option<String>,
    pub misura: Option<u32>,
    pub disposizioni: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceEsito {
    pub tipo: TipoVoce,
    pub criticita: Criticita,
    pub precedenza: u8,
    pub in_evidenza: bool,
    pub tono: Tono,
    pub fatto: String,
    pub causa: String,
    pub partito: Option<String>,
    pub dicastero: Option<String>,
    pub regione: Option<String>,
    pub misura: Option<u32>,
    pub disposizioni: Vec<String>,
    pub famiglia: Option<Famiglia>,
    pub dettagli: Vec<Dettaglio>,
}

impl VoceEsito {
    pub(crate) fn nuova(tipo: TipoVoce, g: Grado, fatto: String, causa: String) -> VoceEsito {
        VoceEsito {
            tipo,
            criticita: g.criticita(),
            precedenza: g.precedenza(),
            in_evidenza: false,
            tono: Tono::Ordinario,
            fatto,
            causa,
            partito: None,
            dicastero: None,
            regione: None,
            misura: None,
            disposizioni: Vec::new(),
            famiglia: None,
            dettagli: Vec::new(),
        }
    }

    /// Per i tipi condizionati: `grado` rende il caso base, il punto del fatto lo cambia.
    pub(crate) fn con_grado(&mut self, g: Grado) -> &mut VoceEsito {
        self.criticita = g.criticita();
        self.precedenza = g.precedenza();
        self
    }
}

#[cfg(test)]
const TUTTI: [TipoVoce; 92] = [
    TipoVoce::EmergenzaLieve,
    TipoVoce::EventoRisolto,
    TipoVoce::MesAvviato,
    TipoVoce::Ristrutturazione,
    TipoVoce::MargineEnergia,
    TipoVoce::PropostaRespinta,
    TipoVoce::PropostaDecaduta,
    TipoVoce::EmendamentoRespinto,
    TipoVoce::AttoEseguito,
    TipoVoce::AttoSaltato,
    TipoVoce::CantiereAvviato,
    TipoVoce::CantiereFermo,
    TipoVoce::CantiereFinito,
    TipoVoce::LeggiSlittate,
    TipoVoce::LeggeApprovata,
    TipoVoce::LeggeRespinta,
    TipoVoce::DecretoConvertito,
    TipoVoce::DecretoDecaduto,
    TipoVoce::BilancioApprovato,
    TipoVoce::BilancioRicondotto,
    TipoVoce::BilancioRespinto,
    TipoVoce::FiduciaRespinta,
    TipoVoce::SfiduciaApprovata,
    TipoVoce::SfiduciaRespinta,
    TipoVoce::SfiduciaScartata,
    TipoVoce::ValoreRinviato,
    TipoVoce::ValoreNonApplicato,
    TipoVoce::MisuraDecaduta,
    TipoVoce::ConfermativoInAttesa,
    TipoVoce::ConfermativoSenzaVoto,
    TipoVoce::ReferendumVotato,
    TipoVoce::ReferendumDecaduto,
    TipoVoce::ReferendumChiesto,
    TipoVoce::FirmeRaccolte,
    TipoVoce::Scissione,
    TipoVoce::MinacciaDiScissione,
    TipoVoce::MinacciaRientrata,
    TipoVoce::PromessaVerificata,
    TipoVoce::PromesseScadute,
    TipoVoce::PromessaFatta,
    TipoVoce::SpesaDiCampagna,
    TipoVoce::CampagnaTagliata,
    TipoVoce::MesFinito,
    TipoVoce::Spread,
    TipoVoce::ControlloUe,
    TipoVoce::ProceduraUe,
    TipoVoce::AmmendaUe,
    TipoVoce::RistrutturazioneFinita,
    TipoVoce::RedditoDeluso,
    TipoVoce::Sondaggio,
    TipoVoce::PrimaStima,
    TipoVoce::Revisione,
    TipoVoce::SituazioneAccesa,
    TipoVoce::SituazioneSpenta,
    TipoVoce::CrisiDelDebito,
    TipoVoce::EntrateDiCassa,
    TipoVoce::RisultatoDelVoto,
    TipoVoce::Incarico,
    TipoVoce::FineDellaPartita,
    TipoVoce::PartitoSciolto,
    TipoVoce::SoddisfazioneVariata,
    TipoVoce::MinacciaDiDimissioni,
    TipoVoce::MinistroEsce,
    TipoVoce::IntesaVariata,
    TipoVoce::UltimatumPosto,
    TipoVoce::UltimatumRitirato,
    TipoVoce::CambioDiLeader,
    TipoVoce::FrangiaEsce,
    TipoVoce::AlleatoEsce,
    TipoVoce::Rimpasto,
    TipoVoce::CrisiAperta,
    TipoVoce::CoalizioneAllargata,
    TipoVoce::GovernoCaduto,
    TipoVoce::CamereSciolte,
    TipoVoce::GuidaRifiutata,
    TipoVoce::GovernoFormato,
    TipoVoce::ProgrammaDichiarato,
    TipoVoce::RotturaDellImpegno,
    TipoVoce::EventoNato,
    TipoVoce::ShockAssorbito,
    TipoVoce::ValoreInVigore,
    TipoVoce::ArrivoScartato,
    TipoVoce::PosizioneSpostata,
    TipoVoce::PartitoDiProtesta,
    TipoVoce::CoalizioneIa,
    TipoVoce::EmendamentoChiesto,
    TipoVoce::MisuraPresentata,
    TipoVoce::PropostaAccolta,
    TipoVoce::EmendamentoDeciso,
    TipoVoce::FiduciaPosta,
    TipoVoce::SfiduciaPresentata,
    TipoVoce::GovernoDimissionario,
];

/// La criticità del tipo, col giocatore al governo o all'opposizione; `None` dove il fatto non
/// accade o la Vista non lo mostra. Per i tipi condizionati rende il caso base.
pub(crate) fn grado(tipo: TipoVoce, al_governo: bool) -> Option<Grado> {
    let (governo, opposizione) = match tipo {
        TipoVoce::EmergenzaLieve
        | TipoVoce::MargineEnergia
        | TipoVoce::AttoEseguito
        | TipoVoce::CantiereAvviato
        | TipoVoce::LeggiSlittate
        | TipoVoce::DecretoConvertito
        | TipoVoce::SfiduciaScartata
        | TipoVoce::ValoreRinviato
        | TipoVoce::ConfermativoSenzaVoto
        | TipoVoce::ReferendumDecaduto
        | TipoVoce::MinacciaDiScissione
        | TipoVoce::MinacciaRientrata
        | TipoVoce::PromessaVerificata
        | TipoVoce::PromesseScadute
        | TipoVoce::PromessaFatta
        | TipoVoce::Spread
        | TipoVoce::Sondaggio
        | TipoVoce::PrimaStima
        | TipoVoce::Revisione
        | TipoVoce::EntrateDiCassa
        | TipoVoce::ArrivoScartato
        | TipoVoce::PosizioneSpostata => (Some(Grado::O), Some(Grado::O)),
        TipoVoce::EventoRisolto
        | TipoVoce::LeggeApprovata
        | TipoVoce::BilancioApprovato
        | TipoVoce::SfiduciaRespinta
        | TipoVoce::ConfermativoInAttesa
        | TipoVoce::ReferendumVotato
        | TipoVoce::Scissione
        | TipoVoce::SpesaDiCampagna
        | TipoVoce::RedditoDeluso
        | TipoVoce::PartitoSciolto
        | TipoVoce::UltimatumRitirato
        | TipoVoce::CambioDiLeader
        | TipoVoce::CoalizioneAllargata
        | TipoVoce::RotturaDellImpegno
        | TipoVoce::PartitoDiProtesta
        | TipoVoce::CoalizioneIa
        | TipoVoce::MisuraPresentata => (Some(Grado::N), Some(Grado::N)),
        TipoVoce::MesAvviato
        | TipoVoce::Ristrutturazione
        | TipoVoce::DecretoDecaduto
        | TipoVoce::BilancioRicondotto
        | TipoVoce::BilancioRespinto
        | TipoVoce::AmmendaUe
        | TipoVoce::CrisiDelDebito => (Some(Grado::C2), Some(Grado::N)),
        TipoVoce::PropostaRespinta | TipoVoce::EmendamentoRespinto => (Some(Grado::C1), None),
        TipoVoce::PropostaDecaduta | TipoVoce::SoddisfazioneVariata | TipoVoce::IntesaVariata => {
            (Some(Grado::O), None)
        }
        TipoVoce::AttoSaltato
        | TipoVoce::CantiereFermo
        | TipoVoce::CantiereFinito
        | TipoVoce::ValoreNonApplicato
        | TipoVoce::MisuraDecaduta
        | TipoVoce::ReferendumChiesto
        | TipoVoce::FirmeRaccolte
        | TipoVoce::MesFinito
        | TipoVoce::ControlloUe
        | TipoVoce::ProceduraUe
        | TipoVoce::RistrutturazioneFinita
        | TipoVoce::SituazioneSpenta
        | TipoVoce::Rimpasto
        | TipoVoce::ShockAssorbito
        | TipoVoce::ValoreInVigore => (Some(Grado::N), Some(Grado::O)),
        TipoVoce::LeggeRespinta => (Some(Grado::O), Some(Grado::N)),
        TipoVoce::FiduciaRespinta
        | TipoVoce::SfiduciaApprovata
        | TipoVoce::RisultatoDelVoto
        | TipoVoce::Incarico
        | TipoVoce::FineDellaPartita
        | TipoVoce::CrisiAperta
        | TipoVoce::GovernoCaduto
        | TipoVoce::CamereSciolte
        | TipoVoce::GovernoFormato => (Some(Grado::C3), Some(Grado::C3)),
        TipoVoce::CampagnaTagliata => (Some(Grado::C1), Some(Grado::C1)),
        TipoVoce::SituazioneAccesa | TipoVoce::EventoNato => (Some(Grado::C1), Some(Grado::N)),
        TipoVoce::MinacciaDiDimissioni => (Some(Grado::C2), None),
        TipoVoce::MinistroEsce => (Some(Grado::C2), Some(Grado::O)),
        TipoVoce::UltimatumPosto | TipoVoce::FrangiaEsce | TipoVoce::AlleatoEsce => {
            (Some(Grado::C2), Some(Grado::C2))
        }
        TipoVoce::GuidaRifiutata | TipoVoce::GovernoDimissionario => (None, Some(Grado::C3)),
        TipoVoce::ProgrammaDichiarato | TipoVoce::FiduciaPosta => (None, Some(Grado::N)),
        TipoVoce::EmendamentoChiesto => (Some(Grado::N), None),
        TipoVoce::PropostaAccolta | TipoVoce::EmendamentoDeciso => (None, Some(Grado::O)),
        TipoVoce::SfiduciaPresentata => (Some(Grado::C3), Some(Grado::N)),
    };
    if al_governo { governo } else { opposizione }
}

/// Il fatto al plurale della voce fusa; `None` per i tipi che non si fondono. Esaustiva: un tipo
/// nuovo deve scegliere qui se si fonde.
fn plurale(tipo: TipoVoce) -> Option<&'static str> {
    use TipoVoce::*;
    let p = match tipo {
        EmergenzaLieve => "emergenze lievi",
        PropostaRespinta => "proposte respinte",
        PropostaDecaduta => "proposte decadute",
        EmendamentoRespinto => "emendamenti respinti",
        AttoEseguito => "atti eseguiti",
        AttoSaltato => "atti saltati",
        CantiereAvviato => "cantieri avviati",
        CantiereFermo => "cantieri fermi",
        CantiereFinito => "cantieri finiti",
        ValoreRinviato => "valori rinviati",
        ValoreNonApplicato => "valori non applicati",
        MisuraDecaduta => "misure decadute",
        PromessaVerificata => "promesse verificate",
        PromesseScadute => "promesse scadute",
        PromessaFatta => "promesse fatte",
        SpesaDiCampagna => "spese di campagna",
        RedditoDeluso => "fasce di reddito deluse",
        EntrateDiCassa => "entrate di cassa",
        PartitoSciolto => "partiti sciolti",
        SoddisfazioneVariata => "ministri cambiano soddisfazione",
        MinacciaDiDimissioni => "ministri minacciano le dimissioni",
        MinistroEsce => "ministri escono",
        IntesaVariata => "alleati cambiano intesa",
        CambioDiLeader => "cambi di leader",
        RotturaDellImpegno => "rotture dell'impegno",
        ShockAssorbito => "shock assorbiti",
        ValoreInVigore => "valori entrano in vigore",
        ArrivoScartato => "valori in arrivo scartati",
        PosizioneSpostata => "posizioni spostate",
        CoalizioneIa => "coalizioni fra partiti IA",
        EmendamentoChiesto => "emendamenti chiesti",
        MisuraPresentata => "misure presentate",
        PropostaAccolta => "proposte accolte",
        EmendamentoDeciso => "emendamenti decisi",
        EventoRisolto
        | MesAvviato
        | Ristrutturazione
        | MargineEnergia
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
        | ConfermativoInAttesa
        | ConfermativoSenzaVoto
        | ReferendumVotato
        | ReferendumDecaduto
        | ReferendumChiesto
        | FirmeRaccolte
        | Scissione
        | MinacciaDiScissione
        | MinacciaRientrata
        | CampagnaTagliata
        | MesFinito
        | Spread
        | ControlloUe
        | ProceduraUe
        | AmmendaUe
        | RistrutturazioneFinita
        | Sondaggio
        | PrimaStima
        | Revisione
        | SituazioneAccesa
        | SituazioneSpenta
        | CrisiDelDebito
        | RisultatoDelVoto
        | Incarico
        | FineDellaPartita
        | UltimatumPosto
        | UltimatumRitirato
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
        | EventoNato
        | PartitoDiProtesta
        | FiduciaPosta
        | SfiduciaPresentata
        | GovernoDimissionario => return None,
    };
    Some(p)
}

/// Se le gemelle (stesso tipo, stessa criticità) si fondono in una voce sola.
pub(crate) fn fondibile(tipo: TipoVoce) -> bool {
    plurale(tipo).is_some()
}

/// «N <fatto al plurale>»: il `fatto` della voce fusa.
pub(crate) fn riassunto(tipo: TipoVoce, n: usize) -> String {
    format!("{n} {}", plurale(tipo).unwrap_or("voci"))
}

/// Il valore che hanno in comune tutte le voci, o il vuoto se differiscono.
fn comune<T: Clone + PartialEq + Default>(voci: &[VoceEsito], campo: fn(&VoceEsito) -> &T) -> T {
    let primo = campo(&voci[0]);
    if voci.iter().all(|v| campo(v) == primo) {
        primo.clone()
    } else {
        T::default()
    }
}

/// La causa più frequente, o «più cause» se nessuna prevale.
fn causa_prevalente(voci: &[VoceEsito]) -> String {
    let mut conti: Vec<(&str, usize)> = Vec::new();
    for v in voci {
        match conti.iter_mut().find(|(c, _)| *c == v.causa) {
            Some((_, n)) => *n += 1,
            None => conti.push((&v.causa, 1)),
        }
    }
    let massimo = conti.iter().map(|&(_, n)| n).max().unwrap_or(0);
    let mut in_testa = conti.iter().filter(|&&(_, n)| n == massimo);
    match (in_testa.next(), in_testa.next()) {
        (Some(&(causa, _)), None) => causa.to_string(),
        _ => "più cause".to_string(),
    }
}

/// Le voci confluite (almeno due, stesso tipo e stessa criticità) in una.
fn fondi(gruppo: &[VoceEsito]) -> VoceEsito {
    let primo = &gruppo[0];
    VoceEsito {
        precedenza: gruppo.iter().map(|v| v.precedenza).max().unwrap_or(0),
        tono: if gruppo.iter().any(|v| v.tono == Tono::Grave) {
            Tono::Grave
        } else {
            Tono::Ordinario
        },
        fatto: riassunto(primo.tipo, gruppo.len()),
        causa: causa_prevalente(gruppo),
        partito: comune(gruppo, |v| &v.partito),
        dicastero: comune(gruppo, |v| &v.dicastero),
        regione: comune(gruppo, |v| &v.regione),
        misura: comune(gruppo, |v| &v.misura),
        disposizioni: comune(gruppo, |v| &v.disposizioni),
        famiglia: comune(gruppo, |v| &v.famiglia),
        dettagli: gruppo
            .iter()
            .map(|v| Dettaglio {
                testo: format!("{}: {}", v.fatto, v.causa),
                partito: v.partito.clone(),
                dicastero: v.dicastero.clone(),
                regione: v.regione.clone(),
                misura: v.misura,
                disposizioni: v.disposizioni.clone(),
            })
            .collect(),
        ..primo.clone()
    }
}

/// Compone l'esito alla fine della chiusura: sale chi parla del giocatore, si fondono le gemelle,
/// si segnano le sette. Non riordina mai.
pub(crate) fn componi(voci: &mut Vec<VoceEsito>, giocatore: &str) {
    for v in voci.iter_mut() {
        let di_ogni_turno = matches!(
            v.tipo,
            TipoVoce::Sondaggio | TipoVoce::Revisione | TipoVoce::EntrateDiCassa
        );
        if v.criticita == Criticita::Ordinaria
            && v.partito.as_deref() == Some(giocatore)
            && !di_ogni_turno
        {
            v.criticita = Criticita::Notevole;
        }
    }

    let mut gruppi: Vec<Vec<VoceEsito>> = Vec::new();
    for v in std::mem::take(voci) {
        let gemelle = |g: &Vec<VoceEsito>| g[0].tipo == v.tipo && g[0].criticita == v.criticita;
        match gruppi.iter_mut().find(|g| fondibile(v.tipo) && gemelle(g)) {
            Some(g) => g.push(v),
            None => gruppi.push(vec![v]),
        }
    }
    voci.extend(gruppi.into_iter().map(|mut g| match g.len() {
        1 => g.remove(0),
        _ => fondi(&g),
    }));

    let mut critiche: Vec<usize> = (0..voci.len())
        .filter(|&i| voci[i].criticita == Criticita::Critica)
        .collect();
    // sort stabile: a parità di precedenza restano le prime nell'ordine della chiusura
    critiche.sort_by_key(|&i| std::cmp::Reverse(voci[i].precedenza));
    critiche.truncate(MASSIMO_IN_EVIDENZA);
    let liberi = MASSIMO_IN_EVIDENZA - critiche.len();
    let notevoli = (0..voci.len())
        .filter(|&i| voci[i].criticita == Criticita::Notevole)
        .take(liberi);
    let scelte: Vec<usize> = critiche.iter().copied().chain(notevoli).collect();
    for (i, v) in voci.iter_mut().enumerate() {
        v.in_evidenza = scelte.contains(&i);
    }
}

#[cfg(test)]
mod tests {
    use super::Grado::{C1, C2, C3, N, O};
    use super::TipoVoce::*;
    use super::*;

    fn voce(tipo: TipoVoce, g: Grado) -> VoceEsito {
        VoceEsito::nuova(tipo, g, format!("{tipo:?}"), "causa".into())
    }

    fn con_causa(tipo: TipoVoce, causa: &str) -> VoceEsito {
        VoceEsito::nuova(tipo, O, "fatto".into(), causa.into())
    }

    fn evidenza(voci: &[VoceEsito]) -> Vec<usize> {
        (0..voci.len()).filter(|&i| voci[i].in_evidenza).collect()
    }

    #[test]
    fn ogni_tipo_ha_un_grado() {
        assert_eq!(grado(FiduciaRespinta, true), Some(C3));
        assert_eq!(grado(SoddisfazioneVariata, false), None);
        assert_eq!(grado(GuidaRifiutata, true), None);
        assert_eq!(grado(AttoEseguito, true), Some(O));
        assert_eq!(grado(LeggeRespinta, true), Some(O));
        assert_eq!(grado(LeggeRespinta, false), Some(N));
        assert_eq!(grado(MinistroEsce, true), Some(C2));
        assert_eq!(TUTTI.len(), 92);
        let distinti: std::collections::BTreeSet<String> =
            TUTTI.iter().map(|t| format!("{t:?}")).collect();
        assert_eq!(distinti.len(), 92);
        for t in TUTTI {
            assert!(
                grado(t, true).is_some() || grado(t, false).is_some(),
                "{t:?} non accade mai"
            );
            if fondibile(t) {
                assert!(!riassunto(t, 3).ends_with(" voci"), "{t:?} senza riassunto");
            }
        }
    }

    /// La fermata `Ultimatum` legge il `partito` della voce: fusa, lo perderebbe nei dettagli.
    #[test]
    fn l_ultimatum_posto_non_si_fonde() {
        assert!(!fondibile(UltimatumPosto));
    }

    #[test]
    fn i_tipi_vanno_in_snake_case() {
        let json = serde_json::to_string(&GovernoDimissionario).unwrap();
        assert_eq!(json, "\"governo_dimissionario\"");
    }

    #[test]
    fn con_grado_cambia_criticita_e_precedenza() {
        let mut v = voce(EventoRisolto, N);
        assert_eq!((v.criticita, v.precedenza), (Criticita::Notevole, 0));
        v.con_grado(C1);
        assert_eq!((v.criticita, v.precedenza), (Criticita::Critica, 1));
        v.con_grado(O);
        assert_eq!((v.criticita, v.precedenza), (Criticita::Ordinaria, 0));
    }

    #[test]
    fn nove_critiche_ne_restano_sette() {
        let mut voci = vec![
            voce(CampagnaTagliata, C1),
            voce(MesAvviato, C2),
            voce(FiduciaRespinta, C3),
            voce(CampagnaTagliata, C1),
            voce(MesAvviato, C2),
            voce(FiduciaRespinta, C3),
            voce(CampagnaTagliata, C1),
            voce(MesAvviato, C2),
            voce(FiduciaRespinta, C3),
        ];
        componi(&mut voci, "x");
        assert_eq!(voci.len(), 9);
        assert_eq!(evidenza(&voci), vec![0, 1, 2, 4, 5, 7, 8]);
    }

    #[test]
    fn due_critiche_e_otto_notevoli() {
        let mut voci = vec![
            voce(LeggeApprovata, N),
            voce(LeggeApprovata, N),
            voce(FiduciaRespinta, C3),
            voce(LeggeApprovata, N),
            voce(LeggeApprovata, N),
            voce(LeggeApprovata, N),
            voce(MesAvviato, C2),
            voce(LeggeApprovata, N),
            voce(LeggeApprovata, N),
            voce(LeggeApprovata, N),
        ];
        componi(&mut voci, "x");
        assert_eq!(evidenza(&voci), vec![0, 1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn le_ordinarie_non_entrano_mai() {
        let mut voci = vec![voce(Spread, O), voce(Spread, O), voce(PrimaStima, O)];
        componi(&mut voci, "x");
        assert!(evidenza(&voci).is_empty());
    }

    #[test]
    fn dieci_atti_una_voce() {
        let mut voci = vec![voce(Spread, O)];
        voci.extend((0..10).map(|_| voce(AttoEseguito, O)));
        voci.push(voce(LeggeApprovata, N));
        componi(&mut voci, "x");
        assert_eq!(voci.len(), 3);
        assert_eq!(voci[1].tipo, AttoEseguito);
        assert_eq!(voci[1].dettagli.len(), 10);
        assert_eq!(voci[1].fatto, "10 atti eseguiti");
        assert_eq!(voci[1].dettagli[0].testo, "AttoEseguito: causa");
    }

    fn con_dicastero(d: &str) -> VoceEsito {
        let mut v = voce(SoddisfazioneVariata, O);
        v.dicastero = Some(d.into());
        v
    }

    #[test]
    fn i_soggetti_restano_nei_dettagli() {
        let mut voci = vec![
            con_dicastero("sanita"),
            con_dicastero("interno"),
            con_dicastero("sanita"),
        ];
        componi(&mut voci, "x");
        assert_eq!(voci.len(), 1);
        assert_eq!(voci[0].dicastero, None);
        let nei_dettagli: Vec<_> = voci[0]
            .dettagli
            .iter()
            .map(|d| d.dicastero.clone())
            .collect();
        assert_eq!(
            nei_dettagli,
            vec![
                Some("sanita".into()),
                Some("interno".into()),
                Some("sanita".into())
            ]
        );

        let mut uguali = vec![
            con_dicastero("sanita"),
            con_dicastero("sanita"),
            con_dicastero("sanita"),
        ];
        componi(&mut uguali, "x");
        assert_eq!(uguali[0].dicastero.as_deref(), Some("sanita"));
    }

    #[test]
    fn una_fondibile_sola_resta_com_e() {
        let mut voci = vec![voce(AttoEseguito, O)];
        componi(&mut voci, "x");
        assert!(voci[0].dettagli.is_empty());
        assert_eq!(voci[0].fatto, "AttoEseguito");
    }

    #[test]
    fn criticita_diverse_non_si_fondono() {
        let mut voci = vec![voce(MinistroEsce, C2), voce(MinistroEsce, N)];
        componi(&mut voci, "x");
        assert_eq!(voci.len(), 2);
    }

    #[test]
    fn il_soggetto_e_il_giocatore() {
        let di = |tipo, p: &str| {
            let mut v = voce(tipo, O);
            v.partito = Some(p.into());
            v
        };
        let mut voci = vec![
            di(PromessaVerificata, "y"),
            di(PromessaVerificata, "x"),
            di(PromessaVerificata, "z"),
            di(EntrateDiCassa, "x"),
            di(EntrateDiCassa, "x"),
        ];
        componi(&mut voci, "x");
        assert_eq!(voci.len(), 3);
        assert_eq!(voci[0].dettagli.len(), 2);
        assert_eq!(voci[0].criticita, Criticita::Ordinaria);
        assert_eq!(voci[1].criticita, Criticita::Notevole);
        assert!(voci[1].dettagli.is_empty());
        assert_eq!(voci[2].tipo, EntrateDiCassa);
        assert_eq!(voci[2].criticita, Criticita::Ordinaria);
        assert_eq!(voci[2].dettagli.len(), 2);
    }

    #[test]
    fn causa_della_fusa() {
        let mut tre = vec![
            con_causa(AttoEseguito, "a"),
            con_causa(AttoEseguito, "a"),
            con_causa(AttoEseguito, "b"),
        ];
        componi(&mut tre, "x");
        assert_eq!(tre[0].causa, "a");

        let mut due = vec![con_causa(AttoEseguito, "a"), con_causa(AttoEseguito, "b")];
        componi(&mut due, "x");
        assert_eq!(due[0].causa, "più cause");
    }

    #[test]
    fn la_fusa_e_grave_se_una_lo_e() {
        let mut voci = vec![voce(AttoEseguito, O), voce(AttoEseguito, O)];
        voci[1].tono = Tono::Grave;
        componi(&mut voci, "x");
        assert_eq!(voci[0].tono, Tono::Grave);
    }

    #[test]
    fn l_ordine_non_cambia() {
        let mut voci = vec![
            voce(Spread, O),
            voce(LeggeApprovata, N),
            voce(FiduciaRespinta, C3),
            voce(Sondaggio, O),
        ];
        componi(&mut voci, "x");
        let tipi: Vec<_> = voci.iter().map(|v| v.tipo).collect();
        assert_eq!(
            tipi,
            vec![Spread, LeggeApprovata, FiduciaRespinta, Sondaggio]
        );
    }
}
