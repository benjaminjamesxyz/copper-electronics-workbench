# Passives source and native-pixel review

- Reviewer: V2PassivesRun (stable backlog owner V2Passives).
- Snapshot: `4c46fa1`; production root `/home/simson/pi-projects/pcb-design-calculator`.
- Intended owned report: `coordination/expansion/reviews/passives.md`.
- Overall gate: **HOLD**. Source: **HOLD** (F1). Pixels: **HOLD** (F2, F3). All six required native images were actually opened and inspected.
- No production edits, commits, builds, tests, lint, formatters, backend-process launches, or native launches performed.

## Coverage and evidence

Read the expansion QA context, expansion contract, all passives backlog entries 083–088, both complete passives research files, the complete production catalog, diagram definitions and feature documentation. Read the relevant baseline inventory entries to check physical novelty. Inspected primitive pins, painter dispatch and label rendering, catalog discovery/evaluation, diagram-map registration, selected-ID lookup, and quantity-legend consumption. New catalog JSON is discovered by `backend/build.rs`; Engine evaluates constraints before ordered outputs and registers `log1p`, `expm1`, and `hypot`. `passivesDiagrams` is spread into `calculatorDiagrams`; the selected ID reaches `DiagramPanel` and every used mark has a painter branch. No silent-dispatch/drop or missing quantity-key defect found.

The diff from `29f9c2b` to `4c46fa1` introduces all three reviewed production files (729 catalog lines, 238 diagram lines, 314 documentation lines). Findings below overlap those additions. Source defects inherited from the research definitions still enter production in this patch; they are not discrepancies to silently normalize away.

Native evidence is Main's `coordination/expansion/native_images.json`: Linux Flutter RepaintBoundary captures at pixelRatio 2, composited onto the actual paper surface without changing geometry. Every reviewed image is 1400×560. Main's receipt says all 228 per-model selection/metadata/example assertions completed, but a later temporary-harness menu lookup failed. That receipt is not an overall native acceptance PASS and is not reviewer-executed verification.

### Exact images actually viewed

1. `/home/simson/pi-projects/pcb-design-calculator/coordination/expansion/screenshots/passives.mlcc_logarithmic_aging.png`
2. `/home/simson/pi-projects/pcb-design-calculator/coordination/expansion/screenshots/passives.dielectric_absorption_recovery.png`
3. `/home/simson/pi-projects/pcb-design-calculator/coordination/expansion/screenshots/passives.mlcc_bias_charge_energy.png`
4. `/home/simson/pi-projects/pcb-design-calculator/coordination/expansion/screenshots/passives.resistor_excess_noise.png`
5. `/home/simson/pi-projects/pcb-design-calculator/coordination/expansion/screenshots/passives.negative_vcr_divider.png`
6. `/home/simson/pi-projects/pcb-design-calculator/coordination/expansion/screenshots/passives.nonideal_r2r_ladder.png`

## Per-model review

