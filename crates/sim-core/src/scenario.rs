//! Scenario di partenza (`scenario.toml`): solo ingressi, i derivati si ricalcolano al caricamento.

use crate::economia::Aliquote;
use crate::elezioni::LeggeElettorale;
use crate::errore::Errore;
use crate::partiti::{DatoGoverno, DatoPartito};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ingresso {
    pub valore: f64,
    pub min: f64,
    pub max: f64,
    pub fonte: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StatoPde {
    Nessuna,
    Aperta,
    Sospesa,
    Inadempiente,
    Intimata,
    Sanzionata,
}

impl StatoPde {
    /// Dalla 126.8 in poi: niente TPI, coesione sospesa.
    pub fn e_inadempiente(self) -> bool {
        matches!(self, Self::Inadempiente | Self::Intimata | Self::Sanzionata)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegimePensioni {
    Retributivo,
    Contributivo,
    QuotaBaseUniversale,
}

impl RegimePensioni {
    /// Dalla leva `regime_pensioni`: 0, 1, 2 arrotondati; fuori, `Contributivo`.
    pub fn da_leva(x: f64) -> RegimePensioni {
        let x = x.round();
        if x == 0.0 {
            RegimePensioni::Retributivo
        } else if x == 2.0 {
            RegimePensioni::QuotaBaseUniversale
        } else {
            RegimePensioni::Contributivo
        }
    }
}

/// Seggi per partito (id) della legislatura uscente; `i64` perché la validazione nomini un
/// seggio negativo invece di fallire nel parsing.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParlamentoUscente {
    pub camera: BTreeMap<String, i64>,
    pub senato: BTreeMap<String, i64>,
}

macro_rules! scenario {
    ($($campo:ident),* $(,)?) => {
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub struct Scenario {
            $(pub $campo: Ingresso,)*
            pub pde: StatoPde,
            /// Tetto di crescita della spesa netta per anno; dopo l'ultimo vale l'ultimo.
            pub tetti_spesa_netta: BTreeMap<u16, f64>,
            pub clausola_nazionale_fino_a: u16,
            /// Aliquote e scaglioni IRPEF del turno zero.
            pub aliquote: Aliquote,
            /// Nome di un'impostazione di `elezioni.toml`.
            pub legge_elettorale: String,
            /// Valori precedenti al turno zero, per i gradini «precedente» delle elettorali.
            pub legge_elettorale_precedente: LeggeElettorale,
            pub parlamento_uscente: ParlamentoUscente,
            #[serde(rename = "partito")]
            pub partiti: Vec<DatoPartito>,
            pub governo: DatoGoverno,
            /// Id di partito, per coalizione.
            pub coalizioni: Vec<Vec<String>>,
            /// Nomi da cui si estraggono quelli dei partiti nuovi.
            pub nomi_partiti: Vec<String>,
        }

        impl Scenario {
            fn ingressi(&self) -> Vec<(String, &Ingresso)> {
                let mut v = vec![$((stringify!($campo).to_string(), &self.$campo)),*];
                v.extend(self.partiti.iter().map(|p| (format!("valence di {}", p.id), &p.valence)));
                v
            }
        }
    };
}

scenario!(
    pil_trimestrale,
    debito_pil,
    costo_medio_debito,
    vita_media_debito,
    tasso_bce,
    inflazione,
    inflazione_t3,
    inflazione_t2,
    output_gap,
    disoccupazione,
    consumi_famiglie_pil,
    investimenti_pil,
    esportazioni_pil,
    importazioni_pil,
    irpef_pil,
    ires_pil,
    iva_pil,
    contributi_pil,
    altre_entrate_pil,
    fondi_coesione_pil,
    pensionati,
    spesa_pensioni,
    pensione_minima,
    spesa_sussidi,
    altre_prestazioni,
    conto_controllo,
    clausola_nazionale_margine,
    premio_sfiducia,
    energia,
    domanda_estera,
);

impl Scenario {
    /// Tetto di crescita della spesa netta per `anno`: dopo l'ultimo in tabella vale l'ultimo,
    /// prima del primo il primo.
    pub fn tetto_spesa_netta(&self, anno: u16) -> f64 {
        self.tetti_spesa_netta
            .range(..=anno)
            .next_back()
            .or_else(|| self.tetti_spesa_netta.iter().next())
            .map_or(0.0, |(_, t)| *t)
    }

    /// Ogni ingresso è finito e nel suo intervallo.
    ///
    /// # Errors
    /// `ScenarioFuoriIntervallo` col nome del primo ingresso fuori.
    pub fn valida(&self) -> Result<(), Errore> {
        for (nome, i) in self.ingressi() {
            let finito = i.valore.is_finite() && i.min.is_finite() && i.max.is_finite();
            if !(finito && i.min <= i.valore && i.valore <= i.max) {
                return Err(Errore::ScenarioFuoriIntervallo {
                    nome,
                    valore: i.valore,
                    min: i.min,
                    max: i.max,
                });
            }
        }
        if self.tetti_spesa_netta.is_empty()
            || self.tetti_spesa_netta.values().any(|t| !t.is_finite())
        {
            return Err(Errore::StrutturaIncoerente(
                "tetti_spesa_netta assenti o non finiti".into(),
            ));
        }
        let a = &self.aliquote;
        let nello_scenario =
            |m: String| Errore::StrutturaIncoerente(format!("scenario.toml [aliquote]: {m}"));
        a.valida().map_err(nello_scenario)?;
        // Con un'aliquota di partenza nulla il gettito di riferimento non ha base di confronto.
        let irpef = (a.irpef.iter().enumerate()).map(|(i, s)| (format!("irpef[{i}]"), s.aliquota));
        let altre = [
            ("iva", a.iva),
            ("ires", a.ires),
            ("contributi", a.contributi),
        ]
        .map(|(nome, v)| (nome.to_string(), v));
        for (nome, valore) in irpef.chain(altre) {
            if valore <= 0.0 {
                return Err(nello_scenario(format!(
                    "{nome}: l'aliquota dello scenario deve essere positiva"
                )));
            }
        }
        Ok(())
    }
}
