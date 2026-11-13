<script module lang="ts">
  import type { AnteprimaFondazione } from "$lib/tipi/AnteprimaFondazione";
  import type { PartitoCatalogo } from "$lib/tipi/PartitoCatalogo";

  // Sopravvive a «‹ Scenario», come l'editor: solo in memoria. `per` è lo scenario dell'anteprima: il guscio tiene il
  // seme dell'ultima anteprima e la fondazione lo riusa, quindi si richiede solo se lo scenario è cambiato.
  // `giro` è quello dell'editor: se è avanzato si è usciti all'edicola e si riparte da capo.
  // `chiesta` numera le richieste d'anteprima: vale solo la risposta dell'ultima.
  const m = $state({
    nome: "",
    posto: null as number | null,
    anteprima: null as AnteprimaFondazione | null,
    per: "",
    giro: -1,
    chiesta: 0,
    partiti: null as PartitoCatalogo[] | null,
  });
  function azzera() {
    m.nome = "";
    m.posto = null;
    m.anteprima = null;
    m.per = "";
  }
</script>

<script lang="ts">
  // Fondazione, forma A · Banco (.scratch/v1-piano-11-ui/prototipi/15-fondazione.prototype.html:255-273): fuori dalla
  // partita, senza cornice. Nome e asse a sinistra, il posto scelto nel dettaglio a lato, «Fonda il partito ›» in fondo.
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { comeErrore, guscio } from "$lib/guscio/chiamate";
  import { partita } from "$lib/partita.svelte";
  import { casa } from "$lib/ossatura";
  import { numero, t } from "$lib/testi/t";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import { editor } from "$lib/editor/editor.svelte";
  import { cosaManca, LUNGHEZZA_NOME, nomeValido, type StatoAnteprima } from "$lib/fondazione/fondazione";
  import Asse from "$lib/fondazione/Asse.svelte";
  import TabellaPosti from "$lib/fondazione/TabellaPosti.svelte";
  import DettaglioPosto from "$lib/fondazione/DettaglioPosto.svelte";

  let guasto = $state<string | null>(null);
  /** il rifiuto di anteprima_fondazione: sta al posto dell'asse */
  let rifiutoAnteprima = $state<string | null>(null);
  /** il rifiuto di fonda_partita: sta sopra il bottone */
  let rifiuto = $state<string | null>(null);
  /** dal clic alla fine della navigazione: partita.occupato torna falso prima che goto finisca */
  let fondazione = $state(false);
  /** l'anteprima di questo scenario è finita in un rifiuto o in un guasto */
  let fallita = $state(false);

  // lo scenario non cambia mentre si sta qui: cambia solo tornando all'editor
  const per = JSON.stringify(editor.scenario);
  const occupato = $derived(partita.occupato || fondazione);
  const statoAnteprima = $derived<StatoAnteprima>(m.anteprima && m.per === per ? "pronta" : fallita ? "fallita" : "in_arrivo");
  const manca = $derived(cosaManca(m.nome, m.posto, statoAnteprima));
  const voce = $derived(m.anteprima?.posizioni.find((p) => p.posizione === m.posto) ?? null);
  const scenario = $derived(
    !editor.scenario ? t("editor.scenario_base")
    : !editor.mosso ? (editor.sorgente ?? editor.scenario.nome)
    : editor.sorgente ? t("editor.nome_modificato", { nome: editor.sorgente })
    : t("editor.nuovo_senza_nome"),
  );

  onMount(async () => {
    if (m.giro !== editor.giro) azzera();
    m.giro = editor.giro;
    partita.guasto = null;
    let n = 0;
    try {
      // arrivando senza passare dall'editor (ricarica, URL diretto) il catalogo non c'è: si chiede al guscio
      m.partiti ??= (await guscio.catalogoScenario()).partiti;
      if (!m.anteprima || m.per !== per) {
        m.anteprima = null;
        n = ++m.chiesta;
        const a = await guscio.anteprimaFondazione(editor.scenario);
        // superata da un rientro con un altro scenario: si scarta
        if (n !== m.chiesta) return;
        m.anteprima = a;
        m.per = per;
      }
    } catch (e) {
      if (n && n !== m.chiesta) return;
      fallita = true;
      const x = comeErrore(e);
      if (x.specie === "rifiuto") rifiutoAnteprima = x.testo;
      else guasto = `${x.testo} ${t("fondazione.guasto_anteprima")}`;
    }
  });

  /** Diretto, senza conferma: il guasto lo scrive partita in partita.guasto, il rifiuto torna qui. */
  async function fonda() {
    if (occupato || manca !== null) return;
    rifiuto = guasto = null;
    partita.guasto = null;
    fondazione = true;
    try {
      const vista = await partita.fonda(editor.scenario, { nome: m.nome.trim(), posizione: m.posto! });
      if (vista) {
        await goto(`/partita/${casa(vista)}`);
        azzera();
      }
    } catch (e) {
      rifiuto = comeErrore(e).testo;
    } finally {
      fondazione = false;
    }
  }

  const indietro = () => !occupato && goto("/scenario");
  // il nome in testa allo scenario in neretto: il testo si spezza sul segnaposto
  const scenarioPrima = t("fondazione.scenario", { nome: "\u0001" }).split("\u0001")[0];
</script>

