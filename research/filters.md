# Analog filter design research — eight calculations

## Scope and evidence

This lane contains exactly eight physically distinct analog-filter design calculations. First-order RC low-pass and high-pass filters use different output nodes/topologies and solve different frequency-selection tasks; neither is an inverse rearrangement of the other. The active high-pass biquad, modified Deliyannis band-pass, Sallen–Key low-pass and Fliege notch have different circuit structures and/or transmission zeros. Butterworth specification synthesis selects a response order and cutoff interval. Buffered RC cascading predicts a multi-stage circuit whose repeated real poles are **not** Butterworth poles.

Excluded: bare AC reactance/resonance calculations, RLC resonator Q, generic voltage gain/dB conversion, op-amp bandwidth sizing, noise budgets and time-domain settling. Filter-specific transmission, Q and gain outputs remain in this lane.

Search performed on 2026-10-09 using web search and direct reads of manufacturer application reports and university teaching material. Search phrases covered RC low/high-pass transfer functions, buffered cascades, Sallen–Key damping, Deliyannis band-pass design, Fliege notch design, and Butterworth passband/stopband inequalities. English-language primary engineering references were included without publication-date restriction because the ideal linear-circuit equations are not time-sensitive. Unreadable Analog Devices results were not used as supporting citations. This is a focused equation audit, not an exhaustive literature review or hardware qualification.

