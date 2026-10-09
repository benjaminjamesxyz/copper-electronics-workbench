# V2Converters source/native-pixel review — snapshot 2cca03e

- Reviewer lane: V2Converters. Date: 2026-10-10.
- Production root: `/home/simson/pi-projects/pcb-design-calculator`, HEAD `2cca03e854eb6842be56411a9153b6ca1b251a42` (verified with `git rev-parse` at review start).
- Basis: read-only inspection + independent arithmetic. No build, test, lint, formatter, process, or native launch was run here; no production file was edited or committed.
- Inputs compared: `research/expansion/converters.md`, `research/expansion/converters.json`, `backend/catalog/converters.json`, `frontend/lib/diagrams/converters.dart`, `frontend/lib/diagrams/primitives.dart`, `frontend/lib/diagrams/catalog.dart` (registry wiring, lines 24/58), `docs/features/converters.md`, backlog items 041–046 (`coordination/expansion/BACKLOG.md:887-1011`).

## Images actually viewed (pixel gate)

Every assigned native PNG was opened as an image (not inferred from filename or source):

1. `/home/simson/pi-projects/pcb-design-calculator/coordination/expansion/screenshots/converters.inverting_charge_pump.png`
2. `/home/simson/pi-projects/pcb-design-calculator/coordination/expansion/screenshots/converters.sepic_ccm_stress.png`
3. `/home/simson/pi-projects/pcb-design-calculator/coordination/expansion/screenshots/converters.forward_reset_winding.png`
4. `/home/simson/pi-projects/pcb-design-calculator/coordination/expansion/screenshots/converters.flyback_dcm_demagnetization.png`
5. `/home/simson/pi-projects/pcb-design-calculator/coordination/expansion/screenshots/converters.llc_half_bridge_fha.png`
6. `/home/simson/pi-projects/pcb-design-calculator/coordination/expansion/screenshots/converters.dual_active_bridge_power.png`

All six present; none missing.

## Source verdict

**Source: PASS with findings** (no numerical, unit, constraint, or scope defect; two annotation/cosmetic findings below).

Independent verification performed (Python `math`, outside the engine):

- Semantic diff of `backend/catalog/converters.json` against `research/expansion/converters.json`: identical for all six IDs — titles, formulas, assumptions, references, inputs, outputs, constraints, and every example input/expected digit-exact. No research-to-catalog drift.
- 50+ spot values recomputed from the printed equations; all matched to 1e-12 relative, including every catalog example field, the research limiting cases (SEPIC unity-conversion RMS 1.4195069566578393 / Cp RMS 1.0037429949942367; forward r=0.5 map; flyback clean-alternate 0.07216878364870322 and n·Ipk·Ddemag/2 = 1 A identity; LLC fs=40 kHz reactance −10.53912559450507; DAB φ=π/2, φ=5π/6 falling-branch equality), and all backlog numerical anchors (BACKLOG lines 892, 916, 939, 962, 985, 1008).
- Physics re-derivation against the cited primary sources, all consistent: charge-pump `Rout = 1/(fs·Cfly) + Rswitch + 4·ESRfly + ESRout` with fs = complete-cycle frequency (S1 convention respected; no fosc confusion); SEPIC uncoupled ripples without the coupled factor 2, switch RMS `√D·hypot(I1+Io, ΔIΣ/√12)`, ripple-inclusive Cp RMS with `(1−D)=Vin/(Vin+Vo)`, branch-positive constraints matching the stated narrower domain; forward `D=n·Vo/Vin`, `Dreset=r·D` from volt-second balance, `idle=(Dlimit−D)(1+r)` identity, `VDS=Vin(1+1/r)`; flyback `Ipk=√(2VoIo/(Lm·fs))`, `D+Ddemag<1` constraint algebraically identical to the interval sum (re-expanded and checked), full-period triangular RMS, internally consistent unity-efficiency limit (no half-applied efficiency); LLC FHA gain `M=1/hypot(1+(1−1/x²)/k, Q(x−1/x))`, `Rac=8n²Ro/π²`, half-bridge `Vo=Vin·M/(2n)`, reactance last term `ωLm/(1+(ωLm/Rac)²)` — the JSON's normalized `hypot` form is algebraically equal (verified symbolically by expansion); DAB signed `P=nV1V2φ(1−|φ|/π)/(2πfsL)` with full ±π domain, `<=π` constraint admitting endpoints exactly as research requires.
- No arbitrary clamps/bounds added anywhere; constraint set matches research exactly (charge-pump `Iload·Rout<Vin`, SEPIC two valley constraints, forward `D<1/(1+r)`, flyback DCM strict, LLC none, DAB `|φ|≤π`).
- `docs/features/converters.md` matches `research/expansion/converters.md` section-for-section (equations, examples, cases, diagram specs, provenance paragraph included).
- Diagram source topology vs the mandatory specs: charge-pump two-panel switch states (closed = straight link, open = contact part; upper plates `+`; Cout/load between rails; load arrow into the negative node); SEPIC node A→Cp→B, L2 ground→B only, diode anode B/cathode O, common ground; forward three windings with specified dot placement (primary top, reset lower, secondary top), reset diode anode at undotted upper terminal / cathode to +Vin (verified via `primitives.dart` turns semantics: `turns:3` ⇒ cathode up), D2 anode at secondary return/cathode at S; flyback reversed dots (primary top, secondary bottom), secondary undotted top → diode anode, three-interval sketch with `n × Ipk` label unbound; LLC series Cr→Lr→P, Lm shunt in parallel with the primary (not in series), full-wave block with two `~` and separate +/− pins, no Cr bypass wire, labeled lower FHA equivalent with `Rac = 8 n² Ro/pi²` and `M = 2 n Vo/Vin` bound to `tank_gain` (dimensionless, correct binding); DAB A→L→dotted primary, B return, secondary dots→C, four switches per bridge, separate returns, `P > 0 →` arrow primary→secondary, phi annotation between waveform transition markers. All quantity bindings checked key-by-key against the catalog — exact.
- Registry: `convertersDiagrams` exported and merged in `catalog.dart` (imports line 24, spread line 58) for all six IDs.

