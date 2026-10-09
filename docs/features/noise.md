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

## Three-stage Friis noise cascade

**ID:** `noise.friis_three_stage` — **expert**. Purpose: refer independently added stage noise back through preceding available power gains. Source [S4, section 3.6, equations 32–40].

For two stages N_out = N0 G1 G2 + N_added1 G1 G2 + N_added2 G2. Divide by N0 G1 G2, and repeat with stage three:

F_c = F1 + (F2 − 1)/G1 + (F3 − 1)/(G1 G2), with Fi = 10^(NFi/10), Gi = 10^(gi/10).

NF_c = 10 log10(F_c), T_e,c = 290(F_c − 1) K. Third-stage gain cancels and is not required. This is not simple RMS aggregation or a voltage-gain cascade.

**Stable numerical realization:** x_i = NFi ln(10)/10; X = expm1(x_1) + expm1(x_2)/G1 + expm1(x_3)/(G1 G2). Compute NF_c = 10 log1p(X)/ln(10), then T_e,c = 290 expm1(NF_c ln(10)/10). This preserves tiny positive excess noise lost by subtracting unity from rounded factors. With NF1 = 1e-16 dB, NF2 = NF3 = 0, g1 = g2 = 0 dB, independently the small-x limit gives NF_c ≈ 1e-16 dB and T_e,c ≈ 6.67749676968e-15 K. Linear factors necessarily round to unity below binary64 spacing. The all-ideal NF/temperature outputs remain exactly zero. Main owns the approved native Rust log1p extension; no local evaluator is introduced.

**Inputs/domain/error rationale:** all NFi in [0,100] dB, gains g1,g2 in [-120,120] dB. Reject nonfinite/out-of-range inputs and negative conventional NF. Gains in [1e-12,1e12] are strictly positive; their product cannot vanish or overflow within these bounds. F_c ≥ 1 and X ≥ 0 guard logarithms; upper ranges bound cascade excess by roughly 1e34. Negative gain is permitted for loss, provided the supplied NF is consistent with physical attenuator temperature. No extra numerical NF minimum is imposed.

**Assumptions:** stable, linear, uncompressed stages, common frequency/passband, uncorrelated added noise, flat parameters, available POWER gains and compatible/matched terminations. Scalar Friis excludes mismatch/multiple reflections and mixer frequency-conversion conventions. Every NF uses T0 = 290 K; this reference is not physical device/source temperature. An arbitrary actual source temperature need not give output SNR degradation equal to standard NF [S4, section 3.5].

**Independent worked example:** NF1 = 2 dB, NF2 = 6 dB, NF3 = 10 dB, g1 = 20 dB, g2 = 10 dB.

- `first_noise_factor` = **1.5848931924611136**.
- `second_noise_factor` = **3.9810717055349722**.
- `third_noise_factor` = **10**.
- `first_power_gain` = **100**.
- `second_power_gain` = **10**.
- Second excess = 2.9810717055349722/100 = 0.029810717055349722; third excess = 9/1000 = 0.009.
- `cascade_noise_factor` = **1.6237039095164632**.
- `cascade_noise_figure_db` = **2.1050683636637153 dB**.
- `cascade_noise_temperature` = **180.8741337597743 K**.

All supporting outputs remain in the original order (three factors, two gains, cascade factor, cascade NF, cascade temperature). The 1e-6 nominal oracle is unchanged; small-NF tests require quantity-scaled tolerances rather than a blanket absolute 1e-6.


## Junction shot noise

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


**Error rationale and output order:** Reject nonfinite values, negative current, nonpositive ENBW and inputs above the declared maxima. Positive bandwidth establishes the integration band; nonnegative current protects the square root, including the exact zero-current limit. Density precedes integrated RMS; no current absolute-value fallback silently accepts signed inputs.


## Three-source uncorrelated RMS noise sum

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


**Error rationale and sequential outputs:** Reject nonfinite, negative or above-1e6-V RMS contributions. Nonnegative inputs represent amplitudes; their squares keep the sole total-noise root nonnegative. Upper bounds bound squared sums. Individual zero inputs and the all-zero budget are admitted. Correlation, common-node and common-bandwidth requirements remain qualitative assumptions, not numeric validation.


