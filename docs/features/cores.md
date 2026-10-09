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

