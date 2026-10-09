# Magnetics and transformers — executable feature catalog

Executable catalog: [backend/catalog/magnetics.json](../../backend/catalog/magnetics.json). Research provenance: [definitions](../../research/magnetics.json) and [derivations](../../research/magnetics.md). This lane contains exactly eight distinct physical design problems: ideal transformer transformation; sinusoidal flux excitation; ungapped toroidal geometry; discrete-gap magnetic reluctance; material-volume field energy; conductor penetration; winding VA allocation; and partial magnetic coupling. Levels: two basic, three intermediate, three expert.

## Research scope and evidence

The supplied research handoff records a search performed 2026-10-09 using web search and direct source reads; implementation preserves that source register and its evidence caveats, without claiming a fresh source-access review. English university teaching material, university research and magnetic-component manufacturers were searched for turns ratios, Faraday's-law waveform factor, rectangular toroid integration, finite-permeability air gaps, field energy density, good-conductor skin depth, winding VA and coupled-coil inductance. No publication-date cutoff was imposed because the closed-form linear models are classical; contemporary operating conditions must still come from component/material data. Commercial calculator pages, unsourced formula aggregators and Wikipedia mirrors were excluded. This is a focused formula review, not an exhaustive literature or standards review.

### Source quality and actual support

| Source | Evidence type and weight | Relevant support |
|---|---|---|
| [MIT 6.014 Lecture 11](https://web.mit.edu/6.014/www/handouts/lecture11_notes.pdf) | University-authored teaching derivations; high weight for ideal linear models, not measured component accuracy | Faraday voltage, ideal turns ratio, magnetic energy density, toroidal and gap models |
| [MIT 8.02 Inductance Worked Examples](https://web.mit.edu/course/8/8.02-esg/Spring03/www/8.02ch32we.pdf) | University-authored worked derivations; high weight within stated geometry | Example 2 integrates the rectangular air-core toroid exactly; examples 1 and 3 show field energy density |
| [MIT Electromagnetic Fields and Energy §9.7](https://www.mit.edu/course/6/6.013_book/www/chapter9/9.7.html) | Authoritative university textbook treatment; high model weight | Finite-permeability air-gap circuit, flux continuity, and linear two-winding linkage matrix. Equations are embedded images; accompanying text states the assumptions |
| [Micrometals Issue L, February 2007](https://web.mit.edu/6.131/www/document/micrometals.pdf) | Original manufacturer engineering guide; high weight for its waveform convention, product-specific for loss data | Printed page 29 relates sine RMS voltage to peak flux, explains 4.44 and SI/CGS conversion. No loss fits are imported |
| [MIT CBA Circuits, Transmission Lines, and Waveguides](https://fab.cba.mit.edu/classes/862.19/notes/circuits.pdf) | University-authored electromagnetic derivation; high weight for conduction-dominant materials | §7.2.1, equations 7.26–7.35 derive diffusion, skin depth and exponential penetration; room-temperature copper conductivity example |
| [Hammond 265 transformer family](https://www.hammfg.com/electronics/transformers/power/265) | Original manufacturer rating tables; high weight for practical RMS/VA interpretation, not universal thermal margin | 5 V × 30 A = 150 VA and 24 V × 10 A = 240 VA examples; series/parallel wiring keeps overall VA constant |
| [Hammond rectifier design guide](https://www.hammfg.com/electronics/transformers/rectifier) | Original manufacturer application guidance; high weight for its caveats | AC voltage references are RMS; winding AC current must be recalculated from DC load current according to rectifier/filter topology |
| [UC Berkeley EECS-2007-3](https://www2.eecs.berkeley.edu/Pubs/TechRpts/2007/EECS-2007-3.pdf) | Original university dissertation with a theoretical subsection; high weight for the coupling definition, no empirical accuracy claim | §3.2.2 equations 3.6–3.8 define reciprocal mutual inductance and k = M/sqrt(L1 L2), with 0 ≤ k ≤ 1 |
| [Stanford EE102 exercises](https://stanford.edu/~boyd/ee102/101exercises.pdf) | University-authored teaching problems; supporting rather than independent proof | Static-circuit exercise 10 explicitly asks for primary-referred transformer load resistance |

The research author reports that all nine cited documents/pages were read. MIT sources share a classical theory base and are not counted as independent experiments. The Berkeley coupling result agrees with the MIT linear linkage formulation. No conflicting ideal-model formulas were found in the included sources. Main evidence gap: these sources do not establish a universal numerical error bound for finite-core fringing, material linearity, or transformer thermal headroom. Consequently user-supplied limits and explicitly labeled conservative screening rules are used; no certification or empirical precision is implied.

## Shared conventions and implementation contract

- SI inputs throughout: volts, amperes, hertz, meters, square/cubic meters, henries, teslas, webers, S/m. VA is apparent winding load, not watts. Turns and relative material constants are dimensionless; `turn` is a human-facing count unit.
- Use the engineering approximation mu0 = 4π × 10^-7 H/m. Its use does not imply that mu0 is an exactly defined modern SI constant. Skin-depth screening also uses eps0 = 8.8541878128 × 10^-12 F/m.
- All divisions and roots have positive input-domain guards; toroid geometry guards the logarithm. Actual turns are positive integers, enforced with `floor(turns) == turns`.
- Constraints contain input keys/constants only, because the integration engine checks them before evaluating outputs. Output expressions may use preceding outputs. Every example supplies all inputs and all expected output keys.
- Material limits are supplied by the user. The catalog can enforce those limits but cannot verify that permeability, saturation margin, frequency dependence, temperature, or hysteresis assumptions actually describe the selected material.
- The independent examples below are preserved from the research handoff, which derived them with standalone high-precision decimal arithmetic and hand reductions, not the backend expression engine. Defaults equal worked-example inputs. Tolerance 10^-6 is an arithmetic comparison tolerance, not physical model accuracy.
- Only the contract operators/functions are used. There is no simulation, table interpolation, iteration or piecewise branch.

The shared Rust engine alone evaluates these expressions; Flutter consumes catalog metadata and results. All input keys are required exactly once, must be finite and must satisfy bounds before input-only constraints execute. Outputs execute in their listed order. Nonfinite/unrepresentable numerical results are errors, not hardware estimates. No build, lint, test, formatter or smoke command was run by this implementation lane.

## `magnetics.turns_ratio` — Ideal transformer turns ratio and reflected load (basic)

ID: `magnetics.turns_ratio`.

**Question:** What secondary voltage and primary-referred resistive load result from specified winding turns?

**Derivation:** Perfect coupling makes both coils link the same changing flux. Faraday's law gives Vp = Np dΦ/dt and Vs = Ns dΦ/dt, so a = Vs/Vp = Ns/Np. Lossless power transfer gives Ip = a Is in magnitude. With a resistive load, Is = Vs/Rload; therefore Vp/Ip = Rload/a². This reflected impedance is an output of the same ideal transformation, not counted as another calculator.

**Formula and outputs:**

- `voltage_ratio = secondary_turns/primary_turns` [1].
- `secondary_voltage = primary_voltage*voltage_ratio` [V RMS].
- `reflected_resistance = load_resistance/voltage_ratio/voltage_ratio` [ohm].

**Input units and domain:** Np, Ns [turn] ≥ 1 and integral; Vp [V RMS] ≥ 0; Rload [ohm] > 0. No DC operation: an ideal transformer steady-state voltage ratio is not a DC power-transfer model. Perfect coupling, negligible magnetizing current, winding drops, leakage, saturation and regulation are assumed. Output voltage is a magnitude; dots/polarity are not resolved here.

**Worked example/defaults:** Np = 1000, Ns = 100, Vp = 230 V, Rload = 24 ohm.

1. a = 100/1000 = **0.1**.
2. Vs = 230 × 0.1 = **23 V RMS**.
3. Rprimary = 24/(0.1²) = **2400 ohm**.

**Sources:** [MIT Lecture 11, section B](https://web.mit.edu/6.014/www/handouts/lecture11_notes.pdf); [Stanford exercise 10](https://stanford.edu/~boyd/ee102/101exercises.pdf).

### Executable input metadata

| Key | Meaning | Unit | Default / worked input | Admissible bounds |
|---|---|---|---:|---|
| `primary_turns` | Primary turns | turn | 1000.0 | >= 1.0 |
| `secondary_turns` | Secondary turns | turn | 100.0 | >= 1.0 |
| `primary_voltage` | Primary RMS voltage | V | 230.0 | >= 0.0 |
| `load_resistance` | Secondary load resistance | ohm | 24.0 | > 0.0 |

**Model-domain and error rationale:** Positive integer turns rule out missing/fractional windings and guard the ratio. Positive load resistance excludes short circuits. Zero primary RMS voltage is admissible and produces zero secondary voltage while the reflected load remains defined.

**Pre-output, input-only constraints:**

- `floor(primary_turns) == primary_turns` — Primary turns must be a positive integer.
- `floor(secondary_turns) == secondary_turns` — Secondary turns must be a positive integer.

**Conditioning:** The implementation computes Rload/a/a rather than Rload/(a²). For Np=1e200, Ns=1 and Rload=1e-300 ohm, the mathematical reflected load is 1e100 ohm even though a² would underflow in binary64. The sequential divisions preserve the model without adding an artificial turns cap.

### Complete independent worked output oracle

All inputs are the default/worked input column above; every result is named below in evaluation order. Relative arithmetic tolerance: **1e-6**, not a physical accuracy claim.

| Output key | Meaning | Unit | Expected value |
|---|---|---|---:|
| `voltage_ratio` | Secondary / primary voltage ratio | 1 | 0.1 |
| `secondary_voltage` | Secondary RMS voltage | V | 23.0 |
| `reflected_resistance` | Primary-referred load resistance | ohm | 2400.0 |


## `magnetics.rectangular_toroid_inductance` — Rectangular-section toroid inductance (expert)

ID: `magnetics.rectangular_toroid_inductance`.

**Question:** What is the inductance of an ungapped toroidal core when radial field variation is retained?

**Derivation:** For inner radius ri, outer radius ro and axial height h, Ampère's law gives H(r) = N I/(2πr), and linear material gives B(r) = mu0 mur N I/(2πr). Integrate flux through one turn:

Φ = ∫ri^ro B(r) h dr = mu0 mur N I h ln(ro/ri)/(2π).

Then L = N Φ/I = mu0 mur N² h ln(ro/ri)/(2π). The MIT worked example derives the air-core formula; replacing mu0 with mu0 mur is justified by the constant linear constitutive relation, not by a manufacturer-specific empirical fit. Maximum B occurs at ri, so the current-specific inner field is also reported and guarded.

**Formula and outputs:**

- `inductance = 4*pi*1e-7*relative_permeability*turns^2*height*ln(outer_radius/inner_radius)/(2*pi)` [H].
- `inner_flux_density = 4*pi*1e-7*relative_permeability*turns*peak_current/(2*pi*inner_radius)` [T].

**Input units/domain:** ri, ro, h [m] > 0; ro > ri; mur [1] > 0; N [turn] ≥ 1 integral; Ipk [A] ≥ 0; linear_flux_limit [T] > 0. The expanded inner-field expression must be ≤ the limit. Distances are radii, not diameters; their ratio is identical if both are consistently converted, but the field guard requires actual radius.

**Limitations:** Homogeneous ungapped rectangular core; evenly distributed winding; toroidal symmetry; negligible leakage, fringing, core losses and winding capacitance; real constant permeability appropriate to the current and frequency. The logarithmic formula is exact within that ideal field model, not exact for every manufactured toroid. It differs physically from the discrete-gap reluctance calculator below and is not an inverse rearrangement.

**Worked example/defaults:** ri = 0.01 m, ro = 0.02 m, h = 0.005 m, mur = 100, N = 50, Ipk = 0.1 A, limit = 0.2 T.

- mu0/(2π) = 2 × 10^-7 H/m.
- L = (2 × 10^-7) × 100 × 2500 × 0.005 × ln(2) = 0.00025 ln(2) = **0.00017328679513998633 H** (173.286795 µH).
- Binner = (2 × 10^-7) × 100 × 50 × 0.1 / 0.01 = **0.01 T**, inside the selected linear domain.

**Sources:** [MIT 8.02 example 2](https://web.mit.edu/course/8/8.02-esg/Spring03/www/8.02ch32we.pdf); [MIT Lecture 11, linear magnetic material and toroidal sections](https://web.mit.edu/6.014/www/handouts/lecture11_notes.pdf).

### Executable input metadata

| Key | Meaning | Unit | Default / worked input | Admissible bounds |
|---|---|---|---:|---|
| `inner_radius` | Core inner radius | m | 0.01 | > 0.0 |
| `outer_radius` | Core outer radius | m | 0.02 | > 0.0 |
| `height` | Core axial height | m | 0.005 | > 0.0 |
| `relative_permeability` | Linear relative permeability | 1 | 100.0 | > 0.0 |
| `turns` | Winding turns | turn | 50.0 | >= 1.0 |
| `peak_current` | Peak winding current | A | 0.1 | >= 0.0 |
| `linear_flux_limit` | Material linear-region flux limit | T | 0.2 | > 0.0 |

**Model-domain and error rationale:** Strictly positive dimensions and outer radius greater than inner radius guard the logarithm and ensure a nonempty annular core. Integer turns represent a real winding. Positive permeability and field limit define the linear model. Nonnegative peak current permits zero-current operation; the inner-radius field guard rejects operation outside the supplied linear material regime.

**Pre-output, input-only constraints:**

- `outer_radius > inner_radius` — Outer core radius must exceed inner core radius.
- `floor(turns) == turns` — Turns must be a positive integer.
- `4*pi*1e-7*relative_permeability*turns*peak_current/(2*pi*inner_radius) <= linear_flux_limit` — Inner-radius field exceeds the supplied linear-material limit.


### Complete independent worked output oracle

All inputs are the default/worked input column above; every result is named below in evaluation order. Relative arithmetic tolerance: **1e-6**, not a physical accuracy claim.

| Output key | Meaning | Unit | Expected value |
|---|---|---|---:|
| `inductance` | Toroidal inductance | H | 0.00017328679513998633 |
| `inner_flux_density` | Peak field at core inner radius | T | 0.01 |


## Lane boundary and parent verification handoff

Magnetic energy is specifically field density/region energy, not transients' circuit stored-energy formula. No passive AC reactance/resonance, power-factor calculator, switch-mode converter inductor sizing, PCB trace resistance or RF transmission-line impedance is included. Ungapped radial toroid integration and discrete-gap magnetic-circuit modeling are separate reserved geometries, not inverse versions of one calculation. Transformer VA is winding allocation; turns ratio is ideal voltage/impedance transformation; coupled-coil inductance is finite-k linkage and wiring.

Parent should evaluate every full example/default, all input-bound and constraint rejections, zero-excitation/zero-field/zero-load cases, integral turns, radial ordering, both gap geometry screens, field limits, the conduction-dominant boundary, coupling endpoints and the documented numerical-conditioning probes after the complete wave. These scenarios are prescribed, not executed or passed by this lane.
