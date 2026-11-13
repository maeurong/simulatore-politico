import { describe, expect, it, vi } from "vitest";
import { alLimite, base, cambiaLegge, codaRiepiloghi, editor, gradino, ritocca, scegli, ultimoVince } from "./editor.svelte";
import editorJson from "../../../../.scratch/v1-piano-11-ui/viste/14-editor.json";
import type { CatalogoScenario } from "$lib/tipi/CatalogoScenario";
import type { ErroreGuscio } from "$lib/guscio/chiamate";
import type { RiepilogoScenario } from "$lib/tipi/RiepilogoScenario";

const c = editorJson as unknown as CatalogoScenario;
const debito = c.cursori.find((x) => x.nome === "debito_pil")!;
const disposizione = (id: string) => c.disposizioni.find((d) => d.id === id)!;

describe("base", () => {
  it("i valori dei cursori e la legge di base, nessun ritocco", () => {
    const s = base(c);
    expect(s.cursori.debito_pil).toBe(136.7);
    expect(s.cursori.legge_elettorale).toBe("legge_180");
    expect(s.disposizioni).toEqual({});
  });
});

describe("ritocca", () => {
  it("sull'Italia di oggi apre un su misura nuovo senza nome", () => {
    const s = ritocca(null, c, (s) => (s.cursori.inflazione = 8));
    expect(s.nome).toBe("");
    expect(s.cursori.inflazione).toBe(8);
  });
  it("non tocca lo scenario di partenza", () => {
    const prima = base(c);
    ritocca(prima, c, (s) => (s.disposizioni.ticket = "alto"));
    expect(prima.disposizioni).toEqual({});
  });
});

describe("cambiaLegge", () => {
  it("toglie i ritocchi delle disposizioni elettorali, non gli altri", () => {
    const s = ritocca(null, c, (s) => {
      s.disposizioni.coalizioni = "no";
      s.disposizioni.ticket = "alto";
    });
    const n = cambiaLegge(s, c, "proporzionale_puro");
    expect(n.cursori.legge_elettorale).toBe("proporzionale_puro");
    expect(n.disposizioni).toEqual({ ticket: "alto" });
  });
});

describe("ogni legge del catalogo", () => {
  const leggi = Object.keys(c.leggi_elettorali);
  const elettorali = c.disposizioni.filter((d) => d.norma === "elettorale");
  it("le leggi del catalogo sono tre e le disposizioni elettorali nove", () => {
    expect(leggi).toHaveLength(3);
    expect(elettorali).toHaveLength(9);
  });
  it.each(leggi)("%s: dopo cambiaLegge nessun ritocco elettorale resta, gli altri sì", (legge) => {
    const sporco = ritocca(null, c, (s) => {
      for (const d of elettorali) s.disposizioni[d.id] = d.gradini.find((g) => g.id !== d.partenza)!.id;
      s.disposizioni.ticket = "alto";
    });
    const n = cambiaLegge(sporco, c, legge);
    expect(n.cursori.legge_elettorale).toBe(legge);
    expect(n.disposizioni).toEqual({ ticket: "alto" });
  });
  it.each(leggi)("%s: i gradini mostrati sono quelli della tabella della legge", (legge) => {
    const n = cambiaLegge(base(c), c, legge);
    for (const d of elettorali) {
      expect(gradino(n, c, d), `${legge} ${d.id}`).toBe(c.leggi_elettorali[legge][d.id]);
      expect(
        d.gradini.map((g) => g.id),
        `${legge} ${d.id}: il gradino della legge è fra quelli offerti`,
      ).toContain(c.leggi_elettorali[legge][d.id]);
    }
  });
  it.each(leggi)("%s: scegliere il gradino della legge non lascia ritocchi; un altro sì, e si rilegge", (legge) => {
    const n = cambiaLegge(base(c), c, legge);
    for (const d of elettorali) {
      const suo = c.leggi_elettorali[legge][d.id];
      expect(scegli(n, c, d, suo).disposizioni, `${legge} ${d.id}`).toEqual({});
      const altro = d.gradini.find((g) => g.id !== suo)!.id;
      const r = scegli(n, c, d, altro);
      expect(r.disposizioni, `${legge} ${d.id}`).toEqual({ [d.id]: altro });
      expect(gradino(r, c, d)).toBe(altro);
    }
  });
  it("una disposizione non elettorale parte dal suo gradino di partenza, qualunque sia la legge", () => {
    const ticket = disposizione("ticket");
    for (const legge of leggi) expect(gradino(cambiaLegge(base(c), c, legge), c, ticket)).toBe(ticket.partenza);
  });
});

