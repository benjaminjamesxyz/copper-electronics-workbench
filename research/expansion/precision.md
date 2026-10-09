# Precision analog expansion: six researched circuit models

## Scope, evidence and implementation handoff

Own files: `research/expansion/precision.json` and this document. Exactly six new models: two basic, two intermediate and two expert. The existing 128-item inventory remains unchanged. Catalog inputs, outputs, constraints and complete examples use the existing expression schema; no new engine function is required.

Read the expansion contract, numerical contract, expansion board, all baseline models and current communication-bus claims before selection. Only electrical front ends are modeled. Sensors retain physical transduction; opto retains photon conversion; measurement retains instrument loading, uncertainty and timing; passives retain nonideal passive networks. Opto confirmed no overlap.

Primary evidence actually read:

1. [Analog Devices University, Chapter 11: The Current Mirror](https://wiki.analog.com/university/courses/electronics/text/chapter-11), sections 11.4–11.5, including finite base current and conservative collector-base compliance.
2. [TI SLOA034, Signal Conditioning Wheatstone Resistive Bridge Sensors](https://www.ti.com/lit/an/sloa034/sloa034.pdf), section 2.2 and Figure 5. Only the instrumentation electronics are adopted, not the bridge transducer.
3. [TI SBOA122, Transimpedance Considerations for High-Speed Amplifiers](https://www.ti.com/lit/an/sboa122/sboa122.pdf), Figure 1, sections 2–3, equations 2–12. The model below derives the Butterworth solution before the source's additional large-source-capacitance simplification.
4. [TI AN-1515 / SNOA474A, A Comprehensive Study of the Howland Current Pump](https://www.ti.com/lit/an/snoa474a/snoa474a.pdf), Figures 1–2 and section 2. The unequal-ratio solution below follows Kirchhoff's laws for that exact basic topology.
5. [TI SLLA065, A Comparison of LinBiCMOS and CMOS Process Technology in LVDS Integrated Circuits](https://www.ti.com/lit/an/slla065/slla065.pdf), Figure 7, equations 7–13 and temperature-compensation discussion, pages 12–14. The reference core, not LVDS signaling, is used.
6. [TI LOG114 data sheet, SBOS301C](https://www.ti.com/lit/ds/symlink/log114.pdf), sections 5.5, 7.1.7 and 7.2.1 / Figure 7-8. The positive-current input convention, nominal 0.375 V/decade law, reference loading and dual-supply connection were read.

ADI MT-077, MT-087 and the ADI bandgap calculator tutorial timed out. They are not supporting citations. TI replacements provide the relevant accessible evidence. Some PDF diagrams extract poorly as text; the explicit topology netlists below remove ambiguity, and implementation review must compare the rendered circuit connectivity to them. No source claims are inferred from search-result text alone.

The independent arithmetic used Python scalar arithmetic and exact fractions for the Howland example. It did not call the Rust engine or evaluate catalog expressions. Physical equations and every example output are derived below. Builds, tests, formatters, linters, application launches and numerical engine runs were not performed.

All voltages reference circuit ground unless noted. Resistance is ohms, capacitance farads, frequency hertz, current amperes, conductance siemens and temperature kelvin. Dimensionless ratios use `1`. Finite binary64 arithmetic is still required: nonrepresentable intermediate expressions must report arithmetic errors, not fabricate finite outputs. Do not add arbitrary minimums to suppress valid boundaries.

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

## 5. `precision.brokaw_first_order_trim` — intermediate

### Purpose and novelty

Choose the common-emitter resistor for first-order PTAT/CTAT temperature cancellation in a Brokaw reference. Nearest baseline `thermal.resistor_tcr_drift` changes a single resistor with temperature; this active reference combines two junction densities and a resistor ratio to cancel a junction-voltage slope. It is not a sensor thermometer, generic Shockley forward-current inversion, zener shunt regulator or duplicate current mirror: the design output is the resistor ratio that synthesizes a compensated voltage.

### Topology, derivation and physical limits

Follow TI SLLA065 Figure 7's core relationship. Q2 has emitter area N times Q1. Their bases are joined at Vref. The equal-current collector servo makes their collector currents equal. Q1 emitter joins node E; Q2 emitter connects through R2 (`ptat_resistance`) to E. R1 (`common_resistor`) connects E to ground. R1 carries both emitter currents; this produces the factor of two that must not be omitted.

An equivalent concrete servo implementation for the explanatory schematic uses two equal collector resistors to a positive supply. An op amp senses Q1 collector at plus and Q2 collector at minus, then drives the common base/Vref node. Equal collector voltages imply equal collector currents. The core equations are the same ideal large-beta equal-current conditions as the source. This sketch illustrates the nonzero operating state; its startup circuit and dynamic compensation are explicitly outside the calculation.

Using exact SI constants k=1.380649e-23 J/K and q=1.602176634e-19 C:

- `thermal_voltage = k T/q`.
- `delta_vbe = (kT/q) ln(N) = VBE1-VBE2`.
- `branch_current = delta_vbe/R2`.
- `Vref = VBE1 + 2(R1/R2)(kT/q) ln(N)`.

For local slope S=`base_emitter_slope`, set `dVref/dT=0`. Thus

- `resistor_ratio = R1/R2 = -S/[2(k/q)ln(N)]`.
- `common_resistor = resistor_ratio R2`.
- `reference_voltage = base_emitter_voltage - S T`.

The final expression is an algebraically simplified evaluation of the same synthesized reference, not a separate temperature extrapolation. Require T>0, N>1, R2>0, VBE>0 and S<0 so the synthesized R1 is positive. N need not be an integer in an integrated geometry model.

VBE and its local slope must be characterized at the actual operating trajectory, including the PTAT bias current. A constant-current VBE slope should not be substituted without correcting its bias dependence. The supplied slope is a design characterization, not a universal -2 mV/K constant. Beta is large, junction temperature is common, collector currents match, and resistor temperature coefficients track. Curvature, finite beta, offsets, current-density limits, startup, supply headroom and self-heating are not predicted. The model is a local trim at the stated temperature, not full-range ppm/K certification.

### Complete independent example

Inputs: `temperature=300`, `area_ratio=8`, `ptat_resistance=10000`, `base_emitter_voltage=0.65`, `base_emitter_slope=-0.002`.

`k/q=0.00008617333262145179 V/K`, `ln(8)=2.0794415416798357`. Delta VBE is 53.75772229141229 mV; each collector current is therefore 5.375772229141229 µA. The positive PTAT derivative must be 2 mV/K, so `R1/R2=5.580593581955471`. The total drop across R1 is 0.6 V and the reference output is 0.65+0.6=1.25 V.

| Output key | Independent value |
|---|---:|
| `thermal_voltage` | 0.025851999786435535 V |
| `delta_vbe` | 0.05375772229141229 V |
| `branch_current` | 0.000005375772229141229 A |
| `resistor_ratio` | 5.580593581955471 |
| `common_resistor` | 55805.93581955471 ohm |
| `reference_voltage` | 1.25 V |

### Tester cases

- Nominal: assert all values, `2 I R1=0.6 V` and the zero local derivative.
- Area-ratio change: set N=64. Since ln(64)=2 ln(8), delta VBE and branch current double; R1/R2 and R1 halve; the synthesized reference remains 1.25 V.
- Resistor scaling: double R2 at unchanged characterized VBE/slope; inferred current halves and synthesized R1 doubles. For physical use, recharacterize VBE/slope if this changes the real bias point.
- Approaching N=1 from above: delta VBE/current tend to zero while required resistor ratio diverges. Exactly N=1 must reject, not divide by zero. Do not impose a made-up minimum N larger than 1.
- Invalid: nonpositive T, R2 or VBE; N<=1; zero or positive VBE slope.

### Mandatory diagram specification

Draw Q1 and Q2 with joined bases at the right-hand Vref node. Q1 emitter joins E directly; Q2 emitter reaches E through R2. Draw R1 from E to ground and label the current in it as static “2I”. Above the collectors draw equal collector resistors to a supply rail and an op-amp servo sensing their lower ends, plus at Q1 collector and minus at Q2 collector. Servo output drives both bases and is Vref. Show no collector-base diode connection: this is not the two-transistor mirror picture.

Exact bindings: N beside Q2 `area_ratio`; R2 `ptat_resistance`; R1 `common_resistor`; each collector arrow `branch_current`; Q1 base-emitter bracket `base_emitter_voltage`; Q1/Q2 VBE-difference annotation `delta_vbe`; output `reference_voltage`. An annotation “local VBE slope” binds `base_emitter_slope`; junction-temperature legend binds `temperature`. Equal collector resistor values and supply voltage are static topology labels, not invented calculated quantities. Place secondary results in the legend to keep the three-device circuit legible.

Caption: “The equal-current servo forces a PTAT voltage across R2. R1 carries both emitter currents, so its contribution has a factor of two. Nonzero operating state and large beta are assumed; startup and curvature are not modeled.”

## 6. `precision.log114_current_ratio` — basic

### Purpose and novelty

Design nominal electrical current compression around the LOG114 with a resistor-derived reference current. Nearest baseline `noise.decibel_to_linear` is unit/ratio conversion without a circuit; here the device enforces virtual-ground current inputs and generates a logarithmic voltage with current-dependent incremental transimpedance. Nearest new TIA has a linear resistor transfer and bandwidth compensation; this frontend intentionally compresses many decades. No physical light conversion is included.

### Topology, law and limits

Use a LOG114 at +5 V/-5 V and 25 °C. Tie VCM IN, COM and VREF GND to ground. Connect its nominal internal 2.5 V VREF output through `reference_resistance` to I2. Positive signal current enters I1. Observe VLOGOUT directly; unused auxiliary amplifiers are not part of this model. Section 7.2.1 permits either input to be signal/reference; this catalog uses I1 as numerator signal and I2 as denominator reference, even though the source's worked application selects the opposite assignment.

At nominal virtual ground:

- `reference_current = 2.5/reference_resistance`.
- `current_ratio = signal_current/reference_current`.
- `log_decades = log10(current_ratio)`.
- `log_output = 0.375 log_decades`.
- `local_transimpedance = dVLOGOUT/dI1 = 0.375/[ln(10) signal_current]`.

For near-equal currents, the catalog evaluates the logarithm as `log1p((I1-I2)/I2)/ln(10)` to retain a small ratio departure. The exact equal-current output is zero. The derivative is local sensitivity, not a claim that the entire large-signal transfer is linear.

Signal current is restricted to the specified 100 pA–10 mA input range. Reference current is restricted to 100 pA–2 mA: the upper bound preserves the internal reference's stated ±2 mA loading condition. Therefore `reference_resistance` lies from 1250 ohm through 25 Gohm. These are physical device limits, not arbitrary numerical clamps. The resulting log output remains inside the dual-supply VLOGOUT swing limits.

Nominal 2.5 V and 0.375 V/decade are used, not guaranteed exact values. Reference tolerance, input bias, offset, log conformity, gain error, leakage and temperature drift are excluded. The 100 pA range is especially sensitive to leakage and the typical 5 pA bias; nominal results must not be described as guaranteed accuracy. VLOGOUT load must be at least 10 kohm with no more than 100 pF; maintain the specified rails and grounding. Current-dependent bandwidth means DC logarithmic conversion is not an arbitrary-speed pulse model.

### Complete independent example

Inputs: `signal_current=0.0001`, `reference_resistance=2500000`.

The 2.5 V reference and 2.5 Mohm resistor generate exactly 1 µA nominally. Signal/reference ratio is 100, or two decades. At 0.375 V/decade, VLOGOUT is 0.75 V. Differentiate the logarithmic current transfer at 100 µA for the local sensitivity.

| Output key | Independent value |
|---|---:|
| `reference_current` | 0.000001 A |
| `current_ratio` | 100 |
| `log_decades` | 2 decades |
| `log_output` | 0.75 V |
| `local_transimpedance` | 1628.6043071371942 ohm |

### Tester cases

- Nominal: assert every value.
- Equal currents: signal=1 µA with nominal reference resistor gives ratio=1, log decades=0 and output=0 exactly; local transimpedance=162860.43071371942 ohm.
- Range endpoints: signal=100 pA gives -1.5 V and signal=10 mA gives +1.5 V with the nominal 1 µA reference.
- One-decade increase adds exactly 0.375 V in the nominal model while decreasing local transimpedance tenfold.
- Reference-resistor boundaries: 1250 ohm gives 2 mA reference current; 25 Gohm gives 100 pA. Both are admitted, subject to all stated external conditions.
- Near unity: use the next representable current above 1 µA; require a positive log output instead of subtraction-induced zero.
- Invalid: signal below 100 pA, above 10 mA, zero/negative/nonfinite; resistor outside [1250,25000000000] ohm or nonfinite.

### Mandatory diagram specification

Draw a LOG114 functional IC with labeled I1 and I2 current-input pins, VREF output, VLOGOUT output, COM, VCM IN, VREF GND and ±5 V supply pins. Draw the resistor externally from VREF to I2, not from a fictitious zero-impedance ground source. Signal-current arrow enters I1. Both input nodes have small “virtual ground” labels; ground COM, VCM IN and VREF GND visibly. The internal block may show two log current converters feeding a difference amplifier, explicitly marked “internal”; it is not an optical sensor.

Exact bindings: input arrow `signal_current`; external resistor `reference_resistance`; I2 arrow `reference_current`; VLOGOUT `log_output`; a side slope annotation `local_transimpedance`. Static “2.5 V nominal” labels VREF; static ±5 V labels identify fixed assumptions, not computed outputs. A ratio annotation may bind `current_ratio`, but never display it as a voltage.

Caption: “Positive currents enter I1 and I2. The internal reference and external resistor set I2; VLOGOUT compresses I1/I2 at nominal 0.375 V/decade. Fixed dual supplies and nominal device behavior are assumed.”

## Handoff and remaining physical risks

All six examples include every input and output in the catalog. Every item has nominal, limiting and invalid-domain guidance plus an actual circuit specification with exact quantity keys. Sources support the stated topologies; derived algebra and approximation choices are explicit. No aliases, inverse duplicates, transducers or generic noise calculations were added.

Implementation must preserve signed directions and the Brokaw factor of two, distinguish Howland load node from amplifier output, check instrumentation internal headroom, and avoid treating TIA compensation as measured stability. The LOG114 scope is deliberately a named device's nominal dual-supply circuit, not a universal log-amplifier model.

Parent should run the six complete examples through the real engine, execute independent boundary tests, and inspect every rendered schematic's wiring and large-text/narrow-window behavior. No worker verification was run. Device matching, actual compensation, leakage, startup, temperature curvature and guaranteed hardware accuracy remain application-level validation requirements, not unresolved algebra or source blockers.
