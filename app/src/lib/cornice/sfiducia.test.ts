import { describe, expect, it } from "vitest";
import type { AnteprimaSfiducia } from "$lib/tipi/AnteprimaSfiducia";
import type { Vista } from "$lib/tipi/Vista";
import { anteprimaPer } from "./sfiducia";

const a = { firme: true, linee: [], passa: false } as AnteprimaSfiducia;
const vista = (n: number) => ({ turno: n }) as unknown as Vista;

describe("anteprimaPer", () => {
  it("la Vista vecchia che risolve dopo la nuova non scrive", async () => {
    const [v1, v2] = [vista(1), vista(2)];
    let corrente = v1;
    let sblocca!: (x: AnteprimaSfiducia) => void;
    const vecchia = anteprimaPer(v1, () => new Promise((r) => { sblocca = r; }), () => corrente);
    corrente = v2;
    sblocca(a);
    expect(await vecchia).toBeUndefined();
  });
  it("la Vista corrente scrive la risposta", async () => {
    const v = vista(1);
    expect(await anteprimaPer(v, async () => a, () => v)).toBe(a);
  });
  it("un guasto sulla Vista corrente dà null, su una superata niente", async () => {
    const v = vista(1);
    const guasta = async () => { throw new Error("x"); };
    expect(await anteprimaPer(v, guasta, () => v)).toBeNull();
    expect(await anteprimaPer(v, guasta, () => vista(2))).toBeUndefined();
  });
});
