# Independent BJT review

- Lane: `bjt`; integrated baseline: `7f12f45`.
- Decision: **clear** — no actionable patch-introduced correctness/security finding established.
- `tested=false`: static source/equation review only. No builds, tests, lint, formatting, process smoke, simulator or desktop runs were executed. Tester worktree and half-authored tests were not used.

## Scope and evidence

Read `coordination/CONTRACT.md`, common acceptance and all eight BJT rows in `coordination/BACKLOG.md`, `coordination/SPRINT.md`, all eight complete `backend/catalog/bjt.json` programs, `research/bjt.json`, `research/bjt.md`, and `docs/features/bjt.md`. Inspected the promotion diff from `906b351` to `7f12f45` (both production files are added by this integration), and the receiving-side catalog embedding, ID lookup, constraint execution, sequential output registration and JSON command dispatch in `backend/build.rs`, `backend/src/lib.rs`, and `backend/src/main.rs`. The new IDs are not routed through a lane-specific switch: every JSON catalog is embedded and every definition is registered by ID, then evaluated through the same generic dispatch. Native `expm1` is registered before model evaluation.

Retrieved and inspected these actual cited primary sources during this review (PDF reader extracted text, not a device simulation):

- [MIT 6.101 Lecture 3](https://web.mit.edu/6.101/www/s2019/handouts/L03_4.pdf): BJT current bookkeeping, two-resistor biasing, Early voltage, hybrid-pi parameters, emitter degeneration and emitter follower.
- [MIT/Ron Roscoe low-frequency hybrid-pi chart](https://ocw.mit.edu/courses/6-101-introductory-analog-electronics-laboratory-spring-2007/1bc47ee7271866912a6eb9d2d1596ff5_hybrid_pi.pdf): finite-beta input loading and high-ro assumptions.
- [onsemi AND9129/D](https://www.onsemi.com/download/application-notes/pdf/and9129-d.pdf): digital-transistor resistor topology, saturation test current ratios, current-drive and resistor voltage principles, controller source-current capability.
- [onsemi TND6093/D](https://www.onsemi.com/download/white-papers/pdf/tnd6093-d.pdf): Board Design current-drive requirement and collector-only dissipation estimate.
- [NIST Boltzmann constant](https://physics.nist.gov/cgi-bin/cuu/Value?k) and [NIST elementary charge](https://physics.nist.gov/cuu/Constants/Value/e.html): exact constants used in every thermal-voltage expression.
- [MIT 6.012 Lecture 19](https://ocw.mit.edu/courses/6-012-microelectronic-devices-and-circuits-fall-2009/9932f87a7b5ec3d088d0e0836a05c622_MIT6_012F09_lec19.pdf), slides 10–11: emitter-tail conservation, alpha_F conversion, exponential current ratio and tanh transfer.

All independent conclusions below are algebraic/static evidence, not observed runtime outputs. The high-ro screen is a disclosed engineering approximation, not a source-derived guaranteed error percentage. The affine Early correction, driver-series subtraction and base-terminal power term are explicitly derived extensions rather than claims that a manufacturer supplied those exact calculator equations.

## Eight-model assessment and parent verification scenarios

### 1. `bjt.saturated_switch_drive` — catalog lines 3–127

Clear. Grounded-emitter plain NPN, no internal shunt. KVL independently gives `Rb=(Vdrive−Vbe)/Ib−Rdriver`, with `Ib=Ic/beta_forced`. The strict headroom constraint enforces positive external resistance; capability equality is allowed. Nominal substitutions give 10 mA, 220 ohm and 22 mW. Power is explicitly at the minimum-drive sizing boundary, not worst-case resistor power. A real resistor rounded downward needs a separate maximum-current/power check, already stated. onsemi AND9129 concerns a resistor-equipped device; the documented plain-transistor derivation deliberately does not copy its shunt current.

Parent scenarios: defaults; `r_driver=0`; capability equality at 0.01 A; reject 0.009 A capability; reject `r_driver=240` ohm with otherwise nominal inputs (zero external resistance); reject negative driver resistance/zero collector current. Establish saturation from actual guaranteed datasheet conditions, not active hFE.

### 2. `bjt.early_output_resistance` — catalog lines 129–232

Clear. Differentiating `Ic=I0(1+Vce/VA)` at fixed Vbe gives `go=I0/VA=Ic/(VA+Vce)`, so `(VA+Vce)/Ic` is correct for operating measured current. All denominators are positive. `Vce>Vbe` enforces reverse collector-base bias. Approximation error `Vce/(VA+Vce)` is relative to the full affine result, not to the approximation. Nominal resistance 105 kohm and conductance 9.523809523809523 uS are consistent; DC `Vce/Ic=5 kohm` is correctly distinguished.

Parent scenarios: nominal; `VA=Vce=5` V gives 10 kohm and error 0.5 at 1 mA; verify `go*ro=1`; reject `Vce=Vbe`; doubling Ic halves both resistances and doubles conductance. Device breakdown and local affine validity remain external checks.

### 3. `bjt.differential_pair_steering` — catalog lines 234–361

Clear. Set `S=alpha*Itail` and `x=Vdiff/VT`. Conservation and the ratio require `Ic1=S*exp(x)/(1+exp(x))` and `Ic2=S/(1+exp(x))`. The difference is correctly evaluated with `expm1(x)`; subtracting nearly equal rounded branch currents would lose tiny signed differences. Differentiation yields the implemented `2*S*exp(x)/(VT*(1+exp(x))^2)`. At balance each branch is `S/2`, difference is zero and differential slope is `S/(2*VT)`. The +/-40 guard bounds exponential and denominator-square growth, while documented leakage/compliance/mismatch caveats prevent interpreting extremely weak branch current as a hardware guarantee. No emitter/collector-tail confusion or factor-of-two error found.

Parent scenarios: balance; both signs of 0.02 V; both signs of 1e-20 V with expected first-order difference about +/-3.82987396750828e-22 A at default tail/beta/T; conservation and branch swap; +/-40 VT endpoints and rejection beyond the envelope; 200/450 K inclusive and 199/451 K rejected. The endpoint dominant branch may round to S; this is not evidence that the weak branch should be zero.

### 4. `bjt.divider_bias` — catalog lines 363–535

Clear. Independent base-loop KVL is `(Vth−Vbe)=Ib*(Rth+(beta+1)*Re)`; collector and emitter bookkeeping and all nine outputs agree. The second constraint is exactly the expanded `Vce>Vbe` condition, equivalent to collector above base, not merely positive Vce. Nominal currents and node voltages agree with the complete oracle. Positive bounds protect sums and divisors. This is not a saturating/cutoff solver; rejecting those regions is deliberate.

Parent scenarios: nominal KCL/KVL and loaded-divider identities; `vcc=3.5` V with nominal divider gives cutoff equality and rejection; `r_collector=10000` ohm with other defaults gives a non-forward-active point and rejection; doubling every resistor halves currents and preserves voltages within this model. Real parameter/thermal corners remain external.

### 5. `bjt.hybrid_pi_parameters` — catalog lines 537–597

Clear. Exact SI constants give `VT=kT/q`; derivative of the ideality-one exponential is `gm=Ic/VT`, and `rpi=beta_ac/gm`. DC and incremental beta are explicitly distinguished. T uses absolute kelvin; this is neither an RMS/peak calculation nor a finite-amplitude gain model. Nominal gm and rpi and their product 100 are consistent.

Parent scenarios: double Ic (gm doubles/rpi halves); double beta_ac (rpi doubles/gm unchanged); inclusive 200/450 K; reject zero Ic/beta, temperatures outside envelope; verify `gm*rpi=beta_ac`. Base-emitter signal must be small relative to VT and the transistor forward-active/low-injection externally.

### 6. `bjt.emitter_degeneration_gain` — catalog lines 599–751

Clear for its expressly disclosed high-ro topology. Independent hybrid-pi KVL gives `Rin=rpi+(beta_ac+1)*Re`; collector KCL gives `vout=−beta_ac*ib*(Rc||RL)`. Source gain includes source resistance once. Negative gains, finite-beta emitter current, parallel collector load and all four nominal oracles are consistent. The supplied ro is a validity screen, intentionally not included as a simple parallel collector resistor for a moving emitter. No fixed physical-accuracy percentage is claimed.

Parent scenarios: `Rs=0` makes source/base gains equal; increasing Rs reduces source-gain magnitude without changing base gain; interchange Rc/RL; reject `ro=19999` ohm and accept 20000 ohm at default collector load; verify nominal 2000-ohm load and 11392.599989321776-ohm input. Finite-ro exact simulation is not represented by this model.

### 7. `bjt.terminal_dissipation` — catalog lines 753–846

Clear. Terminal-power conservation with `Ie=Ic+Ib` independently reduces to `P=Vce*Ic+Vbe*Ib`; base power is not accidentally omitted or assigned to the external resistor. Nonnegative magnitudes and zero inputs are permitted. No division, logarithm, RMS/peak conversion or active-region constraint is required for this DC balance. Nominal total is 0.10014 W.

Parent scenarios: all-zero power vector; `Vce=0.1` V, `Ic=0.1` A, `Vbe=0.9` V, `Ib=0.01` A yields collector/base/total 0.01/0.009/0.019 W (saturation allowed); reject negative magnitudes; verify changing common terminal reference leaves the terminal sum unchanged. Switching/storage losses and SOA remain out of scope.

### 8. `bjt.emitter_follower_loading` — catalog lines 848–964

Clear for disclosed high-ro, AC-coupled-load topology. Independent emitter KCL gives `ie=(beta_ac+1)*ib`; hybrid-pi KVL gives `Rin=rpi+(beta_ac+1)*(Re||RL)`. Emitter output over base/source input gives both implemented positive gains. Because every resistance and rpi is positive, ideal-model gains satisfy `0<Av_source<=Av_base<1`. Nominal 500-ohm effective emitter load, 51792.599989321774-ohm Rin and both gains agree. Collector AC ground makes finite ro an emitter load, but omission is openly stated and screened rather than silently advertised as an exact finite-ro result.

Parent scenarios: `Rs=0` equality; interchange Re/RL; reduced load reduces Rin/base gain; accept ro=5000 ohm and reject 4999 ohm at defaults; temperature/positive-current/beta bounds. Independent DC bias/headroom and small junction signal remain assumptions.

## Findings and untested scope

**Actionable findings: none.** All eight formulas, declared units, input-only guards, signed/zero domains, ordered dependencies, nominal full oracles and substantive physical caveats were assessed. The differential-pair near-balance cancellation present in research is already corrected in executable catalog/docs using native `expm1` without changing the model.

No runtime numerical pass, independent test-suite pass, UI usability pass, exhaustive binary64-extreme accuracy guarantee, standards compliance, transistor SOA qualification, device-specific saturation guarantee or hardware measurement is claimed. PDF equations were read from extracted text and cross-checked by algebra; no SPICE comparison was run. Unbounded positive scalars can produce nonrepresentable results and the inspected shared engine returns nonfinite expression results as errors; the catalog is not claimed accurate over physically meaningless extreme scalars. Parent must execute its independent all-output examples, proposed boundaries/identities and desktop/protocol acceptance after QA integration.
