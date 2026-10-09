# Magnetics and transformers — eight researched calculations

Catalog: [magnetics.json](magnetics.json). This lane contains exactly eight distinct physical design problems: ideal transformer transformation; sinusoidal flux excitation; ungapped toroidal geometry; discrete-gap magnetic reluctance; material-volume field energy; conductor penetration; winding VA allocation; and partial magnetic coupling. Levels: two basic, three intermediate, three expert.

## Research scope and evidence

Search performed 2026-10-09 using web search and direct source reads. English university teaching material, university research and magnetic-component manufacturers were searched for turns ratios, Faraday's-law waveform factor, rectangular toroid integration, finite-permeability air gaps, field energy density, good-conductor skin depth, winding VA and coupled-coil inductance. No publication-date cutoff was imposed because the closed-form linear models are classical; contemporary operating conditions must still come from component/material data. Commercial calculator pages, unsourced formula aggregators and Wikipedia mirrors were excluded. This is a focused formula review, not an exhaustive literature or standards review.

### Source quality and actual support

| Source | Evidence type and weight | Relevant support |
|---|---|---|
| [MIT 6.014 Lecture 11](https://web.mit.edu/6.014/www/handouts/lecture11_notes.pdf) | University-authored teaching derivations; high weight for ideal linear models, not measured component accuracy | Faraday voltage, ideal turns ratio, magnetic energy density, toroidal and gap models |
| [MIT 8.02 Inductance Worked Examples](https://web.mit.edu/course/8/8.02-esg/Spring03/www/8.02ch32we.pdf) | University-authored worked derivations; high weight within stated geometry | Example 2 integrates the rectangular air-core toroid exactly; examples 1 and 3 show field energy density |
| [MIT Electromagnetic Fields and Energy §9.7](https://www.mit.edu/course/6/6.013_book/www/chapter9/9.7.html) | Authoritative university textbook treatment; high model weight | Finite-permeability air-gap circuit, flux continuity, and linear two-winding linkage matrix. Equations are embedded images; accompanying text states the assumptions |
| [Micrometals Issue L, February 2007](https://web.mit.edu/6.131/www/document/micrometals.pdf) | Original manufacturer engineering guide; high weight for its waveform convention, product-specific for loss data | Printed page 29 relates sine RMS voltage to peak flux, explains 4.44 and SI/CGS conversion. No loss fits are imported |
| [MIT CBA Circuits, Transmission Lines, and Waveguides](https://fab.cba.mit.edu/classes/862.19/notes/circuits.pdf) | University-authored electromagnetic derivation; high weight for conduction-dominant materials | §7.2.1, equations 7.26–7.35 derive diffusion, skin depth and exponential penetration; room-temperature copper conductivity example |
| [Hammond 265 transformer family](https://www.hammfg.com/electronics/transformers/power/265) | Original manufacturer rating tables; high weight for practical RMS/VA interpretation, not universal thermal margin | 5 V × 30 A = 150 VA and 24 V × 10 A = 240 VA examples; series/parallel wiring keeps overall VA constant |
| [Hammond rectifier design guide](https://www.hammfg.com/electronics/transformers/rectifier) | Original manufacturer application guidance; high weight for its caveats | AC voltage references are RMS; winding AC current must be recalculated from DC load current according to rectifier/filter topology |
| [UC Berkeley EECS-2007-3](https://www2.eecs.berkeley.edu/Pubs/TechRpts/2007/EECS-2007-3.pdf) | Original university dissertation with a theoretical subsection; high weight for the coupling definition, no empirical accuracy claim | §3.2.2 equations 3.6–3.8 define reciprocal mutual inductance and k = M/sqrt(L1 L2), with 0 ≤ k ≤ 1 |
| [Stanford EE102 exercises](https://stanford.edu/~boyd/ee102/101exercises.pdf) | University-authored teaching problems; supporting rather than independent proof | Static-circuit exercise 10 explicitly asks for primary-referred transformer load resistance |

All nine cited documents/pages were read. MIT sources share a classical theory base and are not counted as independent experiments. The Berkeley coupling result agrees with the MIT linear linkage formulation. No conflicting ideal-model formulas were found in the included sources. Main evidence gap: these sources do not establish a universal numerical error bound for finite-core fringing, material linearity, or transformer thermal headroom. Consequently user-supplied limits and explicitly labeled conservative screening rules are used; no certification or empirical precision is implied.

## Shared conventions and implementation contract

- SI inputs throughout: volts, amperes, hertz, meters, square/cubic meters, henries, teslas, webers, S/m. VA is apparent winding load, not watts. Turns and relative material constants are dimensionless; `turn` is a human-facing count unit.
- Use the engineering approximation mu0 = 4π × 10^-7 H/m. Its use does not imply that mu0 is an exactly defined modern SI constant. Skin-depth screening also uses eps0 = 8.8541878128 × 10^-12 F/m.
- All divisions and roots have positive input-domain guards; toroid geometry guards the logarithm. Actual turns are positive integers, enforced with `floor(turns) == turns`.
- Constraints contain input keys/constants only, because the integration engine checks them before evaluating outputs. Output expressions may use preceding outputs. Every example supplies all inputs and all expected output keys.
- Material limits are supplied by the user. The catalog can enforce those limits but cannot verify that permeability, saturation margin, frequency dependence, temperature, or hysteresis assumptions actually describe the selected material.
- Independent examples below were computed with standalone high-precision decimal arithmetic and hand reductions, not the backend expression engine. Defaults equal worked-example inputs. Tolerance 10^-6 is an arithmetic comparison tolerance, not physical model accuracy.
- Only the contract operators/functions are used. There is no simulation, table interpolation, iteration or piecewise branch.

## 1. Ideal transformer turns ratio and reflected load — basic

ID: `magnetics.turns_ratio`.

**Question:** What secondary voltage and primary-referred resistive load result from specified winding turns?

**Derivation:** Perfect coupling makes both coils link the same changing flux. Faraday's law gives Vp = Np dΦ/dt and Vs = Ns dΦ/dt, so a = Vs/Vp = Ns/Np. Lossless power transfer gives Ip = a Is in magnitude. With a resistive load, Is = Vs/Rload; therefore Vp/Ip = Rload/a². This reflected impedance is an output of the same ideal transformation, not counted as another calculator.

**Formula and outputs:**

- `voltage_ratio = secondary_turns/primary_turns` [1].
- `secondary_voltage = primary_voltage*voltage_ratio` [V RMS].
- `reflected_resistance = load_resistance/voltage_ratio^2` [ohm].

**Input units and domain:** Np, Ns [turn] ≥ 1 and integral; Vp [V RMS] ≥ 0; Rload [ohm] > 0. No DC operation: an ideal transformer steady-state voltage ratio is not a DC power-transfer model. Perfect coupling, negligible magnetizing current, winding drops, leakage, saturation and regulation are assumed. Output voltage is a magnitude; dots/polarity are not resolved here.

**Worked example/defaults:** Np = 1000, Ns = 100, Vp = 230 V, Rload = 24 ohm.

1. a = 100/1000 = **0.1**.
2. Vs = 230 × 0.1 = **23 V RMS**.
3. Rprimary = 24/(0.1²) = **2400 ohm**.

**Sources:** [MIT Lecture 11, section B](https://web.mit.edu/6.014/www/handouts/lecture11_notes.pdf); [Stanford exercise 10](https://stanford.edu/~boyd/ee102/101exercises.pdf).

## 2. Sinusoidal transformer peak flux density — intermediate

ID: `magnetics.sinusoidal_flux_density`.

**Question:** Is the alternating core flux compatible with a chosen material operating limit at the applied sine-wave voltage?

**Derivation:** Let B(t) = Bpk sin(2πft), with Φ = Ae B. Then induced v(t) = N Ae 2πf Bpk cos(2πft). Thus Vrms = 2πf N Ae Bpk/√2. The exact sine coefficient is 2π/√2 = 4.442882938..., commonly rounded to 4.44 in manufacturer guidance. The expression deliberately keeps π and √2; otherwise a strict example tolerance would expose the rounding discrepancy.

**Formula and outputs:**

- `peak_flux_density = sqrt(2)*rms_voltage/(2*pi*frequency*turns*core_area)` [T].
- `peak_flux = peak_flux_density*core_area` [Wb].

**Input units/domain:** Vrms [V] ≥ 0; f [Hz] > 0; N [turn] ≥ 1 integral; Ae [m²] > 0; flux_limit [T] > 0. The expanded expression for Bpk must be ≤ flux_limit. The flux limit is an operating bound chosen using material, frequency and temperature data, not a guaranteed universal saturation value.

**Limitations:** Zero-mean sinusoidal voltage, negligible winding drops and leakage, approximately uniform flux over the effective area. No DC offset, half-wave volt-second imbalance, square-wave waveform factor, or PWM excitation. Staying below a field limit does not establish acceptable core loss.

**Worked example/defaults:** Vrms = 24 V, f = 50 Hz, N = 1000, Ae = 0.0004 m², flux_limit = 0.3 T.

- f N Ae = 50 × 1000 × 0.0004 = 20.
- Bpk = 24√2/(40π) = **0.2700948948471318 T**, below 0.3 T.
- Φpk = Bpk × 0.0004 = **0.00010803795793885272 Wb**.

**Sources:** [Micrometals printed page 29](https://web.mit.edu/6.131/www/document/micrometals.pdf) provides the sine RMS form factor and unit conversion; [MIT Lecture 11](https://web.mit.edu/6.014/www/handouts/lecture11_notes.pdf) supplies Faraday's law.

## 3. Rectangular-section toroid inductance — expert

ID: `magnetics.rectangular_toroid_inductance`.

**Question:** What is the inductance of an ungapped toroidal core when radial field variation is retained?

**Derivation:** For inner radius ri, outer radius ro and axial height h, Ampère's law gives H(r) = N I/(2πr), and linear material gives B(r) = mu0 mur N I/(2πr). Integrate flux through one turn:

Φ = ∫ri^ro B(r) h dr = mu0 mur N I h ln(ro/ri)/(2π).

Then L = N Φ/I = mu0 mur N² h ln(ro/ri)/(2π). The MIT worked example derives the air-core formula; replacing mu0 with mu0 mur is justified by the constant linear constitutive relation, not by a manufacturer-specific empirical fit. Maximum B occurs at ri, so the current-specific inner field is also reported and guarded.

**Formula and outputs:**

- `inductance = 4*pi*1e-7*relative_permeability*turns^2*height*ln(outer_radius/inner_radius)/(2*pi)` [H].
- `inner_flux_density = 4*pi*1e-7*relative_permeability*turns*peak_current/(2*pi*inner_radius)` [T].

**Input units/domain:** ri, ro, h [m] > 0; ro > ri; mur [1] > 0; N [turn] ≥ 1 integral; Ipk [A] ≥ 0; linear_flux_limit [T] > 0. The expanded inner-field expression must be ≤ the limit. Distances are radii, not diameters; their ratio is identical if both are consistently converted, but the field guard requires actual radius.

**Limitations:** Homogeneous ungapped rectangular core; evenly distributed winding; toroidal symmetry; negligible leakage, fringing, core losses and winding capacitance; real constant permeability appropriate to the current and frequency. The logarithmic formula is exact within that ideal field model, not exact for every manufactured toroid. It differs physically from the discrete-gap reluctance calculator below and is not an inverse rearrangement.

**Worked example/defaults:** ri = 0.01 m, ro = 0.02 m, h = 0.005 m, mur = 100, N = 50, Ipk = 0.1 A, limit = 0.2 T.

- mu0/(2π) = 2 × 10^-7 H/m.
- L = (2 × 10^-7) × 100 × 2500 × 0.005 × ln(2) = 0.00025 ln(2) = **0.00017328679513998633 H** (173.286795 µH).
- Binner = (2 × 10^-7) × 100 × 50 × 0.1 / 0.01 = **0.01 T**, inside the selected linear domain.

**Sources:** [MIT 8.02 example 2](https://web.mit.edu/course/8/8.02-esg/Spring03/www/8.02ch32we.pdf); [MIT Lecture 11, linear magnetic material and toroidal sections](https://web.mit.edu/6.014/www/handouts/lecture11_notes.pdf).

## 4. Finite-permeability gapped-core inductance — expert

ID: `magnetics.gapped_core_inductance`.

**Question:** What inductance and peak flux result when neither core reluctance nor gap reluctance is discarded?

**Derivation:** Let the rectangular flux area A = w h. Flux continuity gives the same Φ through the core and narrow air gap. With core path lc **excluding** gaps and total series gap g, Ampère's law gives

N I = Hc lc + Hg g = Φ[lc/(mu0 mur A) + g/(mu0 A)].

Define Rc = lc/(mu0 mur A), Rg = g/(mu0 A); Φ = N I/(Rc+Rg). Therefore L = N Φ/I = N²/(Rc+Rg) and Bpk = Φpk/A. This retains finite core permeability. The often quoted gap-dominant formula mu0 N² A/g follows only if g ≫ lc/mur; that simplification is not used here.

**Formula and sequential outputs:**

1. `core_reluctance = core_length/(4*pi*1e-7*relative_permeability*pole_width*pole_height)` [A/Wb].
2. `gap_reluctance = gap_length/(4*pi*1e-7*pole_width*pole_height)` [A/Wb].
3. `inductance = turns^2/(core_reluctance+gap_reluctance)` [H].
4. `peak_flux_density = turns*peak_current/((core_reluctance+gap_reluctance)*pole_width*pole_height)` [T].

**Input units/domain:** lc, w, h, g [m] > 0; mur [1] ≥ 1; N [turn] ≥ 1 integral; Ipk [A] ≥ 0; linear_flux_limit [T] > 0. Geometry screens require g ≤ 0.1 min(w,h) and g ≤ 0.1 lc. Field guard uses the input-only equivalent mu0 N Ipk/(lc/mur+g) ≤ limit.

**Approximation disclosure:** The 0.1 geometry screens are conservative catalog choices, not thresholds stated by MIT, error guarantees or standard compliance. The source requires the gap to be small relative to its cross-sectional dimensions. Equal core/gap area, uniform linear core, one series path, no fringing/leakage or parallel return branches are assumed. Fringing can materially increase effective gap area even for accepted geometry; actual core datasheet AL, bias curves or finite-element analysis may be needed. Total g is the sum of series gaps, not each individual gap; lc already excludes them.

**Worked example/defaults:** lc = 0.1 m, w = h = 0.01 m, A = 10^-4 m², g = 0.0005 m, mur = 2000, N = 100, Ipk = 1 A, limit = 0.3 T.

- Rc = 0.1/[(4π × 10^-7) × 2000 × 10^-4] = **397887.35772973834 A/Wb**.
- Rg = 0.0005/[(4π × 10^-7) × 10^-4] = **3978873.5772973835 A/Wb**.
- Rc is one tenth of Rg; omitting it would overestimate L by 10% relative to the retained-core result.
- L = 10000/(Rc+Rg) = **0.0022847946571562133 H**.
- Bpk = mu0 × 100/(0.1/2000 + 0.0005) = **0.22847946571562133 T**.
- Geometry: 0.0005 ≤ 0.001 and ≤ 0.01 m; predicted Bpk < 0.3 T.

**Sources:** [MIT §9.7 example 9.7.1](https://www.mit.edu/course/6/6.013_book/www/chapter9/9.7.html); [MIT Lecture 11 section C](https://web.mit.edu/6.014/www/handouts/lecture11_notes.pdf).

## 5. Magnetic field energy in a material volume — intermediate

ID: `magnetics.field_energy_density`.

**Question:** How much recoverable magnetic field energy resides in a specified uniform linear-material region?

**Derivation:** The field energy density is u = ∫0^B H(B') dB'. For H = B/mu with constant mu, u = B²/(2mu). Uniform integration over specified volume V gives W = u V. This is the field-domain calculation needed to allocate energy between core and air-gap regions; it deliberately does not accept circuit L and I or duplicate the transients lane's ½ L I² calculator.

**Formula and outputs:**

- `energy_density = flux_density^2/(2*4*pi*1e-7*relative_permeability)` [J/m³].
- `field_energy = energy_density*volume` [J].

**Input units/domain:** B [T] ≥ 0; mur [1] > 0; V [m³] > 0; linear_flux_limit [T] > 0; B ≤ limit. In air, mur ≈ 1. The field is homogeneous and permeability isotropic, real and constant; no hysteresis/nonlinear B-H behavior. The specified volume may be only an air gap, not the whole component. Different material regions must be integrated separately. No ability to validate user material curves is claimed.

**Worked example/defaults:** B = 0.2 T, mur = 1, V = 5 × 10^-8 m³, limit = 0.3 T. The volume could represent A = 10^-4 m² and gap g = 0.0005 m, but its field is independently specified, not copied from the gapped example.

- u = 0.04/(8π × 10^-7) = **15915.494309189534 J/m³**.
- W = u × 5 × 10^-8 = **0.0007957747154594767 J** (0.795774715 mJ).
- At the same field and volume, a linear material with mur = 100 would contain one hundredth this density; this explains why gap energy can dominate.

**Sources:** [MIT Lecture 11 equations 9 and 15](https://web.mit.edu/6.014/www/handouts/lecture11_notes.pdf); [MIT 8.02 examples 1 and 3](https://web.mit.edu/course/8/8.02-esg/Spring03/www/8.02ch32we.pdf).

## 6. Good-conductor skin depth — expert

ID: `magnetics.conductor_skin_depth`.

**Question:** Over what distance does a sinusoidal field/current amplitude penetrate a good conductor?

**Derivation:** Maxwell equations with J = σE, B = mu H and D = eps E give a conductor propagation relation containing both conduction and displacement current. If σ ≫ ω eps, the spatial field equation reduces to ∇²E = jω mu σ E. Its decaying solution is E(z) = E0 exp[-(1+j)z/delta], with delta = √[2/(ω mu σ)] = √[1/(π f mu σ)].

**Formula/output:**

`skin_depth = sqrt(1/(pi*frequency*4*pi*1e-7*relative_permeability*conductivity))` [m].

**Input units/domain:** f [Hz] > 0; σ [S/m] > 0; mur [1] > 0; epsr [1] > 0. Constraint σ ≥ 100 × 2π f × eps0 × epsr ensures a catalog-selected conduction/displacement ratio of at least 100. This threshold is not a sourced universal standard: it makes the neglected term ≤ 1% of the conduction term, without claiming a complete device error bound. Relative permittivity is used only in this domain check.

**Limitations:** Homogeneous isotropic conductor; real material coefficients suitable for frequency/temperature, no significant dispersion or complex magnetic permeability. Planar half-space reference depth, not a complete wire resistance model. A finite conductor thinner than delta does not suddenly stop conducting; currents simply approach a more uniform distribution. Proximity effect, roughness, anomalous skin effect, ferromagnetic loss, optical behavior and litz-wire benefit are not modeled. The 1/e definition refers to amplitude; Joule-power density decays as exp(-2z/delta).

**Worked example/defaults:** f = 100000 Hz, σ = 5.8 × 10^7 S/m, mur = epsr = 1 (room-temperature copper approximation supported by the source).

- π f mu0 σ = 2.32 × 10^6 π² m^-2.
- delta = 1/[π √(2.32 × 10^6)] = **0.0002089806784938892 m**, or **208.980678 µm**.
- Domain threshold: 100 × 2π × 10^5 × eps0 ≈ 0.000556325 S/m, far below the supplied conductivity; the approximation is conduction-dominant.

**Source:** [MIT CBA §7.2.1, equations 7.26–7.35](https://fab.cba.mit.edu/classes/862.19/notes/circuits.pdf).

## 7. Two-secondary transformer VA budget — basic

ID: `magnetics.transformer_va_budget`.

**Question:** What transformer winding VA allocation is needed for two simultaneous RMS loads with a chosen headroom factor?

**Derivation:** A winding's apparent load is S = Vrms Irms; this is not real watts unless the appropriate power-factor condition holds. Budget each independently rated secondary and sum their allocated VA: Sload = S1 + S2. Apply user headroom h ≥ 1 to obtain Sminimum = h Sload. This sum is a conservative nameplate allocation, not a phasor sum of reactive power or a general prediction of input mains apparent power.

**Formula and outputs:**

- `secondary_one_va = secondary_one_voltage*secondary_one_current` [VA].
- `secondary_two_va = secondary_two_voltage*secondary_two_current` [VA].
- `total_load_va = secondary_one_va+secondary_two_va` [VA].
- `minimum_rating_va = headroom_factor*total_load_va` [VA].

**Input units/domain:** V1, V2 [V RMS] > 0; I1, I2 [A RMS] ≥ 0; headroom [1] ≥ 1. Use I2 = 0 for a single-load budget. No expressions divide by an input, so an empty additional-constraints list is intentional.

**Limitations:** Simultaneously operating separate loads/windings only. Check individual winding ratings, series/parallel topology, line frequency, manufacturer thermal derating and starting current independently. A center-tap or series/parallel reconnection must not be double-counted as another full winding. Headroom = 1.25 is an example user choice, not Hammond's universal recommendation or safety certification. Enter actual winding RMS current for a rectifier load; DC load current can differ substantially and no rectifier conversion model is smuggled into this lane.

**Worked example/defaults:** Secondary one: 24 V RMS, 2 A RMS. Secondary two: 12 V RMS, 0.5 A RMS. Headroom = 1.25.

- S1 = 24 × 2 = **48 VA**.
- S2 = 12 × 0.5 = **6 VA**.
- Sload = 48 + 6 = **54 VA**.
- Sminimum = 54 × 1.25 = **67.5 VA**. Select an available component meeting this budget and each individual winding limit; no stock-size table is assumed.

**Sources:** [Hammond 265 rating tables](https://www.hammfg.com/electronics/transformers/power/265); [Hammond rectifier guide RMS/DC caveats](https://www.hammfg.com/electronics/transformers/rectifier).

## 8. Mutual and series-connected coupled-coil inductance — intermediate

ID: `magnetics.coupled_coil_inductance`.

**Question:** How does imperfect coupling change the inductance when two windings are placed in series in either polarity?

**Derivation:** The coupling definition is k = M/√(L1 L2), where the sign of mutual contribution is supplied by the dot convention. For a reciprocal linear pair, λ1 = L1 i1 ± M i2 and λ2 = L2 i2 ± M i1. Series current is common; adding the induced voltages gives Leq = L1 + L2 ± 2M. Passivity imposes M² ≤ L1 L2, so magnitude k is in [0,1].

**Formula and outputs:**

- `mutual_inductance = coupling*sqrt(inductance_one*inductance_two)` [H].
- `series_aiding = inductance_one+inductance_two+2*mutual_inductance` [H].
- `series_opposing = (sqrt(inductance_one)-sqrt(inductance_two))^2+2*(1-coupling)*sqrt(inductance_one*inductance_two)` [H].

The last expression is algebraically identical to L1 + L2 - 2M, but avoids subtracting nearly equal large terms when the windings are close to perfectly coupled. It is not a clamp or fabricated fallback. At equal L and k = 1, the ideal opposing result is exactly zero. For arbitrary unequal L and k = 1, it is (√L1 - √L2)².

**Input units/domain:** L1, L2 [H] > 0; k [1] in [0,1]. Obtain self-inductances with the other winding open, not shorted, and at matching bias/frequency. The bounds alone guard all square roots and guarantee a passive inductance matrix, so no extra constraints are needed.

**Limitations:** Linear reciprocal coils; no saturation, significant dispersion, losses or winding capacitance. Dot polarity determines which actual physical series connection is aiding; both numerical possibilities are returned to avoid a conditional selector. Near-perfect opposing cancellation leaves parasitics dominant in real hardware. This is not the ideal turns-ratio calculator: it explicitly retains nonunity coupling and finite self-inductances.

**Worked example/defaults:** L1 = 100 µH = 0.0001 H, L2 = 400 µH = 0.0004 H, k = 0.9.

- √(L1 L2) = √(4 × 10^-8 H²) = 0.0002 H.
- M = 0.9 × 0.0002 = **0.00018 H** (180 µH).
- Laiding = 0.0005 + 0.00036 = **0.00086 H** (860 µH).
- Lopposing = 0.0005 - 0.00036 = **0.00014 H** (140 µH).
- Independent stable-form reduction: (0.01 - 0.02)² + 2 × 0.1 × 0.0002 = 0.0001 + 0.00004 = 0.00014 H.

**Sources:** [Berkeley §3.2.2, equations 3.6–3.8](https://www2.eecs.berkeley.edu/Pubs/TechRpts/2007/EECS-2007-3.pdf) defines k and M; [MIT §9.7 terminal relations](https://www.mit.edu/course/6/6.013_book/www/chapter9/9.7.html) supplies the linear linkage matrix from which the two series configurations are derived.

## Boundary decisions and handoff

Magnetic energy is specifically field density/region energy, not transients' circuit stored-energy formula. No passive AC reactance/resonance, power-factor calculator, switch-mode converter inductor sizing, PCB trace resistance or RF transmission-line impedance is included. Ungapped radial toroid integration and discrete-gap magnetic-circuit modeling are separate reserved geometries, not inverse versions of one calculation. Transformer VA is winding allocation; turns ratio is ideal voltage/impedance transformation; coupled-coil inductance is finite-k linkage and wiring.

Parent integration should perform schema parsing, expression/constraint evaluation for defaults and all examples, integer/domain-rejection cases, and calculator rendering after all catalogs land. No build, lint, tests, formatter or backend smoke run was performed in this research assignment. Arithmetic derivation above is research evidence only.
