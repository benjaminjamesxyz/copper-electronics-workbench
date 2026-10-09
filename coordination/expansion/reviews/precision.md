# V2Precision — source and pixel review of the precision lane at 2cca03e

Snapshot: **2cca03e854eb6842be56411a9153b6ca1b251a42** (clean tree, `git status` empty).
Scope reviewed: `backend/catalog/precision.json`, `frontend/lib/diagrams/precision.dart`, `docs/features/precision.md`, against `research/expansion/precision.md`, `research/expansion/precision.json`, backlog items 065–070, `frontend/lib/diagrams/primitives.dart`, `frontend/lib/diagrams/painter.dart`, `backend/src/lib.rs` (engine function registry), and the shared capture harness `frontend/integration_test/expansion_acceptance.dart`.
Owned file: this report only. Production untouched; nothing staged; no builds, tests, lint, formatters, processes or native launches were run. Independent arithmetic used Python binary64 and exact fractions.

## Verification method

- **Source:** `backend/catalog/precision.json` is byte-identical to `research/expansion/precision.json` after JSON canonicalization (verified by sorted-key diff: no differences). `docs/features/precision.md` reproduces the research lane's laws, limits, examples and mandated diagram specs verbatim for all six IDs. Engine coverage: all catalog functions (`sqrt`, `ln`, `log10`, `abs`, `pi`, `log1p`) exist in the `meval` context or are registered in `backend/src/lib.rs:67-73`; input range, finiteness and constraint handling read at `backend/src/lib.rs:101-117`.
- **Numerics:** every output of all six catalog examples was recomputed independently (binary64, plus `fractions.Fraction` for the Howland node solve). All 32 expected values reproduce exactly to full printed precision; worst deviation seen anywhere was 1.1e-14 relative (Howland `output_conductance`, inside the 1e-6 tolerance). Key identities re-derived, not just re-evaluated: mirror KCL (`Iref = Ic(1+2/beta)`); INA three-op-amp node equations; TIA integrator-model denominator `1+s(1/wt+RfCf)+s^2*Rf(Cs+Cf)/wt` with the Butterworth condition `b1^2=2*b2` reproducing `Cf=sqrt(2*wt*Rf*Cs-1)/(wt*Rf)` (numeric check: `b1^2-2*b2 = -1.0e-28`, i.e. exact to roundoff); Howland exact `Vx=100/201`, `Vo=199/201`; Brokaw `2*I*R1 = 0.600000` V and zero local slope by construction; LOG114 endpoints -1.5/+1.5 V via `log1p` (-1.500000000000018, relative 1.2e-14).
- **Device claims:** LOG114 facts cross-checked against the TI SBOS301 datasheet (0.375 V/decade trim, 8-decade 100 pA-10 mA input, 2.5 V reference, single +5 V or bipolar +-5 V operation) via the published datasheet text; no contradiction with the catalog's nominal model or admitted ranges.
- **Pixels:** all six PNGs were opened as images (paths listed per ID below). Geometry was additionally measured programmatically (PIL/numpy) on `rf.first_fresnel_clearance.png` only, for the cross-lane RF analysis in the last section.

## Per-ID findings

### 1. `precision.bjt_mirror_base_current` — image `coordination/expansion/screenshots/precision.bjt_mirror_base_current.png`

- **Source verdict: PASS.** Formulas match ADI ch. 11.5 (`Ic=Iref/(1+2/beta)`, `Ib=Iref/(beta+2)`); all five example values exact; compliance domain `collector_voltage >= base_emitter_voltage` with inclusive boundary as researched; inputs exclusively positive as researched.
- **Pixel verdict: PASS with one P3.** Topology correct in pixels: Q1 collector-base short explicit, joined base net with dots at the base and emitter joins, un-joined collectors, downward Iref and Ic arrows, `Ib each` arrow points into Q2's base (correct reference direction), ground on the emitter rail, dashed collector-to-base bracket present. No clipping or text collisions.
- **PRECISION-D1 / P3 / `frontend/lib/diagrams/precision.dart:29-30` (pixel-verified):** the `compliance_margin` dashed bracket (x~502, y 95-150) has the `fractional_deficit` label `Deficit / Iref` (510,154) sitting between the bracket and its own label `Margin = Vc - VBE` (510,198); nearest-label association reads the bracket as the deficit. Minimal repair: swap the two rows -
```suggestion
      Label(510, 154, 'Margin = Vc − VBE', quantity: 'compliance_margin'),
      Label(510, 198, 'Deficit / Iref', quantity: 'fractional_deficit'),
```

