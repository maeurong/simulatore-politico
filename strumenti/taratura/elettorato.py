"""Elettorato: 20 regioni x 6 condizioni x 3 fasce di reddito.

Uso: python3 -I strumenti/taratura/elettorato.py [--senza-ibf] [--cache DIR]
     python3 -I strumenti/taratura/elettorato.py --selftest

Le quote dei gruppi vengono da un IPF sui margini reali: elettori della Camera 2022, ISTAT (occupati,
disoccupati, inattivi, quinti di reddito), MEF (pensionati, classi di reddito), seme IBF 2022
(--senza-ibf: seme d'indipendenza). Scarica nella cache (fuori dal repo) e scrive dati/base/elettorato.toml.
Le risposte e gli archivi scaricati sono dati: si leggono, non si eseguono. I microdati IBF restano nella cache.
Il resto (redditi, istogrammi, panieri, interessi) è ancora provvisorio: strumenti/taratura/README.md.
"""

import collections
import csv
import datetime
import importlib.util
import io
import json
import math
import pathlib
import sys
import time
import tomllib
import zipfile

BASE = pathlib.Path(__file__).resolve().parents[2] / "dati" / "base"
COMANDO = "python3 -I strumenti/taratura/elettorato.py"
OGGI = datetime.date.today().isoformat()
CACHE = pathlib.Path(sys.argv[sys.argv.index("--cache") + 1] if "--cache" in sys.argv else pathlib.Path.home() / ".cache" / "simulatore-politico-taratura")

URL_CAMERA = "https://documenti.camera.it/leg19/risultatiElettorali2022/UGE/VotiListe/ItaliaVotiListeCIRC.csv"
URL_IBF = "https://www.bancaditalia.it/statistiche/tematiche/indagini-famiglie-imprese/bilanci-famiglie/distribuzione-microdati/documenti/ind22_ascii.zip"
URL_MEF = "https://www1.finanze.gov.it/finanze/analisi_stat/public/v_4_0_0/contenuti/REG_tipo_reddito_2025.csv"
SDMX = "application/vnd.sdmx.data+csv;version=1.0.0"
ANNO_LAVORO, ANNO_QUINTI = 2025, 2024
# assunzione: l'ordine di IREG (codebook dei file annuali non trovato) è quello ISTAT; il gradiente del reddito equivalente per IREG lo conferma
IBF_IREG = ["piemonte", "valle_d_aosta", "lombardia", "trentino_alto_adige", "veneto", "friuli_venezia_giulia", "liguria", "emilia_romagna",
            "toscana", "umbria", "marche", "lazio", "abruzzo", "molise", "campania", "puglia", "basilicata", "calabria", "sicilia", "sardegna"]
SOGLIE_FASCE = (0.3, 0.7)  # decili 1-3 bassa, 4-7 media, 8-10 alta (dati-per-tarare-i-gruppi.md:66)
VUOTE_IBF = 1e-3  # celle senza osservazioni: questa frazione del prodotto dei margini
CONDIZIONI = ["dipendente_privato", "dipendente_pubblico", "autonomo", "pensionato", "disoccupato_inattivo", "studente"]
FASCE = ["bassa", "media", "alta"]
SHARE_PUBBLICO = 0.17  # assunzione: RGS non aperto
SHARE_STUDENTI = 0.06  # assunzione
ISTOGRAMMA = [0.05, 0.12, 0.17, 0.25, 0.16, 0.17, 0.08]  # segnaposto: la PR B lo ricava da ESS
PRIMARIO = {"dipendente_privato": 24000.0, "dipendente_pubblico": 30000.0, "autonomo": 28000.0,
            "pensionato": 0.0, "disoccupato_inattivo": 4000.0, "studente": 3000.0}
