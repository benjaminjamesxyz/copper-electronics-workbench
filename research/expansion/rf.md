# RF expansion research: seven new hardware models

## Scope and handoff

Own `research/expansion/rf.json` and this file. The JSON array contains exactly seven models: two basic, three intermediate and two expert. Each has a complete independent numerical example and the diagram specification below. The baseline inventory, expansion allocation and shared bus were read before selection. No production code, tests, shared registry, research board or baseline model was changed.

| ID | Level | Physical design problem |
|---|---|---|
| `rf.matched_t_attenuator` | basic | Simultaneous equal-port matching and dissipative insertion-loss synthesis |
| `rf.lowpass_l_match` | intermediate | Lumped reactive transformation between unequal real impedances |
| `rf.free_space_link_budget` | basic | One-way spatial spreading and receiver power margin |
| `rf.first_fresnel_clearance` | intermediate | Obstruction geometry from path-length interference |
| `rf.circular_aperture_gain` | intermediate | Collecting aperture and directional gain from antenna geometry |
| `rf.two_tone_im3` | expert | Weak cubic nonlinearity and near-band intermodulation products |
| `rf.monostatic_radar_return` | expert | Target scattering and two-way echo-power budget |

No inverse, alternate units, minor topology variant or supporting output is counted separately. In particular, the three T resistors form one calculation; the two IM3 products form one calculation. The aperture and propagation calculators solve different design questions even though their outputs may feed each other.

## Search boundaries, sources and evidence quality

Research used English-language manufacturer, university-authored textbook, university laboratory and standards material, without a publication-date cutoff. Search terms covered matched T pads, L networks, free-space path loss, Fresnel radius, circular-aperture gain, two-tone IP3 and the radar equation. This is a bounded engineering-model review, not a systematic review of antenna technologies or propagation measurements. Online calculator sites and unsourced formula summaries were not used as evidence.

The following material was actually retrieved and read. Equation numbering refers to the source, not to this document.

