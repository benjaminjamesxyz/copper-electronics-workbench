# Precision analog front ends — executable feature catalog

Six circuit-specific DC and small-signal models. SI units, complete defaults, constraints and independent examples are retained from the [research handoff](../../research/expansion/precision.md). Numerical definitions live in `backend/catalog/precision.json`; static explanatory diagrams live in `frontend/lib/diagrams/precision.dart`. Main integrates the shared registry. No builds, tests, lint, formatters or native-image checks were run in this lane. Source-access notes and evidence provenance remain in the research handoff; manufacturer models are not hardware guarantees.

## 1. `precision.bjt_mirror_base_current` — basic

### Purpose and novelty

Predict the reference-current loss into two base terminals of a matched two-NPN current mirror. This is not `bjt.hybrid_pi_parameters`, `bjt.early_output_resistance` or `bjt.differential_pair_steering`: it solves current-copy bias topology, not incremental transistor parameters or tail-current division. Unlike the Howland model below, the error comes from finite transistor beta, not feedback resistor balance. Unlike the Brokaw core, no temperature-compensated voltage reference is synthesized.

### Topology, law and limits

Q1 and Q2 are matched equal-area NPNs. Both emitters connect to ground. Their bases connect together. Q1 collector connects to that base node, forming the diode-connected input. `reference_current` enters this node. Q2 collector receives the external load current and sits at `collector_voltage`.

ADI section 11.5.1 states `Iout = Iin - 2 Ib`. With equal beta and equal collector currents:

- `Iref = Ic + 2 Ic/beta`.
- `collector_current = Iref/(1 + 2/beta)`.
- `base_current_each = Iref/(beta + 2)`.
- `current_deficit = 2 Iref/(beta + 2)`.
- `fractional_deficit = 2/(beta + 2)`.
- `compliance_margin = collector_voltage - base_emitter_voltage`.

The reference current includes both base currents. This is not the current through Q1 collector alone. For conservative forward-active operation, Q2 collector must not fall below its base: `collector_voltage >= base_emitter_voltage`. Equality is the boundary of zero collector-base bias. This deliberately does not use the much smaller saturation-voltage figure as a precision compliance limit.

The model neglects Early effect, mismatch, leakage, beta variation, self-heating and frequency response. A specified beta must apply at this operating point. Device breakdown, current ratings and thermal limits require separate manufacturer checks. Current and beta must be positive; zero reference current is outside the assumed characterized VBE/beta operating point.

### Complete independent example

Inputs: `reference_current=0.001`, `beta=100`, `base_emitter_voltage=0.65`, `collector_voltage=2`.

Reference-node KCL gives 102 equal base-current units. Each base current is `0.001/102 A`; the collector receives 100 such units.

| Output key | Independent value |
|---|---:|
| `collector_current` | 0.000980392156862745 A |
| `base_current_each` | 0.000009803921568627451 A |
| `current_deficit` | 0.000019607843137254903 A |
| `fractional_deficit` | 0.0196078431372549 |
| `compliance_margin` | 1.35 V |

The output current plus twice either base current equals the supplied 1 mA.

### Tester cases

- Nominal: assert every table value and KCL.
- Large beta: set `beta=1000`; collector current is `0.000998003992015968 A`, fractional deficit `0.001996007984031936`.
- Compliance boundary: set `collector_voltage=0.65`; margin is exactly zero; currents remain unchanged in the ideal model.
- Scaling: double reference current at unchanged characterized beta/VBE; all current outputs double, fractional deficit does not.
- Invalid: beta zero/negative, reference current zero/negative, VBE nonpositive, or collector voltage below VBE.

### Mandatory diagram specification

Canvas 700×280, 16 px margins, labels at least 14 px. Draw the two NPNs side by side, with both emitters joining a grounded bottom rail. Show Q1 collector-to-base short explicitly; show the joined base net and junction dots. An ideal reference-current arrow points downward into Q1's collector/base node; a separate downward arrow enters Q2 collector from the load port. Do not join collectors.

Exact quantity bindings: incoming reference arrow `reference_current`; Q2 collector arrow `collector_current`; base branch arrow `base_current_each`; base node voltage `base_emitter_voltage`; Q2 collector voltage `collector_voltage`; side annotation `fractional_deficit`; collector-to-base voltage bracket `compliance_margin`. Static labels Q1, Q2 and “matched beta” identify devices; `beta` can bind the latter.

