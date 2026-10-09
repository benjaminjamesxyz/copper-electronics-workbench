# V2BatteriesRun — source and native-pixel review at 2cca03e

- Snapshot reviewed: `2cca03e854eb6842be56411a9153b6ca1b251a42` (verified by `git rev-parse HEAD` in the integration root at review time).
- Scope: `backend/catalog/batteries.json`, `frontend/lib/diagrams/batteries.dart`, `docs/features/batteries.md` against `research/expansion/batteries.md`, `research/expansion/batteries.json`, and BACKLOG items 029–034. Six required IDs, all reviewed. Closed oscillator/passives lanes not touched; RF Fresnel defect belongs to the RF lane, not reproduced here.
- Reviewed production read-only. No builds, tests, lint, formatters, project process runs, or native launches performed. All arithmetic below is independent binary64 recomputation from the published expressions, not engine output.

## Native images actually viewed (all six, individual image reads)

1. `coordination/expansion/screenshots/batteries.series_weakest_cell.png`
2. `coordination/expansion/screenshots/batteries.coulomb_state_cycle.png`
3. `coordination/expansion/screenshots/batteries.peukert_runtime.png`
4. `coordination/expansion/screenshots/batteries.kinetic_charge_recovery.png`
5. `coordination/expansion/screenshots/batteries.intercalation_regular_solution.png`
6. `coordination/expansion/screenshots/batteries.sand_depletion_limit.png`

All six files existed and were opened as PNG images; none missing.

## Independent numerical verification (source gate)

Every complete catalog example was recomputed from the catalog's own expression forms with separate Python binary64 arithmetic (never loading catalog outputs as input to the check):

| ID | Worst output relative error vs catalog expected |
|---|---|
| batteries.series_weakest_cell | 8.4e-16 |
| batteries.coulomb_state_cycle | 1.9e-16 |
| batteries.peukert_runtime | 0 (exact) |
| batteries.kinetic_charge_recovery | 1.9e-16 |
| batteries.intercalation_regular_solution | 0 (exact) |
| batteries.sand_depletion_limit | 0 (exact) |

All are float representation noise, ~10 orders inside the contractual 1e-6 relative tolerance. Structural identities also re-derived: Peukert at the rated point returns exactly `Hr`/`Cr` for every allowed `n`, and `n=1` returns the ideal law; KiBaM `q1f+q2f−(q1+q2−It)` conserves to 9.1e-13 on the nominal vector and the rest-only case reproduces the research oracle `3090.2376727811948 / 4909.762327218806`; intercalation at `x=0.5` returns `midpoint_voltage` exactly with slope `−(4RT−2Ω)/F`; Sand scaling laws (j→2j: t/4, qs/2, ℓ/2) hold in the closed form. The stable-form requirements hold in source: `-expm1(-x)`, `exprel(-x)`, and `x*expm1mx_ratio(-x)` have the documented analytic limits, so no `1-exp` cancellation exists anywhere in the shipped expressions.

Resumption lead check: the bus line referring to open "batteries numerical findings" awaiting reviewer reports is **not reproducible at 2cca03e** — all six nominal vectors and the checked limit identities agree with independent arithmetic. No numerical regression exists in this lane at this snapshot; nothing to hand to T2BatteriesRun as a defect.

## Source dispositions per ID

