<script lang="ts">
  // La tabella dei sei posti (15-fondazione.prototype.html:258-261, 271): sceglie la stessa cosa dell'asse.
  // Niente quote della coalizione né seggi: l'anteprima non li porta.
  import { numero, percentuale, t } from "$lib/testi/t";
  import type { AnteprimaFondazione } from "$lib/tipi/AnteprimaFondazione";
  import type { PartitoCatalogo } from "$lib/tipi/PartitoCatalogo";
  import { fra } from "./fondazione";

  type Props = { anteprima: AnteprimaFondazione; partiti: PartitoCatalogo[]; posto: number | null };
  let { anteprima, partiti, posto = $bindable() }: Props = $props();

  const massimo = $derived(Math.max(...anteprima.posizioni.map((p) => p.quota_attesa)));
  const nomeDi = (id: string) => partiti.find((p) => p.id === id)?.nome ?? id;
</script>

<table class="posti">
  <thead><tr><th>{t("fondazione.posto")}</th><th class="n">{t("fondazione.atteso")}</th><th>{t("fondazione.alleati_disponibili")}</th></tr></thead>
  <tbody>
    {#each anteprima.posizioni as p (p.posizione)}
      {@const [a, b] = fra(p.posizione, partiti)}
      {@const alleati = p.disponibilita.filter((d) => d.disponibile)}
      <!-- il clic sulla riga è comodità: da tastiera si sceglie sull'asse -->
      <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_noninteractive_element_interactions -->
      <tr class={{ qui: p.posizione === posto }} onclick={() => (posto = p.posizione)}>
        <td><b>{numero(p.posizione, 1)}</b> <span class="muto">{t("fondazione.fra", { a: a.nome, b: b.nome })}</span></td>
        <td class="n"><span class="qb" style:width="{(p.quota_attesa / massimo) * 70}px"></span>{percentuale(p.quota_attesa)}</td>
        <td>{#if alleati.length}{alleati.map((d) => nomeDi(d.partito)).join(", ")}{:else}<span class="muto">{t("fondazione.nessuno")}</span>{/if}</td>
      </tr>
    {/each}
  </tbody>
</table>

<style>
  .posti { width: 100%; border-collapse: collapse; font-size: var(--c-sec) }
  th { font-size: var(--c-min); letter-spacing: .06em; text-transform: uppercase; color: var(--grigio); font-weight: 700; text-align: left; padding: 2px 6px; border-bottom: var(--filo) }
  td { padding: 4px 6px; border-bottom: var(--filo); vertical-align: baseline }
  tr { cursor: pointer }
  thead tr { cursor: default }
  tbody tr:hover td { background: var(--velo) }
  tr.qui td { background: var(--ink); color: var(--carta) }
  tr.qui .muto { color: var(--carta); opacity: .8 }
  .n { text-align: right; white-space: nowrap; font-weight: 700 }
  .muto { color: var(--grigio) }
  .qb { display: inline-block; height: 8px; background: var(--rosso); vertical-align: middle; margin-right: 5px }
  tr.qui .qb { background: var(--giallo) }
</style>
