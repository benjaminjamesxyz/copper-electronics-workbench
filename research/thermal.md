# Thermal and reliability design — eight researched calculators

This lane owns exactly eight calculations in `research/thermal.json`. They are closed-form engineering estimates, not finite-element simulation, package certification, predicted MTBF, or standards-compliance claims. All absolute temperatures are **kelvin**; to enter a Celsius value add **273.15**. Temperature differences in kelvin and degrees Celsius have the same numerical value. Examples are illustrative design inputs, not specifications for a named product. Constants and physical models come from the primary references below; numerical answers were independently derived using arithmetic and high-precision Decimal exponentials, not the application expression engine.

## Ownership and distinctness

| ID | Level | Physical problem |
|---|---|---|
| `thermal.junction_feedback_equilibrium` | expert | Coupled device-temperature/power equilibrium and local thermal stability |
| `thermal.heatsink_resistance_budget` | intermediate | Allocate a temperature ceiling across a specified dominant series cooling chain |
| `thermal.parallel_conduction_paths` | intermediate | Geometry-derived solid heat-path conductance and heat sharing |
| `thermal.lumped_thermal_step` | intermediate | Heat-storage dynamics of a convectively cooled uniform body |
| `thermal.arrhenius_acceleration` | expert | Equivalent exposure under one thermally activated degradation mechanism |
| `thermal.resistor_power_derating` | basic | Interpolation of a manufacturer flat-then-linear continuous power rating |
| `thermal.resistor_tcr_drift` | basic | Temperature dependence of a resistor's material/component resistance |
| `thermal.free_linear_expansion` | basic | Free dimensional change of an unconstrained hardware member |

The junction problem deliberately includes **electrothermal feedback**, rather than counting `T=Ta+PR` and its inverse as two calculators. The heatsink problem instead assumes independently supplied, temperature-independent dissipation and a known case/interface/sink chain. Solid-conduction paths calculate conductivity/geometry and parallel heat sharing; neither their inverse nor series-resistance addition is separately counted. Thermal capacitance is actual heat storage, not an electrical RC calculator despite the mathematical analogy. MOSFET, converter, resistor and PCB **electrical loss synthesis** remain in their respective lanes; thermal calculators consume the resulting heat power. Expansion is not transmission-line skew or electrical trace geometry.

All output keys differ from input keys. Outputs can reference preceding outputs. Constraints reference **only inputs/constants**, because integration evaluates constraints before outputs. Every example enumerates all inputs and every output. Only permitted scalar operators/functions are used; `min`/`max` implement the derating plateau without conditionals.

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

## 1. Junction temperature with electrothermal feedback

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

## 2. Maximum heatsink thermal resistance

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

## 3. Two parallel solid-conduction heat paths

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

## 4. Lumped body temperature after a power step

**ID:** `thermal.lumped_thermal_step`; **level:** intermediate. **References:** 4–5.

### Model and derivation

For a spatially uniform body, stored heat is characterized by `C_th=rho V cp`. Convection removes `h A_s(T−Ta)` watts. Adding constant uniform internal heating `P` to the university energy balance gives

`C_th dT/dt = P − h A_s(T−Ta)`.

Define `R_th=1/(h A_s)`, `tau=R_th C_th` and `T_inf=Ta+P R_th`. Solving the first-order balance from `T(0)=T_initial` yields

`T(t)=T_inf+(T_initial−T_inf) exp(−t/tau)`.

Applicability is screened using `L_c=V/A_s` and **Bi=h V/(A_s k)<0.1**.

### Units, domain and limitations

Volume m³, density kg/m³, specific heat J/(kg K), area m², convection coefficient W/(m² K), conductivity W/(m K), time s, power W, temperatures K. Heat capacity output is J/K, convection resistance K/W, time constant s and Bi dimensionless. Volume, density, specific heat, area, convection and conductivity are strictly positive. Temperatures are positive, power/time nonnegative. The expanded input-only Biot expression is constrained below 0.1.

An initially hot body with `P=0` cools; an initially colder body warms. At `t=0` the initial temperature is recovered, and at long time the asymptote is recovered. Material properties, ambient and convection remain constant. Radiation, contacts and multiple thermal masses are excluded. A small global Biot number is not proof that localized semiconductor junction heating is uniform; use manufacturer transient thermal impedance or a distributed model when appropriate.

### Independent worked example

Inputs: `V=10^-5 m³`, `rho=2700 kg/m³`, `cp=900 J/(kg K)`, `A_s=0.01 m²`, `h=10 W/(m² K)`, `k=200 W/(m K)`, `Ta=T_initial=298.15 K`, `P=2 W`, `t=243 s`.

