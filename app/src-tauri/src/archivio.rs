//! L'archivio su disco: una cartella per partita (`partite/<id>/`) con l'indice in chiaro, uno
//! snapshot gzip per apertura di turno, mai riscritto, e la bozza del turno in corso; l'albo delle
//! partite finite (`albo/<id>.json`) e gli scenari su misura (`scenari/<nome>.toml`). Ogni file si
//! scrive in modo atomico (ricerca persistenza-dei-salvataggi:51).

use std::cmp::Reverse;
use std::fs::{self, File};
use std::io::{ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use sim_core::salvataggi::{FORMATO_ALBO, FORMATO_INDICE, IndicePartita, VoceAlbo, VocePartita};
use sim_core::{DatiGioco, Errore, Partita, SuMisura, VERSIONE_SCHEMA};

/// Tetto di ogni lettura da disco, dopo la decompressione.
const TETTO: u64 = 64 * 1024 * 1024;
const INDICE: &str = "indice.json";
const BOZZA: &str = "bozza.json.gz";

fn nome_turno(turno: u32) -> String {
    format!("turno-{turno:03}.json.gz")
}

/// Il turno di un nome `turno-NNN.json.gz`; nessuno per i `.tmp` e per gli altri file.
fn turno_del_nome(nome: &str) -> Option<u32> {
    let cifre = nome.strip_prefix("turno-")?.strip_suffix(".json.gz")?;
    if cifre.bytes().all(|b| b.is_ascii_digit()) {
        cifre.parse().ok()
    } else {
        None
    }
}

fn adesso() -> Result<u64, String> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .map_err(|e| format!("orologio di sistema non valido: {e}"))
}

fn errore_lettura(percorso: &Path, e: &std::io::Error) -> String {
    format!("impossibile leggere {}: {e}", percorso.display())
}

/// Al più `TETTO` byte da `lettore`, che legge `percorso`.
fn leggi_da(lettore: impl Read, percorso: &Path) -> Result<String, String> {
    let mut byte = Vec::new();
    (lettore.take(TETTO + 1).read_to_end(&mut byte)).map_err(|e| errore_lettura(percorso, &e))?;
    if byte.len() as u64 > TETTO {
        return Err(format!("{} supera 64 MiB", percorso.display()));
    }
    String::from_utf8(byte).map_err(|e| format!("{} non è testo UTF-8: {e}", percorso.display()))
}

fn apri(percorso: &Path) -> Result<File, String> {
    File::open(percorso).map_err(|e| errore_lettura(percorso, &e))
}

fn leggi(percorso: &Path) -> Result<String, String> {
    leggi_da(apri(percorso)?, percorso)
}

/// Scrittura atomica: temporaneo nella stessa cartella, `sync_all`, `rename`, `sync_all` della
/// cartella. A un crash resta il file vecchio o quello nuovo, più al massimo un `.tmp` orfano.
fn scrivi(percorso: &Path, byte: &[u8]) -> Result<(), String> {
    let errore = |e: std::io::Error| format!("impossibile scrivere {}: {e}", percorso.display());
    let mut tmp = percorso.as_os_str().to_owned();
    tmp.push(".tmp");
    let tmp = PathBuf::from(tmp);
    let mut f = File::create(&tmp).map_err(errore)?;
    f.write_all(byte).map_err(errore)?;
    f.sync_all().map_err(errore)?;
    fs::rename(&tmp, percorso).map_err(errore)?;
    match percorso.parent() {
        Some(cartella) => File::open(cartella)
            .and_then(|c| c.sync_all())
            .map_err(errore),
        None => Ok(()),
    }
}

/// Cancella `percorso`; un file che già manca va bene.
fn togli(percorso: &Path) -> Result<(), String> {
    match fs::remove_file(percorso) {
        Err(e) if e.kind() != ErrorKind::NotFound => Err(format!(
            "impossibile cancellare {}: {e}",
            percorso.display()
        )),
        _ => Ok(()),
    }
}

