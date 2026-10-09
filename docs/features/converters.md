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

