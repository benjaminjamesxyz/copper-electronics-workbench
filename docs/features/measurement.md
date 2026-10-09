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

