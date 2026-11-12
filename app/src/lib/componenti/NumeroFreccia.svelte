<script module lang="ts">
  type Verso = "sale" | "scende";

  /** La variazione sul turno prima, arrotondata da `formato`: ciò che si legge zero è pari. */
  export function freccia(
    valore: number,
    prima: number | undefined,
    formato: (x: number) => string,
    male: Verso | null | undefined,
    formatoVariazione: (x: number) => string = formato,
  ): { verso: Verso | "pari"; giudizio: "bene" | "male" | null; testo: string } | null {
    if (prima === undefined) return null;
    const testo = formatoVariazione(Math.abs(valore - prima));
    if (!/[1-9]/.test(testo)) return { verso: "pari", giudizio: null, testo: "" };
    const verso = valore > prima ? "sale" : "scende";
    return { verso, giudizio: male ? (verso === male ? "male" : "bene") : null, testo };
  }
</script>

<script lang="ts">
  import { t } from "$lib/testi/t";

  type Props = {
    valore: number;
    prima?: number;
    formato: (x: number) => string;
    /** la variazione, se va scritta diversa dal valore («51 ▾2,7»); di base formato */
    formatoVariazione?: (x: number) => string;
    /** il verso che fa male; null: freccia senza colore */
    male?: Verso | null;
    /** il critico va in negativo: si legge anche senza colore */
    sottoSoglia?: boolean;
  };

  let { valore, prima, formato, formatoVariazione, male = null, sottoSoglia = false }: Props = $props();
  const d = $derived(freccia(valore, prima, formato, male, formatoVariazione));
</script>

<!-- tutto su una riga: uno spazio fra i tag staccherebbe la freccia dal numero -->
<span class="nf"><span class:crit={sottoSoglia && !d}>{formato(valore)}</span>{#if d}<span class={["dl", d.giudizio ?? "muto", { crit: sottoSoglia }]}>{#if d.verso === "pari"}<span aria-hidden="true">=</span><span class="nascosto" data-tagli-ignora>{t("componenti.pari")}</span>{:else}<svg class="fr" class:giu={d.verso === "scende"} viewBox="0 0 10 10" role="img" aria-label={t(d.verso === "sale" ? "componenti.sale" : "componenti.scende")}><path d="M5 .5 9.5 9.5H.5Z" /></svg>{d.testo}{/if}</span>{/if}</span>

<style>
  .nf { white-space: nowrap; font-weight: 700 }
  .fr { display: inline-block; width: .62em; height: .62em; margin-right: .2em; fill: currentColor; vertical-align: .02em }
  .fr.giu { transform: rotate(180deg) }
  .dl { font-size: var(--c-min); font-weight: 700; white-space: nowrap; margin-left: .35em }
  .muto { color: var(--grigio) }
  .bene { color: var(--verde) }
  .male { color: var(--rosso) }
  .crit { background: var(--rosso); color: var(--carta); padding: 0 5px }
</style>
