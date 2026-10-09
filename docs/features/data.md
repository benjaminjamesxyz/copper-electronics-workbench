# ADC DAC and sampled systems

The executable definitions live in [backend/catalog/data.json](../../backend/catalog/data.json). These offline Rust catalog programs use the shared sequential expression evaluator; no frontend mathematics or duplicate evaluator is required. The lane excludes analog noise budgets, analog-filter synthesis, general passive transients and digital threshold timing.

## Source register and evidence limits

The research handoff read seven English-language primary manufacturer/university artifacts. Evidence supports the stated ideal models, not device performance, hardware validation or standards compliance. Analog Devices MT-001/MT-002 and related articles appeared in searches but could not be fetched; they are not relied upon. Same-manufacturer tutorials are not independent experimental replication. The strongest limitations are convention differences: bin edges versus reconstruction endpoints, filtered versus full-Nyquist SNR, dynamic ENOB versus DC noise-free resolution, and full-scale versus half-scale acquisition steps.

| Reference | Source and evidence |
|---|---|
| S1 | [TI SLAA694A, General Oversampling of MSP ADCs for Higher Resolution](https://www.ti.com/lit/an/slaa694a/slaa694a.pdf), §§2.1–2.2, equations 1–4. High instructional support for resolution and white-noise filtering; not a device guarantee. |
| S2 | [UT Austin EE445S Lecture 8 Quantization, Dan Jacobellis, 2020-12-02](https://users.ece.utexas.edu/~bevans/courses/realtime/lectures/08_Quantization/notes/2020-12-02%20Lecture%208%20Quantization.pdf). Moderate-to-high university instructional support for midrise conventions and error variance, not a device transition specification. |
| S3 | [TI ADCPro User Guide SBAU128C](https://www.ti.com/lit/ug/sbau128c/sbau128c.pdf), appendix A.1.1 and A.1.3.5. High manufacturer support for error and dynamic ENOB definitions; references to IEEE-1241 do not mean these calculators implement its measurement protocol. |
| S4 | [TI SLAP082 Signal Acquisition and Conditioning Seminar, Section 2](https://www.ti.com/lit/ml/slap082/slap082.pdf), slide 2-15. High instructional support for q/sqrt(12) and full-scale sine SNR; same manufacturer as S1/S3. |
| S5 | [TI AN-236/SNAA079C, An Introduction to the Sampling Theorem](https://www.ti.com/lit/an/snaa079c/snaa079c.pdf), §§2–4. High support for uniform sampling and spectral folding; publication age does not invalidate the theorem. |
| S6 | [TI DAC80501/DAC70501/DAC60501 datasheet SBAS794E](https://www.ti.com/lit/ds/symlink/dac80501.pdf), §8.3.1.1, equation 1. Direct support for code/2^N and configured gain/reference division; arbitrary generic settings are not claimed available on that part. |
| S7 | [TI SBAA081A, Using a SAR Analog-to-Digital Converter for Current Measurement in Motor Control Applications](https://www.ti.com/lit/an/sbaa081a/sbaa081a.pdf), §5, equations 5–10. High support for the single-RC acquisition approximation and initial-condition dependence; real margins remain device-specific. |

## Execution, units and admissible domains

All inputs must be finite and exactly match each definition's keys. Bounds and input-only constraints are evaluated before sequential outputs. Integer bits/codes use equality to `floor`; invalid inputs are errors, not saturation or invented fallbacks. SI units are V, Hz, ohm, F and s; bit, count, dimensionless `1`, V/V and dB explicitly identify counting, ratios and logarithmic units. Engineering bounds limit arithmetic extremes rather than certify hardware capability. Counts through 2^32 are exactly representable in binary64. Every default equals its independently derived example, and every output appears in the expected map at relative tolerance **1e-6**; numerical tolerance is not physical model accuracy. No builds, tests, lint, formatting, engine smoke runs or hardware verification are claimed.

## `data.adc_lsb` — Ideal ADC LSB voltage

**Level/purpose:** Basic. Find the analog width of one uniform ADC code bin, not the code selected for a particular voltage.

**Model/formulas and units:** For integer N and full-scale outer-edge span V_span [V], `code_count = M = 2^N` [count], then `lsb_v = q = V_span/M` [V]. S1 equation 1 directly uses the reference difference divided by 2^N. The sequential LSB expression uses the earlier code-count output.

**Assumptions:** Ideal uniform 2^N bins cover the span. The span is the upper reference minus lower reference, not the separation of highest and lowest reconstruction levels. Offset, gain error, INL, DNL and noise are excluded; resolution is not accuracy.

**Domain/error rationale:** `bits` must be an integer in [1,32]. `full_scale_span` is [1e-9,1e6] V. Fractional bits cannot define a binary converter and nonpositive span cannot define bins. Finite engineering limits keep the code count/LSB representable. The denominator is **2^N, not 2^N−1**.

**Independent example/default:** `bits=12`, `full_scale_span=3.3 V`.

| Output | Worked substitution | Expected |
|---|---|---|
| `code_count` | 2^12 | 4096 count |
| `lsb_v` | 3.3/4096 | 0.0008056640625 V (805.6640625 µV) |

**Parent verification scenarios (not exercised here):** full example, one-bit and 32-bit boundaries, span scaling, fractional-bit rejection and span-range rejection.

## `data.adc_quantization_error` — Ideal ADC code and quantization error

**Level/purpose:** Intermediate; discrete-boundary risk. Quantize a particular voltage, reconstruct the selected bin midpoint and report signed/absolute error. This is not merely bin spacing.

**Model/formulas and units:** Integer resolution N [bit] gives `quantization_step=q=(V_high−V_low)/2^N` [V]. Bin k covers `[V_low+kq,V_low+(k+1)q)`. Sequential outputs are `code=k=floor((V_in−V_low)/q)` [count], `reconstructed_v=V_hat=V_low+(k+1/2)q` [V], `error_v=(k+1/2)q−(V_in−V_low)` [V], then `absolute_error_v=abs(error_v)` [V]. Algebraically the signed error is V_hat−V_in, giving `−q/2 < error <= q/2` in the exact model. S2 supports the explicit midrise convention; S3 A.1.1 supports half-LSB ideal error.

**Assumptions:** Uniform ideal midrise bins, unsigned straight-binary labels even for bipolar analog input. Midpoint reconstruction is neither bottom-edge reconstruction nor universal hardware behavior. Offset, INL, DNL, noise and saturation are excluded; consult device transition specifications.

**Domain/error rationale:** Integer `bits` ∈ [1,32]; all voltages ∈ [−1000,1000] V; span at least 1e-9 V. Require `v_low <= v_in < v_high`. The normalized fraction must also stay below one, rejecting an upper-edge value indistinguishable from full scale after binary64 normalization. Ordered constraints reject reversed/too-small spans before dividing. No out-of-range saturation fallback.

**Numerical correction:** The researched expression subtracted the rounded absolute reconstruction from V_in. At N=32, V_low=999 V, V_high=999.000000001 V and V_in=V_low, the half-LSB is approximately 1.16e-19 V, far below the roughly 1.14e-13 V spacing of binary64 values near 999 V. Absolute reconstruction therefore rounds back to V_low; its direct subtraction would falsely return zero. The implemented lower-endpoint-relative error retains q/2 at that boundary without excluding an admissible narrow span. Absolute reconstruction is necessarily rounded, so subtracting displayed voltages need not recover the accurately evaluated local error. Ordinary floating-point uncertainty still applies close to discontinuous bin transitions; this is not extra ADC accuracy.

**Independent example/default:** `bits=10`, `v_low=−1 V`, `v_high=1 V`, `v_in=0.123 V`.

| Output | Worked substitution | Expected |
|---|---|---|
| `quantization_step` | 2/1024 | 0.001953125 V |
| `code` | floor((0.123+1)×512)=floor(574.976) | 574 count |
| `reconstructed_v` | −1+574.5/512 | 0.1220703125 V |
| `error_v` | 574.5/512−1.123 | −0.0009296875 V |
| `absolute_error_v` | abs(−0.0009296875) | 0.0009296875 V |

The absolute error is below q/2=0.0009765625 V. All original example values and tolerance are retained.

**Parent verification scenarios (not exercised here):** full example; lower endpoint and exact interior bin boundaries; midpoint zero error; below-low/excluded-high rejection; fractional-bit and small/reversed-span rejection; normalized upper-edge rejection; large-offset narrow-span lower endpoint must return q/2, not zero.

## `data.bandlimited_quantization_snr` — Bandlimited ideal quantization SNR

**Level/purpose:** Expert. Predict quantization-only sine SNR after digital noise-bandwidth filtering, including amplitude backoff and process gain.

**Derivation/units:** Let q [V] be the ideal LSB, N [bit] resolution and a [1] the sine peak-to-peak/full-scale ratio. Uniform error between ±q/2 has variance `(1/q) integral(error^2 d(error)) = q^2/12`. White noise in the full one-sided Nyquist band retains power fraction `2B/f_s` in the unity-gain passband's equivalent noise bandwidth B [Hz]. With `oversampling_ratio=OSR=f_s/(2B)` [1], retained noise RMS is `q/sqrt(12 OSR)` [V]. Sine RMS is `a*2^N*q/(2 sqrt(2))` [V], so `snr_linear=a*2^N*sqrt(1.5 OSR)` [RMS amplitude ratio, 1]. Sequential outputs then give `process_gain_db=10 log10(OSR)` [dB] and `snr_db=20 log10(snr_linear)` [dB]. This linear output is an amplitude ratio, not a power ratio.

Equivalently SNR [dB] is `N*20 log10(2)+10 log10(1.5)+20 log10(a)+10 log10(OSR)`. S4 slide 2-15 supports q/sqrt(12) and sine SNR; S1 §2.2 equation 4 supports filtering process gain. Exact logarithmic coefficients replace rounded 6.02 and 1.76.

**Assumptions:** Approximately white, uncorrelated, zero-mean uniform quantization error; a sine inside the converter's linear range and unity-gain digital passband. No noise shaping, overload, distortion, thermal/reference noise, jitter or analog noise budget. Oversampling alone without digital bandwidth restriction earns no process gain. Coherent undithered tones may violate the white-error approximation.

**Domain/error rationale:** Integer `bits` ∈ [4,32]; `sample_rate` and `noise_bandwidth` each ∈ [1e-9,1e12] Hz; `amplitude_fraction` ∈ [1e-6,1]. Require `sample_rate >= 2*noise_bandwidth` (OSR≥1) and `amplitude_fraction*2^bits >=16`. The latter is a conservative disclosed applicability screen, not manufacturer assurance of uncorrelated noise. Positive bandwidth/amplitude guard divisors and logarithms; bounds keep all products/root/log arguments finite. B is equivalent noise bandwidth, not automatically a −3 dB cutoff.

**Independent example/default:** `bits=12`, `sample_rate=1000000 Hz`, `noise_bandwidth=20000 Hz`, `amplitude_fraction=0.8`.

| Output | Worked substitution | Expected |
|---|---|---|
| `oversampling_ratio` | 1000000/(2×20000) | 25 |
| `snr_linear` | 0.8×4096×sqrt(37.5) | 20066.219972879797 |
| `process_gain_db` | 10 log10(25) | 13.979400086720377 dB |
| `snr_db` | 20 log10(20066.219972879797) | 86.04931137647155 dB |

At a=1 and B=f_s/2 the equation reduces to ideal full-Nyquist sine SNR. The example's full-scale/unfiltered 12-bit SNR is about 74.0081 dB; amplitude backoff contributes −1.9382 dB and filtering +13.9794 dB.

**Parent verification scenarios (not exercised here):** example; OSR=1/full-scale identity; doubled sample rate at fixed B gives +10 log10(2) dB; amplitude halving gives −20 log10(2) dB where the 16-LSB screen permits it; fractional bits, B>Nyquist and <16-LSB rejection; exact 16-LSB boundary.

## `data.sar_acquisition_settling` — SAR ADC half-LSB acquisition settling

**Level/purpose:** Expert. Find the theoretical positive acquisition time for a SAR sampling capacitor's initial error to fall to half an N-bit LSB. This converter-specific error target is not a generic RC waveform or digital logic threshold delay.

**Derivation/units:** S7 §5 equations 5–10 model source resistance R_source [ohm] plus sampling-switch resistance R_switch [ohm] charging effective C_sample [F]. The initial error magnitude deltaV [V] decays as `deltaV exp(−t/tau)`, where `time_constant_s=tau=(R_source+R_switch)C_sample` [s]. Half-LSB is `V_span/2^(N+1)`. With `step_fraction=alpha=deltaV/V_span` [1], division and logarithms give `required_time_constants=ln(alpha*2^(N+1))` [1], then `minimum_acquisition_s=tau*required_time_constants` [s]. Outputs retain this sequential order.

**Assumptions:** Input constant during acquisition; one linear time-invariant R/C pole; alpha is a magnitude with 0<alpha≤1. R_switch and C_sample must come from the selected ADC. Amplifier slew/recovery, charge buckets, charge kickback, switch nonlinearity, multiple poles, parasitics and tolerances are excluded. Add device/datasheet margin. The result excludes conversion time, is not maximum throughput, and does not certify accuracy.

**Initial-condition distinction:** Full-scale alpha=1 needs `(N+1)ln(2)` time constants; half-scale alpha=1/2 needs `N ln(2)`. S7's 12-bit ADS7864 example starts the capacitor at midscale and yields about 8.32 tau; that factor must not be reused for a full-scale step. S7 recommends additional time-constant margin for its specific device/application.

**Domain/error rationale:** Integer `bits` ∈ [1,32]; source/switch resistances each ∈ [0,1e9] ohm but their sum must be >0. `sample_capacitance` ∈ [1e-15,1e-3] F; `step_fraction` ∈ [1e-12,1]. Require `step_fraction*2^(bits+1)>1`: already-within-half-LSB errors are intentionally outside this positive-time model, not silently clamped to zero. Zero total R invalidates the acquisition RC representation. All logarithm arguments are greater than one and engineering bounds keep multiplication finite.

**Independent example/default:** `bits=12`, `source_resistance=1000 ohm`, `switch_resistance=20 ohm`, `sample_capacitance=1.5e-11 F` (15 pF), `step_fraction=1`.

| Output | Worked substitution | Expected |
|---|---|---|
| `time_constant_s` | (1000+20)×15e-12 | 1.53e-8 s (15.3 ns) |
| `required_time_constants` | ln(1×2^13)=ln(8192) | 9.010913347279288 |
| `minimum_acquisition_s` | 1.53e-8×9.010913347279288 | 1.3786697421337312e-7 s (137.86697421337312 ns) |

At this time the remaining full-scale error fraction is exp(−ln(8192))=1/8192, exactly the 12-bit half-LSB target.

**Parent verification scenarios (not exercised here):** example; half-scale/full-scale logarithmic factors; either resistance zero with the other positive; capacitance/resistance scaling; initial-error ratio equal/below one rejection; zero total R and fractional bits rejection.

## `data.dynamic_enob` — Dynamic ENOB from measured SINAD

**Level/purpose:** Intermediate. Convert a real ADC's measured combined noise-and-distortion performance into a continuous equivalent ideal full-scale sine resolution. Unlike nominal bandlimited quantization SNR, this consumes a measurement and does not predict noise-only performance or offer another nominal-resolution sizing mode.

**Derivation/units:** Equate measured full-Nyquist SINAD [dB] to the ideal full-scale sine result, `SINAD=20 log10(2)*N_eff+10 log10(3/2)`. Therefore `enob_bits=(sinad_db−10 log10(1.5))/(20 log10(2))` [bit]. The exact constants are 6.020599913279624 dB/bit and 1.760912590556812 dB. S3 A.1.3.5 equation 4 supports dynamic ENOB and distinguishes precision-ADC DC noise-based resolution; S4 gives the familiar rounded relation.

**Assumptions:** An independently measured pure full-scale sine, excluding DC but including harmonics/noise in the declared Nyquist measurement band. Sub-full-scale SINAD needs external amplitude normalization; none is added here. Narrowband/noise-shaped measurements are not directly comparable. This figure separates neither noise from distortion nor predicts DC accuracy, static linearity or noise-free resolution.

**Domain/error rationale:** `sinad_db` ∈ [0,200] dB. There is no integer-bit constraint on the output: ENOB is continuous and can be negative even within this admitted SINAD range. At 0 dB, ENOB is `−ln(1.5)/(2 ln(2))`, approximately −0.29248 bit; it is not clamped. The fixed positive logarithm arguments and nonzero constant denominator need no additional relational guard.

**Independent example/default:** `sinad_db=68 dB`; `enob_bits=(68−1.760912590556812)/6.020599913279624=11.002074272256452 bit`. This is the sole expected output at relative tolerance 1e-6.

**Parent verification scenarios (not exercised here):** example; ideal-sine SINAD for an independently selected integer N recovers N; +20 log10(2) dB increases ENOB by one; zero SINAD yields negative ENOB; out-of-range/nonfinite SINAD rejection.

## `data.nyquist_sampling_margin` — Baseband Nyquist sampling margin

**Level/purpose:** Basic. Compare chosen uniform real sample rate against twice the highest preserved baseband component, reporting available ideal anti-alias transition space.

**Model/formulas and units:** For `sample_rate=f_s` [Hz] and `highest_frequency=f_max` [Hz], sequential outputs are `nyquist_frequency=f_N=f_s/2` [Hz], `frequency_margin=f_N−f_max` [Hz], and `rate_ratio=f_s/(2 f_max)` [1]. S5 §§2 and 4 supports uniform-sampling spectral separation when baseband support lies strictly below f_s/2.

**Assumptions:** Uniform real-valued baseband sampling and a highest-frequency input including every component that must be preserved. Positive margin is the stated ideal no-overlap condition; practical anti-alias attenuation, out-of-band interferers, filter order, aperture response and bandpass undersampling are excluded. The gap is not a filter design or rejection guarantee. Equality is not sufficient phase-robust/practical sampling: a Nyquist sine can produce zero samples at certain phases.

**Domain/error rationale:** Both inputs ∈ [1e-9,1e12] Hz, so the rate-ratio denominator is positive and bounded arithmetic is finite. Zero/negative frequencies are invalid, but **negative margin remains a valid diagnostic output**. No constraint rejects undersampling or equality.

**Independent example/default:** `sample_rate=48000 Hz`, `highest_frequency=20000 Hz`.

| Output | Worked substitution | Expected |
|---|---|---|
| `nyquist_frequency` | 48000/2 | 24000 Hz |
| `frequency_margin` | 24000−20000 | 4000 Hz |
| `rate_ratio` | 48000/(2×20000) | 1.2 |

**Parent verification scenarios (not exercised here):** example; f_s below/equal/above 2 f_max gives negative/zero/positive margin; doubling both frequencies preserves rate_ratio and doubles margin; reject nonpositive/out-of-range frequencies.

## `data.tone_alias_frequency` — Sampled tone alias frequency

**Level/purpose:** Intermediate. Fold a specified nonnegative analog tone into the first real-sampling Nyquist zone; this diagnoses an alias, rather than imposing a no-alias inequality.

**Derivation/units:** Adding integer k times f_s to a tone shifts sample-n phase by `2 pi k n`, leaving the sampled sequence unchanged. Real-valued spectra identify negative-frequency magnitudes. Choose `nearest_sample_multiple=k=floor(f_in/f_s+1/2)` [count], then `alias_frequency=abs(f_in−k f_s)` [Hz], using the earlier k output. The centered remainder lies in [−f_s/2,f_s/2), so its mathematical magnitude is in [0,f_s/2]. S5 §§3–4 supports spectral replication/folding; the nearest-integer formula is a derived closed form. Half-integer ties choose the higher k without changing alias magnitude.

**Assumptions:** Single tone, exact fixed uniform sampling, real first-zone magnitude. No phase or spectral-inversion direction, anti-alias attenuation, aperture behavior or ADC input bandwidth is computed. An alias result does not establish that hardware acquires the analog tone. The input is a nonnegative magnitude; real spectra would reflect a negative tone to its positive magnitude.

**Domain/error rationale:** `input_frequency` ∈ [0,1e12] Hz, `sample_rate` ∈ [1e-9,1e12] Hz; require `input_frequency/sample_rate <=1e9`. DC input is admitted and f_s must be positive for division. The ratio cap limits subtraction precision loss at large multiples rather than asserting exact relative accuracy for aliases approaching zero. Near fold boundaries, binary64 rounding can perturb tie selection or the remainder by the original-frequency arithmetic scale; hardware frequency uncertainty is not modeled.

**Independent example/default:** `input_frequency=173000 Hz`, `sample_rate=48000 Hz`.

| Output | Worked substitution | Expected |
|---|---|---|
| `nearest_sample_multiple` | floor(173000/48000+0.5)=floor(4.1041666…) | 4 count |
| `alias_frequency` | abs(173000−4×48000) | 19000 Hz |

The alias lies below 24000 Hz Nyquist. An exact integer-rate tone aliases to DC; `(k+1/2)f_s` aliases to Nyquist, including the half-rate case.

**Parent verification scenarios (not exercised here):** example; zero input; integer-rate DC aliases; half-integer Nyquist aliases and either side of a fold; added integer sample-rate multiples preserve alias magnitude; ratio=1e9 boundary and larger-ratio rejection; nonpositive sample-rate rejection. Near-zero assertions should use frequency-scale-aware absolute tolerance, not a blanket 1e-6 Hz or relative-only criterion.

## `data.unipolar_dac_output` — Ideal unipolar DAC output voltage

**Level/purpose:** Basic. Translate an unsigned DAC code into ideal static voltage using reference-to-output configured gain. Digital-to-analog reconstruction is not an inverse ADC bin-width feature.

**Model/formulas and units:** Integer N [bit] gives codes D from zero through 2^N−1 [count]. For reference V_ref [V] and gain G [V/V], sequential outputs are `configured_span=G V_ref` [V], `output_v=D*configured_span/2^N` [V], and `maximum_code_v=(2^N−1)*configured_span/2^N` [V]. S6 §8.3.1.1 equation 1 is `DAC_DATA/2^N × VREFIO/DIV × GAIN`; the generic G is GAIN/DIV. The denominator is **2^N, not 2^N−1**: highest-code voltage is one LSB below span.

**Assumptions:** Straight-binary unipolar DAC, adequate output supply headroom/load capability, configured gain incorporating buffer gain and reference division. Generic references/gains are not claimed available settings of the cited part. Reference error, offset, INL, DNL, settling, glitches and loading are excluded. Real hardware must satisfy its output/load specifications; no rail-clipping fallback is added.

**Domain/error rationale:** Integer `bits` ∈ [1,32]; integer `code` ∈ [0,4294967295] and additionally `code <=2^bits−1`. `reference_v` ∈ [1e-9,1e6] V and `gain` ∈ [1e-6,1000] V/V. Cross-input code limit prevents overflow for the selected resolution; integer constraints reject fractional codes/bits. Positive reference/gain define a positive unipolar span, and bounds keep arithmetic finite. Zero and highest valid code are admitted.

**Independent example/default:** `bits=12`, `code=1234`, `reference_v=2.5 V`, `gain=2 V/V`.

| Output | Worked substitution | Expected |
|---|---|---|
| `configured_span` | 2.5×2 | 5 V |
| `output_v` | 1234×5/4096 | 1.50634765625 V |
| `maximum_code_v` | 4095×5/4096 | 4.998779296875 V |

**Parent verification scenarios (not exercised here):** example; zero code gives zero voltage; highest code equals maximum_code_v and stays one LSB below span; one-bit/32-bit boundaries; reference/gain scaling; fractional/overflow code, fractional bits and invalid reference/gain rejection.

## Complete lane handoff and unexercised verification

All eight preserved IDs are executable catalog programs using the shared engine: ADC bin width, midrise deterministic error, bandlimited quantization SNR, SAR half-LSB acquisition, measured-SINAD ENOB, baseband Nyquist margin, sampled-tone folding and unipolar DAC voltage. The basic first commit is followed by discrete/expert risk models and then remaining features. Sources, model limits, finite defaults, input-only constraints, sequential supporting outputs and every independently researched expected value are retained.

The only substantive formula correction is the lower-endpoint-relative quantization error described above; it preserves the physical formula and input domain while avoiding loss of a small error when the absolute reconstruction rounds. No research files or shared engine/UI files are changed. Main should exercise all examples/defaults, the per-feature boundary/identity scenarios and exact-key/nonfinite input rejection after integrating the complete wave. Independent testers/reviewers must consume Main's full integrated snapshot. No build, lint, test, formatter, smoke or hardware verification has been executed for this lane.
