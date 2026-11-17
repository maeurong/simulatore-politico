<script lang="ts">
  // «Le tue leve» (28-opposizione.prototype.html, forma A): una riga per leva col suo stato. Le azioni vere stanno nel dettaglio a lato.
  import { t } from "$lib/testi/t";
  import { nomeDi } from "$lib/partiti";
  import { periodoDi } from "$lib/aula/aula";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import type { Vista } from "$lib/tipi/Vista";
  import RigaOdg from "./RigaOdg.svelte";
  import { leve, type Leva } from "./opposizione";
  import type { Scelta } from "./scelta";

  // passano: su quanti ministri la sfiducia passerebbe, quando le anteprime sono arrivate tutte
  let { vista, scelta, onscegli, passano = null }: { vista: Vista; scelta: Scelta | null; onscegli: (s: Scelta) => void; passano?: { n: number; m: number } | null } = $props();

  const TITOLO = {
    sfiducia_governo: "opposizione.leva_sfiducia_governo", sfiducia_ministro: "opposizione.leva_sfiducia_ministro",
    posto: "opposizione.leva_posto", abrogativo: "opposizione.leva_abrogativo",
  } as const;
  // il confermativo, quando c'è, porta il nome della norma
  const confermativo = $derived(vista.referendum.find((r) => r.tipo === "confermativo" && (r.richiesto || r.scelta)));
  const corso = $derived(vista.referendum.find((r) => r.tipo === "abrogativo"));
  const nome = (id: string | null) => (id ? nomeDi(vista, id) : "");
  const titolo = (l: Leva) => (l.tipo === "confermativo" ? t("opposizione.leva_confermativo", { nome: confermativo?.nome ?? "" }) : t(TITOLO[l.tipo]));

  const SCELTA: Record<Exclude<Leva["tipo"], "posto">, Scelta> = {
    sfiducia_governo: { tipo: "sfiducia", id: null }, sfiducia_ministro: { tipo: "ministro", id: null },
    abrogativo: { tipo: "referendum", id: "abrogativo" }, confermativo: { tipo: "referendum", id: "confermativo" },
  };
  const scelta_di = (l: Leva) => (l.tipo === "posto" ? null : SCELTA[l.tipo]);
  // la riga del ministro resta scelta anche col dicastero aperto nel dettaglio
  const uguale = (s: Scelta | null) => !!s && scelta?.tipo === s.tipo && (s.tipo === "ministro" || scelta.id === s.id);

  function sotto(l: Leva): string {
    if (l.stato === "spenta") return l.motivo ? t(l.motivo) : t("opposizione.posto_nessuno");
    switch (l.tipo) {
      case "sfiducia_governo":
      case "sfiducia_ministro":
        if (l.stato === "altrui") return t("opposizione.sfiducia_altrui", { partito: nome(l.chi) });
        return l.tipo === "sfiducia_ministro" && passano ? `${t("opposizione.sfiducia_sotto")} ${t("opposizione.ministri_passano", passano)}` : t("opposizione.sfiducia_sotto");
      case "posto": {
        const mia = vista.calendario.find((m) => m.promotore === vista.giocatore);
        if (l.stato === "libera") return t("opposizione.posto_libero");
        if (l.stato === "altrui") return t("opposizione.posto_altrui", { partito: nome(l.chi) });
        return mia ? t("opposizione.posto_occupato", { periodo: periodoDi(vista, mia.voto) }) : t("opposizione.posto_nessuno");
      }
      case "abrogativo": {
        if (!corso) return t("opposizione.ref_libero", { anno: vista.data.anno + 1 });
        const firme = t(corso.firme ? "opposizione.ref_firme" : "opposizione.ref_firme_a_fine_turno");
        const base = t("opposizione.ref_in_corso", { nome: corso.nome, firme, periodo: periodoDi(vista, corso.voto) });
        return l.stato === "altrui" ? `${base} · ${t("opposizione.ref_promosso_da", { partito: nome(l.chi) })}` : base;
      }
      case "confermativo":
        return t("opposizione.conf_sotto");
    }
  }
</script>

<section>
  <h2 class="grp"><span>{t("opposizione.leve")}</span></h2>
  <ul>
    {#each leve(vista) as l (l.tipo)}
      {@const s = scelta_di(l)}
      <RigaOdg titolo={titolo(l)} sotto={sotto(l)} segno={l.stato === "presentata" ? "fatto" : ""} scelta={uguale(s)} onscegli={s && l.stato !== "spenta" ? () => onscegli(s) : undefined}>
        {#snippet azioni()}
          {#if l.stato === "presentata" && (l.tipo === "sfiducia_governo" || l.tipo === "sfiducia_ministro")}
            <span class="chip pieno">{t("opposizione.sfiducia_presentata")}</span>
          {:else if l.stato === "presentata" && l.tipo === "abrogativo" && corso}
            <span class="chip">{t(corso.firme ? "opposizione.ref_firme" : "opposizione.ref_promosso")}</span>
          {:else if l.stato === "presentata" && l.tipo === "confermativo"}
            <span class="chip pieno">{t("opposizione.conf_chiesto")}</span>
          {:else if l.tipo === "sfiducia_ministro" && l.stato === "libera" && s}
            <Pulsante forma="link" aria-label="{titolo(l)}: {t('opposizione.scegli_ministro')}" onclick={() => onscegli(s)}>{t("opposizione.scegli_ministro")}</Pulsante>
          {:else if l.tipo === "abrogativo" && l.stato === "libera" && s}
            <Pulsante forma="link" aria-label="{titolo(l)}: {t('opposizione.ref_scegli')}" onclick={() => onscegli(s)}>{t("opposizione.ref_scegli")}</Pulsante>
          {:else if l.tipo === "confermativo" && l.stato === "libera" && s}
            <Pulsante forma="link" aria-label="{titolo(l)}: {t('opposizione.conf_decidi')}" onclick={() => onscegli(s)}>{t("opposizione.conf_decidi")}</Pulsante>
          {:else if l.tipo === "posto" && l.stato === "libera"}
            <a class="link" href="/partita/aula">{t("opposizione.posto_porta")}</a>
          {/if}
        {/snippet}
      </RigaOdg>
    {/each}
  </ul>
</section>

<style>
  .grp { display: flex; justify-content: space-between; gap: 8px; font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--grigio); padding-bottom: 2px; border-bottom: 1.5px solid var(--ink) }
  ul { list-style: none }
  .chip { border: 1.5px solid var(--ink); padding: 0 6px; font-size: var(--c-min); font-weight: 700; white-space: nowrap; line-height: 1.4 }
  .chip.pieno { background: var(--ink); color: var(--carta) }
  .link { color: inherit; text-decoration: underline; text-underline-offset: 2px; font-size: var(--c-sec); white-space: nowrap }
  .link:hover { text-decoration-thickness: 2px }
</style>