| Reference | Evidence type and weight | Material actually used |
|---|---|---|
| [University of Colorado Boulder, PHYS 3330 Lab 3: Filters](https://physicslabs.colorado.edu/courses/phys-3330/lab-guides/lab3/) | University-authored teaching/measurement guide; high weight for elementary circuit equations | Sections 5.4–5.5 give the exact ideal RC transfer functions and phase; source/load effects are discussed explicitly. |
| [TI SNOA224A / AN-779, A Basic Introduction to Filters—Active, Passive, and Switched-Capacitor](https://www.ti.com/lit/an/snoa224a/snoa224a.pdf) | Manufacturer-authored analytical tutorial; high weight for canonical response models | Sections 1.1–1.3 give second-order response numerators, Q, half-power edges and multiplication of isolated sections. |
| [TI SLOA096, More Filter Design on a Budget](https://www.ti.com/lit/an/sloa096/sloa096.pdf) | Manufacturer circuit-design application report; high weight for the stated topology/component relationships | Section 4.1/Figure 11/equations 1–3 specify modified Deliyannis components; section 5.1/Figure 21/equations 4–5 specify Fliege components. |
| [TI SLOA049D, Active Low-Pass Filter Design](https://www.ti.com/lit/an/sloa049d/sloa049d.pdf) | Manufacturer analysis with simulated and measured examples; high weight for ideal equations, conditional for hardware performance | Section 6/Figure 6-1/equation 21 give general Sallen–Key transfer coefficients; sections 11–12 document nonideal/tolerance limitations. |
| [TI CIRCUIT060053, High-pass Sallen-Key filter circuit](https://www.ti.com/tool/CIRCUIT060053) | Manufacturer circuit reference; corroborates realizability, not arbitrary frequency performance | Identifies a second-order Butterworth active high-pass circuit and a finite operating bandwidth. |
| [John A. Gubner, University of Wisconsin, ECE431 notes](https://gubner.ece.wisc.edu/ece431/ece431notesDir/ece431notes2007Dec26.pdf) | University-authored mathematical derivation; high weight for continuous-time Butterworth inequalities | Section 7.3, printed pp. 64–66, equations 7.7–7.10. Only its **analog** equations are used, not the subsequent digital prewarping recipe. |

The multiple TI documents are related manufacturer material, not independent experimental replications. Ideal transfer functions and specification inequalities are algebraic circuit results; confidence is high for those stated models. Hardware attenuation, maximum operating frequency, ultimate notch depth and tolerance yield are not established by this catalog. No claim of standards compliance is made.

## Catalog conventions and domains

`research/filters.json` is the implementation contract: an array of eight objects. Every worked example includes every output, including supporting dimensionless ratios. Constraints use only inputs/constants because they execute before outputs; output expressions may use earlier outputs. Every expression uses the permitted arithmetic/functions, with no complex-number runtime, conditional, array or simulation dependency.

All component units are SI: ohm, F and Hz. Q/order/frequency ratios are dimensionless (`1`), voltage ratios are `V/V`, phase is radians and decibels are explicitly identified. `gain_db` means absolute voltage gain, `20 log10(|H|)`. `attenuation_db` means positive voltage insertion loss, `-20 log10(|H|)`; voltage-ratio squared is **not** an assertion of equal-impedance power transfer.

Input bounds provide a finite numerical design envelope, not a guarantee that the ideal model remains accurate over that entire range. Positive resistors are restricted to 0.001–1e9 ohm and positive capacitors to 1e-15–1 F. Frequencies have an upper bound of 1e12 Hz. Positive probe frequencies for logarithmic high-pass outputs are at least 1e-12 Hz; DC is accepted where zero gain causes no logarithm/undefined phase. Natural-frequency inputs are at least 1e-6 Hz. Q is limited to 0.05–50 where independently specified/derived. Real hardware must additionally satisfy loading, parasitic, op-amp bandwidth, common-mode, output swing and tolerance requirements, outside these models.

Default and example domains were checked by the derivations below: resistances/capacitances are positive; all denominators and square roots are well-defined; logged magnitudes are positive. There are no infinite defaults. Worked numbers were computed independently with Python standard-library complex transfer functions and analytic specification inequalities, **not** by evaluating the catalog expression engine. No build, lint, formatter or test was run.

## 1. `filters.rc_lowpass_response` — basic

**Task:** Determine the cutoff and response of one series-R/shunt-C section, measured across C.

Using the capacitor impedance in the voltage divider:

\[
H(j\omega)=\frac{1/(j\omega C)}{R+1/(j\omega C)}=\frac{1}{1+j\omega RC}.
\]

Let \(x=2\pi fRC=f/f_c\). The real and imaginary denominator give

\[
f_c=\frac{1}{2\pi RC},\quad |H|=\frac{1}{\sqrt{1+x^2}},\quad
\phi=-\arctan x,\quad A=10\log_{10}(1+x^2).
\]

The half-power condition is \(|H|^2=1/2\), hence \(x=1\). At DC, magnitude is 1, phase and attenuation are 0; no singularity occurs.

**Worked example/default:** R = 10000 ohm, C = 1e-8 F, f = 1000 Hz. Independently, \(\omega RC=0.6283185307179586\) and direct complex division yields:

| Output | Value |
|---|---:|
| cutoff_hz | 1591.5494309189535 Hz |
| frequency_ratio | 0.6283185307179586 |
| voltage_gain | 0.8467330159648303 V/V |
| phase_rad | -0.5609821161086238 rad |
| attenuation_db | 1.4450701162052881 dB |

**Limitations/source:** No source resistance or load across the capacitor; include those in a separate circuit analysis if significant. Ideal components only. Colorado Lab 3, section 5.4.

## 2. `filters.rc_highpass_response` — basic

**Task:** Evaluate AC coupling using a series capacitor and a resistor to ground, measured across R.

Reversing the physical positions/output element, not solving a previous formula backward, gives

\[
H(j\omega)=\frac{R}{R+1/(j\omega C)}=\frac{j\omega RC}{1+j\omega RC}.
\]

With \(x=2\pi fRC\),

\[
f_c=\frac{1}{2\pi RC},\quad |H|=\frac{x}{\sqrt{1+x^2}},\quad
\phi=\frac{\pi}{2}-\arctan x,\quad
A=10\log_{10}(1+x^{-2}).
\]

The phase expression avoids dividing by x and is equivalent to \(\arctan(1/x)\) for x > 0. Positive frequency is mandatory: at DC the transmission is zero, phase undefined and logarithmic attenuation infinite. No fake finite DC attenuation is substituted.

**Worked example/default:** R = 10000 ohm, C = 1e-8 F, f = 1000 Hz. Direct complex evaluation of \(j0.62831853/(1+j0.62831853)\) gives:

| Output | Value |
|---|---:|
| cutoff_hz | 1591.5494309189535 Hz |
| frequency_ratio | 0.6283185307179586 |
| voltage_gain | 0.532018044501408 V/V |
| phase_rad | 1.0098142106862729 rad |
| attenuation_db | 5.481472749042987 dB |

**Limitations/source:** R is the effective output shunt resistance; additional loading changes it. Source resistance is assumed negligible. Colorado Lab 3, section 5.5.

## 3. `filters.second_order_highpass` — intermediate

**Task:** Determine second-order active high-pass transmission and the true half-power cutoff from a pole specification.

The canonical stable high-pass biquad has two zeros at DC:

\[
H(s)=\frac{K(s/\omega_0)^2}{(s/\omega_0)^2+s/(Q\omega_0)+1}.
\]

For \(x=f/f_0\), substituting \(s=j\omega\) gives

\[
|H|=\frac{Kx^2}{\sqrt{(1-x^2)^2+(x/Q)^2}}.
\]

To find half-power transmission **relative to K**, set \(|H|^2/K^2=1/2\). With \(t=x^2\), this is

\[
t^2+(2-Q^{-2})t-1=0,
\quad f_{1/2}=f_0\sqrt{\frac{Q^{-2}-2+\sqrt{(2-Q^{-2})^2+4}}{2}}.
\]

The positive root is well-defined for Q > 0. Q = \(1/\sqrt2\) makes t = 1, but arbitrary pole frequency is not generally the cutoff. Probe f must be positive to preserve finite `gain_db`.

**Worked example/default:** f0 = 1000 Hz, Q = 0.7071067811865476, K = 2, f = 500 Hz. Here x = 1/2 and \(|H|=0.5/\sqrt{0.75^2+(0.5/Q)^2}\):

| Output | Value |
|---|---:|
| frequency_ratio | 0.5 |
| voltage_gain | 0.48507125007266594 V/V |
| gain_db | -6.283889300503116 dB |
| half_power_hz | 1000 Hz |

**Limitations/sources:** Ideal stable biquad; may be realized by Sallen–Key or multiple feedback. The response does not extend to infinite frequency in a real op amp. TI SNOA224A sections 1.2–1.3 establish the normalized numerator/denominator; TI CIRCUIT060053 provides a concrete second-order high-pass implementation.

## 4. `filters.deliyannis_bandpass_design` — expert

**Task:** Size the modified Deliyannis band-pass resistor network, not merely report a bare resonator's Q/bandwidth.

Use the exact Figure 11 labels from TI SLOA096. Set \(C_1=C_2=C_0\), \(R_1=R_4=R_0\), short R5 and leave R6 open. The source gives

\[
f_0=\frac{1}{2\pi R_0C_0},\quad Q=\frac{R_3+R_4}{2R_1},\quad R_2R_3=R_0^2.
\]

Consequently,

\[
R_0=\frac{1}{2\pi f_0C_0},\quad R_3=R_0(2Q-1),\quad R_2=\frac{R_0}{2Q-1}.
\]

Without the input attenuation divider, center-frequency gain magnitude is Q. \(Q>0.5\) is essential: at 0.5 the resistor solution becomes zero/infinite. Circuit inversion does not affect the reported magnitude.

For a canonical band-pass section with peak magnitude G,

\[
H(s)=\frac{G s/(Q\omega_0)}{(s/\omega_0)^2+s/(Q\omega_0)+1}.
\]

Here G = Q, yielding \(|H|=x/\sqrt{(1-x^2)^2+(x/Q)^2}\). Relative half-power points solve \(|1-x^2|=x/Q\):

\[
f_L=f_0\frac{2Q}{\sqrt{4Q^2+1}+1},\quad
f_H=f_0\frac{\sqrt{4Q^2+1}+1}{2Q},\quad BW=f_H-f_L=\frac{f_0}{Q}.
\]

The rationalized lower-edge expression avoids subtracting nearly equal numbers. These are **exact** for this ideal second-order model, not a narrow-band approximation; \(f_Lf_H=f_0^2\).

**Worked example/default:** f0 = 1000 Hz, Q = 2, C0 = 1e-8 F, probe f = 1000 Hz. Independently, R3 = 3 R0, R2 = R0/3, and edge factors are \((\sqrt{17}\mp1)/4\):

| Output | Value |
|---|---:|
| r0 (R1 and R4 each) | 15915.494309189537 ohm |
| r3 | 47746.482927568606 ohm |
| r2 | 5305.164769729846 ohm |
| peak_gain | 2 V/V |
| bandwidth_hz | 500 Hz |
| lower_edge_hz | 780.7764064044152 Hz |
| upper_edge_hz | 1280.7764064044152 Hz |
| voltage_gain | 2 V/V |

**Limitations/sources:** No attempt to independently select gain while keeping these resistor equations; TI explains a separate input attenuator for that purpose. High Q increases component-ratio and amplifier-performance demands. TI SLOA096 section 4.1 equations 1–3 establishes the circuit; SNOA224A establishes exact second-order edge/Q definitions.

## 5. `filters.butterworth_spec_synthesis` — expert

**Task:** Select the smallest analog Butterworth low-pass order and permissible cutoff interval from two attenuation requirements.

For unity DC gain,

\[
|H(j2\pi f)|^2=\frac{1}{1+(f/f_c)^{2N}},\quad A(f)=10\log_{10}(1+(f/f_c)^{2N}).
\]

Requirements \(A(f_p)\le A_p\) and \(A(f_s)\ge A_s\) give

\[
(f_p/f_c)^{2N}\le10^{A_p/10}-1,\quad
(f_s/f_c)^{2N}\ge10^{A_s/10}-1.
\]

Dividing the inequalities eliminates cutoff, yielding

\[
N_* =\frac{\ln[(10^{A_s/10}-1)/(10^{A_p/10}-1)]}{2\ln(f_s/f_p)},\quad N=\lceil N_*\rceil.
\]

For that integer order,

\[
\frac{f_p}{(10^{A_p/10}-1)^{1/(2N)}}\le f_c\le
\frac{f_s}{(10^{A_s/10}-1)^{1/(2N)}}.
\]

The catalog selects the lower allowed cutoff to meet the passband specification with equality and maximize stopband rejection for that order. Constraints require As > Ap > 0, fs/fp >= 1.001 and integer N <= 32. These bounds prevent zero logarithm arguments/denominators and impractical unbounded order; they are a catalog envelope, not fundamental physics.

**Worked example/default:** fp = 1000 Hz, fs = 4000 Hz, Ap = 1 dB, As = 40 dB. The attenuation terms are \(10^{0.1}-1\) and 9999, giving:

| Output | Value |
|---|---:|
| order_bound | 3.8092399086702753 |
| order | 4 |
| cutoff_min_hz | 1184.003988964071 Hz |
| cutoff_max_hz | 1264.9268763451062 Hz |
| actual_passband_db | 1 dB |
| actual_stopband_db | 42.29680198990891 dB |

The nonempty interval provides an independent feasibility check. `actual_*` values use the selected lower cutoff.

**Limitations/source:** Ideal continuous-time all-pole response only; no circuit component synthesis, tolerance margin, elliptic notch, or digital prewarp. Gubner ECE431 section 7.3 equations 7.7–7.10 derives precisely these analog inequalities.

## 6. `filters.sallen_key_lowpass` — expert

**Task:** Relate a concrete Sallen–Key circuit's components/amplifier gain to pole frequency, stability damping and response.

Labeling is critical: following TI SLOA049D Figure 6-1, R1 and R2 run from input to the op-amp noninverting node; C1 connects that node to ground; C2 connects the R1/R2 junction to output. The amplifier gain is K = 1 + R4/R3 (a follower is the K = 1 case). The source gives

\[
H(s)=\frac{K}{Ps^2+Ds+1},\quad
P=R_1R_2C_1C_2,\quad D=R_1C_1+R_2C_1+R_1C_2(1-K).
\]

Matching the canonical denominator yields

\[
f_0=\frac{1}{2\pi\sqrt P},\quad Q=\frac{\sqrt P}{D}.
\]

\(P\) has units s² and \(D\) has units s, confirming Hz and dimensionless Q. Positive D is essential for stability. Input-only constraints \(\sqrt P/50\le D\le20\sqrt P\) enforce positive damping and 0.05 <= Q <= 50 before outputs. For equal R and equal C, \(Q=1/(3-K)\), showing why unconstrained large noninverting gain can destabilize this circuit.

With x = f/f0,

\[
|H|=\frac{K}{\sqrt{(1-x^2)^2+(x/Q)^2}}.
\]

Half-power relative to **DC** gain K solves \((1-t)^2+t/Q^2=2\), t = x². Rationalizing the positive root gives

\[
f_{1/2}=f_0\sqrt{\frac{2}{Q^{-2}-2+\sqrt{(Q^{-2}-2)^2+4}}}.
\]

This avoids loss of significance for low Q. The finite Q envelope also bounds the high-Q cancellation in the reciprocal form. DC is allowed and returns K; no quadrant-dependent phase expression is required.

**Worked example/default:** R1 = R2 = 10000 ohm, C1 = C2 = 1e-8 F, K = 2, f = 1000 Hz. Independently P = 1e-8 s², D = 1e-4 s, Q = 1, and the complex denominator is \(1-0.62831853^2+j0.62831853\):

| Output | Value |
|---|---:|
| dc_gain | 2 V/V |
| pole_hz | 1591.5494309189535 Hz |
| section_q | 1 |
| frequency_ratio | 0.6283185307179586 |
| voltage_gain | 2.2925435192800947 V/V |
| gain_db | 7.206351772370593 dB |
| half_power_hz | 2024.482149301843 Hz |

The gain exceeding 2 is legitimate peaking, not clipping prediction. The cutoff above f0 is legitimate for Q = 1. For a unity-gain equal-R Butterworth alternative, choose C2 = 2 C1; Q becomes \(1/\sqrt2\).

**Limitations/source:** Ideal amplifier, no loading, finite GBW, slew rate or parasitic output feedthrough. SLOA049D sections 11–12 explicitly explain departures from the ideal model and component selection. Formula/source: section 6 equation 21.

## 7. `filters.fliege_notch_response` — intermediate

**Task:** Predict a two-op-amp Fliege notch circuit's component-defined center, Q, bandwidth and transmission.

Using TI SLOA096 Figure 21 labels, R3 = R4 = R0, C1 = C2 = C0, R1 = R2 = RQ, and R5 = R6 (the reference uses 100 kohm each). The source component equations are

\[
f_0=\frac{1}{2\pi R_0C_0},\quad R_Q=2QR_0,\quad Q=\frac{R_Q}{2R_0}.
\]

The unity-gain symmetric notch has a pair of transmission zeros at \(\pm j\omega_0\):

\[
H(s)=\frac{s^2+\omega_0^2}{s^2+(\omega_0/Q)s+\omega_0^2},\quad
|H|=\frac{|1-x^2|}{\sqrt{(1-x^2)^2+(x/Q)^2}}.
\]

DC and asymptotically high-frequency gain are unity. At x = 1 the denominator is 1/Q, so magnitude is exactly zero without a singularity. The output intentionally omits logarithmic attenuation; there is no finite true dB value at exact ideal cancellation.

Half-power transmission satisfies \((1-x^2)^2=(x/Q)^2\), giving the same algebraic edge roots as the band-pass, but with opposite pass/reject regions:

\[
f_L=f_0\frac{2Q}{\sqrt{4Q^2+1}+1},\quad
f_H=f_0\frac{\sqrt{4Q^2+1}+1}{2Q},\quad BW=f_0/Q.
\]

This does not duplicate band-pass component synthesis: the circuit has a notch numerator/zeros, two amplifiers and a distinct component-Q relationship. Input-only constraints enforce 0.05 <= RQ/(2 R0) <= 50.

**Worked example/default:** R0 = 10000 ohm, C0 = 1e-7 F, RQ = 200000 ohm, probe f = 160 Hz. Independently f0 = \(1000/(2\pi)\) Hz and Q = 10; direct complex numerator/denominator division gives:

| Output | Value |
|---|---:|
| notch_hz | 159.15494309189535 Hz |
| section_q | 10 |
| frequency_ratio | 1.0053096491487337 |
| voltage_gain | 0.10532346435527205 V/V |
| bandwidth_hz | 15.915494309189535 Hz |
| lower_edge_hz | 151.3960154315485 Hz |
| upper_edge_hz | 167.31150974073802 Hz |

This off-center probe illustrates sensitivity without claiming a real notch-depth limit. At the exact center the ideal model returns zero linear magnitude.

**Limitations/sources:** Exact equal components and ideal amplifiers are essential; mismatch, parasitics and limited loop gain degrade cancellation. Not a prediction of guaranteed hum rejection or wide band-reject performance. SLOA096 section 5.1/Figure 21/equations 4–5; SNOA224A section 1.2 establishes the notch numerator and Q interpretation.

## 8. `filters.buffered_rc_cascade` — intermediate

**Task:** Predict an isolated multi-stage RC circuit, including the overall cutoff shift and accumulated phase/loss.

For N identical sections separated by ideal unity-gain buffers, transfer functions multiply:

\[
H(s)=\left(\frac{1}{1+sRC}\right)^N.
\]

Thus

\[
|H|=(1+x^2)^{-N/2},\quad \phi=-N\arctan x,\quad
A=10N\log_{10}(1+x^2),\quad x=2\pi fRC.
\]

The phase output is **unwrapped**, not forced into a principal interval. Half-power transmission solves \((1+x^2)^{-N}=1/2\):

\[
f_{c,\mathrm{total}}=\frac{\sqrt{2^{1/N}-1}}{2\pi RC}.
\]

N must be a physical integer, guarded by `stage_count == floor(stage_count)`, with 2 <= N <= 32. Different total orders are one parameterized calculation, not separate catalog entries. The individual corner is not the overall corner. Attenuation is evaluated directly from the logarithmic form, avoiding `log10` of a transmission that could numerically underflow for many stages at extreme frequencies.

**Worked example/default:** R = 10000 ohm, C = 1e-8 F, N = 3, f = 1000 Hz. Directly cube the complex first-order transfer function:

| Output | Value |
|---|---:|
| stage_cutoff_hz | 1591.5494309189535 Hz |
| overall_order | 3 |
| cascade_cutoff_hz | 811.4109382567457 Hz |
| frequency_ratio | 0.6283185307179586 |
| voltage_gain | 0.6070709938555954 V/V |
| phase_rad | -1.6829463483258713 rad |
| attenuation_db | 4.3352103486158615 dB |

**Limitations/sources:** Multiplication presumes isolation. A bare unbuffered RC ladder loads preceding sections and needs a different transfer function. Repeated RC real poles cannot reproduce the complex pole pattern of a higher-order Butterworth filter. Colorado Lab 3 section 5.4 supplies the individual stage; TI SNOA224A section 1.3 supplies cascade-product analysis.

## Integration and independent verification handoff

All eight IDs have finite worked defaults and every expected output. Example relative tolerance is 1e-6. The JSON uses only arithmetic, exponentiation, square roots, logarithms, atan, abs, ceil and floor supported by the contract. There is no simulator, code-generated fake result or hidden numerical solver.

Recommended parent behavioral checks (not executed here): compare every worked example; exercise allowed DC for low-pass, band-pass, notch and cascade; reject DC for logarithmic high-pass; reject noninteger stage count; reject Deliyannis Q = 0.5; reject zero/negative components; reject nonpositive Sallen–Key damping and Q outside bounds; reject reversed Butterworth edges/losses and required order > 32. Evaluate the notch at its computed center for zero linear gain, band-pass/notch edge frequencies for 1/sqrt(2) relative transmission, and total cascade cutoff for 1/sqrt(2) transmission. Verify that constraints execute solely from inputs before outputs. These checks distinguish circuit behavior from mere file/catalog counts.
