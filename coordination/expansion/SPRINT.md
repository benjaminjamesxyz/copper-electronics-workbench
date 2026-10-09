# Expansion implementation sprint — 100 new illustrated calculators

## Commitment and evidence boundary

The sixteen-researcher wave is complete. This sprint commits **100 Must-have calculator + mandatory diagram + documentation items**, with **26 basic, 39 intermediate and 35 expert** models. Exactly **16 lanes** own these features. Preserve all **128** existing IDs, numerical models, metadata, examples and diagrams from baseline `29f9c2b`; the release inventory must be **228**. No optional diagrams, deferred features, inverse/renamed duplicates or count-padding outputs are allowed.

The scrum refinement read both contracts, the board's completed expansion index, the expansion communication entries, the full baseline manifest and all **16 JSON + 16 Markdown research handoffs**. Reconciliation yielded 100 research objects, 100 unique IDs, exact equality with the 100 board IDs, and zero ID intersections with the 128 baseline IDs. The physical-model decisions below and the [100 explicit backlog entries](BACKLOG.md) provide semantic reconciliation; ID uniqueness alone is insufficient.

This document reports planning/reconciliation only. It does not claim that implementation, builds, tests, source review, screenshots or desktop checks have passed. No velocity, story points, duration or throughput estimates are invented. Historical first-sprint acceptance is not second-sprint evidence.

## Fixed architecture and ownership

Reuse the [expansion contract](../EXPANSION_CONTRACT.md) and [numerical contract](../CONTRACT.md). Rust owns closed-form numerics; Flutter remains desktop-only and talks to the actual local JSON-lines process. Reuse the current schema, validator, stable expression functions, painter, accessible panel and layout. No second evaluation engine, diagram framework, browser substitute, package dependency or simulator is planned.

Each feature has one independently accountable D2 lane owner. Each D2 owns only `backend/catalog/<lane>.json`, `frontend/lib/diagrams/<lane>.dart` and `docs/features/<lane>.md`. Each T2 owns only `backend/tests/<lane>.rs` and `coordination/expansion/testing/<lane>.json`. Each V2 reviews the integrated implementation read-only and records findings in `coordination/expansion/reviews/<lane>.md`; no reviewer patches another owner's files. Paths identify planned deliverables, not completed artifacts.

Main alone owns shared engine/schema changes, catalog/diagram registration and integration, global checks, actual process/native execution, captures, packaging and cross-lane/root-cause fixes. The board, baseline, research handoffs and original 128 production features are not worker edit targets. An evidence-backed research correction must be escalated to Main and its research owner; it cannot be silently patched or removed from scope.

## Sixteen accountable lanes and isolation

Developer worktrees and branches are mandatory. Testers start from the same complete integrated developer snapshot, not partially implemented lane heads. All relative paths below resolve under the repository root. The common append-only bus is root `COMMUNICATION.md`, not a worktree-local copy.

