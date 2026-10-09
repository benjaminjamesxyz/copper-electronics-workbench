# Oscillator expansion review — HOLD

- Reviewer: V2OscillatorsRun (stable owner V2Oscillators).
- Reviewed integration snapshot: `4c46fa1`.
- Intended repository report: `coordination/expansion/reviews/oscillators.md`.
- Authored artifact: `local://oscillators-review.md`; Main will persist this report verbatim because reviewer tools permit scratch writes only.
- **Source verdict: HOLD.** Scientific equations, nominal examples, units, distinct physical scope and ordinary domain rules agree with the research. Seven admitted-domain numerical conditioning defects and one diagram quantity-placement defect need fixes.
- **Pixel verdict: HOLD.** All seven actual native PNGs were opened as images. Three diagrams have visible text/geometry collisions; the triangle diagram additionally misplaces its square-output quantity label.
- **Overall gate: HOLD.** No image is missing. No execution or post-fix approval is claimed.

## Scope and evidence

Reviewed complete `backend/catalog/oscillators.json`, `frontend/lib/diagrams/oscillators.dart`, `docs/features/oscillators.md`, both oscillator research files, expansion contract and backlog items 015–021. Read the baseline's relevant timing, transient, op-amp, filter and AC models. Read primitive pin definitions, painter dispatch, quantity-legend consumer, diagram registry and main lookup, backend catalog discovery, expression evaluation and output ordering.

The patch against `29f9c2b` introduces all three oscillator files. Research-to-production comparison shows the same model expressions and domains; nominal expected-value differences are only last-digit serialization rounding, far inside `1e-9`. Numerical findings below are inherited research issues introduced into production by this patch, not claims that the developer changed the research algebra.

Capture receipt: `coordination/expansion/native_images.json`, source snapshot `4c46fa1`, actual Linux Flutter RepaintBoundary capture at pixelRatio 2, 1400×560 PNGs, composited onto `#f5f4ef` without geometry changes. Main reports completion of all 228 per-model metadata/example assertions, followed by a later temporary-harness menu failure. That receipt is Main's evidence, not a reviewer execution. No overall native acceptance pass follows from it.

### Actual images viewed

Every path below was opened with the image read tool, not inferred from its name:

1. `/home/simson/pi-projects/pcb-design-calculator/coordination/expansion/screenshots/oscillators.timer_555_astable.png`
2. `/home/simson/pi-projects/pcb-design-calculator/coordination/expansion/screenshots/oscillators.comparator_rc_oscillator.png`
3. `/home/simson/pi-projects/pcb-design-calculator/coordination/expansion/screenshots/oscillators.triangle_integrator_oscillator.png`
4. `/home/simson/pi-projects/pcb-design-calculator/coordination/expansion/screenshots/oscillators.wien_bridge_balance.png`
5. `/home/simson/pi-projects/pcb-design-calculator/coordination/expansion/screenshots/oscillators.loaded_rc_phase_shift.png`
6. `/home/simson/pi-projects/pcb-design-calculator/coordination/expansion/screenshots/oscillators.cross_coupled_lc_startup.png`
7. `/home/simson/pi-projects/pcb-design-calculator/coordination/expansion/screenshots/oscillators.pierce_crystal_startup.png`

No independent source/pixel mismatch was established in this lane. Reported collisions are consistent with the label and geometry coordinates in the source. Main's wider capture-provenance investigation remains outside this review's result.

## Model-by-model assessment