- **batteries.series_weakest_cell — PASS.** Catalog is field-identical to the research JSON (inputs, bounds, expressions, constraints, example). Constraints enforce `soc_i >= soc_floor` for all three cells; a cell exactly at the floor yields zero deliverable charge without clamping. Units coherent (Ah, fractions).
- **batteries.coulomb_state_cycle — PASS.** Matches research field-for-field. Both coupled constraints present (intermediate exhaustion rejected even though recharge would restore SOC; overcharge rejected). `0 < eta <= 1` bounds correct; zero durations/currents admitted.
- **batteries.peukert_runtime — PASS.** Matches research; empty constraint list is intentional (research specifies no catalog constraint). `n >= 1` inclusive, exclusive-zero currents/durations; no fake infinite runtime at zero current.
- **batteries.kinetic_charge_recovery — PASS.** Output expressions use exactly the conditioned stable forms from research, including the `x*expm1mx_ratio(-x)` identity. Constraint 3 is algebraically `h1 <= h2`; constraint 4 is the input-expression-only exhaustion guard with `>=` (exact end-of-interval exhaustion admitted, consistent with research). `lambda = 0` and `t = 0` limits are admissible and exact.
- **batteries.intercalation_regular_solution — PASS.** Global phase guard `Omega < 2RT` present (deliberately stronger than local curvature); endpoint occupancies excluded via exclusive bounds; `ln(x)-log1p(-x)` form present. Capacitance expression equals `−Qmax/dV/dx` (verified numerically, exact).
- **batteries.sand_depletion_limit — PASS.** Expression is algebraically the Cambridge eq. (31) form `(pi D/4)[F c0/(j(1-t+))]^2` (equal to machine precision); `0 <= t+ < 1` bounds match the singular-endpoint exclusion; constraints intentionally empty with the finite-cell caveat documented in assumptions/docs.

Catalog vs research JSON: no field differs on any of the six. `docs/features/batteries.md` carries the research text and oracles verbatim per section, including sources and illustration notes consistent with the rendered diagrams. Registry wiring confirmed: `batteriesDiagrams` spread into the diagram catalog (`frontend/lib/diagrams/catalog.dart:56`), one entry per exact ID, no duplicates.

Diagram quantity bindings: all 53 `quantity:` bindings in `batteries.dart` resolve to existing catalog input/output keys; zero invented keys; plain explanatory labels (`1−η` loss branch, `Recovery flow = λ[c q2−(1−c)q1]`, `qs = j ts (C/m²)`, `I × runtime (Ah)`, `ℓ = √(D ts)`) carry no quantity binding, per the research contract. Input/output ink assignments match each key's direction.

## Pixel dispositions per ID (post-fix recaptures)

- **series_weakest_cell — PASS.** Three plate-pair cells in a single series chain; every cell drawn with the long plate on the right and `−`/`+` markers above the short/long plates respectively (caption states the convention). Adjacent plates only are connected; no shorts, no missing links. Single rightward `Q used` arrow bound to `usable_charge`. Three symbolic SOC columns with static fill lines, one common dashed floor bound to `soc_floor`, `s1/s2/s3` bindings above, `Cell 1/2/3` end-state callouts at right. No collisions or clipping observed.
- **coulomb_state_cycle — PASS.** Left-to-right state flow Initial→Discharge→Recharge→Final; discharge precedes recharge. `Id/td` over discharge, `Ic/tc` over recharge, `sd` between stages, `sf` at final block. Downward muted loss branch from recharge with plain-text `Unretained fraction 1−η`; only `η` is bound to `charge_efficiency`. `Retained recharge (Ah)` and `Capacity (Ah)` bound correctly. No overlaps; all text inside the canvas.
- **peukert_runtime — PASS with P3 (F-BAT-1 below).** Log-log axes labeled, fitted solid line steeper than the dashed ideal −1 reference (correct relative slopes for `n>1`), both slope labels present, rated point marked on the fitted line with `rated_current`/`rated_duration` labels, requested point marked with `runtime`/`load_current` labels, right column binds `peukert_exponent`, `rated_capacity` (+ `Cr (Ah)`), `delivered_capacity` (+ `I × runtime (Ah)`, dimensionally correct). No collisions or clipping.
- **kinetic_charge_recovery — PASS.** Two wells with correct relative widths (`1−c` plain, `Width c` bound) and schematic fill heights consistent with `h2 > h1`; base passage with rightward recovery arrow (bound→available, correct for discharge/rest states); `λ (1/s)` bound at the passage with the explanatory flow formula; load arrow leaves only the available well, bound to `pulse_current`; `Interval t (s)` and `Combined capacity` in the top band; `q2/q1 initial` and `q2/q1 final` callouts bound; `Total final (C)` bottom right. No collisions or clipping.
- **intercalation_regular_solution — PASS.** Lattice with occupied (dotted) and vacant sites; Li+ arrow and separate e− lead both terminate at the same vacancy, matching the reaction text and caption; Ω bracket spans an adjacent site pair and is bound to `interaction_energy` (not to a voltage arrow); `T (K)` top band; `Filling x` below lattice. Right side: descending single-phase V–x curve with correct end-steepening, dashed `Vmid` level meeting the curve at the marked dot labeled `Vmid at x = ½`, tangent segment touching the curve exactly at that dot and labeled `dV/dx`, `Equilibrium V` as a separate callout. Bottom band binds `full_insertion_charge` and `chemical_capacitance`. No collisions or clipping.
- **sand_depletion_limit — PASS.** Vertical Li slab with `x = 0` interface label; separate e− lead into the metal (unbound, correct); leftward Li+ arrow in the electrolyte pointing into the interface, bound to `plating_current_density`; `D (m²/s)` and `t+ fraction` above; concentration axes with dashed initial `c0` profile bound to `salt_concentration` and a depleted profile rising from zero at the interface toward bulk (physically correct shape and direction); diffusion-length dimension line bound to `diffusion_length`; right column binds `sand_time` and `sand_areal_charge` (+ plain `qs = j ts (C/m²)`). No collisions or clipping.