MOLT_FASCIA = (0.55, 1.0, 1.9)
MOLT_MACROAREA = {"nord": 1.1, "centro": 1.0, "mezzogiorno": 0.8}
# affluenza 2022, risultati-elettorali-italia.md:16-27; Valle d'Aosta = nazionale 63,6%
AFFLUENZA = {
    "calabria": 0.506, "sardegna": 0.530, "campania": 0.532, "molise": 0.564, "puglia": 0.565,
    "sicilia": 0.569, "basilicata": 0.586, "lazio": 0.634, "abruzzo": 0.637, "liguria": 0.639,
    "trentino_alto_adige": 0.657, "friuli_venezia_giulia": 0.660, "piemonte": 0.661, "marche": 0.682,
    "umbria": 0.685, "toscana": 0.694, "lombardia": 0.698, "veneto": 0.700, "emilia_romagna": 0.716,
    "valle_d_aosta": 0.636,
}
# assunzione del piano 4: tarata a mano sull'affluenza 2022 coi parametri di default, fedeltà inclusa; la PR B (tara) la sostituisce
# (0,57 = ln sum_j e^{-(4-j)^2} per un elettore al centro con beta = 1, senza fedeltà; Δ = -0,492)
COSTANTE_ASTENSIONE = 0.081
# ticket 17, 17-tabella-degli-interessi.md:28-35
INTERESSI = {
    "dipendente_privato": {"reddito": 0.45, "disoccupazione": 0.25, "qualita_trasporti": 0.10, "qualita_servizio_sanitario": 0.10, "sicurezza": 0.10},
    "dipendente_pubblico": {"reddito": 0.50, "occupazione_pubblica": 0.20, "qualita_servizio_sanitario": 0.15, "qualita_scuola": 0.15},
    "autonomo": {"reddito": 0.55, "durata_processi": 0.15, "sostegno_imprese": 0.15, "sicurezza": 0.15},
    "pensionato": {"reddito": 0.50, "qualita_servizio_sanitario": 0.35, "sicurezza": 0.15},
    "disoccupato_inattivo": {"reddito": 0.35, "disoccupazione": 0.40, "copertura_welfare": 0.25},
    "studente": {"reddito": 0.30, "qualita_scuola": 0.35, "disoccupazione": 0.35},
}
ALIQUOTE = {"bassa": 0.15, "media": 0.25, "alta": 0.33}  # assunzione: aliquote medie per fascia
ASTENSIONE_FASCIA = {"bassa": 0.3, "media": 0.0, "alta": -0.2}  # assunzione: la fascia bassa vota meno
ISTITUTI = (("Istituto A", 1000), ("Istituto B", 800), ("Istituto C", 1200), ("Istituto D", 1500))  # segnaposto
NOMI = ["Giulia", "Marco", "Francesca", "Luca", "Chiara", "Andrea", "Sara", "Matteo", "Elena", "Davide", "Anna", "Paolo"]
COGNOMI = ["Rossi", "Esposito", "Bianchi", "Romano", "Colombo", "Ricci", "Marino", "Greco", "Bruno", "Gallo", "Conti", "De Luca"]


def leggi(nome):
    return tomllib.loads((BASE / nome).read_text(encoding="utf-8"))


def quote(valori):
    """Quote che sommano a 1: arrotondate a 6 decimali, l'ultima corretta."""
    tot = sum(valori)
    assert tot > 0 and all(v >= 0 for v in valori), f"quote: somma {tot}, minimo {min(valori)} (attesi somma > 0, minimo >= 0)"
    q = [round(v / tot, 6) for v in valori[:-1]]
    return q + [round(1 - sum(q), 6)]


def prova_ipf():
    """Caso sintetico 2 x 2 x 2: i margini di una tabella nota tornano entro 1e-9."""
    nota = {(0, 0, 0): 3.0, (0, 0, 1): 1.0, (0, 1, 0): 2.0, (0, 1, 1): 4.0, (1, 0, 0): 5.0, (1, 0, 1): 2.0, (1, 1, 0): 1.0, (1, 1, 1): 2.0}
    margini = {assi: proiezione(nota, assi) for assi in ((0, 1), (0, 2), (1, 2))}
    t = ipf(dict.fromkeys(nota, 1.0), margini)
    for assi, m in margini.items():
        assert all(abs(x - m[k]) < 1e-9 for k, x in proiezione(t, assi).items()), f"margine {assi} non ritrovato"
    nota[0, 0, 0] = 0.0
    margini = {assi: proiezione(nota, assi) for assi in ((0, 1), (0, 2), (1, 2))}
    seme = dict.fromkeys(nota, 1.0)
    seme[0, 0, 0] = 0.0
    t = ipf(seme, margini)
    assert t[0, 0, 0] == 0.0 and all(abs(x - margini[(0, 1)][k]) < 1e-9 for k, x in proiezione(t, (0, 1)).items()), "cella zero consentita dal margine non rimasta zero"
    vuoto = dict.fromkeys(nota, 1.0)
    vuoto[0, 0, 0] = vuoto[0, 0, 1] = 0.0
    try:
        ipf(vuoto, margini)
    except AssertionError:
        return
    raise AssertionError("seme vuoto dove il margine vuole massa: attesa AssertionError")


