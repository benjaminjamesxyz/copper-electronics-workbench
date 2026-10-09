# Winding and magnetic design — feature catalog

Executable definitions: [cores.json](../../backend/catalog/cores.json). Illustrations: [cores.dart](../../frontend/lib/diagrams/cores.dart). Research: [equations and evidence](../../research/expansion/cores.md), [complete schema and vectors](../../research/expansion/cores.json).

Rust evaluates all quantities; Flutter uses the existing static diagram primitives. All lengths use metres, flux uses peak tesla, and current uses RMS amperes. Numerical tolerance 1e-9 is arithmetic-only, not physical accuracy. The sections below preserve the supplied independent derivations, cases and source limitations; implementation did not repeat the research source-access review. Main owns registry integration, independent tests, runtime examples and native image acceptance. No worker checks were run.


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


## `cores.single_layer_air_solenoid`

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

## `cores.square_planar_spiral`

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

## `cores.bobbin_layer_fit`

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

