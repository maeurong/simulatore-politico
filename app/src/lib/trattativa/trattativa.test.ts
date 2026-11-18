import { beforeEach, describe, expect, it } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import type { Comando } from "$lib/tipi/Comando";
import { aChi, allarga, cambiata, defaultDellaChiusura, fuori, haLaMaggioranza, mandaRibaltone, passaggio, predefinita, proponi, quota, ribaltoneDi, richiesteDi, risultato, type EsitoManda } from "./trattativa";
import turnoZeroJson from "../../../../.scratch/v1-piano-11-ui/viste/01-turno-zero.json";
import trattativaJson from "../../../../.scratch/v1-piano-11-ui/viste/02-trattativa-incarico.json";
import campioneJson from "../../../../.scratch/v1-piano-11-ui/viste/27-trattativa.json";

const turnoZero = turnoZeroJson as unknown as Vista;
const v02 = trattativaJson as unknown as Vista;
const campione = campioneJson as unknown as { premio: { vista: Vista; proposte: { nome: string; vista: Vista }[] }; senza_maggioranza: { vista: Vista; allarga_tutti: { vista: Vista } }; guida: { vista: Vista }; capo_dello_stato: { vista: Vista } };
const proposta = (nome: string) => campione.premio.proposte.find((p) => p.nome === nome)!.vista;
type Voci = { dicastero: string; partito: string | null }[];


describe("aChi", () => {
  it("il giocatore, il tecnico, poi gli alleati delle richieste", () => {
    expect(aChi(v02)).toEqual(["giocatore", null, "centrodestra", "destra"]);
  });
});

describe("spartizione in corso", () => {
  const v = campione.premio.vista;
  it("quota: dicasteri e punti dai peso della Vista", () => {
    const mio = v.spartizione.filter((d) => d.partito === "giocatore");
    expect(quota(v, "giocatore")).toEqual({ n: mio.length, punti: mio.reduce((s, d) => s + d.peso, 0) });
    expect(quota(v, "nessuno")).toEqual({ n: 0, punti: 0 });
  });
  it("cambiata: falso sul default, vero dove una riga se ne discosta", () => {
    expect(cambiata(v)).toBe(false);
    expect(cambiata(proposta("avara_destra"))).toBe(true);
  });
  it("proponi: dieci voci nell'ordine della Vista, cambiato solo quel dicastero (anche al tecnico)", () => {
    const c = proponi(v, "economia_finanze", "giocatore") as { dicasteri: Voci };
    expect(c.dicasteri).toHaveLength(10);
    expect(c.dicasteri.map((d) => d.dicastero)).toEqual(v.spartizione.map((d) => d.dicastero));
    const diverse = c.dicasteri.filter((d, i) => d.partito !== v.spartizione[i].partito);
    expect(diverse).toEqual([{ dicastero: "economia_finanze", partito: "giocatore" }]);
    expect((proponi(v, "economia_finanze", null) as { dicasteri: Voci }).dicasteri[0].partito).toBeNull();
  });
  it("predefinita: ogni riga al suo di_default, e dalla spartizione di default non cambia niente", () => {
    const c = predefinita(proposta("avara_destra")) as { tipo: string; dicasteri: Voci };
    expect(c.tipo).toBe("proponi_spartizione");
    expect(c.dicasteri.map((d) => d.partito)).toEqual(v.spartizione.map((d) => d.di_default));
  });
});

describe("richiesteDi", () => {
  it("02: richiesta, offerta e scarto per alleato, a due decimali", () => {
    const [cd, d] = richiesteDi(v02);
    expect([cd.id, cd.offerta]).toEqual(["centrodestra", 2]);
    expect(cd.richiesta).toBeCloseTo(2.51, 2);
    expect(cd.scarto).toBeCloseTo(-0.51, 2);
    expect([d.id, d.offerta]).toEqual(["destra", 7]);
    expect(d.richiesta).toBeCloseTo(6.79, 2);
    expect(d.scarto).toBeCloseTo(0.21, 2);
    expect(cd.nome).toBe("Centrodestra");
  });
  it("intesa e stato: la Destra a 32,1 è sotto l'ultimatum (40), non sotto l'uscita (25)", () => {
    const d = richiesteDi(proposta("avara_destra")).find((x) => x.id === "destra")!;
    expect(d.intesa).toBeCloseTo(32.1, 1);
    expect(d.stato).toBe("ultimatum");
    expect(richiesteDi(campione.premio.vista).map((x) => x.stato)).toEqual(["ok", "ok"]);
  });
  it("le soglie sono strette: a 40 giusti è ok, a 25 giusti è ultimatum, sotto 25 uscita", () => {
    const con = (intesa: number) => richiesteDi({ ...v02, richieste: [{ ...v02.richieste[0], intesa_partenza: intesa }], soglie: { ...v02.soglie, ultimatum: 40, uscita: 25 } })[0].stato;
    expect([con(40), con(39.9), con(25), con(24.9)]).toEqual(["ok", "ultimatum", "ultimatum", "uscita"]);
  });
});

