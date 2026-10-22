"""Stima offline di beta_distanza e fedelta con un logit condizionale su ITANES 2022 (post).

U_k = -beta10 * (x - z_k)^2 + lambda_k + phi * [k = voto 2018], k = 8 partiti, poi astensione (lambda = 0),
x = autocollocazione 0-10 (Q19), z_k = media delle collocazioni del partito (Q18). Stessa forma del gioco
(`crates/sim-core/src/elettorato/voto.rs`). beta_distanza = beta10 / 0,36 (scala 1-7).
Solo libreria standard. I microdati restano fuori dal repo: qui escono solo i parametri.

    python3 -I strumenti/taratura/stima_voto.py [--itanes CARTELLA] [--solo-stima] [--sensibilita]
    python3 -I strumenti/taratura/stima_voto.py --selftest
"""
import argparse, csv, glob, math, os, random, re, sys, tempfile

REPO = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
MANOPOLE = os.path.join(REPO, "dati", "base", "manopole.toml")
CACHE = os.path.expanduser("~/.cache/simulatore-politico-taratura/itanes")
PARTITI = ["FdI", "Lega", "FI", "Azione-IV", "M5S", "PD", "+Europa", "AVS"]
ASTENSIONE = 8
# voto 2022 (voto_post, Q10LHb) -> alternativa; 8 Italexit, 10 Impegno Civico, 11 Noi Moderati, 12 altro: fuori dalla scelta
VOTO22 = {1: 0, 2: 1, 3: 2, 4: 3, 5: 4, 6: 5, 7: 6, 9: 7}
# D4: voto 2018 (Q14b) -> erede 2022 per famiglia politica; None = senza erede, nessun termine di fedeltà
ERede18 = {1: 1, 2: 2, 3: 0, 4: 4, 5: 5, 6: 6, 7: 7, 8: None, 9: None, 10: None}
SCALA = 0.36  # beta7 = beta10 / 0,36


def risolvi(a, b):
    """Gauss con pivot parziale: a x = b, b matrice (colonne)."""
    n = len(a)
    m = [r[:] + c[:] for r, c in zip(a, b)]
    for i in range(n):
        p = max(range(i, n), key=lambda r: abs(m[r][i]))
        if abs(m[p][i]) < 1e-12:
            raise ValueError("matrice singolare: parametro non identificato")
        m[i], m[p] = m[p], m[i]
        for r in range(n):
            if r != i:
                f = m[r][i] / m[i][i]
                m[r] = [x - f * y for x, y in zip(m[r], m[i])]
    return [[x / m[i][i] for x in m[i][n:]] for i in range(n)]


def feature(o, k):
    """Coppie (indice, valore) dei regressori dell'alternativa k: [-d2, [k=m], lambda_k]."""
    f = [(0, -o["d2"][k])]
    if o["m"] == k:
        f.append((1, 1.0))
    if k < ASTENSIONE:
        f.append((2 + k, 1.0))
    return f


def stima(oss, iters=50):
    """MLE per Newton-Raphson. Ritorna (theta, errori standard, log-verosimiglianza)."""
    P = 2 + len(PARTITI)
    th = [0.0] * P
    def passo(th):
        ll, g, H = 0.0, [0.0] * P, [[0.0] * P for _ in range(P)]
        for o in oss:
            fs = [feature(o, k) for k in range(ASTENSIONE + 1)]
            u = [sum(th[i] * v for i, v in f) for f in fs]
            mx = max(u)
            e = [math.exp(x - mx) for x in u]
            s = sum(e)
            p = [x / s for x in e]
            ll += u[o["y"]] - mx - math.log(s)
            mean = [0.0] * P
            for pk, f in zip(p, fs):
                for i, v in f:
                    mean[i] += pk * v
            for i, v in fs[o["y"]]:
                g[i] += v
            for i in range(P):
                g[i] -= mean[i]
                for j in range(P):
                    H[i][j] += mean[i] * mean[j]
            for pk, f in zip(p, fs):
                for i, v in f:
                    for j, w in f:
                        H[i][j] -= pk * v * w
        return ll, g, H  # H = -(informazione osservata)... segno: H = mean mean^T - sum p f f^T
    for _ in range(iters):
        ll, g, H = passo(th)
        info = [[-x for x in r] for r in H]
        d = [r[0] for r in risolvi(info, [[x] for x in g])]
        t = 1.0
        while t > 1e-6:  # dimezza il passo finché la verosimiglianza non cala
            nuovo = [a + t * b for a, b in zip(th, d)]
            if passo(nuovo)[0] >= ll - 1e-9:
                break
            t /= 2
        th = nuovo
        if max(abs(t * x) for x in d) < 1e-9:
            break
    ll, g, H = passo(th)
    inv = risolvi([[-x for x in r] for r in H], [[float(i == j) for j in range(P)] for i in range(P)])
    return th, [math.sqrt(inv[i][i]) for i in range(P)], ll


