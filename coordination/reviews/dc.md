Please persist the following complete report verbatim to coordination/reviews/dc.md and append the completion bus entry below (reviewer fs writer unavailable; production/research/tests/docs remain untouched). Static review complete, tested=false. Confirm persisted path so I can close assignment.

--- REPORT START ---
# Independent review — dc

- Reviewer: ReviewDc, independent of DevDc and TestDc.
- Frozen integrated development baseline: `7f12f45`; compared promotion against `906b351` and inspected root production files.
- Decision: **findings** (one actionable P2 numerical-correctness defect).
- `tested=false`: no build, lint, test, formatter, smoke command, application or engine evaluation executed. Numerical statements below are independently derived algebra and IEEE-754 reasoning, not fabricated runtime observations.
- Scope: all eight definitions in `backend/catalog/dc.json`, corresponding research JSON/derivations, `docs/features/dc.md`, shared `backend/src/lib.rs`, `backend/build.rs`, `backend/src/main.rs`, and downstream catalog/request/result dispatch.

## Actionable finding DC-R1 — P2: preserve source currents when the node rounds to a source

**Calculator/file/lines:** `dc.two_source_node`, `backend/catalog/dc.json:296-301` (source 1), with symmetric source-2 expression at 302-307. Source-current expressions subtract the previously rounded absolute node voltage from the source voltage. The new promoted catalog therefore silently loses a real source current when one source dominates the node conductance.

**Concrete accepted input:** `v_1=524288 V`, `r_1=1.862645149230957e-9 ohm`, `v_2=0 V`, `r_2=134217728 ohm`, `r_load=134217728 ohm`. These are exact powers of two and lie within every voltage/resistance bound and positive-resistance constraint.

**Independent expected behavior:** let `g1=2^29 S`, `g2=gL=2^-27 S`, and `V1=2^19 V`. Exact KCL gives `Vnode=2^19/(1+2^-55)` and `I1=2^-7/(1+2^-55) A`, approximately **0.0078125 A**; `I2=-2^-8/(1+2^-55) A` and `IL=2^-8/(1+2^-55) A`, each approximately 0.00390625 A in magnitude. Thus `I1+I2=IL`.

**Static implementation evidence:** at `g1=2^29`, the binary64 ULP is `2^-23`; each subsequent `2^-27` conductance is smaller than half an ULP and is discarded in the left-associative sum. The numerator is exactly `2^48`; the computed node becomes exactly `2^19`. Current-one then becomes zero, while current-two is -0.00390625 A and load current is +0.00390625 A. The 7.8125 mA KCL residual and 100% error in source-one current are avoidable, not subnormal underflow or a legitimate zero-current boundary. The node is not near zero, so the documented opposing-source/node-zero cancellation caveat does not address this case.

**Evidence versus inference:** catalog expressions, domains and sequential evaluation were read directly. The outputs are a static IEEE-754 proof, not observed engine output. Bucknell's nodal-analysis page and William & Mary's Chapter 2 (point law `I1+I2-I3=0` and the same two-source/three-resistor topology) were fetched and inspected. Expanded-current correction is our algebraic derivation from those conservation equations.

**Correction:** before numerical evaluation, expand source-current numerators to remove subtraction of rounded absolute node voltages: `i_source_1=((v_1-v_2)/r_2+v_1/r_load)/(r_1*g_total)` and `i_source_2=((v_2-v_1)/r_1+v_2/r_load)/(r_2*g_total)`. Retain the researched topology, all keys/units, signed domains and original example; explain equivalent conditioning in feature docs. No clamping or narrowed resistance domain is needed.

**Parent verification scenario:** evaluate the exact vector above; expect nonzero source-one current near 0.0078125 A, source-two -0.00390625 A and load +0.00390625 A, with KCL checked at that current scale. Swap source branches and reverse both source polarities. Keep nominal example, exact opposing-source zero-node, zero excitation and backfeed checks. Use relative tolerance 1e-6 for nonzero expected currents, not a blanket absolute tolerance that would hide lost small currents. No commands were executed here.

## All-eight independent model assessment

All 34 example outputs were read against independently worked circuit equations and the research oracle. Algebra, not the evaluator, establishes the expectations below.

