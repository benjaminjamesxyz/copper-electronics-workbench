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

**Numerical implementation:** Insertion loss uses the equivalent `(10/ln(10))*log1p(x^2)`. For x≈1e-12 the original `1+x^2` rounds to one and loses a real loss of approximately 4.342944819·1e-24 dB; log1p preserves it. The magnitude itself can necessarily round to unity, but no fake zero attenuation is substituted.

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

## `filters.butterworth_spec_synthesis` — Butterworth order and cutoff from attenuation specifications (expert)

**Purpose/model:** Choose the smallest integer analog all-pole low-pass order and a feasible cutoff interval, not circuit components or a digital/prewarped filter. Unity DC gain gives |H|²=1/[1+(f/fc)^(2N)] and A(f)=10log10[1+(f/fc)^(2N)].

Passband A(fp)≤Ap and stopband A(fs)≥As imply

\[
N_*=\frac{\ln[(10^{A_s/10}-1)/(10^{A_p/10}-1)]}{2\ln(f_s/f_p)},\quad
N=\lceil N_*\rceil,\quad
\frac{f_p}{(10^{A_p/10}-1)^{1/(2N)}}\le f_c\le
\frac{f_s}{(10^{A_s/10}-1)^{1/(2N)}}.
\]

Select fc_min: it meets passband loss with equality and maximizes stopband attenuation for that order. Actual attenuation outputs use that selected cutoff. The interval is nonempty for the required integer order.

**Conditioning correction:** Put a=Ap·ln(10)/10 and δ=(As−Ap)·ln(10)/10. The order numerator is exactly

\[
\ln\frac{e^{a+\delta}-1}{e^a-1}
=\operatorname{log1p}\left(\frac{\operatorname{expm1}(\delta)}
{-\operatorname{expm1}(-a)}\right).
\]

Compute δ from the input difference before exponentiation; the original quotient can round to unity for adjacent positive loss inputs, yielding a false zero order and a division by zero. This stable form, also used by the input-only order-cap constraint, preserves a positive lower bound without epsilon clamping or discarding valid close specifications. The cutoff attenuation factors use expm1(a), and actual loss uses (10/ln10)·log1p((f/fc)^(2N)). Shared Rust log1p/expm1 supply these operations; no second evaluator is introduced. Floating-point ceil is still a discrete decision; independently check exact/near integer-order specification boundaries during QA.

**Inputs/domain/error rationale:** `passband_edge`, `stopband_edge` (Hz) each 1e-6…1e12; `passband_loss` (dB) 0.001…100; `stopband_loss` (dB) 0.001…200. Before outputs require fs/fp≥1.001, As>Ap, and ceil(N*)≤32. Positive losses guard attenuation-factor roots/logs, separated frequencies guard the logarithmic denominator, and the pole cap bounds the supported design envelope. These are catalog limits, not physics or realizability certification.

**Independent worked substitution/default:** fp=1000 Hz, fs=4000 Hz, Ap=1 dB, As=40 dB. The original analytic attenuation terms are 10^0.1−1 and 9999. Their log ratio divided by 2ln4 is 3.8092399086702753, so N=4:

| Output | Expected value | Unit |
|---|---:|---|
| order_bound | 3.8092399086702753 | 1 |
| order | 4.0 | 1 |
| cutoff_min_hz | 1184.003988964071 | Hz |
| cutoff_max_hz | 1264.9268763451062 | Hz |
| actual_passband_db | 1.0 | dB |
| actual_stopband_db | 42.29680198990891 | dB |

**Source/limits:** Gubner Wisconsin ECE431 §7.3/Eqs.7.7–7.10, linked above. Ideal continuous-time response only: no topology, tolerances, parasitics, elliptic zeros or digital prewarp. The stable transformations are algebraically derived implementation improvements, not claimed verbatim source equations.

## `filters.sallen_key_lowpass` — General Sallen-Key low-pass pole, Q and response (expert)

**Purpose/topology:** TI SLOA049D Fig.6-1 labels matter: R1 and R2 are series input resistors; C1 connects the noninverting input to ground; C2 connects the R1/R2 junction to the amplifier output. K=1+R4/R3 is the noninverting amplifier gain; K=1 uses a follower.

