import { describe, expect, it } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import type { VoceAlbo } from "$lib/tipi/VoceAlbo";
import { daAlbo, daVista, occhiello, titolo } from "./eredita";
import ereditaJson from "../../../../.scratch/v1-piano-11-ui/viste/30-eredita.json";
import edicolaJson from "../../../../.scratch/v1-piano-11-ui/viste/13-edicola.json";

const vista = ereditaJson.vista as unknown as Vista;
const voce = ereditaJson.voce_albo as unknown as VoceAlbo;
const serie = ereditaJson.serie as VoceAlbo["serie"];
const bozza = () => daVista(vista, serie, voce.nome, voce.scenario, voce.inizio);
const abbandono = (edicolaJson.albo as unknown as VoceAlbo[]).find((a) => a.esito === "Abbandono")!;

describe("daVista", () => {
  it("la bozza del 30: senza seggi, due leggi con l'etichetta, elezioni nell'ordine della Vista", () => {
    const e = bozza();
    expect(e.esito).toBe("senza_seggi");
    expect(e.bozza).toBe(true);
    expect(e.leggi).toEqual([
      { nome: "Concorrenza nei servizi", legge: "L. 79/2031" },
      { nome: "Export di armi", legge: "L. 77/2031" },
    ]);
    expect(e.elezioni.map((x) => x.data)).toEqual(vista.elezioni.map((x) => x.data));
    expect(e.elezioni[0].seggi_giocatore).toEqual([0, 0]);
    expect(e.elezioni[1].seggi_giocatore).toEqual([66, 32]);
    expect(e.elezioni[0].voti[0]).toEqual(["Estrema sinistra", vista.elezioni[0].voti[0].quota]);
  });

  it("copia i campi senza calcolare", () => {
    const e = bozza();
    expect(e.fine).toEqual(vista.data);
    expect(e.turni).toBe(vista.turno);
    expect(e.anni_al_governo).toBe(vista.eredita!.anni_al_governo);
    expect(e.riavvolgimenti).toBe(vista.eredita!.riavvolgimenti);
    expect(e.serie).toBe(serie);
  });
});

describe("daAlbo", () => {
  it("stesse leggi coi nomi e legge null, stessi indicatori della bozza", () => {
    const e = daAlbo(voce);
    expect(e.bozza).toBe(false);
    expect(e.leggi).toEqual([
      { nome: "Concorrenza nei servizi", legge: null },
      { nome: "Export di armi", legge: null },
    ]);
    expect(e.indicatori).toEqual(bozza().indicatori);
  });

  it("l'esito del formato v1 diventa quello della bozza", () => {
    expect(daAlbo(voce).esito).toBe("senza_seggi");
    expect(daAlbo(abbandono).esito).toBe("abbandono");
  });
});

describe("titolo e occhiello", () => {
  it("la stessa partita ha lo stesso titolo nella bozza e nell'albo", () => {
    expect(titolo(bozza())).toBe(titolo(daAlbo(voce)));
  });

  it("occhiello senza seggi: ultimo voto e quota del giocatore", () => {
    expect(occhiello(daAlbo(voce))).toBe("Voto di ottobre–dicembre 2031 · 2,4%, nessun seggio");
    expect(occhiello(bozza())).toBe("Voto di ottobre–dicembre 2031 · 2,4%, nessun seggio");
  });

  it("abbandono: occhiello col trimestre della fine, titolo serio col nome", () => {
    const e = daAlbo(abbandono);
    expect(occhiello(e)).toMatch(/^Partita lasciata · /);
    expect(titolo(e)).toBe(`${abbandono.nome} lascia la partita`);
  });
});
