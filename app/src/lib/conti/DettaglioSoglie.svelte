<script lang="ts">
  // Il dettaglio delle soglie nel lato (prototipo 21): una situazione col righello grande, le 21 grandezze per gruppi (storia 121), una grandezza.
  import { numero, segnato, t, type Chiave } from "$lib/testi/t";
  import type { VoceGrandezza } from "$lib/tipi/VoceGrandezza";
  import type { Vista } from "$lib/tipi/Vista";
  import Forte from "./Forte.svelte";
  import Righello from "./Righello.svelte";
  import { distanza, grandezza, quanti } from "./conti";

  let { vista, scelta, apri }: { vista: Vista; scelta: string; apri: (k: string) => void } = $props();

  const [tipo, id] = $derived(scelta.split(":"));
  // i gruppi seguono l'ordine delle grandezze nella Vista (prototipo 21, `GR`)
  const GRUPPI: [Chiave, number, number][] = [["conti.gr_dicasteri", 0, 11], ["conti.gr_economia", 11, 17], ["conti.gr_ordine", 17, Infinity]];
  /** «Nome (unità)» → [nome, unità]; senza parentesi è un indice. */
  const nomeUnita = (g: VoceGrandezza): [string, string] => {
    const m = g.nome.match(/^(.*?) \((.*)\)$/);
    return m ? [m[1], m[2]] : [g.nome, t("conti.gr_indice")];
  };
  const indice = (g: VoceGrandezza) => !/ \(.*\)$/.test(g.nome);
  const unita = (misura: string) => (misura === "crescita_pil" ? "%" : "");
</script>

{#snippet situazione(i: number)}
  {@const s = vista.situazioni[i]}
  <button type="button" class="sit" class:on={s.accesa} onclick={() => apri(`sit:${i}`)}>
    <span class="n">{s.nome}</span>
    <span>{#if s.accesa}<span class="chip pieno">{t("conti.chip_accesa")}</span>{/if} <b>{numero(s.valore, 1)}{unita(s.misura)}</b></span>
    <span class="riga"><Righello {s} /></span>
    <span class="riga min muto">{distanza(s)}</span>
  </button>
{/snippet}

{#if tipo === "sit"}
  {@const s = vista.situazioni[+id]}
  {@const gr = vista.grandezze.find((g) => g.id === s.misura)}
  {@const sale = s.accensione > s.spegnimento}
  <h4 class="tit">{s.nome} {#if s.accesa}<span class="chip pieno">{t("conti.chip_accesa")}</span>{/if}</h4>
  <Righello {s} W={360} H={30} />
  <dl class="dati">
    <dt>{t("conti.valore_ufficiale")}</dt>
    <dd><b>{numero(s.valore, 1)}{unita(s.misura)}</b> · {s.misura === "crescita_pil" ? t("conti.da_prima_stima") : gr ? t("conti.da_grandezza", { nome: gr.nome }) : t("conti.da_motore")}</dd>
    <dt>{t("conti.si_accende")}</dt>
    <dd>{t(sale ? "conti.verso_sopra" : "conti.verso_sotto", { n: numero(s.accensione, 1) })}{s.trimestri > 1 ? t("conti.di_fila", { n: quanti[s.trimestri] ?? s.trimestri }) : ""}</dd>
    <dt>{t("conti.si_spegne")}</dt>
    <dd>{t(sale ? "conti.verso_sotto" : "conti.verso_sopra", { n: numero(s.spegnimento, 1) })}</dd>
  </dl>
  <p class="sec"><b>{distanza(s)}</b>.</p>
  <p class="sec muto">{t("conti.sit_spiega")}</p>
{:else if tipo === "grandezze"}
  <h4 class="tit">{t("conti.grandezze_titolo")}</h4>
  <p class="min muto">{t("conti.grandezze_spiega")}</p>
  {#each GRUPPI as [nome, a, b] (nome)}
    <h5 class="grp"><span>{t(nome)}</span></h5>
    {#each vista.grandezze.slice(a, b) as g (g.id)}
      {@const v = grandezza(g)}
      <button type="button" class="r" onclick={() => apri(`gr:${g.id}`)}>
        <span class="n">{nomeUnita(g)[0]}</span>
        <span class="x">
          {#if "valore" in v}
            {@const d = v.valore - (indice(g) ? 100 : 0)}
            {numero(v.valore, 1)}&nbsp;<small class:bene={Math.abs(d) >= 0.05 && (g.verso === "sale") === d > 0} class:male={Math.abs(d) >= 0.05 && (g.verso === "sale") !== d > 0}>{Math.abs(d) < 0.05 ? "=" : segnato(d, 1)}</small>
          {:else}
            {numero(v.da, 1)}–{numero(v.a, 1)}&nbsp;<small>{t("conti.gr_regioni_n", { n: g.valori.length })}</small>
          {/if}
        </span>
      </button>
    {/each}
  {/each}
{:else if tipo === "gr"}
  {@const g = vista.grandezze.find((x) => x.id === id)!}
  {@const v = grandezza(g)}
  {@const [nome, u] = nomeUnita(g)}
  {@const i = vista.situazioni.findIndex((s) => s.misura === g.id)}
  <h4 class="tit">{nome}</h4>
  {#if "valore" in v}
    <p class="sec"><b>{numero(v.valore, 1)}</b> · {u}. {t(g.verso === "sale" ? "conti.gr_sale" : "conti.gr_scende")}</p>
  {:else}
    <p class="sec"><Forte testo={t("conti.gr_regioni")} vals={[numero(v.da, 1), numero(v.a, 1)]} /></p>
    <a class="link" href="/partita/paese">{t("conti.gr_paese")}</a>
  {/if}
  <p class="min muto">{t("conti.gr_ritardo")}</p>
  {#if i >= 0}
    <h5 class="grp"><span>{t("conti.gr_soglia")}</span></h5>
    {@render situazione(i)}
  {/if}
{/if}

<style>
  .sit { display: grid; grid-template-columns: minmax(0, 1fr) auto; gap: 2px 8px; align-items: center; width: 100%; padding: 5px 4px; border-bottom: var(--filo) }
  .sit:hover { background: var(--velo) }
  .sit .n { font-weight: 700; font-size: var(--c-sec) }
  .sit.on .n { color: var(--rosso) }
  .riga { grid-column: 1 / -1; display: block }
</style>
