<script lang="ts">
  // Voce dell'ordine del giorno (17-cruscotto.prototype.html, forma A): segno, titolo, sotto col default, azioni a destra.
  // Il rifiuto del core sta sotto la riga, legato alla Vista in cui è nato.
  import { t, elenco } from "$lib/testi/t";
  import { apribile, etichetta } from "$lib/ossatura";
  import { nomeDi } from "$lib/partiti";
  import { partita } from "$lib/partita.svelte";
  import { comeErrore, testoRifiuto } from "$lib/guscio/chiamate";
  import { fiduciaPonibile } from "$lib/aula/aula";
  import type { Comando } from "$lib/tipi/Comando";
  import type { Vista } from "$lib/tipi/Vista";
  import Conferma from "$lib/componenti/Conferma.svelte";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import type { VoceOdg } from "./cruscotto";

  type Props = { voce: VoceOdg; vista: Vista; riporta: (da: Element | null) => void; scelta: boolean; onscegli: () => void };
  let { voce, vista, riporta, scelta, onscegli }: Props = $props();

  const occupato = $derived(partita.occupato ? t("campagna.comando_in_corso") : undefined);
  const ministero = $derived(vista.ministeri.find((m) => m.dicastero === voce.dicastero));
  const rinviata = $derived(ministero?.proposta?.rinviata ?? false);
  const misura = $derived(vista.calendario.find((m) => m.id === voce.misura));
  const alleato = $derived(voce.partito ? nomeDi(vista, voce.partito) : "");
  const contro = $derived(
    misura?.alleati_contro_fiducia.length ? t("aula.fiducia_no", { nomi: elenco(misura.alleati_contro_fiducia.map((id) => nomeDi(vista, id)), ", ") }) : t("aula.fiducia_nessuno_no"),
  );

  // raw: la Vista dentro si confronta per identità
  let rifiuto = $state.raw<{ testo: string; vista: Vista | null } | null>(null);
  async function manda(comando: Comando) {
    if (partita.occupato) return;
    rifiuto = null;
    const da = document.activeElement;
    try {
      await partita.comanda(comando);
    } catch (e) {
      rifiuto = { testo: testoRifiuto(comeErrore(e)), vista: partita.vista };
    }
    riporta(da);
  }
</script>

<li class="r" class:sel={scelta}>
  <span class="seg" class:fatto={voce.fatta} class:urg={voce.urgente && !voce.fatta} aria-hidden="true"></span>
  <button type="button" class="t" aria-pressed={scelta} onclick={onscegli}>{voce.titolo}</button>
  <span class="a" role="group" aria-label={voce.titolo}>
    {#if voce.azioni === "proposta" && voce.dicastero}
      {@const dicastero = voce.dicastero}
      <Pulsante forma="piccolo" spento={occupato} aria-label={t("cruscotto.accogli_di", { nome: voce.titolo })} onclick={() => manda({ tipo: "accogli", dicastero })}>{t("aula.accogli")}</Pulsante>
      <Pulsante forma="piccolo" spento={rinviata ? t("cruscotto.gia_rinviata") : occupato} aria-label={t("cruscotto.rinvia_di", { nome: voce.titolo })} onclick={() => manda({ tipo: "rinvia", dicastero })}>{t("cruscotto.rinvia")}</Pulsante>
      <Pulsante forma="piccolo" spento={occupato} aria-label={t("cruscotto.respingi_di", { nome: voce.titolo })} onclick={() => manda({ tipo: "respingi", dicastero })}>{t("aula.respingi")}</Pulsante>
    {:else if voce.azioni === "emendamento" && voce.misura !== undefined && voce.partito}
      {@const { misura: id, partito } = voce}
      <Pulsante forma="piccolo" spento={occupato} aria-label={t("aula.accogli_di", { nome: alleato })} onclick={() => manda({ tipo: "emendamento", misura: id, partito, accogli: true })}>{t("aula.accogli")}</Pulsante>
      <Pulsante forma="piccolo" spento={occupato} aria-label={t("aula.respingi_di", { nome: alleato })} onclick={() => manda({ tipo: "emendamento", misura: id, partito, accogli: false })}>{t("aula.respingi")}</Pulsante>
    {:else if voce.azioni === "successore" && ministero}
      {@const dicastero = ministero.dicastero}
      {#each ministero.rosa as x, i (x.nome)}
        <Pulsante forma="piccolo" scelto={ministero.scelta === i} spento={occupato}
          aria-label={t("cruscotto.successore_di", { nome: x.nome, dicastero: vista.dicasteri.find((d) => d.id === dicastero)?.nome ?? dicastero })}
          onclick={() => manda({ tipo: "scegli_successore", dicastero, indice: i })}>{x.nome}{#if i === 0 && ministero.scelta === null} <small>{t("evento.default")}</small>{/if}</Pulsante>
      {/each}
    {:else if voce.azioni === "fiducia" && misura}
      {#if voce.urgente}<span class="chip rosso">{t("aula.in_bilico")}</span>{/if}
      {#if misura.fiducia}
        <span class="chip pieno">{t("aula.fiducia_posta")}</span>
      {:else if fiduciaPonibile(vista, misura)}
        <Conferma etichetta={t("aula.poni_fiducia")} conferma={t("aula.poni_fiducia_conferma")} spiegazione={contro} spento={occupato}
          onconferma={() => manda({ tipo: "poni_fiducia", misura: misura.id })} />
      {/if}
    {:else if typeof voce.azioni === "object" && apribile(voce.azioni.rimando, vista)}
      {@const nome = etichetta(voce.azioni.rimando)}
      <a class="link" href="/partita/{voce.azioni.rimando}" aria-label="{nome}: {voce.titolo}">{nome} ›</a>
    {/if}
  </span>
  <span class="s">{voce.sotto}{#if voce.default}{" · "}<b>{t("cruscotto.default")}</b> {voce.default}{/if}</span>
  {#if rifiuto?.vista === vista}<div class="rif"><Rifiuto testo={rifiuto.testo} /></div>{/if}
</li>

<style>
  .r { display: grid; grid-template-columns: 22px minmax(0, 1fr) auto; gap: 2px 10px; align-items: center; padding: 6px; border-bottom: var(--filo) }
  .seg { width: 14px; height: 14px; border: 1.5px solid var(--ink) }
  .seg.fatto { background: var(--ink) }
  .seg.urg { border-color: var(--rosso); background: var(--retino) }
  .r.sel { background: var(--velo); box-shadow: inset 0 0 0 1.5px var(--ink) }
  .t { font-weight: 700; line-height: 1.2; text-align: left }
  .t:hover { text-decoration: underline; text-underline-offset: 2px }
  .a { grid-row: 1 / span 2; grid-column: 3; display: flex; gap: 4px 6px; align-items: center; justify-content: flex-end; flex-wrap: wrap }
  .a :global(.btn small) { font-weight: 400; margin-left: 4px }
  .s { grid-column: 2; font-size: var(--c-min); color: var(--grigio); line-height: 1.25 }
  .s b { font-weight: 700 }
  .rif { grid-column: 2 / -1 }
  .link { color: inherit; text-decoration: underline; text-underline-offset: 2px; font-size: var(--c-sec); white-space: nowrap }
  .link:hover { text-decoration-thickness: 2px }
  .chip { border: 1.5px solid var(--ink); padding: 0 6px; font-size: var(--c-min); font-weight: 700; white-space: nowrap; line-height: 1.4 }
  .chip.rosso { border-color: var(--rosso); color: var(--rosso) }
  .chip.pieno { background: var(--ink); color: var(--carta) }
</style>
