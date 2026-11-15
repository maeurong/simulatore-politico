import { beforeEach, describe, expect, it } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import type { Comando } from "$lib/tipi/Comando";
import { aChi, allarga, completa, defaultDellaChiusura, fuori, haLaMaggioranza, mandaRibaltone, passaggio, pesoDi, proponi, quota, richiesteDi, risultato, sessioneSpartizione, type Bozza, type EsitoManda, type StatoSpartizione } from "./trattativa";
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
  it("fuori senza elezioni: i partiti restano, con zero seggi", () => {
    const f = fuori({ ...senza, elezioni: [] });
    expect(f.map((x) => x.id)).toEqual(fuori(senza).map((x) => x.id));
    expect(f.every((x) => x.camera === 0 && x.senato === 0)).toBe(true);
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
    // l'ordine di Svelte: la partita si libera e l'effetto chiama riprova() prima che la risposta arrivi a chi l'aspetta
    const risolviERiprova = async (e: EsitoManda) => {
      // prima i clic arrivano alla sessione («occupato» li lascia pendenti), poi risponde il core
      await new Promise((r) => setTimeout(r));
      attese.shift()!(e);
      s.riprova();
      await new Promise((r) => setTimeout(r));
    };
    return { s, mandati, risolvi, risolviERiprova, vista: (x: Vista) => (corrente = x), get stato() { return stato; } };
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

  it("il rifiuto resta mentre riparte il pendente e dopo il suo ok; lo azzera il clic successivo", async () => {
    const p = prova();
    nove(p);
    p.s.scegli(ids[9], "giocatore");
    await p.risolvi("ok");
    p.s.scegli(ids[2], "destra");
    p.s.scegli(ids[1], "destra");
    await p.risolvi({ rifiuto: "no" });
    expect(p.mandati).toHaveLength(3);
    expect(p.stato.rifiuto).toEqual({ dicastero: ids[2], testo: "no" });
    p.s.riprova();
    expect(p.mandati).toHaveLength(3);
    await p.risolvi("ok");
    // l'ok del rinvio automatico non azzera il rifiuto: il giocatore non l'ha ancora letto
    expect(p.stato.rifiuto).toEqual({ dicastero: ids[2], testo: "no" });
    expect(p.stato.esito).toBe("partita");
    p.s.scegli(ids[3], "destra");
    expect(p.stato.rifiuto).toBeNull();
  });

  it("due clic pendenti dietro il rifiuto di A: se il core rifiuta il rinvio, tornano indietro tutti e due, col testo sotto l'ultimo", async () => {
    const p = prova();
    nove(p);
    p.s.scegli(ids[9], "giocatore");
    await p.risolvi("ok");
    p.s.scegli(ids[2], "destra");
    p.s.scegli(ids[1], "destra");
    p.s.scegli(ids[3], "centrodestra");
    await p.risolvi({ rifiuto: "no" });
    expect(p.mandati).toHaveLength(3);
    const d = (p.mandati[2] as { dicasteri: { partito: string | null }[] }).dicasteri;
    expect([d[1].partito, d[2].partito, d[3].partito]).toEqual(["destra", "giocatore", "centrodestra"]);
    await p.risolvi({ rifiuto: "neanche" });
    expect([p.stato.bozza[ids[1]], p.stato.bozza[ids[2]], p.stato.bozza[ids[3]]]).toEqual(["giocatore", "giocatore", "giocatore"]);
    expect(p.stato.rifiuto).toEqual({ dicastero: ids[3], testo: "neanche" });
  });

  it("chiudi() con un invio in volo e un clic pendente: la risposta non manda niente, non legge la Vista, non scrive stato", async () => {
    const p = prova();
    nove(p);
    p.s.scegli(ids[9], "giocatore");
    p.s.scegli(ids[1], "destra");
    await new Promise((r) => setTimeout(r));
    const prima = p.stato;
    p.s.chiudi();
    // dopo l'uscita all'edicola la Vista non c'è più: qui si conta chi la legge
    let letta = false;
    p.vista({ ...v02, get richieste() { letta = true; return v02.richieste; } } as Vista);
    await p.risolvi("ok");
    p.s.riprova();
    expect(letta).toBe(false);
    expect(p.mandati).toHaveLength(1);
    expect(p.stato).toBe(prima);
  });

  it("riprova() prima della risposta (l'ordine di Svelte): rifiuto di A con B pendente, B riparte senza A e il rifiuto resta sotto A", async () => {
    const p = prova();
    nove(p);
    p.s.scegli(ids[9], "giocatore");
    await p.risolvi("ok");
    p.s.scegli(ids[2], "destra");
    p.s.scegli(ids[1], "destra");
    await p.risolviERiprova({ rifiuto: "no" });
    expect(p.stato.rifiuto).toEqual({ dicastero: ids[2], testo: "no" });
    expect(p.mandati).toHaveLength(3);
    const d = (p.mandati[2] as { dicasteri: { dicastero: string; partito: string | null }[] }).dicasteri;
    expect(d[2].partito).toBe("giocatore");
    expect(d[1].partito).toBe("destra");
    // il secondo rifiuto è di B: riporta indietro B, non A, e si legge sotto B
    await p.risolvi({ rifiuto: "neanche" });
    expect(p.stato.bozza[ids[1]]).toBe("giocatore");
    expect(p.stato.bozza[ids[2]]).toBe("giocatore");
    expect(p.stato.rifiuto).toEqual({ dicastero: ids[1], testo: "neanche" });
  });

  it("riprova() prima della risposta (l'ordine di Svelte): ok con un clic pendente, riparte una volta sola con l'ultima bozza", async () => {
    const p = prova();
    nove(p);
    p.s.scegli(ids[9], "giocatore");
    p.s.scegli(ids[1], "destra");
    await p.risolviERiprova("ok");
    expect(p.mandati).toHaveLength(2);
    expect((p.mandati[1] as { dicasteri: { partito: string | null }[] }).dicasteri[1].partito).toBe("destra");
    await p.risolvi("ok");
    expect(p.stato.esito).toBe("partita");
  });

  it("una Vista in cui un alleato è entrato svuota la sessione come svuota(), anche con un clic pendente, e non manda niente", async () => {
    const p = prova();
    nove(p);
    p.s.scegli(ids[9], "giocatore");
    await p.risolvi("ok");
    // l'invio di ids[2] fa da comando in volo (nell'app è l'allargamento): il clic su ids[1] resta pendente
    p.s.scegli(ids[2], "destra");
    p.s.scegli(ids[1], "destra");
    expect(p.stato.esito).toBe("pendente");
    p.vista({ ...v02, richieste: [...v02.richieste, { ...v02.richieste[0], partito: "centro" }] });
    p.s.aggiorna();
    expect(p.stato).toEqual({ bozza: {}, rifiuto: null, esito: null, fatta: false });
    await p.risolviERiprova("ok");
    p.s.riprova();
    expect(p.mandati).toHaveLength(2);
    expect(p.stato).toEqual({ bozza: {}, rifiuto: null, esito: null, fatta: false });
    // anche accettata è vuota: un rifiuto dopo non riporta ids[9] alla scelta accettata prima dell'allargamento
    nove(p);
    p.s.scegli(ids[9], "destra");
    await p.risolvi({ rifiuto: "no" });
    expect(p.stato.bozza[ids[9]]).toBeUndefined();
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

  it("svuota: bozza vuota, niente rifiuto né esito, la proposta fatta resta detta, e una bozza nuova riparte", async () => {
    const p = prova();
    nove(p);
    expect(p.stato.fatta).toBe(false);
    p.s.scegli(ids[9], "giocatore");
    await p.risolvi("ok");
    expect(p.stato.fatta).toBe(true);
    p.s.svuota();
    expect(p.stato.fatta).toBe(true);
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
    // senza la guardia `giro` l'«occupato» tardivo qui rimette «pendente»
    expect(p.stato.esito).toBeNull();
    p.s.riprova();
    expect(p.mandati).toHaveLength(1);
    expect(p.stato.esito).toBeNull();
  });

  it("rifiuto di A con B pendente: A torna alla scelta accettata, B riparte da sé e il rifiuto di A resta fino al clic successivo", async () => {
    const p = prova();
    nove(p);
    p.s.scegli(ids[9], "giocatore");
    await p.risolvi("ok");
    p.s.scegli(ids[2], "destra");
    p.s.scegli(ids[1], "destra");
    await p.risolvi({ rifiuto: "no" });
    expect(p.stato.rifiuto).toEqual({ dicastero: ids[2], testo: "no" });
    expect(p.stato.esito).toBe("pendente");
    expect(p.stato.bozza[ids[2]]).toBe("giocatore");
    expect(p.stato.bozza[ids[1]]).toBe("destra");
    expect(p.mandati).toHaveLength(3);
    // ogni clic, anche su un'altra riga, è un gesto dopo aver letto: il rifiuto di A sparisce
    p.s.scegli(ids[3], "destra");
    expect(p.stato.rifiuto).toBeNull();
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