describe("ribaltoneDi", () => {
  it("guida: Tu, Sinistra, Centrosinistra, Centro nell'ordine del core, la Sinistra primo", () => {
    const r = ribaltoneDi(campione.guida.vista)!;
    expect(r.map((x) => x.id)).toEqual(["giocatore", "sinistra", "centrosinistra", "centro"]);
    expect(r.map((x) => x.nome)).toEqual(["Tu", "Sinistra", "Centrosinistra", "Centro"]);
    expect(r.filter((x) => x.primo).map((x) => x.id)).toEqual(["sinistra"]);
    expect(r[1]).toMatchObject({ camera: 134, senato: 71 });
  });
  it("fuori dal ribaltone è null", () => {
    expect(ribaltoneDi(v02)).toBeNull();
  });
});

describe("risultato", () => {
  it("02: affluenza, premio di tre membri, seggi e soglie della coalizione", () => {
    const r = risultato(v02)!;
    expect(r.affluenza).toBeCloseTo(0.714, 3);
    expect(r.premio).toHaveLength(3);
    expect(r.camera).toEqual({ seggi: 224, soglia: 201 });
    expect(r.senato).toEqual({ seggi: 115, soglia: 101 });
  });
  it("le quote portano il nome e quali sono della coalizione", () => {
    const r = risultato(v02)!;
    expect(r.voti.filter((x) => x.miei).map((x) => x.partito).sort()).toEqual(["centrodestra", "destra", "giocatore"]);
    expect(r.voti.every((x) => x.nome.length > 0)).toBe(true);
  });
  it("senza voto letto è null", () => {
    expect(risultato(turnoZero)).toBeNull();
  });
});

describe("haLaMaggioranza e fuori", () => {
  const senza = campione.senza_maggioranza.vista;
  it("vera sulla 02, falsa senza maggioranza", () => {
    expect(haLaMaggioranza(v02)).toBe(true);
    expect(haLaMaggioranza(senza)).toBe(false);
  });
  it("fuori: i disponibili e no, coi seggi dell'ultimo voto e la causa del core", () => {
    const f = fuori(senza);
    expect(f.filter((x) => x.disponibile).map((x) => x.id)).toEqual(["centrosinistra", "centro"]);
    expect(f.filter((x) => !x.disponibile).map((x) => x.id)).toEqual(["estrema_sinistra", "sinistra", "estrema_destra"]);
    expect(f.find((x) => x.id === "centro")).toMatchObject({ nome: "Centro", camera: 17, senato: 6, causa: "più probabilità di governare con te" });
    expect(f.find((x) => x.id === "sinistra")).toMatchObject({ camera: 147, senato: 76, causa: "posizione troppo lontana dalla tua" });
  });
  it("fuori senza elezioni: i partiti restano, con zero seggi", () => {
    const f = fuori({ ...senza, elezioni: [] });
    expect(f.map((x) => x.id)).toEqual(fuori(senza).map((x) => x.id));
    expect(f.every((x) => x.camera === 0 && x.senato === 0)).toBe(true);
  });
});

describe("cambiata dopo l'allargamento", () => {
  it("con tutti dentro la spartizione è quella di default: nessuna riga cambiata", () => {
    const v = campione.senza_maggioranza.allarga_tutti.vista;
    expect(cambiata(v)).toBe(false);
    expect(v.spartizione.every((d) => d.partito === d.di_default)).toBe(true);
  });
});

describe("allarga", () => {
  it("è il comando del core", () => expect(allarga(["centro"])).toEqual({ tipo: "allarga", partiti: ["centro"] }));
});