| Lane | New B/I/E | Total | Developer / Tester / Reviewer | Developer worktree · branch | Tester worktree · branch |
|---|---:|---:|---|---|---|
| sensors | 3/3/1 | 7 | D2Sensors / T2Sensors / V2Sensors | `.worktrees/expand-dev-sensors` · `feat/expand-sensors` | `.worktrees/expand-test-sensors` · `test/expand-sensors` |
| opto | 2/2/3 | 7 | D2Opto / T2Opto / V2Opto | `.worktrees/expand-dev-opto` · `feat/expand-opto` | `.worktrees/expand-test-opto` · `test/expand-opto` |
| oscillators | 2/3/2 | 7 | D2Oscillators / T2Oscillators / V2Oscillators | `.worktrees/expand-dev-oscillators` · `feat/expand-oscillators` | `.worktrees/expand-test-oscillators` · `test/expand-oscillators` |
| rf | 2/3/2 | 7 | D2Rf / T2Rf / V2Rf | `.worktrees/expand-dev-rf` · `feat/expand-rf` | `.worktrees/expand-test-rf` · `test/expand-rf` |
| batteries | 2/1/3 | 6 | D2Batteries / T2Batteries / V2Batteries | `.worktrees/expand-dev-batteries` · `feat/expand-batteries` | `.worktrees/expand-test-batteries` · `test/expand-batteries` |
| protection | 1/3/2 | 6 | D2Protection / T2Protection / V2Protection | `.worktrees/expand-dev-protection` · `feat/expand-protection` | `.worktrees/expand-test-protection` · `test/expand-protection` |
| converters | 1/2/3 | 6 | D2Converters / T2Converters / V2Converters | `.worktrees/expand-dev-converters` · `feat/expand-converters` | `.worktrees/expand-test-converters` · `test/expand-converters` |
| control | 1/3/2 | 6 | D2Control / T2Control / V2Control | `.worktrees/expand-dev-control` · `feat/expand-control` | `.worktrees/expand-test-control` · `test/expand-control` |
| emc | 1/3/2 | 6 | D2Emc / T2Emc / V2Emc | `.worktrees/expand-dev-emc` · `feat/expand-emc` | `.worktrees/expand-test-emc` · `test/expand-emc` |
| cables | 2/2/2 | 6 | D2Cables / T2Cables / V2Cables | `.worktrees/expand-dev-cables` · `feat/expand-cables` | `.worktrees/expand-test-cables` · `test/expand-cables` |
| precision | 2/2/2 | 6 | D2Precision / T2Precision / V2Precision | `.worktrees/expand-dev-precision` · `feat/expand-precision` | `.worktrees/expand-test-precision` · `test/expand-precision` |
| logic | 2/2/2 | 6 | D2Logic / T2Logic / V2Logic | `.worktrees/expand-dev-logic` · `feat/expand-logic` | `.worktrees/expand-test-logic` · `test/expand-logic` |
| motors | 1/3/2 | 6 | D2Motors / T2Motors / V2Motors | `.worktrees/expand-dev-motors` · `feat/expand-motors` | `.worktrees/expand-test-motors` · `test/expand-motors` |
| passives | 1/2/3 | 6 | D2Passives / T2Passives / V2Passives | `.worktrees/expand-dev-passives` · `feat/expand-passives` | `.worktrees/expand-test-passives` · `test/expand-passives` |
| cores | 2/2/2 | 6 | D2Cores / T2Cores / V2Cores | `.worktrees/expand-dev-cores` · `feat/expand-cores` | `.worktrees/expand-test-cores` · `test/expand-cores` |
| measurement | 1/3/2 | 6 | D2Measurement / T2Measurement / V2Measurement | `.worktrees/expand-dev-measurement` · `feat/expand-measurement` | `.worktrees/expand-test-measurement` · `test/expand-measurement` |
| **Total** | **26/39/35** | **100** | **16 developers / 16 testers / 16 reviewers** | **16 isolated developer worktrees** | **16 isolated tester worktrees** |

Research links, exact IDs, accountable names, nominal numerical oracles, limits/errors, diagram topology/labels and prerequisites appear once per feature in [BACKLOG.md](BACKLOG.md). Every ID remains in its researched lane; no feature is shared across developer lanes.

## Semantic novelty decisions and anti-regression conditions

The entire baseline is excluded from new-feature counting. Every backlog entry compares its physical question against the nearest baseline/neighbor, including adjacent domains with similar algebra. Supporting quantities, inverses, units, parameter variations and alternative states of a model remain outputs/cases of one feature. A limiting case may reproduce an old ideal formula without making a distinct nonideal/history/topology model a duplicate; removing the distinguishing behavior would make the new implementation fail acceptance.

### Highest-risk near-neighbors