| ID | Source/model disposition | Native-pixel disposition |
| --- | --- | --- |
| `passives.mlcc_logarithmic_aging` | PASS for inspected scope. One-hour reference, seconds-to-hours conversion, noncompounded percent-per-decade loss, age≥3600 and positive-retained-capacitance guard agree with research. `log1p` preserves near-reference increments. Complete three-output example is consistent: 4 decades, 0.08 loss, 9.2 µF. Novel material-aging clock, not thermal lifetime or TCR. | HOLD, F2. Semilog direction, one-hour anchor and separated capacitor are correct; C(t) touches/intersects the plotted curve. |
| `passives.dielectric_absorption_recovery` | PASS for inspected scope. Derivation correctly builds hidden voltage through finite charge and short histories, then conserves charge after opening. `expm1` preserves tiny charge/recovery intervals; signed voltage and zero times are admitted correctly. All four example outputs are supplied and consistent with RaCa=10 s, tau=10/1.01 s, retained voltage 10(1−exp(−2))exp(−0.1), and final voltage divided by 101. Distinct from externally initialized charge sharing. | PASS. Cm parallels the series Ra–Ca branch; the internal node is isolated from Cm. Pins and explicit rail junctions connect correctly. Open terminals, A-positive/B-reference and Va0-at-release caption match the modeled observation/history. No visible clipping or text/geometry collision found. |
| `passives.mlcc_bias_charge_energy` | HOLD, F1. Physical integration, units, interpolation-only constraints and all five nominal outputs are correct: C_hi=5.2 µF, Q=42 µC, W=199.2 µJ, C_Q=7 µF, C_E=6.64 µF. Factored voltage-window integrals avoid subtraction of large antiderivatives, but endpoint capacitance still cancels. Distinct nonlinear constitutive integration, not a fixed-C energy calculator. | HOLD, F3. The charge hatch represents ∫C dV, separate energy annotation represents ∫VC dV, and capacitor polarity is correct. Cfit is crossed by the plotted curve. |
| `passives.resistor_excess_noise` | PASS for inspected scope. The 0 dB=1 µV/V normalization, 1/f power density, logarithmic band integral and amplitude-density units agree. Negative bias uses magnitude; empty band remains admitted with finite point density. Complete four-output example has a=1e−7, D=3, RMS=1e−6√3 and density=1e−6/√(10 ln 10). `log1p` protects a narrow band. Distinct from Johnson noise and generic RMS addition. | PASS. Source polarity matches resistor VDC reference. Separate series-noise equivalent has an open output port and no accidental short. The conceptual descending amplitude-density curve, band edges and probe are readable; no clipping found. |
| `passives.negative_vcr_divider` | PASS for inspected scope. Kirchhoff's equation produces the documented quadratic; the rationalized root is continuous at k=0, and the `hypot` argument represents the correct discriminant. The whole-interval positive-resistance guard and zero-supply ratio-error continuation are explicit. All five example values agree with the researched root and satisfy Vh+Vo≈1000 V and equal branch currents. Distinct implicit constitutive equilibrium, not a fixed divider/TCR variant. | PASS. Correct upper nonlinear resistor, unloaded output junction, bottom reference, positive-high-side bracket, R0 zero-voltage labeling and ppm/V conversion explanation. No visible clipping or material overlap found. |
| `passives.nonideal_r2r_ladder` | PASS for inspected scope. Three successive Thevenin reductions match the terminated three-node network, unequal switch resistances and grounded load. Bit extraction is exact for integer 0…7; code constraint, R/Vref positivity, nonnegative switches/load and open-load zero are consistent. All nine example outputs are complete and agree with the supplied nodal equations and 1142500/417997 V loaded result. Error includes gain/loading and is correctly not called INL. Distinct component network, not a renamed ideal DAC. | PASS. Permanent left 2R termination is separate from the switched LSB leg; exactly two R links join N0/N1/N2. All selected sources are positive at their upper pins with common grounded returns, and RL connects output to ground. No subladder Thevenin result falsely labels a loaded internal node. No clipping or material overlap found. |

The per-model PASS entries above are bounded source/pixel dispositions, not executable test passes or general hardware certification. Complete nominal examples were checked for equations, units, output completeness and internal consistency; precision runtime execution remains Main's responsibility.

## Actionable findings

### F1 — P2 — Preserve the characterized endpoint capacitance

- ID: `passives.mlcc_bias_charge_energy`.
- Patch anchor: `backend/catalog/passives.json:267–270`, especially expression line 270.
- Evidence class: independent binary64/source analysis, **not an executed engine result**.
- Complete counterexample: `zero_bias_capacitance=1e-6`, `fit_capacitance=1e-24`, `fit_voltage=1`, `lower_voltage=0`, `upper_voltage=1`. All five inputs satisfy the declared domain, and all five mathematical outputs are finite. The endpoint identity requires `upper_capacitance=fit_capacitance=1e-24 F`. Binary64 evaluates `1e-6−1e-24` as `1e-6`; the current outer subtraction therefore returns zero. The finite-result evaluator accepts this zero, contradicting the positive characterized capacitance. This is not physical extrapolation: the request is exactly at the supplied endpoint.
- Expected behavior: retain the positive endpoint value, including small representable fitted capacitance. At this full window, Q≈5e−7 C, W≈1.6666666666666667e−7 J, C_Q≈5e−7 F and C_E≈3.3333333333333335e−7 F; no other output requires overflow or an invalid operation.
- Exact minimal repair: replace the endpoint expression with `fit_capacitance+(zero_bias_capacitance-fit_capacitance)*((fit_voltage-upper_voltage)/fit_voltage)`. This uses the distance from the characterized endpoint and preserves its exact value. Add an independent endpoint assertion with relative-only tolerance for the tiny positive result. Do not clamp the result or arbitrarily exclude positive Cfit. Keep research/production discrepancy recorded for Main rather than silently changing research.
- Follow-through scope: inspect the same subtractive interpolation in charge/energy equivalent outputs for narrow windows near Vfit. Positive endpoint-weighted integration is the appropriate common numerical form; a broad refactor is unnecessary. No separate claim of an executed failure for those outputs is made here.
- Confidence: 1.0.

