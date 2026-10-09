PERSISTENCE HANDOFF — complete ReviewMagnetics report below for coordination/reviews/magnetics.md. Direct filesystem write unavailable (write rejects non-xd; no edit device mounted; review bash is read-only). Please persist this verbatim and append bus completion below. This is a complete independent all-eight review, not a runtime pass.

# Independent review: magnetics

Baseline: 7f12f45. Decision: findings. tested=false. Reviewer: ReviewMagnetics. Production/research/tests/docs were not edited. No builds, tests, lint, formatting, smoke or numerical scripts were executed. Tester worktree was not read or used as an oracle.

## Scope and integration

Reviewed all eight objects in backend/catalog/magnetics.json, complete research/magnetics.json and research/magnetics.md, complete docs/features/magnetics.md, shared CONTRACT.md and magnetics BACKLOG.md rows / SPRINT.md. Read introducing feature history and complete lane patch boundary (git diff 9e18620^ 7303b0a); fixed G1 integration 7f12f45 is the review target. The immediate parent-diff of 7f12f45 has no magnetics changes because the lane was integrated earlier.

Consumer inspected: backend/src/lib.rs. CATALOGS entries are deserialized into Calculation; every output expression is compiled. Engine::calculate validates exact input keys, finiteness and bounds, applies every input-only constraint, then evaluates all outputs in list order and inserts each output into expression context and result map. respond routes Calculate to the selected ID without lane filtering. log1p is registered. No missing output/ID dispatch branch was found. Expression evaluation checks final finiteness; finite underflow to zero does not trigger an error.

## Findings requiring correction

### MAG-R1 — P2: preserve toroid logarithm near equal radii

ID: magnetics.rectangular_toroid_inductance. File: backend/catalog/magnetics.json:189 (actual expression line; initial incremental tool finding selected adjacent added closure line190).

Complete accepted vector: inner_radius=3; outer_radius=3.0000000000000004; height=0.005; relative_permeability=100; turns=50; peak_current=0; linear_flux_limit=0.2. All bounds, radial ordering, integer turns and zero-current flux guard pass mathematically. These are conditioning probes, not a practical toroid build.

Independent expected behavior: the represented outer radius is 3+2^-51. Therefore x=(ro-ri)/ri=2^-51/3≈1.4802973661668753e-16 and L=0.00025*log1p(x)≈3.700743415417188e-20 H; Binner=0. Forming ro/ri first rounds it to 1+2^-52, producing L≈5.551115123125782e-20 H, about 50% high. The positive finite wrong result is accepted by the shared evaluator.

Evidence: catalog expression inspected; independent binary64 rounding / algebra; directly inspected MIT 8.02 example 2 radial integral. Runtime prediction is static inference, not an observed result.

Correction: use log1p((outer_radius-inner_radius)/inner_radius) in the near-unity regime, maintaining range safety for wide radii; no clamp or arbitrary exclusion of valid thin annuli. Update feature conditioning docs. Verification scenario: full vector above at relative 1e-6 on L and exact zero on Binner, plus nominal annulus and wide-ratio cases to avoid regressing the large-argument regime.

### MAG-R2 — P2: avoid reciprocal overflow before skin-depth square root

ID: magnetics.conductor_skin_depth. File: backend/catalog/magnetics.json:453.

Complete accepted vector: frequency=1e-310; conductivity=1; relative_permeability=1; relative_permittivity=1. All positive bounds pass; displacement threshold is approximately 5.56e-319 S/m and is much smaller than conductivity. This is a finite-domain numerical probe, not a practical frequency/device claim.

Independent expected behavior: delta=1/(sqrt(pi)*sqrt(f)*sqrt(mu0)*sqrt(mur)*sqrt(sigma))≈5.032921210448704e157 m, representable. Current expression first forms ~1/(3.947841760435743e-316), which overflows before sqrt. Engine::calculate consequently returns its nonfinite error instead of a valid finite model result.