def economia():
    spec = importlib.util.spec_from_file_location("economia", pathlib.Path(__file__).with_name("economia.py"))
    modulo = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(modulo)
    return modulo


def preleva(fonte, nome, url, accept=None):
    """Byte di una fonte: ogni fonte ha la sua cartella nella cache e un file già scaricato non si riscarica."""
    f = CACHE / fonte / nome
    if not f.exists():
        f.parent.mkdir(parents=True, exist_ok=True)
        f.write_bytes(economia().scarica(url, accept))
        if accept:
            time.sleep(15)  # il servizio ISTAT limita le interrogazioni al minuto
    return f.read_bytes()


def istat(nome, flusso, chiave, anno):
    url = f"{economia().ISTAT}IT1,{flusso},1.0/{chiave}?startPeriod={anno}&endPeriod={anno}"
    testo = preleva("istat", nome, url, SDMX).decode("utf-8-sig")
    return [r for r in csv.DictReader(io.StringIO(testo)) if r["TIME_PERIOD"] == str(anno)]


def proiezione(t, assi):
    """Somma della tabella {(r, c, f): v} sugli assi non nominati."""
    out = collections.defaultdict(float)
    for k, v in t.items():
        out[tuple(k[a] for a in assi)] += v
    return out


def ipf(seme, margini, tol=1e-12, max_giri=10_000):
    """Proporzionamento iterativo: scala a turno la tabella perché ogni margine ({assi: {chiave: bersaglio}})
    torni entro tol (relativo). Le celle a zero restano zero; un margine che vuole massa dove il seme è vuoto
    non è raggiungibile: AssertionError."""
    t = dict(seme)
    for _ in range(max_giri):
        scarto = 0.0
        for assi, bersaglio in margini.items():
            somme = proiezione(t, assi)
            scarto = max([scarto] + [abs(somme[k] - b) / (b or 1) for k, b in bersaglio.items()])
        if scarto <= tol:
            return t
        for assi, bersaglio in margini.items():
            somme = proiezione(t, assi)
            assert all(somme[k] > 0 or b == 0 for k, b in bersaglio.items()), f"ipf: margine {assi} vuole massa dove il seme è vuoto"
            fattore = {k: b / somme[k] if somme[k] else 0.0 for k, b in bersaglio.items()}
            t = {k: v * fattore[tuple(k[a] for a in assi)] for k, v in t.items()}
    raise AssertionError(f"ipf: scarto {scarto:.2e} dopo {max_giri} giri, atteso <= {tol}")


def camera():
    """Elettori della Camera 2022 per regione (19, senza Valle d'Aosta) e totale del file."""
    testo = preleva("camera", "ItaliaVotiListeCIRC.csv", URL_CAMERA).decode("utf-8-sig")
    righe = list(csv.reader(io.StringIO(testo), delimiter=";"))
    assert righe[0][6].split() == ["ELETTORI", "TOTALI"], f"camera: colonna 6 {righe[0][6]!r}, attesa ELETTORI TOTALI"
    out, totale = collections.Counter(), None
    for r in righe[1:]:
        if r[0] == "TOTALE":
            totale = int(r[6])
        elif r[0].isdigit():
            out[r[2].split("/")[0].rstrip("0123456789 ").lower().replace("-", "_").replace(" ", "_")] += int(r[6])
    assert totale == sum(out.values()), f"camera: somma {sum(out.values())}, totale del file {totale}"
    return dict(out), totale


def elettori(ids, elezioni, dati_camera):
    """Elettori per regione dal CSV della Camera; la Valle d'Aosta (collegio a parte, non nel file) dalla popolazione,
    in proporzione agli elettori delle altre regioni."""
    pop = dict.fromkeys(ids, 0)
    for c in elezioni["camera"]["circoscrizione"]:
        pop[c["regione"]] += c["popolazione"]
    e, totale = dati_camera
    assert set(e) == set(ids) - {"valle_d_aosta"}, f"camera: regioni {sorted(e)}, attese le 19 senza valle_d_aosta"
    e["valle_d_aosta"] = round(totale * pop["valle_d_aosta"] / (sum(pop.values()) - pop["valle_d_aosta"]))
    return {r: e[r] for r in ids}


