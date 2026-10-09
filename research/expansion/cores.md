# Cores expansion research: six winding and magnetic models

## Scope and handoff

Exactly six new IDs; two basic, two intermediate, two expert. Only `research/expansion/cores.json` and this document are owned deliverables. Production catalogs, diagrams, tests, registry and board remain unchanged. The existing 128-model baseline and expansion allocation were read before selection. Every example below was derived independently with Python arithmetic, with Decimal used for the tiny-loss example; no Rust engine, builds, tests, lint, formatters or desktop checks were run.

| ID | Level | Distinct design problem |
|---|---|---|
| `cores.single_layer_air_solenoid` | basic | Finite open-air cylindrical winding geometry |
| `cores.square_planar_spiral` | intermediate | Planar square current-sheet inductance |
| `cores.bobbin_layer_fit` | basic | Discrete insulated-wire turns, layers and radial clearance |
| `cores.sinusoidal_steinmetz_loss` | intermediate | Calibrated dissipative material power law |
| `cores.lamination_eddy_loss` | expert | Classical induced-current heating in isolated thin steel sheets |
| `cores.thin_foil_winding_loss` | expert | Series-layer copper proximity/skin loss, controlled low-frequency expansion |

No item is an inverse, renamed formula, extra output or unit variant of an old calculation. The two geometry estimates differ physically: a finite cylindrical current sheet versus four coupled planar current sheets. The two core-loss models are alternatives with different inputs and meanings, not additive components: measured Steinmetz total loss already includes eddy loss. The foil model concerns copper transport-current windings, not magnetic laminations or a free-standing skin-depth calculation.

## Search and evidence boundaries

Search date: 2026-10-09. English-language public manufacturer and university material; no publication-date restriction. Queries covered Wheeler single-layer geometry, Mohan spiral inductance, TI bobbin utilization, Steinmetz calibration, MIT complete-penetration laminations and Dowell foil losses. Excluded toroids, gapped reluctance, turns ratio, flux-from-voltage, mutual coupling, skin-depth-only calculators, nonsinusoidal core models requiring unsupported integration, and unvalidated generic material presets.

Evidence actually read:

- University of Georgia, Haidekker, *Inductance Meter for Small Inductances*, Motivation, 2023: university engineering report, empirical Wheeler formula and sensitivity to actual geometry. https://photonics.engr.uga.edu/inductor_meter/index.html
- Jim Lux, *Wheeler Formulas for Inductance*: secondary technical transcription used only to establish inch/microhenry units. https://www.pe2bz.philpem.me.uk/Power/-%20-%20Power-Design%20-%20-/HighVoltage/wheeler.htm
- Mohan, Hershenson, Boyd and Lee, IEEE JSSC 1999, equation (2), Table II and limits in section II-B: primary research with field-solver and measurement comparison. PDF equation/table images were rendered and read because text extraction dropped equations. https://web.stanford.edu/~boyd/papers/pdf/inductance_expressions.pdf
- TI SLUP126, *Power Transformer Design*, pages 4-4 and 4-11: primary manufacturer engineering guidance on net winding window, finished wire diameter, packing and integral turns. The discrete rectangular-packing equations here are an explicit geometric derivation from those construction definitions, not a claimed verbatim TI equation. https://www.ti.com/lit/ml/slup126/slup126.pdf
- Sullivan, Harris and Herbert, *Core Loss Predictions for General PWM Waveforms from a Simplified Set of Measured Data*, IEEE 2010, section II: primary experimental research discussing sinusoidal Steinmetz calibration, frequency-dependent fitted coefficients and DC-bias limitations. Only its sinusoidal equation is used here. https://www.wcmagnetics.com/wp-content/uploads/2015/03/Core-Loss-Prediction-Square-Wave-Sullivan.pdf
- MIT 6.685, Kirtley, *Class Notes 3: Eddy Currents, Surface Impedances and Loss Mechanisms*, sections 3.4 and 4: university analytic derivation for laminations and normalized empirical core-loss fits. https://ocw.mit.edu/courses/6-685-electric-machines-fall-2013/73ce73e3a02b085d00df37882e7636fe_MIT6_685F13_chapter3.pdf
- Dowell, *Effects of Eddy Currents in Transformer Windings*, Proc. IEE 113(8), 1966, pp. 1387–1394, section 4 and appendix equations (8), (32), (33): primary analytic work, original scanned pages rendered and read. https://www.cpdee.ufmg.br/~troliveira/docs/aulas/fontes/05247417.pdf
- TI SLUP125, *Transformer and Inductor Design*, pp. 3-3–3-6: manufacturer treatment of layer-dependent eddy currents, equal series current, winding portions and interleaving. Scanned pages rendered and read. https://www.ti.com/lit/ml/slup125/slup125.pdf
- Nan and Sullivan, PESC 2003, *An Improved Calculation of Proximity-Effect Loss in High-Frequency Windings of Round Conductors*, sections II and V: primary FEM investigation, read as a warning against applying foil equations blindly to round wire; not used for a round-wire conversion. https://www.wcmagnetics.com/wp-content/uploads/2015/02/proximityeffect.pdf