## Findings

- **F-BAT-1 / P3 / `batteries.peukert_runtime` / `frontend/lib/diagrams/batteries.dart:82-84` — schematic curves do not share the marked rated point.** Evidence (source + pixels): the fitted line and the dashed ideal −1 reference both start at logical (115,65), so they visually intersect there, but the marked rated point `Dot(165,85)` lies only on the fitted line; the ideal line passes ≈17 logical px above it. Physically both curves must coincide at the rated point (at `Ir = Cr/Hr`, Peukert and ideal runtime both equal `Hr`). Impact: minor misleading geometry in an explicitly schematic plot; no quantity misbinding; caption already disclaims computed positions. Minimal correction (optional, non-blocking): originate both curves at the rated point, e.g.
  `Wire([165, 85, 440, 198], ink: DiagramInk.output),` and `Wire([165, 85, 440, 148], dashed: true, ink: DiagramInk.muted),` then recapture `batteries.peukert_runtime.png`.

No other findings. No P0/P1/P2 issues in source or pixels for this lane.

## Verdict

- Source verdict: **PASS 6/6** — catalog, diagrams source, and docs match research and backlog; numerics independently confirmed at 2cca03e.
- Pixel verdict: **PASS 6/6** — all six post-fix recaptures inspected as images; topology, polarity, reference directions, bindings, and text/geometry layout correct; one non-blocking P3 schematic-geometry note (F-BAT-1).
- Lane gate: **PASS.** F-BAT-1 is advisory; no repair is required for the gate. If Main adopts the one-line curve re-anchor, only `batteries.peukert_runtime.png` needs recapture.

## Untested scope (honest)

- No builds, tests, engine execution, or Flutter runs: runtime engine behavior (malformed-input recovery, actual Rust evaluation, accessibility legends, narrow-window layout) was not exercised here and remains Main's process acceptance.
- Diagram titles/captions were reviewed in source and in full-canvas captures; no zoomed crops or color-contrast measurements were taken.
- Invalid-domain behavior was checked by expression/constraint analysis, not by execution.
- The earlier bus mention of open "batteries numerical findings" could not be reproduced at 2cca03e from any expression or oracle I could reconstruct; if a concrete counterexample exists, it predates or postdates this snapshot and needs a reproducing vector.