## Decibel gain to linear ratios

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


**Error rationale and sequential outputs:** Reject nonfinite gain and values outside [-300,300] dB. These numerical limits give finite positive power ratios [1e-30,1e30]; their square root is valid. Zero dB returns unity and negative dB is admitted attenuation. Power ratio precedes equal-impedance voltage magnitude ratio; no absolute-level dBm reinterpretation or polarity inference is made.


## Analog dynamic range and detection margin

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


**Error rationale and sequential outputs:** Reject nonfinite/out-of-range peak, RMS floor or required margin. Positive peak and floor minima prevent division/log singularities; bounded magnitudes keep all ratios finite. Before any outputs, the input-only maximum-RMS >= floor*10^(margin/20) constraint rejects an impossible detection threshold; equality gives exactly zero usable range when the computed scalars are equal. Output order is maximum RMS, minimum RMS, noise-limited range, usable range. A nonnegative required margin is a stated detection policy, not an ADC SNR or hardware-certification rule.


## Noise figure to equivalent input temperature

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


**Stable numerical realization:** T_e = 290 expm1(NF ln(10)/10) avoids subtracting unity from a rounded factor. For NF = 1e-16 dB, independent small-x arithmetic gives T_e ≈ 6.67749676968e-15 K. F necessarily rounds to unity below binary64 spacing, but temperature is separately representable. Zero NF still yields exactly zero temperature. This correction changes neither physical model nor bounds nor nominal expected values.

**Error rationale and sequential outputs:** Reject nonfinite NF, negative NF and values above 100 dB. Nonnegative NF enforces conventional nonnegative added noise; the maximum bounds power factors by 1e10 and temperature by approximately 2.9e12 K. Factor is reported first; stable temperature is evaluated directly from input rather than subtracting one from that rounded factor. The fixed 290 K reference is not an adjustable physical temperature. No cross-input constraint is needed.


## Integration and independent QA handoff

The backend/catalog/noise.json array contains exactly the eight IDs documented above. The shared Rust engine handles finite/exact input keys, inclusive/exclusive bounds, input-only constraints and ordered expression evaluation; Flutter consumes catalog metadata and performs no noise math. The frozen worker engine lacks the newly parent-approved log1p helper; Main must integrate this lane with its extended shared engine.

Recommended checks for Main and independent QA, not executed here:

- Every default/full independent example and every declared output at relative tolerance 1e-6; missing/extra/nonfinite inputs and each range endpoint.
- Johnson density scales as sqrt(R T); RMS scales as sqrt(ENBW); reject nonpositive R/T/B and distinguish open-circuit voltage from matched-load available power.
- Shot noise admits I = 0, rejects signed negative current; RMS integration uses sqrt(B).
- RSS admits all-zero and isolated contributions; 3-4-12 gives 13, not 19.
- Amplifier budget isolates each of all six sources, admits R_S = 0 and zero amplifier densities, retains the ground-resistor variance and input referral by noise gain.
- 0 dB conversion gives unity; negative dB is valid attenuation, with equal-impedance voltage interpretation.
- Dynamic range rejects an impossible margin before output evaluation; equal maximum and threshold gives zero usable range.
- 0 dB conventional NF gives unity factor/zero temperature. At NF = 1e-16 dB, temperature remains approximately 6.67749676968e-15 K despite rounded F = 1.
- All-ideal Friis gives unity factor/zero NF/zero temperature; negative gains are valid. Test tiny first-stage NF with other stages ideal and also NF2 = 1e-16 dB following first-stage gain -120 dB: the second-stage excess must be computed before weighting, so X ≈ ln(10)*1e-5, not zero. Tiny physical quantities need scale-aware tolerances, not blanket absolute 1e-6.

No build, linter, formatter, test, smoke command or numerical engine run was performed by this developer. Sources and independent nominal substitutions are inherited from research with the source-access caveat above; the small-noise corrections are explicitly derived algebraic implementations.
