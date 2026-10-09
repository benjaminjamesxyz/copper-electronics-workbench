# Expansion backlog — 100 required calculator-and-diagram features

## Scope and acceptance contract

This is a planning artifact after the completed research wave, not a runtime or review-pass report. All **100 entries below are Must-have**, across **16 accountable lanes**: **26 basic, 39 intermediate, 35 expert**. Preserve the **128 baseline calculators and diagrams** at `29f9c2b`, yielding **228 total**. No diagram, numerical feature, boundary behavior or documentation is deferred.

[SPRINT.md](SPRINT.md) defines ownership, worktrees, atomic commits, sequencing, integration and closure gates. The [baseline manifest](baseline.json), [expansion contract](../EXPANSION_CONTRACT.md), [numerical contract](../CONTRACT.md) and [research board](../../RESEARCH_BOARD.md) remain authoritative. Planning reconciliation found 100 unique research IDs, exactly the same 100 board IDs, and zero intersections with the 128 baseline IDs. Distinct IDs alone do not establish novelty: each entry records its physical distinction; the sprint records high-risk semantic decisions.

### Required for every entry

- **Ownership:** the named D2 developer owns the numerical model, diagram and feature documentation together. The named T2 tester independently authors behavioral assertions. The named V2 reviewer independently checks scientific source/model correctness and actual native image pixels. Main integrates and executes; workers run no builds, tests, lint, formatters, engines or GUI smoke checks.
- **Numerical oracle:** the linked lane JSON object selected by the entry's exact `id` supplies the **full `example.inputs`, every `example.expected` output and `relative_tolerance`**. The displayed output anchors below are actual researched values, not invented estimates or the full test map. The linked Markdown section contains the independent derivation, units and full nominal/alternate vectors. Test **every** expected output, not just displayed anchors; independently derive additional cases from the cited physics rather than replaying catalog expressions.
- **Zero/error behavior:** use the researched relative tolerance for nonzero nominal outputs. Use exact algebraic-zero checks where the expression permits; use explicitly justified dimensional absolute/ULP residuals for trigonometric nulls and cancellation tests. Never apply a broad absolute floor that hides a small physical result. Cover all input/constraint guards, nonfinite rejection and a subsequent valid request. Preserve signed failed-design diagnostics where the physical domain permits them.
- **Mandatory diagram:** each entry requires its own exact-ID figure. The compact topology and quantity checklist below supplement, rather than replace, the full linked mandatory diagram specification. Preserve all specified pin connections, polarity, reference directions, relation labels and explanatory caption. Research alternatives for label placement may use a readable legend; they never make a calculator's diagram or scientific content optional. Secondary decorative insets are not substitute features.
- **Drawing acceptance:** use existing `primitives.dart` only, `const <lane>Diagrams = <String, CalculatorDiagram>{...}`, 700×280 canvas, at least 16 px margins and 14 px labels. Bind `Label.quantity` only to actual catalog keys. Break wires at components, mark junctions, keep nonjoining crossings and isolated grounds unambiguous. Label derived quantities with their relation. Static explanatory curves are not computed results. V2 checks source plus Main's actual native captures, including accessible legend, contrast, clipping, large text and narrow scrolling.
- **Prerequisite P0:** existing schema, stable expression functions, shared painter/panel and Main-owned registry integration. P0 permits independent developer work; no calculator depends on another calculator's implementation or output. Characterization and model assumptions below are input applicability requirements, not new features or manufactured certification.
- **Prerequisite P1:** only `measurement.rectangular_aperture_response` needs Main's approved zero-safe `sinc(x)`, with `sinc(0)=1`. Main executed its focused regression: one passed, zero failed. Integrated aperture process/GUI acceptance remains pending; no epsilon denominator or positive-frequency floor substitutes for the analytic limit.
- **Completion:** one atomic feature commit contains this calculator, its mandatory diagram and docs. T2 must cover full vectors and important limits/errors; V2 must close source and native-image findings. Main must exercise this feature through both the actual Rust process and native GUI. Shared Main gates also preserve and traverse all 128 old features.

## Sensors — 7 required items

Lane accountability: **D2Sensors / T2Sensors / V2Sensors**. Levels: 3 basic, 3 intermediate, 1 expert.

### 001. `sensors.ntc_steinhart_hart` — Steinhart–Hart NTC thermistor temperature

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Sensors developer; T2Sensors tester; V2Sensors source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/sensors.md#1-sensorsntc_steinhart_hart); [complete JSON example vector](../../research/expansion/sensors.json), selected by `id=sensors.ntc_steinhart_hart`. Require every output at nominal relative tolerance `1e-09`. Numerical anchors: `temperature_kelvin=298.1496681766963` K; `temperature_celsius=24.999668176696318` degC; `temperature_sensitivity=-0.002279553664363643` K/ohm.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use calibrated sensor/material coefficients, declared fit intervals and excitation limits; transduction does not include conditioning electronics.

**Semantic novelty decision — retain as one distinct physical problem.**

Map measured NTC resistance through a calibrated reciprocal-temperature cubic in log resistance. Closest baseline `thermal.resistor_tcr_drift` assumes a linear resistance-temperature slope. Neither that model nor `dc.loaded_wheatstone_bridge` can reproduce this material response. A beta inverse is not a second feature.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

Nominal all-output comparison; larger R gives lower T within the fit interval. `C=0` is the legitimate two-coefficient limiting member, not another ID. As a simple independent algebraic case, `R=1 ohm,A=1/300 1/K,B=0.001 1/K,C=0`, default fit interval gives approximately `T=300 K,t=26.85 degC,dT/dR=-90 K/ohm`. Reject R=0/negative, B=0, C<0, nonpositive fit temperatures, reversed/equal fit bounds, and a resistance predicting T outside the fit interval. Boundary equality in reciprocal temperature is accepted.

**Mandatory diagram topology:** Two-terminal thermistor with temperature arrow; decreasing illustrative R–T curve and separately labeled dT/dR tangent. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `resistance`, `coefficient_a`, `coefficient_b`, `coefficient_c`, `temperature_kelvin`, `temperature_sensitivity`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 002. `sensors.platinum_callendar_van_dusen` — Platinum RTD nonlinear resistance response

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Sensors developer; T2Sensors tester; V2Sensors source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/sensors.md#2-sensorsplatinum_callendar_van_dusen); [complete JSON example vector](../../research/expansion/sensors.json), selected by `id=sensors.platinum_callendar_van_dusen`. Require every output at nominal relative tolerance `1e-09`. Numerical anchors: `resistance=60.25584` ohm; `resistance_sensitivity=0.4053081` ohm/K; `linear_model_error=-0.66116` ohm.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use calibrated sensor/material coefficients, declared fit intervals and excitation limits; transduction does not include conditioning electronics.

**Semantic novelty decision — retain as one distinct physical problem.**

Forward platinum RTD response over both coefficient branches, including a quartic correction below zero Celsius. Closest baseline `thermal.resistor_tcr_drift` lacks both quadratic and subzero quartic curvature. NTC carrier physics and reciprocal-log calibration are different from platinum RTD conduction; these are not inverses.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

Nominal; `t=0` gives `(100,0.39083,0)` in output order. `t=-200` gives `(18.52008,0.4323352,-3.31392)`; `t=850` gives `(390.481125,0.292655,-41.724375)`. Both endpoints are accepted. Values just below/above zero must approach the same resistance and sensitivity. Scaling R0 by ten scales every output by ten. Reject R0<=0, t<-200 and t>850.

**Mandatory diagram topology:** Platinum serpentine with distinct excitation/sense leads; nonlinear R(T), zero-temperature tangent and deviation bracket. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `zero_resistance`, `temperature_celsius`, `resistance`, `resistance_sensitivity`, `linear_model_error`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 003. `sensors.type_t_cold_junction_emf` — Type-T thermocouple EMF with finite cold junction

- **Required / level:** Must-have calculator + diagram + docs; expert.
- **Accountable:** D2Sensors developer; T2Sensors tester; V2Sensors source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/sensors.md#3-sensorstype_t_cold_junction_emf); [complete JSON example vector](../../research/expansion/sensors.json), selected by `id=sensors.type_t_cold_junction_emf`. Require every output at nominal relative tolerance `1e-09`. Numerical anchors: `hot_emf=0.00427851861580027` V; `cold_emf=0.0009919772678200114` V; `hot_sensitivity=4.67849607861716e-05` V/K.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use calibrated sensor/material coefficients, declared fit intervals and excitation limits; transduction does not include conditioning electronics.

**Semantic novelty decision — retain as one distinct physical problem.**

Predict the actual Type-T terminal signal from two junction temperatures, not a generic voltage difference or constant Seebeck slope. No baseline sensor EMF model exists. NTC/RTD are excited resistive sensors; a thermocouple is a self-generated thermoelectric source.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

Nominal; Th=Tc gives exactly zero terminal EMF without requiring both absolute EMFs to be zero. Swapping temperatures negates terminal voltage and swaps absolute EMFs; sensitivity follows the new hot input. Tc=0 gives zero cold EMF. Th=400,Tc=0 gives `terminal_emf=0.02087197005052672 V`, agreeing with the rounded 20.872 mV NIST table. With Tc=100 and Th=100+2^-46, terminal voltage is approximately `6.648542805887075e-19 V`, positive and nonzero; use the divided-difference oracle, not subtraction of binary64 absolute EMFs. Reject either input below 0 or above 400.

**Mandatory diagram topology:** Copper/constantan measurement weld feeds separate open sense terminals through an isothermal reference block; preserve terminal polarity. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `hot_temperature`, `cold_temperature`, `hot_emf`, `cold_emf`, `terminal_emf`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 004. `sensors.hall_plate_angle` — Hall-plate field-angle transduction

- **Required / level:** Must-have calculator + diagram + docs; basic.
- **Accountable:** D2Sensors developer; T2Sensors tester; V2Sensors source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/sensors.md#4-sensorshall_plate_angle); [complete JSON example vector](../../research/expansion/sensors.json), selected by `id=sensors.hall_plate_angle`. Require every output at nominal relative tolerance `1e-09`. Numerical anchors: `normal_field=0.02500000000000001` T; `hall_voltage=0.02500000000000001` V; `angle_sensitivity=-0.04330127018922193` V/rad.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use calibrated sensor/material coefficients, declared fit intervals and excitation limits; transduction does not include conditioning electronics.

**Semantic novelty decision — retain as one distinct physical problem.**

Translate the normal projection of a tilted magnetic field into transverse Hall voltage and angular sensitivity. Closest baseline `magnetics.field_energy_density` predicts stored energy, not carrier separation or sensor voltage. The neighboring AMR calculation predicts longitudinal scattering resistance, has different terminals and different angular symmetry.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

At theta=0 with other defaults, outputs `(0.05 T,0.05 V,0 V/rad)`. At B=0 all three outputs are zero. I=0 leaves normal field unchanged but both voltage outputs zero. Reverse I or RH: both voltage outputs change sign, Bn does not. Reverse field orientation by pi: field and Hall voltage reverse. Bias or field equal to its declared maximum is accepted; larger magnitudes, negative B, zero thickness, or nonpositive limits are rejected. Quarter-turn angular values produce floating-point residuals; do not demand `cos(pi/2)` be exact zero.

**Mandatory diagram topology:** Hall plate with longitudinal bias, separate transverse voltage contacts, surface normal, tilted field and thickness dimension. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `field_magnitude`, `field_angle`, `bias_current`, `plate_thickness`, `normal_field`, `hall_voltage`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 005. `sensors.amr_angular_resistance` — Anisotropic magnetoresistive angular resistance

- **Required / level:** Must-have calculator + diagram + docs; basic.
- **Accountable:** D2Sensors developer; T2Sensors tester; V2Sensors source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/sensors.md#5-sensorsamr_angular_resistance); [complete JSON example vector](../../research/expansion/sensors.json), selected by `id=sensors.amr_angular_resistance`. Require every output at nominal relative tolerance `1e-09`. Numerical anchors: `resistance=1005.0` ohm; `voltage_drop=1.0050000000000001` V; `angular_resistance_sensitivity=-17.320508075688775` ohm/rad.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use calibrated sensor/material coefficients, declared fit intervals and excitation limits; transduction does not include conditioning electronics.

**Semantic novelty decision — retain as one distinct physical problem.**

Predict the longitudinal angular resistance of a ferromagnetic thin-film strip. Closest baseline `thermal.resistor_tcr_drift` responds to temperature, not magnetization orientation. `dc.loaded_wheatstone_bridge` cannot supply the material response. Hall voltage is transverse and odd under field reversal; this AMR resistance is longitudinal and pi-periodic. These are physically distinct sensors, not renamed angle calculations.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

theta=0 gives `(1020 ohm,1.02 V,0 ohm/rad)`; theta=pi/2 approaches `(1000,1,0)` with trigonometric roundoff. Adding pi leaves resistance unchanged. Reversing sensing current changes only voltage sign. DeltaR=0 is a valid isotropic limiting case. DeltaR=-20 gives positive resistance and reversed angular sensitivity; DeltaR=-1000 is rejected because one orientation reaches zero. Reject nonpositive Rperp/current limit and excessive current; limits are inclusive.

**Mandatory diagram topology:** Longitudinal thin-film strip with current/drop polarity and internal magnetization angle; distinguish perpendicular resistance and anisotropy. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `perpendicular_resistance`, `resistance_anisotropy`, `magnetization_angle`, `bias_current`, `resistance`, `voltage_drop`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 006. `sensors.hydrogen_activity_electrode` — Nernst hydrogen-activity electrode

- **Required / level:** Must-have calculator + diagram + docs; basic.
- **Accountable:** D2Sensors developer; T2Sensors tester; V2Sensors source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/sensors.md#6-sensorshydrogen_activity_electrode); [complete JSON example vector](../../research/expansion/sensors.json), selected by `id=sensors.hydrogen_activity_electrode`. Require every output at nominal relative tolerance `1e-09`. Numerical anchors: `sample_ph=4.0` pH; `nernst_slope=0.05915934968478233` V/pH; `activity_sensitivity=256.92579121085845` V.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use calibrated sensor/material coefficients, declared fit intervals and excitation limits; transduction does not include conditioning electronics.

**Semantic novelty decision — retain as one distinct physical problem.**

Convert dimensionless hydrogen chemical activities into logarithmic glass-electrode voltage with temperature-dependent Nernst slope. Closest baseline `diodes.shockley_forward_current` is electronic junction transport, not electrochemical membrane equilibrium. It is not a generic logarithm calculator. Neighboring battery intercalation includes occupancy and interaction-energy physics; no battery voltage/SOC model is claimed here.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

Equal activities `a=b=1e-7` give pH=7 and V exactly equal to offset; slope is unchanged and sensitivity is `256925.79121085845 V` per unit activity. Swapping activities negates only the voltage change relative to offset; sensitivity remains sample-specific. At `a=1e-14,b=1,T=298.15,offset=0.005`, voltage is `-0.8232308955869526 V`, not an invalid log result. A next-representable increase from equal activities and zero offset must produce a positive nonzero electrode voltage. Both temperature endpoints and activity endpoints are valid. Reject activities outside the declared interval, zero/negative activity, and temperatures outside 273.15–373.15 K.

**Mandatory diagram topology:** Glass bulb and reference junction share one solution vessel; high-impedance measurement and calibration-standard activity remain distinct. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `sample_activity`, `standard_activity`, `temperature`, `sample_ph`, `nernst_slope`, `electrode_voltage`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 007. `sensors.differential_gap_capacitor` — Differential variable-gap displacement sensor

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Sensors developer; T2Sensors tester; V2Sensors source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/sensors.md#7-sensorsdifferential_gap_capacitor); [complete JSON example vector](../../research/expansion/sensors.json), selected by `id=sensors.differential_gap_capacitor`. Require every output at nominal relative tolerance `1e-09`. Numerical anchors: `near_capacitance=1.1067734766e-11` F; `far_capacitance=7.378489844e-12` F; `differential_sensitivity=1.9983409994166667e-07` F/m.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use calibrated sensor/material coefficients, declared fit intervals and excitation limits; transduction does not include conditioning electronics.

**Semantic novelty decision — retain as one distinct physical problem.**

Evaluate a moving center electrode between two fixed plates, including the nonlinear difference and derivative. Closest baseline `timing.plate_displacement_current` assumes fixed geometry and differentiates voltage to obtain current. Here geometry moves, voltage is not an input, and opposing capacitances encode displacement. It is not a unit change, inverse current calculation or generic capacitor-energy calculation.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

At x=0, both capacitances equal `8.8541878128e-12 F`, difference and ratio are exactly zero, sensitivity is `1.77083756256e-7 F/m`. Reversing x swaps capacitances and reverses both differences, while sensitivity remains positive and unchanged. Tiny signed x must preserve a tiny signed differential capacitance even when the individual capacitances round identically. As abs(x) approaches g from below, one capacitance and the sensitivity diverge; actual stops/parasitics invalidate this ideal limit before contact. Reject abs(x)>=g, nonpositive dimensions/gap, epsilon_r<1 and violation of the declared thin-gap rule. The 10:1 equality is accepted. Doubling epsilon_r doubles the four dimensional outputs but leaves normalized difference unchanged.

**Mandatory diagram topology:** Moving center electrode between fixed plates; positive displacement reduces right gap g−x and increases left gap g+x. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `rest_gap`, `displacement`, `near_capacitance`, `far_capacitance`, `differential_capacitance`, `normalized_difference`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

## Opto — 7 required items

Lane accountability: **D2Opto / T2Opto / V2Opto**. Levels: 2 basic, 2 intermediate, 3 expert.

### 008. `opto.photodiode_quantum_conversion` — Unity-gain photodiode photon conversion

- **Required / level:** Must-have calculator + diagram + docs; basic.
- **Accountable:** D2Opto developer; T2Opto tester; V2Opto source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/opto.md#1-optophotodiode_quantum_conversion); [complete JSON example vector](../../research/expansion/opto.json), selected by `id=opto.photodiode_quantum_conversion`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `photon_energy=2.3369951260575634e-19` J; `photon_rate=85579981648226.06` 1/s; `photocurrent=1.0283568520120244e-05` A.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Supply wavelength-dependent optical parameters and honor the stated point-source, paraxial, weak-guidance or short-pulse domain.

**Semantic novelty decision — retain as one distinct physical problem.**

Use this basic model to budget the generated reverse photocurrent from monochromatic light. Closest baseline: `diodes.shockley_forward_current`, which predicts thermally activated junction conduction from voltage, and `noise.junction_shot`, which predicts a noise spectral amplitude from current. Neither models photon-to-carrier conversion. The neighboring precision lane owns conversion of that current into voltage; this item has no amplifier, feedback resistor or noise budget. Compared with this lane's absorption item, QE is a supplied external photon-to-collected-carrier ratio, not inferred from slab geometry.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

Nominal: assert all four outputs. With all other inputs unchanged, double power: photon rate and current double while photon energy and responsivity do not change. Zero power gives exact zero rate/current; QE zero gives exact zero responsivity/current but preserves positive incident photon rate. QE one is valid. Reject zero/negative wavelength, negative power and QE outside [0,1]. Apply shared nonfinite-input/error-recovery coverage. The wavelength/QE pair's physical calibration remains a user obligation.

**Mandatory diagram topology:** Photon arrow enters P/depletion/N section; generated conventional current points N/cathode to P/anode without an amplifier. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `wavelength`, `quantum_efficiency`, `optical_power`, `responsivity`, `photocurrent`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 009. `opto.lambertian_target_irradiance` — Lambertian source target irradiance

- **Required / level:** Must-have calculator + diagram + docs; basic.
- **Accountable:** D2Opto developer; T2Opto tester; V2Opto source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/opto.md#2-optolambertian_target_irradiance); [complete JSON example vector](../../research/expansion/opto.json), selected by `id=opto.lambertian_target_irradiance`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `axial_intensity=0.03183098861837907` W/sr; `directional_intensity=0.0275664447710896` W/sr; `target_irradiance=0.4873105007710476` W/m^2.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Supply wavelength-dependent optical parameters and honor the stated point-source, paraxial, weak-guidance or short-pulse domain.

**Semantic novelty decision — retain as one distinct physical problem.**

Use this basic model to position an illumination source and orient a target surface. Closest baseline: `diodes.led_series_resistor` sets electrical drive only. This model instead integrates an angular radiometric distribution and applies a receiving-plane projection. It is not an RF antenna/link budget: no wavelength-dependent antenna aperture or received-radio-power equation is used.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

Nominal: all three outputs. Both angles zero at the same distance gives `target_irradiance=0.795774715459476678844 W/m²`. Doubling distance quarters only irradiance. Zero flux makes all outputs zero. Grazing source or receiver orientation tends to zero; allow the machine cosine residual at represented pi/2, scaled to flux/distance², rather than asserting mathematically exact floating-point zero. Reject negative angles, angles exceeding pi/2, nonpositive distance and negative flux. Close-but-positive distances remain mathematical inputs, but a real extended source at that distance violates the stated point-source assumption.

**Mandatory diagram topology:** Emitter normal, propagation ray and receiving-plane normal define separate emission/incidence angles and distance. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `radiant_flux`, `distance`, `emission_angle`, `incidence_angle`, `target_irradiance`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 010. `opto.layer_absorption_capture` — Inactive and active optical layer absorption

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Opto developer; T2Opto tester; V2Opto source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/opto.md#3-optolayer_absorption_capture); [complete JSON example vector](../../research/expansion/opto.json), selected by `id=opto.layer_absorption_capture`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `dead_absorbed_power=0.0003625384938440363` W; `active_capture_fraction=0.5175365411657797` 1; `transmitted_power=0.0006023884238244042` W.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Supply wavelength-dependent optical parameters and honor the stated point-source, paraxial, weak-guidance or short-pulse domain.

**Semantic novelty decision — retain as one distinct physical problem.**

Use this intermediate model to choose absorbing-layer thickness when a noncollecting entrance layer wastes incoming light. H2 section I equation 1 states `Phi(z)=Phi(0)*exp(-alpha*z)` and discusses penetration depth and carrier recombination in inactive material. Apply that spatial absorption law successively to two homogeneous layers. This is a layer-power partition derived from the source equation, not a manufacturer-specific device fit.

Closest baseline: `noise.decibel_to_linear` converts an already known logarithmic ratio; it does not derive absorption from material coefficients and geometry. `transients.rc_step` is temporal energy storage, not photon attenuation. Closest neighboring item: `opto.photodiode_quantum_conversion` uses measured external QE to determine current. Here the outputs are absorbed optical powers; active absorption does not imply unit collection or predict photocurrent. No extra QE inverse is counted.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

Assert all four outputs and conservation of the three powers. Set both coefficients to zero: inactive and active absorption are zero, active fraction zero, transmission `0.002 W`. Set only inactive thickness to zero: active fraction `0.632120558828557678404`, active power `0.00126424111765711535681 W`, transmission `0.000735758882342884643192 W`, inactive power zero. Active thickness zero eliminates useful capture but preserves entrance loss. Tiny positive active optical depth `1e-16` with no inactive loss must retain a positive capture fraction approximately `1e-16`; this requires `expm1`. A very thick active slab tends to capture all surviving entrance power. Reject any negative power, coefficient or thickness. No invented upper thickness or absorption bound is required.

**Mandatory diagram topology:** Inactive entrance slab precedes active slab; separate absorption arrows and transmitted beam preserve optical-power partition. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `incident_power`, `dead_absorption`, `dead_thickness`, `active_absorption`, `active_thickness`, `dead_absorbed_power`, `active_absorbed_power`, `transmitted_power`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 011. `opto.gaussian_aperture_capture` — Gaussian beam propagation and aperture capture

- **Required / level:** Must-have calculator + diagram + docs; expert.
- **Accountable:** D2Opto developer; T2Opto tester; V2Opto source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/opto.md#4-optogaussian_aperture_capture); [complete JSON example vector](../../research/expansion/opto.json), selected by `id=opto.gaussian_aperture_capture`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `rayleigh_range=0.04963021569652122` m; `beam_radius=0.0001419491843517115` m; `axis_irradiance=315947.0612663579` W/m^2.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Supply wavelength-dependent optical parameters and honor the stated point-source, paraxial, weak-guidance or short-pulse domain.

**Semantic novelty decision — retain as one distinct physical problem.**

Use this expert model to select an aperture or active detector radius in a propagating laser beam. N3 defines w as the 1/e² intensity radius and gives `w(z)²=w0²[1+(lambda*z/(pi*w0²))²]` and `P(a)=Ptotal[1-exp(-2*a²/w²)]`. Its warning about truncation changing subsequent beam propagation is retained.

Closest baseline: `signals.line_propagation` computes delay/wavelength, with no transverse diffraction or aperture integral. Closest neighboring item: fiber temporal dispersion changes pulse width through spectral phase; this model is free-space transverse spreading and spatial interception. Sharing Gaussian algebra does not make these the same physical problem. The entire propagation-plus-capture chain counts as one calculator, not separate radius and power items.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

All five nominal outputs. At `axial_distance=0` and `aperture_radius=0.0001`, radius equals `0.0001 m`, capture fraction equals `1-exp(-2)=0.864664716763387308106`, and power equals `0.00864664716763387308106 W`; axis irradiance is `636619.772367581343076 W/m²`. The Rayleigh range remains unchanged. Swap z to -z: all outputs unchanged. Zero power only zeros the power/irradiance outputs; zero aperture only zeros capture fraction/power. At `abs(z)=zR`, radius is `sqrt(2)*w0`. Tiny a/w must retain positive fraction through `expm1`. Reject nonpositive waist/wavelength, negative aperture/power and divergence above 0.1 rad. Include the admitted 0.1-rad endpoint.

**Mandatory diagram topology:** Gaussian 1/e² envelope, waist and axis meet a centered open aperture; distinguish beam radius from aperture radius. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `wavelength`, `waist_radius`, `axial_distance`, `aperture_radius`, `beam_power`, `beam_radius`, `captured_power`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 012. `opto.step_index_mode_cutoff` — Step-index fiber mode cutoff

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Opto developer; T2Opto tester; V2Opto source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/opto.md#5-optostep_index_mode_cutoff); [complete JSON example vector](../../research/expansion/opto.json), selected by `id=opto.step_index_mode_cutoff`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `numerical_aperture=0.13177253128023306` 1; `normalized_frequency=2.190064550298241` 1; `single_mode_margin=0.21493544970175898` 1.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Supply wavelength-dependent optical parameters and honor the stated point-source, paraxial, weak-guidance or short-pulse domain.

**Semantic novelty decision — retain as one distinct physical problem.**

Use this intermediate model to screen fiber core/index choices for single-mode operation. N4 pages 3–4 supplies `NA=sqrt(ncore²-nclad²)`, `V=2*pi*a*NA/lambda` and the approximate LP11 cutoff `V=2.405`. Its physical introduction describes approximately 1% core/cladding contrast.

Closest baseline: `signals.coax_impedance` computes quasi-TEM electrical characteristic impedance; it does not solve dielectric waveguide higher-mode eligibility. `signals.line_propagation` has no core or cladding. Unlike the Gaussian aperture model, numerical aperture does not say how much a given optical launch couples into the fundamental mode. No mode-count approximation or inverse mode is added as a separate item.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

