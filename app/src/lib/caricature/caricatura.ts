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

/** La rosa: chi ripete naso e colletto di uno già dentro passa alla combinazione successiva, finché è libera. */
export function rosa(candidati: { partito: string; nome: string }[]): Leve[] {
  const visti = new Set<string>();
  const coppie = VARIANTI.naso * VARIANTI.colletto;
  return candidati.map((c) => {
    let salto = 0;
    let l = leveDi("ministro", c.partito, c.nome);
    // shortcut: con più candidati delle coppie possibili (36) nessuna è libera, si tiene la faccia di partenza; da rivedere se le rose superano 36.
    while (visti.size < coppie && visti.has(`${l.naso}:${l.colletto}`)) l = leveDi("ministro", c.partito, c.nome, ++salto);
    visti.add(`${l.naso}:${l.colletto}`);
    return l;
  });
}

/** Il fondo del riquadro dalla soddisfazione: carta, giallo sotto 40, rosso sotto 25. */
export function fondoDi(soddisfazione: number | null): "carta" | "giallo" | "rosso" {
  if (soddisfazione == null) return "carta";
  return soddisfazione < 25 ? "rosso" : soddisfazione < 40 ? "giallo" : "carta";
}
