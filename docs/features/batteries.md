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