def lavoro(ids):
    """Per regione (migliaia, 15-74 anni, 2025): dipendenti, indipendenti, disoccupati, inattivi. ISTAT RCFL."""
    cod = {r[0]: r[4] for r in economia().REGIONI}
    occ = {(r["REF_AREA"], r["POSIZ_PROF"]): float(r["OBS_VALUE"]) for r in istat("occ.csv", "150_938_DF_DCCV_OCCUPATIT1_23", "A" + "." * 13, ANNO_LAVORO)}
    dis = {r["REF_AREA"]: float(r["OBS_VALUE"]) for r in istat("dis.csv", "151_929_DF_DCCV_DISOCCUPT1_6", "", ANNO_LAVORO) if r["SEX"] == "9" and r["LABPROF_STATUS_A"] == "99"}
    ina = {r["REF_AREA"]: float(r["OBS_VALUE"]) for r in istat("ina.csv", "152_928_DF_DCCV_INATTIV1_9", "", ANNO_LAVORO)
           if r["SEX"] == "9" and r["AGE"] == "Y15-74" and r["LABPROF_STATUS_A"] == "IN"}
    return {r: (occ[cod[r], "1"], occ[cod[r], "2"], dis[cod[r]], ina[cod[r]]) for r in ids}


def mef(ids):
    """Pensionati per regione (persone con reddito da pensione) e quota di contribuenti per fascia, MEF anno d'imposta 2024.
    Le classi di reddito individuale passano a fascia dal percentile nazionale del loro punto medio."""
    testo = preleva("mef", "REG_tipo_reddito_2025.csv", URL_MEF).decode("utf-8-sig")
    num = lambda s: int(s.replace(".", "") or 0)
    pens, classi, ordine = collections.Counter(), collections.defaultdict(int), []
    for r in csv.DictReader(io.StringIO(testo), delimiter=";"):
        reg = r["Regione"].split("(")[0].strip().lower().replace(" ", "_").replace("'", "_")
        if reg not in ids:
            continue  # "Mancante/errata"
        k = r["Classi di reddito complessivo in euro"]
        if k not in ordine:
            ordine.append(k)
        pens[reg] += num(r["Reddito da pensione - Frequenza"])
        classi[reg, k] += num(r["Numero contribuenti"])
    nazionale = {k: sum(classi[r, k] for r in ids) for k in ordine}
    totale, prima, fascia = sum(nazionale.values()), 0, {}
    for k in ordine:
        centro = (prima + nazionale[k] / 2) / totale
        fascia[k] = 0 if centro < SOGLIE_FASCE[0] else 1 if centro < SOGLIE_FASCE[1] else 2
        prima += nazionale[k]
    quote_f = {}
    for r in ids:
        v = [sum(classi[r, k] for k in ordine if fascia[k] == f) for f in range(3)]
        quote_f[r] = [x / sum(v) for x in v]
    return dict(pens), quote_f


def quinti(peso_disoccupati):
    """Per condizione la quota di famiglie per fascia, nazionale, ISTAT EU-SILC 2024: bassa = quinto 1 + metà del 2,
    media = metà del 2 + quinto 3 + metà del 4, alta = metà del 4 + quinto 5 (decili 3-4-3).
    La condizione è quella del principale percettore; disoccupato_inattivo mescola 'in cerca' e 'altra non professionale'."""
    q = collections.defaultdict(dict)
    for r in istat("quin.csv", "32_38_DF_DCCV_QUINTOREDD_5", "", ANNO_QUINTI):
        if r["EDU_LEV_MAIN_EARN"] == "99":
            q[r["LABPROF_STATUS_C_MAIN_EARNER"]][r["FIFTH_EQUIV_HOUSE_INC"]] = float(r["OBS_VALUE"]) / 100
    fasce = {k: [x / sum(w) for x in w] for k, v in q.items() if k != "99"  # le percentuali ISTAT sommano 99,9 o 100,1
             for w in [[v["1"] + v["2"] / 2, v["2"] / 2 + v["3"] + v["4"] / 2, v["4"] / 2 + v["5"]]]}
    mix = [peso_disoccupati * a + (1 - peso_disoccupati) * b for a, b in zip(fasce["2B"], fasce["2C"])]
    return {"dipendente_privato": fasce["1B"], "dipendente_pubblico": fasce["1B"], "autonomo": fasce["1A"],
            "pensionato": fasce["2A"], "disoccupato_inattivo": mix, "studente": fasce["2C"]}


