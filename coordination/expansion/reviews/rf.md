# V2Rf source and native-pixel review — RF expansion lane

- Reviewer: V2Rf (independent assurance, resumed lane).
- Snapshot reviewed: **2cca03e** (`2cca03e854eb6842be56411a9153b6ca1b251a42`), production tree clean.
- Scope: `backend/catalog/rf.json`, `frontend/lib/diagrams/rf.dart`, `docs/features/rf.md` against `research/expansion/rf.md`, `research/expansion/rf.json`, `coordination/expansion/BACKLOG.md` sections 022-028, `frontend/lib/diagrams/primitives.dart`, `frontend/lib/diagrams/painter.dart`, `frontend/lib/diagrams/panel.dart`.
- Closed lanes (oscillators, passives) not duplicated.
- This reviewer ran no builds, tests, lint, formatters, processes or native launches. All numerics below are independent Python arithmetic; all pixel observations are from the listed PNGs read as images.

## Images actually viewed (pixel gate)

Every assigned native PNG was opened and inspected as an image (plus zoomed crops and per-pixel sampling for the two defect findings):

1. `coordination/expansion/screenshots/rf.matched_t_attenuator.png`
2. `coordination/expansion/screenshots/rf.lowpass_l_match.png` (zoom crop of the inductor label region inspected)
3. `coordination/expansion/screenshots/rf.free_space_link_budget.png`
4. `coordination/expansion/screenshots/rf.first_fresnel_clearance.png` (crops and per-pixel sampling of both dark blocks)
5. `coordination/expansion/screenshots/rf.circular_aperture_gain.png`
6. `coordination/expansion/screenshots/rf.two_tone_im3.png`
7. `coordination/expansion/screenshots/rf.monostatic_radar_return.png`

All seven files exist, are 1400x560, and their SHA-256 values match `coordination/expansion/native_images.json` (spot-verified `rf.first_fresnel_clearance.png` `6e751936...`, `rf.two_tone_im3.png` `348c0f38...`).

## Source verdict (equations, units, domains, docs)

**Correct and internally consistent, with four confirmed conditioning defects (F2-F5, independently reconfirmed from the T2RfRunR2 receipt).**

- All seven catalog examples reproduce independently to <=1e-14 relative: T-pad 40.90909090909091 / 10.1010101010101 / 0.01 ohm set; L-match sqrt(3), 1.3783222385544804e-7 H, 1.3783222385544801e-11 F with the independent circuit identity Zin = 50 + j0 ohm verified from reactances +j86.60254037844386 / -j115.47005383792516; FSPL 100.0520080561155 dB; Fresnel 5.475330656681841 m radius; aperture 0.47123889803846897 m^2, 6588.849534059469, 38.188095899160565 dBi; IM3 product -60 dBm, total 3.010299956639812 dBm (linear check 1e6); radar -96.43988186210434 dBm via both the dB identity and the linear radar equation (2.2699265981678502e-13 W).
- `research/expansion/rf.json` and `backend/catalog/rf.json` are field-identical for all 7 models (inputs, bounds, expressions, examples). All backlog numerical anchors match.
- Units, signs and reference conventions correct: voltage-attenuation K with `expm1` conditioning (A=1e-12 dB limits stay finite and correct); low-pass orientation (series L, load-side shunt C, `Rl > Rs` constraint); log-sum FSPL with far-field screen `max(10c/f, 2D^2 f/c)`; overflow-safe `min/max` Fresnel form with 100-lambda paraxial screens on both segments; `D > 2 lambda` aperture screen; IM3 domain `f1<f2<2f1`, `Pin<IIP3`, characterized-limit constraint, 10*log10(2) per-tone convention; one-way-range 1/R^4 radar with two-way loss applied once.
- Diagram quantity bindings: every catalog input and output key of each model is bound in `rf.dart` exactly as intended (checked key-by-key against the catalog; `panel.dart` legend lookup `quantities[label.quantity]!` is therefore safe); registry `catalog.dart:55` spreads all seven `rfDiagrams` entries.
- `docs/features/rf.md` matches catalog expressions, bounds, constraints and examples verbatim; the research evidence chain and the explicit disclosure of the non-sourced `10 lambda` screen are preserved.

## Pixel verdict per ID