| Retained feature | Closest existing/new neighbor | Required distinction and decisive acceptance |
|---|---|---|
| `passives.dielectric_absorption_recovery` | `transients.capacitor_charge_sharing`, `transients.rc_step` | One nonideal component has a hidden polarization state prepared by **finite charge → short → open history**. Ordinary sharing starts from two arbitrary specified capacitor voltages. Preserve charge duration, short duration, internal memory and subsequent recovery; zero charge duration gives no rebound, long short erases memory and recovery starts at zero. Draw Cm parallel to series Ra–Ca with hidden node and history strip. Do not replace history by an entered hidden initial voltage. |
| `passives.nonideal_r2r_ladder` | `data.unipolar_dac_output`, `dc.two_source_node` | Solve the component-level three-rung network with **unequal switch resistances, output loading and unswitched 2R LSB termination**. Ideal code/8 appears only as a limiting oracle. Independently solve nodal equations and isolate code-1/2/4 weights; do not force 1:2:4 weights under mismatch. Subladder Thevenin voltages are not loaded internal node voltages. |
| `converters.forward_reset_winding` | `magnetics.turns_ratio`, `power.buck_inductor_design` | A third winding resets magnetizing volt-seconds. With n=Np/Ns and r=Nr/Np, D=nVo/Vin, reset fraction=rD, Dlimit=1/(1+r), and switch stress=Vin+Vin/r. Require positive idle/reset feasibility; reject D=Dlimit. Changing reset ratio changes timing and stress independently of ideal secondary ratio. Preserve dots, reset-diode direction and CCM-output assumption. |
| `precision.bjt_mirror_base_current` | `bjt.divider_bias`, `bjt.hybrid_pi_parameters`, `bjt.differential_pair_steering` | Two matched transistors share a diode-connected reference node; **reference current supplies both base currents**. KCL gives Iref=Ic+2Ib and Ic=Iref·beta/(beta+2). A single-transistor divider operating point or tail-current split does not solve copying deficit/compliance. Preserve the collector/base short, separate output collector and finite-beta error. |
| `protection.copper_adiabatic_fault_limit` | `protection.fuse_pulse_coordination`, `pcb.trace_joule_heat_density`, `thermal.resistor_tcr_drift` | **Integrate temperature-dependent copper resistivity/heating to an allowed endpoint** using the researched 226/234 material constants. This derives k and conductor withstand; fuse coordination consumes supplied device I²t budgets, while old trace heat uses fixed resistivity. Preserve temperature/time validity, small positive temperature difference and Iallow²t=(kS)². I²t is A²s, not joules. |

### Remaining adjacent-domain boundaries

