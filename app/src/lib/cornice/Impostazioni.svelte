<script lang="ts">
  import { numero, t } from "$lib/testi/t";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import { GRADINI, PARTENZA, fattori, impostazioni, scalaApplicata, scegli, sistema, volte } from "$lib/impostazioni.svelte";

  let { onchiudi }: { onchiudi: () => void } = $props();
  const id = $props.id();

  const tetto = $derived(scalaApplicata(2, sistema.larghezza));
  const applicata = $derived(scalaApplicata(impostazioni.scala, sistema.larghezza));
  const [primaApplicata, dopoApplicata] = $derived(
    t("impostazioni.scala_applicata", { scala: volte(impostazioni.scala) }).split("{applicata}"),
  );

  const velocita = [
    ["1", volte(1)],
    ["2", volte(2)],
    ["istantaneo", t("impostazioni.istantaneo")],
  ] as const;
  const movimento = [
    ["sistema", t("impostazioni.come_il_sistema")],
    ["acceso", t("impostazioni.acceso")],
    ["spento", t("impostazioni.spento")],
  ] as const;

  function tasto(e: KeyboardEvent) {
    if (e.key === "Escape" && !e.defaultPrevented) onchiudi();
  }

  let carta: HTMLDivElement;
  let piene = $state(0);
  let esito: HTMLUListElement;
  let attese: ReturnType<typeof setTimeout>[] = [];

  function prova() {
    const { k, muove } = fattori(impostazioni, sistema.riduce);
    attese.forEach(clearTimeout);
    piene = 0;
    if (muove) {
      carta.animate(
        [{ transform: "rotateY(0)" }, { transform: "rotateY(180deg)" }, { transform: "rotateY(180deg)", offset: 0.7 }, { transform: "rotateY(0)" }],
        { duration: 1800 * k, easing: getComputedStyle(carta).getPropertyValue("--e-work") },
      );
    } else {
      carta.animate([{ opacity: 1 }, { opacity: 0 }, { opacity: 1 }], { duration: 600 * k });
    }
    attese = Array.from({ length: 10 }, (_, i) => setTimeout(() => (piene = i + 1), (muove ? 90 : 0) * i * k));
    [...esito.children].forEach((r, i) =>
      r.animate(
        muove ? [{ opacity: 0, transform: "translateY(6px)" }, { opacity: 1, transform: "none" }] : [{ opacity: 0 }, { opacity: 1 }],
        { duration: 260 * k, delay: 350 * i * k, fill: "backwards" },
      ),
    );
  }
</script>

<svelte:window onkeydown={tasto} />

