# V2CablesRun — source and native-pixel review: Cables and connectors

- Snapshot: **2cca03e** (production root `/home/simson/pi-projects/pcb-design-calculator`, read-only).
- Reviewer: V2CablesRun. Scope: the six assigned IDs only; closed oscillator/passives lanes not duplicated.
- Reviewed files: `backend/catalog/cables.json`, `frontend/lib/diagrams/cables.dart`, `docs/features/cables.md`, against `research/expansion/cables.md`, `research/expansion/cables.json`, and the backlog lane (BACKLOG.md items 059–064).
- Registry: `frontend/lib/diagrams/catalog.dart:61` spreads `cablesDiagrams`; all six keys present exactly once (`cables.dart:3,34,72,109,144,174`).
- Byte-level identity: a structural comparison of `backend/catalog/cables.json` against `research/expansion/cables.json` shows **zero differences** in id, title, category, level, description, formula, assumptions, references, inputs, outputs, constraints, examples and tolerances. `docs/features/cables.md` sections 1–6 carry the research model/domain/example text verbatim. No silent normalization was needed or found.

## Independent verification performed (reviewer arithmetic; no engine, build, test or process executed)

Every catalog example and the research's independent cases were recomputed outside the implementation in IEEE-754 double arithmetic, reproducing all **41 expected outputs** to ≤ 7.1e-16 relative (tolerance 1e-6), plus these conditioning probes:

1. `cables.twin_lead_geometry` — `log1p` geometry form equals `acosh(D/d)` exactly at the example (2.477888730288475) and to 5.5e-14 at near-touch `D=1.001 d` (no `x²−1` cancellation); er=1 case (C′=1.1225787117147267e-11 F/m, Zdiff=297.140941322357 Ω) and equal-scaling invariance reproduce. L′ is loop inductance, C′ wire-to-wire, Zdiff=sqrt(L′/C′) consistent TEM pair (matches Ellingson S1 form η0·q/(π√er)).
2. `cables.lossy_rlgc_line` — all 11 nominal outputs match Python `cmath.sqrt(z*y)`, `cmath.sqrt(z/y)` to ≤ 1.9e-16, so the real-expression decomposition is exact, not approximate. Lossless R′=G′=0 gives **exactly** alpha=0, beta=0.03141592653589793, Z0=50+j0 (denominator uses c=1, no 0/0). Heaviside G′=8e-4 gives alpha=0.04, beta=ω√(L′C′), Z0=50+j0 **exactly** (cmath leaves −1.7e-15 residue; the catalog form is cleaner). f=1e-12 Hz preserves beta=1.0182952972441003e-19 rad/m and Zimag=−4.843111779575599e-15 — the normalized half-angle forms do not round the phase to π/2. length=0 returns ratio=1, 0 dB. All input domains (f, L′, C′ > 0; R′, G′, length ≥ 0) are enforced by catalog bounds; no constraints needed.
3. `cables.radial_insulation_stress` — example exact; Vpk=12000 returns E(a)=8656170.24533378 V/m and margin −656170.24533378 V/m (negative margin admitted, no clamp); fixed b/a scaling (×2 radii halves fields, doubles Vlimit) and thin-gap limit E(a)→Vpk/(b−a) reproduce; `log1p((b−a)/a)` exact for ln(b/a).
4. `cables.hertz_holm_contact` — example exact (E*=62189054726.368164 Pa, a=6.22480523898329e-6 m, p0=123222453.18016359 Pa, Rc=0.0013655045698085684 Ω, approach=1.9374100131636908e-8 m); F×8 scales a×2, p0×2, Rc×½, approach×4 as required; the constraint-left `(3FR(1−ν)(1+ν)/(2E))^(1/3)` equals the output `(3FR/(4E*))^(1/3)` to 1.4e-16 (identical algebra, input-only, evaluated before outputs); screens pass at a/R≈0.0031, a/λ≈155.6.
5. `cables.contact_hotspot_temperature` — example exact; Vc=0 valid (Tm=300 K, margin=300 K); Vc=1e-6 V rise 1.7006802720606376e-8 K stays positive via the `u·(u/(Tm+Tbulk))` form (no near-equal subtraction); Vc=0.2 V gives Tm=705.8 K > 600 K so the constraint must reject (no clamping in catalog); Vlimit form equals 2√L·√(Tlim²−Tbulk²) exactly.
6. `cables.cantilever_contact_spring` — example exact (k=520 N/m, F=0.26 N, σ=195000000 Pa, margin=405000000 Pa); δ=L/10 boundary accepted (F=0.52 N, σ=390 MPa); δ=0 leaves the slenderness screen independently testable (stress constraint is a separate entry); b×2 doubles k and F, leaves σ unchanged; E·b/4·(t/L)³ form finite through E=1e308.

