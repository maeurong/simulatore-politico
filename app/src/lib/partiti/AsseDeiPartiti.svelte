<script lang="ts">
  // L'asse da 1 a 7 (24-partiti.prototype.html, forma A): quadrato sulla dichiarata, anello sulla percepita, la freccia fra i due.
  // Chi minaccia la scissione ha la freccia rossa e i fili a mezzo punto e a un punto (le soglie di `campagna`), dal lato dove scivola la percepita; nel tono grave il rosso tace. Il colore non basta mai: il nome sta sopra.
  import { t, numero } from "$lib/testi/t";
  import { coloreDi } from "$lib/partiti";
  import type { VistaCampagna } from "$lib/tipi/VistaCampagna";
  import type { VocePartito } from "$lib/tipi/VocePartito";

  // `onscelta`: i partiti si scelgono (clic, Invio, spazio) e ognuno ha il suo nome accessibile; senza, l'asse è un'immagine sola.
  let { partiti, giocatore, larghezza = 820, scelto = null, onscelta, campagna }: { partiti: VocePartito[]; giocatore: string; larghezza?: number; scelto?: string | null; onscelta?: (id: string) => void; campagna?: VistaCampagna } = $props();
  const pre = $props.id();

  const W = $derived(larghezza), L = 18, PASSO = 16;
  const R = $derived(W - 18);
  const X = (x: number) => L + ((x - 1) / 6) * (R - L);
  const aria = (p: VocePartito) => t("partiti.s_asse_aria", { nome: nome(p), d: numero(p.posizione_dichiarata, 2), p: numero(p.posizione_percepita, 2) }) + (p.minaccia && !p.sciolto ? `, ${t("partiti.s_asse_minaccia")}` : "");
  const nome = (p: VocePartito) => (p.id === giocatore ? t("partiti.tu") : p.nome);
  const colore = (p: VocePartito) => coloreDi(p.id) ?? "var(--grigio)";

  // due dichiarate che si toccano si impilano: ognuno resta alla sua ascissa vera, il giocatore sta in basso e si disegna per ultimo
  const POSTO = 11;
  const livello = $derived.by(() => {
    const per: number[][] = [];
    const lv: Record<string, number> = {};
    const ordine = [...partiti].sort((a, b) => Number(b.id === giocatore) - Number(a.id === giocatore) || a.posizione_dichiarata - b.posizione_dichiarata);
    for (const p of ordine) {
      const x = X(p.posizione_dichiarata);
      let k = 0;
      while (per[k]?.some((o) => Math.abs(o - x) < POSTO)) k++;
      (per[k] ??= []).push(x);
      lv[p.id] = k;
    }
    return lv;
  });
  const sopra = $derived(Math.max(0, ...Object.values(livello)) * POSTO);
  const disegno = $derived([...partiti.filter((p) => p.id !== giocatore), ...partiti.filter((p) => p.id === giocatore)]);

  // i nomi su più file: ognuno sulla prima fila in cui non tocca l'ultimo già messo (da sinistra)
  const file = $derived.by(() => {
    const fine: number[] = [];
    return Object.fromEntries([...partiti].sort((a, b) => a.posizione_dichiarata - b.posizione_dichiarata).map((p) => {
      const x = X(p.posizione_dichiarata), mezzo = (nome(p).length * 7.2 + 14) / 2;
      let k = 0;
      while ((fine[k] ?? -1e9) > x - mezzo) k++;
      fine[k] = x + mezzo;
      return [p.id, k];
    }));
  });
  const righe = $derived(Math.max(0, ...Object.values(file)) + 1);
  const H = $derived(52 + righe * PASSO + sopra);
  const y0 = $derived(H - 22);
  const y = $derived(y0 - 6);
  const ancora = (p: VocePartito) => {
    const x = X(p.posizione_dichiarata), m = nome(p).length * 3.6 + 4;
    return x - L < m ? "start" : R - x < m ? "end" : "middle";
  };
</script>

