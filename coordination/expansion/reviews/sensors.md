# V2SensorsRun — source and native-pixel review of the sensors expansion lane

- Snapshot reviewed: production `2cca03e` (verified via `git rev-parse --short HEAD` at review time). Read-only; no production file was edited, no build/test/lint/formatter/process/native launch was run.
- Scope: `backend/catalog/sensors.json`, `frontend/lib/diagrams/sensors.dart`, `docs/features/sensors.md`, each checked against `research/expansion/sensors.md`, `research/expansion/sensors.json` and the `coordination/expansion/BACKLOG.md` sensors lane entries (001–007). Closed oscillators/passives lanes were not re-reviewed.
- Pixels: every one of the seven native captures below was opened and inspected as an actual image (running-Flutter canvas composited on paper, geometry unchanged), per the Main dispatch stating these are the post-fix recaptures. SHA-256 recorded per file as viewed evidence.

## Images actually viewed (all seven required IDs)

| ID | Image path (viewed) | SHA-256 |
|---|---|---|
| sensors.ntc_steinhart_hart | `coordination/expansion/screenshots/sensors.ntc_steinhart_hart.png` | `167a670faf2f9beb8a10fbc67907b1362dfb0be00fed75f91d2da677c5dd9ee3` |
| sensors.platinum_callendar_van_dusen | `coordination/expansion/screenshots/sensors.platinum_callendar_van_dusen.png` | `551c0cba8ebdbdb77048b259e2b4d7774b05b2e54ee231a4a97b6291004bb0c3` |
| sensors.type_t_cold_junction_emf | `coordination/expansion/screenshots/sensors.type_t_cold_junction_emf.png` | `0c886f641dbd338a7c283744b44c6edc8021ce85b635852cf098f9de436d1024` |
| sensors.hall_plate_angle | `coordination/expansion/screenshots/sensors.hall_plate_angle.png` | `8c678e0cafd4a19513f527bd9b1ee1cc0a8106a7789308d77837e215d059a102` |
| sensors.amr_angular_resistance | `coordination/expansion/screenshots/sensors.amr_angular_resistance.png` | `e3af79d0981161f07e96c33827a78a251e6fa13e1f889ec10b9f6768bf35a772` |
| sensors.hydrogen_activity_electrode | `coordination/expansion/screenshots/sensors.hydrogen_activity_electrode.png` | `3eeffe630f62ad5fee2235c6450065e06960d757ac9daaaaddeff3c21d6d8240` |
| sensors.differential_gap_capacitor | `coordination/expansion/screenshots/sensors.differential_gap_capacitor.png` | `e438662d6cf59c536203c2c77a2999bf13b88d69875bad039fb93c9bb670647b` |

## Per-ID source verdicts

All seven catalog examples were re-derived independently by hand from the research laws (no engine call); every stored expected output reproduces within hand precision, and each matches the research oracle and the backlog numerical anchor exactly.