All four nominal outputs. Doubling core radius doubles V and cutoff wavelength, keeps NA fixed and makes the margin negative; do not reject this physically meaningful multimode result. At a wavelength equal to the independently computed cutoff, V is approximately 2.405 and margin approximately zero; allow subtraction-rounding residual relative to V, not an arbitrary large absolute floor. Increasing cladding index toward core index must reduce positive NA, V and cutoff. Test the 1% contrast endpoint and just above it. Reject equal/reversed indices, either index below one, nonpositive radius/wavelength and contrast beyond the admitted weak-guidance scope.

**Mandatory diagram topology:** Longitudinal core/cladding and circular cross-section show core radius, indices and fundamental-mode/cutoff distinction. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `core_index`, `cladding_index`, `core_radius`, `wavelength`, `normalized_frequency`, `cutoff_wavelength`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 013. `opto.fiber_gaussian_dispersion` — Chirped Gaussian pulse fiber dispersion

- **Required / level:** Must-have calculator + diagram + docs; expert.
- **Accountable:** D2Opto developer; T2Opto tester; V2Opto source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/opto.md#6-optofiber_gaussian_dispersion); [complete JSON example vector](../../research/expansion/opto.json), selected by `id=opto.fiber_gaussian_dispersion`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `normalized_dispersion=-0.4` 1; `width_ratio=0.7211102550927979` 1; `output_peak_power=0.007823901817554269` W.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Supply wavelength-dependent optical parameters and honor the stated point-source, paraxial, weak-guidance or short-pulse domain.

**Semantic novelty decision — retain as one distinct physical problem.**

Use this expert model to determine whether a chirped optical pulse broadens or compresses in a fiber. U1 slide 6.3 gives the width ratio `sqrt((1+C*beta2*L/T0²)²+(beta2*L/T0²)²)`; slide 6.4 states initial compression for `C*beta2<0`. Slides 6.2 and 6.7 distinguish linear dispersion from attenuation and nonlinear terms.

Closest baseline: `signals.rise_time_bandwidth` describes a single-pole electrical edge and `signals.line_propagation` gives nondispersive delay. Neither represents quadratic optical spectral phase or chirp-dependent compression. `noise.uncorrelated_rms_sum` adds unrelated random amplitudes; the two terms here derive from one coherently propagated Gaussian field. The spatial Gaussian item concerns a different domain and boundary-value problem.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

Assert all five outputs. With length zero: normalized dispersion 0, width ratio 1, width `1e-11 s`, FWHM `1.66510922231539551271e-11 s`, peak power `0.00564189583547756286948 W`. Beta2 zero gives the same limit at nonzero length. With C=0 and length 5000 m at the nominal beta2/T0, x=-1 and width ratio sqrt(2). With C=1, length 2500 m gives x=-0.5 and the minimum ratio `1/sqrt(2)`; past that distance it grows again. Reversing both C and beta2 preserves all outputs except the sign of normalized dispersion. Zero energy makes only peak power zero. Reject nonpositive initial width and negative length/energy. No fake finite result may hide unrepresentable arithmetic; no arbitrary upper chirp or fiber-length clamp is part of the physical model.

**Mandatory diagram topology:** Input pulse, dispersive fiber and compressed example output; mark 1/e half-widths, not FWHM spans. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `initial_width`, `initial_chirp`, `beta_two`, `fiber_length`, `output_width`, `output_peak_power`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 014. `opto.sipm_finite_cell_response` — SiPM finite-cell short-pulse response

- **Required / level:** Must-have calculator + diagram + docs; expert.
- **Accountable:** D2Opto developer; T2Opto tester; V2Opto source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/opto.md#7-optosipm_finite_cell_response); [complete JSON example vector](../../research/expansion/opto.json), selected by `id=opto.sipm_finite_cell_response`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `seed_count=800.0` 1; `fired_cells=629.5509444597865` 1; `output_charge=1.0086518131261018e-10` C.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Supply wavelength-dependent optical parameters and honor the stated point-source, paraxial, weak-guidance or short-pulse domain.

**Semantic novelty decision — retain as one distinct physical problem.**

Use this expert model to screen SiPM microcell count and expected output charge for short optical pulses. H2 section V Dynamic range equation 9 gives `Nf=Ncell*(1-exp(-Nph*PDE/Ncell))`. Sections II equations 6–7 give gain in electrons per avalanche and total output charge. A uniform Poisson occupancy derivation yields the same form: the probability a cell receives no detected seed is `exp(-S/Ncell)`.

Closest baseline: `noise.junction_shot` describes current fluctuations without discrete available detection sites. `noise.analog_dynamic_range` compares signal and noise amplitudes; it has no occupancy saturation law. Closest neighboring optical item: unity-gain photodiode current remains proportional to flux within its stated range, whereas this item models finite-cell Geiger-mode saturation and pulsed avalanche charge. Gain is supplied, not a second capacitor-charge calculator.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

Assert all four nominal outputs. Zero photon mean or zero PDE yields seed/fired/charge exactly zero and relative response exactly one. For `photon_count=4e-17`, nominal PDE/cells/gain, seed count is `1.6e-17`, fired count approaches `1.6e-17`, response rounds to one and charge approaches `2.5634826144e-30 C`; do not lose the fired count to cancellation. Increasing photons at fixed cells monotonically increases fired count toward cell count but decreases relative response. Doubling gain doubles only charge. `cell_count=1` is valid, while 1.5 is invalid. Reject zero cell count, nonpositive gain, negative photon mean and PDE outside [0,1]. The nominal uniformity/recovery assumptions cannot be inferred from these four scalar inputs; testers must not claim they were validated.

**Mandatory diagram topology:** Microcell array receives photons; multiple hits on a cell contribute only one fired-cell charge in this model. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `photon_count`, `detection_efficiency`, `cell_count`, `fired_cells`, `output_charge`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

## Oscillators — 7 required items

Lane accountability: **D2Oscillators / T2Oscillators / V2Oscillators**. Levels: 2 basic, 3 intermediate, 2 expert.

### 015. `oscillators.timer_555_astable` — 555 astable charge-discharge timing

- **Required / level:** Must-have calculator + diagram + docs; basic.
- **Accountable:** D2Oscillators developer; T2Oscillators tester; V2Oscillators source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/oscillators.md#1-oscillatorstimer_555_astable--basic); [complete JSON example vector](../../research/expansion/oscillators.json), selected by `id=oscillators.timer_555_astable`. Require every output at nominal relative tolerance `1e-09`. Numerical anchors: `high_time=0.0007624618986159398` s; `low_time=0.0006931471805599453` s; `high_duty=0.5238095238095238` 1.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Keep actual feedback topology, thresholds and startup conventions; device bandwidth, swing, drive level and amplitude stabilization remain explicit limits.

**Semantic novelty decision — retain as one distinct physical problem.**

Determine the settled frequency and output duty of an actual self-triggering 555. T555 §6.3.2 supplies the topology, threshold fractions and charge/discharge equations. The model uses exact `ln(2)` instead of the data sheet's rounded 0.693.

Nearest old models: `timing.timer_clock_divider` needs an existing reference clock; this circuit creates oscillation through internal threshold comparators and a discharge switch. `transients.rc_periodic_switching` takes externally imposed interval lengths; this circuit solves those lengths from thresholds and different current paths. The comparator oscillator below instead has a resistor-defined positive-feedback threshold network and no dedicated discharge transistor.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

doubling C doubles both intervals and period, halves frequency and preserves duty. RA=RB gives duty 2/3. As RA/RB approaches zero from above, duty approaches 1/2 but does not pass below it. Reject zero/negative resistance or capacitance. With defaults except C=1e-12 F, reject the frequency-limit violation. Exercise the 100 kHz period boundary using representable values on both sides; do not assume a decimal-rounded equality is exact binary64 equality.

**Mandatory diagram topology:** VCC–RA–pin7–RB–capacitor path; pins2/6 share timing node, reset4 high, pin1 ground and output3 separate. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `charge_resistance`, `discharge_resistance`, `timing_capacitance`, `high_time`, `low_time`, `period`, `frequency`, `high_duty`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 016. `oscillators.comparator_rc_oscillator` — Three-resistor comparator RC oscillator

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Oscillators developer; T2Oscillators tester; V2Oscillators source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/oscillators.md#2-oscillatorscomparator_rc_oscillator--intermediate); [complete JSON example vector](../../research/expansion/oscillators.json), selected by `id=oscillators.comparator_rc_oscillator`. Require every output at nominal relative tolerance `1e-09`. Numerical anchors: `upper_threshold=4` V; `lower_threshold=2` V; `high_duty=0.6131471927654584` 1.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Keep actual feedback topology, thresholds and startup conventions; device bandwidth, swing, drive level and amplitude stabilization remain explicit limits.

**Semantic novelty decision — retain as one distinct physical problem.**

Design an asymmetric comparator square-wave oscillator from its positive-feedback resistor network. RC describes the equal-resistor case; the following conductance derivation extends the same concrete circuit to unequal resistors. This is not a Schmitt input delay measured after an external edge. It determines two state-dependent thresholds and both intervals of an autonomous cycle. Unlike the 555, both timing directions use the same RT, and unequal duty comes from the threshold network rather than separate charge/discharge resistance paths.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

setting all three threshold resistors to 20 kohm gives VL=5/3 V, VH=10/3 V, both intervals `0.001 ln(2) s`, frequency `721.3475204444817 Hz` and duty 0.5. Scaling all three threshold resistors equally leaves every output unchanged. Doubling supply doubles thresholds but leaves timing unchanged. Swap RU and RL: times exchange and duty complements; thresholds reflect around Vs/2. Large RF narrows the hysteresis window; `log1p` should retain nonzero small intervals. Reject any nonpositive supply/component. The ideal high-frequency limit is not a claim that finite-delay hardware still oscillates at that rate.

**Mandatory diagram topology:** RU/RL/RF define positive-input hysteresis; output charges the negative-input capacitor through RT on a separate feedback path. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `supply_voltage`, `upper_resistance`, `lower_resistance`, `feedback_resistance`, `timing_resistance`, `timing_capacitance`, `upper_threshold`, `lower_threshold`, `high_time`, `low_time`, `frequency`, `high_duty`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 017. `oscillators.triangle_integrator_oscillator` — Integrator and comparator triangle oscillator

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Oscillators developer; T2Oscillators tester; V2Oscillators source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/oscillators.md#3-oscillatorstriangle_integrator_oscillator--intermediate); [complete JSON example vector](../../research/expansion/oscillators.json), selected by `id=oscillators.triangle_integrator_oscillator`. Require every output at nominal relative tolerance `1e-09`. Numerical anchors: `triangle_peak=2` V; `triangle_pp=4` V; `frequency=6250` Hz.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Keep actual feedback topology, thresholds and startup conventions; device bandwidth, swing, drive level and amplitude stabilization remain explicit limits.

**Semantic novelty decision — retain as one distinct physical problem.**

Design the amplitude and period of an op-amp integrator closed around a hysteretic comparator. TRI's triangle-generator block and design steps 3–4 give the amplitude ratio and oscillation frequency. Its single-supply midpoint implementation is translated to symmetric voltages about a stiff reference; no PWM model is added.

Nearest old model: `opamps.integrator_constant_input` integrates a prescribed input from a prescribed initial state. This addition determines self-consistent switching amplitude and repetition frequency, with no prescribed interval. Unlike the RC comparator oscillator, an active integrator maintains a constant input-resistor voltage and produces linear ramps instead of exponential capacitor charging.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

doubling square_peak doubles triangle_peak, triangle_pp and ramp_slew, while period and frequency remain unchanged. Doubling RT doubles triangle amplitude and period but leaves ramp_slew unchanged. Doubling RI doubles period and halves ramp_slew and frequency. With RT/RF approaching zero, predicted frequency diverges; hardware delay/slew limitations invalidate that asymptote, not the algebra. Reject zero/negative amplitude, resistance or capacitance. A high RT/RF is mathematically permitted but requires explicit larger integrator swing, not clipping disguised as a triangle result.

**Mandatory diagram topology:** Close the two-device comparator/inverting-integrator loop; RT and RF meet the comparator positive summing node. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `square_peak`, `triangle_resistance`, `feedback_resistance`, `integrator_resistance`, `integrator_capacitance`, `triangle_peak`, `triangle_pp`, `ramp_slew`, `period`, `frequency`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 018. `oscillators.wien_bridge_balance` — Wien-bridge oscillator loop balance

- **Required / level:** Must-have calculator + diagram + docs; basic.
- **Accountable:** D2Oscillators developer; T2Oscillators tester; V2Oscillators source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/oscillators.md#4-oscillatorswien_bridge_balance--basic); [complete JSON example vector](../../research/expansion/oscillators.json), selected by `id=oscillators.wien_bridge_balance`. Require every output at nominal relative tolerance `1e-09`. Numerical anchors: `frequency=1591.5494309189535` Hz; `feedback_fraction=0.3333333333333333` 1; `loop_excess=0.006666666666666667` 1.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Keep actual feedback topology, thresholds and startup conventions; device bandwidth, swing, drive level and amplitude stabilization remain explicit limits.

**Semantic novelty decision — retain as one distinct physical problem.**

Select a Wien oscillator frequency and check the amplifier's small-signal loop balance. WIEN §8.1 derives the equal-R/equal-C feedback factor of 1/3; §§3 and 5 explain startup and amplitude stabilization. This is neither the existing noninverting amplifier by itself nor an ordinary filter-response calculation: the bridge's zero-phase frequency and loop gain jointly determine oscillator feasibility. No output amplitude or THD is invented.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

RF=20000 gives amplifier_gain=3, loop_gain=1 and loop_excess=0 exactly, with other outputs unchanged. RF=19000 gives 2.9, 29/30 and −1/30 respectively; this must not be rejected. Double either bridge component and frequency halves. Changing gain-setting resistors together preserves gain but scales critical RF. Reject nonpositive components. Probe a representable RF just above 2RG: loop_excess must retain the positive resistor difference even if displayed loop_gain rounds to one.

**Mandatory diagram topology:** Noninverting RF/RG stage closes through series RC and shunt parallel RC; distinguish positive feedback and gain feedback. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `bridge_resistance`, `bridge_capacitance`, `gain_resistance`, `feedback_resistance`, `frequency`, `feedback_fraction`, `critical_feedback_resistance`, `amplifier_gain`, `loop_gain`, `loop_excess`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 019. `oscillators.loaded_rc_phase_shift` — Loaded three-section RC oscillator

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Oscillators developer; T2Oscillators tester; V2Oscillators source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/oscillators.md#5-oscillatorsloaded_rc_phase_shift--intermediate); [complete JSON example vector](../../research/expansion/oscillators.json), selected by `id=oscillators.loaded_rc_phase_shift`. Require every output at nominal relative tolerance `1e-09`. Numerical anchors: `frequency=649.7473343613968` Hz; `feedback_magnitude=0.034482758620689655` 1; `loop_excess=0.034482758620689655` 1.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Keep actual feedback topology, thresholds and startup conventions; device bandwidth, swing, drive level and amplitude stabilization remain explicit limits.

**Semantic novelty decision — retain as one distinct physical problem.**

Design a three-section, unbuffered, high-pass RC oscillator without making the false independent-section assumption. PHASE gives `f0=1/(2 pi RC sqrt(6))` and feedback magnitude 1/29. WIEN §8.2 explicitly demonstrates the large error caused by neglecting loading. Its displayed low-pass circuit and the buffered gain-8 formula are not copied into this high-pass model.

Closest baseline: `filters.buffered_rc_cascade` uses independent buffered low-pass sections driven by an external source. Here sections load each other, the feedback is high-pass and an inverting amplifier closes the oscillation loop. This is not a filter-frequency rename or a second Wien calculation: its feedback polynomial, phase mechanism and required gain differ.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

RF=290000 produces gain 29, loop_gain 1 and loop_excess 0; RF=280000 produces gain 28, loop_gain 28/29 and loop_excess −1/29. Both are valid diagnostic inputs. Doubling C halves frequency without changing any gain output. Doubling R and RF together halves frequency, doubles critical_feedback_resistance and preserves all gain ratios. Reject nonpositive components. Independently evaluate the complex ladder at omega=1/(RC sqrt(6)): real beta must be −1/29 and imaginary beta zero within tolerance; do not test using the buffered response.

**Mandatory diagram topology:** Three series capacitors, first two shunt R branches, third R to virtual ground; no extra third-node shunt. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `section_resistance`, `section_capacitance`, `feedback_resistance`, `frequency`, `feedback_magnitude`, `critical_feedback_resistance`, `amplifier_gain_magnitude`, `loop_gain`, `loop_excess`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 020. `oscillators.cross_coupled_lc_startup` — Cross-coupled LC oscillator startup poles

- **Required / level:** Must-have calculator + diagram + docs; expert.
- **Accountable:** D2Oscillators developer; T2Oscillators tester; V2Oscillators source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/oscillators.md#6-oscillatorscross_coupled_lc_startup--expert); [complete JSON example vector](../../research/expansion/oscillators.json), selected by `id=oscillators.cross_coupled_lc_startup`. Require every output at nominal relative tolerance `1e-09`. Numerical anchors: `tank_frequency=503292121.0448704` Hz; `negative_resistance=-1000` ohm; `startup_frequency=503276392.9203319` Hz.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Keep actual feedback topology, thresholds and startup conventions; device bandwidth, swing, drive level and amplitude stabilization remain explicit limits.

**Semantic novelty decision — retain as one distinct physical problem.**

Compare a cross-coupled pair's active negative conductance with differential tank losses and predict the startup pole pair. LC Figure 2(b) establishes −2/gm between the two differential terminals; the article describes its oscillator use. The following pole expression is independently derived from KCL, not claimed as a quoted source equation.

Closest old models: `ac.series_resonator_bandwidth` and `ac.lossy_coil_antiresonance` describe passive externally driven resonators. This addition includes the active pair, an oscillation threshold, signed envelope growth and the frequency of growing/decaying natural oscillation. Its supporting tank frequency is not separately counted.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

gm=0.001 S gives negative_resistance=−2000 ohm, startup_ratio=1, growth_rate=0, and startup_frequency=tank_frequency. gm=0.0005 S gives −4000 ohm, ratio=0.5 and growth_rate=−12500000 1/s; it is valid but decays. With all inputs `{L:1,C:1,Rp:1,gm:6}`, the complex-pole boundary is exactly reached and must reject; gm=5 is inside it, and gm=7 outside. Use the actual catalog keys when submitting these vectors. Reject nonpositive components or gm. Do not confuse half-circuit gm Rp criteria with this explicitly differential convention.

**Mandatory diagram topology:** Cross-coupled NMOS pair drives differential L/C/Rp; joined-source tail, opposite-drain gate connections and nonjoining crossings. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `tank_inductance`, `tank_capacitance`, `parallel_loss_resistance`, `transistor_transconductance`, `tank_frequency`, `negative_resistance`, `critical_transconductance`, `startup_ratio`, `growth_rate`, `startup_frequency`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 021. `oscillators.pierce_crystal_startup` — Pierce crystal load and startup margin

- **Required / level:** Must-have calculator + diagram + docs; expert.
- **Accountable:** D2Oscillators developer; T2Oscillators tester; V2Oscillators source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/oscillators.md#7-oscillatorspierce_crystal_startup--expert); [complete JSON example vector](../../research/expansion/oscillators.json), selected by `id=oscillators.pierce_crystal_startup`. Require every output at nominal relative tolerance `1e-09`. Numerical anchors: `load_capacitor_each=2e-11` F; `critical_transconductance=0.0003638330966417581` S; `transconductance_headroom=0.0031808345167912092` S.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Keep actual feedback topology, thresholds and startup conventions; device bandwidth, swing, drive level and amplitude stabilization remain explicit limits.

**Semantic novelty decision — retain as one distinct physical problem.**

Choose equal external load capacitors and assess whether the MCU/inverter can overcome the specified crystal loss with the requested startup margin. XTAL §§3.3–3.4 directly gives load capacitance and critical transconductance for equal capacitors, and distinguishes actual gm from maximum supported critical crystal transconductance.

Closest old model: `ac.lossy_coil_antiresonance` concerns a lossy passive coil, not a piezoelectric crystal, shunt capacitance and an inverter loop. Adjacent LC startup concerns a cross-coupled differential negative shunt conductance; this Pierce model concerns an inverter, capacitive loading and crystal ESR. Load sizing and startup ratio are one design problem and one item. No standalone crystal resonance or inverse capacitor calculator is counted.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

Cs=0 gives each external C=24 pF with every gm output unchanged, because nominal CL stays fixed. C0=0 is valid and changes gmcrit by factor `(12/15)^2=0.64`. Doubling ESR doubles gmcrit and required gm, and halves gain_margin. With available gm=0.001 S, gain_margin≈2.7485130111311244 and headroom≈−0.0008191654832087906 S: valid failed-margin result. Set available gm equal to independently computed design gm and check near-zero headroom with a justified absolute tolerance. Reject Cs=CL, Cs>CL, negative C0/Cs, nonpositive f/ESR/CL/gm and margin_target<1. When Cs approaches CL from below, external C approaches zero; parasitic uncertainty then dominates physical accuracy.

**Mandatory diagram topology:** Crystal and bias resistor separately bridge inverter input/output; equal load capacitors independently connect each node to ground. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `crystal_frequency`, `crystal_esr`, `shunt_capacitance`, `load_capacitance`, `stray_capacitance`, `available_transconductance`, `margin_target`, `load_capacitor_each`, `critical_transconductance`, `gain_margin`, `design_transconductance`, `transconductance_headroom`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

## Rf — 7 required items

Lane accountability: **D2Rf / T2Rf / V2Rf**. Levels: 2 basic, 3 intermediate, 2 expert.

### 022. `rf.matched_t_attenuator` — Symmetric matched T attenuator

- **Required / level:** Must-have calculator + diagram + docs; basic.
- **Accountable:** D2Rf developer; T2Rf tester; V2Rf source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/rf.md#1-rfmatched_t_attenuator); [complete JSON example vector](../../research/expansion/rf.json), selected by `id=rf.matched_t_attenuator`. Require every output at nominal relative tolerance `1e-09`. Numerical anchors: `series_resistance=40.90909090909091` ohm; `shunt_resistance=10.1010101010101` ohm; `power_transmission=0.01` 1.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use applicable impedance, far-field geometry, efficiency, RCS or characterized linear-output data; estimates are not propagation guarantees.

**Semantic novelty decision — retain as one distinct physical problem.**

Design a matched attenuator rather than solve a loaded divider. The nearest baseline is `dc.loaded_divider`, which computes voltage under one finite load but does not synthesize a two-port with simultaneous matching. `noise.decibel_to_linear` only converts ratios; `signals.source_series_termination` absorbs a traveling-wave boundary mismatch with one series element. This three-resistor dissipative synthesis is distinct from all three and from the lossless L match below. S1 provides the topology and equations.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Alternate: `Z0=50`, `A=6.020599913279624 dB` gives `K=2`, `Rs=16.666666666666668 ohm`, `Rp=66.66666666666667 ohm`, and `power_transmission=0.25`.
- Small-loss conditioning: `A=1e-12 dB`, `Z0=50`; `Rs approximately 2.878231366242557e-12 ohm` and `Rp approximately 4.342944819032518e14 ohm`. Neither resistor may collapse to a false zero/infinity through `exp(x)-1` cancellation.
- Doubling `Z0` doubles both resistor outputs and leaves power transmission unchanged.
- Reject `A=0`, negative attenuation, zero/negative impedance and nonfinite values. Real resistor parasitics and ratings remain external checks, not hidden guarantees.

**Mandatory diagram topology:** Two series resistors and central shunt form a matched T; output load shares return and reference impedance. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `reference_impedance`, `attenuation_db`, `series_resistance`, `shunt_resistance`, `power_transmission`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 023. `rf.lowpass_l_match` — Lumped low-pass L impedance match

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Rf developer; T2Rf tester; V2Rf source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/rf.md#2-rflowpass_l_match); [complete JSON example vector](../../research/expansion/rf.json), selected by `id=rf.lowpass_l_match`. Require every output at nominal relative tolerance `1e-09`. Numerical anchors: `loaded_q=1.7320508075688772` 1; `series_inductance=1.3783222385544804e-07` H; `shunt_capacitance=1.3783222385544801e-11` F.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use applicable impedance, far-field geometry, efficiency, RCS or characterized linear-output data; estimates are not propagation guarantees.

**Semantic novelty decision — retain as one distinct physical problem.**

The nearest baseline `signals.quarter_wave_transformer` uses distributed phase propagation and a transmission-line section. This model instead synthesizes two **lumped** reactances whose parallel/series transformation cancels at the design frequency. `ac.series_rlc_impedance` evaluates a supplied circuit rather than synthesize a conjugate match. The new model is not a re-labeled quarter-wave section or a generic resonance calculation. S2, section 6.4.1, derives this exact orientation.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- `Rs=50`, `Rl=100`, `f=1e8` gives Q=1, L=`7.957747154594767e-8 H`, C=`1.5915494309189536e-11 F`.
- Doubling frequency halves L and C without changing Q.
- As `Rl -> Rs+`, all three outputs approach zero. Use a representably adjacent `Rl > Rs` to check `(Rl-Rs)/Rs`, not cancellation in `Rl/Rs-1`.
- Reject `Rl=Rs`, `Rl<Rs`, nonpositive resistances/frequency and nonfinite values. Do not impose a minimum nonzero Q merely to simplify testing.

**Mandatory diagram topology:** Source resistor feeds series L; shunt C parallels the higher load resistance at the output node. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `source_resistance`, `load_resistance`, `frequency`, `loaded_q`, `series_inductance`, `shunt_capacitance`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 024. `rf.free_space_link_budget` — Free-space radio link budget

- **Required / level:** Must-have calculator + diagram + docs; basic.
- **Accountable:** D2Rf developer; T2Rf tester; V2Rf source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/rf.md#3-rffree_space_link_budget); [complete JSON example vector](../../research/expansion/rf.json), selected by `id=rf.free_space_link_budget`. Require every output at nominal relative tolerance `1e-09`. Numerical anchors: `free_space_loss_db=100.0520080561155` dB; `received_power_dbm=-78.0520080561155` dBm; `link_margin_db=11.947991943884503` dB.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use applicable impedance, far-field geometry, efficiency, RCS or characterized linear-output data; estimates are not propagation guarantees.

**Semantic novelty decision — retain as one distinct physical problem.**

The nearest baseline `noise.friis_three_stage` concerns amplifier **noise factors**, not electromagnetic propagation; the common historical surname is not the same physical equation. `signals.line_propagation` predicts guided-line delay and wavelength, not antenna spatial spreading. This item calculates received radio power and sensitivity headroom. S3 supplies free-space loss; S7's radiation density and effective aperture supply the directional-gain extension.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Keep all nominal values but double distance: loss=`106.07260796939512 dB`, receive=`-84.07260796939512 dBm`, margin=`5.927392030604879 dB`. This is the independent inverse-square law.
- Increase either gain by 3 dB: receive and margin rise 3 dB; path loss does not change.
- Set sensitivity equal to the calculated received power: zero margin is valid. A higher sensitivity threshold produces a valid negative margin.
- `total_loss_db=0` is valid. Negative directional dBi is valid and must not be clamped.
- Test exact far-field-screen equality and one representable step below; reject only the latter. Reject nonpositive frequency/distance/dimension, negative loss and nonfinite inputs.

