# V2MeasurementRun — source + native-pixel review — measurement lane

- **Snapshot:** 2cca03e854eb6842be56411a9153b6ca1b251a42 (integration HEAD, post oscillator/passives fixes; catalog byte-identical to 4c46fa1 for this lane, confirmed by T2MeasurementRunR2 and by example diff).
- **Reviewed:** `backend/catalog/measurement.json`, `frontend/lib/diagrams/measurement.dart`, `docs/features/measurement.md` against `research/expansion/measurement.md`, `research/expansion/measurement.json`, `coordination/expansion/BACKLOG.md` (items 095–100), engine support (`backend/src/lib.rs:12-14,104-117` input bounds, `backend/src/checked.rs:49` non-finite intermediate guard, `backend/src/numerics.rs:25` zero-safe sinc), registry wiring (`frontend/lib/diagrams/catalog.dart:33,67`), painter semantics (`frontend/lib/diagrams/painter.dart:39-63,81-153`: parts center-anchored, rotated `turns`·90° cw, voltageSource `+` on local left).
- **Images actually viewed (all six):**
  - `coordination/expansion/screenshots/measurement.dmm_accuracy_bound.png`
  - `coordination/expansion/screenshots/measurement.delta_resistance.png`
  - `coordination/expansion/screenshots/measurement.compensated_probe_loading.png`
  - `coordination/expansion/screenshots/measurement.rectangular_aperture_response.png`
  - `coordination/expansion/screenshots/measurement.sampling_jitter_snr.png`
  - `coordination/expansion/screenshots/measurement.correlated_voltage_ratio.png`

## Source verdict (per ID)

All six models are physically faithful to the research, units are consistent (V, ohm, s, Hz, F, rad, dB, 1), example maps in `research/expansion/measurement.json` and the catalog are numerically identical (verified value-by-value), and every nominal example reproduces by independent IEEE-754 arithmetic to ≤5e-16 relative. Distinct physical scope vs. excluded baselines (quantization/ENOB/alias/settling/RMS-sums/loaded divider) is preserved in assumptions. Quantity labels bind all 47 researched keys with correct meanings.

- `measurement.dmm_accuracy_bound` — PASS. Signed reading, % reading + % range arithmetic sum, `abs(reading) ≤ range_value` constraint, signed/zero endpoints handled; Keithley example reproduced exactly.
- `measurement.delta_resistance` — PASS algebraically (split weighted sums verified; reversal/offset invariances hold), with numerical finding MS-4.
- `measurement.compensated_probe_loading` — PASS. Catalog hypot form is algebraically identical to `Ri(1+jωtp)/(A−ω²C+jωB)`; matched reduction `Ri/√(A²+(ωRsτ)²)` reproduces all researched oracles including Rs=0 under/over/Cp=0 limits.
- `measurement.rectangular_aperture_response` — PASS algebraically; sinc(0)=1 exact, no dB clamp, null residual 3.9e-17; numerical finding MS-1.
- `measurement.sampling_jitter_snr` — PASS algebraically (log-sum SNR avoids underflow, 0.1-rad constraint present); numerical findings MS-2, MS-3.
- `measurement.correlated_voltage_ratio` — PASS. Stable hypot identity exact, genuine cancellation at ρ=1 with equal fractions, and the correlation domain is correctly declared (`"min": -1.0, "max": 1.0`, both inclusive; raw object re-inspected at `backend/catalog/measurement.json:462-468`) and enforced at `backend/src/lib.rs:108` — ρ>1 is rejected with the range message before any expression runs. An initial "missing max" candidate finding was withdrawn after re-inspection of the raw object (my earlier query projected only min/exclusive_min); no defect.

## Findings (all independently verified by exact double emulation mirroring the engine's left-associative stack evaluator; none of these rejects an out-of-domain input — every counterexample input is admitted by the catalog)

