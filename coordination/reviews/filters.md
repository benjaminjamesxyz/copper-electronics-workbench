# Independent review — filters

- Reviewed integrated development baseline: `7f12f45` (lane addition diff against `906b351`).
- Decision: **clear** — no evidence-backed, actionable patch-introduced correctness/security defect identified in the eight models.
- Reviewer: ReviewFilters; independent of DevFilters and TestFilters.
- `tested=false`: static source/model/domain review only. No builds, tests, lint, formatters, smoke commands, calculator execution, hardware measurements or desktop runtime checks were performed. Clear is not release certification.

## Scope and provenance

Read `coordination/CONTRACT.md`, filters rows/common acceptance in `coordination/BACKLOG.md`, `coordination/SPRINT.md`, all eight complete objects in `backend/catalog/filters.json` and `research/filters.json`, all of `research/filters.md` and `docs/features/filters.md`. Reviewed the introduced lane patch and existing integration consumers: catalog build inclusion, Rust catalog parsing/ID lookup/constraints/output loop/function registration, JSON-lines request dispatch, Flutter metadata deserialization, result mapping and output iteration. Did not inspect or rely on tester-worktree files.

The research examples have complete expected maps and 1e-6 relative tolerance. All nominal physical equations and substitutions were independently checked algebraically, including transfer-function numerator selection, pole/Q coefficients, edge-root identities, attenuation inequalities and units. The research record claims independently computed complex/Python oracles; this review does not independently certify how those historical computations were executed or claim a new numerical test pass.

## Actually inspected external evidence

