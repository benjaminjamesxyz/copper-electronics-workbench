# Additional converter calculations: research and implementation handoff

Exactly six new models: one basic, two intermediate, three expert. The implementation-ready numerical objects are in `research/expansion/converters.json`. These are topology calculations, not circuit simulators or component-rating certificates. All example numbers below were independently derived with ordinary arithmetic/Python math, outside the Rust engine. No application execution, build, test, lint, formatter or production-file change was performed.

## Scope, novelty and source audit

Read the expansion contract, numerical contract, allocation board, actual baseline power/magnetics/capacitor models, and expansion communication bus. Preserve all 128 old calculations. Small-signal controllers, compensation, loop poles/zeros and stability remain Control's scope; Control explicitly confirmed no collision. Core geometry/loss remains Cores' scope. No buck, boost, LDO, generic efficiency, transformer turns-ratio-only calculation, inverse mode or extra-output-only feature is counted here.

| New ID | Level | Closest old/neighbor model | Distinct physical problem |
|---|---|---|---|
| `converters.inverting_charge_pump` | basic | `transients.capacitor_charge_sharing`, `dc.resistive_source_matching` | Repeated two-phase flying-capacitor charge transport produces an inverted DC rail and a frequency-dependent equivalent resistance. Neither an isolated charge-sharing transient nor an entered source resistance solves this. |
| `converters.sepic_ccm_stress` | intermediate | `power.boost_inductor_design`, `power.buck_input_capacitor_rms` | Two independent inductors and a series energy-transfer capacitor create summed switch current and alternating capacitor branch current. This is not an ordinary boost inductor-sizing equation. |
| `converters.forward_reset_winding` | intermediate | `power.buck_inductor_design`, `magnetics.turns_ratio` | A third winding must remove magnetizing volt-seconds during off-time. Its ratio jointly constrains available duty and switch stress. Output inductor sizing and simple transformer ratio are not the deliverable. |
| `converters.flyback_dcm_demagnetization` | expert | `magnetics.field_energy_density`, `power.boost_inductor_design` | A primary energy-storage interval, isolated secondary demagnetization interval and genuine zero-current interval must all fit in one period. This delivers topology-specific mode feasibility and winding stress, not static field energy. |
| `converters.llc_half_bridge_fha` | expert | `ac.series_rlc_impedance`, `magnetics.turns_ratio`; neighboring Control models | Half-bridge excitation, nonlinear full-wave rectification, transformer reflection and a shunt magnetizing branch determine loaded DC conversion. A plain passive RLC or loop compensator does not have this model. |
| `converters.dual_active_bridge_power` | expert | `power.converter_input_budget`, `magnetics.turns_ratio`; neighboring LLC | Two active square-wave bridges exchange signed power through transfer inductance under phase displacement. The result is not an entered-efficiency budget or a resonant-tank gain model. |

Search boundary: English-language publicly accessible manufacturer technical material, searched 2026-10-09 without a publication-date cutoff. Searches covered TI SEPIC/forward/flyback/DAB/LLC, ADI charge-pump output resistance, ST LLC FHA, and onsemi LLC FHA. Included actual topology equations, switching-state descriptions and operating limitations. Excluded search-answer arithmetic as evidence, controller-specific compensation, ordinary buck/boost alternatives, unsourced schematic blogs and commercial calculator output. This is a focused engineering literature review, not an exhaustive academic systematic review.

Actually read primary manufacturer evidence:

- **S1 — TI LM2662/LM2663, SNVS002E:** [datasheet](https://www.ti.com/lit/ds/symlink/lm2662.pdf), sections 8.1, 8.3 and 9.2.1.2, printed pages 10–13. Describes the two switch phases, frequency convention, output-resistance approximation, fourfold flying-capacitor ESR contribution and single output-capacitor ESR contribution. Manufacturer primary design guidance; approximate resistance, not guaranteed performance for arbitrary parts.
- **S2 — TI Anthony Fagnani, SLVA516A:** [Using the TPS55340 as a SEPIC Converter](https://www.ti.com/lit/an/slva516a/slva516a.pdf), section 1/Figures 1–3 and sections 2.2, 2.4, 2.6, 2.8–2.9. Read state currents, duty relation, coupled-versus-uncoupled distinction, capacitor stress and blocking stress. Primary application report. Its subsequent numerical design uses coupled inductors; this handoff deliberately derives separate-inductor ripple from the preceding state voltages instead of copying the coupled half-inductance equation.
- **S3 — TI/National David Baba, SNVA603:** [Isolated Supply Overview and Design Trade-Offs](https://www.ti.com/lit/an/snva603/snva603.pdf), printed forward-converter page/Figures 3–4 (PDF page 5). Read forward reset sequence, equal-turns 50% limit and general switch blocking relation. Primary manufacturer topology note; the arbitrary reset-ratio timing relation below is explicitly derived by volt-second balance.
- **S4 — TI John Betten, SSZTCW6:** [Designing a DCM flyback converter](https://www.ti.com/lit/ta/ssztcw6/ssztcw6.pdf), February 2024, Figures 1–2, equations 4–10 and 23–27. Read primary/secondary/idle intervals, energy/current relations and flat-top stresses. Primary manufacturer technical article. This handoff takes its ideal, unity-efficiency/zero-drop limit consistently.
- **S5 — onsemi AND90061/D, Rev. 3, January 2023:** [Half-Bridge LLC Resonant Converter Design Using NCP4390/NCV4390](https://www.onsemi.com/download/application-notes/pdf/and90061-d.pdf), pages 2–5, equations 3–7 and Figures 2–7. Read full-wave equivalent load, half-bridge gain, series/shunt equivalent circuit and the distinct integrated-transformer leakage model. Primary manufacturer application note.
- **S6 — TI J. M. Leisten, SLUA733:** [LLC Design for UCC29950](https://www.ti.com/lit/an/slua733/slua733.pdf), abstract and section 1. Explicitly states FHA loses accuracy away from resonance and restricts the rectifier model to diodes or diode-equivalent synchronous operation. Primary manufacturer analysis; used as an accuracy limit, not as evidence that FHA is exact.
- **S7 — TI TIDUES0F, revised April 2026:** [Bidirectional, Dual Active Bridge Reference Design](https://www.ti.com/lit/ug/tidues0f/tidues0f.pdf), sections 2.3.4.1–2.3.4.4, especially equation 6 and soft-switching discussion. Read signed power law, turns-ratio and inductance conventions, maximum at 90 degrees and load/voltage-dependent ZVS. Primary manufacturer reference-design theory; the detailed board's ratings are not generic calculator limits.

Access gaps: ADI MAX1852/MAX1853 and ADI charge-pump article reads timed out; their search snippets were not used as final evidence. TI SLUP263 redirected to login; a regional URL returned 404. ST AN2450/CD00143244 reads timed out. Accessible S1, S5 and S6 replace those sources. TI's flyback HTML endpoint was wrongly returned as binary PDF by the reader; the direct PDF was then successfully extracted and read. PDF extraction loses some equation formatting; only text-supported relations or independently derived relations from the explicitly read equivalent circuits are used here. No independent hardware validation was sought or claimed.

## Shared implementation and diagram rules

- Use the existing JSON formula language only. Constraints reference inputs only. Every physical scale/divisor is strictly positive except admitted zero load/loss and signed DAB phase. No arbitrary numeric caps are used to hide unsupported physics.
- `hypot` preserves RMS sums better than squaring raw large/small currents. Use the expression order in the JSON; do not silently replace it with less stable expanded squares. Do not round intermediate duty, turns ratio or phase.
- Each item gets its own entry in `convertersDiagrams`, using only `frontend/lib/diagrams/primitives.dart`. Use a 700×280 logical canvas, 16 px margins and labels at least 14 px. Preserve pin placement, break wires at components, and mark actual junctions. Do not connect primary and isolated-secondary grounds.
- Quantity bindings listed below are exact JSON keys. Text such as `Np:Ns`, `50% bridge`, `ground` and `illustrative waveform` is unbound explanatory text unless explicitly mapped. Derived labels must show their relation to the bound quantity.
- Diagrams are explanatory static circuits/state sketches. No static curve is presented as a computed waveform. Prefer short symbols on the schematic with fuller accessible legend labels. Do not squeeze a full formula onto a component body.
- Parent runs verification after promotion. Independent testers should assert all example outputs, the limiting/invalid cases below and valid recovery after each rejection. Research includes no run/pass claim.

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

## 4. `converters.flyback_dcm_demagnetization`

### Purpose, equations and physical domain

S4 distinguishes three intervals: primary ramp from zero, secondary ramp to zero, and idle. Let `Lm` be primary magnetizing inductance, `fs` fixed switching frequency and `n=Np/Ns`.

With ideal unity efficiency, output power is `Po=Vo Io = Lm Ipk² fs/2`. Thus `Ipk=sqrt(2VoIo/(Lm fs))`. Primary current slope is `Vin/Lm`, giving `D=Lm fs Ipk/Vin`. During energy delivery, primary-referred reset voltage is `nVo`, giving `Ddemag=Lm fs Ipk/(nVo)`. DCM requires `D+Ddemag<1`; idle is the remainder.

Integrating triangular pulses over a **full** period gives `Ipri,rms=Ipk sqrt(D/3)` and `Isec,rms=n Ipk sqrt(Ddemag/3)`. The secondary pulse peak is `nIpk`. Its average `n Ipk Ddemag/2` must equal `Io`. Ideal switch flat-top stress is `Vin+nVo`; diode reverse stress is `Vo+Vin/n`.

Do not insert an efficiency factor only into energy while retaining ideal output-current balance. This handoff uses the internally consistent ideal limit. Losses need a separate detailed design, not hidden corrections. Core saturation, leakage energy, valley switching, clamps, diode drop and transformer/IC ratings are not predicted. The output capacitor must hold voltage nearly constant.

### Complete independent example

Inputs: `vin=48 V`, `vout=12 V`, `load_current=1 A`, `magnetizing_inductance=100e-6 H`, `switching_frequency=100000 Hz`, `primary_secondary_ratio=2`.

1. `Po=12 W`; energy per cycle `Po/fs=120e-6 J`.
2. `primary_peak_current = sqrt(2×12/(100e-6×100000)) = sqrt(2.4) = 1.5491933384829668 A`.
3. `duty = 100e-6×100000×1.5491933384829668/48 = 0.3227486121839514`.
4. `demagnetization_fraction = 100e-6×100000×1.5491933384829668/(2×12) = 0.6454972243679028`.
5. `idle_fraction = 1-0.3227486121839514-0.6454972243679028 = 0.031754163448145856`.
6. `primary_rms_current = 1.5491933384829668×sqrt(0.3227486121839514/3) = 0.5081327481546147 A`.
7. `secondary_rms_current = 2×1.5491933384829668×sqrt(0.6454972243679028/3) = 1.4372164478523368 A`.
8. `switch_blocking_voltage = 48+2×12 = 72 V`.
9. `diode_reverse_voltage = 12+48/2 = 36 V`.

The energy-per-cycle step is a derivation, not an additional catalog output. The eight actual output keys are steps 2–9. Output-current cross-check: `2×1.5491933384829668×0.6454972243679028/2 = 1 A`.

### Nominal, limiting and invalid cases

- Nominal: above complete map. Its nonzero idle is small; the calculation does not represent a production margin recommendation.
- Clean alternate: `vin=vout=10 V`, `n=1`, `Lm=100e-6 H`, `fs=100000 Hz`, `Io=.03125 A` gives Ipk=.25 A, D=.25, Ddemag=.25, idle=.5, both RMS currents `.25/sqrt(12) = .07216878364870322 A`, both blocking voltages 20 V.
- Physical boundary: same alternate inputs except `Io=.125 A` give Ipk=.5 A and D=Ddemag=.5. Equality must reject, not become a zero-idle DCM success. Slightly lower current admits; slightly higher rejects.
- Decreasing positive load sends current and conduction fractions toward zero and idle toward one. Exact no-load operation is excluded because a regulating converter may pulse-skip or stop rather than maintain this fixed-frequency loaded model.
- Reject nonpositive load, voltage, frequency, turns ratio or magnetizing inductance. An efficiency input is intentionally absent rather than treated as a free inconsistent correction.

### Mandatory diagram specification

Show an isolated flyback power circuit in the upper/middle band and a compact three-interval explanatory current sketch below it.

- Vin positive → primary dotted upper terminal; primary undotted lower terminal → Q drain; Q source → primary ground.
- Secondary **undotted** upper terminal → diode anode; diode cathode → positive output O. Secondary **dotted** lower terminal → isolated output ground. Cout and load connect O to output ground. The reversed winding dots make the diode off during Q on-time.
- Mark `Lm, primary` at the primary winding and an energy arrow “store” during on-time, then “deliver” toward the secondary during demagnetization. These arrows are explanatory, not copper connections.
- Current sketch: primary current rises from zero during D and is zero afterward; secondary current starts at `n Ipk` after turn-off, falls to zero over Ddemag, and remains zero during idle. Align switching boundaries. Use “illustrative, not to scale”; never draw overlapping primary/secondary conduction in this ideal model.
- Bind `Vi` → `vin`, `Vo` → `vout`, `Lm` → `magnetizing_inductance`, `Np/Ns` → `primary_secondary_ratio`, `Ipk` → `primary_peak_current`, `D` → `duty`, `Ddemag` → `demagnetization_fraction`, `idle` → `idle_fraction`. The secondary peak label must read `n × Ipk`, not bind `primary_peak_current` under an unqualified secondary `Ipk` label.
- Caption: “Primary storage, secondary delivery, then zero-current idle. Opposite winding-dot orientation blocks the secondary diode during storage. Ideal DCM; ringing and leakage spikes are omitted.”

## 5. `converters.llc_half_bridge_fha`

### Purpose, equations and physical domain

S5 derives the rectifier load transformation and half-bridge normalization. Define `n=Np/Ns`, actual DC load `Ro`, separate series inductance `Lr`, shunt magnetizing inductance `Lm`, and series capacitor `Cr`.

The sinusoidal rectifier current has peak `pi Io/2`; the square rectifier input voltage has fundamental peak `4Vo/pi`. Their ratio gives secondary AC load `8Ro/pi²`, so primary `Rac=8n²Ro/pi²` ohm. This load transformation is an essential nonlinear-rectifier part of the model, not a plain transformer resistance reflection.

Let `fr=1/(2pi sqrt(LrCr))`, `x=fs/fr`, `k=Lm/Lr`, `Q=sqrt(Lr/Cr)/Rac`. Dividing the complex tank impedance ratio by its real numerator gives:

`M = 1 / hypot(1 + (1-1/x²)/k, Q(x-1/x))`.

S5 uses `m=(Lm+Lr)/Lr`; here `k=m-1`. The two forms are algebraically identical for the **separate Lr** model. The half-bridge relation is `M=2nVo/Vin`, hence `Vo=Vin M/(2n)`. A full bridge would not have that factor two and is not this calculator.

For `omega=2pi fs`, the input impedance is `jomegaLr + 1/(jomegaCr) + (jomegaLm || Rac)`. Its reactance is:

`Xin = omegaLr - 1/(omegaCr) + omegaLm / [1+(omegaLm/Rac)²]`.

The JSON evaluates the last term using normalized `hypot` factors. Positive Xin is inductive; negative Xin is capacitive. This is a descriptive output, not a boolean promise of ZVS. Real switches need sufficient current to discharge their capacitance within dead time.

FHA is approximate near sinusoidal tank operation and is most reliable near series resonance. S6 explicitly warns against relying on it away from resonance. No universal frequency-ratio cutoff provides a physics guarantee, so no artificial bound is inserted. Finite positive Ro is required; open circuit and short circuit are not represented. Integrated magnetics with appreciable secondary leakage require S5's different equivalent model. Diode-equivalent synchronous rectification is acceptable only if it preserves these conduction assumptions.

### Complete independent example

Inputs: `vin=400 V`, `primary_secondary_ratio=4`, `resonant_inductance=20e-6 H`, `magnetizing_inductance=100e-6 H`, `resonant_capacitance=100e-9 F`, `switching_frequency=120000 Hz`, `load_resistance=10 ohm`.

1. `resonant_frequency = 1/(2pi sqrt(20e-6×100e-9)) = 112539.53951963827 Hz`.
2. `normalized_frequency = 120000/112539.53951963827 = 1.0662919051580078`.
3. `ac_load_resistance = 8×4²×10/pi² = 129.69111506219235 ohm`.
4. `quality_factor = sqrt(20e-6/100e-9)/129.69111506219235 = 0.10904475311935749`.
5. `inductance_ratio = 100e-6/20e-6 = 5`.
6. `tank_gain = 1/hypot[1+(1-1/1.0662919051580078²)/5, 0.10904475311935749×(1.0662919051580078-1/1.0662919051580078)] = 0.9763804121956473`.
7. `output_voltage = 400×0.9763804121956473/(2×4) = 48.81902060978236 V`.
8. `omega=753982.2368615504 rad/s`; `input_reactance = omega×20e-6 - 1/(omega×100e-9) + omega×100e-6/[1+(omega×100e-6/129.69111506219235)²] = 58.16865958726068 ohm`.

### Nominal, limiting and invalid cases

- Nominal: above map; positive input reactance does not waive a switching-charge check.
- Series resonance: set fs to `1/(2pi sqrt(LrCr))`. Then x=1, M=1 and Vo=50 V regardless of finite positive Ro. Verify multiple loads; do not confuse the low-frequency Lr+Lm resonance with this point.
- Signed-region example: keep defaults except fs=40000 Hz; the FHA reactance is `-10.53912559450507 ohm`. The arithmetic result is admitted and explicitly capacitive; far-from-resonance accuracy is not claimed.
- Light-load limit Ro→infinity makes Q→0 and exposes the lower resonance. Exact infinity is invalid. Near lower resonance the approximation can predict very large gain; do not clamp it into a supposed regulated voltage or claim device feasibility.
- Reject zero/negative Vin, n, Lr, Lm, Cr, fs or Ro. No imposed requirement `Lm>Lr`: positive values define the stated equivalent circuit, even though practical designs commonly use larger Lm.

### Mandatory diagram specification

Use the actual half-bridge plus the explicitly labeled FHA equivalent, with clear separation rather than an unlabeled generic RLC.

- Main circuit: QH from +Vin to switching node H and QL from H to primary ground. H feeds series Cr then separate Lr then node P. Shunt Lm connects P to primary ground; ideal transformer primary also connects P to primary ground. Draw the transformer secondary feeding a **full-wave diode bridge**, then Cout and Ro. Secondary ground stays isolated.
- Mark transformer ratio `Np/Ns`. The series Cr blocks DC; do not add a wire bypassing it. Do not put Lm in series with Lr.
- If space makes eight rectifier/switch symbols illegible, use a labeled standard full-wave bridge rectangle with two AC terminals and separate +/− DC pins; keep the electrical port connections explicit.
- A small lower-band equivalent can replace the rectifier/transformer with Rac in parallel with Lm, labeled `Rac = 8 n² Ro/pi²`. This is an explanatory equivalent, not another physical load connected to the main circuit.
- Bind `Vin` → `vin`, `Lr` → `resonant_inductance`, `Lm` → `magnetizing_inductance`, `Cr` → `resonant_capacitance`, `Np/Ns` → `primary_secondary_ratio`, `Ro` → `load_resistance`, `Rac` → `ac_load_resistance`, `Vo, FHA` → `output_voltage`. Put `M = 2 n Vo/Vin` next to `tank_gain` only if bound as `M`, not as a voltage.
- Caption: “Half-bridge, separate Lr, ideal full-wave rectifier. Rac represents the rectifier's fundamental load. FHA estimates steady conversion; it is not a time-domain waveform or a ZVS guarantee.”

## 6. `converters.dual_active_bridge_power`

### Purpose, equations and physical domain

S7 equation 6 describes two bipolar square-wave bridges, not the sinusoidal bus formula discussed earlier in the source. Define `n=Np/Ns`, transfer inductance `L` referred to primary, DC buses `V1,V2>0`, and secondary lag `phi` in radians. Primary-referred secondary bridge amplitude is `nV2`.

Integrating the piecewise-linear inductor current against bridge voltage over a switching cycle gives:

`P = n V1 V2 phi (pi-|phi|)/(2 pi² fs L)`.

The JSON's `phi(1-|phi|/pi)/(2pi)` form is identical. Positive phi means primary bridge leads and positive power goes primary→secondary. The ideal maximum magnitude at `|phi|=pi/2` is `Pmax=nV1V2/(8fsL)`. Supporting outputs are voltage ratio `nV2/V1`, average primary current drawn `P/V1`, average secondary current delivered `P/V2`, and signed time lag `phi/(2pi fs)`.

Admit the full single-phase-shift domain `-pi<=phi<=pi`, not merely the monotonic ±90-degree range. The outer half is a falling-power branch with potentially high circulating current. Endpoints and zero phase give zero net power, not a general zero-current or zero-loss condition. Real ZVS depends on commutating current, voltage ratio, capacitance and dead time; it is not inferred from power alone. This model excludes internal bridge phase shifts (EPS/DPS/TPS), DC bias, magnetizing current and losses.

### Complete independent example

Inputs: `primary_voltage=400 V`, `secondary_voltage=200 V`, `primary_secondary_ratio=2`, `switching_frequency=100000 Hz`, `transfer_inductance=50e-6 H`, `phase_shift=pi/6=0.5235987755982988 rad`.

1. `voltage_ratio = 2×200/400 = 1`.
2. With `phi/pi=1/6`, `transferred_power = (2×400×200/(2×100000×50e-6))×(1/6)×(5/6) = 2222.222222222222 W`.
3. `maximum_power = 2×400×200/(8×100000×50e-6) = 4000 W`.
4. `primary_bus_current = 2222.222222222222/400 = 5.555555555555555 A`.
5. `secondary_bus_current = 2222.222222222222/200 = 11.11111111111111 A`.
6. `phase_delay = (pi/6)/(2pi×100000) = 8.333333333333333e-7 s`.

### Nominal, limiting and invalid cases

- Nominal: above full map. Lossless average bus powers agree, despite the 2:1 current scaling.
- Reverse phase `-pi/6`: ratio and maximum power unchanged; transferred power, both bus currents and phase delay negate exactly.
- Zero phase: P=I1=I2=delay=0; ratio=1, maximum=4000 W. Repeat zero phase with mismatched voltages to ensure no assertion that zero net power means zero circulating current.
- `phi=pi/2`: P=4000 W, primary current=10 A, secondary current=20 A, delay=2.5e-6 s. `phi=5pi/6` yields the same positive power as pi/6 but a larger delay, proving the falling-power branch is retained.
- `phi=±pi`: P and bus currents zero, delay=±5e-6 s. These endpoints are mathematical SPS limits, not recommended operating points.
- Reject phase outside ±pi and any nonpositive bus voltage, ratio, frequency or transfer inductance. Negative phase is valid and must not be mistaken for a negative-input error.

### Mandatory diagram specification

Draw two full bridges and a transformer, not a diode-rectified forward converter.

- Primary bridge has four switches between +V1 and primary return. Its two leg midpoints are A and B. A connects through explicit Ltransfer to the transformer primary dotted terminal; primary undotted terminal returns to B.
- Secondary bridge has four switches between +V2 and isolated secondary return. Its two AC leg midpoints C and D connect to secondary dotted and undotted terminals respectively. DC buses each have a capacitor/source shown with + and − polarity. Do not connect the two returns across the isolation barrier.
- Label both bridge drive patterns “bipolar 50% duty”. An arrow over the transformer points primary→secondary and is labeled `P > 0`. Reversal is described in the caption rather than by a diode arrow.
- Reserve x=20–225 for primary bridge, x=255–435 for L/transformer, x=470–680 for secondary bridge. Use the lower band for two compact square-wave sketches: primary positive transition first, secondary positive transition later by phi. Curves are illustrative, not calculated.
- Bind `V1` → `primary_voltage`, `V2` → `secondary_voltage`, `Np/Ns` → `primary_secondary_ratio`, `L` → `transfer_inductance`, `phi` → `phase_shift`, `P` → `transferred_power`, `fs` → `switching_frequency`. Place the signed phase annotation between waveform transition markers, not on a DC source.
- Caption: “Two active full bridges exchange power through primary-referred transfer inductance. Positive phi means the secondary lags and receives power. Static phase sketch; zero net power need not mean zero circulating current.”

## Handoff and unresolved engineering risks

Promote the six JSON objects without changing IDs, quantity keys, sign conventions or declared physical models. Implement all six mandatory diagrams with the existing painter primitives. No engine extension or package is required. Parent owns board/index updates and shared diagram registry.

Independent testers should execute all complete maps, topology boundaries and invalid-domain recovery described above after integration. In particular, test the SEPIC uncoupled-vs-coupled factor, forward reset ratio orientation, full-period flyback RMS, LLC half-bridge factor two and DAB signed/full-angle behavior. Review diagrams against node/dot/diode specifications before native pixel inspection.

No unresolved lane collision remains: Control confirmed the steady-state/feedback split. Remaining risks are explicit model limits, not missing formulas: charge-pump output resistance is approximate; SEPIC assumes small capacitor ripple and branch-positive CCM; forward assumes an existing CCM choke; flyback omits losses/spikes; LLC FHA does not establish far-from-resonance accuracy or ZVS; DAB power alone cannot rate RMS current or ensure soft switching. Hardware characterization, safe component selection, switching simulation, native UI rendering and numerical engine verification are not claimed.
