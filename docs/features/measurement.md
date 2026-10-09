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

