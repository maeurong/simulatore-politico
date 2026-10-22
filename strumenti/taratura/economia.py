"""Pipeline di taratura offline dei dati strutturali del motore economico.

Uso: python3 -I strumenti/taratura/economia.py

Scarica da Eurostat e ISTAT, aggrega e scrive dati/base/economia.toml,
dati/base/regioni.toml e i campi di spesa in dati/base/dicasteri.toml.
Le risposte delle API si leggono come dati (json, csv), mai si eseguono.
Fonti, licenze e modifiche ai dati: strumenti/taratura/README.md.
"""

import csv
import datetime
import io
import json
import pathlib
import time
import urllib.error
import urllib.parse
import urllib.request

EUROSTAT = "https://ec.europa.eu/eurostat/api/dissemination/statistics/1.0/data/"
ISTAT = "https://esploradati.istat.it/SDMXWS/rest/data/"
BASE = pathlib.Path(__file__).resolve().parents[2] / "dati" / "base"
COMANDO = "python3 -I strumenti/taratura/economia.py"
OGGI = datetime.date.today().isoformat()

ANNO_TAVOLA = 2024  # FIGARO, edizione 2026: naio_10_fcp_ip4 copre dal 2022
ANNO_COFOG = 2024
ANNO_MIX = 2023  # ultimo anno con l'A*10 regionale (2024 solo a 6 branche)
ANNO_LAVORO = 2025
EDIZIONE_CET = "2025M12"  # conti territoriali ISTAT, edizione di dicembre 2025

SETTORI = [
    ("A", "Agricoltura, silvicoltura e pesca"),
    ("BTE", "Industria in senso stretto"),
    ("F", "Costruzioni"),
    ("GTI", "Commercio, trasporti, alloggio e ristorazione"),
    ("J", "Informazione e comunicazione"),
    ("K", "Attività finanziarie e assicurative"),
    ("L", "Attività immobiliari"),
    ("MN", "Attività professionali, scientifiche, tecniche e di supporto"),
    ("OTQ", "Amministrazione pubblica, istruzione, sanità e assistenza"),
    ("RTU", "Arte, intrattenimento e altri servizi"),
]
IDS = [s for s, _ in SETTORI]
# Sezione NACE/CPA (prima lettera del codice) -> settore A*10.
SEZIONE = {
    "A": "A", "B": "BTE", "C": "BTE", "D": "BTE", "E": "BTE", "F": "F",
    "G": "GTI", "H": "GTI", "I": "GTI", "J": "J", "K": "K", "L": "L",
    "M": "MN", "N": "MN", "O": "OTQ", "P": "OTQ", "Q": "OTQ",
    "R": "RTU", "S": "RTU", "T": "RTU", "U": "RTU",
}
VALORE_AGGIUNTO_FIGARO = {"B2A3G", "D1", "D21X31", "D29X39"}

# id, nome, macroarea, codici NUTS 2 Eurostat, codice territorio ISTAT, Okun b.
# Okun: Bonaventura (2020), tab. 2, docs/ricerca/modelli/economia-italiana-parametri.md:56-65
# (coefficienti base, senza AR(1)): pubblicazione PDF, nessuna API.
REGIONI = [
    ("piemonte", "Piemonte", "nord", ["ITC1"], "ITC1", -0.239),
    ("valle_d_aosta", "Valle d'Aosta", "nord", ["ITC2"], "ITC2", -0.059),
    ("liguria", "Liguria", "nord", ["ITC3"], "ITC3", -0.262),
    ("lombardia", "Lombardia", "nord", ["ITC4"], "ITC4", -0.220),
    ("trentino_alto_adige", "Trentino-Alto Adige", "nord", ["ITH1", "ITH2"], "ITDA", -0.141),
    ("veneto", "Veneto", "nord", ["ITH3"], "ITD3", -0.198),
    ("friuli_venezia_giulia", "Friuli-Venezia Giulia", "nord", ["ITH4"], "ITD4", -0.172),
    ("emilia_romagna", "Emilia-Romagna", "nord", ["ITH5"], "ITD5", -0.222),
    ("toscana", "Toscana", "centro", ["ITI1"], "ITE1", -0.281),
    ("umbria", "Umbria", "centro", ["ITI2"], "ITE2", -0.345),
    ("marche", "Marche", "centro", ["ITI3"], "ITE3", -0.295),
    ("lazio", "Lazio", "centro", ["ITI4"], "ITE4", -0.391),
    ("abruzzo", "Abruzzo", "mezzogiorno", ["ITF1"], "ITF1", -0.238),
    ("molise", "Molise", "mezzogiorno", ["ITF2"], "ITF2", -0.305),
    ("campania", "Campania", "mezzogiorno", ["ITF3"], "ITF3", -0.369),
    ("puglia", "Puglia", "mezzogiorno", ["ITF4"], "ITF4", -0.403),
    ("basilicata", "Basilicata", "mezzogiorno", ["ITF5"], "ITF5", -0.180),
    ("calabria", "Calabria", "mezzogiorno", ["ITF6"], "ITF6", -0.342),
    ("sicilia", "Sicilia", "mezzogiorno", ["ITG1"], "ITG1", -0.419),
    ("sardegna", "Sardegna", "mezzogiorno", ["ITG2"], "ITG2", -0.251),
]

