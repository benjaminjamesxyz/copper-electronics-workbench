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

## 2. `rf.lowpass_l_match`

### Purpose, novelty and evidence

The nearest baseline `signals.quarter_wave_transformer` uses distributed phase propagation and a transmission-line section. This model instead synthesizes two **lumped** reactances whose parallel/series transformation cancels at the design frequency. `ac.series_rlc_impedance` evaluates a supplied circuit rather than synthesize a conjugate match. The new model is not a re-labeled quarter-wave section or a generic resonance calculation. S2, section 6.4.1, derives this exact orientation.

### Equations and physical limits

For `0 < source_resistance = Rs < load_resistance = Rl` and positive frequency `f`:

`loaded_q = sqrt((Rl-Rs)/Rs)`;
`series_inductance = Rs * loaded_q/(2 pi f)` H;
`shunt_capacitance = loaded_q/(2 pi f Rl)` F.

The input sees `j omega L + [Rl || 1/(j omega C)] = Rs`. The series inductive reactance is positive; the load-side capacitive reactance is negative. This is the low-pass/DC-passing orientation only. Both resistances must be real at the stated frequency. Equal resistances require no network; reversed ordering needs the other topology and is not silently accepted. Finite component Q, parasitic reactance and self-resonance shift the match. Loaded Q is not an exact return-loss bandwidth prediction.

### Complete independent example

Inputs: `source_resistance = 50 ohm`, `load_resistance = 200 ohm`, `frequency = 100000000 Hz`.

| Output key | Independent derivation | Expected |
|---|---|---:|
| `loaded_q` | `sqrt(200/50-1) = sqrt(3)` | `1.7320508075688772` |
| `series_inductance` | `86.60254037844386/(2 pi * 1e8)` | `1.3783222385544804e-7 H` |
| `shunt_capacitance` | `sqrt(3)/(2 pi * 1e8 * 200)` | `1.3783222385544801e-11 F` |

The shunt capacitor has reactance `-115.47005383792516 ohm`. The parallel load/capacitor equivalent is `50 - j86.60254037844386 ohm`; adding the inductor gives `50 + j0 ohm`. This complex circuit identity is an independent tester oracle.

### Nominal, limiting and invalid cases

- `Rs=50`, `Rl=100`, `f=1e8` gives Q=1, L=`7.957747154594767e-8 H`, C=`1.5915494309189536e-11 F`.
- Doubling frequency halves L and C without changing Q.
- As `Rl -> Rs+`, all three outputs approach zero. Use a representably adjacent `Rl > Rs` to check `(Rl-Rs)/Rs`, not cancellation in `Rl/Rs-1`.
- Reject `Rl=Rs`, `Rl<Rs`, nonpositive resistances/frequency and nonfinite values. Do not impose a minimum nonzero Q merely to simplify testing.

### Catalog inputs and bounds

| Key | Meaning | Unit | Default | Bounds |
|---|---|---|---|---|
| `source_resistance` | Lower source resistance | ohm | 50.0 | > 0.0 |
| `load_resistance` | Higher load resistance | ohm | 200.0 | > 0.0 |
| `frequency` | Match frequency | Hz | 100000000.0 | > 0.0 |

All inputs must be finite. Constraints run before outputs; invalid domains return errors, not clamps.

- `load_resistance > source_resistance`: This series-L, load-shunt-C topology requires load resistance greater than source resistance.

### Executable output sequence

| Key | Unit | Expression |
|---|---|---|
| `loaded_q` | 1 | `sqrt((load_resistance-source_resistance)/source_resistance)` |
| `series_inductance` | H | `source_resistance*loaded_q/(2*pi*frequency)` |
| `shunt_capacitance` | F | `loaded_q/(2*pi*frequency*load_resistance)` |

Nominal relative tolerance is `1e-9`, not physical accuracy. Expected zero outputs require an absolute tolerance. All defaults equal the full worked-example inputs above.

### Sources and illustration

- [Michael Steer, Microwave and RF Design III, section 6.4.1, equations 1-10](https://eng.libretexts.org/Bookshelves/Electrical_Engineering/Electronics/Microwave_and_RF_Design_III_-_Networks_%28Steer%29/06%3A_Chapter_6/6.4%3A_The_L_Matching_Network)

The exact-ID diagram in [rf.dart](../../frontend/lib/diagrams/rf.dart) binds quantity keys to the accessible legend. Static geometry is explanatory, not a computed plot.

