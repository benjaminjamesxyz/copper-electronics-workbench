# V2MotorsRun — source and native-pixel review of the six motors calculators

- **Snapshot:** production root `/home/simson/pi-projects/pcb-design-calculator`, HEAD `2cca03e` (verified via `git rev-parse`), post-fix recapture set in `coordination/expansion/screenshots/`.
- **Scope:** `backend/catalog/motors.json`, `frontend/lib/diagrams/motors.dart`, `docs/features/motors.md`, each against `research/expansion/motors.md` / `motors.json` and backlog items 077–082. Diagram registry merge checked at `frontend/lib/diagrams/catalog.dart:64` (`...motorsDiagrams`). Cross-lane scope sweep over all `backend/catalog/*.json` found no second motor/torque/slip/stepper/brake/gear item: the six IDs are physically distinct from each other (steady DC equilibrium, static stepper stiffness, geared ramp sizing, braking transient, salient PMSM dq operating point, induction slip power split) and from every baseline/exclusion item named in research.
- **Method:** read-only source analysis plus independent arithmetic (all six catalog examples recomputed from the physical laws in Python's `math`; all reproduced inside relative `1e-6`, including the slip-multiplied induction form and the `exprel`/`-expm1` braking forms). Every native PNG below was opened as an image and inspected; no builds, tests, lint, formatters, engine runs, processes or native launches were performed by this reviewer.

## Images actually viewed (all six, post-fix recaptures)

| ID | Image path actually viewed |
|---|---|
| `motors.pm_dc_load_equilibrium` | `/home/simson/pi-projects/pcb-design-calculator/coordination/expansion/screenshots/motors.pm_dc_load_equilibrium.png` |
| `motors.stepper_static_microstep` | `/home/simson/pi-projects/pcb-design-calculator/coordination/expansion/screenshots/motors.stepper_static_microstep.png` |
| `motors.geared_axis_acceleration` | `/home/simson/pi-projects/pcb-design-calculator/coordination/expansion/screenshots/motors.geared_axis_acceleration.png` |
| `motors.resistive_dynamic_braking` | `/home/simson/pi-projects/pcb-design-calculator/coordination/expansion/screenshots/motors.resistive_dynamic_braking.png` |
| `motors.salient_pmsm_dq_operating_point` | `/home/simson/pi-projects/pcb-design-calculator/coordination/expansion/screenshots/motors.salient_pmsm_dq_operating_point.png` |
| `motors.induction_slip_power` | `/home/simson/pi-projects/pcb-design-calculator/coordination/expansion/screenshots/motors.induction_slip_power.png` |

## Per-ID source disposition

All six IDs: equations, signs, reference directions, unit bindings and constraint sets match the research handoff and the backlog anchors.

1. **`motors.pm_dc_load_equilibrium`** — catalog expressions identical to research; constraint `motor_constant*voltage >= armature_resistance*load_torque` correctly admits the stall boundary; `current=(load_torque+viscous_drag*speed)/motor_constant` avoids the near-equal voltage subtraction as prescribed. Independent recomputation of the example closes the 88 W power balance. **Source PASS.**
2. **`motors.stepper_static_microstep`** — stable-branch `asin` lag and split-root stiffness `(S/4)*sqrt(Th−T)*sqrt(Th+T)` are cancellation-safe; integer guards for `S` multiples of four and integer `m` present; `Tload<Th` strict. **Source PASS.**
3. **`motors.geared_axis_acceleration`** — reflected inertia `Jl/n²` (efficiency correctly excluded), gear-input torque `(Jl·α+Tload)/(η·n)`, end-of-ramp voltage `K·n·ωf+R·I` all match research; constraint set (positive inertia sum, nondecreasing ramp, `0<η≤1`) correct. **Source PASS.**
4. **`motors.resistive_dynamic_braking`** — signed torque `−K·i`, `tau=J·Rt/K²`, `exprel` travel and `−expm1` energy partition by resistance fraction all match; input minimums alone suffice (no missing constraint); `Rb=0` short-circuit limit admitted as intended. **Source PASS.**
5. **`motors.salient_pmsm_dq_operating_point`** — d/q voltage, saliency torque, `hypot` peaks and copper loss via the current magnitude match the MathWorks convention; power identity `1.5(vd·id+vq·iq)=Pconv+Pcu` holds symbolically and numerically; signed generation retained. **Source PASS.**
6. **`motors.induction_slip_power`** — slip-multiplied algebra gives exact zeros at `s=0` (no division by slip), `Pcu2=s·Pag`, `(1−s)·Pag` split, unconstrained breakdown pair with honest "can exceed one" assumption text; `Xth+X2>0` and slip∈[0,1] guards present. **Source PASS.**

Example-value note (non-blocking): three catalog `expected` values differ from the research/docs prose by one final-digit ulp — `speed` `166.66666666666666` vs docs `…667`, `electromagnetic_torque` `−0.09196986029286058` vs `…059`, `rotor_copper_loss` `761.7049180327868` vs `…9`. All are 1-ulp reorderings of the same arithmetic, inside the declared `1e-6` relative tolerance; no action needed.

## Per-ID pixel disposition

1. **`motors.pm_dc_load_equilibrium`** — Both source symbols render with positive top terminals; current arrow runs along the upper lead toward the motor and enters the back-EMF source's positive terminal; supply loop closes at `y=210`; dashed mechanical coupling is visually distinct from wiring; rotor circle, shaft, load block, `omega` arrow right, opposing `Tload` and `b omega` arrows left. All labels legible and collision-free. **Pixel PASS.**
2. **`motors.stepper_static_microstep`** — Rotor circle with command-field arrow leading the rotor axis and a dashed `load lag` marker between them; lower-left ray pair separated by a conceptual microstep; right-hand quarter-sine plot rises from the origin, flattens at the `Th` limit, carries a conceptual operating dot with dashed local tangent and the `local stiffness` label clear of curve/tangent. No clipping or overlap. **Pixel PASS.**
3. **`motors.geared_axis_acceleration`** — Three outlined blocks with internal labels readable; shaft arrows between blocks; `omega m`/`omega l` arrows above both shafts; `Tload` opposing below the load block; dashed `J reflected` annotation with upward arrow at the motor block; relation text and `Positive ramp` note inside the canvas. **Pixel PASS.**
4. **`motors.resistive_dynamic_braking`** — Closed series loop J–generator–Ra–Rb–return with `+` top / `−` bottom on the generator; `Ig` arrow leaves the positive terminal and matches top-wire direction; dashed `Ea`/`Eb` heat arrows under their own resistors with the "heat, not electrical leads" note; `omega(t)` right vs `Te ≤ 0` left; "Supply disconnected" annotation is clearly not a conducting path. **Pixel PASS.**
5. **`motors.salient_pmsm_dq_operating_point`** — d right / q up axes, permanent-flux arrow on +d, illustrative current vector in quadrant II with dashed projections to `id`/`iq`; both voltage-equation blocks render unclipped; energy-flow arrow with split dot to `Pcu` and onward to `Pconv`, captioned as energy not wiring. **Pixel PASS.**
6. **`motors.induction_slip_power`** — Per-phase loop Vth–Rth–Xth–X2–R2/s with plain `R2/s` body text and unbound `R2`/`s` quantity labels beneath it; `I2 rms` arrow inside the loop in the correct direction; lower power-flow split dot branches to `Rotor heat: Pcu2` and `Mechanical: Pconv` blocks with `s Pag` / `(1−s) Pag` relations, spatially separated from the electrical return. **Pixel PASS.**

## Findings

No blocking (P0–P2) source or pixel findings. All six diagrams match their research topology/polarity/label-binding specs; every quantity label binds an exact catalog input/output key (checked key-by-key, including the deliberately unbound `R2/s`, `b omega`, `Th` flat-top and plain relation texts).

Non-blocking observations (recorded for the record; no repair required, none misleads):

- **O1 / P3** — `frontend/lib/diagrams/motors.dart:62`: the stepper `Tload` label floats in the plot interior near the torque-axis top rather than at the conceptual operating dot's level; the flat `Th` limit and the dot are separately and correctly marked. Purely positional taste; no collision, no wrong binding.
- **O2 / P3** — `frontend/lib/diagrams/motors.dart:65` (spec note): research suggested a plain `delta` marker on the lag arrow; the rendered dashed lag arc plus the bound `load lag` label carries the same meaning unambiguously.
- **O3 / P3** — `frontend/lib/diagrams/motors.dart:76`: geared-diagram `I` label sits ~55 px left of its intake arrow (`V end` occupies the adjacent slot); the single downward arrow makes the binding unambiguous.
- **O4 / P3** — research asked for explicit acceleration arrows above both geared shafts; the shipped diagram conveys ramp direction via `Positive ramp` text plus the `alpha m = n alpha l` relation. No false physics results.

## Numerical-regression check (for Main and T2MotorsRun)

**No numerical regression found in the six motors models.** Targeted conditioning analysis: pm_dc denominator `K²+R·b ≥ K² > 0` (no stall singularity; equality admitted by design); stepper split-root stiffness has no `Th²−T²` cancellation and strict `T<Th` keeps `asin` in domain; braking uses `exprel`/`−expm1`, so `t=0` and `t→0⁺` retain representable travel/energy (no `1−exp(−x)` loss), and `Rb=0` is exact; PMSM `hypot` peaks are overflow-safe in any admitted input regime with meaningful outputs; induction's slip-multiplied form returns exact zeros at `s=0` and never divides by slip, and the breakdown pair is slip-independent. Extreme-input overflow modes (e.g. `V=1e308`) exist only where the physical model itself is meaningless, so they are not defects under the no-arbitrary-bounds contract.

## Verdict

- **Source verdict:** PASS (6/6 IDs).
- **Pixel verdict:** PASS (6/6 images viewed at the paths above).
- **Overall gate:** **PASS** — no missing image, no material open finding for this lane.

## Honest untested scope

- No engine execution: example reproduction is independent arithmetic, not a Rust run; Main's suite remains the executable gate (Main already reports all 228 examples passing at this HEAD).
- Pixel inspection covers the captured canvas only: caption/title wrapping, narrow-window/large-text accessibility and interactive behaviors are outside these captures.
- Hardware calibration, ratings, thermal limits and datasheet conformity are explicitly out of model scope (documented assumptions), not verified here.
- Observations O1–O4 were not recaptured/changed; no production edit was made or requested.
