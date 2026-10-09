# V2ControlRun — source and native-pixel review, `control` lane

- Snapshot reviewed: **2cca03e** (integration HEAD after oscillator/passives conditioning, passives endpoint and eight label-collision fixes; PNGs recaptured post-fix).
- Date: 2026-10-10. Reviewer: V2ControlRun (independent, read-only; no builds/tests/lint/formatters/process/native launches performed).
- Scope: the 6 required control IDs. Production inputs read in full: `backend/catalog/control.json`, `frontend/lib/diagrams/control.dart`, `docs/features/control.md`, cross-checked against `research/expansion/control.md` and `research/expansion/control.json`, the backlog lane entries (BACKLOG.md items 047–052), engine evaluation path `backend/src/lib.rs` + `backend/src/checked.rs`, and diagram registry wiring in `frontend/lib/diagrams/catalog.dart:59`.
- Numerical method: every catalog example and every model identity re-derived by independent scalar arithmetic on paper from the stated transfer functions (detail per ID below). No engine was executed; all runtime claims are source-derived predictions and are labeled as such.
- Pixel method: every PNG opened as an image and inspected. No assessment made from filenames or source alone.

## Images actually viewed (all six)

1. `coordination/expansion/screenshots/control.delay_phase_budget.png`
2. `coordination/expansion/screenshots/control.loop_sensitivity.png`
3. `coordination/expansion/screenshots/control.lead_crossover_synthesis.png`
4. `coordination/expansion/screenshots/control.pi_pole_placement.png`
5. `coordination/expansion/screenshots/control.peak_current_slope_compensation.png`
6. `coordination/expansion/screenshots/control.integral_two_lag_stability.png`

All files existed; none missing. Capture style is consistent with the closed lanes' accepted native captures.

## Per-ID review

### 1. `control.delay_phase_budget` (backlog 047)

Source: **PASS.** Independent recomputation of the example: `360·1000·2e-5 = 7.2`; `60−7.2 = 52.8`; `(60−45)/360/1000 = 4.1666666666666665e-5`; headroom `2.1666666666666664e-5` — all four outputs match exactly. Units correct (Hz·s·360 → degrees); delay lag magnitude-only so crossover immobility assumption is stated. Bounds: `initial_phase_margin ∈ (0,180]`, target `[0,180]`, constraint `target ≤ initial` present and enforced pre-output by the engine (`lib.rs` constraint loop precedes output evaluation). Negative remaining margin/headroom correctly remain computable (no clamp, no rejection) per the documented failed-design semantics. Distinct from timing/signals/opamps exclusions; novelty holds.

Pixel: **PASS.** Image 1: negative-feedback topology correct — `+` upper-left / `−` bottom of summer, forward chain `L0(s) → exp(−s td) → y`, unity pickoff **after** the delay block (dot at x≈600, block ends x≈520), feedback routed below with arrow into the minus input. `td` bound label sits outside/below the delay block. Strip shows three disjoint brackets (Original PM spanning full width; Lag left; Remaining PM right), dashed target tick with its own label, and a separate `Delay budget` label binding `maximum_delay` — no phase bracket is bound to a time quantity, as required. No collisions, no clipping; the illustrative lag/remaining proportions are covered by the caption's "illustrative" clause.

### 2. `control.loop_sensitivity` (backlog 048)

Source: **PASS.** Recomputation: `D=hypot(0.5,0.5)=0.7071067811865476`; `|S|=√2`; `|T|=1`; `y_d=0.014142135623730952`; `y_n=0.001` — all five match. Convention correct: L entered without the summing-junction sign; singular point `L=−1` excluded by constraint `hypot(1+loop_real,loop_imaginary) > 0`, which the engine evaluates **before** any output division, so `1/0` is unreachable (`lib.rs:117-121`). Negative real/imag parts admitted (no min on either); amplitudes `min 0` inclusive; nonfinite inputs rejected. Disturbance-at-output vs noise-in-measurement distinction is explicit in assumptions. `hypot` avoids squaring overflow. Novel vs `opamps.*`/`noise.*` exclusions.

