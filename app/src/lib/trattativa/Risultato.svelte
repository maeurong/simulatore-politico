<script lang="ts">
  // Il risultato del voto in testa alla Trattativa (27-trattativa.prototype.html, «risultato»): quote, Camere a barre, frase dei seggi.
  import { t, elenco, percentuale, type Chiave } from "$lib/testi/t";
  import { coloreDi } from "$lib/partiti";
  import type { Vista } from "$lib/tipi/Vista";
  import BarraSeggi from "$lib/componenti/BarraSeggi.svelte";
  import { barreSeggi } from "$lib/componenti/seggi";
  import { risultato } from "./trattativa";

  let { vista }: { vista: Vista } = $props();
  // senza l'incarico (ribaltone, incarico altrui, dopo «Resta fuori») i seggi «della tua coalizione» non dicono niente di vero
  const fuori = $derived(vista.incarico !== "giocatore");
  const r = $derived(risultato(vista));
  const data = $derived(vista.elezioni.at(-1)?.data);
  const periodo = $derived(data ? t(`cornice.trimestre_${data.trimestre}` as Chiave, { anno: data.anno }).toLowerCase() : "");
  const rami = $derived(r ? barreSeggi(vista, r.voti).map((b) => ({ ...b, ...r[b.ramo] })) : []);
  const mancano = $derived(
    rami.filter((b) => b.seggi < b.soglia).map((b) => t(b.ramo === "camera" ? "campagna.alla_camera" : "trattativa.al_senato", { n: b.soglia - b.seggi })),
  );
  const seggi = $derived({ camera: r?.camera.seggi ?? 0, senato: r?.senato.seggi ?? 0 });
  const frase = $derived(mancano.length ? t("trattativa.frase_mancano", { ...seggi, dove: elenco(mancano, ", ") }) : t("trattativa.frase_maggioranza", seggi));
</script>

{#if r}
  <section class="ris" aria-labelledby="ris-t">
    <h3 class="sez-l" id="ris-t">{t("trattativa.il_voto_di", { periodo })}<span>{t("trattativa.affluenza", { x: percentuale(r.affluenza, 1) })} · {r.premio ? t("trattativa.premio", { chi: elenco(r.premio, ", ") }) : t("trattativa.nessun_premio")}</span></h3>
    <div>
      <ul class="vt">
        {#each r.voti as x (x.partito)}
          <li><i class="sw" style:background={coloreDi(x.partito) ?? undefined} class:nato={coloreDi(x.partito) === null}></i>{#if x.miei && !fuori}<b>{x.nome}</b>{:else}{x.nome}{/if} {percentuale(x.quota, 1)}</li>
        {/each}
      </ul>
      {#each rami as b (b.ramo)}
        <p class="ramo"><b>{t(`edicola.${b.ramo}`)}</b>{fuori ? "" : ` · ${t("paese.seggi_coalizione", { n: b.seggi, tot: b.totale, meta: b.soglia })}`}</p>
        <BarraSeggi righe={b.righe} totale={b.totale} segno={fuori ? undefined : b.seggi} />
      {/each}
    </div>
    <p class="mg">{#if fuori}<span class="min muto">{t("trattativa.filo_solo")}</span>{:else}{frase}<br /><span class="min muto">{t("trattativa.filo")}</span>{/if}</p>
  </section>
{/if}

<style>
  .ris { display: grid; grid-template-columns: minmax(0, 1fr) 260px; gap: 4px 18px; border-bottom: var(--doppio); padding-bottom: 8px }
  .sez-l { grid-column: 1 / -1; font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--grigio); border-bottom: 1.5px solid var(--ink); padding-bottom: 2px; display: flex; justify-content: space-between; gap: 8px }
  .sez-l span { letter-spacing: 0; text-transform: none; font-weight: 400 }
  .vt { list-style: none; display: flex; flex-wrap: wrap; gap: 2px 12px; font-size: var(--c-min); margin: 2px 0 6px }
  .vt li { white-space: nowrap }
  .sw { display: inline-block; width: 10px; height: 10px; border: 1px solid var(--ink); margin-right: 5px; vertical-align: -1px }
  .nato { background: var(--retino), var(--carta) }
  .ramo { font-size: var(--c-sec) }
  .mg { font-size: var(--c-sec); align-self: end }
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }
</style>
