# Analog filter calculators

## Scope, execution and evidence

The eight models in `backend/catalog/filters.json` execute offline in the shared Rust expression engine. The desktop frontend consumes that metadata and performs no filter mathematics. All inputs are required finite scalars; constraints run from inputs before sequential output expressions. Supporting outputs remain part of each result, not separate calculators. Every worked example below preserves the independently derived research oracle with relative tolerance **1e-6**; mathematical tolerance is not physical accuracy. Implementation authoring ran no builds, tests, linters, formatters or engine smoke commands.

RC low-pass and high-pass use different output nodes; active high-pass, modified Deliyannis band-pass, Sallen–Key low-pass and Fliege notch have distinct topologies/transmission zeros. Butterworth synthesis selects specification order/cutoff, whereas buffered RC cascading has repeated real poles and is not Butterworth. Bare AC resonance, op-amp bandwidth, time-domain settling and noise remain outside this lane.

The supplied [research record](../../research/filters.md) reports direct manufacturer/university source reads on 2026-10-09. Its evidence register is retained here, without claiming new hardware qualification or independent experimental replication:

| Source | Evidence and material used |
|---|---|
| [Colorado PHYS 3330 Lab 3](https://physicslabs.colorado.edu/courses/phys-3330/lab-guides/lab3/) | University lab guide §§5.4–5.5: ideal RC equations, phase and loading limitations. |
| [TI SNOA224A / AN-779](https://www.ti.com/lit/an/snoa224a/snoa224a.pdf) | Manufacturer tutorial §§1.1–1.3: biquad numerators, Q, half-power edges and isolated cascades. |
| [TI SLOA096](https://www.ti.com/lit/an/sloa096/sloa096.pdf) | Manufacturer report §4.1/Fig.11/Eqs.1–3: modified Deliyannis; §5.1/Fig.21/Eqs.4–5: Fliege notch. |
| [TI SLOA049D](https://www.ti.com/lit/an/sloa049d/sloa049d.pdf) | Manufacturer analysis §6/Fig.6-1/Eq.21: Sallen–Key coefficients; §§11–12: nonideal/tolerance limitations. |
| [TI CIRCUIT060053](https://www.ti.com/tool/CIRCUIT060053) | Concrete active high-pass circuit with finite operating bandwidth; corroborates realizability, not arbitrary-frequency ideal performance. |
| [Wisconsin ECE431, John A. Gubner](https://gubner.ece.wisc.edu/ece431/ece431notesDir/ece431notes2007Dec26.pdf) | University derivation §7.3, printed pp.64–66/Eqs.7.7–7.10: analog Butterworth inequalities; digital prewarping is not used. |

TI references are related manufacturer material, not independent experimental replications. Unreadable Analog Devices results were excluded in the supplied research. These algebraic ideal models do not establish real attenuation, ultimate notch depth, tolerance yield or standards compliance.

## Units and common domain rules

Resistance is ohm, capacitance F, frequency Hz, Q/order/normalized frequency dimensionless (`1`), voltage magnitude V/V and phase rad. `gain_db=20*log10(|H|)` is absolute voltage gain; `attenuation_db=-20*log10(|H|)` is positive voltage insertion loss. Neither claims equal-impedance power transfer.

Component input bounds are inclusive: R=0.001…1e9 ohm, C=1e-15…1 F. Probe frequency is at most 1e12 Hz; DC is permitted only where phase/logarithms remain defined. Positive high-pass probes start at 1e-12 Hz; directly specified natural frequencies span 1e-6…1e12 Hz. Independently specified/derived Q is constrained to 0.05…50, except Deliyannis requires Q>0.5. These are finite catalog envelopes, not assurances of hardware accuracy. Positive bounds exclude zero divisors; squared terms stay within floating-point exponent range. Qualitative loading, bandwidth, slew, swing, parasitic and matching assumptions cannot be certified from scalar inputs.

## `filters.rc_lowpass_response` — First-order RC low-pass response (basic)

**Purpose/topology:** Series R, shunt C; measure across C with zero source resistance and negligible loading. Ideal resistor/capacitor, without tolerance or parasitic effects.

From the capacitive voltage divider,

\[
H(j\omega)=\frac{1/(j\omega C)}{R+1/(j\omega C)}=\frac{1}{1+j\omega RC}.
\]

With x=2πfRC=f/fc, sequential outputs are

\[
f_c=\frac1{2\pi RC},\quad |H|=\frac1{\sqrt{1+x^2}},\quad
\phi=-\arctan x,\quad A=10\log_{10}(1+x^2).
\]

**Inputs/domain/error rationale:** `r` (ohm) and `c` (F) use the common positive component bounds; `f` (Hz) spans 0…1e12. Default/example: r=10000, c=1e-8, f=1000. No cross-input constraint is needed: RC>0 and the magnitude/log argument is at least 1. DC gives unit gain, zero phase and zero attenuation; at fc the magnitude is 1/√2. Loading changes the actual voltage divider and is excluded, not silently compensated.

**Independent worked substitution:** RC=1e-4 s, x=0.6283185307179586. Direct complex division of 1/(1+jx) produces every result below:

| Output | Expected value | Unit |
|---|---:|---|
| cutoff_hz | 1591.5494309189535 | Hz |
| frequency_ratio | 0.6283185307179586 | 1 |
| voltage_gain | 0.8467330159648303 | V/V |
| phase_rad | -0.5609821161086238 | rad |
| attenuation_db | 1.4450701162052881 | dB |

**Source:** Colorado PHYS 3330 Lab 3 §5.4, linked in the evidence register.

## `filters.deliyannis_bandpass_design` — Modified Deliyannis band-pass component design (expert)

**Purpose/topology:** Synthesize TI SLOA096 Figure 11's modified Deliyannis resistor network, not a bare RLC resonator. C1=C2=C0; R1=R4=R0; R5 is shorted and R6 open. Without the optional input attenuator, peak gain magnitude equals Q; inversion sign is not reported.

The source relationships f0=1/(2πR0C0), Q=(R3+R4)/(2R1) and R2R3=R0² imply

\[
R_0=\frac1{2\pi f_0C_0},\quad R_3=R_0(2Q-1),\quad R_2=\frac{R_0}{2Q-1}.
\]

For x=f/f0 and peak G=Q, the canonical band-pass transfer is

\[
H(s)=\frac{G s/(Q\omega_0)}{(s/\omega_0)^2+s/(Q\omega_0)+1},\quad
|H|=\frac{x}{\sqrt{(1-x^2)^2+(x/Q)^2}}.
\]

Half-power relative to the peak solves |1−x²|=x/Q, yielding exact, not narrow-band, edges:

\[
f_L=f_0\frac{2Q}{\sqrt{4Q^2+1}+1},\quad
f_H=f_0\frac{\sqrt{4Q^2+1}+1}{2Q},\quad BW=f_0/Q.
\]

The rationalized lower edge avoids subtraction cancellation; fLfH=f0² and fH−fL=BW. Output order is r0, r3, r2, peak_gain, bandwidth_hz, lower_edge_hz, upper_edge_hz, voltage_gain.

**Inputs/domain/error rationale:** `center_frequency` (Hz) 1e-6…1e12; `q` (1) strictly greater than 0.5 through 50 inclusive; `c0` (F) 1e-15…1; `f` (Hz) 0…1e12. These guard positive finite resistance sizing and stable damping. At Q=0.5, R3=0 and R2 diverges, so this endpoint is rejected. DC has zero linear transmission without a logarithm and is valid. High-Q practical GBW, slew rate and tolerance requirements are qualitative assumptions, not guarantees.

**Independent worked substitution/default:** center_frequency=1000 Hz, q=2, c0=1e-8 F, f=1000 Hz. R0=1/(2π·1000·1e-8); R3=3R0; R2=R0/3. Edge factors are (√17∓1)/4 and x=1 gives peak gain 2.

| Output | Expected value | Unit |
|---|---:|---|
| r0 (R1 and R4 each) | 15915.494309189537 | ohm |
| r3 | 47746.482927568606 | ohm |
| r2 | 5305.164769729846 | ohm |
| peak_gain | 2.0 | V/V |
| bandwidth_hz | 500.0 | Hz |
| lower_edge_hz | 780.7764064044152 | Hz |
| upper_edge_hz | 1280.7764064044152 | Hz |
| voltage_gain | 2.0 | V/V |

**Sources/limits:** TI SLOA096 §4.1/Fig.11/Eqs.1–3 and SNOA224A band-pass Q/edges, linked above. Exact component ratios and ideal op amp only. Independent arbitrary gain selection needs the excluded input attenuator; no preferred-value synthesis or hardware qualification is claimed.
