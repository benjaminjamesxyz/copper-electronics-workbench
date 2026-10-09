# Optical hardware calculators

Rust evaluates the catalog; Flutter renders metadata and static explanatory diagrams. All defaults equal the complete independent examples below (relative tolerance 1e-6). Numerical tolerance is not physical accuracy. Registry integration, independent tests and native-image acceptance belong to Main. No checks ran during lane implementation.

Source-access statements below are inherited research evidence, not new source retrievals.



## Sources and conventions

Research date: 2026-10-09. English-language manufacturer technical notes and university teaching material were searched without a publication-date exclusion. Search terms covered photodiode quantum efficiency/responsivity, Lambertian irradiance, Gaussian aperture power, step-index NA/V-number, Gaussian fiber dispersion and SiPM finite-cell saturation. LED ballast, generic shot noise, transimpedance stages, RF links, generic dB conversions and electrical propagation delay were excluded. The actual 128-ID baseline formulas, expansion allocation and current bus claims were read before selecting models.

The following sources were actually retrieved and read, not merely cited from search summaries:

- **H1 — manufacturer technical note:** [Hamamatsu, Si photodiodes](https://www.hamamatsu.com/content/dam/hamamatsu-photonics/sites/documents/99_SALES_LIBRARY/ssd/si_pd_kspd9001e.pdf), sections 1-1, 2-1, 2-2 and 2-3; especially equation 2-3. Strong source for the manufacturer's terminology and device limitations; not independent experimental validation of arbitrary entered QE.
- **H2 — manufacturer-authored technical review:** [Hamamatsu, Physics and operation of the MPPC silicon photomultiplier](https://hub.hamamatsu.com/us/en/technical-notes/mppc-sipms/physics-and-operation-of-the-MPPC-silicon-photomultiplier.html), Slawomir Piatek, 2014. Read section I equations 1–3, section II equations 5–7, and section V PDE and Dynamic range, equation 9 and its contrary measured over-saturation evidence. Manufacturer evidence for the adopted engineering model; the underlying saturation study was not independently retrieved. The occupancy law is therefore explicitly an idealized screening model, not a validated device guarantee.
- **N1 — manufacturer optical theory note:** [Newport, Laws of Radiation](https://www.newport.com/t/laws-of-radiation/), Lambert's Law section.
- **N2 — manufacturer training handbook:** [Newport/Oriel, Spectral Irradiance](https://www.newport.com/medias/sys_master/images/images/hfb/hdf/8797196451870/Light-Sources.pdf), pages 2–5 for units, photon energy and inverse-square scaling; page 7 for Lambert's law. N1 and N2 are not independent corroborating sources.
- **N3 — manufacturer optical theory note:** [Newport, Gaussian Beam Optics](https://www.newport.com/n/gaussian-beam-optics/), Gaussian radius convention, enclosed power and propagation. The actual equation images were also read: [enclosed power](https://www.newport.com/medias/sys_master/images/images/h97/h25/8798207508510/GAUS-BM-No-4-S-400w.gif) and [propagation](https://www.newport.com/medias/sys_master/images/images/hb8/h63/8798208589854/GAUS-BM-No-6-S-400w.gif).
- **N4 — manufacturer technical note:** [Newport, Photonics Technical Note 21: Fiber Basics](https://www.newport.com/medias/sys_master/images/images/hd5/ha2/8797095919646/Fiber-Basics.pdf), pages 1–4 for weak guidance, numerical aperture, V-number and LP11 cutoff. This is a step-index screening approximation, not cable certification.
- **U1 — university lecture:** [UCSB, ECE228A Winter 2011, Lecture 6: Propagation in Optical Fibers and Dispersion](https://courses.ece.ucsb.edu/ECE228/228A_W11Blumenthal/Lecture6-228a.pdf), slides 6.2–6.4 and 6.7–6.13. Strong pedagogical support for the closed-form linear-dispersion model; not a device-specific fiber specification.

Evidence gaps and retrieval failures: the UCSB 2009 Lecture14 PDF fetch aborted, and the older Rochester `opt428b.pdf` URL returned HTTP 404. Neither is cited as read evidence. U1 was successfully read instead. Generic chemistry Beer–Lambert search results were not used: H2 directly supplies semiconductor absorption depth. Actual material coefficients, QE, fiber indices/dispersion, source radiation patterns and SiPM gain/PDE must come from the chosen hardware and operating conditions. No standards-compliance or experimental-accuracy claim is made.

## Numerical and diagram conventions

All catalog units are SI, including wavelength in metres, angle in radians and fiber dispersion in s²/m. Dimensionless quantities use unit `1`. Exact SI constants are `h=6.62607015e-34 J s`, `q=1.602176634e-19 C`, `c=299792458 m/s`. Nominal numerical oracles were derived outside Rust using Python standard-library Decimal at 60 decimal digits; trigonometric nominal values use exact radicals for 30° and 45°. Catalog expectations round those independent results to binary64. The unavailable `mpmath` import was replaced with Decimal without installing dependencies.

Only established engine functions are required: `exp`, `expm1`, `exprel`, `sqrt`, `hypot`, `ln`, `cos` and `floor`. Constraints reference inputs only. Use `expm1` for small absorbed/captured fractions, `exprel` for the analytic zero-exposure SiPM response, factored index differences for NA and `hypot` for Gaussian spatial/temporal widths. Unrepresentable floating-point intermediates remain arithmetic errors, not fabricated finite outputs. The proposed model envelopes are not arbitrary test bounds: the Gaussian 0.1-rad guard limits the paraxial approximation, and the fiber 1% contrast guard explicitly defines the weak-guidance screening scope.

Every diagram uses the existing `primitives.dart` and 700×280 logical canvas, 16-pixel margins and labels at least 14 pixels. Export one entry per exact ID through `optoDiagrams`. The specifications below use geometry, not new rendering packages. All labels named as quantities must bind the exact keys shown; all other explanatory text is unbound. Avoid placing every supporting numeric output on the canvas: the accessible legend can show quantities omitted from geometry. No static sketch is a live plot or a numerical result. Parent performs native pixel, narrow-window and large-text acceptance.


## 1. `opto.photodiode_quantum_conversion`

### Purpose, novelty and source

Use this basic model to budget the generated reverse photocurrent from monochromatic light. Closest baseline: `diodes.shockley_forward_current`, which predicts thermally activated junction conduction from voltage, and `noise.junction_shot`, which predicts a noise spectral amplitude from current. Neither models photon-to-carrier conversion. The neighboring precision lane owns conversion of that current into voltage; this item has no amplifier, feedback resistor or noise budget. Compared with this lane's absorption item, QE is a supplied external photon-to-collected-carrier ratio, not inferred from slab geometry.

H1 equation 2-3 states QE in terms of spectral responsivity and wavelength, using the rounded 1240 nm conversion. H2 section I equation 3 gives the underlying exact expression `Iph=q*eta*lambda*P/(h*c)`. N2 supplies `Eph=h*c/lambda`. Use exact SI constants rather than the rounded 1240 factor.

### Model and physical domain

`Eph=h*c/lambda`; `photon_rate=P/Eph`; `responsivity=eta*q*lambda/(h*c)`; `photocurrent=responsivity*P`.

Require positive wavelength, `0<=quantum_efficiency<=1`, and nonnegative optical power. Gain is exactly one. External QE includes window/reflection and collection losses. Choose QE from the correct wavelength, temperature and bias; the calculator does not extrapolate silicon response into arbitrary spectral bands. H1 explains that saturation depends on device area, load, reverse bias and spot size. The model excludes those limits, dark current and circuit loading. The output is a positive magnitude; internal generated conventional current flows from cathode to anode.

### Complete independent example

Inputs: `wavelength=8.5e-7`, `quantum_efficiency=0.75`, `optical_power=2e-5`.

- `photon_energy=(6.62607015e-34*299792458)/(8.5e-7)=2.33699512605756317647e-19 J`.
- `photon_rate=2e-5/photon_energy=8.55799816482260581377e13 1/s`.
- `responsivity=0.75*1.602176634e-19/photon_energy=0.514178426006012242118 A/W`.
- `photocurrent=0.514178426006012242118*2e-5=1.02835685201202448424e-5 A`.

### Tester cases

Nominal: assert all four outputs. With all other inputs unchanged, double power: photon rate and current double while photon energy and responsivity do not change. Zero power gives exact zero rate/current; QE zero gives exact zero responsivity/current but preserves positive incident photon rate. QE one is valid. Reject zero/negative wavelength, negative power and QE outside [0,1]. Apply shared nonfinite-input/error-recovery coverage. The wavelength/QE pair's physical calibration remains a user obligation.

### Mandatory diagram

Draw an optical power arrow from left to a rectangular P/depletion/N semiconductor section centered around x=330, y=130. Mark P/anode below-left and N/cathode below-right. A short arrow inside the section points from cathode/N to anode/P and is explicitly labeled generated conventional current, not electron flow. No amplifier or external bias circuit is implied.

Quantity labels: incoming `P` → `optical_power`; incoming `lambda` → `wavelength`; conversion `eta` → `quantum_efficiency`; outgoing magnitude `Iph` → `photocurrent`; optional unconnected conversion legend `R` → `responsivity`. Caption: “Monochromatic light generates reverse photocurrent; external quantum efficiency includes optical and collection losses. Unity gain; dark current and readout circuitry omitted.”

## 2. `opto.lambertian_target_irradiance`

### Purpose, novelty and source

Use this basic model to position an illumination source and orient a target surface. Closest baseline: `diodes.led_series_resistor` sets electrical drive only. This model instead integrates an angular radiometric distribution and applies a receiving-plane projection. It is not an RF antenna/link budget: no wavelength-dependent antenna aperture or received-radio-power equation is used.

N1 and N2 give `I(theta)=I0*cos(theta)` and inverse-square irradiance. Integrate over the forward hemisphere: `Phi=2*pi*I0*integral[0..pi/2](cos(theta)*sin(theta))dtheta=pi*I0`. The target projected differential area gives the independent `cos(phi)` factor. Therefore `E=Phi*cos(theta)*cos(phi)/(pi*r²)`.

### Model and physical domain

Outputs are `axial_intensity=Phi/pi`, `directional_intensity=I0*cos(theta)` and `target_irradiance=I(theta)*cos(phi)/r²`. Flux is total optical W, not electrical input W or lumen. `distance>0`, both angles lie in `[0,pi/2]`, and flux is nonnegative. The source must be small relative to distance and genuinely Lambertian; packaged LEDs with secondary optics often are not. This is irradiance at a point, not integrated power over a large target. Ignore atmospheric attenuation, occlusion and reflected light. The incidence normal points from the target toward the source; its angle is not measured from the forward-going propagation vector.

### Complete independent example

Inputs: `radiant_flux=0.1`, `distance=0.2`, `emission_angle=0.5235987755982989`, `incidence_angle=0.7853981633974483`. The intended angles are pi/6 and pi/4; the stored floating-point values differ only at rounding precision.

- `axial_intensity=0.1/pi=0.0318309886183790671538 W/sr`.
- `directional_intensity=(0.1/pi)*(sqrt(3)/2)=0.0275664447710896024756 W/sr`.
- `target_irradiance=(0.1/pi)*(sqrt(3)/2)*(1/sqrt(2))/0.2²=0.487310500771047567441 W/m²`.

### Tester cases

Nominal: all three outputs. Both angles zero at the same distance gives `target_irradiance=0.795774715459476678844 W/m²`. Doubling distance quarters only irradiance. Zero flux makes all outputs zero. Grazing source or receiver orientation tends to zero; allow the machine cosine residual at represented pi/2, scaled to flux/distance², rather than asserting mathematically exact floating-point zero. Reject negative angles, angles exceeding pi/2, nonpositive distance and negative flux. Close-but-positive distances remain mathematical inputs, but a real extended source at that distance violates the stated point-source assumption.

### Mandatory diagram

Place a small flat emitter near (95,95) with its normal pointing right; draw a ray down-right to a tilted target centered around (525,180). Add a source-normal dotted reference and a source angular arc `theta`. Draw the target normal pointing back toward the source, with angle `phi` between that normal and the back-directed source line. Draw a separate dimension line parallel to the connecting ray for `r`, avoiding the two angle labels.

Quantity labels: `Phi` → `radiant_flux`; `theta` → `emission_angle`; `phi` → `incidence_angle`; `r` → `distance`; `E` next to the target → `target_irradiance`. Caption: “Ideal Lambertian source in the far field. Source emission and receiver projected area contribute separate cosine factors; E is local irradiance, not total collected power.”


## 3. `opto.layer_absorption_capture`

### Purpose, novelty and source

Use this intermediate model to choose absorbing-layer thickness when a noncollecting entrance layer wastes incoming light. H2 section I equation 1 states `Phi(z)=Phi(0)*exp(-alpha*z)` and discusses penetration depth and carrier recombination in inactive material. Apply that spatial absorption law successively to two homogeneous layers. This is a layer-power partition derived from the source equation, not a manufacturer-specific device fit.

Closest baseline: `noise.decibel_to_linear` converts an already known logarithmic ratio; it does not derive absorption from material coefficients and geometry. `transients.rc_step` is temporal energy storage, not photon attenuation. Closest neighboring item: `opto.photodiode_quantum_conversion` uses measured external QE to determine current. Here the outputs are absorbed optical powers; active absorption does not imply unit collection or predict photocurrent. No extra QE inverse is counted.

### Model and physical domain

Let `ud=dead_absorption*dead_thickness` and `ua=active_absorption*active_thickness`. The inactive loss is `P0*(-expm1(-ud))`; active capture fraction is `exp(-ud)*(-expm1(-ua))`; active absorbed power is `P0` times that fraction; transmitted power is `P0*exp(-ud-ua)`. Their three powers sum to input power in exact arithmetic.

All five inputs are nonnegative; zero thickness and zero absorption are meaningful. Define incident power immediately inside the entrance surface so entrance reflection is not silently included. Normal incidence, no scattering/gain, no coherent interference and no interface/back reflections are required. Absorption coefficients are intensity coefficients in 1/m, not field-amplitude attenuation coefficients. They depend on wavelength and temperature; no wavelength interpolation is performed. Do not reinterpret captured fraction as measured external QE.

### Complete independent example

Inputs: `incident_power=0.002`, `dead_absorption=200000`, `dead_thickness=1e-6`, `active_absorption=100000`, `active_thickness=1e-5`.

Optical depths are `ud=0.2`, `ua=1`. Thus:

- `dead_absorbed_power=0.002*(1-exp(-0.2))=0.000362538493844036282660 W`.
- `active_capture_fraction=exp(-0.2)*(1-exp(-1))=0.517536541165779762025`.
- `active_absorbed_power=0.002*active_capture_fraction=0.00103507308233155952405 W`.
- `transmitted_power=0.002*exp(-1.2)=0.000602388423824404193290 W`.

### Tester cases

Assert all four outputs and conservation of the three powers. Set both coefficients to zero: inactive and active absorption are zero, active fraction zero, transmission `0.002 W`. Set only inactive thickness to zero: active fraction `0.632120558828557678404`, active power `0.00126424111765711535681 W`, transmission `0.000735758882342884643192 W`, inactive power zero. Active thickness zero eliminates useful capture but preserves entrance loss. Tiny positive active optical depth `1e-16` with no inactive loss must retain a positive capture fraction approximately `1e-16`; this requires `expm1`. A very thick active slab tends to capture all surviving entrance power. Reject any negative power, coefficient or thickness. No invented upper thickness or absorption bound is required.

### Mandatory diagram

Draw two adjacent vertical slabs from y=60 to 220: narrow inactive layer x=235..290 and wider active layer x=290..440. Incident ray travels left-to-right. Draw one downward absorbed-power arrow from each slab and a rightward transmitted arrow. Distinct thickness dimension lines above the slabs explain that absorption is distributed through thickness rather than occurring only at the boundaries.

Quantity labels: `P0` → `incident_power`; `d` → `dead_thickness`; `t` → `active_thickness`; `alpha_d` → `dead_absorption`; `alpha_a` → `active_absorption`; `Pd` → `dead_absorbed_power`; `Pa` → `active_absorbed_power`; `Pt` → `transmitted_power`. Use compact labels with clear spacing and move coefficients to the lower legend if needed. Caption: “Single-pass absorption after the entrance surface. Inactive absorption is lost; active absorption is optical capture only. Reflections, scattering and carrier-collection losses are not modeled.”


## 4. `opto.gaussian_aperture_capture`

### Purpose, novelty and source

Use this expert model to select an aperture or active detector radius in a propagating laser beam. N3 defines w as the 1/e² intensity radius and gives `w(z)²=w0²[1+(lambda*z/(pi*w0²))²]` and `P(a)=Ptotal[1-exp(-2*a²/w²)]`. Its warning about truncation changing subsequent beam propagation is retained.

Closest baseline: `signals.line_propagation` computes delay/wavelength, with no transverse diffraction or aperture integral. Closest neighboring item: fiber temporal dispersion changes pulse width through spectral phase; this model is free-space transverse spreading and spatial interception. Sharing Gaussian algebra does not make these the same physical problem. The entire propagation-plus-capture chain counts as one calculator, not separate radius and power items.

### Model and physical domain

`zR=pi*w0²/lambda`; `w=hypot(w0,lambda*z/(pi*w0))`; `F=-expm1(-2*(a/w)²)`; `Pcap=P*F`; `Iaxis=2*P/(pi*w²)`. `Iaxis` is the incident on-axis irradiance at the aperture plane, not transmitted beam-average irradiance.

Require positive wavelength and waist radius, nonnegative aperture radius/power and any signed axial distance. The divergence `lambda/(pi*w0)` must not exceed 0.1 rad. This is an explicitly conservative paraxial screening window: `tan(0.1)/0.1-1` is about 0.00335, not a promise that all physical errors are below that amount. Only circular TEM00, M²=1 propagation in index-one space is represented. A centered, normal circular aperture is required. Values at negative z are valid and symmetric in radius/power. Zero aperture passes zero total power; on-axis incident irradiance remains defined. The output makes no prediction after clipping or about optical damage.

### Complete independent example

Inputs: `wavelength=6.33e-7`, `waist_radius=0.0001`, `axial_distance=0.05`, `aperture_radius=0.00015`, `beam_power=0.01`.

- `rayleigh_range=pi*(0.0001)²/(6.33e-7)=0.0496302156965212201969 m`.
- `beam_radius=sqrt((0.0001)²+[(6.33e-7*0.05)/(pi*0.0001)]²)=0.000141949184351711474797 m`.
- `captured_fraction=1-exp(-2*(0.00015/beam_radius)²)=0.892825633225183769229`.
- `captured_power=0.01*captured_fraction=0.00892825633225183769229 W`.
- `axis_irradiance=2*0.01/(pi*beam_radius²)=315947.061266357865102 W/m²`.

### Tester cases

All five nominal outputs. At `axial_distance=0` and `aperture_radius=0.0001`, radius equals `0.0001 m`, capture fraction equals `1-exp(-2)=0.864664716763387308106`, and power equals `0.00864664716763387308106 W`; axis irradiance is `636619.772367581343076 W/m²`. The Rayleigh range remains unchanged. Swap z to -z: all outputs unchanged. Zero power only zeros the power/irradiance outputs; zero aperture only zeros capture fraction/power. At `abs(z)=zR`, radius is `sqrt(2)*w0`. Tiny a/w must retain positive fraction through `expm1`. Reject nonpositive waist/wavelength, negative aperture/power and divergence above 0.1 rad. Include the admitted 0.1-rad endpoint.

### Mandatory diagram

Draw horizontal optical axis from x=60 to 650 at y=145, waist at x=210 and aperture plane at x=505. Two smooth conceptual envelope curves narrow to the waist and widen toward the aperture. They represent the 1/e² contour, not a hard beam edge. Draw the aperture as two opaque vertical segments separated by an opening centered on the axis; do not draw opaque material across the opening. Show radial dimension arrows from the axis to one envelope and one aperture edge, not full diameters. A z dimension line lies below the envelopes.

Quantity labels: `w0` → `waist_radius`; `w(z)` → `beam_radius`; `a` → `aperture_radius`; `z` → `axial_distance`; incoming `P` → `beam_power`; outgoing `Pcap` → `captured_power`; `lambda` → `wavelength`. Caption: “Illustrative 1/e² Gaussian contours; not a live plot or hard beam boundary. Captured power is evaluated at the centered aperture. Post-aperture diffraction is excluded.”


## 5. `opto.step_index_mode_cutoff`

### Purpose, novelty and source

Use this intermediate model to screen fiber core/index choices for single-mode operation. N4 pages 3–4 supplies `NA=sqrt(ncore²-nclad²)`, `V=2*pi*a*NA/lambda` and the approximate LP11 cutoff `V=2.405`. Its physical introduction describes approximately 1% core/cladding contrast.

Closest baseline: `signals.coax_impedance` computes quasi-TEM electrical characteristic impedance; it does not solve dielectric waveguide higher-mode eligibility. `signals.line_propagation` has no core or cladding. Unlike the Gaussian aperture model, numerical aperture does not say how much a given optical launch couples into the fundamental mode. No mode-count approximation or inverse mode is added as a separate item.

### Model and physical domain

Compute NA with `sqrt(ncore-nclad)*sqrt(ncore+nclad)` to avoid subtracting nearly equal squares. Compute `normalized_frequency`, `cutoff_wavelength=2*pi*a*NA/2.405`, and `single_mode_margin=2.405-V`. Positive margin indicates a single spatial mode in this approximation; both polarization states can still exist. Negative margin is a valid result indicating higher-mode support, not an input error.

Require ordinary dielectric indices at least one, `ncore>nclad`, positive a and wavelength, and relative index contrast `(ncore-nclad)/ncore<=0.01`. This explicit weak-guidance window limits the LP approximation; it is not a rigorous error bound or a claim that 1.01% contrast physically cannot guide light. The fiber must be circular, uniform and straight with a step-index profile. Holding the indices constant when finding cutoff neglects their wavelength dispersion. Actual cable cutoff and bend loss require manufacturer data. Do not treat NA as guaranteed efficient acceptance for a single-mode launch.

### Complete independent example

Inputs: `core_index=1.45`, `cladding_index=1.444`, `core_radius=4.1e-6`, `wavelength=1.55e-6`.

- `numerical_aperture=sqrt((1.45-1.444)*(1.45+1.444))=sqrt(0.017364)=0.131772531280233059049`.
- `normalized_frequency=2*pi*4.1e-6*NA/1.55e-6=2.19006455029824100986`.
- `cutoff_wavelength=2*pi*4.1e-6*NA/2.405=1.41147611349782684627e-6 m`.
- `single_mode_margin=2.405-2.19006455029824100986=0.214935449701758990144`.

### Tester cases

All four nominal outputs. Doubling core radius doubles V and cutoff wavelength, keeps NA fixed and makes the margin negative; do not reject this physically meaningful multimode result. At a wavelength equal to the independently computed cutoff, V is approximately 2.405 and margin approximately zero; allow subtraction-rounding residual relative to V, not an arbitrary large absolute floor. Increasing cladding index toward core index must reduce positive NA, V and cutoff. Test the 1% contrast endpoint and just above it. Reject equal/reversed indices, either index below one, nonpositive radius/wavelength and contrast beyond the admitted weak-guidance scope.

### Mandatory diagram

Show a longitudinal fiber section with a narrow core stripe x=165..545, y=120..165 and wider cladding y=70..215. Add a small circular cross-section to the right with a center-to-core-edge radius arrow, not an outer-cladding-radius arrow. A single smooth fundamental-mode intensity lobe can appear above the longitudinal section as an unbound conceptual sketch; no zigzag ray is presented as a computed single-mode field. Put `V < 2.405: one spatial mode` in static explanatory text.

Quantity labels: `n1` → `core_index`; `n2` → `cladding_index`; `a` → `core_radius`; `lambda` → `wavelength`; `V` → `normalized_frequency`; `lambda_c` → `cutoff_wavelength`. Caption: “Circular weakly guiding step-index model. The LP11 cutoff is approximately V=2.405; the positive margin indicates single spatial mode, not a measured cable rating or coupling efficiency.”


## 6. `opto.fiber_gaussian_dispersion`

### Purpose, novelty and source

Use this expert model to determine whether a chirped optical pulse broadens or compresses in a fiber. U1 slide 6.3 gives the width ratio `sqrt((1+C*beta2*L/T0²)²+(beta2*L/T0²)²)`; slide 6.4 states initial compression for `C*beta2<0`. Slides 6.2 and 6.7 distinguish linear dispersion from attenuation and nonlinear terms.

Closest baseline: `signals.rise_time_bandwidth` describes a single-pole electrical edge and `signals.line_propagation` gives nondispersive delay. Neither represents quadratic optical spectral phase or chirp-dependent compression. `noise.uncorrelated_rms_sum` adds unrelated random amplitudes; the two terms here derive from one coherently propagated Gaussian field. The spatial Gaussian item concerns a different domain and boundary-value problem.

### Model and physical domain

Define the input envelope `A(0,t)=A0*exp(-(1+i*C)*t²/(2*T0²))` and propagation sign convention `dA/dz=-i*beta2*d²A/dt²/2`. These definitions produce the catalog's plus sign in `1+C*x`. The intensity is `P0*exp(-t²/T0²)`; T0 is a 1/e intensity half-width. It is neither RMS width nor FWHM.

Set `x=beta2*L/T0²`, `b=hypot(1+C*x,x)` and `Tout=T0*b`. Solve the Gaussian field propagation, then integrate the resulting intensity: conserved energy `Ep=sqrt(pi)*Ppeak*Tout`. Therefore `output_fwhm=2*sqrt(ln(2))*Tout` and `output_peak_power=Ep/(sqrt(pi)*Tout)`.

Require positive initial width and nonnegative length/energy. C and beta2 are signed. Zero beta2 and zero length are supported exactly without a dispersion-length division. The fiber is single mode with constant beta2 across the relevant spectrum; Kerr phase, higher-order dispersion, PMD and loss are neglected. Near the actual zero-dispersion wavelength, beta3 may invalidate the beta2-only approximation even though the algebra accepts zero. Peak power is a lossless idealization, not a nonlinear/damage rating. `1 ps²/km=1e-27 s²/m`.

### Complete independent example

Inputs: `initial_width=1e-11`, `initial_chirp=1`, `beta_two=-2e-26`, `fiber_length=2000`, `pulse_energy=1e-13`.

- `normalized_dispersion=(-2e-26*2000)/(1e-11)²=-0.4`.
- `width_ratio=sqrt((1-0.4)²+(-0.4)²)=sqrt(0.52)=0.721110255092797858624`.
- `output_width=1e-11*sqrt(0.52)=7.21110255092797858624e-12 s`.
- `output_fwhm=2*sqrt(ln(2))*output_width=1.20072733606122511880e-11 s`.
- `output_peak_power=1e-13/(sqrt(pi)*output_width)=0.00782390181755426780712 W`.

This example is genuine compression, not a positive-only broadening special case.

### Tester cases

Assert all five outputs. With length zero: normalized dispersion 0, width ratio 1, width `1e-11 s`, FWHM `1.66510922231539551271e-11 s`, peak power `0.00564189583547756286948 W`. Beta2 zero gives the same limit at nonzero length. With C=0 and length 5000 m at the nominal beta2/T0, x=-1 and width ratio sqrt(2). With C=1, length 2500 m gives x=-0.5 and the minimum ratio `1/sqrt(2)`; past that distance it grows again. Reversing both C and beta2 preserves all outputs except the sign of normalized dispersion. Zero energy makes only peak power zero. Reject nonpositive initial width and negative length/energy. No fake finite result may hide unrepresentable arithmetic; no arbitrary upper chirp or fiber-length clamp is part of the physical model.

### Mandatory diagram

Use a left input-pulse panel x=40..200, a fiber block x=250..440, and right output-pulse panel x=500..665. Draw positive intensity pulses with the output narrower and taller than the input, matching the documented compression example qualitatively. Mark horizontal width dimensions as 1/e half-widths on one side of each pulse center, not FWHM spans. Within the fiber block label beta2 and L. Add two small frequency-colored or differently marked conceptual wavelets and a static note `C*beta2 < 0 can compress`; do not bind that relation to a nonexistent quantity.

Quantity labels: `T0` → `initial_width`; `C` → `initial_chirp`; `beta2` → `beta_two`; `L` → `fiber_length`; `Tout` → `output_width`; `Ppeak` → `output_peak_power`. Caption: “Illustrative compression case, not a live waveform. Gaussian intensity widths are 1/e half-widths. Linear second-order dispersion conserves pulse energy; chirp sign can reverse initial broadening.”


## 7. `opto.sipm_finite_cell_response`

### Purpose, novelty and source

Use this expert model to screen SiPM microcell count and expected output charge for short optical pulses. H2 section V Dynamic range equation 9 gives `Nf=Ncell*(1-exp(-Nph*PDE/Ncell))`. Sections II equations 6–7 give gain in electrons per avalanche and total output charge. A uniform Poisson occupancy derivation yields the same form: the probability a cell receives no detected seed is `exp(-S/Ncell)`.

Closest baseline: `noise.junction_shot` describes current fluctuations without discrete available detection sites. `noise.analog_dynamic_range` compares signal and noise amplitudes; it has no occupancy saturation law. Closest neighboring optical item: unity-gain photodiode current remains proportional to flux within its stated range, whereas this item models finite-cell Geiger-mode saturation and pulsed avalanche charge. Gain is supplied, not a second capacitor-charge calculator.

### Model and physical domain

`seed_count=photon_count*detection_efficiency`; `u=seed_count/cell_count`; `relative_response=exprel(-u)`; `fired_cells=seed_count*relative_response`; `output_charge=q*avalanche_gain*fired_cells`. The equivalent occupancy expression uses `-cell_count*expm1(-u)`. Using the seed-times-exprel form preserves low exposure and the analytic zero limit without dividing by a zero seed count.

Require nonnegative photon mean, `0<=detection_efficiency<=1`, positive gain and an integer cell count at least one. Photon mean need not be integral; mean fired cells need not be integral. Gain/PDE must match overvoltage, temperature and wavelength. Uniform exposure and initially fully recovered cells are required. The pulse must be much shorter than recovery; pulse spacing must permit recovery. Ignore crosstalk, afterpulsing, dark counts and repeated cell firing. PDE excludes those correlated effects.

**Important physical limitation:** H2 explicitly presents measured over-saturation above the model's cell-count limit. Therefore `fired_cells<=cell_count` is a property of this occupancy model, not a guaranteed bound on measured SiPM charge. Do not hide that caveat in implementation documentation or interpret the result as a detector rating.

### Complete independent example

Inputs: `photon_count=2000`, `detection_efficiency=0.4`, `cell_count=1600`, `avalanche_gain=1000000`.

- `seed_count=2000*0.4=800`.
- `fired_cells=1600*(1-exp(-800/1600))=629.550944459786522234`.
- `relative_response=fired_cells/800=0.786938680574733152792`.
- `output_charge=1.602176634e-19*1000000*fired_cells=1.00865181312610171855e-10 C`.

### Tester cases

Assert all four nominal outputs. Zero photon mean or zero PDE yields seed/fired/charge exactly zero and relative response exactly one. For `photon_count=4e-17`, nominal PDE/cells/gain, seed count is `1.6e-17`, fired count approaches `1.6e-17`, response rounds to one and charge approaches `2.5634826144e-30 C`; do not lose the fired count to cancellation. Increasing photons at fixed cells monotonically increases fired count toward cell count but decreases relative response. Doubling gain doubles only charge. `cell_count=1` is valid, while 1.5 is invalid. Reject zero cell count, nonpositive gain, negative photon mean and PDE outside [0,1]. The nominal uniformity/recovery assumptions cannot be inferred from these four scalar inputs; testers must not claim they were validated.

### Mandatory diagram

Draw an array of identical microcell boxes centered around x=320, y=145, with a few visibly marked as fired. Several optical arrows can land on one cell, but only one outgoing contribution is shown for that cell. A collecting output arrow points right to a charge label. Place a small unbound conceptual saturation curve in a separate lower-right panel only if it fits without crowding; it must be labeled illustrative and not contain calculated numeric coordinates. The drawn cell count is a schematic subset, never a literal rendering of the entered integer.

Quantity labels: `Nph` → `photon_count`; `PDE` → `detection_efficiency`; `Ncell` → `cell_count`; `Nf` → `fired_cells`; `Q` → `output_charge`. Caption: “Uniform short-pulse occupancy of recovered microcells; repeated photons on one cell add no second firing in this model. Array and any response curve are illustrative. Crosstalk, recovery and measured over-saturation are excluded.”

