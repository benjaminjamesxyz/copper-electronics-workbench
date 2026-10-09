Please materialize this complete reviewer report as coordination/reviews/timing.md (my filesystem write tools reject report/bus paths; read-only bash cannot substitute), then append completion/report link plus P2 finding to COMMUNICATION.md. Content follows; no production changes requested.

# Independent review — timing lane

- Reviewer: ReviewTiming; integrated baseline: `7f12f45`.
- Decision: **findings** — one P2 numerical-validation integration defect. All eight requested models reviewed; no formula/topology/unit defect identified within their stated physical assumptions.
- `tested=false`. No builds, lint, tests, formatters, smoke commands, executable launches or independent tester files were used. Static algebra/source review is not a runtime pass.
- Scope: `backend/catalog/timing.json`, `research/timing.json`, `research/timing.md`, `docs/features/timing.md`, timing BACKLOG/Sprint/Contract; plus `frontend/tool/build_desktop.dart` and `.github/workflows/desktop.yml`. Consumer tracing additionally inspected backend catalog embedding, Engine::new/calculate/respond, the actual meval 0.2.0 arithmetic evaluator, and Flutter catalog/results/executable-path consumers.

## Actionable finding T1 — P2: reject masked overflow in crosstalk attenuation

**ID / patch anchor:** `timing.capacitive_crosstalk_peak`, `backend/catalog/timing.json:210` (peak expression; newly introduced catalog). Related consuming code: `backend/src/lib.rs:100-103` checks only each complete expression's final finiteness. meval 0.2.0 `src/expr.rs:42-105` executes raw f64 operators/functions and pushes their results without intermediate finiteness checks.

**Accepted complete input vector:** `r_victim=1e-200 ohm`, `c_mutual=1e-100 F`, `c_victim=1e-100 F`, `voltage_swing=1 V`, `edge_time=1e10 s`, `segment_delay=0 s`. All supplied values are finite, satisfy scalar bounds, and pass the segment-delay constraint. `tau≈2e-300 s` is representable; `edge_time/tau≈5e309` overflows binary64 to infinity.

**Static codepath / impact:** `expm1(-infinity)=-1`, so the attenuation quotient is `1/infinity=0`. The divider factor is 0.5, leaving a finite `peak_excursion=0` and `fast_edge_limit=0.5`. Engine's final-only check accepts that fabricated zero. This contradicts the catalog/docs' explicit promise that nonfinite intermediates are rejected. It is not an ordinary precision loss at an exact-zero result: the analytic large-x peak is `R_v*C_m*DeltaV/edge_time≈1e-310 V`, nonzero and still representable. The catalog's stated policy requires an explicit arithmetic-range error, not silent zero. No physical claim is made for these extreme engineering inputs; they establish a reproducible accepted-domain arithmetic contract violation.

**Independent evidence versus inference:** Catalogue/input/evaluator/dependency code is directly inspected. Infinity propagation and the large-x analytic limit follow IEEE-754 arithmetic and the independently derived KCL response; actual process output is **unexecuted [INFERENCE]**. MIT 6.071 equation 1.5 supplies `i=C dv/dt`; its KCL discussion supports the topology derivation, not an engine-specific expectation.

**Correction:** Validate `r_victim*(c_mutual+c_victim)>0` and evaluate `edge_time/(r_victim*(c_mutual+c_victim))>0` as input-only constraint sides (the current complete-expression finiteness check then rejects infinite ratios), or implement checked intermediate evaluation centrally. Do not introduce an arbitrary minimum positive ratio or clamp/fallback: small representable x is physically valid. Apply matching numerical caveats/docs if the implementation policy changes.

**Parent verification scenario (not executed):** Complete vector above must return an explicit error rather than successful zero. Also cover underflowed tau (`r_victim=1e-200`, `c_mutual=c_victim=1e-200`, other same values), ratio underflow to zero, nominal x=1, small representable x≈1e-16, and zero coupling/swing; the last cases must retain supported physical zero/fast-edge behavior. Main owns verification and shared evaluator changes; lane author owns any catalog/doc guard fix.

## All-eight independent model assessment

### 1. `timing.timer_clock_divider` — catalog lines 3–122