def sintetici(n, theta, z, seed):
    rng = random.Random(seed)
    oss = []
    for _ in range(n):
        x = rng.uniform(0, 10)
        m = rng.choice([None] + list(range(ASTENSIONE + 1)))
        o = {"d2": [(x - zk) ** 2 for zk in z] + [0.0], "m": m}
        u = [sum(theta[i] * v for i, v in feature(o, k)) for k in range(ASTENSIONE + 1)]
        mx = max(u)
        o["y"] = rng.choices(range(ASTENSIONE + 1), [math.exp(a - mx) for a in u])[0]
        oss.append(o)
    return oss


def leggi(cartella, fedele_nr=False, proprie=False, solo_votanti=False, centro_nc=False):
    """Osservazioni dal post ITANES 2022. Ritorna (oss, z, conteggi di scarto)."""
    fs = glob.glob(os.path.join(cartella, "*POST*weighted.tab"))
    if len(fs) != 1:
        sys.exit(f"cerco un solo *POST*weighted.tab in {cartella}, trovati {len(fs)}: vedi README, sezione Stima di beta e fedeltà")
    with open(fs[0], encoding="utf-8", newline="") as f:
        righe = list(csv.DictReader(f, delimiter="\t"))
    def num(r, c):
        try:
            return float(r[c])
        except (KeyError, ValueError):
            return None
    def pos(v):  # codici 1..11 = 0..10; 96-99 = non collocato
        return v - 1 if v is not None and 1 <= v <= 11 else None
    col = [f"Q18{c}" for c in "abcdefgh"]
    pp = [[pos(num(r, c)) for c in col] for r in righe]
    z = [sum(v) / len(v) for v in ([p[k] for p in pp if p[k] is not None] for k in range(8))]
    oss, scarti = [], {"senza_autocollocazione": 0, "voto_22_fuori_scelta": 0, "voto_18_ignoto": 0}
    for r, p in zip(righe, pp):
        x = pos(num(r, "Q19"))
        if x is None and centro_nc:
            x = 5.0  # ponytail: i non collocati al centro, come suggerito in dati-per-tarare-i-gruppi:91
        if x is None:
            scarti["senza_autocollocazione"] += 1
            continue
        if num(r, "Q10LHa") == 2:
            y = ASTENSIONE
        else:
            y = VOTO22.get(int(num(r, "voto_post") or 0))
        if y is None or (solo_votanti and y == ASTENSIONE):
            scarti["voto_22_fuori_scelta"] += 1
            continue
        a, b = num(r, "Q14a"), int(num(r, "Q14b") or 0)
        m = ASTENSIONE if a == 2 else None if a == 3 else ERede18.get(b) if a == 1 and b in ERede18 else "ignoto"
        if m == "ignoto":
            if not fedele_nr:
                scarti["voto_18_ignoto"] += 1
                continue
            m = None
        if solo_votanti and m == ASTENSIONE:
            m = None
        zz = [pk if proprie and pk is not None else zk for pk, zk in zip(p, z)]
        # solo votanti: l'astensione esce dalla scelta con una distanza enorme (utilità ~ -inf)
        oss.append({"x": x, "m": m, "y": y, "d2": [(x - a) ** 2 for a in zz] + [1e4 if solo_votanti else 0.0]})
    return oss, z, scarti


def riga_toml(sez, valore, commento, testo):
    """Sostituisce `valore` e la riga di commento che precede [sez]."""
    pat = re.compile(r"(?:^# [^\n]*\n)?(\[" + sez + r"\]\nvalore = )[^\n]*", re.M)
    nuovo, n = pat.subn(lambda m: f"# {commento}\n{m.group(1)}{valore}", testo, count=1)
    assert n == 1, f"[{sez}] non trovata in manopole.toml"
    return nuovo


def scrivi(path, b10, se_b10, phi, se_phi, n):
    t = open(path, encoding="utf-8").read()
    b7, se7 = b10 / SCALA, se_b10 / SCALA
    t = riga_toml("beta_distanza", f"{b7:.2f}",
                  f"ITANES 2022 post, N={n}: beta10 {b10:.3f} (ES {se_b10:.3f}), beta7 = beta10/0,36 = {b7:.2f} (ES {se7:.2f}); stima_voto.py", t)
    t = riga_toml("fedelta", f"{phi:.2f}",
                  f"ITANES 2022 post, N={n}: phi {phi:.2f} (ES {se_phi:.2f}), tetto: il ricordo del 2018 si sposta verso il 2022; stima_voto.py", t)
    for sez, v in (("beta_distanza", b7), ("fedelta", phi)):
        m = re.search(r"\[" + sez + r"\]\nvalore = [^\n]*\nmin = ([\d.]+)\nmax = ([\d.]+)", t)
        if not float(m[1]) <= v <= float(m[2]):
            print(f"ATTENZIONE: {sez} = {v:.2f} fuori da [{m[1]}, {m[2]}]: allarga a mano min/max e dichiaralo", file=sys.stderr)
    open(path, "w", encoding="utf-8").write(t)