# Dicastero -> (divisioni COFOG sommate, divisioni sottratte), come in
# docs/ricerca/sintesi/stato-del-paese-al-turno-zero.md:88-99. La spesa è TE - D62:
# le prestazioni in denaro (D62) stanno solo nella divisione 10.
DICASTERI = {
    "sanita": (["GF07"], []),
    "istruzione_ricerca": (["GF09", "GF08", "GF0104"], []),
    "economia_finanze": (["GF01"], ["GF0102", "GF0104", "GF0107"]),
    "infrastrutture_trasporti": (["GF0405", "GF06"], []),
    "sviluppo_economico": (["GF04"], ["GF0403", "GF0405"]),
    "ambiente_energia": (["GF0403", "GF05"], []),
    "esteri_difesa": (["GF02", "GF0102"], []),
    "interno": (["GF03"], ["GF0303", "GF0304"]),
    "lavoro_welfare": (["GF10"], []),
    "giustizia": (["GF0303", "GF0304"], []),
}
SPESA_DIECI_DICASTERI = 575.6  # mld, stato-del-paese-al-turno-zero.md:100


def scarica(url, accept=None):
    richiesta = urllib.request.Request(url, headers={"User-Agent": "simulatore-politico-taratura"})
    if accept:
        richiesta.add_header("Accept", accept)
    for tentativo in range(4):
        try:
            with urllib.request.urlopen(richiesta, timeout=300) as risposta:
                return risposta.read()
        except (urllib.error.URLError, ConnectionError) as e:
            # ponytail: backoff fisso; il servizio ISTAT chiude la connessione se si interroga troppo spesso
            if tentativo == 3:
                raise
            print(f"  {e}, riprovo fra 60 s")
            time.sleep(60)


