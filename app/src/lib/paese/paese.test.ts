import { describe, expect, it, vi } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import { CONDIZIONI, FASCE, LIVELLI, affluenzaPerRegione, consensoPerRegione, disposizioniDi, lista, livelloManca, media, segniDi, segniPerRegione, sessioneElettori, sondaggioDi, sondaggioLetto, type EsitoElettori } from "./paese";
import type { ElettoreCampione } from "$lib/tipi/ElettoreCampione";
import { coalizioneDi } from "$lib/partiti";
import turnoZeroJson from "../../../../.scratch/v1-piano-11-ui/viste/01-turno-zero.json";
import paeseJson from "../../../../.scratch/v1-piano-11-ui/viste/23-paese.json";

const zero = turnoZeroJson as unknown as Vista;
const campione = paeseJson as unknown as Record<string, { vista: Vista }>;
const governo = campione.governo.vista;
const grave = campione.grave.vista;

describe("livelloManca", () => {
  it("consenso e affluenza hanno i dati", () => {
    expect(livelloManca("consenso")).toBeNull();
    expect(livelloManca("affluenza")).toBeNull();
  });
  it("gli altri quattro arrivano con la 11g", () => {
    const altri = LIVELLI.filter((l) => l !== "consenso" && l !== "affluenza");
    expect(altri).toHaveLength(4);
    for (const l of altri) expect(livelloManca(l)).toBe("11g");
  });
});

describe("consensoPerRegione", () => {
  const per = consensoPerRegione(governo, coalizioneDi(governo));
  const valori = Object.entries(per) as [string, number][];
  const ordinati = [...valori].sort((a, b) => a[1] - b[1]);
  it("la coalizione di governo va dal 37,6% della Basilicata al 69,2% della Toscana", () => {
    expect(ordinati[0][0]).toBe("basilicata");
    expect(+(ordinati[0][1] * 100).toFixed(1)).toBe(37.6);
    expect(ordinati.at(-1)![0]).toBe("toscana");
    expect(+(ordinati.at(-1)![1] * 100).toFixed(1)).toBe(69.2);
  });
  it("un partito fuori da Vista.partiti non azzera gli altri; da solo non dà dato", () => {
    const solo = consensoPerRegione(governo, ["giocatore"]);
    expect(consensoPerRegione(governo, ["giocatore", "nato_in_partita"])).toEqual(solo);
    expect(Object.values(consensoPerRegione(governo, ["nato_in_partita"])).every((x) => x === null)).toBe(true);
  });
});

describe("sondaggioLetto", () => {
  it("il privato comprato prende il posto del pubblico", () => {
    const l = sondaggioLetto(governo);
    expect(l.privato).toBe(true);
    expect(l.sondaggio).toBe(governo.acquisti.sondaggio_privato);
  });
  it("senza acquisto resta il pubblico", () => {
    const l = sondaggioLetto(zero);
    expect(l.privato).toBe(false);
    expect(l.sondaggio).toBe(zero.sondaggio);
  });
});

describe("affluenzaPerRegione", () => {
  it("venti valori fra 0 e 1", () => {
    const v = Object.values(affluenzaPerRegione(zero));
    expect(v).toHaveLength(20);
    for (const x of v) expect(x!).toBeGreaterThan(0), expect(x!).toBeLessThan(1);
  });
});

describe("segniPerRegione", () => {
  it("l'emergenza aperta va sull'id della regione che porta per nome", () => {
    expect(grave.eventi[0].regione).toBe("Lazio");
    expect(segniPerRegione(grave).lazio).toContain("emergenza");
  });
  it("solo la famiglia emergenza dà il segno: shock, scandalo e gli altri con regione no", () => {
    const altri = (["situazione", "scandalo", "shock", "debito"] as const).map((famiglia) => ({ ...grave.eventi[0], famiglia, regione: "Toscana" }));
    const v = { ...grave, eventi: [...grave.eventi, ...altri] };
    const segni = segniPerRegione(v);
    expect(segni.toscana).toBeUndefined();
    expect(segni.lazio).toEqual(["emergenza"]);
  });
  it("cantieri e atti in agenda con regione danno i loro segni", () => {
    const tutti = Object.values(segniPerRegione(governo)).flat();
    const attesi = governo.ministeri.flatMap((m) => [...(m.agenda?.regione ? ["atto"] : []), ...m.cantieri.filter((c) => c.regione).map(() => "cantiere")]);
    expect(tutti.filter((s) => s !== "emergenza").sort()).toEqual(attesi.sort());
  });
  it("senza atti, cantieri ed emergenze è vuoto", () => {
    expect(segniPerRegione(zero)).toEqual({});
  });
});

describe("lista", () => {
  const valori = consensoPerRegione(governo, ["giocatore"]);
  it("per valore è decrescente e i null stanno in fondo", () => {
    const l = lista({ ...valori, lazio: null, umbria: null }, "valore");
    expect(l).toHaveLength(20);
    expect(l.slice(-2).map((r) => r.valore)).toEqual([null, null]);
    const dati = l.slice(0, -2).map((r) => r.valore!);
    expect(dati).toEqual([...dati].sort((a, b) => b - a));
  });
  it("per nome è in ordine alfabetico italiano, col nome della regione", () => {
    const l = lista(valori, "nome");
    expect(l[0].nome).toBe("Abruzzo");
    expect(l.map((r) => r.nome)).toEqual([...l.map((r) => r.nome)].sort((a, b) => a.localeCompare(b, "it")));
    expect(l.find((r) => r.id === "valle_d_aosta")?.nome).toBe("Valle d’Aosta");
  });
});