fn scrivi_partita(percorso: &Path, p: &Partita) -> Result<(), String> {
    let errore =
        |e: &dyn std::fmt::Display| format!("impossibile salvare {}: {e}", percorso.display());
    let json = p.salva().map_err(|e| errore(&e))?;
    let mut gz = GzEncoder::new(Vec::new(), Compression::default());
    gz.write_all(json.as_bytes()).map_err(|e| errore(&e))?;
    let byte = gz.finish().map_err(|e| errore(&e))?;
    scrivi(percorso, &byte)
}

fn leggi_partita(percorso: &Path) -> Result<Partita, String> {
    let json = leggi_da(GzDecoder::new(apri(percorso)?), percorso)?;
    Partita::carica(&json).map_err(|e| format!("impossibile aprire {}: {e}", percorso.display()))
}

/// L'indice, solo del formato di questa versione: uno d'altro formato non si riscrive come
/// `FORMATO_INDICE`.
fn leggi_indice(cartella: &Path) -> Result<IndicePartita, String> {
    let percorso = cartella.join(INDICE);
    let indice: IndicePartita = serde_json::from_str(&leggi(&percorso)?)
        .map_err(|e| format!("indice illeggibile {}: {e}", percorso.display()))?;
    if indice.formato_indice != FORMATO_INDICE {
        return Err(format!(
            "{}: indice di formato {}, questa versione legge il formato {FORMATO_INDICE}",
            percorso.display(),
            indice.formato_indice
        ));
    }
    Ok(indice)
}

/// `fsync` della cartella, perché una voce creata o cancellata resti su disco.
fn sincronizza(cartella: &Path) -> Result<(), String> {
    File::open(cartella)
        .and_then(|c| c.sync_all())
        .map_err(|e| format!("impossibile sincronizzare {}: {e}", cartella.display()))
}

fn motivo(trovata: u32) -> String {
    Errore::SchemaDiverso {
        trovata,
        attesa: VERSIONE_SCHEMA,
    }
    .to_string()
}

/// L'indice, se lo schema è quello di questa versione: un salvataggio d'altro schema non si apre
/// e nessuno dei suoi file si tocca.
fn indice_apribile(cartella: &Path) -> Result<IndicePartita, String> {
    let indice = leggi_indice(cartella)?;
    match indice.testata.versione_schema {
        VERSIONE_SCHEMA => Ok(indice),
        trovata => Err(format!(
            "{}: {}",
            cartella.join(INDICE).display(),
            motivo(trovata)
        )),
    }
}

fn scrivi_indice(
    cartella: &Path,
    id: &str,
    p: &Partita,
    riavvolgimenti: u32,
) -> Result<(), String> {
    let percorso = cartella.join(INDICE);
    let indice = IndicePartita {
        formato_indice: FORMATO_INDICE,
        id: id.into(),
        testata: p.testata(),
        ultimo_salvataggio: adesso()?,
        riavvolgimenti,
    };
    let json = serde_json::to_string_pretty(&indice)
        .map_err(|e| format!("impossibile scrivere {}: {e}", percorso.display()))?;
    scrivi(&percorso, json.as_bytes())
}

/// I turni degli snapshot di `cartella`, dal più recente.
fn turni(cartella: &Path) -> Result<Vec<u32>, String> {
    let voci = fs::read_dir(cartella).map_err(|e| errore_lettura(cartella, &e))?;
    let mut turni: Vec<u32> = (voci.filter_map(Result::ok))
        .filter_map(|v| turno_del_nome(&v.file_name().to_string_lossy()))
        .collect();
    turni.sort_unstable_by_key(|&t| Reverse(t));
    Ok(turni)
}

pub(crate) struct Archivio {
    radice: PathBuf,
}