Extreme-input probe (admitted-bound corners, reviewer arithmetic only): RLGC stays finite and correct through R′=1e308 or R′=G′=1e150 (hypot-everywhere, `sqrt(s)·sqrt(t)` product form, normalized x/y); `hypot` keeps hotspot finite at Tbulk=1e308; cantilever finite through E=1e308. Overflow appears only at physically impossible corners (R′=G′=1e308 simultaneously → attenuation_db=inf; twin-lead D/d ratio 1e600 → geometry_factor=inf; Hertz F·R ≥ ~1e154·1e154 → constraint inf). The research explicitly defers exactly this class: "Binary64 extremes still require parent numerical review." No wrong finite value was found anywhere; no finding raised.

## Physics, units, domains, distinctness

- Equations match primary sources S1–S9 as claimed: two-wire acosh geometry (S1), RLGC telegrapher cell with exp(+jωt)/exp(−γz) (S2), coaxial 1/r field and inner-surface peak (S3), Hertz sphere-on-flat with E*=E/(2(1−ν²)) for identical bodies (S4), Holm single-spot ρ/2a (S5), Kohlrausch/Wiedemann–Franz hotspot (S6), cantilever k=Ebt³/4L³ and σ=6FL/bt² (S7/S8), Materion context (S9). Sign/reference conventions consistent: alpha ≥ 0, beta > 0, Im Z0 < 0 for capacitive-dominant, E-field + → −, +V/2/−V/2 pair, current left→right with + tap upstream in the hotspot figure, F downward into the sphere, bending/deflection downward.
- Distinct physical scope confirmed against the backlog novelty table: no ID is a rename/inverse of a baseline or sibling (geometry vs loss vs field stress vs contact mechanics vs electrothermal vs spring rate). Screens (a/R ≤ 0.1, a ≥ 10 λ, p0 limit, t ≤ L/10, δ ≤ L/10, Tlimit validity) are stated as engineering screens, not accuracy claims, and are enforced as catalog constraints.
- All units and quantity keys consistent between research, catalog, docs and diagram bindings.

## Native pixel review — images actually opened and inspected

All six at `/home/simson/pi-projects/pcb-design-calculator/coordination/expansion/screenshots/`:

1. `cables.twin_lead_geometry.png` — PASS. Two conductor circles, +V/2 / −V/2, filled dot (out-of-page) and cross (into-page) current symbols; D dimension between centers with dashed extension lines; d dimension across the left circle; two curved E-field lines with arrowheads landing on the negative conductor; legend binds εr, L′, C′, Zdiff; caption line. No clipping, no overlap, no ground/shield, field direction correct.
2. `cables.lossy_rlgc_line.png` — PASS. Series R′ then L′ on the upper conductor; two shunt branches G′ (resistor) and C′ (capacitor) at distinct nodes with **all four junction dots present**; continuation dashes both ends; Δz bracket distinct from `length`; forward +z arrow with `length` extent; bottom legend binds alpha, beta, ratio. Labels clear of wires; both branches tie upper node to return.
3. `cables.radial_insulation_stress.png` — PASS. Concentric section with shield annulus clearly double-lined (b is the shield **inner** surface, arrow tip lands on the inner shield line, length ≈ r90); a-arrow tip on conductor edge; three outward E arrows inside the dielectric; +Vpk inside conductor, shield 0 V; qualitative 1/r curve with endpoint dots, E(a):maximum and E(b) labels clear of the curve; a/b ticks explanatory; Eallow/Vlimit legend. No collision or clipping.
4. `cables.hertz_holm_contact.png` — PASS. Dome-on-flat with flat contact patch; F arrow downward; dashed R curvature arrow anchored on the load axis pointing to the tip surface; current streamlines converge to the patch and spread below without crossing the body outlines; 2a dimension aligned with patch ends, labeled "a; spot width 2a" (never diameter-as-a); constriction-only Rc inset; same-metal E/ν/ρ row; E*, approach, p0 legend. No collision or clipping.
5. `cables.contact_hotspot_temperature.png` — PASS. Two blocks with narrow neck; hotspot circle at neck center with leader to Tm label; Kelvin taps bracket **only** the neck with dots at the neck mouths, + on the left (upstream) and − on the right, consistent with the left→right current arrow; dashed voltmeter link; heat arrows point away from the neck into **both** blocks; Tbulk at both remote ends; Lorenz/ΔT/Tlimit legend. No ambient resistor drawn. No collision or clipping.
6. `cables.cantilever_contact_spring.png` — PASS. Hatched clamped wall; dashed undeformed line and distinct smooth deflected curve; F arrow at the tip pointing down to the displaced tip; δ dimension between the two tip positions; free-length dimension; root-stress leader at the wall; cross-section with horizontal b and vertical t plus "Bending direction" arrow consistent with vertical bending; E/k/σallow legend. No collision or clipping.

Quantity-label bindings were checked in source against the research mandatory-diagram lists: every required binding is present and points at the correct quantity; unbound symbols (E(r), Δz, a/b ticks, I, Heat, undeformed/deflected) are explanatory text exactly as the research allows. Auxiliary outputs without a diagram binding (geometry_factor, impedance_real/imaginary, attenuation_db, field_margin, contact_voltage_limit, temperature_margin, stress_margin) match the research's explicit legend lists — intentional, not omissions.

