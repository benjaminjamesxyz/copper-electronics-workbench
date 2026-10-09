# V2OptoRun — Optical hardware lane source and native-pixel review

- Reviewer lane: V2OptoRun (independent reviewer; no code execution performed by this worker)
- Snapshot reviewed: production root `/home/simson/pi-projects/pcb-design-calculator` at commit `2cca03e854eb6842be56411a9153b6ca1b251a42` (`git rev-parse HEAD` at review start; working tree clean, 0 modified files). Lane files last touched by `81500de fix: close corroborated oscillator, passives and label findings`, an ancestor of the reviewed snapshot.
- Dispatch receipt: Main-supplied capture set `coordination/expansion/screenshots/` (100 PNGs, all mtime 2026-10-10 00:02, post-fix recapture after the oscillator/passives conditioning and label-collision fixes).

## Reviewed artifacts

- `backend/catalog/opto.json` (7 entries, lines 1–729) — full read.
- `frontend/lib/diagrams/opto.dart` (186 lines, all seven `optoDiagrams` entries) — full read.
- `docs/features/opto.md` — full read.
- `research/expansion/opto.md` and `research/expansion/opto.json` — full read (authority for scope, oracles, diagram specs).
- `coordination/expansion/BACKLOG.md` lane entries 008–014 (lines ~160–290) — read; all numerical anchors match catalog examples.
- `frontend/lib/diagrams/primitives.dart` and `frontend/lib/diagrams/painter.dart` — read for label anchoring (top-left), arrowhead (last polyline point), `Dot` (connection marker, r=3.2), `Part` (center anchor; block/circle centered rects).
- `backend/src/lib.rs:71-82`, `backend/src/numerics.rs:16-18` — confirmed `expm1`, `exprel`, `hypot` engine registrations and correct `exprel` definition (`1.0` at `x==0`, else `expm1(x)/x`).

## Images actually viewed (native Flutter canvas captures, image reads — not filenames)

1. `coordination/expansion/screenshots/opto.photodiode_quantum_conversion.png`
2. `coordination/expansion/screenshots/opto.lambertian_target_irradiance.png`
3. `coordination/expansion/screenshots/opto.layer_absorption_capture.png`
4. `coordination/expansion/screenshots/opto.gaussian_aperture_capture.png`
5. `coordination/expansion/screenshots/opto.step_index_mode_cutoff.png`
6. `coordination/expansion/screenshots/opto.fiber_gaussian_dispersion.png`
7. `coordination/expansion/screenshots/opto.sipm_finite_cell_response.png`

All seven required images exist and were opened and inspected individually.

## Independent numerical verification (no engine run; hand/binary64 re-derivation)

Every catalog example was re-derived independently from the stated physics and compared to the stored `expected` (tolerance 1e-6 relative). All match:

- **photodiode_quantum_conversion**: Eph=hc/λ=2.3369951e-19 J; rate 8.5580e13 1/s; R=ηqλ/hc=0.514178426 A/W; Iph=1.0283568520120244e-5 A. ✓
- **lambertian_target_irradiance**: I0=0.1/π; I(30°)=I0·(√3/2); E=I·(1/√2)/0.04=0.4873105007710476 W/m². ✓ (Cosine model, independent cosφ factor, inverse-square all correct.)
- **layer_absorption_capture**: Pd=0.002·(1−e^−0.2); Fa=e^−0.2·(1−e^−1)=0.51753654; Pa=0.0010350731; Pt=0.002·e^−1.2. Three powers sum to P0 in exact arithmetic; catalog values conserve to binary64 roundoff. ✓
- **gaussian_aperture_capture**: zR=0.0496302157 m; w=hypot(1e-4, 1.00743e-4)=1.4194918e-4 m; F=1−e^(−2(a/w)²)=0.89282563; Pcap=8.9282563e-3 W; Iaxis=2P/(πw²)=315947.06 W/m². ✓ `hypot` and `expm1` forms equal the Newport equations algebraically.
- **step_index_mode_cutoff**: NA=√(0.006·2.894)=0.1317725313; V=2.1900645503; λc=1.4114761e-6 m; margin=0.2149354497. ✓ Factored `sqrt(Δn)·sqrt(Σn)` is cancellation-safe.
- **fiber_gaussian_dispersion**: x=−0.4; b=hypot(0.6,0.4)=√0.52=0.7211102551; Tout=7.2111026e-12 s; FWHM=2√(ln2)·Tout=1.2007273e-11 s; Ppeak=Ep/(√π·Tout)=0.0078239018 W. ✓ Matches the chirped-Gaussian ratio √((1+Cx)²+x²) (UCSB ECE228A slide 6.3); energy conservation Ep=√π·Ppeak·Tout consistent.
- **sipm_finite_cell_response**: S=800; Nf=S·exprel(−S/Ncell)=1600(1−e^−0.5)=629.5509445; response=0.7869386806; Q=q·G·Nf=1.0086518e-10 C. ✓ `exprel` form is algebraically identical to Hamamatsu eq. 9 and cancellation-safe at S→0 (response→1, fired→S analytically).

