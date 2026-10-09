# Digital logic and interfaces — executable calculators

These models execute in the shared Rust engine. Flutter provides illustrations and catalog-driven inputs, not a second numerical implementation. Definitions are in [the logic catalog](../../backend/catalog/logic.json); diagrams use the shared primitives. Main owns diagram registration and execution acceptance.

The [research handoff](../../research/expansion/logic.md) retains primary-source access notes and independent derivations. Sources were read during research, not reverified by this implementation. Intel AN42 returned HTTP 403, the Washington lecture required login, and an old EPFL record returned HTTP 405; none supplies claimed evidence here. The accessible author-hosted Network Calculus book supplies the FIFO model.

Every example below enumerates all inputs and outputs with nominal relative tolerance 1e-6. Example inputs are defaults. The engine validates finite inputs, declared bounds and cross-input constraints before sequential outputs. Negative diagnostic margins remain valid. These are engineering estimates, not component qualification, CDC certification or guarantees of hardware reliability. No build, test, formatter, backend process or native UI check is claimed by this implementation.

## 1. `logic.guaranteed_noise_margins` — basic

### Purpose and novelty

Check whether a guaranteed output-high/low pair remains recognizable after ground offset and worst-direction line noise. Nearest baseline: `timing.schmitt_rc_delay_bounds`, which predicts threshold-crossing time through an RC network. This model instead compares static guaranteed voltage regions between two ICs; it has no capacitor, RC time or switching threshold prediction. Nearest new model: current fanout below; valid voltage margins do not establish available drive current and vice versa.

### Model and limits

From TI SZZA036C section 5/Figure 50 and SCAA029 Figure 23, the basic margins are VOH−VIH and VIL−VOL. Define ΔG = receiver ground minus driver ground. A driver-referenced voltage becomes V−ΔG at the receiver. Positive ΔG therefore hurts the high state and helps the low state:

- `high_margin = output_high_min - ground_offset - input_high_min - high_noise`.
- `low_margin = input_low_max - output_low_max + ground_offset - low_noise`.
- `minimum_margin = min(high_margin, low_margin)`.

All quantities are volts. Both noise inputs are nonnegative worst-direction magnitudes, not RMS random-noise values. `ground_offset` is signed. Output and input high/low pairs must each be ordered; negative calculated margins are accepted diagnostic results, not input errors. Zero means no remaining allowance.

Use guaranteed limits at compatible supply, temperature and load corners. A positive recognition margin does not establish safe overvoltage, undershoot, power-off behavior, fanout or timing. No generic family table substitutes for a real part's guarantees. This is positive single-ended logic; differential receivers require a different model.

### Complete independent example

Inputs: `output_high_min=2.4 V`, `output_low_max=0.4 V`, `input_high_min=2 V`, `input_low_max=0.8 V`, `ground_offset=0.1 V`, `high_noise=0.1 V`, `low_noise=0.1 V`.

- `high_margin = 2.4−0.1−2−0.1 = 0.2 V`.
- `low_margin = 0.8−0.4+0.1−0.1 = 0.4 V`.
- `minimum_margin = min(0.2,0.4) = 0.2 V`.

### Independent test cases

- Nominal: every result above, relative tolerance 1e-6.
- Equal references and no noise: set offset and both noise inputs to zero; both margins and minimum are 0.4 V.
- Ground-sign check: retain nominal noise and set offset to −0.1 V; high=0.4 V, low=0.2 V, minimum=0.2 V.
- Exact zero boundary with binary-exact values: VOH=2.5, VOL=0.5, VIH=2, VIL=1, offset=0.25, high_noise=0.25, low_noise=0.25 gives high=0, low=0.5, minimum=0 V.
- Incompatible but valid: nominal inputs with offset=0.4 V give high=−0.1 V, low=0.7 V and minimum=−0.1 V. Do not clamp or reject negative results.
- Invalid: negative noise magnitude, negative guaranteed level, VOH≤VOL, or VIH≤VIL. Reject nonfinite inputs through the existing engine boundary.

### Illustration and accessible quantity bindings

One 700×280 voltage-recognition diagram; use the shared primitives. Left block “Driver” and right block “Receiver” connect through one signal arrow. Under each block show its own ground reference, not a common-ground wire. Draw an offset arrow from driver ground to receiver ground, with + at receiver and − at driver; bind `ground_offset`.

At the center/right, draw two separated schematic voltage ladders, one for high recognition and one for low recognition. Bound labels: “VOH min” → `output_high_min`, “VIH min” → `input_high_min`, “VOL max” → `output_low_max`, “VIL max” → `input_low_max`; margin callouts “High headroom” → `high_margin`, “Low headroom” → `low_margin`. Show adverse high-noise arrow downward and low-noise arrow upward; label those arrows with `high_noise` and `low_noise`. These arrows represent reference shifts and noise budgets, not actual time-domain waveforms. Reserve label boxes outside ladder lines.

Caption: “Guaranteed recognition limits, not switching thresholds. Receiver ground offset subtracts from the received voltage. Static ladders are illustrative; negative reported headroom means an incompatible corner.”

### Catalog input units and bounds

| Key | Unit | Default | Bounds |
|---|---|---|---|
| `output_high_min` | V | `2.4` | >= 0 |
| `output_low_max` | V | `0.4` | >= 0 |
| `input_high_min` | V | `2.0` | >= 0 |
| `input_low_max` | V | `0.8` | >= 0 |
| `ground_offset` | V | `0.1` | Finite, signed |
| `high_noise` | V | `0.1` | >= 0 |
| `low_noise` | V | `0.1` | >= 0 |

Cross-input constraints:
- `output_high_min > output_low_max` — Driver high guarantee must exceed its low guarantee.
- `input_high_min > input_low_max` — Receiver high threshold must exceed its low threshold.

### Sources

- [Texas Instruments SZZA036C, section 5 and Figure 50: logic compatibility](https://www.ti.com/lit/an/szza036c/szza036c.pdf)
- [Texas Instruments SCAA029, Figure 23: DC noise margin](https://www.ti.com/lit/pdf/scaa029)

