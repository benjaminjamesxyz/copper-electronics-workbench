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

## 3. `rf.free_space_link_budget`

### Purpose, novelty and evidence

The nearest baseline `noise.friis_three_stage` concerns amplifier **noise factors**, not electromagnetic propagation; the common historical surname is not the same physical equation. `signals.line_propagation` predicts guided-line delay and wavelength, not antenna spatial spreading. This item calculates received radio power and sensitivity headroom. S3 supplies free-space loss; S7's radiation density and effective aperture supply the directional-gain extension.

### Equations, references and physical limits

`FSPL = 20 log10(4 pi d f/c)` dB.
`received_power_dbm = transmit_power_dbm + transmit_gain_dbi + receive_gain_dbi - total_loss_db - FSPL`.
`link_margin_db = received_power_dbm - sensitivity_dbm`.

The catalog sums logarithms instead of first forming a large `d*f` product. Gains are in dBi in the relevant direction. Loss is a nonnegative total additive budget, not a second subtraction of loss already included in gain. Sensitivity is the actual receiver's threshold for the modulation, rate and error criterion used.

Positive frequency, separation and maximum antenna dimension are required. With `D = antenna_max_dimension`, enforce `distance >= max(10c/f, 2D^2 f/c)`. The aperture term comes from S8. The `10 lambda` term is a disclosed conservative screen for the electrically-small-antenna case where the aperture term alone could lie in the near field. It makes `kr >= 20 pi`; it is not a universal proof of far-field accuracy. The complete path must also be clear and polarization matched. Numerical checks cannot infer terrain or actual patterns from these inputs. Free-space power does not establish regulatory permission, link availability or interference tolerance.

### Complete independent example

Inputs: `frequency=2400000000 Hz`, `distance=1000 m`, `antenna_max_dimension=0.1 m`, `transmit_power_dbm=20`, `transmit_gain_dbi=2`, `receive_gain_dbi=2`, `total_loss_db=2`, `sensitivity_dbm=-90`.

`lambda = 0.12491352416666667 m`; the aperture screen is `0.160110765695113 m` and `10 lambda = 1.2491352416666667 m`; the 1000 m separation passes.

| Output key | Independent derivation | Expected |
|---|---|---:|
| `free_space_loss_db` | `20 log10(4 pi * 1000 / 0.12491352416666667)` | `100.0520080561155 dB` |
| `received_power_dbm` | `20 + 2 + 2 - 2 - 100.0520080561155` | `-78.0520080561155 dBm` |
| `link_margin_db` | `-78.0520080561155 - (-90)` | `11.947991943884503 dB` |

### Nominal, limiting and invalid cases

- Keep all nominal values but double distance: loss=`106.07260796939512 dB`, receive=`-84.07260796939512 dBm`, margin=`5.927392030604879 dB`. This is the independent inverse-square law.
- Increase either gain by 3 dB: receive and margin rise 3 dB; path loss does not change.
- Set sensitivity equal to the calculated received power: zero margin is valid. A higher sensitivity threshold produces a valid negative margin.
- `total_loss_db=0` is valid. Negative directional dBi is valid and must not be clamped.
- Test exact far-field-screen equality and one representable step below; reject only the latter. Reject nonpositive frequency/distance/dimension, negative loss and nonfinite inputs.

### Catalog inputs and bounds

| Key | Meaning | Unit | Default | Bounds |
|---|---|---|---|---|
| `frequency` | Carrier frequency | Hz | 2400000000.0 | > 0.0 |
| `distance` | Antenna separation | m | 1000.0 | > 0.0 |
| `antenna_max_dimension` | Largest dimension of either antenna | m | 0.1 | > 0.0 |
| `transmit_power_dbm` | Equipment transmit power | dBm | 20.0 | Any finite value |
| `transmit_gain_dbi` | Transmit antenna directional gain | dBi | 2.0 | Any finite value |
| `receive_gain_dbi` | Receive antenna directional gain | dBi | 2.0 | Any finite value |
| `total_loss_db` | Total additional link loss | dB | 2.0 | >= 0.0 |
| `sensitivity_dbm` | Receiver sensitivity | dBm | -90.0 | Any finite value |

All inputs must be finite. Constraints run before outputs; invalid domains return errors, not clamps.

- `distance >= max(10*299792458/frequency,2*antenna_max_dimension*(antenna_max_dimension*frequency/299792458))`: Separation must meet the documented far-field screen for frequency and antenna size.

### Executable output sequence

| Key | Unit | Expression |
|---|---|---|
| `free_space_loss_db` | dB | `20*(log10(4*pi)+log10(distance)+log10(frequency)-log10(299792458))` |
| `received_power_dbm` | dBm | `transmit_power_dbm+transmit_gain_dbi+receive_gain_dbi-total_loss_db-free_space_loss_db` |
| `link_margin_db` | dB | `received_power_dbm-sensitivity_dbm` |

Nominal relative tolerance is `1e-9`, not physical accuracy. Expected zero outputs require an absolute tolerance. All defaults equal the full worked-example inputs above.

### Sources and illustration

