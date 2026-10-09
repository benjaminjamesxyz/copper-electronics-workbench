# Protection lane review — V2Protection

Snapshot: **2cca03e** (production root `/home/simson/pi-projects/pcb-design-calculator`, read-only).
Scope: 6 IDs — `protection.fuse_pulse_coordination`, `protection.tvs_surge_load_line`,
`protection.mov_power_law_pulse`, `protection.scr_crowbar_trigger_latch`,
`protection.copper_adiabatic_fault_limit`, `protection.foldback_fault_envelope`.
Reviewed: `backend/catalog/protection.json`, `frontend/lib/diagrams/protection.dart`,
`frontend/lib/diagrams/primitives.dart`, `docs/features/protection.md`,
`research/expansion/protection.md`, `research/expansion/protection.json`,
`coordination/expansion/BACKLOG.md` entries 035–040.

## Images actually viewed (native Flutter captures, composited on paper)

All paths opened with the image reader; geometry inspected against the 700x280 logical marks.

- `coordination/expansion/screenshots/protection.fuse_pulse_coordination.png`
- `coordination/expansion/screenshots/protection.tvs_surge_load_line.png`
- `coordination/expansion/screenshots/protection.mov_power_law_pulse.png`
- `coordination/expansion/screenshots/protection.scr_crowbar_trigger_latch.png`
- `coordination/expansion/screenshots/protection.copper_adiabatic_fault_limit.png`
- `coordination/expansion/screenshots/protection.foldback_fault_envelope.png`

No missing image; all six render at the integration snapshot.

## Source verdict — PASS (no material source defect)

Independent arithmetic (Python double precision, reviewer-executed, no engine run) reproduces every
catalog example for all six IDs within last-ULP differences, far inside `relative_tolerance 1e-6`.
Examples: fuse `pulse_i2t=1.0666666666666667`; TVS `28/44/1232/16/7`; MOV `beta=0.0484550065040282`,
`clamp_voltage=357.77087639996637` (geometric-mean identity `sqrt(320*400)` holds); SCR
`13.438/0.04647272727272727/1.68/1.63/1.4`; copper `material_k=143.1744142046674`
(=226*sqrt(ln(484/324)), consistent with Schneider k≈143); foldback
`0.16/5/0.5882352941176471/0.29411764705882354/.../4.2025`. Research `.md`, research `.json`,
BACKLOG anchors and catalog `expected` maps agree for all six IDs; no source/data divergence.

- Units and dimensions check in all six formulas; margins keep sign (failed designs report negative
  margins rather than being rejected) — correct per research contract.
- Domains: fuse total duration in (0, 10 ms) and allowance in (0,1]; TVS fit interval and positive
  avalanche (off-avalanche `surge_voltage=30` rejects); MOV `I2>I1>0`, `V2>V1>0`, `0<beta<1`, pulse
  inside `[I1,I2]`; SCR `Ih<=IL`, `Von<Vz`, source >= trigger requirement; copper
  `0<=Ti<Tf<=250`, `0<t<=5 s`; foldback `Vin>Vk`, `Ik>Isc`, `R<=Vk/Ik`. All match research scope.
- Conditioning is deliberately good: `log1p` ratio forms (MOV, copper), square-root-scaled fuse
  pulse, foldback normalized denominator `(1-r)+r(Isc/Ik)` (at `R=Vk/Ik`, `Isc=1e-20` it returns the
  exact knee `(5 V, 1 A)` without cancellation), clamped concave maximum for pass stress.
- Physics spot-checks pass: SCR KVL `Vtrip=Vz+Vg+(Rs+Rg)(Igt+Vg/Rgk)` with zener/shunt loading and
  post-turn-on branch collapse (`Von<Vz`); TVS load-line intersection satisfies KVL `Vs=RsI+Vclamp`;
  copper invariant `Iallow^2 * t = (kS)^2` holds; foldback vertex `(Vin-Isc/b)/2` is the true
  interior maximum of `(Vin-V)(Isc+bV)`, and the example correctly shows `4.2025 W > 4 W` knee
  `> 1.8 W` short.
