<script lang="ts">
  // «Il programma del governo» nell'ordine della Vista, e «La maggioranza» coi seggi e la fascia d'intesa (28-opposizione.prototype.html, forma A).
  import { t, type Chiave } from "$lib/testi/t";
  import { nomeDi } from "$lib/partiti";
  import { periodoDi } from "$lib/aula/aula";
  import Caselle from "$lib/componenti/Caselle.svelte";
  import type { FasciaIntesa } from "$lib/tipi/FasciaIntesa";
  import type { Vista } from "$lib/tipi/Vista";
  import RigaOdg from "./RigaOdg.svelte";
  import { maggioranzaAvversaria, programma } from "./opposizione";

  let { vista }: { vista: Vista } = $props();

  const FASCE: Record<FasciaIntesa, { piene: number; nome: Chiave }> = {
    solida: { piene: 3, nome: "partiti.fascia_solida" },
    incrinata: { piene: 2, nome: "partiti.fascia_incrinata" },
    ultimatum: { piene: 1, nome: "partiti.fascia_ultimatum" },
  };
  const inAula = (disposizione: string) => vista.calendario.find((m) => m.valori.some((x) => x.disposizione === disposizione));
  const causa = (id: string) => vista.alleati_avversari.find((a) => a.partito === id)?.causa_ultimatum ?? null;
</script>

<section>
  <h2 class="grp"><span>{t("opposizione.programma")}</span></h2>
  <ul>
    {#each programma(vista) as p, i (p.disposizione)}
      {@const m = inAula(p.disposizione)}
      <RigaOdg titolo="{i + 1}. {p.nome}: {p.nome_gradino}" segno="info" sotto={m ? t("opposizione.programma_in_aula", { periodo: periodoDi(vista, m.voto) }) : t("opposizione.programma_non_ancora")}>
        {#snippet azioni()}{#if m}<span class="chip">{t("aula.scheda_in_aula")}</span>{/if}{/snippet}
      </RigaOdg>
    {/each}
    <RigaOdg titolo={t("opposizione.agende")} segno="info" sotto={t("opposizione.agende_sotto")} />
  </ul>
</section>

<section>
  <h2 class="grp"><span>{t("opposizione.maggioranza")}</span><span>{t("opposizione.maggioranza_fasce")}</span></h2>
  <ul>
    {#each maggioranzaAvversaria(vista) as p (p.partito)}
      {@const c = causa(p.partito)}
      <RigaOdg
        titolo={nomeDi(vista, p.partito)} segno={p.fascia === "ultimatum" ? "urg" : "info"}
        sotto={[t("opposizione.seggi", { c: p.seggi[0], s: p.seggi[1] }), p.fascia === null ? t("opposizione.guida") : "", c ? t("opposizione.ultimatum_causa", { causa: t(`intesa.causa_${c}`) }) : ""].filter(Boolean).join(" · ")}>
        {#snippet azioni()}
          {#if p.fascia}<Caselle piene={FASCE[p.fascia].piene} su={3} rosso={p.fascia === "ultimatum"} nome={t(FASCE[p.fascia].nome)} />{/if}
        {/snippet}
      </RigaOdg>
    {/each}
  </ul>
</section>

<style>
  .grp { display: flex; justify-content: space-between; gap: 8px; font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--grigio); padding-bottom: 2px; border-bottom: 1.5px solid var(--ink) }
  ul { list-style: none }
  .chip { border: 1.5px solid var(--ink); padding: 0 6px; font-size: var(--c-min); font-weight: 700; white-space: nowrap; line-height: 1.4 }
</style>
