<script lang="ts">
  // Il dettaglio del posto scelto (15-fondazione.prototype.html:262-266): quota attesa, la nota sul sondaggio,
  // gli elettori dei due punti accanto, «Con chi correre» coi sette partiti e la causa del core.
  import { numero, percentuale, t, type Chiave } from "$lib/testi/t";
  import type { AnteprimaFondazione } from "$lib/tipi/AnteprimaFondazione";
  import type { PartitoCatalogo } from "$lib/tipi/PartitoCatalogo";
  import type { VocePosizione } from "$lib/tipi/VocePosizione";
  import { conChiCorrere, fraTesto } from "./fondazione";

  type Props = { anteprima: AnteprimaFondazione; voce: VocePosizione; partiti: PartitoCatalogo[] };
  let { anteprima, voce, partiti }: Props = $props();

  const lato = (x: number): Chiave =>
    x < 3 ? "fondazione.a_sinistra" : x > 5 ? "fondazione.a_destra" : x < 4 ? "fondazione.al_centrosinistra" : "fondazione.al_centrodestra";
  const voci = $derived(conChiCorrere(voce, partiti));
  const disponibili = $derived(voci.filter((v) => v.disponibile).length);
  // le due quote in neretto: il testo si spezza sui segnaposto
  const SEGNO = "\u0001";
  const accanto = $derived(t("fondazione.accanto_a_voi", { a: SEGNO, b: SEGNO }).split(SEGNO));
  const quotaAccanto = (i: number) => percentuale(anteprima.istogramma[Math.floor(voce.posizione) - 1 + i]);
</script>

<div>
  <p class="occ">{t("fondazione.occhiello_posto", { posto: numero(voce.posizione, 1), lato: t(lato(voce.posizione)) })}</p>
  <h2 class="tit">{fraTesto(voce.posizione, partiti)}</h2>
</div>
<div>
  <p class="grandenum">{percentuale(voce.quota_attesa)}</p>
  <p class="sec muto">{t("fondazione.nota_atteso")}</p>
  <p class="min muto nota">{t("fondazione.e_un_sondaggio")}</p>
</div>
<p class="sec">{accanto[0]}<b>{quotaAccanto(0)}</b>{accanto[1]}<b>{quotaAccanto(1)}</b>{accanto[2]}</p>
<section aria-labelledby="titolo-con-chi">
  <div class="titsez">
    <h3 id="titolo-con-chi" class="tit">{t("fondazione.con_chi_correre")}</h3>
    <span class="dx min muto">{t("fondazione.disponibili", { n: disponibili })}</span>
  </div>
  <ul class="alleati">
    {#each voci as v (v.partito.id)}
      <li class={["al", v.disponibile ? "si" : "no"]}>
        <span class="seg" aria-hidden="true">{v.disponibile ? "✓" : "✕"}</span>
        <span class="nm"><span class="nascosto">{t(v.disponibile ? "fondazione.si_disponibile" : "fondazione.non_disponibile")}</span>
          {v.partito.nome}</span>
        <span class="ca">{v.causa}</span>
      </li>
    {/each}
  </ul>
</section>

<style>
  .occ { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--rosso) }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance }
  h2.tit { font-size: var(--c-sez) }
  h3.tit { font-size: var(--c-riq) }
  .grandenum { font-family: var(--tit); font-weight: 800; font-size: var(--c-test); line-height: 1; color: var(--rosso) }
  .nota { margin-top: 2px }
  .sec { font-size: var(--c-sec) }
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }

  .titsez { display: flex; align-items: baseline; gap: 12px; border-bottom: var(--forte); padding-bottom: 4px; margin-bottom: 6px }
  .titsez .dx { margin-left: auto }
  .alleati { list-style: none; display: grid }
  .al { display: grid; grid-template-columns: 18px minmax(0, 1fr); gap: 0 6px; padding: 3px 0; border-bottom: var(--filo); font-size: var(--c-sec); align-items: baseline }
  .seg { font-weight: 700 }
  .al.si .seg { color: var(--verde) }
  .al.si .nm { font-weight: 700 }
  .al.no, .al.no .seg { color: var(--grigio) }
  .ca { grid-column: 2; font-size: var(--c-min); color: var(--grigio) }
</style>
