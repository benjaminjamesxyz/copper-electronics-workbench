# V2LogicRun — independent source and native-pixel review, digital logic lane

- Snapshot: `2cca03e854eb6842be56411a9153b6ca1b251a42` (working tree confirmed clean of lane edits; Main: persist this report verbatim to `coordination/expansion/reviews/logic.md`).
- Scope: six required IDs — `logic.guaranteed_noise_margins`, `logic.dc_current_fanout`, `logic.cmos_package_power`, `logic.synchronizer_metastability`, `logic.three_stage_logical_effort`, `logic.fifo_envelope_depth`.
- Inputs reviewed end to end: `backend/catalog/logic.json` (770 lines), `frontend/lib/diagrams/logic.dart` (211 lines), `docs/features/logic.md` (438 lines), against `research/expansion/logic.md` (341 lines) and `research/expansion/logic.json`, plus the backlog lane. Downstream dispatch checked: registry merge at `frontend/lib/diagrams/catalog.dart:63` (`...logicDiagrams`); engine constraint right-sides are full expressions (`backend/src/lib.rs:67`), sequential output insertion (`backend/src/lib.rs:122-126`), and required math functions present (`backend/src/lib.rs:71-83`).
- All six native PNGs were opened and inspected as images (not filenames, not source alone):
  - `/home/simson/pi-projects/pcb-design-calculator/coordination/expansion/screenshots/logic.guaranteed_noise_margins.png`
  - `/home/simson/pi-projects/pcb-design-calculator/coordination/expansion/screenshots/logic.dc_current_fanout.png`
  - `/home/simson/pi-projects/pcb-design-calculator/coordination/expansion/screenshots/logic.cmos_package_power.png`
  - `/home/simson/pi-projects/pcb-design-calculator/coordination/expansion/screenshots/logic.synchronizer_metastability.png` (plus a 2x zoom crop of the FF2 region for the finding below)
  - `/home/simson/pi-projects/pcb-design-calculator/coordination/expansion/screenshots/logic.three_stage_logical_effort.png`
  - `/home/simson/pi-projects/pcb-design-calculator/coordination/expansion/screenshots/logic.fifo_envelope_depth.png`
- No builds, tests, lint, formatters, backend processes, engine runs or native launches were performed. All arithmetic below is independent binary64/Decimal hand evaluation, not an executed engine result.

## Source verdict per ID (catalog = `backend/catalog/logic.json`, diagrams = `frontend/lib/diagrams/logic.dart`, docs = `docs/features/logic.md`)