Conditioning idioms required by research (`expm1` for small fractions, `exprel` for the SiPM zero limit, factored NA, `hypot` for Gaussian widths, factored `(β2/T0)·(L/T0)` and `(2/π)·(P/w)/w`) are all present in the shipped expressions. No fabricated finite fallbacks, no arbitrary clamps beyond the two research-defined screening envelopes (Gaussian 0.1 rad divergence, fiber 1% contrast), both inclusive `<=` as specified.

## Cross-artifact consistency

- `research/expansion/opto.json` vs `backend/catalog/opto.json`: programmatic field-by-field diff over all 7 IDs (title, category, level, description, formula, assumptions, inputs, outputs, constraints, references, example incl. expected values) — **zero differences**.
- `docs/features/opto.md` mirrors research definitions and oracles; no drift found.
- Backlog 008–014 numerical anchors match catalog expected values exactly.
- All diagram quantity bindings resolve to real catalog keys of their own ID: photodiode (optical_power, wavelength, quantum_efficiency, responsivity, photocurrent), lambertian (radiant_flux, emission_angle, incidence_angle, distance, target_irradiance), layer_absorption (incident_power, dead_thickness, active_thickness, dead_absorption, active_absorption, dead_absorbed_power, active_absorbed_power, transmitted_power), gaussian (wavelength, waist_radius, beam_radius, aperture_radius, axial_distance, beam_power, captured_power), step_index (core_index, cladding_index, core_radius, wavelength, normalized_frequency, cutoff_wavelength), fiber_dispersion (initial_width, initial_chirp, beta_two, fiber_length, output_width, output_peak_power), sipm (photon_count, detection_efficiency, cell_count, fired_cells, output_charge). Omitted outputs (active_capture_fraction, rayleigh_range, captured_fraction, axis_irradiance, numerical_aperture, single_mode_margin, normalized_dispersion, width_ratio, output_fwhm, seed_count, relative_response) are permitted legend omissions under the research convention.
- Units: W, m, rad, 1/s, A/W, A, J, W/sr, W/m², 1/m, s²/m, C, dimensionless `1` — all correct and consistent across catalog/docs.
- Distinct physical scope confirmed across the 7 IDs and against baseline: photon conversion, angular radiometry, layered attenuation, transverse diffraction capture, dielectric waveguide cutoff, quadratic spectral phase, discrete Geiger occupancy — no two items share a boundary-value problem; no overlap with the 128 baseline per research novelty analysis.

## Source verdict: PASS

No material source finding. Physics, reference directions, domains, constraints, units, conditioning and examples all reconcile with research and primary sources (Hamamatsu eq. 2-3 / MPPC eq. 1,3,9; Newport Lambert/Gaussian/fiber notes; UCSB ECE228A slides 6.2–6.4).

## Pixel verdict (per-ID)

All seven captures rendered sharply on the paper surface, 700×280 canvas fully inside frame, no clipped marks, no text/geometry overlap found at native scale:

1. **photodiode_quantum_conversion** — P | Depletion | N sections with internal dividers at the block boundaries; 'Depletion' fits its 80 px lane; input P/λ arrow enters from the left; green internal arrow runs right→left, i.e. N(cathode)→P(anode), matching the conventional photocurrent direction in a reverse-biased junction; Anode below-left, Cathode below-right, correctly placed under their regions; η/R/Iph legend right of the block; bottom note clear. Topology, polarity and labels correct.
2. **lambertian_target_irradiance** — emitter bar with source point Dot; dashed horizontal source normal; θ arc between normal and ray; ray to target Dot on the tilted target surface (the solid 45° line through the target point, perpendicular to the surface normal); dashed target normal points back toward the source side; φ arc sits between target normal and back-directed ray, matching the catalog definition "Target normal to source direction angle"; r dimension line parallel below the ray; E beside the target; explainer note present. Correct.
3. **layer_absorption_capture** — two adjacent slabs with divider; entrance ray stops at the entrance face; thickness dimension arrows with d/t above; αd inside the narrow layer, αa inside the wide layer; downward Pd/Pa arrows exit the slab bottoms to their labels; rightward Pt arrow; conservation note "P0 = Pd + Pa + Pt". Correct.
4. **gaussian_aperture_capture** — dashed axis; envelopes narrow to the waist and widen to the aperture; waist-radius arrow w0 to the envelope; green w(z) arrow at the aperture plane with dashed tie to the envelope endpoint; aperture drawn as two double-line opaque segments with a clear opening (no material across the opening); 'a' arrow spans axis→opening edge only; z dimension below; beam envelope terminates at the aperture plane (no post-aperture beam, matching the caption); Pcap label clear. Correct (two non-blocking observations below).
5. **step_index_mode_cutoff** — cladding block with core band; n2/n1 labels inside their own regions; λ arrow enters on the core axis; conceptual fundamental lobe above the section, labeled illustrative; cross-section with separate cladding and core circles, center Dot, radius arrow from center to core edge only (not the cladding radius); a label inside the cladding ring; static "V < 2.405: one spatial mode"; V and λc as bound output labels. Correct.
6. **fiber_gaussian_dispersion** — two axis panels with positive pulses; output pulse drawn narrower and taller than input, matching the documented compression example; T0 and Tout dashed to the 1/e level with one-sided half-width arrows (not FWHM spans); β2 and L inside the fiber block; two distinctly-marked wavelets; static "C·β2 < 0 can compress" unbound note; Ppeak label. Correct.
7. **sipm_finite_cell_response** — 3×2 microcell array; three optical arrows land on one cell whose center carries a fired Dot; a second fired cell (bottom right) contributes exactly one wire each; both wires join at a junction Dot from which the single green output arrow carries Q; Nf labels the collected contribution; "Multiple hits → one firing per recovered cell" and fired-subset notes present. Contributes-to-one-node semantics and one-charge-per-fired-cell both visible. Correct.

## Findings

**No material findings.** No severity ≥ P2 issue, no missing image, no misbound quantity, no topology/polarity error, no collision or clipping observed.

Non-blocking P3 observations (recorded for completeness; no correction requested, none blocks PASS):

- O1 / P3 / `frontend/lib/diagrams/opto.dart:87-91` (gaussian): drawn aperture half-opening (~55 px) is smaller than the drawn 1/e² envelope half-width (~65 px) at the aperture plane, while the worked example has aperture_radius > beam_radius (a/w≈1.057, F=0.893). The caption explicitly disclaims scale ("geometry is not to scale") and the research spec mandates conceptual envelopes; the clipped-tails picture remains qualitatively honest. No repair required.
- O2 / P3 / `frontend/lib/diagrams/opto.dart:96` (gaussian): `w(z)` label sits ~85 px left of its dimension arrow; binding stays unambiguous because the beam-radius arrow is the only green radius arrow at the aperture plane. No repair required.
- O3 / P3 / `frontend/lib/diagrams/opto.dart:44` (layer_absorption): 'Inactive' text begins 39 px left of its slab face; its `d` dimension and `αd` sit directly beneath inside the layer, so the association is unambiguous. No repair required.

## Gate: **PASS** (source PASS ∧ pixel PASS; all 7 required images present and viewed)

## Untested scope (honest)

- No engine execution, builds, tests, or process launches by this reviewer (contract). Numerical agreement rests on independent re-derivation plus Main's receipt that all 228 complete examples and the native journey pass at `2cca03e` (which also evidences engine `floor`/`pi` availability used by sipm/lambertian/gaussian constraints).
- Registry/panel dispatch wiring (title/caption/legend rendering around the canvas), narrow-window and large-text behavior, and accessible-legend acceptance are parent-owned and not verified here; captures inspected are the canvas region.
- Physical calibration of real devices (QE/PDE/index/fiber data at operating points) is explicitly a user obligation and outside static review.
- Tester-side behavioral coverage for the 7 IDs belongs to T2OptoRun and was not inspected here.
