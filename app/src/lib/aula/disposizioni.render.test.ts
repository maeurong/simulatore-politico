import { describe, expect, it } from "vitest";
import { render } from "svelte/server";
import type { Vista } from "$lib/tipi/Vista";
import ordinario from "../../../../.scratch/v1-piano-11-ui/viste/03-governo-ordinario.json";
import Disposizioni from "./Disposizioni.svelte";

describe("Disposizioni, render", () => {
  it("ogni «Cambia ›» ha il nome accessibile della sua disposizione", () => {
    const vista = ordinario as unknown as Vista;
    const { body } = render(Disposizioni, { props: { vista, bozza: [], aperta: null, onapri: () => {}, oncambia: () => {} } });
    const attese = vista.disposizioni.filter((d) => d.norma !== "elettorale");
    for (const d of attese) expect(body).toContain(`aria-label="Cambia: ${d.nome}"`);
    expect(body.match(/aria-label="Cambia: /g)).toHaveLength(attese.length);
  });
});