describe("defaultDellaChiusura", () => {
  it("la spartizione sulla 02, la guida sulla guida, niente sul capo dello Stato", () => {
    expect(defaultDellaChiusura(v02)).toBe("cornice.senza_proposta");
    expect(defaultDellaChiusura(campione.guida.vista)).toBe("cornice.senza_scelta_guida");
    expect(defaultDellaChiusura(campione.capo_dello_stato.vista)).toBeNull();
  });
});
describe("mandaRibaltone", () => {
  beforeEach(() => { passaggio.fuoco = false; });
  const pausa = () => Promise.resolve();
  // manda a risposta manuale: finché è in volo il fuoco è atteso dalla pagina nuova
  function prova(occupato = false) {
    const mandati: Comando[] = [];
    let rispondi!: (e: EsitoManda) => void;
    const manda = (c: Comando) => { mandati.push(c); return new Promise<EsitoManda>((r) => (rispondi = r)); };
    return { mandati, manda, occupato: () => occupato, rispondi: (e: EsitoManda) => rispondi(e) };
  }

  it("riuscito: il fuoco è atteso mentre il comando è in volo, poi spento anche senza rimontaggio", async () => {
    const p = prova();
    const esito = mandaRibaltone("chiedi_guida", p.manda, p.occupato, pausa);
    expect(passaggio.fuoco).toBe(true);
    p.rispondi("ok");
    expect(await esito).toBe("ok");
    expect(p.mandati).toEqual([{ tipo: "chiedi_guida" }]);
    expect(passaggio.fuoco).toBe(false);
  });
  it("il fuoco resta fino a che la pagina nuova ha montato (`dopo`)", async () => {
    const p = prova();
    let monta!: () => void;
    const esito = mandaRibaltone("rifiuta_ribaltone", p.manda, p.occupato, () => new Promise<void>((r) => (monta = r)));
    p.rispondi("ok");
    await Promise.resolve(); await Promise.resolve();
    expect(passaggio.fuoco).toBe(true);
    monta();
    await esito;
    expect(passaggio.fuoco).toBe(false);
  });
  it("chi consuma il fuoco lo azzera: la risposta del comando non lo riaccende mentre `dopo` è ancora in attesa", async () => {
    const p = prova();
    let monta!: () => void;
    const esito = mandaRibaltone("chiedi_guida", p.manda, p.occupato, () => new Promise<void>((r) => (monta = r)));
    passaggio.fuoco = false; // l'onMount della pagina nuova
    p.rispondi("ok");
    await Promise.resolve(); await Promise.resolve();
    // qui il `finally` non ha ancora girato: se restasse acceso, lo avrebbe riacceso la risposta
    expect(passaggio.fuoco).toBe(false);
    monta();
    await esito;
    expect(passaggio.fuoco).toBe(false);
  });
  it("un `dopo` che rigetta non lascia il fuoco acceso", async () => {
    const p = prova();
    const esito = mandaRibaltone("chiedi_guida", p.manda, p.occupato, () => Promise.reject(new Error("tick")));
    p.rispondi("ok");
    await expect(esito).rejects.toThrow("tick");
    expect(passaggio.fuoco).toBe(false);
  });
  it("guasto e rifiuto: l'esito passa com'è e il fuoco si spegne", async () => {
    for (const e of ["guasto", { rifiuto: "Sinistra non ti cede la guida" }] as EsitoManda[]) {
      const p = prova();
      const esito = mandaRibaltone("chiedi_guida", p.manda, p.occupato, pausa);
      p.rispondi(e);
      expect(await esito).toEqual(e);
      expect(passaggio.fuoco).toBe(false);
    }
  });
  it("partita occupata: nessuna chiamata e il fuoco non si tocca", async () => {
    const p = prova(true);
    expect(await mandaRibaltone("chiedi_guida", p.manda, p.occupato, pausa)).toBe("occupato");
    expect(p.mandati).toEqual([]);
    expect(passaggio.fuoco).toBe(false);
  });
  it("due invii di fila: una chiamata sola, e il secondo non spegne il fuoco del primo", async () => {
    const p = prova();
    let occupato = false;
    const guarda = () => occupato;
    const manda = (c: Comando) => { occupato = true; return p.manda(c); };
    const primo = mandaRibaltone("chiedi_guida", manda, guarda, pausa);
    expect(await mandaRibaltone("chiedi_guida", manda, guarda, pausa)).toBe("occupato");
    expect(passaggio.fuoco).toBe(true);
    p.rispondi("ok");
    await primo;
    expect(p.mandati).toHaveLength(1);
    expect(passaggio.fuoco).toBe(false);
  });
});
