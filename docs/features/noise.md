# Noise measurement and analog budgets — implemented features

The completed lane provides exactly **eight** distinct calculations: resistor thermal noise, junction shot noise, combination of independently measured RMS contributions, a complete non-inverting amplifier white-noise budget, logarithmic gain conversion, analog detection/dynamic-range budgeting, device equivalent input noise temperature, and a three-stage noise cascade. Levels span basic, intermediate and expert. These are closed-form models executed by the shared Rust expression evaluator; none requires numerical simulation, conditionals or arrays.

## Ownership and conventions

- ADC quantization SNR, SINAD and ENOB remain with the data lane. The dynamic-range calculation here takes an externally determined **analog RMS noise floor** and a sinusoidal swing limit; no ADC resolution appears.
- Amplifier gain synthesis remains with the opamps lane. Noise gain is a necessary intermediate in a six-source noise budget, not a ninth standalone calculator.
- Filter synthesis remains with the filters lane. Bandwidth inputs here are **equivalent noise bandwidths** (ENBW), which users must obtain from the actual measurement/filter response. For example, a single-pole low-pass has ENBW equal to pi/2 times its -3 dB bandwidth [S2, equation 8]. This explanation is not an additional calculator.
- All noise spectra here are **one-sided**, with spectral densities in V/sqrt(Hz) or A/sqrt(Hz). Noise power spectral density is the square of the amplitude spectral density. Multiply a white amplitude density by sqrt(ENBW), not by ENBW, to obtain RMS amplitude.
- Input units are SI, except explicitly identified logarithmic dB. Absolute temperature is K, never Celsius. RMS noise is not peak noise; no universal deterministic peak-to-peak conversion is asserted.
- Constants are the exact SI values **k = 1.380649e-23 J/K** [S5] and **q = 1.602176634e-19 C** [S6]. Older application reports round these constants; examples below deliberately use the modern exact values.
- Numerical limits in JSON prevent unreasonable magnitude/overflow and guard algebraic domains; they are not device ratings. Qualitative physical assumptions such as independence, white spectra, adequate loop gain, matching, and the classical condition h f << k T must be verified by the designer. The inputs cannot independently establish those conditions.

## Primary sources

Source-access evidence is inherited from the research handoff: its author reported the references below successfully retrieved. This implementation did not repeat source retrieval; unavailable Analog Devices pages and gated Keysight PDFs are not used as evidence.

- **[S1] Texas Instruments, SLVA043B, Noise Analysis in Operational Amplifier Circuits.** Shot Noise and Thermal Noise (printed page 2), Adding Noise Sources (printed pages 4–5), and integration/ENBW sections. The older interior pages show the earlier revision suffix. <https://www.ti.com/lit/an/slva043b/slva043b.pdf>
- **[S2] Texas Instruments, Michael Steffes, SBOA066A, Noise Analysis for High-Speed Op Amps, revised January 2005.** Sections 2–4, equations 1–5; section 6, equation 11, and explicit common-downstream-bandlimit assumptions. <https://www.ti.com/lit/an/sboa066a/sboa066a.pdf>
- **[S3] Keysight, Useful Formulas, Conversions, and Reference Information.** Understanding Decibels gives 10 log10 of a power ratio and derives 20 log10 of a voltage ratio only for equal resistance. <https://helpfiles.keysight.com/csg/89600B/Webhelp/Subsystems/gettingstarted/content/concepts_decibels.htm>
- **[S4] Rohde & Schwarz, Mike Leffel and Christopher Stumpf, 1SL378 version 0e, The Cold Source Technique for Noise Figure Measurements, December 2021.** Section 3.1 gives dB/linear relations, reference temperature and equivalent-temperature conversion; sections 3.3–3.4 derive the added-noise model; section 3.6 derives Friis cascade equations 32–40. <https://scdn.rohde-schwarz.com/ur/pws/dl_downloads/dl_application/application_notes/1sl378/1SL378_0e_ColdSrcNF.pdf>
- **[S5] NIST, CODATA Boltzmann constant.** Exact modern SI value. <https://physics.nist.gov/cgi-bin/cuu/Value?k>
- **[S6] NIST, CODATA elementary charge.** Exact modern SI value. <https://physics.nist.gov/cgi-bin/cuu/Value?e>
- **[S7] Texas Instruments, SNOA390B, OA-11 A Tutorial on Applying Op Amps to RF Applications, revised April 2013.** Section 12 Dynamic Range Calculation (printed pages 38–39) defines the minimum detectable signal from the noise floor plus a required detectability margin, and usable dynamic range from that threshold to a maximum permitted signal. Sections 2 and 7 discuss sinusoidal RMS conversion and independently established swing/distortion limits. <https://www.ti.com/lit/an/snoa390b/snoa390b.pdf>

Worked examples are original substitutions computed independently with ordinary scalar arithmetic/Python math, not calculator-engine output or copied manufacturer examples. Every declared output is retained in backend/catalog/noise.json at relative tolerance 1e-6. Defaults equal worked-example inputs. No builds, tests, linters, formatters, smoke commands or engine execution were run by this developer; acceptance checks belong to the integration owner.

## Resistor Johnson noise

**ID:** `noise.johnson_resistor` — **basic**. Sources [S1], [S5]; classical approximation also explained in [S4, equations 14–16].

A resistor has one-sided open-circuit thermal voltage power spectral density

\[
S_v=4kTR\quad[\mathrm{V^2/Hz}].
\]

