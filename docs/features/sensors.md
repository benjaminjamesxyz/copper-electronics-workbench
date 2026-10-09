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

