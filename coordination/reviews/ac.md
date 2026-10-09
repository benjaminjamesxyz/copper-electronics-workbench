Please persist the following full Markdown as coordination/reviews/ac.md (my direct filesystem write is unavailable). Append bus text after the report. This is the complete independent ReviewAc deliverable, tested=false.

# Independent review — AC impedance and resonance

- Lane: `ac`; reviewed integrated development baseline: `7f12f45` (AC patch compared with `906b351`).
- Decision: **findings** — one P2 numerical correctness finding; all eight models independently reviewed.
- Scope: `backend/catalog/ac.json`, `research/ac.json`, `research/ac.md`, `docs/features/ac.md`; coordination contract and AC backlog/sprint acceptance. Read-only examination of catalog embedding, shared evaluator, JSON-lines dispatch, Flutter generic catalog/input/output consumers. No implementation edits or commits.
- Execution: **tested=false**. No builds, tests, lint, formatters, process smoke or arithmetic scripts were run. Numerical counterexample below is an analytic/binary64 representation proof, not an observed runtime result.

## F1 — P2: Rationalize nearly equal power-factor correction

**Calculator/location:** `ac.shunt_power_factor_correction`; `backend/catalog/ac.json:191`, expression for `capacitor_reactive_power`. `required_capacitance` (192) and `capacitor_current_rms` (195) inherit its relative error. All lines are introduced in the AC implementation patch.

**Accepted input vector:** `real_power=1e12 W`, `voltage_rms=230 V RMS`, `frequency=50 Hz`, `initial_pf=0.5`, `target_pf=0.5000000000000002`. Every scalar is inside its inclusive range and target >= initial.

**Independent expected behavior:** Let t(x)=sqrt(1-x²)/x and delta=2^-52. The decimal target parses to 0.5+delta. Then Qc=1e12[t(0.5)-t(0.5+delta)]. Since t'(x)=-1/[x² sqrt(1-x²)], Qc=1e12*(8/sqrt(3))*delta+O(1e12*delta²), approximately **0.00102558009940457 var**, strictly positive. Capacitance must equal Qc/(2*pi*50*230²), and capacitor RMS current must equal Qc/230. This is the same physical model as Illinois slides 47 and 56–58, not a changed physical approximation.

**Static defect evidence:** The current expression separately evaluates the two t values near sqrt(3), both in binary64 binade [1,2). Each is a multiple of 2^-52, so their subtraction is an integer multiple of 2^-52; subtraction itself is exact here by Sterbenz's lemma. The true small difference is approximately **4.618802153517006** of these spacings. Even the nearest integer (5) differs by over **8%**; other possible rounded differences are worse. The final multiplication's relative rounding cannot remove this error. Both terms individually have normal floating-point accuracy, but their difference does not. Bounds/finite-output guards accept the misleading result.

**Correction:** Evaluate the difference in rationalized form, with a=initial_pf, b=target_pf:
`(b-a)*(b+a)/(a*b*(b*sqrt((1-a)*(1+a))+a*sqrt((1-b)*(1+b))))`.
This removes subtraction of nearly equal large terms and factors `1-PF²` near unity. Preserve the explicitly supported equal-PF result, including a=b=1, by handling that removable singularity correctly (e.g. a parent-owned stable scalar helper if the expression grammar cannot represent the required equality case). Do not narrow the accepted PF range or clamp a nonzero correction to zero. Update formula-conditioning documentation with the actual final implementation.

**Parent verification scenarios:** Compare the complete adjacent-PF vector to an independent high-precision oracle at 1e-6 relative tolerance for nonzero Qc/C/Ic; include a more ordinary small PF increment, default 0.8→0.95, unity target from lagging initial, equal PF below unity and both PF=1 exactly. Preserve target<initial rejection and monotonic/nonnegative sizing. These scenarios are prescribed, not executed.

**Confidence:** 0.99. Evidence is inspected source/expression plus analytic representation proof. Exact current backend numeric output is unobserved, and no runtime pass/fail claim is made.

## Eight-model review matrix