Independently counted inclusive timer ticks: a compare value K produces K+1 timer clocks per toggle; two toggles per output period give `f_clk/[2P(K+1)]`. Inspected Microchip datasheet section 17.9.2 explicitly confirms this and prescalers 1,8,64,256,1024. `floor(ideal_count+0.5)` is nearest-period rounding, intentionally not minimum frequency error. Maximum-target guard implies count>=1; rounded-count upper guard gives compare<=65535. Polynomial equality restricts the prescaler set. All Hz/seconds/% outputs and nominal complete oracle are consistent: count333, compare332, period333us, frequency3003.003003003Hz, error+0.100100100100%. Parent boundary scenarios: counts1 and65536, half-integer rounding, invalid prescaler/count/frequency. No actionable defect found.

### 2. `timing.capacitive_crosstalk_peak` — lines 124–246

Derived the stated victim KCL independently: `(Cv+Cm)dv/dt+v/Rv=Cm*dVa/dt`; constant ramp forcing gives monotone rise, end-of-ramp peak and subsequent decay with `tau=Rv(Cv+Cm)`. Thus divider-limit times `-expm1(-x)/x` is correct; for x>0 its attenuation is in (0,1]. `expm1` and quotient-first ordering preserve the x≈1e-16 fast-edge limit. Nonnegative swing/coupling and positive shunt capacitance/resistance/duration agree with selected domain; zero coupling/swing yields zero at ordinary representable ratios. Nominal tau20ns, divider1V, peak1-exp(-1)=0.6321205588285577V agree with complete oracle. Full-ramp versus10–90% and lumped/distributed/clamp exclusions are honest. T1 is the remaining range-validation defect; ordinary physical formula is correct.

### 3. `timing.uart_sampling_margin` — lines 248–390

Independently placed transmitted stop interval at `(1+D+P)/Btx` through `(2+D+P)/Btx` and sample interval at `n/Brx` through `n/Brx+1/(S*Brx)`, n=1+D+P+0.5. Early/late boundary distances and guard subtraction are correct. Sign convention for mismatch is receiver-relative-to-transmitter; negative margins remain diagnostic successes. Bounds plus floor/polynomial constraints enforce D integer5–9, P0/1 and S8/16. Nominal complete oracle follows these times: stop center979.381443299us, quantization6.443298969us, early41.881443299us, late55.841924399us, minimum41.881443299us, mismatch+1.041666666667%. Additional independent equal-clock case at9600 8N1/16x/G0: early=1/(2B), late=7/(16B), so minimum=45.5729166667us. Slower receiver9000 produces a negative late margin (~−20.8333333333us). Manufacturer three-sample voting/start filtering is explicitly not claimed; inspected AVR datasheet sections22.8.1–.3 confirm why receiver-specific caveats are needed. No actionable formula/domain defect found; general engine nonfinite-intermediate limitations remain relevant at arbitrary rate extremes, not a separate duplicate finding.

### 4. `timing.plate_displacement_current` — lines 392–516

From `D=epsilon*V/gap`, derived `J=epsilon/gap*dV/dt`, `C=epsilon*width*length/gap`, and `I=C*dV/dt`. Inspected MIT6.013 equations3.1.10–.12/3.2.1 confirm parallel-plate capacitance, capacitor current and displacement term. Fixed epsilon0 and exact c are appropriately disclosed engineering constants. Units F,V/s,A,A/m² and signed swing are coherent; no RMS conversion is applicable to a full linear edge. Geometry/quasistatic constraints consistently select approximate no-fringing/electrically-short regimes; explicit lack of an accuracy/emissions/isolation guarantee is appropriate. Nominal complete oracle: C42.50010150144pF, slew5e9V/s, I0.2125005075072A, J354.167512512A/m². Sign inversion and zero swing preserve exact physical signs/zeros; parent should test equality and rejection for both geometric guards. No actionable defect found.

### 5. `timing.i2c_pullup_bounds` — lines 518–611

Independently derived sink current at VOL and 30–70% RC crossings. Difference is `RC*ln(7/3)`, not10–90%ln9; lower bound `(Vs−VOL)/Isink` matches inspected TI SLVA689 equations1–6 and worked example. Positive denominator guards and VOL<Vs are correct; negative interval width is intentionally preserved. Full nominal oracle is consistent: Rmin966.6666666667ohm, Rmax1770.3337517157ohm, width803.6670850491ohm. Weakest-device/current-mode/tolerance and noncompliance caveats are honest; no inferred resistor tolerance or fabricated guarantee. Parent should exercise VOL0, VOL>=Vs rejection, and a negative-width bus configuration. No actionable defect found.

### 6. `timing.schmitt_rc_delay_bounds` — lines 613–752

