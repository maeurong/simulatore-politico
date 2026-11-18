import { describe, expect, it } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import { chiaveDi, fondoDi, leveDi, rosa, salti, saltiDeiLeader, VARIANTI, type Leve } from "./caricatura";
import v01 from "../../../../.scratch/v1-piano-11-ui/viste/01-turno-zero.json";
import v03 from "../../../../.scratch/v1-piano-11-ui/viste/03-governo-ordinario.json";
import v30j from "../../../../.scratch/v1-piano-11-ui/viste/30-eredita.json";
import v17 from "../../../../.scratch/v1-piano-11-ui/viste/17-cruscotto.json";

const chiavi = Object.keys(VARIANTI) as (keyof typeof VARIANTI)[];
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
    expect(leveDi("leader", "Centro", "Giorgio Valli")).toMatchObject({ naso: 4, mento: 1, fronte: 0, nuca: 0, occhio: 0, colletto: 0 });
    expect(leveDi("ministro", "Destra", "Niccolò Sarà")).toMatchObject({ naso: 2, mento: 2, fronte: 0, nuca: 0, occhio: 0, colletto: 2 });
    expect(leveDi("tecnico", "Sinistra", "Chiara Lombardo")).toMatchObject({ naso: 3, mento: 2, fronte: 1, nuca: 0, occhio: 0, colletto: 4 });
  });

  const chiave = (i: number) => ["ministro", "tecnico", "leader"][i % 3] as "ministro" | "tecnico" | "leader";
  const seiLeve = (l: Leve) => `${l.naso}${l.mento}${l.fronte}${l.nuca}${l.occhio}${l.colletto}`;

  it("le sei leve su 600 chiavi sono quelle di prima dei segni (FNV-1a dell'istantanea presa dal codice prima della modifica)", () => {
    const tutte = Array.from({ length: 600 }, (_, i) => seiLeve(leveDi(chiave(i), "P" + (i % 7), "Nome " + i, i % 4)));
    let f = 2166136261;
    for (const c of tutte.join(",")) f = Math.imul(f ^ c.charCodeAt(0), 16777619) >>> 0;
    expect(f).toBe(4241386166);
  });

  it("orecchino e cappello danno tutte e quattro le combinazioni su 600 chiavi", () => {
    const c = new Set(Array.from({ length: 600 }, (_, i) => { const l = leveDi(chiave(i), "P" + (i % 7), "Nome " + i); return `${l.orecchino}:${l.cappello}`; }));
    expect(c.size).toBe(4);
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

describe("salti col genere", () => {
  const stessa = (genere?: "donna" | "uomo") => ROSA_UGUALE.slice(0, 2).map((c) => ({ ...c, genere }));
  it("stesso naso e colletto, genere diverso: nessuno salta", () => {
    const [a, b] = ROSA_UGUALE;
    expect(salti("ministro", [{ ...a, genere: "uomo" }, { ...b, genere: "donna" }])).toEqual([0, 0]);
  });
  it("due donne uguali: la seconda salta; senza genere vale uomo", () => {
    expect(salti("ministro", stessa("donna"))[1]).toBeGreaterThan(0);
    expect(salti("ministro", stessa())).toEqual(salti("ministro", stessa("uomo")));
  });
});

describe("saltiDeiLeader", () => {
  const s03 = saltiDeiLeader(v03);
  const leader = (v: { partiti: { id: string; nome: string; leader: string; genere_leader?: string | null }[] }, s: Record<string, number>) =>
    v.partiti.filter((p) => p.leader).map((p) => ({ id: p.id, g: p.genere_leader, l: leveDi("leader", p.nome, p.leader, s[p.id]) }));

  it("sui sette leader della Vista 03 nessuna coppia ha lo stesso naso, colletto e genere", () => {
    const l = leader(v03, s03);
    expect(l).toHaveLength(7);
    expect(new Set(l.map((x) => `${coppia(x.l)}:${x.g}`)).size).toBe(7);
  });

  it("salti fissi: chi non collide (a pari genere) tiene la faccia del prototipo, gli altri passano oltre", () => {
    expect(s03).toEqual({ estrema_sinistra: 0, sinistra: 0, centrosinistra: 0, centro: 0, centrodestra: 0, destra: 0, estrema_destra: 1 });
    const l = Object.fromEntries(leader(v03, s03).map((x) => [x.id, coppia(x.l)]));
    expect(l).toMatchObject({ estrema_sinistra: "4:0", sinistra: "5:5", centrosinistra: "1:5", centrodestra: "4:1" });
  });

  it("il partito del giocatore non entra: nome e leader suoi non muovono nessuno", () => {
    const altro = { ...v01, partiti: v01.partiti.map((p) => (p.id === v01.giocatore ? { ...p, nome: "Altro nome", leader: "Tizio Caio" } : p)) };
    const s = saltiDeiLeader(altro);
    expect(s).toEqual(s03);
    expect(s[v01.giocatore]).toBeUndefined();
  });

  it("gli sciolti restano nel conto", () => {
    const v30 = v30j.vista;
    const s = saltiDeiLeader(v30);
    expect(v30.partiti.find((p) => p.id === "estrema_sinistra")!.sciolto).toBe(true);
    expect(Object.keys(s)).toHaveLength(9);
    expect(saltiDeiLeader({ ...v30, partiti: v30.partiti.map((p) => ({ ...p, sciolto: false })) })).toEqual(s);
  });

  it("i nati si accodano: i salti già dati non cambiano", () => {
    const v = v17.crisi_provvisorio.vista;
    const s17 = saltiDeiLeader(v);
    expect(Object.keys(s17)).toHaveLength(9);
    const base = v.partiti.filter((p) => !p.id.startsWith("nuovo_"));
    expect(saltiDeiLeader({ ...v, partiti: base })).toEqual(Object.fromEntries(Object.entries(s17).filter(([id]) => !id.startsWith("nuovo_"))));
  });
});

const soglie = v03.soglie as Vista["soglie"];

describe("fondoDi", () => {
  it("carta, giallo sotto soglie.minaccia, rosso sotto soglie.dimissioni", () => {
    expect(fondoDi(39.9, soglie)).toBe("giallo");
    expect(fondoDi(40, soglie)).toBe("carta");
    expect(fondoDi(24.9, soglie)).toBe("rosso");
    expect(fondoDi(null, soglie)).toBe("carta");
  });
  it("segue le soglie della Vista, non numeri scritti nel codice", () => {
    expect(fondoDi(45, { ...soglie, minaccia: 50 })).toBe("giallo");
  });
});

describe("chiaveDi", () => {
  it("giocatore per il partito del giocatore, il nome visualizzato per gli altri", () => {
    const v = v03 as unknown as Vista;
    expect(chiaveDi(v, "giocatore")).toBe("giocatore");
    expect(chiaveDi(v, "destra")).toBe("Destra");
  });
  it("segue v.giocatore, non la parola", () => {
    const v = { ...v03, giocatore: "destra" } as unknown as Vista;
    expect(chiaveDi(v, "destra")).toBe("giocatore");
  });
});