- **Sensors / precision / opto:** sensor constitutive transduction is distinct from amplifier conditioning. NTC reciprocal-log calibration and platinum quadratic/subzero-quartic response are not linear TCR. Hall transverse odd symmetry differs from AMR longitudinal pi-periodic resistance. Moving differential capacitor geometry is not voltage-driven plate displacement current. Optical photon conversion uses supplied QE; layer absorption partitions optical power without claiming collection efficiency; TIA/log circuits remain Precision-owned.
- **Sensors / batteries:** glass-electrode hydrogen activity uses a membrane Nernst potential. Intercalation additionally models occupancy/vacancy entropy, pair interactions, stable single-phase domain and chemical differential capacitance. Coulomb accounting, weakest-cell series feasibility, Peukert rate loss, two-well history/recovery and planar electrolyte depletion are six different questions, not renamed Ah/A budgets.
- **Oscillators / passive response / timing:** self-generated switching or regenerative feedback determines intervals/startup. The 555 discharge path, comparator threshold network, active-integrator triangle loop, Wien bridge, mutually loaded RC chain, differential negative-conductance LC and crystal/inverter network are separate topologies. They do not count externally imposed RC switching, open-loop ramps, passive resonance, ordinary filter response or clock division again. Signed decaying/marginal startup diagnostics remain valid where specified. The phase-shift chain has no extra third-node shunt; LC values are differential and gm is per transistor; Pierce requires actual available gm.
- **RF / EMC / cables / noise:** radio Friis propagation is not the baseline Friis noise-factor cascade. Matched T attenuation dissipates in a matched two-port; L matching uses lumped reactances, unlike the quarter-wave line. Fresnel geometry, aperture synthesis and one-way propagation answer different design questions. Radar adds passive target scattering and fourth-power range loss. EMC owns short-wire unintentional radiation, aperture leakage, common-mode conversion, conducted harmonics, FM occupancy and measured-parasitic snubbers; intentional RF links and cable field/line/contact mechanics remain separate.
- **Converters / control / cores:** charge transport, two-inductor SEPIC current paths, forward reset, DCM flyback energy intervals, half-bridge rectifier-loaded LLC FHA and signed dual-bridge power are topology-specific steady models. Controller delay/sensitivity, lead shaping, PI coefficient matching, sampled-current contraction and cubic sensor-lag stability are independent feedback questions. Core geometry, discrete insulated packing and losses are not converter operating points or controller models. LLC must retain Rac=8n²Ro/pi² and half-bridge normalization; flyback RMS uses the full period; DAB retains negative phase and the falling-power branch.
- **Protection / EMC / thermal:** fuse pulse survival and semiconductor clearing coordination deliberately count **once**. TVS affine source intersection, MOV fitted power-law pulse, SCR regenerative trigger/latch/hold and foldback fault operating-point/envelope optimization are not fixed diode clipping, ordinary base drive or LDO loss. EMC owns RC snubbers. Copper material-temperature integration belongs to Protection, not Cables.
- **Cables / baseline interconnect:** twin-lead has a distinct transverse boundary geometry; distributed lossy RLGC is not ideal delay or lumped RLC. Radial insulation stress is a dielectric field/breakdown screen, not coax impedance inverted. Hertz/Holm constriction, Kohlrausch hotspot and cantilever normal force solve separate microscopic conduction, thermal and elastic problems. None is uniform rho·length/area or a generic lumped thermal step.
- **Precision / baseline op amps:** instrumentation checks hidden first-stage clipping; TIA source capacitance and finite GBW require second-order damping; Howland mismatch creates signed output conductance; Brokaw synthesizes PTAT/CTAT cancellation; LOG114 implements an actual logarithmic current front end. Keep circuit connectivity, internal-node limits and matching conventions; an ordinary scalar gain or dB conversion fails novelty.
- **Logic / measurement / timing:** voltage recognition, current fanout, characterized internal CMOS Cpd, stochastic metastability, endpoint-constrained gate sizing and FIFO arrival/service storage are distinct from existing I2C/UART/setup/divider/delay. Measurement gain/offset specification, three-reading thermal-EMF cancellation, compensated probe loading, finite integration aperture, slope-converted jitter and correlated ratio uncertainty are not ADC quantization/alias/acquisition or independent RMS summation. No reliability or coverage guarantee follows from an estimate.
- **Motors / electrical transients:** steady back-EMF equilibrium, spatial stepper stiffness, accelerating geared inertias, generator braking, salient dq coupling and induction slip power require electromechanical laws absent from generic Ohm/RL/loss formulas. Preserve energy identities, signed generation where admitted, peak dq conventions and the exact zero-slip limit.
- **Passives / cores:** dielectric aging is not thermal aging/TCR; integrated differential C(V) differs from an entered energy-equivalent C; DC-bias 1/f excess noise differs from Johnson noise; implicit negative-VCR equilibrium differs from a fixed loaded divider. Cores owns all magnetic constructions/losses. Empirical total Steinmetz loss must not be added to its already included classical lamination mechanism; foil transport/proximity loss is distinct from magnetic-sheet eddy loss and standalone skin depth.

### Rejected duplication and evidence escalation

Research already merged separate fuse-survival and semiconductor-clearing candidates into one `protection.fuse_pulse_coordination`; `protection.foldback_fault_envelope` supplies the distinct sixth item. Research excluded uniform-loop Faraday pickup because it would invert existing sinusoidal flux. Neither discarded proposal appears as another backlog item.

No remaining genuine duplicate or missing-science implementation blocker was identified during this refinement. This is a reasoned scope decision, not scientific-review closure. Research discloses inaccessible originals and readable replacements/mirrors, including Wheeler, AMR and Pierce provenance; failed retrievals must not be described as verified primary evidence. Follow the linked source-access notes and independent derivations. Physical approximation screens, calibration uncertainty, first-order metrology limits and manufacturer applicability remain visible.

If D2/T2/V2 finds a real duplicate, equation/source gap, wrong topology, contradictory oracle or unsupported domain, report exact IDs, source section, reproducing vector and proposed correction to Main and the accountable lane through root `COMMUNICATION.md`. Keep the affected item required and block its acceptance. Main resolves with research ownership before promotion; no silent feature deletion, inverse replacement, arbitrary clamp or quota reduction. A sourced correction must retain 100 distinct features and update affected vectors/docs/diagrams before closure; any reduction needs explicit user approval.

## Prerequisites and execution order

