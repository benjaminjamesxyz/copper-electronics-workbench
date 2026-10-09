# Noise measurement and analog budgets — researched definitions

This lane contains exactly **eight** distinct calculations: resistor thermal noise, junction shot noise, combination of independently measured RMS contributions, a complete non-inverting amplifier white-noise budget, logarithmic gain conversion, analog detection/dynamic-range budgeting, device equivalent input noise temperature, and a three-stage noise cascade. Levels span basic, intermediate and expert. These are closed-form models suitable for the expression evaluator in `coordination/CONTRACT.md`; none requires numerical simulation, conditionals or arrays.

## Ownership and conventions

- ADC quantization SNR, SINAD and ENOB remain with the data lane. The dynamic-range calculation here takes an externally determined **analog RMS noise floor** and a sinusoidal swing limit; no ADC resolution appears.
- Amplifier gain synthesis remains with the opamps lane. Noise gain is a necessary intermediate in a six-source noise budget, not a ninth standalone calculator.
- Filter synthesis remains with the filters lane. Bandwidth inputs here are **equivalent noise bandwidths** (ENBW), which users must obtain from the actual measurement/filter response. For example, a single-pole low-pass has ENBW equal to pi/2 times its -3 dB bandwidth [S2, equation 8]. This explanation is not an additional calculator.
- All noise spectra here are **one-sided**, with spectral densities in V/sqrt(Hz) or A/sqrt(Hz). Noise power spectral density is the square of the amplitude spectral density. Multiply a white amplitude density by sqrt(ENBW), not by ENBW, to obtain RMS amplitude.
- Input units are SI, except explicitly identified logarithmic dB. Absolute temperature is K, never Celsius. RMS noise is not peak noise; no universal deterministic peak-to-peak conversion is asserted.
- Constants are the exact SI values **k = 1.380649e-23 J/K** [S5] and **q = 1.602176634e-19 C** [S6]. Older application reports round these constants; examples below deliberately use the modern exact values.
- Numerical limits in JSON prevent unreasonable magnitude/overflow and guard algebraic domains; they are not device ratings. Qualitative physical assumptions such as independence, white spectra, adequate loop gain, matching, and the classical condition h f << k T must be verified by the designer. The inputs cannot independently establish those conditions.

## Primary sources

All URLs were researched using search and directly read. The source documents below were successfully retrieved; unavailable Analog Devices pages and gated Keysight PDFs are not needed as evidence for this catalog.

- **[S1] Texas Instruments, SLVA043B, Noise Analysis in Operational Amplifier Circuits.** Shot Noise and Thermal Noise (printed page 2), Adding Noise Sources (printed pages 4–5), and integration/ENBW sections. The older interior pages show the earlier revision suffix. <https://www.ti.com/lit/an/slva043b/slva043b.pdf>
- **[S2] Texas Instruments, Michael Steffes, SBOA066A, Noise Analysis for High-Speed Op Amps, revised January 2005.** Sections 2–4, equations 1–5; section 6, equation 11, and explicit common-downstream-bandlimit assumptions. <https://www.ti.com/lit/an/sboa066a/sboa066a.pdf>
- **[S3] Keysight, Useful Formulas, Conversions, and Reference Information.** Understanding Decibels gives 10 log10 of a power ratio and derives 20 log10 of a voltage ratio only for equal resistance. <https://helpfiles.keysight.com/csg/89600B/Webhelp/Subsystems/gettingstarted/content/concepts_decibels.htm>
- **[S4] Rohde & Schwarz, Mike Leffel and Christopher Stumpf, 1SL378 version 0e, The Cold Source Technique for Noise Figure Measurements, December 2021.** Section 3.1 gives dB/linear relations, reference temperature and equivalent-temperature conversion; sections 3.3–3.4 derive the added-noise model; section 3.6 derives Friis cascade equations 32–40. <https://scdn.rohde-schwarz.com/ur/pws/dl_downloads/dl_application/application_notes/1sl378/1SL378_0e_ColdSrcNF.pdf>
- **[S5] NIST, CODATA Boltzmann constant.** Exact modern SI value. <https://physics.nist.gov/cgi-bin/cuu/Value?k>
- **[S6] NIST, CODATA elementary charge.** Exact modern SI value. <https://physics.nist.gov/cgi-bin/cuu/Value?e>
- **[S7] Texas Instruments, SNOA390B, OA-11 A Tutorial on Applying Op Amps to RF Applications, revised April 2013.** Section 12 Dynamic Range Calculation (printed pages 38–39) defines the minimum detectable signal from the noise floor plus a required detectability margin, and usable dynamic range from that threshold to a maximum permitted signal. Sections 2 and 7 discuss sinusoidal RMS conversion and independently established swing/distortion limits. <https://www.ti.com/lit/an/snoa390b/snoa390b.pdf>

