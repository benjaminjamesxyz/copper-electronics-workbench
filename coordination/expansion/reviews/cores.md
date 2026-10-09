# V2Cores expansion review — winding & magnetic design (6 IDs)

- Reviewer: V2CoresRunR2 (independent, read-only). Date: 2026-10-10.
- Snapshot: **2cca03e** (full: 2cca03e854eb6842be56411a9153b6ca1b251a42), production root /home/simson/pi-projects/pcb-design-calculator.
- Sources inspected in full: `backend/catalog/cores.json` (888 L), `frontend/lib/diagrams/cores.dart` (273 L), `docs/features/cores.md` (330 L), `research/expansion/cores.md`, `research/expansion/cores.json`, backlog entries 089-094 in `coordination/expansion/BACKLOG.md`, `frontend/lib/diagrams/primitives.dart` (mark semantics).
- Cross-consistency: `research/expansion/cores.json` is byte-equivalent (canonical-JSON diff) to `backend/catalog/cores.json`; backlog numerical anchors match catalog examples; docs examples match both.

## Independent source verification (all arithmetic recomputed from primary forms, not from catalog text)

1. `cores.single_layer_air_solenoid` — Wheeler inch form L[µH]=N²r²/(9r+10l) re-derived with r_in=r/0.0254 reproduces the catalog expression and the nominal anchor 4.037956793862305e-06 H exactly; pitch 0.0015 m; scaling/inclusive anchors reproduced (8.075913587724612e-06, 2.072109407376709e-06). Constraints (integer turns, l≥r, wire_dia≤pitch≤r/5) match the documented screens. Units m/H throughout. PASS physics.
2. `cores.square_planar_spiral` — Mohan eq.(2)/Table II square coefficients (1.27, 2.07, 0.18, 0.13) verified; di/do/davg/fill conventions consistent; nominal 5.825908169108072e-09 H and doubling anchor 1.1651816338216145e-08 reproduced; inclusive s=3w boundary documented. Diagram topology: one continuous spiral, dashed underpass joins only at the central Dot and crosses four conductors undotted (no false junctions). PASS physics.
3. `cores.bobbin_layer_fit` — floor/ceil packing, build n·do+(n−1)t, fill N·π·dc²/(4·Wu·H) verified; anchors 30/4/0.0027/0.0013/0.2727076956241139 reproduced; 90→3-layer vs 91→4-layer jump verified; one-layer no-tape case consistent. PASS physics.
4. `cores.sinusoidal_steinmetz_loss` — reference-eliminated Steinmetz pv=pref(f/fref)^α(Bpk/Bref)^β verified; nominal 210325.46208198377 W/m³ reproduced; all four fit-rectangle constraints cover BOTH operating and reference points; ordered-bounds and exclusive-min exponents correct. PASS physics.
5. `cores.lamination_eddy_loss` — pv=(πfBpk·t)²/(6ρ) re-derived from MIT RMS form ω²B_rms²t²/(12ρ) with B_rms=Bpk/√2 — agrees; penetration ratio t√(πfµ0µr/ρ) = t/δ correct; nominal anchors reproduced; x=0.3 boundary at 284.9658289940749 Hz matches docs. PASS physics.
6. `cores.thin_foil_winding_loss` — the x⁴ truncation was independently re-derived: Dowell portion Rac/Rdc = Re[M+(m²−1)D/3], M=z·coth z, D=2z·tanh(z/2), z=(1+j)x gives Re M=1+4x⁴/45, Re D=x⁴/3 ⇒ excess=(5m²−1)x⁴/45 — the catalog formula is exact-truncation, not an approximation error. Full complex evaluation at x=0.5 reproduces the docs' truncation table to all printed digits (m=1: 1.0055423618, m=3: 1.0609577347, m=10: 1.6913076020). First neglected term −[16/4725+17(m²−1)/3780]x⁸ verified. Rdc/excess/loss anchors and the f=0, I=0 DC limits verified; the (I·√R)² / product-before-square conditioning forms behave as documented. PASS physics.

No unit errors, no reference-direction errors, no constraint gaps, no scope overlap between the six models or with closed lanes.

## Findings