Independently solved zero-initial RC charging for VT crossing, `-RC ln(1−VT/Vs)`, and checked monotonic ordered threshold/delay corners. Actual TI SN74AHC14 section8.2.2 confirms RC/threshold formula; section5.5 supports guaranteed rising thresholds, not universal50% timing. Native `log1p` is registered in consuming engine and preserves tiny positive ratios (5e-17V at5V with RC2us and gate delays0 gives leading crossing2e-23s). Original research uses algebraically equivalent ln; docs explain numerical cutover honestly. Gates preserve positive RC/supply, ordered positive thresholds below final voltage, nonnegative ordered gate delays and equality-width0. Full oracle RC2us, earliest1.39629436112us, latest2.79258872224us, window1.39629436112us is correct. Illustrative default thresholds are explicitly not asserted to be SN74AHC14 specifications. No actionable physical/formula defect found; floating-point range-exhaustion handling is part of shared numerical limitation identified by T1.

### 7. `timing.synchronous_setup_slack` — lines 754–854

Independently placed data arrival CQ+path and capture at T+capture-minus-launch skew. Latest allowed arrival subtracts signed library setup and nonnegative worst-case uncertainty. Inspected UMBC sections20–23 explicitly confirm positive skew helps setup and slack formula including uncertainty; section18 permits negative setup. All inputs/results seconds; RMS jitter is explicitly excluded from worst-case penalty. Complete nominal oracle9.8ns available,7.5ns arrival,+2.3ns slack is correct. Negative setup/skew/slack and zeroCQ/path/uncertainty are deliberate domains; hold/CDC/OCV/CPPR are not claimed. Parent should test signed setup/skew and negative slack without rejection. No actionable defect found.

### 8. `timing.inductive_turnoff_spike` — lines 856–965

Independently applied passive-sign `v=L di/dt` to falling positive current and KVL in the stated low-side topology; spike magnitude is L*(Ii−If)/tf, switch voltage adds supply. Inspected MIT6.071 equation1.27 and TI SNOA382 section4.7 substantiate constitutive law/parasitic transient, while supply addition is a topology-specific derived extension, honestly labeled. No gate-fall-time, avalanche/ringing/SOA prediction is implied. Full oracle slew4e8A/s, spike16V, switch28V is correct. L0 and Ii=If are allowed zeros; increasing current/nonpositive interval reject. Parent should cover these zeros and rejected orientation-domain cases. No actionable defect found.

## Actual source evidence ledger

