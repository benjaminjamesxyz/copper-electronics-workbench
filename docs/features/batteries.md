# Battery design — executable features

Rust evaluates the definitions in [`backend/catalog/batteries.json`](../../backend/catalog/batteries.json). Flutter uses the model-specific static illustrations in `batteriesDiagrams`; these illustrations are not computed plots. SOC and occupancy are fractions, not percentages. Charge uses explicitly labeled Ah or C; Peukert alone uses hours. Inputs and outputs retain the research schema, defaults and complete examples.

The independent research examples below are numerical oracles, not observed runtime results. Main owns process, native-image and independent test acceptance. All estimates require the stated chemistry, calibration and geometry assumptions; none certifies hardware safety.

## 1. `batteries.series_weakest_cell` — basic

### Purpose and novelty

Find usable charge before the first cell reaches the chosen discharge floor in an unbalanced three-series string. Also report each cell's remaining SOC at that instant. This answers a pack-design/balancing question: why a nominally large cell can strand charge when another cell limits the string.

Nearest baseline: `dc.series_parallel_branches` solves resistive currents; it has no capacity, SOC or electrochemical cutoff. Nearest new neighbor: `batteries.coulomb_state_cycle` updates one cell through time; this model instead solves simultaneous series-cell feasibility by the minimum remaining charge. Neither is a voltage, energy or current rearrangement.

### Equation, convention and physical limits

With `Ci` in Ah, initial `si` and common floor `sfloor` as fractions:

`Qi = Ci(si-sfloor)`; `Qusable=min(Q1,Q2,Q3)`; `si,end=si-Qusable/Ci`.

Charge extraction is positive. Series cells pass the same Ah, not the same change in fractional SOC. TI SSZT724 page 1 states weakest-cell usability and differing cell SOC; the minimum and terminal states follow by enforcing all three nonnegative charge margins simultaneously. TI's prose about differing discharge rates is interpreted as differing fractional-SOC rates, not unequal series currents.

Require positive capacities, SOC in `[0,1]`, and every initial SOC at or above the floor. An initial cell exactly at its floor correctly gives zero deliverable charge. Capacities must represent the actual rate/temperature/cutoff conditions. Ignore balancing, self-discharge, voltage sag and current-dependent capacity. A real BMS can stop earlier from voltage or temperature; no safe discharge rating is implied.

### Complete independent example

Inputs: `capacity_one=3 Ah`, `capacity_two=2.8 Ah`, `capacity_three=3.2 Ah`, `soc_one=.9`, `soc_two=.85`, `soc_three=.95`, `soc_floor=.1`.

- Cell margins: `3(.9-.1)=2.4 Ah`, `2.8(.85-.1)=2.1 Ah`, `3.2(.95-.1)=2.72 Ah`.
- `usable_charge=min(2.4,2.1,2.72)=2.1 Ah`.
- `end_soc_one=.9-2.1/3=.2`.
- `end_soc_two=.85-2.1/2.8=.1`.
- `end_soc_three=.95-2.1/3.2=.29375`.

Cell 2 limits the pack even though it started above the floor. This decimal derivation is independent of the Rust evaluator.

### Independent tester cases

Nominal: assert all four values above. Limiting: with all capacities `3 Ah`, all SOC `.9` and floor `.1`, output `2.4 Ah` and three `.1` states; tied limiting cells must work. With `soc_two=.1` and other nominal inputs, usable charge is zero and all final SOCs equal initial SOCs. Permuting the complete cell tuples must preserve usable charge and permute state outputs. Invalid: capacity zero/negative, SOC outside `[0,1]`, or any initial SOC below floor. Do not clamp invalid inputs.

### Sources and illustration

