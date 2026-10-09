# Bipolar transistor design: executable feature reference

## Scope, conventions and implementation

This lane contains exactly eight distinct physical design problems, with three basic, three intermediate and two expert calculations. Definitions are in [the executable catalog](../../backend/catalog/bjt.json). A DC operating point, a saturated-switch drive requirement, incremental device parameters, an inverting degenerated stage, terminal dissipation, collector Early resistance, matched-pair large-signal steering and a noninverting emitter buffer are different problems, not inverse rearrangements. The two amplifier stages use different output nodes and feedback/loading topologies. Several supporting outputs within one problem are deliberately bundled, not counted as additional calculators.

All voltages, currents, resistances, temperatures, conductances and powers use V, A, ohm, K, S and W respectively. Gain is V/V; beta, alpha, ratios and fractional errors are dimensionless. NPN signs are used. PNP use requires consistent transformation to magnitudes and reversed polarities; these definitions do not directly solve signed PNP networks. `beta` is DC current gain; `beta_ac` is incremental current gain. Neither should be assumed to be a universal device constant.

The exact SI constants are `k = 1.380649e-23 J/K` and `q = 1.602176634e-19 C` [S5, S6]. Consequently `VT = k*T/q`, and at 300 K, `VT = 0.0258519997864355323 V`. This is more precise than the 26 mV lecture approximation. Temperature-based models use an explicit 200–450 K catalog envelope, not a claim that any particular transistor is rated over that range.

The Rust shared engine evaluates all catalog expressions; Flutter consumes its metadata and results without its own formula implementation. Every expression uses only the contract's scalar arithmetic and supported functions. Outputs are ordered for dependency evaluation; all constraints use input keys only because constraints execute before outputs. Every example specifies all inputs and every output. Positive resistance/current denominators are protected by input bounds; the matched-pair exponential argument is explicitly bounded. Device breakdown, safe operating area, temperature-dependent beta/Vbe, high injection and actual small-signal amplitude cannot be inferred from these few inputs: each model states those external preconditions rather than claiming numerical validation is device qualification.

The numerical worked examples below were calculated independently using high-precision decimal arithmetic, charge conservation and hand-substituted circuit equations, not a catalog evaluator. They are the finite, valid default cases in the JSON. No build, lint, tests, formatter or application smoke run was performed.

## Primary references

- **S1 — MIT 6.101, Lecture 3 (Spring 2019), BJT part 1.** Sections “BJT Current Relationship,” “Early Voltage,” “Two Resistor Biasing,” and slides 42–44 on hybrid-pi, emitter degeneration and common collector. https://web.mit.edu/6.101/www/s2019/handouts/L03_4.pdf
- **S2 — Ron Roscoe, MIT Low Frequency Hybrid-pi Equation Chart (2005; OCW Spring 2007).** CE with emitter resistance and common-collector models. https://ocw.mit.edu/courses/6-101-introductory-analog-electronics-laboratory-spring-2007/1bc47ee7271866912a6eb9d2d1596ff5_hybrid_pi.pdf
- **S3 — onsemi AND9129/D, Understanding a Digital Transistor Datasheet (December 2013).** Distinguishes active hFE from saturation test conditions; resistor drive examples and driver capability. https://www.onsemi.com/download/application-notes/pdf/and9129-d.pdf
- **S4 — onsemi TND6093/D, Low VCE(sat) BJTs in Automotive Applications (December 2016).** Board-design discussion of base-drive current and collector dissipation. https://www.onsemi.com/download/white-papers/pdf/tnd6093-d.pdf
- **S5 — NIST, 2022 CODATA Boltzmann constant.** Exact SI numerical value. https://physics.nist.gov/cgi-bin/cuu/Value?k
- **S6 — NIST, 2022 CODATA elementary charge.** Exact SI numerical value. https://physics.nist.gov/cuu/Constants/Value/e.html
- **S7 — Clifton Fonstad, MIT 6.012 Lecture 19, Differential Amplifier Stages (November 17, 2009), slides 10–11.** Large-signal emitter-coupled pair with alpha_F and tail emitter-current conservation. https://ocw.mit.edu/courses/6-012-microelectronic-devices-and-circuits-fall-2009/9932f87a7b5ec3d088d0e0836a05c622_MIT6_012F09_lec19.pdf

The research handoff reports that sources were retrieved directly and read; implementation preserves that evidence without claiming a new source retrieval. onsemi notes are evidence for models and specification interpretation, not a device-independent guarantee of saturation or safety. Source S3 concerns resistor-equipped digital transistors; the external-resistor calculator below explicitly omits their internal base-emitter shunt and derives the corresponding plain-BJT topology.