1. **`ac.inductor_reactance` (lines 3–22):** Independently derived Z=j*omega*L from v=L di/dt. XL=2*pi*f*L, I=V_RMS/XL, phase +pi/2 match MIT M1 12.2.11–17. SI dimensions and RMS convention correct. Positive f/L bounds guard the reciprocal; V=0 is permitted and reported phase is impedance phase. Endpoint products/current stay finite. No actionable finding.
2. **`ac.lossy_coil_antiresonance` (25–48):** Independently added 1/(r+j*omega*L)+j*omega*C, obtaining B=omega*C-omega*L/[r²+(omega*L)²]. Positive-frequency B=0 gives omega²=(L-C*r²)/(L²*C), Z=L/(C*r), Icoil=V*sqrt(C/L), copper loss=Icoil²*r=V²/Z. Strict L>C*r² and positive r correctly reject DC-only/no-resonance and ideal infinite tank limits. The source/coil current distinction is correct. Expert topology is series-loss coil in parallel with C, not three shunt branches. Phase resonance is explicitly not asserted to be maximum impedance. Near-threshold subtraction is intrinsically sensitive and its caveat is present; bounded products are finite. No actionable finding.
3. **`ac.capacitor_reactance` (51–71):** Independently derived I=j*omega*C*V, Z=-j/(omega*C). Negative imaginary sign/phase, positive magnitude and RMS loading agree with MIT M1 12.2.18–24. Positive f/C guard division, zero V remains valid; endpoint reciprocal and currents remain finite. No actionable finding.
4. **`ac.series_rlc_impedance` (74–97):** Independently summed R+j*omega*L-j/(omega*C). Magnitude, atan(X/R), RMS current and R/|Z| displacement PF are correct; strictly positive R makes atan's quadrant valid and prevents resonant infinite current. Below/above resonance signs match MIT M1 12.3.6/15 and 12.4.4/5. All intermediate squares bounded far below overflow; near-resonance cancellation caveat and scale-aware zero handling are explicit. No actionable finding.
5. **`ac.parallel_rlc_impedance` (100–124):** Independently summed G+jB with G=1/R and B=omega*C-1/(omega*L), then conjugate-inverted. Real/imaginary/modulus/phase and V_RMS|Y| are correct; G>0 guards every inversion and atan, including B=0 where Z=R. Correct topology distinction from F2 coil model, S/ohm units and inductive/capacitive phase reversal. Finite envelope protects squared admittance. No actionable finding.
6. **`ac.series_resonator_bandwidth` (127–147):** Independently imposed resistor power half its voltage-driven resonant value, giving |omega*L-1/(omega*C)|=R. Positive quadratic roots, Q=sqrt(L/C)/R, bandwidth=R/(2*pi*L), and fL=f0²/fH agree with MIT M2 1.9–19. Lower root formulation avoids low-Q cancellation; f0 is geometric endpoint mean, not arithmetic. Positive components and inclusive envelopes keep squares/roots/division finite. Extremely high-Q endpoint-difference resolution limit is disclosed; direct bandwidth remains correct. No actionable finding.
7. **`ac.sinusoidal_power_triangle` (150–172):** Independently period-averaged sqrt(2)V cos(omega*t)*sqrt(2)I cos(omega*t-phi). P=VI cos(phi), Q=VI sin(phi), S=VI and PF=cos(phi) match Illinois 45–48 and MIT M1 12.4.4. Passive phase guard runs before trig; leading negative Q is preserved. Strict positive RMS amplitudes make phase meaningful; purely reactive endpoint cosine residue is honestly disclosed. No actionable finding.
8. **`ac.shunt_power_factor_correction` (175–199):** Ideal single-phase physical formula/topology, positive capacitor var magnitude, V_RMS² denominator and source currents are correct; target>=initial, equality and unity domains are intentional. Illinois correction derivation corroborates model; harmonic/bank safety limitations and no three-phase claim are honest. One actionable conditioning finding F1; no separate formula/unit/guard finding.

## Sources actually inspected in this review