- [Wurth Elektronik ANR010 Range Estimator v1.1, section 4.1, equations 4-7](https://www.we-online.com/components/media/o164527v410%20ANR010_Range_Estimation.pdf)
- [MIT Lincoln Laboratory, Introduction to Radar Systems: The Radar Equation, slides 5-8 (gain and effective aperture)](https://www.ll.mit.edu/sites/default/files/outreach/doc/2018-07/lecture%202.pdf)
- [Amrita Virtual Lab, Antenna Gain Measurement, procedure 1 (2D squared over wavelength)](https://vlab.amrita.edu/?brch=180&cnt=1&sim=307&sub=3)

The exact-ID diagram in [rf.dart](../../frontend/lib/diagrams/rf.dart) binds quantity keys to the accessible legend. Static geometry is explanatory, not a computed plot.

## 4. `rf.first_fresnel_clearance`

### Purpose, novelty and evidence

The nearest neighbor is the free-space link budget, but this item solves **path-interference clearance geometry**, not power spreading or an inverse range budget. No baseline model computes an ellipsoidal Fresnel region. `signals.line_propagation` supplies only wavelength/delay. S3 section 3 provides the radius expression and warns about points close to either endpoint.

### Equations, signs and limits

With `d1 = distance_from_tx`, `d2 = distance_to_rx`, and `lambda = c/f`:

`zone_radius = sqrt(lambda d1 d2/(d1+d2))` m;
`required_clearance = clearance_fraction * zone_radius` m;
`clearance_margin = available_clearance - required_clearance` m.

Expanding the two off-axis ray lengths gives path excess `r^2/(2d1) + r^2/(2d2)`. Setting that to `lambda/2` derives the first-zone formula independently. `available_clearance` is the signed perpendicular gap between line of sight and the closest obstacle point: positive is clear, negative means the obstacle crosses line of sight. Negative margin is a useful design failure, not invalid input.

The explicit paraxial domain is `d1,d2 >= 100 lambda`. This is a local engineering accuracy screen derived from `r/di <= sqrt(lambda/di) <= 0.1`, not an arbitrary range clamp or a manufacturer-mandated value. The first neglected term in each square-root ray-length expansion is at most approximately 0.25% of its quadratic term at that boundary. More exact ellipsoid/terrain models are needed closer to an endpoint. The fraction is a chosen planning rule in [0,1]; the 0.6 default is not asserted to eliminate diffraction loss. Earth curvature and atmospheric refraction must be accounted for before entering actual clearance.

The catalog evaluates `d1*d2/(d1+d2)` as `min(d1,d2)/(1+min(d1,d2)/max(d1,d2))` and factors the square root to avoid needless product overflow.

### Complete independent example

Inputs: `frequency=2400000000 Hz`, `distance_from_tx=400 m`, `distance_to_rx=600 m`, `clearance_fraction=0.6`, `available_clearance=4 m`.

The effective distance is `400*600/1000 = 240 m`; each segment exceeds the `12.491352416666667 m` paraxial screen.

| Output key | Independent derivation | Expected |
|---|---|---:|
| `zone_radius` | `sqrt(0.12491352416666667 * 240)` | `5.475330656681841 m` |
| `required_clearance` | `0.6 * 5.475330656681841` | `3.2851983940091043 m` |
| `clearance_margin` | `4 - 3.2851983940091043` | `0.7148016059908957 m` |

### Nominal, limiting and invalid cases

- Swap d1 and d2: every output remains unchanged.
- Hold total length 1000 m and use d1=d2=500 m: radius=`5.588235950786855 m`, the maximum along this link.
- Quadruple frequency: radius and required clearance halve; changing obstacle clearance alone changes only margin.
- `clearance_fraction=0` yields zero requirement and margin equal to available clearance. Fraction=1 requests the full first zone. Both are valid.
- Available clearance zero or negative is valid and shows obstruction. Screen equality is valid; one endpoint below 100 wavelengths is rejected.
- Reject nonpositive frequency/endpoint distances, fraction outside [0,1] and nonfinite values. A point at an antenna endpoint is outside this approximation, not silently reported as a zero-radius zone.

### Catalog inputs and bounds

| Key | Meaning | Unit | Default | Bounds |
|---|---|---|---|---|
| `frequency` | Carrier frequency | Hz | 2400000000.0 | > 0.0 |
| `distance_from_tx` | TX to obstacle-plane distance | m | 400.0 | > 0.0 |
| `distance_to_rx` | Obstacle-plane to RX distance | m | 600.0 | > 0.0 |
| `clearance_fraction` | Required fraction of first-Fresnel radius | 1 | 0.6 | >= 0.0; <= 1.0 |
| `available_clearance` | Signed perpendicular obstacle clearance | m | 4.0 | Any finite value |

All inputs must be finite. Constraints run before outputs; invalid domains return errors, not clamps.

- `distance_from_tx >= 100*299792458/frequency`: TX-to-obstacle distance must be at least 100 wavelengths for the documented paraxial screen.
- `distance_to_rx >= 100*299792458/frequency`: Obstacle-to-RX distance must be at least 100 wavelengths for the documented paraxial screen.

### Executable output sequence

| Key | Unit | Expression |
|---|---|---|
| `zone_radius` | m | `sqrt(299792458/frequency)*sqrt(min(distance_from_tx,distance_to_rx)/(1+min(distance_from_tx,distance_to_rx)/max(distance_from_tx,distance_to_rx)))` |
| `required_clearance` | m | `clearance_fraction*zone_radius` |
| `clearance_margin` | m | `available_clearance-required_clearance` |

Nominal relative tolerance is `1e-9`, not physical accuracy. Expected zero outputs require an absolute tolerance. All defaults equal the full worked-example inputs above.

### Sources and illustration

- [Wurth Elektronik ANR010 Range Estimator v1.1, section 3, equations 1-3 and wavelength-distance limitation](https://www.we-online.com/components/media/o164527v410%20ANR010_Range_Estimation.pdf)

The exact-ID diagram in [rf.dart](../../frontend/lib/diagrams/rf.dart) binds quantity keys to the accessible legend. Static geometry is explanatory, not a computed plot.