def selftest():
    z = [0.5, 2.0, 3.5, 4.5, 5.0, 3.0, 4.0, 1.5]
    vero = [0.35, 1.2] + [0.3, 0.1, 0.0, -0.4, 0.2, 0.5, -0.8, -0.2]
    th, se, _ = stima(sintetici(30000, vero, z, 20261023))
    for i, nome in ((0, "beta"), (1, "phi")):
        err = abs(th[i] - vero[i]) / vero[i]
        assert err < 0.05, f"{nome}: {th[i]:.4f} contro {vero[i]} (errore {err:.1%})"
        assert abs(th[i] - vero[i]) < 4 * se[i], f"{nome}: vero fuori da 4 errori standard"
    # ES calibrato: su 40 repliche piccole la copertura a 2 ES non può crollare
    cop = 0
    for s in range(40):
        t2, s2, _ = stima(sintetici(1500, vero, z, s))
        cop += abs(t2[0] - vero[0]) < 2 * s2[0]
    assert cop >= 32, f"copertura ES a 2 sigma: {cop}/40"
    # il ricondizionamento del toml cambia solo le due voci
    orig = ("# vecchio\n[beta_distanza]\nvalore = 1.0\nmin = 0.3\nmax = 3.0\nticket = \"14\"\n\n"
            "# vecchio\n[fedelta]\nvalore = 1.5\nmin = 0.0\nmax = 4.0\nticket = \"14\"\n")
    with tempfile.TemporaryDirectory() as d:
        p = os.path.join(d, "m.toml")
        open(p, "w").write(orig)
        scrivi(p, 0.36, 0.036, 1.1, 0.1, 100)
        nuovo = open(p).read()
    assert "valore = 1.00\n" in nuovo and "valore = 1.10\n" in nuovo and "vecchio" not in nuovo
    assert nuovo.count("min = ") == 2 and 'ticket = "14"' in nuovo
    # input malformato: errore chiaro, non traceback
    with tempfile.TemporaryDirectory() as d:
        try:
            leggi(d)
            raise AssertionError("cartella vuota accettata")
        except SystemExit as e:
            assert "POST" in str(e)
    print("selftest ok: beta", round(th[0], 4), "phi", round(th[1], 4), f"copertura 2ES {cop}/40")


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--itanes", default=CACHE, help="cartella con il post ITANES 2022 (.tab), fuori dal repo")
    ap.add_argument("--manopole", default=MANOPOLE)
    ap.add_argument("--solo-stima", action="store_true", help="non scrivere manopole.toml")
    ap.add_argument("--sensibilita", action="store_true", help="ristima con quattro scelte diverse")
    ap.add_argument("--selftest", action="store_true")
    a = ap.parse_args()
    if a.selftest:
        return selftest()
    varianti = [("base", {})]
    if a.sensibilita:
        varianti += [("posizioni proprie dei partiti", {"proprie": True}),
                     ("voto 2018 non ricordato = senza termine", {"fedele_nr": True}),
                     ("solo votanti", {"solo_votanti": True}),
                     ("non collocati al centro (x=5)", {"centro_nc": True})]
    for nome, kw in varianti:
        oss, z, sc = leggi(a.itanes, **kw)
        th, se, ll = stima(oss)
        print(f"[{nome}] N={len(oss)} scarti={sc} loglik={ll:.1f}")
        print("  z partiti:", {p: round(v, 2) for p, v in zip(PARTITI, z)})
        print(f"  beta10 = {th[0]:.4f} (ES {se[0]:.4f})  beta7 = {th[0]/SCALA:.3f} (ES {se[0]/SCALA:.3f})  phi = {th[1]:.3f} (ES {se[1]:.3f})")
        if nome == "base":
            print("  lambda:", {p: f"{v:.2f}±{s:.2f}" for p, v, s in zip(PARTITI, th[2:], se[2:])})
            base = (th[0], se[0], th[1], se[1], len(oss))
    if not a.solo_stima:
        scrivi(a.manopole, *base)
        print("scritto", a.manopole)


if __name__ == "__main__":
    main()