describe("scegli", () => {
  it("il gradino senza ritocco toglie la voce: per le elettorali è quello della legge scelta", () => {
    const coalizioni = disposizione("coalizioni");
    const s = scegli(null, c, coalizioni, "no");
    expect(s.disposizioni).toEqual({ coalizioni: "no" });
    expect(scegli(s, c, coalizioni, "si").disposizioni).toEqual({});
    const p = cambiaLegge(base(c), c, "proporzionale_puro");
    expect(scegli(p, c, coalizioni, "no").disposizioni).toEqual({});
  });
});

describe("alLimite", () => {
  it("agli estremi della taratura", () => {
    expect(alLimite(160, debito)).toBe(true);
    expect(alLimite(110, debito)).toBe(true);
    expect(alLimite(150, debito)).toBe(false);
  });
  it.each(c.cursori.map((x) => [x.nome, x] as const))("%s: minimo e massimo sì, il passo dentro no", (_, cur) => {
    const dentro = (cur.max - cur.min) / 100;
    expect(alLimite(cur.min, cur)).toBe(true);
    expect(alLimite(cur.max, cur)).toBe(true);
    expect(alLimite(cur.min + dentro, cur)).toBe(false);
    expect(alLimite(cur.max - dentro, cur)).toBe(false);
  });
  it("l'intervallo del campione è quello della ricerca per ognuno dei sei", () => {
    expect(c.cursori.map((x) => [x.nome, x.min, x.max])).toEqual([
      ["debito_pil", 110, 160],
      ["disoccupazione", 4.7, 11.6],
      ["inflazione", 0, 8],
      ["ricchezza", -20, 20],
      ["eta_mediana", -5, 3],
      ["orientamento", -1, 1],
    ]);
  });
});

describe("ultimoVince", () => {
  // promesse risolte a mano: una per chiamata di f, nell'ordine
  function aMano() {
    const attese: { risolvi: (r: string) => void; rifiuta: (e: unknown) => void }[] = [];
    const f = vi.fn((_a: string) => new Promise<string>((risolvi, rifiuta) => attese.push({ risolvi, rifiuta })));
    const esiti: [string, string | ErroreGuscio][] = [];
    let tenuto: string | ErroreGuscio | null = null;
    const chiedi = ultimoVince(f, (r, a) => {
      esiti.push([a, r]);
      tenuto = r;
    });
    return { f, attese, esiti, chiedi, tenuto: () => tenuto };
  }
  const svuota = () => new Promise((r) => setTimeout(r, 0));
  const chiamate = (f: { mock: { calls: string[][] } }) => f.mock.calls.map((x) => x[0]);

  it("una in volo per volta; arrivata, parte l'ultima chiesta nel frattempo", async () => {
    const { f, attese, esiti, chiedi, tenuto } = aMano();
    chiedi("a");
    chiedi("b");
    chiedi("c");
    expect(chiamate(f)).toEqual(["a"]);
    attese[0].risolvi("A");
    await svuota();
    expect(chiamate(f)).toEqual(["a", "c"]);
    attese[1].risolvi("C");
    await svuota();
    expect(esiti).toEqual([["a", "A"], ["c", "C"]]);
    expect(tenuto()).toBe("C");
  });

  it("un errore di a arriva all'esito e non blocca c", async () => {
    const { f, attese, esiti, chiedi, tenuto } = aMano();
    chiedi("a");
    chiedi("c");
    attese[0].rifiuta({ specie: "rifiuto", testo: "no" });
    await svuota();
    expect(chiamate(f)).toEqual(["a", "c"]);
    attese[1].risolvi("C");
    await svuota();
    expect(esiti).toEqual([["a", { specie: "rifiuto", testo: "no" }], ["c", "C"]]);
    expect(tenuto()).toBe("C");
  });

  it("un lancio sincrono di f arriva all'esito come errore e la coda prosegue", async () => {
    const esiti: unknown[] = [];
    const f = vi.fn((a: string) => {
      if (a === "a") throw new Error("rotto");
      return Promise.resolve(a.toUpperCase());
    });
    const chiedi = ultimoVince(f, (r, a) => esiti.push([a, r]));
    chiedi("a");
    await svuota();
    expect(esiti).toEqual([["a", { specie: "guasto", testo: "Error: rotto" }]]);
    chiedi("b");
    await svuota();
    expect(chiamate(f)).toEqual(["a", "b"]);
    expect(esiti.at(-1)).toEqual(["b", "B"]);
  });

  it("senza richieste in attesa non richiama f", async () => {
    const { f, attese, chiedi } = aMano();
    chiedi("a");
    attese[0].risolvi("A");
    await svuota();
    expect(chiamate(f)).toEqual(["a"]);
  });
});

