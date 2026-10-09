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
