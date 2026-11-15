<script lang="ts">
  // I referendum (storia 110): in attesa del voto (al governo «Invita all'astensione» sull'abrogativo, all'opposizione la scelta sul confermativo,
  // tutti e due diretti) e gli esiti. «Promuovi» sta nel dettaglio di una disposizione, in Disposizioni.
  import { tick } from "svelte";
  import { t, numero, type Chiave } from "$lib/testi/t";
  import { nomeDi } from "$lib/partiti";
  import { partita } from "$lib/partita.svelte";
  import type { Comando } from "$lib/tipi/Comando";
  import type { Vista } from "$lib/tipi/Vista";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import { quando } from "./aula";
  import { esitoDi, misuraDelConfermativo } from "./referendum";

  let { vista, manda, rifiuto }: {
    vista: Vista; manda: (c: Comando) => Promise<boolean>; rifiuto: { testo: string; tipo: Comando["tipo"] } | null;
  } = $props();

  const occupato = $derived(partita.occupato ? t("campagna.comando_in_corso") : undefined);
  const delReferendum = $derived(rifiuto && (rifiuto.tipo === "invita_astensione" || rifiuto.tipo === "confermativo") ? rifiuto : null);
  let radice: HTMLElement;
  // il bottone premuto sparisce con la risposta: se il fuoco stava lì passa alla sezione
  async function vai(c: Comando) {
    const dentro = radice.contains(document.activeElement);
    if (!(await manda(c))) return;
    await tick();
    if (dentro && !radice.contains(document.activeElement)) radice.focus({ preventScroll: true });
  }
</script>

<section class="ref" bind:this={radice} tabindex="-1" aria-label={t("aula.scheda_referendum")}>
  <h4 class="tit">{t("aula.referendum_in_attesa")}</h4>
  {#each vista.referendum as r, i (`${r.nome}|${i}`)}
    {@const misura = r.scelta ? misuraDelConfermativo(vista, r) : null}
    <div class="voce">
      <span>
        <b>{r.nome}</b>
        <span class="muto">· {t(`aula.referendum_${r.tipo}` as Chiave)}{r.promotore ? `, ${t("aula.referendum_promosso_da", { partito: nomeDi(vista, r.promotore) })}` : ""}</span>
      </span>
      <span class="chip">{t(r.firme ? "aula.referendum_indetto" : "aula.referendum_firme")}</span>
      <small>{quando(vista, r.voto)}{r.invito_astensione ? ` · ${t("aula.referendum_invito")}` : ""}</small>
      {#if vista.governa && r.tipo === "abrogativo" && !r.invito_astensione}
        <span class="az">
          <Pulsante forma="piccolo" spento={occupato} onclick={() => vai({ tipo: "invita_astensione" })}>{t("aula.invita_astensione")}</Pulsante>
          <span class="min muto">{t("aula.invita_quorum")}</span>
        </span>
      {/if}
      {#if misura !== null}
        <span class="az">
          <Pulsante forma="piccolo" spento={occupato} onclick={() => vai({ tipo: "confermativo", misura, chiedi: true })}>{t("aula.chiedi_confermativo")}</Pulsante>
          <Pulsante forma="piccolo" spento={occupato} onclick={() => vai({ tipo: "confermativo", misura, chiedi: false })}>{t("aula.rinuncia_confermativo")}</Pulsante>
        </span>
      {/if}
    </div>
  {:else}
    <p class="sec muto">{t("aula.referendum_nessuno")}</p>
  {/each}
  {#if delReferendum}<Rifiuto testo={delReferendum.testo} />{/if}

  <h4 class="tit">{t("aula.referendum_esiti")}</h4>
  {#each vista.esiti_referendum as e, i (`${e.nome}|${i}`)}
    <div class="voce">
      <span><b>{e.nome}</b> <span class="muto">· {t(`aula.referendum_${e.tipo}` as Chiave)}</span></span>
      <span class="chip" class:pieno={esitoDi(e) === "approvato"}>{esitoDi(e) === "decaduto" ? e.decaduto : t(`aula.referendum_${esitoDi(e)}` as Chiave)}</span>
      {#if e.decaduto === null}<small>{t("aula.referendum_cifre", { si: numero(e.si), no: numero(e.no), votanti: numero(e.votanti), aventi: numero(e.aventi_diritto) })}</small>{/if}
    </div>
  {:else}
    <p class="sec muto">{t("aula.referendum_esiti_nessuno")}</p>
  {/each}
</section>

<style>
  .ref { display: grid; gap: 6px; align-content: start }
  .ref:focus-visible { outline-offset: 2px }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; font-size: var(--c-riq) }
  .tit:not(:first-child) { margin-top: 10px }
  .sec { font-size: var(--c-sec) }
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }
  .voce { display: grid; grid-template-columns: minmax(0, 1fr) auto; gap: 1px 10px; font-size: var(--c-sec); padding-bottom: 5px; border-bottom: var(--filo); align-items: baseline }
  .voce small { grid-column: 1 / -1; color: var(--grigio); font-size: var(--c-min) }
  .chip { border: 1.5px solid var(--ink); padding: 0 6px; font-size: var(--c-min); font-weight: 700; white-space: nowrap; line-height: 1.4 }
  .chip.pieno { background: var(--ink); color: var(--carta) }
  .az { grid-column: 1 / -1; display: flex; gap: 6px; align-items: center; flex-wrap: wrap; margin-top: 3px }
</style>