describe("codaRiepiloghi", () => {
  const riepilogo = editorJson.riepiloghi[0].riepilogo as unknown as RiepilogoScenario;
  const svuota = () => new Promise((r) => setTimeout(r, 0));

  it("l'esito arriva nello stato; il successo toglie rifiuto e guasto", async () => {
    editor.azzera();
    editor.rifiuto = "vecchio";
    editor.guasto = "vecchio";
    codaRiepiloghi(() => Promise.resolve(riepilogo))(null);
    await svuota();
    expect(editor.riepilogo).toBe(riepilogo);
    expect(editor.rifiuto).toBeNull();
    expect(editor.guasto).toBeNull();
  });

  it("un successo arrivato dopo azzera() si scarta: il riepilogo vecchio non rientra", async () => {
    editor.azzera();
    const attese: ((r: RiepilogoScenario) => void)[] = [];
    const chiedi = codaRiepiloghi(() => new Promise((risolvi) => attese.push(risolvi)));
    chiedi(null);
    editor.azzera();
    attese[0](riepilogo);
    await svuota();
    expect(editor.riepilogo).toBeNull();
    expect(editor.rifiuto).toBeNull();
    expect(editor.guasto).toBeNull();
  });

  it("dopo azzera() un esito chiesto nel giro nuovo si accetta", async () => {
    editor.azzera();
    const attese: ((r: RiepilogoScenario) => void)[] = [];
    const chiedi = codaRiepiloghi(() => new Promise((risolvi) => attese.push(risolvi)));
    chiedi(null);
    editor.azzera();
    chiedi(null);
    attese[0](riepilogo);
    await svuota();
    attese[1](riepilogo);
    await svuota();
    expect(editor.riepilogo).toBe(riepilogo);
  });

  it("un rifiuto tiene fermo l'ultimo riepilogo e spegne valido; il successo successivo lo riaccende", async () => {
    editor.azzera();
    editor.esito(riepilogo, editor.giro);
    editor.esito({ specie: "rifiuto", testo: "no" }, editor.giro);
    expect(editor.riepilogo).toBe(riepilogo);
    expect(editor.rifiuto).toBe("no");
    expect(editor.valido).toBe(false);
    editor.esito(riepilogo, editor.giro);
    expect(editor.rifiuto).toBeNull();
    expect(editor.valido).toBe(true);
  });

  it("un guasto non spegne valido: la partita resta com'era e si può ancora fondare", () => {
    editor.azzera();
    editor.esito({ specie: "guasto", testo: "rotto" }, editor.giro);
    expect(editor.guasto).toBe("rotto");
    expect(editor.valido).toBe(true);
  });

  it("un esito arrivato dopo azzera() si scarta, anche quello in coda", async () => {
    editor.azzera();
    const attese: ((e: unknown) => void)[] = [];
    const chiedi = codaRiepiloghi(() => new Promise((_, rifiuta) => attese.push(rifiuta)));
    chiedi(null);
    chiedi(base(c));
    editor.azzera();
    attese[0]({ specie: "rifiuto", testo: "no" });
    await svuota();
    attese[1]({ specie: "guasto", testo: "rotto" });
    await svuota();
    expect(editor.rifiuto).toBeNull();
    expect(editor.guasto).toBeNull();
    expect(editor.riepilogo).toBeNull();
  });
});
