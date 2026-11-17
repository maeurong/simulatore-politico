import { describe, expect, it } from "vitest";
import { render } from "svelte/server";
import type { Vista } from "$lib/tipi/Vista";
import bilancio from "../../../../.scratch/v1-piano-11-ui/viste/04-governo-bilancio.json";
import opposizione from "../../../../.scratch/v1-piano-11-ui/viste/28-opposizione.json";
import Schema from "./Schema.svelte";
import DettaglioSchema from "./DettaglioSchema.svelte";
import { schema } from "./schema";

const ordinario = bilancio as unknown as Vista;
const fine = (opposizione as unknown as Record<string, Record<string, Vista>>).fine.vista;
const nulla = () => {};

describe("Schema, render", () => {
  it.each([["04", ordinario], ["28 fine", fine]] as const)("%s: testa, «Come si legge» e il numero dei fili", (_, vista) => {
    const { body } = render(Schema, { props: { vista } });
    expect(body).toContain(`Tavola n. ${vista.turno} · Come funziona lo Stato`);
    expect(body).toContain("Come si legge");
    expect(body).toContain(`${schema(vista).fili.length} fili in questo trimestre`);
  });

  it("il filo scelto: da chi a chi, la frase intera e il rimando alla sezione; la fonte tecnica no", () => {
    const S = schema(ordinario), f = S.fili.find((x) => x.tipo === "proposta")!;
    const { body } = render(DettaglioSchema, { props: { vista: ordinario, nodi: S.nodi, fili: S.fili, scelta: { tipo: "filo", id: f.id }, id: "d", onfilo: nulla, onchiudi: nulla } });
    expect(body).toContain("→ Presidente del Consiglio");
    expect(body).toContain(f.frase.split("«")[0]);
    expect(body).toContain('href="/partita/cruscotto"');
    expect(body).not.toContain(f.fonte);
  });

  it("la riga scelta: i suoi fili col verso", () => {
    const S = schema(ordinario);
    const { body } = render(DettaglioSchema, { props: { vista: ordinario, nodi: S.nodi, fili: S.fili, scelta: { tipo: "riga", id: "pdc" }, id: "d", onfilo: nulla, onchiudi: nulla } });
    const suoi = S.fili.filter((f) => f.da === "pdc" || f.a === "pdc");
    expect(body.match(/class="ver/g)).toHaveLength(suoi.length);
    expect(body).toContain("→ Camera");
    const verso = suoi.find((f) => f.a === "pdc")!;
    expect(body).toContain(`← ${S.nodi.find((n) => n.id === verso.da)!.nome}`);
  });
});