1. **Research and planning gate:** use the completed research snapshot and this exact 100-entry backlog. Preserve baseline manifest and first-sprint files. Main provisions isolated branches/worktrees without switching another worker's checkout or assuming a remote.
2. **Shared prerequisite P0:** existing schema/validator/stable functions and diagram primitives apply to all features. Main prepares shared registry integration. No feature waits on another lane's implementation; adjacent outputs are conceptual inputs, not runtime dependencies.
3. **Shared prerequisite P1:** Main added zero-safe `sinc(x)=sin(x)/x`, `sinc(0)=1`, context registration and a small regression. Parent executed `cargo test --lib sinc_preserves_dc_and_aperture_response`: one passed, zero failed. Full aperture process/GUI boundary acceptance remains pending. Only `measurement.rectangular_aperture_response` requires this new primitive. Do not exclude DC/zero aperture, add epsilon denominators or present dB at a true null. Other lanes use existing stable functions unchanged.
4. **Development wave:** run sixteen D2 lanes concurrently. Prioritize each lane's topology/conditioning-sensitive items for source implementation first, without reducing the rest: the five high-risk comparisons above, loaded RC/LC/Pierce startup, RLGC near-lossless/near-DC, small-dimensional foil heat, hidden instrumentation clipping, covariance cancellation and aperture zero limits. This is risk order, not a claim of estimated velocity.
5. **Developer integration gate:** Main receives ordered explicit commit lists, feature IDs and unresolved findings from every D2. Integrate all 100 atomic feature commits into `integration/electronics-calculators`, then complete shared registration. No partial or unreviewed count-padding item passes into the QA snapshot.
6. **Independent QA wave:** sixteen T2 testers and sixteen V2 reviewers assess the same complete integrated snapshot. This is **32 simultaneous workers maximum**. Development workers must finish before this full QA wave; any retained developer or repair worker consumes a slot, so reduce active QA slots rather than exceed 32. Reviewers first inspect source/science and later inspect Main's actual native images; initial source-only approval is not final approval.
7. **Main execution and remediation:** integrate explicit tester commits, run the full wave of actual checks once, capture process/native evidence, and fix actionable findings at their root. Failed checks receive focused confirmation after a real fix, not repeated runs to doubt reported failures. Revisit any shared-code blast radius without changing old behavior. Recapture changed diagrams and return changed models/images to their V2 owner for closure.
8. **Release gate:** every item, lane report, execution record and native-image review must close before delivery. No pending feature or deferred diagram is a completed sprint.

## Atomic feature commits and handoffs

Each D2 produces one atomic feature commit per completed calculator: numerical definition, exact-ID diagram and feature docs travel together. Expected scope is **100 independently attributable feature commits**, not sixteen catalog-only batches followed by a disconnected illustration pass. Existing entries in each lane file accumulate normally. No worker edits shared registry/engine files to make its branch stand alone.

A D2 handoff lists exact commit SHAs in order, exact IDs covered, three owned paths, source/example references and any unresolved issues. Main cherry-picks explicit SHAs in that order; do not use wildcard branch merges, implicit ranges, force pushes, shared checkout switches or presumed remotes. Main's shared integration changes stay separate and attributable. Testers also provide ordered explicit commit lists and per-ID behavioral scenarios. No worktree is discarded before its accepted commits and evidence are preserved.

Every handoff goes to the root append-only [communication bus](../../COMMUNICATION.md), addressed to Main and the relevant D2/T2/V2 lane. Use read plus anchored append; stale hashes require reread/retry. Direct messages supplement, never replace, the durable handoff. This scrum assignment does not itself create commits or modify production/research/board files.

## Integration and QA gates — required observations, not assumed passes

### A. Scope, catalog and preservation

- Account for exactly 100 new unique IDs and exact equality with the backlog; total live inventory **228**, each with one mandatory diagram.
- Preserve every baseline ID/model/formula/assumption/reference/input/output/default/example/diagram and visible metadata; no renaming, replacement or accidental registration loss. Compare against the baseline snapshot and original implementation, not just a total count.
- All new definitions use complete finite input/example maps, exact catalog keys, source-backed units/limits and valid defaults. Constraints guard the researched domain before output evaluation. Signed diagnostics are not converted to false validation failures.
- Main integrates every lane map and checks **228 diagram registrations**, including all 128 original maps. No generic placeholder diagram, fake computed plot or fallback for an unimplemented figure passes.

