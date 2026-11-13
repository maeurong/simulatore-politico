import { describe, expect, it, vi } from "vitest";
import { alLimite, base, cambiaLegge, ritocca, scegli, ultimoVince } from "./editor.svelte";
import editorJson from "../../../../.scratch/v1-piano-11-ui/viste/14-editor.json";
import type { CatalogoScenario } from "$lib/tipi/CatalogoScenario";
import type { ErroreGuscio } from "$lib/guscio/chiamate";

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

  it("senza richieste in attesa non richiama f", async () => {
    const { f, attese, chiedi } = aMano();
    chiedi("a");
    attese[0].risolvi("A");
    await svuota();
    expect(chiamate(f)).toEqual(["a"]);
  });
});
