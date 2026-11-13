import { describe, expect, it } from "vitest";
import { altrove } from "./comune.mjs";

// Su altrove poggia il codice d'uscita 1 «pagina finita altrove» delle due batterie.
describe("altrove", () => {
  it("non è altrove se il percorso d'arrivo è quello chiesto, con o senza query", () => {
    expect(altrove("/scenario", "/scenario")).toBe(false);
    expect(altrove("/?vista=13-edicola.json", "/")).toBe(false);
    expect(altrove("/partita/cruscotto?vista=03-governo-ordinario.json", "/partita/cruscotto")).toBe(false);
  });

  it("è altrove se una Vista inesistente ha rimandato all'edicola", () => {
    expect(altrove("/partita/cruscotto?vista=non-esiste.json", "/")).toBe(true);
  });
});