1. [Colorado PHYS3330 Lab 3](https://physicslabs.colorado.edu/courses/phys-3330/lab-guides/lab3/): directly read sections 5.4–5.5. Confirms output-across-C low-pass `1/(1+jwRC)`, output-across-R high-pass `jwRC/(1+jwRC)`, phase signs and loading/source caveats.
2. [TI SLOA096](https://www.ti.com/lit/an/sloa096/sloa096.pdf): directly read PDF extraction for section 4.1/Figure 11/equations 1–3 and section 5.1/Figure 21/equations 4–5. Confirms Deliyannis R1=R4=R0, equal capacitors, gain=Q=(R3+R4)/(2R1), reciprocal R2/R3 sizing and Q>0.5 limit; Fliege frequency components, RQ=2QR0, equal R5/R6 and unity-gain limitation. Diagram topology reviewed via the extracted labels/text; no claim of bench validation.
3. [TI SLOA049D](https://www.ti.com/lit/an/sloa049d/sloa049d.pdf): directly read Figure 6-1/equation 21 extraction. Confirms grounded C1, output-feedback C2, K=1+R4/R3 and the denominator coefficients used by this catalog. Mathematical PDF extraction is imperfect; circuit-label agreement was additionally checked by independent nodal algebra below.
4. [TI SNOA224A / AN-779](https://www.ti.com/lit/an/snoa224a/snoa224a.pdf): directly read sections 1.1–1.3. Confirms high-pass quadratic numerator, low-pass constant numerator, band-pass linear numerator, notch sum of quadratic and constant terms, Q definition, geometric edge center, and cascade-product construction.
5. [Gubner ECE431 notes](https://gubner.ece.wisc.edu/ece431/ece431notesDir/ece431notes2007Dec26.pdf): directly read section 7.3, printed pp.64–65, equations 7.7–7.10. Confirms continuous-time Butterworth attenuation inequalities, integer order bound and cutoff interval. Later digital prewarping is correctly excluded.

The additional [TI CIRCUIT060053](https://www.ti.com/tool/CIRCUIT060053) realization link was retained and assessed from catalog/research descriptions, but not independently fetched in this review. Arbitrary-Q canonical high-pass correctness rests on the inspected canonical tutorial and direct algebra, not that particular circuit's hardware operating range. Related TI references are not independent experimental replications. No physical accuracy, notch depth, op-amp stability or standards-compliance certification is inferred.

## All-eight model audit and independent verification handoff

All scenarios below are prescribed to Main; none were executed here. Each complete example's output map was read and its worked substitutions/model agreement assessed, rather than checking only a headline number.

### 1. `filters.rc_lowpass_response`

**Assessment: clear.** Capacitor-divider topology and phase sign agree with Colorado. `x=2*pi*f*r*c`; magnitude squared is `1/(1+x^2)`, phase `-atan(x)`, insertion loss `10/ln(10)*log1p(x^2)`. RC has seconds, cutoff has Hz, gain is V/V. At DC, gain 1/phase 0/loss 0 are admissible. Positive component lower bounds guard cutoff division. With maximum f/R/C, x is about 6.3e21 and x squared remains far below binary64 overflow. Stable log1p preserves small loss independently of a unity-rounded magnitude.

**Independent alternate oracle:** r=1000 ohm, c=1e-6 F, f=0: cutoff=1000/(2*pi) Hz, ratio=0, gain=1, phase=0, loss=0. At f=1000/(2*pi), complex denominator `1+j` gives gain=1/sqrt(2), phase=-pi/4 and loss=10*log10(2). Parent should reject r=0, c=0 and negative frequency; compare tiny positive loss with scale-sensitive rather than blanket absolute tolerance.

### 2. `filters.rc_highpass_response`

**Assessment: clear.** Series-C/shunt-R topology gives complementary numerator jx and positive phase. `atan(1/x)` preserves small high-frequency phase rather than subtracting two nearly equal angles. Loss is `10/ln(10)*log1p(1/x^2)`; positive f excludes zero-gain log/undefined DC phase. Minimum admitted x is approximately 6.3e-30; inverse square stays finite. Upper x remains finite and inverse square representable.

**Independent alternate oracle:** r=1000, c=1e-6, f=1000/(2*pi): ratio=1, gain=1/sqrt(2), phase=+pi/4, loss=10*log10(2). Parent should reject f=0 and nonpositive components. At large x compare phase to 1/x and loss to `(10/ln(10))/x^2` with quantity-specific relative tolerance.

### 3. `filters.second_order_highpass`

**Assessment: clear.** Numerator is K(s/w0)^2, not the low-pass numerator. With t=x^2, half-passband-power solves `t^2+(2-Q^-2)*t-1=0`, matching the positive-root output. Cutoff is referenced to asymptotic K, not resonant peak. Positive f/K prevent log(0); Q=0.05..50 bounds the root cancellation, and x=1e-24..1e18 keeps required powers finite and magnitude positive. No RMS/peak conversion is involved: a linear sinusoidal voltage ratio applies to either consistently specified amplitude convention.

**Independent alternate oracle:** pole_frequency=1000, q=1, passband_gain=2, f=1000: x=1, gain=2, gain_db=20*log10(2), half_power_hz=`1000*sqrt((sqrt(5)-1)/2)`. At the returned half-power frequency expect gain=2/sqrt(2). Parent should test q endpoints and reject f=0/q=0/nonpositive K. The Q=1 case distinguishes true cutoff from pole frequency.

### 4. `filters.deliyannis_bandpass_design`

**Assessment: clear.** Manufacturer resistor mapping and peak gain Q agree with the implementation. R3/R0=2Q-1 and R2/R0=1/(2Q-1) imply R2R3=R0^2; strict Q>0.5 rejects zero/infinite component solution. Gain `x/sqrt((1-x^2)^2+(x/Q)^2)` gives zero at DC and Q at center without a logged-zero singularity. Rationalized lower edge has no subtraction cancellation. Exact edge identities are fH-fL=f0/Q and fLfH=f0^2. Ideal high-Q/GBW/slew/tolerance limits are explicitly qualitative, not inferred from the scalar envelope.

**Independent alternate oracle:** center_frequency=1000, q=1, c0=1e-8, f=1000: all four relevant resistor values R1,R2,R3,R4 equal R0=1/(2*pi*1000*1e-8), gain=peak=1, bandwidth=1000, edges=1000*(sqrt(5)-1)/2 and 1000*(sqrt(5)+1)/2. Parent should evaluate both edges for gain=1/sqrt(2), f=0 for gain=0, and reject q=0.5.

### 5. `filters.butterworth_spec_synthesis`

**Assessment: clear within this static review.** Source inequalities require `N >= ln(S/P)/(2*ln(fs/fp))`, P=10^(Ap/10)-1, S=10^(As/10)-1, and cutoff between fp/P^(1/(2N)) and fs/S^(1/(2N)). Catalog uses the correct bound, ceil, direction and lower-cutoff selection. For a=Ap*ln(10)/10 and delta=(As-Ap)*ln(10)/10, `log1p(expm1(delta)/(-expm1(-a)))` is exactly ln[(exp(a+delta)-1)/(exp(a)-1)]. Positive loss bounds, As>Ap, ratio>=1.001 and the order<=32 constraint guard all output divisions and logarithms. These same input-only expressions execute before outputs. Adjacent unequal loss inputs retain a positive numerator; no artificial epsilon or clamp is used. Finite frequency/loss bounds plus the order-bound relation keep actual stopband powers within exponent range, even though the frequency ratio itself can be large.

**Independent alternate oracle:** fp=1000, fs=2000, Ap=`10*log10(2)`, As=`10*log10(17)` gives real-arithmetic N*=2, N=2, cutoff_min=cutoff_max=1000; losses follow 1+1^4=2 and 1+2^4=17. This is an algebraic boundary, not a claim about a particular rounded decimal vector's ceil result. Parent must check binary64-adjacent specification inputs near integer order with a high-precision oracle for the actual rounded inputs, plus adjacent positive losses, reversed edges/losses, ratio below 1.001 and order>32 rejection. No discrete-ceil runtime behavior was certified here.

### 6. `filters.sallen_key_lowpass`

**Assessment: clear.** Independent nodal derivation: for junction a and noninverting node b, `(b-a)/R2+s*C1*b=0`, hence a=b(1+s*R2*C1). Combining junction KCL with output K*b gives input=b*[1+s*(R1*C1+R2*C1+R1*C2*(1-K))+s^2*R1*R2*C1*C2]. This independently confirms the TI capacitor mapping and the catalog's P/D. Constraints sqrt(P)/50<=D<=20sqrt(P) enforce positive damping and 0.05<=Q<=50 before division. K>=1 matches ideal noninverting implementation. DC gain K, natural pole frequency and true half-DC-power cutoff are correctly distinct. With a=Q^-2-2, cutoff squared ratio is the positive root of t^2+a*t-1=0, rationalized as 2/(a+sqrt(a^2+4)). Q bounds constrain cancellation on the high-Q side; bounds keep P and response powers finite.

**Independent alternate oracle:** r1=r2=10000, c1=1e-8, c2=2e-8, K=1, f=0: P=2e-8 s^2, D=2e-4 s, Q=1/sqrt(2), pole=cutoff=1/(2*pi*sqrt(2e-8)), dc_gain=voltage_gain=1, gain_db=0. Parent should check unity equal-R/equal-C Q=0.5, equal-R/C K=3 rejection (D=0), negative damping, Q range boundaries and gain=K/sqrt(2) at reported cutoff. Finite-component cancellation and exact Q-boundary behavior remain runtime acceptance scenarios, not pass claims.

### 7. `filters.fliege_notch_response`

**Assessment: clear.** Manufacturer Q resistor mapping agrees; numerator 1-x^2 cancels at center while damping term keeps the denominator positive. Ratio=f/notch_hz means exactly reusing an admitted reported center produces x=1 in binary64, without a special case or fake attenuation floor. DC gives unity; omit logarithmic attenuation at the ideal zero. Q guards 0.05..50 are input-only. Exact edges use half-power relative to unity passbands, with width f0/Q and product f0^2, not linear symmetry about f0. Source and docs explicitly warn that perfect component cancellation is not achievable notch-depth certification.

**Independent alternate oracle:** r0=10000, c0=1e-7, rq=20000, f=0: notch=1000/(2*pi), Q=1, ratio=0, gain=1, bandwidth=notch, edges=notch*(sqrt(5)-1)/2 and notch*(sqrt(5)+1)/2. Parent should reuse reported notch for gain exactly 0 when that frequency is within the input envelope, evaluate edge gains=1/sqrt(2), and reject Q below/above bounds. For independently rounded center frequencies use a cancellation-scale oracle, not an arbitrary finite dB floor.

### 8. `filters.buffered_rc_cascade`

**Assessment: clear.** Interstage isolation justifies multiplication of identical first-order responses; the assumptions exclude an unbuffered ladder. N=2..32 and N=floor(N) require a physical integer. Cutoff shift sqrt(expm1(ln(2)/N)), magnitude exp[-N/2*log1p(x^2)], unwrapped phase -N*atan(x) and direct logarithmic loss are algebraically correct. Direct loss avoids log(0) if an extreme linear transmission underflows. Such underflow is honestly disclosed rather than represented as physical exact cancellation. Repeated real poles are correctly distinguished from Butterworth poles.

**Independent alternate oracle:** r=1000, c=1e-6, stage_count=2, f=1000/(2*pi): x=1, voltage_gain=1/2, phase=-pi/2, loss=20*log10(2), stage_cutoff=1000/(2*pi), cascade_cutoff=stage_cutoff*sqrt(sqrt(2)-1), order=2. Parent should test f=0, N=32, reject N=2.5/1/33, and evaluate reported cascade cutoff for gain=1/sqrt(2). Test tiny loss with output-specific relative tolerance and extreme underflow alongside finite dB loss.

## Cross-boundary and guards audit

`backend/build.rs` includes every catalog JSON without a domain-specific allow-list. `Engine::new` deserializes all definitions, indexes every ID and compiles every sequential output and input constraint. `Engine::calculate` uses the ID map, checks exact input keys, finite values and exclusive/inclusive bounds, applies relational constraints, and iterates all output expressions with previous outputs available. Shared `log1p` and `expm1` are registered to native binary64 primitives, so the new catalog function use is consumed rather than silently dropped. JSON `calculate` dispatch routes every ID through this same engine, returns the complete output map, and errors explicitly. Flutter deserializes every supplied calculator, derives domain choices dynamically, maps every returned output and displays `c.outputs` by metadata key. No new filter ID/output/function is lost at a consuming-side switch or fixed registry in the inspected code.

All filter magnitudes are nonnegative by design; there are no signed source-voltage/current or power inputs to hide. Negative `gain_db` is preserved and low-pass/unwrapped phase remains signed. No extra RMS/peak factor belongs in a dimensionless linear voltage transfer. Component/frequency/Q/count/specification guards and qualitative assumptions match the documented model restrictions. All eight model limitations remain visible; no standards-compliance or hardware-accuracy promises were introduced.

## Findings and untested scope

**Actionable findings: none.** No production/research/tests/docs edits requested. Do not reinterpret this as every finite input numerically verified: pathological rounded inputs, integer-order boundaries and severe component conditioning were assessed algebraically but not exercised. Main remains responsible for all eight complete example executions, independent alternate vectors, boundary/error regressions, relative/scale-sensitive tiny-output assertions, actual Rust process integration, Flutter desktop interaction and native-platform acceptance. No tester source or test-result evidence was used. Hardware/source experimental results, tolerances, parasitic behavior, op-amp GBW/slew/swing/common-mode and standards compliance remain outside the ideal-model certification scope.
