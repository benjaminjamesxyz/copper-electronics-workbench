# Cable and connector calculators

Rust evaluates [the catalog](../../backend/catalog/cables.json); Flutter uses `cablesDiagrams` for static illustrations. No numerical implementation lives in Flutter. Existing calculators remain unchanged.

Source evidence, retrieval limitations and independent example derivations come from [the research handoff](../../research/expansion/cables.md). Source codes S1–S9 below refer to its linked evidence inventory. These are engineering screening models, not product ratings or certification. Defaults and complete expected vectors are retained in the catalog. No build, test, formatter, native rendering or runtime validation was run during this implementation. Parent acceptance must exercise all examples, limiting cases, errors and diagram pixels.

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
