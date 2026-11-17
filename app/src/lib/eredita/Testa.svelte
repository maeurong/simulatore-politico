<script lang="ts">
  // La testa dell'ultima edizione (30-eredita.prototype.html, .apre): occhiello serio col fatto, titolo, bollo, riga.
  import { anniAlGoverno, esitoTesto, periodo } from "$lib/edicola/edicola";
  import { t, type Chiave } from "$lib/testi/t";
  import { occhiello, titolo, type Edizione } from "./eredita";

  let { edizione }: { edizione: Edizione } = $props();
  const inizio = $derived(t(`cornice.trimestre_${edizione.inizio.trimestre}` as Chiave, { anno: edizione.inizio.anno }));
  const riga = $derived(
    t("eredita.riga", {
      scenario: edizione.scenario, inizio, fine: periodo(edizione.fine), turni: t("edicola.turni", { n: edizione.turni }),
      governo: anniAlGoverno(edizione.anni_al_governo),
      riavvolgimenti: edizione.riavvolgimenti ? t("edicola.riavvolgimenti", { n: edizione.riavvolgimenti }) : t("eredita.nessun_riavvolgimento"),
    }),
  );
  const esito = $derived(esitoTesto(edizione.esito === "abbandono" ? "Abbandono" : "Senza seggi"));
</script>

<article class="apre">
  <div>
    <p class="o">{occhiello(edizione)}</p>
    <h2 class="tit">{titolo(edizione)}</h2>
  </div>
  <span class="bollo" class:ink={edizione.esito === "abbandono"}>{esito}</span>
  <p class="riga">{riga}.</p>
</article>

<style>
  .apre { display: grid; grid-template-columns: minmax(0, 1fr) auto; gap: 4px 18px; padding-bottom: 10px; border-bottom: 1px solid var(--ink); align-items: end }
  .o { font-size: 17px; font-weight: 700 }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.04; text-wrap: balance; font-size: var(--c-apre) }
  .riga { font-size: var(--c-lead); color: var(--grigio); grid-column: 1 / -1 }
  .bollo {
    display: inline-block; border: var(--doppio); border-color: var(--rosso); color: var(--rosso); font-weight: 700; letter-spacing: .12em;
    text-transform: uppercase; padding: 1px 9px; transform: rotate(-5deg); font-size: var(--c-min); white-space: nowrap;
  }
  .bollo.ink { border-color: var(--ink); color: var(--ink) }
</style>