| ID | Independent assessment | Additional oracle/boundary |
|---|---|---|
| `dc.ohm_current` | `I=V/R`; passive sign, ohm/V/A units and strictly positive resistance correct. Zero and signed voltages remain admissible. Nominal -3.3/2200=-0.0015 A. | V=-2,R=4 gives -0.5 A; V=0 gives 0. Reject R=0/negative/outside [1e-9,1e12]. |
| `dc.resistor_dissipation` | `P=I^2R` correct for steady DC; sign invariant, no RMS/peak ambiguity. Nominal 0.012²·680=0.09792 W. Does not claim a safe rating or temperature. | I=±0.1,R=100 gives 1 W; I=0 gives 0. Positive resistance/domain guards correct. |
| `dc.series_parallel_branches` | Two independent series pairs, not loaded midpoints. Branch sums and `AB/(A+B)` match equal voltage/KCL. Nominal 300/600/200 ohm and total/branch currents 0.06/0.04/0.02 A correct. | V=6, R1=R2=2,R3=R4=4 gives A=4,B=8,Req=8/3 ohm,I=2.25 A,I1=1.5 A,I2=0.75 A. Swapping branches preserves Req/total. |
| `dc.loaded_divider` | Top resistor feeding parallel bottom/load; Thevenin resistance and finite loading exact. Resistance-defined fractional error avoids 0/0 at Vin=0. Nominal 4 V,10000/3 ohm,3 V,0.0003 A,0.25 correct. | Vin=6 and all resistors=100 gives Vopen=3 V,Rout=50 ohm,Vloaded=2 V,Iload=0.02 A,error=1/3. Vin=0 gives voltage/current zero without losing finite error. |
| `dc.resistive_source_matching` | Fixed positive source resistance; differentiating load power gives Rmatch=Rs and Pmax=Vth²/(4Rs). Source-loss output/power conservation correct. Matching is expressly not maximum efficiency; zero-source nonunique optimum honest. Nominal 1 A,6 V,6 W,3 W,3 ohm,6.75 W correct. | Vth=4,Rs=Rl=2 gives I=1 A,Vload=2 V,both losses=2 W,Pmax=2 W. Negative V reverses current/voltage, not dissipations. |
| `dc.loaded_wheatstone_bridge` | Floating detector between midpoints; independent arm Thevenin sources yield differential/current signs and loaded common mode correctly. Original-network nodal KCL independently validates all seven nominal outputs (-2.5 V,1250 ohm,-0.001 A,-1.25 V,5.5 V,6.75 V,6.125 V). Exact balanced case is nonsingular. | Vs=12,all four arms=100,Rd=100 gives Vopen=0,Rout=100,Id=Vloaded=0,Va=Vb=Vcm=6. Reversing excitation reverses voltage/current signs. Near-balance absolute-scale precision caveat is honest. |
| `dc.delta_to_star` | Incident-arm product divided by delta sum is correctly labeled a/b/c; terminal-pair equivalence independently confirms 150/100/300 ohm nominal outputs. Newly introduced center must remain floating, explicitly documented. | All delta arms=300 gives all star arms=100; terminal-pair resistance=200 ohm before/after. Positive bounds prevent zero denominators. |
| `dc.two_source_node` | Analytic Millman/KCL topology and signs correct; ideal sources explicitly must sink backfeed. Nominal G=0.002 S,V=6.75 V,I1=0.00525 A,I2=-0.001875 A,IL=0.003375 A correct. **Current conditioning is defective: DC-R1.** | V1=6,V2=-6,all R=1000 gives G=0.003 S,Vnode=0,I1=0.006 A,I2=-0.006 A,IL=0. Zero-node/cancellation acceptance retained; also verify DC-R1. |

All catalog model constraints reference inputs only; supporting outputs are ordered correctly. Positive resistance floors and finite ceilings bound products/reciprocals without overflow for ordinary in-envelope voltage/current magnitudes. Voltage/current input ranges admit arbitrarily tiny finite values: subnormal output underflow is not ruled out by the stated upper bounds, and mathematical tolerance is not a promise of arbitrary relative precision. No additional actionable underflow finding is claimed without a representable required-output counterexample. No DC model uses transcendental small/large-argument functions; the material conditioning concerns are subtractive bridge/node balance and source-current recovery.

## Sources actually inspected

