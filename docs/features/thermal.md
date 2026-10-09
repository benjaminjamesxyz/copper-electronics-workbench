# Thermal and reliability design — executable calculator models

This lane owns exactly eight calculations in `backend/catalog/thermal.json`. They are closed-form engineering estimates, not finite-element simulation, package certification, predicted MTBF, or standards-compliance claims. All absolute temperatures are **kelvin**; to enter a Celsius value add **273.15**. Temperature differences in kelvin and degrees Celsius have the same numerical value. Examples are illustrative design inputs, not specifications for a named product. Constants and physical models come from the primary references below; numerical answers were independently derived using arithmetic and high-precision Decimal exponentials, not the application expression engine.


## Primary references

1. **Texas Instruments SLVA462, Nikhil Seshasayee, Understanding Thermal Dissipation and Design of a Heatsink (2011)** — equations relating junction temperature, ambient, power and thermal resistance; series package/interface/heatsink chain and airflow dependence, pages 1–3. [PDF](https://www.ti.com/lit/an/slva462/slva462.pdf).
2. **Texas Instruments SLUA844B, Andy Chen, How to Properly Evaluate Junction Temperature with Thermal Metrics (2019 revision)** — board-dependent junction-to-ambient resistance, forced single-face applicability of junction-to-case resistance, and multiple thermal conduction paths, pages 4–7. [PDF](https://www.ti.com/lit/an/slua844b/slua844b.pdf). Its introductory generic “10 °C halves life” rule is **not** used; the reliability calculation instead uses mechanism-specific Arrhenius parameters.
3. **Texas Instruments SNVAAA4, Power Supply Designs for eBike That Meet the New GB42295 National Standard (2025)** — Table 2-2 note 2 describes positive MOSFET resistance temperature coefficient, approximately 4500 ppm/°C, and dependence of junction temperature on total dissipation. [PDF](https://www.ti.com/lit/an/snvaaa4/snvaaa4.pdf). The 0.004/K example power coefficient is illustrative, not a claim that all devices have this coefficient. The closed-form feedback/stability result below is an explicit derivation, not a quoted manufacturer formula.
4. **University of South Carolina ECHE 460, Price, Matthews and Papathansiou, Heat Transfer Analysis in Solids (2000)** — Fourier conduction equations 1–2, lumped energy balance equations 5–12, resistance interpretation equations 13–14 and `Bi<0.1` criterion equation 15. [PDF](https://cse.sc.edu/~gatzke/460/F-Heat%20Conduction.pdf).
5. **Brigham Young University ME340, Lumped Capacitance Method, table 5.1** — spatially uniform temperature, `rho V cp`, convection resistance, characteristic length `V/As`, thermal time constant and Biot criterion. [PDF](https://www.et.byu.edu/~vps/ME340/TABLES/5.1.pdf).
6. **NIST/SEMATECH e-Handbook of Statistical Methods, 8.1.5.1 Arrhenius** — life model, acceleration-factor ratio and mechanism-dependent activation energy. [Reference](https://www.itl.nist.gov/div898/handbook/apr/section1/apr151.htm). The handbook's historical Celsius offset and rounded Boltzmann value are not copied into the implementation; inputs are already kelvin and the constant comes from current CODATA.
7. **NIST CODATA, Boltzmann constant in eV/K** — `8.617333262… × 10^-5 eV/K`, exact in principle following SI definitions. [Constant](https://physics.nist.gov/cgi-bin/cuu/Value?kev). Catalog decimal `8.617333262145e-5` is a finite high-precision representation.
8. **Vishay Sfernice 53048, Power Dissipation in High Precision Vishay Sfernice Chip Resistors and Arrays (2009)** — sections D–E explain flat/linear conventional derating, thermal origins and mounting/solder-pad limitations. [PDF](https://www.vishay.com/docs/53048/pprachp.pdf).
9. **Vishay Dale 30405, Bryan Yarborough, Temperature Coefficient of Resistance for Current Sensing (2020)** — signed ppm/temperature definition and linear resistance equation, pages 1–2; component versus element TCR, self-heating and nonlinearity, page 4. [PDF](https://www.vishay.com/docs/30405/whitepapertcr.pdf).
10. **NIST Journal of Research 100(2), David R. Smith and F. R. Fickett, Low-Temperature Properties of Silver (1995), section 2.3.3** — definition of directional linear expansion coefficient and its strong possible temperature dependence. [PDF](https://nvlpubs.nist.gov/nistpubs/jres/100/2/j12smi.pdf). This supplies the physical definition, not an aluminum material coefficient; the catalog coefficient is an illustrative user-supplied material property.

Sources were retrieved and read during research. Incomplete extraction warnings on irrelevant PDF material do not supply evidence for unobserved diagrams: only the stated readable text/equations are used here.


## Execution contract

The shared Rust engine alone evaluates these scalar programs. Inputs must contain exactly the catalog keys and finite numeric values, satisfy their inclusive/exclusive ranges, and pass input-only constraints before any sequential outputs are evaluated. Invalid domains and nonfinite intermediate arithmetic are explicit errors, never clamps or fallback results. All eight independent example vectors and expected outputs retain a relative tolerance of `1e-6`; numerical agreement is not physical model accuracy.

## `thermal.resistor_power_derating` — Linear resistor power derating

**ID:** `thermal.resistor_power_derating`; **level:** basic. **Reference:** 8.

### Model and derivation

Take the manufacturer's full-rating point `(T_rated,P_rated)` and zero-power endpoint `(T_zero,0)`. Straight-line interpolation gives the fractional rating `(T_zero−Ta)/(T_zero−T_rated)` on the sloped segment. The plateau and zero bound are encoded without a conditional:

- `f=min(1,max(0,(T_zero−Ta)/(T_zero−T_rated)))`;
- `P_allowed=P_rated f`;
- `margin=P_allowed−P_actual`.

This is a specified rating-curve interpolation, not a separately counted inverse of a junction-temperature equation.

### Units, domain and limitations

Temperatures K, powers W, fraction dimensionless. Rated power is positive, actual power nonnegative, temperatures positive, `T_zero>T_rated`, and `Ta<=T_zero`. Values above the zero-power endpoint are rejected as outside the curve. At the endpoint the allowance is zero; below the breakpoint it is full rated power. A negative margin is intentionally permitted and is an informative overload indication.

The actual datasheet curve may use ambient, terminal, case or solder-pad temperature. This calculation specifically labels ambient; do not substitute another reference uncritically. It does not authorize increasing rated power at low temperature, even though detailed manufacturer thermal analysis can establish different application limits. Actual mounting, pulse capability, maximum voltage and solder/board limits remain independent restrictions.

### Independent worked example

Inputs: `P_rated=0.25 W`, full-rating breakpoint `343.15 K` (70 °C), zero endpoint `428.15 K` (155 °C), ambient `373.15 K` (100 °C), actual heat `0.12 W`.

Remaining temperature span is 55 K out of 85 K. **f=11/17=0.6470588235294118**, **P_allowed=11/68=0.16176470588235295 W**, and **margin=0.04176470588235294 W**. The illustrative 70/155 °C curve is not asserted to apply to every resistor.


### Executable input and output contract

| Input key | Label | Unit | Default | Admissible scalar range |
|---|---|---|---|---|
| `rated_power` | Full continuous power rating | W | 0.25 | > 0 |
| `rated_temperature` | Full-rating temperature breakpoint | K | 343.15 | > 0 |
| `zero_power_temperature` | Zero-power temperature endpoint | K | 428.15 | > 0 |
| `ambient_temperature` | Datasheet ambient reference temperature | K | 373.15 | > 0 |
| `actual_power` | Actual continuous resistor dissipation | W | 0.12 | >= 0 |

Outputs are evaluated in the following order and may use earlier outputs.

| Output key | Unit | Executable expression | Independently expected example value |
|---|---|---|---|
| `rating_fraction` | 1 | `min(1,max(0,(zero_power_temperature-ambient_temperature)/(zero_power_temperature-rated_temperature)))` | 0.6470588235294118 |
| `allowed_power` | W | `rated_power*rating_fraction` | 0.16176470588235295 |
| `power_margin` | W | `allowed_power-actual_power` | 0.04176470588235294 |

Complete independent example input vector (relative tolerance `1e-6`):

```json
{
  "rated_power": 0.25,
  "rated_temperature": 343.15,
  "zero_power_temperature": 428.15,
  "ambient_temperature": 373.15,
  "actual_power": 0.12
}
```

## `thermal.junction_feedback_equilibrium` — Junction temperature with electrothermal feedback

**ID:** `thermal.junction_feedback_equilibrium`; **level:** expert. **References:** 1–3.

### Model and derivation

Let device dissipation near a fixed electrical operating point be approximated by

`P(Tj) = P_ref [1 + beta (Tj − T_ref)]`.

`P_ref` is watts, `beta` is fractional power change in 1/K, and `R_th` is the complete application junction-to-ambient resistance in K/W. This model can represent approximately constant-current conduction-dominated loss, or another experimentally fitted total-power characteristic; it does not compute electrical losses.

The heat balance is `Tj − Ta = R_th P(Tj)`. Set `x=Tj−Ta`; then

`x = R_th P_ref [1 + beta(Ta−T_ref) + beta x]`.

Therefore

- `g = R_th P_ref beta` (dimensionless feedback gain);
- `Tj = Ta + R_th P_ref [1 + beta(Ta−T_ref)] / (1−g)`;
- `P_eq = P_ref [1 + beta(Tj−T_ref)]`.

For positive heat capacitance, a small temperature perturbation obeys `C d(deltaT)/dt = (P_ref beta − 1/R_th) deltaT`. Consequently `g<1` is the local stability condition. A formal algebraic root with `g>=1` is not a valid stable design result and is rejected.

### Units, domain and limitations

All temperatures K, power W, resistance K/W, `beta` 1/K. The fit minimum and maximum are K and must be ordered. Ambient and reference temperatures must lie in the entered fit interval. Reference power and thermal resistance are strictly positive; beta is nonnegative; fitted power at ambient must remain positive. The feedback guard is evaluated **before** division by `1−g`. Computed junction temperature must not exceed the fit maximum. The lower fit bound follows automatically because positive dissipation and `g<1` imply `Tj>Ta`.

The interval is supplied from actual characterization, not inferred from the illustrative defaults. The chosen maximum should also respect the device's intended operating limit. The result is extremely sensitive to uncertain power/thermal coefficients near `g=1`. Datasheet JEDEC `R_theta_JA` is not a universal application resistance. Hotspots, temperature-dependent convection, protection behavior and nonlinear leakage/SOA are outside the model.

### Independent worked example

Inputs: `Ta=T_ref=298.15 K`, `P_ref=2 W`, `beta=0.004/K`, `R_th=20 K/W`, fit interval `[233.15,423.15] K`.

`g=20×2×0.004=0.16`. Rise is `40/0.84 = 1000/21 = 47.6190476190476 K`. Thus **Tj=345.7690476190476 K** and **P_eq=2/0.84=2.380952380952381 W**. Independent balance check: `20×2.380952380952381=47.6190476190476 K`. The zero-feedback estimate would be 338.15 K, showing why this is not merely inverse heatsink sizing.


### Executable input and output contract

| Input key | Label | Unit | Default | Admissible scalar range |
|---|---|---|---|---|
| `ambient_temperature` | Ambient temperature | K | 298.15 | > 0 |
| `reference_temperature` | Power reference temperature | K | 298.15 | > 0 |
| `reference_power` | Device power at reference temperature | W | 2 | > 0 |
| `power_coefficient` | Fractional power temperature coefficient | 1/K | 0.004 | >= 0 |
| `thermal_resistance` | Application junction-to-ambient resistance | K/W | 20 | > 0 |
| `fit_temperature_min` | Minimum validated model temperature | K | 233.15 | > 0 |
| `fit_temperature_max` | Maximum validated model temperature | K | 423.15 | > 0 |

Outputs are evaluated in the following order and may use earlier outputs.

| Output key | Unit | Executable expression | Independently expected example value |
|---|---|---|---|
| `feedback_gain` | 1 | `thermal_resistance*reference_power*power_coefficient` | 0.16 |
| `junction_temperature` | K | `ambient_temperature+thermal_resistance*reference_power*(1+power_coefficient*(ambient_temperature-reference_temperature))/(1-feedback_gain)` | 345.7690476190476 |
| `equilibrium_power` | W | `reference_power*(1+power_coefficient*(ambient_temperature-reference_temperature))/(1-feedback_gain)` | 2.380952380952381 |

Complete independent example input vector (relative tolerance `1e-6`):

```json
{
  "ambient_temperature": 298.15,
  "reference_temperature": 298.15,
  "reference_power": 2,
  "power_coefficient": 0.004,
  "thermal_resistance": 20,
  "fit_temperature_min": 233.15,
  "fit_temperature_max": 423.15
}
```

### Numerical implementation

Implementation evaluates equilibrium power directly as `P_ref[1+beta(Ta-T_ref)]/(1-g)`. This follows the same heat balance, but avoids subtracting rounded absolute junction/reference temperatures when evaluating fitted power. No model domain or example oracle changes.

## `thermal.arrhenius_acceleration` — Arrhenius accelerated-life exposure

**ID:** `thermal.arrhenius_acceleration`; **level:** expert. **References:** 6–7.

### Model and derivation

NIST's mechanism-specific life model is `t_f(T)=A exp(Ea/(k_B T))`. Taking the use/test life ratio cancels `A`:

`AF=t_f(T_use)/t_f(T_test)=exp[(Ea/k_B)(1/T_use−1/T_test)]`.

For constant test exposure, **t_equivalent=AF t_test**. The equivalent is an amount of use-temperature exposure under this model, not an observed lifetime.

### Units, domain and limitations

Activation energy eV (explicit non-SI convention), `k_B=8.617333262145e-5 eV/K`, absolute temperatures K and duration s. Positive activation energy and positive temperatures are required; test temperature must be at least use temperature, giving `AF>=1`. Zero-duration exposure is allowed and gives zero equivalent exposure. Equal temperatures give `AF=1`.

Activation energy must be established for the relevant materials and failure mechanism. Raising stress must not create a different failure process; voltage, humidity and mechanical stress are unchanged. The relevant temperatures are at the failure site. This is not a “10 °C doubles aging” heuristic, generic solder fatigue model, MTBF estimator or confidence analysis. The catalog activation-energy maximum of 10 eV is a broad numerical input limit, not a claim of typical mechanism energies.

To prevent numerical overflow, constrain the input-only exponent to `ln(1e300/max(test_duration,1))`; thus both the factor and equivalent seconds remain at most 10^300. This is a computational-domain limit, not a physical acceleration limit. Duration also has a finite broad input maximum of 10^100 s.

### Independent worked example

Inputs: `Ea=0.7 eV`, `T_use=328.15 K` (55 °C), `T_test=398.15 K` (125 °C), test exposure `360000 s` (100 h).

Independent reciprocal-temperature arithmetic gives exponent **4.352152076475486**. Its high-precision exponential is **AF=77.64538204961666**. Consequently **equivalent exposure=27,952,337.537862 s**, or approximately 7764.538205 h. The acceleration means use-temperature exposure is longer, not shorter, than the accelerated-test exposure.


### Executable input and output contract

| Input key | Label | Unit | Default | Admissible scalar range |
|---|---|---|---|---|
| `activation_energy` | Characterized activation energy | eV | 0.7 | > 0; <= 10 |
| `use_temperature` | Use failure-site temperature | K | 328.15 | > 0 |
| `test_temperature` | Accelerated-test failure-site temperature | K | 398.15 | > 0 |
| `test_duration` | Test exposure duration | s | 360000 | >= 0; <= 1e+100 |

Outputs are evaluated in the following order and may use earlier outputs.

| Output key | Unit | Executable expression | Independently expected example value |
|---|---|---|---|
| `acceleration_factor` | 1 | `exp(activation_energy/8.617333262145e-5*((test_temperature-use_temperature)/test_temperature)/use_temperature)` | 77.64538204961666 |
| `equivalent_use_duration` | s | `test_duration*acceleration_factor` | 27952337.537862 |

Complete independent example input vector (relative tolerance `1e-6`):

```json
{
  "activation_energy": 0.7,
  "use_temperature": 328.15,
  "test_temperature": 398.15,
  "test_duration": 360000
}
```

### Numerical implementation

Implementation computes the exponent as `(Ea/k_B)*((T_test-T_use)/T_test)/T_use`, algebraically identical to the reciprocal difference. This avoids cancellation between nearly equal reciprocals and retains exact unity at equal temperatures. The input-only overflow guard uses the same expression. No model domain or example oracle changes.

## `thermal.heatsink_resistance_budget` — Maximum heatsink thermal resistance

**ID:** `thermal.heatsink_resistance_budget`; **level:** intermediate. **References:** 1–2.

### Model and derivation

For a dominant series heat-flow path,

`Tj = Ta + P (R_jc + R_cs + R_sa)`.

Applying a chosen design junction ceiling gives

- `R_total,max=(Tj_limit−Ta)/P`;
- `R_sa,max=R_total,max−R_jc−R_cs`.

A selected heatsink must have actual `R_sa <= R_sa,max` under the installation's cooling conditions.

### Units, domain and limitations

Temperature K, dissipation W, all thermal resistances K/W. Power and junction-to-case resistance are strictly positive; interface resistance is nonnegative. The junction limit must exceed ambient and the remaining heatsink budget must be **strictly positive**. A zero/negative budget is an infeasible design request, not a negative resistance to display.

Use the specified top or bottom case face and ensure nearly all heat exits through it. TI warns against applying `R_theta_JC` to ordinary package-top measurements without this condition. Do not add datasheet `R_theta_JA` to the chain. Use worst-case steady dissipation after any required electrical temperature correction; this calculator itself assumes supplied power is constant. Interface pressure, spreading, airflow, orientation, enclosure and heatsink radiation affect actual performance.

### Independent worked example

Inputs: `Ta=313.15 K`, ceiling `393.15 K`, `P=10 W`, `R_jc=1.5 K/W`, `R_cs=0.5 K/W`.

Available rise is 80 K, so **total budget=8 K/W** and **maximum sink resistance=8−1.5−0.5=6 K/W**. At that bound the sink rises 60 K, interface 5 K and package 15 K, totaling exactly 80 K.


### Executable input and output contract

| Input key | Label | Unit | Default | Admissible scalar range |
|---|---|---|---|---|
| `ambient_temperature` | Ambient temperature | K | 313.15 | > 0 |
| `junction_limit` | Design junction-temperature limit | K | 393.15 | > 0 |
| `dissipated_power` | Device dissipated power | W | 10 | > 0 |
| `junction_case_resistance` | Junction-to-case thermal resistance | K/W | 1.5 | > 0 |
| `case_sink_resistance` | Case-to-sink interface resistance | K/W | 0.5 | >= 0 |

Outputs are evaluated in the following order and may use earlier outputs.

| Output key | Unit | Executable expression | Independently expected example value |
|---|---|---|---|
| `total_resistance_budget` | K/W | `(junction_limit-ambient_temperature)/dissipated_power` | 8 |
| `maximum_sink_resistance` | K/W | `total_resistance_budget-junction_case_resistance-case_sink_resistance` | 6 |

Complete independent example input vector (relative tolerance `1e-6`):

```json
{
  "ambient_temperature": 313.15,
  "junction_limit": 393.15,
  "dissipated_power": 10,
  "junction_case_resistance": 1.5,
  "case_sink_resistance": 0.5
}
```

## `thermal.parallel_conduction_paths` — Two parallel solid-conduction heat paths

**ID:** `thermal.parallel_conduction_paths`; **level:** intermediate. **References:** 2 and 4.

### Model and derivation

Integrating steady one-dimensional Fourier conduction through uniform length `L_i`, conductivity `k_i` and cross section `A_i` gives

`Q_i=k_i A_i (T_hot−T_sink)/L_i` and `R_i=L_i/(k_i A_i)`.

At common isothermal endpoints both paths share the same temperature drop and energy conservation requires `P=Q_1+Q_2`. Thus conductances add:

- `R_eq=1/(1/R_1+1/R_2)`;
- `T_hot=T_sink+P R_eq`;
- `Q_i=P R_eq/R_i`.

The power-based expression for `Q_i` avoids subtracting nearly equal absolute temperatures.

### Units, domain and limitations

Lengths m; cross sections m²; directional conductivities W/(m K); heat W; temperatures K; resistances K/W. Each length, area and conductivity is strictly positive, sink temperature positive and source heat nonnegative. These input domains guard every division, so no additional cross-input constraint is needed.

Paths must physically join the same two isothermal nodes. Neglect contact/interface resistance, lateral interaction, internal generation, spreading resistance and heat losses along the paths. For anisotropic material choose conductivity along heat flow. This is a solid-path estimate, not a PCB thermal spreading or thermal-via qualification model.

### Independent worked example

Path 1: `L1=0.005 m`, `A1=0.0001 m²`, `k1=200 W/(m K)`; path 2: `L2=0.002 m`, `A2=0.0004 m²`, `k2=1 W/(m K)`. Common sink `298.15 K`; total heat `21 W`.

**R1=0.25 K/W**, **R2=5 K/W**. Conductance is `4+0.2=4.2 W/K`, hence **R_eq=5/21=0.2380952380952381 K/W**. Total rise is `21×5/21=5 K`, so **T_hot=303.15 K**. **Q1=5/0.25=20 W**, **Q2=5/5=1 W**; heat flows sum to 21 W.


### Executable input and output contract

| Input key | Label | Unit | Default | Admissible scalar range |
|---|---|---|---|---|
| `path_one_length` | Path 1 heat-flow length | m | 0.005 | > 0 |
| `path_one_area` | Path 1 cross-sectional area | m^2 | 0.0001 | > 0 |
| `path_one_conductivity` | Path 1 thermal conductivity | W/(m*K) | 200 | > 0 |
| `path_two_length` | Path 2 heat-flow length | m | 0.002 | > 0 |
| `path_two_area` | Path 2 cross-sectional area | m^2 | 0.0004 | > 0 |
| `path_two_conductivity` | Path 2 thermal conductivity | W/(m*K) | 1 | > 0 |
| `sink_temperature` | Common sink temperature | K | 298.15 | > 0 |
| `source_power` | Total source heat flow | W | 21 | >= 0 |

Outputs are evaluated in the following order and may use earlier outputs.

| Output key | Unit | Executable expression | Independently expected example value |
|---|---|---|---|
| `path_one_resistance` | K/W | `path_one_length/(path_one_conductivity*path_one_area)` | 0.25 |
| `path_two_resistance` | K/W | `path_two_length/(path_two_conductivity*path_two_area)` | 5 |
| `equivalent_resistance` | K/W | `1/(1/path_one_resistance+1/path_two_resistance)` | 0.2380952380952381 |
| `hot_temperature` | K | `sink_temperature+source_power*equivalent_resistance` | 303.15 |
| `path_one_heat` | W | `source_power*equivalent_resistance/path_one_resistance` | 20 |
| `path_two_heat` | W | `source_power*equivalent_resistance/path_two_resistance` | 1 |

Complete independent example input vector (relative tolerance `1e-6`):

```json
{
  "path_one_length": 0.005,
  "path_one_area": 0.0001,
  "path_one_conductivity": 200,
  "path_two_length": 0.002,
  "path_two_area": 0.0004,
  "path_two_conductivity": 1,
  "sink_temperature": 298.15,
  "source_power": 21
}
```