- **M1:** https://ocw.mit.edu/courses/8-02t-electricity-and-magnetism-spring-2005/77f396b6d9907db677e2b94da90613be_cha12ac_circuits.pdf — directly fetched PDF extraction; inspected constitutive laws/RMS definitions, series impedance/phase and RMS power/half-power derivations in sections 12.2–12.4.
- **M2:** https://ocw.mit.edu/courses/6-071j-introduction-to-electronics-signals-and-measurement-spring-2006/5bcec4bfba5f2e99754b77509e9e7ab4_resonance_qfactr.pdf — directly fetched PDF extraction; inspected exact series roots/product, Q/bandwidth and parallel impedance equations 1.9–1.27. This is ideal university analysis, not experimental validation.
- **G:** http://hyperphysics.phy-astr.gsu.edu/hbase/electric/parres.html — directly inspected text explaining phase versus impedance-max resonance and admittance addition. Formula images were **not decoded**, and no uninspected image equation is claimed as evidence. Lossy-coil identities were independently derived from the displayed topology/admittance.
- **I:** https://courses.grainger.illinois.edu/ece333/fa2018/handouts/ECE_333_Lecture%202_Power%20System%20Basics.pdf — directly fetched PDF extraction; inspected RMS conventions and slides 45–48, 52–58. The document's illustrative mechanical-output wording was not adopted: catalog/docs correctly specify real electrical input power.

No manufacturer/Eaton content, hardware measurement, standard compliance or neglected-parasitic validation is claimed. Research provenance/access caveats are retained. The numerical 1e-6 example tolerance is correctly separated from physical accuracy and component ratings.

## Consumer/cross-boundary review

`backend/build.rs` embeds all JSON catalogs; `Engine::new` deserializes/compiles each catalog and registers every ID; `Engine::calculate` applies exact named finite inputs/bounds and input-only constraints before sequential outputs. `respond` explicitly routes Calculate to `Engine::calculate`; `main` frames every response as JSON-lines. Flutter catalog selection/input forms and result rendering iterate metadata and output keys generically, so none of the eight AC IDs/outputs is silently dropped by an unmatched lane dispatch. This is static path inspection, not executed integration verification.

## Untested scope and handoff

All eight physical models, numerical domains, default/example consistency, signs/units, stated assumptions and citations were reviewed independently of the tester worktree. No tester files were read or relied upon. All runtime tests, independent high-precision executions, rendered desktop interaction, packaging/platform checks, real-component validation and formula-image decoding remain unexercised by this reviewer. Main owns remediation and execution of prescribed scenarios. One P2 finding remains open at the reviewed baseline.

--- END REPORT ---

Append-only bus payload (read latest anchor before append):

## ReviewAc → Main / Dev-ac / Test-ac: review complete, one P2 finding
- Integrated baseline `7f12f45`; all eight AC models independently reviewed, including directly inspected MIT/Illinois PDF derivations and HyperPhysics textual phase/admittance evidence. Complete report: [coordination/reviews/ac.md](coordination/reviews/ac.md). `tested=false`; no builds/tests/lint/formatters/smoke/arithmetic scripts run.
- Actionable F1: `ac.shunt_power_factor_correction`, `backend/catalog/ac.json:191`. Accepted `P=1e12 W`, `V=230 V RMS`, `f=50 Hz`, `initial_pf=0.5`, `target_pf=0.5000000000000002` requires analytically approximately `0.00102558009940457 var`, but separate binary64 tan terms guarantee >8% relative cancellation error inherited by C/Ic. Rationalize with factored roots and preserve equal/unity zero cases. Parent owns fix and high-precision/relative-sensitive verification; no current-runtime value claimed. Other seven models have no actionable findings.
- Reviewer file-write device was unavailable; Main persists this handed-off report and anchored bus append. No implementation edits or git commit.


## Reviewer handoff addenda

Small report-text correction before persisting: in review matrix item 5 replace 'Correct topology distinction from F2 coil model' with 'Correct topology distinction from the lossy-coil model' (only F1 is a finding). Full report and append payload were sent. Please acknowledge when coordination/reviews/ac.md and COMMUNICATION.md entry are saved so acceptance can close; all substantive review work is complete.