<aside class="lato" aria-labelledby="{id}-titolo" tabindex="-1" {@attach (el) => el.focus()}>
  <div class="capo">
    <h4 id="{id}-titolo" class="tit">{t("impostazioni.titolo")}</h4>
    <Pulsante forma="link" onclick={onchiudi}>{t("impostazioni.chiudi")}</Pulsante>
  </div>

  <div class="riga" role="group" aria-labelledby="{id}-scala">
    <span id="{id}-scala" class="k">{t("impostazioni.scala")}</span>
    <div class="scelte">
      {#each GRADINI as g (g)}
        <span class="gradino" class:fuori={g > tetto}>
          <Pulsante forma="piccolo" scelto={g === impostazioni.scala} onclick={() => scegli({ scala: g })}
            title={g > tetto ? t("impostazioni.gradino_non_entra", { larghezza: numero(sistema.larghezza), scala: volte(tetto) }) : undefined}
            >{volte(g)}</Pulsante>
        </span>
      {/each}
    </div>
    <p class="nota" class:avv={applicata < impostazioni.scala} aria-live="polite">
      {#if applicata < impostazioni.scala}{primaApplicata}<b>{volte(applicata)}</b>{dopoApplicata}{:else}{t("impostazioni.nota_scala")}{/if}
    </p>
  </div>

  <div class="riga" role="group" aria-labelledby="{id}-velocita">
    <span id="{id}-velocita" class="k">{t("impostazioni.velocita")}</span>
    <div class="scelte">
      {#each velocita as [v, etichetta] (v)}
        <Pulsante forma="piccolo" scelto={impostazioni.velocita === v} onclick={() => scegli({ velocita: v })}>{etichetta}</Pulsante>
      {/each}
    </div>
    <p class="nota">{t("impostazioni.nota_velocita")}</p>
  </div>

  <div class="riga" role="group" aria-labelledby="{id}-movimento">
    <span id="{id}-movimento" class="k">{t("impostazioni.movimento")}</span>
    <div class="scelte">
      {#each movimento as [v, etichetta] (v)}
        <Pulsante forma="piccolo" scelto={impostazioni.movimento === v} onclick={() => scegli({ movimento: v })}>{etichetta}</Pulsante>
      {/each}
    </div>
    <p class="nota">
      {#if impostazioni.movimento === "sistema"}{t(sistema.riduce ? "impostazioni.sistema_riduce" : "impostazioni.sistema_non_riduce")}.{/if}
      {t("impostazioni.nota_movimento_spento")}
    </p>
  </div>

  <div class="riga">
    <span><Pulsante forma="link" onclick={() => scegli(PARTENZA)}>{t("impostazioni.torna_ai_valori")}</Pulsante></span>
    <p class="nota">{t("impostazioni.nota_valori")}</p>
  </div>

  <div class="prova">
    <div class="carta">
      <div class="in" bind:this={carta}>
        <div class="faccia">{t("impostazioni.prova_ruolo")}<br /><b>{t("impostazioni.prova_nome")}</b></div>
        <div class="faccia retro" aria-hidden="true">{t("impostazioni.prova_motto")}</div>
      </div>
    </div>
    <div class="caselle" aria-hidden="true">
      {#each { length: 10 }, i (i)}<i class:piena={i < piene}></i>{/each}
    </div>
    <ul class="esito" bind:this={esito}>
      {#each ["impostazioni.prova_esito_1", "impostazioni.prova_esito_2", "impostazioni.prova_esito_3"] as const as chiave (chiave)}
        <li>{t(chiave)}</li>
      {/each}
    </ul>
    <Pulsante forma="bordo" onclick={prova}>{t("impostazioni.prova_pulsante")}</Pulsante>
  </div>
</aside>

<style>
  /* il lato del prototipo 33: un filetto forte a sinistra, la pagina sotto resta in vista */
  .lato { min-height: 0; overflow: hidden auto; border-left: var(--forte); padding-left: var(--canale); display: grid; gap: 10px; align-content: start }
  /* tabindex="-1": prende il fuoco all'apertura, ma il contorno visibile resta sui comandi */
  .lato:focus { outline: none }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance; font-size: var(--c-riq) }
  :global(:root.grave) .tit { font-weight: 600 }
  .capo { display: flex; align-items: baseline; justify-content: space-between; gap: 10px; border-bottom: var(--doppio); padding-bottom: 3px; margin-bottom: 2px }

  .riga { display: grid; gap: 5px; padding: var(--s2) 0; border-top: var(--filo) }
  .capo + .riga { border-top: 0 }
  .k { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; color: var(--grigio); font-weight: 700 }
  .scelte { display: flex; gap: var(--s1); flex-wrap: wrap }
  .nota { font-size: var(--c-sec); color: var(--grigio); max-width: 58ch }
  .nota b { color: var(--ink) }
  .nota.avv, .nota.avv b { color: var(--rosso) }

  /* oltre il tetto: tratteggiato ma si sceglie (non è aria-disabled) */
  .gradino { display: contents }
  .fuori :global(.btn) { border-style: dashed }
  .fuori :global(.btn:not(.scelto, :hover)) { border-color: var(--grigio); color: var(--grigio) }

  .prova { display: grid; gap: 10px; justify-items: start; padding-top: var(--s2); border-top: var(--filo) }
  .carta { width: 150px; height: 96px; perspective: 600px }
  .in { position: relative; width: 100%; height: 100%; transform-style: preserve-3d }
  .faccia { position: absolute; inset: 0; border: var(--bordo); display: grid; place-items: center; backface-visibility: hidden; padding: var(--s2); text-align: center; font-size: var(--c-sec); background: var(--carta) }
  .retro { transform: rotateY(180deg); background: var(--ink); color: var(--carta) }
  .caselle { display: flex; gap: 2px }
  .caselle i { width: 9px; height: 14px; border: var(--bordo); flex: none }
  .caselle i.piena { background: var(--ink) }
  .esito { font-size: var(--c-sec); list-style: none; justify-self: stretch }
  .esito li { padding: 3px 0; border-top: var(--filo) }
</style>