**Mandatory diagram topology:** Distinct TX/RX antennas, one-way spreading arrow and distance; annotate gain, loss, received power and sensitivity margin. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `frequency`, `distance`, `antenna_max_dimension`, `transmit_power_dbm`, `transmit_gain_dbi`, `receive_gain_dbi`, `total_loss_db`, `sensitivity_dbm`, `free_space_loss_db`, `received_power_dbm`, `link_margin_db`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 025. `rf.first_fresnel_clearance` — First-Fresnel obstruction clearance

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Rf developer; T2Rf tester; V2Rf source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/rf.md#4-rffirst_fresnel_clearance); [complete JSON example vector](../../research/expansion/rf.json), selected by `id=rf.first_fresnel_clearance`. Require every output at nominal relative tolerance `1e-09`. Numerical anchors: `zone_radius=5.475330656681841` m; `required_clearance=3.2851983940091043` m; `clearance_margin=0.7148016059908957` m.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use applicable impedance, far-field geometry, efficiency, RCS or characterized linear-output data; estimates are not propagation guarantees.

**Semantic novelty decision — retain as one distinct physical problem.**

The nearest neighbor is the free-space link budget, but this item solves **path-interference clearance geometry**, not power spreading or an inverse range budget. No baseline model computes an ellipsoidal Fresnel region. `signals.line_propagation` supplies only wavelength/delay. S3 section 3 provides the radius expression and warns about points close to either endpoint.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Swap d1 and d2: every output remains unchanged.
- Hold total length 1000 m and use d1=d2=500 m: radius=`5.588235950786855 m`, the maximum along this link.
- Quadruple frequency: radius and required clearance halve; changing obstacle clearance alone changes only margin.
- `clearance_fraction=0` yields zero requirement and margin equal to available clearance. Fraction=1 requests the full first zone. Both are valid.
- Available clearance zero or negative is valid and shows obstruction. Screen equality is valid; one endpoint below 100 wavelengths is rejected.
- Reject nonpositive frequency/endpoint distances, fraction outside [0,1] and nonfinite values. A point at an antenna endpoint is outside this approximation, not silently reported as a zero-radius zone.

**Mandatory diagram topology:** TX/RX line of sight, Fresnel envelope and obstacle plane; separate zone radius, available clearance and required clearance. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `frequency`, `distance_from_tx`, `distance_to_rx`, `clearance_fraction`, `available_clearance`, `zone_radius`, `required_clearance`, `clearance_margin`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 026. `rf.circular_aperture_gain` — Circular-aperture antenna gain

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Rf developer; T2Rf tester; V2Rf source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/rf.md#5-rfcircular_aperture_gain); [complete JSON example vector](../../research/expansion/rf.json), selected by `id=rf.circular_aperture_gain`. Require every output at nominal relative tolerance `1e-09`. Numerical anchors: `effective_aperture=0.47123889803846897` m^2; `gain_linear=6588.849534059469` 1; `gain_dbi=38.188095899160565` dBi.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use applicable impedance, far-field geometry, efficiency, RCS or characterized linear-output data; estimates are not propagation guarantees.

**Semantic novelty decision — retain as one distinct physical problem.**

This model turns a physical aperture and efficiency into directional gain and collecting area. The baseline has no antenna-aperture synthesis. Its nearest neighbor `rf.free_space_link_budget` accepts gain and predicts propagation; it does not derive gain from geometry. `signals.coax_impedance` describes guided fields and is unrelated. S4 provides the circular-aperture gain relation; S7 provides the gain/effective-area identity.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Double diameter: effective area and linear gain quadruple; gain becomes `44.20869581244019 dBi`.
- Double frequency at fixed diameter/efficiency: area is unchanged and gain quadruples. This assumes the supplied efficiency remains applicable.
- Efficiency=1 is accepted. Halving efficiency halves both area and gain, decreasing dBi by `3.010299956639812 dB`.
- `D=2lambda` is excluded; a representably larger diameter is accepted. Reject efficiency<=0, efficiency>1, nonpositive diameter/frequency and nonfinite inputs.
- As efficiency approaches zero from above, linear area/gain approach zero and logarithmic gain falls without an arbitrary dBi clamp.

**Mandatory diagram topology:** Circular physical aperture receives boresight waves; distinguish physical diameter from effective collecting area and efficiency. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `diameter`, `frequency`, `aperture_efficiency`, `effective_aperture`, `gain_linear`, `gain_dbi`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 027. `rf.two_tone_im3` — Equal-tone third-order intermodulation

- **Required / level:** Must-have calculator + diagram + docs; expert.
- **Accountable:** D2Rf developer; T2Rf tester; V2Rf source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/rf.md#6-rftwo_tone_im3); [complete JSON example vector](../../research/expansion/rf.json), selected by `id=rf.two_tone_im3`. Require every output at nominal relative tolerance `1e-09`. Numerical anchors: `lower_product_frequency=99000000.0` Hz; `upper_product_frequency=102000000.0` Hz; `total_fundamental_power_dbm=3.010299956639812` dBm.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use applicable impedance, far-field geometry, efficiency, RCS or characterized linear-output data; estimates are not propagation guarantees.

**Semantic novelty decision — retain as one distinct physical problem.**

This is a nonlinear RF spectral model absent from the baseline. `noise.analog_dynamic_range` compares signal and random noise floors; it does not create deterministic in-band spurs. `data.dynamic_enob` interprets measured SINAD but does not predict amplifier cubic mixing. S5 directly states frequency products, slopes and intercept equations; S6 establishes power-reference-plane practice.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Increase per-tone input by 1 dB to -19 dBm: each fundamental becomes 1 dBm, each IM3 becomes -57 dBm, separation becomes 58 dB, total becomes `4.010299956639812 dBm`. Frequencies and OIP3 do not change.
- Increase IIP3 by 3 dB while holding all other inputs: each IM3 drops 6 dB; fundamentals stay fixed.
- Exact characterized-limit equality is accepted. An input increment that puts summed fundamental power above that limit is rejected, even if each tone separately remains below it.
- Reject equal/reversed tones, f2>=2f1, nonpositive frequencies, Pin>=IIP3 and nonfinite values. Negative dBm intercepts and negative gain are not intrinsically invalid if all relational domains pass.
- Very low Pin produces small but finite logarithmic product power. No arbitrary noise-floor clamp is allowed; the model does not claim the spur can be measured below instrument noise.

**Mandatory diagram topology:** Two tones enter amplifier; static spectrum has inner fundamentals and outer IM3 stems with per-tone and total-power distinction. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `lower_frequency`, `upper_frequency`, `input_tone_power_dbm`, `gain_db`, `input_intercept_dbm`, `maximum_total_linear_output_dbm`, `lower_product_frequency`, `upper_product_frequency`, `output_tone_power_dbm`, `output_intercept_dbm`, `product_power_dbm`, `carrier_to_product_db`, `total_fundamental_power_dbm`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 028. `rf.monostatic_radar_return` — Monostatic point-target radar return

- **Required / level:** Must-have calculator + diagram + docs; expert.
- **Accountable:** D2Rf developer; T2Rf tester; V2Rf source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/rf.md#7-rfmonostatic_radar_return); [complete JSON example vector](../../research/expansion/rf.json), selected by `id=rf.monostatic_radar_return`. Require every output at nominal relative tolerance `1e-09`. Numerical anchors: `received_power_dbm=-96.43988186210434` dBm; `echo_margin_db=3.560118137895657` dB.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use applicable impedance, far-field geometry, efficiency, RCS or characterized linear-output data; estimates are not propagation guarantees.

**Semantic novelty decision — retain as one distinct physical problem.**

The nearest new neighbor is `rf.free_space_link_budget`. A one-way communications link has an active transmitter and a receiving antenna at the opposite endpoint, giving inverse-square range loss. This model instead has a **passive scattering target** described by RCS and the same antenna on transmit and receive. Outbound and return spreading create inverse-fourth-power range dependence. It is not the communications equation inverted or given extra outputs. No baseline model includes target scattering. S7 slides 5–10 explicitly derive these stages.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Double range: echo power divides by 16, received=`-108.4810816886636 dBm`, margin=`-8.481081688663592 dB`. The negative margin is a valid result.
- Double RCS: echo rises `3.010299956639812 dB`. Increase the common antenna gain 1 dB: echo rises 2 dB because gain acts on both paths.
- Add 1 dB to total loss: echo and margin each fall 1 dB, not 2 dB.
- Zero total loss is accepted. Threshold equal to computed echo gives zero margin. Arbitrarily small positive RCS remains logarithmically meaningful without pretending zero RCS has finite dBm.
- Accept exact far-field-screen equality; reject smaller range. Increasing the target dimension can invalidate a previously accepted range without changing nominal radar gain or RCS.
- Reject zero/negative RCS, frequency, range or dimensions; reject negative total loss and nonfinite values. Poorly characterized RCS cannot be repaired by a numeric input clamp.

**Mandatory diagram topology:** One radar and passive target; separate outgoing/return arrows share one-way range and show target RCS. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `frequency`, `range`, `antenna_max_dimension`, `target_max_dimension`, `transmit_power_dbm`, `antenna_gain_dbi`, `radar_cross_section`, `total_loss_db`, `receiver_threshold_dbm`, `received_power_dbm`, `echo_margin_db`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

## Batteries — 6 required items

Lane accountability: **D2Batteries / T2Batteries / V2Batteries**. Levels: 2 basic, 1 intermediate, 3 expert.

### 029. `batteries.series_weakest_cell` — Three-series pack usable charge and stranded state

- **Required / level:** Must-have calculator + diagram + docs; basic.
- **Accountable:** D2Batteries developer; T2Batteries tester; V2Batteries source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/batteries.md#1-batteriesseries_weakest_cell--basic); [complete JSON example vector](../../research/expansion/batteries.json), selected by `id=batteries.series_weakest_cell`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `usable_charge=2.1` Ah; `end_soc_one=0.2` 1; `end_soc_three=0.29375` 1.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Supply cell/chemistry calibration and consistent charge/state units; fit, phase-stability and transport assumptions remain visible.

**Semantic novelty decision — retain as one distinct physical problem.**

Find usable charge before the first cell reaches the chosen discharge floor in an unbalanced three-series string. Also report each cell's remaining SOC at that instant. This answers a pack-design/balancing question: why a nominally large cell can strand charge when another cell limits the string.

Nearest baseline: `dc.series_parallel_branches` solves resistive currents; it has no capacity, SOC or electrochemical cutoff. Nearest new neighbor: `batteries.coulomb_state_cycle` updates one cell through time; this model instead solves simultaneous series-cell feasibility by the minimum remaining charge. Neither is a voltage, energy or current rearrangement.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

Nominal: assert all four values in the linked research vector. Limiting: with all capacities `3 Ah`, all SOC `.9` and floor `.1`, output `2.4 Ah` and three `.1` states; tied limiting cells must work. With `soc_two=.1` and other nominal inputs, usable charge is zero and all final SOCs equal initial SOCs. Permuting the complete cell tuples must preserve usable charge and permute state outputs. Invalid: capacity zero/negative, SOC outside `[0,1]`, or any initial SOC below floor. Do not clamp invalid inputs.

**Mandatory diagram topology:** Three explicitly polarized series cells share one charge-used arrow; initial/final SOC and discharge floor remain per-cell. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `capacity_one`, `capacity_two`, `capacity_three`, `soc_one`, `soc_two`, `soc_three`, `soc_floor`, `usable_charge`, `end_soc_one`, `end_soc_two`, `end_soc_three`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 030. `batteries.coulomb_state_cycle` — Coulomb-counted discharge and recharge state

- **Required / level:** Must-have calculator + diagram + docs; basic.
- **Accountable:** D2Batteries developer; T2Batteries tester; V2Batteries source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/batteries.md#2-batteriescoulomb_state_cycle--basic); [complete JSON example vector](../../research/expansion/batteries.json), selected by `id=batteries.coulomb_state_cycle`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `discharged_soc=0.3` 1; `stored_charge=1.9` Ah; `final_soc=0.68` 1.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Supply cell/chemistry calibration and consistent charge/state units; fit, phase-stability and transport assumptions remain visible.

**Semantic novelty decision — retain as one distinct physical problem.**

Track discharge followed by recharge while separating terminal charge throughput from chemically retained Ah. This is useful for a battery gauge state budget. Nearest baseline is `transients.capacitor_constant_current`, which predicts an electrostatic voltage ramp and energy; this model predicts normalized chemical state and irreversibly lost recharge coulombs. `power.converter_input_budget` uses power efficiency and cannot replace coulombic efficiency.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

Nominal: assert all three outputs. Zero both durations: discharged and final SOC `.4`, stored charge zero. Unity efficiency with nominal durations: `.3`, `2 Ah`, `.7`. Exact full-charge endpoint: `full_capacity=1`, `initial_soc=.5`, zero discharge current/time, `charge_current=.5`, `charge_time=3600`, efficiency one gives final one. Invalid: discharge nominal battery for `10000 s` at 1 A, or charge nominal case for `10000 s` at 2 A; reject intermediate exhaustion even if a later recharge would restore SOC. Also reject invalid capacity, efficiency and fraction ranges.

**Mandatory diagram topology:** Initial state flows through discharge then recharge to final state; show unretained recharge charge separately. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `full_capacity`, `initial_soc`, `discharge_current`, `discharge_time`, `charge_current`, `charge_time`, `charge_efficiency`, `discharged_soc`, `stored_charge`, `final_soc`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 031. `batteries.peukert_runtime` — Peukert rate-dependent discharge runtime

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Batteries developer; T2Batteries tester; V2Batteries source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/batteries.md#3-batteriespeukert_runtime--intermediate); [complete JSON example vector](../../research/expansion/batteries.json), selected by `id=batteries.peukert_runtime`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `rated_current=5` A; `runtime=3.5355339059327378` h; `delivered_capacity=70.71067811865476` Ah.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Supply cell/chemistry calibration and consistent charge/state units; fit, phase-stability and transport assumptions remain visible.

**Semantic novelty decision — retain as one distinct physical problem.**

Estimate rate-capacity loss under a constant-current discharge using a calibrated empirical exponent. This is not ideal `Ah/A` runtime except at the explicit ideal limit `n=1`. No baseline models battery rate dependence. The nearest new model, KiBaM, instead evolves two internal availability states and represents rest recovery; Peukert has no state history or recovery mechanism.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

Nominal: assert all outputs. At `load_current=5 A` with other nominal inputs, runtime is `20 h` and delivered capacity `100 Ah` for every allowed exponent. At `peukert_exponent=1`, nominal current gives `5 h`, `100 Ah`. Double load current: runtime scales by `2^-n` and delivered capacity by `2^(1-n)`. Invalid: zero current, zero rated duration/capacity, exponent below one and nonfinite values. Do not add a zero-current fake infinite lifetime. Low-current extrapolation is documented as physically unsupported, not silently clamped.

**Mandatory diagram topology:** Illustrative log-log current/runtime curve with slope −n, ideal −1 reference and rated/requested operating points. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `rated_capacity`, `rated_duration`, `load_current`, `peukert_exponent`, `rated_current`, `runtime`, `delivered_capacity`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 032. `batteries.kinetic_charge_recovery` — Kinetic available and bound charge after a pulse

- **Required / level:** Must-have calculator + diagram + docs; expert.
- **Accountable:** D2Batteries developer; T2Batteries tester; V2Batteries source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/batteries.md#4-batterieskinetic_charge_recovery--expert); [complete JSON example vector](../../research/expansion/batteries.json), selected by `id=batteries.kinetic_charge_recovery`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `available_final=2068.8116360940267` C; `bound_final=4731.188363905973` C; `total_final=6800` C.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Supply cell/chemistry calibration and consistent charge/state units; fit, phase-stability and transport assumptions remain visible.

**Semantic novelty decision — retain as one distinct physical problem.**

Predict chemically available and temporarily bound charge after a current pulse or rest. This supplies state for duty-cycle battery studies without pretending to simulate full electrochemistry. The model is distinct from Peukert: stored charge remains conserved during rest while accessible charge recovers. It is distinct from `transients.capacitor_charge_sharing`: wells represent empirical reaction availability, do not have electrical voltages/capacitances, and the load has an available-charge exhaustion condition.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

Nominal: assert all three values. With only `pulse_current=0`, expect `available_final=3090.2376727811948 C`, `bound_final=4909.762327218806 C`, total `8000 C`; accessible charge increases without new stored charge. With only `equalization_rate=0`, expect `1800 C`, `5000 C`, `6800 C`. Zero duration returns `3000`, `5000`, `8000 C`. Equilibrated wells with zero current remain unchanged for all durations/rates. Test tiny positive `lambda*t` without substituting `1-exp` or `1-exprel`; the stable expressions must preserve small transfers. Invalid: `c=0` or `1`, a well above capacity, reversed initial height order, negative load/rate/time, or nominal state with `pulse_current=10 A` for `600 s` (available well exhausts).

**Mandatory diagram topology:** Available and bound charge wells exchange through a passage; load drains only available well, not both. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `maximum_charge`, `available_fraction`, `available_initial`, `bound_initial`, `equalization_rate`, `pulse_current`, `pulse_duration`, `available_final`, `bound_final`, `total_final`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 033. `batteries.intercalation_regular_solution` — Single-phase intercalation voltage and chemical capacitance

- **Required / level:** Must-have calculator + diagram + docs; expert.
- **Accountable:** D2Batteries developer; T2Batteries tester; V2Batteries source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/batteries.md#5-batteriesintercalation_regular_solution--expert); [complete JSON example vector](../../research/expansion/batteries.json), selected by `id=batteries.intercalation_regular_solution`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `equilibrium_voltage=3.5093321437299303` V; `voltage_slope=-0.06015999692680433` V; `chemical_capacitance=179521.28576635703` F.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Supply cell/chemistry calibration and consistent charge/state units; fit, phase-stability and transport assumptions remain visible.

**Semantic novelty decision — retain as one distinct physical problem.**

Estimate an insertion electrode's equilibrium potential and local differential charge capacity, including pair interactions among occupied and vacant sites. The closest baseline is `diodes.shockley_forward_current`, an exponential carrier-transport law; this is a lattice free-energy equilibrium law. The closest new sensor is hydrogen-activity Nernst transduction, but the battery model adds occupancy/vacancy entropy, pair interaction energy, global phase stability and a charge-versus-voltage derivative. The sensor researcher confirmed this boundary.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

Nominal: assert all outputs. At `x=.5`, voltage equals `midpoint_voltage` exactly; slope is `-(4RT-2Omega)/F` and capacitance `Qmax F/(4RT-2Omega)`. At `Omega=0`, recover the ideal lattice entropy model. For complementary fillings `.3` and `.7`, voltage deviations from midpoint reverse sign; slope and capacitance match. Negative Omega is valid and lowers differential capacitance. As Omega approaches `2RT` from below at half filling, capacitance diverges; reject equality and above rather than clamping. Reject zero/one occupancy, nonpositive temperature/charge. Positive finite physical values close to endpoints remain mathematically admissible; nonfinite arithmetic is an engine error, not grounds for an invented cutoff.

**Mandatory diagram topology:** Occupied/vacant insertion lattice with distinct ion/electron arrows; pair-interaction and voltage/capacity annotations. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `midpoint_voltage`, `temperature`, `interaction_energy`, `filling_fraction`, `full_insertion_charge`, `equilibrium_voltage`, `voltage_slope`, `chemical_capacitance`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 034. `batteries.sand_depletion_limit` — Sand electrolyte-depletion charging limit

- **Required / level:** Must-have calculator + diagram + docs; expert.
- **Accountable:** D2Batteries developer; T2Batteries tester; V2Batteries source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/batteries.md#6-batteriessand_depletion_limit--expert); [complete JSON example vector](../../research/expansion/batteries.json), selected by `id=batteries.sand_depletion_limit`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `sand_time=203.10002310425608` s; `sand_areal_charge=20310.002310425607` C/m^2; `diffusion_length=0.0001425131653933264` m.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Supply cell/chemistry calibration and consistent charge/state units; fit, phase-stability and transport assumptions remain visible.

**Semantic novelty decision — retain as one distinct physical problem.**

Estimate an electrolyte transport limit during constant-current lithium plating: time for initially uniform electrolyte concentration at a planar consuming interface to reach zero. No baseline model describes electrochemical mass transport. It differs from KiBaM, which fits lumped accessible/bound charge; Sand time uses diffusivity, concentration and transference to predict local depletion. It is not fuse I²t or a protection trip rating.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

Nominal: assert all three outputs. Double current to `200 A/m²`: time divides by four, areal charge halves, diffusion length halves. Double concentration: time quadruples, areal charge quadruples, diffusion length doubles. Set transference to zero: time and areal charge are `.36` of nominal, diffusion length `.6` of nominal. As transference approaches one, predicted time and length grow without bound; finite geometry generally invalidates the physical approximation before the mathematical endpoint. Reject transference one, above one or below zero; reject zero/negative diffusivity, concentration or current. Do not produce a finite result for zero current or silently assign a special depletion time.

**Mandatory diagram topology:** Planar lithium plating interface, electrolyte concentration profile and diffusion depth; ionic and electronic paths stay distinct. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `salt_diffusivity`, `salt_concentration`, `plating_current_density`, `cation_transference`, `sand_time`, `sand_areal_charge`, `diffusion_length`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

## Protection — 6 required items

Lane accountability: **D2Protection / T2Protection / V2Protection**. Levels: 1 basic, 3 intermediate, 2 expert.

### 035. `protection.fuse_pulse_coordination` — Fuse pulse survival and semiconductor clearing budget

- **Required / level:** Must-have calculator + diagram + docs; basic.
- **Accountable:** D2Protection developer; T2Protection tester; V2Protection source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/protection.md#1-protectionfuse_pulse_coordination--basic); [complete JSON example vector](../../research/expansion/protection.json), selected by `id=protection.fuse_pulse_coordination`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `pulse_i2t=1.0666666666666667` A² s; `pulse_margin=0.5333333333333333` A² s; `clearing_utilization=0.675` 1.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use waveform/temperature-matched device data or the stated copper material constants; negative design margins are not safety approvals.

**Semantic novelty decision — retain as one distinct physical problem.**

Select a fuse that survives a known inrush yet clears before a specified semiconductor thermal integral is exceeded. Closest old model: `pcb.trace_joule_heat_density`, which multiplies fixed resistivity by current density and time to calculate deposited heat. This model instead integrates an explicitly nonrectangular current waveform and distinguishes fuse pre-arcing from arcing/total clearing. It is not a second energy unit or inverse. Pulse survival and semiconductor coordination are deliberately one item, not two counts.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: all six values in the linked research vector, not merely finite outputs.
- Rectangle: set both edge times to zero and `hold_time=0.002`; `pulse_i2t=0.8`, `pulse_margin=0.8`, utilization `0.5`; clearing outputs stay unchanged.
- Triangle: set hold time to zero, retain 1 ms edges; `pulse_i2t=0.2666666666666667`.
- Zero current: pulse integral/utilization zero; pulse margin `1.6`; fault clearing outputs unchanged.
- Failed design, still valid model: `device_withstand_i2t=2` yields clearing margin `−0.7`, utilization `1.35`.
- Reject zero total duration, total duration exactly 10 ms, any negative duration/current, zero pre-arcing/withstand, or allowance outside `(0,1]`.

**Mandatory diagram topology:** Trapezoidal current with separate edge/hold durations; distinct pre-arcing, clearing and device-withstand I²t budgets, never joules. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `peak_current`, `rise_time`, `hold_time`, `fall_time`, `pre_arcing_i2t`, `device_withstand_i2t`, `pulse_i2t`, `clearing_i2t`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 036. `protection.tvs_surge_load_line` — TVS avalanche surge load line

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Protection developer; T2Protection tester; V2Protection source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/protection.md#2-protectiontvs_surge_load_line--intermediate); [complete JSON example vector](../../research/expansion/protection.json), selected by `id=protection.tvs_surge_load_line`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `surge_current=28` A; `clamp_voltage=44` V; `current_margin=7` A.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use waveform/temperature-matched device data or the stated copper material constants; negative design margins are not safety approvals.

**Semantic novelty decision — retain as one distinct physical problem.**

Find actual clamp stress when source impedance and finite TVS avalanche slope interact. Closest baseline models are `diodes.biased_two_sided_clipper` and `diodes.zener_shunt_resistor_window`. Those use fixed clipping/shunt levels. Here the nonlinear-device avalanche branch is locally affine, so surge current changes clamp voltage and must be solved simultaneously with source impedance. This is a transient-rating/load-line design problem, not another zener resistor inverse. Neighbor MOV uses a multiplicative power law and imposed pulse current, not this affine source intersection.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal linked research vector; verify source KVL `100 = 2×28+44` independently.
- Ideal-slope limit `dynamic_resistance=0`: `I=35 A`, `V=30 V`, `P=1050 W`, voltage margin `30 V`, current margin zero.
- Fit endpoints: source `32.5 V` gives `1 A` and `30.5 V`; source `280 V` gives `100 A` and `80 V`. Both valid; the latter returns negative protection margins.
- Lower `protected_voltage_limit=40` gives `voltage_margin=−4`, not a domain error.
- Reject zero source resistance, negative slope, reversed/equal fit limits, or source levels producing current outside the fit interval. `surge_voltage=30` is off-avalanche and must reject rather than produce a fabricated clamp result.

**Mandatory diagram topology:** Source and series resistance feed cathode-positive TVS shunt; affine avalanche branch intersects descending source load line. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `surge_voltage`, `source_resistance`, `breakdown_intercept`, `dynamic_resistance`, `surge_current`, `clamp_voltage`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 037. `protection.mov_power_law_pulse` — MOV nonlinear pulse clamp and absorption

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Protection developer; T2Protection tester; V2Protection source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/protection.md#3-protectionmov_power_law_pulse--intermediate); [complete JSON example vector](../../research/expansion/protection.json), selected by `id=protection.mov_power_law_pulse`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `voltage_exponent=0.0484550065040282` 1; `current_exponent=20.637702317032343` 1; `energy_margin=0.6422291236000337` J.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use waveform/temperature-matched device data or the stated copper material constants; negative design margins are not safety approvals.

**Semantic novelty decision — retain as one distinct physical problem.**

Estimate a metal-oxide varistor's nonlinear protection-region clamp, incremental resistance and absorption using two measured/datasheet points. Unlike the TVS affine intercept-plus-slope model, grain-boundary conduction uses `I=K V^alpha` and has a current-dependent incremental resistance. Unlike `diodes.shockley_forward_current`, this is a bulk MOV empirical power law, not a forward PN-junction exponential or thermal-voltage calculation. Pulse absorption is a supporting output, not a separately counted energy calculator.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal linked research vector, including the geometric-mean identity.
- At `pulse_current=1`, voltage equals exactly the first fitted value `320 V`; at `100 A`, equals `400 V`.
- Double pulse duration: exponent, clamp and differential resistance unchanged; energy doubles.
- Set allowance below absorbed energy: return a negative margin, not a fabricated safe result.
- Reject equal/reversed currents, equal/decreasing voltages, `beta>=1`, pulse current outside either endpoint, zero pulse duration or nonpositive allowance.
- Close-point oracle: choose `I2=2`, `V2=320×2^0.05`; fitted beta is `0.05`. The tester should calculate this independently at high precision rather than snapshot implementation output.