Caption: “Reference current supplies one collector and two bases. The output collector stays at or above the base voltage; matched DC devices and no Early effect are assumed.” Break all wires at transistor bodies and place explicit dots at the base and emitter joins.


## 2. `precision.instrumentation_internal_headroom` — intermediate

### Purpose and novelty

Design a high-input-impedance three-op-amp instrumentation amplifier while checking hidden internal clipping. Nearest baseline is `opamps.difference_ratio_error`, a single difference amplifier with resistor mismatch. This model has two additional input amplifiers and a floating gain resistor; its limiting behavior depends on their individual output nodes. It is not a renamed scalar noninverting gain or a second bridge-transducer calculation.

### Topology, law and limits

The signal inputs are `common_voltage ± differential_voltage/2`. Each drives an op amp's noninverting terminal. Both first-stage feedback resistors equal `feedback_resistance`; `gain_resistance` connects the two inverting terminals.

The negative-input branch output Va drives the final op amp's minus input through `difference_input_resistance`. The positive-input branch output Vb drives the final plus input through another equal input resistor. `difference_feedback_resistance` connects final output to its minus input; an equal resistor connects final plus input to the low-impedance `reference_voltage` source. This is the Figure 5 topology with a driven reference replacing ground.

Let `G1=1+2Rf/Rg`, `k=Rd/Ri`:

- `first_stage_gain = G1`.
- `total_gain = G1 k`.
- `first_low_voltage = Va = Vcm - G1 Vd/2`.
- `first_high_voltage = Vb = Vcm + G1 Vd/2`.
- `difference_pin_voltage = (k Vb + Vref)/(1+k)`.
- `output_voltage = Vref + G1 k Vd`.

“Low” and “high” are branch names, not sorting operations: negative Vd swaps their numerical order. This is stated in catalog assumptions and must be retained in UI wording.

Check both first-stage outputs and the final output against the supplied guaranteed loaded output range. Check both signal input voltages and the final difference-stage pin voltage against the supplied common-mode range. Constraints use inputs only. A final output that is within range does not rescue a saturated input stage.

The same guaranteed limits apply to all three amplifiers. Matching is ideal; no bias current, resistor mismatch, noise, bandwidth or output-current prediction is included. The reference source must have negligible impedance. Actual amplifier drive capability and stability need a device-level check; the voltage range supplied must be valid under the intended loading.

### Complete independent example

Inputs: `feedback_resistance=10000`, `gain_resistance=1000`, `difference_input_resistance=10000`, `difference_feedback_resistance=20000`, `common_voltage=2`, `differential_voltage=0.01`, `reference_voltage=0.5`, `output_min=-10`, `output_max=10`, `common_min=-8`, `common_max=8`.

`G1=1+20=21`, `k=2`, `G=42`. Input pins are 1.995 and 2.005 V. The gain-resistor current is 10 µA; each feedback resistor develops 0.1 V. Therefore Va=1.895 V and Vb=2.105 V. The difference-stage plus node is `(2×2.105+0.5)/3=1.57 V`.

| Output key | Independent value |
|---|---:|
| `first_stage_gain` | 21 |
| `total_gain` | 42 |
| `first_low_voltage` | 1.895 V |
| `first_high_voltage` | 2.105 V |
| `difference_pin_voltage` | 1.57 V |
| `output_voltage` | 0.92 V |

### Tester cases

- Nominal: assert all values and final difference-stage KCL.
- Zero differential voltage: Va=Vb=2 V, final output=0.5 V and final input pin=1.5 V.
- Reverse differential: Vd=-0.01 V gives Va=2.105 V, Vb=1.895 V, final pin=1.43 V and final output=0.08 V.
- Hidden clipping: set `common_voltage=7.89`, `differential_voltage=0.21`, keeping other inputs nominal. Both signal pins (7.785 V and 7.995 V) remain inside [-8,8] V and final output=9.32 V remains inside [-10,10] V, but Vb=10.095 V. Reject specifically because the first-stage output clips.
- Boundary: final output exactly on either supplied endpoint is accepted if all other nodes fit.
- Invalid: nonpositive resistance, reversed/equal limit intervals, any signal/common-mode pin outside its interval, or any of the three outputs outside its interval.

