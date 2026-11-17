import { describe, expect, it } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import { t } from "$lib/testi/t";
import turnoZero from "../../../../.scratch/v1-piano-11-ui/viste/01-turno-zero.json";
import bilancio from "../../../../.scratch/v1-piano-11-ui/viste/04-governo-bilancio.json";
import grave from "../../../../.scratch/v1-piano-11-ui/viste/08-tono-grave.json";
import opposizione from "../../../../.scratch/v1-piano-11-ui/viste/28-opposizione.json";
import { schema } from "./schema";

const zero = turnoZero as unknown as Vista;
const ordinario = bilancio as unknown as Vista;
const tono = grave as unknown as Vista;
const stati = opposizione as unknown as Record<string, Record<string, Vista>>;
const fine = stati.fine.vista;
const conSfiducia = stati.sfiducia.dopo_comando;

const tipi = (v: Vista, tipo: string) => schema(v).fili.filter((f) => f.tipo === tipo);

describe("schema, governo ordinario (04)", () => {
  const s = schema(ordinario);
  it("un filo proposta per ogni ministero con una proposta, verso il Presidente", () => {
    const attesi = ordinario.ministeri.filter((m) => m.proposta).map((m) => `d:${m.dicastero}`);
    expect(attesi).toHaveLength(6);
    expect(tipi(ordinario, "proposta").map((f) => f.da)).toEqual(attesi);
    expect(tipi(ordinario, "proposta").every((f) => f.a === "pdc")).toBe(true);
  });
  it("due fili di maggioranza, Camera e Senato", () => {
    expect(tipi(ordinario, "maggioranza").map((f) => f.a)).toEqual(["camera", "senato"]);
  });
  it("nessun filo ha un capo senza nodo", () => {
    const ids = new Set(s.nodi.map((n) => n.id));
    for (const f of s.fili) {
      expect(ids.has(f.da), f.id).toBe(true);
      expect(ids.has(f.a), f.id).toBe(true);
    }
  });
  it("ordine dei nodi: Presidente primo nel governo, Mercati primo nel Paese, situazioni in fondo", () => {
    const di = (c: number) => s.nodi.filter((n) => n.colonna === c);
    expect(di(2)[0].tipo).toBe("presidente");
    expect(di(3)[0].tipo).toBe("mercati");
    expect(di(3).at(-1)!.tipo).toBe("situazione");
    expect(di(1).map((n) => n.id)).toEqual(["camera", "senato"]);
  });
  it("gli id dei fili sono unici", () => {
    expect(new Set(s.fili.map((f) => f.id)).size).toBe(s.fili.length);
  });
  it("un cantiere è tratteggiato a puntini, un'agenda con effetto ritardato è in ritardo", () => {
    expect(tipi(ordinario, "cantiere").every((f) => f.cantiere)).toBe(true);
    expect(tipi(ordinario, "agenda").length).toBeGreaterThan(0);
  });
  it("un evento in una regione parte dal nodo della regione, per nome", () => {
    const ev = tipi(ordinario, "evento");
    expect(ev.map((f) => f.da)).toEqual(["r:emilia_romagna"]);
  });
  it("solo le situazioni accese hanno un filo", () => {
    expect(tipi(ordinario, "situazione")).toHaveLength(ordinario.situazioni.filter((x) => x.accesa).length);
  });
  it("un filo spread dai Mercati all'Economia", () => {
    const f = tipi(ordinario, "spread");
    expect(f).toHaveLength(1);
    expect([f[0].da, f[0].a]).toEqual(["mer", "d:economia_finanze"]);
  });
});

describe("schema, grave (08)", () => {
  it("la minaccia di dimissioni è un filo dal ministero al Presidente", () => {
    const f = tipi(tono, "minaccia");
    expect(f.map((x) => [x.da, x.a, x.contrario])).toEqual([["d:sanita", "pdc", true]]);
  });
});

describe("schema, turno zero (01)", () => {
  const s = schema(zero);
  it("Camera e Senato spenti, il filo «si vota a fine turno»", () => {
    expect(s.nodi.filter((n) => n.tipo === "camera").map((n) => n.spento)).toEqual([true, true]);
    expect(tipi(zero, "maggioranza").map((f) => f.etichetta)).toEqual([t("schema.et_si_vota")]);
  });
  it("il governo uscente è centrodestra e destra", () => {
    expect(s.nodi.filter((n) => n.tipo === "partito" && n.alGoverno).map((n) => n.id)).toEqual(["p:centrodestra", "p:destra"]);
  });
  it("il Presidente è «uscente · chi guida»", () => {
    expect(s.nodi.find((n) => n.tipo === "presidente")!.sotto).toMatch(/^uscente · /);
  });
});

describe("schema, all'opposizione (28 fine)", () => {
  it("nessun filo agenda: la Vista non le porta", () => {
    expect(tipi(fine, "agenda")).toHaveLength(0);
  });
  it("un filo intesa per ogni voce di alleati_avversari", () => {
    expect(fine.alleati_avversari.length).toBeGreaterThan(0);
    expect(tipi(fine, "intesa").map((f) => f.da)).toEqual(fine.alleati_avversari.map((a) => `p:${a.partito}`));
  });
  it("un partito sciolto non ha riga e il suo filo non si disegna", () => {
    const v = { ...fine, partiti: fine.partiti.map((p) => (p.id === "sinistra" ? { ...p, sciolto: true } : p)) } as Vista;
    expect(tipi(v, "intesa").map((f) => f.da)).not.toContain("p:sinistra");
    expect(tipi(v, "intesa")).toHaveLength(fine.alleati_avversari.length - 1);
  });
});

describe("schema, sfiducia e posto riservato (28)", () => {
  it("un filo sfiducia dal partito al Presidente e un filo posto dal partito alla Camera", () => {
    expect(tipi(conSfiducia, "sfiducia").map((f) => [f.da, f.a])).toEqual([["p:giocatore", "pdc"]]);
    expect(tipi(conSfiducia, "posto").map((f) => [f.da, f.a])).toEqual([["p:estrema_destra", "camera"]]);
  });
  it("la sfiducia a un ministro arriva al suo dicastero", () => {
    const v = { ...conSfiducia, sfiducia: { dicastero: "sanita", partito: "giocatore" } } as Vista;
    expect(tipi(v, "sfiducia").map((f) => f.a)).toEqual(["d:sanita"]);
  });
  it("senza sfiducia e senza posto nessuno dei due fili", () => {
    const v = { ...stati.sfiducia.prima, posto_riservato_di: null } as Vista;
    expect(tipi(v, "sfiducia")).toHaveLength(0);
    expect(tipi(v, "posto")).toHaveLength(0);
  });
});
