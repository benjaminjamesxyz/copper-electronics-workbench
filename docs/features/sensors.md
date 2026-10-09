# Sensor transduction — executable calculator models

Rust evaluates the catalog; diagrams are static explanations, not live plots. Examples are engineering estimates, not device specifications or certification. Calibration and actual component ratings remain necessary. Nominal relative tolerance is 1e-9; numerical tolerance is not physical accuracy. The shared engine rejects nonfinite inputs and enforces scalar bounds and input-only constraints before evaluating outputs.

## 1. sensors.ntc_steinhart_hart

**Level:** intermediate.

- [ILX Lightwave TNSTEIN-1, equation (1), 2005](https://api.p1.mks.com/medias/sys_master/images/images/hdb/hac/8797291479070/TN-STEIN-1-Thermistor-Constant-Conversions-Beta-to-Steinhart-Hart.pdf)


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

**Level:** intermediate.

- [Texas Instruments SDAA150, section 1.2.2.1, equations 21 and 23–26](https://www.ti.com/document-viewer/lit/html/SDAA150/GUID-D7335C6E-A97D-4849-80CC-6781C31A787C.html)


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

**Level:** expert.

- [NIST SRD 60, Type T reference function, 0 to 400 degrees Celsius](https://its90.nist.gov/downloadFiles/type_t.tab.txt)
- [Texas Instruments SBAA274A, sections 1.1 and 1.3](https://www.ti.com/lit/an/sbaa274a/sbaa274a.pdf)


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

**Level:** basic.

- [Allegro AN296065, Hall Effect Principles, equation 1](https://www.allegromicro.com/en/insights-and-innovations/technical-documents/hall-effect-sensor-ic-publications/allegro-hall-effect-sensor-ics)


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

**Level:** basic.

- [Philips Semiconductors, Magnetoresistive sensors for magnetic field measurement, 2000, appendix 1 page 15 equation 2 (manufacturer document mirror)](https://manualsdump.com/en/manuals/philips-magnetoresistive_sensor/271677/15)


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


**Source provenance.** Research read the Philips application note through a third-party mirror, including the equation image. No authenticated NXP-hosted replacement was obtained; the usermanual.wiki PDF route returned a 403 page. Ambiguous resistivity notation does not define the sign: calibrated perpendicular resistance and signed parallel-minus-perpendicular resistance explicitly determine both endpoints.


## 6. sensors.hydrogen_activity_electrode

**Level:** basic.

- [Texas Instruments AN-1852 SNOA529A, section 1.1 equations 1–2](https://www.ti.com/lit/an/snoa529a/snoa529a.pdf)


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