## Pixel verdict

**Pixel: HOLD** — one material finding (F1) plus one cosmetic family (F2). Everything else inspected clean: no clipping outside the 700×280 logical canvas, no wire crossing drawn as connected without a Dot, no primary/secondary (or bridge-to-bridge) return shorted across isolation, switch open/closed states render as specified, winding-dot markers sit at the specified terminals, arrows and reference directions correct except F1, all bound labels legible and mean the bound quantities except F1's neighbor.

### Findings

**F1 — P2 — converters.sepic_ccm_stress — misleading iCp arrow annotation**
- File: `frontend/lib/diagrams/converters.dart:87-88`
- Evidence (source + PNG 2): a vertical downward arrow `Wire([257, 113, 257, 167], arrow: true)` is captioned `Label(276, 96, 'iCp →', ...)`. The rendered image shows a down-pointing arrow beside the Q branch labeled "iCp →". iCp is defined through Cp horizontally left-to-right (`'Cp on: −iL2; off: +iL1'`, `converters.dart:90`); the drawn arrow's direction contradicts its own label's `→`, and its position (between the Q drain node and Cp, parallel to the switch branch) binds the iCp name to the switch current location. Reference directions are contract-critical; a reader will mis-assign the capacitor-current direction.
- Expected: the direction cue near Cp pointing left-to-right; the vertical arrow either labeled as the switch current or removed.
- Minimal correction (two lines, replacing line 88 and keeping line 87):

