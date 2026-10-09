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

