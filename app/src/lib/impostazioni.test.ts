import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { avvisoScala, impostazioni, PARTENZA, fattori, formaStretta, leggi, passo, scalaApplicata, scorciatoia, scrivi } from "./impostazioni.svelte";

describe("leggi e scrivi", () => {
  it("assente, illeggibile, versione ignota o fuori elenco: valori di partenza", () => {
    for (const testo of [null, "{", '{"versione":2,"scala":1.4}', '{"versione":1,"scala":3}', "null", "[]"]) {
      expect(leggi(testo)).toEqual(PARTENZA);
    }
  });
  it("andata e ritorno", () => {
    const i = { scala: 1.4, velocita: "2", movimento: "spento" } as const;
    expect(leggi(scrivi(i))).toEqual(i);
  });
});

describe("scala", () => {
  it("scalaApplicata: il gradino più alto che la finestra regge, mai sotto 1", () => {
    expect(scalaApplicata(2, 1440)).toBe(1.4);
    expect(scalaApplicata(1.4, 1280)).toBe(1.25);
    expect(scalaApplicata(1.25, 1280)).toBe(1.25);
    expect(scalaApplicata(1, 900)).toBe(1);
    expect(scalaApplicata(1.6, 1700)).toBe(1.6);
  });
  it("formaStretta sotto i 1150 px utili", () => {
    expect(formaStretta(1280, 1.25)).toBe(true);
    expect(formaStretta(1440, 1.25)).toBe(false);
    expect(formaStretta(1440, 1.4)).toBe(true);
  });
  it("passo: il gradino accanto, fermo agli estremi", () => {
    expect(passo(2, 1)).toBe(2);
    expect(passo(1, -1)).toBe(1);
    expect(passo(1.1, 1)).toBe(1.25);
  });
});

describe("fattori", () => {
  it("velocità e movimento", () => {
    expect(fattori({ ...PARTENZA, velocita: "2", movimento: "acceso" }, false)).toEqual({ k: 0.5, muove: true });
    expect(fattori({ ...PARTENZA, movimento: "sistema" }, true).muove).toBe(false);
    expect(fattori({ ...PARTENZA, velocita: "istantaneo" }, false).k).toBe(0);
  });
  it("movimento acceso muove anche col sistema che riduce", () => {
    expect(fattori({ ...PARTENZA, movimento: "acceso" }, true).muove).toBe(true);
  });
  it("movimento spento non muove mai", () => {
    expect(fattori({ ...PARTENZA, movimento: "spento" }, false).muove).toBe(false);
    expect(fattori({ ...PARTENZA, movimento: "spento" }, true).muove).toBe(false);
  });
});

describe("scorciatoia", () => {
  const tasto = (x: Partial<KeyboardEvent>) => ({ key: "+", metaKey: true, ctrlKey: false, altKey: false, repeat: false, preventDefault() {}, ...x }) as KeyboardEvent;
  it("con Alt o in ripetizione il tasto non è suo", () => {
    expect(scorciatoia(tasto({ altKey: true }))).toBe(false);
    expect(scorciatoia(tasto({ repeat: true }))).toBe(false);
  });
  describe("tasto suo", () => {
    beforeEach(() => {
      vi.stubGlobal("localStorage", { setItem() {} });
    });
    afterEach(() => {
      vi.unstubAllGlobals();
      vi.useRealTimers();
      impostazioni.scala = 1;
      avvisoScala.testo = "";
    });
    it("Cmd+ alza di un gradino, avvisa e dice che il tasto era suo", () => {
      vi.useFakeTimers();
      const prevenuto = vi.fn();
      expect(scorciatoia(tasto({ preventDefault: prevenuto }))).toBe(true);
      expect(prevenuto).toHaveBeenCalledOnce();
      expect(impostazioni.scala).toBe(1.1);
      expect(avvisoScala.testo).not.toBe("");
    });
    it("Cmd0 torna a 1×; l'avviso si spegne dopo 1,5 s", () => {
      vi.useFakeTimers();
      impostazioni.scala = 1.6;
      expect(scorciatoia(tasto({ key: "0" }))).toBe(true);
      expect(impostazioni.scala).toBe(1);
      vi.advanceTimersByTime(1500);
      expect(avvisoScala.testo).toBe("");
    });
    it("un altro tasto non è suo", () => {
      expect(scorciatoia(tasto({ key: "a" }))).toBe(false);
    });
  });
});
