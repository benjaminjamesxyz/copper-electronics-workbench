# Independent review: data lane

- Integrated review baseline: `7f12f45`; lane patch inspected against `906b351`.
- Decision: **clear** — no actionable patch-introduced correctness/security finding established.
- `tested=false`: static source/algebra/evidence review only. No builds, tests, lint, formatters, smoke, numerical scripts, or hardware execution.
- Independence: reviewed all eight production catalog objects, research definitions/derivations and feature documentation without consuming tester files.

## Review coverage and inspected evidence

Read `coordination/CONTRACT.md`, data rows of `coordination/BACKLOG.md`, `coordination/SPRINT.md`, all of `backend/catalog/data.json`, `research/data.json`, `research/data.md`, and `docs/features/data.md`. Inspected generic consumers in `backend/build.rs`, `backend/src/lib.rs`, `backend/src/main.rs`, `frontend/lib/models.dart`, and `frontend/lib/backend_client.dart`. The build embedding includes every JSON catalog, `Engine::new` loads every object, `calculate` routes by ID and evaluates declared outputs sequentially, and the process response returns every output. No lane-specific whitelist drops these IDs or supporting outputs in the inspected dispatch path.

Actual primary-source inspection in this review (not merely inherited research claims):

1. [TI SLAA694A](https://www.ti.com/lit/an/slaa694a/slaa694a.pdf), sections 2.1–2.2, equations 1–4: reference span divided by 2^N, sine RMS SNR and filtering process gain.
2. [UT Austin Quantization lecture notes](https://users.ece.utexas.edu/~bevans/courses/realtime/lectures/08_Quantization/notes/2020-12-02%20Lecture%208%20Quantization.pdf), both extracted pages: midrise/midtread distinction and uniform-error variance. The notes also use a reconstruction-level endpoint convention with 2^N−1; this is not substituted for the catalog's explicit outer-bin-edge span convention.
3. [TI SBAU128C](https://www.ti.com/lit/ug/sbau128c/sbau128c.pdf), appendix A.1.1 quantization-error excerpts and A.1.3.1–A.1.3.5: RMS noise/distortion measurement, DC exclusion, Nyquist-band measurement and dynamic ENOB. Extraction exposes the familiar rounded ENOB constants; exact constants in the catalog follow independently from its ideal-sine derivation.
4. [TI SLAP082](https://www.ti.com/lit/ml/slap082/slap082.pdf), slide 2-15 extracted text: q/sqrt(12), SINAD=6.02N+1.76, and dynamic ENOB distinction.
5. [TI SNAA079C](https://www.ti.com/lit/an/snaa079c/snaa079c.pdf), sections 2–3 and figure captions for spectral folding: sampling strictly above twice the highest preserved component, bandlimiting and alias ambiguity.
6. [TI DAC80501 datasheet SBAS794E](https://www.ti.com/lit/ds/symlink/dac80501.pdf), section 8.3.1.1: straight-binary code/2^N times reference/divider and gain, code range 0 through 2^N−1. Generic gain in the model is explicitly not a promise of arbitrary physical device settings.
7. [TI SBAA081A](https://www.ti.com/lit/an/sbaa081a/sbaa081a.pdf), section 5, equations 5–10 and accompanying text: source plus switch resistance, effective sampling capacitance, initial capacitor voltage, half-LSB target and the device-specific half-scale 8.32-tau example.

Extracted PDF text has flattened equation layouts; independent algebra below supplies the mathematical check. None of these documents establishes bench accuracy or implementation of an IEEE measurement procedure. The unavailable Analog Devices sources remain explicitly unavailable and are not relied upon.

## Findings

**None meeting the actionable, patch-introduced, provable-impact criteria.** No severity-tagged issue or implementation change is requested. The scenarios below are independent parent-verification prescriptions, not observed runtime results.

## All eight model assessments

### `data.adc_lsb` — catalog lines 3–31

Confirmed q=span/2^N with outer-bin-edge span and count 2^N. The integer-bit guard admits 1 through 32; positive span bounds prevent zero width. Smallest admitted q is 1e-9/2^32, about 2.33e-19 V; largest is 1e6/2, so neither underflow nor overflow is approached. All code counts through 2^32 are exactly representable in binary64. The nominal 3.3/4096 oracle is consistent with independent arithmetic.

Independent parent scenarios: N=1, span=2 V gives count=2 and q=1 V; N=32, span=1 V gives count=4294967296 and q=2^-32 V; doubling span doubles q. Fractional bits and nonpositive span must reject. No accuracy, INL, DNL or compliance claim is made.

### `data.adc_quantization_error` — catalog lines 36–78

Confirmed explicit uniform midrise bins, unsigned labels even for signed analog intervals, excluded upper endpoint, midpoint reconstruction and signed reconstruction-minus-input error. Guards order the endpoints and constrain span before division. The normalized upper-end guard and v_in<v_high protect the nonsaturated final bin. Using local coordinates for error avoids the author's documented loss from subtracting rounded absolute reconstruction at large offsets; this algebraic change preserves the researched physical law and example values.

Independent parent vector: bits=2, v_low=-2 V, v_high=2 V, v_in=-1.5 V gives q=1 V, code=0, reconstructed=-1.5 V, error=0 V, absolute error=0 V. At v_in=-1 V the next bin code=1 reconstructs -0.5 V and error=+0.5 V. At v_in=-2 V the first bin has error=+0.5 V; v_in=2 V rejects. For bits=32, v_low=999 V, v_high=999.000000001 V, v_in=v_low, expect error=q/2 rather than zero, with q based on the actual represented endpoint difference.

Conditioning caveat retained: binary64 subtraction/normalization can change code choice extremely close to an interior transition; absolute midpoint reconstruction may round when q is smaller than voltage spacing. Documentation expressly discloses both, so these are not reported as undisclosed physical guarantees. Away from transitions, the exact model has -q/2<error<=q/2.

### `data.bandlimited_quantization_snr` — catalog lines 83–127

Independently derived sine RMS=a*span/(2*sqrt(2)) and in-band quantization RMS=q/sqrt(12*OSR), with OSR=fs/(2B). Their ratio is a*2^N*sqrt(1.5*OSR), matching the program. Its linear output is an RMS amplitude ratio, so 20log10 is correct; process gain is a power/bandwidth effect using 10log10(OSR). B is one-sided equivalent noise bandwidth, not automatically a -3 dB corner. Integer N, B<=fs/2, positive frequencies/amplitude and the disclosed 16-LSB screen guard the model. Worst admitted OSR is 5e20; the largest amplitude ratio is below 1.2e20, safely finite.

Independent parent vector: N=4, a=1, fs=32 Hz, B=16 Hz gives OSR=1, linear ratio=16*sqrt(1.5), process gain=0 dB and SNR=20log10(16*sqrt(1.5)). Holding B fixed and doubling fs raises SNR by 10log10(2). With N=5, halving a from 1 to 0.5 remains at the 16-LSB screen and lowers SNR by 20log10(2). Below-screen amplitude and B>fs/2 must reject.

Expert validity is conditional on white, uncorrelated error and a sine inside a unity-gain passband; coherent undithered tones, noise shaping, distortion/jitter and analog noise are excluded. The conservative 16-LSB screen is honestly a modeling screen, not primary-source proof of whiteness or hardware accuracy.

### `data.sar_acquisition_settling` — catalog lines 132–173

Independent equation: initial normalized error alpha decays as alpha*exp(-t/tau), tau=(Rs+Rsw)*Cs. Equating it to 2^-(N+1) gives t=tau*ln(alpha*2^(N+1)). Correct full-scale factor is (N+1)ln2; half-scale factor is N ln2. The cited 12-bit 8.32-tau device example is half-scale, and the catalog correctly does not misapply it to a full-scale step. Either resistance may be zero, but their sum must be positive. Already-within-half-LSB errors intentionally reject rather than being silently clamped. All log arguments exceed one and bounded products stay finite; pow2 scaling does not introduce exp-minus-one cancellation near the log's lower boundary.

Independent parent vector: N=1, Rs=1 ohm, Rsw=0, Cs=1e-6 F, alpha=1 gives tau=1e-6 s, required constants=ln4 and acquisition=1e-6*ln4 s. With alpha=0.5 this becomes ln2; alpha=0.25 rejects. Swapping the nonzero resistance preserves tau; multiplying Cs by ten multiplies acquisition by ten.

This is an ADC-specific single-pole theoretical minimum, not throughput, amplifier nonlinear settling, margin certification or full converter accuracy. Necessary datasheet resistance/capacitance and real settling margin are explicit assumptions.

### `data.dynamic_enob` — catalog lines 178–201

Confirmed inverse of the ideal full-scale sine relationship with exact 20log10(2) and 10log10(1.5) constants. The input is measured full-Nyquist SINAD, not nominal quantization-only SNR or DC noise-free bits; no hidden amplitude-backoff correction is claimed. Admitted SINAD is 0–200 dB, and negative/continuous ENOB is retained at its low end. The constant denominator is nonzero; no numerical singularity or overflow is approached.

Independent parent vector: SINAD=10log10(1.5)+8*20log10(2) dB must recover 8 bits; adding 20log10(2) raises ENOB by exactly one bit algebraically. At 0 dB, ENOB=-log2(1.5)/2, about -0.29248 bit. Input above 200 dB rejects. Narrowband and sub-full-scale measurements need external interpretation, as disclosed.

### `data.nyquist_sampling_margin` — catalog lines 206–229

Confirmed fs/2, signed fs/2-fmax and ratio fs/(2*fmax). Positive bounded frequencies guard division, but negative and zero margin are intentionally valid diagnostics rather than errors. Strict positive margin is the stated ideal baseband condition; equality is not called practical or phase-robust no-alias sampling. The reported gap is not anti-alias attenuation/filter synthesis.

Independent parent vectors: highest_frequency=10 Hz with fs=10,20,30 Hz gives Nyquist=5,10,15 Hz; margins=-5,0,+5 Hz; ratios=0.5,1,1.5. Scaling both frequencies by ten preserves ratios and scales margins by ten. No bandpass undersampling claim is made.

### `data.tone_alias_frequency` — catalog lines 234–256

Confirmed k=floor(fin/fs+1/2) and |fin-k*fs|. Independently, replacing fin by fin+k*fs changes sample phase by 2*pi*k*n, so nearest-integer folding gives the correct real first-zone magnitude. Ties choose the higher k without changing mathematical magnitude. DC is admitted, fs is positive and fin/fs<=1e9 caps subtraction conditioning. Integer k stays exactly representable; intermediate multiples remain finite. A nonnegative frequency input is an explicitly specified magnitude domain, not a missing signed-input feature.

Independent parent vectors at fs=8 Hz: fin=0,8,12,11,13 Hz give k=0,1,2,1,2 and aliases=0,0,4,3,3 Hz respectively. Adding 8 Hz preserves each alias magnitude. Ratio beyond 1e9 rejects. A ratio near its cap may lose small residual precision when subtracting large multiples, and a fold-boundary result may perturb by the original-frequency binary64 arithmetic scale; feature docs explicitly warn against blanket near-zero relative assertions. Use frequency-scale-aware absolute tolerances without claiming exact near-zero relative accuracy. No phase, inversion direction or ADC track-and-hold bandwidth guarantee is made.

### `data.unipolar_dac_output` — catalog lines 261–291

Confirmed generic configured span=Vref*gain, output=code*span/2^N and highest output=(2^N-1)*span/2^N. This matches the inspected primary datasheet transfer and does not incorrectly use 2^N-1 as denominator. Integer bits/code and cross-resolution code bounds prevent fractional/overflow codes. Zero code is valid. Positive bounded reference/gain define a unipolar span, with finite maximum intermediate code*span below 4.3e18. Adequate output headroom/loading and excluded static/dynamic errors are explicit; arbitrary generic inputs are not claimed supported DAC80501 settings.

Independent parent vector: N=2, code=3, reference=2 V, gain=1 gives span=2 V and both output/highest-code=1.5 V. Code=0 gives output=0 V while highest-code stays 1.5 V; code=4 rejects. Doubling reference or gain doubles all voltage outputs. Test N=32/code=4294967295 and fractional-code rejection independently.

## Untested scope and handoff

No runtime result or pass is asserted. Domain test authoring/execution belongs to Test-data/Main; this review did not inspect half-authored tests. Parent should exercise all eight complete nominal vectors, every independent scenario above, exact-key/nonfinite rejection, all catalog bounds/cross-input guards, UI metadata/results and native desktop process packaging. Linux/macOS/Windows behavior, hardware performance, source experiments and standards compliance are unverified here. Static consumer inspection is not an end-to-end UI pass. No production/research/test/docs files were edited, and no Git commit was made.
