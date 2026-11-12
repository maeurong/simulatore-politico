<script lang="ts">
  // Pagina di prova dei componenti di base, caricata da /sviluppo/componenti solo in sviluppo:
  // i testi qui sono di prova e non stanno in interfaccia.json.
  import "$lib/stile/fondamenta.css";
  import { numero, t } from "$lib/testi/t";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Conferma from "$lib/componenti/Conferma.svelte";
  import NumeroFreccia from "$lib/componenti/NumeroFreccia.svelte";
  import Dettaglio from "$lib/componenti/Dettaglio.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import Segnaposto from "$lib/componenti/Segnaposto.svelte";
  import Impostazioni from "$lib/cornice/Impostazioni.svelte";

  let grave = $state(false);
  let scelta = $state<"ferma" | "dirotta" | null>(null);
  let confermate = $state(0);
  let rifiuto = $state(false);
  let impostazioni = $state(false);
  let escAlContenitore = $state(0);
  const scegli = (s: "ferma" | "dirotta") => (scelta = scelta === s ? null : s);

  $effect(() => {
    document.documentElement.classList.toggle("grave", grave);
    return () => document.documentElement.classList.remove("grave");
  });

  const inchiostri = [
    ["carta", "Il foglio: l’unico fondo.", "#F4ECD8 · —"],
    ["ink", "Testo, filetti forti, pieni.", "#1F1A17 · 14,6 : 1"],
    ["grigio", "Testo secondario, filetti sottili.", "#5E5750 · 6,0 : 1"],
    ["rosso", "Testata, ciò che va male, il critico.", "#B3122B · 5,9 : 1"],
    ["verde", "Ciò che va bene.", "#2E6B45 · 5,4 : 1"],
    ["biro", "Note a biro, motti, rimandi allo schema.", "#1D4E89 · 7,1 : 1"],
    ["giallo", "Evidenziatore: solo sotto l’inchiostro, mai testo né filetto.", "#FFD23F · 1,2 : 1"],
  ];
  const filetti = [
    ["border-top:var(--filo)", "1 px grigio · fra le righe"],
    ["border-top:var(--bordo)", "1,5 px · riquadri e caselle"],
    ["border-top:var(--forte)", "2 px · pulsanti"],
    ["border-top:var(--doppio)", "doppio · testa e piede di pagina"],
    ["height:12px;background:var(--retino);border:var(--bordo)", "retino · zona di pericolo, vignette"],
    ["height:12px;background:var(--velo)", "velo · riga scelta, passaggio del puntatore"],
    ["height:12px;border:1.5px dashed var(--grigio)", "tratteggio · spento o non ancora valido"],
  ];
  const uno = (x: number) => numero(x, 1);
</script>