impl Archivio {
    pub(crate) fn nuovo(radice: PathBuf) -> Result<Archivio, String> {
        for sotto in ["partite", "albo", "scenari"] {
            let c = radice.join(sotto);
            fs::create_dir_all(&c)
                .map_err(|e| format!("impossibile creare {}: {e}", c.display()))?;
        }
        Ok(Archivio { radice })
    }

    /// Un nome che resta un solo file dentro la sua cartella: 1-60 caratteri, niente `/`, `\`,
    /// `..`, punto iniziale, caratteri di controllo.
    pub(crate) fn valida_nome(nome: &str) -> Result<(), String> {
        let lunghezza = nome.chars().count();
        if !(1..=60).contains(&lunghezza) {
            return Err(format!(
                "nome {nome:?} di {lunghezza} caratteri: ne servono da 1 a 60"
            ));
        }
        if nome.starts_with('.')
            || nome.contains(['/', '\\'])
            || nome.contains("..")
            || nome.chars().any(char::is_control)
        {
            return Err(format!(
                "nome {nome:?} non valido: niente «/», «\\», «..», punto iniziale o caratteri di controllo"
            ));
        }
        Ok(())
    }

    fn cartella(&self, id: &str) -> Result<PathBuf, String> {
        Self::valida_nome(id)?;
        Ok(self.radice.join("partite").join(id))
    }

    /// I nomi validi dei file di `sotto` che finiscono in `estensione`, senza l'estensione, in
    /// ordine. I `.tmp` non finiscono in `estensione`.
    fn nomi(&self, sotto: &str, estensione: &str) -> Vec<String> {
        let Ok(voci) = fs::read_dir(self.radice.join(sotto)) else {
            return Vec::new();
        };
        let mut nomi: Vec<String> = (voci.filter_map(Result::ok))
            .filter_map(|v| {
                let nome = v.file_name().to_string_lossy().into_owned();
                nome.strip_suffix(estensione).map(str::to_owned)
            })
            .filter(|n| Self::valida_nome(n).is_ok())
            .collect();
        nomi.sort();
        nomi
    }

    /// La cartella della partita, il suo primo snapshot e l'indice. Una cartella che esiste già
    /// è un errore: nessuna partita si sovrascrive.
    pub(crate) fn crea(&self, p: &Partita, id: &str) -> Result<(), String> {
        let cartella = self.cartella(id)?;
        fs::create_dir(&cartella)
            .map_err(|e| format!("impossibile creare {}: {e}", cartella.display()))?;
        sincronizza(&self.radice.join("partite"))?;
        scrivi_partita(&cartella.join(nome_turno(p.testata().turno)), p)?;
        scrivi_indice(&cartella, id, p, p.riavvolgimenti())
    }

    /// Lo snapshot dell'apertura del turno, poi via la bozza e l'indice aggiornato. Il contatore
    /// dei riavvolgimenti resta quello dell'indice.
    pub(crate) fn snapshot(&self, id: &str, p: &Partita) -> Result<(), String> {
        let cartella = self.cartella(id)?;
        let indice = leggi_indice(&cartella)?;
        scrivi_partita(&cartella.join(nome_turno(p.testata().turno)), p)?;
        togli(&cartella.join(BOZZA))?;
        scrivi_indice(&cartella, id, p, indice.riavvolgimenti)
    }

    pub(crate) fn bozza(&self, id: &str, p: &Partita) -> Result<(), String> {
        scrivi_partita(&self.cartella(id)?.join(BOZZA), p)
    }

