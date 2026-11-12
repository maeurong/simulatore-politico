//! Le voci dell'esito scritte dalla partita, nel punto del fatto.

use super::Partita;
use super::vita::virgola;
use crate::disposizioni::data_di;
use crate::esito::{Grado, TipoVoce, VoceEsito, grado};
use crate::eventi::{Bersaglio, DatoOpzione, EventoAperto, emergenze};
use crate::legislatura::campagna::migliaia;
use crate::ministri::{FattoAtto, Proposta};
use crate::parlamento;

/// «N euro dal fondo di riserva (Q% del costo)» o «N euro a debito».
pub(super) fn pagamento(costo: i64, quota: f64, dal_fondo: bool) -> String {
    let pagato = (quota * costo as f64).round() as i64;
    if dal_fondo {
        format!(
            "{} euro dal fondo di riserva ({:.0}% del costo)",
            migliaia(pagato),
            quota * 100.0
        )
    } else {
        format!("{} euro a debito", migliaia(pagato))
    }
}

impl Partita {
    /// La voce nasce col grado del catalogo per chi governa ora; `None` se il catalogo dice «–».
    /// «Al governo» è `al_governo`: Presidente del Consiglio, anche da uscente in trattativa,
    /// perché i suoi ministri eseguono ancora (Scelta 22 della spec); la fermata sul governo
    /// guarda invece `governa`.
    pub(super) fn voce(
        &mut self,
        tipo: TipoVoce,
        fatto: String,
        causa: String,
    ) -> Option<&mut VoceEsito> {
        let g = grado(tipo, self.al_governo())?;
        self.esito.push(VoceEsito::nuova(tipo, g, fatto, causa));
        self.esito.last_mut()
    }

    /// Come `voce`, col partito di indice `j` nel campo `partito`: l'ingresso delle 25 ex mosse.
    /// Nessuna voce se `j` non è un partito.
    pub(super) fn voce_di(
        &mut self,
        j: usize,
        tipo: TipoVoce,
        fatto: String,
        causa: String,
    ) -> Option<&mut VoceEsito> {
        let id = self.partiti.get(j)?.id.clone();
        let v = self.voce(tipo, fatto, causa)?;
        v.partito = Some(id);
        Some(v)
    }

    /// Il giocatore è Presidente del Consiglio (Scelta 7).
    pub(super) fn al_governo(&self) -> bool {
        self.governo.presidente == self.giocatore
    }

    /// « in {regione}», o niente.
    fn in_regione(&self, r: Option<usize>) -> String {
        (r.and_then(|r| self.dati.regioni().get(r)))
            .map_or(String::new(), |r| format!(" in {}", r.nome))
    }

    fn id_regione(&self, r: Option<usize>) -> Option<String> {
        r.and_then(|r| self.dati.regioni().get(r))
            .map(|r| r.id.clone())
    }

    /// Fatto 2: l'emergenza lieve del turno prima, pagata dal fondo per la `quota`.
    pub(super) fn voce_lieve(&mut self, aperto: &EventoAperto, quota: f64) {
        let Some(e) = self.dati.eventi().get(aperto.evento) else {
            return;
        };
        let fatto = format!(
            "{} lieve{}, {}",
            e.nome,
            self.in_regione(aperto.regione),
            pagamento(aperto.costo, quota, true)
        );
        let tipo = emergenze::EVENTI.iter().position(|id| *id == e.id);
        let causa = match (tipo, aperto.regione) {
            (Some(3), _) => match data_di(aperto.turno).trimestre {
                2 => "la stagione secca di primavera".to_string(),
                3 => "la stagione secca d'estate".to_string(),
                _ => "la stagione secca".to_string(),
            },
            (Some(t), Some(r)) => {
                let quale = ["sismica", "idrogeologica", "d'incendio"].get(t).copied();
                let indice = emergenze::indice_pericolosita(&self.dati, t, r);
                format!(
                    "pericolosità {} {}",
                    quale.unwrap_or(""),
                    emergenze::a_parole(indice)
                )
            }
            _ => "emergenza naturale".to_string(),
        };
        let (regione, tono) = (self.id_regione(aperto.regione), aperto.tono);
        if let Some(v) = self.voce(TipoVoce::EmergenzaLieve, fatto, causa) {
            v.regione = regione;
            v.tono = tono;
        }
    }

