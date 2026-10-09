# Sensor transduction expansion research

## Scope and handoff

Exactly seven new calculations: three basic, three intermediate, one expert. Definitions: `research/expansion/sensors.json`. All seven require their own diagram. Only these two research files and append-only communication entries are owned here. No production files, existing IDs, callers, test fixtures, configuration, exports or shared registry were changed. Developers must promote the catalog, implement seven `sensorsDiagrams` entries using the existing primitives, and publish feature documentation. Parent owns registry integration and verification.

| ID | Level | Distinct transduction problem |
|---|---|---|
| `sensors.ntc_steinhart_hart` | intermediate | Calibrated logarithmic-cubic NTC thermometry |
| `sensors.platinum_callendar_van_dusen` | intermediate | Platinum resistance including subzero quartic response |
| `sensors.type_t_cold_junction_emf` | expert | Nonlinear thermoelectric EMF from two junction temperatures |
| `sensors.hall_plate_angle` | basic | Transverse Hall voltage from the normal component of a tilted field |
| `sensors.amr_angular_resistance` | basic | Longitudinal ferromagnetic resistance versus internal magnetization angle |
| `sensors.hydrogen_activity_electrode` | basic | Logarithmic chemical-activity-to-voltage transduction |
| `sensors.differential_gap_capacitor` | intermediate | Opposed moving-gap capacitances and nonlinear displacement sensitivity |

No inverse modes or supporting outputs are counted separately. The baseline exclusion inventory was read in full as IDs, formulas, input keys and output keys. Closest exclusions are the linear resistor TCR, loaded Wheatstone bridge, plate displacement current, magnetic energy and semiconductor junction laws. None models these seven constitutive transducers. Precision owns conditioning electronics; opto owns optical transducers. Batteries confirmed ownership of regular-solution intercalation and chemical capacitance, not the glass-electrode hydrogen-activity model.

## Search boundaries and evidence

Research question: which closed-form, physically distinct sensor models add nonlinear transduction without duplicating the 128 baseline calculations? Searches covered English manufacturer application notes, NIST reference functions and university repositories, accessed 2026-10-09. No publication-date cutoff was imposed. Search phrases included Steinhart–Hart coefficients, Callendar–Van Dusen constants, Type-T reference coefficients and cold-junction correction, Hall normal field, AMR cosine-squared resistance, Nernst pH electrode, and differential capacitive displacement. Gas-sensor empirical curves were considered but not selected: no verified primary power-law calibration was established. Piezo RC decay was excluded because it would overlap the existing electrical transient models.

Evidence actually read:

1. **Manufacturer primary application note, strong equation evidence:** [ILX Lightwave TNSTEIN-1](https://api.p1.mks.com/medias/sys_master/images/images/hdb/hac/8797291479070/TN-STEIN-1-Thermistor-Constant-Conversions-Beta-to-Steinhart-Hart.pdf), equation (1), January 2005. Gives `1/T=C1+C2 ln(R)+C3 ln(R)^3`, with kelvin and ohms. Warns that coefficients inferred from a beta model are less accurate than resistance-temperature calibration.
2. **Manufacturer primary engineering note, strong coefficient/range evidence:** [TI SDAA150 section 1.2.2.1](https://www.ti.com/document-viewer/lit/html/SDAA150/GUID-D7335C6E-A97D-4849-80CC-6781C31A787C.html), equations 21 and 23–26. Gives both platinum branches and A/B/C constants. This is not a direct purchase/read of IEC 60751, and no compliance certification is claimed. The prose calls the negative-temperature inverse a quadratic; the displayed forward equation is quartic. This implementation uses the displayed forward law, not that prose or the approximate inverse fit.
3. **Government primary reference function, strongest numerical authority:** [NIST SRD 60 Type-T data](https://its90.nist.gov/downloadFiles/type_t.tab.txt), coefficient block for 0–400 degrees Celsius. All nine coefficients and their mV/Celsius units were read. [TI SBAA274A](https://www.ti.com/lit/an/sbaa274a/sbaa274a.pdf), sections 1.1 and 1.3, supplies the isothermal connection and EMF-domain cold-junction law. TI's summary polynomial-order table is not used; the NIST coefficient block controls.
4. **Manufacturer primary physics explanation, strong ideal-model evidence:** [Allegro AN296065](https://www.allegromicro.com/en/insights-and-innovations/technical-documents/hall-effect-sensor-ic-publications/allegro-hall-effect-sensor-ics), Hall Effect Principles and equation image, explicitly read. `VH=I B_perp/(n q t)`. The catalog generalizes the material factor to signed calibrated `hall_coefficient`; projection and angular differentiation are elementary derivations, not manufacturer device specifications.
5. **Manufacturer primary document through a third-party mirror, moderate provenance strength:** [Philips Semiconductors, Magnetoresistive sensors for magnetic field measurement](https://manualsdump.com/en/manuals/philips-magnetoresistive_sensor/271677/15), September 6, 2000, appendix 1 page 15, equation (2). Both extracted text and the original page image were read. Equation (2) states `R=R0+DeltaR cos²(phi)` and defines the current/magnetization geometry. The page's resistivity notation/sign rendering is ambiguous; catalog parameters are explicitly calibrated perpendicular resistance and signed parallel-minus-perpendicular difference, so endpoint definitions fix the convention. The usermanual.wiki PDF route redirected to a 403 page; it is not claimed as accessed evidence. No authenticated NXP-hosted replacement was obtained.
6. **Manufacturer primary application note, strong ideal-model evidence:** [TI AN-1852 / SNOA529A](https://www.ti.com/lit/an/snoa529a/snoa529a.pdf), section 1.1, equations (1)–(2), figures 2–3. Gives Nernst slope, polarity, temperature dependence, high source impedance, offset/span calibration needs. Modern exact SI Boltzmann and elementary-charge constants replace the older rounded R/F numbers; this changes only constant rounding.
7. **Manufacturer primary application note, strong geometry evidence:** [TI SNOA927A](https://www.ti.com/lit/pdf/snoa927), section 1 equation (1), section 5.2 and section 6.1. Gives parallel-plate capacitance, thin-gap/fringing limits and differential sensing. The two opposed-gap equations and derivatives below are explicitly derived from that law. The [TU Eindhoven Goossens thesis](https://pure.tue.nl/ws/files/46970239/779574-1.pdf), section 2, was also examined but uses an overlapping cylindrical implementation, so it is not claimed as direct evidence for this planar topology.

Analog Devices CN0394 and AN-28 web fetches timed out; TI's successfully read guide supplies the required cold-junction evidence. Manufacturer application notes explain ideal models, not the performance of every real component. No peer-reviewed accuracy study, calibration certificate or hardware measurement was performed. Model equations are well supported; component accuracy remains device- and calibration-dependent.

## Shared numerical and illustration contract

- Inputs/defaults/examples and every output appear in the JSON. All nominal examples below use the complete JSON default input map. Decimal arithmetic at 60-digit precision independently derived temperature, thermocouple, Nernst and capacitance oracles. Elementary trigonometric identities independently derive the Hall/AMR examples. No Rust engine was used.
- JSON relative tolerance is `1e-9` for nominal nonzero outputs. Exact algebraic zeros need exact-zero assertions where the expressions permit them. Trigonometric quarter-turn zeros need a justified absolute floating-point angular residual, not a relative comparison to zero.
- Only existing expression functions/operators are used. Constraints reference inputs only. No new evaluator primitive is requested. Nonfinite values must remain rejected by the existing schema/engine.
- Thermocouple difference uses a factored divided difference, not subtraction of nearly equal EMFs. Differential capacitance uses a factored difference. The Nernst logarithm uses two nonnegative `log1p` arguments, preventing cancellation near equal activities without approaching `log1p(-1)` at extreme ratios.
- Every diagram uses the existing `primitives.dart`, canvas 700×280, margins at least 16 px and labels at least 14 px. Reserve the lower band for short notes; put lengthy explanations in the caption. Exact quantity bindings are listed below. Geometry and curves are static explanations, never claimed as live output plots. Use component bodies with broken wire paths and explicit junctions; no unmarked crossing connections.

## 1. sensors.ntc_steinhart_hart

**Purpose and novelty.** Map measured NTC resistance through a calibrated reciprocal-temperature cubic in log resistance. Closest baseline `thermal.resistor_tcr_drift` assumes a linear resistance-temperature slope. Neither that model nor `dc.loaded_wheatstone_bridge` can reproduce this material response. A beta inverse is not a second feature.

**Model.** Let `L=ln(R/1 ohm)` and `D=A+B L+C L³`. Then `T=1/D`, `t=T-273.15`, and `dT/dR=-T²(B+3C L²)/R`. Coefficients carry inverse-kelvin units under the stipulated numerical-ohm convention. The accepted family has `B>0`, `C>=0`, guaranteeing negative local sensitivity for positive resistance and temperature. Signed A is allowed. The fit interval must be positive and ordered; `1/Tmax<=D<=1/Tmin` simultaneously rejects nonphysical inverse temperature and extrapolation. This family restriction is explicit, not a claim that every possible thermistor fit has positive C.

**Validity.** Static sensor temperature, negligible self-heating/lead/leakage errors. Use independently calibrated coefficients and their real interval. The example is an illustrative plausible coefficient set, not a specified commercial thermistor or a certified interval.

**Complete example inputs.** `resistance=10000 ohm`, `coefficient_a=0.001129148 1/K`, `coefficient_b=0.000234125 1/K`, `coefficient_c=8.76741e-8 1/K`, `fit_temperature_min=233.15 K`, `fit_temperature_max=398.15 K`. With `L=ln(10000)=9.21034037197618`, substitute into D and differentiate:

| Output key | Independent expected value |
|---|---:|
| `temperature_kelvin` | 298.1496681766963 K |
| `temperature_celsius` | 24.999668176696318 degC |
| `temperature_sensitivity` | -0.002279553664363643 K/ohm |

**Tester cases.** Nominal all-output comparison; larger R gives lower T within the fit interval. `C=0` is the legitimate two-coefficient limiting member, not another ID. As a simple independent algebraic case, `R=1 ohm,A=1/300 1/K,B=0.001 1/K,C=0`, default fit interval gives approximately `T=300 K,t=26.85 degC,dT/dR=-90 K/ohm`. Reject R=0/negative, B=0, C<0, nonpositive fit temperatures, reversed/equal fit bounds, and a resistance predicting T outside the fit interval. Boundary equality in reciprocal temperature is accepted.

**Mandatory diagram.** Left area x=40–290: a two-terminal thermistor body at (150,130), separate sense leads to x=70 and x=230, and a temperature arrow toward the bead from above. Bind `resistance` to “R measured” beside the leads and `temperature_kelvin` to “T” beside the bead. Right area x=370–650: conceptual decreasing R-versus-T curve, vertical axis “R”, horizontal axis “T”; show a tangent with label `temperature_sensitivity` explicitly “dT/dR”, not “dR/dT”. Bind `coefficient_a`, `coefficient_b`, `coefficient_c` to a short equation legend below the curve if room permits, otherwise omit these secondary bindings. Caption: “Calibrated Steinhart–Hart thermistor law; curve is illustrative. Sense excitation must not materially heat the bead. Reported sensitivity is dT/dR, the reciprocal of the plotted tangent slope.”

## 2. sensors.platinum_callendar_van_dusen

**Purpose and novelty.** Forward platinum RTD response over both coefficient branches, including a quartic correction below zero Celsius. Closest baseline `thermal.resistor_tcr_drift` lacks both quadratic and subzero quartic curvature. NTC carrier physics and reciprocal-log calibration are different from platinum RTD conduction; these are not inverses.

**Model.** Celsius `t`, `u=min(t,0)`, `A=3.9083e-3 K^-1`, `B=-5.775e-7 K^-2`, `C=-4.183e-12 K^-4`:

`R=R0[1+A t+B t²+C(u-100)u³]`.

`dR/dt=R0[A+2B t+C(4u-300)u²]`.

`linear_model_error=R0[B t²+C(u-100)u³]` is the error versus the tangent model `R0(1+A t)`. It is computed directly rather than subtracting nearly equal resistances. The branch representation and its first derivative are continuous at zero.

**Validity.** `R0>0`; `-200<=t<=850 degC`, nominal IEC platinum coefficient family. Package rating and actual calibration can be narrower. Neglect lead resistance, strain, self-heating and hysteresis. Not an IEC tolerance certificate.

**Complete example inputs.** `zero_resistance=100 ohm`, `temperature_celsius=-100 degC`. The normalized contributions are `A t=-0.39083`, `B t²=-0.005775`, and `C(t-100)t³=-0.0008366`.

| Output key | Independent expected value |
|---|---:|
| `resistance` | 60.25584 ohm |
| `resistance_sensitivity` | 0.4053081 ohm/K |
| `linear_model_error` | -0.66116 ohm |

**Tester cases.** Nominal; `t=0` gives `(100,0.39083,0)` in output order. `t=-200` gives `(18.52008,0.4323352,-3.31392)`; `t=850` gives `(390.481125,0.292655,-41.724375)`. Both endpoints are accepted. Values just below/above zero must approach the same resistance and sensitivity. Scaling R0 by ten scales every output by ten. Reject R0<=0, t<-200 and t>850.

**Mandatory diagram.** Left x=45–260: a platinum serpentine element inside a sensor outline, with four unconnected measurement/excitation lead stubs terminating at the element's two endpoint junctions. Two sense leads are distinct from two current leads; do not depict another resistor network. Bind `temperature_celsius` above the element and `resistance` below. Right x=360–650: conceptual R(t) curve through an indicated zero-temperature reference, with a straight tangent at zero. Bind `zero_resistance` to the zero-temperature intercept, `resistance_sensitivity` to “local dR/dT”, and `linear_model_error` to a vertical bracket explicitly “R − tangent-at-zero model”. Caption: “Nominal platinum Callendar–Van Dusen response; subzero quartic term joins the positive-temperature quadratic smoothly. Curves and deviation bracket are explanatory, not computed.”

## 3. sensors.type_t_cold_junction_emf

**Purpose and novelty.** Predict the actual Type-T terminal signal from two junction temperatures, not a generic voltage difference or constant Seebeck slope. No baseline sensor EMF model exists. NTC/RTD are excited resistive sensors; a thermocouple is a self-generated thermoelectric source.

**Model and source coefficients.** NIST defines `E(t)=sum(c_i t^i)` in mV and Celsius. This item deliberately implements the complete positive-temperature branch, `0<=Th,Tc<=400 degC`; it does not pretend to cover the negative branch.

| i | c_i in mV/(degC)^i |
|---|---:|
| 0 | 0 |
| 1 | 0.038748106364 |
| 2 | 0.00003329222788 |
| 3 | 2.0618243404e-7 |
| 4 | -2.1882256846e-9 |
| 5 | 1.0996880928e-11 |
| 6 | -3.0815758772e-14 |
| 7 | 4.547913529e-17 |
| 8 | -2.7512901673e-20 |

Outputs use volts, multiplying by 1e-3. `Vterminal=E(Th)-E(Tc)` and `hot_sensitivity=dE(Th)/dTh`. The JSON uses Horner evaluation for individual EMFs. Its terminal expression factors each polynomial difference using `Th^i-Tc^i=(Th-Tc) sum(k=0..i-1,Th^(i-1-k) Tc^k)`; thus near-equal temperatures retain their physical differential signal. The coefficient sum is bounded and well-conditioned over the stated branch. Names “hot” and “cold” identify junction roles; Th<Tc is valid and produces negative terminal voltage.

**Validity and signs.** Copper positive leg, constantan negative leg; both termination junctions share the measured isothermal reference temperature. Homogeneous wire and negligible measurement current. No tolerance class, parasitic thermal-junction voltage, finite meter loading or calibration error included. Correction uses EMF, not simply E(Th−Tc).

**Complete example inputs.** `hot_temperature=100 degC`, `cold_temperature=25 degC`. Independently sum the tabulated polynomial at each temperature and differentiate term by term:

| Output key | Independent expected value |
|---|---:|
| `hot_emf` | 0.00427851861580027 V |
| `cold_emf` | 0.0009919772678200114 V |
| `terminal_emf` | 0.003286541347980259 V |
| `hot_sensitivity` | 0.0000467849607861716 V/K |

**Tester cases.** Nominal; Th=Tc gives exactly zero terminal EMF without requiring both absolute EMFs to be zero. Swapping temperatures negates terminal voltage and swaps absolute EMFs; sensitivity follows the new hot input. Tc=0 gives zero cold EMF. Th=400,Tc=0 gives `terminal_emf=0.02087197005052672 V`, agreeing with the rounded 20.872 mV NIST table. With Tc=100 and Th=100+2^-46, terminal voltage is approximately `6.648542805887075e-19 V`, positive and nonzero; use the divided-difference oracle, not subtraction of binary64 absolute EMFs. Reject either input below 0 or above 400.

**Mandatory diagram.** Draw a measurement weld at (100,140), upper copper leg from the weld toward (470,85), lower constantan leg toward (470,195). The two legs do not reconnect at the right. Enclose their right terminations within an isothermal block x=435–515. Open measurement terminals at x=600, with upper “+ Cu” and lower “− constantan”; `terminal_emf` labels the voltage across these open terminals. Bind `hot_temperature` by the weld and `cold_temperature` above the isothermal block. Optional lower legend binds `hot_emf` and `cold_emf` to “E(Th), E(Tc), each relative to 0 degC”; never label either as the actual terminal signal. Caption: “Type-T copper/constantan pair with isothermal reference connections. Vterminal = E(Th) − E(Tc); both junction temperatures use the NIST 0–400 degC branch.”

## 4. sensors.hall_plate_angle

**Purpose and novelty.** Translate the normal projection of a tilted magnetic field into transverse Hall voltage and angular sensitivity. Closest baseline `magnetics.field_energy_density` predicts stored energy, not carrier separation or sensor voltage. The neighboring AMR calculation predicts longitudinal scattering resistance, has different terminals and different angular symmetry.

**Model.** `Bn=B cos(theta)`, `VH=RH I Bn/t`, `dVH/dtheta=-RH I B sin(theta)/t`. `RH` is signed, in m³/C; `I` is signed longitudinal current; `t` is plate thickness in meters; theta is radians from the selected positive plate normal. `RH` includes the chosen voltage-contact polarity convention. For a simple one-carrier material its magnitude is `1/(n q)`; no universal carrier sign or geometry factor is presumed.

**Validity.** Homogeneous plate, uniform field, fixed temperature, negligible loading and offset, calibrated linear Hall regime. `B>=0`, `t>0`, positive user-supplied field/current limits, `B<=field_limit`, `abs(I)<=current_limit`. These are real calibration inputs, not guessed universal sensor ratings. Both zero bias and zero field remain valid. No amplifier rail/clipping is modeled.

**Complete example inputs.** `field_magnitude=0.05 T`, `field_angle=1.0471975511965976 rad` (pi/3), `bias_current=0.001 A`, `hall_coefficient=0.01 m³/C`, `plate_thickness=1e-5 m`, `field_limit=0.2 T`, `current_limit=0.005 A`. Since cos(pi/3)=1/2 and sin(pi/3)=sqrt(3)/2:

| Output key | Independent expected value |
|---|---:|
| `normal_field` | 0.025 T |
| `hall_voltage` | 0.025 V |
| `angle_sensitivity` | -0.04330127018922193 V/rad |

The two values stored as 0.02500000000000001 in JSON differ only by binary64 evaluation of the supplied radian input.

**Tester cases.** At theta=0 with other defaults, outputs `(0.05 T,0.05 V,0 V/rad)`. At B=0 all three outputs are zero. I=0 leaves normal field unchanged but both voltage outputs zero. Reverse I or RH: both voltage outputs change sign, Bn does not. Reverse field orientation by pi: field and Hall voltage reverse. Bias or field equal to its declared maximum is accepted; larger magnitudes, negative B, zero thickness, or nonpositive limits are rejected. Quarter-turn angular values produce floating-point residuals; do not demand `cos(pi/2)` be exact zero.

**Mandatory diagram.** Draw an oblique Hall plate occupying x=110–380,y=100–200. Current arrow crosses it longitudinally left-to-right and binds `bias_current`. Distinct transverse voltage contacts on the other two sides connect to open “+” and “−” sense terminals labeled `hall_voltage`; they must not short the bias contacts. From the plate center draw an outward normal arrow and a tilted B arrow with theta arc; bind `field_magnitude`, `field_angle` and `normal_field` to B, theta and “B normal” respectively. A thickness bracket binds `plate_thickness`. Caption: “Transverse Hall response follows the component normal to the plate. Contact signs use the entered signed Hall coefficient; geometry is schematic, not a calibrated device drawing.”

## 5. sensors.amr_angular_resistance

**Purpose and novelty.** Predict the longitudinal angular resistance of a ferromagnetic thin-film strip. Closest baseline `thermal.resistor_tcr_drift` responds to temperature, not magnetization orientation. `dc.loaded_wheatstone_bridge` cannot supply the material response. Hall voltage is transverse and odd under field reversal; this AMR resistance is longitudinal and pi-periodic. These are physically distinct sensors, not renamed angle calculations.

**Model.** `R=Rperp+DeltaR cos²(theta)`, `V=I R`, `dR/dtheta=-2 DeltaR sin(theta)cos(theta)`. Define `DeltaR=Rparallel−Rperp` from calibration; it may be signed. The constraint `Rperp+min(DeltaR,0)>0` ensures positive resistance at every orientation without incorrectly rejecting negative AMR materials. Input angle is internal magnetization relative to current, not automatically the external magnetic-field angle.

**Validity.** Uniform single-domain thin film, known quasi-static magnetization, fixed temperature and negligible self-heating. Field-to-magnetization rotation, hysteresis, domain switching, barber-pole bias structures and bridges are outside this model. `abs(I)<=current_limit` uses the actual calibrated sensing-current limit. A saturated applied field can align magnetization, but no such alignment is silently assumed.

**Complete example inputs.** `perpendicular_resistance=1000 ohm`, `resistance_anisotropy=20 ohm`, `magnetization_angle=1.0471975511965976 rad`, `bias_current=0.001 A`, `current_limit=0.005 A`. At pi/3, cos²=1/4 and `-2 DeltaR sin cos=-10 sqrt(3)`.

| Output key | Independent expected value |
|---|---:|
| `resistance` | 1005 ohm |
| `voltage_drop` | 1.005 V |
| `angular_resistance_sensitivity` | -17.320508075688775 ohm/rad |

**Tester cases.** theta=0 gives `(1020 ohm,1.02 V,0 ohm/rad)`; theta=pi/2 approaches `(1000,1,0)` with trigonometric roundoff. Adding pi leaves resistance unchanged. Reversing sensing current changes only voltage sign. DeltaR=0 is a valid isotropic limiting case. DeltaR=-20 gives positive resistance and reversed angular sensitivity; DeltaR=-1000 is rejected because one orientation reaches zero. Reject nonpositive Rperp/current limit and excessive current; limits are inclusive.

**Mandatory diagram.** Draw a long thin-film strip x=80–430,y=110–180 with two end contacts. Longitudinal I arrow points right and binds `bias_current`; voltage-drop polarity is “+” at the left and “−” at the right, binding `voltage_drop`. Inside the strip draw internal magnetization M at theta from the rightward current direction; bind `magnetization_angle` to that arc. Right-side legend binds `perpendicular_resistance` to “R perpendicular”, `resistance_anisotropy` to “R parallel − R perpendicular” and `resistance` to “R(theta)”. Caption: “AMR changes longitudinal resistance as internal magnetization rotates. The response repeats every 180 degrees; theta is not necessarily the external-field angle. No Wheatstone bridge is included.”

## 6. sensors.hydrogen_activity_electrode

**Purpose and novelty.** Convert dimensionless hydrogen chemical activities into logarithmic glass-electrode voltage with temperature-dependent Nernst slope. Closest baseline `diodes.shockley_forward_current` is electronic junction transport, not electrochemical membrane equilibrium. It is not a generic logarithm calculator. Neighboring battery intercalation includes occupancy and interaction-energy physics; no battery voltage/SOC model is claimed here.

**Model.** Let `K=kB/q=1.380649e-23/1.602176634e-19 V/K`. `pH=-log10(a_sample)`, `S=K T ln(10)`, `V=Voffset+K T ln(a_sample/a_standard)`, and `dV/da_sample=K T/a_sample`. Positive output increases with hydrogen activity and therefore decreases with pH. Voffset is measured at equal activities at the same temperature.

The catalog evaluates the logarithm as `log1p(max(a-b,0)/b)-log1p(max(b-a,0)/a)`. Both log arguments are nonnegative. This algebra is identical to `ln(a/b)`, stable at equal activities, and avoids rounding a very negative fractional change onto -1. Sensitivity unit is V per unit dimensionless activity, represented as V in the JSON.

**Validity.** Activities, not raw molar concentrations. Ideal monovalent Nernst membrane, no loading, electrode aging, sodium interference, liquid-junction error, calibration slope error or reference-buffer temperature drift. The declared engineering envelope is pH 0–14 (`1e-14<=activity<=1`) and 0–100 degC (`273.15<=T<=373.15 K`), matching the illustrated application range; actual electrode ratings can be narrower. It is not a claim that pH cannot exist outside this interval. Calibration is mandatory for accurate real use.

**Complete example inputs.** `sample_activity=1e-4`, `standard_activity=1e-7`, `temperature=298.15 K`, `offset_voltage=0.005 V`. `pH_sample=4`, `ln(a_sample/a_standard)=3 ln(10)`, so V is the offset plus three Nernst slopes.

| Output key | Independent expected value |
|---|---:|
| `sample_ph` | 4 pH |
| `nernst_slope` | 0.05915934968478233 V/pH |
| `electrode_voltage` | 0.182478049054347 V |
| `activity_sensitivity` | 256.92579121085845 V per unit activity |

**Tester cases.** Equal activities `a=b=1e-7` give pH=7 and V exactly equal to offset; slope is unchanged and sensitivity is `256925.79121085845 V` per unit activity. Swapping activities negates only the voltage change relative to offset; sensitivity remains sample-specific. At `a=1e-14,b=1,T=298.15,offset=0.005`, voltage is `-0.8232308955869526 V`, not an invalid log result. A next-representable increase from equal activities and zero offset must produce a positive nonzero electrode voltage. Both temperature endpoints and activity endpoints are valid. Reject activities outside the declared interval, zero/negative activity, and temperatures outside 273.15–373.15 K.

**Mandatory diagram.** A solution vessel spans x=65–350,y=80–210. Show glass sensing bulb and reference junction as distinct structures immersed in the same sample, not two separate baths. Sample activity label binds `sample_activity` inside the vessel; temperature label binds `temperature`. Reference/calibration legend outside the vessel binds `standard_activity` explicitly as “calibration standard activity”, not necessarily the actual internal fill composition. Separate leads go to open voltmeter terminals x=540, with sensing lead “+”, reference lead “−”, and `electrode_voltage` between them. A lower legend binds `sample_ph` and `nernst_slope`. Caption: “Ideal glass-electrode response is logarithmic in hydrogen activity. Standard activity and offset describe calibration; both electrodes share the sample temperature. High-impedance measurement and calibration are required.”

## 7. sensors.differential_gap_capacitor

**Purpose and novelty.** Evaluate a moving center electrode between two fixed plates, including the nonlinear difference and derivative. Closest baseline `timing.plate_displacement_current` assumes fixed geometry and differentiates voltage to obtain current. Here geometry moves, voltage is not an input, and opposing capacitances encode displacement. It is not a unit change, inverse current calculation or generic capacitor-energy calculation.

**Derivation.** `A=width*length`, `epsilon=epsilon0*epsilon_r`, `g=rest_gap`, x positive toward the near plate. `Cnear=epsilon A/(g-x)`, `Cfar=epsilon A/(g+x)`. Algebra gives `DeltaC=2 epsilon A x/[(g-x)(g+x)]` and `(Cnear-Cfar)/(Cnear+Cfar)=x/g`. Differentiate to obtain `dDeltaC/dx=epsilon A[(g-x)^-2+(g+x)^-2]`. JSON evaluates the difference directly rather than subtracting nearly equal capacitances. `epsilon0=8.8541878128e-12 F/m` is the explicit adopted constant; source's 8.85e-12 is rounded.

**Validity.** Identical overlapping rectangular areas, parallel plates, uniform positive dielectric with `epsilon_r>=1`, negligible parasitics/fringing/tilt. `abs(x)<g` prevents contact. Enforce `g+abs(x)<=min(width,length)/10` as an explicit thin-gap screening heuristic. TI states separation must be small compared with plate dimensions; the 10:1 criterion is this calculator's conservative screening convention, not a quoted source tolerance or guaranteed accuracy. No mechanical stiffness/electrostatic pull-in calculation; motion is externally prescribed. Each capacitance is measured center-to-one-fixed-plate, not a series pair with floating center.

**Complete example inputs.** `plate_width=0.01 m`, `plate_length=0.01 m`, `rest_gap=0.0001 m`, `displacement=0.00002 m`, `relative_permittivity=1`. Area is `1e-4 m²`; instantaneous gaps are 80 and 120 micrometers.

| Output key | Independent expected value |
|---|---:|
| `near_capacitance` | 1.1067734766e-11 F |
| `far_capacitance` | 7.378489844e-12 F |
| `differential_capacitance` | 3.689244922e-12 F |
| `normalized_difference` | 0.2 |
| `differential_sensitivity` | 1.9983409994166667e-7 F/m |

**Tester cases.** At x=0, both capacitances equal `8.8541878128e-12 F`, difference and ratio are exactly zero, sensitivity is `1.77083756256e-7 F/m`. Reversing x swaps capacitances and reverses both differences, while sensitivity remains positive and unchanged. Tiny signed x must preserve a tiny signed differential capacitance even when the individual capacitances round identically. As abs(x) approaches g from below, one capacitance and the sensitivity diverge; actual stops/parasitics invalidate this ideal limit before contact. Reject abs(x)>=g, nonpositive dimensions/gap, epsilon_r<1 and violation of the declared thin-gap rule. The 10:1 equality is accepted. Doubling epsilon_r doubles the four dimensional outputs but leaves normalized difference unchanged.

**Mandatory diagram.** Draw three vertical electrodes at x=150,330,550 from y=80 to y=200. Outer electrodes are fixed; central electrode has a right-pointing displacement arrow to show positive x toward the right-hand “near” plate. Label right gap “g − x” and left gap “g + x”; these relation labels must not bind `rest_gap` as if either equals g. Bind `rest_gap` in a separate zero-displacement legend and bind `displacement` to the arrow. Bind `near_capacitance` in the right gap and `far_capacitance` in the left gap. The central plate has its own lead exiting downward; the fixed plates have independent outward leads. A short lower legend binds `differential_capacitance` to “Cnear − Cfar” and `normalized_difference` to “(Cnear − Cfar)/(Cnear + Cfar)”. Caption: “A moving center plate decreases one gap while increasing the other. Three terminals remain separate; x is positive rightward. Ideal parallel-plate geometry neglects fringe fields, parasitics and electrostatic loading.”

## Implementation, tester and reviewer handoff

Implement exactly these seven IDs and one diagram per ID. No new backend expression function is required. Keep temperature units explicit: NTC and Nernst use kelvin; RTD and Type-T use Celsius because their reference coefficients require it. Preserve signed sensor polarities and positive/negative displacement. Do not replace the factored thermocouple/capacitance expressions with subtractive shortcuts. Do not replace the two-positive-log1p Nernst identity with a single near-minus-one argument.

Independent testers should assert every nominal output, every listed valid limit, scalar and relational rejection, signed symmetry, near-equal thermocouple temperatures, near-equal activities and tiny signed capacitor displacement. Reviewers should examine actual diagram pixels for Hall transverse versus AMR longitudinal terminal placement, thermocouple non-shorted termination leads, pH calibration-label meaning, and capacitor gap-sign consistency. Parent must run the real Rust examples, all tests and native desktop/diagram journeys only after the complete research/development waves.

No builds, tests, lint, formatters, engine calls, production changes or hardware verification were performed. No accuracy/compliance claim extends beyond the stated ideal models. Residual risks: mirrored AMR document provenance, illustrative calibration data, component-specific operating limits, and unmodeled physical nonidealities. These are explicitly documented, not unresolved implementation blockers.
