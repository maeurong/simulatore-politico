<script lang="ts">
  // Le tre colonne dell'ultima edizione: le elezioni, il Paese che lasci, le leggi che restano (30-eredita.prototype.html, .col3).
  // Le leggi portano l'etichetta solo dalla Vista (schema 19); riletta dall'albo, solo il nome.
  import { periodo, maleSe } from "$lib/edicola/edicola";
  import NumeroFreccia from "$lib/componenti/NumeroFreccia.svelte";
  import { numero, percentuale, t } from "$lib/testi/t";
  import type { Edizione } from "./eredita";

  let { edizione }: { edizione: Edizione } = $props();
</script>

<div class="col3">
  <section>
    <div class="titsez"><h3 class="tit">{t("eredita.le_elezioni")}</h3></div>
    {#each edizione.elezioni as e, k (k)}
      {@const tot = e.voti.reduce((s, v) => s + v[1], 0) || 1}
      {@const mio = e.voti.find((v) => v[0] === edizione.nome)}
      {@const [primaQuota, dopoQuota] = t("edicola.esito_giocatore", { nome: edizione.nome, camera: e.seggi_giocatore[0], senato: e.seggi_giocatore[1] }).split("{quota}")}
      <div class="el">
        <p class="sec"><b>{t("edicola.voto_di", { periodo: periodo(e.data) })}</b> <span class="muto">{t("edicola.affluenza", { quota: percentuale(e.affluenza) })}</span></p>
        <div class="barra" role="img" aria-label={e.voti.map(([n, q]) => `${n} ${percentuale(q)}`).join(", ")}>
          {#each e.voti as [n, q], j (j)}<i class:mio={n === edizione.nome} style:flex={q / tot} title="{n} {percentuale(q)}"></i>{/each}
        </div>
        <div class="etich" aria-hidden="true">
          {#each e.voti as [n, q], j (j)}<span class:mio={n === edizione.nome} style:flex={q / tot} data-tagli-ignora={n === edizione.nome ? undefined : ""}>{q > 0.07 ? `${n.split(" ").pop()} ${numero(q * 100)}%` : ""}</span>{/each}
        </div>
        <p class="sec">{primaQuota}<b>{mio ? percentuale(mio[1]) : t("edicola.assente")}</b>{dopoQuota}</p>
      </div>
    {/each}
  </section>

  <section>
    <div class="titsez"><h3 class="tit">{t("eredita.il_paese")}</h3><span class="dx min muto">{t("eredita.contro_zero")}</span></div>
    <table class="ind">
      <thead><tr><th>{t("edicola.indicatore")}</th><th class="r">{t("edicola.turno_zero")}</th><th class="r">{t("edicola.fine")}</th><th><span class="nascosto">{t("edicola.scarto")}</span></th></tr></thead>
      <tbody>
        {#each edizione.indicatori as i (i.id)}
          {@const decimali = i.id === "spread" ? 0 : 1}
          <tr>
            <td class="muto">{i.nome}</td>
            <td class="r">{numero(i.turno_zero, decimali)}</td>
            <td class="r"><b>{numero(i.fine, decimali)}</b></td>
            <td class="r"><NumeroFreccia valore={i.fine} prima={i.turno_zero} formato={() => ""} formatoVariazione={(x) => numero(x, decimali)} male={maleSe(i.id)} /></td>
          </tr>
        {/each}
      </tbody>
    </table>
  </section>

  <section>
    <div class="titsez"><h3 class="tit">{t("eredita.le_leggi")}</h3><span class="dx min muto">{edizione.leggi.length}</span></div>
    {#if edizione.leggi.length}
      <ul class="leggi">
        {#each edizione.leggi as l (l.nome)}
          <li><b>{l.nome}</b>{#if l.legge}<span class="min muto">{l.legge}</span>{/if}</li>
        {/each}
      </ul>
    {:else}
      <p class="sec muto">{t("eredita.nessuna_legge")}</p>
    {/if}
  </section>
</div>

<style>
  .col3 { display: grid; grid-template-columns: minmax(0, 1fr) minmax(0, 1.3fr) minmax(0, .9fr); gap: 0 var(--canale) }
  .col3 > section { border-left: var(--filo); padding-left: var(--canale); min-width: 0 }
  .col3 > section:first-child { border-left: 0; padding-left: 0 }
  .titsez { display: flex; align-items: baseline; gap: 12px; border-bottom: var(--forte); padding-bottom: 4px; margin-bottom: 6px }
  .titsez .dx { margin-left: auto }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance; font-size: var(--c-riq) }
  .sec { font-size: var(--c-sec) }
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }

  .ind { width: 100%; border-collapse: collapse; font-size: var(--c-sec) }
  .ind th { font-size: var(--c-min); color: var(--grigio); letter-spacing: .06em; text-transform: uppercase; font-weight: 400; text-align: left; padding: 0 0 3px }
  .ind td { padding: 2px 0 }
  .ind td + td, .ind th + th { padding-left: 8px }
  .ind .r { text-align: right; white-space: nowrap }
  .ind th.r { white-space: normal }

  .el { display: grid; gap: 4px; padding-top: 6px; border-top: var(--filo) }
  .el:first-child { border-top: 0; padding-top: 0 }
  .barra { display: flex; gap: 2px; height: 16px }
  .barra i { background: var(--grigio); min-width: 0 }
  .barra i:nth-child(even) { background: var(--ink) }
  .barra i.mio { background: var(--rosso) }
  /* contain: le etichette, che ci stiano o no, non allargano la colonna */
  .etich { display: flex; gap: 2px; font-size: var(--c-min); color: var(--grigio); contain: inline-size }
  /* un'etichetta che non ci sta non si mostra: va a capo dietro un primo elemento vuoto e l'altezza di una riga la taglia */
  /* 1.35em è l'interlinea della radice: il ripiego per un WebKit senza lh */
  .etich span { min-width: 0; display: flex; flex-wrap: wrap; height: 1.35em; height: 1lh; overflow: hidden; white-space: nowrap }
  .etich span::before { content: ""; height: 1.35em; height: 1lh }
  .etich span.mio { min-width: max-content; color: var(--rosso); font-weight: 700 }

  .leggi { list-style: none; display: grid }
  .leggi li { padding: 5px 0; border-top: var(--filo); font-size: var(--c-sec); display: grid; grid-template-columns: minmax(0, 1fr) auto; gap: 2px 10px }
  .leggi li:first-child { border-top: 0 }
</style>
