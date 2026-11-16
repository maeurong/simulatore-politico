<script lang="ts">
  // L'economia (storie 118, 119): prima stima, revisione, spread, poi debito, regole UE e redditi per fascia nella forma del prototipo 36.
  import { numero, segnato, t, type Chiave } from "$lib/testi/t";
  import { periodo } from "$lib/edicola/edicola";
  import { partita } from "$lib/partita.svelte";
  import type { Stima } from "$lib/tipi/Stima";
  import type { Vista } from "$lib/tipi/Vista";
  import Spread from "./Spread.svelte";
  import { percorso, procedura, sottoAttese } from "./conti";

  let { vista, scelta = $bindable() }: { vista: Vista; scelta: string | null } = $props();

  const STIMA = ["crescita_pil", "disoccupazione", "inflazione_annua", "disavanzo"] as const;
  const cifra = (s: Stima, k: (typeof STIMA)[number]) => `${k === "crescita_pil" ? segnato(s[k], 1) : numero(s[k], 1)}%`;
  const studi = $derived(vista.acquisti.centro_studi);
  const rev = $derived(vista.statistica.revisione);
  const ue = $derived(vista.regole_ue);
  const pde = $derived(procedura(ue));
  const d = $derived(vista.debito);
  const scegli = (k: string) => () => (scelta = k);
</script>

