import { describe, expect, it } from "vitest";
import { render } from "svelte/server";
import Faccia from "./Faccia.svelte";
import Caricatura from "$lib/caricature/Caricatura.svelte";
import { nomeDi } from "$lib/partiti";
import type { Vista } from "$lib/tipi/Vista";
import v03json from "../../../../.scratch/v1-piano-11-ui/viste/03-governo-ordinario.json";

const html = (props: Record<string, unknown>) => render(Faccia, { props: { misura: 46, ...props } as never }).body;

describe("Faccia", () => {
  it("con ruolo e nome disegna la caricatura, e le iniziali restano nascoste dietro", () => {
    const h = html({ nome: "Giorgio Valli", ruolo: "ministro", partito: "Centro" });
    expect(h).toContain("<svg");
    expect(h).toContain(">GV<");
    expect(h).not.toContain("solo");
  });
  it("senza ruolo restano le iniziali, senza disegno", () => {
    const h = html({ nome: "Giorgio Valli" });
    expect(h).not.toContain("<svg");
    expect(h).toContain("solo");
    expect(h).toContain(">GV<");
  });
  it("con il ruolo ma il nome vuoto restano le iniziali (vuote), senza disegno", () => {
    const h = html({ nome: "", ruolo: "ministro" });
    expect(h).not.toContain("<svg");
    expect(h).toContain("solo");
  });
  it("il disegno è nascosto ai lettori di schermo: il nome sta già scritto accanto", () => {
    expect(html({ nome: "Giorgio Valli", ruolo: "leader", partito: "Centro" })).toMatch(/<span class="disegno[^"]*" aria-hidden="true">/);
  });
  it("un leader vero della Vista 03, con la chiave delle schede (il nome del partito), ha la faccia del prototipo 32", () => {
    const v = v03json as unknown as Vista;
    const p = v.partiti.find((x) => x.id === "centrodestra")!;
    // calcolato eseguendo con node l'algoritmo del prototipo (32-caricature.prototype.html:393-406, 588) su «leader:Centrodestra:Silvia Moretti»
    const leve = { naso: 4, mento: 1, fronte: 0, nuca: 1, occhio: 0, colletto: 1 };
    const caricatura = (props: Record<string, unknown>) => render(Caricatura, { props: { ruolo: "leader", nome: p.leader, misura: 46, ...props } as never }).body;
    const disegno = (h: string) => h.match(/<svg[\s\S]*<\/svg>/)![0];
    expect(p.leader).toBe("Silvia Moretti");
    expect(disegno(html({ nome: p.leader, ruolo: "leader", partito: nomeDi(v, p.id) }))).toBe(disegno(caricatura({ partito: "", leve })));
    // con l'id la faccia è un'altra: la chiave conta
    expect(disegno(caricatura({ partito: p.id }))).not.toBe(disegno(caricatura({ partito: "", leve })));
  });
});
