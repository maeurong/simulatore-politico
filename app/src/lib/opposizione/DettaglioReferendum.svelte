<script lang="ts">
  // Referendum nel dettaglio a lato (28-opposizione.prototype.html, stato `referendum` e `confHtml`): l'abrogativo con le disposizioni e la conferma
  // sul posto, il confermativo con «Chiedi il referendum» · «Non chiederlo» senza conferma.
  import { t, type Chiave } from "$lib/testi/t";
  import { nomeDi } from "$lib/partiti";
  import { partita } from "$lib/partita.svelte";
  import { misuraDelConfermativo } from "$lib/aula/referendum";
  import { periodoDi } from "$lib/aula/aula";
  import { testoRifiuto } from "$lib/guscio/chiamate";
  import Conferma from "$lib/componenti/Conferma.svelte";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import type { Comando } from "$lib/tipi/Comando";
  import type { Vista } from "$lib/tipi/Vista";
  import type { VoceDisposizione } from "$lib/tipi/VoceDisposizione";

  let { vista, quale, manda, rifiuto }: {
    vista: Vista; quale: "abrogativo" | "confermativo"; manda: (c: Comando) => Promise<boolean>; rifiuto: { testo: string; chiave: string } | null;
  } = $props();

  const occupato = $derived(partita.occupato ? t("campagna.comando_in_corso") : undefined);
  const LIMITI: Chiave[] = ["opposizione.ref_limite_1", "opposizione.ref_limite_2", "opposizione.ref_limite_3", "opposizione.ref_limite_4"];
  const corso = $derived(vista.referendum.find((r) => r.tipo === "abrogativo"));
  const abrogabili = $derived(vista.disposizioni.filter((d) => d.referendabile));
  const chiusi = $derived(vista.disposizioni.filter((d) => !d.referendabile));
  const gradino = (d: VoceDisposizione) => d.gradini.find((g) => g.id === d.in_vigore)?.nome ?? d.in_vigore;

  const conf = $derived(vista.referendum.find((r) => r.tipo === "confermativo" && (r.richiesto || r.scelta)));
  const misura = $derived(conf?.scelta ? misuraDelConfermativo(vista, conf) : null);
</script>

{#if quale === "abrogativo"}
  <div class="rf">
    <ul class="limiti">{#each LIMITI as k (k)}<li>{t(k)}</li>{/each}</ul>
    {#if corso}
      <div class="box">
        <span class="occ">{t("opposizione.ref_in_corso_titolo", { partito: corso.promotore ? nomeDi(vista, corso.promotore) : "" })}</span>
        <span class="sec">{t("opposizione.ref_in_corso", { nome: corso.nome, firme: t(corso.firme ? "opposizione.ref_firme" : "opposizione.ref_firme_a_fine_turno"), periodo: periodoDi(vista, corso.voto) })}</span>
      </div>
    {/if}
    <span class="occ">{t("opposizione.ref_abrogabili", { n: abrogabili.length })}</span>
    <ul class="elenco">
      {#each abrogabili as d (d.id)}
        <li>
          <span class="nm"><b>{d.nome}</b> <span class="muto">· {gradino(d)}</span></span>
          {#if d.rifiuto_referendum === null}
            <Conferma etichetta={t("opposizione.ref_promuovi")} conferma={t("opposizione.ref_promuovi_conferma")} spento={occupato} onconferma={() => manda({ tipo: "promuovi_referendum", disposizione: d.id })} />
          {/if}
          <span class="sotto">
            <small>{d.legge}</small>
            {#if d.rifiuto_referendum !== null}<span class="min muto">{testoRifiuto({ specie: "rifiuto", testo: d.rifiuto_referendum })}</span>{/if}
            {#if rifiuto?.chiave === `ref:${d.id}`}<Rifiuto testo={rifiuto.testo} />{/if}
          </span>
        </li>
      {/each}
    </ul>
    <span class="occ">{t("opposizione.ref_non_referendabili", { n: chiusi.length })}</span>
    <p class="sec muto"><span class="min">{t("opposizione.ref_non_referendabile")}:</span> {chiusi.map((d) => d.nome).join(" · ")}</p>
  </div>
{:else if conf}
  <div class="rf">
    <p class="sec"><b>{conf.nome}</b></p>
    <p class="sec">{t("opposizione.conf_regola")}</p>
    {#if misura !== null}
      <div class="az">
        <Pulsante forma="piccolo" scelto={conf.richiesto} spento={occupato} onclick={() => manda({ tipo: "confermativo", misura, chiedi: true })}>{t("opposizione.conf_chiedi")}</Pulsante>
        <Pulsante forma="piccolo" scelto={!conf.richiesto} spento={occupato} onclick={() => manda({ tipo: "confermativo", misura, chiedi: false })}>{t("opposizione.conf_non_chiederlo")}</Pulsante>
      </div>
      <p class="min muto">{t("opposizione.conf_nota")}</p>
    {/if}
    {#if rifiuto?.chiave === "confermativo"}<Rifiuto testo={rifiuto.testo} />{/if}
  </div>
{:else}
  <p class="muto">{t("opposizione.conf_nessuno")}</p>
{/if}

<style>
  .rf { display: grid; gap: 6px }
  .occ { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700 }
  .sec { font-size: var(--c-sec) }
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }
  .limiti { padding-left: 16px; font-size: var(--c-sec); display: grid; gap: 2px }
  .box { border: var(--bordo); padding: 6px 8px; display: grid; gap: 2px }
  .elenco { list-style: none; display: grid }
  .elenco li { display: grid; grid-template-columns: minmax(0, 1fr) auto; gap: 1px 10px; align-items: center; font-size: var(--c-sec); padding: 4px 0; border-bottom: var(--filo) }
  .sotto { grid-column: 1 / -1; display: grid; gap: 1px }
  .sotto small { color: var(--grigio); font-size: var(--c-min) }
  .az { display: flex; gap: 6px; flex-wrap: wrap }
</style>
