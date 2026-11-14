import { describe, expect, it } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import type { Comando } from "$lib/tipi/Comando";
import { aChi, allarga, completa, defaultDellaChiusura, fuori, haLaMaggioranza, pesoDi, proponi, quota, richiesteDi, risultato, sessioneSpartizione, type Bozza, type EsitoManda, type StatoSpartizione } from "./trattativa";
import turnoZeroJson from "../../../../.scratch/v1-piano-11-ui/viste/01-turno-zero.json";
import trattativaJson from "../../../../.scratch/v1-piano-11-ui/viste/02-trattativa-incarico.json";
import campioneJson from "../../../../.scratch/v1-piano-11-ui/viste/27-trattativa.json";

const turnoZero = turnoZeroJson as unknown as Vista;
const v02 = trattativaJson as unknown as Vista;
const campione = campioneJson as unknown as { premio: { pesi: number[] }; senza_maggioranza: { vista: Vista }; guida: { vista: Vista }; capo_dello_stato: { vista: Vista } };
const ids = v02.dicasteri.map((d) => d.id);
const piena = (b: Bozza = {}): Bozza => Object.fromEntries(ids.map((id) => [id, id in b ? b[id] : "giocatore"]));

describe("pesoDi", () => {
  it("sui dieci dicasteri della 02 dà i pesi del campione", () => {
    expect(ids.map(pesoDi)).toEqual(campione.premio.pesi);
    expect(ids.map(pesoDi)).toEqual([3, 1, 1, 1, 1, 1, 1, 2, 1, 2]);
  });
});

describe("aChi", () => {
  it("il giocatore, il tecnico, poi gli alleati delle richieste", () => {
    expect(aChi(v02)).toEqual(["giocatore", null, "centrodestra", "destra"]);
  });
});

describe("completa e proponi", () => {
  const nove = Object.fromEntries(ids.slice(1).map((id) => [id, "giocatore"]));
  it("nove dicasteri assegnati non bastano, dieci sì (anche col tecnico)", () => {
    expect(completa(v02, {})).toBe(false);
    expect(completa(v02, nove)).toBe(false);
    expect(completa(v02, { ...nove, [ids[0]]: null })).toBe(true);
  });
  it("proponi è null a bozza incompleta", () => {
    expect(proponi(v02, nove)).toBeNull();
  });
  it("a bozza completa ha dieci assegnazioni nell'ordine della Vista, e il tecnico è partito null", () => {
    const c = proponi(v02, piena({ [ids[1]]: null, [ids[0]]: "destra" }));
    expect(c).toMatchObject({ tipo: "proponi_spartizione" });
    const d = (c as { dicasteri: { dicastero: string; partito: string | null }[] }).dicasteri;
    expect(d.map((x) => x.dicastero)).toEqual(ids);
    expect(d).toHaveLength(10);
    expect(d[0].partito).toBe("destra");
    expect(d[1].partito).toBeNull();
  });
});