| ID | Scientific/source assessment | Native-pixel assessment |
|---|---|---|
| `oscillators.timer_555_astable` | Correct ideal one-third/two-thirds timing, separate charge/discharge paths, positive components, 100 kHz guideline constraint and all five nominal outputs. Numerical HOLD N1. | Correct RA–DISCH–RB–C topology; pins 2/6 share timing node; pins 8/4 high; pin 1 grounded; output separate. Charge/discharge directions and illustrative high/low cycle agree. No material overlap or clipping found. |
| `oscillators.comparator_rc_oscillator` | Correct conductance thresholds and asymmetric `log1p` intervals; all six example outputs agree. Positive feedback at + and timing capacitor at − are correct. Numerical HOLD N2. | RU/RL/RF threshold network and separate RT/C path are correct. RL-ground/timing-path crossing has no Dot and is not a short under the explicit contract. No material collision or clipping found. |
| `oscillators.triangle_integrator_oscillator` | Correct switching condition `Vtri=−RT/RF*Vsq`, slope and amplitude-independent period; all five example outputs agree. Numerical HOLD N3; label-placement HOLD D1. | Two-device loop closes correctly; integrator C returns to −; both references are correct. Falling ramp under positive square output is correct. The ±Vsq label is beside the wrong side of the comparator, not its square output. |
| `oscillators.wien_bridge_balance` | Correct equal-R/C bridge, beta=1/3, A=1+RF/RG, signed resistor-difference excess and all six example outputs. Failed/marginal startup remains diagnostic. Numerical HOLD N4. | Series RC and shunt R∥C connect correctly to +; RF/RG connect to −. Two native text/geometry collisions: D2. |
| `oscillators.loaded_rc_phase_shift` | Independent nodal elimination agrees with `x^3/(x^3+6x^2+5x+1)`; at `x=j/sqrt(6)`, beta=−1/29. Third R terminates at virtual ground without an extra shunt. All six nominal outputs and signed diagnostics agree. Numerical HOLD N5. | Correct three series C sections and two physical shunt R branches; third R returns to −. Critical-RF label crosses the return wire: D3. |
| `oscillators.cross_coupled_lc_startup` | Correct differential −2/gm, per-device gm, signed alpha, complex-pole restriction, and all six nominal outputs. Decaying and marginal cases remain allowed. Numerical HOLD N6. | Opposite-drain gate connections, nonjoining crossing, joined-source tail, center-tapped bias, complete differential L/C/Rp and voltage signs are correct. Total L is not mislabeled as a half-inductor. No material collision or clipping found. |
| `oscillators.pierce_crystal_startup` | Correct equal-load reduction, `4 ESR [2πf(C0+CL)]²`, actual-gm interpretation, zero C0/Cs allowance, CL>Cs guard, M≥1 and signed headroom. All five example outputs agree. Numerical HOLD N7. | Crystal and RF independently bridge inverter nodes; C1/C2 independently reach ground; ESR/C0 are legends, not extra components. OSC_OUT label crosses its node wire: D4. |

### Scope and ordinary-domain reasoning

These are seven distinct autonomous-loop problems, not inverses of baseline passive resonance, externally driven RC response, clock division, amplifier gain or constant-input integration. Component positivity and special domain restrictions are physically justified. No arbitrary maximum was added to suppress a failed startup result. Output dependencies are declared before use. SI units are consistent: RC times in seconds, inverse times in Hz, gm in S, differential resistance in ohms, alpha in 1/s, and ratios dimensionless.

All 39 nominal output entries were compared against the research derivations. Limiting reasoning confirms RA=RB gives 2/3 duty; equal comparator threshold resistors give half duty; supply scaling does not change comparator timing; triangle square amplitude cancels from period; Wien RF=2RG and phase RF=29R yield zero excess; LC gm=2/Rp yields zero growth; and Pierce Cs=0, C0=0 and below-target gm retain their documented meanings.

No primary-source ambiguity required fresh retrieval: the reviewed circuit equations can be checked directly from charge/discharge laws, nodal KCL and the stated differential/terminal conventions. Manufacturer/university source-access claims remain inherited research evidence. This reviewer did not independently re-fetch the TI, Razavi or ST PDFs. Physical limits remain device-specific; nominal numerical precision is not hardware accuracy.

## Actionable findings — source/numerics

All numerical counterexamples below are **independent source/IEEE-754 analysis, not executed engine failures**. `backend/src/checked.rs:19–29,49–50` rejects non-finite intermediate operations, even when later scaling would restore a finite value. The catalog admits these finite positive inputs, and each counterexample has representable complete mathematical outputs. They are numerical-domain probes, not proposed buildable hardware values. T2OscillatorsRun has been notified and is authoring regression cases; Main owns execution. Do not add arbitrary component bounds or weaken the intermediate checker.

### N1 — P2: Scale the 555 timing terms before adding resistances