Evidence gaps and rejected evidence: Wheeler's original 1928 Berkeley scan returned a browser challenge; it was not read and is not claimed verified. The university Wheeler equation is accessible, with units corroborated separately. A FIRGELLI calculator page was read and excluded: its stated millimetre equation lacks the inch-to-millimetre factor, and arbitrary permeability multiplication is unsuitable for open ferrite geometry. TI SLUP124's main pages were not text-extractable and are not relied upon. No named material calibration, insulation certification, real coil measurement or application-specific FEM validation is supplied. Model equations have high analytic confidence within stated idealizations; empirical accuracy outside source populations remains unproven.

All examples use SI except dimensionless turns/layers. The JSON uses only existing expression-language operations. Positive geometric lengths are mandatory. Integer turns/layers are guarded. Inputs describe engineering magnitudes, not signed current directions. The diagrams establish illustrative current/field directions without changing magnitude-only outputs.

## Diagram contract for all six

Implement exactly one map entry per ID in `coresDiagrams`, using only existing `primitives.dart`, on a 700×280 canvas. Maintain 16-pixel margins and labels of at least 14 pixels. Coordinates below are layout zones, not a request for a new renderer. Bind only the exact quantity keys listed below; any unbound current, flux or coordinate arrow is explicitly conceptual. Geometry is static and not to scale. No plot is a computed result. No source or lead wire crosses a component body; schematic crossings require explicit dots only when joined.

## 1. `cores.single_layer_air_solenoid`

**Purpose and novelty.** Select a finite cylindrical air winding before winding hardware. Closest old item is `magnetics.rectangular_toroid_inductance`: that integrates a closed magnetic path through a toroidal material section; Wheeler handles open-space finite-length field spreading. Neither old core-reluctance formula supplies this end correction.

**Equation and units.** UGA gives `L_uH=r_in² N²/(9r_in+10l_in)`; Lux explicitly states inches and microhenries. Substituting `r_in=r/0.0254`, `l_in=l/0.0254` gives `L_H=(1e-6/0.0254)r²N²/(9r+10l)`. Evaluation divides the denominator by `r`, avoiding a needless radius square. `pitch=l/N`. Radius is the conductor-centre radius; length is the axial current-sheet span, defined as `N*pitch`, not necessarily the distance between first/last wire centres. Inductance is positive and independent of the illustrative current direction.

**Validity.** Single, uniform, closely spaced layer on air/nonmagnetic former; isolated from ground planes, metal shields and magnetic bodies; quasistatic/DC inductance well below self resonance. The app admits `l>=r`, `wire_diameter<=pitch<=r/5`. These are conservative geometry screens chosen here for a moderately elongated current-sheet coil, not a published universal 1% claim. Wire insulation controls fit; resistance, current rating and winding capacitance are excluded.

### Complete independent numerical example

Inputs:

```json
{
  "radius": 0.01,
  "length": 0.03,
  "turns": 20,
  "wire_diameter": 0.001
}
```

Expected outputs:

```json
{
  "inductance": 4.037956793862305e-06,
  "turn_pitch": 0.0015
}
```

Substitution: `9r+10l=0.39 m`; `r²*N²=0.04 m²`; `L=(1e-6/0.0254)*0.04/0.39=4.037956793862305e-6 H`; `pitch=0.03/20=0.0015 m`. All two output quantities are accounted for.