<div class="camp">
  <section>
    <h3 class="tit">Inchiostri</h3>
    {#each inchiostri as [v, ruolo, valori] (v)}
      <div class="ink"><i style="background:var(--{v})"></i><b>{v}</b><code>{valori}</code><small>{ruolo}</small></div>
    {/each}
    <p class="min muto nota">Contrasto sulla carta. Nel tono grave rosso, verde e biro diventano inchiostro e il giallo sparisce.</p>
    <p class="nota"><Pulsante forma="piccolo" scelto={grave} onclick={() => (grave = !grave)}>Tono grave</Pulsante></p>
    <h4 class="tit">Filetti e campiture</h4>
    <div class="fil">
      {#each filetti as [stile, nome] (nome)}<i style={stile}></i><span>{nome}</span>{/each}
    </div>
  </section>

  <section>
    <h3 class="tit">Caratteri</h3>
    {#if !grave}<div class="car uno"><code>72 · testata</code><span class="nomino">{t("cornice.nomino")}</span></div>{/if}
    <div class="car"><code>44 · apertura</code><span><span class="tit" style="font-size:var(--c-apre)">Piove sul Po e sui conti</span></span></div>
    <div class="car"><code>30 · pagina</code><span><span class="tit" style="font-size:var(--c-pag)">Come funziona lo Stato</span></span></div>
    <div class="car"><code>23 · sezione</code><span><span class="tit" style="font-size:var(--c-sez)">Il borsino dei ministri</span></span></div>
    <div class="car"><code>18 · riquadro</code><span><span class="tit" style="font-size:var(--c-riq)">Emergenza idrogeologica</span></span></div>
    <div class="car"><code>19 · numero</code><b style="font-size:var(--c-num)">56,5% · +24 / +15 · 1.000</b></div>
    <div class="car"><code>16 · attacco</code><span style="font-size:var(--c-lead)">Il fondo di riserva copre un miliardo.</span></div>
    <div class="car"><code>14 · corpo</code><span>Formazione pagata dalle imprese</span></div>
    <div class="car"><code>13 · secondo</code><span class="sec muto">Simone Ricci · Centrodestra</span></div>
    <div class="car"><code>12 · minimo</code><span class="min muto">vicino alla sua posizione</span></div>
    <div class="car"><code>12 · occhiello</code><span class="occ">Emergenza · Marche</span></div>
    {#if !grave}<div class="car"><code>13 · motto</code><span class="sec motto">«Meno sigle, più primari contenti.»</span></div>{/if}
  </section>

  <section>
    <h3 class="tit">Componenti</h3>
    <h4 class="tit primo">Numero con freccia</h4>
    <div class="fila grande">
      <NumeroFreccia valore={71} prima={61} formato={numero} male="scende" />
      <NumeroFreccia valore={51} prima={53.7} formato={numero} formatoVariazione={uno} male="scende" />
      <NumeroFreccia valore={33} prima={45.2} formato={numero} formatoVariazione={uno} male="scende" sottoSoglia />
      <NumeroFreccia valore={60} prima={60} formato={numero} male="scende" />
      <NumeroFreccia valore={86} prima={83} formato={(x) => `${numero(x)} pb`} male="sale" />
      <NumeroFreccia valore={24} prima={20} formato={numero} />
      <NumeroFreccia valore={38} formato={numero} sottoSoglia />
    </div>
    <p class="min muto">La freccia dice il verso, il colore il giudizio (lo spread che sale è rosso); senza giudizio resta grigia. Sotto soglia la variazione va in negativo.</p>

    <h4 class="tit">Pulsante d’azione</h4>
    <div class="fila">
      <Pulsante forma="chiude">{t("cornice.chiudi_il_turno")}</Pulsante>
      <Pulsante forma="bordo">Presenta</Pulsante>
      <Pulsante forma="bordo" scelto={scelta === "ferma"} onclick={() => scegli("ferma")}>{t("componenti.ferma")}</Pulsante>
      <Pulsante forma="bordo" scelto={scelta === "dirotta"} onclick={() => scegli("dirotta")}>{t("componenti.dirotta")}</Pulsante>
      <Pulsante forma="piccolo">{t("componenti.ferma")}</Pulsante>
      <Pulsante forma="piccolo" scelto>Scelto</Pulsante>
    </div>
    <div class="fila">
      <Pulsante forma="bordo" spento="Camere sciolte" onclick={() => (rifiuto = true)}>Poni la fiducia</Pulsante>
      <Pulsante forma="piccolo" spento="niente da fermare: margine esaurito">{t("componenti.ferma")}</Pulsante>
    </div>
    <div class="fila">
      <Pulsante forma="link">Altre 14 notizie nel registro ›</Pulsante>
      <Pulsante forma="link" spento="Turno zero: nessun turno passato">Registro</Pulsante>
    </div>

    <h4 class="tit">Conferma sul posto</h4>
    <div class="fila">
      <Conferma etichetta={t("componenti.chiedi_le_dimissioni")} conferma={t("componenti.conferma_le_dimissioni")} onconferma={() => confermate++} />
    </div>
    <div class="fila">
      <Conferma etichetta="Torna al turno 1" conferma="Conferma: cancella 1 turno" spiegazione="Il turno 2 si perde." onconferma={() => confermate++} />
    </div>
    <!-- contenitore che chiude con Esc, come il pannello delle impostazioni: l'Esc della conferma non deve arrivarci -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="fila contenitore" onkeydown={(e) => { if (e.key === "Escape") escAlContenitore++; }}>
      <Conferma etichetta="Cancella la bozza" conferma="Conferma: cancella la bozza" onconferma={() => confermate++} />
      <span class="min muto">Esc arrivati al contenitore: {escAlContenitore}</span>
    </div>
    <p class="min muto">Confermate: {confermate}. Tab, Invio, Esc.</p>

    <h4 class="tit">Rifiuto</h4>
    <div class="fila">
      <Pulsante forma="bordo" onclick={() => (rifiuto = !rifiuto)}>Presenta il decreto</Pulsante>
      {#if rifiuto}<Rifiuto testo="Il decreto supera il fondo di riserva di 1,2 miliardi." />{/if}
    </div>

    <h4 class="tit">Segnaposto</h4>
    <Segnaposto sezione={t("sommario.paese")} fetta="11c" />
  </section>
</div>

<div class="lato">
  <div class="corpo">
    <Segnaposto sezione={t("sommario.governo")} fetta="11b" />
    <p class="nota"><Pulsante forma="link" scelto={impostazioni} onclick={() => (impostazioni = !impostazioni)}>{t("impostazioni.titolo")}</Pulsante></p>
  </div>
  {#if impostazioni}
  <Impostazioni onchiudi={() => (impostazioni = false)} />
  {:else}
  <Dettaglio aria-label="Dettaglio di prova">
    <div class="occ">Sanità</div>
    <h4 class="tit" style="font-size:var(--c-sez)">Sara Ferraro</h4>
    <p class="sec muto">Destra</p>
    {#each { length: 12 }, i (i)}
      <p class="sec">Riga {i + 1} del riquadro a lato: largo var(--lato), scorre in verticale.</p>
    {/each}
    <div class="fila"><Pulsante forma="bordo">{t("componenti.ferma")}</Pulsante><Pulsante forma="bordo">{t("componenti.dirotta")}</Pulsante></div>
  </Dettaglio>
  {/if}
</div>

<style>
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance }
  :global(:root.grave) .tit { font-weight: 600 }
  .occ { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--rosso) }
  .muto { color: var(--grigio) }
  .sec { font-size: var(--c-sec) }
  .min { font-size: var(--c-min) }
  .motto { font-style: italic; color: var(--biro) }
  .nomino { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 800; color: var(--rosso); font-size: var(--c-test); line-height: 1.05; white-space: nowrap }

  .camp { display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); gap: 10px var(--canale); padding: var(--s3) var(--marg) 14px; align-content: start }
  .camp > * { min-width: 0 }
  h3 { font-size: var(--c-sez); border-bottom: var(--doppio); padding-bottom: 3px; margin-bottom: var(--s2) }
  h4 { font-size: var(--c-riq); margin: var(--s3) 0 5px }
  h4.primo { margin-top: 0 }
  .nota { margin-top: 6px }

  .ink { display: grid; grid-template-columns: 26px 1fr auto; gap: 2px 10px; align-items: center; padding: var(--s1) 0; border-top: var(--filo); font-size: var(--c-sec) }
  .ink i { width: 26px; height: 26px; border: var(--bordo); grid-row: span 2 }
  .ink small { grid-column: 2/-1; color: var(--grigio); font-size: var(--c-min) }
  code { font: inherit; font-size: var(--c-min); color: var(--grigio) }
  .car { display: grid; grid-template-columns: 54px minmax(0, 1fr); gap: 0 10px; align-items: baseline; padding: 3px 0; border-top: var(--filo) }
  .car.uno { grid-template-columns: 1fr }
  .car > span:last-child { overflow: hidden; white-space: nowrap; text-overflow: ellipsis }
  .fil { display: grid; grid-template-columns: 70px 1fr; gap: 6px 10px; align-items: center; font-size: var(--c-min); color: var(--grigio) }
  .fil i { display: block; height: 0 }

  .fila { display: flex; gap: 6px; margin-top: var(--s2); flex-wrap: wrap; align-items: center }
  .grande { gap: 18px; font-size: 17px; margin-top: 0 }
  .contenitore { border: 1.5px dashed var(--grigio); padding: var(--s1) var(--s2) }

  /* flex, non grid: il 30% di --lato si misura sulla riga intera, come nella sezione del prototipo */
  .lato { display: flex; gap: var(--canale); padding: 0 var(--marg) var(--marg); height: 640px }
  .corpo { flex: 1; min-width: 0 }
</style>
