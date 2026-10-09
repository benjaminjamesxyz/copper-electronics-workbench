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