## Findings

**None material.** No numerical, physical, units, domain, binding, topology, polarity or pixel defect meets the reporting bar at 2cca03e. Two P3 informational observations, recorded for completeness; neither requires change (both are disambiguated by their captions and carry no electrical-semantic risk):

- O1 / P3 / `frontend/lib/diagrams/cables.dart:120` — `Dot(180, 60)` sits on the Hertz force arrow as the curvature-arrow anchor. Elsewhere in this repository a Dot marks an electrical junction; here the context is mechanical and the caption states arrows are illustrative. No change requested.
- O2 / P3 / `frontend/lib/diagrams/cables.dart:154-155` — the hotspot "Current I" arrow floats above the left block without touching it. Purely directional illustration; the caption declares geometry/arrows static. No change requested.

## Verdict

- **Source verdict: PASS** — catalog identical to research spec; equations, units, domains, constraints, conditioning forms and all 41 example outputs independently reproduce (≤ 7.1e-16 relative); six distinct physical scopes confirmed; docs faithful.
- **Pixel verdict: PASS** — all six native PNGs opened and inspected; correct topology, pins, polarity and reference directions; no missing element, collision, clipping or misleading curve found.
- **Overall: PASS** for the cables lane at snapshot 2cca03e.

## Untested scope (honest)

- No build, test, lint, formatter, process, engine call or native launch was run. Example/limiting-case reproduction is reviewer double arithmetic, not executed Rust output; Main's 228-example run at 2cca03e is the executed evidence.
- The malformed/nonfinite input protocol (NaN/inf/missing rejection through the real engine) is T2CablesRun's and Main's to execute; I verified only that catalog bounds/constraints encode the stated domains.
- Binary64 corner overflow (simultaneous ~1e308 RLGC pairs, D/d ratios ~1e600, F·R products ~1e308) is deferred by the research to parent numerical review; it was probed analytically only.
- Pixel review covers the six recaptured composites as supplied; I did not re-render geometry or verify the Flutter runtime beyond the delivered images.

---

## Addendum — corroboration of T2CablesRunR2 deferred-corner findings (commit 80ebd78)

T2CablesRunR2 reported three deferred-corner conditioning regressions; all three were independently corroborated by reviewer arithmetic at 2cca03e (catalog expressions evaluated as written):

- **C1 / P3 / `cables.twin_lead_geometry` / `backend/catalog/cables.json:15` (geometry_factor expression).** Inputs `{wire_diameter:1e308, center_spacing:1.5e308, relative_permittivity:2.25}` pass the `D>d` constraint, but the literal `(center_spacing+wire_diameter)/wire_diameter` sums to 2.5e308 → inf, so every output is inf although all depend only on D/d=1.5. Since `(D+d)/d = D/d+1` exactly, the minimal repair is to evaluate the ratio first: `sqrt(center_spacing/wire_diameter+1)`. Corroborated: ratio-first yields 0.9624236501192069 = `acosh(1.5)` exactly; as-written yields inf.
- **C2 / P3 / `cables.hertz_holm_contact` / `backend/catalog/cables.json` small-contact screen (both screens share the `(… )^(1/3)` left side).** `{normal_force:10000, sphere_radius:0.01, young_modulus:150e9, poisson_ratio:0, elastic_pressure_limit:1e10, …}` gives exact a=0.001=0.1·R, which the research explicitly permits ("Equality on the defined screening boundaries is permitted"), but `pow(1e-9,1/3)` rounds to 0.0010000000000000005 > 0.001 → spurious rejection. Corroborated. Minimal repair: compare cubes — drop `^(1/3)` from the left side and change the right to `(0.1*sphere_radius)^3` (corroborated: 1e-9 ≤ 1e-9 accepts); the diffusive screen and its 10·mean_free_path right side can stay in radius form or be cubed consistently.
- **C3 / P3 / `cables.contact_hotspot_temperature` / `backend/catalog/cables.json` hotspot-vs-limit constraint.** `{contact_voltage:1e-10, bulk_temperature:300, lorenz_number:2.45e-8, temperature_limit:300}`: true Tm = 300+1.7e-16 K strictly exceeds the 300 K limit for any Vc>0, but `hypot` rounds to exactly 300.0, so the constraint admits while `temperature_rise`=1.7006802721088436e-16 K > 0 and `temperature_margin`=0.0 — an internally inconsistent admitted result. Corroborated. Minimal repair: rewrite the constraint as `contact_voltage <= 2*sqrt(lorenz_number)*sqrt(temperature_limit-bulk_temperature)*sqrt(temperature_limit+bulk_temperature)`; at Tlimit=Tbulk the right side is exact 0.0, so any positive Vc is rejected consistently, and decisions match the Tm-form everywhere else (corroborated both corners).

These are open regressions for Main to execute and repair before lane closure; they do not overturn the source or pixel dispositions above (all ordinary-magnitude examples and limits remain exact), and they stay inside the binary64-corner scope both lanes deferred. No production edits or commits made from this lane.