### Independent test scenarios

- Nominal: complete vector above.
- Geometric scaling: multiply radius, length and wire diameter by two, keeping turns fixed. Inductance doubles to `8.07591358772461e-6 H`; pitch becomes `0.003 m`.
- Inclusive fit limits: radius `0.01`, length `0.01`, turns `10`, wire diameter `0.001` gives pitch `0.001 m` and `L=2.07210940737671e-6 H`; all other inputs explicitly as stated.
- Invalid: zero radius/length/diameter; turns 1.5; length below radius; finished diameter above pitch; pitch above radius/5. A later valid request must recover.

### Mandatory physical-geometry diagram

Left zone x=50–440, y=70–190: side view of a cylindrical nonmagnetic former with evenly spaced visible turns. A small end-view circle at x=570,y=125 shows the **mean conductor-centre** radius, not the bore radius. Axial dimension arrow below winding binds `length`; radial arrow binds `radius`; one enlarged turn-spacing bracket binds `turn_pitch`; wire cross-section callout binds `wire_diameter`. Place `turns` above the coil and `inductance` in the lower right. Draw a conceptual input-current arrow on the lead and an axial field arrow consistent with winding handedness; neither is a calculated quantity. Caption: “Single-layer, nonmagnetic finite coil; current-sheet length is N times pitch. Lead inductance and self capacitance are omitted; geometry is not to scale.”

## 2. `cores.square_planar_spiral`

**Purpose and novelty.** Lay out a square spiral using lithographic/PCB dimensions. Closest old item is toroidal inductance, and the closest new neighbor is the cylindrical solenoid above. Neither contains the four-sheet mutual-field geometry, radial fill correction or central opening of a planar spiral. The model estimates inductance only, not generic resonance.

**Equation and units.** Mohan equation (2), Table II square coefficients are `(c1,c2,c3,c4)=(1.27,2.07,0.18,0.13)`. Define `a=N*w+(N-1)*s`, `di=do-2a`, `davg=(do+di)/2`, `fill=a/davg`. Then `L=mu0*N²*davg*c1/2*[ln(c2/fill)+c3*fill+c4*fill²]`, with `mu0=4*pi*1e-7 H/m`. Square diameters mean side dimensions, not diagonal lengths. All supplied lengths are metres; `L` is henries.

**Validity.** Thin, isolated, uniform square spiral, nonmagnetic surroundings, omitted underpass and terminal routing. Require integer `N>=1`, positive opening and `s<=3w`. The source's <=8% statement applies to its examined geometries under that spacing limit, not every macroscopic PCB layout. Its main simulated database used 100–480 micrometre outside diameters and 0.5–100 nH; the worked geometry is within that scale. Quasistatic geometric scaling does not establish high-frequency lumped validity. A nearby ground plane, conductive substrate loss, large copper thickness, or resonance needs EM characterization.

### Complete independent numerical example

Inputs:

```json
{
  "outer_side": 0.0003,
  "turns": 4,
  "trace_width": 1e-05,
  "spacing": 1e-05
}
```

Expected outputs:

```json
{
  "inner_side": 0.00015999999999999996,
  "average_side": 0.00022999999999999995,
  "fill_ratio": 0.3043478260869566,
  "inductance": 5.825908169108072e-09
}
```

Substitution: `a=4*10e-6+3*10e-6=70e-6 m`; `di=300e-6-140e-6=160e-6 m`; `davg=230e-6 m`; `fill=70/230=0.3043478260869565`. The bracket is `ln(2.07/(7/23))+0.18*(7/23)+0.13*(7/23)^2`. Multiplying by `4*pi*1e-7*16*230e-6*1.27/2` gives `5.825908169108072e-9 H`. Tiny last-digit representation differences in inner/average sides are floating-point representation, not model tolerance.

### Independent test scenarios

- Nominal: complete vector above.
- Scale every length by two: inner side `0.00032 m`, average side `0.00046 m`, unchanged fill, inductance `1.1651816338216144e-8 H`.
- Limiting construction: `turns=1` remains allowed when the positive opening fits; no turn gap contributes to the inner-side formula. Approach the central-opening limit from above and ensure finite positive inductance.
- Invalid: noninteger turns; outside side equal to or below `2[Nw+(N-1)s]`; zero width or spacing; spacing above three widths. At `spacing=3*trace_width`, the spacing screen is inclusive if opening remains positive.