    /// La bozza, se si legge e il suo turno è quello dell'ultimo snapshot leggibile; altrimenti
    /// quello snapshot (ricerca persistenza-dei-salvataggi:115). Si cancella solo la bozza che si
    /// legge ed è d'un altro turno. Deviazione voluta dalla ricerca: la bozza illeggibile resta,
    /// così un errore di I/O passeggero non cancella le scelte, e la sostituiranno `snapshot` o
    /// `bozza`. Il contatore dei riavvolgimenti è sempre quello dell'indice.
    pub(crate) fn riprendi(&self, id: &str) -> Result<Partita, String> {
        let cartella = self.cartella(id)?;
        let indice = indice_apribile(&cartella)?;
        let snapshot = (turni(&cartella)?.into_iter())
            .find_map(|t| leggi_partita(&cartella.join(nome_turno(t))).ok())
            .ok_or_else(|| format!("nessuno snapshot leggibile in {}", cartella.display()))?;
        let bozza = cartella.join(BOZZA);
        let mut p = match leggi_partita(&bozza) {
            Ok(b) if b.testata().turno == snapshot.testata().turno => b,
            Ok(_) => {
                togli(&bozza)?;
                snapshot
            }
            Err(_) => snapshot,
        };
        p.imposta_riavvolgimenti(indice.riavvolgimenti);
        Ok(p)
    }

    /// Torna all'apertura del turno `turno`: via la bozza e gli snapshot successivi, dal più
    /// recente, poi il contatore cresce di uno nell'indice. Un crash a metà riapre la partita a
    /// un turno fra `turno` e quello di prima, e si può ripetere. Limite: se cade fra le
    /// cancellazioni e la scrittura dell'indice, il riavvolgimento non si conta e l'indice resta
    /// al turno vecchio fino al prossimo snapshot.
    pub(crate) fn riavvolgi(&self, id: &str, turno: u32) -> Result<Partita, String> {
        let cartella = self.cartella(id)?;
        let indice = indice_apribile(&cartella)?;
        let percorso = cartella.join(nome_turno(turno));
        if turno > indice.testata.turno {
            return Err(format!(
                "impossibile riavvolgere a {}: la partita è al turno {}",
                percorso.display(),
                indice.testata.turno
            ));
        }
        let mut p = leggi_partita(&percorso)?;
        togli(&cartella.join(BOZZA))?;
        for m in turni(&cartella)?.into_iter().filter(|&m| m > turno) {
            togli(&cartella.join(nome_turno(m)))?;
        }
        let riavvolgimenti = indice.riavvolgimenti.saturating_add(1);
        p.imposta_riavvolgimenti(riavvolgimenti);
        scrivi_indice(&cartella, id, &p, riavvolgimenti)?;
        Ok(p)
    }

    /// Le partite in corso, dalla più recente: legge solo gli indici. Salta senza fermarsi le
    /// cartelle senza indice, con un indice illeggibile o d'altro formato, o con un `id` che non
    /// è il nome della cartella o non passa `valida_nome`.
    pub(crate) fn elenca(&self) -> Vec<VocePartita> {
        let Ok(voci) = fs::read_dir(self.radice.join("partite")) else {
            return Vec::new();
        };
        let mut elenco: Vec<VocePartita> = (voci.filter_map(Result::ok))
            .filter_map(|v| {
                let indice = leggi_indice(&v.path()).ok()?;
                let coincide = v.file_name() == indice.id.as_str();
                (coincide && Self::valida_nome(&indice.id).is_ok()).then_some(indice)
            })
            .map(|indice| {
                let trovata = indice.testata.versione_schema;
                VocePartita {
                    apribile: trovata == VERSIONE_SCHEMA,
                    motivo: (trovata != VERSIONE_SCHEMA).then(|| motivo(trovata)),
                    indice,
                }
            })
            .collect();
        elenco.sort_by_key(|v| Reverse(v.indice.ultimo_salvataggio));
        elenco
    }

