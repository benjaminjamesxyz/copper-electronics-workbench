# AC impedance and resonance — eight researched calculations

## Scope and evidence

This lane contains exactly **8 distinct steady-state physical calculations**. Inductor and capacitor reactances are different constitutive laws for magnetic and electric storage, not inverse rearrangements of one calculation. Series and parallel impedance models use different topologies; resonance characterization, lossy-coil phase resonance, measured-load complex power, and compensating-capacitor sizing solve separate design questions. Filter transfer functions/synthesis, time-domain transients, magnetic geometry, DC resistor power, and switch-mode conversion remain outside this lane.

Research used web search and direct reads of university teaching material. Search phrases included `site.edu series parallel RLC impedance resonance Q bandwidth lecture`, `site.edu parallel resonance coil resistance frequency L C R resonance`, and `site.edu power factor correction capacitor P tan phi capacitance voltage omega`. A manufacturer search for Eaton power-factor correction was also attempted, but its PDFs timed out; no uninspected Eaton content is cited as evidence. There was no date restriction: these are classical linear circuit laws, not a survey of current component technology. English-language university and manufacturer derivations were preferred; generic calculator sites, filter synthesis, and unsupported practical ratings were excluded. Clemson's available 2020 lab manual was inspected as corroboration of ideal resonance, but was not used as evidence for the exact lossy-coil formula.

### Sources and weighting