1. `logic.guaranteed_noise_margins` — PASS. Formulas match TI SZZA036C/SCAA029 margins with signed ΔG; positive ΔG hurts high/helps low exactly as encoded (`logic.json:63-75`). Negative margins preserved (no clamp). Ordering constraints VOH>VOL, VIH>VIL present; noise magnitudes ≥0; `ground_offset` correctly unsigned-bounded. Example independently re-derived: 0.2 / 0.4 / 0.2 V ✓. Diagram spec conformance (separate grounds, dashed offset arrow with −/+ orientation, two ladders, adverse arrow directions down/up) confirmed in source and pixels.
2. `logic.dc_current_fanout` — PASS. Floor/min/headroom algebra matches conservation N·IIH≤Isource, N·|IIL|≤Isink; magnitude convention documented. Receiver currents strictly positive via `min:0`+`exclusive_min` ✓; zero driver limits and zero loads admitted; whole-number `load_count` constraint present. Nominal example re-derived exactly: `0.004/0.00002 = 200.0` and `0.008/0.0004 = 20.0` are binary64-exact, so the floors are exact, not luck; the fractional-ratio research vector `0.0039/0.0002 = 19.499999999999996 → 19` also confirms downward rounding. Headroom signs (including negative overload) preserved.
3. `logic.cmos_package_power` — PASS. SCAA035B form P=N·C·V²·f with no spurious ½; per-section Cpd semantics, once-only package leakage, separate input/output rates all encoded; supply >0 exclusive, integer sections ≥1, zero rates/capacitances admitted. Example re-derived exactly: 66e-6 / 0.026136 / 0.02178 / 0.047982 W / 0.01454 A ✓. Diagram correctly keeps Cpd as a dashed explanatory element, not a wired capacitor.
4. `logic.synchronizer_metastability` — PASS. Log-domain MTBF identical to exp(tres/tau)/(T0·fclock·fdata) without overflow; `required_resolution_time` clamped at 0; margin signed; N-chain Poisson extension is the documented derived extension, flagged as such. Rare-event guard `aperture_parameter < 1/data_transition_rate` encoded as a cross-input constraint; integer N≥1; tres≥0. Nominal example re-derived: 13.371779276130074 / 11.371779276130074 / 1.8995e-9 / 1.0047e-10 s ✓. The catalog example differs from the research/docs prose by exactly 1 ulp on two values (`…074` vs `…073`; `…4683e-9` vs `…4684e-9`) — association-order rounding, relative deviation ~1e-16 against tolerance 1e-6; immaterial, no action.
5. `logic.three_stage_logical_effort` — PASS. Log-sum effort expression equals F^(1/3) without overflow; both sizing equations reproduce the same middle effort (C2=9e-15, C3=36e-15 nominal, verified); D=3f+P; g≥1 domain via input minima; zero parasitic admitted. Nominal re-derived: f=4, C2=9e-15 F, C3=36e-15 F, D=16, 3.2e-10 s ✓.
6. `logic.fifo_envelope_depth` — PASS. B=b+rT for R≥r, ceil to whole words, bits, T+b/R delay — matches Le Boudec–Thiran Theorems 1.4.1/1.4.2 specialization. R≥r and T≥1/R constraints present with correct messages; equality R=r admitted; burst/width positive integers; arrival rate may be zero. Nominal re-derived: 89 / 89 / 2848 / 2.28e-6 ✓; equal-rate 114, min-T 64.5→65, binary-exact 4.5→5 all consistent.

Research-JSON vs production-catalog field comparison across all six IDs (keys, labels, units, defaults, bounds, exclusivity, constraints, expressions, examples): no substantive difference. Docs `docs/features/logic.md` mirror the catalog tables and constraints faithfully. All six quantity bindings used by the diagrams exist as catalog input/output keys (checked one by one); no quantity label binds another node or an uncomputed intermediate.

## Pixel verdict per ID (actual captures, post-fix recapture set)

1. `logic.guaranteed_noise_margins.png` — PASS. Driver/Receiver blocks each with its own ground (no common-ground wire); dashed ΔG arrow driver→receiver with − at driver, + at receiver; high ladder VOH above VIH with headroom bracket; low ladder VIL above VOL; high-noise arrow downward, low-noise arrow upward; all labels clear, no clipping or overlap.
2. `logic.dc_current_fanout.png` — PASS. Branched net with explicit junction dots at the three taps; three representative receiver blocks (no resistor icons); HIGH arrow driver→inputs, LOW arrow inputs→driver; N-identical label, IIH/|IIL| and DC Nmax callouts clear of wires; no collision.
3. `logic.cmos_package_power.png` — PASS. Supply arrow with VCC and I average; dashed internal Cpd box (not wired); real output capacitor with junction dot on the output path and ground return; three dashed accounting arrows to Quiescent / Internal switching / Load cycling; Total supply power callout; no collisions.
4. `logic.synchronizer_metastability.png` — **FAIL (one defect, F1 below).** Two-stage chain, shared destination-clock net with junction dots at both clock pins, decay envelope with time arrow, right-side reliability callouts all correct; but the "May be metastable" label collides with the FF2 block.
5. `logic.three_stage_logical_effort.png` — PASS. Generic Gate 1/2/3 chain, g1/g2/g3 above, C1/C2/C3 callouts below the respective input pins and not drawn as shunt loads; only CL drawn as a real capacitor to ground; path-delay arrow and equal-effort callout clear; no collision.
6. `logic.fifo_envelope_depth.png` — PASS. Source→FIFO→Reader row with bound rates; envelope plot: α starts at positive burst intercept on the word axis, β flat until T then rising steeper than α; vertical bracket at T exactly spans α(T) to the axis (= b+rT); dashed horizontal arrow at height b from the axis to the β crossing (= T+b/R); T bound below the axis; legend labels clear; no collision (verified the β top end stops ~16 px below the "Data storage bits" text band).

