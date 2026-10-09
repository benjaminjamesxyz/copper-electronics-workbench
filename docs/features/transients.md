# Passive time-domain transients — executable features

This lane contains exactly eight physical calculations in [`backend/catalog/transients.json`](../../backend/catalog/transients.json): two one-state voltage-step problems, an imposed-current capacitor ramp, isolated two-capacitor redistribution, periodic two-level excitation, finite-slope voltage-source tracking, a loaded two-capacitor RC ladder, and an underdamped series-RLC step. Charge/discharge, time-target inversions and stored-energy outputs are deliberately **not** counted as separate calculators. No frequency-domain filter synthesis, converter sizing, digital-edge threshold, ADC acquisition requirement, standalone magnetic-energy calculation or transistor drive design is included.

All models are closed-form lumped-element calculations, not circuit simulations. The expressions use contract arithmetic/functions and the parent-approved shared Rust scalar primitives `expm1mx_ratio`, `step_overdamped_scaled` and `step_underdamped_scaled` for stable, amplitude-aware evaluation of the same analytic response near zero. No lane-local evaluator or frontend mathematics is introduced. Outputs are sequential scalars; constraints use input keys/constants only. Every supporting output is included in its independent example. Units are SI: resistance in ohm, capacitance in F, inductance in H, time in s, voltage in V, current in A, charge in C, energy in J and angular frequency in rad/s. Dimensionless quantities use `1`.

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


## `transients.rc_ladder_step` — loaded passive two-stage ladder