### MS-1 — P2 — `measurement.rectangular_aperture_response` — `backend/catalog/measurement.json:280`
`"expression": "sinc(pi*frequency*aperture_time)"` evaluates `(π·f)·T` left-to-right: f=1e308, T=0 (both admitted; T min is inclusive) gives π·1e308=inf and the evaluator (`backend/src/checked.rs:49`) aborts with "Model produced a non-finite intermediate value" before the ×0 — the zero-time instantaneous limit wrongly errors instead of returning {1, 1, 0.2, 0}. Agrees with T2MeasurementRunR2's MEASUREMENT-T1 (this reviewer's refinement: wrongful rejection, not NaN outputs).
```json
      "expression": "sinc(frequency*aperture_time*pi)"
```
Nominal 25·0.01=0.25 exact → identical π/4 value; whole-cycle residual unchanged (100·0.01 rounds to exactly 1.0).

### MS-2 — P2 — `measurement.sampling_jitter_snr` — `backend/catalog/measurement.json:368` and `:385`
Phase product `2*pi*frequency*timing_jitter` overflows before multiplying the subnormal: f=1e308, σt=1e-312 (true phase 6.283185307179586e-04 ≤ 0.1, in-domain) → `2π·1e308`=inf → the constraint at `:385` aborts evaluation; the request is wrongly rejected instead of returning phase 6.283e-04, error 4.443e-04, SNR 64.036 dB. Agrees with MEASUREMENT-T2. Apply the same reorder to both the constraint left side and the `phase_jitter` output:
```json
      "expression": "frequency*timing_jitter*2*pi"
```
```json
      "left": "frequency*timing_jitter*2*pi",
```

### MS-3 — P2 — `measurement.sampling_jitter_snr` — `backend/catalog/measurement.json:374`
`(input_peak/sqrt(2))*(2*pi*frequency*timing_jitter)`: f=1e-200, σt=1e-200, A=1e200 (all admitted) — the inner product underflows to 0.0 (finite, so no guard fires) and `error_rms` silently returns 0.0 V although the true value 4.442882938158366e-200 V is representable. Violates the "preserve representable tiny values" rule. Agrees with MEASUREMENT-T3. One ordering covers nominal, overflow and underflow regimes (verified: nominal 8.885765876316731e-05, ≤2 ulp from catalog; f=1e308/σt=1e-312/A=1e308 gives 4.442882938151547e+304 with no intermediate overflow):
```json
      "expression": "input_peak*timing_jitter*2*pi*frequency/sqrt(2)"
```