### 2. `precision.instrumentation_internal_headroom` — image `coordination/expansion/screenshots/precision.instrumentation_internal_headroom.png`

- **Source verdict: PASS.** All six outputs exact (`0.9199999999999999` included). Constraint set is complete per research: interval ordering x2, both first-stage outputs and final output vs output range x6, both signal pins and the final difference-stage pin vs common-mode range x6. The assumption retaining the Va/Vb branch-naming caveat is present.
- **Pixel verdict: PASS with one P3.** Topology correct in pixels: A1/A2 plus-inputs driven, Rf from each output to its minus pin, single Rg between minus pins, dots at every 3-way junction; Va (upper) reaches the final minus pin and Vb (lower) the final plus pin; all four final-stage resistors labeled with matched values; lower Rd terminates in the open Vref port, not ground; undotted crossings (at logical (450,150), (470,110), (420,150)) correctly render as non-connections. No clipping or collisions.
- **PRECISION-D2 / P3 / `frontend/lib/diagrams/precision.dart:44-46` (source + pixel):** the research/docs mandate that `common_voltage`/`differential_voltage` bind **only** in the explanatory bracket "Inputs = Vcm ± Vd/2", never to individual input-wire labels. The implementation keeps the bracket static and instead binds standalone `Vcm` (20,82) and `Vd` (20,110) labels floating between the two input wires; the `Vcm` label sits 27 px below the top wire and can be misread as that wire's name (top input is `Vcm+Vd/2`, not `Vcm`). `Label` carries a single `quantity`, so the minimal repair keeps two labels but relocates them beside the bracket text where they read as its legend (bracket text ends near x~153):
```suggestion
      Label(20, 18, 'Inputs = Vcm ± Vd/2'),
      Label(170, 18, 'Vcm', quantity: 'common_voltage'),
      Label(215, 18, 'Vd', quantity: 'differential_voltage'),
```

### 3. `precision.transimpedance_butterworth` — image `coordination/expansion/screenshots/precision.transimpedance_butterworth.png`

- **Source verdict: PASS.** The retained finite-`Cf` Butterworth solution is correct (identity verified numerically to -1e-28); solvability constraint `4*pi*GBW*Rf*Cs >= 1` is exactly `2*wt*Rf*Cs >= 1`; swing constraint present; `input_peak` admits zero as researched; signed `dc_transimpedance` retained. Boundary behavior note for the tester (not a catalog defect): at GBW = `1/(4*pi*Rf*Cs)` exactly, binary64 evaluation of the constraint passes while `sqrt` yields `Cf ~ 2.98e-18` F rather than exact 0, and `butterworth_bandwidth` evaluates to `11253.953784266849` Hz versus the exact-math `11253.953951963826` (relative 1.5e-8, from the 3e-8 relative perturbation of `Cs+Cf`); assertions on this boundary need a small relative tolerance on the bandwidth or an absolute tolerance justified by the roundoff, and `Cf=0` must be asserted with an absolute floor.
- **Pixel verdict: PASS, clean.** Plus input grounded; current source at lower left with its internal arrow pointing up into the summing node (matches the stated positive-`input_peak` convention); Cs shunt to ground with junction dots at (225,180)/(270,180); Rf and Cf are two separate horizontal branches sharing exactly the same two vertical risers (dots at (270,95), (530,95), (530,160)); no wire crosses any component body; no photodiode/optical symbolism; `dc_transimpedance` appears only in the static `Vo/Iin -> -Rf at DC` annotation, not on a node. All labels legible, nothing clipped.

### 4. `precision.howland_mismatch_load` — image `coordination/expansion/screenshots/precision.howland_mismatch_load.png`

