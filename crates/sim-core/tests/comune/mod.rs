use sim_core::dati::{DatiGioco, FileDati};

#[allow(
    dead_code,
    reason = "ogni crate di test usa solo una parte degli helper"
)]
pub fn file_base() -> FileDati {
    FileDati {
        dicasteri: include_str!("../../../../dati/base/dicasteri.toml").into(),
        manopole: include_str!("../../../../dati/base/manopole.toml").into(),
    }
}

#[allow(
    dead_code,
    reason = "ogni crate di test usa solo una parte degli helper"
)]
pub fn dati_base() -> DatiGioco {
    DatiGioco::da_testi(&file_base()).expect("i dati di base devono essere validi")
}
