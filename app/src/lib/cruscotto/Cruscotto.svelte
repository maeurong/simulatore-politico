<script lang="ts">
  // Cruscotto, forma A · Ordine del giorno (17-cruscotto.prototype.html): gli eventi in testa; al turno zero le due scelte del voto, al governo i gruppi con il conto al default.
  import { tick, untrack } from "svelte";
  import { t, elenco, numero, type Chiave } from "$lib/testi/t";
  import { apribile, etichetta } from "$lib/ossatura";
  import { chiaveDi } from "$lib/caricature/caricatura";
  import Faccia from "$lib/componenti/Faccia.svelte";
  import type { Vista } from "$lib/tipi/Vista";
  import Crisi from "./Crisi.svelte";
  import DettaglioVoce from "./DettaglioVoce.svelte";
  import Evento from "./Evento.svelte";
  import Riga from "./Riga.svelte";
  import Voce from "./Voce.svelte";
  import { inEvidenza, ordineDelGiorno, righeDelVoto, risponde, testa, type Gruppo } from "./cruscotto";

  let { vista }: { vista: Vista } = $props();
  const voto = $derived(righeDelVoto(vista));
  const { dichiarati, disponibili } = $derived(voto.coalizione);
  const { spese, euro } = $derived(voto.campagna);
  const stato = $derived(
    dichiarati.length
      ? elenco(dichiarati, ", ")
      : `${t("cruscotto.coalizione_nessuna")} · ${t("cruscotto.disponibili", { nomi: disponibili.length ? elenco(disponibili, ", ") : t("cruscotto.nessun_disponibile") })}`,
  );
  const rimando = $derived(`${etichetta("campagna")} ›`);

  const governo = $derived(risponde(vista));
  const odg = $derived(ordineDelGiorno(vista));
  const GRUPPI: [Gruppo, Chiave][] = [
    ["da_decidere", "cruscotto.da_decidere"], ["proposte", "cruscotto.proposte"], ["emendamenti", "cruscotto.emendamenti"],
    ["si_vota", "cruscotto.si_vota"], ["occhio", "cruscotto.occhio"],
  ];
  const gruppi = $derived(GRUPPI.map(([g, titolo]) => ({ titolo, voci: odg.filter((x) => x.gruppo === g) })).filter((g) => g.voci.length));
  const riquadro = $derived(testa(vista));
  const evidenza = $derived(inEvidenza(vista));
  const dicastero = (id: string) => vista.dicasteri.find((d) => d.id === id)?.nome ?? id;
  // la riga scelta e il decreto d'evento scelto (uno solo dei due): solo in memoria, al turno nuovo si azzerano.
  // `turno` è un derived: una Vista nuova dello stesso turno (dopo un comando) non li tocca.
  let scelta = $state<string | null>(null);
  let norma = $state<string | null>(null);
  const turno = $derived(vista.turno);
  $effect.pre(() => {
    void turno;
    untrack(() => {
      scelta = null;
      norma = null;
    });
  });
  const corrente = $derived(odg.find((x) => x.chiave === scelta) ?? odg[0] ?? null);
  const normaViva = $derived(norma && vista.eventi.some((e) => e.norme.includes(norma!)) ? norma : null);
  let corpo: HTMLElement | undefined = $state();
  // se la voce su cui stava il fuoco sparisce con la sua risposta, il fuoco passa al corpo, che non è un'azione: un secondo Invio non ne fa un'altra; altrimenti non si tocca
  async function riporta(da: Element | null) {
    await tick();
    if (!da || da.isConnected) return;
    corpo?.focus({ preventScroll: true });
  }
</script>

