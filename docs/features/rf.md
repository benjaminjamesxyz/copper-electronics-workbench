# RF networks, antennas and links

The [RF catalog](../../backend/catalog/rf.json) executes in the shared Rust engine. Flutter desktop supplies illustrative diagrams, not a second numerical engine. Physical models, defaults and complete independent examples follow the [RF research](../../research/expansion/rf.md) and [sprint acceptance cases](../../coordination/expansion/BACKLOG.md#rf--7-required-items).

SI units apply except explicit dB, dBi and dBm; dBm references 1 mW. The speed of light is exactly 299792458 m/s. Negative dBm and design margins remain valid. Nonfinite inputs/results are errors. Numerical applicability screens do not establish hardware accuracy, availability or compliance.

Source retrieval and independent example derivation belong to the research handoff, not to runtime verification. Research records inaccessible ADI/Utah/legacy Mini-Circuits material; none is claimed as verified here. Only the linked readable replacements support these models. No builds, tests, lint, formatters, process runs or native image checks were run by this implementation worker; Main owns those checks.

## 1. `rf.matched_t_attenuator`

### Purpose, novelty and evidence

Design a matched attenuator rather than solve a loaded divider. The nearest baseline is `dc.loaded_divider`, which computes voltage under one finite load but does not synthesize a two-port with simultaneous matching. `noise.decibel_to_linear` only converts ratios; `signals.source_series_termination` absorbs a traveling-wave boundary mismatch with one series element. This three-resistor dissipative synthesis is distinct from all three and from the lossless L match below. S1 provides the topology and equations.

### Equations and physical limits

Let `A = attenuation_db > 0`, `Z0 = reference_impedance > 0` and `K = 10^(A/20)`, the **voltage** attenuation ratio. S1's `K` is a power attenuation ratio, so its square root is this document's `K`.

- `series_resistance = Z0 (K-1)/(K+1)` ohm, used for each of the two series arms.
- `shunt_resistance = 2 Z0 K/(K^2-1)` ohm, from their junction to common return.
- `power_transmission = 1/K^2`, dimensionless, only with matched termination.

The catalog uses negative-exponential forms and `expm1` to preserve a small positive attenuation without subtracting nearly equal exponentials. Neither attenuation at a mismatched load nor resistor power ratings is inferred. At `A -> 0+`, the series arms approach zero and the shunt approaches infinity. Zero attenuation is deliberately excluded because a finite three-resistor pad no longer exists. No DC voltage polarity is needed; RF port power travels left to right.

### Complete independent example

Inputs: `reference_impedance = 50 ohm`, `attenuation_db = 20 dB`.

`K = 10`; therefore:

| Output key | Independent derivation | Expected |
|---|---|---:|
| `series_resistance` | `50 * 9/11` | `40.90909090909091 ohm` |
| `shunt_resistance` | `100 * 10/99` | `10.1010101010101 ohm` |
| `power_transmission` | `1/100` | `0.01` |

Independent physical check for the tester: `Rs + [Rp || (Rs + 50)] = 50 ohm`. An ABCD or nodal solution with the second port terminated in 50 ohm gives voltage transmission `0.1` and power transmission `0.01`; do not merely evaluate the production synthesis formula again.

### Nominal, limiting and invalid cases

- Alternate: `Z0=50`, `A=6.020599913279624 dB` gives `K=2`, `Rs=16.666666666666668 ohm`, `Rp=66.66666666666667 ohm`, and `power_transmission=0.25`.
- Small-loss conditioning: `A=1e-12 dB`, `Z0=50`; `Rs approximately 2.878231366242557e-12 ohm` and `Rp approximately 4.342944819032518e14 ohm`. Neither resistor may collapse to a false zero/infinity through `exp(x)-1` cancellation.
- Doubling `Z0` doubles both resistor outputs and leaves power transmission unchanged.
- Reject `A=0`, negative attenuation, zero/negative impedance and nonfinite values. Real resistor parasitics and ratings remain external checks, not hidden guarantees.

### Catalog inputs and bounds

| Key | Meaning | Unit | Default | Bounds |
|---|---|---|---|---|
| `reference_impedance` | Equal port reference impedance | ohm | 50.0 | > 0.0 |
| `attenuation_db` | Matched insertion loss | dB | 20.0 | > 0.0 |

All inputs must be finite. Constraints run before outputs; invalid domains return errors, not clamps.


### Executable output sequence

| Key | Unit | Expression |
|---|---|---|
| `series_resistance` | ohm | `reference_impedance*(-expm1(-ln(10)*attenuation_db/20))/(1+exp(-ln(10)*attenuation_db/20))` |
| `shunt_resistance` | ohm | `reference_impedance*exp(-ln(10)*attenuation_db/20)*2/(-expm1(-ln(10)*attenuation_db/10))` |
| `power_transmission` | 1 | `exp(-ln(10)*attenuation_db/10)` |

Nominal relative tolerance is `1e-9`, not physical accuracy. Expected zero outputs require an absolute tolerance. All defaults equal the full worked-example inputs above.

### Sources and illustration

- [Michael Steer, Microwave and RF Design III, section 5.5.2, equations 2, 7 and 8](https://eng.libretexts.org/Bookshelves/Electrical_Engineering/Electronics/Microwave_and_RF_Design_III_-_Networks_(Steer)/05%3A_Chapter_5/5.5%3A_Terminations_and_Attenuators)

The exact-ID diagram in [rf.dart](../../frontend/lib/diagrams/rf.dart) binds quantity keys to the accessible legend. Static geometry is explanatory, not a computed plot.

