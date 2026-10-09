# Additional power converter calculators

Rust evaluates [the converter catalog](../../backend/catalog/converters.json); Flutter renders inputs, results and static illustrative circuits. Inputs use SI units; all ratios and duty fractions are dimensionless. Each model retains its researched validation domain, stable expressions and complete example. No device-rating, isolation, ZVS or hardware-performance certification is implied.

## Source provenance

The [research handoff](../../research/expansion/converters.md) records the sources actually read, access gaps and independent derivations. Implementation does not claim a fresh source audit or runtime verification.

- **S1 — TI LM2662/LM2663, SNVS002E:** [datasheet](https://www.ti.com/lit/ds/symlink/lm2662.pdf), sections 8.1, 8.3 and 9.2.1.2, printed pages 10–13. Describes the two switch phases, frequency convention, output-resistance approximation, fourfold flying-capacitor ESR contribution and single output-capacitor ESR contribution. Manufacturer primary design guidance; approximate resistance, not guaranteed performance for arbitrary parts.
- **S2 — TI Anthony Fagnani, SLVA516A:** [Using the TPS55340 as a SEPIC Converter](https://www.ti.com/lit/an/slva516a/slva516a.pdf), section 1/Figures 1–3 and sections 2.2, 2.4, 2.6, 2.8–2.9. Read state currents, duty relation, coupled-versus-uncoupled distinction, capacitor stress and blocking stress. Primary application report. Its subsequent numerical design uses coupled inductors; this handoff deliberately derives separate-inductor ripple from the preceding state voltages instead of copying the coupled half-inductance equation.
- **S3 — TI/National David Baba, SNVA603:** [Isolated Supply Overview and Design Trade-Offs](https://www.ti.com/lit/an/snva603/snva603.pdf), printed forward-converter page/Figures 3–4 (PDF page 5). Read forward reset sequence, equal-turns 50% limit and general switch blocking relation. Primary manufacturer topology note; the arbitrary reset-ratio timing relation below is explicitly derived by volt-second balance.
- **S4 — TI John Betten, SSZTCW6:** [Designing a DCM flyback converter](https://www.ti.com/lit/ta/ssztcw6/ssztcw6.pdf), February 2024, Figures 1–2, equations 4–10 and 23–27. Read primary/secondary/idle intervals, energy/current relations and flat-top stresses. Primary manufacturer technical article. This handoff takes its ideal, unity-efficiency/zero-drop limit consistently.
- **S5 — onsemi AND90061/D, Rev. 3, January 2023:** [Half-Bridge LLC Resonant Converter Design Using NCP4390/NCV4390](https://www.onsemi.com/download/application-notes/pdf/and90061-d.pdf), pages 2–5, equations 3–7 and Figures 2–7. Read full-wave equivalent load, half-bridge gain, series/shunt equivalent circuit and the distinct integrated-transformer leakage model. Primary manufacturer application note.
- **S6 — TI J. M. Leisten, SLUA733:** [LLC Design for UCC29950](https://www.ti.com/lit/an/slua733/slua733.pdf), abstract and section 1. Explicitly states FHA loses accuracy away from resonance and restricts the rectifier model to diodes or diode-equivalent synchronous operation. Primary manufacturer analysis; used as an accuracy limit, not as evidence that FHA is exact.
- **S7 — TI TIDUES0F, revised April 2026:** [Bidirectional, Dual Active Bridge Reference Design](https://www.ti.com/lit/ug/tidues0f/tidues0f.pdf), sections 2.3.4.1–2.3.4.4, especially equation 6 and soft-switching discussion. Read signed power law, turns-ratio and inductance conventions, maximum at 90 degrees and load/voltage-dependent ZVS. Primary manufacturer reference-design theory; the detailed board's ratings are not generic calculator limits.

Access gaps: ADI MAX1852/MAX1853 and ADI charge-pump article reads timed out; their search snippets were not used as final evidence. TI SLUP263 redirected to login; a regional URL returned 404. ST AN2450/CD00143244 reads timed out. Accessible S1, S5 and S6 replace those sources. TI's flyback HTML endpoint was wrongly returned as binary PDF by the reader; the direct PDF was then successfully extracted and read. PDF extraction loses some equation formatting; only text-supported relations or independently derived relations from the explicitly read equivalent circuits are used here. No independent hardware validation was sought or claimed.


## 1. `converters.inverting_charge_pump`

### Purpose, equations and physical domain

A single flying capacitor alternates between charging from the positive supply and transferring charge into a negative reservoir. One complete two-phase cycle occurs at `switching_frequency = fs` Hz. For LM2662 specifically, `fosc = 2 fs`; do not copy the oscillator value into the cycle-frequency field.

S1 models output as an ideal `-Vin` source with a resistance. Its capacitor transport term `2/(fosc Cfly)` becomes `1/(fs Cfly)`. The switch contribution is entered as an **effective total resistance**, not multiplied by a guessed count of switches. From S1's current-weighted ESR description:

- `Rtransport = 1/(fs Cfly)` ohm.
- `Rout ≈ Rtransport + Rswitch + 4 ESRfly + ESRout` ohm.
- `droop = Iload Rout` V; `Vout = -Vin + droop` V.

`Iload >= 0` is a magnitude: conventional load current enters the negative output from ground. Positive droop means the negative rail becomes less negative. Require `Iload Rout < Vin`. With zero load the output is exactly `-Vin` in this first-order model.

The approximation requires steady periodic operation and a reservoir large enough for small ripple. Flying capacitance must include voltage-bias/temperature derating. On-resistance, temperature, finite settling, controller current limit and oscillator losses matter in real designs. High frequency does not eliminate the switch/ESR floor. Neither waveform ripple nor efficiency is claimed. This generic calculation does not confer any specific IC voltage/current rating.

### Complete independent example

Inputs: `vin=5 V`, `load_current=0.02 A`, `switching_frequency=10000 Hz`, `flying_capacitance=47e-6 F`, `switch_resistance=1 ohm`, `flying_esr=0.05 ohm`, `output_esr=0.05 ohm`.

1. `transport_resistance = 1/(10000 × 47e-6) = 2.1276595744680855 ohm`.
2. `output_resistance = 2.1276595744680855 + 1 + 4×0.05 + 0.05 = 3.3776595744680855 ohm`.
3. `voltage_droop = 0.02 × 3.3776595744680855 = 0.06755319148936172 V`.
4. `output_voltage = -5 + 0.06755319148936172 = -4.9324468085106385 V`.

### Nominal, limiting and invalid cases

- Nominal: above complete map; preserve the negative output sign.
- Zero load: keep every other nominal input, set `load_current=0`; both resistances stay unchanged, droop is zero and output is `-5 V`.
- Frequency scaling: doubling fs halves **only** the transport contribution, not the switch or ESR floor.
- Ideal transport-only vector: `vin=5`, `fs=10000`, `Cfly=100e-6`, all three resistance inputs zero, `Iload=1` gives `Rtransport=Rout=1 ohm`, droop `1 V`, output `-4 V`.
- Collapse boundary: same transport-only vector with `Iload=5` must reject equality. Larger load must reject, not return a positive output.
- Reject zero/negative frequency or flying capacitance, nonpositive Vin, negative load or any negative resistance. Reject nonfinite values through the shared numerical boundary.

### Mandatory diagram specification

Draw two side-by-side switch-state panels, not an ordinary fixed capacitor divider. Reserve x=20–320 for phase A and x=360–680 for phase B. Each panel shows the same flying capacitor with its upper plate marked `+`.

- Phase A: upper plate connects through closed S1 to `+Vin`; lower plate connects through closed S3 to ground. S2 and S4 are shown open, not as an accidental short.
- Phase B: upper plate connects through closed S2 to ground; lower plate connects through closed S4 to the negative output. S1 and S3 are open. Reservoir Cout sits between ground (positive plate) and Vout (negative plate); load is in parallel with Cout. An arrow points through the load from ground into the negative-output node.
- Bind `Vin` → `vin`; `Cf` → `flying_capacitance`; `fs` → `switching_frequency`; `Iload` → `load_current`; `Vout` → `output_voltage`. Add a compact resistance annotation `Rout` → `output_resistance`, outside the switch network. State `Rout is equivalent, not an extra physical resistor`.
- Caption: “A charges the flying capacitor; B transfers charge to the negative reservoir. fs counts complete A+B cycles. Static switch states; output ripple and switching transients are not plotted.”


## 2. `converters.sepic_ccm_stress`

### Purpose, equations and physical domain

S2 Figures 1–3 show a SEPIC with input inductor L1, ground-to-diode-node inductor L2, series transfer capacitor Cp, low-side switch and output diode. During switch-on, both inductors see `+Vin` in their defined current directions. During off-time both see `-Vout`. Volt-second balance gives `D=Vout/(Vin+Vout)`. Capacitor charge balance and lossless power balance give `I1=Iin=Vout Iout/Vin`, `I2=Iout`.

For **uncoupled** inductors, `deltaI1=Vin D/(fs L1)` and `deltaI2=Vin D/(fs L2)`. Do not include the factor two from S2's later coupled-winding sizing equation. Switch current is the sum of both synchronous triangular currents during D and zero otherwise:

- `IQpk = I1+Iout+(deltaI1+deltaI2)/2`.
- `IQrms = sqrt(D) hypot(I1+Iout, (deltaI1+deltaI2)/sqrt(12))`.
- `VCp,DC=Vin`; ideal switch and reverse diode stress is `Vin+Vout`.

Define capacitor current left-to-right. It is `-iL2` during D and `+iL1` during `1-D`. Integrating each affine-current segment over its actual duration gives:

`ICp,rms² = D [Iout²+deltaI2²/12] + (1-D)[I1²+deltaI1²/12]`.

This ripple-inclusive extension is independently derived from S2's switching states. S2 equation 22 is its negligible-ripple limit `I1 sqrt((1-D)/D)`. It is not falsely attributed as S2's printed exact equation.

The calculator explicitly requires `I1 > deltaI1/2` and `Iout > deltaI2/2`. This branch-positive subset is sufficient for CCM; a SEPIC may retain diode-current CCM even when an individual branch reverses. Such operating points are outside this deliberately stated model, not incorrectly called universally DCM. Cp and Cout voltages must have negligible ripple for these slopes. Coupled inductors, losses and diode recovery are excluded.

### Complete independent example

Inputs: `vin=12 V`, `vout=18 V`, `load_current=1 A`, `switching_frequency=200000 Hz`, `input_inductance=100e-6 H`, `second_inductance=150e-6 H`.

1. `duty = 18/(12+18) = 0.6`.
2. `input_current = 18×1/12 = 1.5 A`.
3. `input_ripple = 12×0.6/(200000×100e-6) = 0.36 A` peak-to-peak.
4. `second_ripple = 12×0.6/(200000×150e-6) = 0.24 A` peak-to-peak.
5. `switch_peak_current = 1.5+1+(0.36+0.24)/2 = 2.8 A`.
6. `switch_rms_current = sqrt[0.6×(2.5²+0.6²/12)] = sqrt(3.768) = 1.9411336893681486 A`.
7. `coupling_cap_rms_current = sqrt[0.6×(1²+0.24²/12)+0.4×(1.5²+0.36²/12)] = sqrt(1.5072) = 1.2276807402578245 A`.
8. `blocking_voltage = 12+18 = 30 V`.

The valleys are `1.32 A` and `0.88 A`; both satisfy the admitted branch-current domain. Average Cp current is `-0.6×1 + 0.4×1.5 = 0`.

### Nominal, limiting and invalid cases

- Nominal: above complete map; both boost-like and buck-like conversion ratios are allowed without swapping formulas.
- Unity conversion: `vin=vout=12`, `Iout=1`, `fs=200000`, `L1=L2=100e-6` gives D=.5, Iin=1 A, both ripples=.3 A, peak=2.3 A, switch RMS `1.4195069566578393 A`, Cp RMS `1.0037429949942367 A`, blocking=24 V.
- Increasing both inductances reduces ripple; asymptotic switch RMS is `(Iin+Iout)sqrt(D)` and Cp RMS is `sqrt(Iin Iout)`.
- With nominal voltages/frequency/load, `L1=12e-6 H` makes the L1 valley exactly zero and must reject; `L2=18e-6 H` likewise rejects. Test just above and below each physical boundary with sensible decimal separation.
- Reject zero or negative voltage, load, frequency or either inductance. Do not impose `Vout>Vin`: SEPIC supports both sides.

### Mandatory diagram specification

Draw the actual nonisolated SEPIC, approximately x=30–660 and y=55–210:

- Vin positive node → L1 → node A. Q connects A to ground. Cp connects A to node B, with Cp left plate positive in the DC sense.
- L2 connects ground to B; its current arrow points upward. Do not connect L2 to the positive output node.
- Diode anode at B, cathode at output O. Cout and load connect O to common ground. Input source returns to that ground.
- L1 current points left-to-right. Q current points downward. Label near Q: `iQ=iL1+iL2 (on)`, unbound. Label Cp current direction left-to-right, with `on: -iL2; off: +iL1`, unbound.
- Bind `Vi` → `vin`, `Vo` → `vout`, `L1` → `input_inductance`, `L2` → `second_inductance`, `IQ,pk` → `switch_peak_current`, `ICp,rms` → `coupling_cap_rms_current`, `D` → `duty`. Keep the RMS label beside Cp, not across its wire.
- Caption: “Separate, uncoupled inductors. The switch carries their sum; Cp alternates between their currents. Ideal CCM with nearly constant capacitor voltages; static explanatory circuit.”


## 3. `converters.forward_reset_winding`

### Purpose, equations and physical domain

S3's dedicated reset winding returns magnetizing energy to the input after switch turn-off. Define `n=Np/Ns` and `r=Nr/Np`, both dimensionless positive ratios. The output choke is assumed CCM. Then `D=n Vout/Vin`.

While Q is on, primary voltage is `+Vin`; while reset diode conducts, primary voltage is `-Vin/r`. Magnetizing volt-second balance gives `Vin D T = (Vin/r) Dreset T`, so `Dreset=r D`. Both intervals must fit:

- `D < Dlimit=1/(1+r)`.
- `idle=1-D-rD=(Dlimit-D)(1+r)`; the latter is the catalog expression to preserve small positive margin near the boundary.
- `treset=rD/fs` seconds.
- `VDS,flat=Vin+Vin/r` volts.

This intentionally calculates reset feasibility, not output inductance, flux density, winding construction or saturation. The gain equation assumes an output choke already in CCM. A real controller needs extra margin for tolerances, minimum off-time and transients. Equal reset/primary turns give the familiar D<0.5 and flat-top stress 2Vin; changing reset turns trades duty margin against switch stress.

### Complete independent example

Inputs: `vin=48 V`, `vout=12 V`, `primary_secondary_ratio=1.5`, `reset_primary_ratio=1`, `switching_frequency=100000 Hz`.

1. `duty = 1.5×12/48 = 0.375`.
2. `reset_fraction = 1×0.375 = 0.375`.
3. `duty_limit = 1/(1+1) = 0.5`.
4. `idle_fraction = 1-0.375-0.375 = 0.25`.
5. `reset_time = 0.375/100000 = 3.75e-6 s`.
6. `switch_blocking_voltage = 48×(1+1/1) = 96 V`.

### Nominal, limiting and invalid cases

- Nominal: above map; reset completes 2.5 microseconds before the next on-time.
- Shorter reset winding: keep defaults except `reset_primary_ratio=.5`. Expected D=.375, reset fraction=.1875, limit=2/3, idle=.4375, reset time=1.875e-6 s, stress=144 V.
- Boundary: default ratios with `vout=16 V` give D=.5 and zero idle; reject equality. `vout=15.999 V` is admitted with a small positive margin; `vout=16.001 V` rejects.
- Doubling frequency halves reset time but does not change duty fractions or ideal voltage stress.
- Reject nonpositive bus/output voltage, ratio or frequency. Do not invent an integer-ratio restriction: ratios of integer turn counts are generally fractional.

### Mandatory diagram specification

Draw a three-winding, single-switch forward circuit, not a flyback. The reset connection is the teaching element.

- Primary winding top dotted terminal connects to +Vin; bottom undotted terminal connects to Q drain; Q source connects primary ground.
- Reset winding lower **dotted** terminal connects to primary ground. Upper **undotted** terminal connects to reset-diode anode; diode cathode connects to +Vin. This polarity reverse-biases the reset diode during Q on-time and returns energy to the input during reset.
- Secondary top dotted terminal connects to rectifier D1 anode. D1 cathode connects node S, then output choke Lout, then output O. Secondary bottom is isolated secondary ground. Freewheel diode D2 has anode at secondary ground and cathode at S. Cout/load return O to secondary ground. Never join secondary ground to primary ground.
- Allocate primary/reset circuit x=25–310 and secondary circuit x=345–670. Put turns-ratio annotations over the transformer rather than on wires. A short lower-band three-segment timing bar may mark `D`, `Dreset`, and `idle`; proportions are illustrative only.
- Bind `Vin` → `vin`, `Vo` → `vout`, `Np/Ns` → `primary_secondary_ratio`, `Nr/Np` → `reset_primary_ratio`, `D` → `duty`, `Dreset` → `reset_fraction`, `idle` → `idle_fraction`, `VDS` → `switch_blocking_voltage`. If timing labels crowd, retain ratios and reset diode in the main circuit and put computed fractions in the legend.
- Caption: “Reset winding returns magnetizing energy to the input while the freewheel diode feeds the CCM output choke. Reset must finish before the next on-time. Ideal stress excludes leakage spikes.”

