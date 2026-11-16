import { describe, expect, it } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import { fuori } from "$lib/trattativa/trattativa";
import { defaultDi, fondoBasta, inEvidenza, opzioneUnica, ordineDelGiorno, righeDelVoto, risponde, motivoAccogli, motivoCrisi, scelteAlDefault, testa } from "./cruscotto";
import turnoZeroJson from "../../../../.scratch/v1-piano-11-ui/viste/01-turno-zero.json";
import trattativaJson from "../../../../.scratch/v1-piano-11-ui/viste/02-trattativa-incarico.json";
import governoJson from "../../../../.scratch/v1-piano-11-ui/viste/03-governo-ordinario.json";
import crisiJson from "../../../../.scratch/v1-piano-11-ui/viste/05-crisi.json";
import opposizioneJson from "../../../../.scratch/v1-piano-11-ui/viste/06-opposizione-governo-ia.json";
import cruscottoJson from "../../../../.scratch/v1-piano-11-ui/viste/17-cruscotto.json";
import governoSchedaJson from "../../../../.scratch/v1-piano-11-ui/viste/18-governo.json";
import aulaJson from "../../../../.scratch/v1-piano-11-ui/viste/19-aula.json";
import campagnaJson from "../../../../.scratch/v1-piano-11-ui/viste/26-campagna.json";

const turnoZero = turnoZeroJson as unknown as Vista;
const trattativa = trattativaJson as unknown as Vista;
const governo = governoJson as unknown as Vista;
const crisi = crisiJson as unknown as Vista;
const opposizione = opposizioneJson as unknown as Vista;
const sceltaFatta = (cruscottoJson as unknown as { scelte_fatte: { vista: Vista } }).scelte_fatte.vista;
const stati = cruscottoJson as unknown as Record<"ultimatum" | "esercizio_provvisorio" | "crisi_provvisorio", { vista: Vista }>;
const dopoVoto = (aulaJson as unknown as { dopo_voto: { vista: Vista } }).dopo_voto.vista;
const dichiarata = (campagnaJson as unknown as { dichiarata: Vista }).dichiarata;

describe("scelteAlDefault", () => {
  it("gli eventi senza scelta più le voci al default dell'ordine del giorno", () => {
    expect(scelteAlDefault(governo)).toBe(4);
    expect(scelteAlDefault(sceltaFatta)).toBe(2);
    expect(scelteAlDefault(stati.ultimatum.vista)).toBe(1);
    expect(scelteAlDefault(stati.esercizio_provvisorio.vista)).toBe(1);
  });
  it("la crisi e i sei interim; lo scandalo a opzione unica non conta", () => {
    expect(scelteAlDefault(crisi)).toBe(7);
  });
  it("in esercizio provvisorio la crisi non conta, il bilancio nemmeno", () => {
    expect(scelteAlDefault(stati.crisi_provvisorio.vista)).toBe(6);
  });
  it("al turno zero le proposte del governo uscente non contano", () => {
    expect(scelteAlDefault(turnoZero)).toBe(0);
  });
});

