# Hardware measurement and metrology

Executable definitions: [Rust catalog](../../backend/catalog/measurement.json). Illustrations: [Flutter diagrams](../../frontend/lib/diagrams/measurement.dart). Rust owns all numerics; Flutter displays static explanatory diagrams, not calculated plots. Shared registration and execution belong to Main. No build, test, lint, formatter, process or native check was run for this implementation.

Sources and independent arithmetic below are retained from the [research handoff](../../research/expansion/measurement.md). They describe engineering models, not calibration, instrument guarantees or certification. Nominal output tolerance is 1e-6 relative; cancellation/null tests require the stated absolute tolerances.

## `measurement.dmm_accuracy_bound`

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


## `measurement.delta_resistance`

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


## `measurement.compensated_probe_loading`

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


## `measurement.rectangular_aperture_response`

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


## `measurement.sampling_jitter_snr`

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