Pixel: **PASS.** Image 2: actual injection topology drawn — `d` enters the **second** summer from the top; `n` enters a **third** summer placed on the feedback path (not the output node), which returns to the first summer's minus input; arrows on the right-to-left feedback segment point left, correct for signal direction. Bottom legend binds `d → y: |S|`, `n → y: |T|`, `|yd|`, `|yn|` with no addition sign between the two contributions. All signs legible; no overlap or clipping.

### 3. `control.lead_crossover_synthesis` (backlog 049)

Source: **PASS.** Recomputation with φ=30°: `α=(cos30/(1+sin30))² = 1/3` exactly (cosine form, cancellation-free near 90° as research demands); `fz=1000/√3=577.3502691896258`; `fp=1732.0508075688772`; `Kc=√α/M=1.1547005383792517`; `Kc/α=3.4641016151377544`; `τz=2.7566444771089603e-4` — all six match. Normalization identity `M·Kc/√α = 1` verified. Exclusive bounds `0<φ<90`, `fc>0`, `M>0` keep every denominator strictly positive (no division-by-zero path exists). Boost explicitly not equated to phase margin. Distinct from passive filter synthesis.

Pixel: **PASS.** Image 3: two aligned schematic panels; magnitude low plateau bound `Kc`, high plateau bound `Kc/α`, rise starting at the `fz` dashed line and ending at the `fp` dashed line; phase hump peaks at the center dashed line with `φmax` bound adjacent; `fc` bound at the center marker; geometric-mean alignment of the three markers is visually correct (peak at center). Static transfer `C(s)=Kc(1+sτ)/(1+sατ)` top-left and `M |C(jωc)| = 1` note with `M` bound. Curves read as schematic, caption disclaims computed Bode plots. No collisions/clipping.

### 4. `control.pi_pole_placement` (backlog 050)

Source: **PASS.** Recomputation: `2ζωnτ=3 → Kp=(3−1)/2=1`; `Ki=τωn²/K=200`; reconstructed denominator `0.01s²+3s+400 ∝ s²+300s+40000`, i.e. `ωn=200`, `ζ=0.75` — coefficient matching exact; `Ti=0.005`; `fz=Ki/(2πKp)=31.830988618379067` — match. Units: dimensionless K,Kp; Ki in 1/s; ωn in rad/s (correctly not Hz). Constraint `2ζωnτ > 1` strictly enforced pre-output, so `Kp>0` and `Ti`, `fz` divisions are unreachable-zero. All ζ>0 admitted (no `√(1−ζ²)` anywhere); numerator-zero caveat stated. Distinct from RLC transient and op-amp integrator models.

Pixel: **PASS.** Image 4: error node explicitly splits (dot) into parallel `Kp` and `Ki/s` branches; both merge with `+` signs into the second summer; plant `K/(1+sτ)` driven to output `y`; unity feedback picked off after the plant returns to the minus input. Bindings: `Kp`, `Ki`, `K`, `τ`, `ωn`, `ζ`, `PI zero` (`zero_frequency`) all present and none bound to a wrong node. Target-polynomial match text static and clear. Minor cosmetic inconsistency: the plant transfer text renders inside its block while `Kp`/`Ki/s` texts render outside theirs — no collision, no ambiguity; not raised as a defect.

### 5. `control.peak_current_slope_compensation` (backlog 051) — **material source finding C1**

Source: **HOLD (C1).** Model derivation independently re-derived from ramp geometry and confirmed: peak-time displacement `Δt=−δ/(m1+ma)` gives `δnext/δ = (ma−m2)/(m1+ma)`; boundary `ma > (m2−m1)/2` with the documented `max(0,·)` nonnegative form; deadbeat at `ma=m2`; `D=m2/(m1+m2)` from `m1D=m2(1−D)`. Example values (λ=−1/3, δnext=−0.0033333333333333335, boundary 100000, swing 2 V, reserve 2/3, duty 0.75) all match. Physics, sign conventions (positive falling magnitude, same sense node, ramp added to sensed current) and the marginal-vs-strict boundary distinction are correct.