- **S1 — University-authored teaching text:** Michael Steer, *Microwave and RF Design III — Networks*, [section 5.5.2, Terminations and Attenuators](https://eng.libretexts.org/Bookshelves/Electrical_Engineering/Electronics/Microwave_and_RF_Design_III_-_Networks_(Steer)/05%3A_Chapter_5/5.5%3A_Terminations_and_Attenuators). Equations 2, 7 and 8 establish the matched symmetric T pad, power ratio and dB loss. High confidence for ideal circuit algebra; no component/layout validation is implied. LibreTexts is the accessible host of the university author's text, not a second independent experimental source.
- **S2 — University-authored teaching text:** Steer, [section 6.4.1, The L Matching Network](https://eng.libretexts.org/Bookshelves/Electrical_Engineering/Electronics/Microwave_and_RF_Design_III_-_Networks_%28Steer%29/06%3A_Chapter_6/6.4%3A_The_L_Matching_Network). Equations 1–10 and Figures 3–4 explicitly derive the load-shunt/source-series form for `Rs < Rl`. High confidence for ideal single-frequency synthesis; finite-Q behavior is not characterized.
- **S3 — Manufacturer application note:** Würth Elektronik, [ANR010, Range Estimator, version 1.1, July 2023](https://www.we-online.com/components/media/o164527v410%20ANR010_Range_Estimation.pdf). Section 3, equations 1–3, gives Fresnel geometry and explicitly requires endpoint distances much larger than wavelength. Section 4.1, equations 4–7, gives free-space spatial loss and sensitivity budgeting. High confidence in the closed-form idealization, not in a real installation's range. The manufacturer explicitly warns against applying free-space range to obstructed/ground-reflected paths. PDF prose about Fresnel phases is not used; the radius equations and path-difference derivation below are the evidence.
- **S4 — Standards body:** [ITU-R RS.1813-1, February 2011](https://www.itu.int/dms_pubrec/itu-r/rec/rs/R-REC-RS.1813-1-201102-S!!PDF-E.pdf), recommends 1 and the `Gmax` definition on printed page 2. Gives `Gmax = 10 log10(eta pi^2 D^2/lambda^2)` and uses `D > 2 lambda`. High confidence for the stated aperture law. The complete recommendation is an Earth-exploration passive-sensor pattern model for 1.4–100 GHz; this calculator uses only the general aperture-gain relation and does not claim to implement that pattern or establish compliance.
- **S5 — Manufacturer technical reference:** Mini-Circuits, [Terms and Definitions: Amplifiers](https://blog.minicircuits.com/amplifier-terms-defined/), “Two-tone third-order intercept point” and “1 dB compression point.” Explicitly gives the 1:3 slopes, `2f1-f2` and `2f2-f1`, per-tone power convention, `OIP3 = Pout + A/2` and input/output intercept gain shift. High confidence for small-signal intercept extrapolation; part-specific validity requires characterization. Its claim that compression can begin 5–10 dB below P1dB is why P1dB is not treated as the supplied safe linear limit.
- **S6 — Manufacturer measurement guide:** Texas Instruments, [TRF1208 EVM guide SBOU266B, section 3.3](https://www.ti.com/document-viewer/lit/html/SBOU266B/GUID-95C24EAC-F2FA-4597-8D9E-02715CFD493B). Read the OIP3 equation, reference-plane loss correction, generator-isolation and analyzer-linearity guidance. Corroborates S5's intercept convention. The prose's “8 dBm total / 2 dBm per tone” example is not used as an average-power identity: two equal incoherent-in-frequency fundamentals sum to per-tone power plus `10 log10(2)`, not plus 6 dB. Peak-envelope and average power must not be conflated.
- **S7 — University laboratory course:** MIT Lincoln Laboratory, [Introduction to Radar Systems: The Radar Equation](https://www.ll.mit.edu/sites/default/files/outreach/doc/2018-07/lecture%202.pdf), slides 5–10 and 13. Read the outward power-density, RCS, return power-density and effective-aperture derivation. High confidence for deterministic far-field radar algebra. Detection probability, fluctuating RCS and integration behavior are not covered by the chosen model. Slide 6 independently supports `G = 4 pi Ae/lambda^2`, which also supplies directional gains for the one-way link.
- **S8 — University instructional laboratory:** Amrita, [Antenna Gain Measurement](https://vlab.amrita.edu/?brch=180&cnt=1&sim=307&sub=3), theory and procedure 1. Explicitly requires horn separation at least `2 D^2/lambda`. Used for the aperture far-field screen, not as empirical validation of all antenna types.

Access gaps: Analog Devices' matching article, IP3 article and MT-053 PDF timed out. The University of Utah scanned L-network PDF returned incomplete text. Mini-Circuits' legacy AN60-038 URL returned HTTP 403; its accessible manufacturer-hosted replacement was read instead. None of the inaccessible documents is cited as verified evidence. No source was found that makes a single range threshold sufficient for all antennas; the extra `10 lambda` screen below is deliberately labeled a conservative engineering choice, not a sourced universal law.

All numerical examples were derived independently with Python arithmetic in Eval, not by evaluating catalog expression strings or running the Rust engine. No build, tests, lint, formatting, process or UI checks were run. Parent owns implementation verification and independent tester execution.

## Shared conventions and illustration requirements

Use exact `c = 299792458 m/s`. SI units apply except explicitly labeled dB, dBi and dBm. dBm is referenced to 1 mW. Negative dBm and negative design margins are valid, not errors. All inputs must be finite; no artificial upper bounds are imposed on power, gain or ordinary component values. The existing engine must still reject unrepresentable nonfinite results.

Each diagram uses the existing primitives on a 700×280 canvas with 16 px minimum outer margin, labels at least 14 px, and explicit space between geometry and text. Import only `primitives.dart`; export exactly seven entries in `rfDiagrams`. Components interrupt wires at their actual pins. Connected branches have junction dots; unrelated crossings do not. Any curve, spectrum or radiation lobe described below is static explanatory geometry, never a claimed computed plot. `Label.quantity` binds only the exact keys listed for that model; arithmetic relation labels remain plain explanatory text unless an exact output key exists.

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

### Mandatory diagram

Place input port near `(55,105)` and output near `(590,105)`. Draw two horizontal resistors centered near `(215,105)` and `(440,105)`, with a junction at `(325,105)`. Draw one vertical resistor from that junction toward a return rail at `y=220`. Draw the output load between the output node and the same rail. Label both series arms `Rs` with `series_resistance`; label center branch `Rp` with `shunt_resistance`. Bind the output termination and a separate input-reference label to `reference_impedance`. Bind a top arrow `Loss A` to `attenuation_db` and a bottom-right text `Pout/Pin` to `power_transmission`. Wires must stop at resistor bodies; mark both branch junctions.

Caption: “Equal-reference T pad: both series resistors equal Rs. Power transmission applies with the output terminated in Z0; ideal lumped resistors only.”

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

### Mandatory diagram

Draw a source and source resistor on the left, series inductor across the upper path near `(315,95)`, and output junction near `(455,95)`. From that junction, draw parallel branches for a capacitor near `x=455` and load resistor near `x=590`, returning to `y=220`. Label the source resistor `Rs` with `source_resistance`; load `Rl` with `load_resistance`; inductor `L` with `series_inductance`; capacitor `C` with `shunt_capacitance`. Bind `frequency` at the input and `loaded_q` in a clear top-right annotation. The capacitor must be on the higher-resistance load side. An arrow above the input identifies “Zin = Rs at f” as a relation, not a new quantity key.

Caption: “Series L plus load-side shunt C transforms Rl down to Rs at one frequency. Ideal components; Rs < Rl; Q is fixed by the resistance ratio.”

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

### Mandatory diagram

Place TX equipment/antenna at left near `(95,130)` and RX antenna/equipment near `(595,130)`. Use a single left-to-right propagation arrow with spreading arcs that do not suggest a wire connection. Add a horizontal separation dimension line below. Bind TX `Pt` to `transmit_power_dbm`, its antenna gain to `transmit_gain_dbi`, RX gain to `receive_gain_dbi`, separation to `distance`, and a top carrier label to `frequency`. A separate annotation `Largest antenna size` binds `antenna_max_dimension`. Put `FSPL` / `free_space_loss_db` and `Other loss` / `total_loss_db` along separate clear arrow bands. At RX bind `Pr` / `received_power_dbm`, `Sensitivity` / `sensitivity_dbm`, and `Margin` / `link_margin_db`. Do not make arc spacing or antenna size appear numerically scaled.

Caption: “One-way far-field free-space link. Gains face each other; additional losses are subtracted once. Geometry is schematic; obstructions and fading are not modeled.”

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

### Mandatory diagram

Draw TX at `(55,110)` and RX at `(645,110)` with a horizontal dashed line of sight. Draw an explanatory ellipse between them, largest at midpoint. Put the obstacle plane at about `x=285`; draw an obstacle rising from below but stopping beneath the line. Dimension TX-to-plane as `distance_from_tx` and plane-to-RX as `distance_to_rx`. At the obstacle plane, show a vertical line from LOS to the lower ellipse boundary for `zone_radius`, and a distinct vertical gap from LOS to obstacle top for `available_clearance`; keep these arrows horizontally separated. Bind `frequency` above the ellipse, `clearance_fraction` and `required_clearance` in a clear right-side relation label, and `clearance_margin` below. The two distances are axial, not slant ray lengths.

Caption: “First Fresnel zone from a half-wavelength path excess. Clearance is perpendicular to line of sight; ellipse and obstacle are illustrative, not a computed terrain profile.”

## 5. `rf.circular_aperture_gain`

### Purpose, novelty and evidence

This model turns a physical aperture and efficiency into directional gain and collecting area. The baseline has no antenna-aperture synthesis. Its nearest neighbor `rf.free_space_link_budget` accepts gain and predicts propagation; it does not derive gain from geometry. `signals.coax_impedance` describes guided fields and is unrelated. S4 provides the circular-aperture gain relation; S7 provides the gain/effective-area identity.

### Equations and limits

For diameter `D`, aperture efficiency `eta`, and frequency `f`:

`effective_aperture = eta pi D^2/4` m²;
`gain_linear = eta (pi D f/c)^2`;
`gain_dbi = 10 log10(eta) + 20 log10(pi D f/c)`.

Require `D > 2c/f`, `f > 0` and `0 < eta <= 1`. The diameter criterion matches the large-aperture domain in S4. The general aperture relation is not restricted to S4's example service band, but it requires an actual efficiency appropriate to the operating frequency. Zero efficiency yields zero collecting power and negative-infinite dBi, so it is outside this finite-logarithmic model. Gains describe boresight, polarization-matched, conjugate-matched operation. No beamwidth, sidelobes, mismatch, radome or feedline behavior is inferred. This is not a patch/dipole antenna formula.

### Complete independent example

Inputs: `diameter=1 m`, `frequency=10000000000 Hz`, `aperture_efficiency=0.6`.

`lambda=0.0299792458 m`; physical circular area=`pi/4 m²`; diameter is about 33.36 wavelengths.

| Output key | Independent derivation | Expected |
|---|---|---:|
| `effective_aperture` | `0.6 * pi/4` | `0.47123889803846897 m^2` |
| `gain_linear` | `4 pi * 0.47123889803846897 / 0.0299792458^2` | `6588.849534059469` |
| `gain_dbi` | `10 log10(6588.849534059469)` | `38.188095899160565 dBi` |

### Nominal, limiting and invalid cases

- Double diameter: effective area and linear gain quadruple; gain becomes `44.20869581244019 dBi`.
- Double frequency at fixed diameter/efficiency: area is unchanged and gain quadruples. This assumes the supplied efficiency remains applicable.
- Efficiency=1 is accepted. Halving efficiency halves both area and gain, decreasing dBi by `3.010299956639812 dB`.
- `D=2lambda` is excluded; a representably larger diameter is accepted. Reject efficiency<=0, efficiency>1, nonpositive diameter/frequency and nonfinite inputs.
- As efficiency approaches zero from above, linear area/gain approach zero and logarithmic gain falls without an arbitrary dBi clamp.

### Mandatory diagram

Draw a circular aperture front view centered near `(200,130)` with a vertical diameter dimension at `x=90`. Add several parallel incident-wave arrows at right pointing toward the aperture, labeled “boresight incidence.” Bind `diameter` on the diameter dimension, `frequency` beside the wave arrows and `aperture_efficiency` beside a visibly smaller conceptual collecting-area annotation. Do not depict the effective aperture as an actual smaller metal disk. In a separate right-side formula panel bind `effective_aperture`, `gain_linear` and `gain_dbi`; label the first “Ae = eta × physical area.” No quantity key may label a drawn field amplitude or actual beamwidth.

Caption: “Circular physical aperture with efficiency eta. Ae is an effective collecting area, not a second physical opening; gain is boresight and the drawing is not to scale.”

## 6. `rf.two_tone_im3`

### Purpose, novelty and evidence

This is a nonlinear RF spectral model absent from the baseline. `noise.analog_dynamic_range` compares signal and random noise floors; it does not create deterministic in-band spurs. `data.dynamic_enob` interprets measured SINAD but does not predict amplifier cubic mixing. S5 directly states frequency products, slopes and intercept equations; S6 establishes power-reference-plane practice.

### Equations, units and domain

Let `f1=lower_frequency`, `f2=upper_frequency`, `Pin=input_tone_power_dbm`, `G=gain_db`, and `IIP3=input_intercept_dbm`.

- `lower_product_frequency = 2f1-f2` Hz.
- `upper_product_frequency = 2f2-f1` Hz.
- `output_tone_power_dbm = Pin+G`.
- `output_intercept_dbm = IIP3+G`.
- `carrier_to_product_db = 2(IIP3-Pin)`.
- `product_power_dbm = output_tone_power_dbm-carrier_to_product_db` for each product.
- `total_fundamental_power_dbm = output_tone_power_dbm+10 log10(2)`.

These follow from the intercept equation `OIP3=Pout+(Pout-PIM3)/2`. Both input powers are per tone; adding 3.0103 dB gives their total **average** fundamental power. Two-tone peak-envelope power is higher and must already be allowed for in the supplied characterized linear-output limit.

Require `f1<f2<2f1` so this near-band formulation has two positive ordered products, `Pin<IIP3`, and total fundamental average output not exceeding `maximum_total_linear_output_dbm`. The latter is supplied from characterization for the actual bias, frequency, spacing, temperature and crest factor; it is not P1dB or OIP3. The engine cannot infer an unprovided amplifier response or verify the manufacturer's characterization. The two equal IM3 levels require a weak cubic response with sufficiently flat gain at both products. No compression, memory, unequal-tone or mixer-LO model is claimed.

### Complete independent example

Inputs: `lower_frequency=100000000 Hz`, `upper_frequency=101000000 Hz`, `input_tone_power_dbm=-20`, `gain_db=20`, `input_intercept_dbm=10`, `maximum_total_linear_output_dbm=10`.

| Output key | Independent derivation | Expected |
|---|---|---:|
| `lower_product_frequency` | `2*100e6-101e6` | `99000000 Hz` |
| `upper_product_frequency` | `2*101e6-100e6` | `102000000 Hz` |
| `output_tone_power_dbm` | `-20+20` | `0 dBm` |
| `output_intercept_dbm` | `10+20` | `30 dBm` |
| `product_power_dbm` | `3*(-20)+20-2*10` | `-60 dBm` |
| `carrier_to_product_db` | `0-(-60)` | `60 dB` |
| `total_fundamental_power_dbm` | each fundamental is 1 mW; total is 2 mW | `3.010299956639812 dBm` |

Each product is `1e-9 W`; each fundamental is `1e-3 W`; their ratio is `1e6`. This is an independent linear-power interpretation of the dB prediction.

### Nominal, limiting and invalid cases

- Increase per-tone input by 1 dB to -19 dBm: each fundamental becomes 1 dBm, each IM3 becomes -57 dBm, separation becomes 58 dB, total becomes `4.010299956639812 dBm`. Frequencies and OIP3 do not change.
- Increase IIP3 by 3 dB while holding all other inputs: each IM3 drops 6 dB; fundamentals stay fixed.
- Exact characterized-limit equality is accepted. An input increment that puts summed fundamental power above that limit is rejected, even if each tone separately remains below it.
- Reject equal/reversed tones, f2>=2f1, nonpositive frequencies, Pin>=IIP3 and nonfinite values. Negative dBm intercepts and negative gain are not intrinsically invalid if all relational domains pass.
- Very low Pin produces small but finite logarithmic product power. No arbitrary noise-floor clamp is allowed; the model does not claim the spur can be measured below instrument noise.

### Mandatory diagram

Left panel: two tone-generator arrows into a single amplifier block, labeled `lower_frequency`, `upper_frequency`, common `input_tone_power_dbm`, `gain_db` and `input_intercept_dbm`. Right panel: a static frequency-axis spectrum with two taller inner stems at f1/f2 and two shorter outer stems at `lower_product_frequency` and `upper_product_frequency`. Label inner height “per tone” with `output_tone_power_dbm`; label outer height “each IM3” with `product_power_dbm`. A vertical separation annotation binds `carrier_to_product_db`. In clear lower text, bind `output_intercept_dbm`, `total_fundamental_power_dbm` and `maximum_total_linear_output_dbm`; do not draw OIP3 as a real additional tone. Keep stems and text separated; use short stem labels and let the quantity legend expand names.

Caption: “Equal-tone weak-cubic model: IM3 changes 3 dB for a 1 dB input change. Spectrum heights/spacing are illustrative; IP3 is an extrapolated intersection, not an output tone.”

## 7. `rf.monostatic_radar_return`

### Purpose, novelty and evidence

The nearest new neighbor is `rf.free_space_link_budget`. A one-way communications link has an active transmitter and a receiving antenna at the opposite endpoint, giving inverse-square range loss. This model instead has a **passive scattering target** described by RCS and the same antenna on transmit and receive. Outbound and return spreading create inverse-fourth-power range dependence. It is not the communications equation inverted or given extra outputs. No baseline model includes target scattering. S7 slides 5–10 explicitly derive these stages.

### Equations and physical limits

Linear SI derivation:

`incident_density = Pt G/(4 pi R^2)` W/m²;
`return_density = incident_density * sigma/(4 pi R^2)` W/m²;
`Ae = G lambda^2/(4 pi)` m²;
`Pr = return_density * Ae/L = Pt G^2 lambda^2 sigma/((4 pi)^3 R^4 L)` W.

Consequently:

`received_power_dbm = transmit_power_dbm + 2*antenna_gain_dbi + 20 log10(c/f) + 10 log10(radar_cross_section) - 30 log10(4pi) - 40 log10(range) - total_loss_db`;
`echo_margin_db = received_power_dbm - receiver_threshold_dbm`.

Logs take the numerical SI values. The catalog expands logarithms to avoid explicitly forming R^4, large linear gains or tiny returned power. Range is one-way. RCS has square-meter units but is not geometric area: it is frequency-, aspect- and polarization-specific scattering strength. Positive RCS is required for finite dBm. Total loss includes the full two-way/system budget and is subtracted once; it must not accidentally be doubled twice.

Use the largest dimensions of both antenna and target in `R >= max(10 lambda, 2 max(Dant,Dtarget)^2/lambda)`. The aperture phase criterion is extended to the target to keep incident-wave/RCS use in a far-field regime; the ten-wavelength minimum is an explicit conservative screen, not a statement that all RCS models become accurate exactly there. The target must be sufficiently isolated and accurately characterized. Pulsed Pt is in-pulse power, not average duty-cycle power, unless the receiver threshold is formulated for the same averaging. No range resolution, clutter, coherent integration, detection probability or false-alarm probability is inferred.

### Complete independent example

Inputs: `frequency=10000000000 Hz`, `range=1000 m`, `antenna_max_dimension=0.3 m`, `target_max_dimension=1 m`, `transmit_power_dbm=30`, `antenna_gain_dbi=30`, `radar_cross_section=1 m^2`, `total_loss_db=3`, `receiver_threshold_dbm=-100`.

Independent linear values are `Pt=1 W`, `G=1000`, `lambda=0.0299792458 m`, `L=10^0.3=1.9952623149688795`. The target-dimension screen is `66.7128190396304 m`, so 1000 m is admissible.

`Pr = [1 * 1000^2 * 0.0299792458^2 * 1] / [(4pi)^3 * 1000^4 * 1.9952623149688795] = 2.2699265981678502e-13 W`.

| Output key | Independent derivation | Expected |
|---|---|---:|
| `received_power_dbm` | `10 log10(2.2699265981678502e-13 / 1e-3)` | `-96.43988186210434 dBm` |
| `echo_margin_db` | `-96.43988186210434 - (-100)` | `3.560118137895657 dB` |

### Nominal, limiting and invalid cases

- Double range: echo power divides by 16, received=`-108.4810816886636 dBm`, margin=`-8.481081688663592 dB`. The negative margin is a valid result.
- Double RCS: echo rises `3.010299956639812 dB`. Increase the common antenna gain 1 dB: echo rises 2 dB because gain acts on both paths.
- Add 1 dB to total loss: echo and margin each fall 1 dB, not 2 dB.
- Zero total loss is accepted. Threshold equal to computed echo gives zero margin. Arbitrarily small positive RCS remains logarithmically meaningful without pretending zero RCS has finite dBm.
- Accept exact far-field-screen equality; reject smaller range. Increasing the target dimension can invalidate a previously accepted range without changing nominal radar gain or RCS.
- Reject zero/negative RCS, frequency, range or dimensions; reject negative total loss and nonfinite values. Poorly characterized RCS cannot be repaired by a numeric input clamp.

### Mandatory diagram

Draw one radar antenna/equipment group at `(90,130)` and one passive target silhouette at `(590,130)`. Draw an upper arrow from radar to target and a lower arrow back to radar, visibly opposite directions. Label one-way `range` on a separate dimension line. Bind `transmit_power_dbm` to the outgoing path and `received_power_dbm` to the returning path. Label target `sigma` with `radar_cross_section`; label its extent with `target_max_dimension`. Label radar antenna extent with `antenna_max_dimension`, common gain with `antenna_gain_dbi`, carrier with `frequency`, and full budget with `total_loss_db`. Bind `receiver_threshold_dbm` and `echo_margin_db` in a receiver-side annotation. Plain relation text may show “outward 1/R² × return 1/R²”; do not bind one arrow length to round-trip range.

Caption: “Monostatic radar: the same antenna transmits and receives. Target RCS scatters the signal; two spreading paths yield 1/R^4 echo power. Arrows are schematic, not ray-tracing output.”

## Integration and residual risk handoff

Promote these seven objects only after the parent novelty gate. Implement one matching diagram entry per ID and update the owned feature documentation; no engine extension is required. Existing functions, including `expm1`, `min`, `max`, square root and base-10 logarithm, cover all expressions. Source limits must remain visible in UI metadata and documentation; do not treat a numerical domain pass as a measured antenna or amplifier qualification.

Independent testers should use the complete examples above, direct network identities, inverse-square versus inverse-fourth-power scaling, frequency/geometry symmetries, per-tone power sums, exact domain edges and nonfinite rejection. Use absolute tolerances for expected zero outputs/margins; do not weaken small nonzero resistor or reactive-element cases into finite-only checks. Parent should exercise the real Rust process, native quantity legends, diagram pixels, large-text/narrow-window access and negative-margin presentation.

EMC was notified directly that RF excludes short-wire common-mode radiation. No unresolved cross-lane collision remains known. The main residual risks are physical, not missing formulas: real component parasitics, uncertain aperture efficiency/RCS, propagation obstruction/fading and an improperly characterized linear-output limit. The explicit far-field and paraxial screens are engineering model boundaries, not measurement-based guarantees.

Skipped: production edits, tests, builds, lint, formatting and runtime/UI checks, as required for the research wave. Parent must verify implementation and all seven actual rendered diagrams.