<svg class="ax" width="100%" height={H} viewBox="0 0 {W} {H}" preserveAspectRatio="xMidYMid meet" role={onscelta ? "group" : "img"} aria-label={t("partiti.asse_aria")}>
  <defs>
    <marker id="{pre}-pt" viewBox="0 0 6 6" refX="5" refY="3" markerWidth="6" markerHeight="6" orient="auto"><path d="M0 0L6 3L0 6Z" class="punta" /></marker>
    <marker id="{pre}-pr" viewBox="0 0 6 6" refX="5" refY="3" markerWidth="6" markerHeight="6" orient="auto"><path d="M0 0L6 3L0 6Z" class="punta min" /></marker>
    <pattern id="{pre}-ret" width="4" height="4" patternUnits="userSpaceOnUse" patternTransform="rotate(45)"><rect width="4" height="4" class="carta" /><line x1="0" y1="0" x2="0" y2="4" class="tratto" /></pattern>
  </defs>
  <line x1={L} y1={y0} x2={R} y2={y0} class="base" />
  {#each [1, 2, 3, 4, 5, 6, 7] as i (i)}
    <line x1={X(i)} y1={y0} x2={X(i)} y2={y0 + 5} class="base" stroke-width="1" />
    <text x={X(i)} y={y0 + 16} text-anchor="middle">{i}</text>
  {/each}
  <text x={L - 14} y={y0 + 16} text-anchor="start">◂</text>
  <text x={R + 14} y={y0 + 16} text-anchor="end">▸</text>

  <!-- i gambi prima, i nomi dopo: il nome di un partito non lo taglia il gambo di un altro -->
  {#each disegno as p (p.id)}
    <line x1={X(p.posizione_dichiarata)} y1={y - livello[p.id] * POSTO - 5} x2={X(p.posizione_dichiarata)} y2={y - sopra - 14 - file[p.id] * PASSO + 3} opacity={p.sciolto ? 0.45 : 1} style:stroke={colore(p)} stroke-width="1" />
  {/each}
  {#each disegno as p (p.id)}
    {@const xd = X(p.posizione_dichiarata)}
    {@const xp = X(p.posizione_percepita)}
    {@const yq = y - livello[p.id] * POSTO}
    {@const yl = y - sopra - 14 - file[p.id] * PASSO}
    {@const c = colore(p)}
    {@const nato = coloreDi(p.id) === null}
    {#snippet segno()}
      {#if campagna && p.minaccia && !p.sciolto}
        {#each [{ d: campagna.minaccia_scissione, tratteggio: true }, { d: campagna.soglia_scissione, tratteggio: false }] as { d, tratteggio }, i (i)}
          {@const xf = X(Math.min(7, Math.max(1, p.posizione_dichiarata + (Math.sign(p.posizione_percepita - p.posizione_dichiarata) || 1) * d)))}
          <line x1={xf} y1={yq - 9} x2={xf} y2={yq + 3} class="filo" stroke-width={tratteggio ? 1 : 2} stroke-dasharray={tratteggio ? "2 2" : undefined} />
        {/each}
      {/if}
      {#if Math.abs(xp - xd) > 2}
        <line x1={xd} y1={yq} x2={xp - Math.sign(xp - xd) * 4} y2={yq} class="freccia" class:min={p.minaccia && !p.sciolto} marker-end="url(#{pre}-{p.minaccia && !p.sciolto ? "pr" : "pt"})" />
      {/if}
      <circle cx={xp} cy={yq} r="4.5" class="anello" style:stroke={c} />
      <text class="nm" class:sc={p.sciolto} x={xd + (ancora(p) === "start" ? -6 : ancora(p) === "end" ? 6 : 0)} y={yl} text-anchor={ancora(p)}>{nome(p)}</text>
      <rect x={xd - 5} y={yq - 5} width="10" height="10" class="quad" style:fill={nato ? `url(#${pre}-ret)` : c} />
    {/snippet}
    {#if onscelta}
      <g opacity={p.sciolto ? 0.45 : 1} class="sceglie" class:scelto={scelto === p.id} role="button" tabindex="0" aria-label={aria(p)} aria-pressed={scelto === p.id}
        onclick={() => onscelta(p.id)} onkeydown={(e) => (e.key === "Enter" || e.key === " ") && (e.preventDefault(), onscelta(p.id))}>{@render segno()}</g>
    {:else}
      <g opacity={p.sciolto ? 0.45 : 1}>{@render segno()}</g>
    {/if}
  {/each}
</svg>

<style>
  .ax { display: block }
  .sceglie { cursor: pointer }
  .sceglie:hover .nm, .scelto .nm { text-decoration: underline; text-underline-offset: 2px }
  .base { stroke: var(--ink); stroke-width: 1.5 }
  text { font: 400 11px var(--txt); fill: var(--grigio) }
  .nm { font-weight: 700; font-size: 12px; fill: var(--ink); paint-order: stroke; stroke: var(--carta); stroke-width: 3px; stroke-linejoin: round }
  .nm.sc { text-decoration: line-through; fill: var(--grigio) }
  .anello { fill: var(--carta); stroke-width: 2 }
  .quad { stroke: var(--ink); stroke-width: 1 }
  .freccia { stroke: var(--grigio); stroke-width: 1.5 }
  .punta { fill: var(--grigio) }
  .freccia.min { stroke: var(--rosso) }
  .punta.min { fill: var(--rosso) }
  :global(:root.grave) .freccia.min { stroke: var(--grigio) }
  :global(:root.grave) .punta.min { fill: var(--grigio) }
  .filo { stroke: var(--rosso) }
  .carta { fill: var(--carta) }
  .tratto { stroke: var(--grigio); stroke-width: 1.2 }
</style>