**Mandatory diagram topology:** Imposed rectangular current pulse drives MOV; logarithmic fit shows both endpoints, clamp and absorbed energy. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `current_one`, `voltage_one`, `current_two`, `voltage_two`, `pulse_current`, `pulse_duration`, `voltage_exponent`, `clamp_voltage`, `absorbed_energy`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 038. `protection.scr_crowbar_trigger_latch` — Zener-triggered SCR crowbar latch window

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Protection developer; T2Protection tester; V2Protection source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/protection.md#4-protectionscr_crowbar_trigger_latch--intermediate); [complete JSON example vector](../../research/expansion/protection.json), selected by `id=protection.scr_crowbar_trigger_latch`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `trigger_source_voltage=13.438` V; `available_gate_current=0.04647272727272727` A; `holding_release_voltage=1.4` V.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use waveform/temperature-matched device data or the stated copper material constants; negative design margins are not safety approvals.

**Semantic novelty decision — retain as one distinct physical problem.**

Design a zener-triggered SCR crowbar with the correct gate-shunt/source loading and distinguish triggering, latching and holding. Closest baseline `bjt.saturated_switch_drive` computes sustained BJT base drive but has no regenerative latch or distinct holding boundary. Closest old zener sizing does not model a state transition that collapses its own trigger voltage. This model is not an SCR current-gain substitution for a BJT formula.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal linked research vector; source resistance must contribute to trigger loading, not only anode current.
- Set fault source exactly `13.438 V`: gate current is `0.005 A`, available anode current `1.2238 A`, latch margin `1.1738 A`, release boundary unchanged.
- Set `latching_current=2 A`, keeping holding at `0.02 A`: trigger remains valid, latch margin `−0.32 A`; no assertion that it latches.
- Holding equal to latching is accepted as the model's limiting case. Setting holding above latching rejects.
- Reject source below the computed trigger requirement, zero source/gate/shunt resistance, nonpositive gate requirement, or `on_voltage>=zener_voltage`.
- Larger Rgk reduces wasted shunt current; the finite value must remain part of the gate-current equation.

**Mandatory diagram topology:** Source/Rs feeds SCR anode and Rg/zener gate branch; gate-shunt resistor returns to cathode; preserve distinct A/K/G pins. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `zener_voltage`, `gate_resistance`, `gate_shunt_resistance`, `fault_voltage`, `source_resistance`, `latching_current`, `holding_current`, `available_gate_current`, `available_anode_current`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 039. `protection.copper_adiabatic_fault_limit` — Copper conductor temperature-limited fault withstand

- **Required / level:** Must-have calculator + diagram + docs; expert.
- **Accountable:** D2Protection developer; T2Protection tester; V2Protection source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/protection.md#5-protectioncopper_adiabatic_fault_limit--expert); [complete JSON example vector](../../research/expansion/protection.json), selected by `id=protection.copper_adiabatic_fault_limit`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `material_k=143.1744142046674` A sqrt(s)/mm²; `allowable_fault_current=809.9167934122551` A; `area_margin_mm2=1.5306105315166842` mm².
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use waveform/temperature-matched device data or the stated copper material constants; negative design margins are not safety approvals.

**Semantic novelty decision — retain as one distinct physical problem.**

Derive a conductor's fault withstand from temperature-dependent copper heating and a permitted thermal endpoint. Closest baseline `pcb.trace_joule_heat_density` uses constant resistivity to report heat, not a material-temperature integral or insulation endpoint. Closest fuse item consumes supplied device I²t values, whereas this model derives the conductor limit from material behavior and initial/final temperatures. Cables confirmed no overlap.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal complete linked research vector; verify `Iallow² t=(kS)²` as an independent physical invariant.
- Change clearing time to `2 s`: allowable current halves, required area doubles, k and allowable integral stay unchanged.
- Zero fault current: minimum area zero, area margin equals installed `4 mm²`; withstand outputs remain nonzero.
- Initial and final temperatures approaching each other from below: k and withstand tend to zero, required area grows. A tiny positive temperature difference must not cancel in the logarithm.
- The 5 s endpoint and 0 °C initial endpoint are admitted. Reject longer intervals, zero/negative time, zero area, equal/inverted temperatures, subzero initial temperature or final temperature above 250 °C.
- A high specified fault current may give a negative area margin; keep that result rather than reject a physically meaningful failed design.

**Mandatory diagram topology:** Copper cross-section with fault-current/area labels; initial-to-limit thermal history, clearing interval and no-heat-escape annotation. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `area_mm2`, `initial_temperature`, `limit_temperature`, `clearing_time`, `fault_current`, `material_k`, `minimum_area_mm2`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 040. `protection.foldback_fault_envelope` — Foldback limiter fault equilibrium and peak pass stress

- **Required / level:** Must-have calculator + diagram + docs; expert.
- **Accountable:** D2Protection developer; T2Protection tester; V2Protection source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/protection.md#6-protectionfoldback_fault_envelope--expert); [complete JSON example vector](../../research/expansion/protection.json), selected by `id=protection.foldback_fault_envelope`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `foldback_slope=0.16` A/V; `knee_load_resistance=5` ohm; `maximum_pass_power=4.2025` W.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use waveform/temperature-matched device data or the stated copper material constants; negative design margins are not safety approvals.

**Semantic novelty decision — retain as one distinct physical problem.**

Find a resistive fault's operating point on a characterized foldback branch, then locate worst pass-element dissipation over the entire branch. A short circuit need not be the worst heating condition. Closest baseline `power.ldo_dissipation` takes fixed voltage/current as inputs and merely budgets loss; this model derives current from a nonlinear protection characteristic and optimizes stress. Closest `dc.resistive_source_matching` is a passive Thevenin source with a decreasing I–V line, whereas foldback has an output-dependent increasing current limit and a distinct regulation knee. It is not a renamed maximum-power-transfer calculation.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal complete vector; verify `Vfault=R Ifault`, the affine current relation and the maximum exceeding both endpoint powers.
- Hard short `R=0`: fault voltage zero, current `0.2 A`, power `1.8 W`; optimized envelope remains unchanged.
- Knee `R=5`: fault voltage `5 V`, current `1 A`, power `4 W`.
- Upper endpoint maximum: change Vin to `12 V`; unconstrained vertex is `5.375 V`, so reported peak voltage is `5 V`, peak current `1 A`, maximum `7 W`.
- Lower endpoint maximum: keep Vin=9, Vk=5, Ik=1 but set Isc=`0.9 A`; b=`0.02 A/V`, vertex negative, peak voltage zero and maximum power `8.1 W`.
- Reject equal/reversed knee and short currents, zero Isc, input not above knee, negative fault resistance or resistance above `Vk/Ik`.
- Conditioning probe: use `short_current=1e-20 A` with the other nominal inputs except `fault_resistance=5 ohm`. The physical solution is exactly the knee `(5 V,1 A)`, and fault pass power is `4 W`. The normalized positive-term denominator preserves this limit; no new helper or artificial minimum short-current bound is used.

**Mandatory diagram topology:** Series pass device feeds resistive fault; rising foldback branch, load-line intersection and worst-stress point remain distinct. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `input_voltage`, `knee_voltage`, `knee_current`, `short_current`, `fault_resistance`, `fault_output_current`, `fault_output_voltage`, `fault_pass_power`, `peak_stress_voltage`, `peak_stress_current`, `maximum_pass_power`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

## Converters — 6 required items

Lane accountability: **D2Converters / T2Converters / V2Converters**. Levels: 1 basic, 2 intermediate, 3 expert.

### 041. `converters.inverting_charge_pump` — Single-flying-capacitor inverter load regulation

- **Required / level:** Must-have calculator + diagram + docs; basic.
- **Accountable:** D2Converters developer; T2Converters tester; V2Converters source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/converters.md#1-convertersinverting_charge_pump); [complete JSON example vector](../../research/expansion/converters.json), selected by `id=converters.inverting_charge_pump`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `transport_resistance=2.1276595744680855` ohm; `output_resistance=3.3776595744680855` ohm; `output_voltage=-4.9324468085106385` V.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Preserve topology, ratio orientation, switching-frequency and conduction-mode conventions; do not infer ratings or soft switching.

**Semantic novelty decision — retain as one distinct physical problem.**

Nearest baseline/neighbor: `transients.capacitor_charge_sharing`, `dc.resistive_source_matching`. Repeated two-phase flying-capacitor charge transport produces an inverted DC rail and a frequency-dependent equivalent resistance. Neither an isolated charge-sharing transient nor an entered source resistance solves this.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: above complete map; preserve the negative output sign.
- Zero load: keep every other nominal input, set `load_current=0`; both resistances stay unchanged, droop is zero and output is `-5 V`.
- Frequency scaling: doubling fs halves **only** the transport contribution, not the switch or ESR floor.
- Ideal transport-only vector: `vin=5`, `fs=10000`, `Cfly=100e-6`, all three resistance inputs zero, `Iload=1` gives `Rtransport=Rout=1 ohm`, droop `1 V`, output `-4 V`.
- Collapse boundary: same transport-only vector with `Iload=5` must reject equality. Larger load must reject, not return a positive output.
- Reject zero/negative frequency or flying capacitance, nonpositive Vin, negative load or any negative resistance. Reject nonfinite values through the shared numerical boundary.

**Mandatory diagram topology:** Two switch-state panels show the same flying capacitor: charge from Vin, then ground upper plate and transfer from lower plate to negative rail. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `vin`, `load_current`, `switching_frequency`, `flying_capacitance`, `output_resistance`, `output_voltage`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 042. `converters.sepic_ccm_stress` — Uncoupled-inductor SEPIC CCM component stress

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Converters developer; T2Converters tester; V2Converters source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/converters.md#2-converterssepic_ccm_stress); [complete JSON example vector](../../research/expansion/converters.json), selected by `id=converters.sepic_ccm_stress`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `duty=0.6` 1; `input_current=1.5` A; `blocking_voltage=30.0` V.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Preserve topology, ratio orientation, switching-frequency and conduction-mode conventions; do not infer ratings or soft switching.

**Semantic novelty decision — retain as one distinct physical problem.**

Nearest baseline/neighbor: `power.boost_inductor_design`, `power.buck_input_capacitor_rms`. Two independent inductors and a series energy-transfer capacitor create summed switch current and alternating capacitor branch current. This is not an ordinary boost inductor-sizing equation.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: above complete map; both boost-like and buck-like conversion ratios are allowed without swapping formulas.
- Unity conversion: `vin=vout=12`, `Iout=1`, `fs=200000`, `L1=L2=100e-6` gives D=.5, Iin=1 A, both ripples=.3 A, peak=2.3 A, switch RMS `1.4195069566578393 A`, Cp RMS `1.0037429949942367 A`, blocking=24 V.
- Increasing both inductances reduces ripple; asymptotic switch RMS is `(Iin+Iout)sqrt(D)` and Cp RMS is `sqrt(Iin Iout)`.
- With nominal voltages/frequency/load, `L1=12e-6 H` makes the L1 valley exactly zero and must reject; `L2=18e-6 H` likewise rejects. Test just above and below each physical boundary with sensible decimal separation.
- Reject zero or negative voltage, load, frequency or either inductance. Do not impose `Vout>Vin`: SEPIC supports both sides.

**Mandatory diagram topology:** Vin–L1–A, Q from A to ground, Cp from A to B, L2 ground to B and diode B to output; separate uncoupled inductors. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `vin`, `vout`, `input_inductance`, `second_inductance`, `duty`, `switch_peak_current`, `coupling_cap_rms_current`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 043. `converters.forward_reset_winding` — Single-switch forward transformer-reset feasibility

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Converters developer; T2Converters tester; V2Converters source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/converters.md#3-convertersforward_reset_winding); [complete JSON example vector](../../research/expansion/converters.json), selected by `id=converters.forward_reset_winding`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `duty=0.375` 1; `reset_fraction=0.375` 1; `switch_blocking_voltage=96.0` V.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Preserve topology, ratio orientation, switching-frequency and conduction-mode conventions; do not infer ratings or soft switching.

**Semantic novelty decision — retain as one distinct physical problem.**

Nearest baseline/neighbor: `power.buck_inductor_design`, `magnetics.turns_ratio`. A third winding must remove magnetizing volt-seconds during off-time. Its ratio jointly constrains available duty and switch stress. Output inductor sizing and simple transformer ratio are not the deliverable.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: above map; reset completes 2.5 microseconds before the next on-time.
- Shorter reset winding: keep defaults except `reset_primary_ratio=.5`. Expected D=.375, reset fraction=.1875, limit=2/3, idle=.4375, reset time=1.875e-6 s, stress=144 V.
- Boundary: default ratios with `vout=16 V` give D=.5 and zero idle; reject equality. `vout=15.999 V` is admitted with a small positive margin; `vout=16.001 V` rejects.
- Doubling frequency halves reset time but does not change duty fractions or ideal voltage stress.
- Reject nonpositive bus/output voltage, ratio or frequency. Do not invent an integer-ratio restriction: ratios of integer turn counts are generally fractional.

**Mandatory diagram topology:** Three-winding forward topology includes correctly dotted reset winding/diode returning energy to Vin, output choke and on/reset/idle intervals. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `vin`, `vout`, `primary_secondary_ratio`, `reset_primary_ratio`, `duty`, `reset_fraction`, `idle_fraction`, `switch_blocking_voltage`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 044. `converters.flyback_dcm_demagnetization` — DCM flyback energy and demagnetization budget

- **Required / level:** Must-have calculator + diagram + docs; expert.
- **Accountable:** D2Converters developer; T2Converters tester; V2Converters source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/converters.md#4-convertersflyback_dcm_demagnetization); [complete JSON example vector](../../research/expansion/converters.json), selected by `id=converters.flyback_dcm_demagnetization`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `primary_peak_current=1.5491933384829668` A; `duty=0.3227486121839514` 1; `diode_reverse_voltage=36.0` V.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Preserve topology, ratio orientation, switching-frequency and conduction-mode conventions; do not infer ratings or soft switching.

**Semantic novelty decision — retain as one distinct physical problem.**

Nearest baseline/neighbor: `magnetics.field_energy_density`, `power.boost_inductor_design`. A primary energy-storage interval, isolated secondary demagnetization interval and genuine zero-current interval must all fit in one period. This delivers topology-specific mode feasibility and winding stress, not static field energy.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: above complete map. Its nonzero idle is small; the calculation does not represent a production margin recommendation.
- Clean alternate: `vin=vout=10 V`, `n=1`, `Lm=100e-6 H`, `fs=100000 Hz`, `Io=.03125 A` gives Ipk=.25 A, D=.25, Ddemag=.25, idle=.5, both RMS currents `.25/sqrt(12) = .07216878364870322 A`, both blocking voltages 20 V.
- Physical boundary: same alternate inputs except `Io=.125 A` give Ipk=.5 A and D=Ddemag=.5. Equality must reject, not become a zero-idle DCM success. Slightly lower current admits; slightly higher rejects.
- Decreasing positive load sends current and conduction fractions toward zero and idle toward one. Exact no-load operation is excluded because a regulating converter may pulse-skip or stop rather than maintain this fixed-frequency loaded model.
- Reject nonpositive load, voltage, frequency, turns ratio or magnetizing inductance. An efficiency input is intentionally absent rather than treated as a free inconsistent correction.

**Mandatory diagram topology:** Isolated flyback has reversed secondary dot orientation and diode; primary ramp, secondary decay and zero-current idle occupy one full period. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `vin`, `vout`, `magnetizing_inductance`, `primary_secondary_ratio`, `primary_peak_current`, `duty`, `demagnetization_fraction`, `idle_fraction`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 045. `converters.llc_half_bridge_fha` — Half-bridge LLC loaded fundamental-harmonic gain

- **Required / level:** Must-have calculator + diagram + docs; expert.
- **Accountable:** D2Converters developer; T2Converters tester; V2Converters source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/converters.md#5-convertersllc_half_bridge_fha); [complete JSON example vector](../../research/expansion/converters.json), selected by `id=converters.llc_half_bridge_fha`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `resonant_frequency=112539.53951963827` Hz; `normalized_frequency=1.0662919051580078` 1; `input_reactance=58.16865958726068` ohm.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Preserve topology, ratio orientation, switching-frequency and conduction-mode conventions; do not infer ratings or soft switching.

**Semantic novelty decision — retain as one distinct physical problem.**

Nearest baseline/neighbor: `ac.series_rlc_impedance`, `magnetics.turns_ratio`; neighboring Control models. Half-bridge excitation, nonlinear full-wave rectification, transformer reflection and a shunt magnetizing branch determine loaded DC conversion. A plain passive RLC or loop compensator does not have this model.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: above map; positive input reactance does not waive a switching-charge check.
- Series resonance: set fs to `1/(2pi sqrt(LrCr))`. Then x=1, M=1 and Vo=50 V regardless of finite positive Ro. Verify multiple loads; do not confuse the low-frequency Lr+Lm resonance with this point.
- Signed-region example: keep defaults except fs=40000 Hz; the FHA reactance is `-10.53912559450507 ohm`. The arithmetic result is admitted and explicitly capacitive; far-from-resonance accuracy is not claimed.
- Light-load limit Ro→infinity makes Q→0 and exposes the lower resonance. Exact infinity is invalid. Near lower resonance the approximation can predict very large gain; do not clamp it into a supposed regulated voltage or claim device feasibility.
- Reject zero/negative Vin, n, Lr, Lm, Cr, fs or Ro. No imposed requirement `Lm>Lr`: positive values define the stated equivalent circuit, even though practical designs commonly use larger Lm.

**Mandatory diagram topology:** Half bridge feeds Cr/Lr then parallel Lm/transformer; full-wave secondary rectifier and separate labeled FHA equivalent show Rac normalization. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `vin`, `primary_secondary_ratio`, `resonant_inductance`, `magnetizing_inductance`, `resonant_capacitance`, `load_resistance`, `ac_load_resistance`, `tank_gain`, `output_voltage`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 046. `converters.dual_active_bridge_power` — Dual-active-bridge single-phase-shift power exchange

- **Required / level:** Must-have calculator + diagram + docs; expert.
- **Accountable:** D2Converters developer; T2Converters tester; V2Converters source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/converters.md#6-convertersdual_active_bridge_power); [complete JSON example vector](../../research/expansion/converters.json), selected by `id=converters.dual_active_bridge_power`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `voltage_ratio=1.0` 1; `transferred_power=2222.222222222222` W; `phase_delay=8.333333333333333e-07` s.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Preserve topology, ratio orientation, switching-frequency and conduction-mode conventions; do not infer ratings or soft switching.

**Semantic novelty decision — retain as one distinct physical problem.**

Nearest baseline/neighbor: `power.converter_input_budget`, `magnetics.turns_ratio`; neighboring LLC. Two active square-wave bridges exchange signed power through transfer inductance under phase displacement. The result is not an entered-efficiency budget or a resonant-tank gain model.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: above full map. Lossless average bus powers agree, despite the 2:1 current scaling.
- Reverse phase `-pi/6`: ratio and maximum power unchanged; transferred power, both bus currents and phase delay negate exactly.
- Zero phase: P=I1=I2=delay=0; ratio=1, maximum=4000 W. Repeat zero phase with mismatched voltages to ensure no assertion that zero net power means zero circulating current.
- `phi=pi/2`: P=4000 W, primary current=10 A, secondary current=20 A, delay=2.5e-6 s. `phi=5pi/6` yields the same positive power as pi/6 but a larger delay, proving the falling-power branch is retained.
- `phi=±pi`: P and bus currents zero, delay=±5e-6 s. These endpoints are mathematical SPS limits, not recommended operating points.
- Reject phase outside ±pi and any nonpositive bus voltage, ratio, frequency or transfer inductance. Negative phase is valid and must not be mistaken for a negative-input error.

**Mandatory diagram topology:** Two isolated active full bridges couple through primary-referred transfer L and transformer; waveform lag carries signed power direction. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `primary_voltage`, `secondary_voltage`, `primary_secondary_ratio`, `switching_frequency`, `transfer_inductance`, `phase_shift`, `transferred_power`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

## Control — 6 required items

Lane accountability: **D2Control / T2Control / V2Control**. Levels: 1 basic, 3 intermediate, 2 expert.

### 047. `control.delay_phase_budget` — Feedback transport-delay phase-margin budget

- **Required / level:** Must-have calculator + diagram + docs; basic.
- **Accountable:** D2Control developer; T2Control tester; V2Control source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/control.md#1-controldelay_phase_budget--basic); [complete JSON example vector](../../research/expansion/control.json), selected by `id=control.delay_phase_budget`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `added_phase_lag=7.2` deg; `remaining_phase_margin=52.8` deg; `delay_headroom=2.1666666666666664e-05` s.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Supply the documented plant/return-ratio/slope model with its sign and frequency conventions; local margins do not certify global stability.

**Semantic novelty decision — retain as one distinct physical problem.**

Budget additional propagation/computation latency in an already stable feedback loop. Unlike `timing.synchronous_setup_slack`, this is a frequency-domain feedback stability constraint, not a register arrival-time constraint. Unlike `signals.line_propagation`, it does not derive travel time from cable geometry. Unlike `opamps.dominant_pole_gain_bandwidth`, no amplifier dominant-pole model or bandwidth estimate is solved.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: the complete linked research vector.
- Zero delay with other inputs unchanged: lag `0`, remaining margin `60`, maximum delay and headroom both `1/24000 s`.
- `added_delay=1/24000 s`: remaining margin `45 deg`, headroom zero within floating-point tolerance.
- `added_delay=0.0002 s`: lag `72 deg`, remaining margin `-12 deg`, headroom `-0.00015833333333333333 s`; this must calculate, not reject.
- Target equal to initial margin: maximum delay zero; any positive delay has negative headroom.
- Reject zero/negative crossover, nonpositive initial margin, initial margin above 180, negative target, target above initial margin, negative delay, and nonfinite inputs.

**Mandatory diagram topology:** Negative feedback picks off after pure delay; initial phase margin, added lag and remaining target budget remain distinct. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `crossover_frequency`, `initial_phase_margin`, `target_phase_margin`, `added_delay`, `added_phase_lag`, `remaining_phase_margin`, `maximum_delay`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 048. `control.loop_sensitivity` — Complex loop disturbance and measurement-noise sensitivity

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Control developer; T2Control tester; V2Control source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/control.md#2-controlloop_sensitivity--intermediate); [complete JSON example vector](../../research/expansion/control.json), selected by `id=control.loop_sensitivity`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `return_difference=0.7071067811865476` 1; `sensitivity_magnitude=1.4142135623730951` 1; `noise_output=0.001` V.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Supply the documented plant/return-ratio/slope model with its sign and frequency conventions; local margins do not certify global stability.

**Semantic novelty decision — retain as one distinct physical problem.**

Predict whether feedback rejects an output disturbance or amplifies it, and separately quantify transmission of measurement noise. Nearest old models are `opamps.dominant_pole_gain_bandwidth` and `noise.noninverting_amplifier_budget`. Neither accepts a complex return ratio nor distinguishes output disturbance from measurement noise injection. This is not another noise-source sum or scalar closed-loop amplifier gain.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: complete linked research vector.
- `loop_real=0,loop_imaginary=0`, same amplitudes: return difference `1`, sensitivity `1`, complementary sensitivity `0`, disturbance output `0.01 V`, noise output `0`.
- `loop_real=9,loop_imaginary=0`, same amplitudes: return difference `10`, sensitivity `0.1`, complementary sensitivity `0.9`, disturbance output `0.001 V`, noise output `0.0009 V`.
- Conjugating L must leave all five outputs unchanged.
- `loop_real=-1,loop_imaginary=0.001`: return difference `0.001`, sensitivity `1000`; the large result must not be suppressed.
- Zero disturbance/noise amplitudes are valid and independently produce zero contributions.
- Reject the exact singular point, negative amplitudes and nonfinite inputs. Do not reject negative real/imaginary return-ratio components.

**Mandatory diagram topology:** Output disturbance enters after forward block; measurement noise enters feedback before reference subtraction; distinguish S and T paths. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `loop_real`, `loop_imaginary`, `disturbance_amplitude`, `noise_amplitude`, `sensitivity_magnitude`, `complementary_magnitude`, `disturbance_output`, `noise_output`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 049. `control.lead_crossover_synthesis` — Phase-lead compensator at a chosen loop crossover

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Control developer; T2Control tester; V2Control source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/control.md#3-controllead_crossover_synthesis--intermediate); [complete JSON example vector](../../research/expansion/control.json), selected by `id=control.lead_crossover_synthesis`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `pole_zero_ratio=0.3333333333333333` 1; `zero_frequency=577.3502691896258` Hz; `zero_time_constant=0.00027566444771089603` s.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Supply the documented plant/return-ratio/slope model with its sign and frequency conventions; local margins do not certify global stability.

**Semantic novelty decision — retain as one distinct physical problem.**

Choose a controller zero, pole and gain for a desired phase boost at an intended loop crossover. The nearest old calculators, `filters.rc_highpass_response` and `filters.butterworth_spec_synthesis`, compute passive frequency response or attenuation-based filter order. This model designs a finite-gain lead controller from a feedback phase requirement and the measured plant magnitude, preserving a specified crossover. It is not an inverse of either filter calculator.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: all outputs in the linked research vector. Independently substitute these coefficients into C(j2πfc); magnitude must be `2` and phase must be `30 deg`.
- Double fc: zero/pole frequencies double, time constant halves, alpha and both gains stay unchanged.
- Double M: both gains halve; frequencies, alpha and time constant stay unchanged.
- As positive boost tends toward zero, alpha tends to one, both corners approach fc and both gains approach `1/M`.
- Near 90 degrees, e.g. `89.999 deg`, corners must remain ordered and finite at ordinary scales. Their geometric mean remains fc; do not evaluate alpha with cancellation-prone `1-sin φ`.
- Reject zero/negative boost, boost at or above 90, nonpositive crossover or plant magnitude, and nonfinite inputs.

**Mandatory diagram topology:** Aligned log-frequency magnitude/phase panels mark zero, crossover and pole; finite gain plateaus and peak phase boost are explicit. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `phase_boost`, `crossover_frequency`, `plant_magnitude`, `zero_frequency`, `pole_frequency`, `dc_gain`, `high_frequency_gain`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 050. `control.pi_pole_placement` — PI pole placement for a first-order voltage-domain plant

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Control developer; T2Control tester; V2Control source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/control.md#4-controlpi_pole_placement--intermediate); [complete JSON example vector](../../research/expansion/control.json), selected by `id=control.pi_pole_placement`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `proportional_gain=1` 1; `integral_gain=200` 1/s; `zero_frequency=31.830988618379067` Hz.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Supply the documented plant/return-ratio/slope model with its sign and frequency conventions; local margins do not certify global stability.

**Semantic novelty decision — retain as one distinct physical problem.**

