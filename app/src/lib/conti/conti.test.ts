import { describe, expect, it } from "vitest";
import type { Vista } from "$lib/tipi/Vista";
import { distanza, grandezza, percorso, procedura, reale, sottoAttese } from "./conti";
import contiJson from "../../../../.scratch/v1-piano-11-ui/viste/21-conti.json";

const conti = contiJson as unknown as Record<string, { vista: Vista }>;
const situazione = (stato: string, nome: string) => conti[stato].vista.situazioni.find((x) => x.nome === nome)!;

describe("distanza", () => {
  it("oltre la soglia da un trimestre", () => {
    expect(distanza(situazione("vicina", "Recessione"))).toBe("oltre la soglia da un trimestre: si accende al secondo di fila");
  });
  it("accesa: quanto manca a spegnerla", () => {
    expect(distanza(situazione("studi_03", "Caro energia"))).toBe("accesa · si spegne sotto 5: mancano 7,5");
  });
  it("spenta: quanto manca ad accenderla", () => {
    expect(distanza(situazione("studi_03", "Criminalità"))).toBe("si accende sotto 85: mancano 14,7");
  });
  it("oltre 2 e trimestri 3; oltre la soglia senza oltre", () => {
    const base = { nome: "x", misura: "x", valore: -4, accensione: -1, spegnimento: 0, accesa: false, trimestri: 3, oltre: 2 };
    expect(distanza(base)).toBe("oltre la soglia da due trimestri: si accende al terzo di fila");
    expect(distanza({ ...base, oltre: 0 })).toBe("oltre la soglia");
  });
});

describe("procedura e percorso", () => {
  it("sospesa non è rossa, inadempiente sì", () => {
    expect(procedura(conti.studi_03.vista.regole_ue).male).toBe(false);
    expect(procedura(conti.pde.vista.regole_ue).male).toBe(true);
    expect(procedura(conti.energia.vista.regole_ue).male).toBe(true);
  });
  it("percorso", () => {
    expect(percorso(-0.27)).toBe("0,27 punti sotto il percorso");
    expect(percorso(0)).toBe("sul percorso");
    expect(percorso(0.6)).toBe("0,60 punti sopra il percorso");
  });
});

describe("reale, grandezza, sottoAttese", () => {
  it("la perdita reale e la deflazione", () => {
    expect(reale(1.0)).toEqual({ testo: "−1,0%", tono: "male" });
    expect(reale(-1.0)).toEqual({ testo: "+1,0%", tono: "bene" });
    expect(reale(0.04)).toEqual({ testo: "0,0%", tono: "neutro" });
  });
  it("le tre regionali danno da/a, le altre il valore", () => {
    const gs = conti.studi_03.vista.grandezze;
    const regionali = gs.filter((g) => "da" in grandezza(g)).map((g) => g.id);
    expect(regionali).toEqual(["stock_infrastrutture", "investimenti_privati", "domanda_regionale"]);
    expect(gs.filter((g) => "valore" in grandezza(g)).length).toBe(gs.length - 3);
  });
  it("sotto le attese", () => {
    expect(sottoAttese({ fascia: "bassa", reddito: 1, crescita: 0.1, attesa: 0.3 } as never)).toBe(true);
    expect(sottoAttese({ fascia: "bassa", reddito: 1, crescita: 0.3, attesa: 0.3 } as never)).toBe(false);
  });
});
