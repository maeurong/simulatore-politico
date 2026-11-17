import { describe, expect, it } from "vitest";
import motti from "./pungolo.json";
import { battutaDi, chiaveDi, didascaliaDi, mottoDi, titoloDi, titoloFine, type ChiaveTitolo } from "./pungolo";
import type { VoceEsito } from "$lib/tipi/VoceEsito";
import v30 from "../../../../.scratch/v1-piano-11-ui/viste/30-eredita.json";
import v03 from "../../../../.scratch/v1-piano-11-ui/viste/03-governo-ordinario.json";

const ID = v03.dicasteri.map((d) => d.id);

describe("pungolo", () => {
  it("quaranta motti, quattro per ciascuno dei dieci dicasteri, nell'ordine della Vista", () => {
    expect(Object.keys(motti.motti)).toEqual(ID);
    expect(ID).toHaveLength(10);
    for (const id of ID) expect((motti.motti as Record<string, string[]>)[id]).toHaveLength(4);
  });

  it("nessun motto contiene una cifra", () => {
    for (const m of Object.values(motti.motti).flat()) expect(m).not.toMatch(/\d/);
  });

  it("mottoDi è deterministico e sta fra i motti del dicastero", () => {
    const m = mottoDi("Simone Gallo", "economia_finanze");
    expect(motti.motti.economia_finanze).toContain(m);
    expect(mottoDi("Simone Gallo", "economia_finanze")).toBe(m);
  });

  it("un dicastero senza motti dà null", () => {
    expect(mottoDi("Simone Gallo", "nessuno")).toBeNull();
  });
});

describe("titoli della fine", () => {
  const titoli = motti.titoli_fine;

  it("sedici titoli", () => {
    expect(titoli).toHaveLength(16);
  });

  it("nessuna cifra e nessun nome di partito dei dati", () => {
    const nomi = v30.vista.partiti.map((p) => p.nome.toLowerCase());
    for (const x of titoli) {
      expect(x).not.toMatch(/\d/);
      for (const n of nomi) expect(x.toLowerCase()).not.toContain(n);
    }
  });

  it("titoloFine è deterministico, sta fra i sedici e dipende dalla partita", () => {
    const x = titoloFine("Concordia", "Italia di oggi", { anno: 2026, trimestre: 4 });
    expect(titoli).toContain(x);
    expect(titoloFine("Concordia", "Italia di oggi", { anno: 2026, trimestre: 4 })).toBe(x);
    const visti = new Set(Array.from({ length: 40 }, (_, i) => titoloFine(`P${i}`, "Italia di oggi", { anno: 2026, trimestre: 4 })));
    expect(visti.size).toBeGreaterThan(4);
  });
});

const voce = (tipo: string, famiglia: string | null = null) => ({ tipo, famiglia }) as unknown as VoceEsito;
const TITOLI = motti.titoli as Record<ChiaveTitolo, string[]>;
const TIPI = Object.values(import.meta.glob("../tipi/TipoVoce.ts", { query: "?raw", import: "default", eager: true }))[0] as string;
const tipiVoce = () => [...TIPI.split("export type TipoVoce =")[1].matchAll(/"([a-z_]+)"/g)].map((m) => m[1]);

describe("titoli della prima pagina", () => {
  it("ogni TipoVoce ha la sua chiave, gli eventi una per famiglia: 100 chiavi, 79 con testi, 182 titoli", () => {
    for (const t of tipiVoce()) {
      if (t === "evento_nato" || t === "evento_risolto") {
        for (const f of ["situazione", "emergenza", "scandalo", "shock", "debito"]) expect(TITOLI).toHaveProperty([`${t}|${f}`]);
      } else expect(TITOLI).toHaveProperty([t]);
    }
    const chiavi = Object.values(TITOLI);
    expect(chiavi).toHaveLength(100);
    expect(chiavi.filter((x) => x.length)).toHaveLength(79);
    expect(chiavi.flat()).toHaveLength(182);
  });

  it("nessun titolo supera 55 caratteri o contiene cifre", () => {
    for (const x of Object.values(TITOLI).flat()) {
      expect(x.length).toBeLessThanOrEqual(55);
      expect(x).not.toMatch(/\d/);
    }
  });

  it("chiaveDi: gli eventi per famiglia, gli altri per tipo", () => {
    expect(chiaveDi(voce("evento_nato", "emergenza"))).toBe("evento_nato|emergenza");
    expect(chiaveDi(voce("legge_approvata"))).toBe("legge_approvata");
  });

  it("titoloDi è stabile per id, turno e posto", () => {
    const v = voce("legge_approvata");
    const x = titoloDi("a1b2", v, 5, 0);
    expect(TITOLI.legge_approvata).toContain(x);
    expect(titoloDi("a1b2", v, 5, 0)).toBe(x);
  });

  it("cambia da un turno al successivo per le chiavi con più di una variante", () => {
    for (const [k, titoli] of Object.entries(TITOLI)) {
      if (titoli.length < 2) continue;
      const [tipo, famiglia] = k.split("|");
      const v = voce(tipo, famiglia ?? null);
      expect(titoloDi("a1b2", v, 7, 0)).not.toBe(titoloDi("a1b2", v, 8, 0));
      expect(titoloDi("a1b2", v, 7, 0)).not.toBe(titoloDi("a1b2", v, 7, 1));
    }
  });

  it("l'id della partita sposta la variante", () => {
    const v = voce("legge_approvata");
    const visti = new Set(Array.from({ length: 40 }, (_, i) => titoloDi(`p${i}`, v, 5, 0)));
    expect(visti.size).toBeGreaterThan(1);
  });

  it("una chiave senza titoli dà null; così un evento senza famiglia", () => {
    expect(TITOLI.sondaggio).toEqual([]);
    expect(titoloDi("a1b2", voce("sondaggio"), 5, 0)).toBeNull();
    expect(titoloDi("a1b2", voce("evento_nato", null), 5, 0)).toBeNull();
  });
});

describe("didascalie e battute", () => {
  it("quattordici didascalie, quattordici battute", () => {
    expect(Object.keys(motti.didascalie)).toHaveLength(14);
    expect(Object.keys(motti.battute)).toHaveLength(14);
  });

  it("didascaliaDi: la frase per tipo, o per tipo e famiglia", () => {
    expect(didascaliaDi(voce("legge_approvata"))).toBe("«Passata. Adesso qualcuno la legga»");
    expect(didascaliaDi(voce("evento_nato", "emergenza"))).toBe("«Qualcuno ha visto il fondo di riserva?»");
  });

  it("didascaliaDi dà null per un tipo fuori dai quattordici", () => {
    expect(didascaliaDi(voce("spread"))).toBeNull();
    expect(didascaliaDi(voce("evento_nato", "scandalo"))).toBeNull();
  });

  it("battutaDi: al massimo 45 caratteri, minuscola iniziale, senza punto", () => {
    expect(battutaDi("spread")).toBe("i mercati non votano, ma contano");
    for (const b of Object.values(motti.battute)) {
      expect(b.length).toBeLessThanOrEqual(45);
      expect(b).toMatch(/^[a-zàèéìòù]/);
      expect(b).not.toMatch(/\.$/);
    }
  });
});