## Findings

### F1 / P2 / `logic.synchronizer_metastability` / `frontend/lib/diagrams/logic.dart:116`

- Evidence (pixels): in `coordination/expansion/screenshots/logic.synchronizer_metastability.png` the word "metastable" crosses the FF2 block's left edge; the final letters sit on/inside the block outline (confirmed with a 2x zoom crop of the region). Same defect class as the eight label collisions already fixed and recaptured in this wave.
- Evidence (source): `Label(241, 28, 'May be metastable', ink: DiagramInk.muted)`. Labels paint from their top-left corner (`frontend/lib/diagrams/painter.dart` case `Label`, `text.paint(canvas, Offset(mark.x, mark.y))`); the string is ≈135 canvas px wide at fontSize 14, ending ≈x376. FF2 is centered at x=405 with width 80 (`logic.dart:117`), so its left edge is x=365 — the text tail overlaps it by ≈11 px. Shifting alone cannot fix it: the block band starts at y=35 and the label at y=28 straddles it, so x≤230 would hit FF1 instead (the window is empty).
- Expected behavior: annotation text clear of all geometry.
- Minimal correction (two-line split, same wording, matching the existing High/headroom two-line precedent; both lines verified clear of FF1/FF2 blocks, the FF1/FF2 name labels, and the y=65 interstage wire):

```suggestion
      Label(248, 16, 'May be', ink: DiagramInk.muted),
      Label(248, 36, 'metastable', ink: DiagramInk.muted),
```

  Recapture `logic.synchronizer_metastability.png` after the edit. No other mark in this diagram (or the other five) needs to move.

No other findings: no numerical regression, no unit/reference/polarity error, no missing constraint, no cross-boundary dispatch gap (all diagram-bound quantities resolve to real catalog keys; engine constraint evaluation covers every constraint form used, including `1/service_rate` and `1/data_transition_rate` right-sides and `floor` whole-number checks).

## Gate

- Source: PASS (6/6).
- Pixels: HOLD (5/6; F1 blocks).
- **Overall: HOLD** — solely on F1; everything else is release-quality. After Main applies the two-line label fix and recaptures `logic.synchronizer_metastability.png`, this lane is PASS with no re-review of the other five images needed (they are unaffected).

## Untested scope (honest)

- No engine execution: all expected values are independent hand/Decimal derivations; runtime acceptance (228-example suite, malformed-input recovery) remains Main's executed gate.
- Numerical conditioning was analyzed by reading, not fuzzed: pathological-but-valid extreme magnitudes (e.g. `tres` near f64 max over tau, subnormal capacitances) were not exhaustively enumerated; the log-domain/monotone expression forms make overflow unreachable for admitted finite inputs, but this is analysis, not execution.
- Caption text is authored in `logic.dart` and rendered by the app panel outside the 700×280 canvas; captions are not visible inside the captured PNGs, so caption wrapping/layout on narrow windows was not pixel-verified.
- Physical truth of user-supplied datasheet constants (real VOH/VIH/Cpd/T0/tau, actual FIFO guarantees) is by design the designer's responsibility and untestable here.
- The RF-lane Fresnel dark-block defect is another lane's scope; not inspected, not duplicated.
