# Signals: researched transmission-line calculations

Eight distinct calculations, implemented as definitions in [signals.json](signals.json). The lane covers basic, intermediate and expert hardware design. A wavelength is an output of line propagation, not a ninth calculator or an inverse rearrangement counted as a separate physical problem. Source termination, load reflection and quarter-wave transformation represent three different boundary/design tasks. PCB trace current, resistance and pair skew; EMC coupling; filter synthesis; op-amp gain-bandwidth; and digital RC threshold timing remain outside this lane.

## Research scope and evidence quality

Research question: which closed-form, real-valued models can estimate isolated transmission-line geometries, propagation, boundary reflections and practical matching choices using the contract's expression language?

Searches used public web search followed by direct source reads: Hammerstad–Jensen microstrip equations, symmetric stripline fringe correction, TI reflection/source termination, TI rise-time bandwidth, and Ellingson coax/quarter-wave derivations. English-language manufacturer application notes and university-authored open textbooks were included, without a publication-date cutoff. Wiki, vendor calculator result pages and unattributed equations were excluded as supporting evidence. This is a focused engineering-model review, not a systematic literature census. Six directly inspected references support the eight definitions:

| Ref | Inspected source and origin | Evidentiary role / limitations |
|---|---|---|
| S1 | [Michael Steer, *Microstrip Transmission Lines*, NC State](https://eng.libretexts.org/Bookshelves/Electrical_Engineering/Electronics/Fundamentals_of_Microwave_and_RF_Design_%28Steer%29/04%3A_Planar_Transmission_Lines/4.04%3A_Microstrip_Transmission_Lines) | University-authored technical textbook: explicit fit coefficients, source references and applicability ranges. Strong support for implementing the documented quasi-static fit, not a primary experimental validation of a specific PCB. |
| S2 | [Michael Steer, *Stripline*, NC State](https://eng.libretexts.org/Bookshelves/Electrical_Engineering/Electronics/Microwave_and_RF_Design_II_-_Transmission_Lines_%28Steer%29/03%3A_Planar_Transmission_Lines/3.07%3A_Stripline) | University-authored technical textbook: finite-thickness equations and quoted accuracy domain, tracing the fit to its cited literature. Same author as S1; not independent corroboration of S1. |
| S3 | [Steven W. Ellingson, *Coaxial Line*, Virginia Tech](https://eng.libretexts.org/Bookshelves/Electrical_Engineering/Electro-Optics/Book%3A_Electromagnetics_I_%28Ellingson%29/03%3A_Transmission_Lines/3.10%3A__Coaxial_Line) | University-authored analytic derivation of TEM coax quantities; high confidence within explicit low-loss assumptions. Geometry and material variability remain outside the model. |
| S4 | [Texas Instruments AN-807, *Reflections: Computations and Waveforms*](https://www.ti.com/lit/an/snla027b/snla027b.pdf) | Manufacturer application report, revised reprint of the Fairchild handbook; develops launch and boundary relations from circuit laws. Strong support for ideal reflection/termination physics; no certification of particular drivers. |
| S5 | [John Rice / Texas Instruments, *Are You Accurately Measuring the Picosecond Rise Time of Your GaN Device?*](https://www.ti.com/document-viewer/lit/html/SSZTCJ0/GUID-3646E4F7-2FCF-4F97-8465-C0F6DF48F197) | Manufacturer technical article derives rounded 0.35/tr and shows measurement limitations. Device marketing context does not change the elementary single-pole derivation. |
| S6 | [Steven W. Ellingson, *Quarter-Wavelength Transmission Line*, Virginia Tech](https://phys.libretexts.org/Bookshelves/Electricity_and_Magnetism/Electromagnetics_I_%28Ellingson%29/03%3A_Transmission_Lines/3.19%3A_Quarter-Wavelength_Transmission_Line) | University-authored analytic impedance-inversion/matching derivation; high confidence for lossless lines and real resistances. Same author as S3, not an additional independent measurement campaign. |

There are six directly inspected sources (S1–S6); the manufacturer article's extracted step-response expression loses the minus sign, and some textbook examples contain dimensional/typographical mistakes. Definitions here follow the correctly derived physical equations, not those incidental transcription/example errors. In particular, propagation velocity is **1/sqrt(L'C')**, not 1/(L'C'), and an exponential step is **1-exp(-t/tau)**. No disagreement in the underlying ideal-line relationships was found. The material not established here is quantitative accuracy on solder-masked, rough-copper, asymmetric, dispersive or coupled fabricated stackups; use a suitable field solver or measurement for those cases. Fit confidence is conditional on stated domains and is not a standards-compliance assertion.

## Numerical and implementation conventions

All geometry uses metres, time seconds, frequency hertz, impedance ohms and voltage volts. Relative permittivity, ratios, fit coefficients and power fractions have unit `1`. Electrical length is in radians. Use natural logarithms. The speed constant is exactly 299792458 m/s. The coax/microstrip impedance coefficient is the textbook's rounded 60 ohm; do not replace it with a different free-space-impedance coefficient while retaining these expected numbers.

Only contract operators/functions are needed: arithmetic, powers, sqrt, ln, exp and max. Outputs are sequential; constraints contain input keys/constants only. Every example lists every output. The examples below were calculated independently using ordinary Python mathematical arithmetic from the source equations, not the Rust engine or the catalog evaluator. These are research numerical derivations, not executed implementation tests. Defaults equal the worked-example inputs and have finite valid outputs. Relative numerical tolerance is 1e-6; it is not a statement of physical fit accuracy.

## 1. Thin-conductor microstrip impedance — expert

**ID:** `signals.microstrip_impedance`. **Source:** S1, sections 4.4.1–4.4.2, equations 9–14.

An isolated microstrip lies on a homogeneous substrate with an air region above it. Its inhomogeneous electric field is represented by an effective relative permittivity, epsilon_eff = C'/C'_air. Because the nonmagnetic inductance is unaffected by replacing the dielectric with air, Z0 = Z_air/sqrt(epsilon_eff). Hammerstad–Jensen fits the electrostatic geometry as follows, with u = w/h:

- a = 1 + ln[(u^4 + (u/52)^2)/(u^4 + 0.432)]/49 + ln[1 + (u/18.1)^3]/18.7.
- b = 0.564[(epsilon_r - 0.9)/(epsilon_r + 3)]^0.053.
- epsilon_eff = (epsilon_r + 1)/2 + (epsilon_r - 1)(1 + 10/u)^(-a b)/2.
- F = 6 + (2 pi - 6) exp[-(30.666/u)^0.7528].
- Z_air = 60 ln[F/u + sqrt(1 + (2/u)^2)] ohm.
- Z0 = Z_air/sqrt(epsilon_eff).

**Domain guards:** w,h,f > 0; t >= 0; 1 <= epsilon_r <= 128; 0.01 <= w/h <= 100. Positive geometry makes every logarithm/power/root argument valid. Source quotes effective-permittivity fit accuracy better than 0.2% within the preceding ratio/permittivity range; the air-impedance fit is quoted better than 0.1% for w/h < 1000. These numerical-fit claims exclude physical stackup errors.

This is the **zero-thickness** fit. Supplied t is an applicability input, not a hidden finite-thickness correction. The catalog conservatively enforces t/h <= 0.01 and t/w <= 0.01. It also enforces f h sqrt(epsilon_r)/c <= 0.01, corresponding to a substrate height no more than 1% of the bulk-dielectric wavelength. These explicit engineering restrictions implement “thin conductor” and “electrically thin substrate”; they do not come with a new published error guarantee. Ordinary 35-micrometre PCB copper may not pass this deliberately thin-film model. No solder mask, neighboring conductor, roughness, dispersive material, finite plane or etched trapezoid is modeled.

**Independent example/default:** w = 0.8 mm = 0.0008 m, h = 0.4 mm, t = 2 micrometres, epsilon_r = 3.66, f = 1 GHz. Then u = 2, a = 0.9995302713283848, b = 0.5382736881490864, F = 6.000115105754384, epsilon_eff = 2.8372126159506297, Z_air = 89.09056365708382 ohm and **Z0 = 52.89148559435723 ohm**. Thickness ratios are 0.005 and 0.0025; electrical thickness is approximately 0.0025526, below 0.01. Effective permittivity lies between air and substrate, as required physically.

## 2. Symmetric finite-thickness stripline impedance — expert

**ID:** `signals.stripline_impedance`. **Source:** S2, section 3.7.1, equations 1–3.

A centered strip between two ground planes is entirely immersed in dielectric, so its TEM permittivity is the bulk epsilon_r. Let b denote **total** ground-plane separation (not distance to a single plane), u = w/b and q = t/b. The empirical effective width and dimensionless fringe correction are:

- u_eff = u - max(0, 0.35 - u)^2/(1 + 12q).
- C_f = (2/pi) ln[1/(1-q) + 1] - (q/pi) ln[1/(1-q)^2 - 1].
- Z0 = (30 pi/sqrt(epsilon_r))(1-q)/(u_eff + C_f).

The `max` expression reproduces the published narrow/wide branches without unsupported conditionals: subtract the correction only when u < 0.35. The implemented second logarithm uses the algebraically equivalent **q(2-q)/(1-q)^2** argument to avoid subtracting nearly equal numbers for small q. Despite its name, C_f is dimensionless here, not a lumped capacitance in farads.

**Domain guards:** w,b,t > 0; epsilon_r >= 1; 0 < t/b < 0.025; b-t > 0; w/(b-t) > 0.05. These are the source's quoted better-than-1% fit conditions; all logarithm arguments are then positive. The geometry must also be centered, homogeneous and operated in its TEM regime; those physical conditions are disclosed assumptions rather than pretended machine-checkable geometry. A strip of exactly zero thickness is excluded from this finite-thickness logarithmic expression; it would need the analytic limiting formula rather than an arbitrary epsilon thickness. No asymmetric embedding, coupled traces or losses are modeled.

**Independent example/default:** w = 0.6 mm, b = 1 mm, t = 18 micrometres, epsilon_r = 4. Then u = 0.6, q = 0.018, u_eff = 0.6, C_f = 0.4659693534707028 and **Z0 = 43.41181070235101 ohm**. The width/clearance ratio is 0.6/0.982 = 0.61099796 > 0.05; q < 0.025. Retaining full pi and logarithms rather than rounded 94.25/0.441 values matters for the exact numerical reference.

## 3. Homogeneous coaxial line impedance — intermediate

**ID:** `signals.coax_impedance`. **Source:** S3.

For inner-conductor **outer radius** a and outer-conductor **inner radius** b, the homogeneous nonmagnetic coax has L' = mu0 ln(b/a)/(2 pi) and C' = 2 pi epsilon0 epsilon_r/ln(b/a). Applying Z0 = sqrt(L'/C') gives:

**Z0 = 60 ln(b/a)/sqrt(epsilon_r) ohm**, using the customary rounded coefficient.

**Domain guards:** b > a > 0; epsilon_r >= 1. Ratios are dimensionless and the logarithm is positive. The model presumes concentric conductors, uniform filler and a low-loss TEM mode: R' << omega L' and G' << omega C'. The catalog does not infer these inequalities without conductor/dielectric-loss inputs. Skin-dependent loss, dielectric attenuation, higher modes and braid effects are excluded; the shield's exterior radius or jacket diameter must not be substituted for b.

**Independent example/default:** a = 0.5 mm, b = 1.5 mm, epsilon_r = 2.25. b/a = 3, sqrt(epsilon_r) = 1.5, ln(3) = 1.0986122886681098, giving **Z0 = 40 ln(3) = 43.94449154672439 ohm**.

## 4. Transmission-line delay and wavelength — basic

**ID:** `signals.line_propagation`. **Sources:** S3 phase-velocity/wavelength derivation and S1 effective-permittivity discussion.

A low-loss nondispersive line has v = 1/sqrt(L'C') = c/sqrt(epsilon_eff). Consequently:

- One-way flight time t_d = l/v seconds.
- Round-trip flight time = 2 t_d.
- Guide wavelength lambda = v/f metres.
- Electrical length theta = beta l = 2 pi l/lambda radians.

These are related outputs of one propagation model, not four independent calculators. The reported theta is unwrapped positive phase accumulation for positive l, not the negative transfer-function phase angle.

**Domain guards:** l >= 0, f > 0 and epsilon_eff >= 1. These ensure finite positive velocity/wavelength and valid denominators/roots. Use bulk epsilon_r for homogeneous coax/stripline, but **effective** permittivity for microstrip. Identifying phase velocity with signal-flight/group velocity presumes negligible dispersion. No pair skew, waveform degradation or “electrically short” pass/fail heuristic is included.

**Independent example/default:** l = 0.15 m, epsilon_eff = 3 and f = 1 GHz. v = 299792458/sqrt(3) = **173085256.3273196 m/s**; t_d = **8.666249406959115e-10 s** (0.8666249407 ns); round trip = **1.733249881391823e-9 s**; lambda = **0.17308525632731958 m**; theta = **5.445165094215932 rad**. A zero-length line legitimately returns zero time and zero electrical length while retaining a nonzero wavelength.

## 5. Resistive-load reflection and first arrival — intermediate

**ID:** `signals.load_reflection`. **Sources:** S4 sections 5–6; S6 states the load coefficient explicitly.

At the load, voltage continuity gives V_L = V_i + V_r, and current continuity gives I_L = (V_i - V_r)/Z0. Substituting V_L = R_L I_L and solving:

- Gamma_L = (R_L - Z0)/(R_L + Z0).
- V_r = Gamma_L V_i.
- V_L = (1 + Gamma_L) V_i.
- Reflected power fraction = Gamma_L^2; accepted load power fraction = 1 - Gamma_L^2.

**Domain guards:** Z0 > 0 and finite R_L >= 0; the denominator is strictly positive. Incident voltage may be signed, including zero. The voltage input is the wave arriving at the load, **not** the driver open-circuit voltage. Fractions describe the power split for this real-impedance boundary; for zero excitation they still describe the coefficient-based split but are not a measured 0/0 ratio. An ideal short is allowed: Gamma = -1 and V_L = 0. An infinite ideal open cannot be a finite JSON input; the open limit is Gamma = +1 but is not silently simulated by an arbitrary huge resistance. Reactive loads and subsequent multiple bounces are excluded.

**Independent example/default:** Z0 = 50 ohm, R_L = 75 ohm, V_i = 1.2 V. Gamma = 25/125 = **0.2**; V_r = **0.24 V**; V_L = **1.44 V**; reflected fraction = **0.04**; accepted fraction = **0.96**. Energy check by hand: incident power 1.2^2/50 = 0.0288 W and load power 1.44^2/75 = 0.027648 W, exactly 96% of incident power. These are single-boundary wave powers, not a new generic resistor-power calculator.

## 6. Source-series termination for an open receiver — intermediate

**ID:** `signals.source_series_termination`. **Source:** S4 sections 3, 5 and 10.

The source is an ideal step V_step with Thevenin resistance R_driver. A series resistor adjacent to the driver makes total source resistance equal to the line impedance:

- R_series = Z0 - R_driver.
- Initial wave V_launch = V_step Z0/(R_driver + R_series + Z0).
- Open receiver has Gamma_L = +1, so its first-arrival voltage is V_receiver = 2 V_launch.

The matched source has Gamma_S = 0 and absorbs the returning reflected wave. This model therefore solves **component selection at the source and the launched step**, rather than inverting the load reflection calculation. Under an exact match the launch is one-half of the open-circuit step.

**Domain guards:** Z0 > 0, 0 <= R_driver <= Z0. A driver with resistance already above Z0 cannot be corrected by adding a passive series resistor and must be rejected, not clamped to a false zero-ohm solution. V_step is signed; R_driver = Z0 legitimately yields R_series = 0. Physical restrictions: single point-to-point line, ideal high-impedance/open receiver, negligible receiver capacitance, source resistor at the driver and approximately constant driver resistance. Branches, nonlinear pull-up/pull-down I–V behavior and finite edge waveforms are not modeled.

**Independent example/default:** Z0 = 60 ohm, R_driver = 18 ohm and V_step = 3.3 V. R_series = **42 ohm**; V_launch = 3.3*60/(18+42+60) = **1.65 V**; first-arrival open-receiver voltage = **3.3 V**. The driver end does not jump to full voltage immediately; returning-wave absorption happens after a round trip. Practical standard-value selection and IBIS verification are not claimed by this ideal calculator.

## 7. Single-pole rise-time equivalent bandwidth — basic

**ID:** `signals.rise_time_bandwidth`. **Source:** S5, equations 1–2 and accompanying discussion.

For the normalized step y(t) = 1 - exp(-t/tau): t10 = -tau ln(0.9), t90 = -tau ln(0.1), and t_r = t90 - t10 = tau ln(9). Since the first-order pole's -3 dB bandwidth is 1/(2 pi tau):

**BW = ln(9)/(2 pi t_r) = 0.3496991525660598/t_r**, conventionally rounded to 0.35/t_r.

**Domain guard:** t_r > 0 seconds, ensuring valid division. The exact logarithmic factor is intentionally used in both the definition and expected output. Only a monotonic single-real-pole exponential response with 10–90% rise time is represented. Arbitrary digital edges, different threshold conventions, ringing and multipole/Gaussian responses are not represented by the same exact factor. This is an equivalent bandwidth, not a universal spectral cutoff, data-rate/Nyquist estimate, filter synthesis or a sufficient oscilloscope bandwidth for 1% accuracy. S5 itself explains that accurate measurement needs substantially more bandwidth than the edge's equivalent value.

**Independent example/default:** t_r = 2 ns = 2e-9 s. ln(9) = 2.1972245773362196, so **BW = 174849576.2830299 Hz** (174.8495763 MHz). Rounding the factor to 0.35 would instead give 175 MHz, a small but intentional numerical difference.

## 8. Quarter-wave impedance transformer — expert

**ID:** `signals.quarter_wave_transformer`. **Source:** S6.

For a lossless section of length lambda/4, its input impedance is Z_in = Z_t^2/R_L. Setting Z_in equal to positive real reference resistance R_S gives:

- Required section characteristic impedance Z_t = sqrt(R_S R_L) ohm.
- Required physical length l = lambda/4 = c/[4 f sqrt(epsilon_eff)] metres.

This is a **distributed impedance-inversion/matching design** with a new intermediate impedance, not a rearranged propagation formula or resistive source termination. Its length uses the wavelength inside the matching section, never free-space wavelength by default.

**Domain guards:** R_S > 0, R_L > 0, f > 0 and epsilon_eff >= 1. These ensure valid square root and nonzero length denominator. Complex/reactive loads are deliberately excluded: they generally require an additional transformation before a real quarter-wave section can match them. The input epsilon_eff belongs to the **transformer section**, not automatically either adjoining line. For microstrip it depends on section width and must be obtained from that section's actual geometry/material. The ideal match is exact at the design frequency only; discontinuities, dispersion, loss, fabrication tolerances and match bandwidth are not estimated.

**Independent example/default:** R_S = 50 ohm, R_L = 100 ohm, f = 2.4 GHz, epsilon_eff = 2.25. Z_t = sqrt(5000) = **70.71067811865476 ohm**. Velocity is c/1.5; section length = 299792458/(4*2.4e9*1.5) = **0.020818920694444444 m** (20.81892069 mm). Independent impedance-inversion substitution gives Z_in = 70.71067811865476^2/100 = 50 ohm, as designed. Converting the target impedance to a physical strip width is not separately counted or silently approximated here.

## Integration handoff

Files contain exactly the eight IDs listed above. Cross-input constraints use only inputs/constants; output dependencies proceed in source order and no output key duplicates an input key. Expected maps cover every displayed output. No build, lint, test or formatter was run in this research lane. Parent should run schema/expression validation, all examples and invalid-domain cases during integration, especially microstrip ratio/thickness/electrical-thickness limits, stripline zero-thickness/0.025 boundary and narrow-width branch, coax reversed radii, reflection short/match limits, source resistance exceeding Z0, and zero frequency/rise time.