Choose two controller gains from a first-order voltage-domain plant and desired closed-loop characteristic polynomial. This is not `opamps.integrator_constant_input`, which computes an isolated capacitor ramp, or `transients.rlc_underdamped_step`, which computes a passive RLC transient. No step waveform or overshoot is returned. Unlike the preceding lead item, this model introduces integral action and directly assigns two closed-loop polynomial coefficients rather than adding a localized phase boost.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: complete vector plus reconstructed polynomial above.
- Double plant gain to 4: Kp `0.5`, Ki `100 1/s`, Ti `0.005 s`, zero `100/π Hz`; the target denominator remains unchanged after normalization.
- `damping_ratio=1` with other defaults: Kp `1.5`, Ki `200`, Ti `0.0075`, zero `200/(3π) Hz`; polynomial has a repeated pole at `-200 rad/s`.
- `damping_ratio=2`: valid overdamped target; no square root of `1-ζ²` may be introduced.
- At `natural_frequency=200`, `plant_time_constant=0.01`, `damping_ratio=0.25`, the gain-positive boundary equals one and must reject. Smaller damping also rejects; just above the boundary returns small positive Kp.
- Reject nonpositive K, τ, ωn or ζ and nonfinite inputs.

**Mandatory diagram topology:** Split error into parallel Kp and Ki/s branches, sum into first-order plant, then unity negative feedback. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `plant_gain`, `plant_time_constant`, `natural_frequency`, `damping_ratio`, `proportional_gain`, `integral_gain`, `zero_frequency`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 051. `control.peak_current_slope_compensation` — Peak-current sampled perturbation and slope compensation

- **Required / level:** Must-have calculator + diagram + docs; expert.
- **Accountable:** D2Control developer; T2Control tester; V2Control source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/control.md#5-controlpeak_current_slope_compensation--expert); [complete JSON example vector](../../research/expansion/control.json), selected by `id=control.peak_current_slope_compensation`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `sample_multiplier=-0.3333333333333333` 1; `next_perturbation=-0.0033333333333333335` V; `steady_duty=0.75` 1.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Supply the documented plant/return-ratio/slope model with its sign and frequency conventions; local margins do not certify global stability.

**Semantic novelty decision — retain as one distinct physical problem.**

Assess subharmonic instability of a clocked peak-current inner loop and choose an artificial ramp boundary. Nearest old calculations are `power.buck_inductor_design`, `power.boost_inductor_design` and `power.boost_rhp_zero`. Those describe steady inductor ripple or a continuous averaged right-half-plane zero. This item describes a sampled-data cycle-to-cycle perturbation eigenvalue, a different instability mechanism. It uses sensed slopes directly and does not size converter components.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: all six outputs in the linked research vector.
- No ramp with default slopes: λ `-3`, next perturbation `-0.03 V`, ramp swing zero, reserve `-2`; calculation must report instability rather than reject.
- `ramp_slope=100000`: λ `-1`, reserve zero, alternating nondecaying map; this boundary is not stable.
- `ramp_slope=300000`: λ zero, next perturbation zero, reserve one, ramp swing `3 V`; source's deadbeat case.
- `rising_slope=300000`, `falling_slope=100000`, `ramp_slope=0`: boundary zero, λ `-1/3`, reserve `2/3`, duty `0.25`; zero ramp is already stable.
- Equal rising/falling slopes with zero ramp gives λ `-1` and duty `0.5`, marginal.
- Negating initial perturbation negates only next perturbation. Zero initial perturbation is valid. Changing Ts changes only ramp swing, not the map eigenvalue or inferred duty.
- Reject nonpositive m1/m2/Ts, negative ramp, and nonfinite values. Signed initial perturbations are valid.

**Mandatory diagram topology:** Sense-voltage triangles and artificial ramp show successive cycle perturbations, slope signs, clock period and signed multiplier. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `rising_slope`, `falling_slope`, `ramp_slope`, `switching_period`, `initial_perturbation`, `sample_multiplier`, `next_perturbation`, `ramp_amplitude`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 052. `control.integral_two_lag_stability` — Integral-regulator stability limit with plant and sensor lag

- **Required / level:** Must-have calculator + diagram + docs; expert.
- **Accountable:** D2Control developer; T2Control tester; V2Control source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/control.md#6-controlintegral_two_lag_stability--expert); [complete JSON example vector](../../research/expansion/control.json), selected by `id=control.integral_two_lag_stability`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `critical_integral_gain=750` 1/s; `gain_factor_to_boundary=7.5` 1; `critical_frequency=112.53953951963827` Hz.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Supply the documented plant/return-ratio/slope model with its sign and frequency conventions; local margins do not certify global stability.

**Semantic novelty decision — retain as one distinct physical problem.**

Find how a finite feedback-sensor pole limits a pure integral voltage regulator. The nearest baseline is `opamps.dominant_pole_gain_bandwidth`, but the present characteristic polynomial is cubic, with an independent sensor pole and an integrator-gain instability boundary. It is not another bandwidth calculation, passive cascade response or RLC step. It differs from the PI item in both controller topology and plant/sensor order: here the design question is the permissible gain range, not two-coefficient pole assignment.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: complete vector and independent Routh column above.
- Ki=750: critical gain `750`, factor `1`, reserve `0`, same critical frequency; return marginal boundary rather than claiming strict stability.
- Ki=1000: factor `0.75`, reserve `-1/3`, same critical gain/frequency; accepted unstable proposal.
- Exchange plant and sensor time constants: all four outputs remain unchanged.
- Set both time constants to `0.001 s`: critical gain `1000 1/s`, factor `10`, reserve `0.9`, critical frequency `1000/(2π) Hz`.
- As the sensor time constant approaches zero positively, critical gain and boundary frequency grow; exactly zero is outside the two-lag model and must reject.
- Reject nonpositive K/Ki/tp/ts and nonfinite inputs.

**Mandatory diagram topology:** Integral controller and first-order plant feed output; independent first-order sensor lies in negative feedback return. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `plant_gain`, `plant_time_constant`, `sensor_time_constant`, `integral_gain`, `critical_integral_gain`, `critical_frequency`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

## Emc — 6 required items

Lane accountability: **D2Emc / T2Emc / V2Emc**. Levels: 1 basic, 3 intermediate, 2 expert.

### 053. `emc.thin_slot_shielding` — Thin-slot shield leakage estimate

- **Required / level:** Must-have calculator + diagram + docs; basic.
- **Accountable:** D2Emc developer; T2Emc tester; V2Emc source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/emc.md#1-emcthin_slot_shielding--basic); [complete JSON example vector](../../research/expansion/emc.json), selected by `id=emc.thin_slot_shielding`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `wavelength=2.99792458` m; `shielding_db=37.495214231999306` dB; `field_ratio=0.013342563807926084` 1.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Preserve the specified source waveform, coupling geometry, receiver/load or measured parasitic experiment; no compliance claim.

**Semantic novelty decision — retain as one distinct physical problem.**

Estimate whether a single thin shield slot is likely to dominate electric-field leakage. Closest baseline: `magnetics.conductor_skin_depth`, which predicts penetration into bulk conductive material. This model predicts leakage through an opening using aperture geometry, not bulk absorption, skin depth or an inverse of that calculation. RF's circular-aperture antenna gain concerns intentional radiation and is not this shield-slot model.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: every output above, relative tolerance 1e-6.
- Small-aperture endpoint: frequency=100000000, L=0.149896229, w=0.002, t=0.0005 gives wavelength=2.99792458, SE=20 dB and field_ratio=0.1; accept equality within arithmetic precision.
- Halve nominal L while retaining valid w/t: SE increases by 6.020599913279624 dB and field_ratio halves.
- t=0 is valid and changes no result; t=L/10 is valid.
- Reject f<=0, L<=0, w<=0, t<0, w>=L, t>L/10 and L>wavelength/20. No compliance label should appear for valid results.

**Mandatory diagram topology:** Long slot in shield face with wall-thickness inset; incoming/transmitted fields cross the opening, not solid metal. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `frequency`, `slot_length`, `slot_width`, `shield_thickness`, `wavelength`, `shielding_db`, `field_ratio`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 054. `emc.rc_common_mode_conversion` — RC input-filter common-mode conversion

- **Required / level:** Must-have calculator + diagram + docs; expert.
- **Accountable:** D2Emc developer; T2Emc tester; V2Emc source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/emc.md#2-emcrc_common_mode_conversion--expert); [complete JSON example vector](../../research/expansion/emc.json), selected by `id=emc.rc_common_mode_conversion`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `mismatch_time=-2e-07` s; `conversion_gain=0.0008041073247070083` 1; `differential_rms=8.041073247070084e-05` V.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Preserve the specified source waveform, coupling geometry, receiver/load or measured parasitic experiment; no compliance claim.

**Semantic novelty decision — retain as one distinct physical problem.**

Predict differential noise created by mismatched input EMI-filter legs and suppress it with a bridging capacitor. Closest baselines: `filters.rc_lowpass_response` describes one scalar branch, and `opamps.difference_ratio_error` describes a resistive active amplifier. Neither predicts common-mode conversion through a coupled three-capacitor input network. This is also not the baseline injected capacitive-crosstalk pulse model.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: every output above at 1e-6 relative tolerance.
- Set capacitance_one=capacitance_two=1e-8: D, gain and differential voltage are exactly zero, regardless of Cd.
- Set frequency=0: retain D=−2e-7 s, but both amplitude outputs are zero.
- Set Cd=0: D remains −2e-7 s, gain=**0.009009434213906478**, differential_rms=**0.0009009434213906478 V**. Independently subtract two uncoupled complex RC responses.
- Swap complete branch pairs: D changes sign; both magnitudes are unchanged.
- Set common_mode_rms=0: gain remains nominal and voltage becomes zero.
- Reject nonpositive R/C1/C2, negative Cd/frequency/common-mode RMS and nonfinite values. Huge arithmetic intermediates must fail explicitly, not return plausible finite zero through a nonfinite normalization.

**Mandatory diagram topology:** Common source drives separate R/C receiver legs; bridging Cd connects signal nodes while both shunt capacitors share return. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `resistance_one`, `resistance_two`, `capacitance_one`, `capacitance_two`, `differential_capacitance`, `common_mode_rms`, `differential_rms`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 055. `emc.trapezoid_conducted_harmonic` — Trapezoidal conducted-current harmonic

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Emc developer; T2Emc tester; V2Emc source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/emc.md#3-emctrapezoid_conducted_harmonic--intermediate); [complete JSON example vector](../../research/expansion/emc.json), selected by `id=emc.trapezoid_conducted_harmonic`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `harmonic_frequency=300000` Hz; `dc_current=0.0004` A; `termination_voltage_rms=0.004403413021595208` V.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Preserve the specified source waveform, coupling geometry, receiver/load or measured parasitic experiment; no compliance claim.

**Semantic novelty decision — retain as one distinct physical problem.**

Predict a selected spectral line from the actual conducted pulse current entering a known resistive RF termination. Closest baselines: `signals.rise_time_bandwidth` has one edge-time bandwidth heuristic, while `power.buck_input_capacitor_rms` integrates total waveform RMS. Neither resolves a selected harmonic with duty-cycle nulls and the second edge-shape sinc factor. This is a Fourier-generation model, not a renamed single-pole filter.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: all four outputs in the linked research vector at 1e-6 relative tolerance.
- Triangular endpoint: change D=0.5, tr=5e-6 s, n=1, retain fs=100000, Ipk=0.001, R=50. Expected frequency=100000 Hz, DC=0.0005 A, RMS current=2sqrt(2)×0.001/π²=**0.0002865795841253782 A**, RMS voltage=**0.014328979206268907 V**.
- For D=0.5 and n=2, the harmonic is analytically zero; an absolute null tolerance scaled to Ipk times floating-point sine error is appropriate. Never compare a numerical sine residual relatively to mathematical zero.
- Ipk=0 gives zero DC/current/voltage while retaining fn. Complement D with 1−D: harmonic magnitude stays equal, DC changes.
- As tr→0+, the edge factor tends to one. Small representable arguments must not be clamped to an arbitrary floor.
- Reject fractional n, n<1, D endpoints/outside, tr<=0, overlapping edges, fs<=0 and R<=0. A nonfinite/underflowed sine denominator must produce an explicit error.

**Mandatory diagram topology:** Trapezoid marks full edge time and half-height width D/fs; separate termination and harmonic-spectrum annotations. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `switching_frequency`, `pulse_current`, `duty_half_height`, `edge_time`, `termination_resistance`, `harmonic_frequency`, `harmonic_current_rms`, `termination_voltage_rms`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 056. `emc.sinusoidal_dither_occupancy` — Switching-frequency dither harmonic occupancy

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Emc developer; T2Emc tester; V2Emc source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/emc.md#4-emcsinusoidal_dither_occupancy--intermediate); [complete JSON example vector](../../research/expansion/emc.json), selected by `id=emc.sinusoidal_dither_occupancy`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `harmonic_modulation_index=15` 1; `occupied_bandwidth=64000` Hz; `next_harmonic_gap=26000` Hz.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Preserve the specified source waveform, coupling geometry, receiver/load or measured parasitic experiment; no compliance claim.

**Semantic novelty decision — retain as one distinct physical problem.**

Select switching-frequency modulation depth and rate while inspecting occupied harmonic bands and their overlap. Closest baseline: `timing.timer_clock_divider` has a static divided clock frequency, and `signals.rise_time_bandwidth` estimates unmodulated edge bandwidth. The new physical mechanism is FM sideband redistribution. It does not duplicate the unmodulated trapezoid line amplitude above or measurement's sampling-jitter SNR.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: all three outputs in the linked research vector.
- n=5 with other defaults: index=25, width=104000 Hz, gap=−14000 Hz. Accept the negative gap.
- Small positive deviation=1 Hz: index=0.0015, width=4006 Hz, gap=95993 Hz. This demonstrates Carson's conservative narrowband behavior, not actual 98%-power measurement.
- At fixed Δf/fm, scale fs, Δf and fm equally: index stays fixed; width and gap scale.
- Reject Δf<=0, Δf>=fs, fm<=0, fm>=fs, fs<=0, noninteger n or n<1. The valid no-overlap/overlap crossing is not a rejection boundary.

**Mandatory diagram topology:** Adjacent harmonic bands show occupied width and signed interband gap; inset defines sinusoidal frequency deviation and modulation rate. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `switching_frequency`, `peak_deviation`, `modulation_frequency`, `harmonic_order`, `harmonic_modulation_index`, `occupied_bandwidth`, `next_harmonic_gap`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 057. `emc.ringing_rc_snubber` — Measured-ringing RC snubber design

- **Required / level:** Must-have calculator + diagram + docs; expert.
- **Accountable:** D2Emc developer; T2Emc tester; V2Emc source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/emc.md#5-emcringing_rc_snubber--expert); [complete JSON example vector](../../research/expansion/emc.json), selected by `id=emc.ringing_rc_snubber`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `parasitic_capacitance=1e-09` F; `parasitic_inductance=2.8144773233982716e-08` H; `snubber_power=0.216` W.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Preserve the specified source waveform, coupling geometry, receiver/load or measured parasitic experiment; no compliance claim.

**Semantic novelty decision — retain as one distinct physical problem.**

Infer physical switch-node parasitics from an added-capacitor experiment, then select a shunt series-RC suppression network and budget charge/discharge loss. Closest baselines: `transients.rlc_underdamped_step` predicts a known series RLC circuit, and `mosfet.gate_loop_damping_resistor` inserts a resistor in a gate loop. This model identifies unknown node parasitics and designs a different topology with a DC-blocking snubber capacitor. It is not another RLC inverse mode: two independent frequency measurements plus a known perturbation identify both L and C, then constrain the practical suppression network.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: all five outputs in the linked research vector at 1e-6 relative tolerance.
- Set ζ=1 and k=4: Cp and Lp unchanged; Rs=**2.6525823848649224 ohm**, Cs=**4e-9 F**, P=**0.288 W**. Both endpoints are valid.
- V=0 leaves all inferred/design parameters unchanged and P=0.
- Double Ctest at unchanged measured frequencies: Cp/Cs/P double, Lp/Rs halve. Reconstruct both frequency identities independently.
- f1 approaching f0 from below is ill-conditioned but mathematically admissible if the arithmetic remains representable. Test that Cp remains positive and the actual small frequency difference is retained.
- Reject f1>=f0, f1<=0, Ctest<=0, fs<=0 or fs>=f1, ζ outside [0.5,1], k outside [3,4], and negative V. Reject nonfinite arithmetic rather than fabricate a zero component.

**Mandatory diagram topology:** Upstream parasitic L and node Cp plus separate shunt series Rs–Cs branch; temporary test capacitor is clearly detached from final design. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `original_frequency`, `loaded_frequency`, `test_capacitance`, `voltage_swing`, `parasitic_capacitance`, `parasitic_inductance`, `snubber_resistance`, `snubber_capacitance`, `snubber_power`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 058. `emc.short_cable_radiation` — Short common-mode cable radiation estimate

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Emc developer; T2Emc tester; V2Emc source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/emc.md#6-emcshort_cable_radiation--intermediate); [complete JSON example vector](../../research/expansion/emc.json), selected by `id=emc.short_cable_radiation`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `electrical_length=0.03335640951981521` 1; `far_field_parameter=20.958450219516816` 1; `electric_field_rms=6.283185307179587e-06` V/m.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Preserve the specified source waveform, coupling geometry, receiver/load or measured parasitic experiment; no compliance claim.

**Semantic novelty decision — retain as one distinct physical problem.**

Estimate broadside far-field electric interference from an electrically short cable carrying a known uniform in-phase common-mode current. Closest baseline: `signals.line_propagation` predicts propagation delay/wavelength inside an interconnect, not radiated fields. RF explicitly reserves intentional antenna/link calculations and leaves this unintended-emission model to EMC. No cable impedance, skin-depth or antenna gain/radiation-resistance duplicate is added.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: all three outputs in the linked research vector.
- Double distance to 20 m: electrical_length unchanged; kr doubles; E halves to 3.1415926535897935e-6 V/m.
- Icm=0: field exactly zero; geometry outputs unchanged.
- At f=1e8, ell=0.299792458 m gives ell/wavelength=0.1; with r=10 m it is the admitted short-element endpoint.
- At f=1e8 and nominal ell, r=10×299792458/(2π×1e8) m gives kr=10; a smaller r is rejected, not treated as far field.
- Reject nonpositive f/ell/r, negative RMS current, ell/wavelength>0.1, kr<10 and nonfinite values. Do not reject zero common-mode current.

**Mandatory diagram topology:** Uniform common-mode current on short cable; broadside observer with electric field parallel to cable and perpendicular to ray. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `frequency`, `cable_length`, `common_mode_current_rms`, `distance`, `electrical_length`, `far_field_parameter`, `electric_field_rms`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

## Cables — 6 required items

Lane accountability: **D2Cables / T2Cables / V2Cables**. Levels: 2 basic, 2 intermediate, 2 expert.

### 059. `cables.twin_lead_geometry` — Homogeneous parallel-wire line geometry

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Cables developer; T2Cables tester; V2Cables source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/cables.md#1-cablestwin_lead_geometry--intermediate); [complete JSON example vector](../../research/expansion/cables.json), selected by `id=cables.twin_lead_geometry`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `geometry_factor=2.477888730288475` 1; `inductance_per_length=9.9115549211539e-07` H/m; `differential_impedance=198.09396088157135` ohm.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use characterized RLGC/material properties and applicable geometry, elastic, thermal and dielectric validity screens.

**Semantic novelty decision — retain as one distinct physical problem.**

Nearest baseline/neighbor: `signals.coax_impedance`, `signals.microstrip_impedance`, `signals.stripline_impedance`. New balanced two-round-wire transverse boundary-value geometry, without a plane or shield. Uses acosh spacing, not a renamed coax logarithm or an inverse of an existing geometry. One geometry is counted once, including its L′/C′ outputs.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

set er=1 with unchanged geometry: C′ becomes `1.1225787117147267e-11 F/m`, Zdiff becomes `297.140941322357 ohm`, L′ and q stay unchanged. Scaling both lengths equally must leave all outputs unchanged. For `D=d*(1+u)` and small positive u, `q~sqrt(2u)`; verify positive impedance rather than rounded zero. Invalid: D=d, D<d, d=0, er<1. Do not replace the exact acosh geometry by the large-spacing `ln(2D/d)` approximation.

**Mandatory diagram topology:** Two equal round conductors without shield/plane; opposite potentials/currents, center spacing and wire diameter dimensions. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `wire_diameter`, `center_spacing`, `relative_permittivity`, `inductance_per_length`, `capacitance_per_length`, `differential_impedance`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 060. `cables.lossy_rlgc_line` — Exact lossy distributed RLGC line

- **Required / level:** Must-have calculator + diagram + docs; expert.
- **Accountable:** D2Cables developer; T2Cables tester; V2Cables source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/cables.md#2-cableslossy_rlgc_line--expert); [complete JSON example vector](../../research/expansion/cables.json), selected by `id=cables.lossy_rlgc_line`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `angular_frequency=6283185.307179586` rad/s; `series_impedance_magnitude=2.5431085506270352` ohm/m; `attenuation_db=1.5679547500148998` dB.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use characterized RLGC/material properties and applicable geometry, elastic, thermal and dielectric validity screens.

**Semantic novelty decision — retain as one distinct physical problem.**

Nearest baseline/neighbor: `signals.line_propagation`, `signals.load_reflection`, `ac.series_rlc_impedance`. Distributed telegrapher equations with both series and dielectric loss; predicts complex Z0 and exponential loss outside the low-loss approximation. Not ideal delay, resistive reflection, or a lumped RLC impedance.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Set R′=G′=0: s=`1.5707963267948966`, t=`0.0006283185307179586`, x=-1, y=0, alpha=0, beta=`0.03141592653589793`, Z0=`50+j0 ohm`, ratio=1 and loss=0. Same f and length as nominal.
- Set only length=0: all local line outputs stay nominal; ratio=1 and loss=0.
- Set G′=0.0008 S/m with other nominal inputs: R′/L′=G′/C′ (Heaviside balance); `gamma=0.04+j*0.03141592653589793`, `Z0=50+j0`. Ratio is `exp(-0.4)`, loss is `8/ln(10)` dB. Test zero imaginary part with a stated ULP-scaled residual.
- Keep nominal RLGC but f=`1e-12 Hz`: `alpha≈0.006324555320336759`, `beta≈1.0182952972441003e-19`, `Zreal≈316.2277660168379`, `Zimag≈-4.8431117795755995e-15`. Beta must not vanish because a phase angle rounded to pi/2.
- Invalid: negative R′ or G′, L′=0, C′=0, f=0, negative length. Tiny loss must not be excluded by an arbitrary lower loss bound.

**Mandatory diagram topology:** Differential RLGC cell has series R′/L′ and distinct shunt G′/C′ paths; label per-length quantities and illustrative Δz correctly. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `series_resistance_per_length`, `inductance_per_length`, `shunt_conductance_per_length`, `capacitance_per_length`, `length`, `attenuation_constant`, `phase_constant`, `forward_amplitude_ratio`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 061. `cables.radial_insulation_stress` — Cylindrical cable insulation electric stress

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Cables developer; T2Cables tester; V2Cables source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/cables.md#3-cablesradial_insulation_stress--intermediate); [complete JSON example vector](../../research/expansion/cables.json), selected by `id=cables.radial_insulation_stress`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `inner_surface_field=1442695.0408889633` V/m; `outer_surface_field=360673.7602222408` V/m; `field_margin=6557304.959111037` V/m.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use characterized RLGC/material properties and applicable geometry, elastic, thermal and dielectric validity screens.

**Semantic novelty decision — retain as one distinct physical problem.**

Nearest baseline/neighbor: `signals.coax_impedance`, `timing.plate_displacement_current`; Protection. Dielectric field concentration and breakdown screening, not wave impedance, plate edge current or fault-current heating. Same cylindrical hardware can have distinct field-stress and impedance problems. Protection owns adiabatic short-circuit withstand.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

Vpk=0 gives both fields=0, unchanged voltage_limit, margin=8e6. Vpk=12000 gives Einner=`8656170.24533378 V/m` and negative margin about `-656170.24533378 V/m`, which must remain an accepted screening result. At fixed b/a, doubling a and b halves fields and doubles voltage limit. For b-a much smaller than a, Einner approaches Vpk/(b-a), recovering the planar local limit. Invalid b<=a, a<=0 or nonpositive field limit. AC RMS voltage must be converted to peak externally; silently treating RMS as peak understates stress.

**Mandatory diagram topology:** Concentric conductors with outward radial field; b is shield inner radius and peak stress occurs at conductor surface. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `conductor_radius`, `shield_inner_radius`, `peak_voltage`, `allowable_field`, `inner_surface_field`, `outer_surface_field`, `voltage_limit`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 062. `cables.hertz_holm_contact` — Elastic rounded-contact constriction resistance

- **Required / level:** Must-have calculator + diagram + docs; expert.
- **Accountable:** D2Cables developer; T2Cables tester; V2Cables source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/cables.md#4-cableshertz_holm_contact--expert); [complete JSON example vector](../../research/expansion/cables.json), selected by `id=cables.hertz_holm_contact`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `effective_modulus=62189054726.368164` Pa; `contact_radius=6.22480523898329e-06` m; `indentation=1.9374100131636908e-08` m.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use characterized RLGC/material properties and applicable geometry, elastic, thermal and dielectric validity screens.

**Semantic novelty decision — retain as one distinct physical problem.**

Nearest baseline/neighbor: `pcb.trace_resistance_drop`, `pcb.via_barrel_resistance`; contact spring below. Force-dependent elastic contact area and spreading/constriction current paths, not uniform conductor rho-length/area. Spring force is an input; beam bending is not counted here.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

F=0.08 N scales a by 2, p0 by 2, Rc by 1/2 and approach by 4, retaining the nominal material/geometry. Expected a=`1.244961047796658e-5 m`, p0=`246444906.36032718 Pa`, Rc=`0.0006827522849042842 ohm`, indentation=`7.749640052654763e-8 m`; E* unchanged. F approaches zero only until the diffusive lower-size screen fails; do not imply a zero-force closed metallic connection. Invalid F=0, nu=-1 or 0.5, pressure limit=1e8 Pa with nominal mechanics, mean_free_path=1e-6 m with nominal mechanics, or geometry causing a/R>0.1. Equality on the defined screening boundaries is permitted.

**Mandatory diagram topology:** Spherical tip presses flat contact; distinguish curvature R, contact radius a, full spot width 2a and converging current paths. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `normal_force`, `sphere_radius`, `young_modulus`, `poisson_ratio`, `resistivity`, `effective_modulus`, `contact_radius`, `maximum_pressure`, `constriction_resistance`, `indentation`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 063. `cables.contact_hotspot_temperature` — Metallic contact voltage–hotspot temperature

- **Required / level:** Must-have calculator + diagram + docs; basic.
- **Accountable:** D2Cables developer; T2Cables tester; V2Cables source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/cables.md#5-cablescontact_hotspot_temperature--basic); [complete JSON example vector](../../research/expansion/cables.json), selected by `id=cables.contact_hotspot_temperature`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `hotspot_temperature=339.8679215248663` K; `temperature_rise=39.86792152486629` K; `temperature_margin=260.1320784751337` K.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use characterized RLGC/material properties and applicable geometry, elastic, thermal and dielectric validity screens.

**Semantic novelty decision — retain as one distinct physical problem.**

Nearest baseline/neighbor: `thermal.junction_feedback_equilibrium`, `thermal.lumped_thermal_step`; Hertz/Holm above. Microscopic electrothermal contact law with electron thermal conduction, no lumped package thermal resistance/capacitance. Takes measured contact voltage, not a renamed resistance output or generic Joule-heating calculation.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