- **Source verdict: PASS.** Exact-fraction verification of the example (`Vx=100/201`, `Vo=199/201`); `amplifier_current` decomposition matches the research narrative (49.2537...uA + 49.7512...uA); constraint set covers ordering, positive total conductance, load-node common mode, amplifier swing and `abs(...)` current bound. Negative `g` correctly admitted (only the loaded total conductance is constrained).
- **Pixel verdict: PASS, clean.** Netlist exact: R1 from `Vp` port to Vx, R2 from output back to Vx (junction dot at (290,120)), R3 from `Vn` port to minus pin, R4 as the separate upper negative-feedback branch (dot at (330,160)), grounded load on Vx with downward `IL` arrow, `Io` arrow on the output wire before the R2/R4 split dot at (570,140), `Norton g (equivalent)` as a side annotation with no phantom resistor. Undotted crossings at (330,120), (290,160), (240,160) correctly non-connecting. No collisions or clipping.

### 5. `precision.brokaw_first_order_trim` — image `coordination/expansion/screenshots/precision.brokaw_first_order_trim.png`

- **Source verdict: PASS.** All six outputs exact; `2*I*R1 = 0.6` V identity and zero local derivative hold by construction; `area_ratio` exclusive minimum 1 rejects N=1 (no invented larger minimum); slope constraint `< 0` present; SI constants exact.
- **Pixel verdict: PASS with one P3.** Topology correct in pixels, including the three details the research flagged: (a) the servo output routes around the top to Q1's base and the shared-base net taps that wire at a dot (logical (120,150)) and continues to Q2's base - electrically one Vref net, verified as an unbroken line in pixels (the apparent 20 px "gap" at Q1's base pin is covered by the servo wire's final segment (24,150)->(140,150) passing through the dot); (b) plus input senses Q1's collector, minus senses Q2's collector (dots at (170,100)/(330,110)); (c) no collector-base diode connection; R2 between Q2's emitter and node E; R1 from E to ground with static `2I`; equal-Rc supply rail labels static. Loop sign is negative (base rise lowers both collector voltages), consistent with the researched servo.
- **PRECISION-D3 / P3 / `frontend/lib/diagrams/precision.dart:186-188` (pixel-verified placement):** `I each` (branch_current, at 40,91) and `VBE1` (base_emitter_voltage, at 39,124) are bound to quantities but float at the left margin ~130 px from their referents, immediately beside the left-hand Vref base-rail wire (x=24) - the nearest wire to both labels carries base/servo current, not `branch_current`, and neither labels a VBE bracket as mandated. Minimal repair: move the two labels to their referents -
```suggestion
      Label(100, 91, 'I each', quantity: 'branch_current'),
      Label(196, 163, 'VBE1', quantity: 'base_emitter_voltage'),
```
  (left of Q1's collector arrow and under Q1's base lead respectively; both spots are empty paper in the current capture).

### 6. `precision.log114_current_ratio` — image `coordination/expansion/screenshots/precision.log114_current_ratio.png`

- **Source verdict: PASS.** `log1p((I1-I2)/I2)/ln(10)` formulation verified: exact 0 at equal currents, exact 2.0 decades at the example, endpoint outputs -1.5/+1.5 V within 1.2e-14 relative; ranges 100 pA-10 mA and 1250 ohm-25 Gohm match the researched device limits (datasheet cross-checked); empty `constraints` array is correct because both ranges are enforced as input min/max (inclusive endpoints admitted as researched).
- **Pixel verdict: PASS, clean.** LOG114 functional block with I1/I2 inputs (arrows entering, both with "virtual ground" labels), external Rref routed from the VREF pin over the top to I2 (not from ground), I2 arrow direction into the pin, VLOGOUT output arrow with `log_output` bound to `Vo`, static `2.5 V nominal` and `+-5 V` labels, COM/VCM IN/REF GND each visibly grounded, `current_ratio` and `local_transimpedance` as side annotations (ratio not drawn as a voltage), no optical symbolism. The Rref return riser crosses the I1 wire at logical (180,110) without a dot - correct non-connection. No collisions or clipping.

## Cross-lane analysis assigned to this lane: RF Fresnel dark blocks (RF-FR-1)

Observed in `coordination/expansion/screenshots/rf.first_fresnel_clearance.png`: two solid slabs of RGB(63,73,69) covering logical x 54-128 and 570-648, y 44-176 - exactly the ellipse bounding box height at both high-curvature ends of the Fresnel oval, hiding the TX/RX dots and both ellipse tips.

New evidence beyond the V2Rf report:

1. **The capture is not stale.** `/tmp/copper-expansion-acceptance.json` (mtime 2026-10-10 00:02, same minute as all PNG mtimes) records the harness run that produced these captures: all 100 IDs visited/computed against the 2cca03e catalog with **zero** layout issues. The harness (`expansion_acceptance.dart:82`) captures via `RepaintBoundary.toImage` from the freshly compiled workbench. `rf.dart`'s Fresnel marks are unchanged since their introducing commit (`7dec94c`) and `painter.dart` since `29f9c2b` (22:08, before the 23:42 integration HEAD). A clean recapture at 2cca03e will therefore reproduce the slabs; recapture alone is not the fix.
2. **The slabs carry the oval's own stroke ink.** RGB(63,73,69) equals the `muted`/`onSurfaceVariant` stroke color of `Part(CircuitPart.circle, 350,110, width:590, height:130, ink: muted)` (`rf.dart:118` -> `painter.dart:152-153` `canvas.drawOval`). The repository's painter fills nothing except 3.2 px Dots and tiny switch-contact circles - no mark can paint a 74x132 filled slab; nor can any stroke setting (1.8 px) produce one.
3. **The trigger is extreme aspect ratio.** Slab geometry (solid rectangle spanning the full bbox height at the tip columns, thin correct arc everywhere else) matches a broken stroked-conic rasterization where the tip's radius of curvature `b^2/a = 65^2/295 ~ 14.3` px is far smaller than the artifact width. Every other captured oval is near-square and renders correctly - e.g. the 64x94 and 40x66 ovals in `protection.copper_adiabatic_fault_limit.png` are clean thin strokes; the Fresnel oval (590x130, 4.5:1) is the only extreme case in all 100 captures.
4. **Minimal repairs (owner: Main with T2RfRun/V2Rf; rf.dart is not a precision-lane file).** Either
   - in `painter.dart`, for `CircuitPart.circle` whose `width/max(height,1)` ratio exceeds ~2.5 (or min dimension < ~24), emit a stroked polyline path of ~64 segments instead of `drawOval` - single-point, protects every future diagram; or
   - in `rf.dart:118` only, replace the oval Part with an explicit dense `Wire` polyline of the same ellipse (muted ink, optionally dashed to match the "illustrative" caption).
   Both preserve geometry exactly; neither touches any computed value. After either fix, recapture `rf.first_fresnel_clearance.png` and re-verify the tips, TX/RX dots and the arc are the only ink near the ends.

## Gate

**Verdict: HOLD** - but only because of the shared gate rule that any material finding blocks PASS across reviewed scope, and the RF material defect (RF-FR-1) was explicitly assigned to this pass for analysis and remains open.

- **Precision lane source: PASS.** No physics, unit, sign, domain, constraint-completeness or example deviation found; catalog byte-identical to research; docs faithful; engine support confirmed.
- **Precision lane pixels: PASS** - all six images viewed; topology, polarity, reference directions, quantity bindings and readability verified; three P3 findings (PRECISION-D1..D3) are label-placement polish, none material, none blocking on their own.
- **Blocking item (not precision-lane code):** RF-FR-1 Fresnel slabs - now root-caused above as a live rendering defect of the extreme flat oval; needs the painter/rf.dart mitigation plus recapture. This lane's report raises the earlier "corrupted/stale capture" hypothesis as contradicted by evidence.

## Untested scope (honest)

- No code was executed: no engine run, no `cargo test`, no Flutter build, no recapture. Numerical "verification" is independent recomputation of catalog expressions, not execution of the Rust evaluator; the capture-time acceptance run (228 examples green, zero layout issues) is Main-executed evidence I inspected, not something I ran.
- Constraint/rejection behavior (e.g. reversed intervals, N=1, GBW below solvability, compliance violations) was verified by reading the constraint expressions and input ranges, not by probing the engine.
- Font rendering at other window scales/DPI and the legend rendering outside the RepaintBoundary were not observable from the captured canvas.
- LOG114's +-2 mA reference-load condition and 10 kohm/100 pF VLOGOUT-load condition were taken from the research lane's datasheet reading (SBOS301 sections 5.5/7.x); I verified the transfer, ranges, reference voltage and supply options against the published datasheet text but did not obtain the PDF pages for those two secondary conditions.
