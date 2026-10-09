# V2EmcRun — EMC lane source and native-pixel review

- Snapshot: **2cca03e** (read-only production root `/home/simson/pi-projects/pcb-design-calculator`).
- Reviewer: V2EmcRun. Scope: all six EMC IDs against `research/expansion/emc.md`, `research/expansion/emc.json`, `coordination/expansion/BACKLOG.md` (items 053-058), `backend/catalog/emc.json`, `frontend/lib/diagrams/emc.dart`, `docs/features/emc.md`.
- Method: independent binary64 recomputation of every catalog example and every backlog/tester anchor; exact complex 2x2 nodal solve for the coupled filter; full source read of catalog, diagrams, primitives and docs; every listed PNG opened and inspected as an image. No build, test, lint, formatter, process, desktop launch, or commit was performed.

## Images actually viewed (native captures, composited canvas on paper)

1. `coordination/expansion/screenshots/emc.thin_slot_shielding.png`
2. `coordination/expansion/screenshots/emc.rc_common_mode_conversion.png`
3. `coordination/expansion/screenshots/emc.trapezoid_conducted_harmonic.png`
4. `coordination/expansion/screenshots/emc.sinusoidal_dither_occupancy.png`
5. `coordination/expansion/screenshots/emc.ringing_rc_snubber.png`
6. `coordination/expansion/screenshots/emc.short_cable_radiation.png`

All six required images exist and were viewed. None missing.

## Independent numerical verification (source gate, all six IDs)

Recomputed in binary64 outside the implementation; every catalog `expected` value and every backlog anchor reproduced within floating-point rounding (<= 2 ulp):

- thin_slot: lambda=2.99792458 m; SE=37.495214231999306 dB; ratio=0.013342563807926084.
- rc_common_mode: D=-2e-7 s; gain=0.000804107324707...; Vdiff=8.041073247070084e-05 V; Cd=0 variant 0.009009434213906362 matches the documented 0.009009434213906478.
- trapezoid: fn=300 kHz; Idc=4e-4 A; In=8.806826043190419e-05 A; Vn=0.004403413021595209 V; triangular endpoint 2*sqrt(2)*Ipk/pi^2 = 0.0002865795841253782 reproduced.
- dither: beta=15; Bn=64000 Hz; gap=26000 Hz; n=5 overlap -14000 Hz; df=1 Hz narrowband case 0.0015/4006/95993.
- snubber: Cp=1e-9 F; Lp=2.814477323398272e-08 H; Rs=3.53677651315323 ohm; Cs=3e-9 F; P=0.216 W; zeta=1,k=4 endpoint 2.6525823848649224 ohm / 4e-9 F / 0.288 W.
- cable: l/lambda=0.03335640951981521; kr=20.958450219516816; E=6.2831853071795875e-06 V/m; doubled distance halves E exactly.

Targeted probe of the coupled filter at unbalanced R1=500 ohm, R2=2000 ohm with Cd>0: the catalog expression equals the exact complex nodal solution to 7e-16 relative. The Cd terms genuinely cancel in V1-V2 for any R1,R2 (R2Cd-R2Cd and R1Cd-R1Cd), so the "general R1/R2" claim in research/docs holds. No numerical defect.

Physics identities checked against the cited primary sources as documented in research (Laird aperture rule 20*log10(lambda/2L); TI Rice/Gehrke/Segal eq (1) with peak-to-RMS /sqrt(2) and the convolution double-sinc; Carson Bn=2(n*df+fm); TI SLVA790A two-frequency identification with factored small-difference form; MIT 10.2.8 broadside with mu0*f*I*l/(2r) and RMS-consistent scaling). Units and dimensions verified for every output. Screening constraints match the research domains exactly (L>w, L>=10t, L<=lambda/20; tr*fs<=min(D,1-D) plus positive-argument underflow guard; df<fs, fm<fs; f1<f0, fs<f1, zeta in [0.5,1], k in [3,4]; l/lambda<=0.1, kr>=10). Signed/admitted-zero semantics preserved (signed mismatch_time and signed dither gap; zero Cd/frequency/Vcm/Icm admitted; no compliance labels).

## Source findings

No blocking source defect. Four catalog-vs-research textual discrepancies, all verified algebraically equivalent or equivalent-or-stricter; reported per contract, no normalization requested:

1. **P3 / info - `emc.thin_slot_shielding` expression rewritten to log-difference.** `backend/catalog/emc.json:65` uses `20*(log10(wavelength)-log10(2*slot_length))` instead of research `20*log10(wavelength/(2*slot_length))`. Identical value within 1 ulp at 1e-6 tolerance; strictly better conditioned for tiny admitted slot lengths (avoids quotient overflow). No action.
2. **P3 / info - `emc.ringing_rc_snubber` gained a representability constraint.** `backend/catalog/emc.json:578-580` bounds `2*pi*f0*sqrt(Ctest)*sqrt((f1/(f0-f1))*(f1/(f0+f1))) <= 1.3407807929942596e161`. The left side is exactly the denominator of the catalog's factored `parasitic_inductance` expression, so it rejects only inputs whose Lp would collapse below ~5.6e-323 H (deep-subnormal/zero), matching the research handoff "nonfinite/zero-underflow critical intermediates must report arithmetic errors". Nominal and tester vectors are ~120 orders below the bound. No action.
3. **P3 / info - `emc.short_cable_radiation` far_field_parameter and its constraint regrouped** (`backend/catalog/emc.json:684,701`) to `(2*pi*frequency/299792458)*distance`. Same value, overflow-safer association for extreme frequency*distance products. No action.
4. **P3 / info - `emc.short_cable_radiation` electric_field_rms regrouped** (`backend/catalog/emc.json:690`) to `2*pi*1e-7*common_mode_current_rms*(cable_length*frequency/distance)`. Same value within tolerance; `cable_length*frequency` is bounded <=3e7 by the admitted l/lambda<=0.1 domain, so no overflow path exists. No action.