Worked examples are original substitutions computed independently with ordinary scalar arithmetic/Python math, not calculator-engine output or copied manufacturer examples. They are stored with every declared output in JSON, at relative tolerance 1e-6. Defaults equal worked-example inputs and are finite, within bounds and within all algebraic constraints. No build, formatter, linter, engine test or runtime verification was run in this research lane.

## 1. Resistor Johnson noise

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

## 2. Junction shot noise

**ID:** `noise.junction_shot` — **intermediate**. Sources [S1], [S6].

Independent charge carriers crossing a junction barrier have Poisson counting variance. A single carrier-current stream of magnitude I produces one-sided current power spectral density

\[
S_i=2qI\quad[\mathrm{A^2/Hz}].
\]

Consequently

\[
i_n=\sqrt{2qI}\quad[\mathrm{A/\sqrt{Hz}}],\qquad
 i_{n,\mathrm{rms}}=\sqrt{2qIB}=i_n\sqrt B\quad[\mathrm A].
\]

**Inputs/domain:** 0 <= I <= 1000 A, 0 < B <= 1e12 Hz. Zero carrier current produces exactly zero shot noise in this ideal model. I is a nonnegative magnitude, not signed current. For independent photocurrent and dark-current streams, their positive magnitudes contribute to the shot-noise variance. A tiny *net* current resulting from cancellation of large opposing streams cannot be substituted as though only one stream exists. Unity Fano factor, no avalanche gain, and white spectra are assumed. Shot noise is not the model for every ordinary metallic-conductor DC current; thermal, flicker and readout noise are separate.

**Independent example:** I = 1e-6 A, B = 10000 Hz.

- 2qI = 3.204353268e-25 A²/Hz.
- `current_noise_density` = **5.660700723408719e-13 A/sqrt(Hz)** = 0.566070072341 pA/sqrt(Hz).
- RMS variance = 3.204353268e-21 A².
- `current_noise_rms` = **5.660700723408719e-11 A** = 56.6070072341 pA RMS.

## 3. Three-source uncorrelated RMS noise sum

**ID:** `noise.uncorrelated_rms_sum` — **basic**. Source [S1, equations 4–6].

For three zero-mean voltage processes x1, x2, x3 measured at the same node,

\[
\langle(x_1+x_2+x_3)^2\rangle=
\sum_i\langle x_i^2\rangle+2\sum_{i<j}\langle x_ix_j\rangle.
\]

Zero pairwise correlation eliminates the cross terms, giving

\[
v_\mathrm{total}=\sqrt{v_1^2+v_2^2+v_3^2}.
\]

This calculation aggregates already established contributions; it does not generate Johnson/shot noise or apply gain transfers. It is therefore distinct from the source-specific and amplifier models.

**Inputs/domain:** each RMS contribution is 0 to 1e6 V. Zero entries and an all-zero budget are valid. Units, reference node, bandwidth and weighting must agree. Gaussian statistics are not necessary for the variance identity. Coherent ripple, shared reference fluctuations and correlated input voltage/current noise cannot automatically be treated as independent.

**Independent example:** 3, 4 and 12 microvolt RMS. Squared values sum to (9 + 16 + 144) microvolt² = 169 microvolt². `total_noise_rms` = **13 microvolt = 0.000013 V**, not the arithmetic sum of 19 microvolt.

## 4. Complete non-inverting amplifier white-noise budget

**ID:** `noise.noninverting_amplifier_budget` — **expert**. Sources [S2, equations 1–5 and 11], [S5].

The non-inverting signal reaches the plus input through source resistance Rs. Feedback Rf connects output to minus input; Rg connects minus input to ground. Let en be the amplifier voltage-noise density and i+ and i- be the respective input-current-noise densities. All three resistors are at T.

The white-noise transfers to the output are [S2]:

| Source | Output transfer magnitude |
|---|---|
| Amplifier input voltage en | GN = 1 + Rf/Rg |
| Plus-input current i+ | Rs GN |
| Source-resistor voltage sqrt(4kTRs) | GN |
| Minus-input current i- | Rf |
| Feedback-resistor voltage sqrt(4kTRf) | 1 |
| Ground-resistor current sqrt(4kT/Rg) | Rf |

Sum all six output variances:

\[
e_o^2=(e_nG_N)^2+(i_+R_SG_N)^2+4kTR_SG_N^2
 +(i_-R_F)^2+4kTR_F+4kTR_F^2/R_G.
\]

Because 4kTRf + 4kTRf²/Rg = 4kTRf GN, the compact implementable form is

\[
e_o=\sqrt{[e_n^2+(i_+R_S)^2+4kTR_S]G_N^2+(i_-R_F)^2+4kTR_FG_N}.
\]

For a common downstream ENBW B,

\[
v_{o,\mathrm{rms}}=e_o\sqrt B,\qquad v_{\mathrm{in,eq,rms}}=v_{o,\mathrm{rms}}/G_N.
\]

**Inputs/domain:** 0 <= Rs <= 1e9 ohm, 0 < Rf <= 1e9 ohm, 0.001 <= Rg <= 1e9 ohm; en, i+, i- are nonnegative, at most 0.001 in their SI density units; 0 < T <= 2000 K and 0 < B <= 1e12 Hz. The strictly positive Rg makes the gain and input referral well-defined. Rs = 0 is an ideal noiseless source-impedance limit. Loop gain must be high, resistances and GN approximately constant, classical thermal approximation valid, and all sources white/uncorrelated. A lower-bandwidth common downstream filter must dominate any source-specific rolloff. Otherwise integrate each actual transfer spectrum separately. 1/f noise, resistor excess noise, EMI, output loading noise, PSRR effects and stability assessment are outside this model.

**Independent example:** Rs = 1 kohm, Rf = 9 kohm, Rg = 1 kohm, T = 300 K, en = 5 nV/sqrt(Hz), i+ = 1 pA/sqrt(Hz), i- = 2 pA/sqrt(Hz), B = 20000 Hz.

- `noise_gain` = 1 + 9000/1000 = **10 V/V**.
- The six output spectral variances, in V²/Hz, are respectively **2.5e-15**, **1.0e-16**, **1.6567788e-15**, **3.24e-16**, **1.49110092e-16**, and **1.341990828e-15**.
- Their sum is **6.07187972e-15 V²/Hz**.
- `output_noise_density` = **7.79222671641425e-8 V/sqrt(Hz)** = 77.9222671641 nV/sqrt(Hz).
- `output_noise_rms` = sqrt(6.07187972e-15 × 20000) = **1.1019872703439002e-5 V** = 11.0198727034 microvolt.
- `input_referred_noise_rms` = output RMS/10 = **1.1019872703439002e-6 V**.

## 5. Decibel gain to linear ratios

**ID:** `noise.decibel_to_linear` — **basic**. Source [S3].

The definition g = 10 log10(P2/P1) implies

\[
r_P=10^{g/10}.
\]

If the real impedances are equal, P is proportional to V², so the voltage *magnitude* ratio is

\[
r_V=\sqrt{r_P}=10^{g/20}.
\]

This is one conversion calculator with two related outputs, not separately counted forward and inverse rearrangements. It supplies gains for noise budgets and does not predict any noise source.

**Inputs/domain:** -300 <= g <= 300 dB ensures finite positive ratios. Negative values describe attenuation. dB is relative; do not enter an absolute dBm or dBV level. The equal-impedance condition is essential if deriving voltage magnitude from a power gain. Phase and polarity are not recovered.

**Independent example:** g = -6 dB.

- `power_ratio` = 10^(-0.6) = **0.251188643150958**.
- `voltage_ratio` = sqrt(0.251188643150958) = **0.5011872336272722 V/V**.
- Exactly half voltage is -6.020599913 dB; -6 dB is only approximately half voltage.

## 6. Analog dynamic range and detection margin

**ID:** `noise.analog_dynamic_range` — **intermediate**. Sources [S7, section 12], [S3].

This calculation relates a known analog swing limit and noise floor to a specified minimum signal-to-noise margin. It is more than gain-unit conversion: it establishes an absolute minimum usable signal and remaining design range.

