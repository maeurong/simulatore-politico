<script lang="ts">
  // Sfiducia al governo o a un ministro nel dettaglio a lato (28-opposizione.prototype.html, stati `sfiducia` e `passa`): la linea di ogni partito,
  // l'esito se nulla cambia, le firme a parole, cosa costa se è respinta, «Presenta la sfiducia» con la conferma sul posto.
  import { t } from "$lib/testi/t";
  import { nomeDi } from "$lib/partiti";
  import { partita } from "$lib/partita.svelte";
  import Conferma from "$lib/componenti/Conferma.svelte";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import type { AnteprimaSfiducia } from "$lib/tipi/AnteprimaSfiducia";
  import type { Comando } from "$lib/tipi/Comando";
  import type { Vista } from "$lib/tipi/Vista";
  import { conteggio } from "./sfiducia";

  // anteprima: undefined = non ancora arrivata, null = guasto o rifiuto
  let { vista, dicastero, anteprima, manda, rifiuto }: {
    vista: Vista; dicastero: string | null; anteprima: AnteprimaSfiducia | null | undefined; manda: (c: Comando) => Promise<boolean>; rifiuto: string | null;
  } = $props();

  const occupato = $derived(partita.occupato ? t("campagna.comando_in_corso") : undefined);
  const ministro = $derived(vista.ministeri.find((m) => m.dicastero === dicastero)?.ministro ?? null);
  const seggi = (id: string, ramo: "camera" | "senato") => vista.seggi.find((s) => s.partito === id)?.[ramo] ?? 0;
  const partiti = $derived([...vista.partiti.filter((p) => p.al_governo), ...vista.partiti.filter((p) => !p.al_governo)].filter((p) => seggi(p.id, "camera") + seggi(p.id, "senato") > 0));
  const voto = (id: string) => anteprima?.linee.find((l) => l.partito === id)?.si;

  // una sfiducia a turno: se c'è, o è questa, o ne occupa il posto
  const presentata = $derived(vista.sfiducia);
  const questa = $derived(presentata !== null && presentata.dicastero === dicastero);
  const motivo = $derived.by(() => {
    if (presentata && !questa) return t("opposizione.sfiducia_gia_presentata");
    if (questa && presentata!.partito !== vista.giocatore) return t("opposizione.sfiducia_altrui", { partito: nomeDi(vista, presentata!.partito) });
    if (anteprima && !anteprima.firme) return t("opposizione.sfiducia_firme_no");
    return occupato;
  });
</script>

<div class="sf">
  {#if ministro}<p class="sec"><b>{ministro.nome}</b> <span class="muto">· {nomeDi(vista, ministro.partito)}</span></p>{/if}
  <dl class="dati">
    <dt>{t("opposizione.sfiducia_firme")}</dt>
    <dd>{#if anteprima}{t(anteprima.firme ? "opposizione.sfiducia_firme_si" : "opposizione.sfiducia_firme_no")}{:else}{t(anteprima === null ? "opposizione.sfiducia_senza_anteprima" : "opposizione.sfiducia_in_arrivo")}{/if}</dd>
    <dt>{t("opposizione.sfiducia_se_nulla")}</dt>
    <dd>{#if anteprima}<b class:male={anteprima.passa}>{t(anteprima.passa ? "opposizione.sfiducia_passa" : "opposizione.sfiducia_non_passa")}</b>{:else}<span class="muto">{t(anteprima === null ? "opposizione.sfiducia_senza_anteprima" : "opposizione.sfiducia_in_arrivo")}</span>{/if}</dd>
    <dt>{t("opposizione.sfiducia_respinta")}</dt>
    <dd>{t("opposizione.sfiducia_respinta_costo")}</dd>
  </dl>

  {#if anteprima}
    {@const camera = conteggio(vista, anteprima, "camera")}
    {@const senato = conteggio(vista, anteprima, "senato")}
    <table class="tab">
      <thead><tr><th>{t("opposizione.sfiducia_partito")}</th><th class="n">{t("opposizione.sfiducia_camera")}</th><th class="n">{t("opposizione.sfiducia_senato")}</th><th>{t("opposizione.sfiducia_voto")}</th></tr></thead>
      <tbody>
        {#each partiti as p (p.id)}
          <tr class:gov={p.al_governo}>
            <td>{p.nome}{#if p.id === vista.giocatore}{" "}<span class="muto">· {t("opposizione.sfiducia_tu")}</span>{/if}</td>
            <td class="n">{seggi(p.id, "camera")}</td><td class="n">{seggi(p.id, "senato")}</td>
            <td>{#if voto(p.id)}<b>{t("aula.si")}</b>{:else}<span class="muto">{t("aula.no")}</span>{/if}</td>
          </tr>
        {/each}
        <tr class="tot"><td>{t("opposizione.sfiducia_si_servono")}</td><td class="n">{camera.si} · {camera.serve}</td><td class="n">{senato.si} · {senato.serve}</td><td>{t(anteprima.passa ? "opposizione.sfiducia_passa" : "opposizione.sfiducia_non_passa")}</td></tr>
      </tbody>
    </table>
  {/if}

  <div class="az">
    {#if questa && presentata?.partito === vista.giocatore}
      <span class="chip">{t("opposizione.sfiducia_presentata")}</span>
    {:else if motivo}
      <Pulsante forma="bordo" spento={motivo}>{t("opposizione.sfiducia_presenta")}</Pulsante>
    {:else}
      <Conferma etichetta={t("opposizione.sfiducia_presenta")} conferma={t("opposizione.sfiducia_presenta_conferma")} onconferma={() => manda({ tipo: "sfiducia", dicastero })} />
    {/if}
    {#if rifiuto}<Rifiuto testo={rifiuto} />{/if}
  </div>
  <p class="min muto">{t("opposizione.sfiducia_sotto")}</p>
</div>

<style>
  .sf { display: grid; gap: 8px }
  .sec { font-size: var(--c-sec) }
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }
  .male { color: var(--rosso) }
  .dati { display: grid; grid-template-columns: auto 1fr; gap: 3px 12px; font-size: var(--c-sec); align-items: baseline }
  .dati dt { color: var(--grigio) }
  .tab { width: 100%; border-collapse: collapse; font-size: var(--c-sec) }
  .tab th { font-size: var(--c-min); letter-spacing: .06em; text-transform: uppercase; color: var(--grigio); text-align: left; font-weight: 700; padding: 3px 4px; border-bottom: 1.5px solid var(--ink) }
  .tab td { padding: 3px 4px; border-bottom: var(--filo) }
  .tab .n { text-align: right; font-variant-numeric: tabular-nums }
  .tab tr.gov td:first-child { box-shadow: inset 3px 0 0 var(--ink) }
  .tab tr.tot td { border-top: 1.5px solid var(--ink); border-bottom: 0; font-weight: 700 }
  .az { display: flex; gap: 6px 10px; align-items: center; flex-wrap: wrap }
  .chip { border: 1.5px solid var(--ink); background: var(--ink); color: var(--carta); padding: 0 6px; font-size: var(--c-min); font-weight: 700; line-height: 1.4 }
</style>