def ibf():
    """Seme IBF 2022: persone adulte per (regione, condizione, fascia di reddito familiare equivalente), pesate con pesofit2.
    Reddito equivalente = reddito familiare netto / radice del numero di componenti; fasce sul percentile nazionale pesato.
    Condizione da QUAL: 1-3 dipendente (pubblico per SHARE_PUBBLICO), 4-5 autonomo, 6 pensionato, 7 studente se sotto i 30 anni."""
    z = zipfile.ZipFile(io.BytesIO(preleva("ibf", "ind22_ascii.zip", URL_IBF)))
    leggi = lambda nome: csv.DictReader(io.TextIOWrapper(z.open(f"CSV/{nome}.csv"), encoding="utf-8-sig"))
    persone_, y = list(leggi("carcom22")), {r["NQUEST"]: float(r["Y"]) for r in leggi("rfam22")}
    n = collections.Counter(r["NQUEST"] for r in persone_)
    rec = []
    for r in persone_:
        eta, q, reg, w = int(r["ETA"]), int(r["QUAL"]), IBF_IREG[int(r["IREG"]) - 1], float(r["pesofit2"])
        red = y[r["NQUEST"]] / math.sqrt(n[r["NQUEST"]])
        if eta < 18:
            continue
        if q <= 3:
            rec += [(red, reg, "dipendente_pubblico", w * SHARE_PUBBLICO), (red, reg, "dipendente_privato", w * (1 - SHARE_PUBBLICO))]
        else:
            rec.append((red, reg, {4: "autonomo", 5: "autonomo", 6: "pensionato"}.get(q) or ("studente" if eta < 30 else "disoccupato_inattivo"), w))
    rec.sort()
    totale, prima, seme = sum(x[3] for x in rec), 0, collections.defaultdict(float)
    for red, reg, c, w in rec:
        centro = (prima + w / 2) / totale
        seme[reg, c, FASCE[0 if centro < SOGLIE_FASCE[0] else 1 if centro < SOGLIE_FASCE[1] else 2]] += w
        prima += w
    return seme


def persone(el, lav, pensionati):
    """Margine (b): persone per regione e condizione. Dipendenti (quota pubblica assunta), indipendenti, disoccupati e inattivi
    da ISTAT; pensionati da MEF; studenti assunti; i disoccupati o inattivi sono gli inattivi meno pensionati e studenti
    più i disoccupati (almeno i disoccupati). Ogni regione è scalata ai suoi elettori."""
    out = {}
    for r, (dip, aut, dis, ina) in lav.items():
        p = {"dipendente_pubblico": dip * SHARE_PUBBLICO, "dipendente_privato": dip * (1 - SHARE_PUBBLICO), "autonomo": aut,
             "pensionato": pensionati[r] / 1000, "studente": SHARE_STUDENTI * el[r] / 1000}
        p["disoccupato_inattivo"] = max(dis + ina - p["pensionato"] - p["studente"], dis)
        tot = sum(p.values())
        out[r] = {c: el[r] * x / tot for c, x in p.items()}
    return out


def quote_gruppi(ids, el, pers, fasce_mef, fasce_istat, seme_ibf):
    """Per regione le 18 quote (condizione x fascia), in ordine canonico: IPF 20 x 6 x 3 sui margini
    (b) regione x condizione, (c) regione x fascia, (d) condizione x fascia nazionale.
    (c) e (d) non hanno la stessa somma per fascia: (c) si ricalibra prima su righe = elettori e colonne = totali di (d)."""
    rc = {(r, c): pers[r][c] for r in ids for c in CONDIZIONI}
    cf = {(c, f): sum(pers[r][c] for r in ids) * fasce_istat[c][i] for c in CONDIZIONI for i, f in enumerate(FASCE)}
    rf = ipf({(r, f): el[r] * fasce_mef[r][i] for r in ids for i, f in enumerate(FASCE)},
             {(0,): {(r,): el[r] for r in ids}, (1,): {(f,): sum(cf[c, f] for c in CONDIZIONI) for f in FASCE}})
    indipendenza = {(r, c, f): rc[r, c] * rf[r, f] / el[r] for r in ids for c in CONDIZIONI for f in FASCE}
    if seme_ibf is None:
        seme = indipendenza
    else:
        scala = sum(el.values()) / sum(seme_ibf.values())
        seme = {k: seme_ibf[k] * scala if seme_ibf.get(k) else VUOTE_IBF * v for k, v in indipendenza.items()}
    t = ipf(seme, {(0, 1): rc, (0, 2): rf, (1, 2): cf})
    return {r: quote([t[r, c, f] for c in CONDIZIONI for f in FASCE]) for r in ids}


