//! Scenario di partenza (`scenario.toml`): solo ingressi, i derivati si ricalcolano al caricamento.

use crate::errore::Errore;
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegimePensioni {
    Retributivo,
    Contributivo,
    QuotaBaseUniversale,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegimeSussidi {
    Assicurativo,
    Universale,
}

macro_rules! scenario {
    ($($campo:ident),* $(,)?) => {
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub struct Scenario {
            $(pub $campo: Ingresso,)*
            pub regime_pensioni: RegimePensioni,
            pub regime_sussidi: RegimeSussidi,
            pub pde: StatoPde,
            /// Tetto di crescita della spesa netta per anno; dopo l'ultimo vale l'ultimo.
            pub tetti_spesa_netta: BTreeMap<u16, f64>,
            pub clausola_nazionale_fino_a: u16,
        }

        impl Scenario {
            fn ingressi(&self) -> Vec<(&'static str, &Ingresso)> {
                vec![$((stringify!($campo), &self.$campo)),*]
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
    eta_pensione,
    indicizzazione_pensioni,
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
    /// Ogni ingresso è finito e nel suo intervallo.
    ///
    /// # Errors
    /// `ScenarioFuoriIntervallo` col nome del primo ingresso fuori.
    pub fn valida(&self) -> Result<(), Errore> {
        for (nome, i) in self.ingressi() {
            let finito = i.valore.is_finite() && i.min.is_finite() && i.max.is_finite();
            if !(finito && i.min <= i.valore && i.valore <= i.max) {
                return Err(Errore::ScenarioFuoriIntervallo {
                    nome: nome.into(),
                    valore: i.valore,
                    min: i.min,
                    max: i.max,
                });
            }
        }
        Ok(())
    }
}