def eurostat(dataset, **filtri):
    """Righe (dimensioni, valore) di una risposta JSON-stat Eurostat."""
    url = EUROSTAT + dataset + "?" + urllib.parse.urlencode({"format": "JSON", "lang": "EN", **filtri}, doseq=True)
    print("GET", url)
    d = json.loads(scarica(url))
    ids, size = d["id"], d["size"]
    cats = [sorted(d["dimension"][k]["category"]["index"], key=d["dimension"][k]["category"]["index"].get) for k in ids]
    passi = [1] * len(ids)
    for i in range(len(ids) - 2, -1, -1):
        passi[i] = passi[i + 1] * size[i + 1]
    for n, v in d["value"].items():
        n = int(n)
        yield {k: c[(n // p) % z] for k, p, z, c in zip(ids, passi, size, cats)}, v


def istat(flusso, chiave, anno):
    url = f"{ISTAT}IT1,{flusso},1.0/{chiave}?startPeriod={anno}&endPeriod={anno}"
    print("GET", url)
    testo = scarica(url, "application/vnd.sdmx.data+csv;version=1.0.0").decode("utf-8-sig")
    time.sleep(15)  # il servizio ISTAT limita le interrogazioni al minuto
    return [r for r in csv.DictReader(io.StringIO(testo)) if r["TIME_PERIOD"] == str(anno)]


def settore(codice):
    return SEZIONE[codice[4]] if codice.startswith("CPA_") else None


def quote(valori):
    """Quote che sommano a 1: arrotondate a 6 decimali, l'ultima corretta."""
    tot = sum(valori)
    assert tot > 0 and all(v >= 0 for v in valori), valori
    q = [round(v / tot, 6) for v in valori[:-1]]
    return q + [round(1 - sum(q), 6)]


def f6(xs):
    return ", ".join(f"{x:.6f}" for x in xs)


def stringa(s):
    return json.dumps(s, ensure_ascii=False)  # una stringa JSON è una stringa base TOML valida


def intestazione(fonti):
    righe = [f"# Generato da `{COMANDO}` il {OGGI}. Non modificare a mano.", "# Fonti:"]
    righe += [f"#   {f}" for f in fonti]
    righe += [
        "# Licenze: Eurostat, riuso con citazione e dichiarazione delle modifiche;",
        "#   ISTAT, CC BY 4.0. Modifiche ai dati: strumenti/taratura/README.md.",
    ]
    return "\n".join(righe) + "\n"


def tavola():
    """Matrice A, importazioni e domanda finale A*10 dalla tavola FIGARO, Italia."""
    z = {(i, j): 0.0 for i in IDS for j in IDS}
    imp, va, uscite = dict.fromkeys(IDS, 0.0), dict.fromkeys(IDS, 0.0), dict.fromkeys(IDS, 0.0)
    domanda = {k: dict.fromkeys(IDS, 0.0) for k in ("consumi", "investimenti", "spesa_pubblica", "esportazioni")}
    colonna = {"P3_S14": "consumi", "P3_S15": "consumi", "P51G": "investimenti", "P3_S13": "spesa_pubblica"}

    for r, v in eurostat("naio_10_fcp_ip4", c_dest="IT", time=ANNO_TAVOLA):
        assert r["unit"] == "MIO_EUR" and r["time"] == str(ANNO_TAVOLA), r
        i, j, origine = settore(r["prd_ava"]), settore(r["prd_use"]), r["c_orig"]
        if i is None:
            assert origine == "DOM", r
            if j and r["prd_ava"] in VALORE_AGGIUNTO_FIGARO:
                va[j] += v
            elif j:
                assert v == 0, r  # acquisti diretti di residenti/non residenti: solo domanda finale
        elif j:
            if origine == "IT":
                z[i, j] += v
            else:
                imp[j] += v
        elif origine == "IT" and r["prd_use"] in colonna:
            domanda[colonna[r["prd_use"]]][i] += v

    for r, v in eurostat("naio_10_fcp_ip4", c_orig="IT", time=ANNO_TAVOLA):
        i = settore(r["prd_ava"])
        assert i, r
        uscite[i] += v
        if r["c_dest"] != "IT":
            domanda["esportazioni"][i] += v  # P6: impieghi esteri, intermedi e finali, dei prodotti italiani

    produzione = {j: sum(z[i, j] for i in IDS) + imp[j] + va[j] for j in IDS}
    for s in IDS:
        scarto = abs(uscite[s] - produzione[s]) / produzione[s]
        print(f"  {s:4} produzione {produzione[s]:12.1f} mln, impieghi {uscite[s]:12.1f} mln, scarto {scarto:.2e}")
        assert scarto < 1e-3, s

    a = [[round(z[i, j] / produzione[j], 6) for j in IDS] for i in IDS]
    importazioni = [round(imp[j] / produzione[j], 6) for j in IDS]
    valore_aggiunto = [round(1 - sum(a[i][k] for i in range(10)) - importazioni[k], 6) for k in range(10)]
    vettori = {k: quote([v[s] for s in IDS]) for k, v in domanda.items()}
    return a, importazioni, valore_aggiunto, vettori


def piramide():
    righe = istat("22_289_DF_DCIS_POPRES1_1", "A.IT.JAN.9..99", 2026)
    per_eta = {r["AGE"]: int(float(r["OBS_VALUE"])) for r in righe}
    chiavi = [f"Y{e}" for e in range(100)] + ["Y_GE100"]
    valori = [per_eta[k] for k in chiavi]
    print(f"  piramide: {sum(valori)} residenti, totale ISTAT {per_eta['TOTAL']}")
    assert sum(valori) == per_eta["TOTAL"]
    return valori


def regioni():
    def per_geo(dataset, **filtri):
        out = {}
        for r, v in eurostat(dataset, time=ANNO_LAVORO, sex="T", age="Y15-74", **filtri):
            assert r["unit"] in ("PC", "THS_PER"), r
            out[r["geo"]] = v
        return out

    disoccupazione = per_geo("lfst_r_lfu3rt", isced11="TOTAL")
    forze = per_geo("lfst_r_lfp2act")

    codici = "+".join(r[4] for r in REGIONI)
    branche = "+".join(["M_N" if s == "MN" else s for s in IDS] + ["_T"])
    va = {}
    for r in istat("93_1227_DF_DCCN_TNA1_1", f"A.{codici}.B1G_B_W2_S1.{branche}...V...{EDIZIONE_CET}", ANNO_MIX):
        va[r["REF_AREA"], r["BRKDW_INDUSTRY_NACE_REV2"].replace("M_N", "MN")] = float(r["OBS_VALUE"])

    out = []
    for rid, nome, macro, nuts, cod, okun in REGIONI:
        fl = sum(forze[g] for g in nuts)
        u = sum(disoccupazione[g] * forze[g] for g in nuts) / fl
        valori = [va[cod, s] for s in IDS]
        totale = va[cod, "_T"]
        assert abs(sum(valori) - totale) / totale < 1e-3, (rid, sum(valori), totale)
        out.append((rid, nome, macro, round(fl, 1), round(u, 2), okun, quote(valori)))
    naz = sum(r[3] * r[4] for r in out) / sum(r[3] for r in out)
    print(f"  forze di lavoro: somma regioni {sum(r[3] for r in out):.1f}, Italia {forze['IT']:.1f} migliaia")
    print(f"  disoccupazione nazionale pesata {naz:.3f}% (Eurostat IT {disoccupazione['IT']}%)")
    return out, naz


def dicasteri():
    voce = {}
    for r, v in eurostat("gov_10a_exp", geo="IT", sector="S13", unit="MIO_EUR", time=ANNO_COFOG,
                         na_item=["TE", "D1", "P2", "D632", "D62"]):
        voce[r["cofog99"], r["na_item"]] = v
    pil = next(v for _, v in eurostat("nama_10_gdp", geo="IT", unit="CP_MEUR", na_item="B1GQ", time=ANNO_COFOG))

    def somma(divisioni, voci):
        piu, meno = divisioni
        return sum(voce[d, x] for x in voci for d in piu) - sum(voce[d, x] for x in voci for d in meno)

    out = {}
    for d, divisioni in DICASTERI.items():
        spesa = somma(divisioni, ["TE"]) - somma(divisioni, ["D62"])
        salari = somma(divisioni, ["D1"])
        acquisti = somma(divisioni, ["P2", "D632"])
        print(f"  {d:26} spesa {spesa / 1000:6.1f}  salari {salari / 1000:5.1f}  acquisti {acquisti / 1000:5.1f} mld")
        out[d] = [round(x / pil * 100, 4) for x in (spesa, salari, acquisti)]
    print(f"  PIL {ANNO_COFOG}: {pil / 1000:.1f} mld")
    return out, pil


def scrivi_economia(a, importazioni, valore_aggiunto, vettori, eta):
    t = intestazione([
        f"Eurostat naio_10_fcp_ip4 (FIGARO, tavola prodotto per prodotto {ANNO_TAVOLA}), c_dest=IT e c_orig=IT",
        "ISTAT 22_289_DF_DCIS_POPRES1_1, chiave A.IT.JAN.9..99 (residenti al 1° gennaio 2026, stima)",
    ])
    t += f"\nanno_tavola = {ANNO_TAVOLA}\n\nsettori = [\n"
    t += "".join(f"  {{ id = {stringa(s)}, nome = {stringa(n)} }},\n" for s, n in SETTORI) + "]\n\n"
    t += "# a[i][j]: input interno del settore i per unità di produzione del settore j (colonne = chi compra)\n"
    t += "a = [\n" + "".join(f"  [{f6(riga)}],\n" for riga in a) + "]\n\n"
    t += f"importazioni = [{f6(importazioni)}]\n"
    t += f"valore_aggiunto = [{f6(valore_aggiunto)}]\n\n"
    t += "".join(f"{k} = [{f6(vettori[k])}]\n" for k in ("consumi", "investimenti", "esportazioni", "spesa_pubblica"))
    t += "\n# residenti per età 0..99, poi 100 e oltre\npiramide = [\n"
    t += "".join("  " + ", ".join(str(x) for x in eta[k:k + 10]) + ",\n" for k in range(0, 101, 10)) + "]\n"
    (BASE / "economia.toml").write_text(t, encoding="utf-8")


def scrivi_regioni(righe):
    t = intestazione([
        f"Eurostat lfst_r_lfu3rt e lfst_r_lfp2act, {ANNO_LAVORO}, 15-74 anni",
        f"ISTAT 93_1227_DF_DCCN_TNA1_1, valore aggiunto ai prezzi correnti {ANNO_MIX}, edizione {EDIZIONE_CET}",
        "Okun: Bonaventura (2020), Economic Bulletin, tab. 2 (b della forma in differenze)",
    ])
    for rid, nome, macro, fl, u, okun, mix in righe:
        t += f"\n[[regione]]\nid = {stringa(rid)}\nnome = {stringa(nome)}\nmacroarea = {stringa(macro)}\n"
        t += f"forze_lavoro = {fl}\ndisoccupazione_2025 = {u}\nokun = {okun}\n"
        t += "mix = { " + ", ".join(f"{s} = {q:.6f}" for s, q in zip(IDS, mix)) + " }\n"
    (BASE / "regioni.toml").write_text(t, encoding="utf-8")


def scrivi_dicasteri(quote_pil, investimenti):
    """Aggiunge i campi di spesa dopo `ruoli` di ogni dicastero; rieseguibile."""
    generati = ("spesa_pil", "salari_pil", "acquisti_pil", "settori")
    righe, visti, corrente = [], [], None
    for riga in (BASE / "dicasteri.toml").read_text(encoding="utf-8").splitlines():
        if riga.split(" =")[0] in generati or riga.startswith("# [taratura]"):
            continue
        righe.append(riga)
        if len(righe) == 1:
            righe.append(f"# [taratura] spesa_pil, salari_pil, acquisti_pil e settori: generati da `{COMANDO}`")
            righe.append(f"# [taratura] il {OGGI}, da Eurostat gov_10a_exp {ANNO_COFOG} e nama_10_gdp, in % del PIL.")
        if riga.startswith("id = "):
            corrente = json.loads(riga.split("=", 1)[1])
        if riga.startswith("ruoli = "):
            spesa, salari, acquisti = quote_pil[corrente]
            righe += [f"spesa_pil = {spesa}", f"salari_pil = {salari}", f"acquisti_pil = {acquisti}"]
            if corrente == "infrastrutture_trasporti":
                righe.append("settori = { " + ", ".join(f"{s} = {q:.6f}" for s, q in zip(IDS, investimenti)) + " }")
            visti.append(corrente)
    assert sorted(visti) == sorted(DICASTERI), visti
    (BASE / "dicasteri.toml").write_text("\n".join(righe) + "\n", encoding="utf-8")


def main():
    print(f"FIGARO {ANNO_TAVOLA}")
    a, importazioni, valore_aggiunto, vettori = tavola()
    print("COFOG")
    quote_pil, pil = dicasteri()
    print("Regioni")
    righe_regioni, disoccupazione_naz = regioni()
    print("Piramide")
    eta = piramide()

    print("Invarianti")
    assert len(SETTORI) == 10 and len(a) == 10 and all(len(r) == 10 for r in a)
    for j in range(10):
        colonna = sum(a[i][j] for i in range(10)) + importazioni[j]
        print(f"  colonna {IDS[j]:4} somma a + importazioni = {colonna:.6f}, valore aggiunto {valore_aggiunto[j]:.6f}")
        assert colonna < 1 and valore_aggiunto[j] > 0
    for k, v in vettori.items():
        print(f"  {k:15} somma {sum(v):.12f}")
        assert len(v) == 10 and abs(sum(v) - 1) < 1e-9
    assert len(righe_regioni) == 20
    for r in righe_regioni:
        assert abs(sum(r[6]) - 1) < 1e-9, r[0]
    print(f"  20 regioni, ogni mix somma 1 entro 1e-9 (max scarto {max(abs(sum(r[6]) - 1) for r in righe_regioni):.1e})")
    print(f"  disoccupazione nazionale 2025 pesata: {disoccupazione_naz:.3f}%")
    assert 6.0 <= disoccupazione_naz <= 6.2
    totale = sum(q[0] for q in quote_pil.values())
    atteso = SPESA_DIECI_DICASTERI / (pil / 1000) * 100
    print(f"  somma spesa_pil {totale:.4f}% del PIL, attesa {atteso:.4f}% (575,6 mld)")
    assert abs(totale - atteso) <= 0.1
    assert len(eta) == 101

    scrivi_dicasteri(quote_pil, vettori["investimenti"])  # primo: il suo assert precede ogni scrittura
    scrivi_economia(a, importazioni, valore_aggiunto, vettori, eta)
    scrivi_regioni(righe_regioni)
    print("Scritti economia.toml, regioni.toml, dicasteri.toml in", BASE)


if __name__ == "__main__":
    main()
