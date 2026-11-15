import type { Vista } from "$lib/tipi/Vista";
import type { VistaSoglie } from "$lib/tipi/VistaSoglie";
import { nomeDi } from "$lib/partiti";

export type Ruolo = "ministro" | "tecnico" | "leader";
export type Leve = { naso: number; mento: number; fronte: number; nuca: number; occhio: number; colletto: number };
export const VARIANTI: Leve = { naso: 6, mento: 3, fronte: 2, nuca: 2, occhio: 2, colletto: 6 };

// djb2 + finalizzatore MurmurHash3 come nel prototipo 32 (32-caricature.prototype.html:393-406): non si usa `hash` di
// testi/t.ts (FNV-1a) perché lo stesso nome deve dare la stessa faccia del prototipo approvato.
function djb2(s: string): number {
  let h = 5381;
  for (let i = 0; i < s.length; i++) h = (((h << 5) + h + s.charCodeAt(i)) >>> 0);
  return h;
}
function mescola(h: number): number {
  h ^= h >>> 16;
  h = Math.imul(h, 0x85ebca6b) >>> 0;
  h ^= h >>> 13;
  h = Math.imul(h, 0xc2b2ae35) >>> 0;
  return (h ^ (h >>> 16)) >>> 0;
}

/** Le leve dall'hash di `ruolo:partito:nome`; `salto` passa alla combinazione successiva. */
export function leveDi(ruolo: Ruolo, partito: string, nome: string, salto = 0): Leve {
  let h = mescola(djb2(`${ruolo}:${partito}:${nome}`)) + salto;
  const leva = (b: number) => {
    const v = h % b;
    h = Math.floor(h / b);
    return v;
  };
  const { naso: n, mento: m, fronte: f, nuca: u, occhio: o, colletto: c } = VARIANTI;
  return { naso: leva(n), mento: leva(m), fronte: leva(f), nuca: leva(u), occhio: leva(o), colletto: leva(c) };
}

type Candidato = { partito: string; nome: string };

/** I salti della rosa: chi ripete naso e colletto di uno già dentro passa alla combinazione successiva, finché è libera. */
export function salti(ruolo: Ruolo | Ruolo[], candidati: Candidato[]): number[] {
  const visti = new Set<string>();
  const coppie = VARIANTI.naso * VARIANTI.colletto;
  return candidati.map((c, i) => {
    const r = Array.isArray(ruolo) ? ruolo[i] : ruolo;
    let salto = 0;
    let l = leveDi(r, c.partito, c.nome);
    // shortcut: con più candidati delle coppie possibili (36) nessuna è libera, si tiene la faccia di partenza; da rivedere se le rose superano 36.
    while (visti.size < coppie && visti.has(`${l.naso}:${l.colletto}`)) l = leveDi(r, c.partito, c.nome, ++salto);
    visti.add(`${l.naso}:${l.colletto}`);
    return salto;
  });
}

export function rosa(candidati: Candidato[], ruolo: Ruolo = "ministro"): Leve[] {
  const s = salti(ruolo, candidati);
  return candidati.map((c, i) => leveDi(ruolo, c.partito, c.nome, s[i]));
}

/**
 * Il salto di ogni leader, dalla lista dei partiti della Vista nell'ordine della Vista (gli sciolti restano in lista, i nati si
 * accodano: i salti già dati non cambiano). Se un leader cambia (successione), la sua coppia può spostare la faccia di chi viene dopo nell'ordine. Il partito del giocatore non si disegna e non entra nel conto: rinominarlo non muove nessuno.
 */
export function saltiDeiLeader(v: { giocatore: string; partiti: { id: string; nome: string; leader: string }[] }): Record<string, number> {
  const ps = v.partiti.filter((p) => p.id !== v.giocatore && p.leader);
  const s = salti("leader", ps.map((p) => ({ partito: p.nome, nome: p.leader })));
  return Object.fromEntries(ps.map((p, i) => [p.id, s[i]]));
}

/** La chiave del partito nell'hash: `giocatore` per il partito del giocatore (decisione 3: rinominarlo non cambia la faccia), il nome visualizzato per gli altri. */
export function chiaveDi(v: Vista, partito: string): string {
  return partito === v.giocatore ? "giocatore" : nomeDi(v, partito);
}

/** Il fondo del riquadro dalla soddisfazione: carta; giallo sotto `soglie.minaccia`; rosso sotto `soglie.dimissioni`. */
export function fondoDi(soddisfazione: number | null, soglie: VistaSoglie): "carta" | "giallo" | "rosso" {
  if (soddisfazione == null) return "carta";
  return soddisfazione < soglie.dimissioni ? "rosso" : soddisfazione < soglie.minaccia ? "giallo" : "carta";
}