**C1 (P1):** the shipped multiplier expression divides by an input that legitimately takes 0, so the documented no-ramp analysis is unreachable.
- Anchor: `backend/catalog/control.json:493` — `"expression": "(ramp_slope-falling_slope)/ramp_slope/(1+rising_slope/ramp_slope)"`.
- Admission: `backend/catalog/control.json:467-471` — `ramp_slope` has inclusive `min: 0` (no `exclusive_min`).
- Guard: `backend/src/checked.rs:49` — the evaluator rejects any non-finite intermediate with `"Model produced a non-finite intermediate value; check its assumptions"`.
- Counterexample (source-derived, not executed): input `{rising_slope:100000, falling_slope:300000, ramp_slope:0, switching_period:1e-5, initial_perturbation:0.01}`. Step 1 `0−300000 = −300000` (finite, pushed); step 2 `−300000/0 = −∞` → evaluator errors; **all six outputs are lost**, not just λ. Research mandates exactly this case: `research/expansion/control.md` §5 test list — "No ramp with default slopes: λ `-3`, next perturbation `-0.03 V`, … calculation **must report instability rather than reject**." A second mandated vector breaks the same way: `{rising:300000, falling:100000, ramp:0}` requires λ=−1/3, but computes `−100000/0 → −∞` → error. Both are the canonical uncompensated and already-stable boundary cases the expert-level model exists to show; `sample_multiplier` is the first output, so `next_perturbation` and `stability_reserve` die with it.
- Provenance: research defined the direct form `(ramp_slope-falling_slope)/(rising_slope+ramp_slope)` (`research/expansion/control.json:183`); the catalog diverged to a ramp_slope-factorized rewrite whose only effect is a division by an admitted zero. The research form's denominator `rising+ramp > 0` is unconditionally safe (`rising_slope` has `exclusive_min: 0`).
- Minimal repair (no behavior change for ramp>0, restores all documented zero-ramp vectors):
  `backend/catalog/control.json:493` →
  ```json
        "expression": "(ramp_slope-falling_slope)/(rising_slope+ramp_slope)"
  ```
  No clamp, no new bound, no engine change.

Pixel: **PASS.** Image 5: sense-voltage axes with two period ticks and `Ts` bracket on the first period; solid nominal triangle pair and dashed perturbed trajectory with alternating cycle-start separation (dashed above solid at the first boundary — positive δ; below at the second — negative δnext); dashed peaks shift earlier/higher on the positive cycle and later/lower on the negative one, consistent with the linearized map. δ labeled at the first boundary, `δnext` **on** the second cycle boundary as specified. Right panel: resetting sawtooth with `ma` and full-period swing `ma·Ts` bound, `Add to sense` arrow pointing at the waveform panel, `Reset each Ts`. `δnext = λ δ` with λ bound. No collisions or clipping.

### 6. `control.integral_two_lag_stability` (backlog 052)

Source: **PASS.** Recomputation: clearing `1+CPH` gives `tpts·s³+(tp+ts)s²+s+K·Ki`; cubic Routh reduces to `(tp+ts) > tpts·K·Ki` ⇒ `Ki,crit=(1/tp+1/ts)/K = 750`; factor `7.5`; reserve `13/15 = 0.8666666666666667`; boundary mode from `(tpts·s²+1)(s+(tp+ts)/(tpts))` ⇒ `ω=1/√(tp·ts)`, `f=112.53953951963827` — all four match; Routh first column independently reproduced (0.000002, 0.003, 13/15, 200). Marginal (not strict) at equality stated; negative reserve admitted; all four inputs `exclusive_min: 0` so every division/sqrt stays positive; reciprocal square roots evaluated separately per the conditioning note. Physical scope (gain bound, not crossover) distinct from the PI item.

