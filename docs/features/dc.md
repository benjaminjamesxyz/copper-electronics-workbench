# DC circuits and resistor networks — executable features

## Scope and evidence

This lane contains exactly eight distinct ideal, static resistive calculations, with three basic, three intermediate and two expert definitions in [the executable catalog](../../backend/catalog/dc.json). The research handoff reports web search and direct reads of university-authored teaching materials and the publisher's original textbook pages, accessed 2026-10-09. Searches covered Ohm's law, resistor dissipation, series/parallel reduction, loading, Thevenin equivalents, maximum power transfer, Wheatstone bridges, delta/star transformation and nodal analysis. English-language sources were used without a publication-date restriction because these are established circuit identities, not time-sensitive device performance claims. Manufacturer bridge pages were sought but could not be directly fetched; they are not needed for or counted as evidence here. An unreadable Harvard scan and unavailable Rhode Island/RPI links were likewise excluded from the adopted references.

These are exact equations within the stated linear lumped-element models, **not empirical component-rating estimates**. Real-device suitability requires checking resistance tolerance, voltage/current/power ratings, source sink capability and nonideal behavior separately. Temperature, thermal derating, noise, PCB trace resistance, semiconductor models, AC impedance, transients, converters and active op-amp summing are outside this lane. No standards-compliance claim is made.

### Adopted primary teaching references

Here “primary teaching reference” means material directly hosted by its university author or original publisher, not a third-party summary; these sources are not presented as new experimental research.

