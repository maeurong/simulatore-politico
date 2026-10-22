use crate::caso::Caso;
use crate::dati::DatiGioco;
use crate::economia::Economia;
use crate::errore::Errore;
use crate::vista::{Vista, VoceDicastero};
use serde::{Deserialize, Serialize};
#[cfg(test)]
use ts_rs::TS;

pub const VERSIONE_SCHEMA: u32 = 2;

/// Trimestre di calendario: `trimestre` va da 1 a 4.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct Trimestre {
    pub anno: u16,
    pub trimestre: u8,
}

impl Trimestre {
    pub const TURNO_ZERO: Trimestre = Trimestre {
        anno: 2026,
        trimestre: 4,
    };

    pub fn precedente(self) -> Trimestre {
        if self.trimestre == 1 {
            Trimestre {
                anno: self.anno.saturating_sub(1),
                trimestre: 4,
            }
        } else {
            Trimestre {
                anno: self.anno,
                trimestre: self.trimestre.saturating_sub(1),
            }
        }
    }

    pub fn successivo(self) -> Trimestre {
        if self.trimestre == 4 {
            Trimestre {
                anno: self.anno.saturating_add(1),
                trimestre: 1,
            }
        } else {
            Trimestre {
                anno: self.anno,
                trimestre: self.trimestre.saturating_add(1),
            }
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Partita {
    versione_schema: u32,
    caso: Caso,
    dati: DatiGioco,
    turno: u32,
    data: Trimestre,
    economia: Economia,
}

impl Partita {
    pub fn nuova(dati: DatiGioco, seme: u64) -> Partita {
        Partita {
            versione_schema: VERSIONE_SCHEMA,
            caso: Caso::nuovo(seme),
            economia: Economia::iniziale(&dati),
            dati,
            turno: 0,
            data: Trimestre::TURNO_ZERO,
        }
    }

    // Per ora: economia (primi blocchi) e calendario. Ordine previsto quando arriva il motore:
    // atti dei ministri → aula → motore (economia, poi elettorato) → soglie → eventi del turno
    // dopo; voto delle elezioni dopo il motore.
    pub fn chiudi_turno(&mut self) {
        self.economia.passo(&self.dati, &mut self.caso, self.data);
        self.turno = self.turno.saturating_add(1);
        self.data = self.data.successivo();
    }

    /// Stato vero dell'economia: per il CLI e i test, mai per la `Vista`.
    pub fn economia(&self) -> &Economia {
        &self.economia
    }

    pub fn vista(&self) -> Vista {
        Vista {
            turno: self.turno,
            data: self.data,
            spread_pb: self.economia.spread.round() as i32,
            dicasteri: self
                .dati
                .dicasteri()
                .iter()
                .map(|d| VoceDicastero {
                    id: d.id.clone(),
                    nome: d.nome.clone(),
                })
                .collect(),
        }
    }

    /// Snapshot JSON completo: schema, stato del generatore (con il seme), dati, calendario,
    /// economia.
    ///
    /// # Errors
    /// Serializzazione fallita, o stato che riletto non torna identico
    /// (per esempio una manopola NaN, scritta come `null`): chi salva non
    /// deve scrivere un file illeggibile.
    pub fn salva(&self) -> Result<String, Errore> {
        let s = serde_json::to_string(self)
            .map_err(|e| Errore::SalvataggioNonRiuscito(e.to_string()))?;
        match Partita::carica(&s) {
            Ok(riletta) if riletta == *self => Ok(s),
            Ok(_) => Err(Errore::SalvataggioNonRiuscito(
                "lo stato riletto è diverso da quello salvato".into(),
            )),
            Err(e) => Err(Errore::SalvataggioNonRiuscito(format!(
                "lo stato salvato non si rilegge: {e}"
            ))),
        }
    }

    /// Ricostruisce una partita da uno snapshot JSON, validandolo.
    ///
    /// # Errors
    /// JSON illeggibile, schema diverso da `VERSIONE_SCHEMA`, trimestre fuori
    /// da 1-4, dati che non passano `DatiGioco::valida`, o `economia` che non combacia coi dati
    /// (regioni diverse o in altro ordine, `prezzi_settori`/`gap_settori` di lunghezza sbagliata).
    pub fn carica(json: &str) -> Result<Partita, Errore> {
        let valore: serde_json::Value = serde_json::from_str(json)
            .map_err(|e| Errore::SalvataggioIlleggibile(e.to_string()))?;
        let trovata = valore
            .get("versione_schema")
            .and_then(serde_json::Value::as_u64)
            .and_then(|v| u32::try_from(v).ok())
            .unwrap_or(0);
        if trovata != VERSIONE_SCHEMA {
            return Err(Errore::SchemaDiverso {
                trovata,
                attesa: VERSIONE_SCHEMA,
            });
        }
        let partita: Partita = serde_json::from_value(valore)
            .map_err(|e| Errore::SalvataggioIlleggibile(e.to_string()))?;
        if !(1..=4).contains(&partita.data.trimestre) {
            return Err(Errore::SalvataggioIlleggibile(format!(
                "trimestre {} fuori da 1-4",
                partita.data.trimestre
            )));
        }
        partita.dati.valida()?;
        let e = &partita.economia;
        let regioni_uguali = e.regioni.iter().map(|r| &r.id).eq(partita
            .dati
            .regioni()
            .iter()
            .map(|r| &r.id));
        if !regioni_uguali {
            return Err(Errore::SalvataggioIlleggibile(
                "economia.regioni non combacia con le regioni dei dati".into(),
            ));
        }
        let id_dicasteri = || partita.dati.dicasteri().iter().map(|d| &d.id);
        let bilanci = std::iter::once(&e.bilancio).chain(&e.bilancio_approvato);
        for b in bilanci {
            if !b.dicasteri.iter().map(|d| &d.id).eq(id_dicasteri()) {
                return Err(Errore::SalvataggioIlleggibile(
                    "economia.bilancio non combacia con i dicasteri dei dati".into(),
                ));
            }
        }
        let n_settori = partita.dati.struttura().settori.len();
        for (campo, n) in [
            ("prezzi_settori", e.prezzi_settori.len()),
            ("gap_settori", e.gap_settori.len()),
        ] {
            if n != n_settori {
                return Err(Errore::SalvataggioIlleggibile(format!(
                    "economia.{campo} ha {n} voci, i settori sono {n_settori}"
                )));
            }
        }
        partita.economia.controlla_coerenza(partita.data)?;
        Ok(partita)
    }
}

/// Tetto agli importi in euro di un salvataggio: oltre, le somme in `i64` rischiano di traboccare.
const LIMITE_EURO: i64 = 1_000_000_000_000_000;

fn ok(condizione: bool, messaggio: &str) -> Result<(), Errore> {
    if condizione {
        Ok(())
    } else {
        Err(Errore::SalvataggioIlleggibile(messaggio.into()))
    }
}

impl Economia {
    /// Invarianti dei conti che un salvataggio manomesso potrebbe rompere. L'ordine conta:
    /// i limiti sugli importi vengono prima delle somme che li usano.
    fn controlla_coerenza(&self, data: Trimestre) -> Result<(), Errore> {
        let in_limite = |x: i64| x.abs() <= LIMITE_EURO;
        let bilanci = std::iter::once(&self.bilancio).chain(&self.bilancio_approvato);
        let (c, st) = (&self.conti, &self.sfc.stock);
        ok(data.anno <= 2200, "data.anno oltre il 2200")?;
        ok(
            (1..=8).contains(&self.storico.len()) && self.storico.iter().all(|t| t.pil > 0),
            "economia.storico: servono da 1 a 8 voci con pil > 0",
        )?;
        ok(
            !self.pensioni.is_empty()
                && self
                    .pensioni
                    .iter()
                    .all(|a| a.pensionati >= 0 && a.importo >= 0),
            "economia.pensioni: vuote o con valori negativi",
        )?;
        ok(
            self.sussidio_per_disoccupato >= 0,
            "economia.sussidio_per_disoccupato negativo",
        )?;
        ok(
            self.tasso_medio_debito.is_finite(),
            "economia.tasso_medio_debito non finito",
        )?;
        ok(
            self.sfc.livello_prezzi_area.is_finite() && self.sfc.livello_prezzi_area > 0.0,
            "economia.sfc.livello_prezzi_area non finito o non positivo",
        )?;
        let importi = [
            self.debito,
            self.sussidio_per_disoccupato,
            c.entrate.irpef,
            c.entrate.ires,
            c.entrate.iva,
            c.entrate.contributi,
            c.entrate.altre,
            c.entrate.fondi_coesione,
            c.spese.dicasteri,
            c.spese.pensioni,
            c.spese.sussidi,
            c.spese.altre_prestazioni,
            c.spese.interessi,
            c.spese.ammende,
            st.famiglie,
            st.imprese,
            st.stato,
            st.estero,
        ];
        let importi_bilancio = bilanci
            .flat_map(|b| &b.dicasteri)
            .flat_map(|d| [d.spesa, d.salari, d.acquisti, d.margine]);
        ok(
            importi.into_iter().chain(importi_bilancio).all(in_limite),
            "economia: un importo supera 1e15 euro",
        )?;
        ok(
            st.stato == -self.debito,
            "economia.sfc.stock.stato diverso dal debito cambiato di segno",
        )?;
        ok(
            self.residuo_stock() == 0,
            "economia.sfc.stock non somma a zero",
        )?;
        ok(
            self.disavanzo_trimestre == c.disavanzo(),
            "economia.disavanzo_trimestre diverso da quello dei conti",
        )
    }
}
