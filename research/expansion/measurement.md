# Measurement expansion research

Exactly six new calculations: one basic, three intermediate, two expert. The numerical definitions are in `measurement.json`. This report specifies the physical models, independent examples, tester cases and six mandatory diagrams. No production code, tests, build, lint, formatter or engine execution was performed.

## Scope, novelty and prerequisites

Read the expansion contract, numerical contract, expansion allocation, complete baseline ID/formula inventory and expansion communication bus before selecting these models. Existing ADC LSB, quantization, ENOB, alias, Nyquist, acquisition settling, independent RMS sums, ordinary RC response, resistive loaded dividers and rise-time/bandwidth conversion were excluded. Precision and logic were notified of measurement ownership.

| New ID | Level | Nearest baseline or neighbor | Distinct physical problem |
|---|---|---|---|
| `measurement.dmm_accuracy_bound` | basic | `data.adc_quantization_error` | Manufacturer gain/offset accuracy envelope depends on reading and selected range, not ADC code width. |
| `measurement.delta_resistance` | intermediate | `dc.ohm_current`; sensor transduction | A three-reading reversal experiment rejects constant and linearly drifting thermoelectric EMF; it does not merely rearrange Ohm's law. |
| `measurement.compensated_probe_loading` | expert | `dc.loaded_divider`, `filters.rc_lowpass_response`; precision amplifier stages | Two parallel-RC arms create a compensation zero plus source-loading poles; predicts loading and adjustment of a passive measurement probe. |
| `measurement.rectangular_aperture_response` | intermediate | `data.sar_acquisition_settling`, `data.tone_alias_frequency` | Uniform measurement integration has sinc nulls and midpoint time attribution; no acquisition RC or alias mapping. |
| `measurement.sampling_jitter_snr` | intermediate | `data.bandlimited_quantization_snr`; logic timing | Random sampling-time error converts signal slope into voltage noise; independent of bit depth and clock-rate division. |
| `measurement.correlated_voltage_ratio` | expert | `noise.uncorrelated_rms_sum`; precision reference design | Ratio sensitivities introduce a negative covariance contribution and common-reference cancellation; evaluates a measurement process, not independent voltage-noise addition. |

**Shared engine prerequisite:** Main explicitly approved `sinc(x)=sin(x)/x`, with `sinc(0)=1`, and owns implementation and tests. Every other expression uses the existing function set. Native `sin(x)/x` is well-conditioned near zero; do not introduce an arbitrary frequency floor, epsilon denominator or needless approximation series. Do not convert aperture gain to dB: true nulls would produce an infinite value. No other shared engine change is requested.

**Evidence actually read:** Keithley/Tektronix accuracy handbook and delta-method article; Tektronix probe primer; University of Cincinnati probe laboratory notes; NI DMM aperture tutorial; TI SNAP004 PDF pages 18–20; NIST TN 1297 Appendix A and section 6. Manufacturer technical writing, university teaching notes and national-metrology guidance are primary sources here. The probe's finite-source extension and the boxcar integral are explicitly derived below rather than falsely attributed as verbatim source equations.

**Access gaps:** Analog Devices MT-007 PDF, its HTML route and its clocks/timing PDF timed out. Keysight's DMM blog returned HTTP 403. These inaccessible pages are not supporting references in the catalog. TI and Keithley supplied accessible replacements. University probe equations are embedded images; the textual topology and variable definitions were read, and the transfer equation was independently derived from that topology. No inaccessible image equation is claimed as read. Search-result summaries were not treated as evidence; one aperture search summary incorrectly called a null −3.92 dB, which is not used here.

## 1. `measurement.dmm_accuracy_bound`

### Purpose, source and validity