describe("ordineDelGiorno", () => {
  it("la 03: tre proposte nell'ordine dei dicasteri, ciascuna col suo default", () => {
    const p = ordineDelGiorno(governo).filter((x) => x.gruppo === "proposte");
    expect(p.map((x) => x.dicastero)).toEqual(["infrastrutture_trasporti", "ambiente_energia", "esteri_difesa"]);
    expect(p.every((x) => x.default === "in sospeso al più un turno, poi respinta" && x.alDefault && !x.fatta)).toBe(true);
  });
  it("una proposta rinviata non è al default e ha la sua riga", () => {
    const r = ordineDelGiorno(sceltaFatta).find((x) => x.dicastero === "ambiente_energia");
    expect(r).toMatchObject({ gruppo: "proposte", alDefault: false, fatta: true });
  });
  it("dopo il voto: due emendamenti aperti, il testo si vota com'è", () => {
    const e = ordineDelGiorno(dopoVoto).filter((x) => x.gruppo === "emendamenti");
    expect(e).toHaveLength(2);
    expect(e.map((x) => x.default)).toEqual(Array(2).fill("a fine turno vale respinto"));
  });
  it("l'ultimatum della Sinistra sta in «Da tenere d'occhio»", () => {
    const o = ordineDelGiorno(stati.ultimatum.vista).filter((x) => x.gruppo === "occhio");
    expect(o.map((x) => x.partito)).toEqual(["sinistra"]);
    expect(o[0].azioni).toEqual({ rimando: "partiti" });
  });
  it("il bilancio da presentare sta in «Da decidere» con il rimando; nella crisi non c'è", () => {
    expect(ordineDelGiorno(stati.esercizio_provvisorio.vista)[0]).toMatchObject({ gruppo: "da_decidere", azioni: { rimando: "bilancio" }, urgente: true });
    expect(ordineDelGiorno(stati.crisi_provvisorio.vista).some((x) => x.azioni === "successore")).toBe(true);
    expect(ordineDelGiorno(stati.crisi_provvisorio.vista).some((x) => typeof x.azioni === "object")).toBe(false);
  });
  it("i sei interim della crisi: una voce ciascuno, senza ministro", () => {
    expect(ordineDelGiorno(crisi).filter((x) => x.azioni === "successore")).toHaveLength(6);
  });
  it("fuori dal governo niente", () => {
    expect(ordineDelGiorno(turnoZero)).toEqual([]);
    expect(ordineDelGiorno(opposizione)).toEqual([]);
  });
});

describe("fondoBasta", () => {
  it("falso sulla 03: fondo 0, evento da un miliardo", () => {
    expect(fondoBasta(governo, governo.eventi[0])).toBe(false);
  });
  it("vero se il fondo copre il costo", () => {
    expect(fondoBasta({ ...governo, bilancio: { ...governo.bilancio, fondo_residuo: 2e9 } }, governo.eventi[0])).toBe(true);
  });
});

describe("defaultDi e opzioneUnica", () => {
  it("l'opzione col campo default", () => {
    expect(defaultDi(governo.eventi[0])?.id).toBe("fondo");
    expect(defaultDi({ ...governo.eventi[0], opzioni: [] })).toBeNull();
  });
  it("opzione unica: lo scandalo della crisi sì, l'emergenza no", () => {
    expect(opzioneUnica(crisi.eventi[0])).toBe(true);
    expect(opzioneUnica(governo.eventi[0])).toBe(false);
  });
});

describe("risponde", () => {
  it("il giocatore che governa, non in trattativa né all'opposizione", () => {
    expect(risponde(governo)).toBe(true);
    expect(risponde(trattativa)).toBe(false);
    expect(risponde(opposizione)).toBe(false);
  });
});

describe("righeDelVoto", () => {
  it("al turno zero: nessun dichiarato, due disponibili, nessuna spesa", () => {
    const r = righeDelVoto(turnoZero);
    expect(r.coalizione.dichiarati).toEqual([]);
    expect(r.coalizione.disponibili).toEqual(["Centrodestra", "Destra"]);
    expect(r.campagna).toEqual({ spese: 0, euro: 0 });
  });
  it("coalizione dichiarata: i due nomi", () => {
    expect(righeDelVoto(dichiarata).coalizione.dichiarati).toEqual(["Centrodestra", "Destra"]);
  });
  it("le spese sommano gli euro", () => {
    const spesa = { bersaglio: { tipo: "regione", nome: "Lazio" }, leva: "comizi", euro: 200000 };
    const v = { ...turnoZero, campagna: { attiva: true, spese: [spesa, spesa, spesa] } } as unknown as Vista;
    expect(righeDelVoto(v).campagna).toEqual({ spese: 3, euro: 600000 });
  });
});

describe("testa", () => {
  const c = cruscottoJson as unknown as Record<"crisi_provvisorio" | "dimissioni", { vista: Vista }>;
  it("crisi da allargare, crisi in esercizio provvisorio, dimissioni annunciate, o niente", () => {
    expect(testa(crisi)).toBe("crisi");
    expect(testa(c.crisi_provvisorio.vista)).toBe("crisi_provvisorio");
    expect(testa(c.dimissioni.vista)).toBe("dimissioni");
    expect(testa(governo)).toBeNull();
  });
  it("chi non governa non ha riquadro", () => {
    expect(testa(opposizione)).toBeNull();
    expect(testa(turnoZero)).toBeNull();
  });
});

