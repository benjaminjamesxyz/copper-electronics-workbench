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