1. Bucknell, Rich Kozick, [ELEC 101 nodal analysis](https://www.eg.bucknell.edu/~kozick/elec10105/nodevoltage.html): consistent signed KCL, resistor-current construction and node solution. The specific closed form is independently derived, not falsely attributed verbatim.
2. William & Mary, [Analog Electronics Chapter 2](https://physics.wm.edu/~evmik/classes/Physics_252_Analog_Electronics/lab_manuals/LabManual_Chpt2.pdf): pp. 1-2 same two-source topology/KCL, pp. 3-4 Thevenin loading, p. 5 the maximum-power/efficiency wording caveat, pp. 6-7 bridge exercises. Loaded midpoint/common-mode equations are derived here, not printed-source claims.
3. Colorado Boulder, [PHYS3330 Lab 2](https://physicslabs.colorado.edu/courses/phys-3330/lab-guides/lab2/): sections 2.4 maximum power, 2.6 loaded-divider direct reduction and 2.7 Thevenin values. Confirmed dated August 19, 2026 as research states.
4. UCSB, Loai Salem, [ECE10A Lecture 12](https://web.ece.ucsb.edu/~parhami/pres_folder/10A_lecture12_notes.pdf): pp. 54-61 terminal-pair equality derivation and incident-delta-arm product over sum.

OpenStax Ohm-law, power and series/parallel links were preserved and their research descriptions read, but not freshly fetched by this reviewer; those elementary identities were assessed by independent conservation/constitutive algebra. Research's failed manufacturer/Harvard/Rhode Island/RPI access is disclosed rather than counted as inspected evidence. No standards compliance or package-safety claim is made. Ideal topology validity is conditional on actual sources, resistor ratings and nonideal behavior.

## Shared engine/startup/JSON-lines and consuming dispatch

- `backend/build.rs` enumerates sorted JSON catalogs and embeds them by include_str; no runtime network/file fetch needed. Directory read/enumeration failure is flattened and could yield incomplete startup catalogs; this behavior predates the integrated lane changes and is not reported as a patch-introduced finding.
- `Engine::new` parses definitions and expression syntax once, rejects duplicate IDs/keys and invalid constraint operators, reserves pi/e, then executes every default through the same bounds/constraints/output ordering. All inspected DC definitions meet current contracts. Startup does not explicitly enforce lane counts/levels/metadata/example expected completeness or compare examples; these are existing substrate limitations, not invented regressions from this lane. Default evaluation resolves unknown symbols/order for the current catalog.
- Runtime `calculate` checks exact key count/membership, every finite scalar, inclusive/exclusive bounds, and input-only constraint expressions before output iteration. Nonfinite evaluated constraints/outputs become errors. The shared evaluator checks completed expression finiteness, not every internal arithmetic operation: masked intermediate nonfiniteness remains an existing design limitation, with no demonstrated new DC path because all its denominators/products are bounded and positive.
- Request JSON is deserialized once per line into explicit Catalog/Calculate variants; `respond` explicitly handles both, returns successful ID/output maps or `ok:false` errors for parse/unknown-ID/domain failures. `main.rs` iterates request lines, serializes/flushes one response per line, continues after request errors, and reserves stderr for startup/I/O diagnostics. I/O decoding/write failure ends the process rather than pretending recovery; ordinary invalid requests do not poison engine state by inspection.
- Inputs/output map allocations occur per calculation (HashMap and BTreeMap), but expressions are compiled once. No patch-introduced performance/security blocker is evidenced in these eight small calculators.
- Consuming-side dispatch read: `frontend/lib/backend_client.dart` serializes requests, decodes `ok/error`, drains stderr, and recovers its request queue after errors; `frontend/lib/models.dart` decodes every catalog definition generically; `frontend/lib/main.dart:45-90,183-203` maps catalog items, issues selected-ID calculate requests and iterates declared output keys. No hardcoded lane list or silent drop of the eight new IDs/outputs was found. Explicit error UI and revision guards were inspected, not exercised.
- Added global functions log1p/expm1mx/step_overdamped/step_underdamped are registered in the expression context and consumed by the relevant non-DC catalogs; DC expressions do not depend on them. Their full independent numerical certification belongs to other domain/shared review, not a fabricated DC test pass.

## Untested scope and handoff

No domain tester branch or half-authored tester file was consumed. Existing parent-owned protocol test intent was read only, not executed. No CLI recovery, UTF-8/I/O fault, default/example execution, custom expression fault injection, schema mutation, Flutter interaction, packaging or physical hardware test was performed. Parent-reported checks remain parent evidence, not this reviewer's observations. Parent must run integrated domain tests and DC-R1 regressions after remediation. Report decision is findings until DC-R1 has an evidence-backed disposition.

Persistence note: reviewer ordinary-path write was rejected by the xd-only tool transport, and the read-only bash contract prohibits shell writes. Report and durable bus text were supplied to Main for permitted parent persistence; this does not authorize any reviewer production edit.
--- REPORT END ---

BUS APPEND (anchored append only):
## ReviewDc → Main / Dev-dc / Test-dc: independent review complete; one P2 finding
- Baseline `7f12f45`; all eight DC models and shared engine/startup/JSON-lines consumer dispatch reviewed. Decision `findings`; `tested=false`; no builds/lint/tests/formatters/smoke/application commands run.
- DC-R1 `dc.two_source_node`, `backend/catalog/dc.json:296-301` (symmetric source 2 at 302-307): accepted exact-power-of-two vector `{v_1:524288,r_1:1.862645149230957e-9,v_2:0,r_2:134217728,r_load:134217728}` statically loses the entire 0.0078125 A source-1 current and violates KCL. Expand source-current conductance numerators before division; preserve sign/domain/topology and regress original/swapped/reversed-polarity inputs with scale-sensitive tolerances.
- Source/evidence, all-eight independent oracles, correction and untested scope: [complete DC review](coordination/reviews/dc.md). Main owns fixes and all executed verification. Reviewer report/bus writing was blocked by xd-only transport, so Main persists this supplied handoff without a reviewer production edit.

## Reviewer handoff addenda

ReviewDc complete review output is submitted incrementally: verdict incorrect, confidence .98, one P2 finding. Final review locator should be source-one backend/catalog/dc.json:296-301 (6 lines; my earlier incremental finding covered 296-307, so normalize to 296-301 to honor <=10-line finding ranges). Report Markdown and completion bus text were fully delivered in my previous message. I cannot honestly claim disk persistence without parent action; please save coordination/reviews/dc.md and anchored-append COMMUNICATION.md. No production changes or runtime verification by reviewer.