Vc=0 gives Tm=300 K, rise=0, unchanged voltage limit, margin=300 K. Tlimit=Tbulk with Vc=0 is valid and gives limit voltage=0/margin=0. Vc=1e-6 V gives rise about `1.7006802720606382e-8 K`; it must remain positive. Invalid: Tbulk<=0, Lorenz<=0, Tlimit<Tbulk or Vc=0.2 V at nominal temperatures, which exceeds the 600 K validity bound. Do not clamp the over-limit temperature into a plausible result.

**Mandatory diagram topology:** Two equal-temperature metal bodies join through constriction; Kelvin voltage spans neck only and heat flows outward in both directions. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `contact_voltage`, `bulk_temperature`, `lorenz_number`, `temperature_limit`, `hotspot_temperature`, `temperature_rise`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 064. `cables.cantilever_contact_spring` — Cantilever connector-contact spring

- **Required / level:** Must-have calculator + diagram + docs; basic.
- **Accountable:** D2Cables developer; T2Cables tester; V2Cables source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/cables.md#6-cablescantilever_contact_spring--basic); [complete JSON example vector](../../research/expansion/cables.json), selected by `id=cables.cantilever_contact_spring`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `spring_rate=520` N/m; `normal_force=0.26` N; `stress_margin=405000000` Pa.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use characterized RLGC/material properties and applicable geometry, elastic, thermal and dielectric validity screens.

**Semantic novelty decision — retain as one distinct physical problem.**

Nearest baseline/neighbor: `thermal.free_linear_expansion`; Motors. Elastic bending creates connector normal force from mating displacement. No thermal expansion, winding, electromechanical torque or motor model. Electrical connectivity depends on mechanical contact force, but no resistance fit is bundled as another feature.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

delta=0 returns the same spring_rate, normal_force=0, root_stress=0 and stress_margin=600e6 Pa. delta=0.001 m reaches the small-deflection screen exactly: F=0.52 N, sigma=390e6 Pa and margin=210e6 Pa; accepted. Doubling b doubles k and F but leaves stress unchanged at imposed deflection. Invalid: L=0, t>L/10, delta>L/10, or nominal geometry with allowable_stress=100e6 Pa. Test the slenderness boundary separately using delta=0 so an unrelated stress constraint does not hide the geometry result.

**Mandatory diagram topology:** Clamped-free beam has tip deflection/load and root stress; cross-section separates bending thickness from width. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `beam_length`, `beam_width`, `beam_thickness`, `young_modulus`, `tip_deflection`, `allowable_stress`, `spring_rate`, `normal_force`, `root_stress`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

## Precision — 6 required items

Lane accountability: **D2Precision / T2Precision / V2Precision**. Levels: 2 basic, 2 intermediate, 2 expert.

### 065. `precision.bjt_mirror_base_current` — Matched BJT current mirror base-current error

- **Required / level:** Must-have calculator + diagram + docs; basic.
- **Accountable:** D2Precision developer; T2Precision tester; V2Precision source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/precision.md#1-precisionbjt_mirror_base_current--basic); [complete JSON example vector](../../research/expansion/precision.json), selected by `id=precision.bjt_mirror_base_current`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `collector_current=0.000980392156862745` A; `base_current_each=9.803921568627451e-06` A; `compliance_margin=1.35` V.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use characterized beta, GBW, swing/common-mode/current limits or reference-device data; preserve the exact active netlist.

**Semantic novelty decision — retain as one distinct physical problem.**

Predict the reference-current loss into two base terminals of a matched two-NPN current mirror. This is not `bjt.hybrid_pi_parameters`, `bjt.early_output_resistance` or `bjt.differential_pair_steering`: it solves current-copy bias topology, not incremental transistor parameters or tail-current division. Unlike the Howland model below, the error comes from finite transistor beta, not feedback resistor balance. Unlike the Brokaw core, no temperature-compensated voltage reference is synthesized.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: assert every table value and KCL.
- Large beta: set `beta=1000`; collector current is `0.000998003992015968 A`, fractional deficit `0.001996007984031936`.
- Compliance boundary: set `collector_voltage=0.65`; margin is exactly zero; currents remain unchanged in the ideal model.
- Scaling: double reference current at unchanged characterized beta/VBE; all current outputs double, fractional deficit does not.
- Invalid: beta zero/negative, reference current zero/negative, VBE nonpositive, or collector voltage below VBE.

**Mandatory diagram topology:** Two grounded-emitter NPNs share bases; Q1 collector/base short explicit, Q2 collector separate, reference and output current arrows downward. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `reference_current`, `beta`, `base_emitter_voltage`, `collector_voltage`, `collector_current`, `base_current_each`, `fractional_deficit`, `compliance_margin`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 066. `precision.instrumentation_internal_headroom` — Three-op-amp instrumentation internal headroom

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Precision developer; T2Precision tester; V2Precision source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/precision.md#2-precisioninstrumentation_internal_headroom--intermediate); [complete JSON example vector](../../research/expansion/precision.json), selected by `id=precision.instrumentation_internal_headroom`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `first_stage_gain=21.0` 1; `total_gain=42.0` 1; `output_voltage=0.9199999999999999` V.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use characterized beta, GBW, swing/common-mode/current limits or reference-device data; preserve the exact active netlist.

**Semantic novelty decision — retain as one distinct physical problem.**

Design a high-input-impedance three-op-amp instrumentation amplifier while checking hidden internal clipping. Nearest baseline is `opamps.difference_ratio_error`, a single difference amplifier with resistor mismatch. This model has two additional input amplifiers and a floating gain resistor; its limiting behavior depends on their individual output nodes. It is not a renamed scalar noninverting gain or a second bridge-transducer calculation.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: assert all values and final difference-stage KCL.
- Zero differential voltage: Va=Vb=2 V, final output=0.5 V and final input pin=1.5 V.
- Reverse differential: Vd=-0.01 V gives Va=2.105 V, Vb=1.895 V, final pin=1.43 V and final output=0.08 V.
- Hidden clipping: set `common_voltage=7.89`, `differential_voltage=0.21`, keeping other inputs nominal. Both signal pins (7.785 V and 7.995 V) remain inside [-8,8] V and final output=9.32 V remains inside [-10,10] V, but Vb=10.095 V. Reject specifically because the first-stage output clips.
- Boundary: final output exactly on either supplied endpoint is accepted if all other nodes fit.
- Invalid: nonpositive resistance, reversed/equal limit intervals, any signal/common-mode pin outside its interval, or any of the three outputs outside its interval.

**Mandatory diagram topology:** Two input amplifiers with floating gain resistor feed four-resistor difference stage; reference port and all internal nodes remain explicit. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `feedback_resistance`, `gain_resistance`, `difference_input_resistance`, `difference_feedback_resistance`, `common_voltage`, `differential_voltage`, `reference_voltage`, `first_low_voltage`, `first_high_voltage`, `difference_pin_voltage`, `output_voltage`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 067. `precision.transimpedance_butterworth` — Finite-GBW transimpedance Butterworth compensation

- **Required / level:** Must-have calculator + diagram + docs; expert.
- **Accountable:** D2Precision developer; T2Precision tester; V2Precision source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/precision.md#3-precisiontransimpedance_butterworth--expert); [complete JSON example vector](../../research/expansion/precision.json), selected by `id=precision.transimpedance_butterworth`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `feedback_capacitance=5.63965054967668e-12` F; `butterworth_bandwidth=388147.3164084609` Hz; `noise_gain_zero=15065.833923508984` Hz.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use characterized beta, GBW, swing/common-mode/current limits or reference-device data; preserve the exact active netlist.

**Semantic novelty decision — retain as one distinct physical problem.**

Choose feedback capacitance for a finite-GBW current-to-voltage front end with input shunt capacitance. Nearest baseline is `opamps.dominant_pole_gain_bandwidth`, which has frequency-independent noise gain and first-order closed-loop voltage response. This circuit's noise gain rises because of source capacitance, creating a second-order response whose damping must be designed. It is not ordinary RC filtering, a generic noise budget or photodiode conversion. Opto explicitly owns the physical photocurrent and confirmed this electronic boundary.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: assert all values and substitute Cf into the damping identity.
- Zero signal: set `input_peak=0`; output peak is zero, all design outputs remain unchanged.
- Solvability boundary: with nominal R and Cs, GBW=`1/(4π R Cs)` ≈7957.747154594768 Hz gives Cf=0 and bandwidth≈11253.953951963826 Hz. Construct representable neighboring inputs carefully: a rounded value immediately below the inequality must reject, not be clamped to zero.
- Invalid solvability: GBW=1000 Hz with nominal R and Cs cannot realize a nonnegative Butterworth Cf; reject.
- Swing boundary: input peak=40 µA gives exactly 4 V and is accepted; a larger value rejects.
- Invalid: nonpositive R/Cs/GBW/swing or negative signal amplitude.

**Mandatory diagram topology:** Current source injects into grounded-reference op-amp minus node; Cs shunts input and separate Rf/Cf branches bridge output to summing node. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `feedback_resistance`, `source_capacitance`, `gain_bandwidth`, `input_peak`, `feedback_capacitance`, `butterworth_bandwidth`, `dc_transimpedance`, `output_peak`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 068. `precision.howland_mismatch_load` — Howland current pump resistor mismatch and load

- **Required / level:** Must-have calculator + diagram + docs; expert.
- **Accountable:** D2Precision developer; T2Precision tester; V2Precision source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/precision.md#4-precisionhowland_mismatch_load--expert); [complete JSON example vector](../../research/expansion/precision.json), selected by `id=precision.howland_mismatch_load`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `norton_drive_current=0.0001` A; `output_conductance=1e-06` S; `amplifier_current=9.900497512437811e-05` A.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use characterized beta, GBW, swing/common-mode/current limits or reference-device data; preserve the exact active netlist.

**Semantic novelty decision — retain as one distinct physical problem.**

Predict bidirectional load current and compliance when a basic Howland pump's resistor ratios are not perfectly balanced. Nearest baseline is `opamps.difference_ratio_error`, but the positive feedback and grounded current-output port change the physical problem: mismatch creates signed source output conductance and load-dependent current. Nearest new mirror model has a transistor base-current deficit, not this resistor-feedback law. No transistor foldback limiter or protection threshold is modeled.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: assert all values, both node KCL equations and amplifier-current decomposition.
- Perfect balance: set R4=10000; J=100 µA, g=0, Vx=0.5 V, IL=100 µA, Vo=1 V and amplifier current=100 µA.
- Reverse drive: set positive drive=-1 V with nominal resistors; all current/voltage outputs reverse, while g stays +1 µS.
- Admissible negative conductance: R4=10100 gives g=-1 µS and Vx=`100/199`≈0.5025125628140703 V at nominal load. Do not reject solely because g is negative.
- Zero drives: both drives zero gives exact zero voltages/currents while retaining g.
- Singular/unstable: R4=30000 with nominal R1–R3 and RL=5000 gives zero total conductance; use neighboring representable cases to distinguish this boundary from a tiny positive denominator. R4=40000 is strictly invalid.
- Compliance: reduce output limit below required Vo, common limit below Vx, or current limit below required absolute amplifier current; each must reject independently.

**Mandatory diagram topology:** Exact four-resistor Howland network: R2 positive feedback meets loaded Vx, R4 negative feedback remains separate; show signed current output. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `r_one`, `r_two`, `r_three`, `r_four`, `positive_drive`, `negative_drive`, `load_resistance`, `output_conductance`, `load_voltage`, `load_current`, `amplifier_voltage`, `amplifier_current`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 069. `precision.brokaw_first_order_trim` — Brokaw bandgap first-order resistor trim

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Precision developer; T2Precision tester; V2Precision source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/precision.md#5-precisionbrokaw_first_order_trim--intermediate); [complete JSON example vector](../../research/expansion/precision.json), selected by `id=precision.brokaw_first_order_trim`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `thermal_voltage=0.025851999786435535` V; `delta_vbe=0.05375772229141229` V; `reference_voltage=1.25` V.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use characterized beta, GBW, swing/common-mode/current limits or reference-device data; preserve the exact active netlist.

**Semantic novelty decision — retain as one distinct physical problem.**

Choose the common-emitter resistor for first-order PTAT/CTAT temperature cancellation in a Brokaw reference. Nearest baseline `thermal.resistor_tcr_drift` changes a single resistor with temperature; this active reference combines two junction densities and a resistor ratio to cancel a junction-voltage slope. It is not a sensor thermometer, generic Shockley forward-current inversion, zener shunt regulator or duplicate current mirror: the design output is the resistor ratio that synthesizes a compensated voltage.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: assert all values, `2 I R1=0.6 V` and the zero local derivative.
- Area-ratio change: set N=64. Since ln(64)=2 ln(8), delta VBE and branch current double; R1/R2 and R1 halve; the synthesized reference remains 1.25 V.
- Resistor scaling: double R2 at unchanged characterized VBE/slope; inferred current halves and synthesized R1 doubles. For physical use, recharacterize VBE/slope if this changes the real bias point.
- Approaching N=1 from above: delta VBE/current tend to zero while required resistor ratio diverges. Exactly N=1 must reject, not divide by zero. Do not impose a made-up minimum N larger than 1.
- Invalid: nonpositive T, R2 or VBE; N<=1; zero or positive VBE slope.

**Mandatory diagram topology:** Unequal-area BJTs share bases; Q2 emitter has R2, common emitter path R1 carries 2I; equal collector branches feed servo amplifier. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `temperature`, `area_ratio`, `ptat_resistance`, `base_emitter_voltage`, `base_emitter_slope`, `delta_vbe`, `branch_current`, `common_resistor`, `reference_voltage`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 070. `precision.log114_current_ratio` — LOG114 resistor-referenced logarithmic front end

- **Required / level:** Must-have calculator + diagram + docs; basic.
- **Accountable:** D2Precision developer; T2Precision tester; V2Precision source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/precision.md#6-precisionlog114_current_ratio--basic); [complete JSON example vector](../../research/expansion/precision.json), selected by `id=precision.log114_current_ratio`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `reference_current=1e-06` A; `current_ratio=100.0` 1; `local_transimpedance=1628.6043071371942` ohm.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use characterized beta, GBW, swing/common-mode/current limits or reference-device data; preserve the exact active netlist.

**Semantic novelty decision — retain as one distinct physical problem.**

Design nominal electrical current compression around the LOG114 with a resistor-derived reference current. Nearest baseline `noise.decibel_to_linear` is unit/ratio conversion without a circuit; here the device enforces virtual-ground current inputs and generates a logarithmic voltage with current-dependent incremental transimpedance. Nearest new TIA has a linear resistor transfer and bandwidth compensation; this frontend intentionally compresses many decades. No physical light conversion is included.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: assert every value.
- Equal currents: signal=1 µA with nominal reference resistor gives ratio=1, log decades=0 and output=0 exactly; local transimpedance=162860.43071371942 ohm.
- Range endpoints: signal=100 pA gives -1.5 V and signal=10 mA gives +1.5 V with the nominal 1 µA reference.
- One-decade increase adds exactly 0.375 V in the nominal model while decreasing local transimpedance tenfold.
- Reference-resistor boundaries: 1250 ohm gives 2 mA reference current; 25 Gohm gives 100 pA. Both are admitted, subject to all stated external conditions.
- Near unity: use the next representable current above 1 µA; require a positive log output instead of subtraction-induced zero.
- Invalid: signal below 100 pA, above 10 mA, zero/negative/nonfinite; resistor outside [1250,25000000000] ohm or nonfinite.

**Mandatory diagram topology:** LOG114 I1 receives signal; VREF drives I2 through external resistor; COM/VCM/VREF grounds and ±5 V rails remain explicit. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `signal_current`, `reference_resistance`, `reference_current`, `current_ratio`, `log_output`, `local_transimpedance`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

## Logic — 6 required items

Lane accountability: **D2Logic / T2Logic / V2Logic**. Levels: 2 basic, 2 intermediate, 2 expert.

### 071. `logic.guaranteed_noise_margins` — Guaranteed logic-level noise margins with ground offset

- **Required / level:** Must-have calculator + diagram + docs; basic.
- **Accountable:** D2Logic developer; T2Logic tester; V2Logic source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/logic.md#1-logicguaranteed_noise_margins--basic); [complete JSON example vector](../../research/expansion/logic.json), selected by `id=logic.guaranteed_noise_margins`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `high_margin=0.2` V; `low_margin=0.4` V; `minimum_margin=0.2` V.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use guaranteed logic data, characterized Cpd/metastability/logical-effort parameters or a valid FIFO arrival/service envelope.

**Semantic novelty decision — retain as one distinct physical problem.**

Check whether a guaranteed output-high/low pair remains recognizable after ground offset and worst-direction line noise. Nearest baseline: `timing.schmitt_rc_delay_bounds`, which predicts threshold-crossing time through an RC network. This model instead compares static guaranteed voltage regions between two ICs; it has no capacitor, RC time or switching threshold prediction. Nearest new model: current fanout below; valid voltage margins do not establish available drive current and vice versa.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: every result above, relative tolerance 1e-6.
- Equal references and no noise: set offset and both noise inputs to zero; both margins and minimum are 0.4 V.
- Ground-sign check: retain nominal noise and set offset to −0.1 V; high=0.4 V, low=0.2 V, minimum=0.2 V.
- Exact zero boundary with binary-exact values: VOH=2.5, VOL=0.5, VIH=2, VIL=1, offset=0.25, high_noise=0.25, low_noise=0.25 gives high=0, low=0.5, minimum=0 V.
- Incompatible but valid: nominal inputs with offset=0.4 V give high=−0.1 V, low=0.7 V and minimum=−0.1 V. Do not clamp or reject negative results.
- Invalid: negative noise magnitude, negative guaranteed level, VOH≤VOL, or VIH≤VIL. Reject nonfinite inputs through the existing engine boundary.

**Mandatory diagram topology:** Driver/receiver voltage ladders have separate ground references; positive offset points from driver ground toward receiver ground. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `output_high_min`, `output_low_max`, `input_high_min`, `input_low_max`, `ground_offset`, `high_noise`, `low_noise`, `high_margin`, `low_margin`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 072. `logic.dc_current_fanout` — Logic output DC-current fanout

- **Required / level:** Must-have calculator + diagram + docs; basic.
- **Accountable:** D2Logic developer; T2Logic tester; V2Logic source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/logic.md#2-logicdc_current_fanout--basic); [complete JSON example vector](../../research/expansion/logic.json), selected by `id=logic.dc_current_fanout`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `high_fanout=200` count; `low_fanout=20` count; `sink_headroom=0.0008` A.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use guaranteed logic data, characterized Cpd/metastability/logical-effort parameters or a valid FIFO arrival/service envelope.

**Semantic novelty decision — retain as one distinct physical problem.**

Determine the number of input pins one output can drive while preserving the output's characterized high/low voltage guarantees. Nearest baseline: `timing.i2c_pullup_bounds`, which solves an open-drain resistor/edge-time design window. Here a push-pull output must meet two static input-current sums; no pullup resistance or rise-time equation is used. This is not `dc.ohm_current`: there is no assumed resistive input model, and bipolar high/low input currents can differ greatly.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: all five outputs in the linked research vector.
- Fractional ratio must round downward: source=0.0039 A, receiver_high=0.0002 A, retain nominal sink/low input, loads=18; high=19, low=20, maximum=19, source headroom=0.0003 A, sink headroom=0.0008 A.
- Exactly exhausted low drive: nominal with loads=20; source headroom=0.0036 A, sink headroom=0; fanout outputs unchanged.
- Overloaded but valid: nominal with loads=21; source headroom=0.00358 A, sink headroom=−0.0004 A. Preserve the sign.
- Zero capabilities and zero loads: source=sink=0, positive receiver currents; all five outputs are zero.
- Invalid: either receiver current zero/negative, either driver current negative, fractional/negative load count, or nonfinite input. Tests near integer ratios should use binary-exact currents where exact floor placement matters.

**Mandatory diagram topology:** One driver fans out through explicit junctions to representative receivers; distinguish high-state source and low-state sink current directions. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `source_limit`, `sink_limit`, `receiver_high_current`, `receiver_low_current`, `load_count`, `maximum_fanout`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 073. `logic.cmos_package_power` — CMOS logic internal and output-load power budget

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Logic developer; T2Logic tester; V2Logic source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/logic.md#3-logiccmos_package_power--intermediate); [complete JSON example vector](../../research/expansion/logic.json), selected by `id=logic.cmos_package_power`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `static_power=6.6e-05` W; `internal_power=0.026136` W; `average_current=0.01454` A.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use guaranteed logic data, characterized Cpd/metastability/logical-effort parameters or a valid FIFO arrival/service envelope.

**Semantic novelty decision — retain as one distinct physical problem.**

Budget the supply demand of CMOS logic using the manufacturer's internal Cpd characterization plus independently switched output loads. Nearest baseline: `mosfet.gate_charge_supply` predicts external gate-drive charge delivery for a power transistor; `mosfet.output_capacitance_hard_turn_on_loss` predicts dissipated Coss energy during hard turn-on. This model instead includes CMOS internal-node/through-current characterization, active logic-section multiplicity, whole-package leakage and different input/output rates. Neither prior model represents this package operating-power budget. It is not converter efficiency or a thermal solver.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: every output above.
- Static limit: both rates zero; static=total=66e-6 W, internal=load=0, average current=20e-6 A.
- No output switching: set output rate zero only; total=0.026202 W, average current=0.00794 A, load=0; internal/static unchanged.
- No load and no quiescent current: CL=Iq=0; total=internal=0.026136 W, average current=0.00792 A; static/load=0.
- Doubling both rates doubles internal/load power, not leakage. Doubling supply at fixed characterized parameters multiplies dynamic terms by four and leakage power by two; this is an algebraic sensitivity test, not permission to exceed real device voltage limits.
- Invalid: zero/negative supply, negative rates/current/capacitance, zero or fractional active sections, nonfinite inputs.

**Mandatory diagram topology:** Supply feeds characterized internal Cpd and separate physical output load; internal equivalent capacitance is not an external circuit part. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `supply_voltage`, `active_sections`, `power_capacitance`, `input_cycle_rate`, `load_capacitance`, `output_cycle_rate`, `static_power`, `internal_power`, `load_power`, `total_power`, `average_current`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 074. `logic.synchronizer_metastability` — Synchronizer metastability reliability budget

- **Required / level:** Must-have calculator + diagram + docs; expert.
- **Accountable:** D2Logic developer; T2Logic tester; V2Logic source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/logic.md#4-logicsynchronizer_metastability--expert); [complete JSON example vector](../../research/expansion/logic.json), selected by `id=logic.synchronizer_metastability`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `log10_chain_mtbf=13.371779276130074` 1; `log10_system_mtbf=11.371779276130074` 1; `resolution_margin=1.0047045788153165e-10` s.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use guaranteed logic data, characterized Cpd/metastability/logical-effort parameters or a valid FIFO arrival/service envelope.

**Semantic novelty decision — retain as one distinct physical problem.**

Quantify the exponential dependence of asynchronous sampling reliability on available regeneration time. Nearest baseline: `timing.synchronous_setup_slack` checks deterministic synchronous arrival timing, whereas this model assumes asynchronous aperture encounters and characterizes unresolved metastability statistically. `thermal.arrhenius_acceleration` is another exponential model but describes temperature-activated wear, not bistable regeneration. No clock jitter, sampling-aperture SNR or measurement model is claimed.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: all four outputs in the linked research vector.
- Shorter resolution: nominal with tres=1e-9 s gives log chain=4.6858896380650366, log system=2.6858896380650366, required time unchanged, margin=−8.995295421184684e-10 s. Negative margin must remain visible.
- N=1 raises system log MTBF by exactly two relative to nominal and lowers required time by tau·ln(100).
- Tiny target: nominal with target=1e-8 s gives required time=0 and margin=2e-9 s; MTBF outputs are unchanged.
- Zero resolution: default rates/fit/N give chain log=−4 and system log=−6; required time remains nominal and margin is its negative. This checks mathematical boundary behavior, not device accuracy there.
- Large exponent: tres=1e-6 s gives chain log=8681.8896380650366; it remains finite without exp(20000). The result is an extreme mathematical extrapolation only.
- Invalid: zero/negative rate, T0, tau or target; negative resolution; N<1 or fractional; T0=1e-6 s with fdata=1e6 1/s reaches the disallowed aperture-factor equality. Nonfinite input must fail before evaluation.

**Mandatory diagram topology:** Two destination-clock registers receive asynchronous data; common clock branches explicitly and interstage resolution interval is marked. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `clock_frequency`, `data_transition_rate`, `aperture_parameter`, `resolution_constant`, `resolution_time`, `chain_count`, `log10_system_mtbf`, `required_resolution_time`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 075. `logic.three_stage_logical_effort` — Three-stage CMOS logical-effort gate sizing

- **Required / level:** Must-have calculator + diagram + docs; expert.
- **Accountable:** D2Logic developer; T2Logic tester; V2Logic source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/logic.md#5-logicthree_stage_logical_effort--expert); [complete JSON example vector](../../research/expansion/logic.json), selected by `id=logic.three_stage_logical_effort`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `optimal_stage_effort=4` 1; `second_input_capacitance=9e-15` F; `path_delay=3.2e-10` s.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use guaranteed logic data, characterized Cpd/metastability/logical-effort parameters or a valid FIFO arrival/service envelope.

**Semantic novelty decision — retain as one distinct physical problem.**

Choose two internal gate sizes of a fixed three-stage CMOS path under endpoint capacitance constraints. Nearest baseline: `opamps.dominant_pole_gain_bandwidth` models analog closed-loop bandwidth; `timing.synchronous_setup_slack` consumes a supplied path delay. Neither optimizes CMOS device sizing under gate-topology logical effort. Neighbor `logic.dc_current_fanout` deals with static input current, not capacitance ratios or propagation delay. Oscillator stage-period calculations, if present elsewhere, are not the endpoint-constrained gate-sizing problem here.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: all five outputs in the linked research vector.
- All-inverter alternate: C1=1e-15 F, CL=64e-15 F, all g=1, P=3, tau=1e-11 s; f=4, C2=4e-15 F, C3=16e-15 F, D=15, delay=1.5e-10 s.
- Scale both endpoint capacitances by 1000: internal capacitances also scale by 1000, while effort and both delays remain nominal.
- Equal loads: C1=CL=1e-15 F, all g=1, P=3 and tau=1e-11 s gives f=1, C2=C3=1e-15 F, D=6 and delay=6e-11 s.
- Ideal parasitic boundary P=0 is accepted; nominal effort/sizes remain, D=12 and delay=2.4e-10 s.
- Independent minimization check: perturb either internal capacitance in the alternate all-inverter example and sum g_i·h_i+p_i directly. The sum cannot fall below 15. This tests the physical objective, not a snapshot of the same implementation expression.
- Invalid: endpoint C≤0, process unit≤0, negative P or any g<1. Nonfinite quantities fail at engine input validation.