def istogrammi():
    return {(c, f): list(ISTOGRAMMA) for c in CONDIZIONI for f in FASCE}


def redditi(regioni, pers, scenario):
    nazionale = {c: sum(pers[r["id"]][c] for r in regioni) for c in CONDIZIONI}
    trasf = {c: 0.0 for c in CONDIZIONI}
    trasf["pensionato"] = scenario["spesa_pensioni"]["valore"] * 1e9 / scenario["pensionati"]["valore"]
    trasf["disoccupato_inattivo"] = scenario["spesa_sussidi"]["valore"] * 1e9 / nazionale["disoccupato_inattivo"]
    return {
        (r["id"], c, f): (round(PRIMARIO[c] * MOLT_FASCIA[i] * MOLT_MACROAREA[r["macroarea"]], 2), round(trasf[c] * MOLT_FASCIA[i], 2))
        for r in regioni for c in CONDIZIONI for i, f in enumerate(FASCE)
    }


def panieri(economia):
    return {f: quote(economia["consumi"]) for f in FASCE}  # segnaposto: la PR B li differenzia


def astensione(regioni):
    return {r: round(math.log((1 - AFFLUENZA[r]) / AFFLUENZA[r]) + COSTANTE_ASTENSIONE, 6) for r in regioni}


def costruisci():
    regioni = leggi("regioni.toml")["regione"]
    scenario, economia = leggi("scenario.toml"), leggi("economia.toml")
    ids = [r["id"] for r in regioni]
    dati_camera = camera()
    el = elettori(ids, leggi("elezioni.toml"), dati_camera)
    lav, (pens, fasce_mef) = lavoro(ids), mef(ids)
    peso_dis = sum(l[2] for l in lav.values()) / sum(l[2] + l[3] for l in lav.values())
    pers = persone(el, lav, pens)
    seme = None if "--senza-ibf" in sys.argv else ibf()
    return {
        "regioni": regioni, "elettori": el, "quote": quote_gruppi(ids, el, pers, fasce_mef, quinti(peso_dis), seme), "istogrammi": istogrammi(),
        "redditi": redditi(regioni, pers, scenario), "panieri": panieri(economia), "astensione": astensione(ids),
        "attesi": dati_camera[1] + el["valle_d_aosta"],
    }


def tabella(v):
    """Tabella TOML in linea: `{ k = x, ... }`."""
    return "{ " + ", ".join(f"{k} = {x}" for k, x in v.items()) + " }"


def scrivi(d):
    s = json.dumps
    t = f"# Generato da `{COMANDO}` il {OGGI}. Non modificare a mano.\n"
    t += "# Quote dei gruppi da IPF sui margini reali, il resto provvisorio: vedi strumenti/taratura/README.md.\n\n"
    t += f"aliquote = {tabella(ALIQUOTE)}\n"
    t += f"astensione_condizione = {tabella(dict.fromkeys(CONDIZIONI, 0.0))}\n"
    t += f"astensione_fascia = {tabella(ASTENSIONE_FASCIA)}\n"
    t += f"nomi = {s(NOMI, ensure_ascii=False)}\ncognomi = {s(COGNOMI, ensure_ascii=False)}\n\n[paniere]\n"
    t += "".join(f"{f} = [{', '.join(f'{x:.6f}' for x in d['panieri'][f])}]\n" for f in FASCE)
    for c, pesi in INTERESSI.items():
        t += f"\n[interessi.{c}]\n" + "".join(f"{k} = {w:.2f}\n" for k, w in pesi.items())
    for nome, campione in ISTITUTI:
        t += f"\n[[istituto]]\nnome = {s(nome)}\ncampione = {campione}\n"
    for r in d["regioni"]:
        i = r["id"]
        t += f"\n[[regione]]\nid = {s(i)}\nelettori = {d['elettori'][i]}\nastensione = {d['astensione'][i]}\n"
    for r in d["regioni"]:
        i = r["id"]
        for k, (c, f) in enumerate((c, f) for c in CONDIZIONI for f in FASCE):
            primario, trasf = d["redditi"][i, c, f]
            t += (f"\n[[gruppo]]\nregione = {s(i)}\ncondizione = {s(c)}\nfascia = {s(f)}\nquota = {d['quote'][i][k]:.6f}\n"
                  f"posizione = [{', '.join(f'{x:.2f}' for x in d['istogrammi'][c, f])}]\nprimario = {primario}\ntrasferimenti = {trasf}\n")
    return t