Evidence: catalog / consuming evaluator inspected; binary64 range inequality; directly inspected MIT CBA equation 7.33. Failure path is a static inference, not an executed process observation.

Correction: use inverse product of square roots or equivalent scaled formulation; preserve both positive-frequency domain and the good-conductor screen. Update docs. Verification: vector above must succeed with finite delta at relative 1e-6; retain nominal copper example, f^-1/2 / sigma^-1/2 / mur^-1/2 identities, and threshold equality/rejection.

### MAG-R3 — P2: do not underflow B squared before dividing by permeability

ID: magnetics.field_energy_density. File: backend/catalog/magnetics.json:729.

Complete accepted vector: flux_density=1e-162; relative_permeability=1; volume=1e300; linear_flux_limit=0.3. Bounds and linear-field limit pass. This deliberately extreme volume is an arithmetic range probe, not a practical physical region.

Independent expected behavior: u=B²/(8*pi*1e-7)≈3.978873577297384e-319 J/m³; W=u*V≈3.978873577297384e-19 J. Both are representable (u subnormal). B²≈1e-324 is below half the least positive binary64 subnormal and rounds to zero first, so both current outputs become zero; no error is emitted for finite zero.

Evidence: catalog and sequential output consumer inspected; binary64 range and direct algebra; inspected MIT Lecture11 equation9 and MIT 8.02 energy-density examples. Predicted outputs are static inference, not observed runtime.

Correction: scale B by sqrt(2*mu0*mur) before squaring, or another range-safe equivalent, preserving zero-field behavior and no fabricated floor. Update conditioning docs. Verification: use a quantization-aware absolute tolerance around ~4e-319 J/m³, not relative 1e-6 there, and a justified relative/scale-sensitive tolerance on ~3.978873577e-19 J; keep nominal, zero-field and field-limit tests. This avoids a blanket absolute 1e-6 masking the bug.

## All-eight model assessment and independent checks