- Diagram semantics vs source: all mandatory quantity bindings present and correct
  (`peak_current`, `pulse_i2t`, `pre_arcing_i2t`, `clearing_i2t`, `device_withstand_i2t`;
  `surge_voltage`, `source_resistance`, `surge_current`, `clamp_voltage`, `breakdown_intercept`,
  `dynamic_resistance`; `pulse_current`, `pulse_duration`, MOV endpoint keys, `voltage_exponent`,
  `absorbed_energy`; SCR full bind list; `fault_current`, `area_mm2`, `initial_temperature`,
  `limit_temperature`, `clearing_time`, `material_k`, `minimum_area_mm2`; foldback full bind list).
  Polarities/reference directions correct: TVS K(+)=protected node, A(-)=return, avalanche K to A;
  MOV source arrow and +/- consistent; SCR zener cathode toward Rg/node A, anode toward G, Rgk G to
  K with explicit junction dots, anode current A to K; foldback pass input +, output -, load line
  rising from origin.

## Pixel verdict — HOLD (1 material defect, 2 advisory)

Findings (severity / ID / file:line / evidence / minimal correction):

1. **P2 — fuse label clipped at canvas edge** — `protection.fuse_pulse_coordination` —
   `frontend/lib/diagrams/protection.dart:29` — in
   `coordination/expansion/screenshots/protection.fuse_pulse_coordination.png` the right-panel block
   label renders as `Semi devic...` with the final glyph cut by the right edge of the capture
   (`Label(590, 90, 'Semi device')` is ~110 logical px wide at size 14, overflowing both the 92 px
   block (578-670) and the 700 px canvas). Other five captures show no clipping. Correction — match
   the one-word style of `Source`/`Fuse` and center in the block:
   ```dart
       Label(594, 90, 'Device'),
   ```
2. **P3 — TVS line label association** — `protection.tvs_surge_load_line` —
   `frontend/lib/diagrams/protection.dart:64` — in
   `coordination/expansion/screenshots/protection.tvs_surge_load_line.png` the unbound `Source`
   annotation sits at (586,83), ~18 px from the ascending avalanche line's top end and ~65 px from
   the descending source line it names, so it visually attaches to the TVS line. Not a data error
   (`dV/dI = rd` and `Vb` bind correctly; ink colors disambiguate). Minimal correction: move it to
   the upper-left end of the source line, e.g. `Label(455, 55, 'Source')`.
3. **P3 — foldback stress-label proximity** — `protection.foldback_fault_envelope` —
   `frontend/lib/diagrams/protection.dart:204` — in
   `coordination/expansion/screenshots/protection.foldback_fault_envelope.png` `Stress V*` at
   (482,166) is ~32 px from the Fault intersection dot but ~110 px from the stress locator dot
   (572,102) it names (`I*` sits by the locator). Association is recoverable via the `*` pairing,
   but the layout invites misreading the fault point as the worst-stress point. Minimal correction:
   move it clear of the fault dot without hitting `I*` at (582,128), e.g. `Label(585, 88, 'Stress
   V*')` above `I*`, or shift `I*` down and place the label beside the locator.

Topology/pin/polarity pixel checks: wires connect only where intended (junction dots at SCR gate
node (480,155), Rgk-return (480,245), TVS protected node (300,70), foldback output node (285,85));
no accidental shorts or open circuits; no forward-diode mis-orientation (blocks used where
primitives lack SCR/TVS/MOV glyphs); dimension brackets (Vclamp spans, tpulse, tclear, S) are
detached from wires, so no false connections; graph intersections are geometrically consistent
(TVS dot at the actual line crossing (542,134); foldback Fault dot lies on both branch and load
line; stress locator on the branch); no text/line collisions in the other captures.

## Gate

**HOLD** — source PASS; pixels blocked only by finding 1 (visible label truncation in a delivered
capture). Findings 2-3 are advisory and do not gate. After Main applies the line-29 correction and
recaptures `protection.fuse_pulse_coordination.png`, this lane is PASS without re-review of the
other five images; optionally fold the two P3 label moves into the same edit.

## Untested scope (honest)

- No build, test, engine run, or desktop launch performed by this reviewer; example reproduction was
  independent scalar arithmetic, not the Rust evaluator.
- Diagram `title`/`caption` strings are not visible in the captures (canvas-crop); their text was
  reviewed in source only, including the SCR caption's `Von < Vz` statement.
- Numerical conditioning beyond the catalog examples (tester's `1e-20` short-current probe, fit
  endpoints, 10 ms/5 s/250 degC boundaries) was verified analytically from the expressions, not
  executed.
- Primary sources P1-P7 were accepted from the research handoff's read evidence (URLs and section
  claims internally consistent); no re-fetch, except that the copper constants were cross-checked
  against the independently stated Schneider k≈143 identity, which holds.