### Mandatory physical-geometry diagram

Draw one continuous four-turn square spiral in x=70–330, y=35–225, with visible central opening. Outer lead exits left; inner lead exits via an explicitly dashed, electrically separate underpass representation labeled “underpass omitted from L model,” never a false same-layer crossing. Top dimension binds `outer_side`; central-opening bracket binds `inner_side`. Right-side enlarged adjacent trace pair shows `trace_width` across copper and `spacing` across dielectric. Lower right labels bind `turns`, `fill_ratio`, and `inductance`; `average_side` may remain in the accessible legend. A conceptual current arrow follows the spiral clockwise. Caption: “Square current-sheet estimate; thin isolated spiral, no ground plane. Underpass, loss and self resonance are excluded; drawing is not to scale.”

## 3. `cores.bobbin_layer_fit`

**Purpose and novelty.** Determine whether a specified winding physically fits. Closest old item, transformer VA budget, sizes apparent-power capacity but cannot predict winding construction. This model adds integer packing, insulation, partial-layer rounding and radial clearance. It is not an electrical resistance or turns-ratio calculation.

**Equation and units.** From TI's net-width/finished-diameter construction definition, let `Wu=W-2m`, `q=floor(Wu/do)`, `n=ceil(N/q)`. The radial build is `h=n*do+(n-1)*t`, remaining height `H-h`, and copper fill `N*pi*dc²/(4*Wu*H)`. Fill uses the entire available net rectangular window, not only the occupied winding rectangle. Turns and layers are exact integers; all lengths are metres. Positive margin means unused radial space.

**Validity.** Square-packed circular insulated wire, no wire nesting/compression, one uniform gauge, layer tape only between occupied layers. Width/height are already net of bobbin walls and base insulation. Both end margins are reserved separately. Partial last layers consume full radial height. Reject zero available turns, copper diameter exceeding finished diameter, noninteger requested turns or excessive build. Maximum finished-wire dimensions and minimum manufactured bobbin clearances belong in a real design. Crossovers, lead exits and electrical safety margins are not automatically designed.

### Complete independent numerical example

Inputs:

```json
{
  "window_width": 0.02,
  "window_height": 0.004,
  "end_margin": 0.001,
  "wire_diameter": 0.0006,
  "copper_diameter": 0.0005,
  "tape_thickness": 0.0001,
  "turns": 100
}
```

Expected outputs:

```json
{
  "turns_per_layer": 30,
  "layers_required": 4,
  "radial_build": 0.0026999999999999997,
  "height_margin": 0.0013000000000000004,
  "copper_fill": 0.2727076956241139
}
```

Substitution: `Wu=0.020-2*0.001=0.018 m`; `q=floor(0.018/0.0006)=30`; `n=ceil(100/30)=4`. Then `h=4*0.0006+3*0.0001=0.0027 m`, clearance `0.004-0.0027=0.0013 m`. Bare copper area is `100*pi*(0.0005)^2/4=1.963495408493621e-5 m²`; usable window is `0.018*0.004=7.2e-5 m²`; fill is `0.2727076956241139`.

### Independent test scenarios

- Nominal: complete vector above.
- Discrete boundary with the other nominal inputs: `turns=90` gives three layers and build `0.002 m`; `turns=91` jumps to four layers and build `0.0027 m`. No interpolation across the jump.
- With `turns=30`, one layer has build exactly `0.0006 m` even when tape thickness is nonzero: no tape lies before the first or after the last layer.
- Exact radial fit is accepted (`window_height=0.0027`, otherwise nominal), with zero clearance. Zero end margin or zero interlayer tape is geometrically allowed; this does not certify electrical safety.
- Invalid: only `window_width=0.001` with nominal margins, zero diameter, copper diameter above finished diameter, fractional turns, or `window_height=0.0026` with the nominal 100 turns.

### Mandatory physical-geometry diagram