| Calculator | Formula/topology/units/guards assessment | Independent algebra and prescribed verification |
|---|---|---|
| turns_ratio | Correct ideal shared-flux turns/RMS ratio and inverse-square resistive reflection. Positive integral turns and Rload>0; Vp=0 intentionally permitted. Sequential load/a/a improves range without altering model. No additional finding. | Vs/Vp=Ns/Np and primary load=Rload*(Np/Ns)^2; nominal a=.1, Vs=23 V, Rprimary=2400 ohm; equal turns leaves voltage/load unchanged; Vp=0 gives Vs=0; reject fractional/zero turns and R<=0. |
| sinusoidal_flux_density | Faraday derivative requires exact 2pi/sqrt2 sine factor, correctly used; RMS excitation -> peak T/Wb; no DC offset/core-loss certification. Positive f/A, integral turns, and input-only user field limit are appropriate. No additional finding. | B=24sqrt2/(40pi)=.270094894847... T and Phi=B*4e-4=.0001080379579... Wb; doubling f/N/A halves B; Phi independent of A for fixed V/f/N; V=0 both zero; guard and integer checks. |
| rectangular_toroid_inductance | Correct radial integration and inner-radius peak, not mean-radius approximation. Positive radii/height/mur and outer>inner, integer turns and field limit. MAG-R1 arithmetic conditioning defect. | L=2e-7*mur*N²*h*ln(ro/ri); nominal coefficient .00025*ln2=.00017328679514 H; Binner=.01 T; monotonic ri/ro and I=0; MAG-R1 thin-annulus regression. |
| gapped_core_inductance | Correct one-series-path finite core plus total-gap reluctance; core length excludes gaps and same rectangular area. Both 0.1 geometry screens are explicit approximation choices, not standards/error guarantees. Positive dimensions/gap, mur>=1, integer turns, user linear limit. No additional finding. | NI=Hc*lc+Hg*g gives B=mu0*NI/(lc/mur+g), L=mu0*N²*A/(lc/mur+g). Nominal Rc/Rg=.1, L=.002284794657... H, B=.228479465716 T; zero I gives B=0; test both geometry screens independently and exact boundaries; omitting Rc overstates nominal L by10%. |
| field_energy_density | Correct uniform linear region u=integral H dB=B²/(2mu), not total-device energy/nonlinear hysteresis. Magnitude B>=0 and B<=user limit; volume/mur>0. MAG-R3 underflow defect. | Nominal u=.04/(8pi*1e-7)=15915.494309... J/m³, W=u*5e-8=.000795774715... J; twice B ->4x density; twice volume ->2x energy; zero B both0; MAG-R3 range probe. |
| conductor_skin_depth | Correct amplitude 1/e good-conductor half-space depth, f in Hz and no accidental omega/RMS factor. Positive coefficients; sigma>=100omega epsilon. Threshold choice/caveats honest; no finite-wire/proximity/AC resistance inference. MAG-R2 reciprocal conditioning defect. | delta=1/[pi*sqrt(2.32e6)]=.00020898067849 m nominal; 4x f/sigma/mur ->half depth. Threshold sigma/(omega epsilon)=100 equality allowed, below rejected; MAG-R2 low-f regression. |
| coupled_coil_inductance | Correct reciprocal passive k magnitude[0,1], open-circuit self-inductances>0 and aiding/opposing dot convention. Separate roots and rationalized root difference are model-equivalent and improve stated small/large/near-perfect regimes. No additional finding. | M=.9sqrt(1e-4*4e-4)=.00018 H; series values .00086/.00014 H. At k0 M0 and both series L1+L2; equal L,k1 ->opposing0; at k1 unequal ->(sqrtL1-sqrtL2)^2; swap winding invariant; documented1e±200 probes and adjacent-self-inductance mismatch must use relative/scale tolerances. |
| transformer_va_budget | Correct simultaneous winding RMS VA allocation, not phasor mains VA/watts; headroom>=1 explicitly user-chosen. Voltage>0 and current>=0 permit disabled second winding/all-zero loads. Winding/rectifier/derating caveats honest. No additional finding. | 24*2=48 VA,12*.5=6 VA,sum54,headroom1.25->67.5; I2=0 ->headroom*V1I1; all currents0 ->all outputs0; headroom1 equals sum; reject negative current/headroom<1. |

Above identities and nominal substitutions are hand/algebraic review, not executed tests. Research nominal oracle values are unchanged except harmless last-bit decimal rounding. No source supports 1e-6 physical device accuracy or the screening thresholds as a compliance standard; docs correctly separate arithmetic tolerance from model accuracy and source-author access claims from implementation access.

## Primary sources actually inspected

- MIT Lecture11 https://web.mit.edu/6.014/www/handouts/lecture11_notes.pdf : Faraday/turns ratio, field energy, toroidal/gap derivations.
- MIT8.02 worked examples https://web.mit.edu/course/8/8.02-esg/Spring03/www/8.02ch32we.pdf : example2 radial toroid integration, examples1/3 field energy.
- MIT electromagnetic text https://www.mit.edu/course/6/6.013_book/www/chapter9/9.7.html : gap-small-relative-to-pole requirement, uniform/linear region assumptions, reciprocal two-winding linkage text. Equation images were not decoded; source prose plus independent Ampere/flux-continuity derivation provides support, not a claim of image inspection.
- MIT CBA https://fab.cba.mit.edu/classes/862.19/notes/circuits.pdf : §7.2.1 eq7.26-7.35 conduction/displacement, amplitude depth, half-space and copper example.
- Micrometals https://web.mit.edu/6.131/www/document/micrometals.pdf : printed page29 sine RMS form factor, SI/CGS conversion; text extraction is imperfect/two-column but factor and units visible. No loss fit imported or accuracy claim made.
- Hammond265 https://www.hammfg.com/electronics/transformers/power/265 : RMS VA table5V*30A=150VA,24V*10A=240VA, invariant series/parallel total.
- Hammond rectifier guide https://www.hammfg.com/electronics/transformers/rectifier : RMS voltage and topology-dependent DC/winding current conversion caveat.
- Berkeley https://www2.eecs.berkeley.edu/Pubs/TechRpts/2007/EECS-2007-3.pdf : §3.2.2 eq3.6-3.8 M reciprocity and k=M/sqrt(L1L2),0..1; returned archive redirect inspected.
- Stanford https://stanford.edu/~boyd/ee102/101exercises.pdf : static-circuit exercise10 reflected load problem. It states a problem, not a standalone solved proof; reflection independently derived using ideal lossless power and voltage ratio.