Pixel: **PASS.** Image 6: forward path `Ki/s → K/(1+s tp) → y` with pickoff **after** the plant; sensor block `1/(1+s ts)` in the return path with the entering arrow pointing **left** into its right port and its output routed right-to-left back up into the minus input — ports and arrows agree with the stated flow; `Sensor return ←` aids direction. Bindings `Ki`, `K`, `tp`, `ts` correct (sensor pole not placed in the forward path). `Strictly stable: Ki < Ki,crit` with `Ki,crit` bound, and `Boundary mode` bound to `critical_frequency`, not called a crossover. No collisions/clipping.

## Additional dispositions (contract-required reporting, no silent normalization)

- **C2 (P3, informational):** catalog expressions for four outputs are associativity-conditioned rewrites of the research forms with identical algebra and ≤1–2 ulp divergence, far inside the 1e-6 nominal tolerance: `control.delay_phase_budget` `backend/catalog/control.json:61` (`360*(fc*td)` vs `360*fc*td`); `control.pi_pole_placement` `backend/catalog/control.json:400` (`Ki/(Kp*2*pi)` vs `Ki/Kp/(2*pi)`); `control.integral_two_lag_stability` `backend/catalog/control.json:606-612` (factored `(1/tp)/K*(1+(1/ts)/(1/tp))` vs `(1/tp+1/ts)/K`) and `backend/catalog/control.json:627` (single-denominator vs sequential reciprocal roots). All remain finite on every admitted domain (checked per-denominator); no repair required. Reported because the contract requires source/data discrepancies to be surfaced, not normalized silently.
- **C3 (P3, informational):** example literals differ in notation only (`0.00002` vs `2e-05`, `0.000041666666666666665` vs `4.1666666666666665e-05`) between `research/expansion/control.json` and the catalog; both parse to the identical f64. No action.
- Docs (`docs/features/control.md`) mirror research equations, bounds, captions and limits faithfully for all six IDs; backlog anchors 047–052 match catalog example values exactly. Registry wiring `...controlDiagrams` present (`frontend/lib/diagrams/catalog.dart:59`). No doc drift found.
- RF lane note (not my scope, not duplicated): the Fresnel ellipse-end dark-block pixel defect is acknowledged as open per the resumption brief; no control-lane bearing.

## Verdict

- **Source: HOLD** — one material defect, C1 (P1), blocks the documented zero-ramp behavior of `control.peak_current_slope_compensation`.
- **Pixels: PASS** — all six native captures viewed as images; correct topology, pins, polarity, reference directions, quantity bindings; no collision or clipping found.
- **Overall: HOLD** until Main applies the C1 one-line expression repair (nominal example unaffected), reruns the peak-current examples plus the two zero-ramp scenarios above, and recaptures nothing (diagram untouched — no image change needed).

## Honest untested scope

- No engine, test, process, or UI process was executed by this reviewer (worker contract). C1 is a source-derived IEEE-754 prediction (`−300000/0 → −∞` under the checked evaluator); Main must reproduce it by running the vector, as with all numerical findings.
- Example verification is independent paper arithmetic at the nominal vectors; non-nominal research test cases were checked algebraically, not numerically exhaustively.
- Pixel review is of static composited captures; live app interaction (zoom, resize, font scaling, theme) and dynamic label-value rendering were not observable in these images.
- Extreme denormal inputs (e.g. both time constants ≈5e-324 in the two-lag model) drive `critical_frequency`'s denominator subnormal and will error via the same non-finite guard; this matches the research's explicitly accepted "extreme finite floating-point inputs beyond practical hardware scales" risk and is not raised as a defect.
- Global stability certification, hardware applicability and multi-crossover loops remain outside every model by documented assumption.