<section>
  <div class="capo"><h3 class="tit">{t("conti.economia")}</h3><span class="sec muto">{periodo(vista.statistica.trimestre)}</span></div>

  <h4 class="grp"><span>{t("conti.prima_stima")}</span></h4>
  <div class="num4">
    {#each STIMA as k (k)}
      {@const nota = t(`conti.nota_${k}` as Chiave)}
      <button type="button" class="c" class:sel={scelta === `stat:${k}`} aria-pressed={scelta === `stat:${k}`} onclick={scegli(`stat:${k}`)}>
        <span class="k">{t(`conti.stat_${k}` as Chiave)}{nota ? ` · ${nota}` : ""}</span>
        <span class="v">{cifra(vista.statistica.prima_stima, k)}</span>
        {#if studi}<span class="k">{t("conti.centro_studi")}: <b>{cifra(studi.stima, k)}</b></span>{/if}
      </button>
    {/each}
  </div>
  {#if rev}
    <p class="riv sec">
      <span class="muto">{t("conti.revisione", { periodo: periodo(rev.trimestre) })}</span>
      {#each STIMA as k (k)}
        {@const scarto = rev.riveduta[k] - rev.prima_stima[k]}
        <span>{t(`conti.stat_${k}` as Chiave)} {numero(rev.prima_stima[k], 1)} → <b>{numero(rev.riveduta[k], 1)}</b> <span class:muto={Math.abs(scarto) < 0.05}>({segnato(scarto, 1)})</span></span>
      {/each}
    </p>
  {:else}
    <p class="riv sec muto">{t("conti.senza_revisione")}</p>
  {/if}

  <button type="button" class="spread" class:sel={scelta === "spread"} aria-pressed={scelta === "spread"} onclick={scegli("spread")}>
    <span class="grp"><span>{t("conti.spread")}</span><span class="ink">{t("conti.pb", { n: numero(vista.spread_pb) })}</span></span>
    <Spread serie={partita.serie} soglia={vista.soglie.crisi_debito_pb} />
  </button>

  <h4 class="grp"><span>{t("conti.debito")}</span></h4>
  <button type="button" class="r" class:sel={scelta === "debito"} aria-pressed={scelta === "debito"} onclick={scegli("debito")}>
    <span class="n">{t("conti.debito_pil")}</span><span class="x">{numero(d.debito_pil, 1)}%</span>
  </button>
  {#if d.mes}
    <button type="button" class="r due" class:sel={scelta === "mes"} aria-pressed={scelta === "mes"} onclick={scegli("mes")}>
      <span class="n"><span class="chip pieno">{t("conti.mes")}</span> {t("conti.fino_a_periodo", { periodo: periodo(d.mes.fine) })}</span>
      <span class="x">{t("conti.vincoli", { n: d.mes.vincoli.length })}</span>
    </button>
  {/if}
  {#if d.ristrutturazione}
    <button type="button" class="r due" class:sel={scelta === "ristr"} aria-pressed={scelta === "ristr"} onclick={scegli("ristr")}>
      <span class="n"><span class="chip rosso">{t("conti.ristrutturato")}</span> {periodo(d.ristrutturazione.taglio)}</span>
      <span class="x">{t("conti.premio", { n: numero(d.ristrutturazione.premio_pb) })}</span>
    </button>
  {/if}

  <h4 class="grp"><span>{t("conti.regole_ue")}</span></h4>
  {#snippet riga(cls: string, n: string, x: string, male = false, dopo = "")}
    <button type="button" class="r {cls}" class:sel={scelta === "ue"} aria-pressed={scelta === "ue"} onclick={scegli("ue")}>
      <span class="n">{n}</span><span class="x"><span class:male>{x}</span>{#if dopo}&nbsp;<small>{dopo}</small>{/if}</span>
    </button>
  {/snippet}
  {@render riga("due", t("conti.pde"), pde.testo, pde.male, ue.procedura === "nessuna" ? "" : ue.turni_in_stato === 0 ? t("conti.da_questo_turno") : t("conti.da_turni", { n: ue.turni_in_stato }))}
  {@render riga("", t("conti.conto_controllo"), t("conti.del_pil", { n: numero(ue.conto_controllo, 1) }))}
  <button type="button" class="r due" class:sel={scelta === "ue"} aria-pressed={scelta === "ue"} onclick={scegli("ue")}>
    <span class="n">{t("conti.prossimo_controllo")} <small>{t("conti.quando_controllo")}</small></span>
    <span class="x">{ue.proiezione ? t("conti.proiezione", { anno: ue.proiezione.anno, percorso: percorso(ue.proiezione.deviazione) }) : t("conti.manca_un_anno")}</span>
  </button>
  {#if ue.coesione_sospesa}
    <button type="button" class="r" class:sel={scelta === "ue"} aria-pressed={scelta === "ue"} onclick={scegli("ue")}>
      <span class="n male">{t("conti.coesione_sospesa")}</span><span class="x">›</span>
    </button>
  {/if}
  {#if ue.margine_energia !== null}
    {@render riga("due", t("conti.margine_energia"), t("conti.energia_chiesto", { anno: ue.margine_energia }))}
  {/if}

  <h4 class="grp"><span>{t("conti.redditi")}</span></h4>
  {#each vista.redditi as r (r.fascia)}
    {@const k = `red:${r.fascia}`}
    <button type="button" class="r due" class:sel={scelta === k} aria-pressed={scelta === k} onclick={scegli(k)}>
      <span class="n">{t(`conti.fascia_${r.fascia}`)} <small>{t("conti.reddito_anno", { n: numero(r.reddito) })}</small></span>
      <span class="x"><span class:male={sottoAttese(r)}>{segnato(r.crescita, 1)}%</span> <small>{t("conti.attese", { n: segnato(r.attesa, 1) })}</small></span>
    </button>
  {/each}
</section>

<style>
  .num4 { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 6px 12px }
  .c { display: grid; grid-template-columns: minmax(0, 1fr); align-content: start; border-top: var(--bordo); padding-top: 3px }
  /* forma stretta: una casella per riga, i numeri grandi non entrano in mezza colonna */
  :global(:root.stretta) .num4 { grid-template-columns: minmax(0, 1fr) }
  .c:hover { background: var(--velo) }
  .c.sel { background: var(--velo); box-shadow: inset 0 0 0 1.5px var(--ink) }
  .c .k { font-size: var(--c-min); color: var(--grigio); line-height: 1.2 }
  .c .k b { color: var(--ink) }
  .v { font-family: var(--tit); font-size: var(--c-pag); font-weight: 800; line-height: 1.05 }
  .riv { display: grid; gap: 2px; margin-top: 4px }
  .spread { display: grid; gap: 4px; width: 100% }
  .spread:hover { background: var(--velo) }
  .spread.sel { background: var(--velo); box-shadow: inset 0 0 0 1.5px var(--ink) }
  .spread .grp { margin-top: 0 }
  .ink { color: var(--ink) }
</style>
