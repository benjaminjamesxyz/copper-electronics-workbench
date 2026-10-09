# AC impedance and resonance

The eight calculators in `backend/catalog/ac.json` run in the shared Rust expression engine; the desktop frontend displays their metadata and results without implementing numerical formulas. This lane covers sinusoidal steady-state passive networks, resonance characterization and load power/correction. Filter synthesis, transient ringing, magnetic geometry and converter efficiency are separate domains.

## Sources and evidence

- **[M1] MIT OCW, _Alternating-Current Circuits_, sections 12.2–12.4 and 12.6.** [PDF](https://ocw.mit.edu/courses/8-02t-electricity-and-magnetism-spring-2005/77f396b6d9907db677e2b94da90613be_cha12ac_circuits.pdf). University-authored constitutive laws, impedance, phase and RMS power; instructional analytical evidence, not component-parasitic validation.
- **[M2] MIT OCW, Chaniotakis and Cory, _Frequency response: Resonance, Bandwidth, Q factor_, Spring 2006.** [PDF](https://ocw.mit.edu/courses/6-071j-introduction-to-electronics-signals-and-measurement-spring-2006/5bcec4bfba5f2e99754b77509e9e7ab4_resonance_qfactr.pdf). Exact ideal half-power endpoints and Q/topology derivations; not independent empirical confirmation of M1.
- **[G] Georgia State University HyperPhysics, R. Nave, _Parallel Resonant Circuits_.** [Source](http://hyperphysics.phy-astr.gsu.edu/hbase/electric/parres.html). Research read the HTTP source after HTTPS certificate failure. Distinguishes zero-susceptance phase resonance from impedance maximum. The winding-loss formulas below are explicitly derived from admittance, not copied from an uninspected graphic.
- **[I] University of Illinois, George Gross, ECE 333, _Power System Basics – AC Analysis_.** [PDF](https://courses.grainger.illinois.edu/ece333/fa2018/handouts/ECE_333_Lecture%202_Power%20System%20Basics.pdf). Slides 45–48 derive complex power; 52–58 derive correction and C = Qc/(omega Vc²). Worked inputs below are independent of the lecture examples.

Research used direct university material. An Eaton manufacturer-PDF lookup timed out; no inaccessible Eaton content is cited. The cited analytical laws agree under their ideal models, not for unspecified real parts; they establish no neglected ESR, core-loss, harmonic or capacitor-bank safety performance.

## Units, execution and numerical domains

Use frequency f in Hz, omega = 2 pi f in rad/s, L in H, C in F, R in ohm, **RMS** voltage/current in V/A, admittance in S, P in W, Q in var, apparent power in VA and phase in rad. Quality factor and displacement power factor have unit `1`. With exp(j omega t), impedance phase is voltage minus current phase: positive is inductive lag, negative is capacitive lead.

Inclusive finite numerical envelopes are f = 1e-6…1e12 Hz, L = 1e-15…1e3 H, C = 1e-18…1e3 F, R = 1e-9…1e12 ohm, and ordinary source voltage = 0…1e9 V. They bound intermediates away from overflow/underflow and guard reciprocals; they are not physical operating ratings. Actual self-resonance, bias, saturation and applicable AC loss require component data. Feature-specific power/PF envelopes are stated below and in the catalog.

The engine requires exactly the finite named input keys, applies bounds and input-only model constraints before evaluating outputs sequentially, and rejects domain/nonfinite arithmetic. Output expressions use only inputs, constants and earlier outputs. No complex evaluator or Flutter numerical implementation is required. All defaults equal each complete worked input vector. Every example includes all output keys with relative tolerance **1e-6**; research independently derived them using elementary arithmetic and complex phasors/atan2, not runtime results. Numerical tolerance is not physical accuracy. This implementation handoff makes no build/test claim.

## `ac.inductor_reactance` — ideal inductor AC loading (basic)

**Purpose/topology:** Determine reactance and RMS current of a linear ideal inductor driven at one frequency.

From v = L di/dt, V = j omega L I, so ZL = j omega L, XL = 2 pi f L, I_rms = V_rms/XL and impedance phase = +pi/2. Dimensions (1/s)(V s/A) = ohm. Source: [M1], §12.2.2, equations 12.2.11–12.2.17.

**Domain/error rationale:** Positive f and L within the shared envelopes prevent a zero current divisor; V is nonnegative and may be zero. Zero-drive phase is the component impedance phase, not a measured waveform phase. Assumptions exclude copper/core loss, saturation and self-capacitance. Operate below self-resonance and within the component's actual linear/current limits; L is supplied, not synthesized from geometry.

**Complete independent example inputs:** `frequency=1000 Hz`, `inductance=0.01 H`, `voltage_rms=5 V`. omega = 6283.18530718 rad/s, XL = 6283.18530718 × 0.01; 5/XL gives current.

| Output key | Expected value | Unit |
|---|---:|---|
| `reactance` | 62.83185307179586 | ohm |
| `current_rms` | 0.07957747154594767 | A RMS |
| `impedance_phase` | 1.5707963267948966 | rad |

**Parent verification scenarios:** Scale f/L; V=0; reject f=0 or L=0; confirm +pi/2 impedance phase.

## `ac.lossy_coil_antiresonance` — lossy-coil phase resonance (expert)

**Purpose/topology:** Find the unity-power-factor resonance and circulating-current/copper-loss consequences of a series (r + j omega L) branch in parallel with an ideal capacitor, without additional shunt loading.

Adding branch admittances gives Y = r/[r²+(omega L)²] + j{omega C - omega L/[r²+(omega L)²]}. For positive omega, zero susceptance implies r² + omega²L² = L/C, hence omega_r² = (L-Cr²)/(L²C). The factored numerator avoids subtracting two large reciprocal terms. At that frequency G=rC/L, so Zr=L/(Cr) is purely real; source current is V/Zr. Coil branch impedance magnitude is sqrt(L/C), giving Icoil=V sqrt(C/L) and copper loss=Icoil²r=V²/Zr. Ideal comparison frequency is 1/(2 pi sqrt(LC)). Sources: [G] (admittance and resonance definition) and [M1] (phasors); these specialized identities are the displayed independent derivation.

**Units/domain/error rationale:** C r² has units H; the strict input-only constraint L > C r² guards the square root and excludes equality, which leaves no positive-frequency resonance. r, L and C are positive within shared envelopes; r=0 is excluded because ideal resonant tank impedance is infinite. V may be zero. Zr is ohm, frequency Hz, current A RMS and copper loss W.

**Assumptions/limits:** r and L are constant applicable AC values; DC winding resistance is not automatically appropriate. Core loss, self-capacitance, capacitor ESR and external loading are excluded. This is zero-susceptance **phase resonance, not necessarily maximum impedance**; no high-Q or lossless-frequency shortcut is used. Near L=C r² the result is inherently sensitive to component/rounding uncertainty; the strict guard does not certify component tolerances.

**Complete independent example inputs:** `coil_resistance=30 ohm`, `inductance=0.01 H`, `capacitance=1e-6 F`, `voltage_rms=5 V`. L-Cr²=0.0091 H; omega_r²=91000000 s^-2; omega_r=9539.39201417 rad/s. Coil admittance=0.003-j0.00953939201417 S; capacitor admittance=+j0.00953939201417 S. Their sum is 0.003 S; coil current is 5/100=0.05 A and loss is 0.05²×30.

| Output key | Expected value | Unit |
|---|---:|---|
| `resonance_frequency` | 1518.2413931464207 | Hz |
| `ideal_lc_frequency` | 1591.5494309189535 | Hz |
| `resonance_resistance` | 333.3333333333333 | ohm |
| `source_current_rms` | 0.015 | A RMS |
| `coil_current_rms` | 0.05 | A RMS |
| `copper_loss` | 0.075 | W |

Reactive circulating currents cancel, so coil current exceeds source current without creating power. **Parent verification scenarios:** Reject L<=Cr² and r=0; verify zero net susceptance and V²/Zr=Icoil²r; V=0 remains valid.

## `ac.capacitor_reactance` — ideal capacitor AC loading (basic)

**Purpose/topology:** Determine an ideal capacitor's signed imaginary impedance and positive current/reactance magnitudes.

From i=C dv/dt, I=j omega C V, so ZC=1/(j omega C)=-j/(omega C). XC=1/(2 pi f C)>0; Im(ZC)=-XC; I_rms=V_rms/XC; impedance phase=-pi/2. Reciprocal omega C has units V/A=ohm. Source: [M1], §12.2.3, equations 12.2.18–12.2.24.

**Domain/error rationale:** f,C positive within shared bounds guard reciprocals; DC is excluded because ideal capacitive impedance diverges. V>=0 includes zero drive (phase remains the component impedance phase). The negative imaginary sign prevents misuse of positive XC in complex sums.

**Assumptions/limits:** Linear lumped ideal capacitor, one sinusoidal frequency, no ESR, leakage, dielectric loss, ESL or bias dependence. Stay below self-resonance; voltage/current ratings must come from the actual component, not the numerical envelopes.

**Complete independent example inputs:** `frequency=1000 Hz`, `capacitance=1e-6 F`, `voltage_rms=5 V`. omega C=0.00628318530718 S; its reciprocal gives XC, and the independent constitutive current omega C V gives I.

| Output key | Expected value | Unit |
|---|---:|---|
| `reactance_magnitude` | 159.15494309189535 | ohm |
| `impedance_imaginary` | -159.15494309189535 | ohm |
| `current_rms` | 0.031415926535897934 | A RMS |
| `impedance_phase` | -1.5707963267948966 | rad |

**Parent verification scenarios:** Increasing frequency reduces XC and increases current; V=0; reject f=0/C=0; confirm negative imaginary impedance and phase.

## `ac.series_rlc_impedance` — complete series network (intermediate)

**Purpose/topology:** Determine input impedance, phase, RMS current and displacement PF of series R, L and C at a specified off-resonance frequency.

Series voltages add: Z=R+j omega L+1/(j omega C)=R+jX, X=omega L-1/(omega C). |Z|=sqrt(R²+X²); phase=atan(X/R); I=V/|Z|; PF=R/|Z|=cos(phase). R>0 puts Z in the right half-plane, so atan suffices without atan2. Real/imaginary/modulus outputs are ohm; phase rad; current A RMS; PF dimensionless. Source: [M1], §§12.3–12.4, equations 12.3.6, 12.3.15, 12.4.5.

**Domain/error rationale:** f,R,L,C strictly positive within shared envelopes; V>=0. Positive R prevents infinite ideal-LC current at resonance and guards atan/current divisors. At cancellation X=0, |Z|=R, phase=0, PF=1. Floating-point subtraction may leave a small near-resonance residue; zero-boundary comparisons need scale-aware absolute tolerance.

**Assumptions/limits:** Lumped linear constant elements, one sinusoidal frequency; total R may include source/winding/ESR only where constant ohmic loss is justified. Frequency-dependent/nonlinear losses, gain/filter synthesis and time settling are excluded.

**Complete independent example inputs:** `frequency=1000 Hz`, `resistance=100 ohm`, `inductance=0.01 H`, `capacitance=1e-6 F`, `voltage_rms=5 V`. Independently add ZR=100, ZL=j62.83185307179586, ZC=-j159.15494309189535; take the complex modulus/atan2, then compute V/|Z| for current and R/|Z| for PF.

| Output key | Expected value | Unit |
|---|---:|---|
| `impedance_real` | 100.0 | ohm |
| `impedance_imaginary` | -96.32309002009949 | ohm |
| `impedance_magnitude` | 138.84573335547688 | ohm |
| `impedance_phase` | -0.7666714810055317 | rad |
| `current_rms` | 0.03601118939102619 | A RMS |
| `power_factor` | 0.7202237878205238 | 1 |

Negative phase identifies current lead. **Parent verification scenarios:** Compare independent complex impedance, below/above/resonant frequency signs and PF=1 at resonance; V=0; reject zero components/frequency/resistance.

## `ac.parallel_rlc_impedance` — three parallel branches (intermediate)

**Purpose/topology:** Determine source admittance/impedance and RMS current for independent parallel R, ideal L and ideal C branches, not a winding-loss series-RL branch.

Parallel currents add: Y=1/R+1/(j omega L)+j omega C=G+jB, G=1/R, B=omega C-1/(omega L). Complex inversion yields Z=(G-jB)/(G²+B²). Thus Re(Z)=G/(G²+B²), Im(Z)=-B/(G²+B²), |Z|=1/sqrt(G²+B²), phase=-atan(B/G), I=V sqrt(G²+B²). G/B use S; impedance ohm; phase rad; current A RMS. Sources: [M2], parallel circuit/equation 1.20; [G], admittance definition/addition. Scalar equations expand these complex relations.

**Domain/error rationale:** f,R,L,C strictly positive and finite within shared bounds; V>=0. G>0 guards inversion and atan even when B=0. At ideal resonance Z=R, not infinity; a shunt resistor still dissipates power. Near reactive cancellation, comparisons to zero require scale-aware tolerance.

**Assumptions/limits:** Linear lumped single-frequency ideal branches. R is a **shunt** resistor; lossy series-RL coil topology belongs to `ac.lossy_coil_antiresonance`. No extra source/load impedance or parasitics are included.

**Complete independent example inputs:** `frequency=1000 Hz`, `resistance=1000 ohm`, `inductance=0.01 H`, `capacitance=1e-6 F`, `voltage_rms=5 V`. Add branch admittances 0.001, -j0.01591549430919, +j0.00628318530718 S; independently invert their complex sum and compute its modulus/atan2. Source current is V times admittance modulus.

| Output key | Expected value | Unit |
|---|---:|---|
| `conductance` | 0.001 | S |
| `susceptance` | -0.009632309002009948 | S |
| `impedance_real` | 10.66309788872204 | ohm |
| `impedance_imaginary` | 102.71025378285057 | ohm |
| `impedance_magnitude` | 103.26227718156345 | ohm |
| `impedance_phase` | 1.4673496474397956 | rad |
| `current_rms` | 0.04842039258158743 | A RMS |

This result is inductive although the series example is capacitive at the same L/C/f: impedance and admittance sums differ physically. **Parent verification scenarios:** Independent admittance inversion/conjugate sign, B=0 => Z=R, V=0 and rejected zero inputs.

## `ac.series_resonator_bandwidth` — exact series half-power endpoints (intermediate)

**Purpose/topology:** Characterize an existing voltage-driven series RLC resonator's resonance, Q and exact resistor half-power endpoints without filter synthesis or a high-Q approximation.

Reactive cancellation gives omega0=1/sqrt(LC), f0=omega0/(2 pi). Stored-energy/loss ratio gives Q=omega0 L/R=sqrt(L/C)/R. Under fixed RMS voltage, resistor power is V²R/[R²+(omega L-1/(omega C))²]. Half resonant power requires |omega L-1/(omega C)|=R. Solving the positive quadratic roots:

- omegaH=sqrt(omega0²+(R/(2L))²)+R/(2L).
- omegaL=sqrt(omega0²+(R/(2L))²)-R/(2L).
- omegaH omegaL=omega0², so the program instead evaluates **omegaL=omega0²/omegaH**, avoiding low-Q subtractive cancellation.
- Delta omega=R/L; bandwidth=R/(2 pi L)=f0/Q.

The catalog uses equivalent Hz arithmetic fH=sqrt(f0²+(bandwidth/2)²)+bandwidth/2, fL=f0²/fH. f0 is the **geometric**, not arithmetic, endpoint mean. Source: [M2], equations 1.9–1.19, particularly 1.12. Input R/L/C units are ohm/H/F, frequency outputs Hz and Q dimensionless.

**Domain/error rationale:** Positive bounded R,L,C guard LC roots and divisions. All positive Q in those numerical envelopes are allowed; neither high-Q nor underdamping is required. The positive upper root is computed before the lower root, guarding sequential division. At extreme high Q, floating-point subtraction of returned endpoints can lose the small bandwidth; the direct bandwidth output remains the analytical R/(2 pi L).

**Assumptions/limits:** Constant linear total series resistance, ideal L/C and constant-amplitude ideal voltage drive. Include source resistance in R when the ohmic approximation is justified. Half power refers to resistor dissipation relative to resonance. Frequency-varying loss can invalidate Q/bandwidth identities. Driven resonance is not the time-domain damped natural frequency.

**Complete independent example inputs:** `resistance=10 ohm`, `inductance=0.01 H`, `capacitance=1e-6 F`. omega0=10000 rad/s; R/L=1000 rad/s. Independently solve omega²LC ± omegaRC -1=0: omegaL=9512.49219725 and omegaH=10512.49219725 rad/s. Their reactances are -10/+10 ohm, |Z|=sqrt(2)R and resistor power half its resonant value.

| Output key | Expected value | Unit |
|---|---:|---|
| `resonance_frequency` | 1591.5494309189535 | Hz |
| `quality_factor` | 10.0 | 1 |
| `bandwidth` | 159.15494309189535 | Hz |
| `upper_half_power` | 1673.11509740738 | Hz |
| `lower_half_power` | 1513.9601543154847 | Hz |

**Parent verification scenarios:** Low/high Q, fH-fL=bandwidth where resolved, fH fL=f0², positive low root at low Q, and rejection of zero R/L/C. Do not assert symmetric Hz endpoints.
