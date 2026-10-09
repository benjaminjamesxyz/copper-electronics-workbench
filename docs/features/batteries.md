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