describe("inEvidenza", () => {
  it("sulla 03: Economia, Sviluppo economico e Ambiente, nessuno in preavviso", () => {
    const e = inEvidenza(governo);
    expect(e.map((x) => x.dicastero)).toEqual(["economia_finanze", "sviluppo_economico", "ambiente_energia"]);
    expect(e.some((x) => x.preavviso)).toBe(false);
  });
  it("i ministri con minaccia sono in preavviso", () => {
    const scontenti = (governoSchedaJson as unknown as { scontenti: { vista: Vista } }).scontenti.vista;
    const e = inEvidenza(scontenti);
    expect(e.filter((x) => x.preavviso).map((x) => x.dicastero)).toEqual(scontenti.ministeri.filter((m) => m.ministro?.minaccia).map((m) => m.dicastero));
    expect(e.filter((x) => x.preavviso).length).toBe(6);
  });
  it("fuori dal governo nessuno", () => {
    expect(inEvidenza(opposizione)).toEqual([]);
  });
});

describe("disponibili della crisi", () => {
  it("Sinistra e Centrodestra, gli altri con la causa del core", () => {
    const f = fuori(crisi);
    expect(f.filter((x) => x.disponibile).map((x) => x.id)).toEqual(["sinistra", "centrodestra"]);
    expect(f.find((x) => x.id === "destra")?.causa).toBe("guiderebbe la propria coalizione");
  });
});

describe("dimissioni date in crisi", () => {
  // il core non azzera `crisi` con Dimettiti: la Vista porta tutte e due
  const dimessa = { ...crisi, dimissioni: true } as Vista;
  it("il riquadro è quello delle dimissioni, non più l'allargamento", () => {
    expect(testa(dimessa)).toBe("dimissioni");
    expect(testa({ ...stati.crisi_provvisorio.vista, dimissioni: true } as Vista)).toBe("dimissioni");
  });
  it("la crisi non conta più fra le scelte al default", () => {
    expect(scelteAlDefault(dimessa)).toBe(scelteAlDefault(crisi) - 1);
  });
});

describe("motivoCrisi", () => {
  it("in crisi, per chi governa, il motivo; altrove niente", () => {
    expect(motivoCrisi(crisi)).toBe("in crisi: solo ordinaria amministrazione");
    expect(motivoCrisi(stati.crisi_provvisorio.vista)).toBeDefined();
    expect(motivoCrisi(governo)).toBeUndefined();
    expect(motivoCrisi(stati.esercizio_provvisorio.vista)).toBeUndefined();
    expect(motivoCrisi(opposizione)).toBeUndefined();
  });
});

// clone con un campo cambiato: le Viste campione non si toccano
const con = (v: Vista, f: (c: Vista) => void): Vista => {
  const c = structuredClone(v);
  f(c);
  return c;
};

