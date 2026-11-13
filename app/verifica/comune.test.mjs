import { describe, expect, it } from "vitest";
import { altrove, nomeValido, nonDissolvenze } from "./comune.mjs";

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

// Sul nome poggia il fatto che gli screenshot non escano da `uscita`.
describe("nomeValido", () => {
  it("accetta lettere, cifre, trattino e sottolineatura", () => {
    expect(nomeValido("edicola-vuota_2")).toBe(true);
  });

  it("rifiuta il vuoto e tutto ciò che può uscire dalla cartella", () => {
    expect(nomeValido("")).toBe(false);
    expect(nomeValido("../x")).toBe(false);
    expect(nomeValido("a/b")).toBe(false);
  });
});

// Su nonDissolvenze poggia il guasto MOVIMENTO (spec, storia 55).
describe("nonDissolvenze", () => {
  it("le dissolvenze di opacity restano ammesse", () => {
    expect(nonDissolvenze(["opacity", "opacity"])).toEqual([]);
    expect(nonDissolvenze([])).toEqual([]);
  });

  it("ogni altra proprietà è fuori, una volta sola", () => {
    expect(nonDissolvenze(["opacity", "transform", "transform"])).toEqual(["transform"]);
  });
});