**Mandatory diagram topology:** Three generic unbranched gates with distinct logical efforts/input capacitances feed one final load; mark total path delay. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `first_input_capacitance`, `final_load_capacitance`, `first_logical_effort`, `second_logical_effort`, `third_logical_effort`, `optimal_stage_effort`, `second_input_capacitance`, `third_input_capacitance`, `path_delay`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 076. `logic.fifo_envelope_depth` — Hardware FIFO depth from burst and service guarantees

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Logic developer; T2Logic tester; V2Logic source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/logic.md#6-logicfifo_envelope_depth--intermediate); [complete JSON example vector](../../research/expansion/logic.json), selected by `id=logic.fifo_envelope_depth`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `backlog_bound=89` word; `usable_depth=89` word; `delay_bound=2.28e-06` s.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use guaranteed logic data, characterized Cpd/metastability/logical-effort parameters or a valid FIFO arrival/service envelope.

**Semantic novelty decision — retain as one distinct physical problem.**

Size actual on-chip FIFO data storage for a digital interface with bounded burst traffic and delayed guaranteed read service. Nearest baseline: `timing.uart_sampling_margin` checks receiver bit-sampling alignment; timer division checks a generated rate. Neither predicts queue occupancy or memory capacity. New metastability reliability evaluates whether CDC sampling fails; this model instead budgets the backlog created by otherwise correct clock-domain and arbitration latency. It adds a hardware storage design problem, not a different protocol name for a timing-margin calculation.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: all four outputs in the linked research vector.
- Equal-rate limit: set arrival rate=service rate=50e6 word/s; backlog=depth=114 words, bits=3648, delay=2.28e-6 s. The backlog stays bounded; do not reject equality.
- One finite burst: set arrival rate=0; backlog=depth=64, bits=2048, delay=2.28e-6 s.
- Minimum admitted asynchronous word-phase latency: T=1/50e6=20e-9 s with other nominal inputs gives backlog=64.5, depth=65, bits=2080, delay=1.3e-6 s. The fractional bound rounds upward.
- Binary-exact rounding example: b=3 words, r=2 word/s, R=4 word/s, T=0.75 s, width=8; backlog=4.5, depth=5, bits=40, delay=1.5 s.
- Invalid: R<r; R≤0; T<1/R; fractional/zero/negative burst or word width; negative arrival rate; nonfinite inputs. An invalid envelope in real traffic is outside what numerical validation can detect.

**Mandatory diagram topology:** Burst source feeds FIFO then guaranteed-rate reader; arrival/service envelope inset shows backlog, latency and word/bit capacity. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `burst_words`, `arrival_rate`, `service_rate`, `service_latency`, `word_bits`, `backlog_bound`, `usable_depth`, `storage_bits`, `delay_bound`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

## Motors — 6 required items

Lane accountability: **D2Motors / T2Motors / V2Motors**. Levels: 1 basic, 3 intermediate, 2 expert.

### 077. `motors.pm_dc_load_equilibrium` — PM DC motor loaded torque-speed equilibrium

- **Required / level:** Must-have calculator + diagram + docs; basic.
- **Accountable:** D2Motors developer; T2Motors tester; V2Motors source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/motors.md#1-motorspm_dc_load_equilibrium); [complete JSON example vector](../../research/expansion/motors.json), selected by `id=motors.pm_dc_load_equilibrium`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `speed=166.66666666666666` rad/s; `current=3.6666666666666665` A; `friction_loss=27.77777777777778` W.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use consistent machine, peak/RMS, torque and gear conventions with characterized constants; preserve the stated operating quadrant.

**Semantic novelty decision — retain as one distinct physical problem.**

Determine the coupled steady operating point, not merely current through a resistor. A load increase changes armature current, winding drop, back EMF and mechanical speed together. Closest baseline is `dc.resistive_source_matching`; a motor's generated EMF is speed-dependent and mechanical drag is not a resistor load. Unlike the geared-ramp item, this model solves an unknown steady speed from imposed voltage and drag; it contains no acceleration or transmission.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: all six outputs in the linked research vector and both constitutive equations.
- Ideal unloaded limit: defaults except `load_torque=0, viscous_drag=0` gives `speed=240`, `current=0`, `back_emf=24`, and all three power outputs zero. Do not call this real motor no-load current.
- Stall boundary: defaults except `load_torque=1.2` gives `speed=0`, `current=12`, `back_emf=0`, `shaft_power=0`, `copper_loss=288`, `friction_loss=0`.
- Unenergized limit: `voltage=0, load_torque=0` gives all outputs zero, including with positive drag.
- Reject load above `1.2 N m` at the nominal supply, zero/nonpositive `K` or `R`, and negative supply/load/drag.

**Mandatory diagram topology:** Supply, winding R and opposing back-EMF form electrical loop; separate mechanical balance shows load and viscous drag. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `voltage`, `armature_resistance`, `motor_constant`, `viscous_drag`, `load_torque`, `speed`, `current`, `back_emf`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 078. `motors.stepper_static_microstep` — Stepper microstep torque and static load deflection

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Motors developer; T2Motors tester; V2Motors source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/motors.md#2-motorsstepper_static_microstep); [complete JSON example vector](../../research/expansion/motors.json), selected by `id=motors.stepper_static_microstep`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `microstep_angle=0.001963495408493621` rad; `incremental_torque=0.0490085701647803` N m; `angular_stiffness=24.49489742783178` N m/rad.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use consistent machine, peak/RMS, torque and gear conventions with characterized constants; preserve the stated operating quadrant.

**Semantic novelty decision — retain as one distinct physical problem.**

Separate controller resolution from mechanical accuracy and restoring torque. The baseline clock divider only quantizes electrical timing; it cannot predict rotor lag or torque. This item has a spatial sinusoidal torque law, distinct from all DC/PMSM speed or energy models. It does not calculate a pulse generator or dynamic pull-out speed.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: all four outputs in the linked research vector; substitution of `load_lag` into the torque law returns `0.1 N m`.
- Full-step unloaded limit: defaults except `microsteps_per_step=1, load_torque=0` gives angles `0.031415926535897934` and `0`, torque `0.5`, stiffness `25`.
- Increasing `m` reduces `incremental_torque` and command angle but leaves static lag and stiffness unchanged for the same load/current vector.
- As `load_torque` approaches `0.5` from below, `load_lag` approaches `pi/100=0.031415926535897934 rad` and stiffness approaches zero. Do not round an admitted subcritical value up to holding torque.
- Reject `microsteps_per_step=1.5`, `full_steps_per_turn=202`, zero holding torque, negative load and load equal to or exceeding holding torque.

**Mandatory diagram topology:** Rotor axis lags commanded current vector; distinguish command microstep, static load lag, restoring torque and angular stiffness. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `full_steps_per_turn`, `microsteps_per_step`, `holding_torque`, `load_torque`, `microstep_angle`, `incremental_torque`, `load_lag`, `angular_stiffness`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 079. `motors.geared_axis_acceleration` — Geared-axis acceleration current and voltage requirement

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Motors developer; T2Motors tester; V2Motors source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/motors.md#3-motorsgeared_axis_acceleration); [complete JSON example vector](../../research/expansion/motors.json), selected by `id=motors.geared_axis_acceleration`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `reflected_inertia=0.0002` kg m^2; `motor_acceleration=400` rad/s^2; `required_final_voltage=18.666666666666668` V.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use consistent machine, peak/RMS, torque and gear conventions with characterized constants; preserve the stated operating quadrant.

**Semantic novelty decision — retain as one distinct physical problem.**

Translate a required mechanical ramp into motor drive current and voltage. The distinguishing laws are two-inertia acceleration, ratio-squared kinetic-energy reflection and lossy transmission torque. This is not an inverse of the steady DC load model: both shaft speeds are prescribed, torque accelerates inertia, and gearbox efficiency acts only across the transmission.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: all six outputs in the linked research vector. Verify torque on both sides of the gearbox independently.
- Lossless gear: change only `gear_efficiency=1`; outputs are `reflected_inertia=0.0002`, `motor_acceleration=400`, `motor_torque=0.2`, `armature_current=2.5`, `motor_final_speed=200`, `required_final_voltage=18.5`.
- Constant speed: set `initial_speed=final_speed=20`; acceleration becomes zero, torque is `0.4/9=0.044444444444444446 N m`, current `0.5555555555555556 A`, end voltage `16.555555555555557 V`; inertia output remains `0.0002`.
- `gear_ratio=1, gear_efficiency=1` reduces torque to `(Jm+Jl)*alpha_load+Tload`, not to just load torque.
- Reject a decreasing speed ramp, zero duration, zero ratio, efficiency outside `(0,1]`, negative inertias or two zero inertias. Do not reject a zero load torque or one zero inertia.

**Mandatory diagram topology:** Motor inertia, lossy gearbox and load inertia form mechanical chain; ratio, efficiency, acceleration, current and voltage stay distinct. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `motor_inertia`, `load_inertia`, `gear_ratio`, `gear_efficiency`, `load_torque`, `final_speed`, `reflected_inertia`, `motor_torque`, `armature_current`, `motor_final_speed`, `required_final_voltage`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 080. `motors.resistive_dynamic_braking` — PM motor resistive dynamic braking transient

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Motors developer; T2Motors tester; V2Motors source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/motors.md#4-motorsresistive_dynamic_braking); [complete JSON example vector](../../research/expansion/motors.json), selected by `id=motors.resistive_dynamic_braking`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `mechanical_tau=0.8` s; `speed=36.787944117144235` rad/s; `winding_energy=2.161661791908468` J.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use consistent machine, peak/RMS, torque and gear conventions with characterized constants; preserve the stated operating quadrant.

**Semantic novelty decision — retain as one distinct physical problem.**

Predict how a spinning PM machine dissipates mechanical energy after disconnecting its source. The resistor current is generated by speed and produces opposing torque, which changes that speed. Closest baseline `transients.rl_step` holds electrical excitation independently of mechanics; its time constant is `L/R`, not `J R/K^2`. Closest sibling DC model is a forced steady operating point, not free deceleration. No capacitor, battery, switching spike or protection coordination is involved.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: all seven outputs and `brake_energy+winding_energy+J*speed^2/2=10 J`.
- Reduced-model `time=0`: `tau=0.8`, `speed=100`, current `2.5`, torque `-0.25`, and travel/both energy outputs zero.
- Tiny time `1e-20 s`: first-order travel is `1e-18 rad`, brake energy `1.875e-19 J`, winding energy `6.25e-20 J`. These positive values must not vanish through `1-exp(-x)` cancellation.
- Long time tends to zero current/speed/torque, `80 rad` travel, `7.5 J` brake heat and `2.5 J` winding heat. No finite “stop time” should be fabricated.
- `brake_resistance=0` yields zero external heat, all removed energy in the winding, and `tau=0.2 s` with other nominal inputs.
- `initial_speed=0` gives zero motion/current/torque/heat while retaining the parameter-defined positive time constant.
- Reject negative time/speed/brake resistance, zero/nonpositive inertia, armature resistance or motor constant.

**Mandatory diagram topology:** Disconnected spinning generator closes through winding/brake resistors; current leaves positive generator terminal and torque opposes rotation. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `inertia`, `armature_resistance`, `brake_resistance`, `initial_speed`, `mechanical_tau`, `speed`, `braking_current`, `electromagnetic_torque`, `brake_energy`, `winding_energy`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 081. `motors.salient_pmsm_dq_operating_point` — Salient PMSM steady dq torque and voltage

- **Required / level:** Must-have calculator + diagram + docs; expert.
- **Accountable:** D2Motors developer; T2Motors tester; V2Motors source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/motors.md#5-motorssalient_pmsm_dq_operating_point); [complete JSON example vector](../../research/expansion/motors.json), selected by `id=motors.salient_pmsm_dq_operating_point`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `electrical_speed=400` rad/s; `d_voltage=-5` V; `stator_copper_loss=37.5` W.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use consistent machine, peak/RMS, torque and gear conventions with characterized constants; preserve the stated operating quadrant.

**Semantic novelty decision — retain as one distinct physical problem.**

Compute a salient three-phase machine operating point from d/q current commands. The distinguishing physics is the interaction of permanent flux, current-generated saliency flux and cross-axis speed EMF. Neither AC impedance nor the DC torque-speed line models this. This is not field-oriented controller synthesis, MTPA optimization or inverter-modulation sizing.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: all eight outputs, plus the power identity above.
- Surface-magnet limit: set `d_inductance=q_inductance`; torque becomes `1.5*p*psi*iq`, independent of d current, while voltage and losses retain d-current effects.
- Zero speed: speed voltage terms vanish, so `vd=Rs*id`, `vq=Rs*iq`, and converted power is zero even if torque is nonzero.
- Generating case: keep nominal inputs except `q_current=-10`. Then `electrical_speed=400`, `d_voltage=3`, `q_voltage=17`, `phase_voltage_peak=sqrt(298)=17.26267650163207`, `phase_current_peak=sqrt(125)`, torque `-3.15`, converted power `-315`, copper loss `37.5`; terminal electrical power is `-277.5 W`.
- `d_current=q_current=0` yields zero torque/current/copper loss; rotating magnets still require `vq=omega_e*psi`, not zero voltage.
- Reject fractional/nonpositive pole pairs, nonpositive inductances/flux and negative stator resistance. Accept zero resistance as the ideal lossless limit; do not reject negative d/q currents or speed.

**Mandatory diagram topology:** Rotor-oriented d/q axes show positive d flux and nominal negative-d/positive-q current; voltage equations and power flow preserve peak conventions. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `magnet_flux`, `d_current`, `q_current`, `electrical_speed`, `d_voltage`, `q_voltage`, `electromagnetic_torque`, `converted_power`, `stator_copper_loss`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 082. `motors.induction_slip_power` — Induction motor slip torque and air-gap power split

- **Required / level:** Must-have calculator + diagram + docs; expert.
- **Accountable:** D2Motors developer; T2Motors tester; V2Motors source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/motors.md#6-motorsinduction_slip_power); [complete JSON example vector](../../research/expansion/motors.json), selected by `id=motors.induction_slip_power`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `synchronous_speed=157.07963267948966` rad/s; `rotor_speed=149.22565104551518` rad/s; `breakdown_torque=180.43194441580198` N m.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use consistent machine, peak/RMS, torque and gear conventions with characterized constants; preserve the stated operating quadrant.

**Semantic novelty decision — retain as one distinct physical problem.**

Estimate induction-machine torque and rotor heating from slip and a Thevenin equivalent. The rotor's speed-dependent effective resistance partitions air-gap power between heat and mechanical conversion. Baseline AC impedance calculates only an electrical response; source-matching lacks synchronous speed, rotor slip and electromechanical power conversion. This item is not a DC motor variant or a slip-to-speed unit conversion.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: all nine outputs. Independently verify `Pconv=Te*rotor_speed` and `Pag=Pcu2+Pconv`.
- Synchronous limit `slip=0`: rotor current, air-gap power, torque, rotor copper loss and converted power are exactly zero; rotor speed equals synchronous speed. Both breakdown outputs retain their parameter-defined nominal values. No division-by-zero error is allowed.
- Locked rotor `slip=1`: speed and converted power are zero; current `100.3113655692617 A`, air-gap and rotor copper power `12074.844074844075 W`, torque `76.87084486301274 N m`. Synchronous and breakdown outputs remain unchanged.
- At `slip=0.19402850002906638`, calculated torque equals the independent peak value. Neighboring slips on both sides give smaller torque for these defaults.
- With `Rth=0`, analytic checks are `s_peak=R2/(Xth+X2)` and `T_peak=3*Vth^2/[2*omega_s*(Xth+X2)]`.
- Increasing `rotor_resistance` changes peak slip but not peak torque if other Thevenin quantities remain fixed. A peak slip above one is permitted and clearly labeled as outside the motoring interval.
- At zero Thevenin voltage, electrical power/current/torque outputs vanish, but synchronous speed and the parameter-defined peak-slip location remain defined.
- Reject slip below zero or above one, zero frequency/rotor resistance, fractional/nonpositive pole pairs, negative impedances and zero total leakage reactance.

**Mandatory diagram topology:** Per-phase Thevenin circuit feeds effective R2/s; show R2 and slip separately plus air-gap power split into rotor heat and conversion. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `slip`, `thevenin_voltage`, `thevenin_resistance`, `thevenin_reactance`, `rotor_resistance`, `rotor_reactance`, `rotor_current`, `air_gap_power`, `rotor_copper_loss`, `converted_power`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

## Passives — 6 required items

Lane accountability: **D2Passives / T2Passives / V2Passives**. Levels: 1 basic, 2 intermediate, 3 expert.

### 083. `passives.mlcc_logarithmic_aging` — Ferroelectric capacitor logarithmic aging

- **Required / level:** Must-have calculator + diagram + docs; basic.
- **Accountable:** D2Passives developer; T2Passives tester; V2Passives source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/passives.md#1-passivesmlcc_logarithmic_aging--basic); [complete JSON example vector](../../research/expansion/passives.json), selected by `id=passives.mlcc_logarithmic_aging`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `elapsed_decades=4` decade; `fractional_loss=0.08` 1; `aged_capacitance=9.2e-06` F.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use actual component-fit/history/switch parameters; preserve hidden state, constitutive nonlinearity and network loading.

**Semantic novelty decision — retain as one distinct physical problem.**

Estimate ferroelectric dielectric-permittivity loss with elapsed time after deaging. Closest baseline: `thermal.arrhenius_acceleration`, which scales thermally activated life exposure, not capacitance. `thermal.resistor_tcr_drift` is an instantaneous temperature law, not a logarithmic material-relaxation clock. No baseline includes capacitor aging.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: all three values in the linked research vector.
- Reference boundary: age=3600 s gives exactly zero decades/loss and the reference capacitance.
- No-aging boundary: aging rate=0 keeps C constant at any otherwise valid age.
- At age=36,000 s and rate=2, C=9.8 µF; this is one decade, not ten.
- Reject age below 3600 s, zero/negative capacitance, negative aging rate, and any fit yielding loss≥1. Example: rate=25 and age=36,000,000 s is exactly zero capacitance and invalid.
- A zero-aging result does not imply zero DC-bias or temperature effects.

**Mandatory diagram topology:** Semilog time-since-deaging plot anchors capacitance at one hour and marks later aged value; separate ferroelectric component symbol. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `capacitance_one_hour`, `aging_percent_decade`, `age`, `aged_capacitance`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 084. `passives.dielectric_absorption_recovery` — Capacitor dielectric-memory voltage recovery

- **Required / level:** Must-have calculator + diagram + docs; expert.
- **Accountable:** D2Passives developer; T2Passives tester; V2Passives source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/passives.md#2-passivesdielectric_absorption_recovery--expert); [complete JSON example vector](../../research/expansion/passives.json), selected by `id=passives.dielectric_absorption_recovery`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `memory_voltage=7.823809897829776` V; `recovery_time_constant=9.900990099009901` s; `recovered_voltage=0.04924980040079833` V.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use actual component-fit/history/switch parameters; preserve hidden state, constitutive nonlinearity and network loading.

**Semantic novelty decision — retain as one distinct physical problem.**

A capacitor can recover terminal voltage after being shorted because slow polarization retains charge. Closest baseline: `transients.capacitor_charge_sharing`. That baseline starts from two externally specified capacitor voltages and models their connection through a resistor. Here the hidden state is established by a finite-duration charge, depleted during a short, then released after short removal. The same conservation laws apply, but the design problem is a nonideal single component's history-dependent dielectric memory, not another ideal RC delay or a renamed redistribution output. The finite charge/short preparation is essential; do not replace it with an arbitrary initial hidden voltage.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: complete table.
- At recovery_time=0, recovered_voltage=0 while the other outputs are unchanged.
- At charge_time=0 or charge_voltage=0, all voltage outputs are zero, but tau remains positive.
- Negating charge_voltage negates all three voltages and leaves tau unchanged.
- Long charge with short_time=0: V_inf tends to V_ch C_a/(C_m+C_a). Long short forces both recovery voltages toward zero. Long recovery approaches V_inf, never above it for positive excitation.
- Small positive recovery time has slope `V_a0/(R_a C_m)`; nominal values at 1e−12 s give approximately 7.82380989783e−15 V, not zero from subtractive cancellation.
- Reject nonpositive capacitances/resistance and negative times. A finite instrument input resistance is outside this model, not an extra input silently ignored.

**Mandatory diagram topology:** Cm parallels series Ra–Ca branch with isolated hidden node; open recovery terminals and explicit charge→short→open history strip. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `main_capacitance`, `absorption_capacitance`, `absorption_resistance`, `memory_voltage`, `recovered_voltage`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 085. `passives.mlcc_bias_charge_energy` — Voltage-dependent capacitor usable charge and energy

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Passives developer; T2Passives tester; V2Passives source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/passives.md#3-passivesmlcc_bias_charge_energy--intermediate); [complete JSON example vector](../../research/expansion/passives.json), selected by `id=passives.mlcc_bias_charge_energy`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `upper_capacitance=5.2e-06` F; `usable_charge=4.2e-05` C; `energy_equivalent_capacitance=6.64e-06` F.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use actual component-fit/history/switch parameters; preserve hidden state, constitutive nonlinearity and network loading.

**Semantic novelty decision — retain as one distinct physical problem.**

Calculate usable charge and energy when a ceramic capacitor's differential capacitance changes across the operating window. Closest baseline: `transients.capacitor_constant_current` assumes a fixed C and computes ideal stored energy. `mosfet.output_capacitance_hard_turn_on_loss` accepts an already energy-equivalent C; it does not integrate a C(V) characteristic. This model introduces a nonlinear constitutive relation, not extra fixed-capacitance outputs.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal table and independent charge/energy integrals.
- Constant-C boundary C_fit=C_0 gives C_Q=C_E=C_0, Q=C_0 ΔV and W=C_0 Δ(V²)/2.
- With nominal fit and window 0…10 V: C_hi=4 µF, Q=70 µC, W=300 µJ, C_Q=7 µF, C_E=6 µF.
- As window width approaches zero around a positive voltage, both equivalent capacitances approach local differential C; both dimensional amounts approach zero. Test a narrow representable window without cancellation from antiderivative subtraction.
- Reject equal/reversed voltage endpoints, upper voltage above V_fit, negative lower voltage, nonpositive capacitances/V_fit, and C_fit>C_0.

**Mandatory diagram topology:** Differential C(V) line with voltage window; distinguish ∫C dV charge area from ∫VC dV energy annotation. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `zero_bias_capacitance`, `fit_capacitance`, `fit_voltage`, `lower_voltage`, `upper_voltage`, `usable_charge`, `usable_energy`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 086. `passives.resistor_excess_noise` — Biased resistor excess noise from noise index

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Passives developer; T2Passives tester; V2Passives source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/passives.md#4-passivesresistor_excess_noise--intermediate); [complete JSON example vector](../../research/expansion/passives.json), selected by `id=passives.resistor_excess_noise`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `decade_voltage_ratio=1e-07` V/V; `band_decades=3` decade; `excess_noise_density=2.0839733249330516e-07` V/sqrt(Hz).
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use actual component-fit/history/switch parameters; preserve hidden state, constitutive nonlinearity and network loading.

**Semantic novelty decision — retain as one distinct physical problem.**

Predict DC-bias-driven resistance-fluctuation noise from manufacturer noise index. Closest baseline: `noise.johnson_resistor` is bias-independent white thermal noise. `noise.uncorrelated_rms_sum` merely combines amplitudes. Here a new physical source has 1/f power density, an unusual manufacturer normalization and logarithmic bandwidth dependence. No white-noise sum or general dB converter is being counted again.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal complete table.
- A single decade gives exactly |V_DC|a; two decades increase RMS by sqrt(2), not 2.
- Zero DC bias gives zero excess-noise amplitudes, not zero thermal noise. Negative bias has the same amplitudes as positive bias.
- f_lo=f_hi=f_probe gives zero integrated noise and a finite spectral density.
- Noise index=0 means 1e−6 V/V. This guards against the factor-one-million normalization error.
- Reject f_lo≤0, f_hi<f_lo, and a probe outside the integration/characterization band.

**Mandatory diagram topology:** DC-biased resistor and explanatory series-noise inset; 1/√f amplitude-density curve marks integration band and probe frequency. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `noise_index_db`, `dc_voltage`, `lower_frequency`, `upper_frequency`, `excess_noise_rms`, `excess_noise_density`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 087. `passives.negative_vcr_divider` — Self-consistent high-voltage divider VCR error

- **Required / level:** Must-have calculator + diagram + docs; expert.
- **Accountable:** D2Passives developer; T2Passives tester; V2Passives source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/passives.md#5-passivesnegative_vcr_divider--expert); [complete JSON example vector](../../research/expansion/passives.json), selected by `id=passives.negative_vcr_divider`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `high_voltage=989.901029888954` V; `high_resistance=9801999.798040994` ohm; `fractional_ratio_error=0.009897011104604824` 1.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use actual component-fit/history/switch parameters; preserve hidden state, constitutive nonlinearity and network loading.

**Semantic novelty decision — retain as one distinct physical problem.**

A high-voltage resistor's value depends on its own terminal voltage, so its divider voltage cannot first be computed using a fixed resistance. Closest baselines are `dc.loaded_divider` and `thermal.resistor_tcr_drift`. Neither solves a constitutive parameter that depends on the unknown voltage itself. This is a nonlinear implicit circuit equilibrium and transfer-ratio error, not a renamed linear drift formula or an inverse divider calculation.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal table and current equality V_h/R_h=V_o/R_b.
- k=0 recovers V_h=990 V, R_h=9.9 MΩ, V_o=10 V and zero ratio error.
- V_s=0 gives zero voltages/error and R_h=R_0.
- For a small negative k, error approaches `−k V_s/(1+r)²`, positive. Use this to test a tiny nonzero coefficient without subtractive cancellation.
- R_b=R_0 with V_s=100 V and k=0 gives equal 50 V drops; negative k increases the bottom voltage.
- Reject positive VCR, negative supply, nonpositive resistances, and kV_s≤−1. Example default −10 ppm/V with 100,000 V is invalid at equality. Passing the guard never certifies physical voltage rating.

**Mandatory diagram topology:** Unloaded divider uses upper R_h(V_h), separate zero-voltage R0 and measured Vh polarity; no fixed-resistor substitution. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `supply_voltage`, `high_resistance_zero`, `bottom_resistance`, `vcr_ppm_volt`, `high_voltage`, `output_voltage`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 088. `passives.nonideal_r2r_ladder` — Three-bit R-2R ladder with unequal switch resistance

- **Required / level:** Must-have calculator + diagram + docs; expert.
- **Accountable:** D2Passives developer; T2Passives tester; V2Passives source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/passives.md#6-passivesnonideal_r2r_ladder--expert); [complete JSON example vector](../../research/expansion/passives.json), selected by `id=passives.nonideal_r2r_ladder`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `lsb_thevenin_resistance=1024.3902439024391` ohm; `lsb_thevenin_voltage=2.4390243902439024` V; `code_error_lsb=-0.6267628715038625` LSB.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use actual component-fit/history/switch parameters; preserve hidden state, constitutive nonlinearity and network loading.

**Semantic novelty decision — retain as one distinct physical problem.**