    /// Fatti 3, 4, 5, 9, 10, 11: l'evento risolto con l'`opzione`. `coda` dice il seguito (costo,
    /// esito dello scandalo); la valence di `colpito` era `prima` all'inizio della risoluzione.
    pub(super) fn voce_evento(
        &mut self,
        aperto: &EventoAperto,
        opzione: &DatoOpzione,
        mut coda: Vec<String>,
        colpito: usize,
        prima: f64,
    ) {
        let Some(e) = self.dati.eventi().get(aperto.evento) else {
            return;
        };
        let (nome, famiglia) = (e.nome.clone(), e.famiglia);
        let persona = (aperto.persona.clone())
            .or_else(|| aperto.bersaglio.and_then(|b| self.nome_del(b)))
            .map_or(String::new(), |p| format!(" ({p})"));
        let per_default = aperto.scelta.is_none();
        let mut fatto = format!(
            "{nome}{}{persona}, «{}»{}",
            self.in_regione(aperto.regione),
            opzione.nome,
            if per_default { " per default" } else { "" }
        );
        let partito = self.partiti.get(colpito);
        let delta = partito.map_or(0.0, |p| p.valence - prima);
        if let Some(p) = partito.filter(|_| delta != 0.0) {
            let segno = format!("{delta:+.2}").replace('.', ",");
            coda.push(format!("valence {segno} a {}", p.nome));
        }
        for parte in coda.iter().filter(|c| !c.is_empty()) {
            fatto.push_str("; ");
            fatto.push_str(parte);
        }
        let causa = aperto.causa.testo(&self.dati);
        let dicastero = match aperto.bersaglio {
            Some(Bersaglio::Ministro { dicastero }) => {
                self.dati.dicasteri().get(dicastero).map(|d| d.id.clone())
            }
            _ => None,
        };
        let id_partito = (aperto.bersaglio.is_some())
            .then(|| self.partiti.get(colpito).map(|p| p.id.clone()))
            .flatten();
        let (regione, al_governo) = (self.id_regione(aperto.regione), self.al_governo());
        let Some(v) = self.voce(TipoVoce::EventoRisolto, fatto, causa) else {
            return;
        };
        if per_default && al_governo {
            v.con_grado(Grado::C1);
        }
        v.tono = aperto.tono;
        v.famiglia = Some(famiglia);
        v.regione = regione;
        v.dicastero = dicastero;
        v.partito = id_partito;
    }

    /// Fatti 6, 7, 8: ciò che l'evento ha avviato (programma del MES, ristrutturazione col
    /// `taglio` in euro, margine UE per l'energia).
    pub(super) fn voce_seguito(&mut self, tipo: TipoVoce, aperto: &EventoAperto, taglio: i64) {
        let evento = (self.dati.eventi().get(aperto.evento)).map_or("", |e| e.nome.as_str());
        let scelta = format!("la scelta nelle «{evento}»");
        let m = |nome: &str| self.dati.manopola(nome);
        let (fatto, causa) = match tipo {
            TipoVoce::MesAvviato => {
                let fine = self.economia.programma_mes.unwrap_or(self.turno);
                let vincoli = parlamento::nome(&self.dati, &self.condizionalita());
                let mut fatto = format!(
                    "Programma del MES di {} trimestri, finisce a {}; vincolati {vincoli}",
                    m("durata_mes").round(),
                    parlamento::mese(data_di(fine))
                );
                // All'opposizione l'intesa degli alleati del governo avversario non si vede.
                if self.al_governo() {
                    let intesa = virgola(m("intesa_mes"));
                    fatto.push_str(&format!("; ogni alleato perde {intesa} punti d'intesa"));
                }
                let causa = format!("{scelta}, {}", aperto.causa.testo(&self.dati));
                (fatto, causa)
            }
            TipoVoce::Ristrutturazione => (
                format!(
                    "Debito tagliato del {:.0}%, {} euro; premio di {:.0} pb sullo spread; \
                     redditi primari delle fasce media e alta giù del {:.0}% per {:.0} turni",
                    m("taglio_ristrutturazione") * 100.0,
                    migliaia(taglio),
                    m("premio_ristrutturazione"),
                    m("perdita_ristrutturazione") * 100.0,
                    m("turni_ristrutturazione").round()
                ),
                scelta,
            ),
            TipoVoce::MargineEnergia => (
                format!("Margine UE per l'energia chiesto per il {}", self.data.anno),
                format!("la scelta nel «{evento}»"),
            ),
            _ => return,
        };
        if let Some(v) = self.voce(tipo, fatto, causa) {
            v.tono = aperto.tono;
        }
    }