<div class="fondazione">
  <header class="testa">
    <Pulsante forma="link" aria-disabled={occupato ? "true" : undefined} onclick={indietro}>{t("fondazione.indietro")}</Pulsante>
    <span class="nomino">{t("cornice.nomino")}</span>
    <h1 class="dove">{t("fondazione.dove")}</h1>
    <p class="fuori">{scenarioPrima}<b>{scenario}</b></p>
  </header>
  <p class="nascosto" role="status" data-tagli-ignora>{occupato ? t("fondazione.in_corso") : ""}</p>
  {#if partita.guasto ?? guasto}<p class="guasto" role="alert">{partita.guasto ?? guasto}</p>{/if}

  <main class="corpo">
    <div class="sx">
      <section aria-labelledby="titolo-nome">
        <div class="titsez"><h2 id="titolo-nome" class="tit">{t("fondazione.il_nome")}</h2></div>
        <label class="nome">
          <input bind:value={m.nome} maxlength={LUNGHEZZA_NOME} placeholder={t("fondazione.segnaposto_nome")}
            aria-label={t("fondazione.etichetta_nome")} aria-describedby="contatore" data-tagli-ignora />
          <span id="contatore" class="cnt">{t("fondazione.contatore", { n: m.nome.length, max: LUNGHEZZA_NOME })}</span>
        </label>
      </section>

      <section aria-labelledby="titolo-posto">
        <div class="titsez">
          <h2 id="titolo-posto" class="tit">{t("fondazione.il_posto")}</h2>
          <span class="dx min muto">{t("fondazione.sei_mezzi_punti")}</span>
        </div>
        {#if m.anteprima && m.partiti}
          <Asse anteprima={m.anteprima} partiti={m.partiti} bind:posto={m.posto} etichetta="titolo-posto" />
        {:else if rifiutoAnteprima}
          <Rifiuto testo={rifiutoAnteprima} />
        {/if}
      </section>

      {#if m.anteprima && m.partiti}
        <TabellaPosti anteprima={m.anteprima} partiti={m.partiti} bind:posto={m.posto} />
      {/if}
    </div>

    <aside class="lato">
      {#if voce && m.anteprima && m.partiti}
        <DettaglioPosto anteprima={m.anteprima} {voce} partiti={m.partiti} />
      {:else}
        <p class="vuoto">{t("fondazione.vuoto")}</p>
      {/if}
      <div class="fonda">
        <p class="chi">
          {#if m.posto === null}{nomeValido(m.nome) ? m.nome.trim() : t("fondazione.il_vostro_partito")}
          {:else}{t("fondazione.chi_posto", { nome: nomeValido(m.nome) ? m.nome.trim() : t("fondazione.il_vostro_partito"), posto: numero(m.posto, 1) })}{/if}
        </p>
        {#if !manca}<p class="min muto">{t("fondazione.si_vota_alla_chiusura")}</p>{/if}
        {#if rifiuto}<Rifiuto testo={rifiuto} />{/if}
        <div>
          <Pulsante forma="chiude" spento={manca ? t(manca) : undefined} aria-disabled={occupato ? "true" : undefined}
            aria-busy={occupato || undefined} onclick={fonda}>{t("fondazione.fonda_il_partito")}</Pulsante>
        </div>
      </div>
    </aside>
  </main>
</div>

<style>
  /* la pagina sta ferma; scorrono la colonna di sinistra e, se non basta, il dettaglio */
  .fondazione { position: fixed; inset: 0; display: flex; flex-direction: column; overflow: hidden }
  .testa { flex: none; display: flex; align-items: end; gap: 16px; padding: 12px var(--marg) 8px; border-bottom: var(--doppio) }
  .nomino { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 800; font-size: var(--c-pag); color: var(--rosso); line-height: 1; white-space: nowrap }
  .dove { font-family: var(--tit); font-size: var(--c-sez); font-weight: 600; line-height: 1.1 }
  .fuori { margin-left: auto; padding-bottom: 3px; text-align: right; font-size: var(--c-sec); color: var(--grigio) }
  .fuori b { color: var(--ink) }
  .guasto { flex: none; padding: 4px var(--marg); border-bottom: var(--filo); font-size: var(--c-sec); font-weight: 700; color: var(--rosso) }

  .corpo { flex: 1; min-height: 0; display: grid; grid-template-columns: minmax(0, 1fr) var(--lato); grid-template-rows: minmax(0, 1fr); gap: 12px var(--canale); padding: 14px var(--marg) 16px; overflow: hidden }
  .sx { display: grid; gap: 18px; align-content: start; min-width: 0; min-height: 0; overflow: hidden auto; padding-right: 6px }
  /* il filetto finisce col dettaglio, come nel prototipo; oltre l'altezza del corpo il dettaglio scorre */
  .lato { align-self: start; max-height: 100%; border-left: var(--bordo); padding-left: var(--canale); display: grid; gap: 12px; align-content: start; min-width: 0; min-height: 0; overflow: hidden auto }

  .titsez { display: flex; align-items: baseline; gap: 12px; border-bottom: var(--forte); padding-bottom: 4px; margin-bottom: 6px }
  .titsez .dx { margin-left: auto }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance; font-size: var(--c-riq) }
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }

  .nome { display: flex; gap: 10px; align-items: baseline }
  .nome input { font-family: var(--tit); font-weight: 700; font-size: var(--c-sez); color: var(--ink); border: 0; border-bottom: var(--forte); background: transparent; padding: 2px; min-width: 0; flex: 1 }
  .nome input::placeholder { color: var(--grigio); opacity: .7 }
  .cnt { font-size: var(--c-min); color: var(--grigio); white-space: nowrap }

  .vuoto { border: 1.5px dashed var(--grigio); color: var(--grigio); padding: 14px; font-size: var(--c-sec) }
  .fonda { border-top: var(--doppio); padding-top: 10px; display: grid; gap: 6px }
  .chi { font-family: var(--tit); font-weight: 700; font-size: var(--c-riq); overflow-wrap: anywhere }
</style>