For a real, frequency-independent resistor at uniform absolute temperature, the white amplitude density is

\[
e_n=\sqrt{4kTR}\quad[\mathrm{V/\sqrt{Hz}}].
\]

Integrating its squared density through a unity-passband response with ENBW B yields

\[
v_{n,\mathrm{rms}}=\sqrt{4kTRB}=e_n\sqrt B\quad[\mathrm V].
\]

**Inputs/domain:** R > 0 ohm, T > 0 K, B > 0 Hz. JSON upper bounds are R <= 1e12 ohm, T <= 2000 K and B <= 1e12 Hz. These are algebraic operating bounds; the model separately requires h f << k T at all frequencies contributing materially to the noise integral. At cryogenic temperature or very high frequency use the quantum expression, not this approximation. Resistor parasitics, bias-dependent excess noise and loading are excluded.

**Important loading limitation:** this is an open-circuit voltage. Across an equal matched load, voltage is halved; available thermal noise power is kTB, not 4kTB [S4, equations 18–19]. Do not square this open-circuit voltage and divide by R to claim delivered matched-load power.

**Independent example:** R = 10000 ohm, T = 300 K, B = 20000 Hz.

- 4kTR = 1.6567788e-16 V²/Hz.
- `voltage_noise_density` = sqrt(1.6567788e-16) = **1.2871591976131002e-8 V/sqrt(Hz)** = 12.8715919761 nV/sqrt(Hz).
- 4kTRB = 3.3135576e-12 V².
- `voltage_noise_rms` = sqrt(3.3135576e-12) = **1.8203179941977172e-6 V** = 1.82031799420 microvolt RMS.


**Error rationale:** Reject nonfinite values, nonpositive resistance/absolute temperature/ENBW, and values outside the catalog operating bounds. These restrictions protect the nonnegative square root and meaningful white-noise integration; they cannot certify the qualitative classical or frequency-flat assumptions. Outputs are evaluated in declared order: density first, RMS second.

## Complete non-inverting amplifier white-noise budget

**ID:** `noise.noninverting_amplifier_budget` — **expert**. Purpose: integrate all six uncorrelated white-noise sources in a high-loop-gain non-inverting stage. Sources [S2, equations 1–5 and 11], [S5].

The plus input sees source resistance R_S. R_F joins output to minus input; R_G joins minus input to ground. All resistors share absolute temperature T. Noise gain G_N = 1 + R_F/R_G. The six output-density contributions are:

| Physical source | Output transfer |
|---|---|
| Amplifier voltage density e_n | G_N |
| Plus-input current density i_plus | R_S G_N |
| Source-resistor voltage density sqrt(4kTR_S) | G_N |
| Minus-input current density i_minus | R_F |
| Feedback-resistor voltage density sqrt(4kTR_F) | 1 |
| Ground-resistor current density sqrt(4kT/R_G) | R_F |

Thus e_o² = (e_n G_N)² + (i_plus R_S G_N)² + 4kTR_S G_N² + (i_minus R_F)² + 4kTR_F + 4kTR_F²/R_G. Combining the last two terms gives 4kTR_F G_N, not an omission of R_G thermal noise. Output RMS v_o = sqrt(e_o²) sqrt(B); input-referred RMS v_in = v_o/G_N.

**Inputs/domain and error rationale:** 0 ≤ R_S ≤ 1e9 ohm; 0 < R_F ≤ 1e9 ohm; 0.001 ≤ R_G ≤ 1e9 ohm. Voltage density e_n is in V/sqrt(Hz); current densities i_plus and i_minus are in A/sqrt(Hz); all are 0 to 0.001. 0 < T ≤ 2000 K and 0 < B ≤ 1e12 Hz. Reject nonfinite/out-of-range values. Positive R_G guards division; G_N ≥ 1 guards input referral; variance terms are nonnegative. R_S = 0 is a valid ideal-source limit; zero amplifier densities are also valid. The operating maxima bound intermediate squares/products and are not hardware ratings.

**Assumptions:** six sources uncorrelated and white, classical thermal approximation, flat resistor values/noise gain, high loop gain and negligible source-specific rolloff in the measurement band. A common downstream filter sets ENBW. Otherwise integrate each actual source-transfer spectrum separately. Excludes flicker/excess noise, output loading noise, EMI and PSRR effects; not stability analysis or gain synthesis.

**Independent worked example:** R_S = 1000 ohm, R_F = 9000 ohm, R_G = 1000 ohm, e_n = 5e-9 V/sqrt(Hz), i_plus = 1e-12 A/sqrt(Hz), i_minus = 2e-12 A/sqrt(Hz), T = 300 K, B = 20000 Hz.

- `noise_gain` = 1 + 9000/1000 = **10 V/V**.
- Six output spectral variances: 2.5e-15, 1.0e-16, 1.6567788e-15, 3.24e-16, 1.49110092e-16 and 1.341990828e-15 V²/Hz.
- Sum = 6.07187972e-15 V²/Hz.
- `output_noise_density` = **7.79222671641425e-8 V/sqrt(Hz)**.
- `output_noise_rms` = sqrt(6.07187972e-15 × 20000) = **1.1019872703439002e-5 V**.
- `input_referred_noise_rms` = output RMS/10 = **1.1019872703439002e-6 V**.

Sequential output order is noise gain, output density, output RMS, input-referred RMS; the independent example tolerance is 1e-6.