Directly inspected during this review:
- [TI SLVA689](https://www.ti.com/lit/pdf/slva689), equations1–6 and example.
- [TI SN74AHC14](https://www.ti.com/lit/ds/symlink/sn74ahc14.pdf), sections5.5/8.2.2; slow-input Schmitt suitability/application limits.
- [Microchip ATmega datasheet DS40002211A](https://ww1.microchip.com/downloads/en/DeviceDoc/ATmega640-1280-1281-2560-2561-Datasheet-DS40002211A.pdf), section17.9.2 toggle frequency/prescalers and22.8 UART oversampling/voting/operational limits.
- [MIT6.071 capacitor/inductor notes](https://ocw.mit.edu/courses/6-071j-introduction-to-electronics-signals-and-measurement-spring-2006/eac1995aca830cfd0de6bab60a8abf10_capactr_inductr.pdf), equations1.5/1.27 and capacitor KCL discussion.
- [MIT6.013 chapter3](https://ocw.mit.edu/courses/6-013-electromagnetics-and-applications-spring-2009/be09fe40701d8ba037ddc18fea301449_MIT6_013S09_chap03.pdf), quasistatic introduction,3.1.10–.12 and3.2.1.
- [TI/National SNOA382](https://www.ti.com/lit/pdf/snoa382), sections3.6/4.5–4.7, capacitive versus inductive/distributed coupling and Ldi/dt.
- [UMBC timing analysis](https://eclipse.umbc.edu/robucci/cmpeRSD/lectures/Physical_Design_Flow_and_Timing_Analysis/Physical_Design_Flow_and_Timing_Analysis.html), sections18–23.

Direct Microchip HTML CTC and first UART URLs returned HTTP503; indexed web search also failed. They are **not claimed inspected**. Available cited Microchip PDF supplied independent actual manufacturer evidence for CTC and receiver caveats. TI SCEA046 was not separately fetched because inspected SN74AHC14 already supplies the material Schmitt slow-input/threshold evidence. Reviewed equations derived here from cited laws are not falsely presented as verbatim manufacturer acceptance models.

## Native executable placement / workflow review

`frontend/tool/build_desktop.dart:14–27` builds Rust release then Flutter host-platform release and copies `pcb-calculator[.exe]` into the application's executable directory: Linux `build/linux/{x64|arm64}/release/bundle`, Windows `build/windows/x64/runner/Release`, macOS `pcb_design_calculator.app/Contents/MacOS`. Inspected Linux/Windows CMake BINARY_NAME and macOS AppInfo.xcconfig PRODUCT_NAME match the expected app name. `frontend/lib/backend_client.dart:22–30` consumes the adjacent file path based on Platform.resolvedExecutable, with explicit environment override and shell-free engine Process.start. Thus no silent desktop-path routing mismatch is visible for the wrapper's stated host/default targets.

Platform assumptions remain **unverified at runtime**: Linux architecture detection depends on Dart Platform.version containingarm64; Windows destination is specificallyx64; Cargo output assumes backend/target without custom target-dir/target override; macOS single Cargo-native engine is not evidence of a universal bundle. These are inspected assumptions, not evidence-backed defects under default stated usage and not expanded distribution promises. File executable permissions, signing/notarization, OS architecture compatibility, packaged process launch and artifact extraction behavior were not exercised.

`.github/workflows/desktop.yml` contains Linux/Windows/macOS build matrix, target-specific Windows smoke executable path, Rust tests and all-research process smoke, Flutter analysis, wrapper packaging and platform-specific artifact paths. Only Linux has an actual Flutter integration-test step, and that step points at the debug backend environment override, not the copied release backend. Therefore workflow presence/build/upload is **not native packaged-runtime acceptance**, especially not Windows/macOS acceptance. No OS pass is inferred; parent must separately launch each packaged artifact without override and verify child process startup, actual examples, permissions and native target compatibility. No actionable build-path defect was established statically; no CI workflow was run.

## Integration dispatch and remaining acceptance

All eight JSON definitions are included by backend's generic catalog embedding/iteration. Engine compiles every sequential output and input-only constraint, registers nativeexpm1/log1p, looks up all IDs and routes both catalog/calculate requests through explicit Request branches. Flutter generically maps all metadata/inputs/outputs and sends selected IDs; no timing-specific variant or silent unmatched dispatch was found. T1 lies specifically at the expression evaluation boundary despite correct producer algebra.

Complete research/default/example maps and documented units match promoted definitions (Schmitt's approved algebraically equivalentlog1p is the documented numerical cutover). Independent oracles above derive from laws/time boundaries rather than runtime engine or tester-generated expectations. Hardware models remain idealized closed forms, not simulation, I2C/UART compliance, STA, EMC or safety certificates.

Main should execute independent timing behavioral tests, all complete examples/defaults, specified signed/zero/discrete/domain cases, short/longx and nonfinite-range handling, then real packaged desktop checks on each available OS. Those checks are **prescribed, not executed**. Report status concerns reviewed baseline only; any remediation needs explicit parent verification and independent disposition.

Persistence note: ReviewTiming completed the above review and delivered findings/report content directly to Main. Reviewer report/bus file writes are unavailable in its mounted tool set; Main materializes this artifact and appends the completion link without altering reviewer conclusions.


## Reviewer handoff addenda

Precision correction for ReviewTiming finding/report before materialization: exact peak expression anchor is backend/catalog/timing.json:202, NOT210 (202 found by numbered grep). Engine final-only evaluator is lib.rs:99–102. Update all T1/bus anchors to202. Correct model assessment ranges from numbered ID/closure anchors: timer3–118; crosstalk120–235; UART237–386; plate388–511; I2C513–615; Schmitt617–761; setup763–861; inductive863–966. Earlier broad per-model ranges were approximate headings, not exact. T1 accepted vector/algebra unchanged. Please apply these replacements in report content just sent; final result will also carry correct202.

ReviewTiming final handoff: lane=timing; decision=findings; reviewed=8/8; tested=false; report_path=coordination/reviews/timing.md; report_persisted=false (read confirms absent). One P2: timing.capacitive_crosstalk_peak at backend/catalog/timing.json:202, masked nonfinite attenuation ratio; full accepted vector/correction/parent verification in prior messages and complete Markdown report. Native wrapper/workflow statically reviewed; Windows/macOS/Linux packaged runtime all unverified by reviewer. Report and bus filesystem writes are impossible under mounted tool contract (explicit rejection); please materialize report with corrected anchors and append completion link centrally. All reachable review work complete, no production/test/research/doc edits or executed checks.
