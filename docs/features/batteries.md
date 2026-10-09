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