| Ref | Original source and actual URL | Evidence weight and use |
|---|---|---|
| S1 | OpenStax / Rice University, *University Physics Volume 2*, [9.4 Ohm's Law](https://openstax.org/books/university-physics-volume-2/pages/9-4-ohms-law) | High for the linear resistor equation, signed polarity and its nonohmic limitations; original publisher textbook. |
| S2 | OpenStax, [9.5 Electrical Energy and Power](https://openstax.org/books/university-physics-volume-2/pages/9-5-electrical-energy-and-power) | High for electrical work rate and resistor dissipation; part of the same book as S1, not a separate independent experiment. |
| S3 | OpenStax, [10.2 Resistors in Series and Parallel](https://openstax.org/books/university-physics-volume-2/pages/10-2-resistors-in-series-and-parallel) | High for topology-specific reduction from current continuity and voltage equality; same textbook family. |
| S4 | University of Colorado Boulder, [PHYS 3330 Lab 2: Modeling Measurement Systems Using Voltage Dividers](https://physicslabs.colorado.edu/courses/phys-3330/lab-guides/lab2/) | High for source loading and maximum-power matching (§2.4), finite divider loading (§2.6), and divider Thevenin equivalents (§2.7); university laboratory teaching material, last updated August 19, 2026. |
| S5 | William & Mary, [Analog Electronics Chapter 2: Kirchhoff's Laws and Thevenin's Theorem](https://physics.wm.edu/~evmik/classes/Physics_252_Analog_Electronics/lab_manuals/LabManual_Chpt2.pdf) | High for KCL/KVL, the two-source/three-resistor circuit (pp. 1–2), the Thevenin load relation (pp. 3–4), and Wheatstone exercises (pp. 6–7). The loaded-bridge midpoint equations below are explicitly derived here, not claimed to be printed in this source. |
| S6 | Loai G. Salem, UCSB, [ECE 10A Lecture 12](https://web.ece.ucsb.edu/~parhami/pres_folder/10A_lecture12_notes.pdf), pp. 54–61 | High for delta/star terminal-equivalence derivation and incident-arm product over delta sum; original university lecture notes. |
| S7 | Prof. Rich Kozick, Bucknell University, [ELEC 101: Outline of the Nodal Analysis Method](https://www.eg.bucknell.edu/~kozick/elec10105/nodevoltage.html) | High for systematic signed KCL and resistor-current construction; original faculty course page. The specific two-source closed form is derived here using this method. |

S4, S5, S6 and S7 are separately authored course materials; their agreement follows common conservation laws, not a count of empirical replications. No conflicting circuit identity was found. One wording caveat matters: S5 calls a maximum-power match “most efficient transfer” but defines it by maximizing watts; it is **not** maximum energy efficiency. This catalog uses the unambiguous maximum-load-power interpretation, supported by S4 and differentiation below. Confidence in the equations is high within the ideal linear models; these references do not establish real package safety or accuracy near floating-point cancellation.

## Common units, domain and expression rules

All resistances are in **ohm**, voltages in **V**, currents in **A**, power in **W**, conductance in **S**, and ratios in dimensionless unit **1**. JSON input/output keys are distinct ASCII snake_case identifiers. Each output is evaluated in its listed order; supporting outputs are intentionally included in the complete expected-output dictionaries. Constraints use **input keys only**, so they are safe to evaluate before outputs.

The catalog accepts resistances from `1e-9` to `1e12` ohm, signed voltages from `-1e6` to `1e6` V, and (in the power calculation) signed current from `-1e6` to `1e6` A. These generous finite limits are a **numerical operating envelope**, not limits of circuit physics or assertions about realizable safe components. They prevent extreme finite binary64 inputs from causing overflow/underflow in products, reciprocals and denominators. Positive resistance bounds and explicit constraints guard all divisions. There are no logarithms, square roots, conditional expressions or singular balanced-bridge divisions. Shorts and infinite open loads are intentionally not inputs; the finite-load model's limits are described analytically instead.

All defaults equal their respective worked-example inputs, lie within these bounds, and give positive denominators by construction. Example values were independently obtained from the rational arithmetic and circuit equations shown below, not from the backend expression evaluator. JSON uses relative tolerance `0.000001` for the independent expected values. Implementation preserves the researched equations and examples. No build, lint, automated test, formatter, smoke calculation or application run was performed by this lane; integration verification belongs to Main.

## Execution and domain errors

The Rust shared expression engine evaluates the catalog; there is no lane evaluator or frontend numerical implementation. Required finite input keys must match exactly. Resistance bounds reject shorts, negative resistances, and out-of-envelope values before arithmetic. Signed voltage/current bounds preserve zero and polarity reversal; violating a numerical bound is an input error, not an automatic clamp or component failure. All divisors are positive because each resistance is positive. Constraints use inputs only; sequential outputs reuse prior results. For cancellation near bridge balance or opposing-source node zero, relative precision cannot be guaranteed; use a physically scaled absolute tolerance in independent boundary checks rather than rejecting legitimate balance or backfeed.

## 1. `dc.ohm_current` — Signed Ohm-law current (basic)

**Design question:** What direction and magnitude of DC current flows through one known resistor?

**Model and derivation:** Under the passive sign convention, `V = I R`; therefore `I = V/R`. Positive current enters the resistor terminal designated positive voltage. Reversing source polarity reverses current. This is a constitutive relation for an ohmic device, not a universal semiconductor model. Source: S1.

**Inputs:** `voltage` (V), `resistance` (ohm). **Output:** `current` (A).

**Domain:** `resistance > 0`; voltage can be positive, negative or zero. The minimum resistance and maximum voltage additionally enforce the common finite numerical envelope. Resistance is constant; noise, temperature and parasitics are not included.

**Independent example/default:** `voltage = -3.3 V`, `resistance = 2200 ohm`.

`I = (-33/10)/2200 = -33/22000 = -0.0015 A`.

The negative sign means current flows opposite the chosen positive direction. This is the only generic inverse-Ohm-law calculator in the lane; other rearrangements are not separately counted.

### Complete expected output map

| Output key | Expected value | Unit |
|---|---:|---|
| `current` | `-0.0015` | A |

Relative tolerance: `1e-6`; these values are the independently derived research oracle, not engine observations.

## 6. `dc.loaded_wheatstone_bridge` — Loaded Wheatstone bridge and common-mode voltage (expert)

**Design question:** What differential signal and common-mode voltage actually appear at an unbalanced bridge when the detector has finite resistance?

**Unambiguous topology and signs:** `R1` connects excitation-to-left midpoint, `R2` left-to-ground, `R3` excitation-to-right midpoint, `R4` right-to-ground. `Rd` connects left directly to right, with no separate connection to ground. Detector current is positive left-to-right; differential voltage is left minus right.

**Derivation:** Apply the divider Thevenin equivalent separately to the two arms:

`Va0 = Vs R2/(R1+R2)`; `Vb0 = Vs R4/(R3+R4)`;

`Ra = R1 R2/(R1+R2)`; `Rb = R3 R4/(R3+R4)`.

The floating detector closes a loop between the two Thevenin outputs, so `Vopen = Va0-Vb0`, `Routput = Ra+Rb`, and `Id = Vopen/(Routput+Rd)`. The actual midpoints are `Va = Va0-Id Ra`, `Vb = Vb0+Id Rb`. Hence `Vloaded = Id Rd = Va-Vb`, and `Vcommon = (Va+Vb)/2`.

These equations are this report's explicit derivation from S4's divider equivalents and S5's KCL/KVL and Thevenin load relation. S5 also includes Wheatstone bridge design/laboratory exercises. No unsourced manufacturer behavior is assumed.

**Inputs:** `v_excitation` (V), `r_1` through `r_4`, `r_detector` (ohm). **Outputs:** `v_open`, `v_loaded`, `v_left`, `v_right`, `v_common` (V); `r_output` (ohm); `i_detector` (A).

**Domain and limitations:** All five resistances positive and finite; excitation can have either sign or be zero. Balanced bridge condition is `R1 R4 = R2 R3`, making both differential voltage and detector current exactly zero in the mathematical model. Because no expression divides by `Vopen`, this case is supported. Near balance the subtraction of nearly equal ratios can lose relative binary64 precision. A real amplifier with grounded or unequal input impedances, input bias currents, finite excitation impedance or sensor lead resistances does not satisfy this topology. No strain-gauge temperature or noise modeling is included.

**Independent example/default:** `Vs = 10 V`, `R1 = R2 = R3 = 1000 ohm`, `R4 = 3000 ohm`, `Rd = 1250 ohm`.

- Open midpoints are `5 V` and `7.5 V`, so `Vopen = -2.5 V`.
- `Ra = 500 ohm`, `Rb = 750 ohm`, so `Routput = 1250 ohm`.
- The detector current is `-2.5/(1250+1250) = -0.001 A`; `Vloaded = -1.25 V`.
- The loaded midpoints are `Va = 5-(-0.001)(500) = 5.5 V`, `Vb = 7.5+(-0.001)(750) = 6.75 V`.
- `Vcommon = (5.5+6.75)/2 = 6.125 V`.
- Independent original-network KCL confirms these numbers: at the left, `(10-5.5)/1000 = 5.5/1000 + (5.5-6.75)/1250 = 0.0045 A`; at the right, `(10-6.75)/1000 + (5.5-6.75)/1250 = 6.75/3000 = 0.00225 A`.

### Complete expected output map

| Output key | Expected value | Unit |
|---|---:|---|
| `v_open` | `-2.5` | V |
| `r_output` | `1250.0` | ohm |
| `i_detector` | `-0.001` | A |
| `v_loaded` | `-1.25` | V |
| `v_left` | `5.5` | V |
| `v_right` | `6.75` | V |
| `v_common` | `6.125` | V |

Relative tolerance: `1e-6`; these values are the independently derived research oracle, not engine observations.

## 8. `dc.two_source_node` — Two-source resistive node with load (expert)

**Design question:** What voltage and backfeed currents occur when two independently driven source/resistor branches share a resistively loaded node?

**Topology:** Source 1 voltage `V1` connects through `R1` to the node; source 2 voltage `V2` connects through `R2`; `Rl` connects the node to common ground. Both ideal source voltages are specified relative to this same ground. Positive source current flows toward the node; positive load current flows toward ground.

**Derivation:** Signed KCL is

`(V1-V)/R1 + (V2-V)/R2 = V/Rl`.

Collect terms: `G = 1/R1+1/R2+1/Rl`, `Vnode = (V1/R1+V2/R2)/G`; then `I1 = (V1-Vnode)/R1`, `I2 = (V2-Vnode)/R2`, `Il = Vnode/Rl`. This is Millman's conductance-weighted node solution with the load treated as a zero-volt branch. Sources: S7's nodal method and S5's directly relevant two-source/three-resistor circuit. This is **passive summing**, not an op-amp summing amplifier or virtual-ground circuit.

**Inputs:** `v_1`, `v_2` (V); `r_1`, `r_2`, `r_load` (ohm). **Outputs:** `g_total` (S), `v_node` (V), `i_source_1`, `i_source_2`, `i_load` (A).

**Domain and limitations:** Positive finite branch/load resistances guarantee `G > 0`. Signed source voltages and signed source currents are necessary: an opposing source can sink current. Ideal-source assumptions are only applicable if the actual sources can support those calculated sink currents; diode-isolated supplies or non-sinking regulators need different models. Node-voltage cancellation is allowed but can lose relative floating-point accuracy near zero. No active amplifier, transients or current limits are included.

**Independent example/default:** `V1 = 12 V`, `R1 = 1000 ohm`, `V2 = 3 V`, `R2 = Rl = 2000 ohm`.

- Multiplying original KCL by 2000 gives `2(12-V)+(3-V) = V`, hence `27 = 4V`, or `Vnode = 6.75 V`.
- `G = 1/1000+1/2000+1/2000 = 0.002 S`.
- `I1 = (12-6.75)/1000 = 0.00525 A`.
- `I2 = (3-6.75)/2000 = -0.001875 A`, meaning source 2 must sink 1.875 mA.
- `Il = 6.75/2000 = 0.003375 A`.
- KCL cross-check: `0.00525-0.001875 = 0.003375 A`.

### Complete expected output map

| Output key | Expected value | Unit |
|---|---:|---|
| `g_total` | `0.002` | S |
| `v_node` | `6.75` | V |
| `i_source_1` | `0.00525` | A |
| `i_source_2` | `-0.001875` | A |
| `i_load` | `0.003375` | A |

Relative tolerance: `1e-6`; these values are the independently derived research oracle, not engine observations.