**Level:** expert. **Design purpose:** predict startup of an unbuffered two-capacitor ladder, where the second stage loads the first. Sources: [HMC KCL and dynamics methods](https://pages.hmc.edu/mspencer/e151/sp26/notes/02.pdf), [MIT second-order real-pole step response](https://web.mit.edu/2.151/www/Handouts/FirstSecondOrder.pdf). The specific circuit equations below are independently derived.

### Topology and derivation

The ideal source drives R1, then node one, then R2, then node two. C1 connects node one to ground; C2 connects node two to ground. Both initially uncharged; no output load and no interstage buffer.

KCL gives

`(Vs−v1)/R1=C1 dv1/dt+(v1−v2)/R2`,

`(v1−v2)/R2=C2 dv2/dt`.

From the second equation `v1=v2+R2 C2 dv2/dt`. Substitute into the first:

`a d²v2/dt²+b dv2/dt+v2=Vs`,

where `a=R1R2C1C2` has units `s²` and `b=R1C1+R1C2+R2C2` has units `s`.

The cross term **R1C2** is essential. Multiplying two independently buffered RC responses would omit loading and solve the wrong circuit. Factoring the homogeneous characteristic polynomial as `(1+tau_s s)(1+tau_f s)` gives

`tau_s=(b+sqrt(b²−4a))/2`, `tau_f=a/tau_s`.

With zero initial voltage and current,

`v2(t)=Vs[1−(tau_s exp(−t/tau_s)−tau_f exp(−t/tau_f))/(tau_s−tau_f)]`,

`iC2(t)=C2 Vs[exp(−t/tau_s)−exp(−t/tau_f)]/(tau_s−tau_f)`,

`v1(t)=v2(t)+R2 iC2(t)`.

Writing `x=R1C1`, `y=R1C2`, `z=R2C2`, `b²−4a=(x−z)²+y²+2y(x+z)>0` for positive components. Thus this passive topology has distinct real decay modes mathematically. The catalog additionally guards numerical separation. The fast constant is obtained from `a/tau_s`, rather than cancellation-prone `(b−sqrt(...))/2`.

### Domain and limitations

All component values are positive; `t>=0`. The input-only discriminant constraint requires `(b²−4a)/b²>=1e-8`. No buffer, output resistance, leakage, initial capacitor charge or inductance is modeled. Signed source steps are valid. The calculation is a startup problem with both node voltages, not frequency-response filter synthesis. The promoted catalog delegates short-time evaluation to the shared stable scalar primitive for this same analytic response; no waveform clipping or additional time restriction is substituted.

### Independent worked example

`R1=R2=1 kohm`, `C1=C2=1 microF`, `Vs=5 V`, `t=1 ms`.

- `a=10^-6 s²`; `b=0.003 s`.
- `sqrt(b²−4a)=sqrt(5)×0.001 s`.
- `tau_s=(3+sqrt(5)) ms/2=0.002618033988749895 s`.
- `tau_f=(3−sqrt(5)) ms/2=0.0003819660112501051 s`.
- Substitution gives `v2=1.066772003483159 V` and `iC2=0.001363044688312645 A`.
- Independently using the resistor voltage gives `v1=v2+1000 iC2=2.429816691795804 V`.


### Executable input contract

| Key | Unit | Default | Admissible range |
|---|---|---:|---|
| `r_one` | ohm | 1000.0 | [0.001, 1000000000.0] |
| `r_two` | ohm | 1000.0 | [0.001, 1000000000.0] |
| `c_one` | F | 1e-06 | [1e-15, 1.0] |
| `c_two` | F | 1e-06 | [1e-15, 1.0] |
| `v_step` | V | 5.0 | [-1000000.0, 1000000.0] |
| `time` | s | 0.001 | [0.0, 1000000000.0] |

`step_overdamped_scaled(t,slow_rate,fast_rate,amplitude)` is the integration-owned stable scalar evaluation of the displayed analytic step, with `slow_rate=1/tau_s`, `fast_rate=1/tau_f`, strictly ordered and positive, and `amplitude=Vs`. Amplitude is passed into the primitive rather than multiplied by an already rounded normalized response, preserving representable output voltages when the unit-amplitude response underflows. It changes numerical evaluation only, not topology, waveform, accepted time domain or public outputs. Current uses `exp(−t/tau_s)[−expm1(−t(1/tau_f−1/tau_s))]` instead of subtracting two nearly identical decays. The input-only discriminant guard retains the researched near-degenerate exclusion and protects the root and modal difference; positive components protect all other divisors. At t=0 both node voltages and downstream current are exactly zero. At default components and t=1e-20 s, the independent startup asymptote is `v2≈Vs*t²/(2a)=2.5e-34 V`, not zero. Signed steps remain valid. This requires the parent-owned stable primitive at integration; no lane-local evaluator or Taylor-seed output is introduced.

Additional input-only constraints (checked before any output):

- `(r_one*c_one+r_one*c_two+r_two*c_two)^2-4*r_one*r_two*c_one*c_two >= 1e-8*(r_one*c_one+r_one*c_two+r_two*c_two)^2`: Distinct modal time constants require a relative discriminant of at least 1e-8.

### Complete worked outputs (SI)

All input values for this independent example are `{"r_one":1000.0,"r_two":1000.0,"c_one":1e-06,"c_two":1e-06,"v_step":5.0,"time":0.001}`.

| Output key | Unit | Independent expected value | Executable expression |
|---|---|---:|---|
| `quadratic_coefficient` | s^2 | 1e-06 | `r_one*r_two*c_one*c_two` |
| `linear_coefficient` | s | 0.003 | `r_one*c_one+r_one*c_two+r_two*c_two` |
| `tau_slow` | s | 0.002618033988749895 | `(linear_coefficient+sqrt(linear_coefficient^2-4*quadratic_coefficient))/2` |
| `tau_fast` | s | 0.0003819660112501051 | `quadratic_coefficient/tau_slow` |
| `voltage_two` | V | 1.0667720034831585 | `step_overdamped_scaled(time,1/tau_slow,1/tau_fast,v_step)` |
| `capacitor_two_current` | A | 0.0013630446883126453 | `c_two*v_step*exp(-time/tau_slow)*(-expm1(-time*(1/tau_fast-1/tau_slow)))/(tau_slow-tau_fast)` |
| `voltage_one` | V | 2.4298166917958035 | `voltage_two+r_two*capacitor_two_current` |


## `transients.rlc_underdamped_step` — oscillatory startup and voltage stress

**Level:** expert. **Design purpose:** estimate time-domain ringing, first capacitor-voltage peak and a guaranteed model-error envelope following a positive source step. Sources: [MIT series-RLC prelab](https://ocw.mit.edu/courses/2-003-modeling-dynamics-and-control-i-spring-2005/d29876dfd3a4df303146df410d26041e_prelab5_sp2005.pdf), [MIT Rowell second-order response](https://web.mit.edu/2.151/www/Handouts/FirstSecondOrder.pdf).

### Derivation

In the series RLC, `i=C dvC/dt`. KVL yields

`LC d²vC/dt²+RC dvC/dt+vC=Vs`.

With zero initial current and capacitor voltage, define

`alpha=R/(2L)`, `omega_n=1/sqrt(LC)`, `zeta=alpha/omega_n`,

`omega_d=sqrt(omega_n²−alpha²)`.

In the underdamped regime,

`vC(t)=Vs{1−exp(−alpha t)[cos(omega_d t)+(alpha/omega_d)sin(omega_d t)]}`,

`i(t)=Vs exp(−alpha t)sin(omega_d t)/(L omega_d)`.

Differentiate vC to find its stationary points. The first positive-time maximum occurs at `t_peak=pi/omega_d`; its fractional overshoot is

`Mp=exp(−alpha pi/omega_d)` and `Vpeak=Vs(1+Mp)`.

These are temporal stress estimates, not additional resonance or Q calculators.

### Conservative settling derivation

The normalized error is `exp(−alpha t)[cos(omega_d t)+(alpha/omega_d)sin(omega_d t)]`. Its oscillatory bracket has maximum absolute magnitude

`sqrt(1+(alpha/omega_d)²)=1/sqrt(1−zeta²)`.

Thus a sufficient condition for **all later** normalized errors to remain below `epsilon` is

`t >= ln[1/(epsilon sqrt(1−zeta²))]/alpha`.

This reported time is an envelope bound, not the exact last crossing or the heuristic `4/(zeta omega_n)`.

### Domain and limitations

`R,L,C,Vs>0`, `t>=0`, `0<epsilon<1`. Total R includes winding/source resistance and series ESR. The model requires `alpha²<omega_n²`; the implemented input-only guard strengthens this to `alpha² <= (1−1e-8)/(LC)` for numerical margin. It does not silently use an underdamped expression for critically damped or overdamped circuits. Initial energy is zero; nonlinear clamps, core saturation, dielectric loss, distributed modes and component ratings are excluded. Angular frequency is in rad/s, never confused with hertz.

### Independent worked example

`R=10 ohm`, `L=1 mH`, `C=10 microF`, `Vs=5 V`, `t=100 microseconds`, `epsilon=0.01`.

- `alpha=10/(2×0.001)=5000 s^-1`.
- `omega_n=1/sqrt(10^-8)=10000 rad/s`; `zeta=0.5`.
- `omega_d=sqrt(100000000−25000000)=8660.254037844386 rad/s`.
- At 100 microseconds, `alpha t=0.5`, `omega_d t=0.8660254037844386`; direct trigonometric substitution gives `vC=1.701499233041492 V`, `i=0.2667535975573465 A`.
- `t_peak=pi/8660.254037844386=0.0003627598728468436 s`.
- `Mp=exp(−pi/sqrt(3))=0.1630335348215805`; `Vpeak=5.815167674107903 V`.
- Envelope factor `=1/sqrt(0.75)`; the 1% sufficient settling bound is `ln[1/(0.01 sqrt(0.75))]/5000=0.0009498022444427963 s`.


### Executable input contract

| Key | Unit | Default | Admissible range |
|---|---|---:|---|
| `r` | ohm | 10.0 | [0.001, 1000000000.0] |
| `l` | H | 0.001 | [1e-12, 1000000.0] |
| `c` | F | 1e-05 | [1e-15, 1.0] |
| `v_step` | V | 5.0 | (0.0, 1000000.0] |
| `time` | s | 0.0001 | [0.0, 1000000000.0] |
| `error_fraction` | 1 | 0.01 | [1e-12, 1.0) |

The shared integration-owned `step_underdamped_scaled(u,zeta,amplitude)` evaluates the derived analytic response with `u=omega_n*time`, the declared damping ratio and `amplitude=Vs`. The first two arguments are dimensionless; ordinary seconds or `omega_d*time` would be incorrect. Amplitude-aware evaluation preserves representable voltages when the unit-amplitude response would underflow before an outside amplitude multiplication. It changes only numerical conditioning, keeping every output and the full researched time domain. At t=0 voltage/current are exactly zero; at default components and t=1e-20 s the independent startup asymptote is `vC≈Vs*omega_n²*t²/2=2.5e-32 V`. The input-only squared-frequency guard rejects critical/overdamping and protects omega_d and the settling-envelope root. R, L, C and positive step amplitude retain their finite positive ranges; `0<epsilon<1` protects the logarithm. At peak_time the waveform equals peak_voltage; envelope_settling_bound is sufficient for all later times, not the exact last crossing. No clipping or pulse-rating guarantee is implied. Very large oscillatory phases have ordinary finite-precision argument-resolution limitations.

Additional input-only constraints (checked before any output):

- `(r/(2*l))^2 <= (1-1e-8)/(l*c)`: Circuit must be underdamped with a relative squared-frequency margin of at least 1e-8.

### Complete worked outputs (SI)

All input values for this independent example are `{"r":10.0,"l":0.001,"c":1e-05,"v_step":5.0,"time":0.0001,"error_fraction":0.01}`.

| Output key | Unit | Independent expected value | Executable expression |
|---|---|---:|---|
| `alpha` | 1/s | 5000.0 | `r/(2*l)` |
| `omega_n` | rad/s | 10000.0 | `1/sqrt(l*c)` |
| `damping_ratio` | 1 | 0.5 | `alpha/omega_n` |
| `omega_d` | rad/s | 8660.254037844386 | `sqrt(omega_n^2-alpha^2)` |
| `capacitor_voltage` | V | 1.701499233041492 | `step_underdamped_scaled(omega_n*time,damping_ratio,v_step)` |
| `series_current` | A | 0.2667535975573465 | `v_step/(l*omega_d)*exp(-alpha*time)*sin(omega_d*time)` |
| `peak_time` | s | 0.00036275987284684356 | `pi/omega_d` |
| `overshoot_fraction` | 1 | 0.16303353482158048 | `exp(-alpha*peak_time)` |
| `peak_voltage` | V | 5.815167674107903 | `v_step*(1+overshoot_fraction)` |
| `envelope_settling_bound` | s | 0.0009498022444427963 | `ln(1/(error_fraction*sqrt(1-damping_ratio^2)))/alpha` |


## `transients.rc_periodic_switching` — incomplete charging in periodic steady state

**Level:** intermediate. **Design purpose:** find ripple and pulse-start currents when a capacitor never fully resets between successive high/low source intervals. Sources: [CSU square-wave RC laboratory](https://www.engr.colostate.edu/ECE202/common/LABS/Lab_5.pdf), [MIT one-interval RC solution](https://circuits.mit.edu/F25/psets/05/rc_tutorial).

### Fixed-point derivation

The same R and C apply in both intervals. Let `Th` and `Tl` be positive durations; define `a=exp(−Th/tau)` and `b=exp(−Tl/tau)`, `tau=RC`. At the beginning of the high interval, capacitor voltage is `Vmin`; at its end it is `Vmax`:

`Vmax=Vh+(Vmin−Vh)a`.

After the low interval, periodicity requires

`Vmin=Vl+(Vmax−Vl)b`.

Solving these **coupled periodic boundary conditions** yields

`Vmin=[Vl(1−b)+b Vh(1−a)]/(1−ab)`,

`Vmax=Vmin+(Vh−Vmin)(1−a)`,

`Vripple=Vmax−Vmin`.

The stable start-of-cycle recurrence has multiplier `ab<1`, which explains why startup conditions are forgotten. Over a whole cycle, integrating `tau dv/dt+v=Vs` makes the derivative integral vanish; therefore

`Vaverage=(Vh Th+Vl Tl)/(Th+Tl)`.

The current immediately after a rising source edge is `(Vh−Vmin)/R`; immediately after a falling edge it is `(Vl−Vmax)/R`. Source voltage jumps but capacitor voltage does not.

### Domain and limitations

`R,C,Th,Tl>0`, `Vh>=Vl`. The high/low names define extrema, not merely labels. Equal source levels give zero ripple. Steady periodic operation is required; this is not the first-pulse trajectory. No load, variable switching resistance or inductive parasitic is included. The catalog requires both durations normalized by RC to be at least `1e-7` as the conservative researched numerical domain; the implementation evaluates exponential differences with `expm1`. This restriction is numerical, not a physical lower bound on pulse width.

### Independent worked example

`R=1 kohm`, `C=1 microF`, `Vh=5 V`, `Vl=0`, `Th=Tl=1 ms`.

- `tau=0.001 s`; `a=b=exp(−1)=0.3678794411714423`.
- Symmetry simplifies `Vmin=5a/(1+a)=1.344707106849976 V` and `Vmax=5/(1+a)=3.655292893150024 V`.
- Ripple `=2.310585786300049 V`; average `=2.5 V`.
- Rising-edge current `=3.6552928931500245 mA`; falling-edge current `=−3.6552928931500245 mA`.


### Executable input contract

| Key | Unit | Default | Admissible range |
|---|---|---:|---|
| `r` | ohm | 1000.0 | [0.001, 1000000000.0] |
| `c` | F | 1e-06 | [1e-15, 1.0] |
| `v_high` | V | 5.0 | [-1000000.0, 1000000.0] |
| `v_low` | V | 0.0 | [-1000000.0, 1000000.0] |
| `time_high` | s | 0.001 | [1e-15, 1000000000.0] |
| `time_low` | s | 0.001 | [1e-15, 1000000000.0] |

Let `A=−expm1(−Th/tau)`, `B=−expm1(−Tl/tau)`, `D=−expm1(−(Th+Tl)/tau)`, and `DeltaV=Vh−Vl`. Executable extrema are `Vl+DeltaV*b*A/D` and `Vl+DeltaV*A/D`; ripple is directly `DeltaV*A*B/D`. Currents are directly `DeltaV*B/(R*D)` and `−DeltaV*A/(R*D)`, so adding a tiny ripple to a large DC voltage does not erase ripple/current outputs. These expressions follow by rearranging the fixed point and are not a new model. The researched >=1e-7 interval/tau guards remain explicit conservative representability restrictions despite improved expm1 conditioning. Vh>=Vl defines extrema; equal levels yield exactly zero ripple and edge currents. Average capacitor voltage equals time-weighted source voltage only for this unloaded periodic steady state.

Additional input-only constraints (checked before any output):

- `v_high >= v_low`: High source level must be at least the low source level.
- `time_high/(r*c) >= 1e-7`: High interval must be at least 1e-7 time constants within the researched numerical domain.
- `time_low/(r*c) >= 1e-7`: Low interval must be at least 1e-7 time constants within the researched numerical domain.

### Complete worked outputs (SI)

All input values for this independent example are `{"r":1000.0,"c":1e-06,"v_high":5.0,"v_low":0.0,"time_high":0.001,"time_low":0.001}`.

| Output key | Unit | Independent expected value | Executable expression |
|---|---|---:|---|
| `tau` | s | 0.001 | `r*c` |
| `high_decay` | 1 | 0.36787944117144233 | `exp(-time_high/tau)` |
| `low_decay` | 1 | 0.36787944117144233 | `exp(-time_low/tau)` |
| `voltage_min` | V | 1.3447071068499756 | `v_low+(v_high-v_low)*low_decay*(-expm1(-time_high/tau))/(-expm1(-(time_high+time_low)/tau))` |
| `voltage_max` | V | 3.6552928931500244 | `v_low+(v_high-v_low)*(-expm1(-time_high/tau))/(-expm1(-(time_high+time_low)/tau))` |
| `ripple` | V | 2.3105857863000487 | `(v_high-v_low)*(-expm1(-time_high/tau))*(-expm1(-time_low/tau))/(-expm1(-(time_high+time_low)/tau))` |
| `average_voltage` | V | 2.5 | `(v_high*time_high+v_low*time_low)/(time_high+time_low)` |
| `high_initial_current` | A | 0.0036552928931500245 | `(v_high-v_low)*(-expm1(-time_low/tau))/(-expm1(-(time_high+time_low)/tau))/r` |
| `low_initial_current` | A | -0.0036552928931500245 | `-(v_high-v_low)*(-expm1(-time_high/tau))/(-expm1(-(time_high+time_low)/tau))/r` |


## `transients.rl_step` — closed-path RL buildup and decay

**Level:** basic. **Design purpose:** predict coil-current buildup or resistive decay without substituting an opened-inductor/clamp model. Sources: [MIT lecture](https://circuits.mit.edu/_static/S26/handouts/lec06a/slides.pdf), [MIT Rowell sections 1.1 and 1.3](https://web.mit.edu/2.151/www/Handouts/FirstSecondOrder.pdf).

### Model and derivation

With `L di/dt + Ri = Vs` and initial current `I0`,

`tau=L/R`, `If=Vs/R`,

`iL(t)=If+(I0−If)exp(−t/tau)`,

`vL(t)=L di/dt=(Vs−R I0)exp(−t/tau)`.

The normalized current-error settling time is again `−tau ln(epsilon)`. `Vs=0` is source-free current decay in the **remaining closed series loop**. This is a different energy-storage element and current state from the RC voltage problem, not an inverse rearrangement of it. Standalone magnetic energy is intentionally omitted because it belongs to the magnetics lane.

### Domain and limitations

`R>0`, `L>0`, `t>=0`, `0<epsilon<1`. R includes winding/source resistance. L must remain linear and unsaturated; initial current is continuous for finite voltage. No diode, avalanche, arcing, open-circuit interruption, hysteretic/core loss or nonlinear clamp is included. If `I0=If`, the settling output is only a time-scale metric.

### Independent worked example

`R=10 ohm`, `L=10 mH`, `Vs=12 V`, `I0=0`, `t=1 ms`, `epsilon=0.01`.

- `tau=0.01/10=0.001 s`; `If=12/10=1.2 A`.
- `iL=1.2(1−exp(−1))=0.7585446705942692 A`.
- `vL=12 exp(−1)=4.414553294057308 V`.
- `t_settle=0.001 ln(100)=0.004605170185988092 s`.


### Executable input contract

| Key | Unit | Default | Admissible range |
|---|---|---:|---|
| `r` | ohm | 10.0 | [0.001, 1000000000.0] |
| `l` | H | 0.01 | [1e-12, 1000000.0] |
| `v_source` | V | 12.0 | [-1000000.0, 1000000.0] |
| `i_initial` | A | 0.0 | [-1000000.0, 1000000.0] |
| `time` | s | 0.001 | [0.0, 1000000000.0] |
| `error_fraction` | 1 | 0.01 | [1e-12, 1.0) |

The executable current is `I0 exp(−t/tau)+If[−expm1(−t/tau)]`, algebraically identical to the derived waveform but preserving short-time buildup and small source-free residuals. At zero time it returns I0 exactly. Signed source/initial current, zero source, and an already steady current are admissible. Positive R/L and the tolerance bounds protect all divisions and logarithms; this closed-path model must not be used for an opened coil.

No additional relational constraint is needed: positive component minima protect divisors, time is nonnegative, and any logarithm has an explicit positive tolerance bound. Signed excitations and valid zero boundaries are not rejected.

### Complete worked outputs (SI)

All input values for this independent example are `{"r":10.0,"l":0.01,"v_source":12.0,"i_initial":0.0,"time":0.001,"error_fraction":0.01}`.

| Output key | Unit | Independent expected value | Executable expression |
|---|---|---:|---|
| `tau` | s | 0.001 | `l/r` |
| `final_current` | A | 1.2 | `v_source/r` |
| `inductor_current` | A | 0.7585446705942692 | `i_initial*exp(-time/tau)+final_current*(-expm1(-time/tau))` |
| `inductor_voltage` | V | 4.414553294057308 | `(v_source-r*i_initial)*exp(-time/tau)` |
| `settling_time` | s | 0.004605170185988092 | `-tau*ln(error_fraction)` |


## `transients.capacitor_constant_current` — imposed-current ramp and stored energy

**Level:** basic. **Design purpose:** determine charge accumulation and voltage/energy changes under a constant capacitor current. Sources: [MIT capacitor constitutive equation](https://circuits.mit.edu/F25/psets/05/rc_tutorial), [GSU energy law](https://hyperphysics.gsu.edu/hbase/electric/capeng.html).

### Model and derivation

Integrate `i=C dv/dt` with constant signed `I`:

`dv/dt=I/C`, `vC(t)=V0+It/C`, `deltaQ=It`.

The final stored energy is `EC=C vC(t)²/2`; change from the initial state is `deltaE=C[vC(t)²−V0²]/2`. Unlike voltage-driven RC charging, the capacitor current does not exponentially decay. This calculator specifies the excitation and does not design an active current source, op-amp or converter.

### Domain and limitations

`C>0`, `t>=0`; current and initial voltage may be signed. No division by I occurs, so zero current is valid. Current-source compliance, load/leakage and polarized-capacitor allowable voltage must be checked separately. Ideal negative-current discharge may cross zero voltage; the mathematical result must not be interpreted as permission to reverse-bias a polarized capacitor. ESR, ESL, absorption and changing capacitance are excluded.

### Independent worked example

`C=100 microF`, `I=10 mA`, `V0=1 V`, `t=20 ms`.

- `dv/dt=0.01/0.0001=100 V/s`.
- `vC=1+100×0.02=3 V`.
- `deltaQ=0.01×0.02=0.0002 C`.
- `EC=0.0001×9/2=0.00045 J`.
- `deltaE=0.0001×(9−1)/2=0.0004 J`.


### Executable input contract

| Key | Unit | Default | Admissible range |
|---|---|---:|---|
| `c` | F | 0.0001 | [1e-15, 1.0] |
| `current` | A | 0.01 | [-1000000.0, 1000000.0] |
| `v_initial` | V | 1.0 | [-1000000.0, 1000000.0] |
| `time` | s | 0.02 | [0.0, 1000000000.0] |

All finite signed currents in range, including zero, are accepted; t=0 returns the initial voltage and zero charge/energy change. The executable signed energy change is `deltaQ*(V0+deltaQ/(2C))`, derived by expanding the difference of squares; it does not subtract rounded energies. Negative energy change is a physically valid reduction in stored energy and is not clamped. C is strictly positive through its finite lower bound; no current divisor or target-voltage feasibility constraint is needed.

No additional relational constraint is needed: positive component minima protect divisors, time is nonnegative, and any logarithm has an explicit positive tolerance bound. Signed excitations and valid zero boundaries are not rejected.

### Complete worked outputs (SI)

All input values for this independent example are `{"c":0.0001,"current":0.01,"v_initial":1.0,"time":0.02}`.

| Output key | Unit | Independent expected value | Executable expression |
|---|---|---:|---|
| `voltage_slope` | V/s | 100.0 | `current/c` |
| `capacitor_voltage` | V | 3.0 | `v_initial+voltage_slope*time` |
| `charge_change` | C | 0.0002 | `current*time` |
| `stored_energy` | J | 0.00045 | `c*capacitor_voltage^2/2` |
| `energy_change` | J | 0.0004 | `charge_change*(v_initial+charge_change/(2*c))` |


## `transients.capacitor_charge_sharing` — isolated redistribution between unequal capacitors

**Level:** intermediate. **Design purpose:** predict equalized voltage, initial/current pulse decay and irreversible redistribution loss between two isolated capacitor banks. Sources: [Berkeley section 3.1](https://www2.eecs.berkeley.edu/Pubs/TechRpts/2021/EECS-2021-25.pdf), [MIT capacitor dynamics](https://circuits.mit.edu/F25/psets/05/rc_tutorial).

### Model and independent unequal-capacitance derivation

The capacitors have a common negative reference; connect their positive nodes through R. No source or load remains connected. Let `DeltaV=V1−V2` initially, and positive current flow from one to two.

Conservation of charge gives `C1 v1+C2 v2=C1 V1+C2 V2`, hence

`Vf=(C1 V1+C2 V2)/(C1+C2)`.

Using `dv1/dt=−i/C1`, `dv2/dt=i/C2` and `i=(v1−v2)/R`, the voltage difference obeys

`d(v1−v2)/dt = −(v1−v2)/(R Ceq)`, where `Ceq=C1 C2/(C1+C2)`.

Consequently

`tau=R Ceq`,

`v1(t)=Vf+[C2/(C1+C2)] DeltaV exp(−t/tau)`,

`v2(t)=Vf−[C1/(C1+C2)] DeltaV exp(−t/tau)`,

`i(t)=DeltaV exp(−t/tau)/R`,

`Qtransfer(t)=Ceq DeltaV[1−exp(−t/tau)]`.

Initial energy is `(C1 V1²+C2 V2²)/2`; final energy is `(C1+C2)Vf²/2`. Their difference simplifies to

`Eloss=Ceq DeltaV²/2`.

At finite t, integrated resistive dissipation is `Eloss[1−exp(−2t/tau)]`. The Berkeley source presents equal-C and source-limit cases; the unequal-C extension here follows from the displayed conservation law, not an unsupported claim about the source.

### Domain and limitations

`C1>0`, `C2>0`, `R>0`, `t>=0`. Signed initial voltages are allowed, as is `V1=V2` with zero redistribution. Zero R is excluded because ideal instantaneous connection is not this dissipative model. Parasitic inductance can change peak current and waveform substantially. Switch resistance/ESR may be lumped into R only when their linear approximation is appropriate. No charge pump, converter efficiency or bootstrap capacitance is calculated.

### Independent worked example

`C1=100 microF`, `C2=200 microF`, `R=10 ohm`, `V1=8 V`, `V2=2 V`, `t=2/3 ms`.

- `Ceq=(100×200/300) microF=66.66666666666667 microF`.
- `tau=0.0006666666666666668 s`, so `t/tau=1` to rounding.
- `Vf=(100×8+200×2)/300=4 V`.
- `v1=4+4 exp(−1)=5.471517764685769 V`.
- `v2=4−2 exp(−1)=3.264241117657115 V`.
- `i=0.6 exp(−1)=0.2207276647028654 A`.
- `Qtransfer=400 microC×(1−exp(−1))=252.8482235314231 microC`.
- Initial energy `=0.0036 J`; final energy `=0.0024 J`.
- Total loss `=0.0012 J`; loss accumulated at t `=0.0012(1−exp(−2))=0.001037597660116065 J`.


### Executable input contract

| Key | Unit | Default | Admissible range |
|---|---|---:|---|
| `c_one` | F | 0.0001 | [1e-15, 1.0] |
| `c_two` | F | 0.0002 | [1e-15, 1.0] |
| `r` | ohm | 10.0 | [0.001, 1000000000.0] |
| `v_one` | V | 8.0 | [-1000000.0, 1000000.0] |
| `v_two` | V | 2.0 | [-1000000.0, 1000000.0] |
| `time` | s | 0.0006666666666666668 | [0.0, 1000000000.0] |

Initial-state decay plus equilibrium buildup (`v_i=Vi exp(−t/tau)+Vf[−expm1(−t/tau)]`) and `−expm1(−t/tau)` charge transfer preserve startup continuity; accumulated energy uses `−expm1(−2t/tau)`. At t=0 the original node voltages and zero transferred charge/loss are returned. Equal initial voltages produce no current, charge transfer or loss; either voltage polarity and either transfer direction are supported. Positive capacitor minima protect both the total-capacitance divisor and tau; positive resistance excludes the nonphysical ideal instantaneous zero-R connection. Total charge and `initial_energy=final_energy+total_loss` are independent conservation identities. Very small corrections to nonzero absolute node voltages may still round away.

No additional relational constraint is needed: positive component minima protect divisors, time is nonnegative, and any logarithm has an explicit positive tolerance bound. Signed excitations and valid zero boundaries are not rejected.

### Complete worked outputs (SI)

All input values for this independent example are `{"c_one":0.0001,"c_two":0.0002,"r":10.0,"v_one":8.0,"v_two":2.0,"time":0.0006666666666666668}`.

| Output key | Unit | Independent expected value | Executable expression |
|---|---|---:|---|
| `equivalent_capacitance` | F | 6.666666666666667e-05 | `c_one*c_two/(c_one+c_two)` |
| `tau` | s | 0.0006666666666666668 | `r*equivalent_capacitance` |
| `final_voltage` | V | 4.0 | `(c_one*v_one+c_two*v_two)/(c_one+c_two)` |
| `voltage_one` | V | 5.471517764685769 | `v_one*exp(-time/tau)+final_voltage*(-expm1(-time/tau))` |
| `voltage_two` | V | 3.2642411176571153 | `v_two*exp(-time/tau)+final_voltage*(-expm1(-time/tau))` |
| `current` | A | 0.2207276647028654 | `(v_one-v_two)/r*exp(-time/tau)` |
| `transferred_charge` | C | 0.0002528482235314231 | `equivalent_capacitance*(v_one-v_two)*(-expm1(-time/tau))` |
| `initial_energy` | J | 0.0036 | `(c_one*v_one^2+c_two*v_two^2)/2` |
| `final_energy` | J | 0.0024 | `(c_one+c_two)*final_voltage^2/2` |
| `total_loss` | J | 0.0012 | `equivalent_capacitance*(v_one-v_two)^2/2` |
| `dissipated_energy` | J | 0.0010375976601160648 | `total_loss*(-expm1(-2*time/tau))` |


## `transients.rc_ramp_source` — capacitor tracking of a ramped source

**Level:** intermediate. **Design purpose:** predict startup lag and current when the source ramps at finite slope; distinguish voltage-ramp excitation from an imposed constant capacitor current. Sources: [MIT Rowell equations 21, 30 and 32](https://web.mit.edu/2.151/www/Handouts/FirstSecondOrder.pdf), [MIT RC governing law](https://circuits.mit.edu/F25/psets/05/rc_tutorial).

### Derivation

For `Vs(t)=Vs0+mt`, solve `tau dvC/dt+vC=Vs0+mt`. The particular solution is `Vs0+m(t−tau)`. Adding the homogeneous term and imposing `vC(0)=V0` gives

`vC(t)=Vs0+m(t−tau)+(V0−Vs0+m tau)exp(−t/tau)`.

The source-minus-capacitor tracking error can be evaluated without subtracting two very large ramp voltages:

`Verror=m tau+(Vs0−V0−m tau)exp(−t/tau)`.

Then `iC=Verror/R`. Long after startup, current approaches `Cm` and lag approaches `m tau`; initial current remains `(Vs0−V0)/R`, not necessarily `Cm`.

### Domain and limitations

`R,C>0`, `t>=0`; `m`, `Vs0` and `V0` may be signed. The source must continue to follow the ramp over the entire selected interval. Ramp stopping/clipping and nonlinear load/current limits are not represented. The promoted implementation isolates the second-order ramp term using the shared stable `expm1mx_ratio` primitive and scaled multiplication order, preserving the researched short-time domain; absolute nonzero voltages still have ordinary machine-spacing limitations. This is neither amplifier slew-rate design nor a frequency-domain filter calculation.

### Independent worked example

`R=1 kohm`, `C=1 microF`, `Vs0=V0=0`, `m=1000 V/s`, `t=2 ms`.

- `tau=0.001 s`; `Vs=1000×0.002=2 V`.
- `vC=1000[0.002−0.001+0.001 exp(−2)]=1.135335283236613 V`.
- Tracking error `=1−exp(−2)=0.8646647167633873 V`.
- Current `=0.8646647167633873/1000=0.0008646647167633873 A`.


### Executable input contract

| Key | Unit | Default | Admissible range |
|---|---|---:|---|
| `r` | ohm | 1000.0 | [0.001, 1000000000.0] |
| `c` | F | 1e-06 | [1e-15, 1.0] |
| `v_source_initial` | V | 0.0 | [-1000000.0, 1000000.0] |
| `v_initial` | V | 0.0 | [-1000000.0, 1000000.0] |
| `slope` | V/s | 1000.0 | [-1000000000.0, 1000000000.0] |
| `time` | s | 0.002 | [0.0, 1000000000.0] |

Let `q(z)=(exp(z)−1−z)/z²`, with its analytic continuation `q(0)=1/2`. The identity `t−tau[1−exp(−t/tau)]=(t²/tau)*q(−t/tau)` isolates the quadratic startup term. The parent-owned primitive `expm1mx_ratio(z)` computes q stably near zero without forming an underflowed quadratic remainder. The executable voltage is `V0 exp(−x)+Vs0[−expm1(−x)]+m*t*(t/tau)*expm1mx_ratio(−x)`, where `x=t/tau`; multiplying `m*t` before the second small factor preserves representable ramp contributions. Tracking error is independently `(Vs0−V0)exp(−x)+m*tau[−expm1(−x)]`, avoiding subtraction of large absolute ramp voltages. At t=0 the capacitor retains V0 exactly, and arbitrary signed or zero slopes remain admissible. With default components and t=1e-20 s, the independent leading term is `m*t²/(2*tau)=5e-35 V`, not zero. Positive R/C protect divisions; the primitive's analytic zero limit needs no extra short-time guard or seed output. Very small increments to a nonzero absolute voltage may necessarily round away.

No additional relational constraint is needed: positive component minima protect divisors, time is nonnegative, and any logarithm has an explicit positive tolerance bound. Signed excitations and valid zero boundaries are not rejected.

### Complete worked outputs (SI)

All input values for this independent example are `{"r":1000.0,"c":1e-06,"v_source_initial":0.0,"v_initial":0.0,"slope":1000.0,"time":0.002}`.

| Output key | Unit | Independent expected value | Executable expression |
|---|---|---:|---|
| `tau` | s | 0.001 | `r*c` |
| `source_voltage` | V | 2.0 | `v_source_initial+slope*time` |
| `capacitor_voltage` | V | 1.1353352832366128 | `v_initial*exp(-time/tau)+v_source_initial*(-expm1(-time/tau))+slope*time*(time/tau)*expm1mx_ratio(-time/tau)` |
| `tracking_error` | V | 0.8646647167633873 | `(v_source_initial-v_initial)*exp(-time/tau)+slope*tau*(-expm1(-time/tau))` |
| `capacitor_current` | A | 0.0008646647167633873 | `tracking_error/r` |