- **ID:** `oscillators.timer_555_astable`.
- **Anchor:** `backend/catalog/oscillators.json:79–84`; related expressions at 52 and 76.
- **Counterexample:** `charge_resistance=1e308`, `discharge_resistance=1e308`, `timing_capacitance=1e-308`.
- **Evidence/impact:** `2*discharge_resistance` overflows inside the period constraint before C can scale it. The valid nominal period is about `3 ln(2)=2.0794415416798357 s`, well below 100 kHz. The high-time sum and resistance-only duty ratio would also overflow after fixing only the constraint.
- **Expected complete result:** high_time≈`2 ln(2)`, low_time≈`ln(2)`, period≈`3 ln(2)`, frequency≈`1/[3 ln(2)]`, high_duty≈`2/3`.
- **Minimal repair:** Form high_time as `ln(2)*RA*C + ln(2)*RB*C`; retain the scaled low_time and sum the times for period. Form the constraint from the same individually scaled RC terms. Use `high_time/period` for duty. Update all affected expressions together.

### N2 — P2: Normalize comparator threshold conductances before summing

- **ID:** `oscillators.comparator_rc_oscillator`.
- **Anchor:** `backend/catalog/oscillators.json:173–183`.
- **Counterexample:** `supply_voltage=2`, `upper_resistance=1`, `lower_resistance=1`, `feedback_resistance=1e308`, `timing_resistance=1e150`, `timing_capacitance=1e150`.
- **Evidence/impact:** The upper-threshold numerator multiplies 2 by approximately `1e308`; the shared denominator also adds two approximately `1e308` terms. Both exceed binary64 although each threshold is approximately 1 V. Timing remains finite through `log1p`: both half-cycles≈`1e-8 s`, frequency≈`5e7 Hz`, duty=0.5. The physically tiny threshold separation rounds away in displayed binary64 voltages, but that does not justify rejecting the representable timing result.
- **Minimal repair:** Normalize all three positive conductance weights by the largest conductance (equivalently use `m/RU`, `m/RL`, `m/RF` with `m=min(RU,RL,RF)`). Compute each bounded threshold fraction first, then multiply by Vs. Keep the stable `log1p` timing expressions.

### N3 — P2: Multiply integrator R and C before the period's constant factor

- **ID:** `oscillators.triangle_integrator_oscillator`.
- **Anchor:** `backend/catalog/oscillators.json:311–315`.
- **Counterexample:** `square_peak=1`, `triangle_resistance=1`, `feedback_resistance=1`, `integrator_resistance=1e308`, `integrator_capacitance=1e-308`.
- **Evidence/impact:** Leading `4*integrator_resistance` overflows even though RI*C≈1. All outputs are finite: triangle_peak≈1 V, triangle_pp≈2 V, ramp_slew≈1 V/s, period≈4 s, frequency≈0.25 Hz.
- **Minimal repair:** Parenthesize the balanced RC product before multiplying by 4, e.g. `4*(integrator_resistance*integrator_capacitance)*(triangle_resistance/feedback_resistance)`. Preserve amplitude-independent timing and the existing slew expression.

### N4 — P2: Divide Wien frequency by the larger scale first

- **ID:** `oscillators.wien_bridge_balance`.
- **Anchor:** `backend/catalog/oscillators.json:397–401`.
- **Counterexample:** `bridge_resistance=1e-310`, `bridge_capacitance=1e308`, `gain_resistance=10000`, `feedback_resistance=20200`.
- **Evidence/impact:** `1/(2*pi)/bridge_resistance` overflows before division by C. RC≈0.01 s and frequency≈15.9154943091895 Hz are finite; remaining outputs are 1/3, 20000 ohm, 3.02, 3.02/3 and 1/150.
- **Minimal repair:** Reorder the two positive divisions using their numerical magnitudes: `1/(2*pi)/max(bridge_resistance,bridge_capacitance)/min(bridge_resistance,bridge_capacitance)`. This avoids both the exposed reciprocal overflow and an unconditional RC-product overflow. Preserve the signed resistor-difference excess calculation.

### N5 — P2: Apply the same balanced division to loaded-RC frequency

