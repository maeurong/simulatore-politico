import { describe, expect, it } from "vitest";
import { fondoDi, leveDi, rosa, saltiDeiLeader, VARIANTI, type Leve } from "./caricatura";
import v01 from "../../../../.scratch/v1-piano-11-ui/viste/01-turno-zero.json";
import v03 from "../../../../.scratch/v1-piano-11-ui/viste/03-governo-ordinario.json";
import v17 from "../../../../.scratch/v1-piano-11-ui/viste/17-cruscotto.json";

const chiavi = Object.keys(VARIANTI) as (keyof Leve)[];
const coppia = (l: Leve) => `${l.naso}:${l.colletto}`;
const ROSA_UGUALE = ["Luca Bianchi", "Marco Marino", "Simone Bruno"].map((nome) => ({ partito: "Partito del giocatore", nome }));

// I sette leader della Vista 01 (01-turno-zero.json, partiti[].leader).
const LEADER = [
  ["Estrema sinistra", "Anna Ferrucci"],
  ["Sinistra", "Luca Benedetti"],
  ["Centrosinistra", "Chiara Lombardo"],
  ["Centro", "Giorgio Valli"],
  ["Centrodestra", "Silvia Moretti"],
  ["Destra", "Davide Ranieri"],
  ["Estrema destra", "Bruno Castellani"],
] as const;

describe("leveDi", () => {
  it("è deterministica e ogni leva sta sotto la sua variante", () => {
    const a = leveDi("ministro", "Centro", "Mario Rossi");
    expect(leveDi("ministro", "Centro", "Mario Rossi")).toEqual(a);
    for (const k of chiavi) {
      expect(a[k]).toBeGreaterThanOrEqual(0);
      expect(a[k]).toBeLessThan(VARIANTI[k]);
    }
  });

  it("coincide con l'algoritmo del prototipo 32 (righe 393-406, calcolato eseguendolo con node)", () => {
    expect(leveDi("leader", "Centro", "Giorgio Valli")).toEqual({ naso: 4, mento: 1, fronte: 0, nuca: 0, occhio: 0, colletto: 0 });
    expect(leveDi("ministro", "Destra", "Niccolò Sarà")).toEqual({ naso: 2, mento: 2, fronte: 0, nuca: 0, occhio: 0, colletto: 2 });
    expect(leveDi("tecnico", "Sinistra", "Chiara Lombardo")).toEqual({ naso: 3, mento: 2, fronte: 1, nuca: 0, occhio: 0, colletto: 4 });
  });

  it("su cento nomi escono almeno cinque nasi e cinque colletti diversi", () => {
    const l = Array.from({ length: 100 }, (_, i) => leveDi("ministro", "Centro", `Nome Cognome${i}`));
    expect(new Set(l.map((x) => x.naso)).size).toBeGreaterThanOrEqual(5);
    expect(new Set(l.map((x) => x.colletto)).size).toBeGreaterThanOrEqual(5);
  });

  it("ruolo, partito e nome cambiano la faccia", () => {
    const base = leveDi("ministro", "Centro", "Mario Rossi");
    const altre = [leveDi("tecnico", "Centro", "Mario Rossi"), leveDi("ministro", "Destra", "Mario Rossi"), leveDi("ministro", "Centro", "Mario Russi")];
    expect(altre.some((x) => JSON.stringify(x) !== JSON.stringify(base))).toBe(true);
  });

  it("il salto passa alla combinazione successiva", () => {
    expect(leveDi("ministro", "Centro", "Mario Rossi", 1)).not.toEqual(leveDi("ministro", "Centro", "Mario Rossi", 0));
  });

  it("sui sette leader della Vista 01 nessuna coppia ha le sei leve uguali", () => {
    const l = LEADER.map(([p, n]) => JSON.stringify(leveDi("leader", p, n)));
    expect(new Set(l).size).toBe(7);
  });
});

describe("rosa", () => {
  it("tre candidati con stesso naso e colletto escono con tre coppie distinte", () => {
    const grezze = ROSA_UGUALE.map((c) => leveDi("ministro", c.partito, c.nome));
    expect(new Set(grezze.map(coppia)).size).toBe(1);
    expect(new Set(rosa(ROSA_UGUALE).map(coppia)).size).toBe(3);
  });

  it("una rosa già distinta resta com'è", () => {
    const c = LEADER.slice(0, 3).map(([partito, nome]) => ({ partito, nome }));
    expect(new Set(c.map((x) => coppia(leveDi("ministro", x.partito, x.nome)))).size).toBe(3);
    expect(rosa(c)).toEqual(c.map((x) => leveDi("ministro", x.partito, x.nome)));
  });

  it("rosa vuota", () => expect(rosa([])).toEqual([]));

  it("oltre le 36 coppie possibili termina", () => {
    const c = Array.from({ length: 40 }, (_, i) => ({ partito: "P", nome: `N${i}` }));
    expect(rosa(c)).toHaveLength(40);
  });
});

describe("saltiDeiLeader", () => {
  const s01 = saltiDeiLeader(v01);
  const facce = (v: { partiti: { id: string; nome: string; leader: string }[] }, s:Record<string, number>) =>
    v.partiti.filter((p) => p.leader).map((p) => ({ id: p.id, l: leveDi("leader", p.nome, p.leader, s[p.id]) }));

  it("sui sette leader della Vista 03 nessuna coppia ha lo stesso naso e lo stesso colletto", () => {
    const l = facce(v03, saltiDeiLeader(v03));
    expect(l).toHaveLength(7);
    expect(new Set(l.map((x) => coppia(x.l))).size).toBe(7);
  });

  it("il salto è lo stesso dalla Vista 01 e dalla 03", () => {
    expect(saltiDeiLeader(v03)).toEqual(s01);
  });

  it("chi non collide tiene la faccia del prototipo; chi collide cambia", () => {
    const mosso = Object.entries(s01).filter(([, n]) => n > 0).map(([id]) => id);
    expect(mosso.length).toBeGreaterThan(0);
    for (const x of facce(v01, s01)) {
      const p = v01.partiti.find((q) => q.id === x.id)!;
      expect(s01[x.id] > 0 ? x.l : leveDi("leader", p.nome, p.leader)).toEqual(x.l);
    }
  });

  it("il partito del giocatore non entra: rinominarlo non muove nessuno", () => {
    const rinominato = { ...v01, partiti: v01.partiti.map((p) => (p.id === v01.giocatore ? { ...p, nome: "Altro nome" } : p)) };
    expect(saltiDeiLeader(rinominato)).toEqual(s01);
    expect(s01[v01.giocatore]).toBeUndefined();
  });

  it("i nati si accodano e gli sciolti restano: i salti già dati non cambiano", () => {
    const s17 = saltiDeiLeader(v17.crisi_provvisorio.vista);
    expect(Object.keys(s17)).toHaveLength(9);
    const base = v17.crisi_provvisorio.vista.partiti.filter((p) => !p.id.startsWith("nuovo_"));
    expect(saltiDeiLeader({ ...v17.crisi_provvisorio.vista, partiti: base })).toEqual(
      Object.fromEntries(Object.entries(s17).filter(([id]) => !id.startsWith("nuovo_"))),
    );
  });
});

describe("fondoDi", () => {
  it("carta, giallo sotto 40, rosso sotto 25", () => {
    expect(fondoDi(39.9)).toBe("giallo");
    expect(fondoDi(40)).toBe("carta");
    expect(fondoDi(24.9)).toBe("rosso");
    expect(fondoDi(null)).toBe("carta");
  });
});