{#if governo}
  <div class="cr">
    <div class="odg" bind:this={corpo} tabindex="-1">
      {#if riquadro}<Crisi {vista} tipo={riquadro} {riporta} />{/if}
      {#each vista.eventi as evento, i (i)}
        <Evento {vista} {evento} indice={i} puoRispondere={governo} norma={normaViva} onnorma={(id) => { norma = id; }} />
      {/each}
      {#each gruppi as g (g.titolo)}
        {@const n = g.voci.filter((x) => x.alDefault).length}
        <section>
          <h2 class="grp"><span>{t(g.titolo)}</span>{#if n}<span>{t("cruscotto.al_default", { n })}</span>{/if}</h2>
          <ul>{#each g.voci as voce (voce.chiave)}<Voce {voce} {vista} {riporta} scelta={!normaViva && corrente?.chiave === voce.chiave} onscegli={() => { scelta = voce.chiave; norma = null; }} />{/each}</ul>
        </section>
      {/each}
      {#if !gruppi.length && !vista.eventi.length && !riquadro}<p class="muto">{t("cruscotto.nessuna_scelta")}</p>{/if}
      <section>
        <h2 class="grp"><span>{t("cruscotto.evidenza")}</span>{#if apribile("governo", vista)}<a class="link" href="/partita/governo">{etichetta("governo")} ›</a>{/if}</h2>
        {#if evidenza.length}
          <ul class="minis">
            {#each evidenza as m (m.dicastero)}
              <li>
                <Faccia nome={m.ministro.nome} genere={m.ministro.genere} ruolo={m.ministro.tecnico ? "tecnico" : "ministro"} partito={chiaveDi(vista, m.ministro.partito)} dicastero={m.dicastero} misura={34} />
                <span><b>{dicastero(m.dicastero)}</b> · {m.ministro.nome}<br />
                  <span class="muto">{#if m.preavviso}<span class="male">{t("cruscotto.preavviso", { n: numero(Math.round(m.ministro.soddisfazione)) })}</span>{#if m.agenda}{" · "}{/if}{/if}{m.agenda?.nome ?? ""}</span></span>
              </li>
            {/each}
          </ul>
        {:else}<p class="muto">{t("cruscotto.evidenza_nessuno")}</p>{/if}
      </section>
    </div>
    <aside class="lato" aria-label={t("aula.dettaglio")}>
      {#key `${vista.turno}:${normaViva ?? corrente?.chiave}`}<DettaglioVoce {vista} voce={corrente} norma={normaViva} {riporta} />{/key}
    </aside>
  </div>
{:else}
{#each vista.eventi as evento, i (i)}<Evento {vista} {evento} indice={i} puoRispondere={governo} />{/each}
<section class="odg">
  <h2 class="grp">{t("cruscotto.da_decidere")}</h2>
  <ul>
    <Riga titolo={t("cruscotto.coalizione")} {stato} {rimando} href="/partita/campagna" fatta={dichiarati.length > 0} urgente />
    <Riga
      titolo={t("cruscotto.campagna")}
      stato={spese ? t("cruscotto.spese", { n: spese, euro: numero(euro) }) : t("cruscotto.nessuna_spesa")}
      {rimando} href="/partita/campagna" fatta={spese > 0} />
  </ul>
</section>
{/if}

<style>
  .odg { outline: none }
  div.odg { display: grid; gap: 10px; align-content: start; min-width: 0; min-height: 0; overflow: hidden auto }
  .cr { display: grid; grid-template-columns: minmax(0, 1fr) var(--lato); grid-template-rows: minmax(0, 1fr); gap: 10px var(--canale); height: 100%; min-height: 0 }
  .lato { border-left: var(--forte); padding-left: var(--canale); display: grid; gap: 8px; align-content: start; min-height: 0; overflow: hidden auto; padding-bottom: 8px }
  .minis { display: grid; gap: 6px; padding-top: 6px }
  .minis li { display: flex; gap: 8px; align-items: center; font-size: var(--c-sec) }
  .male { color: var(--rosso); font-weight: 700 }
  .link { color: inherit; text-decoration: underline; text-underline-offset: 2px; font-size: var(--c-sec); text-transform: none; letter-spacing: 0; font-weight: 400 }
  .muto { color: var(--grigio) }
  .grp { display: flex; justify-content: space-between; gap: 8px; font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--grigio); padding-bottom: 2px; border-bottom: 1.5px solid var(--ink) }
  ul { list-style: none }
</style>
