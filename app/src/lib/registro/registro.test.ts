import { describe, expect, it } from "vitest";
import registroJson from "../../../../.scratch/v1-piano-11-ui/viste/25-registro.json";
import type { Vista } from "$lib/tipi/Vista";
import type { VoceEsito } from "$lib/tipi/VoceEsito";
import { cancella, collegate, corregge, passa, soggetti, turnoDa, type Filtro } from "./registro";

/* eslint-disable @typescript-eslint/no-explicit-any */
const catena = registroJson as any[];
const turni = catena.map((c, i) => turnoDa(i, c.dopo.esito as VoceEsito[]));
const vista = catena[8].dopo as Vista;
const tutte: Filtro = { criticita: "tutte", sezione: null, soggetto: null };
const voce = (tipo: string, turno: number, pred: (v: VoceEsito) => boolean = () => true) => turni[turno].voci.find((v) => v.tipo === tipo && pred(v))!;

describe("passa", () => {
  it("critiche tiene solo le critiche, notevoli anche le notevoli", () => {
    const v = turni[4].voci;
    expect(v.filter((x) => passa(x, { ...tutte, criticita: "critiche" })).every((x) => x.criticita === "critica")).toBe(true);
    expect(v.filter((x) => passa(x, { ...tutte, criticita: "critiche" })).length).toBe(turni[4].critiche);
    expect(v.filter((x) => passa(x, { ...tutte, criticita: "notevoli" })).every((x) => x.criticita !== "ordinaria")).toBe(true);
    expect(v.filter((x) => passa(x, tutte)).length).toBe(v.length);
  });
  it("la sezione guarda SEZIONE_DI", () => {
    expect(passa(voce("prima_stima", 4), { ...tutte, sezione: "conti" })).toBe(true);
    expect(passa(voce("prima_stima", 4), { ...tutte, sezione: "aula" })).toBe(false);
  });
  it("un partito passa anche da una voce fusa che lo ha solo in un dettaglio", () => {
    const fusa = voce("emendamento_respinto", 4);
    expect(fusa.partito).toBeNull();
    expect(passa(fusa, { ...tutte, soggetto: { tipo: "partito", id: "centrodestra" } })).toBe(true);
    expect(passa(fusa, { ...tutte, soggetto: { tipo: "partito", id: "sinistra" } })).toBe(false);
  });
  it("il bilancio è una misura senza disposizioni", () => {
    const f = { ...tutte, soggetto: { tipo: "bilancio", id: "3" } } as Filtro;
    expect(passa(voce("bilancio_approvato", 4), f)).toBe(true);
    expect(passa(voce("legge_respinta", 5, (v) => v.misura === 1), f)).toBe(false);
  });
});

describe("soggetti", () => {
  const s = soggetti(turni, vista);
  const nome = (tipo: string, id: string) => s.find((x) => x.soggetto.tipo === tipo && x.soggetto.id === id)?.nome;
  it("nomi di partiti, disposizioni e bilancio con l'anno del trimestre della voce", () => {
    expect(nome("partito", "centrodestra")).toBe(vista.partiti.find((p) => p.id === "centrodestra")!.nome);
    expect(nome("disposizione", "autonomia_differenziata")).toBe(vista.disposizioni.find((d) => d.id === "autonomia_differenziata")!.nome);
    expect(nome("bilancio", "3")).toBe("Legge di bilancio · 2027");
  });
  it("le regioni vengono da REGIONI", () => {
    const r = s.filter((x) => x.soggetto.tipo === "regione");
    for (const x of r) expect(x.nome).not.toBe(x.soggetto.id);
  });
  it("un id che la Vista non ha resta col suo id", () => {
    const v = { ...voce("legge_respinta", 5), disposizioni: ["sconosciuta"] };
    expect(soggetti([turnoDa(5, [v])], vista).find((x) => x.soggetto.tipo === "disposizione")?.nome).toBe("sconosciuta");
  });
});

describe("corregge", () => {
  it("la revisione del turno 4 dà la prima stima del turno 3", () => {
    const r = corregge(voce("revisione", 4), 4, turni);
    expect(r).toBe(voce("prima_stima", 3));
  });
  it("un'altra voce, o il turno zero senza turno prima, no", () => {
    expect(corregge(voce("prima_stima", 4), 4, turni)).toBeNull();
    expect(corregge(voce("revisione", 4), 4, turni.slice(4))).toBeNull();
  });
});

describe("collegate", () => {
  it("la legge respinta su autonomia_differenziata trova la stessa disposizione negli altri turni", () => {
    const v = voce("legge_respinta", 5, (x) => x.disposizioni.includes("autonomia_differenziata"));
    const c = collegate(v, 5, turni);
    expect(c.stesse.length).toBeGreaterThan(0);
    for (const [t, y] of c.stesse) {
      expect(t).not.toBe(5);
      expect(y.disposizioni.includes("autonomia_differenziata") || y.partito === v.partito).toBe(true);
    }
  });
  it("le voci fuse che hanno il soggetto solo in una riga vanno in nelleRighe", () => {
    const v = voce("intesa_variata", 4);
    const c = collegate(v, 4, turni);
    expect(c.nelleRighe.every(([t, y]) => t !== 4 && !c.stesse.some(([, z]) => z === y))).toBe(true);
    expect(c.stesse.length + c.nelleRighe.length).toBeGreaterThan(0);
  });
});

describe("cancella", () => {
  it("dal 5 all'8 quattro chiusure", () => {
    expect(cancella(5, 8, turni).chiusure).toBe(4);
    expect(cancella(8, 8, turni).chiusure).toBe(1);
  });
  it("i voti e i governi nati nel mezzo", () => {
    const t = [turnoDa(2, [{ tipo: "risultato_del_voto", tono: "neutro", criticita: "critica" } as unknown as VoceEsito]), turnoDa(3, [{ tipo: "governo_formato", criticita: "notevole" } as unknown as VoceEsito]), turnoDa(4, [])];
    expect(cancella(3, 4, t)).toEqual({ chiusure: 2, voti: [], governi: [3] });
    expect(cancella(2, 4, t)).toEqual({ chiusure: 3, voti: [2], governi: [3] });
  });
});