## 2. `bjt.saturated_switch_drive` — base drive and external resistor limit

**Level:** basic. **Purpose:** size the base drive of a plain, grounded-emitter NPN switch. This is one saturated-switch calculation, not a second active-bias inverse formula.

### Derivation and units

Choose a forced current ratio supported by the selected transistor's saturation specification, `beta_forced=Ic/Ib`. Then

`Ib_required = Ic/beta_forced` in A.

For a Thevenin driver with minimum open-circuit/source voltage Vdrive_min and series resistance Rdriver, KVL gives

`Vdrive_min = Vbe_sat_max + Ib_required*(Rdriver+Rb)`.

Hence `Rb_max=(Vdrive_min-Vbe_sat_max)/Ib_required-Rdriver` in ohm, and resistor dissipation at that boundary is `Ib_required^2*Rb_max` in W. S3 provides current-drive and resistor-voltage principles; S4 emphasizes driver source-current capability. The driver-series correction is a direct KVL extension.

### Domain and limitations

Ic, forced beta, source capability and on-state Vbe are positive; Rdriver is nonnegative. The headroom constraint requires a strictly positive external Rb. Required Ib must not exceed supplied driver capability. A forced beta of 10 is an example, not a universal rule or guarantee: choose the ratio from guaranteed Vce(sat) test conditions at appropriate Ic and temperature. There is no internal base-emitter shunt in this topology. Round the real resistor downward and evaluate tolerances, maximum driver voltage/current, resistor worst-case power, switching-storage delay and the load/SOA separately. The reported resistor power is specifically at the minimum-drive boundary, not worst-case maximum-drive dissipation.

### Independent worked example

Inputs: Ic=0.1 A, forced beta=10, Vdrive_min=3.3 V, Vbe_sat_max=0.9 V, Rdriver=20 ohm, available driver current=0.02 A.

- `Ib_required=0.1/10=0.01 A`.
- Total drive-path resistance is `(3.3-0.9)/0.01=240 ohm`.
- External `Rb_max=240-20=220 ohm`.
- `Prb=(0.01)^2*220=0.022 W`.
- `0.01 A <= 0.02 A`; driver drop is 0.2 V, leaving 2.2 V across Rb.

### Complete executable example oracle

All input values above equal the catalog defaults; relative tolerance is `1e-6` (numerical agreement, not physical accuracy). Every sequential output is listed below.

| Output key | Independently derived expected value | Unit |
|---|---:|---|
| `i_base_required` | `0.01` | A |
| `r_base_max` | `220` | ohm |
| `p_base_resistor` | `0.022` | W |


## 6. `bjt.early_output_resistance` — collector incremental resistance

**Level:** expert. **Purpose:** distinguish differential collector resistance from DC Vce/Ic and show the correction to the usual VA/Ic approximation.

### Derivation and units

S1 illustrates approximately straight output curves extrapolating to negative Early voltage and gives `ro≈VA/Ic`. Let VA be its positive magnitude. At fixed Vbe and temperature, the affine straight-line model is

`Ic(Vce)=I0*(1+Vce/VA)`.

Then `go=dIc/dVce=I0/VA`. Because the supplied Ic is measured at the supplied Vce, `I0=Ic/(1+Vce/VA)`, so

`go=Ic/(VA+Vce)` in S, `ro=(VA+Vce)/Ic` in ohm.

The source's common approximation is `ro_approx=VA/Ic`. Its fractional underestimate relative to the full affine-model value is `Vce/(VA+Vce)`. Retaining Vce is a stated algebraic refinement of the source's straight-line model; it is not a claim that real BJT characteristics obey an exact global law.

### Domain and limitations

VA, Ic, Vce and Vbe are positive; require Vce>Vbe to keep the collector above base. VA is neither a breakdown voltage nor the current operating Vce. No artificial Vce<<VA restriction is imposed on the corrected expression; the approximation error is returned explicitly. Device output curves still must be locally approximately affine, below breakdown and high injection. Collector series resistance, thermal slope and nonlinear Early-voltage dependence are omitted.

### Independent worked example

Inputs: VA=100 V, Vce=5 V, Vbe=0.7 V, measured Ic=0.001 A.

- `I0=1 mA/1.05=0.952380952380952 mA`.
- `go=I0/100=9.52380952380952 uS`.
- `ro=1/go=105000 ohm`.
- Approximation: `ro_approx=100000 ohm`.
- Fractional underestimate: `(105000-100000)/105000=0.0476190476190476` (4.7619047619%).

In contrast `Vce/Ic=5000 ohm` is DC resistance, not this small-signal ro.