- **ID:** `oscillators.loaded_rc_phase_shift`.
- **Anchor:** `backend/catalog/oscillators.json:503–507`.
- **Counterexample:** `section_resistance=1e-310`, `section_capacitance=1e308`, `feedback_resistance=3e-309`.
- **Evidence/impact:** The first division in `1/(2*pi*sqrt(6))/R/C` overflows. The complete result is finite: frequency≈6.49747334361397 Hz; feedback_magnitude=1/29; critical_feedback_resistance≈2.9e-309 ohm; amplifier_gain_magnitude≈30; loop_gain≈30/29; loop_excess≈1/29. Decimal subnormal inputs imply small representation rounding, not non-finite outputs.
- **Minimal repair:** Divide the constant by `max(section_resistance,section_capacitance)` and then `min(...)`, as in N4. Do not change the loading polynomial, factor sqrt(6), gain 29 or signed excess.

### N6 — P2: Halve gm before the LC startup-ratio multiplication

- **ID:** `oscillators.cross_coupled_lc_startup`.
- **Anchor:** `backend/catalog/oscillators.json:630–634`.
- **Counterexample:** `tank_inductance=1`, `tank_capacitance=1`, `parallel_loss_resistance=1e308`, `transistor_transconductance=2`.
- **Evidence/impact:** The strict complex-pole constraint admits this case: `abs(1−1e-308)<2`. The ratio then computes `2*1e308/2`, overflowing before halving. Complete outputs remain finite: tank_frequency≈1/(2π), negative_resistance=−1 ohm, critical_transconductance≈2e-308 S, startup_ratio≈1e308, growth_rate≈0.5/s, startup_frequency≈sqrt(3)/(4π).
- **Minimal repair:** Use `(transistor_transconductance/2)*parallel_loss_resistance`. Retain the differential convention and signed growth.

### N7 — P2: Include ESR scaling before squaring the Pierce susceptance

- **ID:** `oscillators.pierce_crystal_startup`.
- **Anchor:** `backend/catalog/oscillators.json:761–771`.
- **Counterexample:** `crystal_frequency=1e-100`, `crystal_esr=1e200`, `shunt_capacitance=0`, `load_capacitance=1e-100`, `stray_capacitance=0`, `available_transconductance=1e-198`, `margin_target=1`.
- **Evidence/impact:** `[2π f(C0+CL)]²` underflows to zero before multiplication by `4 ESR`. Critical gm is actually `16π²*1e-200≈1.5791367041742973e-198 S`, representable. The subsequent gain-margin division by the incorrectly zero result fails. Expected load_capacitor_each≈2e-100 F, gain_margin≈100/(16π²), design_transconductance≈1.5791367041742973e-198 S, headroom≈−5.791367041742973e-199 S.
- **Minimal repair:** Balance ESR into the quantity being squared, e.g. `(4*pi*crystal_frequency*sqrt(crystal_esr)*(shunt_capacitance+load_capacitance))^2`, with multiplication order retaining that balance. This also avoids leading `4*ESR` overflow when the final gm is finite. Keep failed-margin headroom signed and zero C0/Cs admitted.

## Actionable findings — diagram source and observed pixels

All anchors below are in newly introduced `frontend/lib/diagrams/oscillators.dart`. Proposed coordinates are logical 700×280 coordinates. Main must recapture after repair; these are minimal source repairs, not claims that repaired pixels were observed.

### D1 — P2: Put the square-output voltage label at the comparator output

- **ID:** `oscillators.triangle_integrator_oscillator`.
- **Anchor:** `frontend/lib/diagrams/oscillators.dart:118`.
- **Observed evidence:** Native image places `±Vsq` at logical (20,69), beside the left RF/RT summing path. The actual square-output junction is (250,132), right of the comparator. No leader associates the label with that junction.
- **Impact:** The quantity is explicitly `square_peak`, not the voltage of the comparator's positive summing node. With nominal values, that summing node crosses zero and reaches about ±2.857 V after switching; it is not a ±5 V square output. This fails the requirement to mark the square output with its correct quantity.
- **Minimal repair:** Move `±Vsq` into the free space next to the comparator-output/RF junction, e.g. label near (260,85), and add a clearly non-electrical annotation leader to (250,132) if needed. Preserve `quantity: 'square_peak'`. Do not move the wiring or relabel the summing input as square output.