\[
H(s)=\frac{K}{Ps^2+Ds+1},\quad
P=R_1R_2C_1C_2,\quad D=R_1C_1+R_2C_1+R_1C_2(1-K).
\]

P has units s² and D units s. Matching the canonical denominator yields f0=1/(2π√P) and Q=√P/D. For x=f/f0,

\[
|H|=\frac{K}{\sqrt{(1-x^2)^2+(x/Q)^2}},\quad
G_\mathrm{dB}=20\log_{10}|H|.
\]

Half-power **relative to DC gain**, not a resonant peak, solves (1−t)²+t/Q²=2, t=x². The rationalized positive root gives

\[
f_{1/2}=f_0\sqrt{\frac{2}{Q^{-2}-2+\sqrt{(Q^{-2}-2)^2+4}}}.
\]

The rationalized form avoids low-Q subtractive cancellation; the Q≤50 envelope bounds cancellation in its reciprocal form. Sequential outputs retain dc_gain, pole_hz, section_q, frequency_ratio, voltage_gain, gain_db and half_power_hz.

**Inputs/domain/error rationale:** `r1`,`r2` (ohm) 0.001…1e9; `c1`,`c2` (F) 1e-15…1; `amplifier_gain` (V/V) 1…100; `f` (Hz) 0…1e12. Input-only constraints √P/50≤D≤20√P require positive damping and 0.05≤Q≤50 before computing any output. Nonpositive D is unstable, not merely an output error to hide. Equal R,C give Q=1/(3−K), explaining the rejected high-gain region. DC is admissible and returns gain K. Positive K and bounded x keep gain/logs finite.

**Independent worked substitution/default:** r1=r2=10000 ohm, c1=c2=1e-8 F, amplifier_gain=2, f=1000 Hz. P=1e-8 s², D=1e-4 s and Q=1. Direct complex division uses denominator 1−0.62831853²+j0.62831853:

| Output | Expected value | Unit |
|---|---:|---|
| dc_gain | 2.0 | V/V |
| pole_hz | 1591.5494309189535 | Hz |
| section_q | 1.0 | 1 |
| frequency_ratio | 0.6283185307179586 | 1 |
| voltage_gain | 2.2925435192800947 | V/V |
| gain_db | 7.206351772370593 | dB |
| half_power_hz | 2024.482149301843 | Hz |

Gain above K and cutoff above f0 are legitimate Q=1 peaking, not clipping. A unity-gain equal-R Butterworth alternative uses C2=2C1 and Q=1/√2.

**Source/limits:** TI SLOA049D §6/Fig.6-1/Eq.21, with §§11–12 nonideal limitations, linked above. No finite GBW, output impedance, loading, slew, saturation, parasitics or component tolerance effects; stable ideal damping is not hardware stability certification.

## `filters.fliege_notch_response` — Fliege notch component and frequency response (intermediate)

**Purpose/topology:** Two-op-amp equal-component Fliege notch per TI SLOA096 Fig.21: R3=R4=R0, C1=C2=C0, R1=R2=RQ and R5=R6 (nominally 100 kohm). The coincident zero/pole natural frequency and Q are f0=1/(2πR0C0), Q=RQ/(2R0).

\[
H(s)=\frac{s^2+\omega_0^2}{s^2+(\omega_0/Q)s+\omega_0^2},\quad
|H|=\frac{|1-x^2|}{\sqrt{(1-x^2)^2+(x/Q)^2}},\quad x=f/f_0.
\]

DC and asymptotically high-frequency gains are unity. At x=1 the denominator is 1/Q>0 and linear gain is zero. There is deliberately **no dB output**: exact ideal cancellation has infinite logarithmic attenuation, not a fabricated finite floor.

**Conditioning correction:** Compute `frequency_ratio=f/notch_hz`, reusing the earlier output rather than multiplying 2πfR0C0. Supplying the reported `notch_hz` then divides an identical finite value by itself, giving exactly x=1 and zero gain, without special-casing input or suppressing an error. Independently rounded center-frequency inputs can still yield a tiny legitimate floating-point residual; this is not a physical mismatch prediction.

Half-power transmission relative to unity solves (1−x²)²=(x/Q)²:

\[
f_L=f_0\frac{2Q}{\sqrt{4Q^2+1}+1},\quad
f_H=f_0\frac{\sqrt{4Q^2+1}+1}{2Q},\quad BW=f_H-f_L=f_0/Q.
\]

These exact edges have fLfH=f0², but unlike band-pass the central region is rejected. Sequential outputs: notch_hz, section_q, frequency_ratio, voltage_gain, bandwidth_hz, lower_edge_hz, upper_edge_hz.

**Inputs/domain/error rationale:** `r0`,`rq` (ohm) 0.001…1e9; `c0` (F) 1e-15…1; `f` (Hz) 0…1e12. Input-only constraints require 0.05≤rq/(2r0)≤50. Positive components prevent frequency/Q divisors from vanishing, Q bounds control the supported damping envelope, and the magnitude denominator is positive at both DC and notch. DC and exact cancellation remain valid.

**Independent worked substitution/default:** r0=10000 ohm, c0=1e-7 F, rq=200000 ohm, f=160 Hz. f0=1000/(2π) Hz and Q=10; direct complex numerator/denominator division at the off-center probe gives:

| Output | Expected value | Unit |
|---|---:|---|
| notch_hz | 159.15494309189535 | Hz |
| section_q | 10.0 | 1 |
| frequency_ratio | 1.0053096491487337 | 1 |
| voltage_gain | 0.10532346435527205 | V/V |
| bandwidth_hz | 15.915494309189535 | Hz |
| lower_edge_hz | 151.3960154315485 | Hz |
| upper_edge_hz | 167.31150974073802 | Hz |

**Sources/limits:** TI SLOA096 §5.1/Fig.21/Eqs.4–5 and SNOA224A §1.2 notch numerator/Q, linked above. Exact component matching, ideal amplifiers and unity symmetric passbands are assumptions. Real mismatch, parasitics and finite loop gain limit notch depth; ideal zero is not a hardware hum-rejection guarantee.

## `filters.rc_highpass_response` — First-order RC high-pass response (basic)

**Purpose/topology:** Series coupling C and shunt R to ground, output across R. Ideal components, negligible source resistance and no additional output loading; this reverses the actual physical output element, not a previous formula.

\[
H(j\omega)=\frac{R}{R+1/(j\omega C)}
=\frac{j\omega RC}{1+j\omega RC},\quad x=2\pi fRC,\quad f_c=\frac1{2\pi RC}.
\]

Then |H|=x/√(1+x²), phase=π/2−atan(x)=atan(1/x) for x>0, and A=10log10(1+x^−2). Sequential outputs are cutoff_hz, frequency_ratio, voltage_gain, phase_rad and attenuation_db.

**Conditioning correction:** Use `atan(1/x)` to preserve tiny high-frequency phase lead: π/2−atan(x) can round to zero while the true lead is ≈1/x. Use `(10/ln(10))*log1p(1/x^2)` for insertion loss: adding tiny x^−2 to unity would lose it. These are equivalent formulas, not frequency-dependent special cases. The matching low-pass insertion loss likewise uses log1p(x²).

**Inputs/domain/error rationale:** `r` (ohm) 0.001…1e9; `c` (F) 1e-15…1; `f` (Hz) 1e-12…1e12. Positive component/frequency bounds give strictly positive x and finite inverse/log arguments; no cross-input constraint is needed. At DC the linear gain is zero, phase is undefined and attenuation is infinite, so f=0 is rejected rather than assigned invented finite values. At fc, gain=1/√2 and phase=π/4. R is the effective shunt resistance; extra loading changes the model.

**Independent worked substitution/default:** r=10000 ohm, c=1e-8 F, f=1000 Hz; x=0.6283185307179586. Direct complex evaluation of jx/(1+jx) gives:

| Output | Expected value | Unit |
|---|---:|---|
| cutoff_hz | 1591.5494309189535 | Hz |
| frequency_ratio | 0.6283185307179586 | 1 |
| voltage_gain | 0.532018044501408 | V/V |
| phase_rad | 1.0098142106862729 | rad |
| attenuation_db | 5.481472749042987 | dB |

**Source:** Colorado PHYS 3330 Lab 3 §5.5, linked above. Real coupling capacitors/source/load impedances and parasitics require separate circuit analysis.