### Complete executable example oracle

All input values above equal the catalog defaults; relative tolerance is `1e-6` (numerical agreement, not physical accuracy). Every sequential output is listed below.

| Output key | Independently derived expected value | Unit |
|---|---:|---|
| `r_output` | `105000` | ohm |
| `g_output` | `9.523809523809523e-06` | S |
| `r_output_approx` | `100000` | ohm |
| `approximation_relative_error` | `0.047619047619047616` | 1 |


## 7. `bjt.differential_pair_steering` — large-signal matched-pair currents

**Level:** expert. **Purpose:** model static nonlinear current splitting in a matched discrete emitter-coupled pair, rather than generic op-amp gain.

### Derivation and units

Matched devices at equal temperature have the same saturation-current factor. Since their emitters share a node,

`Vbe1-Vbe2=Vbase1-Vbase2=Vdiff` and `Ic1/Ic2=exp(Vdiff/VT)`.

The tail current here is total **emitter** current, as in S7 slides 10–11. For equal constant beta, `alpha=beta/(beta+1)` and `Ic1+Ic2=alpha*Itail`. Define `a=exp(Vdiff/VT)`. Solving current conservation gives

`Ic1=alpha*Itail*a/(1+a)`, `Ic2=alpha*Itail/(1+a)` in A.

Then `Idiff=Ic1-Ic2`. Differentiating the current difference gives local large-signal transfer slope

`gdiff=2*alpha*Itail*a/(VT*(1+a)^2)` in S.

This is equivalent to the source's tanh relation, but implemented with the permitted `exp` function because `tanh` is not in the contract. Finite beta prevents incorrectly treating an emitter tail current as an equal collector-current sum.

### Domain and limitations

Tail current and beta are positive; T is 200–450 K. Enforce `abs(Vdiff)/VT<=40` before exponentiation. The bound is a numerical/model envelope, not a measured cutoff boundary; leakage and mismatch dominate sufficiently small branch currents in real devices. Equal devices, equal constant beta, ideal tail source, no emitter degeneration and adequate collector/tail-source compliance are explicit physical preconditions. The inputs do not include collector supplies or common-mode voltage, so compliance must be established externally, not inferred. The pair is a large-signal circuit calculator and is not constrained to Vdiff<<VT.

### Independent worked example

Inputs: total emitter tail=0.002 A, matched beta=100, T=300 K, Vdiff=0.02 V.

- `alpha=100/101=0.990099009900990`.
- `a=exp(0.02/0.0258519997864355)=2.16763029616484`.
- Total collector current is `0.002*(100/101)=0.00198019801980198 A`.
- Allocate this sum in the ratio `2.16763029616484:1`: `Ic1=0.001355062560591518 A`, `Ic2=0.000625135459210462308 A`.
- `Idiff=0.000729927101381055582 A`.
- Independent slope calculation using `2*Ic1*Ic2/(VT*(Ic1+Ic2))` gives `gdiff=0.0330948723388099594 S`.

Useful physical identities for parent validation: changing Vdiff's sign swaps the two collector currents; zero differential input gives equal currents `alpha*Itail/2`; the collector-current sum remains `alpha*Itail`, not Itail.

### Complete executable example oracle

All input values above equal the catalog defaults; relative tolerance is `1e-6` (numerical agreement, not physical accuracy). Every sequential output is listed below.

| Output key | Independently derived expected value | Unit |
|---|---:|---|
| `v_thermal` | `0.02585199978643553` | V |
| `alpha` | `0.9900990099009901` | 1 |
| `current_ratio` | `2.1676302961648406` | 1 |
| `i_collector_one` | `0.001355062560591518` | A |
| `i_collector_two` | `0.0006251354592104623` | A |
| `i_difference` | `0.0007299271013810556` | A |
| `g_differential` | `0.03309487233880996` | S |


### Near-balance numerical conditioning

The executable difference uses `alpha*Itail*expm1(Vdiff/VT)/(1+current_ratio)` rather than subtracting two nearly equal collector currents. From the preceding conservation equations, `Idiff=alpha*Itail*(exp(x)-1)/(1+exp(x))`; native `expm1` preserves the small signed difference when `x` is near zero, without narrowing the physical domain. At zero input it is exactly zero. Its independent limiting slope is `alpha*Itail/(2*VT)`; at the nominal tail/beta/temperature this is approximately `0.0382987396750828 S`. The individual collector outputs retain their original formulas and all researched expected values are unchanged. Parent QA should cover very small positive/negative Vdiff, zero, sign symmetry and the +/-40 VT endpoints.

