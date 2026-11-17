<script lang="ts">
  // Gli emendamenti degli alleati sul bilancio presentato (storia 129; prototipo 22, lato della forma B): Accogli · Respingi diretti,
  // il costo «se accogli» detto dal core (in aula indicizzato sui dicasteri chiesti, poi il riepilogo), il default; deciso, com'è stato deciso.
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import { guscio } from "$lib/guscio/chiamate";
  import { nomeDi } from "$lib/partiti";
  import { partita } from "$lib/partita.svelte";
  import { numero, segnato, t } from "$lib/testi/t";
  import type { Bilancio } from "$lib/tipi/Bilancio";
  import type { RiepilogoBilancio } from "$lib/tipi/RiepilogoBilancio";
  import type { Vista } from "$lib/tipi/Vista";
  import type { VoceMisura } from "$lib/tipi/VoceMisura";
  import { euroSegnato, seAccogli } from "./bilancio";

  let { vista, misura, inAula, emenda, rifiutoEm }: {
    vista: Vista; misura: VoceMisura | null; inAula: Bilancio; emenda: (partito: string, accogli: boolean) => void; rifiutoEm: (partito: string) => string | null;
  } = $props();

  const nome = (id: string) => nomeDi(vista, id);
  const em = $derived(misura?.emendamenti ?? []);
  const aperti = $derived(em.filter((e) => e.stato === "aperto"));
  const occupato = $derived(partita.occupato ? t("campagna.comando_in_corso") : undefined);

  // raw: il costo vale per il bilancio in aula su cui è chiesto, confrontato per identità
  let costi = $state.raw<{ inAula: Bilancio; per: Record<string, RiepilogoBilancio | string> } | null>(null);
  $effect(() => {
    const b = inAula, chiesti = aperti;
    const chiedi = { indicizza: guscio.indicizza, riepilogo: guscio.riepilogoBilancio };
    void Promise.all(chiesti.map(async (e) => [e.partito, await seAccogli(chiedi, b, e)] as const)).then((x) => {
      if (inAula === b) costi = { inAula: b, per: Object.fromEntries(x) };
    });
  });
  const costo = (p: string) => (costi?.inAula === inAula ? costi.per[p] : undefined);
</script>

{#if em.length}
  <div class="grp"><span>{t("aula.emendamenti")}</span><span>{t("bilancio.aperti", { n: aperti.length })}</span></div>
  {#each em as e (e.partito)}
    {@const c = costo(e.partito)}
    {@const intesa = vista.alleati.find((a) => a.partito === e.partito)?.intesa}
    {@const rif = rifiutoEm(e.partito)}
    <div class="em" class:chiuso={e.stato !== "aperto"}>
      <span class="t">{nome(e.partito)}</span>
      <span class="min muto">{e.motivo}</span>
      {#if e.stato === "aperto"}
        <div class="riga">
          <Pulsante forma="piccolo" spento={occupato} aria-label={t("aula.accogli_di", { nome: nome(e.partito) })} onclick={() => emenda(e.partito, true)}>{t("aula.accogli")}</Pulsante>
          <Pulsante forma="piccolo" spento={occupato} aria-label={t("aula.respingi_di", { nome: nome(e.partito) })} onclick={() => emenda(e.partito, false)}>{t("aula.respingi")}</Pulsante>
          {#if typeof c === "object"}
            <span class="min muto">{t("bilancio.se_accogli", { costo: euroSegnato(c.costo_spesa), n: segnato(c.crescita_spesa_netta, 1) })}</span>
          {:else if c}
            <span class="min muto">{t("bilancio.se_accogli_testo", { testo: c })}</span>
          {/if}
        </div>
        <span class="min muto">{t("bilancio.senza_scelta")}</span>
      {:else}
        <div class="riga">
          <span class="chip" class:rosso={e.stato === "respinto"}>{t(e.stato === "accolto" ? "aula.accolto" : "aula.respinto")}</span>
          {#if e.stato === "respinto" && intesa !== undefined}<span class="min muto">{t("bilancio.intesa", { nome: nome(e.partito), n: numero(intesa) })}</span>{/if}
        </div>
      {/if}
      {#if rif}<Rifiuto testo={rif} />{/if}
    </div>
  {/each}
{:else}
  <p class="sec muto">{t("bilancio.nessun_emendamento")}</p>
{/if}

<style>
  /* prototipo 22: .em, .em.chiuso, .chip */
  .em { border: 1.5px solid var(--ink); padding: 6px 8px; display: grid; gap: 4px; font-size: var(--c-sec) }
  .em.chiuso { border-style: dashed; border-color: var(--grigio) }
  .t { font-weight: 700 }
  .riga { display: flex; gap: 8px; align-items: center; flex-wrap: wrap }
  .chip { display: inline-block; border: 1.5px solid var(--ink); padding: 0 6px; font-size: var(--c-min); font-weight: 700; white-space: nowrap; line-height: 1.4 }
  .chip.rosso { border-color: var(--rosso); color: var(--rosso) }
</style>