Draw the bobbin window cross-section in x=70–430,y=55–220: flange margins shaded left/right, four square-packed rows of circular insulated-wire cross sections, final row visibly partial. Copper cores and insulation rings must be visually distinct. Thin tape bands occur only between occupied rows. Bind full axial bracket to `window_width`, two identical margin brackets each to `end_margin`, vertical available bracket to `window_height`, occupied bracket to `radial_build`, and remaining clear strip to `height_margin`. Right inset binds outer circle diameter `wire_diameter` and inner copper diameter `copper_diameter`; tape callout binds `tape_thickness`. `turns_per_layer`, `layers_required` and `copper_fill` get short right-hand labels or accessible legend entries. Caption: “Ordered square packing; incomplete last layer still consumes full height. Dimensions are net of bobbin/base insulation; lead and crossover space must be reserved separately.”

## 4. `cores.sinusoidal_steinmetz_loss`

**Purpose and novelty.** Predict dissipated core heat using a measured material calibration. Closest old items are sinusoidal flux amplitude and stored magnetic energy density: neither models hysteretic/eddy dissipation. Closest new neighbor is lamination eddy loss, which predicts one physical mechanism from sheet dimensions; this model is an empirical total-loss fit and must not be added to that result.

**Equation and units.** Sullivan section II gives `pv=k*f^alpha*Bpk^beta`. MIT section 4 uses the normalized power fit. Eliminate the unit-dependent `k` using a reference: `pv=pref*(f/fref)^alpha*(Bpk/Bref)^beta`; `P=pv*Ve`; `Ecycle=P/f`. `pref` is W/m³, frequency is Hz, flux is peak tesla (not RMS or peak-to-peak), volume is m³. All reported losses are positive magnitudes.

**Validity.** Same material, waveform, zero DC bias and fixed temperature as the calibration; sinusoidal flux only. Require both reference and operating points inside the supplied frequency and flux rectangles. These bounds must come from measured/fitted data and remain below saturation. The app cannot authenticate them. No universal alpha/beta presets or material-specific precision claim is made. Positive fitted exponents describe the increasing-loss regime. Defaults are a clearly hypothetical calibration, not invented data for a named material. PWM, DC bias, minor loops and temperature extrapolation need a different calibrated model.

### Complete independent numerical example

Inputs:

```json
{
  "frequency": 150000,
  "flux_peak": 0.08,
  "reference_frequency": 100000,
  "reference_flux_peak": 0.1,
  "reference_loss_density": 200000,
  "frequency_exponent": 1.5,
  "flux_exponent": 2.5,
  "core_volume": 1e-05,
  "fit_frequency_min": 50000,
  "fit_frequency_max": 200000,
  "fit_flux_min": 0.05,
  "fit_flux_max": 0.15
}
```

Expected outputs:

```json
{
  "loss_density": 210325.46208198377,
  "core_loss": 2.103254620819838,
  "energy_per_cycle": 1.4021697472132253e-05
}
```

Substitution: frequency multiplier `(150000/100000)^1.5=1.8371173070873836`; flux multiplier `(0.08/0.1)^2.5=0.5724334022399461`. Thus `pv=200000*1.8371173070873836*0.5724334022399461=210325.46208198377 W/m³`; `P=pv*1e-5=2.103254620819838 W`; `Ecycle=P/150000=1.4021697472132253e-5 J`.

### Independent test scenarios

- Nominal: complete vector above.
- Reference identity: with `frequency=100000` and `flux_peak=0.1`, all other inputs nominal, outputs are exactly `200000 W/m³`, `2 W`, `20e-6 J` in the mathematical model.
- Double only `core_volume`: loss density stays fixed; total heat and per-cycle loss double.
- Inclusive fit endpoints are admitted; changing the fitted range must not silently clamp input. Both reference and operating points must fit.
- Invalid: frequency zero; negative/zero flux; reversed bounds; operating or reference point outside a bound; zero reference loss/volume or nonpositive fitted exponent. Incorrect material/temperature cannot be detected numerically and remains the user's calibration responsibility.

### Mandatory physical-geometry diagram