Mass is 0.027 kg. **C_th=24.3 J/K**, **R_th=10 K/W**, **tau=243 s**, **Bi=10×10^-5/(0.01×200)=0.00005** and **T_inf=318.15 K**. At one time constant,

`T=318.15−20 exp(−1)` with `exp(−1)=0.3678794411714423`, giving **T(243)=310.7924111765712 K**. Rise is 63.2120559% of its ultimate 20 K value.

## 5. Arrhenius accelerated-life exposure

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

## 6. Linear resistor power derating

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

## 7. Resistor resistance drift from temperature

**ID:** `thermal.resistor_tcr_drift`; **level:** basic. **Reference:** 9.

### Model and derivation

Vishay defines signed TCR as `(R−R_ref)/(R_ref (T−T_ref))`, conventionally quoted in ppm per degree. Converting ppm to a dimensionless fraction gives

- `d=(TCR_ppm × 10^-6)(T−T_ref)`;
- `deltaR=R_ref d`;
- `R=R_ref(1+d)`.

The formula is evaluated even when temperatures are equal; there is no division by `T−T_ref`.

### Units, domain and limitations

Resistance ohm; temperatures K; signed component TCR ppm/K; drift dimensionless. Reference resistance is positive. Fit interval endpoints are positive and ordered; both reference and operating temperatures must lie in that interval. `1+d>0` rejects a physically invalid nonpositive extrapolated resistance. Negative TCR and cooling are allowed.

The entered temperature is the **element** temperature including self-heating. This model computes drift only, not electrical power or a thermal equilibrium. Datasheet `±100 ppm/K` is a bound, not a measured +100 slope; evaluate both signs for bounds. Vishay warns that terminal contributions matter, TCR curves may be nonlinear, and overload damage is not reversible TCR.

### Independent worked example

Inputs: `R_ref=10000 ohm`, signed TCR `+100 ppm/K`, reference `298.15 K`, operating `358.15 K`, valid interval `[218.15,443.15] K`.

Temperature difference is 60 K. **d=100×10^-6×60=0.006**, **deltaR=60 ohm**, **R=10060 ohm**. The 0.6% change is signed positive, with no need to infer electrical loading.

## 8. Free linear thermal expansion

**ID:** `thermal.free_linear_expansion`; **level:** basic. **Reference:** 10.

### Model and derivation

The directional linear expansion coefficient is the normalized change of length with temperature. With a constant mean coefficient and small strain, integrate the first-order relation relative to initial length:

- `epsilon=alpha(T_final−T_initial)`;
- `deltaL=L_initial epsilon`;
- `L_final=L_initial+deltaL`.

For a differential coefficient normalized to instantaneous length, the exact constant-coefficient solution is exponential; the catalog intentionally uses the ordinary **first-order small-strain engineering approximation**. A supplied mean engineering coefficient over the interval directly supports the linear form.

### Units, domain and limitations

Lengths m, temperature K, directional coefficient 1/K, strain dimensionless. Initial length positive, temperatures positive, validated interval ordered and both temperatures inside it. Signed CTE, heating and cooling are supported. The domain guard **abs(alpha deltaT)<=0.01** limits the first-order model to at most 1% strain; it is a declared approximation threshold, not a material standard. This bound also guarantees positive final length.

The member is unconstrained and uniformly heated. No stress, curvature, differential-material warpage or solder-fatigue life is inferred. For PCB anisotropy use the coefficient along the modeled length. Do not apply a constant coefficient across glass transition, phase change, or a region with strong coefficient variation without appropriate characterization.

### Independent worked example

Inputs: `L_initial=0.1 m`, illustrative `alpha=23×10^-6/K`, initial `298.15 K`, final `378.15 K`, fit interval `[273.15,423.15] K`.

Temperature difference is 80 K. **epsilon=0.00184**, **deltaL=0.000184 m** (0.184 mm), and **L_final=0.100184 m**. The 0.184% free strain meets the declared 1% approximation limit. The coefficient is an illustrative input, not a value attributed to the silver reference.

## Integration handoff

The catalog provides exactly eight complete input/default/domain/output/example definitions and a 10^-6 relative comparison tolerance per example. All numerical examples include supporting outputs. Cross-input/model guards reject unstable feedback, exhausted heatsink headroom, excessive Biot number, unrepresentable acceleration exposure, inverted derating intervals, out-of-fit material temperatures and nonphysical resistance extrapolation. All displayed calculations have finite valid default inputs and independently derived finite expected outputs.

No repository builds, linters, formatters, engine checks or tests were run by this researcher. The integration owner should run catalog schema/expression/domain checks and independently exercise example equality, boundary values (including equal Arrhenius temperatures, t=0, beta=0, zero heating, derating breakpoint/endpoint and negative TCR/cooling), and the stated invalid-domain rejections after promoting the catalog.