### B. Independent numerical behavior and real process

- T2 writes substantive independent all-output nominal, limiting, invalid-domain and valid-recovery assertions for every owned ID. Use linked full vectors and derivations, conservation/KCL identities, alternate complex/nodal reasoning and boundary-specific tolerances. No source-text, count-only, finite/nonempty, copied-default snapshot or wiring-only substitute tests.
- Main runs the complete Rust suite and relevant Flutter checks/build after all lane work lands, with shared primitive regressions and baseline regressions included. Workers never execute checks in flight.
- Main exercises **all 228 complete example requests through the actual built Rust JSON-lines process**, comparing every researched output and recording ID, input vector, expected/actual output, tolerance, errors and success/failure. Test errors followed by valid recovery and process lifecycle behavior; library tests do not substitute for the real process.
- Mandatory numerical regressions include the distinguishing high-risk models, admitted zero/near-zero/signed limits, integer boundaries, small representable dimensional results and domain rejections in BACKLOG.md. Main must execute P1 before aperture acceptance.

### C. Actual native desktop and images

- Main traverses **all 228 IDs in the real Flutter desktop**. Record each selection's title/category/level, input/output labels and units, preserved baseline metadata and correct diagram; a registry loop or source render is not native traversal.
- Main enters the complete researched example and calculates **all 100 new examples through the native GUI**. Compare every displayed result to the full oracle using declared numerical tolerance plus documented display rounding; reject silent field omission, stale output or mismatched binding. Preserve baseline behavior through its process coverage and existing native journeys.
- Main captures an actual native image for **every one of the 100 new diagrams** and retains ID-to-image mapping, snapshot/commit identity and journey evidence. V2 reviewers inspect both the scientific/source model and the image pixels. A source-only review, SVG/source sketch or unobserved screenshot filename does not pass.
- Check narrow-window scrolling, large-text layout, label/geometry clearance, legend/key consistency, polarity, junctions, contrast and selection/result updates. Include representative complex circuits/geometry and preserve the original 128 diagram coverage/accessibility behavior.
- Exercise native search/filter/navigation, keyboard submission, invalid-input recovery and owned-backend lifecycle recovery using actual process integration. Build/package the desktop release and launch the packaged application **without an engine-path override**, then complete a real calculation and clean close.
- Report only host/platform checks actually observed. Linux success does not imply Windows/macOS runtime, remote CI, hardware accuracy, EMC/safety certification or calibrated metrology.

### D. Review closure and release evidence

Each T2 scenario record identifies feature ID, independent expected values/tolerance, boundary/error rationale and execution request. Each V2 report identifies reviewed integration commit, source references, numerical/topology findings, native image IDs and whether source and pixel acceptance both closed. A missing image is an open gate, not a pass.

Main retains executed check output, all-228 process results, all-228 native traversal, all-100 GUI examples, all-100 native images, packaging/no-override launch evidence and final review closure. Main fixes all actionable findings, supplies focused numerical confirmation or recaptured images, and obtains V2 closure. No report may infer completion from planned paths or research examples alone.

## Current actionable blockers and limits

- **Science/novelty:** no unresolved duplicate or missing source prerequisite identified in planning. The listed semantic conditions are mandatory; simplifying one away blocks that feature.
- **Shared execution gate:** Main's focused `sinc` regression passed. Aperture DC/zero-time/null/negative-lobe cases through the integrated model remain required before acceptance. This does not block independent catalog/diagram authoring.
- **Implementation/acceptance:** all 100 remain planned, not claimed implemented or checked here. Actual 228-process/native traversal, 100 GUI examples, native captures, independent tests/reviews and packaged launch are still required deliverables of the fixed pipeline.
- **Physical limitations:** calibration, idealizations and source-access caveats remain in research and feature docs. Never market engineering estimates as device guarantees or standards certification.

No builds, tests, lint, formatters, process/native runs, screenshots or Git commits were performed by this planning assignment. Main owns those checks and final evidence.