### Mandatory diagram specification

Draw two input amplifiers at the left and one difference amplifier at the right. Va is the upper/negative-input branch, Vb the lower/positive-input branch. Include both feedback resistors and the single resistor between the two minus pins. Show all four final-stage resistors with matched-value labels; the lower reference resistor terminates in a voltage-source port, not ground. The final minus input must receive Va, and plus input must receive Vb through its resistor.

Exact quantity bindings: first-stage resistors `feedback_resistance`, cross resistor `gain_resistance`, both final input resistors `difference_input_resistance`, both final feedback/reference resistors `difference_feedback_resistance`, Va `first_low_voltage`, Vb `first_high_voltage`, reference port `reference_voltage`, final input node `difference_pin_voltage`, final output `output_voltage`. Input labels bind `common_voltage` and `differential_voltage` only in an explanatory bracket reading “inputs = Vcm ± Vd/2”; do not bind either individual input wire to Vcm or Vd alone.

Caption: “Three amplifier outputs need headroom. Input common mode can clip an internal stage even when the final differential output fits. Va/Vb identify branches, not sorted voltages.” Use the full 700×280 surface with clear labels; put gain and limits in the catalog-linked legend rather than crowding the schematic.


## 3. `precision.transimpedance_butterworth` — expert

### Purpose and novelty

Choose feedback capacitance for a finite-GBW current-to-voltage front end with input shunt capacitance. Nearest baseline is `opamps.dominant_pole_gain_bandwidth`, which has frequency-independent noise gain and first-order closed-loop voltage response. This circuit's noise gain rises because of source capacitance, creating a second-order response whose damping must be designed. It is not ordinary RC filtering, a generic noise budget or photodiode conversion. Opto explicitly owns the physical photocurrent and confirmed this electronic boundary.

### Topology and derivation

An electrical current source injects current into an op amp's inverting node. `source_capacitance` connects that node to AC ground. The noninverting input is grounded. `feedback_resistance` and the calculated `feedback_capacitance` connect in parallel from output to inverting node.

Let `R=Rf`, `Cs=source_capacitance`, `Cf=feedback_capacitance`, `wt=2π gain_bandwidth`. Adopt the standard large-DC-gain dominant-pole approximation `A(s)=wt/s`, as the integrator limit of SBOA122 equations 2–7. With current entering the summing node, KCL gives

`Vo/Iin = -R / [1 + s(1/wt + R Cf) + s² R(Cs+Cf)/wt]`.

The sign is negative; SBOA122's current-arrow convention may express the opposite sign. This catalog states its own arrow explicitly.

For Butterworth damping, the squared linear denominator coefficient is twice the quadratic coefficient:

`(1 + wt R Cf)² = 2 wt R(Cs+Cf)`.

The linear Cf terms cancel. Therefore

- `Cf = sqrt(2 wt R Cs - 1)/(wt R)`.
- `butterworth_bandwidth = sqrt(wt/[R(Cs+Cf)])/(2π)`.
- `dc_transimpedance = -R`.
- `output_peak = input_peak R`, the low-frequency peak magnitude.
- `noise_gain_zero = 1/[2π R(Cs+Cf)]`.

The minus-3-dB bandwidth equals the natural frequency only because Q is fixed to `1/sqrt(2)`. The formula retains Cf in the natural frequency instead of silently assuming `Cs >> Cf`. The noise-gain zero is a loop-design frequency, not the closed-loop bandwidth.

### Domain and limits

Require positive R, Cs and GBW; require `2 wt R Cs >= 1`. This is a genuine solvability condition for a nonnegative Butterworth Cf. At equality Cf=0 is allowed, and no output divides by Cf. Require nonnegative input peak and positive guaranteed symmetric output swing; reject `input_peak R` above that swing.

The amplifier model assumes infinite DC gain but finite GBW and one integrating pole. Real secondary poles, package/parasitic feedback capacitance, current-source finite resistance, leakage, bias, noise, slew, output current and source voltage compliance are excluded. A unity-gain-stable label alone is not a stability proof. The user must verify the actual amplifier loop and include all device/PCB/source capacitance in Cs. There is no imposed arbitrary Cs/Cf ratio because the derived model does not need that simplification.