def controlla(d):
    assert len(d["regioni"]) == 20 and all(d["elettori"][r["id"]] > 0 for r in d["regioni"]), "attese 20 regioni con elettori > 0"
    tot = sum(d["elettori"].values())
    assert tot == d["attesi"], f"elettori {tot}, attesi {d['attesi']} (elettori del CSV + Valle d'Aosta)"
    # Affluenza del solo termine regionale (senza partiti né fedeltà): stesso intervallo del test Rust
    # affluenza_del_turno_zero_plausibile, coglie un segno o una costante sbagliati.
    affluenza = sum(d["elettori"][r] / (1 + math.exp(a)) for r, a in d["astensione"].items()) / tot
    assert 0.5 < affluenza < 0.8, f"affluenza implicita {affluenza:.3f}, attesa fra 0,5 e 0,8"
    for i, q in d["quote"].items():
        assert len(q) == 18 and min(q) >= 0 and abs(sum(q) - 1) < 1e-9, f"{i}: quote somma {sum(q):.12f}, minimo {min(q)}"
    for k, h in d["istogrammi"].items():
        assert len(h) == 7 and min(h) >= 0 and abs(sum(h) - 1) < 1e-9, f"{k}: istogramma somma {sum(h):.12f}"
    for k, (p, t) in d["redditi"].items():
        assert p + t > 0, f"{k}: primario + trasferimenti = {p + t}, atteso > 0"
    for c, pesi in INTERESSI.items():
        assert abs(sum(pesi.values()) - 1) < 1e-9, f"{c}: interessi somma {sum(pesi.values())}"
    for f, p in d["panieri"].items():
        assert len(p) == 10 and abs(sum(p) - 1) < 1e-9, f"paniere {f}: {len(p)} quote, somma {sum(p):.12f}"
    letto = tomllib.loads(scrivi(d))
    assert len(letto["regione"]) == 20 and len(letto["gruppo"]) == 360, f"{len(letto['regione'])} regioni, {len(letto['gruppo'])} gruppi, attesi 20 e 360"
    atteso = [(r["id"], c, f) for r in d["regioni"] for c in CONDIZIONI for f in FASCE]
    assert [(g["regione"], g["condizione"], g["fascia"]) for g in letto["gruppo"]] == atteso, "gruppi fuori ordine canonico"
    assert "qualita_trasporti = 0.10" in scrivi(d), "peso trasporti non scritto con due decimali"


def main():
    d = costruisci()
    controlla(d)
    if "--selftest" in sys.argv:
        prova_ipf()
        # I test Rust leggono il file committato: deve essere l'uscita dello script (intestazione
        # con la data esclusa). Dopo `sim-cli tara` (Task 12) il file cambia: il confronto va ristretto lì.
        committato = (BASE / "elettorato.toml").read_text(encoding="utf-8").splitlines()[1:]
        assert scrivi(d).splitlines()[1:] == committato, "elettorato.toml non è l'uscita dello script: rilancialo"
        print("selftest ok: 20 regioni, 360 gruppi")
        return
    destinazione = BASE / "elettorato.toml"
    (BASE / "elettorato.toml.tmp").write_text(scrivi(d), encoding="utf-8")
    (BASE / "elettorato.toml.tmp").replace(destinazione)
    print("Scritto elettorato.toml in", BASE)


if __name__ == "__main__":
    main()