    /// `albo/<id>.json` in chiaro, poi la cartella della partita si cancella. Ripetibile dopo un
    /// crash fra i due passi: la cartella che manca nel frattempo va bene. Senza la cartella
    /// all'inizio, invece, è un errore e l'albo non si scrive.
    pub(crate) fn chiudi_nell_albo(&self, id: &str, v: &VoceAlbo) -> Result<(), String> {
        let cartella = self.cartella(id)?;
        if !cartella.is_dir() {
            return Err(format!(
                "impossibile chiudere nell'albo: {} non esiste",
                cartella.display()
            ));
        }
        let percorso = self.radice.join("albo").join(format!("{id}.json"));
        let json = serde_json::to_string_pretty(v)
            .map_err(|e| format!("impossibile scrivere {}: {e}", percorso.display()))?;
        scrivi(&percorso, json.as_bytes())?;
        match fs::remove_dir_all(&cartella) {
            Err(e) if e.kind() != ErrorKind::NotFound => {
                return Err(format!(
                    "impossibile cancellare {}: {e}",
                    cartella.display()
                ));
            }
            _ => {}
        }
        sincronizza(&self.radice.join("partite"))
    }

    /// Le voci dell'albo, in ordine di id. Salta senza fermarsi i file che non si leggono e
    /// quelli d'altro formato.
    pub(crate) fn albo(&self) -> Vec<VoceAlbo> {
        let cartella = self.radice.join("albo");
        (self.nomi("albo", ".json").into_iter())
            .filter_map(|id| leggi(&cartella.join(format!("{id}.json"))).ok())
            .filter_map(|testo| serde_json::from_str::<VoceAlbo>(&testo).ok())
            .filter(|v| v.formato_albo == FORMATO_ALBO)
            .collect()
    }

    /// I nomi degli scenari su misura salvati, in ordine.
    pub(crate) fn scenari(&self) -> Vec<String> {
        self.nomi("scenari", ".toml")
    }

    fn percorso_scenario(&self, nome: &str) -> Result<PathBuf, String> {
        Self::valida_nome(nome)?;
        Ok(self.radice.join("scenari").join(format!("{nome}.toml")))
    }

    /// `scenari/<nome>.toml` col file di differenze dalla base. Lo stesso nome lo sovrascrive;
    /// un nome che differisce da uno presente solo per le maiuscole si rifiuta, perché su APFS
    /// sarebbe lo stesso file.
    pub(crate) fn salva_scenario(&self, s: &SuMisura, base: &DatiGioco) -> Result<(), String> {
        let percorso = self.percorso_scenario(&s.nome)?;
        let minuscolo = s.nome.to_lowercase();
        if let Some(altro) =
            (self.scenari().into_iter()).find(|n| *n != s.nome && n.to_lowercase() == minuscolo)
        {
            return Err(format!(
                "impossibile salvare {}: c'è già lo scenario «{altro}», che differisce solo per le maiuscole",
                percorso.display()
            ));
        }
        scrivi(&percorso, s.in_toml(base).as_bytes())
    }