[Keithley, E-Handbook to Understanding Electrical Test and Measurement](https://www.tek.com/en/documents/fact-sheet/e-handbook-understanding-electrical-test-and-measurement), “Step 2 | Designing the Measurement System,” subsections Accuracy and Instrumentation error, explicitly gives `±(% reading + % range)` and worked 0.5 V / 2 V-range examples. It distinguishes gain error, offset error, calibration interval and temperature coefficient.

Let signed reading be `reading` in V, selected positive full-scale range be `range_value` in V, and percentages `reading_percent`, `range_percent` be numerical percentages, not fractions. Then:

- `reading_error_bound = abs(reading) * reading_percent / 100` V.
- `range_error_bound = range_value * range_percent / 100` V.
- `total_error_bound` is their arithmetic sum, not RSS.
- `lower_bound = reading - total_error_bound`; `upper_bound = reading + total_error_bound` V.

The selected range means the manufacturer's named range value, not the full negative-to-positive span. This model deliberately does not infer uncertainty distributions or calibration confidence from a specification bound. Use only coefficients applicable to the actual function, temperature, time since calibration, speed and range. Manufacturer overrange behavior is excluded by `abs(reading) <= range_value`. Source loading and lead errors are separate physical effects.

### Complete independent example

Inputs: `reading=0.5 V`, `range_value=2 V`, `reading_percent=0.003 %`, `range_percent=0.001 %`.

Direct decimal arithmetic gives:

| Output key | Derivation | Expected |
|---|---|---|
| `reading_error_bound` | 0.5 × 0.00003 | 0.000015 V |
| `range_error_bound` | 2 × 0.00001 | 0.000020 V |
| `total_error_bound` | 0.000015 + 0.000020 | 0.000035 V |
| `lower_bound` | 0.5 − 0.000035 | 0.499965 V |
| `upper_bound` | 0.5 + 0.000035 | 0.500035 V |

This reproduces the manufacturer example through independent arithmetic, without invoking the application engine.

### Tester handoff

- Nominal: assert all five outputs above.
- Negative reading −0.5 V, other inputs nominal: contributions and total unchanged; endpoints −0.500035 and −0.499965 V.
- Zero reading: reading contribution 0, range contribution and total 20 µV, endpoints −20 and +20 µV. Do not divide by reading to report an undefined relative error.
- Zero coefficients are allowed and return a zero-width interval. Exactly `abs(reading)=range_value` is allowed.
- Reject range ≤0, negative percentages, `abs(reading)>range_value`, and nonfinite inputs.

### Mandatory diagram specification

Canvas 700×280, 16 px margins, labels ≥14 px. Left region: DC DUT node connected to a voltmeter rectangle, common return grounded; show + and − meter terminals. Label meter with `reading` and `range_value`. Right region: horizontal voltage interval, center dot for `reading`, endpoint ticks bound to `lower_bound` and `upper_bound`. A bracket from center to upper endpoint binds `total_error_bound`. Below, two additive error arrows labeled `reading_error_bound` and `range_error_bound`; describe their sum, not a root-sum-square. Coefficients `reading_percent` and `range_percent` belong in the caption/quantity legend if canvas space is insufficient.

Caption: “The reading term grows with signal magnitude; the range term remains at zero input. This is a specification bound, not a statistical confidence interval. Interval geometry is illustrative, not a live scale.”

## 2. `measurement.delta_resistance`

### Purpose, source and derivation

[Keithley, Low-Voltage Measurement Techniques](https://www.tek.com/en/documents/technical-article/low-voltage-measurement-techniques), “The Delta Method of Measuring Resistance” and “Equipment Requirements,” explicitly derives three readings at alternating current and explains equally timed intervals, source settling, thermal drift and four-wire sensing.

Let readings occur at times `−dt, 0, +dt`, with equal current magnitudes `+I, −I, +I`. Keep voltage lead polarity fixed. If resistance is R and EMF is `E(t)=E0+m t`, then:

`V1=I R+E0−m dt`, `V2=−I R+E0`, `V3=I R+E0+m dt`.

Solving the three equations yields:

- `resistive_voltage = ((V1−V2)+(V3−V2))/4 = I R` V.
- `resistance = resistive_voltage/test_current` Ω.
- `midpoint_emf = ((V1+V2)+(V3+V2))/4 = E0` V.
- `emf_drift_rate = (V3−V1)/(2 sample_interval)` V/s.

The catalog splits weighted differences to avoid unnecessarily forming `2*V2` or a large sum. Cancellation of a large actual offset cannot recover digits already lost by the measurement instrument. DUT self-heating, current-reversal magnitude mismatch, nonlinear resistance and quadratic thermal drift remain real errors. For `E(t)=E0+m t+q t²`, the recovered resistive voltage is biased by `q dt²/2`; this supplies a concrete model-limit warning, not an unsupported guarantee.

### Complete independent example

Choose a 0.1 Ω resistor, 0.01 A test current, 0.1 s interval, midpoint EMF 14 µV and drift +20 µV/s. The physical forward model gives all input readings independently:

- `voltage_first = 1 mV + 14 µV − 2 µV = 0.001012 V`.
- `voltage_second = −1 mV + 14 µV = −0.000986 V`.
- `voltage_third = 1 mV + 14 µV + 2 µV = 0.001016 V`.

Outputs:

| Output key | Independent calculation | Expected |
|---|---|---|
| `resistive_voltage` | (1.998 mV + 2.002 mV)/4 | 0.001 V |
| `resistance` | 0.001/0.01 | 0.1 Ω |
| `midpoint_emf` | (26 µV + 30 µV)/4 | 0.000014 V |
| `emf_drift_rate` | 4 µV / 0.2 s | 0.000020 V/s |

### Tester handoff

- Nominal: assert all four outputs.
- Constant offset: readings `{0.001010, −0.000990, 0.001010}` V, nominal I and dt, return `{resistive_voltage:0.001,resistance:0.1,midpoint_emf:0.000010,emf_drift_rate:0}`.
- Add the same signed voltage to all three readings: resistance and slope must stay unchanged; midpoint EMF shifts by that voltage.
- Swap first and third readings: resistance and midpoint EMF stay unchanged; drift changes sign.
- Pure linear EMF with zero resistance: readings `{12e-6,14e-6,16e-6}` V return 0 V, 0 Ω, 14 µV and 20 µV/s, within cancellation-aware absolute tolerance on the zero outputs.
- Reject current ≤0, interval ≤0, or a negative delta resistive voltage with the stated passive-resistor polarity. Signed individual readings and negative EMF/drift are valid.
- Nonlinear drift cannot be detected from only these three observations: test the documented `q dt²/2` bias as a limitation, not as a rejected input.

### Mandatory diagram specification

Left half: Kelvin resistance setup. Draw a resistor with separate outer force connections and inner sense connections; current source drives +I/−I, nanovoltmeter has fixed + terminal at the resistor's upper sense node. Place an EMF source in a sense lead, not in the force branch; + direction contributes to the positive meter reading. Bind resistor to `resistance`, current magnitude to `test_current`, and sense-lead EMF to `midpoint_emf`.

Right half: timing strip with three equal-spaced sample markers `+I, −I, +I`; bind markers to `voltage_first`, `voltage_second`, `voltage_third`. Two equal interval brackets share `sample_interval`. Add a tilted dashed EMF baseline labeled `emf_drift_rate` and the algebra `(V1−2V2+V3)/4` with result key `resistive_voltage`. Keep the waveform normalized and static.

Caption: “Keep sense polarity fixed while reversing force current. Equally spaced +I, −I, +I readings cancel constant and linear thermal EMF after settling; curved drift and self-heating remain.”

## 3. `measurement.compensated_probe_loading`

### Purpose, evidence and derived model

[University of Cincinnati, ECECS 349 Measurements Lab supplementary notes](https://homepages.uc.edu/~ferendam/Courses/EE_349/Scope_PROBE.htm), Figures 1–3, defines the probe as Rp parallel Cp, scope as Ri parallel scope capacitance, and coax cable as additional parallel capacitance. Its text states the compensated 10× divider becomes independent of frequency. [Tektronix, ABCs of Probes Primer](https://www.tek.com/en/documents/whitepaper/abcs-probes-primer), “Zero Signal Source Loading,” “Compensate Your Probes” and input-impedance discussion, documents finite loading, over/under compensation and frequency-dependent tip impedance.

The finite real source resistance is an independent extension of those explicit circuit models. With `tp=Rp Cp`, `ti=Ri Ci`, `Zp=Rp/(1+s tp)` and `Zi=Ri/(1+s ti)`:

`H(s)=Zi/(Rs+Zp+Zi)=Ri(1+s tp)/(A+s B+s² C)`.

Here `A=Rs+Rp+Ri` Ω, `B=Rs(tp+ti)+Rp ti+Ri tp` Ω·s, and `C=Rs tp ti` Ω·s². At `s=jω`, the denominator has real part `A−ω²C` and imaginary part `ωB`. Magnitude follows from these two orthogonal parts. The JSON normalizes both by A and uses `hypot`, avoiding raw squares of resistance-weighted terms.

`matched_probe_capacitance = Ri Ci/Rp` F makes `tp=ti`. `dc_gain=Ri/A` is dimensionless. `ac_gain=|H(jω)|`. `scope_voltage=source_voltage*ac_gain` is the voltage actually reaching the scope connector before its 10× display scaling.

At matched time constants τ, cancellation gives `H(s)=Ri/[A+s Rs τ]`. Thus matching preserves the tip-to-scope ratio but cannot eliminate source loading. This is the essential distinction from a renamed resistive divider.

The model is lumped, not a broadband prediction for a long probe cable. Use only where cable electrical length, lead inductance and scope input-amplifier response are negligible. Those facts cannot be inferred from the scalar R/C entries, so no arbitrary maximum frequency is claimed. Cp=0 explicitly describes an uncompensated probe; frequency=0 and Rs=0 are valid limits.

### Complete independent example

Inputs: `source_voltage=1 V peak`, `source_resistance=1000 Ω`, `probe_resistance=9e6 Ω`, `probe_capacitance=10e-12 F`, `scope_resistance=1e6 Ω`, `scope_capacitance=90e-12 F`, `frequency=1e6 Hz`.

Both arm time constants equal `90e-6 s`. Therefore A=10,001,000 Ω and `ω Rs τ = 565486.6776461628 Ω`. Independently use the matched circuit reduction, not the catalog's general expression:

- `matched_probe_capacitance = 90e-12*(1e6/9e6) = 1e-11 F`.
- `dc_gain = 1e6/10001000 = 0.0999900009999 V/V`.
- `ac_gain = 1e6 / sqrt(10001000² + 565486.6776461628²) = 0.09983054361596318 V/V`.
- `scope_voltage = 1*ac_gain = 0.09983054361596318 V peak`.

A separate complex-impedance evaluation `Zi/(Rs+Zp+Zi)` produced the same independent oracle in Python. This was numerical research, not application execution.

### Tester handoff

- Nominal: assert all four outputs.
- Set frequency=0: AC gain equals DC gain and output is 0.0999900009999 V; matched capacitance stays 10 pF.
- Set source resistance=0 with matched arms: DC gain=AC gain=0.1, scope voltage=0.1 V, independently of frequency within lumped validity.
- Under-compensated test: Rs=0, Cp=5 pF, others nominal. Matched capacitance remains 10 pF, DC gain=.1, AC gain and output=0.05263181693942273.
- Over-compensated test: Rs=0, Cp=20 pF, others nominal. Matched capacitance=10 pF, DC gain=.1, AC gain and output=0.1818180179402501. Do not clamp AC gain to DC gain.
- Cp=0, others nominal: AC gain and output=0.00019646537529400082; valid uncompensated limit.
- Source voltage=0 yields scope voltage=0 without changing gains. Reject negative source voltage, resistance, capacitance or frequency; Rp, Ri and Ci must be strictly positive.

### Mandatory diagram specification

Use the entire circuit width with an ideal source at x≈60, series Rs at x≈160, probe tip at x≈240, scope node at x≈480, and grounded return y≈230. From probe tip to scope node draw two parallel paths: Rp at y≈100 and Cp at y≈170, with junction dots at both ends. Neither is a ground shunt. From scope node to the return draw two distinct parallel branches Ri and Ci. Break wires at all part bodies. Put a separate output arrow from the scope node to the return, + at the node, bound to `scope_voltage`.

Quantity bindings: source=`source_voltage`; series Rs=`source_resistance`; probe resistor=`probe_resistance`; bridging capacitor=`probe_capacitance`; shunt resistor=`scope_resistance`; shunt capacitor=`scope_capacitance`. Frequency label uses `frequency`. A small compensation callout uses `matched_probe_capacitance` with the explicit relation `Rp Cp = Ri Ci`; gain quantities can remain in the accessible legend.

Caption: “Two parallel-RC arms form the compensated divider. Ci includes lumped cable capacitance. Matching the arm time constants does not remove finite-Rs source loading; the scope output is before readout scaling.”

## 4. `measurement.rectangular_aperture_response`

### Purpose, source and exact integral

[NI, Adjusting the Reading Rate of a DMM](https://www.ni.com/en/support/documentation/supplemental/07/adjusting-the-reading-rate-of-a-dmm.html), “DMM Measurement Cycle” and “Aperture Time,” states that readings during the aperture are averaged and that choosing whole mains periods rejects 50/60 Hz interference. The source distinguishes aperture from settling and notes real hardware's reading-rate tradeoffs.

For a causal, uniformly weighted continuous average:

`y(t)=(1/T) integral[t−T,t] x(u) du`.

Substitute a complex sinusoid `exp(j 2π f u)` and integrate:

`H(f)=exp(−jπ f T) sin(π f T)/(π f T)`.

The integral's DC limit is exactly 1; the T→0 limit is instantaneous observation. The signed multiplier at the midpoint is `midpoint_gain=sinc(π f T)`; magnitude is `amplitude_ratio=abs(midpoint_gain)`; `measured_peak=input_peak*amplitude_ratio`; the result corresponds to `effective_delay=T/2` before its end timestamp.

This is exact for the specified rectangular averaging model, not every DMM's proprietary weighting/filter chain. Analog input bandwidth and discrete ADC details remain outside scope. Negative sinc lobes invert polarity; an end-stamped phase is not simply −πfT without accounting for that inversion. There is deliberately no dB output, preserving true nulls.

### Complete independent example

Inputs: `frequency=25 Hz`, `aperture_time=.01 s`, `input_peak=.2 V`. Here `x=π/4`, so:

| Output key | Exact expression | Expected |
|---|---|---|
| `midpoint_gain` | (sqrt(2)/2)/(π/4) | 0.9003163161571061 V/V |
| `amplitude_ratio` | abs(midpoint_gain) | 0.9003163161571061 V/V |
| `measured_peak` | .2 × midpoint_gain | 0.18006326323142122 V |
| `effective_delay` | .01/2 | 0.005 s |

### Tester handoff

- Nominal: assert all four outputs.
- DC: frequency=0, nominal T and amplitude: `{midpoint_gain:1,amplitude_ratio:1,measured_peak:.2,effective_delay:.005}`.
- Instantaneous limit: T=0, nominal frequency and amplitude: `{midpoint_gain:1,amplitude_ratio:1,measured_peak:.2,effective_delay:0}`. Both f=T=0 are valid.
- Half cycle: f=50 Hz, T=.01 s: signed and magnitude gain `2/π=0.6366197723675814`, peak=.12732395447351627 V, delay=.005 s.
- Whole cycle: f=100 Hz, T=.01 s: mathematical gains and peak are zero, delay=.005 s. Floating `sin(pi)` leaves a tiny residual; use a justified absolute tolerance near 1e-15 on dimensionless gain, not a demand for exact trigonometric zero or an epsilon-clamped implementation.
- Negative lobe: f=150 Hz, T=.01 s: signed gain `−2/(3π)=−.2122065907891938`, magnitude=.2122065907891938, peak=.04244131815783876 V, delay=.005 s.
- Zero amplitude keeps the response and delay but gives zero output amplitude. Reject any negative input; nonfinite rejection remains shared.

### Mandatory diagram specification

Time-domain waveform with a highlighted rectangular aperture from `t−T` to t. Show a sinusoid crossing the aperture and a horizontal integral-average indicator. Mark t−T/2 with a dot; show an arrow from midpoint to t labeled `effective_delay`. Aperture bracket binds `aperture_time`; input peak bracket binds `input_peak`; output amplitude callout binds `measured_peak`; the sinusoid label binds `frequency`. A right-hand inset shows an illustrative signed sinc lobe crossing zero, with midpoint multiplier bound to `midpoint_gain` and magnitude bound to `amplitude_ratio`. Do not bind a live numerical value to a static point on the curve.

Caption: “Uniform integration reports the aperture average at its end; the average corresponds to its midpoint. Whole-cycle sinusoids cancel, and negative sinc lobes invert the midpoint-referenced signal. Curves are explanatory, not computed.”

## 5. `measurement.sampling_jitter_snr`

### Purpose, primary evidence and domain

[Texas Instruments SNAP004, PLL Fundamentals Part 4: PLL Clocking Applications](https://www.ti.com/lit/pdf/snap004), pages 18–20, describes sampling jitter and derives RMS voltage error from the time derivative of a sinusoid. Page 7 also discusses integration bandwidth. The calculation consumes a single total RMS sampling-time uncertainty, not a second generic independent-noise sum.

For `v(t)=A sin(2πft)` and small time error δt, `δv≈2π f A cos(2πft) δt`. With zero-mean jitter independent of phase and RMS σt:

- `phase_jitter = 2π f σt` rad.
- `error_rms = A*phase_jitter/sqrt(2)` V.
- `jitter_snr = 20 log10[(A/sqrt(2))/error_rms] = −20 log10(2π f σt)` dB.

The catalog calculates the logarithm as the sum of logarithms, avoiding premature underflow of a positive small product. It does not manufacture a finite SNR at exactly zero frequency, jitter or signal amplitude.

The physical first-order approximation is explicitly bounded here by RMS input phase error ≤0.1 rad. For Gaussian timing error, the exact RMS signal-error ratio is `sqrt(2*(1−exp(−phase_jitter²/2)))`; comparison at 0.1 rad gives linearization overestimate 0.001250260091084865, or about 0.125%. This error estimate is specific to Gaussian jitter, while the linear result only uses the variance. Large rare non-Gaussian excursions may violate the approximation even with a small RMS. The model is not a quantization SNR, ENOB estimate or promise of ADC performance.

### Complete independent example

Inputs: `frequency=10e6 Hz`, `timing_jitter=2e-12 s RMS`, `input_peak=1 V`.

`phase_jitter = 2π*(10e6)*(2e-12) = .00012566370614359174 rad`.

The sinusoid's RMS slope is `2π*10e6/sqrt(2)=44428829.38158366 V/s`; multiplying by 2 ps gives `error_rms=.00008885765876316733 V`. Dividing 1/sqrt(2) V signal RMS by that error and taking 20 log10 gives `jitter_snr=78.01580271955808 dB`.

### Tester handoff

- Nominal: assert all three outputs.
- Double only frequency to 20 MHz: phase=.0002513274122871835 rad, error=.00017771531752633465 V, SNR=71.99520280627846 dB. Doubling only jitter has the same effect.
- Double only input amplitude: phase and SNR unchanged, error doubles. This distinguishes jitter SNR from an additive fixed-voltage noise model.
- At the 0.1-rad boundary: error=A*.1/sqrt(2), SNR=20 dB. Values just above the bound are rejected; values below it remain valid.
- Positive arbitrarily small jitter is physically admitted; no arbitrary time floor. SNR rises without a hard clamp. Guard against avoidable log-product underflow.
- Reject frequency ≤0, jitter ≤0, amplitude ≤0, or phase product >0.1. Negative/zero jitter is not a meaningful finite-SNR case.

### Mandatory diagram specification

Draw one increasing portion of a sinusoid with a nominal sample dot. Place a horizontal timing-error bracket centered on the ideal instant, labeled `timing_jitter` with explicit “RMS, not peak-to-peak.” Project nearby sample instants vertically to illustrate corresponding voltage errors. Bind sine amplitude to `input_peak`, sine frequency to `frequency`; bind a separate RMS error annotation to `error_rms`. A slope tangent is labeled `dv/dt`, not numerically equated to `error_rms`. Right-hand equation callout: `σφ=2πfσt` binds `phase_jitter`; lower callout binds `jitter_snr`.

Caption: “Sampling-time variation converts sinusoidal slope into voltage noise. Brackets illustrate uncertainty, not bounded worst-case jitter; the SNR excludes other ADC errors and requires small phase error.”

## 6. `measurement.correlated_voltage_ratio`

### Purpose, evidence and stable propagation

[NIST TN 1297 Appendix A](https://www.nist.gov/pml/nist-technical-note-1297/nist-tn-1297-appendix-law-propagation-uncertainty), equations A-1 through A-3, defines a measurement process and first-order covariance propagation. [NIST section 6](https://www.nist.gov/pml/nist-technical-note-1297/nist-tn-1297-6-expanded-uncertainty), 6.1–6.3, defines `U=k uc` and explains why k alone does not guarantee coverage. The section 6 web page is marked no longer updated; these algebraic definitions are used, not a claim of current certification policy.

For positive voltage estimates Va and Vb and ratio `r=Va/Vb`, sensitivities are `∂r/∂Va=1/Vb` and `∂r/∂Vb=−Va/Vb²`. With covariance `rho ua ub`, NIST A-3 gives:

`u(r)²=(ua/Vb)²+(Va ub/Vb²)²−2 rho Va ua ub/Vb³`.

Define `a=ua/Va`, `b=ub/Vb`. Then `u(r)/r=sqrt(a²+b²−2 rho a b)`. Direct subtraction is poorly conditioned for nearly common-mode uncertainty. Use the identical nonnegative form:

`relative_standard_uncertainty = hypot(a−b, sqrt(2*(1−rho))*sqrt(a)*sqrt(b))`.

Square roots are factored to avoid forming the product `a*b` prematurely. The allowed rho interval [−1,1] makes the covariance matrix positive semidefinite; ua and ub are nonnegative. Zero ua or ub is valid, with correlation then immaterial to the result. Positive voltages avoid denominator singularity and sign ambiguities in relative standard uncertainty.

**Approximation scope:** both relative standard uncertainties are restricted to ≤1% as an explicit first-order engineering domain. This ceiling is not prescribed by NIST and is not a universal guarantee of accuracy for an arbitrary probability distribution. Ratio distributions with appreciable denominator probability near zero need a distribution-aware or Monte Carlo analysis, not this calculator. Even within the domain, systematic bias and higher-order terms can matter at stringent metrology targets. No claim of exact 95% coverage or standards compliance is made.

### Complete independent example

Inputs: `numerator_voltage=5 V`, `denominator_voltage=2.5 V`, `numerator_uncertainty=.002 V`, `denominator_uncertainty=.0015 V`, `correlation=.8`, `coverage_factor=2`.

Thus a=.0004, b=.0006 and r=2. Direct covariance arithmetic, independent of the catalog's stable identity:

`u_rel²=(.0004)²+(.0006)²−2*.8*.0004*.0006 = 1.36e-7`.

| Output key | Expected |
|---|---|
| `voltage_ratio` | 2 V/V |
| `relative_standard_uncertainty` | 0.00036878177829171557 |
| `ratio_standard_uncertainty` | 0.0007375635565834311 V/V |
| `expanded_uncertainty` | 0.0014751271131668623 V/V |

Multiplying the first-order standard uncertainty by two does not itself establish a coverage probability.

### Tester handoff

- Nominal: assert all four outputs.
- rho=0, other values nominal: r=2, relative u=.0007211102550927978, standard u=.0014422205101855955, expanded u=.002884441020371191.
- rho=−1: r=2, relative u=.001, standard u=.002, expanded u=.004.
- rho=+1: r=2, relative u=.0002, standard u=.0004, expanded u=.0008.
- Exact common fractional uncertainty: ua=.002 V, ub=.001 V, rho=1, nominal voltage estimates and k. Ratio=2 and all three uncertainty outputs=0. This is genuine cancellation in the model, not a clamped negative radicand.
- Near-perfect correlation with equal a=b: relative u equals `a*sqrt(2*(1−rho))` and must stay positive for rho<1. Test one representable step below 1 with high-precision independent arithmetic.
- Both uncertainties zero are valid for any allowed rho and return zero uncertainty. k scales expanded uncertainty only.
- Reject rho outside [−1,1], negative uncertainties, nonpositive voltages or k, and relative standard uncertainty above the declared 1% first-order scope. Test exactly 1% as accepted.

### Mandatory diagram specification

Two positive DC nodes feed separate ideal voltmeter/channel blocks labeled Va and Vb; their common return is explicit. Bind their values to `numerator_voltage` and `denominator_voltage`, and uncertainty brackets to `numerator_uncertainty` and `denominator_uncertainty`. Above both blocks, a shared calibration/reference influence branches into both measurement paths. This is a dashed influence arrow, not a physical short; bind it to `correlation` with the label “error correlation ρ.” Channel outputs feed a ratio block labeled `Va/Vb`, bound to `voltage_ratio`.

A lower annotation shows the negative covariance term `−2ρab` and a result bracket bound to `ratio_standard_uncertainty`. Follow it by a separate multiplication arrow `× k`, with `coverage_factor`, terminating at `expanded_uncertainty`. Keep `relative_standard_uncertainty` in the legend if needed. Do not depict the uncertainty brackets as a known probability distribution.

Caption: “Shared reference errors can cancel in a ratio because numerator and denominator sensitivities have opposite signs. Correlation is a property of the measurement errors, not a wire. Expanded uncertainty needs separately justified coverage.”

## Integration and acceptance handoff

Only `research/expansion/measurement.json` and this report are owned deliverables; the root communication bus is append-only coordination. Parent owns allocation-board updates. Developers must promote all six researched models, preserve all 128 baseline items, add exactly one diagram per ID through existing primitives, and retain the documented signs, units, guards and limits.

Parent should exercise every complete nominal map, the alternate/limiting cases above, invalid-domain recovery and native quantity labels. The aperture helper must land before its catalog is evaluated. Model formulas and standalone arithmetic were researched; no backend evaluation, schema verification, automated test, native image or build result is claimed. Remaining risks are declared physical approximations and the pending shared helper, not missing source evidence or omitted calculations.