- [Texas Instruments SSZT724, HEV/EV Battery Management Systems Explained Simply, pages 1-2](https://www.ti.com/lit/pdf/sszt724)

Three polarized series cells share one extracted-charge arrow. Symbolic initial SOC bars share a floor; separate callouts identify the three cutoff states. Bar heights are not computed.

## 2. `batteries.coulomb_state_cycle` — basic

### Purpose and novelty

Track discharge followed by recharge while separating terminal charge throughput from chemically retained Ah. This is useful for a battery gauge state budget. Nearest baseline is `transients.capacitor_constant_current`, which predicts an electrostatic voltage ramp and energy; this model predicts normalized chemical state and irreversibly lost recharge coulombs. `power.converter_input_budget` uses power efficiency and cannot replace coulombic efficiency.

### Equation, convention and physical limits

`sd=s0-Id td/(3600 C)`; `Qstored=eta Ic tc/3600`; `sf=sd+Qstored/C`.

`Id` and `Ic` are nonnegative magnitudes in A; times are s; full capacity is Ah. Discharge always comes first. Victron section 7.2.7 explicitly states that 10 Ah at 95% charge efficiency stores 9.5 Ah. The equations integrate this charge balance over constant-current intervals. The model uses a synchronized initial SOC and constant capacity/efficiency. It does not infer SOC from voltage or simulate CC–CV charging.

Require `C>0`, `0<=s0<=1`, times/currents nonnegative, and `0<eta<=1`. Require the discharge not to consume more than initial charge and the final state not to exceed one. Zero duration and zero current are valid. No self-discharge, Peukert effect, temperature dependence, sensor-error correction or aging. An otherwise valid SOC result does not mean the chosen charging current/voltage is safe.

### Complete independent example

Inputs: `full_capacity=5 Ah`, `initial_soc=.4`, `discharge_current=1 A`, `discharge_time=1800 s`, `charge_current=2 A`, `charge_time=3600 s`, `charge_efficiency=.95`.

- Discharge extracts `1×1800/3600=.5 Ah`; `discharged_soc=.4-.5/5=.3`.
- Charger passes `2×3600/3600=2 Ah`; `stored_charge=.95×2=1.9 Ah`.
- `final_soc=.3+1.9/5=.68`.

The lost `.1 Ah` is charge inefficiency, not a claim of `.1 Ah` of energy loss at a fixed voltage.

### Independent tester cases

Nominal: assert all three outputs. Zero both durations: discharged and final SOC `.4`, stored charge zero. Unity efficiency with nominal durations: `.3`, `2 Ah`, `.7`. Exact full-charge endpoint: `full_capacity=1`, `initial_soc=.5`, zero discharge current/time, `charge_current=.5`, `charge_time=3600`, efficiency one gives final one. Invalid: discharge nominal battery for `10000 s` at 1 A, or charge nominal case for `10000 s` at 2 A; reject intermediate exhaustion even if a later recharge would restore SOC. Also reject invalid capacity, efficiency and fraction ranges.

### Sources and illustration

- [Victron BMV-712 Smart manual, sections 7.2.1, 7.2.7, 7.2.8 and 7.2.10-13](https://www.victronenergy.com/media/pg/BMV-712_Smart/en/all-features-and-settings.html)

The connected state flow shows discharge before recharge, a separate unretained-charge branch and retained Ah. Arrows represent coulomb accounting, not converter power efficiency.

## 3. `batteries.peukert_runtime` — intermediate

### Purpose and novelty

Estimate rate-capacity loss under a constant-current discharge using a calibrated empirical exponent. This is not ideal `Ah/A` runtime except at the explicit ideal limit `n=1`. No baseline models battery rate dependence. The nearest new model, KiBaM, instead evolves two internal availability states and represents rest recovery; Peukert has no state history or recovery mechanism.

### Equation, convention and physical limits

Victron chapter 8 gives `Cp=I^n t`. Equating the invariant at the rated point `Ir=Cr/Hr` and requested current gives:

`t=Hr(Ir/I)^n`; `Cdelivered=It=Cr(Ir/I)^(n-1)`.

Use Ah, hours and amperes consistently. The direct delivered-capacity expression avoids multiplying a potentially tiny runtime by a large current. `n>=1` describes the intended loss-with-rate family. No arbitrary upper exponent bound is copied from Victron's instrument configuration range: a fitted exponent outside that range is not mathematically invalid, though its applicability still needs experimental evidence.

A fully charged battery, same temperature and same cutoff are required at both rating and requested load. Fit `n` from actual constant-current data and stay inside that calibration range. The model is unreliable for interrupted loads, very high currents and low-current extrapolation. In particular, infinite delivered capacity as current tends to zero for `n>1` is a model failure, not a physical prediction; zero current is excluded.

### Complete independent example

Inputs: `rated_capacity=100 Ah`, `rated_duration=20 h`, `load_current=20 A`, `peukert_exponent=1.25`.

- `rated_current=100/20=5 A`.
- Current ratio `5/20=1/4`.
- `runtime=20(1/4)^(5/4)=20/(4 sqrt(2))=3.5355339059327378 h`.
- `delivered_capacity=100(1/4)^(1/4)=100/sqrt(2)=70.71067811865476 Ah`.

These exact radical forms supply an independent oracle.

### Independent tester cases

Nominal: assert all outputs. At `load_current=5 A` with other nominal inputs, runtime is `20 h` and delivered capacity `100 Ah` for every allowed exponent. At `peukert_exponent=1`, nominal current gives `5 h`, `100 Ah`. Double load current: runtime scales by `2^-n` and delivered capacity by `2^(1-n)`. Invalid: zero current, zero rated duration/capacity, exponent below one and nonfinite values. Do not add a zero-current fake infinite lifetime. Low-current extrapolation is documented as physically unsupported, not silently clamped.

### Sources and illustration

- [Victron SmartShunt manual, chapter 8: Battery capacity and Peukert exponent](https://www.victronenergy.com/media/pg/SmartShunt/en/battery-capacity-and-peukert-exponent.html)
- [Jongerden and Haverkort, Battery Modeling, section 5.1 equation (1)](https://ris.utwente.nl/ws/portalfiles/portal/5101624/BatteryRep4.pdf)

The schematic log-log plot shows the fitted slope −n, a dashed ideal −1 reference, and symbolic rated/requested points. Positions are not computed results.

## 4. `batteries.kinetic_charge_recovery` — expert

### Purpose and novelty

Predict chemically available and temporarily bound charge after a current pulse or rest. This supplies state for duty-cycle battery studies without pretending to simulate full electrochemistry. The model is distinct from Peukert: stored charge remains conserved during rest while accessible charge recovers. It is distinct from `transients.capacitor_charge_sharing`: wells represent empirical reaction availability, do not have electrical voltages/capacitances, and the load has an available-charge exhaustion condition.

### Equation, convention and physical limits

Twente section 5.3 equation (4) gives:

`dq1/dt=-I+k(h2-h1)`; `dq2/dt=-k(h2-h1)`; `h1=q1/c`; `h2=q2/(1-c)`.

Define the input decay rate `lambda=k/[c(1-c)]`, not the original `k`. Then `dq1/dt=-I+lambda[c q2-(1-c)q1]`, and `dq2/dt=-lambda[c q2-(1-c)q1]`. Set `a=c q2,0-(1-c)q1,0`, `x=lambda t` and `u=1-exp(-x)`.

The propagated states are:

`q1f=q1,0+a u-I t[c+(1-c)exprel(-x)]`;

`q2f=q2,0-a u-I(1-c)t x expm1mx_ratio(-x)`;

`qtotal,f=q1,0+q2,0-I t`.

Here `exprel(z)=(exp(z)-1)/z`, analytic limit one, and `expm1mx_ratio(z)=(exp(z)-1-z)/z²`, analytic limit one-half. Identity `x expm1mx_ratio(-x)=1-exprel(-x)` avoids cancellation in the small-time bound-well discharge term. Use `-expm1(-x)` for `u`. All quantities are coulombs, amperes, seconds and inverse seconds.

Require positive combined capacity, `0<c<1`, `lambda>=0`, nonnegative pulse current/duration and initial states. Both wells must be within their allocated full capacities. Require `h1<=h2`, which includes equilibrium and states produced by discharge/rest. This is deliberately a discharge/rest model, not a charge-acceptance model. With these states and `I>=0`, the bound well never gains charge or exceeds its initial capacity. Available charge either decreases or initially rises then decreases; thus nonnegative endpoints suffice to reject a pulse that exhausts it during the interval. The catalog's final-availability inequality has input expressions only. Exhaustion is not concealed by clamping.

`lambda=0` is admitted as the disconnected-well limit, and `t=0` returns the initial state exactly. Parameters need measured fitting; the source notes the original model targets lead-acid batteries and other chemistries may need changed kinetics. Do not infer terminal voltage, thermal safety or charge current acceptance.

### Complete independent example

Inputs: `maximum_charge=10000 C`, `available_fraction=.4`, `available_initial=3000 C`, `bound_initial=5000 C`, `equalization_rate=.001 s^-1`, `pulse_current=2 A`, `pulse_duration=600 s`.

Well capacities are `4000 C` and `6000 C`; starting heights are `7500 C` and `8333.33333333333 C`, so the discharge-conditioned guard holds. Total starts at `8000 C`. Let `E=exp(-.6)=.5488116360940264`.

For an independent oracle, use the source's integrating-factor form rather than the catalog's conditioned expression:

`q1f=3000 E+[(8000×.001×.4-2)/.001](1-E)-[2×.4/.001](.6-1+E)`.

This becomes `1646.434908282079+541.4260366871683-119.0493088752212=2068.8116360940267 C`.

`q2f=5000 E+8000×.6(1-E)-[2×.6/.001](.6-1+E)`

`=2744.058180470132+2165.704146748673-178.5739633128318=4731.188363905973 C`.

`total_final=8000-2×600=6800 C`, equal to the sum of the two outputs. Independent Python arithmetic evaluated these analytical expressions; the Rust engine was not used.

### Independent tester cases

Nominal: assert all three values. With only `pulse_current=0`, expect `available_final=3090.2376727811948 C`, `bound_final=4909.762327218806 C`, total `8000 C`; accessible charge increases without new stored charge. With only `equalization_rate=0`, expect `1800 C`, `5000 C`, `6800 C`. Zero duration returns `3000`, `5000`, `8000 C`. Equilibrated wells with zero current remain unchanged for all durations/rates. Test tiny positive `lambda*t` without substituting `1-exp` or `1-exprel`; the stable expressions must preserve small transfers. Invalid: `c=0` or `1`, a well above capacity, reversed initial height order, negative load/rate/time, or nominal state with `pulse_current=10 A` for `600 s` (available well exhausts).

### Sources and illustration

- [Jongerden and Haverkort, Battery Modeling, section 5.3, Figure 6 and equation (4), pages 8-10](https://ris.utwente.nl/ws/portalfiles/portal/5101624/BatteryRep4.pdf)

Two chemical wells exchange charge through a passage. Only the available well feeds the load. The equalization rate is λ, not the original well conductance k; levels are symbolic.

## 5. `batteries.intercalation_regular_solution` — expert

### Purpose and novelty

Estimate an insertion electrode's equilibrium potential and local differential charge capacity, including pair interactions among occupied and vacant sites. The closest baseline is `diodes.shockley_forward_current`, an exponential carrier-transport law; this is a lattice free-energy equilibrium law. The closest new sensor is hydrogen-activity Nernst transduction, but the battery model adds occupancy/vacancy entropy, pair interaction energy, global phase stability and a charge-versus-voltage derivative. The sensor researcher confirmed this boundary.

### Equation, convention and physical limits

MIT notes pages 6–7 give a regular-solution free energy per site and its derivative. Expressing interaction energy per mole rather than per particle:

`g=RT[x ln x+(1-x)ln(1-x)]+Omega x(1-x)+mu0 x`.

For monovalent insertion relative to a fixed reference:

`V=Vmid-[RT ln(x/(1-x))+Omega(1-2x)]/F`.

Differentiate directly:

`dV/dx=-(RT/[x(1-x)]-2Omega)/F`;

`Cchem=-dQ/dV=Qmax F x(1-x)/[RT-2Omega x(1-x)]`, where inserted charge `Q=Qmax x`.

`Vmid` includes the chemical reference offset and equals voltage at half filling. `Omega` is J/mol and may be negative. `T` is K. `x` is lithium occupancy, which can increase on discharge at a positive insertion electrode; do not equate it with pack SOC without a chemistry-specific mapping. Slope unit V means volts per dimensionless filling fraction. Chemical capacitance is in C/V=F and is positive by the specified minus sign.

Guard `0<x<1`, `T>0`, `Qmax>0`, and **globally** `Omega<2RT`. Since `x(1-x)<=1/4`, this guarantees positive curvature everywhere and excludes the critical point and all two-phase miscibility. Merely checking local curvature would admit metastable branches inside a miscibility gap; that weaker guard is intentionally not used. At stronger attraction, common-tangent phase equilibrium is required, not an unphysical rising voltage segment.

The fixed-reference, homogeneous, quasi-equilibrium model ignores kinetic/ohmic overpotential, stress, electrolyte-activity changes, hysteresis and double-layer charge. It is not a calibrated universal lithium-ion OCV curve, and especially not the real two-phase LiFePO4 plateau. Use `ln(x)-log1p(-x)` to avoid constructing an unnecessarily extreme occupancy ratio. Endpoint singularities are physically outside the finite-occupancy model and are rejected.

### Complete independent example

Inputs: `midpoint_voltage=3.5 V`, `temperature=298.15 K`, `interaction_energy=3000 J/mol`, `filling_fraction=.3`, `full_insertion_charge=10800 C`.

`RT=2478.957029602388 J/mol`; `2RT=4957.914059204776 J/mol`, so 3000 is globally single-phase. `x(1-x)=.21`, `ln(.3/.7)=-.8472978603872037`, interaction term `3000(1-.6)=1200 J/mol`.

- `equilibrium_voltage=3.5-[2478.957029602388×(-.8472978603872037)+1200]/96485.33212=3.5093321437299303 V`.
- `voltage_slope=-[2478.957029602388/.21-6000]/96485.33212=-.06015999692680433 V`.
- `chemical_capacitance=-10800/(-.06015999692680433)=179521.28576635703 F`.

The independent derivative, not an ideal-solution capacitance expression, supplies the oracle.

### Independent tester cases

Nominal: assert all outputs. At `x=.5`, voltage equals `midpoint_voltage` exactly; slope is `-(4RT-2Omega)/F` and capacitance `Qmax F/(4RT-2Omega)`. At `Omega=0`, recover the ideal lattice entropy model. For complementary fillings `.3` and `.7`, voltage deviations from midpoint reverse sign; slope and capacitance match. Negative Omega is valid and lowers differential capacitance. As Omega approaches `2RT` from below at half filling, capacitance diverges; reject equality and above rather than clamping. Reject zero/one occupancy, nonpositive temperature/charge. Positive finite physical values close to endpoints remain mathematically admissible; nonfinite arithmetic is an engine error, not grounds for an invented cutoff.

### Sources and illustration

- [MIT 10.626 Lithium ion batteries, Regular solution model and Faradaic capacitance, pages 6-8](https://ocw.mit.edu/courses/10-626-electrochemical-energy-systems-spring-2014/e87e16b891cb649be6032e375c30e322_MIT10_626S14_Lec13_Lithi.pdf)

Occupied and vacant sites illustrate insertion with separate ion and electron paths, a pair-interaction bracket, and a schematic single-phase voltage curve and tangent. Occupancy is not pack SOC.

## 6. `batteries.sand_depletion_limit` — expert

### Purpose and novelty

Estimate an electrolyte transport limit during constant-current lithium plating: time for initially uniform electrolyte concentration at a planar consuming interface to reach zero. No baseline model describes electrochemical mass transport. It differs from KiBaM, which fits lumped accessible/bound charge; Sand time uses diffusivity, concentration and transference to predict local depletion. It is not fuse I²t or a protection trip rating.

### Equation, convention and physical limits

Cambridge paper equation (31) states, for cation charge number one:

`ts=(pi D/4)[F c0/(j(1-tplus))]^2`.

`qs=j ts` and `ell=sqrt(D ts)` are supporting areal-charge and diffusion-length outputs of the same model, not separate calculations. `D` is effective salt/ambipolar diffusivity in m²/s, `c0` is mol/m³, positive `j` is plating-current magnitude in A/m², and `tplus` is the cation transference fraction. `qs` is C/m², not cell Ah. The surface consumes cations; the ionic concentration gradient supplies them from bulk electrolyte. The squared combination has units s.

Require `D,c0,j>0` and `0<=tplus<1`. The derivation presumes one-dimensional planar semi-infinite transport, constant coefficients, initially uniform concentration and no convection. The physical counter boundary must be far beyond `ell`. The catalog intentionally models an ideal semi-infinite domain; it does not pretend to infer finite-cell applicability from an unspecified separator. A supplied device geometry must be checked independently against the reported length. As an example, a 5 mm transport region is much longer than the nominal 0.143 mm diffusion length, unlike a 20 µm separator.

The same source's equation (32) gives finite-cell `jlim=2 F c0 D/[(1-tplus)L]` and explains that below it a finite cell need not deplete. That is a physical caveat, not another claimed calculator or hidden output. Never apply finite Sand time blindly below a finite cell's limiting current. At `tplus=1` this expression is singular; no finite depletion prediction exists in this model and the endpoint is rejected. Concentrated electrolytes can require different thermodynamics and transport definitions; do not substitute a tracer diffusivity or ideal-ion conductivity conversion without justification.

No safe-charge certification is possible from Sand time. SEI, dendrites, roughness, field concentration, nucleation and thermal behavior can cause failure before or after the ideal depletion threshold. The actual cited paper explicitly warns that the lithium interface is more complex because of SEI inhomogeneity.

### Complete independent example

Inputs: `salt_diffusivity=1e-10 m²/s`, `salt_concentration=1000 mol/m³`, `plating_current_density=100 A/m²`, `cation_transference=.4`.

- Effective consumption denominator `j(1-tplus)=100×.6=60 A/m²`.
- `F c0/[j(1-tplus)]=96485.33212×1000/60=1608088.8686666666 s/m`.
- `sand_time=(pi×1e-10/4)×1608088.8686666666²=203.10002310425608 s`.
- `sand_areal_charge=100×203.10002310425608=20310.002310425607 C/m²`.
- `diffusion_length=sqrt(1e-10×203.10002310425608)=.0001425131653933264 m`.

This is an ideal long-domain plating example, not a recommended charge protocol. The nominal areal charge corresponds to approximately `.5642 mAh/cm²`, but that conversion is explanatory and not an extra catalog output.

### Independent tester cases

Nominal: assert all three outputs. Double current to `200 A/m²`: time divides by four, areal charge halves, diffusion length halves. Double concentration: time quadruples, areal charge quadruples, diffusion length doubles. Set transference to zero: time and areal charge are `.36` of nominal, diffusion length `.6` of nominal. As transference approaches one, predicted time and length grow without bound; finite geometry generally invalidates the physical approximation before the mathematical endpoint. Reject transference one, above one or below zero; reject zero/negative diffusivity, concentration or current. Do not produce a finite result for zero current or silently assign a special depletion time.

### Sources and illustration

- [Ellison and Grey, Engineering considerations for practical lithium-air electrolytes, Ionic transport, equations (31)-(32), PDF page 6 and discussion pages 18-19](https://api.repository.cam.ac.uk/server/api/core/bitstreams/a34f3449-0c37-4b97-a66f-3f24697e9426/content)

Separate electron and cation paths terminate at a planar plating interface. Initial and depleted concentration profiles are schematic; the diffusion-length dimension is a scale, not a sharp front.