    /// Fatti 12 e 13: le proposte tolte da `decidi_proposte`, respinte per silenzio o decadute.
    pub(super) fn voci_proposte(&mut self, tolte: Vec<(usize, Proposta, Option<&'static str>)>) {
        let costo = virgola(self.dati.manopola("costo_respingere"));
        for (i, p, decaduta) in tolte {
            let Some(d) = self.dati.dicasteri().get(i) else {
                continue;
            };
            let ministro = (self.ministri.dicasteri.get(i))
                .and_then(|st| st.ministro.as_ref())
                .map_or("", |m| m.nome.as_str());
            let chi = format!("{ministro} ({})", d.nome);
            let (id, testo) = (d.id.clone(), p.tipo.testo(&self.dati));
            let (tipo, fatto, causa) = match decaduta {
                None => (
                    TipoVoce::PropostaRespinta,
                    format!(
                        "Respinta la proposta di {chi}, {testo}; perde {costo} punti di soddisfazione"
                    ),
                    format!(
                        "in sospeso da {} turni senza risposta",
                        p.turni_in_sospeso + 1
                    ),
                ),
                Some(motivo) => (
                    TipoVoce::PropostaDecaduta,
                    format!("Decade senza costo la proposta di {chi}, {testo}"),
                    motivo.to_string(),
                ),
            };
            if let Some(v) = self.voce(tipo, fatto, causa) {
                v.dicastero = Some(id);
            }
        }
    }

    /// Fatto 14: l'emendamento dell'alleato `j` alla `misura` respinto per default, con l'intesa
    /// persa (`costo`).
    pub(super) fn voce_emendamento_respinto(&mut self, j: usize, misura: u32, costo: f64) {
        let Some(m) = self.parlamento.misure.iter().find(|m| m.id == misura) else {
            return;
        };
        let legge = parlamento::nome_misura(&self.dati, m);
        let disposizioni: Vec<String> = (m.valori.iter())
            .filter_map(|&(k, _)| self.dati.disposizioni().get(k).map(|x| x.id.clone()))
            .collect();
        let nome = self.partiti.get(j).map_or("", |p| p.nome.as_str());
        let fatto = format!(
            "Respinto l'emendamento di {nome} su {legge}; perde {} punti d'intesa",
            virgola(costo)
        );
        let causa = "senza risposta prima del voto".to_string();
        if let Some(v) = self.voce_di(j, TipoVoce::EmendamentoRespinto, fatto, causa) {
            v.misura = Some(misura);
            v.disposizioni = disposizioni;
        }
    }

    /// Fatti 16-20: un atto dei ministri eseguito, saltato, o un cantiere avviato, fermo, finito.
    pub(super) fn voce_atto(&mut self, f: FattoAtto) {
        let Some(d) = self.dati.dicasteri().get(f.dicastero) else {
            return;
        };
        let (dicastero, atto, euro) = (&d.nome, &f.atto, migliaia(f.euro));
        let dove = self.in_regione(f.regione);
        let fatto = match f.tipo {
            TipoVoce::AttoEseguito => {
                format!("Il dicastero {dicastero} esegue «{atto}»{dove} con {euro} euro")
            }
            TipoVoce::AttoSaltato => {
                format!("Il dicastero {dicastero} non esegue «{atto}»{dove}, annunciato in agenda")
            }
            TipoVoce::CantiereAvviato => format!(
                "Il dicastero {dicastero} avvia il cantiere «{atto}»{dove}, {euro} euro a turno \
                 per {} turni",
                f.turni
            ),
            TipoVoce::CantiereFermo => {
                format!("Il cantiere «{atto}»{dove} del dicastero {dicastero} resta fermo un turno")
            }
            TipoVoce::CantiereFinito => format!(
                "Finito il cantiere «{atto}»{dove} del dicastero {dicastero}, {euro} euro spesi \
                 in tutto; gli effetti partono"
            ),
            _ => return,
        };
        let id = d.id.clone();
        // Il motivo dell'agenda porta «: », che nel dettaglio di una voce fusa separa il fatto
        // dalla causa.
        let causa = f.motivo.replace(": ", ", ");
        let regione = self.id_regione(f.regione);
        if let Some(v) = self.voce(f.tipo, fatto, causa) {
            v.dicastero = Some(id);
            v.regione = regione;
        }
    }
}