Resolve actual component-network bit weights, source impedance and loaded output when switch resistances are unequal. Closest baseline: `data.unipolar_dac_output` assumes an ideal code fraction with supplied gain; no component topology is represented. `dc.two_source_node` is a single node, not a terminated multi-rung ladder. This model solves the complete specified ladder; the ideal DAC formula appears only as a comparison oracle. It is not merely a new bit-width parameterization.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal table; verify loaded node equations independently, not by replaying catalog expressions.
- All switches zero and G_load=0: T0=T1=R_out=R and V_load=V_ref code/8 for every code 0…7.
- Same ideal switches with finite G: V_load=V_ref code/[8(1+GR)]. At R=1 kΩ, G=0.001 S and code=4, V_ref=5 V, result is 1.25 V.
- Code=0 gives all four voltage outputs zero, zero code error, and positive resistance outputs.
- For fixed switch/load values, compare codes 1,2,4 to isolate the three physical bit weights; their sum must equal code-7 output. Do not impose ideal 1:2:4 weights.
- With all other inputs unchanged, doubling every resistance and halving G leaves output voltages unchanged and doubles all Thevenin resistances.
- Reject code −1,8 or fractional 2.5; zero/negative R or V_ref; negative switch resistance or load conductance.

**Mandatory diagram topology:** Three nodes connect through R links; each switched 2R leg includes its own switch resistance, separate unswitched 2R LSB termination and output load. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `ladder_resistance`, `lsb_switch_resistance`, `middle_switch_resistance`, `msb_switch_resistance`, `reference_voltage`, `code`, `load_conductance`, `loaded_voltage`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

## Cores — 6 required items

Lane accountability: **D2Cores / T2Cores / V2Cores**. Levels: 2 basic, 2 intermediate, 2 expert.

### 089. `cores.single_layer_air_solenoid` — Single-layer air solenoid geometry

- **Required / level:** Must-have calculator + diagram + docs; basic.
- **Accountable:** D2Cores developer; T2Cores tester; V2Cores source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/cores.md#1-coressingle_layer_air_solenoid); [complete JSON example vector](../../research/expansion/cores.json), selected by `id=cores.single_layer_air_solenoid`. Require every output at nominal relative tolerance `1e-09`. Numerical anchors: `inductance=4.037956793862305e-06` H; `turn_pitch=0.0015` m.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use actual winding dimensions, insulation and material fit data; keep thin-sheet/foil and geometry validity limits, not rating claims.

**Semantic novelty decision — retain as one distinct physical problem.**

Select a finite cylindrical air winding before winding hardware. Closest old item is `magnetics.rectangular_toroid_inductance`: that integrates a closed magnetic path through a toroidal material section; Wheeler handles open-space finite-length field spreading. Neither old core-reluctance formula supplies this end correction.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: complete linked research vector.
- Geometric scaling: multiply radius, length and wire diameter by two, keeping turns fixed. Inductance doubles to `8.07591358772461e-6 H`; pitch becomes `0.003 m`.
- Inclusive fit limits: radius `0.01`, length `0.01`, turns `10`, wire diameter `0.001` gives pitch `0.001 m` and `L=2.07210940737671e-6 H`; all other inputs explicitly as stated.
- Invalid: zero radius/length/diameter; turns 1.5; length below radius; finished diameter above pitch; pitch above radius/5. A later valid request must recover.

**Mandatory diagram topology:** Finite single-layer air winding with mean conductor-center radius, axial span, pitch and finished-wire diameter dimensions. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `radius`, `length`, `turns`, `wire_diameter`, `inductance`, `turn_pitch`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 090. `cores.square_planar_spiral` — Square planar spiral current-sheet inductance

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Cores developer; T2Cores tester; V2Cores source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/cores.md#2-coressquare_planar_spiral); [complete JSON example vector](../../research/expansion/cores.json), selected by `id=cores.square_planar_spiral`. Require every output at nominal relative tolerance `1e-09`. Numerical anchors: `inner_side=0.00015999999999999996` m; `average_side=0.00022999999999999995` m; `inductance=5.825908169108072e-09` H.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use actual winding dimensions, insulation and material fit data; keep thin-sheet/foil and geometry validity limits, not rating claims.

**Semantic novelty decision — retain as one distinct physical problem.**

Lay out a square spiral using lithographic/PCB dimensions. Closest old item is toroidal inductance, and the closest new neighbor is the cylindrical solenoid above. Neither contains the four-sheet mutual-field geometry, radial fill correction or central opening of a planar spiral. The model estimates inductance only, not generic resonance.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: complete linked research vector.
- Scale every length by two: inner side `0.00032 m`, average side `0.00046 m`, unchanged fill, inductance `1.1651816338216144e-8 H`.
- Limiting construction: `turns=1` remains allowed when the positive opening fits; no turn gap contributes to the inner-side formula. Approach the central-opening limit from above and ensure finite positive inductance.
- Invalid: noninteger turns; outside side equal to or below `2[Nw+(N-1)s]`; zero width or spacing; spacing above three widths. At `spacing=3*trace_width`, the spacing screen is inclusive if opening remains positive.

**Mandatory diagram topology:** Continuous square spiral has positive central opening and separately depicted underpass; distinguish width, gap and inner/outer sides. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `outer_side`, `turns`, `trace_width`, `spacing`, `inner_side`, `average_side`, `fill_ratio`, `inductance`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 091. `cores.bobbin_layer_fit` — Insulated round-wire bobbin layer fit

- **Required / level:** Must-have calculator + diagram + docs; basic.
- **Accountable:** D2Cores developer; T2Cores tester; V2Cores source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/cores.md#3-coresbobbin_layer_fit); [complete JSON example vector](../../research/expansion/cores.json), selected by `id=cores.bobbin_layer_fit`. Require every output at nominal relative tolerance `1e-09`. Numerical anchors: `turns_per_layer=30` turn; `layers_required=4` layer; `copper_fill=0.2727076956241139` 1.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use actual winding dimensions, insulation and material fit data; keep thin-sheet/foil and geometry validity limits, not rating claims.

**Semantic novelty decision — retain as one distinct physical problem.**

Determine whether a specified winding physically fits. Closest old item, transformer VA budget, sizes apparent-power capacity but cannot predict winding construction. This model adds integer packing, insulation, partial-layer rounding and radial clearance. It is not an electrical resistance or turns-ratio calculation.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: complete linked research vector.
- Discrete boundary with the other nominal inputs: `turns=90` gives three layers and build `0.002 m`; `turns=91` jumps to four layers and build `0.0027 m`. No interpolation across the jump.
- With `turns=30`, one layer has build exactly `0.0006 m` even when tape thickness is nonzero: no tape lies before the first or after the last layer.
- Exact radial fit is accepted (`window_height=0.0027`, otherwise nominal), with zero clearance. Zero end margin or zero interlayer tape is geometrically allowed; this does not certify electrical safety.
- Invalid: only `window_width=0.001` with nominal margins, zero diameter, copper diameter above finished diameter, fractional turns, or `window_height=0.0026` with the nominal 100 turns.

**Mandatory diagram topology:** Bobbin window shows insulated square-packed layers, partial final row, end margins and tape only between occupied layers. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `window_width`, `window_height`, `end_margin`, `wire_diameter`, `copper_diameter`, `tape_thickness`, `turns_per_layer`, `layers_required`, `radial_build`, `height_margin`, `copper_fill`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 092. `cores.sinusoidal_steinmetz_loss` — Calibrated sinusoidal core-loss scaling

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Cores developer; T2Cores tester; V2Cores source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/cores.md#4-coressinusoidal_steinmetz_loss); [complete JSON example vector](../../research/expansion/cores.json), selected by `id=cores.sinusoidal_steinmetz_loss`. Require every output at nominal relative tolerance `1e-09`. Numerical anchors: `loss_density=210325.46208198377` W/m^3; `core_loss=2.103254620819838` W; `energy_per_cycle=1.4021697472132253e-05` J.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use actual winding dimensions, insulation and material fit data; keep thin-sheet/foil and geometry validity limits, not rating claims.

**Semantic novelty decision — retain as one distinct physical problem.**

Predict dissipated core heat using a measured material calibration. Closest old items are sinusoidal flux amplitude and stored magnetic energy density: neither models hysteretic/eddy dissipation. Closest new neighbor is lamination eddy loss, which predicts one physical mechanism from sheet dimensions; this model is an empirical total-loss fit and must not be added to that result.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: complete linked research vector.
- Reference identity: with `frequency=100000` and `flux_peak=0.1`, all other inputs nominal, outputs are exactly `200000 W/m³`, `2 W`, `20e-6 J` in the mathematical model.
- Double only `core_volume`: loss density stays fixed; total heat and per-cycle loss double.
- Inclusive fit endpoints are admitted; changing the fitted range must not silently clamp input. Both reference and operating points must fit.
- Invalid: frequency zero; negative/zero flux; reversed bounds; operating or reference point outside a bound; zero reference loss/volume or nonpositive fitted exponent. Incorrect material/temperature cannot be detected numerically and remains the user's calibration responsibility.

**Mandatory diagram topology:** Closed core with circulating flux and separate zero-centered sinusoid; mark peak B, frequency, core volume and dissipated heat. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `frequency`, `flux_peak`, `reference_frequency`, `reference_flux_peak`, `reference_loss_density`, `core_volume`, `core_loss`, `energy_per_cycle`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 093. `cores.lamination_eddy_loss` — Thin-lamination classical eddy loss

- **Required / level:** Must-have calculator + diagram + docs; expert.
- **Accountable:** D2Cores developer; T2Cores tester; V2Cores source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/cores.md#5-coreslamination_eddy_loss); [complete JSON example vector](../../research/expansion/cores.json), selected by `id=cores.lamination_eddy_loss`. Require every output at nominal relative tolerance `1e-09`. Numerical anchors: `penetration_ratio=0.12566370614359174` 1; `eddy_loss_density=210.55156055657307` W/m^3; `eddy_loss=0.021055156055657307` W.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use actual winding dimensions, insulation and material fit data; keep thin-sheet/foil and geometry validity limits, not rating claims.

**Semantic novelty decision — retain as one distinct physical problem.**

Select electrical-steel lamination thickness from classical eddy heating. Closest old item is `magnetics.conductor_skin_depth`: that gives field penetration but no induced-current distribution or dissipated power. Here penetration only screens the complete-penetration approximation. This is distinct from copper winding transport-current loss and measured total Steinmetz loss.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: complete linked research vector.
- Halve only metal thickness: penetration ratio halves to `0.06283185307179587`; density/heat quarter to `52.63789013914327 W/m³` and `0.005263789013914327 W`.
- Zero frequency with otherwise nominal inputs: penetration ratio and both losses are zero. Zero flux at nominal frequency: penetration ratio stays nominal, both losses zero.
- The default geometry reaches `x=0.3` at about `284.965828994075 Hz`; use the defining expression rather than a rounded boundary when testing inclusive acceptance. A frequency of `400 Hz` violates the thin-sheet screen.
- Invalid: zero/negative resistivity or thickness; negative frequency or flux; flux above `linear_flux_limit`; effective relative permeability below one. A thickness large enough to violate penetration must reject, not return the optimistic thin-sheet heat.

**Mandatory diagram topology:** Insulated sheet stack shows metal-only thickness, in-plane B and opposite eddy currents without transport current. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `flux_peak`, `lamination_thickness`, `metal_volume`, `penetration_ratio`, `eddy_loss_density`, `eddy_loss`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 094. `cores.thin_foil_winding_loss` — Thin-foil multilayer winding AC loss

- **Required / level:** Must-have calculator + diagram + docs; expert.
- **Accountable:** D2Cores developer; T2Cores tester; V2Cores source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/cores.md#6-coresthin_foil_winding_loss); [complete JSON example vector](../../research/expansion/cores.json), selected by `id=cores.thin_foil_winding_loss`. Require every output at nominal relative tolerance `1e-09`. Numerical anchors: `thickness_ratio=0.23954419411992858` 1; `resistance_dc=0.00516` ohm; `ac_excess_loss=0.0016612403122077928` W.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use actual winding dimensions, insulation and material fit data; keep thin-sheet/foil and geometry validity limits, not rating claims.

**Semantic novelty decision — retain as one distinct physical problem.**

Estimate layer-dependent copper AC heat for a thin series foil winding. Closest old item is skin depth; it does not model current redistribution, series layers, winding resistance or loss. Closest new neighbor is magnetic-sheet eddy heating, where no transport current exists and the material is ferromagnetic. Here the winding carries current, and its self/proximity field grows through successive layers.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: complete linked research vector.
- DC limit, `frequency=0`, other inputs nominal: `thickness_ratio=0`, `resistance_dc=resistance_ac=0.00516 ohm`, `ac_excess_factor=0`, `winding_loss=0.516 W`, `ac_excess_loss=0`.
- Zero current at nominal frequency leaves all resistance/ratio outputs unchanged and both heat outputs zero.
- Halving only frequency divides excess factor by four. Increasing layer count increases both geometric Rdc and proximity coefficient; do not replace the coefficient with an isolated-foil skin term.
- Thin-limit frequency is about `435681.0896620524 Hz` for the nominal material/thickness. `500000 Hz` must reject; test the exact defining boundary separately. `foil_width=20*foil_thickness` is an inclusive geometry screen.
- Invalid: fractional/zero layer count; nonpositive thickness, width, length or resistivity; negative current/frequency; width below twenty thicknesses.
- Pure numerical conditioning probe, not a realizable hardware operating point: set `frequency=1e-170 Hz` and `current_rms=1e100 A`, leaving every other nominal input unchanged. Decimal derivation gives `x=7.5750525368848788e-89`, true dimensionless excess `3.2194579693949486e-353` (correctly rounds to zero in binary64), but additional AC heat `1.6612403122077935e-155 W` remains representable and must not be lost. `Rdc=Rac=0.00516 ohm` to binary64 precision and total heat is approximately `5.16e197 W`. This distinguishes permissible rounding of the ratio from premature loss of the dimensional result.

**Mandatory diagram topology:** Series foil layers show equal transport-current direction, insulation, metal thickness/width and mean-turn path; distinguish excess and total heat. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `foil_thickness`, `foil_width`, `layers`, `mean_turn_length`, `current_rms`, `ac_excess_factor`, `resistance_ac`, `winding_loss`, `ac_excess_loss`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

## Measurement — 6 required items

Lane accountability: **D2Measurement / T2Measurement / V2Measurement**. Levels: 1 basic, 3 intermediate, 2 expert.

### 095. `measurement.dmm_accuracy_bound` — DMM reading-plus-range accuracy bound

- **Required / level:** Must-have calculator + diagram + docs; basic.
- **Accountable:** D2Measurement developer; T2Measurement tester; V2Measurement source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/measurement.md#1-measurementdmm_accuracy_bound); [complete JSON example vector](../../research/expansion/measurement.json), selected by `id=measurement.dmm_accuracy_bound`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `reading_error_bound=1.5e-05` V; `range_error_bound=2e-05` V; `upper_bound=0.500035` V.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use applicable instrument/probe specifications and uncertainty/jitter conventions; keep the documented first-order and drift assumptions.

**Semantic novelty decision — retain as one distinct physical problem.**

Nearest baseline/neighbor: `data.adc_quantization_error`. Manufacturer gain/offset accuracy envelope depends on reading and selected range, not ADC code width.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: assert all five outputs in the linked research vector.
- Negative reading −0.5 V, other inputs nominal: contributions and total unchanged; endpoints −0.500035 and −0.499965 V.
- Zero reading: reading contribution 0, range contribution and total 20 µV, endpoints −20 and +20 µV. Do not divide by reading to report an undefined relative error.
- Zero coefficients are allowed and return a zero-width interval. Exactly `abs(reading)=range_value` is allowed.
- Reject range ≤0, negative percentages, `abs(reading)>range_value`, and nonfinite inputs.

**Mandatory diagram topology:** Voltmeter with polarity/range beside bounded reading interval; additive reading/range errors are separate, not RSS uncertainty. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `reading`, `range_value`, `reading_percent`, `range_percent`, `reading_error_bound`, `range_error_bound`, `total_error_bound`, `lower_bound`, `upper_bound`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 096. `measurement.delta_resistance` — Three-point delta resistance and thermal-EMF rejection

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Measurement developer; T2Measurement tester; V2Measurement source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/measurement.md#2-measurementdelta_resistance); [complete JSON example vector](../../research/expansion/measurement.json), selected by `id=measurement.delta_resistance`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `resistive_voltage=0.001` V; `resistance=0.1` ohm; `emf_drift_rate=2e-05` V/s.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use applicable instrument/probe specifications and uncertainty/jitter conventions; keep the documented first-order and drift assumptions.

**Semantic novelty decision — retain as one distinct physical problem.**

Nearest baseline/neighbor: `dc.ohm_current`; sensor transduction. A three-reading reversal experiment rejects constant and linearly drifting thermoelectric EMF; it does not merely rearrange Ohm's law.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: assert all four outputs.
- Constant offset: readings `{0.001010, −0.000990, 0.001010}` V, nominal I and dt, return `{resistive_voltage:0.001,resistance:0.1,midpoint_emf:0.000010,emf_drift_rate:0}`.
- Add the same signed voltage to all three readings: resistance and slope must stay unchanged; midpoint EMF shifts by that voltage.
- Swap first and third readings: resistance and midpoint EMF stay unchanged; drift changes sign.
- Pure linear EMF with zero resistance: readings `{12e-6,14e-6,16e-6}` V return 0 V, 0 Ω, 14 µV and 20 µV/s, within cancellation-aware absolute tolerance on the zero outputs.
- Reject current ≤0, interval ≤0, or a negative delta resistive voltage with the stated passive-resistor polarity. Signed individual readings and negative EMF/drift are valid.
- Nonlinear drift cannot be detected from only these three observations: test the documented `q dt²/2` bias as a limitation, not as a rejected input.

**Mandatory diagram topology:** Kelvin resistance experiment retains fixed voltage polarity during +I,−I,+I sequence; EMF source lies in sense lead. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `test_current`, `sample_interval`, `voltage_first`, `voltage_second`, `voltage_third`, `resistive_voltage`, `resistance`, `midpoint_emf`, `emf_drift_rate`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 097. `measurement.compensated_probe_loading` — Compensated passive probe with source loading

- **Required / level:** Must-have calculator + diagram + docs; expert.
- **Accountable:** D2Measurement developer; T2Measurement tester; V2Measurement source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/measurement.md#3-measurementcompensated_probe_loading); [complete JSON example vector](../../research/expansion/measurement.json), selected by `id=measurement.compensated_probe_loading`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `matched_probe_capacitance=1e-11` F; `dc_gain=0.0999900009999` V/V; `scope_voltage=0.09983054361596318` V.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use applicable instrument/probe specifications and uncertainty/jitter conventions; keep the documented first-order and drift assumptions.

**Semantic novelty decision — retain as one distinct physical problem.**

Nearest baseline/neighbor: `dc.loaded_divider`, `filters.rc_lowpass_response`; precision amplifier stages. Two parallel-RC arms create a compensation zero plus source-loading poles; predicts loading and adjustment of a passive measurement probe.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: assert all four outputs.
- Set frequency=0: AC gain equals DC gain and output is 0.0999900009999 V; matched capacitance stays 10 pF.
- Set source resistance=0 with matched arms: DC gain=AC gain=0.1, scope voltage=0.1 V, independently of frequency within lumped validity.
- Under-compensated test: Rs=0, Cp=5 pF, others nominal. Matched capacitance remains 10 pF, DC gain=.1, AC gain and output=0.05263181693942273.
- Over-compensated test: Rs=0, Cp=20 pF, others nominal. Matched capacitance=10 pF, DC gain=.1, AC gain and output=0.1818180179402501. Do not clamp AC gain to DC gain.
- Cp=0, others nominal: AC gain and output=0.00019646537529400082; valid uncompensated limit.
- Source voltage=0 yields scope voltage=0 without changing gains. Reject negative source voltage, resistance, capacitance or frequency; Rp, Ri and Ci must be strictly positive.

**Mandatory diagram topology:** Source Rs feeds probe Rp∥Cp between tip/scope nodes, then Ri∥Ci to return; probe capacitor is not a ground shunt. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `source_voltage`, `source_resistance`, `probe_resistance`, `probe_capacitance`, `scope_resistance`, `scope_capacitance`, `frequency`, `matched_probe_capacitance`, `scope_voltage`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 098. `measurement.rectangular_aperture_response` — Rectangular measurement-aperture response

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Measurement developer; T2Measurement tester; V2Measurement source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/measurement.md#4-measurementrectangular_aperture_response); [complete JSON example vector](../../research/expansion/measurement.json), selected by `id=measurement.rectangular_aperture_response`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `midpoint_gain=0.9003163161571061` V/V; `amplitude_ratio=0.9003163161571061` V/V; `effective_delay=0.005` s.
- **Prerequisites:** P0; P1 — Main must execute zero-safe sinc coverage before acceptance. Use applicable instrument/probe specifications and uncertainty/jitter conventions; keep the documented first-order and drift assumptions.

**Semantic novelty decision — retain as one distinct physical problem.**

Nearest baseline/neighbor: `data.sar_acquisition_settling`, `data.tone_alias_frequency`. Uniform measurement integration has sinc nulls and midpoint time attribution; no acquisition RC or alias mapping.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: assert all four outputs.
- DC: frequency=0, nominal T and amplitude: `{midpoint_gain:1,amplitude_ratio:1,measured_peak:.2,effective_delay:.005}`.
- Instantaneous limit: T=0, nominal frequency and amplitude: `{midpoint_gain:1,amplitude_ratio:1,measured_peak:.2,effective_delay:0}`. Both f=T=0 are valid.
- Half cycle: f=50 Hz, T=.01 s: signed and magnitude gain `2/π=0.6366197723675814`, peak=.12732395447351627 V, delay=.005 s.
- Whole cycle: f=100 Hz, T=.01 s: mathematical gains and peak are zero, delay=.005 s. Floating `sin(pi)` leaves a tiny residual; use a justified absolute tolerance near 1e-15 on dimensionless gain, not a demand for exact trigonometric zero or an epsilon-clamped implementation.
- Negative lobe: f=150 Hz, T=.01 s: signed gain `−2/(3π)=−.2122065907891938`, magnitude=.2122065907891938, peak=.04244131815783876 V, delay=.005 s.
- Zero amplitude keeps the response and delay but gives zero output amplitude. Reject any negative input; nonfinite rejection remains shared.

**Mandatory diagram topology:** Uniform aperture spans t−T to t; midpoint t−T/2 and causal delay are marked beside signed sinc and amplitude annotations. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `frequency`, `aperture_time`, `input_peak`, `midpoint_gain`, `amplitude_ratio`, `measured_peak`, `effective_delay`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 099. `measurement.sampling_jitter_snr` — Sinusoidal sampling-jitter noise limit

- **Required / level:** Must-have calculator + diagram + docs; intermediate.
- **Accountable:** D2Measurement developer; T2Measurement tester; V2Measurement source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/measurement.md#5-measurementsampling_jitter_snr); [complete JSON example vector](../../research/expansion/measurement.json), selected by `id=measurement.sampling_jitter_snr`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `phase_jitter=0.00012566370614359174` rad; `error_rms=8.885765876316733e-05` V; `jitter_snr=78.01580271955808` dB.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use applicable instrument/probe specifications and uncertainty/jitter conventions; keep the documented first-order and drift assumptions.

**Semantic novelty decision — retain as one distinct physical problem.**

Nearest baseline/neighbor: `data.bandlimited_quantization_snr`; logic timing. Random sampling-time error converts signal slope into voltage noise; independent of bit depth and clock-rate division.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: assert all three outputs.
- Double only frequency to 20 MHz: phase=.0002513274122871835 rad, error=.00017771531752633465 V, SNR=71.99520280627846 dB. Doubling only jitter has the same effect.
- Double only input amplitude: phase and SNR unchanged, error doubles. This distinguishes jitter SNR from an additive fixed-voltage noise model.
- At the 0.1-rad boundary: error=A*.1/sqrt(2), SNR=20 dB. Values just above the bound are rejected; values below it remain valid.
- Positive arbitrarily small jitter is physically admitted; no arbitrary time floor. SNR rises without a hard clamp. Guard against avoidable log-product underflow.
- Reject frequency ≤0, jitter ≤0, amplitude ≤0, or phase product >0.1. Negative/zero jitter is not a meaningful finite-SNR case.

**Mandatory diagram topology:** Sine sample point has horizontal RMS timing-error bracket and resulting voltage-error projection; RMS is not peak-to-peak. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `frequency`, `timing_jitter`, `input_peak`, `phase_jitter`, `error_rms`, `jitter_snr`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

### 100. `measurement.correlated_voltage_ratio` — Correlated voltage-ratio measurement uncertainty

- **Required / level:** Must-have calculator + diagram + docs; expert.
- **Accountable:** D2Measurement developer; T2Measurement tester; V2Measurement source/native-image reviewer.
- **Full research and oracle:** [physical derivation, sources, full cases and diagram](../../research/expansion/measurement.md#6-measurementcorrelated_voltage_ratio); [complete JSON example vector](../../research/expansion/measurement.json), selected by `id=measurement.correlated_voltage_ratio`. Require every output at nominal relative tolerance `1e-06`. Numerical anchors: `voltage_ratio=2.0` V/V; `relative_standard_uncertainty=0.00036878177829171557` 1; `expanded_uncertainty=0.0014751271131668623` V/V.
- **Prerequisites:** P0; no new engine primitive or cross-feature dependency. Use applicable instrument/probe specifications and uncertainty/jitter conventions; keep the documented first-order and drift assumptions.

**Semantic novelty decision — retain as one distinct physical problem.**

Nearest baseline/neighbor: `noise.uncorrelated_rms_sum`; precision reference design. Ratio sensitivities introduce a negative covariance contribution and common-reference cancellation; evaluates a measurement process, not independent voltage-noise addition.

**Important numerical limits and error acceptance** (all remaining linked research cases also required):

- Nominal: assert all four outputs.
- rho=0, other values nominal: r=2, relative u=.0007211102550927978, standard u=.0014422205101855955, expanded u=.002884441020371191.
- rho=−1: r=2, relative u=.001, standard u=.002, expanded u=.004.
- rho=+1: r=2, relative u=.0002, standard u=.0004, expanded u=.0008.
- Exact common fractional uncertainty: ua=.002 V, ub=.001 V, rho=1, nominal voltage estimates and k. Ratio=2 and all three uncertainty outputs=0. This is genuine cancellation in the model, not a clamped negative radicand.
- Near-perfect correlation with equal a=b: relative u equals `a*sqrt(2*(1−rho))` and must stay positive for rho<1. Test one representable step below 1 with high-precision independent arithmetic.
- Both uncertainties zero are valid for any allowed rho and return zero uncertainty. k scales expanded uncertainty only.
- Reject rho outside [−1,1], negative uncertainties, nonpositive voltages or k, and relative standard uncertainty above the declared 1% first-order scope. Test exactly 1% as accepted.

**Mandatory diagram topology:** Two positive voltage channels share reference influence via dashed nonphysical paths; show individual uncertainties, correlation, ratio and coverage factor. Follow the linked full netlist/geometry, caption and polarity instructions.

**Quantity-label acceptance:** `numerator_voltage`, `denominator_voltage`, `numerator_uncertainty`, `denominator_uncertainty`, `correlation`, `coverage_factor`, `voltage_ratio`, `relative_standard_uncertainty`, `ratio_standard_uncertainty`, `expanded_uncertainty`. Each key must retain its researched meaning; compound or scaled relation text must not mislabel a bound value. Source inspection and native pixels must agree.

