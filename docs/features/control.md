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