### MS-4 — P2 — `measurement.delta_resistance` — `backend/catalog/measurement.json:107` (constraint left side and `resistive_voltage` output expression, same line)
The split form `(voltage_first-voltage_second)/4+(voltage_third-voltage_second)/4` leaves −6.776263578034403e-21 V for the admitted pure-linear-EMF zero-resistance vector {9.765625e-05, 1.953125e-04, 2.9296875e-04} V (I=0.01 A, Δt=100 s), so the `>= 0` constraint wrongly rejects an explicit backlog boundary ("Pure linear EMF with zero resistance … within cancellation-aware absolute tolerance"; the backlog's own default-scale vector {12,14,16} µV happens to round to exact 0, masking this). The mean form is exactly 0.0 for both vectors, exactly 0.001 for the nominal example, and preserves the swap/additive invariants. Agrees with MEASUREMENT-T4. Change BOTH occurrences (constraint left and output expression) consistently:
```json
"left": "((voltage_first+voltage_third)/2-voltage_second)/2"
```
```json
"expression": "((voltage_first+voltage_third)/2-voltage_second)/2"
```
`midpoint_emf` needs no change (no constraint bounds it; verified 1.953125e-04 exact).

### MS-5 — P2 — `measurement.correlated_voltage_ratio` — `frontend/lib/diagrams/measurement.dart:210`
Pixel: the `Vb` quantity label at (257,72) (text band y≈72–88) is struck through by the dashed Ch A→ratio routing wire at y=79 (`Wire([220,135,240,135,240,79,560,79,560,105], …)`). Visible dash strikethrough on a quantity-acceptance label. Move it into the clear gap between the y=79 dash, the ρ dash (y=54), and the Vb source circle (top y=117; the y=105 wire starts at x=280, right of the text):
```dart
      Label(257, 93, 'Vb', quantity: 'denominator_voltage'),
```

### MS-6 — P3 — `measurement.rectangular_aperture_response` — `frontend/lib/diagrams/measurement.dart:147`
Pixel: the descending sine polyline ((230,157)→(250,177)→(270,184)) grazes the descender band of the `Delay` label (text top-left (228,163), 14 px). Word remains fully legible; cosmetic. Non-blocking.
```dart
      Label(170, 163, 'Delay', quantity: 'effective_delay'),
```
(170,163) is clear of the curve, the dashed average line (y=119), the midpoint drop (x=210, y 125–193) and the delay arrow (y=191).

## Pixel verdict (per ID)

- `measurement.dmm_accuracy_bound` — PASS. Meter polarity (+ upper, − lower), grounded common return, interval dot/ticks, center→upper total-bound bracket, two additive (not RSS) arrows, all nine quantity labels legible; interval marked "not to scale"; no clipping or overlap.
- `measurement.delta_resistance` — PASS. Kelvin force/sense separation with junction dots, EMF battery in the sense lead with + facing the nV meter (matches "E adds to V"), current source arrow drives +I, fixed sense polarity; timing strip +I/−I/+I markers bound to the three voltage readings, two equal Δt brackets, tilted dashed EMF drift line, algebra bound to `resistive_voltage`. No collisions.
- `measurement.compensated_probe_loading` — PASS. Cp bridges tip→scope node (dots at both junctions), not a ground shunt; Ri∥Ci are distinct branches to the return; common ground on the return; wires break at part bodies; output arrow (+ at node height, − at return) reads unambiguously as the pre-scaling output annotation; all nine bindings legible.
- `measurement.rectangular_aperture_response` — PASS with MS-6 (P3). Aperture box t−T…t, midpoint dot on the average line, delay arrow, T bracket, amplitude bracket, signed-sinc inset crossing zero at π with negative lobe, legend bindings for all four outputs; no live value bound to a static curve point.
- `measurement.sampling_jitter_snr` — PASS. Nominal sample dot plus two jittered instants, vertical projections, σt bracket labeled "RMS, not peak-to-peak", δv projection bracket, dv/dt tangent not equated to error_rms, amplitude bracket, right-hand callouts bound to `phase_jitter`, `error_rms`, `jitter_snr`. No collisions.
- `measurement.correlated_voltage_ratio` — MS-5 (P2) otherwise PASS. Two channels with +/− terminals, explicit shared return with ground, dashed (nonphysical) shared-reference influence branching into both paths and labeled "Error correlation ρ", ratio block `Va/Vb`, ua/ub brackets, u(r) → ×k → U=k·u(r) chain, `relative_standard_uncertainty` in the legend.

## Gate

**HOLD.** Blocking: MS-1, MS-2, MS-3, MS-4 (source), MS-5 (pixel). Non-blocking: MS-6. All six images present and inspected; no missing-image blocker. After Main lands MS-1..MS-4, rerun `cargo test --manifest-path backend/Cargo.toml --test measurement` and release the four intentional failing gates (T2MeasurementRunR2 MEASUREMENT-T1..T4); after MS-5 recapture `measurement.correlated_voltage_ratio.png` (MS-6 would ride along on an aperture recapture if Main takes it).

## Honest untested scope

- No builds, tests, lint, formatters, processes or native launches were run (contract). All numerics are independent IEEE-754 double emulations of the catalog expressions under the engine's left-associative stack evaluator semantics read from `backend/src/checked.rs`; no Rust binary was executed by this reviewer.
- Primary sources (Keithley, Tektronix, UCincinnati, NI, TI, NIST) were not re-fetched this pass; the research report's source readings and declared access gaps (MT-007, Keysight 403, embedded-image equations) are accepted as stated.
- Pixel review covers only the six recaptured captures at the listed paths, at the delivered resolution/theme; alternate DPR, themes, window sizes and live-GUI behavior were not observed.
- Diagram renderer behavior beyond the provided captures (e.g., text glyph metrics on other platforms) was reasoned from `painter.dart`, not observed.
- The four source repairs' full effect on the tester's ordered scenario file and the 228-example suite is Main's execution; this review predicts, not executes, their outcomes.
