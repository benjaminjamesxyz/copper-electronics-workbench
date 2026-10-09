# Feedback and control design calculators

The [Rust catalog](../../backend/catalog/control.json) owns all numerical evaluation. Flutter displays the exact-key static illustrations. All inputs must be finite; invalid domains produce errors. Signed failed-design diagnostics remain visible.

## Sources and evidence

Source register and access limitations are preserved from [control research](../../research/expansion/control.md); implementation does not claim fresh source verification or hardware evidence.

| Key | Source and actually read sections | Evidence type and weight |
|---|---|---|
| N | [NASA NTRS, Time Domain Stability Margin Assessment](https://ntrs.nasa.gov/api/citations/20160013363/downloads/20160013363.pdf), slides 4–6 and 12, with limitations on slides 23–24 | Government-hosted engineering presentation; direct phase-delay and margin method. Used for the mathematical conversion, not aerospace compliance claims. |
| S | [Lehigh University ME450, Lecture 5: Classical Feedback Control](https://www.lehigh.edu/~eus204/teaching/ME450_SIRC/lectures/lecture05.pdf), slides 13–15, equations 10–19 | University teaching source; explicit signal injection points and algebra for S, T and negative feedback. Strong for the stated ideal LTI identities. |
| L | [Toronto Metropolitan University, Introduction to Control Systems §13.2](https://pressbooks.library.torontomu.ca/controlsystems/chapter/13-2lead-controller/), equations 13-1–13-11 and design discussion | University-authored open textbook; explicit lead transfer, extrema and crossover normalization. Strong for the stated transfer function. |
| P | [University of Utah ME3210 Lab 7: PID Controllers](https://my.mech.utah.edu/~me3200/labs/S05Labs/S05_PID_L07.pdf), page 1 equation 1 and closed-loop block description | University laboratory handout; explicit parallel PID convention. Used only for the controller convention, not the motor approximation. |
| M | [MIT 18.03 supplementary notes, chapter 13](https://ocw.mit.edu/courses/18-03-differential-equations-spring-2010/d48ecf2076cc72288ff8d35c859d7551_MIT18_03S10_chapter_13.pdf), page 1 equation 1 and characteristic polynomial | University mathematical notes; exact second-order normalization. Used to derive PI coefficient matching, not a new passive-transient model. |
| C | [Texas Instruments SLUA837, Slope Compensation Design for UCC2897A](https://www.ti.com/lit/an/slua837/slua837.pdf), §§2.1–2.2, equation 8 and deadbeat discussion | Manufacturer application report with analysis and later experimental discussion. Strong for the ideal peak-current perturbation model; device-specific implementation is not generalized. |
| R | [University of Illinois ECE486 Lecture 7](https://courses.physics.illinois.edu/ece486/sp2025/documentation/handbook/lec07.html), Routh array derivation and Example 3 | University teaching source; necessary-and-sufficient cubic criterion. Strong within the exact rational topology. |

Evidence gaps and source hygiene:

- Michigan CTMS frequency-response page returned HTTP 403. Two Caltech PDFs failed certificate verification. Neither inaccessible source is used in the catalog.
- Berkeley's first-order-controller slides were readable but text extraction duplicated symbols; they are not needed or cited for these models.
- The TMU page includes typographical intermediate expressions and heuristic phase-margin/overshoot relations. This work uses the internally consistent transfer function and equations 13-4 through 13-7, independently derived below. It does not use those heuristics.
- NASA uses an omega-like symbol for a frequency multiplied by 360. Catalog input is explicitly Hz; angular-frequency formulas would instead use radians and a factor of one.
- Some PDF math glyphs extract poorly. The working formulas below are derived from the readable transfer functions or ramp geometry, rather than blindly transcribed OCR.
- These sources support ideal analytic design. They do not establish a real board's plant model, safe actuator headroom, component tolerance margins, sampling behavior or nonlinear stability.


Examples below are independent research oracles, not executed implementation results. Main owns tests, real-process examples and native-image acceptance.

## 1. `control.delay_phase_budget` — basic

### Purpose, novelty and source

Budget additional propagation/computation latency in an already stable feedback loop. Unlike `timing.synchronous_setup_slack`, this is a frequency-domain feedback stability constraint, not a register arrival-time constraint. Unlike `signals.line_propagation`, it does not derive travel time from cable geometry. Unlike `opamps.dominant_pole_gain_bandwidth`, no amplifier dominant-pole model or bandwidth estimate is solved.

Source N, slides 4–6 and 12, gives phase lag from delay and applies it at gain crossover.

### Equations, units, signs and validity

For a pure delay, `H_d(jω)=exp(-jωt_d)`. It has unit magnitude, so a specified original unity-gain crossover `f_c` does not move. Its lag magnitude in degrees is `360 f_c t_d`. Therefore:

- `added_phase_lag = 360 crossover_frequency added_delay`.
- `remaining_phase_margin = initial_phase_margin - added_phase_lag`.
- `maximum_delay = (initial_phase_margin - target_phase_margin)/(360 crossover_frequency)`.
- `delay_headroom = maximum_delay - added_delay`.

Frequency is Hz, delay is seconds and angles are degrees. Initial margin is in `(0,180]`; target is nonnegative and cannot exceed initial margin. Zero added delay is valid. Negative remaining margin/headroom must remain visible, not be clamped or rejected. A target of zero returns the marginal boundary, not a recommended operating point.

This is exact only for added pure delay at the stated crossover. It assumes a stable nominal continuous-time SISO loop with one relevant crossover. Multiple crossings, sampling/hold amplitude response, time variation, nonlinear clamps and jitter need a full loop assessment. It is not a statement that every loop with a positive local phase margin is stable.

### Complete independent example

Inputs: `crossover_frequency=1000 Hz`, `initial_phase_margin=60 deg`, `target_phase_margin=45 deg`, `added_delay=0.00002 s`.

| Output key | Independent derivation | Expected |
|---|---|---:|
| `added_phase_lag` | `360 × 1000 × 0.00002` | `7.2 deg` |
| `remaining_phase_margin` | `60 - 7.2` | `52.8 deg` |
| `maximum_delay` | `15/360000` | `0.000041666666666666665 s` |
| `delay_headroom` | `15/360000 - 0.00002` | `0.000021666666666666664 s` |

### Independent test cases

- Nominal: the complete vector above.
- Zero delay with other inputs unchanged: lag `0`, remaining margin `60`, maximum delay and headroom both `1/24000 s`.
- `added_delay=1/24000 s`: remaining margin `45 deg`, headroom zero within floating-point tolerance.
- `added_delay=0.0002 s`: lag `72 deg`, remaining margin `-12 deg`, headroom `-0.00015833333333333333 s`; this must calculate, not reject.
- Target equal to initial margin: maximum delay zero; any positive delay has negative headroom.
- Reject zero/negative crossover, nonpositive initial margin, initial margin above 180, negative target, target above initial margin, negative delay, and nonfinite inputs.

### Mandatory diagram

Top half: a negative-feedback path with a reference arrow from `(25,80)` into a summing circle centered `(100,80)`, plus on its left input and minus on its bottom input. Error flows right through a block `L0(s)` at `(170,50)`–`(305,110)`, then through a pure-delay block `exp(-s td)` at `(370,50)`–`(520,110)`, then to output `(650,80)`. Take the unity-feedback pickoff after the delay and route below the blocks at y=145 to the minus input. Label `td` with quantity `added_delay` outside the delay block.

Bottom half: a phase-budget strip at y=205. Mark a fixed crossover with quantity `crossover_frequency`; show separate brackets for the original margin (`initial_phase_margin`), consumed lag (`added_phase_lag`) and retained margin (`remaining_phase_margin`). A separate text label `delay budget` binds `maximum_delay`; do not bind a phase bracket to a time value. Place the target marker with quantity `target_phase_margin`.

Caption: “A pure delay changes phase but not gain. The phase budget applies at the stated crossover; the strip is illustrative, not a plotted loop response.”

## 2. `control.loop_sensitivity` — intermediate

### Purpose, novelty and source

Predict whether feedback rejects an output disturbance or amplifies it, and separately quantify transmission of measurement noise. Nearest old models are `opamps.dominant_pole_gain_bandwidth` and `noise.noninverting_amplifier_budget`. Neither accepts a complex return ratio nor distinguishes output disturbance from measurement noise injection. This is not another noise-source sum or scalar closed-loop amplifier gain.

Source S, slides 13–15 equations 10–19, gives `L=GK`, `S=1/(1+L)`, `T=L/(1+L)` and the distinct injection transfers.

### Equations, units, signs and validity

Use the negative-feedback convention `y=L(r-y-n)+d`, where `d` is an additive output disturbance and `n` is noise added to the measured output. Thus `y=T r+S d-T n`. Enter `L=a+jb` without folding the summing-junction minus sign into L.

Let `D=hypot(1+a,b)`. Then `|S|=1/D`, `|T|=hypot(a,b)/D`. Separate output amplitudes are `d/D` and `n|T|`. Dimensionless gains multiply voltage amplitudes; all amplitudes must use one convention, such as RMS. Noise transmission has a negative complex sign, but its reported amplitude is nonnegative.

The singular point `a=-1,b=0` is excluded. `hypot` avoids premature overflow from squaring real/imaginary inputs. A point close to -1 legitimately returns large amplification. A measured single-frequency point cannot prove global closed-loop stability; these steady-state amplitudes require that stability independently. Do not sum the two amplitudes without their relative phase or stochastic correlation.

### Complete independent example

Inputs: `loop_real=-0.5`, `loop_imaginary=-0.5`, `disturbance_amplitude=0.01 V`, `noise_amplitude=0.001 V`.

`1+L=0.5-j0.5`; its magnitude is `sqrt(1/2)`. Here `S=1+j` and `T=-j`, so their complex sum is one even though their magnitudes do not sum to one.

| Output key | Independent derivation | Expected |
|---|---|---:|
| `return_difference` | `sqrt(0.25+0.25)` | `0.7071067811865476` |
| `sensitivity_magnitude` | `sqrt(2)` | `1.4142135623730951` |
| `complementary_magnitude` | `sqrt(0.5)/sqrt(0.5)` | `1` |
| `disturbance_output` | `0.01 sqrt(2)` | `0.014142135623730952 V` |
| `noise_output` | `0.001 × 1` | `0.001 V` |

### Independent test cases

- Nominal: complete vector above.
- `loop_real=0,loop_imaginary=0`, same amplitudes: return difference `1`, sensitivity `1`, complementary sensitivity `0`, disturbance output `0.01 V`, noise output `0`.
- `loop_real=9,loop_imaginary=0`, same amplitudes: return difference `10`, sensitivity `0.1`, complementary sensitivity `0.9`, disturbance output `0.001 V`, noise output `0.0009 V`.
- Conjugating L must leave all five outputs unchanged.
- `loop_real=-1,loop_imaginary=0.001`: return difference `0.001`, sensitivity `1000`; the large result must not be suppressed.
- Zero disturbance/noise amplitudes are valid and independently produce zero contributions.
- Reject the exact singular point, negative amplitudes and nonfinite inputs. Do not reject negative real/imaginary return-ratio components.

### Mandatory diagram

Draw the actual injection topology, not a generic feedback loop. Reference enters a plus/minus summing circle at `(100,90)`. Forward block `L(jω)` occupies `(200,60)`–`(360,120)`; quantity labels `Re L` and `Im L` bind `loop_real` and `loop_imaginary` above/below it. Its output reaches a second plus summing circle at `(470,90)`. A vertical down arrow labeled `d` with `disturbance_amplitude` enters the top of this second summer. Output y leaves to the right, then branches into the feedback path. A third plus summer on that feedback path adds `n`, with quantity `noise_amplitude`, before returning to the negative input of the first summer. This must not place n on the physical output node.

Below the loop, show two separate transfer statements: `d → y: |S|` bound to `sensitivity_magnitude`, and `n → y: |T|` bound to `complementary_magnitude`. Adjacent outcome labels bind `disturbance_output` and `noise_output`, without an addition sign between them.

Caption: “Output disturbances follow S; measurement noise follows -T. Magnitudes shown separately at one frequency; the diagram does not establish loop stability or combine the sources.”