### D2 — P2: Clear both Wien labels from the drawn components

- **ID:** `oscillators.wien_bridge_balance`.
- **Anchors:** `frontend/lib/diagrams/oscillators.dart:142` and `:161–162`.
- **Observed evidence:** The native `C series` text touches/overlaps the upper capacitor plate region. Its anchor (315,16) puts the text above a capacitor centered (330,42), height 28, with plates beginning y=28. The end of `Positive feedback`, anchored (175,86), crosses the op-amp's upper-left outline near (280,92).
- **Impact:** These are visible symbol/text intersections, not a preference about whitespace. Component outline and explanatory label no longer remain distinct.
- **Minimal repair:** Move `C series` right to (355,16), clear of the vertical plates. Move `Positive feedback` above the arrow to approximately (155,56), leaving its bottom clear of the y=80 arrow. Keep bindings and topology unchanged.

### D3 — P2: Move the loaded-RC critical-resistance legend off the return wire

- **ID:** `oscillators.loaded_rc_phase_shift`.
- **Anchor:** `frontend/lib/diagrams/oscillators.dart:209`.
- **Observed evidence:** Native `Critical RF` text is visibly crossed by the vertical wire at logical x=60, y=96…176. Its anchor is (22,119), so the wire runs through the word `Critical`.
- **Impact:** The label is obscured and appears embedded in a circuit segment unrelated to the legend's output quantity.
- **Minimal repair:** Move that label to the free interior, e.g. (90,113), away from the x=60 return and the y=96 inverting-input wire. Preserve `critical_feedback_resistance` binding and the three-section termination.

### D4 — P2: Move the Pierce output-node label off its vertical wire

- **ID:** `oscillators.pierce_crystal_startup`.
- **Anchor:** `frontend/lib/diagrams/oscillators.dart:263`.
- **Observed evidence:** Native `OSC_OUT` is crossed by the vertical output/bias wire at logical x=365. Its text anchor is (340,90), inside that wire's y=45…120 span.
- **Impact:** Node identification is visibly struck through by a conductor.
- **Minimal repair:** Put `OSC_OUT` in the clear gap below the inverter output wire, e.g. (275,147), ending before x=365 and above the crystal path at y=180. Keep the output node and RF/crystal parallel wiring unchanged.

## Integration, handoff and closure

- All seven IDs are present in the exact-ID map; `catalog.dart` imports and spreads `oscillatorsDiagrams`; `main.dart:167` looks up by `c.id` and constructs DiagramPanel. The painter explicitly handles Wire, Label, Dot and Part. Every oscillator quantity key resolves to an input/output in the catalog, and the legend retains the same binding. No silent consumer drop was found.
- `backend/build.rs` includes catalog JSON files, Engine indexes IDs, constraints run before outputs, and calculated outputs become available to subsequent expressions. No new command/enum dispatch was introduced by this lane.
- Main and T2OscillatorsRun received numerical evidence directly. Tester is authoring regression vectors, including the additional Wien/phase cases. Addressed bus appendix: `local://oscillators-review-bus.md`; Main will append it to root `COMMUNICATION.md`.
- Closure requires Main-executed regression results for N1–N7, preservation of all complete nominal examples and ordinary boundary/recovery cases, corrected source for D1–D4, and fresh actual native captures for the four changed diagrams. Re-open those images before changing the gate to PASS.

## Honest untested scope

No builds, tests, lint, formatters, calculation processes, desktop/native launches, commits or production edits were run by this reviewer. No hardware validation, fresh primary-PDF retrieval, full-application narrow-window/large-text journey, screen-reader execution, Windows/macOS behavior or post-fix recapture was performed. Source analysis and seven actual captured canvases are the exercised review scope. Finite-input counterexamples are mathematical/IEEE evidence awaiting Main execution; they do not validate extreme values as realizable hardware.

Skipped: production edits and all execution, per assignment. Risk: HOLD remains until numerical regressions and material diagram findings close with fresh evidence.