### Complete independent example

Inputs: `feedback_resistance=100000`, `source_capacitance=1e-10`, `gain_bandwidth=1e7`, `input_peak=1e-5`, `output_peak_limit=4`.

`wt=62831853.07179586 rad/s`, `wt R Cs=628.3185307179587`. Insert these in the independently derived quadratic damping equation. The positive capacitor solution is 5.63965054967668 pF. The complete denominator gives the frequency below, not the simplified `sqrt(GBW/(2π R Cs))` approximation.

| Output key | Independent value |
|---|---:|
| `feedback_capacitance` | 5.63965054967668e-12 F |
| `butterworth_bandwidth` | 388147.3164084609 Hz |
| `dc_transimpedance` | -100000 ohm |
| `output_peak` | 1 V |
| `noise_gain_zero` | 15065.833923508984 Hz |

### Tester cases

- Nominal: assert all values and substitute Cf into the damping identity.
- Zero signal: set `input_peak=0`; output peak is zero, all design outputs remain unchanged.
- Solvability boundary: with nominal R and Cs, GBW=`1/(4π R Cs)` ≈7957.747154594768 Hz gives Cf=0 and bandwidth≈11253.953951963826 Hz. Construct representable neighboring inputs carefully: a rounded value immediately below the inequality must reject, not be clamped to zero.
- Invalid solvability: GBW=1000 Hz with nominal R and Cs cannot realize a nonnegative Butterworth Cf; reject.
- Swing boundary: input peak=40 µA gives exactly 4 V and is accepted; a larger value rejects.
- Invalid: nonpositive R/Cs/GBW/swing or negative signal amplitude.

### Mandatory diagram specification

Draw the op amp with its plus input grounded. Draw an electrical current-source symbol from ground into the minus node with arrow toward that node. Cs runs from the same node to ground. Draw two separate feedback branches: Rf above the op amp and Cf farther above it, both connecting the exact same summing/output nodes. Mark explicit junctions; never wire through either component body. No photodiode symbol or optical ray appears.

Exact quantity bindings: current source `input_peak`; shunt capacitor `source_capacitance`; resistor `feedback_resistance`; feedback capacitor `feedback_capacitance`; op-amp GBW annotation `gain_bandwidth`; output magnitude bracket `output_peak`; textual minus-3-dB annotation `butterworth_bandwidth`. A static transfer annotation may say “Vo/Iin → -Rf at DC”; it must not label a voltage node with `dc_transimpedance` as though it were volts.

Caption: “Current entering the summing node produces negative output. Cf sets Butterworth damping in the single-integrator op-amp model; the displayed bandwidth is not a measured stability guarantee.” Avoid a fake live frequency plot.


## 4. `precision.howland_mismatch_load` — expert

### Purpose and novelty

Predict bidirectional load current and compliance when a basic Howland pump's resistor ratios are not perfectly balanced. Nearest baseline is `opamps.difference_ratio_error`, but the positive feedback and grounded current-output port change the physical problem: mismatch creates signed source output conductance and load-dependent current. Nearest new mirror model has a transistor base-current deficit, not this resistor-feedback law. No transistor foldback limiter or protection threshold is modeled.

### Exact topology and law

Use TI AN-1515 Figure 1's basic circuit, not the improved five-resistor circuit:

- R1 (`r_one`) connects `positive_drive` to load node Vx.
- R2 (`r_two`) connects op-amp output Vo to Vx.
- Vx directly connects to the op amp's plus input and the top of `load_resistance`; load bottom is ground.
- R3 (`r_three`) connects `negative_drive` to the minus input.
- R4 (`r_four`) connects Vo to the minus input.

For an ideal op amp in its linear range, both pins equal Vx. Minus-node KCL gives `Vo=(1+k)Vx-k Vn`, where `k=R4/R3`. Load-node KCL gives

`IL=(Vp-Vx)/R1 + (Vo-Vx)/R2 = J-g Vx`,

with `J=Vp/R1-k Vn/R2`, `g=1/R1-k/R2`. Therefore