Left x=50–290,y=70–210: an ungapped closed rectangular core with a schematic winding and a shaded volume callout `core_volume`. Draw a clockwise conceptual circulating-flux arrow entirely within the core, with a magnitude label `flux_peak`. Right x=360–620,y=55–165: a sinusoidal flux sketch centred on zero, with positive peak label `flux_peak` and period bracket labeled “T=1/f” bound to `frequency` only through that explicit relation. Add an outward heat arrow from the core bound to `core_loss`; a small cycle-area label binds `energy_per_cycle`. Reference fit labels `reference_loss_density`, `reference_frequency`, `reference_flux_peak` belong to a calibration callout, not to the live sinusoid. Caption: “Sinusoidal zero-bias flux at calibration temperature; empirical total core heat, not a computed B–H loop. Reference data are supplied, and the sketch is illustrative.”

## 5. `cores.lamination_eddy_loss`

**Purpose and novelty.** Select electrical-steel lamination thickness from classical eddy heating. Closest old item is `magnetics.conductor_skin_depth`: that gives field penetration but no induced-current distribution or dissipated power. Here penetration only screens the complete-penetration approximation. This is distinct from copper winding transport-current loss and measured total Steinmetz loss.

**Equation and units.** MIT section 3.4 assumes a sheet-centred coordinate `y`, uniform parallel sinusoidal flux and no net transport current. In RMS phasors, `Jx=-j*omega*B_rms*y/rho`. Averaging `rho*|J|²` across `-t/2<=y<=t/2` gives `pv=omega²*B_rms²*t²/(12*rho)`. With `B_rms=Bpk/sqrt(2)`, this is `pv=(pi*f*Bpk*t)²/(6*rho)`. `P=pv*Vmetal`. The dimensionless screening ratio is `t*sqrt(pi*f*mu0*mur/rho)`. Resistance is bulk resistivity in ohm metres, not sheet resistance. Volume excludes insulation and voids.

**Validity.** Thin, mutually electrically isolated sheets; flux lies in the sheet plane and is uniform through thickness; no transport current, saturation, normal flux or edge effects. Effective permeability/resistivity belong to the operating temperature and excitation. The complete-penetration premise is source-backed; `t/depth<=0.3` is the explicit conservative engineering screen selected here, not a published accuracy certification. Also enforce a user-supplied linear-material flux limit. Classical loss excludes hysteresis, excess/domain-wall effects, edge damage, burr shorts and joints. Zero frequency/flux gives zero heat. No standalone skin-depth output is added.

### Complete independent numerical example

Inputs:

```json
{
  "frequency": 50,
  "flux_peak": 0.8,
  "lamination_thickness": 0.0002,
  "resistivity": 5e-07,
  "relative_permeability": 1000,
  "metal_volume": 0.0001,
  "linear_flux_limit": 1.2
}
```

Expected outputs:

```json
{
  "penetration_ratio": 0.12566370614359174,
  "eddy_loss_density": 210.55156055657307,
  "eddy_loss": 0.021055156055657307
}
```

Substitution: `x=0.0002*sqrt(pi*50*4*pi*1e-7*1000/5e-7)=0.12566370614359174`, safely below 0.3. `pv=(pi*50*0.8*0.0002)^2/(6*5e-7)=210.55156055657307 W/m³`; `P=pv*1e-4=0.021055156055657307 W`. The `0.8 T` input is peak; using it as RMS would double the answer incorrectly.

### Independent test scenarios

- Nominal: complete vector above.
- Halve only metal thickness: penetration ratio halves to `0.06283185307179587`; density/heat quarter to `52.63789013914327 W/m³` and `0.005263789013914327 W`.
- Zero frequency with otherwise nominal inputs: penetration ratio and both losses are zero. Zero flux at nominal frequency: penetration ratio stays nominal, both losses zero.
- The default geometry reaches `x=0.3` at about `284.965828994075 Hz`; use the defining expression rather than a rounded boundary when testing inclusive acceptance. A frequency of `400 Hz` violates the thin-sheet screen.
- Invalid: zero/negative resistivity or thickness; negative frequency or flux; flux above `linear_flux_limit`; effective relative permeability below one. A thickness large enough to violate penetration must reject, not return the optimistic thin-sheet heat.

### Mandatory physical-geometry diagram

