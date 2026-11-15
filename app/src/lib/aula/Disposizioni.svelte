<script lang="ts">
  // Le disposizioni in vigore (storia 107), righe chiuse per dicastero; i gradini si aprono solo sulla riga scelta o con un valore già in bozza:
  // in vigore a fondo pieno, in bozza in giallo, in arrivo a tratteggio blu, bloccato a retino.
  import { tick } from "svelte";
  import { t, type Chiave } from "$lib/testi/t";
  import { partita } from "$lib/partita.svelte";
  import { testoRifiuto } from "$lib/guscio/chiamate";
  import type { Comando } from "$lib/tipi/Comando";
  import type { Vista } from "$lib/tipi/Vista";
  import type { VoceDisposizione } from "$lib/tipi/VoceDisposizione";
  import Conferma from "$lib/componenti/Conferma.svelte";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import { perDicastero, type Bozza } from "./bozza";
  import Scala from "./Scala.svelte";

  let { vista, bozza, aperta, onapri, oncambia, manda, rifiuto }: {
    vista: Vista; bozza: Bozza; aperta: string | null; onapri: (id: string | null) => void; oncambia: (disposizione: string, gradino: string) => void;
    manda: (c: Comando) => Promise<boolean>; rifiuto: { testo: string; tipo: Comando["tipo"] } | null;
  } = $props();
  const occupato = $derived(partita.occupato ? t("campagna.comando_in_corso") : undefined);

  const gruppi = $derived(perDicastero(vista));
  const nomeGradino = (d: VoceDisposizione, id: string) => d.gradini.find((g) => g.id === id)?.nome ?? id;
  const periodo = (d: { anno: number; trimestre: number }) => t(`cornice.trimestre_${d.trimestre}` as Chiave, { anno: d.anno }).toLowerCase();
  let radice: HTMLElement;
  // la riga «forzata» si richiude col gradino tolto: se il fuoco stava sul gradino smontato passa al «Cambia ›» della stessa riga
  async function scegli(d: VoceDisposizione, gradino: string) {
    const da = document.activeElement;
    oncambia(d.id, gradino);
    await tick();
    if (da && !da.isConnected) radice.querySelector<HTMLElement>(`[data-riga="${CSS.escape(d.id)}"]`)?.focus();
  }
  // il «Promuovi» confermato sparisce con la risposta (il core non lo ammette più): il fuoco torna al «Proponi ›» della riga
  async function promuovi(d: VoceDisposizione) {
    const dentro = radice.contains(document.activeElement);
    if (!(await manda({ tipo: "promuovi_referendum", disposizione: d.id }))) return;
    await tick();
    if (dentro && !radice.contains(document.activeElement)) radice.querySelector<HTMLElement>(`[data-riga="${CSS.escape(d.id)}"]`)?.focus();
  }
  const inEsame = (d: VoceDisposizione) => vista.calendario.some((m) => m.valori.some((x) => x.disposizione === d.id));
</script>

<div class="disp" bind:this={radice} role="group" aria-label={t("aula.disposizioni")}>
  {#each gruppi as g (g.nome)}
    <div class="cg"><span>{g.nome}</span></div>
    {#each g.voci as d (d.id)}
      {@const forzata = bozza.some((x) => x.disposizione === d.id)}
      {@const aperto = aperta === d.id || forzata}
      <div class="dr" class:sel={aperto}>
        <span class="nm">{d.nome}<small>{d.legge}{d.precedente !== d.in_vigore ? ` · ${t("aula.disp_prima", { gradino: nomeGradino(d, d.precedente) })}` : ""}</small></span>
        {#if aperto}
          <Scala {d} {bozza} onscegli={(g) => scegli(d, g)} />
        {:else}
          <span><b>{nomeGradino(d, d.in_vigore)}</b>{#each d.in_arrivo as a (a.gradino)} <span class="arr">{t("aula.disp_arrivo", { gradino: nomeGradino(d, a.gradino), periodo: periodo(a.data) })}</span>{/each}</span>
        {/if}
        <span class="x">
          {#if inEsame(d)}<span class="chip">{t("aula.in_esame")}</span>{/if}
          {#if !forzata}<Pulsante forma="link" data-riga={d.id} aria-label={t(aperto ? "aula.chiudi_di" : vista.governa ? "aula.cambia_di" : "aula.proponi_di", { nome: d.nome })} onclick={() => onapri(aperto ? null : d.id)}>{aperto ? t("aula.chiudi_riga") : t(vista.governa ? "aula.cambia" : "aula.proponi")}</Pulsante>{/if}
        </span>
        {#if aperto && !vista.governa}
          <div class="ref">
            {#if d.rifiuto_referendum !== null}
              <span class="min muto">{testoRifiuto({ specie: "rifiuto", testo: d.rifiuto_referendum })}</span>
            {:else}
              <Conferma etichetta={t("aula.referendum_promuovi")} conferma={t("aula.referendum_promuovi_conferma")} spento={occupato}
                onconferma={() => promuovi(d)} />
              {#if rifiuto?.tipo === "promuovi_referendum" && aperta === d.id}<Rifiuto testo={rifiuto.testo} />{/if}
            {/if}
          </div>
        {/if}
      </div>
    {/each}
  {/each}
</div>

<style>
  .disp { display: grid; align-content: start }
  .cg { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; margin-top: 12px; padding-bottom: 2px; border-bottom: 1.5px solid var(--ink) }
  .cg:first-child { margin-top: 0 }
  .dr { display: grid; grid-template-columns: minmax(150px, 24%) minmax(0, 1fr) auto; gap: 2px 12px; align-items: center; padding: 5px 4px; border-bottom: var(--filo); font-size: var(--c-sec) }
  .dr.sel { background: var(--velo) }
  .nm { font-weight: 700; line-height: 1.2 }
  .nm small { display: block; font-weight: 400; color: var(--grigio); font-size: var(--c-min) }
  .arr { color: var(--biro); font-weight: 700; white-space: nowrap; margin-left: 8px }
  .ref { grid-column: 1 / -1; display: flex; gap: 8px; align-items: center; flex-wrap: wrap }
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }
  .x { display: flex; gap: 8px; align-items: center; justify-content: flex-end; font-size: var(--c-min); white-space: nowrap }
  .chip { border: 1.5px solid var(--ink); padding: 0 6px; font-weight: 700; line-height: 1.4 }
</style>
