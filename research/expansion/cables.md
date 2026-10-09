# Cable and connector expansion research

Exactly six new models: two basic, two intermediate and two expert. Machine-readable definitions are in `research/expansion/cables.json`. This is research, not an implementation or an acceptance-test result. All 128 baseline IDs remain untouched.

## Scope, source quality and search boundaries

The expansion contract, numerical schema, expansion allocation and actual baseline exclusion inventory were consulted before selection. Searches covered English-language university electromagnetics/contact-mechanics texts, university-hosted contact research, and manufacturer connector guidance, without a publication-date restriction. Sources were retrieved on 2026-10-09. Search terms included “transmission line RLGC attenuation”, “twin lead Ellingson”, “coaxial maximum electric field”, “Hertz Holm contact resistance”, “Kohlrausch contact voltage temperature”, and “cantilever connector force”. These are engineering screening models, not compliance calculations or fitted guarantees for any specific cable or connector.

Read evidence:

- **S1 — university-authored textbook:** [Virginia Tech, S. W. Ellingson, Electromagnetics II §7.1](https://eng.libretexts.org/Bookshelves/Electrical_Engineering/Electro-Optics/Book%3A_Electromagnetics_II_%28Ellingson%29/07%3A_Transmission_Lines_Redux/7.01%3A_Parallel_Wire_Transmission_Line). Exact logarithmic forms for parallel-wire L′ and C′; low-loss impedance; exposed-field and real-jacket limitations. Strong evidence for the stated ideal geometry, not for arbitrary commercial twisted pairs.
- **S2 — university-authored textbook:** [NCSU, Michael Steer, Microwave and RF Design II §2.2.1–2.2.2](https://eng.libretexts.org/Bookshelves/Electrical_Engineering/Electronics/Microwave_and_RF_Design_II_-_Transmission_Lines_(Steer)/02%3A_Transmission_Lines/2.02%3A_Transmission_Line_Theory). RLGC differential cell; equations 7/8/12 give propagation, traveling wave and characteristic impedance. Strong analytic evidence. Only these equations are used; the page's conflicting wavelength definitions are not used.
- **S3 — university-authored textbook:** [Virginia Tech, Ellingson §7.4](https://phys.libretexts.org/Bookshelves/Electricity_and_Magnetism/Electromagnetics_II_(Ellingson)/07:_Transmission_Lines_Redux/7.04:_Power_Handling_Capability_of_Coaxial_Cable). Derives potential, radial field and inner-surface peak field from Laplace's equation. Strong ideal-field evidence. Its later power optimization is not another proposed calculator and is not used.
- **S4 — university lecture:** [University of Utah, ME EN 7960 Topic 7](https://www.mech.utah.edu/~me7960/lectures/Topic7-ContactStressesAndDeformations.pdf), slides 7-5/7-6. Circular Hertz contact radius, maximum pressure and sphere-on-flat limit. Strong classical mechanics evidence under elastic half-space assumptions.
- **S5 — university-hosted research paper:** [Jackson, Malucci, Angadi and Polchow, A Simplified Model of Multiscale Electrical Contact Resistance](https://www.eng.auburn.edu/~jacksr7/SimplifiedModelofMultiscaleECRJacksonMalucciAngadiPolchow.pdf), page 2 equation 3. Holm single-spot resistance `(rho1+rho2)/(4a)`. The paper explicitly says one spot is not a rough multi-asperity connector. This handoff uses that restricted result, not the paper's empirical multiscale fits.
- **S6 — university-hosted primary modeling research:** [Made, Gan and Tan, ECS Transactions 33(12), 23–34 (2010), DOI 10.1149/1.3501031](https://dr.ntu.edu.sg/server/api/core/bitstreams/f6cee9fd-a286-4c14-ac2c-4b32c161afae/content). Page 4 equation 4 describes diffusive/ballistic contact resistance; page 10 preceding equation 23 explicitly gives the Kohlrausch relation and Lorenz constant `2.45e-8 V²/K²`. This is research on Cu–Cu bonded interconnects, not a connector product qualification. Its established contact-temperature law is cited to earlier work, so it is not independent experimental validation of every connector use.
- **S7 — university teaching source:** [Wright State University, ME 499/699 Homework 1 solutions](https://cecs.wright.edu/~rsrin/Courses/Materials-Selection/ME%20499-699-07/Fall%202006/HW-1-solutions.pdf), page 3. Gives `delta=F L³/(3 E I)` and `I=b h³/12` before specializing to a square section.
- **S8 — university teaching derivation:** [MIT, David Roylance, Stresses in Beams (2000)](https://web.mit.edu/course/3/3.11/www/modules/bstress.pdf), equations 5–7, pages 4–5. Rectangular second moment and `sigma=-M y/I`; derives linear-elastic, small-rotation assumptions.
- **S9 — primary materials-manufacturer guidance:** [Materion, What Makes a Good Spring Material? (2024)](https://www.materion.com/insights/blog/what-makes-a-good-spring-material). Relates contact spring stiffness to geometry/modulus, separates yield from stiffness, and warns about stress relaxation, fatigue and electrical heating. Contextual support for connector use; not the source of a certified allowable stress.

Evidence gaps/rejected retrievals: an Orléans thesis PDF fetch aborted; an Iowa State beam formula sheet had incomplete text extraction. Neither supports a retained equation. An Arizona Hertz tutorial was read, but its inconsistent effective-modulus conventions make Utah the retained mechanics reference. A ZF switch catalogue was read for possible contact-temperature evidence; it provided product resistance ratings, not the required equation, so it was excluded. No unaccessed source is claimed as verified. Homogeneous field theory and elastic mechanics have high analytic confidence; actual oxide-bearing, rough, plated or aging contacts have lower applicability confidence and require measurements. No thermal ratings, material strength or dielectric strength defaults are manufacturer promises.

## Cross-lane and baseline novelty

| New ID | Closest baseline/neighbor | Why genuinely different |
|---|---|---|
| `cables.twin_lead_geometry` | `signals.coax_impedance`, `signals.microstrip_impedance`, `signals.stripline_impedance` | New balanced two-round-wire transverse boundary-value geometry, without a plane or shield. Uses acosh spacing, not a renamed coax logarithm or an inverse of an existing geometry. One geometry is counted once, including its L′/C′ outputs. |
| `cables.lossy_rlgc_line` | `signals.line_propagation`, `signals.load_reflection`, `ac.series_rlc_impedance` | Distributed telegrapher equations with both series and dielectric loss; predicts complex Z0 and exponential loss outside the low-loss approximation. Not ideal delay, resistive reflection, or a lumped RLC impedance. |
| `cables.radial_insulation_stress` | `signals.coax_impedance`, `timing.plate_displacement_current`; Protection | Dielectric field concentration and breakdown screening, not wave impedance, plate edge current or fault-current heating. Same cylindrical hardware can have distinct field-stress and impedance problems. Protection owns adiabatic short-circuit withstand. |
| `cables.hertz_holm_contact` | `pcb.trace_resistance_drop`, `pcb.via_barrel_resistance`; contact spring below | Force-dependent elastic contact area and spreading/constriction current paths, not uniform conductor rho-length/area. Spring force is an input; beam bending is not counted here. |
| `cables.contact_hotspot_temperature` | `thermal.junction_feedback_equilibrium`, `thermal.lumped_thermal_step`; Hertz/Holm above | Microscopic electrothermal contact law with electron thermal conduction, no lumped package thermal resistance/capacitance. Takes measured contact voltage, not a renamed resistance output or generic Joule-heating calculation. |
| `cables.cantilever_contact_spring` | `thermal.free_linear_expansion`; Motors | Elastic bending creates connector normal force from mating displacement. No thermal expansion, winding, electromechanical torque or motor model. Electrical connectivity depends on mechanical contact force, but no resistance fit is bundled as another feature. |

The selected items do not claim EMC shield transfer impedance, antenna/matching design, DC copper, contact lifetime, or thermal fault withstand. Contact-area mechanics, contact temperature and spring elasticity are independent physical design questions, not alternative inverses of one equation.

## Numerical and diagram conventions

All examples below were derived outside the Rust engine using independent Python arithmetic; the RLGC oracle used Python complex multiplication/division/square roots rather than the catalog's real expression decomposition. No engine, tests, builds, lint or formatter was run. JSON includes every input and every expected output, with `relative_tolerance=1e-6`. Exact zeros need exact or physically scaled residual assertions, not relative division by zero.

Every diagram must use existing primitives only, 700×280 canvas, 16 px margins, labels at least 14 px. The following are specifications, not rendered images. Coordinate suggestions separate geometry and quantity legend. All named bindings are exact catalog keys. Unbound symbols such as `Delta z` and `E(r)` are explanatory text, not additional calculated values. Each figure gets its own caption. Curves are static explanations, not live plots. No wire crosses a component body; crossing wires connect only where a junction is explicitly marked.

## 1. `cables.twin_lead_geometry` — intermediate

### Physical model and units

S1 gives `q=acosh(D/d)=ln(D/d+sqrt((D/d)²-1))`, `L′=mu0*q/pi`, `C′=pi*eps0*er/q`. Differential impedance is `sqrt(L′/C′)`. Here `D=center_spacing`, `d=wire_diameter`, both in m; er is dimensionless. L′ is the complete differential loop inductance, not the inductance of only one wire. C′ is between the two conductors, not each conductor to ground. `mu0=4*pi*1e-7 H/m`, `eps0=8.8541878128e-12 F/m` are explicit engineering constants.

Domain: d>0, D>d, er>=1; homogeneous nonmagnetic material, straight parallel equal round conductors, negligible conductor/internal-inductance effects and no nearby metal. Do not apply the result exactly to a partly air/partly jacket field or to arbitrary twisted pairs. The catalog avoids `x²-1` cancellation near touching wires: with `u=(D-d)/d`, evaluate `q=log1p(u+sqrt(u)*sqrt((D+d)/d))`.

### Complete worked example

Inputs: `wire_diameter=0.001 m`, `center_spacing=0.006 m`, `relative_permittivity=2.25`.

- `geometry_factor = acosh(6) = 2.477888730288475`.
- `inductance_per_length = 4e-7*q = 9.9115549211539e-7 H/m`.
- `capacitance_per_length = pi*8.8541878128e-12*2.25/q = 2.525802101358135e-11 F/m`.
- `differential_impedance = sqrt(L′/C′) = 198.09396088157135 ohm`.

Independent cases: set er=1 with unchanged geometry: C′ becomes `1.1225787117147267e-11 F/m`, Zdiff becomes `297.140941322357 ohm`, L′ and q stay unchanged. Scaling both lengths equally must leave all outputs unchanged. For `D=d*(1+u)` and small positive u, `q~sqrt(2u)`; verify positive impedance rather than rounded zero. Invalid: D=d, D<d, d=0, er<1. Do not replace the exact acosh geometry by the large-spacing `ln(2D/d)` approximation.

### Mandatory diagram

Cross-section left half: circles centered near (145,135) and (355,135), radius 32. Draw center-spacing dimension between their centers at y=64, and a diameter dimension across one circle at y=195. Put `+V/2` and `-V/2` inside respective circles; dot/cross symbols indicate equal/opposite longitudinal currents. Draw two curved E-field lines from positive to negative conductor; no ground/shield. Right legend near x=440 binds `relative_permittivity` as er, `inductance_per_length` as loop L′, `capacitance_per_length` as pair C′, and `differential_impedance` as Zdiff. Dimension bindings: `center_spacing` and `wire_diameter`. Caption: “Homogeneous balanced round-wire pair. D is center spacing; L′ is loop inductance and C′ is between wires. Field sketch is not to scale.”

## 2. `cables.lossy_rlgc_line` — expert

### Physical model and units

S2 equations 7/8/12 give `gamma²=(R′+j omega L′)(G′+j omega C′)` and `Z0²=(R′+j omega L′)/(G′+j omega C′)`. Use `exp(+j omega t)` phasors, `gamma=alpha+j beta`, and forward factor `exp(-gamma z)`. R′ [ohm/m] is loop series resistance; L′ [H/m], G′ [S/m], C′ [F/m]; omega=2*pi*f. Alpha [Np/m] and beta [rad/m] both belong to the same selected frequency. Voltage ratio is `exp(-alpha*length)` and loss is `20*alpha*length/ln(10)` dB.

Domain: f,L′,C′>0; R′,G′,length>=0. The line must be uniform and described by one valid distributed mode. R′ and G′ need not be small. RLGC must come from appropriate characterization at f. This is not loaded insertion loss; no source/load mismatch, backward wave, time-domain causality or pulse simulation is silently supplied.

For stable real-expression evaluation, let `s=|R′+j omega L′|`, `t=|G′+j omega C′|`, `x+j y=[(R′+j omega L′)/s]*[(G′+j omega C′)/t]`. The two normalized propagation-square outputs are x and y. With `h=hypot(x,y)` and `q=y/sqrt(2*(h+abs(x)))`, the real/imaginary roots are `hypot(sqrt(max(x,0)),q)` and `hypot(sqrt(max(-x,0)),q)`. Multiply each by `sqrt(s)*sqrt(t)`. This algebra preserves both lossless alpha=0 and positive near-DC beta without subtracting close square roots or evaluating `cos(pi/2-small)`.

For Z0, the normalized quotient real part is `c=(R′/s)(G′/t)+(omega L′/s)(omega C′/t)>=0`; its imaginary part is `d=(omega L′/s)(G′/t)-(R′/s)(omega C′/t)`. Then `Re Z0=sqrt(s)/sqrt(t)*sqrt((1+c)/2)` and `Im Z0=sqrt(s)/sqrt(t)*d/[2 sqrt((1+c)/2)]`. This avoids subtracting nearly equal atan angles. Exact/near Heaviside balance can still have ordinary subtraction residuals; evaluate zero against a characteristic-impedance-scaled floating-point bound, not against a made-up physical offset.

### Complete worked example

Inputs: R′=2 ohm/m, L′=2.5e-7 H/m, G′=2e-5 S/m, C′=1e-10 F/m, f=1e6 Hz, length=10 m.

Use `z=2+j*1.5707963267948966` and `y=0.00002+j*0.0006283185307179586`. Independently compute `sqrt(z*y)` and `sqrt(z/y)`.

| Exact output key | Expected value |
|---|---:|
| `angular_frequency` | 6283185.307179586 rad/s |
| `series_impedance_magnitude` | 2.5431085506270352 ohm/m |
| `shunt_admittance_magnitude` | 0.0006286367600161275 S/m |
| `normalized_propagation_square_real` | -0.5923346865096576 |
| `normalized_propagation_square_imaginary` | 0.8056920125938358 |
| `attenuation_constant` | 0.018051746169367573 Np/m |
| `phase_constant` | 0.03567668678383984 rad/m |
| `impedance_real` | 57.63732387347136 ohm |
| `impedance_imaginary` | -26.89559334274006 ohm |
| `forward_amplitude_ratio` | 0.8348381028824459 |
| `attenuation_db` | 1.5679547500148998 dB |

Independent cases:

- Set R′=G′=0: s=`1.5707963267948966`, t=`0.0006283185307179586`, x=-1, y=0, alpha=0, beta=`0.03141592653589793`, Z0=`50+j0 ohm`, ratio=1 and loss=0. Same f and length as nominal.
- Set only length=0: all local line outputs stay nominal; ratio=1 and loss=0.
- Set G′=0.0008 S/m with other nominal inputs: R′/L′=G′/C′ (Heaviside balance); `gamma=0.04+j*0.03141592653589793`, `Z0=50+j0`. Ratio is `exp(-0.4)`, loss is `8/ln(10)` dB. Test zero imaginary part with a stated ULP-scaled residual.
- Keep nominal RLGC but f=`1e-12 Hz`: `alpha≈0.006324555320336759`, `beta≈1.0182952972441003e-19`, `Zreal≈316.2277660168379`, `Zimag≈-4.8431117795755995e-15`. Beta must not vanish because a phase angle rounded to pi/2.
- Invalid: negative R′ or G′, L′=0, C′=0, f=0, negative length. Tiny loss must not be excluded by an arbitrary lower loss bound.

### Mandatory diagram

Draw one infinitesimal RLGC cell and continuation marks: upper conductor enters x=55,y=105; series resistor at x≈165 then series inductor at x≈295; output node at x≈405. Return conductor y=200. Two distinct shunt branches from the same upper node to return: G′*Delta z resistor at x=460 and C′*Delta z capacitor at x=565, each with explicit endpoint junctions. A bracket identifies `Delta z` as an illustrative differential section, not `length`. Bind resistor label to `series_resistance_per_length` with text “R′; element R′ Δz”; similarly `inductance_per_length`, `shunt_conductance_per_length`, `capacitance_per_length`. Above the line show forward +z arrow and total extent `length`. Small bottom legend binds `attenuation_constant`, `phase_constant`, `forward_amplitude_ratio`; label the latter `|V+(l)/V+(0)|`. Caption: “Infinitesimal distributed cell, repeated over length l. Forward wave decays as exp(-alpha z); no source/load mismatch is included.”

## 3. `cables.radial_insulation_stress` — intermediate

### Physical model and units

S3 derives `V(r)=Vpk*ln(b/r)/ln(b/a)` and outward radial `E(r)=Vpk/[r ln(b/a)]`. Maximum occurs at r=a, not at the midpoint of the dielectric. Here a=`conductor_radius`, b=`shield_inner_radius`, m; Vpk=`peak_voltage`, V. Field is V/m. `voltage_limit=allowable_field*a*ln(b/a)` and `field_margin=allowable_field-E(a)`.

Domain a>0, b>a, Vpk>=0, allowable_field>0. Use a homogeneous long concentric cable without end effects, voids, eccentricity, space charge or contamination. The field limit must come from applicable material/application data; no universal breakdown field is inferred. This electrostatic screening can return a negative margin. It does not model the post-breakdown conducting state. `log1p((b-a)/a)` avoids thin-gap logarithm cancellation.

### Complete worked example

Inputs: a=0.001 m, b=0.004 m, Vpk=2000 V, allowable_field=8e6 V/m. `ln(b/a)=ln(4)=1.3862943611198906`.

- `inner_surface_field = 2000/(0.001*ln(4)) = 1442695.0408889633 V/m`.
- `outer_surface_field = 2000/(0.004*ln(4)) = 360673.7602222408 V/m`.
- `voltage_limit = 8e6*0.001*ln(4) = 11090.354888959124 V`.
- `field_margin = 8e6-1442695.0408889633 = 6557304.959111037 V/m`.

Independent cases: Vpk=0 gives both fields=0, unchanged voltage_limit, margin=8e6. Vpk=12000 gives Einner=`8656170.24533378 V/m` and negative margin about `-656170.24533378 V/m`, which must remain an accepted screening result. At fixed b/a, doubling a and b halves fields and doubles voltage limit. For b-a much smaller than a, Einner approaches Vpk/(b-a), recovering the planar local limit. Invalid b<=a, a<=0 or nonpositive field limit. AC RMS voltage must be converted to peak externally; silently treating RMS as peak understates stress.

### Mandatory diagram

Concentric cross-section centered near (190,140): filled inner disk radius 35, outer conductor ring inner radius 90. Clearly distinguish b as the shield **inner** surface. Two radius arrows in different angular directions bind `conductor_radius` as a and `shield_inner_radius` as b. Inner conductor marked +Vpk and shield 0; bind `peak_voltage`. Draw three radial outward E arrows in insulation. Right side x≈370 shows a qualitative decreasing 1/r curve from a to b, with endpoint labels bound to `inner_surface_field` and `outer_surface_field`; axis ticks a/b are explanatory. Separate legend binds `allowable_field` and `voltage_limit`. Caption: “Ideal concentric insulation: peak stress occurs at the inner conductor. The 1/r curve is illustrative; voltage is peak, not RMS. Voids and ends are excluded.”

## 4. `cables.hertz_holm_contact` — expert

### Physical model and units

S4 gives two-body Hertz contact; take one curvature radius to infinity and identical material properties to obtain `E*=E/[2(1-nu²)]`. A spherical connector tip of curvature R under compressive force F produces `a=[3 F R/(4 E*)]^(1/3)` and center pressure `p0=3F/(2*pi*a²)`. S5 equation 3 then gives `Rc=rho/(2a)` for identical clean metals. Relative elastic approach is `a²/R`.

Inputs F [N], R [m], E [Pa], nu dimensionless, rho [ohm m], electron_mean_free_path [m], elastic_pressure_limit [Pa]. All except nu are positive; -1<nu<0.5. The factor `(1-nu)*(1+nu)` avoids endpoint cancellation in `1-nu²`. Elastic-pressure validity is checked before outputs using the input-only expanded expression.

Model screens: a/R<=0.1 and a/mean_free_path>=10. These are explicitly conservative engineering separation-of-scale screens, not sourced exact 10% accuracy guarantees. S4 requires contact small relative to bodies; S6 shows why finite mean free path adds ballistic resistance. A qualified p0 limit is required; uniaxial yield strength cannot be silently substituted. No oxide, plating, rough multi-spot interaction, bulk path or self-heating is included. This is a clean-contact lower-complexity estimate, not total measured connector resistance.

### Complete worked example

Inputs: F=0.01 N, R=0.002 m, E=110e9 Pa, nu=0.34, rho=1.7e-8 ohm m, mean_free_path=4e-8 m, elastic_pressure_limit=3e8 Pa.

- `effective_modulus = 110e9/[2*(1-0.34²)] = 62189054726.368164 Pa`.
- `contact_radius = [3*0.01*0.002/(4*effective_modulus)]^(1/3) = 6.22480523898329e-6 m`.
- `maximum_pressure = 3*0.01/(2*pi*contact_radius²) = 123222453.18016359 Pa`.
- `constriction_resistance = 1.7e-8/(2*contact_radius) = 0.0013655045698085684 ohm`.
- `indentation = contact_radius²/0.002 = 1.9374100131636908e-8 m`.

The example has a/R≈0.0031124, a/mean_free_path≈155.62, p0<300 MPa, so all physical screens pass.

Independent cases: F=0.08 N scales a by 2, p0 by 2, Rc by 1/2 and approach by 4, retaining the nominal material/geometry. Expected a=`1.244961047796658e-5 m`, p0=`246444906.36032718 Pa`, Rc=`0.0006827522849042842 ohm`, indentation=`7.749640052654763e-8 m`; E* unchanged. F approaches zero only until the diffusive lower-size screen fails; do not imply a zero-force closed metallic connection. Invalid F=0, nu=-1 or 0.5, pressure limit=1e8 Pa with nominal mechanics, mean_free_path=1e-6 m with nominal mechanics, or geometry causing a/R>0.1. Equality on the defined screening boundaries is permitted.

### Mandatory diagram

Draw a rounded upper contact resting on a thick lower flat, interface around y=145 and x=180. Force arrow points downward into the upper sphere, labeled `normal_force`. Curvature arrow binds `sphere_radius`, explicitly “R, tip curvature”. Contact patch shown by a short interface segment with dimension `2a`; bind `contact_radius` with text “a; spot width 2a”, never label the full diameter as a. Current streamlines narrow through the patch and spread into the lower electrode. Right inset: equivalent single constriction resistor labeled `constriction_resistance`, separate from any unmodeled leads. Lower legend binds `maximum_pressure`, `effective_modulus`, `indentation`. Mark both bodies “same metal, E / nu / rho” with bindings `young_modulus`, `poisson_ratio`, `resistivity` if space permits. Caption: “One clean elastic spherical contact on a flat. Current constricts through radius a; oxide, rough multi-spot and bulk lead resistance are excluded.”

## 5. `cables.contact_hotspot_temperature` — basic

### Physical model and units

S6 page 10 gives `Tm=sqrt(Tbulk²+Vc²/(4*Lorenz))`, with the quoted ideal-metal Lorenz number `2.45e-8 V²/K²`. This follows from steady coupled electric/thermal conduction with a constant Wiedemann–Franz ratio. Temperature units are kelvin, never Celsius. Vc is the contact-only voltage-drop magnitude, not supply voltage.

Domain: Tbulk>0, Lorenz>0, Vc>=0, Tlimit>=Tbulk. The calculated hotspot must not exceed the supplied material/model validity limit; this bound should precede softening, melting or geometry change. Both remote electrodes have equal temperature, identical metal and no significant alternative heat-loss path at the constriction. Oxide-film heating, thermoelectric asymmetry and arcs violate the model. The relation does not supply connector current rating or housing temperature.

Use `hypot(Tbulk,Vc/(2*sqrt(Lorenz)))`. With `u=Vc/(2*sqrt(Lorenz))`, evaluate rise as `u*(u/(Tm+Tbulk))`, not subtraction of nearly equal temperatures. Contact voltage at the limit is `2*sqrt(Lorenz)*sqrt(Tlimit-Tbulk)*sqrt(Tlimit+Tbulk)`.

### Complete worked example

Inputs: contact_voltage=0.05 V, bulk_temperature=300 K, lorenz_number=2.45e-8 V²/K², temperature_limit=600 K.

- `hotspot_temperature = sqrt(90000+0.0025/(9.8e-8)) = 339.8679215248663 K`.
- `temperature_rise = 39.86792152486629 K`.
- `contact_voltage_limit = 2*sqrt(2.45e-8)*sqrt(600²-300²) = 0.16266530054071152 V`.
- `temperature_margin = 600-339.8679215248663 = 260.1320784751337 K`.

Independent cases: Vc=0 gives Tm=300 K, rise=0, unchanged voltage limit, margin=300 K. Tlimit=Tbulk with Vc=0 is valid and gives limit voltage=0/margin=0. Vc=1e-6 V gives rise about `1.7006802720606382e-8 K`; it must remain positive. Invalid: Tbulk<=0, Lorenz<=0, Tlimit<Tbulk or Vc=0.2 V at nominal temperatures, which exceeds the 600 K validity bound. Do not clamp the over-limit temperature into a plausible result.

### Mandatory diagram

Two metal blocks at x≈75–290 and x≈365–580 connected by a narrow neck at x≈325,y≈130. Current arrow runs left to right. Kelvin voltage taps bracket only the neck; + left and - right, label binding `contact_voltage`. Remote ends both bind `bulk_temperature` and carry “Tbulk”. Hotspot circle at the neck binds `hotspot_temperature`. Heat arrows point away from the hotspot into **both** metal blocks. Lower legend binds `lorenz_number`, `temperature_rise`, `temperature_limit`; use no lumped ambient thermal resistor. Caption: “Steady microscopic contact heating with equal remote metal temperatures. Vc is the constriction drop only; Wiedemann–Franz conduction sets the hotspot, not housing thermal resistance.”

## 6. `cables.cantilever_contact_spring` — basic

### Physical model and units

S7 gives `delta=F L³/(3 E I)` and `I=b t³/12`, hence `k=E b t³/(4 L³)` and `F=k delta`. From S8 with root moment F L and outer fiber t/2, `sigma_root=6 F L/(b t²)=1.5 E (t/L)(delta/L)`. S9 connects these laws to contact normal force and explains why increased yield strength does not by itself increase elastic stiffness.

Inputs L,b,t,delta in m; E and allowable_stress in Pa. F is N; k is N/m. L,b,t,E,allowable_stress>0; delta>=0. Model screens require t<=L/10, delta<=L/10 and calculated stress<=allowable_stress. These 10:1/0.1 screens explicitly operationalize slender-beam/small-rotation assumptions and do not claim universal error limits. Material remains elastic. Width is perpendicular to bending; thickness is in bending direction. A straight, initially unstrained clamped-free beam under a tip normal load is required. Actual contact roots may have stress concentrations, and actual life depends on relaxation/fatigue not included here.

### Complete worked example

Inputs: L=0.01 m, b=0.002 m, t=0.0002 m, E=130e9 Pa, delta=0.0005 m, allowable_stress=600e6 Pa.

- `spring_rate = 130e9*0.002/4*(0.0002/0.01)³ = 520 N/m`.
- `normal_force = 520*0.0005 = 0.26 N`.
- `root_stress = 1.5*130e9*(0.0002/0.01)*(0.0005/0.01) = 195000000 Pa`.
- `stress_margin = 600000000-195000000 = 405000000 Pa`.

Independent cases: delta=0 returns the same spring_rate, normal_force=0, root_stress=0 and stress_margin=600e6 Pa. delta=0.001 m reaches the small-deflection screen exactly: F=0.52 N, sigma=390e6 Pa and margin=210e6 Pa; accepted. Doubling b doubles k and F but leaves stress unchanged at imposed deflection. Invalid: L=0, t>L/10, delta>L/10, or nominal geometry with allowable_stress=100e6 Pa. Test the slenderness boundary separately using delta=0 so an unrelated stress constraint does not hide the geometry result.

### Mandatory diagram

Side view x≈95–405: clamped wall at x=95; undeformed beam horizontal at y=100; a distinct smooth deflected centerline ends near (405,150). Tip mating force arrow downward binds `normal_force`. Vertical tip dimension between original and displaced tips binds `tip_deflection`. Horizontal free-span dimension binds `beam_length`; mark maximum bending stress at the clamped root with `root_stress`. Right inset around x=525,y=100 shows rectangular cross-section with horizontal `beam_width` and vertical `beam_thickness`, explicitly noting bending is vertical. Lower legend binds `young_modulus`, `spring_rate` and `allowable_stress`. Caption: “Straight rectangular connector leaf under imposed tip travel. The deflected shape is illustrative; normal force is not total mating friction force. Elastic small-deflection model only.”

## Implementation and independent-test handoff

Implement all six JSON objects with existing closed-form engine functions; no new expression primitive is required. Export six entries in the eventual `cablesDiagrams` map using the specification above. Keep mathematical helper-output units and exact quantity names intact. Parent owns registry/catalog integration. There are no calls, fixtures, exports, configuration or production changes in this research-only deliverable.

Independent testers should transcribe complete example vectors rather than snapshot implementation defaults, assert every output, then exercise stated zero/limiting/invalid cases and malformed/nonfinite input recovery through the real Rust protocol. For vanishing quantities, compare to a dimensionally meaningful independent oracle and use documented ULP allowances only for mathematical-zero cancellation. Binary64 extremes still require parent numerical review; finite input alone does not guarantee every intermediate/output is representable. No arbitrary practical range was introduced to hide that limitation.

Source equations, physical exclusions and six independent worked examples are complete. No blocked source is needed by a retained model. Remaining acceptance work belongs to implementation/test/review: real Rust evaluation, native diagram rendering, quantity bindings, readable layout, model-domain error behavior and catalog-wide novelty approval. No production, UI, hardware or cross-platform acceptance is claimed here.
