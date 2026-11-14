<script lang="ts">
  // Le spese della campagna (26-campagna.prototype.html, forma A): fino a tre riquadri; ogni modifica chiede l'anteprima, e il comando parte da Campagna.
  import { t, percentuale, numero } from "$lib/testi/t";
  import type { Condizione } from "$lib/tipi/Condizione";
  import type { Spesa } from "$lib/tipi/Spesa";
  import type { Vista } from "$lib/tipi/Vista";
  import type { VoceAnteprimaCampagna } from "$lib/tipi/VoceAnteprimaCampagna";
  import { partita } from "$lib/partita.svelte";
  import Caselle from "$lib/componenti/Caselle.svelte";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import { LEVE, MASSIMO_SPESE, aggiungiSpesa, conto, nomeBersaglio, quotaNelBersaglio, taglio } from "./campagna";

  const CONDIZIONI: Condizione[] = ["dipendente_privato", "dipendente_pubblico", "autonomo", "pensionato", "disoccupato_inattivo", "studente"];
  const FASCE = { poco: 1, medio: 2, molto: 3 };

  let { vista, spese, voci, rifiuto, modifica, esci, apriRegione }: {
    vista: Vista;
    spese: Spesa[];
    /** l'ultima anteprima riuscita, una voce per spesa */
    voci: VoceAnteprimaCampagna[] | null;
    /** il rifiuto del core e la spesa che l'ha causato */
    rifiuto: { indice: number; testo: string } | null;
    modifica: (spese: Spesa[], indice: number) => void;
    esci: () => void;
    apriRegione: () => void;
  } = $props();

  const taglia = $derived(taglio(spese, conto(vista, spese).cassa));
  const presi = $derived(new Set(spese.map((s) => JSON.stringify(s.bersaglio))));
  const occupato = $derived(partita.occupato ? "true" : undefined);

  // le mosse discrete partono subito; l'importo parte all'uscita dal campo
  function subito(nuove: Spesa[], indice: number) {
    if (partita.occupato) return;
    modifica(nuove, indice);
    esci();
  }
  const con = (i: number, parte: Partial<Spesa>) => spese.map((s, k) => (k === i ? { ...s, ...parte } : s));
  const aggiungi = (condizione: Condizione) => {
    const bersaglio = { condizione };
    if (!presi.has(JSON.stringify(bersaglio))) subito(aggiungiSpesa(spese, bersaglio), spese.length);
  };
  const euroDi = (testo: string) => Number(testo.replace(/\D/g, ""));
</script>

<section aria-label={t("campagna.spesa")}>
  {#each spese as s, i (i)}
    {@const nome = nomeBersaglio(vista, s.bersaglio)}
    {@const q = quotaNelBersaglio(vista, s.bersaglio)}
    {@const voce = voci?.length === spese.length ? voci[i] : null}
    {@const tg = taglia[i]}
    <div class="sp">
      <div class="t">
        <b>{nome}</b>
        <Pulsante forma="link" aria-disabled={occupato} aria-label={t("campagna.togli_spesa_di", { nome })} onclick={() => subito(spese.filter((_, k) => k !== i), i)}>{t("campagna.togli_spesa")}</Pulsante>
      </div>
      <div class="min muto">
        {q.pubblica === null ? "" : t("campagna.quota_pubblica", { q: percentuale(q.pubblica) })}{q.privata === null ? "" : `, ${t("campagna.quota_privata", { q: percentuale(q.privata) })}`}
      </div>
      <div class="leve" role="group" aria-label={t("campagna.leva")}>
        {#each LEVE as l (l)}
          <Pulsante forma="piccolo" scelto={s.leva === l} title={t(`leva.${l}_effetto`)} aria-disabled={occupato} onclick={() => s.leva !== l && subito(con(i, { leva: l }), i)}>{t(`leva.${l}`)}</Pulsante>
        {/each}
      </div>
      <p class="nascosto">{t(`leva.${s.leva}`)}: {t(`leva.${s.leva}_effetto`)}</p>
      <div class="riga">
        <label class="euro">
          <input type="text" inputmode="numeric" autocomplete="off" aria-label={t("campagna.importo", { nome })} value={s.euro ? String(s.euro) : ""}
            oninput={(e) => modifica(con(i, { euro: euroDi(e.currentTarget.value) }), i)} onblur={esci}
            onkeydown={(e) => e.key === "Enter" && (e.preventDefault(), esci())} /> €
        </label>
        <span class="min muto">{numero(s.euro)} €</span>
        {#if voce}<Caselle piene={FASCE[voce.intensita]} su={3} nome={voce.intensita} />{/if}
      </div>
      {#if voce}<div class="min muto"><i>{t("componenti.perche")}</i> {voce.causa}</div>{/if}
      {#if tg.esito !== "intera"}
        <div class="avv min"><b>{t("campagna.si_taglia")}</b> {tg.esito === "cade" ? t("campagna.cade") : t("campagna.scende_a", { euro: numero(tg.paga) })}</div>
      {/if}
      {#if rifiuto?.indice === i}<div class="rif"><Rifiuto testo={rifiuto.testo} /></div>{/if}
    </div>
  {/each}
  {#if spese.length >= MASSIMO_SPESE}
    <div class="sp vuota">{t("campagna.massimo_tre")}</div>
  {:else}
    <div class="sp vuota">
      <span>{t("campagna.bersaglio_n", { n: spese.length + 1 })}</span>
      <div class="chips">
        {#each CONDIZIONI as c (c)}
          {@const preso = presi.has(JSON.stringify({ condizione: c }))}
          <Pulsante forma="piccolo" aria-disabled={preso || partita.occupato ? "true" : undefined} title={preso ? t("campagna.bersaglio_ripetuto") : undefined} onclick={() => aggiungi(c)}>{t(`condizione.${c}`)}</Pulsante>
        {/each}
        <Pulsante forma="piccolo" aria-disabled={occupato} onclick={() => !partita.occupato && apriRegione()}>{t("campagna.una_regione")}</Pulsante>
      </div>
      {#if rifiuto?.indice === spese.length}<div class="rif"><Rifiuto testo={rifiuto.testo} /></div>{/if}
    </div>
  {/if}
  <p class="min muto">{t("campagna.si_paga")}</p>
</section>

<style>
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }
  .sp { border: var(--bordo); padding: 7px 9px; margin-bottom: 8px; display: grid; gap: 5px }
  .sp.vuota { border-style: dashed; border-color: var(--grigio); color: var(--grigio); font-size: var(--c-sec) }
  .t { display: flex; justify-content: space-between; align-items: baseline; gap: 8px }
  .t b { font-family: var(--tit); font-variation-settings: var(--wonk); font-size: var(--c-lead) }
  .leve { display: inline-flex; gap: 4px; flex-wrap: wrap }
  .riga { display: flex; gap: 6px 10px; align-items: center; flex-wrap: wrap }
  .euro input { width: 9ch; border: var(--bordo); padding: 1px 4px; font-weight: 700; font-variant-numeric: tabular-nums; background: var(--carta) }
  .chips { display: flex; flex-wrap: wrap; gap: 4px }
  .avv { border: 1.5px solid var(--rosso); padding: 3px 8px }
  .avv b { color: var(--rosso) }
  .rif { border: 1.5px solid var(--rosso); padding: 4px 8px }
</style>