| ID | Disposition |
|---|---|
| `rf.matched_t_attenuator` | PASS. Series-shunt-series topology, junction dots, wires stop at resistor bodies, load Z0 to common rail, no collisions or clipping. |
| `rf.lowpass_l_match` | **FAIL (F1)**: `L` label overlaps the inductor humps. Topology otherwise correct (source, Rs, L, node with C and Rl branches, dots present). |
| `rf.free_space_link_budget` | PASS with P3 note (F6). Antennas face each other, spreading arcs do not touch the propagation arrow, dimension line correct. |
| `rf.first_fresnel_clearance` | **FAIL (F7, P1)**: two large solid dark blocks over both ellipse ends; committed source cannot produce them. Visible geometry (LOS, obstacle, r1/h arrows, d1/d2, bottom labels) is correct and collision-free. |
| `rf.circular_aperture_gain` | PASS with P3 note (F8). No clipping; right panel inside canvas. |
| `rf.two_tone_im3` | PASS. Stem ordering/heights, per-tone vs each-IM3 bindings, C/IM3 bracket, no OIP3-as-tone. |
| `rf.monostatic_radar_return` | PASS. Opposed one-way arrows, target polygon, Dt/sigma/R bindings, no collisions. |

## Findings

### F1 (P2) — `L` label collides with the inductor symbol
- ID/file: `rf.lowpass_l_match`, `frontend/lib/diagrams/rf.dart` (`Label(307, 68, 'L', ...)`).
- Evidence: zoom crop of `rf.lowpass_l_match.png` (px about 600-760 x 110-230) shows the `L` glyph stem crossing the first two inductor humps; hump tops reach logical y about 78 while the label box spans y 68-82 at x 307-316, inside the coil x-range 285-345.
- Expected: label clear of geometry.
- Correction: change that label to
  `Label(352, 56, 'L', quantity: 'series_inductance', ink: DiagramInk.output),`
  (right of the coil at x 352, above the humps; clear of the `Zin` arrow at y 52 and of `Loaded Q` at (500,25)). Main must recapture `rf.lowpass_l_match.png` after the edit.

### F2 (P2) — `rf.lowpass_l_match` `shunt_capacitance` intermediate overflow — CONFIRMED
- `backend/catalog/rf.json` (expression `loaded_q/(2*pi*frequency*load_resistance)`). Independent check: with `{Rs:1e100, Rl:2e100, f:1e210}` the denominator `2*pi*f*Rl = 1.2566e311` overflows to +inf so the engine returns 0, although the reassociated value `loaded_q/(2*pi*frequency)/load_resistance = 7.9577e-312` F is representable (verified in IEEE double arithmetic).
- Correction: `loaded_q/(2*pi*frequency)/load_resistance`.

### F3 (P2) — `rf.lowpass_l_match` `loaded_q` argument overflow — CONFIRMED
- `backend/catalog/rf.json` (expression `sqrt((load_resistance-source_resistance)/source_resistance)`). With `{Rs:1e-200, Rl:1e200}` the ratio `(Rl-Rs)/Rs = 1e400` overflows before `sqrt`; `sqrt(load_resistance-source_resistance)/sqrt(source_resistance) = 1e200` is representable and exact for this domain (`Rl > Rs > 0`).
- Correction: `sqrt(load_resistance-source_resistance)/sqrt(source_resistance)`.

### F4 (P2) — `rf.lowpass_l_match` `series_inductance` intermediate overflow — CONFIRMED
- `backend/catalog/rf.json` (expression `source_resistance*loaded_q/(2*pi*frequency)`). With `{Rs:50, Rl:200, f:1e308}` the product `2*pi*f = 6.28e308` overflows, zeroing L (and C), although `source_resistance*loaded_q/(2*pi)/frequency = 1.3783222385544802e-307` H is representable.
- Correction: `source_resistance*loaded_q/(2*pi)/frequency` (and the C expression likewise).

