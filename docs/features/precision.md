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