describe("media", () => {
  it("le quote dei partiti sui votanti e l'astensione sommano a uno", () => {
    for (const v of [zero, governo]) {
      const m = media(v, sondaggioLetto(v).sondaggio);
      const votanti = m.partiti.reduce((s, p) => s + p.quota, 0);
      expect(votanti).toBeCloseTo(1, 9);
      expect(votanti * (1 - m.astensione) + m.astensione).toBeCloseTo(1, 9);
    }
  });
  it("senza istituti non c'è niente da mostrare", () => {
    expect(media(zero, { ...zero.sondaggio, istituti: [] })).toEqual({ partiti: [], astensione: 0 });
  });
});

describe("griglia condizione per fascia", () => {
  it("sei condizioni per tre fasce: diciotto caselle", () => {
    expect(CONDIZIONI).toHaveLength(6);
    expect(FASCE).toEqual(["bassa", "media", "alta"]);
    expect(CONDIZIONI.length * FASCE.length).toBe(18);
  });
});

describe("disposizioniDi", () => {
  it("in Campania valgono quelle col perimetro che la contiene, mai quelle di tutto il Paese", () => {
    const d = disposizioniDi(governo, "campania");
    expect(d.map((x) => x.id)).toEqual(["decontribuzione", "credito_zes"]);
    expect(governo.disposizioni.some((x) => x.perimetro.length === 0)).toBe(true);
    expect(d.every((x) => x.perimetro.includes("campania"))).toBe(true);
  });
  it("una regione fuori da ogni perimetro non ne ha", () => {
    expect(disposizioniDi(governo, "toscana")).toEqual([]);
  });
});

describe("sondaggioDi", () => {
  it("una voce per partito attivo, dal più al meno votato, e sommano a uno", () => {
    for (const v of [zero, governo]) {
      const s = sondaggioDi(v, "toscana");
      expect(s).toHaveLength(v.partiti.filter((p) => !p.sciolto).length);
      expect(s.reduce((a, p) => a + p.quota, 0)).toBeCloseTo(1, 9);
      expect(s.map((p) => p.quota)).toEqual([...s.map((p) => p.quota)].sort((a, b) => b - a));
    }
  });
  it("col privato comprato legge il privato, non il pubblico", () => {
    const privato = sondaggioDi(governo, "toscana");
    const pubblico = sondaggioDi({ ...governo, acquisti: { ...governo.acquisti, sondaggio_privato: null } }, "toscana");
    expect(privato.map((p) => p.quota)).not.toEqual(pubblico.map((p) => p.quota));
  });
  it("una regione sconosciuta non dà voci", () => {
    expect(sondaggioDi(zero, "atlantide")).toEqual([]);
  });
});

describe("segniDi", () => {
  it("il cantiere dell'Umbria porta il nome dell'atto, per esteso", () => {
    const s = segniDi(governo, "umbria");
    expect(s).toHaveLength(1);
    expect(s[0].segno).toBe("cantiere");
    expect(s[0].testo).toMatch(/^cantiere in corso: .+/);
  });
  it("l'emergenza delle Marche porta nome e causa", () => {
    const s = segniDi(governo, "marche");
    expect(s.map((x) => x.segno)).toEqual(["emergenza"]);
    expect(s[0].testo).toContain("Emergenza idrogeologica");
  });
  it("una regione senza segni non ne dà", () => {
    expect(segniDi(zero, "umbria")).toEqual([]);
  });
});

describe("sessioneElettori", () => {
  const e = (nome: string): ElettoreCampione => ({ nome, eta: 30, posizione: 4, voto: null, motivo: "protesta" });
  function aMano() {
    const attese: { risolvi: (r: ElettoreCampione[]) => void; rifiuta: (x: unknown) => void }[] = [];
    const leggi = vi.fn((_r: string, _c: string, _f: string) => new Promise<ElettoreCampione[]>((risolvi, rifiuta) => attese.push({ risolvi, rifiuta })));
    const esiti: EsitoElettori[] = [];
    const sessione = sessioneElettori(leggi as never, (x) => esiti.push(x));
    return { leggi, attese, esiti, sessione };
  }
  const svuota = () => new Promise((r) => setTimeout(r, 0));

  it("cambiare casella o regione in volo mostra solo l'ultima scelta", async () => {
    const { leggi, attese, esiti, sessione } = aMano();
    sessione.chiedi("abruzzo", "autonomo", "alta");
    sessione.chiedi("abruzzo", "autonomo", "bassa");
    sessione.chiedi("molise", "autonomo", "bassa");
    attese[0].risolvi([e("A")]);
    await svuota();
    expect(esiti).toEqual([]);
    expect(leggi.mock.calls.map((c) => c.join("|"))).toEqual(["abruzzo|autonomo|alta", "molise|autonomo|bassa"]);
    attese[1].risolvi([e("C")]);
    await svuota();
    expect(esiti).toEqual([{ elettori: [e("C")] }]);
  });
  it("una risposta arrivata dopo l'annulla non si mostra", async () => {
    const { attese, esiti, sessione } = aMano();
    sessione.chiedi("abruzzo", "autonomo", "alta");
    sessione.annulla();
    attese[0].risolvi([e("A")]);
    await svuota();
    expect(esiti).toEqual([]);
  });
  it("il rifiuto del core è un rifiuto senza prefisso, il guasto un guasto", async () => {
    const { attese, esiti, sessione } = aMano();
    sessione.chiedi("atlantide", "autonomo", "alta");
    attese[0].rifiuta({ specie: "rifiuto", testo: "comando non ammesso: regione sconosciuta" });
    await svuota();
    sessione.chiedi("abruzzo", "autonomo", "alta");
    attese[1].rifiuta({ specie: "guasto", testo: "core non risponde" });
    await svuota();
    expect(esiti).toEqual([{ rifiuto: "regione sconosciuta" }, { guasto: "core non risponde" }]);
  });
});
