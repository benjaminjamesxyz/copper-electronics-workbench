# Passive time-domain transients — executable features

This lane contains exactly eight physical calculations in [`backend/catalog/transients.json`](../../backend/catalog/transients.json): two one-state voltage-step problems, an imposed-current capacitor ramp, isolated two-capacitor redistribution, periodic two-level excitation, finite-slope voltage-source tracking, a loaded two-capacitor RC ladder, and an underdamped series-RLC step. Charge/discharge, time-target inversions and stored-energy outputs are deliberately **not** counted as separate calculators. No frequency-domain filter synthesis, converter sizing, digital-edge threshold, ADC acquisition requirement, standalone magnetic-energy calculation or transistor drive design is included.

All models are closed-form lumped-element calculations, not circuit simulations. The expressions use only the contract's arithmetic, `exp`, `expm1`, `ln`, `sqrt`, `sin`, `cos`, and `pi`, with sequential scalar outputs. Constraints use input keys and constants only. Every supporting output is included in its JSON example. Units are SI: resistance in ohm, capacitance in F, inductance in H, time in s, voltage in V, current in A, charge in C, energy in J and angular frequency in rad/s. Dimensionless quantities use `1`.

## Primary-source ledger

Sources were accessed during this research. Citations support the physical laws and response families; extensions independently derived below are explicitly distinguished from formulas printed verbatim in sources.