describe("le voci decise escono dal conto", () => {
  it("un emendamento deciso non è più nell'ordine del giorno né nel conto", () => {
    const deciso = con(dopoVoto, (c) => {
      const e = c.calendario.flatMap((m) => m.emendamenti).find((x) => x.stato === "aperto")!;
      e.stato = "accolto";
    });
    expect(ordineDelGiorno(deciso).filter((x) => x.gruppo === "emendamenti").length).toBe(ordineDelGiorno(dopoVoto).filter((x) => x.gruppo === "emendamenti").length - 1);
    expect(scelteAlDefault(deciso)).toBe(scelteAlDefault(dopoVoto) - 1);
  });
  it("un successore scelto è una voce fatta, fuori dal conto", () => {
    const scelto = con(crisi, (c) => {
      c.ministeri.find((m) => m.ministro === null && m.rosa.length)!.scelta = 0;
    });
    const voce = ordineDelGiorno(scelto).find((x) => x.chiave.startsWith("suc:") && x.fatta)!;
    expect(voce).toMatchObject({ fatta: true, alDefault: false, default: "" });
    expect(scelteAlDefault(scelto)).toBe(scelteAlDefault(crisi) - 1);
  });
  it("un interim con la rosa vuota non ha voce né conta", () => {
    const vuoto = con(crisi, (c) => {
      c.ministeri.find((m) => m.ministro === null && m.rosa.length)!.rosa = [];
    });
    expect(ordineDelGiorno(vuoto).filter((x) => x.chiave.startsWith("suc:")).length).toBe(5);
    expect(scelteAlDefault(vuoto)).toBe(scelteAlDefault(crisi) - 1);
  });
  it("il bilancio presentato non ha voce e il conto scende", () => {
    const v = stati.esercizio_provvisorio.vista;
    const presentato = con(v, (c) => {
      c.bilancio.presentato = true;
    });
    expect(ordineDelGiorno(presentato).some((x) => x.chiave === "bilancio")).toBe(false);
    expect(scelteAlDefault(presentato)).toBe(scelteAlDefault(v) - 1);
  });
  it("un evento senza opzioni conta come scelta pendente (nessuna opzione unica)", () => {
    const senza = con(governo, (c) => {
      c.eventi[0].opzioni = [];
    });
    expect(opzioneUnica(senza.eventi[0])).toBe(false);
    expect(scelteAlDefault(senza)).toBe(scelteAlDefault(governo));
  });
});

describe("gli eventi contano anche per chi non governa (lettura 1 del piano)", () => {
  it("opposizione e trattativa: solo gli eventi con scelta aperta, nessuna voce dell'ordine del giorno", () => {
    for (const v of [opposizione, trattativa]) {
      expect(ordineDelGiorno(v)).toEqual([]);
      expect(scelteAlDefault(v)).toBe(v.eventi.filter((e) => !opzioneUnica(e) && e.scelta === null).length);
    }
    expect(scelteAlDefault(opposizione)).toBe(1);
  });
});

describe("motivoCrisi con le dimissioni date", () => {
  it("resta: il core rifiuta finché la crisi dura", () => {
    expect(motivoCrisi({ ...crisi, dimissioni: true } as Vista)).toBeDefined();
  });
});

describe("inEvidenza sulla crisi", () => {
  it("non lancia e non include gli interim", () => {
    const e = inEvidenza(crisi);
    expect(e.every((x) => x.ministro !== null)).toBe(true);
    const attesi = crisi.ministeri.filter((m) => m.ministro && (m.agenda?.in_evidenza || m.ministro.minaccia)).map((m) => m.dicastero);
    expect(e.map((x) => x.dicastero)).toEqual(attesi);
  });
});

describe("fondoBasta al confine", () => {
  it("il fondo uguale al costo basta", () => {
    const e = { ...governo.eventi[0], costo: 5 };
    expect(fondoBasta(con(governo, (c) => { c.bilancio.fondo_residuo = 5; }), e)).toBe(true);
    expect(fondoBasta(con(governo, (c) => { c.bilancio.fondo_residuo = 4; }), e)).toBe(false);
  });
});

describe("motivoAccogli", () => {
  it("in esercizio provvisorio il core rifiuta Accogli (anche per le riforme): motivo suo", () => {
    const m = motivoAccogli(stati.esercizio_provvisorio.vista);
    expect(m).toBeDefined();
    expect(m).not.toBe(motivoCrisi(crisi));
  });
  it("in crisi vale il motivo della crisi, anche in esercizio provvisorio", () => {
    expect(motivoAccogli(crisi)).toBe(motivoCrisi(crisi));
    expect(motivoAccogli(stati.crisi_provvisorio.vista)).toBe(motivoCrisi(crisi));
  });
  it("al governo ordinario niente", () => {
    expect(motivoAccogli(governo)).toBeUndefined();
  });
});

describe("costo della proposta", () => {
  it("in mln come nel resto dell'app, non il numero intero", () => {
    const v = con(governo, (c) => {
      c.ministeri.find((m) => m.proposta)!.proposta!.importo = 182539410;
    });
    const voce = ordineDelGiorno(v).find((x) => x.gruppo === "proposte")!;
    expect(voce.sotto).toContain("183 mln €");
    expect(voce.sotto).not.toContain("182.539.410");
  });
});