Let Vp be the maximum allowed **sinusoidal peak amplitude**, Vn the integrated RMS noise, and M the required power SNR margin in dB. Then

\[
V_\mathrm{max,rms}=V_p/\sqrt2,\qquad
V_\mathrm{min,rms}=V_n10^{M/20}.
\]

The corresponding range to the noise floor and the usable range above the detection threshold are

\[
DR_n=20\log_{10}(V_\mathrm{max,rms}/V_n),
\qquad DR_u=20\log_{10}(V_\mathrm{max,rms}/V_\mathrm{min,rms})=DR_n-M.
\]

The threshold follows because required power ratio is 10^(M/10) and signal/noise amplitude ratio is its square root. TI's section 12 uses the same noise-plus-detectability-margin construction in power units; the voltage implementation here follows from the same-node, equal-impedance relation in [S3].

**Inputs/domain:** 1e-12 <= Vp <= 1e6 V, 1e-18 <= Vn <= 1e6 V, 0 <= M <= 120 dB. The input-only constraint

`maximum_sine_peak/sqrt(2) >= noise_rms*10^(required_snr_db/20)`

requires the maximum available signal to meet the requested margin; equality gives zero usable dynamic range. Positive amplitude bounds guard both logarithms. The supplied swing must already satisfy clipping, slew-rate, output-current and distortion criteria at the relevant frequency. Noise must be measured or budgeted at the same reference node, bandwidth and weighting. The margin is an engineering detection criterion, not a statistical confidence or false-alarm model. This is neither spurious-free dynamic range nor ADC quantization SNR.

**Independent example:** Vp = 2 V, Vn = 10 microvolt RMS, required M = 20 dB.

- `maximum_signal_rms` = 2/sqrt(2) = **1.414213562373095 V**.
- `minimum_signal_rms` = 10e-6 × 10^(20/20) = **0.0001 V**.
- The maximum-to-noise amplitude ratio is 141421.3562373095; `noise_limited_dynamic_range_db` = **103.01029995663981 dB**.
- The maximum-to-threshold amplitude ratio is 14142.13562373095; `usable_dynamic_range_db` = **83.01029995663981 dB**.

## 7. Noise figure to equivalent input temperature

**ID:** `noise.figure_equivalent_temperature` — **intermediate**. Source [S4, equations 4, 10 and 27–31].

The device-added input-referred noise power Na is represented by kTeB. At the standard source reference temperature T0 = 290 K,

\[
F=1+N_a/(kT_0B)=1+T_e/T_0.
\]

Starting with measured/specification noise figure NF in dB,

\[
F=10^{NF/10},\qquad T_e=290(F-1)\quad[\mathrm K].
\]

The distinct physical purpose is device-added-noise equivalence; this is not a second count for a generic dB rearrangement or a cascade calculation. T0 is deliberately fixed at the conventional 290 K reference, not a free arbitrary physical-temperature input.

**Inputs/domain:** 0 <= NF <= 100 dB. These imply F >= 1 and Te >= 0 for conventional nonnegative-added-noise devices. NF = 0 produces F = 1 and Te = 0. Source termination, frequency, bandwidth convention and the 290 K reference must match the measurement/datasheet. Equivalent input noise temperature is not the physical device temperature. An arbitrary actual source temperature changes actual output noise/SNR degradation; this conversion alone does not predict it. Mismatch and nonclassical negative-added-noise models are excluded.

**Independent example:** NF = 3 dB.

- `noise_factor` = 10^0.3 = **1.9952623149688795**.
- `equivalent_noise_temperature` = 290 × 0.9952623149688795 = **288.62607134097505 K**.
- Do not approximate 3 dB as exactly F = 2 when matching the worked precision.

**Numerical implementation correction:** Evaluate T_e as 290 expm1(NF ln(10)/10), not 290 times a rounded linear noise factor minus one. For NF = 1e-16 dB the first-order small-x result is 290 ln(10) 1e-17 ≈ 6.67749676968e-15 K; the naive power-minus-one expression loses it to zero. The returned factor necessarily rounds to unity when its excess is below binary64 spacing, while the independent temperature expression retains the small value. The zero-NF boundary remains exactly zero and the nominal 3 dB oracle is unchanged. This is a stable algebraic realization, not an additional physical model or raised minimum NF.