### F5 (P2) — `rf.circular_aperture_gain` `gain_linear` intermediate overflow — CONFIRMED
- `backend/catalog/rf.json` (expression `(sqrt(aperture_efficiency)*pi*(diameter*frequency/299792458))^2`). With `{D:1e100, f:1e210, eta:1e-300}` the inner `D*f = 1e310` overflows so gain is reported infinite, although the reassociated `(sqrt(aperture_efficiency)*pi/299792458*diameter*frequency)^2 = 1.0981415890099112e+304` is representable (independently verified).
- Correction: `(sqrt(aperture_efficiency)*pi/299792458*diameter*frequency)^2`.

### F7 (P1) — Fresnel PNG shows two large dark filled blocks not producible from committed source
- ID/files: `rf.first_fresnel_clearance`; pixels vs `frontend/lib/diagrams/rf.dart` Fresnel marks + `frontend/lib/diagrams/painter.dart`.
- Evidence: in `rf.first_fresnel_clearance.png` two solid fills of exactly the muted ink RGB (63,73,69) (= `colors.onSurfaceVariant`) cover logical about [52.5..127.5]x[42.5..175] and its mirror [572.5..647.5], i.e. both ellipse-end regions: they obscure the ellipse vertices and most of the outline (only fragments survive), the TX/RX port Dots and the local label area. This image is unique among the captures: about 24x more muted-ink pixels than any other RF image (7991 vs <=333 sampled); the other PNGs show flat paper at the same coordinates.
- Source analysis: committed source cannot paint them. `DiagramPainter` strokes wires and parts (stroke-only paints; the only filled primitives are `Dot` r=3.2 and 8x4 arrowheads), and the Fresnel mark list contains no filled or occluding mark — `Part(CircuitPart.circle, 350, 110, width: 590, height: 130, ink: DiagramInk.muted)` renders a solid 1.8-logical-wide oval stroke. The receipt (`native_images.json`) records this exact file SHA, so the recorded capture is faithfully a corrupted/stale render, not a source-accurate one; the "refreshed after 81500de" claim does not hold for this ID.
- Minimal repair: re-capture `rf.first_fresnel_clearance` from a clean build of 2cca03e (`flutter test integration_test/expansion_acceptance.dart -d linux`) and re-verify pixels; the committed marks should render a clean muted ellipse with no blocks. If the blocks reproduce on a clean recapture, the defect is in the capture/compositing path (transparent-canvas-over-paper step), not in `rf.dart` — do not "fix" the diagram source for this.

### F6 (P3, non-blocking) — free-space `Margin`/`Sensitivity` labels detached from RX, adjacent to the distance line
- `frontend/lib/diagrams/rf.dart` (`Label(462, 200, 'Margin', ...)`, `Label(482, 239, 'Sensitivity', ...)`). Research section 3 binds them "at RX"; as rendered they float above/below the distance dimension line mid-canvas and can be misread as line annotations (ink color is the only disambiguator). No glyph collision. Optional correction: relocate both to the RX end (e.g. `Margin` to (604,200), `Sensitivity` to (560,239)); recapture.

### F8 (P3, non-blocking) — aperture diagram lacks the "visibly smaller conceptual collecting-area annotation"
- `frontend/lib/diagrams/rf.dart` (`rf.circular_aperture_gain`). Research section 5 requires `aperture_efficiency` bound beside a visibly smaller conceptual collecting-area annotation (explicitly not a metal disk). The caption claims the conceptual annotation, but only a plain `Efficiency eta` label exists. Optional correction: add a small muted dashed circle (e.g. `Part(CircuitPart.circle, 200, 130, width: 96, height: 96, ink: DiagramInk.muted)` plus a "conceptual Ae" note) before recapture.

## Gate

**HOLD.** Blocking: F7 (P1 pixel defect; a source-accurate capture is missing), F1 (P2 collision), F2-F5 (P2 confirmed conditioning defects — same accepted overflow-conditioning pattern as SEN-T1; coordinated with T2RfRunR2's intentionally failing regressions, expected to pass after Main applies the four reassociations and reruns `cargo test --test rf`). F6/F8 are non-blocking.

Untested scope (honest): no engine execution by this reviewer — the 228-example pass, constraint rejection behavior, malformed-input recovery and layout gate rest on Main's native runs and the tester lane; PNG evidence is the 2cca03e recapture set, which F7 shows is not trustworthy for the Fresnel ID until recaptured; diagram behavior at large text scale / narrow window and the accessibility legend were reviewed from source only; no other-lane interactions re-verified.
