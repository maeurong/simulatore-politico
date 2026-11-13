<script lang="ts">
  // Un cursore del Paese al turno zero (14-editor.prototype.html:314-322): l'input nativo fa frecce e puntatore,
  // anche sotto zoom; sopra, solo la tacca di oggi in percentuale.
  import { numero, segnato, t, type Chiave } from "$lib/testi/t";
  import type { CursoreCatalogo } from "$lib/tipi/CursoreCatalogo";
  import { alLimite } from "./editor.svelte";

  type Props = {
    cur: CursoreCatalogo;
    valore: number;
    /** spento (salvataggio in volo): il fuoco resta, il valore non si muove */
    fermo?: boolean;
    onvalore: (v: number) => void;
  };
  let { cur, valore, fermo = false, onvalore }: Props = $props();

  // ponytail: i passi del prototipo (14-editor.prototype.html:246); l'età a passo intero come la griglia di taratura
  const PASSO: Record<string, number> = { debito_pil: 0.1, disoccupazione: 0.1, inflazione: 0.1, ricchezza: 1, eta_mediana: 1, orientamento: 0.05 };
  const passo = $derived(PASSO[cur.nome] ?? 0.1);

  /** Lo scarto col segno, e lo zero con «±» come nel prototipo (14-editor.prototype.html:230). */
  const scarto = (v: number) => {
    const s = segnato(v);
    return /[1-9]/.test(s) ? s : `±${s}`;
  };

  /** Il valore come si legge, con l'unità accanto. */
  function forma(v: number): [string, string] {
    switch (cur.nome) {
      case "ricchezza": return [`${scarto(v)}%`, t("editor.unita.ricchezza")];
      case "eta_mediana": return [t("editor.anni", { n: Math.abs(v), valore: scarto(v) }), t("editor.unita.eta_mediana")];
      case "orientamento":
        if (Math.abs(v) < 0.005) return [t("editor.orientamento_centro"), ""];
        return [numero(Math.abs(v), 2), t(v < 0 ? "editor.orientamento_sinistra" : "editor.orientamento_destra")];
      default: return [`${numero(v, 1)}%`, t(`editor.unita.${cur.nome}` as Chiave)];
    }
  }
  // gli estremi e oggi in forma corta, col segno; l'orientamento a zero si dice «al centro di oggi»
  const corto = (v: number) => (cur.nome === "orientamento" && Math.abs(v) >= 0.005 ? segnato(v, 2) : forma(v)[0]);

  const [v, unita] = $derived(forma(valore));
  const mosso = $derived(Math.abs(valore - cur.valore) > 1e-9);
  const limite = $derived(alLimite(valore, cur));
  const oggi = $derived(((cur.valore - cur.min) / (cur.max - cur.min)) * 100);
  const nome = $derived(t(`editor.cursore.${cur.nome}` as Chiave));
</script>

<div class={["cur", { mosso, fermo }]}>
  <div class="r1"><span class="nm">{nome}</span><span class="v">{v} <small>{unita}</small></span></div>
  <div class="pista">
    <input type="range" min={cur.min} max={cur.max} step={passo} value={valore} aria-label={nome} aria-valuetext="{v} {unita}"
      aria-disabled={fermo || undefined}
      oninput={(e) => (fermo ? (e.currentTarget.value = String(valore)) : onvalore(Number(e.currentTarget.value)))} />
    <span class="tacca" style:left="{oggi}%" aria-hidden="true"></span>
  </div>
  <div class="r3">
    <span>{corto(cur.min)}</span>
    {#if limite}<span class="al">{t("editor.al_limite")}</span>{:else}<span>{t("editor.oggi", { valore: corto(cur.valore) })}</span>{/if}
    <span>{corto(cur.max)}</span>
  </div>
  {#if cur.nome === "inflazione"}<p class="nota">{t("editor.nota_inflazione")}</p>{/if}
</div>

<style>
  .cur { display: grid; gap: 2px; padding: 6px 8px 8px; border-left: 3px solid transparent; min-width: 0 }
  .cur.mosso { border-left-color: var(--giallo); background: var(--velo) }
  .cur.fermo input { opacity: .5; cursor: default }
  .r1 { display: flex; align-items: baseline; gap: 8px }
  .nm { font-weight: 700 }
  .v { margin-left: auto; font-family: var(--tit); font-weight: 700; font-size: var(--c-num); text-align: right }
  .v small { font: 400 var(--c-min) var(--txt); color: var(--grigio) }
  .pista { position: relative; height: 22px }
  input { width: 100%; margin: 0; height: 22px }
  .tacca { position: absolute; top: 15px; width: 2px; height: 9px; background: var(--rosso); pointer-events: none; transform: translateX(-1px) }
  .r3 { display: flex; justify-content: space-between; gap: 8px; font-size: var(--c-min); color: var(--grigio) }
  .al { color: var(--rosso); font-weight: 700 }
  .nota { font-size: var(--c-min); color: var(--grigio) }
</style>