## 8. Three-stage Friis noise cascade

**ID:** `noise.friis_three_stage` — **expert**. Source [S4, section 3.6, equations 32–40].

For the first two stages, output noise is

\[
N_o=N_0G_1G_2+N_{a1}G_1G_2+N_{a2}G_2.
\]

Dividing by N0 G1 G2 gives F1 + (F2 - 1)/G1. Repeating for a third stage yields

\[
F_c=F_1+\frac{F_2-1}{G_1}+\frac{F_3-1}{G_1G_2}.
\]

Each Fi = 10^(NFi/10) and each Gi = 10^(gi/10) is **linear**; substituting dB values directly into the rational expression is incorrect. The returned cascade measures are

\[
NF_c=10\log_{10}F_c,\qquad T_{e,c}=290(F_c-1).
\]

The third-stage gain cancels out of an input-referred noise factor and therefore is not required as an input. Stage-added noise is referred back through the preceding available power gains. This physical propagation through a cascade is distinct from both simple RSS aggregation and single-device temperature conversion.

**Inputs/domain:** each NFi is 0 to 100 dB; each preceding power gain gi is -120 to +120 dB. Thus Gi is strictly positive, G1 G2 is nonzero, Fc >= 1 and the log is well-defined, including the all-ideal Fc = 1 case. Lossy stages are permitted by negative gain, but users must supply a physically consistent NF for the stage's actual temperature. All stages must be linear/stable and uncompressed, with uncorrelated added noise and compatible/matched impedances; gains are available power gains, not voltage ratios. Parameters must describe a common frequency/passband. Scalar Friis does not replace mismatch/multiple-reflection analysis or a frequency-converting mixer noise convention.

The cascade noise figure uses the standard 290 K source reference. When actual source temperature differs, use equivalent noise temperature to compute actual noise; do not assert that the output SNR necessarily degrades by the quoted standard NF under every source condition [S4, section 3.5].

**Independent example:** stage noise figures 2, 6 and 10 dB; first two power gains 20 and 10 dB.

- `first_noise_factor` = **1.5848931924611136**.
- `second_noise_factor` = **3.9810717055349722**.
- `third_noise_factor` = **10**.
- `first_power_gain` = **100**; `second_power_gain` = **10**.
- First-stage factor = 1.5848931924611136; second-stage excess term = 2.9810717055349722/100 = 0.029810717055349722; third-stage excess term = 9/1000 = 0.009.
- `cascade_noise_factor` = **1.6237039095164632**.
- `cascade_noise_figure_db` = 10 log10(1.6237039095164632) = **2.1050683636637153 dB**.
- `cascade_noise_temperature` = 290 × 0.6237039095164632 = **180.8741337597743 K**.

**Numerical implementation correction:** Let x_i = NFi ln(10)/10 and X = expm1(x_1) + expm1(x_2)/G1 + expm1(x_3)/(G1 G2). The cascade NF is evaluated as 10 log1p(X)/ln(10); its temperature is 290 expm1(NF_c ln(10)/10), reusing the preceding stable NF output. This avoids losing tiny positive added-noise factors when subtracting one from a rounded linear factor. At first-stage NF = 1e-16 dB, other NFs = 0 and gains = 0 dB, first-order independent arithmetic gives cascade NF ≈ 1e-16 dB and temperature ≈ 6.67749676968e-15 K. The linear factor must round to unity below binary64 spacing; that does not justify returning zero for the representable NF/temperature. Exact all-ideal NF/temperature remain zero. Main approved the shared Rust native `log1p` extension; `expm1` was already supported. No nominal example or domain change.

## Implementation handoff

`research/noise.json` uses supported arithmetic, `sqrt`, `ln`, `log10`, `expm1` and the parent-approved `log1p` extension. Supporting output keys appear before outputs that depend on them. The sole cross-input constraint (analog dynamic range) uses only input keys and constants because constraints are evaluated before outputs. Expected examples include every supporting and final output key. No output key duplicates an input key or reserved `pi`/`e`.

The integration owner should exercise example/default numerical evaluation, input-bound rejection, zero shot current, all-zero RMS sum, ideal 0 dB noise figure/cascade, negative-gain cascade, impossible dynamic-range margin, and the constraint-before-outputs path in the global engine checks. These checks are recommended, not claimed to have been run here.