### F2 — P2 — Move the aged-capacitance label clear of the curve

- ID: `passives.mlcc_logarithmic_aging`.
- Patch anchor: `frontend/lib/diagrams/passives.dart:18`.
- Pixel evidence: image 1 above. The green descending line runs into the lower edge of the green C(t) label, near native x≈780–833, y≈275–286. Source places `Label(390, 125, 'C(t)', ...)` across the line from (85,85) to (485,155); the line is at logical y≈138.4 at the label's left edge. The label is not separated from the plotted geometry as required.
- Expected behavior: readable quantity label with an unambiguous gap from the conceptual line; no change to curve/quantity meaning.
- Exact minimal repair: change the C(t) label's y coordinate from `125` to `115`, retaining x, text, quantity binding and ink. Recapture the native image and check clearance.
- Confidence: 0.98.

### F3 — P2 — Move the fitted-capacitance label above the curve

- ID: `passives.mlcc_bias_charge_energy`.
- Patch anchor: `frontend/lib/diagrams/passives.dart:73`.
- Pixel evidence: image 3 above. The green C(V) line visibly crosses the bottom of the black Cfit text around native x≈714–766, y≈292–304. Source places `Label(357, 135, 'Cfit', ...)` on the line from (70,70) to (410,160); that line reaches logical y≈146 at the label's left edge. This is an actual text/geometry collision, not a requested aesthetic redesign.
- Expected behavior: Cfit is clearly separate from the curve while still labeling its fitted endpoint.
- Exact minimal repair: change the Cfit label's y coordinate from `135` to `115`, preserving its other fields. Recapture and inspect the native image.
- Confidence: 1.0.

## Scientific source handling

The complete research and feature documents identify the primary sources and explicit model boundaries: [KYOCERA AVX dielectric aging](https://rfs.kyocera-avx.com/userFiles/uploads/pdfs/dielectric_aging.pdf), [TI SBOA597 §4.5](https://www.ti.com/lit/pdf/sboa597), [ADI RAQ192](https://www.analog.com/en/resources/analog-dialogue/raqs/raq-issue-192.html), [Vishay resistor noise](https://www.vishay.com/doc?49159=), [Vishay fixed-resistor basics](https://www.vishay.com/docs/28771/basics.pdf), [Vishay VCR FAQ](https://www.vishay.com/en/landingpage/rifaq/index.html#bp6), and [ADI voltage-mode ladder topology](https://wiki.analog.com/university/courses/tutorials/cmos-dac-chapter). No remaining physical ambiguity required new external retrieval for these findings. Primary retrieval claims remain inherited research evidence, not reviewer-retrieved evidence. The documented one-branch polarization, reversible linear C(V), characterized 1/f range, fixed-temperature negative VCR and constant switch resistance are limitations, not newly discovered defects.

## Required closure and untested scope

Main must fix F1–F3, execute the independent endpoint regression plus complete passives examples/boundaries, and return changed native images for F2/F3 closure. Overall **HOLD** remains until those material findings close. T2PassivesRun and Main received the numerical counterexample through direct messages; the corresponding root-bus handoff is supplied separately for persistence.

Not checked: builds/tests/lint/formatters, backend runtime, native interaction/navigation, narrow-window or large-text behavior, changed-image recapture, Windows/macOS, hardware calibration, and exhaustive floating-point extremes. No production edits or unsupported execution claims. Empirical model validity and component ratings remain user responsibilities.