1. **sensors.ntc_steinhart_hart — source: PASS with finding S1.** Forward law, `T*(T/R)*(B+3CL²)` sensitivity form, `B>0, C>=0` family, ordered fit interval and the two reciprocal-temperature bounds all match research and ILX equation (1) under the 1-ohm convention. Example reproduces (D≈0.00335402 → T≈298.14967 K; sensitivity −T²(B+3CL²)/R ≈ −0.00227955). Conditioning defect S1 below.
2. **sensors.platinum_callendar_van_dusen — source: PASS.** Both branches via `u=min(t,0)`; `dR/dt=R0[A+2Bt+C(4u−300)u²]` and direct `linear_model_error` match research; −100 °C example reproduces exactly (60.25584 / 0.4053081 / −0.66116); branch continuity at 0 holds; inclusive −200…850 bounds enforced on the input. No grouping hazard: bounded powers, single leading scale factor.
3. **sensors.type_t_cold_junction_emf — source: PASS.** All nine NIST 0–400 °C coefficients match the SRD 60 block. Horner EMFs, the factored terminal divided-difference (verified complete through the i=8 term at `backend/catalog/sensors.json:253`) and the term-by-term derivative match research. 100/25 °C example reproduces (E(100)=4.2785186 mV, E(25)=0.9919773 mV, sensitivity 46.784961 µV/K). EMF-domain cold-junction semantics and unshorted reference topology documented correctly. No overflow possible inside 0…400 °C.
4. **sensors.hall_plate_angle — source: PASS with finding S3.** `Bn=B cosθ`, `VH=RH·I·Bn/t`, negative angular derivative; signed RH/I admitted, inclusive calibrated limits, zero bias/field valid. pi/3 example reproduces (binary64 `0.02500000000000001` consistent with the documented radian note). Grouping defect S3 below.
5. **sensors.amr_angular_resistance — source: PASS with finding S2.** `R=R⊥+ΔR cos²θ`, passive-sign `V=IR`, `−2ΔR sinθcosθ`, positivity constraint `R⊥+min(ΔR,0)>0`, inclusive current limit — all match research; pi/3 example reproduces (1005.0 exactly survives binary64 since the 5e-15 residue is below the ulp at 1005). Grouping defect S2 below.
6. **sensors.hydrogen_activity_electrode — source: PASS.** Nernst slope from exact SI constants, `pH=−log10 a`, sensitivity `KT/a`; the two-nonnegative-`log1p` identity is algebraically `ln(a/b)` and cancellation-free (verified for a>b, a<b, a=b). Bounded envelope [1e-14,1]×[273.15,373.15] keeps every intermediate finite; example reproduces (0.182478049054347 V). Clean.
7. **sensors.differential_gap_capacitor — source: PASS.** Direct `Cnear`/`Cfar`, factored `2εA·(x/(g−x))/(g+x)`, `x/g` ratio, and `near/(g−x)+far/(g+x)` sensitivity (algebraically `εA[(g−x)⁻²+(g+x)⁻²]`) match research; example reproduces all five outputs including 1.9983409994e-7 F/m. Contact and thin-gap rules with inclusive 10:1 boundary as specified. Clean. (A candidate extreme-scale finding was investigated and withdrawn: for `width=length=1e299 m, rest_gap=1e298 m, x=0` the capacitance intermediates overflow, but `differential_sensitivity` is genuinely non-representable for the same vector, so the engine's non-finite rejection is correct behavior, not a defect.)

Distinct physical scope confirmed: none of the seven collides with the baseline exclusions (linear TCR, Wheatstone, fixed-geometry displacement current, field energy, Shockley) nor with each other; Hall (transverse, odd under field reversal) vs AMR (longitudinal, π-periodic) boundaries are stated and implemented.

## Per-ID pixel verdicts (all seven: PASS)

1. **NTC:** thermistor body at (150,130) with separated sense leads, temperature arrow onto the bead, `T (K)`→`temperature_kelvin`, `R measured`→`resistance`; decreasing conceptual R–T curve with dashed tangent touching at the marked point, `dT/dR`→`temperature_sensitivity` (inverse meaning stated in caption and note); A/B/C legend bound. No clipping, no text/geometry collision, margins respected.
2. **Platinum:** dashed sensor outline, serpentine element with endpoint junction dots; four distinct leads (I+/I− excitation, V+/V− sense) terminating only at the two element junctions — no unintended network; `t (°C)` and `R(t)` bound; curve passes through the dotted `R₀` zero-temperature intercept, dashed zero-tangent diverges from the concave curve, right-edge bracket marks `R − tangent-at-zero model`→`linear_model_error`; `Local dR/dT`→`resistance_sensitivity`. Tangent-line construction misses the intercept dot by ≈3 px, fully hidden inside the dot radius — not visible, not flagged. Clean.
3. **Type-T:** weld dot at (100,140); copper (+, upper) and constantan (−, lower) legs do not reconnect; both termination junctions sit inside the dashed isothermal block; separate wires run to open-circle terminals with `+ Cu` / `− constantan`; `Vterminal`→`terminal_emf` across the open pair; `Th`→`hot_temperature` at the weld, `Isothermal Tc`→`cold_temperature` on the block; bottom legend binds `hot_emf`/`cold_emf` each "ref. 0 °C", never as the terminal signal. Topology is the correct EMF-domain cold-junction circuit. Clean.
4. **Hall:** oblique plate with thickness extrusion; longitudinal I arrow with contacts at the left/right edges; transverse sense contacts on the other two edges running to open `+ sense`/`− sense` terminals — bias and sense nets remain distinct (no dot, no shared node); `+n` normal arrow, tilted B arrow, θ arc between them; thickness bracket `t`; `Bn = B cos θ`→`normal_field`, `VH (+ minus −)`→`hall_voltage`. No short, no collision, no clipping. Clean.
5. **AMR:** long strip with end-contact dots; I arrow above pointing right; `+` left / `−` right with `V = Vleft − Vright`→`voltage_drop` (consistent passive convention for rightward current); internal M arrow with dashed current-direction reference and θ arc→`magnetization_angle`; right legend binds `R perpendicular`, `ΔR = R∥ − R⊥`, `R(θ)`; π-periodicity disclaimer in caption/note. Clean.
6. **Hydrogen:** single vessel, dashed fill line; glass bulb (two leads into a bulb outline) and reference junction (tube with porous frit) both immersed in the same sample; bulb lead to `+ sense`, internal reference lead to `− reference`, `Velectrode`→`electrode_voltage` at the high-impedance meter; `a(sample)`/`T (K)` bound inside the vessel; `Calibration standard activity`→`standard_activity` outside the vessel as required; `Sample pH`/`Nernst slope (V/pH)` legend. The reference lead crosses the (non-electrical) vessel wall outline — expected for an immersed probe, no implied junction. Clean.
7. **Capacitor:** three vertical electrodes at x=150/330/550 with independent outer leads and a dedicated downward center lead; rightward `+x` arrow→`displacement`; `g + x` left / `g − x` right relation labels (neither bound to `rest_gap`); `Cfar` in the left gap, `Cnear` in the right gap — gap-sign naming consistent with positive-x-toward-near; `g at x = 0`→`rest_gap` in a separate legend; `ΔC = Cnear − Cfar`→`differential_capacitance` and `ΔC/(Cnear+Cfar) = x/g`→`normalized_difference`. Clean.

No RF-lane-style dark block, clipping, overlap, mislabel or topology error exists in any of the seven captures. The closed-lane RF ellipse defect is untouched by this lane and remains Main's open item.

## Actionable findings (source; all new in this expansion, none affects any nominal example)

Root class shared by S1–S3: catalog expressions evaluate left-associatively on an RPN stack and the engine rejects on the first non-finite intermediate (`backend/src/checked.rs:49`, message "Model produced a non-finite intermediate value"). A prefix product that overflows therefore fails a request whose exact outputs are all representable. Evidence below is independent source+binary64 analysis, not an executed run; Main owns reproduction.

### S1 — NTC sensitivity overflows `T*(T/R)` before the small coefficient multiplies in
- Severity: P2. ID: `sensors.ntc_steinhart_hart`. File: `backend/catalog/sensors.json:83` (the `temperature_sensitivity` expression, lines 81–86).
- Evidence: inputs `resistance=1, coefficient_a=1e-300, coefficient_b=1e-310, coefficient_c=0, fit_temperature_min=273.15, fit_temperature_max=1e300` satisfy every constraint (L=0, D=1e-300, boundary `D=1/Tmax` inclusive). Exact outputs `temperature_kelvin=1e300`, `temperature_celsius≈1e300`, `temperature_sensitivity=−T²·B/R=−1e290` are all representable, but `temperature_kelvin*(temperature_kelvin/resistance)` = 1e600 → inf → the whole request is rejected.
- Correction (nominal value unchanged, exact same law):
```suggestion
-temperature_kelvin*((coefficient_b+3*coefficient_c*ln(resistance)^2)*temperature_kelvin/resistance)
```

### S2 — AMR angular sensitivity computes `2*ΔR` (→ inf for ΔR=1e308) though the exact result never exceeds |ΔR|
- Severity: P2. ID: `sensors.amr_angular_resistance`. File: `backend/catalog/sensors.json:470` (the `angular_resistance_sensitivity` expression, lines 468–472).
- Evidence: `perpendicular_resistance=1e308, resistance_anisotropy=1e308, magnetization_angle=pi/4, bias_current=1e-5` passes both constraints; exact outputs `resistance=1.5e308`, `voltage_drop=1.5e303`, `angular_resistance_sensitivity=−1e308` are representable, but the leading `2*resistance_anisotropy` = 2e308 → inf → rejection. Since |sinθ·2cosθ| ≤ 1, the regrouped form below keeps every intermediate ≤ max(|ΔR|, |final|), i.e. overflow-safe for all admitted inputs.
- Correction:
```suggestion
-(resistance_anisotropy*sin(magnetization_angle))*(2*cos(magnetization_angle))
```

### S3 — Hall voltage/sensitivity multiply `RH*I` first; large-RH/small-gap-scale vectors lose representable outputs, and the same grouping can turn an exact zero into NaN
- Severity: P3. ID: `sensors.hall_plate_angle`. File: `backend/catalog/sensors.json:357` (`hall_voltage`) and `backend/catalog/sensors.json:363` (`angle_sensitivity`).
- Evidence: `field_magnitude=1e200, field_angle=0, bias_current=1e200, hall_coefficient=1e200, plate_thickness=1e300, field_limit=1e200, current_limit=1e200` passes both inclusive limits; exact `hall_voltage=1e300` and `angle_sensitivity=0`, but `hall_coefficient*bias_current` = 1e400 → inf, and on the sensitivity the following `inf*sin(0)` even produces NaN. Reordering the division ahead of the large product repairs this vector:
```suggestion
hall_coefficient*(field_magnitude/plate_thickness)*cos(field_angle)*bias_current
```
```suggestion
-hall_coefficient*(field_magnitude/plate_thickness)*sin(field_angle)*bias_current
```
- Honest limit: no fixed ordering in the expression DSL is overflow-free over the whole admitted domain (e.g. subnormal `plate_thickness` with huge `B` makes `field_magnitude/plate_thickness` itself non-representable while the exact voltage is finite); S3 is the best single-ordering repair, not a complete scale-invariance fix. Reported at P3 for that reason.

Nominal regression check for Main: each suggested form reproduces its stored example within the 1e-9 relative tolerance (S1: 0.00227955…; S2: −17.320508075688…; S3: 0.025 / 0.025 / −0.043301270189…); only ulp-level rounding changes. No test, example or documentation text needs to change with them.

## Gate

- Pixel gate: PASS — all seven required images present, actually viewed, no material visible defect.
- Source gate: HOLD — three admitted-domain conditioning findings (S1, S2, S3) lose representable finite outputs to intermediate overflow; all nominal examples, constraints, units, semantics and documentation are otherwise correct and mutually consistent across research/catalog/docs/backlog.
- Overall: **HOLD** solely pending Main's reproduction and the S1–S3 expression repairs (plus recapture only if Main changes any diagram — none of S1–S3 touches geometry, so current captures can stand).

## Untested scope (honest)

- No engine, test-suite, lint, formatter, desktop or native process was run by this reviewer; all numerical evidence is independent hand/arithmetic source analysis, and the S1–S3 expected behaviors (rejection vs repaired values) are predictions for Main to execute.
- Negative-branch Type-T operation, real device calibration accuracy, tolerance classes, self-heating, loading and all other documented nonidealities remain out of scope by design.
- Diagram semantics were judged from the seven static captures at 2cca03e; interactive rendering (hover, window resize, font fallback) was not observed.
- The research-side AMR mirror provenance caveat stands as documented; no authenticated NXP-hosted source was consulted here.