```dart
      Label(297, 96, 'iCp →'),
      Label(276, 96, 'iQ ↓'),
```

  (`(297,96)` sits directly below the Cp symbol, clear of its plates (end y≈88), of the down arrow (x=257), and of L2's arrow (x=382); the down arrow at x=257 then correctly annotates the on-time switch current, consistent with the existing `iQ = iL1 + iL2 (on)` legend.)

**F2 — P3 — five capacitor-adjacent labels graze their symbols' plate/lead glyphs (cosmetic; all legible)**
- Instances (file `frontend/lib/diagrams/converters.dart`): line 44 `Label(565, 121, 'Cout')` (charge pump; 'C' touches the right plate end of the 50-wide vertical cap at 555), line 67 `Label(297, 44, '+ Cp')` (SEPIC; 'Cp' crosses the left plate top at x=310), line 75 `Label(565, 132, 'Cout')` (SEPIC; touches right plate end x≤568), line 129 `Label(588, 190, 'Cout / load')` (forward; 'ad' crosses the load resistor's bottom lead at x=650, y=190–204), line 166 `Label(527, 96, 'Cout')` (flyback; touches right plate end x≤528), line 231 `Label(604, 142, 'Co')` (LLC; touches right plate end x≤608).
- Evidence: visible in PNGs 1–5 as slight glyph/line contact; no text is obscured and no quantity meaning changes.
- Minimal correction: shift each listed x by +12 (e.g. `Label(577, 121, 'Cout')`, `Label(309, 44, '+ Cp')` — note line 67's '+ Cp' then also clears plate x=310 — `Label(577, 132, 'Cout')`, `Label(600, 190, 'Cout / load')`, `Label(539, 96, 'Cout')`, `Label(616, 142, 'Co')`). Optional; not blocking on its own.

### Per-ID pixel notes

- `converters.inverting_charge_pump` (PNG 1): both panels correct — phase A charges Cfly (+ upper plate) via closed S1/S3 links with S2/S4 open contacts to GND/Vout stubs; phase B inverts via closed S2 to GND(+) and S4 to Vout; S1/S3 open; Cout and load parallel between rails; downward load arrow into the negative node; `Vout (−)`, `Iload ↓`, `fs`, `Rout: equivalent loss` annotations bound and outside the switch network; caption carries both mandatory sentences. PASS (F2 cosmetic only).
- `converters.sepic_ccm_stress` (PNG 2): topology and all junction dots correct; L2 arrow up, diode anode B→cathode O; legend row correct. F1 defect present → this ID blocks.
- `converters.forward_reset_winding` (PNG 3): dot placement exactly as specified; reset diode cathode to +Vin (reverse-biased at on-time, correct `turns:3` orientation); freewheel D2 anode at secondary return; returns isolated; bottom fractions row bound. PASS (F2 cosmetic only).
- `converters.flyback_dcm_demagnetization` (PNG 4): reversed dots block the secondary diode during storage; store/deliver arrows clearly explanatory; three-interval sketch has no primary/secondary conduction overlap, dashed boundaries align, `iSec: n × Ipk` unbound as required. PASS (F2 cosmetic only).
- `converters.llc_half_bridge_fha` (PNG 5): half-bridge stack, series Cr→Lr, shunt Lm parallel to the primary, no Cr bypass, full-wave block with two AC and separate +/− DC pins, isolated output return, labeled FHA equivalent with correct `Rac`/`M` bindings. PASS (F2 cosmetic only).
- `converters.dual_active_bridge_power` (PNG 6): both four-switch bridges, A→L→dotted primary / B return, secondary dots→C undotted→D, separate returns, all rail crossings dot-free (caption also states it), `P > 0 →` arrow primary→secondary, primary transition leads secondary by phi between the dashed markers, `phi`/`fs`/bus labels bound. PASS.

## Gate

**HOLD** — material finding F1 (P2) is open. After the two-line `converters.dart` repair (F1) and recapture of `converters.sepic_ccm_stress.png`, this lane is PASS; F2 is optional polish and does not block. No numerical regression exists to report: the catalog reproduces the research oracle digit-for-digit for all six IDs.

## Untested scope (honest)

- No engine, test suite, build, formatter, or desktop process was run by this reviewer; the "all examples pass" claim at 2cca03e remains Main's executed check. My numeric verification is independent re-derivation, not execution of the Rust engine.
- Invalid-input rejection (zero/negative/NaN/inf, equality boundaries such as SEPIC L1=12 µH, forward vout=16 V, flyback Io=0.125 A, DAB |φ|>π) was verified only by reading constraint expressions against the shared boundary semantics, not by executing them.
- Pixel review covers the six static diagram captures only: font rendering, interactive UI, re-layout at other window sizes, and app-side value formatting were not observed.
- Primary-source reread was limited to ambiguity resolution of the already-cited equations (S1–S7 as summarized in the research handoff); no fresh literature audit was performed.
- Original 128 calculators and other lanes untouched and out of scope.

## Cross-reference: T2ConvertersRun conditioning findings (appended during this review)

T2ConvertersRun's bus entries CONV-T1..T3 (LLC ratio square, DAB bus-product ordering, flyback `Lm·fs` product) landed on the bus while this report was being finalized. Read-only corroboration: the arithmetic contradiction is real for the two inspected in detail — DAB `V1·(n·V2/L/fs)` at `V=2e154` forms `4e308 > binary64 max` although `Pmax = 4e308/8 = 5e307 W` is representable, and flyback `Lm·fs = 1e400 → inf` although the true `duty = √2·0.01` is representable — i.e., expression-ordering conditioning defects in the production JSON at extreme magnitudes, owned by Main/T2ConvertersRun. These are outside this review's normal-magnitude oracle scope and do not change the source agreement (catalog == research, example-scale exact) or the pixel verdicts above; the HOLD gate stands on F1. Reconditioning those expressions must not alter the six example outputs.