University theory is primary mathematical-model evidence, not empirical characterization of components; several MIT citations share the same classical theory and are not independent experiments.

## Untested scope and disposition

No Rust-process/public-engine execution, tests, builds, lint, formatting, Flutter/native UI, OS packaging or hardware measurements were performed. No tester branch results were assumed. Main owns verifying all complete examples, domains and proposed regressions after remediation. No security/model-topology/RMS-vs-peak error was found beyond the three numerical defects. This is not blanket certification of arbitrary f64 extremes or manufactured-component accuracy.

Persistence caveat: reviewer could not directly save files with mounted tools; parent must persist this report and append the bus entry. Findings were delivered immediately to Main and incremental findings stream.

BUS APPEND AFTER SAVE:
## ReviewMagnetics → Main / Dev-magnetics / Test-magnetics: independent review complete
Reviewed all8 models at7f12f45; decision findings; tested=false. Report coordination/reviews/magnetics.md. ThreeP2 numerical defects: MAG-R1 toroid ln ratio near-equal radii(:189), MAG-R2 skin-depth reciprocal overflow(:453), MAG-R3 field-energy premature B² underflow(:729). Full accepted counterexamples/independent expected values/corrections/source evidence and parent verification scenarios in report. No production/research/test/doc changes or checks executed; parent owns remediation/testing. Parent persisted report/bus on behalf of reviewer because direct filesystem transport was unavailable.

## Reviewer handoff addenda

FOURTH immediate P2 and report addendum MAG-R4: magnetics.coupled_coil_inductance actual expression backend/catalog/magnetics.json:532 (incremental finding selected adjacent added line540). Full accepted vector L1=1e-200 H,L2=1e200 H,k=1e-300. sqrtL1*sqrtL2≈1, so expected M=1e-300 H; left-associative k*sqrtL1 underflows1e-400 to0 first, M0. Swapped windings returns representable1e-300; numerical violation of reciprocity. All outputs mathematically representable (series aiding/opposing≈1e200). Correction coupling*(sqrt(L1)*sqrt(L2)); roots-first product geometric mean is finite for finite positive L1,L2. Verify this and swap at relative1e-6 on tiny M, not blanket absolute1e-6. Static IEEE/source evidence, not executed. Berkeley §3.2.2 eq3.7/3.8 inspected. Amend report coupled-coil row from 'No additional finding' to 'MAG-R4 multiplication-order underflow'; final summary/bus count FOUR P2, add this full finding section. Remaining user-requested all8 review complete. Awaiting your authorized persistence of complete report/bus payload; direct filesystem write tool unavailable.

Final magnetics QA handoff: all8 reviewed independently; FOUR actionable P2 findings immediately submitted; complete report payload + MAG-R4 addendum already delivered. Intended coordination/reviews/magnetics.md remains absent on direct read. Exact remaining blocker is unavailable filesystem-write/anchored-append tool (write COMMUNICATION.md rejected; xd inventory has no edit tool; bash expressly read-only). Please save report with MAG-R4 and append bus entry on my behalf. tested=false. No verification commands or modifications. I cannot truthfully claim saved report until you persist it; all reachable review completed.