1. **MIT 6.200, Optional Review: RC Circuit Derivations and Key Values.** Gives `i=C dv/dt`, source-free RC decay, the `RC` time constant and arbitrary-initial-condition step response. [Course tutorial](https://circuits.mit.edu/F25/psets/05/rc_tutorial).
2. **MIT 6.200, Lecture 11: First-Order Driven Response.** Describes RC/RL continuity, long-time behavior and RL final current `V/R`. Some PDF equation pages are poorly extracted; the independently readable Rowell handout below supplies the time-constant/response mathematics. [Lecture slides](https://circuits.mit.edu/_static/S26/handouts/lec06a/slides.pdf).
3. **D. Rowell, MIT 2.151, Review of First- and Second-Order System Response, 22 October 2004.** Sections 1.1–1.3 give first-order decay, RC/RL time constants, step and ramp responses, and initial-condition superposition. Equations 21, 30 and 32 support ramp excitation; sections 2.1–2.2 give second-order roots and step responses. [MIT handout](https://web.mit.edu/2.151/www/Handouts/FirstSecondOrder.pdf).
4. **R. Nave, Georgia State University, HyperPhysics: Energy Stored on a Capacitor.** Gives the capacitor energy law and discusses resistive charging loss independent of charging resistance. [University reference](https://hyperphysics.gsu.edu/hbase/electric/capeng.html).
5. **Wen Chuen Liu, UC Berkeley, Hybrid Switched-Capacitor Converters for High-Performance Power Conversions, Technical Report EECS-2021-25, section 3.1.** Equations 3.1 and 3.3 and Figure 3.1 discuss voltage-source and two-capacitor charging, equal-capacitance energy redistribution loss, resistance dependence of peak current and the explicit positive-resistance assumption. Only that isolated physical process is used here; no converter is sized. [Berkeley dissertation](https://www2.eecs.berkeley.edu/Pubs/TechRpts/2021/EECS-2021-25.pdf).
6. **Colorado State University, ECE 202 Experiment 5, Step Response of First- and Second-Order Circuits.** Pages 2–3 describe RC circuits driven by repetitive square-wave excitation and incomplete/near-complete charging depending on pulse duration relative to `RC`. The arbitrary-duty periodic fixed-point formula below is derived from the MIT one-interval solution, not claimed to be printed in this laboratory handout. [Laboratory handout](https://www.engr.colostate.edu/ECE202/common/LABS/Lab_5.pdf).
7. **Matthew Spencer, Harvey Mudd College E151, Review of Linear Circuits & Dynamics.** Presents KCL/KVL, circuit linearity, interconnection/loading and circuit step-response methods. This supports the nodal method used for the independently derived unbuffered ladder; it is not cited as printing the specific ladder formula. [Teaching notes](https://pages.hmc.edu/mspencer/e151/sp26/notes/02.pdf).
8. **MIT 2.003, Spring 2005 Prelab 5.** Specifies series-RLC capacitor-output step responses, damping regimes and the need to add inductor winding resistance to the selected series resistor. [Prelab PDF](https://ocw.mit.edu/courses/2-003-modeling-dynamics-and-control-i-spring-2005/d29876dfd3a4df303146df410d26041e_prelab5_sp2005.pdf).

Independent example values below were computed directly from the derived equations and elementary numerical constants, not from the calculator expression engine. The independent research arithmetic is retained unchanged; no implementation build, lint, tests, formatter or smoke run was performed. JSON relative tolerance is `1e-6`.


## Catalog interpretation

The Rust engine alone evaluates these scalar programs. All input bounds below are finite numerical envelopes, not hardware ratings. Exactly named finite inputs are required; output expressions run sequentially after input-only constraints. Defaults equal the worked input vector unless explicitly listed otherwise. Domain errors reject the request rather than clamp a waveform or fabricate a result. The tables preserve every independent research output at relative tolerance `1e-6`. Near-zero results need output-specific absolute tolerances; relative error is not meaningful for an exact zero. Physical accuracy remains limited by the stated lumped, linear models.

## `transients.rc_step` — arbitrary-initial-condition RC step

**Level:** basic. **Design purpose:** predict startup/discharge voltage and current, settling duration, stored energy and accumulated resistor pulse energy. Sources: [MIT RC tutorial](https://circuits.mit.edu/F25/psets/05/rc_tutorial), [GSU capacitor energy](https://hyperphysics.gsu.edu/hbase/electric/capeng.html).

### Model and derivation

An ideal post-switch source `Vs` drives series `R` into capacitor `C`, initially at `V0`. Define positive current into the capacitor's positive terminal. KVL and the capacitor law yield

`RC dvC/dt + vC = Vs`.

The initial condition selects

`tau = RC`,

`vC(t) = Vs + (V0 − Vs) exp(−t/tau)`,

`iC(t) = (Vs − V0) exp(−t/tau)/R`.

Normalized remaining error is `|vC−Vs|/|V0−Vs| = exp(−t/tau)` when `V0 != Vs`. Therefore the time to a specified fraction `epsilon` of the original error is `−tau ln(epsilon)`. An arbitrary reachable voltage target can be translated to `epsilon=|Vtarget−Vs|/|V0−Vs|`; it is not a second calculation.

The capacitor energy is `EC=C vC²/2`. Integrating `iC² R` from zero to `t` gives

`ER(t) = C(Vs−V0)²[1−exp(−2t/tau)]/2`.

This includes charging loss and discharge dissipation with one physical circuit. For discharge, set `Vs=0`. At full discharge, `ER(∞)=C V0²/2`.

### Domain and limitations

`R>0`, `C>0`, `t>=0`, `0<epsilon<1`. In the catalog, positive finite input bounds ensure nonzero denominators; the tolerance lower bound is `1e-12`. Signed voltages are supported. When `V0=Vs`, there is no error to settle; the reported time is only the conventional normalized-response time scale. R may incorporate lumped source resistance and ESR, but dielectric absorption, leakage, ESL, voltage-dependent capacitance and load paths are omitted. The computed energy is not a resistor pulse-rating certification.

### Independent worked example

`R=1000 ohm`, `C=1 microF`, `Vs=5 V`, `V0=0`, `t=1 ms`, `epsilon=0.01`.

- `tau=1000×10^-6=0.001 s`.
- `exp(−t/tau)=exp(−1)=0.3678794411714423`.
- `vC=5(1−exp(−1))=3.160602794142788 V`.
- `iC=0.005 exp(−1)=0.001839397205857212 A`.
- `t_settle=0.001 ln(100)=0.004605170185988092 s`.
- `EC=0.5×10^-6×3.160602794142788²=4.9947050111716 microJ`.
- `ER=12.5 microJ×(1−exp(−2))=10.80830895954234 microJ`.


### Executable input contract

| Key | Unit | Default | Admissible range |
|---|---|---:|---|
| `r` | ohm | 1000.0 | [0.001, 1000000000.0] |
| `c` | F | 1e-06 | [1e-15, 1.0] |
| `v_source` | V | 5.0 | [-1000000.0, 1000000.0] |
| `v_initial` | V | 0.0 | [-1000000.0, 1000000.0] |
| `time` | s | 0.001 | [0.0, 1000000000.0] |
| `error_fraction` | 1 | 0.01 | [1e-12, 1.0) |

The executable voltage is `V0 exp(−t/tau)+Vs[−expm1(−t/tau)]`; accumulated resistor energy uses `−expm1(−2t/tau)`. Both are algebraically identical to the derivation, with accurate exponential differences near zero and a preserved small residual during source-free discharge. Current is evaluated from the original voltage difference rather than subtracting rounded terminal voltages. At `t=0`, voltage/current retain the initial condition and accumulated loss is exactly zero. Equal initial/source voltage produces zero transient current/loss; signed voltage and discharge remain valid.

No additional relational constraint is needed: positive component minima protect divisors, time is nonnegative, and any logarithm has an explicit positive tolerance bound. Signed excitations and valid zero boundaries are not rejected.

### Complete worked outputs (SI)

All input values for this independent example are `{"r":1000.0,"c":1e-06,"v_source":5.0,"v_initial":0.0,"time":0.001,"error_fraction":0.01}`.

| Output key | Unit | Independent expected value | Executable expression |
|---|---|---:|---|
| `tau` | s | 0.001 | `r*c` |
| `capacitor_voltage` | V | 3.1606027941427883 | `v_initial*exp(-time/tau)+v_source*(-expm1(-time/tau))` |
| `capacitor_current` | A | 0.0018393972058572117 | `(v_source-v_initial)/r*exp(-time/tau)` |
| `settling_time` | s | 0.004605170185988092 | `-tau*ln(error_fraction)` |
| `capacitor_energy` | J | 4.9947050111716e-06 | `c*capacitor_voltage^2/2` |
| `resistor_energy` | J | 1.080830895954234e-05 | `c*(v_source-v_initial)^2/2*(-expm1(-2*time/tau))` |