    pub(crate) fn carica_scenario(&self, nome: &str, base: &DatiGioco) -> Result<SuMisura, String> {
        let percorso = self.percorso_scenario(nome)?;
        SuMisura::da_toml(&leggi(&percorso)?, base)
            .map_err(|e| format!("{}: {e}", percorso.display()))
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use sim_core::legislatura::Fondazione;
    use sim_core::salvataggi::FORMATO_ALBO;

    /// Cartella temporanea del test, cancellata anche se il test fallisce.
    pub(crate) struct Cartella(pub(crate) PathBuf);

    impl Drop for Cartella {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    pub(crate) fn cartella(test: &str) -> Cartella {
        let c = std::env::temp_dir().join(format!("sim-{test}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&c);
        Cartella(c)
    }

    fn dati() -> DatiGioco {
        crate::carica_dati(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../dati/base")).unwrap()
    }

    const ID: &str = "00000000000000a1";

    fn fondata(seme: u64) -> Partita {
        let f = Fondazione {
            nome: "Partito nuovo".into(),
            posizione: 3.5,
        };
        Partita::fonda(dati(), seme, &f).unwrap()
    }

    /// Archivio con una partita fondata e il suo `turno-000`.
    fn con_partita(c: &Cartella) -> (Archivio, Partita) {
        let a = Archivio::nuovo(c.0.clone()).unwrap();
        let p = fondata(7);
        a.crea(&p, ID).unwrap();
        (a, p)
    }

    fn chiudi(a: &Archivio, p: &mut Partita) {
        p.chiudi_turno();
        a.snapshot(ID, p).unwrap();
    }

    fn file(c: &Cartella, nome: &str) -> PathBuf {
        c.0.join("partite").join(ID).join(nome)
    }

    fn nomi(c: &Cartella) -> Vec<String> {
        let mut v: Vec<String> = fs::read_dir(c.0.join("partite").join(ID))
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        v.sort();
        v
    }

    fn indice(c: &Cartella) -> IndicePartita {
        serde_json::from_str(&fs::read_to_string(file(c, "indice.json")).unwrap()).unwrap()
    }

    #[test]
    fn crea_snapshot_riprendi() {
        let c = cartella("crea_snapshot_riprendi");
        let (a, mut p) = con_partita(&c);
        chiudi(&a, &mut p);
        chiudi(&a, &mut p);
        assert_eq!(a.riprendi(ID).unwrap(), p);
        assert_eq!(indice(&c).testata, p.testata());
        assert_eq!(indice(&c).id, ID);

        // Una bozza dello stesso turno vince sullo snapshot.
        let mut altra = fondata(8);
        altra.chiudi_turno();
        altra.chiudi_turno();
        a.bozza(ID, &altra).unwrap();
        assert_eq!(a.riprendi(ID).unwrap(), altra);
        assert!(file(&c, "bozza.json.gz").exists());
    }

    #[test]
    fn riavvolgi_cancella_il_futuro_e_conta() {
        let c = cartella("riavvolgi_cancella_il_futuro_e_conta");
        let (a, mut p) = con_partita(&c);
        let mut al_primo = None;
        for _ in 0..3 {
            chiudi(&a, &mut p);
            al_primo.get_or_insert_with(|| p.salva().unwrap());
        }
        a.bozza(ID, &p).unwrap();
        let r = a.riavvolgi(ID, 1).unwrap();
        assert_eq!(
            nomi(&c),
            ["indice.json", "turno-000.json.gz", "turno-001.json.gz"]
        );
        assert_eq!(indice(&c).riavvolgimenti, 1);
        assert_eq!(indice(&c).testata.turno, 1);
        assert_eq!(r.riavvolgimenti(), 1);
        let mut atteso = Partita::carica(&al_primo.unwrap()).unwrap();
        atteso.imposta_riavvolgimenti(1);
        assert_eq!(r, atteso);
        assert_eq!(a.riprendi(ID).unwrap().riavvolgimenti(), 1);

        let e = a.riavvolgi(ID, 5).unwrap_err();
        assert!(e.contains("turno-005.json.gz"), "{e}");
        assert_eq!(indice(&c).riavvolgimenti, 1);
    }

    #[test]
    fn bozza_rovinata_ripiega_sullo_snapshot() {
        let c = cartella("bozza_rovinata_ripiega_sullo_snapshot");
        let (a, mut p) = con_partita(&c);
        chiudi(&a, &mut p);

        // Bozza troncata a metà e un temporaneo orfano.
        a.bozza(ID, &p).unwrap();
        let bozza = fs::read(file(&c, "bozza.json.gz")).unwrap();
        fs::write(file(&c, "bozza.json.gz"), &bozza[..bozza.len() / 2]).unwrap();
        fs::write(file(&c, "turno-002.json.gz.tmp"), b"mezzo file").unwrap();
        assert_eq!(a.riprendi(ID).unwrap(), p);
        // La bozza illeggibile resta: la sostituiranno `snapshot` o `bozza`.
        assert_eq!(
            fs::read(file(&c, "bozza.json.gz")).unwrap(),
            &bozza[..bozza.len() / 2]
        );
        assert_eq!(a.elenca().len(), 1);

        // Bozza vecchia accanto a uno snapshot più nuovo: crash fra la scrittura di
        // `turno-002` e la cancellazione della bozza del turno 1.
        chiudi(&a, &mut p);
        fs::write(file(&c, "bozza.json.gz"), &bozza).unwrap();
        assert_eq!(a.riprendi(ID).unwrap(), p);
        assert!(!file(&c, "bozza.json.gz").exists());

        // Snapshot più recente illeggibile: vale l'ultimo che si legge.
        let turno_due = fs::read(file(&c, "turno-002.json.gz")).unwrap();
        fs::write(file(&c, "turno-002.json.gz"), &turno_due[..10]).unwrap();
        assert_eq!(a.riprendi(ID).unwrap().testata().turno, 1);
    }

    #[test]
    fn schema_diverso_elencato_e_intatto() {
        let c = cartella("schema_diverso_elencato_e_intatto");
        let (a, mut p) = con_partita(&c);
        chiudi(&a, &mut p);
        a.bozza(ID, &p).unwrap();
        let mut i = indice(&c);
        i.testata.versione_schema = 10;
        fs::write(file(&c, "indice.json"), serde_json::to_string(&i).unwrap()).unwrap();

        // Non si elencano: una cartella senza indice, un indice d'altro formato, un `id` che non
        // è il nome della cartella, un `id` che `valida_nome` rifiuta.
        fs::create_dir_all(c.0.join("partite/senza-indice")).unwrap();
        let altro = |cartella: &str, id: &str, formato: u32| {
            let mut j = serde_json::to_value(&i).unwrap();
            j["id"] = id.into();
            j["formato_indice"] = formato.into();
            fs::create_dir_all(c.0.join("partite").join(cartella)).unwrap();
            fs::write(
                c.0.join("partite").join(cartella).join(INDICE),
                j.to_string(),
            )
            .unwrap();
        };
        altro("futuro", "futuro", FORMATO_INDICE + 1);
        altro("cartella", "altro-id", FORMATO_INDICE);
        altro(".nascosta", ".nascosta", FORMATO_INDICE);

        let prima: Vec<(String, Vec<u8>)> = nomi(&c)
            .into_iter()
            .map(|n| (n.clone(), fs::read(file(&c, &n)).unwrap()))
            .collect();
        let elenco = a.elenca();
        assert_eq!(elenco.len(), 1);
        assert!(!elenco[0].apribile);
        let motivo = elenco[0].motivo.clone().unwrap();
        assert!(motivo.contains("10") && motivo.contains("11"), "{motivo}");
        assert!(a.riprendi(ID).is_err());
        assert!(a.riavvolgi(ID, 0).is_err());
        let dopo: Vec<(String, Vec<u8>)> = nomi(&c)
            .into_iter()
            .map(|n| (n.clone(), fs::read(file(&c, &n)).unwrap()))
            .collect();
        assert_eq!(prima, dopo);
    }

    #[test]
    fn indice_d_altro_formato_non_si_riscrive() {
        let c = cartella("indice_d_altro_formato_non_si_riscrive");
        let (a, mut p) = con_partita(&c);
        let mut j: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(file(&c, INDICE)).unwrap()).unwrap();
        j["formato_indice"] = 2.into();
        fs::write(file(&c, INDICE), j.to_string()).unwrap();
        let prima = fs::read(file(&c, INDICE)).unwrap();

        p.chiudi_turno();
        let e = a.snapshot(ID, &p).unwrap_err();
        assert!(
            e.contains("indice.json") && e.contains("formato 2") && e.contains("formato 1"),
            "{e}"
        );
        assert!(a.riprendi(ID).is_err());
        assert!(a.riavvolgi(ID, 0).is_err());
        assert_eq!(fs::read(file(&c, INDICE)).unwrap(), prima);
        assert!(!file(&c, "turno-001.json.gz").exists());
    }

    #[test]
    fn nome_fuori_cartella_rifiutato() {
        for n in [
            "../x",
            "a/b",
            "a\\b",
            ".nascosto",
            "",
            &"x".repeat(61),
            "a\u{7}b",
        ] {
            assert!(Archivio::valida_nome(n).is_err(), "{n:?}");
        }
        for n in [ID, "Riforma è già qui", &"è".repeat(60)] {
            assert_eq!(Archivio::valida_nome(n), Ok(()), "{n:?}");
        }
    }

    #[test]
    fn albo_e_cartella_cancellata() {
        let c = cartella("albo_e_cartella_cancellata");
        let (a, mut p) = con_partita(&c);
        p.abbandona().unwrap();
        let v = p.voce_albo().unwrap();
        a.chiudi_nell_albo(ID, &v).unwrap();
        assert!(!c.0.join("partite").join(ID).exists());
        assert!(a.elenca().is_empty());

        // Ripetibile: una cartella rimasta a metà cancellazione si chiude di nuovo.
        fs::create_dir_all(c.0.join("partite").join(ID)).unwrap();
        fs::write(file(&c, "turno-000.json.gz"), b"resto").unwrap();
        a.chiudi_nell_albo(ID, &v).unwrap();
        assert!(!c.0.join("partite").join(ID).exists());
        // Senza cartella è un errore, e l'albo non si scrive.
        let e = a.chiudi_nell_albo("00000000000000b2", &v).unwrap_err();
        assert!(e.contains("00000000000000b2"), "{e}");
        assert!(!c.0.join("albo/00000000000000b2.json").exists());

        // Un file rotto, uno d'altro formato e un `.tmp` orfano si saltano.
        fs::write(c.0.join("albo/rotto.json"), "{").unwrap();
        fs::write(c.0.join("albo/orfano.json.tmp"), "{").unwrap();
        let mut futura = serde_json::to_value(&v).unwrap();
        futura["formato_albo"] = (FORMATO_ALBO + 1).into();
        fs::write(c.0.join("albo/futura.json"), futura.to_string()).unwrap();
        assert_eq!(a.albo(), [v]);
    }

    #[test]
    fn scenario_andata_e_ritorno() {
        let c = cartella("scenario_andata_e_ritorno");
        let a = Archivio::nuovo(c.0.clone()).unwrap();
        let base = dati();
        let mut s = SuMisura::base(&base);
        s.nome = "Più debito".into();
        s.cursori.debito_pil += 10.0;
        a.salva_scenario(&s, &base).unwrap();
        fs::write(c.0.join("scenari/orfano.toml.tmp"), "nome = ").unwrap();
        assert_eq!(a.scenari(), ["Più debito"]);
        assert_eq!(a.carica_scenario("Più debito", &base).unwrap(), s);
        // Lo stesso nome si sovrascrive; uno che differisce solo per le maiuscole no.
        a.salva_scenario(&s, &base).unwrap();
        s.nome = "più DEBITO".into();
        let e = a.salva_scenario(&s, &base).unwrap_err();
        assert!(e.contains("Più debito"), "{e}");
        assert_eq!(a.scenari(), ["Più debito"]);

        s.nome = "../fuori".into();
        assert!(a.salva_scenario(&s, &base).is_err());
        assert!(a.carica_scenario("../fuori", &base).is_err());
    }

    #[test]
    fn file_oltre_il_tetto_rifiutato() {
        let c = cartella("file_oltre_il_tetto_rifiutato");
        let a = Archivio::nuovo(c.0.clone()).unwrap();
        let grande = vec![b'#'; usize::try_from(TETTO).unwrap() + 1];
        fs::write(c.0.join("scenari/grande.toml"), grande).unwrap();
        let e = a.carica_scenario("grande", &dati()).unwrap_err();
        assert!(e.contains("grande.toml") && e.contains("64 MiB"), "{e}");
    }
}