All findings are binary64 intermediate overflow/underflow losses: the exact real output is finite and representable while the authored expression loses it. Every vector below was independently re-verified (constraints admissible, fp behavior and exact value computed). None affects any physically meaningful magnitude; all six nominal examples reproduce at 1e-9. Severity P2 (medium) per the lane's own conditioning standard — the foil model explicitly engineers this class (docs: "Preserve representable dimensional excess heat…"), so the same standard applies lane-wide. Repairs are expression-string only; `exp`/`ln` are existing engine functions (used across catalog). No arbitrary bounds added.

| # | Sev | ID | Location | Defect (verified vector) | Minimal correction (verified equivalent) |
|---|-----|----|----------|--------------------------|------------------------------------------|
| F1 | P2 | cores.single_layer_air_solenoid | backend/catalog/cores.json:63 | r=1e-100, l=1e-100, N=1e160, w=1e-261 (all constraints pass): turns^2=1e320→inf, request rejected; exact L=2.0721094073767093e214 H (corroborates tester CORE-T1) | `(1e-6/0.0254)*radius*turns*(turns/(9+10*length/radius))` — vector yields 2.0721094073767093e214; nominal anchor rel diff 0.0 |
| F2 | P2 | cores.square_planar_spiral | backend/catalog/cores.json:187 | N=1e160, w=s=1e-170, do=1e-9: di>0, s≤3w hold, fill=0.25; turns^2→inf rejected; exact L=1.3833e305 H | `4*pi*1e-7*turns*(turns*average_side)*1.27/2*(ln(2.07/fill_ratio)+0.18*fill_ratio+0.13*fill_ratio^2)` — nominal rel diff 0.0 |
| F3 | P2 | cores.bobbin_layer_fit | backend/catalog/cores.json:329 | W=1e201, m=0, do=dc=1e200, H=1e201, N=1 (q=10, 1 layer, fits): dc^2=1e400→inf rejected; exact fill=0.00785398163397448 | `turns*pi*copper_diameter*(copper_diameter/(4*(window_width-2*end_margin)))/window_height` — vector exact; nominal rel diff 4.1e-16 |
| F4 | P2 | cores.sinusoidal_steinmetz_loss | backend/catalog/cores.json:504 | f=1e160, fref=1, B=1e-160, Bref=1, pref=3, α=β=2 (inside fit rect): (f/fref)^α=inf, ×B^2=1e-320 subnormal → inf rejected; exact pv=3 W/m³, P=6 W, E=6e-160 J (corroborates CORE-T2) | `reference_loss_density*exp(frequency_exponent*ln(frequency/reference_frequency)+flux_exponent*ln(flux_peak/reference_flux_peak))` — vector yields exactly 3.0; nominal rel diff 1.4e-16 |
| F5 | P2 | cores.lamination_eddy_loss | backend/catalog/cores.json:695 (via 689) | f=120, B=1e-170, t=1.5e-4, ρ=8e-7, V=1e100: density underflows to admitted 0, heat lost; exact heat 6.661982970735316e-238 W representable (corroborates CORE-T3; penetration 0.07300401616752501 verified) | eddy_loss: `metal_volume*(pi*frequency*flux_peak*lamination_thickness)*(pi*frequency*flux_peak*lamination_thickness/(6*resistivity))` — vector yields 6.661982970735314e-238; nominal anchor rel diff 0.0. Keep eddy_loss_density as-is (admitted zero matches foil precedent and the tester's expected map) |
| F6 | P2 | cores.thin_foil_winding_loss | backend/catalog/cores.json:826 and 844 | m=1e160, thickness_ratio=1e-100 (admissible): (5m²−1)/45=inf × ratio^4=0 → NaN at :826, and sqrt(inf) → inf at :844; exact excess factor ~1.1e-81, exact excess heat 5.73e-82 W representable. At m=1e160, ratio=1e-80 exact excess heat 0.0573333333 W — also lost | :826 → `(5*layers*(layers*thickness_ratio^4)-thickness_ratio^4)/45` (vector→0 admitted, m·(m·x⁴) never materializes m²; nominal rel diff 1.3e-16; m=1 exact). :844 → `(current_rms*sqrt(resistance_dc))^2*(5*(layers*thickness_ratio*thickness_ratio)^2-thickness_ratio^4)/45` (vector→5.733333333333331e-82 exact; mid vector→0.057333333333333326; m=1 factor exact; nominal rel diff 1.3e-16) |

F7 (P3, non-blocking): cores.thin_foil_winding_loss diagram binds `current_rms` by name only — `frontend/lib/diagrams/cores.dart:258` Label 'Equal Irms'. Backlog label acceptance (key present, correct meaning) is satisfied, but docs' mandatory-diagram prose "Bind the current magnitude arrow to `current_rms`" is not literally realized (direction dots exist; no magnitude arrow carries the quantity). Minimal correction if Main wants literal docs conformance: add e.g. `Wire([57, 95, 57, 141], arrow: true, ink: DiagramInk.input)` beside the return symbol with the existing label — requires recapture of that one PNG. Not required for acceptance.

Note (all F1-F6): string-level reassociation closes every demonstrated instance and all physical magnitudes by hundreds of orders; a theoretical tail remains for inputs within a few ulp of f64 max (e.g. solenoid N≈1e308 with l/r≈3e307 can still overflow the denominator). Complete coverage would need engine-side scale-aware evaluation — Main's call, not required for this lane's acceptance.

## Pixel review (all six native captures inspected as images)

Exact paths viewed:
- coordination/expansion/screenshots/cores.single_layer_air_solenoid.png — helix solid/dashed alternation, front-current-up arrow on first conductor, axial B arrow pointing left (right-hand-rule consistent), pitch bracket between first two turn tops, length arrow spanning the current-sheet envelope, end-view circle with centre dot + radius arrow, wire-diameter callout. No clipping, no text/geometry overlap (B label and I label clear their nearest lines). OK.
- coordination/expansion/screenshots/cores.square_planar_spiral.png — 4-turn continuous spiral, central dot junction, dashed underpass with four undotted crossings, outer-side arrow spans the true 190-px square, di arrow inside the clear opening, enlarged width/gap inset arrows bind copper vs dielectric. OK.
- coordination/expansion/screenshots/cores.bobbin_layer_fit.png — 8-8-8-3 rows (partial top layer), three tape bands strictly between occupied rows, hatched end margins with paired Margin arrows, h/H/clearance arrows bind build/window/clearance correctly, double-circle wire inset with finished-Ø arrow. OK.
- coordination/expansion/screenshots/cores.sinusoidal_steinmetz_loss.png — closed ungapped core, clockwise flux loop inside core material, winding zigzag on the left leg, zero-centred sine from origin with Bpk at first peak, T=1/f bracket spanning exactly one period, calibration callout row. OK (flux loop crossing the winding symbol is standard schematic layering, no false junction).
- coordination/expansion/screenshots/cores.lamination_eddy_loss.png — three separated sheets, in-plane Bpk arrow, antiparallel +Jx/−Jx pair on the sheet face matching the ±t/2 section inset, B-out-of-page ⊙ consistent with J⊥B geometry, metal-only t bracket, insulation gaps distinct. OK.
- coordination/expansion/screenshots/cores.thin_foil_winding_loss.png — three foil strips with current dots, ⊗ return, H=0 dashed boundary, rising MMF staircase, width-b and thickness-t callouts, mean-centreline inset, four result labels. OK (see F7 for the optional missing magnitude arrow).

No missing captures; all six post-fix recaptures (dated Oct 10 00:02) received and viewed.

## Verdicts

- Source verdict: **HOLD** — physics, units, constraints, examples and diagram topology are correct, but F1-F6 (P2) are confirmed representable-output losses in shipped expressions; one-line string repairs verified above.
- Pixel verdict: **PASS** — 6/6 captures clean; no clipping, collision, false junction, or mislabeled quantity found.
- Overall gate: **HOLD** pending Main's application of F1-F6 (backend/catalog/cores.json only; no recapture needed since diagrams are untouched). F7 optional.

## Honest untested scope

- No code was executed by this reviewer (per contract): all fp behaviors above are IEEE-754 semantics verified in an independent Python REPL, not the Rust engine; Main should run the repaired expressions plus the tester's 21 scenarios after applying F1-F6.
- Capture receipt accepted from Main's dispatch (files present, post-fix timestamps); reviewer did not witness the Flutter capture session.
- Repairs were verified for the listed vectors, nominal anchors, and boundary/m=1 cases; an fp tail remains within a few ulp of f64 max (noted above).
- RF lane's Fresnel end-block defect is out of this lane's scope and was not re-analyzed.
- Docs/research prose was cross-checked against catalog data (JSON identical), not re-sourced from the web; primary-form derivations (Wheeler, Mohan, Dowell, MIT) were verified mathematically instead.