- **[M1] MIT OCW, _Alternating-Current Circuits_, sections 12.2–12.4 and 12.6.** [Direct PDF](https://ocw.mit.edu/courses/8-02t-electricity-and-magnetism-spring-2005/77f396b6d9907db677e2b94da90613be_cha12ac_circuits.pdf). University-authored course material; high weight for explicit constitutive laws, series impedance, phase, and RMS average power. An instructional primary reference, not an experimental validation of component parasitics.
- **[M2] MIT OCW, Chaniotakis and Cory, _Frequency response: Resonance, Bandwidth, Q factor_, Spring 2006.** [Direct PDF](https://ocw.mit.edu/courses/6-071j-introduction-to-electronics-signals-and-measurement-spring-2006/5bcec4bfba5f2e99754b77509e9e7ab4_resonance_qfactr.pdf). University-authored analytical derivation; high weight for exact ideal half-power endpoints, Q, and series/parallel circuit topology. Same university as M1, so not counted as independent empirical confirmation.
- **[G] Georgia State University HyperPhysics, R. Nave, _Parallel Resonant Circuits_.** [Source](http://hyperphysics.phy-astr.gsu.edu/hbase/electric/parres.html). University teaching reference; high weight for its explicit distinction between phase-defined resonance and impedance maximum, and admittance addition. HTTPS fetch failed because of the site's certificate; HTTP source was successfully read. The specialized winding-loss equation below is independently derived from the circuit's admittance rather than copied from a graphic.
- **[I] University of Illinois at Urbana–Champaign, George Gross, ECE 333, _Power System Basics – AC Analysis_, 2006–2018.** [Direct PDF](https://courses.grainger.illinois.edu/ece333/fa2018/handouts/ECE_333_Lecture%202_Power%20System%20Basics.pdf). University-authored lecture derivation; high weight for slides 45–48 (complex power) and 52–58 (correction and capacitance). Slide 58 explicitly gives C = Qc/(omega Vc²). No numerical worked example from the slides is reused here.

The cited laws agree within their stated ideal models. A potentially misleading terminology difference is resolved explicitly: for a winding-loss parallel tank, phase resonance need not coincide with the maximum of impedance. Confidence is high in the algebra under the models, not in predictions of an unspecified real component. No evidence was sought or found here to justify omitted core loss, ESR frequency dependence, harmonic distortion, or capacitor-bank safety compliance.

## Shared conventions and implementation domains

Use f in Hz, angular frequency omega = 2 pi f in rad/s, L in H, C in F, resistance in ohm, voltage/current in **RMS** V/A, conductance/susceptance in S, real power in W, reactive power in var, apparent power in VA, and phase in rad. `j` is explanatory notation only; JSON expressions are real scalars. The phasor convention is exp(j omega t). Impedance phase is voltage phase minus current phase: positive means current lags (inductive), negative means current leads (capacitive). Power factor and quality factor are dimensionless (`unit: "1"`).

JSON inputs have broad finite numerical envelopes to prevent extreme floating-point overflow/underflow, not to assert physical component validity: f = 1e-6…1e12 Hz; L = 1e-15…1e3 H; C = 1e-18…1e3 F; R = 1e-9…1e12 ohm; ordinary source V = 0…1e9 V. Power-factor and power-input envelopes are stated in their corresponding JSON input metadata. Positive lower bounds guard all reciprocals. Component-specific operating limits, particularly self-resonance and saturation, must additionally be checked by the designer.

The two cross-input constraints are evaluated using **inputs only**: L > C r² for lossy resonance; target_pf >= initial_pf for correction. A third constraint bounds abs(phase) <= pi/2 for a passive load. The phase constraint precedes trigonometric outputs. All square-root arguments are nonnegative after input bounds and these constraints. Each output references only inputs/constants or preceding outputs; each example contains every output key. No conditionals, complex-number operations, arrays, or unsupported functions are needed in implementation.

All JSON defaults equal the worked-example inputs. Numerical examples were independently computed with elementary arithmetic and, for impedance, Python complex phasors and atan2, **not** the backend expression engine. Relative comparison tolerance is 1e-6. This is research-example arithmetic, not a build/test/formatter run.

## 1. `ac.inductor_reactance` — ideal inductor AC loading (basic)

**Design question:** What reactance and RMS current does an ideal inductor present at the operating frequency?

From v = L di/dt, a sinusoidal phasor obeys V = j omega L I. Therefore ZL = j omega L, XL = 2 pi f L, I_rms = V_rms/XL, and arg(ZL) = +pi/2. Dimensions are (1/s)(V s/A) = ohm. Voltage leads current by a quarter-cycle, independent of amplitude.

**Inputs and domain:** positive f and L, nonnegative RMS voltage. The nonzero lower bounds ensure finite current, including at V = 0. The displayed phase at zero drive is the component impedance phase, not a measurable waveform phase.

**Limits:** ideal linear lumped model; no copper/core loss, saturation or winding capacitance. Operate below self-resonance and check actual component current ratings. This calculator takes L as supplied; it does not derive inductance from magnetic geometry.

**Source:** [M1], equations 12.2.11–12.2.17.

**Independent example:** f = 1000 Hz, L = 0.01 H, V = 5 V RMS. Since omega = 6283.18530718 rad/s, XL = 62.83185307179586 ohm. Dividing 5 V by this reactance gives 0.07957747154594767 A RMS. Phase = 1.5707963267948966 rad. These are all three JSON expected outputs.

## 2. `ac.capacitor_reactance` — ideal capacitor AC loading (basic)

**Design question:** What signed impedance and AC current does a capacitor present?

From i = C dv/dt, I = j omega C V and ZC = 1/(j omega C) = -j/(omega C). Define the positive magnitude XC = 1/(2 pi f C); imaginary impedance is -XC. RMS current is V_rms/XC, and impedance phase is -pi/2. Farad has units A s/V, so reciprocal omega C has units V/A.

**Inputs and domain:** positive f and C, nonnegative RMS voltage. DC is excluded because ideal capacitive reactance would diverge. The negative sign is explicit to avoid adding an unsigned capacitive magnitude into a complex-impedance sum.

**Limits:** no ESR, leakage, dielectric loss, bias-dependent capacitance or ESL. Above self-resonance a real capacitor is not capacitive. Rating selection is not implied.

**Source:** [M1], equations 12.2.18–12.2.24.

**Independent example:** f = 1000 Hz, C = 1e-6 F, V = 5 V RMS. XC = 1/0.00628318530718 = 159.15494309189535 ohm; Im(ZC) = -159.15494309189535 ohm. From the constitutive-current form omega C V, I = 0.031415926535897934 A RMS. Phase = -1.5707963267948966 rad. All four expected outputs are shown.

## 3. `ac.series_rlc_impedance` — complete series network (intermediate)

**Design question:** What current, magnitude and phase will a series RLC network draw at a specified off-resonance frequency?

Series voltages add, so Z = R + j omega L + 1/(j omega C) = R + jX, where X = omega L - 1/(omega C). Its magnitude is sqrt(R² + X²). Because R > 0, atan(X/R) gives the full required phase without atan2. Current is V_rms/|Z| and displacement power factor is cos(phi) = R/|Z|. Resistance is real impedance; both real and imaginary outputs are in ohm.

**Inputs and domain:** positive R, L, C, f; nonnegative V. R > 0 prevents the ideal infinite-current LC singularity at resonance and keeps the atan denominator positive. At exact resonance, X = 0, |Z| = R, phase = 0, and PF = 1.

**Limits:** constant lumped elements, one sinusoidal frequency. A constant R can include winding/ESR/source resistance, but cannot represent frequency-dependent or non-ohmic loss. This is not filter gain or waveform settling.

**Source:** [M1], equations 12.3.6, 12.3.15 and 12.4.5.

**Independent example:** R = 100 ohm, L = 0.01 H, C = 1e-6 F, f = 1000 Hz, V = 5 V RMS. Direct addition of ZR = 100, ZL = j62.83185307179586, and ZC = -j159.15494309189535 gives:

| Output | Expected value |
|---|---:|
| Real impedance | 100 ohm |
| Imaginary impedance | -96.32309002009949 ohm |
| Magnitude (complex modulus) | 138.84573335547688 ohm |
| Phase (atan2 of complex result) | -0.7666714810055317 rad |
| Source RMS current | 0.03601118939102619 A |
| Displacement PF | 0.7202237878205238 |

Negative phase correctly identifies capacitive current lead.

## 4. `ac.parallel_rlc_impedance` — three parallel branches (intermediate)

**Design question:** What is the source impedance of a resistor, ideal inductor and ideal capacitor wired as separate parallel branches?

Currents add under a shared terminal voltage. Thus Y = 1/R + 1/(j omega L) + j omega C = G + jB with G = 1/R and B = omega C - 1/(omega L). Complex inversion gives Z = (G-jB)/(G²+B²). Therefore Re(Z) = G/(G²+B²), Im(Z) = -B/(G²+B²), |Z| = 1/sqrt(G²+B²), phase = -atan(B/G), and I_rms = V_rms sqrt(G²+B²).

**Inputs and domain:** positive finite parallel R, L, C and f; nonnegative V. G > 0 guarantees all denominators nonzero even when B = 0. At ideal parallel resonance, B = 0 and Z = R: a finite resistor dissipates power despite cancellation of reactive branch currents.

**Limits:** importantly, R is a **shunt** resistor, not winding resistance in series with L. That distinct practical topology is calculator 6. No source/load additional impedance or component parasitics are modeled.

**Sources:** [M2], parallel circuit and equation 1.20; [G], definition and addition of admittance. The displayed scalar equations are algebraic expansion of the cited complex relations.

**Independent example:** f = 1000 Hz, R = 1000 ohm, L = 0.01 H, C = 1e-6 F, V = 5 V RMS. Compute separate complex branch admittances and invert their sum:

| Output | Expected value |
|---|---:|
| Conductance | 0.001 S |
| Susceptance | -0.009632309002009948 S |
| Real impedance | 10.66309788872204 ohm |
| Imaginary impedance | 102.71025378285057 ohm |
| Impedance magnitude | 103.26227718156345 ohm |
| Impedance phase | 1.4673496474397956 rad |
| Source RMS current | 0.04842039258158743 A |

The sign is inductive, despite the series network in example 3 being capacitive at the same L, C and frequency: series impedance and parallel admittance are physically different additions.

## 5. `ac.series_resonator_bandwidth` — resonance characterization (intermediate)

**Design question:** For an existing voltage-driven series resonator, what are resonance, energy-loss Q and the exact resistor half-power endpoints?

Reactive cancellation gives omega0 L = 1/(omega0 C), hence omega0 = 1/sqrt(LC) and f0 = omega0/(2 pi). The ratio of maximum stored energy to per-cycle resistive loss gives Q = omega0 L/R = sqrt(L/C)/R. Resistor power at constant source RMS voltage is V² R/[R² + (omega L - 1/(omega C))²]. Relative half power requires abs(omega L - 1/(omega C)) = R. Solving the two positive quadratic roots gives:

- omegaH = sqrt(omega0² + (R/(2L))²) + R/(2L).
- omegaL = sqrt(omega0² + (R/(2L))²) - R/(2L).
- Their product is omega0², so the implemented lower endpoint is omegaL = omega0²/omegaH, avoiding subtraction cancellation at low Q.
- Delta omega = omegaH - omegaL = R/L; delta f = R/(2 pi L) = f0/Q.

The JSON performs the equivalent arithmetic directly in Hz: fH = sqrt(f0² + (bandwidth/2)²) + bandwidth/2 and fL = f0²/fH. No high-Q approximation is used; f0 is the **geometric**, not arithmetic, mean of endpoints.

**Inputs and domain:** positive constant total series R, L, C. All positive Q are valid. A damped natural time-domain frequency is a different quantity and is not substituted for the driven resonance.

**Limits:** half-power bandwidth refers to resistor dissipation with constant-amplitude ideal voltage excitation. Real source impedance must be included in total R where justified. Frequency-varying loss can break the ideal Q/bandwidth relation. No filter synthesis or transfer gain is produced.

**Source:** [M2], equations 1.9–1.19 and geometric-mean relation 1.12.

**Independent example:** R = 10 ohm, L = 0.01 H, C = 1e-6 F. omega0 = 10000 rad/s and R/L = 1000 rad/s. Independently solve omega² L C ± omega R C - 1 = 0, obtaining omegaL = 9512.49219725 and omegaH = 10512.49219725 rad/s. The expected outputs are f0 = 1591.5494309189535 Hz; Q = 10; bandwidth = 159.15494309189535 Hz; fH = 1673.11509740738 Hz; fL = 1513.9601543154847 Hz. The reactance at the two roots is respectively -10 and +10 ohm, giving |Z| = sqrt(2)R and half the resonant resistor power.

## 6. `ac.lossy_coil_antiresonance` — practical phase resonance (expert)

**Design question:** How does winding resistance shift the unity-power-factor resonance of a coil branch in parallel with C, and what currents/losses occur there?

The circuit has exactly two parallel branches: (r + j omega L) and an ideal C. Applying [G]'s admittance addition and [M1]'s component impedances gives

Y = r/[r² + (omega L)²] + j{omega C - omega L/[r² + (omega L)²]}.

For a positive omega, zero susceptance requires r² + omega² L² = L/C. Therefore

omega_r² = 1/(LC) - r²/L² = (L - C r²)/(L² C).

A positive-frequency solution exists only when **L > C r²**. JSON guards precisely this input-only inequality and uses the factored numerator rather than subtracting two large reciprocal terms. At this frequency, G = r C/L and Zr = 1/G = L/(C r), a purely real finite input resistance. Source current is V/Zr. The coil branch magnitude is sqrt(r² + omega_r² L²) = sqrt(L/C), hence Icoil = V sqrt(C/L). Copper loss = Icoil² r = V²/Zr. The ideal comparison frequency 1/(2 pi sqrt(LC)) shows the winding-resistance shift.

**Units:** C r² has units H, so the existence test compares like dimensions. Zr is in ohm; currents are A RMS; winding loss is W.

**Inputs and domain:** strictly positive r, L, C; V >= 0; L > C r². For r = 0 an ideal tank has infinite impedance and zero source current at resonance, so that singular ideal limit is excluded. Equality yields no positive-frequency phase resonance. The default r is deliberately non-negligible to demonstrate the shift.

**Limits and terminology:** this is **phase resonance**, defined by zero susceptance, not an assertion about maximum impedance. At finite r these definitions can differ, as [G] explicitly warns. No high-Q shortcut, approximate lossless resonance, or lossy-resonator bandwidth claim is used. Winding resistance must be its applicable AC value, not automatically its DC resistance. External shunt resistance, core loss, self-capacitance and capacitor ESR would alter the model.

**Sources:** [G], phase-resonance versus impedance-resonance distinction and admittance; [M1], element phasor relations. The specialized equation and current identities are the above independent derivation, not an unverified quotation from a web-search summary.

**Independent example:** r = 30 ohm, L = 0.01 H, C = 1e-6 F, V = 5 V RMS. L - C r² = 0.0091 H; omega_r² = 91000000 s^-2 and omega_r = 9539.39201417 rad/s. At that frequency the complex coil admittance is 0.003 - j0.00953939201417 S and capacitor admittance is +j0.00953939201417 S, leaving exactly 0.003 S real. Expected outputs:

| Output | Expected value |
|---|---:|
| Phase-resonance frequency | 1518.2413931464207 Hz |
| Lossless comparison frequency | 1591.5494309189535 Hz |
| Input resistance at phase resonance | 333.3333333333333 ohm |
| Source RMS current | 0.015 A |
| Coil branch RMS current (5/100) | 0.05 A |
| Winding dissipation (0.05² × 30) | 0.075 W |

The branch current exceeds source current because reactive branch currents circulate and cancel; this is not a power-creation effect.

## 7. `ac.sinusoidal_power_triangle` — measured electrical load power (basic)

**Design question:** What electrical real, reactive and apparent powers follow from RMS voltage/current and their measured phase difference?

Set v(t) = sqrt(2)V cos(omega t) and i(t) = sqrt(2)I cos(omega t - phi). Averaging v i over a period eliminates the double-frequency term and gives P = VI cos(phi). RMS apparent power is S = VI, and the quadrature component of complex power is Q = VI sin(phi). Consequently P² + Q² = S² and displacement PF = cos(phi).

**Inputs and domain:** V and I strictly positive, abs(phi) <= pi/2 for a passive load. Phase is in **radians**, not degrees. PF = 0 at the purely reactive endpoints is permitted (floating-point cosine may leave a tiny residue). Positive phi/Q means inductive lag; negative phi/Q means capacitive lead. A zero-current load is excluded because its waveform phase/power-factor interpretation is undefined.

**Limits:** applies only to sinusoidal voltage and current of the same frequency; a distorted load's true PF cannot be inferred from fundamental phase alone. Input electrical real power is not shaft output power or converter efficiency. This calculator does not recalculate DC resistor dissipation.

**Source:** [I], slides 45–48; [M1], equation 12.4.4 corroborates RMS average power.

**Independent example:** V = 230 V RMS, I = 4 A RMS, phi = pi/6 = 0.5235987755982988 rad. S = 230 × 4 = 920 VA. P = 920 sqrt(3)/2 = 796.7433714816837 W; Q = 920/2 = 460 var; PF = sqrt(3)/2 = 0.8660254037844387. This exact-angle construction independently supplies all four expected values.

## 8. `ac.shunt_power_factor_correction` — compensator sizing (intermediate)

**Design question:** What capacitance connected across a single-phase inductive load improves its lagging power factor while reducing source current?

For positive lagging PF = cos(phi), tan(phi) = sqrt(1-PF²)/PF. At constant real power P, load reactive demand is P tan(phi_initial); the desired residual demand is P tan(phi_target). The capacitor supplies the difference as leading reactive power. Its **positive rating magnitude** is

Qc = P[sqrt(1-initial_pf²)/initial_pf - sqrt(1-target_pf²)/target_pf].

A shunt capacitor carries Ic = omega C V, so its supplied reactive magnitude is Qc = V Ic = omega C V². Therefore C = Qc/(2 pi f V²). Before/after source RMS currents are P/(V initial_pf) and P/(V target_pf); capacitor RMS current is Qc/V. The actual capacitor's signed Q in the load convention would be -Qc; the catalog label deliberately specifies magnitude.

**Inputs and domain:** P, V, f > 0; 0 < initial_pf <= target_pf <= 1. Bounds prevent square-root violations and zero PF divisions. The cross-input constraint rejects attempts to reduce PF using this capacitor-only correction model. Equality gives zero required capacitance/current and is a valid no-correction result. Unity target is valid because its reactive term is zero. Both supplied PF values are lagging magnitudes, not arbitrary leading factors.

**Limits:** single-phase capacitor across the specified RMS terminal voltage; no delta/wye three-phase ambiguity. Real load power and voltage must stay constant. This is an ideal fundamental-frequency estimate, not an approved capacitor-bank installation specification. Harmonic resonance, overcorrection under variable loads, switching/discharge protection, and capacitor voltage/current ratings are separate work.

**Source:** [I], slides 47, 52–58. Qc follows by subtracting the two cited power triangles; slide 58 explicitly supplies the capacitance relationship.

**Independent example:** P = 1000 W, V = 230 V RMS, f = 50 Hz, initial PF = 0.8, target PF = 0.95. Initial tan(phi) = 0.75; desired tan(phi) = sqrt(0.0975)/0.95 = 0.328684105178863. Initial reactive demand is 750 var and residual is 328.684105178863 var. Their difference, divided by omega V², determines C:

| Output | Expected value |
|---|---:|
| Capacitor reactive magnitude | 421.3158948211366 var |
| Required capacitance | 0.000025351420515678246 F (25.351420515678246 microfarad) |
| Initial source RMS current | 5.434782608695652 A |
| Corrected source RMS current | 4.576659038901602 A |
| Capacitor RMS current | 1.8318082383527678 A |

The capacitor reactive current subtracts from the load's reactive current, not arithmetically from the total current magnitude. P remains 1000 W; improved PF does not imply energy conversion gain.

## Handoff

Catalog: `research/ac.json`; derivations: this file. All eight entries have actual cited primary instructional references, positive/finite valid defaults, complete independently computed expected outputs, explicit model boundaries, and expressions within the contract grammar. No backend/UI files, other lane files, commits, build, lint, tests or formatter runs were performed. Parent integration should evaluate the examples, exercise resonance/no-resonance and correction constraints, and check UI phase/reactive-power sign labels during the normal verification wave.