describe("quota", () => {
  it("dicasteri e punti per titolare; i non assegnati non contano", () => {
    const b = piena({ [ids[0]]: null, [ids[1]]: "destra" });
    expect(quota(v02, b, "giocatore")).toEqual({ n: 8, punti: 10 });
    expect(quota(v02, b, null)).toEqual({ n: 1, punti: 3 });
    expect(quota(v02, b, "destra")).toEqual({ n: 1, punti: 1 });
    expect(quota(v02, {}, "giocatore")).toEqual({ n: 0, punti: 0 });
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

describe("sessioneSpartizione", () => {
  // come partita.occupato: finché un comando è in volo, un altro non parte
  function prova(inizio: Vista = v02) {
    const mandati: Comando[] = [];
    const attese: ((e: EsitoManda) => void)[] = [];
    let aperti = 0;
    let stato!: StatoSpartizione;
    let corrente = inizio;
    const s = sessioneSpartizione(
      () => corrente,
      (c) => {
        if (aperti) return Promise.resolve<EsitoManda>("occupato");
        aperti++;
        mandati.push(c);
        return new Promise<EsitoManda>((r) => attese.push((e) => { aperti--; r(e); }));
      },
      (x) => (stato = x),
    );
    const risolvi = async (e: EsitoManda) => {
      attese.shift()!(e);
      await new Promise((r) => setTimeout(r));
    };
    return { s, mandati, risolvi, vista: (x: Vista) => (corrente = x), get stato() { return stato; } };
  }
  const nove = (p: ReturnType<typeof prova>) => ids.slice(0, 9).forEach((id) => p.s.scegli(id, "giocatore"));

  it("a bozza incompleta non parte niente; alla decima parte un comando, riuscito dice «partita»", async () => {
    const p = prova();
    nove(p);
    expect(p.mandati).toHaveLength(0);
    expect(p.stato.esito).toBeNull();
    p.s.scegli(ids[9], null);
    expect(p.mandati).toHaveLength(1);
    expect(p.stato.esito).toBe("pendente");
    await p.risolvi("ok");
    expect(p.stato.esito).toBe("partita");
    expect(p.stato.rifiuto).toBeNull();
  });

  it("il rifiuto della decima riporta la bozza a incompleta, senza «partita»", async () => {
    const p = prova();
    nove(p);
    p.s.scegli(ids[9], "destra");
    await p.risolvi({ rifiuto: "Centro non è nella coalizione" });
    expect(p.stato.bozza[ids[9]]).toBeUndefined();
    expect(completa(v02, p.stato.bozza)).toBe(false);
    expect(p.stato.rifiuto).toEqual({ dicastero: ids[9], testo: "Centro non è nella coalizione" });
    expect(p.stato.esito).toBeNull();
  });

  it("il rifiuto di una modifica riporta quel dicastero alla scelta accettata", async () => {
    const p = prova();
    nove(p);
    p.s.scegli(ids[9], "giocatore");
    await p.risolvi("ok");
    p.s.scegli(ids[2], "destra");
    expect(p.stato.bozza[ids[2]]).toBe("destra");
    await p.risolvi({ rifiuto: "no" });
    expect(p.stato.bozza[ids[2]]).toBe("giocatore");
    expect(p.stato.rifiuto?.dicastero).toBe(ids[2]);
    expect(p.stato.esito).toBeNull();
    // la scelta rifiutata ricliccata riparte (non è più la scelta della bozza)
    p.s.scegli(ids[2], "destra");
    expect(p.mandati).toHaveLength(3);
  });

  it("la stessa scelta ricliccata non manda niente", async () => {
    const p = prova();
    nove(p);
    p.s.scegli(ids[9], "giocatore");
    await p.risolvi("ok");
    p.s.scegli(ids[9], "giocatore");
    expect(p.mandati).toHaveLength(1);
    expect(p.stato.esito).toBe("partita");
  });

  it("clic in volo: pendente, riparte una volta sola con l'ultima bozza", async () => {
    const p = prova();
    nove(p);
    p.s.scegli(ids[9], "giocatore");
    p.s.scegli(ids[1], "destra");
    p.s.scegli(ids[2], "centrodestra");
    expect(p.mandati).toHaveLength(1);
    expect(p.stato.esito).toBe("pendente");
    await p.risolvi("ok");
    expect(p.stato.esito).toBe("pendente");
    p.s.riprova();
    p.s.riprova();
    expect(p.mandati).toHaveLength(2);
    const d = (p.mandati[1] as { dicasteri: { dicastero: string; partito: string | null }[] }).dicasteri;
    expect(d[1].partito).toBe("destra");
    expect(d[2].partito).toBe("centrodestra");
    await p.risolvi("ok");
    expect(p.stato.esito).toBe("partita");
  });

  it("il rifiuto si azzera quando riparte il pendente", async () => {
    const p = prova();
    nove(p);
    p.s.scegli(ids[9], "giocatore");
    await p.risolvi("ok");
    p.s.scegli(ids[2], "destra");
    p.s.scegli(ids[1], "destra");
    await p.risolvi({ rifiuto: "no" });
    expect(p.stato.rifiuto).not.toBeNull();
    p.s.riprova();
    expect(p.stato.rifiuto).toBeNull();
    expect(p.mandati).toHaveLength(3);
  });

  it("un guasto riporta la scelta indietro e non dice «partita»", async () => {
    const p = prova();
    nove(p);
    p.s.scegli(ids[9], "giocatore");
    await p.risolvi("guasto");
    expect(p.stato.bozza[ids[9]]).toBeUndefined();
    expect(p.stato.esito).toBeNull();
    expect(p.stato.rifiuto).toBeNull();
  });

  it("una Vista in cui un alleato è uscito scarta le scelte che non esistono più, e non manda niente", async () => {
    const p = prova();
    nove(p);
    p.s.scegli(ids[9], "destra");
    await p.risolvi("ok");
    p.vista({ ...v02, richieste: v02.richieste.filter((r) => r.partito !== "destra") });
    p.s.aggiorna();
    expect(Object.values(p.stato.bozza)).not.toContain("destra");
    expect(completa(p.vista({ ...v02, richieste: v02.richieste.filter((r) => r.partito !== "destra") }), p.stato.bozza)).toBe(false);
    expect(p.stato.esito).toBeNull();
    expect(p.mandati).toHaveLength(1);
  });

  it("la Vista cambia con il comando in volo: l'alleato uscito non rientra e «sta partendo» non resta per sempre", async () => {
    const p = prova();
    nove(p);
    p.s.scegli(ids[9], "destra");
    p.vista({ ...v02, richieste: v02.richieste.filter((r) => r.partito !== "destra") });
    p.s.aggiorna();
    await p.risolvi("ok");
    expect(p.stato.esito).toBeNull();
    p.s.scegli(ids[9], "giocatore");
    await p.risolvi({ rifiuto: "no" });
    expect(p.stato.bozza[ids[9]]).toBeUndefined();
  });

  it("svuota: bozza vuota, niente rifiuto né esito, e una bozza nuova riparte", async () => {
    const p = prova();
    nove(p);
    p.s.scegli(ids[9], "giocatore");
    await p.risolvi("ok");
    p.s.svuota();
    expect(p.stato.bozza).toEqual({});
    expect(p.stato.esito).toBeNull();
    expect(p.stato.rifiuto).toBeNull();
    nove(p);
    p.s.scegli(ids[9], "giocatore");
    expect(p.mandati).toHaveLength(2);
  });

  it("svuota con un invio in volo: la risposta tardiva, ok o rifiuto, non tocca la sessione", async () => {
    for (const e of ["ok", { rifiuto: "no" }] as EsitoManda[]) {
      const p = prova();
      nove(p);
      p.s.scegli(ids[9], "giocatore");
      p.s.svuota();
      await p.risolvi(e);
      expect(p.stato.bozza).toEqual({});
      expect(p.stato.rifiuto).toBeNull();
      expect(p.stato.esito).toBeNull();
      // accettata è vuota: un rifiuto successivo non riporta niente alla scelta dell'invio scartato
      nove(p);
      p.s.scegli(ids[9], "destra");
      await p.risolvi({ rifiuto: "no" });
      expect(p.stato.bozza[ids[9]]).toBeUndefined();
      expect(p.stato.bozza[ids[0]]).toBe("giocatore");
    }
  });

  it("svuota con un invio che sta per riprovare: «occupato» tardivo non lascia pendente", async () => {
    const p = prova();
    nove(p);
    p.s.scegli(ids[9], "giocatore");
    p.s.scegli(ids[1], "destra");
    p.s.svuota();
    await p.risolvi("ok");
    p.s.riprova();
    expect(p.mandati).toHaveLength(1);
    expect(p.stato.esito).toBeNull();
  });
});