- `norton_drive_current = J`.
- `output_conductance = g`.
- `load_voltage = J/(1/RL+g)`.
- `load_current = load_voltage/RL`.
- `amplifier_voltage = (1+k)load_voltage-k Vn`.
- `amplifier_current = (Vo-Vx)/R2 + (Vo-Vx)/R4`.

Positive load current leaves Vx into the load. Positive amplifier current leaves the op amp output. The two are not generally equal because the drive resistors also carry current. Perfect balance requires `R1/R2=R3/R4`; then g=0, not an invalid infinite numeric output. No reciprocal-g output is required.

### Domain and limits

All resistances are positive. The DC closed-loop branch must have `1/RL+g>0`. Negative g is admitted when the finite passive load retains this inequality. At equality the loop is singular; beyond it the assumed negative-feedback operating branch fails. Near the boundary, small resistor errors create huge sensitivity.

Reject load/pin voltage outside the supplied guaranteed common-mode interval, amplifier voltage outside the supplied output interval, and absolute amplifier current above the supplied source/sink bound. Output and common-mode intervals must be strictly ordered. DC rejection is not a dynamic-stability certificate. Real op-amp poles, small compensation capacitors, load capacitance, output resistance, offset and bias must be checked separately. Source/sink limits should be conservative guaranteed values at the required voltage and temperature.

### Complete independent example

Inputs: `r_one=10000`, `r_two=10000`, `r_three=10000`, `r_four=9900`, `positive_drive=1`, `negative_drive=0`, `load_resistance=5000`, `output_min=-10`, `output_max=10`, `common_min=-8`, `common_max=8`, `output_current_limit=0.01`.

`k=99/100`, J=100 µA and g=1 µS. The load conductance is 200 µS, so Vx=`100/201 V`. Vo=`(199/100)(100/201)=199/201 V`. The R2 current is 49.25373134328358 µA; the R4 current is 49.75124378109453 µA. Their sum is the amplifier output current.

| Output key | Independent value |
|---|---:|
| `norton_drive_current` | 0.0001 A |
| `output_conductance` | 0.000001 S |
| `load_voltage` | 0.4975124378109453 V |
| `load_current` | 0.00009950248756218905 A |
| `amplifier_voltage` | 0.9900497512437811 V |
| `amplifier_current` | 0.00009900497512437811 A |

### Tester cases

- Nominal: assert all values, both node KCL equations and amplifier-current decomposition.
- Perfect balance: set R4=10000; J=100 µA, g=0, Vx=0.5 V, IL=100 µA, Vo=1 V and amplifier current=100 µA.
- Reverse drive: set positive drive=-1 V with nominal resistors; all current/voltage outputs reverse, while g stays +1 µS.
- Admissible negative conductance: R4=10100 gives g=-1 µS and Vx=`100/199`≈0.5025125628140703 V at nominal load. Do not reject solely because g is negative.
- Zero drives: both drives zero gives exact zero voltages/currents while retaining g.
- Singular/unstable: R4=30000 with nominal R1–R3 and RL=5000 gives zero total conductance; use neighboring representable cases to distinguish this boundary from a tiny positive denominator. R4=40000 is strictly invalid.
- Compliance: reduce output limit below required Vo, common limit below Vx, or current limit below required absolute amplifier current; each must reject independently.

### Mandatory diagram specification

Draw the exact four-resistor netlist above. Place Vx below/left of the op amp plus pin and connect its grounded load vertically. R1 feeds Vx from a left drive source. R2 returns from the amplifier output to Vx; R4 is the distinct upper negative-feedback branch. R3 runs from the other drive source to the minus pin. Show the plus-feedback branch joining Vx, not the minus node. Clearly separate the load port from the amplifier output.

Exact bindings: R1 `r_one`, R2 `r_two`, R3 `r_three`, R4 `r_four`; source ports `positive_drive`, `negative_drive`; load `load_resistance`; Vx `load_voltage`; downward load arrow `load_current`; op-amp output `amplifier_voltage`; output arrow before its R2/R4 split `amplifier_current`. A side annotation may bind `output_conductance` with label “Norton g”; do not depict that calculated equivalent as an extra physical resistor.

Caption: “Basic Howland pump: ratio mismatch creates signed output conductance. The load node and amplifier output are different nodes; current and voltage compliance are checked separately. DC model only.”