`docs/features/emc.md` matches the research text and catalog semantics verbatim, including all model-boundary and no-compliance caveats. Diagram bindings use the exact catalog keys; quantity meanings verified against the model definitions.

## Pixel findings (native captures)

Topology, pins, junction dots, polarity and reference directions verified in pixels for every diagram:

- rc_common_mode: single common source bus feeding both rails; R1/R2 in-line with wire breaks at bodies; C1/C2 shunt to the outer return loop that wraps outside the bridge (no rail short, no ambiguous crossing); Cd vertical between node junctions with dots; receiver terminals open with `+ V1` / `- V2` matching Vdiff=V1-V2. Clean.
- trapezoid: half-height dashed level with dots at the two 50% crossings; `D/fs` bracket between them with `duty_half_height` and `switching_frequency` bound as separate legend keys (no duration bound to the dimensionless duty); full-edge `tr (0-100%)` bracket on the rise; current arrow into R with return; static conceptual spectral line, not a computed height. Clean.
- dither: inset waveform about dashed fs centerline with df arrow center-to-peak (peak, not peak-to-peak); two static outlined bands with descriptive (not live-bound) center labels; `occupied_bandwidth` spanning band one's full width; `next_harmonic_gap` between adjacent edges; signed-overlap caveat in caption. Band widths are mutually proportional (160 px : 210 px = 64 kHz : 84 kHz); static per spec. Clean.
- snubber: Lp upstream in the ringing rail; switch node dotted with Cp shunt to return; Rs-Cs series branch tapped on the same switch-node net down to the return (electrically across the node); identification inset shows Ctest directly across Cp marked "Test only; remove before Rs-Cs"; f1 ring visibly slower than f0. Clean.
- cable: vertical cable with l dimension line and end ticks; Icm arrow parallel, pointing up; dashed broadside ray to observer with perpendicularity mark; E arrow vertical (parallel to cable, perpendicular to ray) at the observer; no ground image. Clean.
- thin_slot: face, long horizontal slot, wall-thickness inset, L/w/t dimensions, legend and both output labels all legible, no clipping, width clearly distinct from thickness. One P3 note below.

### F-EMC-1 / P3 — thin-slot field arrows originate/terminate over the implied metal bands

- ID: `emc.thin_slot_shielding`; `frontend/lib/diagrams/emc.dart:18-19`.
- Evidence: the slot opening spans y=115..140. The incident arrow `Wire([250,157,310,122])` starts 17 px below the slot bottom edge (inside the metal band y=140..200) and the transmitted arrow `Wire([310,122,370,101])` ends 14 px above the slot top edge (inside the band y=65..115). The diagram's own caption and the research mandate say "arrows pass through the opening, not through metal" / "Incoming and transmitted field arrows cross the slot, not the solid metal"; the arrow tails visibly overlap solid metal. Visible in `coordination/expansion/screenshots/emc.thin_slot_shielding.png`. Non-blocking: the shafts do traverse the opening and no quantity or topology is misrepresented.
- Correction (minimal): confine both arrows to the slot interior, e.g.
  `Wire([235,138,300,121], arrow: true, ink: DiagramInk.input)` and
  `Wire([300,121,365,118], arrow: true, ink: DiagramInk.output)`, then recapture.

No other text/geometry collision, clipping, arrow, polarity or orientation defect found in any of the six captures.

## Verdicts per ID

| ID | Source verdict | Pixel verdict |
|---|---|---|
| emc.thin_slot_shielding | PASS | PASS with F-EMC-1 (P3, cosmetic arrow tails over metal) |
| emc.rc_common_mode_conversion | PASS | PASS |
| emc.trapezoid_conducted_harmonic | PASS | PASS |
| emc.sinusoidal_dither_occupancy | PASS | PASS |
| emc.ringing_rc_snubber | PASS | PASS |
| emc.short_cable_radiation | PASS | PASS |

## Overall gate: PASS

All six images present and viewed; all six models independently verified; catalog discrepancies are equivalent conditioning refactorings, reported not normalized; the single pixel note is P3 and non-material. No numerical regression exists to hand to Main/T2EmcRun — the shared bus records the clean verification and the exact unbalanced-R cross-check so nobody re-chases the Cd-cancellation algebra.

## Honest untested scope

- Arithmetic verification is independent binary64 recomputation and a complex 2x2 nodal solve, not an executed engine; the tester lane and Main own executed suites.
- Captures were reviewed as supplied composited images per Main's receipt; the desktop app was not launched and captures were not regenerated.
- Caption/title strings are reviewed in source only; the captures show the 700x280 canvas area without the caption strip.
- Primary-source PDFs (Laird, TI x3, MIT) were not re-fetched this session; equations were checked against the research quotations and known physics, not against the original documents.
- Closed oscillator/passives lanes were not re-reviewed; engine input-validation plumbing (exclusive_min handling, NaN/inf rejection) is outside this lane's diff.
- F-EMC-1's proposed replacement coordinates are derived from source geometry and inspected pixels; the recapture itself is Main's to execute.