Show a three-sheet insulated lamination stack in perspective at x=45–370,y=60–205. Bind a metal-only thickness bracket to `lamination_thickness`; insulation gaps are shown but never included in that bracket. In-plane sinusoidal B arrow binds `flux_peak`; use two opposite induced-current arrows at positive/negative sheet-depth positions to demonstrate circulating eddies with zero transport current. Right inset is one sheet thickness section with centre coordinate y=0, faces y=±t/2, and qualitative antisymmetric J arrows. A heat arrow binds `eddy_loss`; labels bind `penetration_ratio` and `eddy_loss_density`. A stack-metal-only callout binds `metal_volume`. Caption: “Parallel flux penetrates each electrically isolated thin sheet. Current density changes sign across its midplane; only classical eddy heat is estimated. Insulation volume is excluded.”

## 6. `cores.thin_foil_winding_loss`

**Purpose and novelty.** Estimate layer-dependent copper AC heat for a thin series foil winding. Closest old item is skin depth; it does not model current redistribution, series layers, winding resistance or loss. Closest new neighbor is magnetic-sheet eddy heating, where no transport current exists and the material is ferromagnetic. Here the winding carries current, and its self/proximity field grows through successive layers.

**Source-backed derivation.** Dowell appendix equations (32), (33) define `M=z*coth(z)`, `D=2z*tanh(z/2)`. Equation (8), divided by the geometric DC winding resistance, gives `Rac/Rdc=Re[M+(m²-1)*D/3]`, with porosity one for solid foil, `z=(1+j)*x`, `x=t/depth=t*sqrt(pi*f*mu0/rho)`.

Expand analytically rather than subtract nearly equal hyperbolic functions:

- `M=1+z²/3-z⁴/45+2z⁶/945-z⁸/4725+...`.
- `D=z²-z⁴/12+z⁶/120-17z⁸/20160+...`.
- Therefore `Re(M)=1+4x⁴/45-16x⁸/4725+...` and `Re(D)=x⁴/3-17x⁸/1260+...`.
- Keeping through `x⁴` gives `excess=(5m²-1)*x⁴/45`, `Rac=Rdc*(1+excess)`.
- `Rdc=rho*m*MLT/(b*t)` for one foil turn per layer. Total heat is `I_rms²*Rac`; additional AC heat is `I_rms²*Rdc*excess`.

This calculator is explicitly the **thin-foil asymptotic model**, not the full high-frequency Dowell formula with a hidden numerical cutoff. The physical thin-layer premise comes from Dowell; the specific `x<=0.5` screen is chosen from the truncation analysis here. The first neglected term is `-[16/4725+17(m²-1)/3780]*x⁸`. At `x=0.5`, the retained total ratio versus the independently evaluated full expression is: one layer 1.0055555556 versus 1.0055423618; three layers 1.0611111111 versus 1.0609577347; ten layers 1.6930555556 versus 1.6913076020. These examples quantify truncation only, not real-winding accuracy. Do not imply a universal measured-error guarantee.

**Validity and signs.** Nonmagnetic solid foil, sinusoidal RMS current, one turn per layer, equal approximate mean turn lengths, series layers. The portion has zero tangential magnetic field at one side and monotonically increasing MMF through the layers. No interior interleaving, gap-fringing, parallel current sharing, nearby return-field changes or significant curvature/edge fields. Foil width is at least twenty foil thicknesses as a declared sheet-geometry screen. Round-wire/litz conversion is explicitly excluded: Nan/Sullivan demonstrates potentially large errors there. Resistivity is at operating temperature; thermal rise and harmonics are not modeled. Current is a magnitude; the diagram's dot/cross orientation defines the conceptual field only.

**Numerical implementation.** Compute `x` directly from `sqrt(f)`, never from an infinite DC penetration depth. Admit `f=0` and `I=0`. Keep `ac_excess_factor` separately; do not calculate it by subtracting 1 from a rounded total ratio. Preserve representable dimensional excess heat using `(I*x*sqrt(Rdc)*sqrt((5m²-1)/45)*x)^2`, independently of the possibly underflowed dimensionless excess output. This uses existing primitives and requires no shared helper or frequency floor.

### Complete independent numerical example

Inputs:

```json
{
  "frequency": 100000,
  "foil_thickness": 5e-05,
  "foil_width": 0.02,
  "layers": 3,
  "mean_turn_length": 0.1,
  "resistivity": 1.72e-08,
  "current_rms": 10
}
```

Expected outputs:

```json
{
  "thickness_ratio": 0.23954419411992858,
  "resistance_dc": 0.00516,
  "ac_excess_factor": 0.0032194579693949473,
  "resistance_ac": 0.005176612403122077,
  "winding_loss": 0.5176612403122077,
  "ac_excess_loss": 0.0016612403122077928
}
```

Substitution: `x=50e-6*sqrt(pi*100000*4*pi*1e-7/1.72e-8)=0.23954419411992858`; `Rdc=1.72e-8*3*0.1/(0.02*50e-6)=0.00516 ohm`. The coefficient `(5*3²-1)/45=44/45`; `excess=(44/45)*x^4=0.0032194579693949473`. Therefore `Rac=0.00516*(1+excess)=0.005176612403122077 ohm`, total heat `100*Rac=0.5176612403122077 W`, and additional AC heat `100*0.00516*excess=0.0016612403122077928 W`.

### Independent test scenarios

- Nominal: complete vector above.
- DC limit, `frequency=0`, other inputs nominal: `thickness_ratio=0`, `resistance_dc=resistance_ac=0.00516 ohm`, `ac_excess_factor=0`, `winding_loss=0.516 W`, `ac_excess_loss=0`.
- Zero current at nominal frequency leaves all resistance/ratio outputs unchanged and both heat outputs zero.
- Halving only frequency divides excess factor by four. Increasing layer count increases both geometric Rdc and proximity coefficient; do not replace the coefficient with an isolated-foil skin term.
- Thin-limit frequency is about `435681.0896620524 Hz` for the nominal material/thickness. `500000 Hz` must reject; test the exact defining boundary separately. `foil_width=20*foil_thickness` is an inclusive geometry screen.
- Invalid: fractional/zero layer count; nonpositive thickness, width, length or resistivity; negative current/frequency; width below twenty thicknesses.
- Pure numerical conditioning probe, not a realizable hardware operating point: set `frequency=1e-170 Hz` and `current_rms=1e100 A`, leaving every other nominal input unchanged. Decimal derivation gives `x=7.5750525368848788e-89`, true dimensionless excess `3.2194579693949486e-353` (correctly rounds to zero in binary64), but additional AC heat `1.6612403122077935e-155 W` remains representable and must not be lost. `Rdc=Rac=0.00516 ohm` to binary64 precision and total heat is approximately `5.16e197 W`. This distinguishes permissible rounding of the ratio from premature loss of the dimensional result.

### Mandatory physical-geometry diagram

Left x=55–320,y=55–220: three separated solid foil layers, one turn per layer, shown as axial-width strips with thin insulation gaps. Bracket total layer count with `layers`; individual metal thickness binds `foil_thickness`; strip axial width binds `foil_width`. A side inset shows a single wrap and its centreline path bracket `mean_turn_length`. Put equal-direction current dots in the foil cross sections and a separate opposite return winding outside the modeled portion; annotate the left boundary “H=0” and a rising conceptual MMF staircase across the three series layers. This is not a parallel-foil connection. Bind the current magnitude arrow to `current_rms`. Right/lower result labels bind `ac_excess_factor`, `resistance_ac`, `winding_loss`, and `ac_excess_loss`; other quantities remain in the legend. Caption: “One series turn per solid-foil layer; one-dimensional field starts at zero on one side. Thin-foil expansion only (t/depth ≤ 0.5); current redistribution is schematic, not a live field solution.”

## Integration and independent QA handoff

Promote all six definitions without changing old IDs. Implement one described physical diagram per ID using exact JSON keys. No shared numerical helper or new expression feature is required. Parent owns catalog loading/registry integration and executes the complete examples through the real Rust process, geometric/fit/domain boundaries, the tiny dimensional foil-loss probe and native diagram inspection. Numeric example tolerance is arithmetic-only, not empirical physical accuracy. For representable tiny results use scale-aware relative/ULP assertions; a generic absolute tolerance must not allow zero to pass.

Unresolved engineering risks: Wheeler's original scan remains inaccessible, but a university equation and explicit corroborated unit conversion were read; empirically calibrated material bounds and temperatures must be supplied honestly; field screens are declared engineering approximations rather than certification. All requested research artifacts and six concrete diagram specifications are complete. No production implementation, runtime verification, physical coil measurement, FEM comparison or safety certification is claimed.
