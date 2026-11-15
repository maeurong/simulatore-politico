<script lang="ts">
  // «In aula» (storia 105): una riga per misura (segno, nome, forma, chi la porta, quando si vota; le linee dei partiti, la forbice
  // alla Camera, «in bilico», gli emendamenti aperti, «fiducia»); sotto, gli ultimi voti col conteggio esatto e il motivo della decadenza.
  import { t, numero, elenco, type Chiave } from "$lib/testi/t";
  import { nomeDi } from "$lib/partiti";
  import { inOrdine } from "$lib/partiti/schieramento";
  import type { Vista } from "$lib/tipi/Vista";
  import type { VoceLinea } from "$lib/tipi/VoceLinea";
  import { chiLaPorta, formaDi, quando, ultimiVoti } from "./aula";

  let { vista, scelta, onscelta }: { vista: Vista; scelta: number | null; onscelta: (id: number) => void } = $props();

  const partiti = $derived(inOrdine(vista));
  const camera = $derived(vista.seggi.reduce((s, x) => s + x.camera, 0));
  const nome = (id: string) => nomeDi(vista, id);
  const aperti = (m: { emendamenti: { stato: string }[] }) => m.emendamenti.filter((e) => e.stato === "aperto").length;
  const voti = $derived(ultimiVoti(vista));

  const dice = (linee: VoceLinea[]) => {
    const di = (si: boolean) => elenco(partiti.filter((p) => linee.some((l) => l.partito === p.id && l.si === si)).map((p) => p.nome), ", ") || t("aula.nessuno");
    return t("aula.linee", { si: di(true), no: di(false) });
  };
</script>

<div class="righe" role="group" aria-label={t("aula.in_aula")}>
  {#each vista.calendario as m (m.id)}
    {@const n = aperti(m)}
    <button type="button" class="r" class:sel={scelta === m.id} aria-pressed={scelta === m.id} onclick={() => onscelta(m.id)}>
      <span class="seg" class:urg={m.forbice.in_bilico} aria-hidden="true"></span>
      <span class="t" title={m.nome}><span class="nm">{m.nome}</span> <small>· {formaDi(m)}</small></span>
      <span class="a">
        <span class="lin" role="img" aria-label={dice(m.linee)}>
          {#each partiti as p (p.id)}
            {@const si = m.linee.find((l) => l.partito === p.id)?.si ?? false}
            <i data-tagli-ignora class:si class:g={p.id === "giocatore"} title="{p.nome}: {si ? t('aula.si') : t('aula.no')}"></i>
          {/each}
        </span>
        <span class="fb" role="img" aria-label={t("aula.emiciclo", { ramo: "Camera", da: numero(m.forbice.camera[0]), a: numero(m.forbice.camera[1]), n: numero(m.forbice.maggioranza_camera) })}>
          <i style:left="{(m.forbice.camera[0] / camera) * 100}%" style:width="{Math.max(1, ((m.forbice.camera[1] - m.forbice.camera[0]) / camera) * 100)}%"></i>
          <b style:left="{(m.forbice.maggioranza_camera / camera) * 100}%"></b>
        </span>
        {#if m.forbice.in_bilico}<span class="chip rosso">{t("aula.in_bilico")}</span>{/if}
        {#if n}<span class="chip">{t("aula.emendamenti_aperti", { n })}</span>{/if}
        {#if m.fiducia}<span class="chip pieno">{t("aula.fiducia")}</span>{/if}
      </span>
      <span class="s">{chiLaPorta(vista, m, nome)} · {quando(vista, m.voto)}</span>
    </button>
  {:else}
    <p class="sec muto">{t("aula.niente_in_esame")}</p>
  {/each}
</div>

<h4 class="tit sotto">{t("aula.ultimi_voti")}</h4>
{#each voti as e, i (i)}
  <div class="voce">
    <span class="nv" title={e.nome}><b>{e.nome}</b> <span class="muto">· {formaDi(e)}{e.fiducia_respinta ? ` · ${t("aula.fiducia_respinta")}` : e.fiducia ? ` · ${t("aula.con_la_fiducia")}` : ""}</span></span>
    <span class="chip" class:ok={e.approvata} class:rosso={!e.approvata}>{t(e.decaduta ? "aula.decaduta" : e.approvata ? "aula.approvata" : "aula.respinta")}</span>
    <small>{e.decaduta ?? t("aula.conteggio", { cs: numero(e.camera.si), cn: numero(e.camera.no), ss: numero(e.senato.si), sn: numero(e.senato.no), periodo: t(`cornice.trimestre_${e.data.trimestre}` as Chiave, { anno: e.data.anno }).toLowerCase() })}</small>
  </div>
{:else}
  <p class="sec muto">{t("aula.nessun_voto")}</p>
{/each}

<style>
  .righe { display: grid }
  .r { display: grid; grid-template-columns: 22px minmax(0, 1fr) auto; gap: 2px 10px; align-items: center; padding: 6px; border-bottom: var(--filo); cursor: pointer; text-align: left }
  .r:hover { background: var(--velo) }
  .r.sel { background: var(--velo); box-shadow: inset 0 0 0 1.5px var(--ink) }
  .seg { width: 14px; height: 14px; border: 1.5px solid var(--ink) }
  .seg.urg { border-color: var(--rosso); background: var(--retino) }
  .t { font-weight: 700; line-height: 1.2 }
  .nm { overflow: hidden; display: -webkit-box; line-clamp: 2; -webkit-line-clamp: 2; -webkit-box-orient: vertical }
  .t small { font-weight: 400; color: var(--grigio); font-size: var(--c-sec) }
  .s { grid-column: 2; font-size: var(--c-min); color: var(--grigio); line-height: 1.25 }
  .a { grid-row: 1 / span 2; grid-column: 3; display: flex; gap: 4px; align-items: center; white-space: nowrap }
  .lin { display: inline-flex; gap: 2px }
  .lin i { width: 11px; height: 11px; border: 1.5px solid var(--ink); flex: none }
  .lin i.si { background: var(--ink) }
  .lin i.g { border-color: var(--rosso) }
  .lin i.g.si { background: var(--rosso) }
  .fb { position: relative; display: inline-block; height: 10px; width: 70px; border: 1px solid var(--grigio); background: var(--carta); flex: none }
  .fb i { position: absolute; top: 0; bottom: 0; background: var(--retino); border-left: 2px solid var(--ink); border-right: 2px solid var(--ink) }
  .fb b { position: absolute; top: -4px; bottom: -4px; width: 2px; background: var(--rosso) }
  .chip { display: inline-block; border: 1.5px solid var(--ink); padding: 0 6px; font-size: var(--c-min); font-weight: 700; white-space: nowrap; line-height: 1.4 }
  .chip.rosso { border-color: var(--rosso); color: var(--rosso) }
  .chip.pieno { background: var(--ink); color: var(--carta) }
  .chip.ok { border-color: var(--verde); color: var(--verde) }
  .sotto { font-size: var(--c-riq); margin-top: 12px }
  .voce { display: grid; grid-template-columns: minmax(0, 1fr) auto; gap: 1px 10px; padding: 6px 0; border-top: var(--filo); font-size: var(--c-sec) }
  .nv { overflow: hidden; display: -webkit-box; line-clamp: 2; -webkit-line-clamp: 2; -webkit-box-orient: vertical }
  .voce small { grid-column: 1 / -1; color: var(--grigio); font-size: var(--c-min) }
  .muto { color: var(--grigio) }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance }
</style>
