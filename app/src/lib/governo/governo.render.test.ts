import { describe, expect, it, vi } from "vitest";
import { render } from "svelte/server";
import type { Vista } from "$lib/tipi/Vista";
import ordinario from "../../../../.scratch/v1-piano-11-ui/viste/03-governo-ordinario.json";
import crisi from "../../../../.scratch/v1-piano-11-ui/viste/05-crisi.json";
import grave from "../../../../.scratch/v1-piano-11-ui/viste/08-tono-grave.json";
import governo from "../../../../.scratch/v1-piano-11-ui/viste/18-governo.json";

vi.mock("$app/navigation", () => ({ goto: vi.fn() }));

import Governo from "./Governo.svelte";
import CartaMinistro from "./CartaMinistro.svelte";
import { figurine } from "./governo";

const stati = governo as unknown as Record<string, { vista: Vista }>;
const viste: [string, Vista][] = [
  ["03", ordinario as unknown as Vista],
  ["05-crisi", crisi as unknown as Vista],
  ["08-tono-grave", grave as unknown as Vista],
  ...["scelte", "interim", "scontenti", "tecnico", "rodaggio"].map((k): [string, Vista] => [`18 ${k}`, stati[k].vista]),
];

describe("Governo, render", () => {
  it.each(viste)("%s: non lancia e c'è l'album", (_, vista) => {
    const { body } = render(Governo, { props: { vista } });
    expect(body).toContain("album");
  });
  it("la crisi dice «Crisi di governo» una volta sola nella pagina", () => {
    const { body } = render(Governo, { props: { vista: crisi as unknown as Vista } });
    expect(body).not.toContain("ordinaria amministrazione");
  });
});

describe("CartaMinistro, interim", () => {
  it("tre candidati, niente Ferma, Dirotta né dimissioni", () => {
    const vista = stati.interim.vista;
    const f = figurine(vista).find((x) => x.dicastero === "ambiente_energia")!;
    const { body } = render(CartaMinistro, { props: { vista, f, grave: false, manda: async () => true, rifiuto: null, ondirotta: () => {}, anima: false, volta: false, onrigira: () => {} } });
    expect(body.match(/class="cand/g)).toHaveLength(3);
    expect(body).not.toMatch(/Ferma/);
    expect(body).not.toMatch(/Dirotta/);
    expect(body).not.toContain("Chiedi le dimissioni");
  });
});
