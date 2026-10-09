# Oscillator design — executable feature catalog

Catalog models preserve the researched SI schemas, constraints, complete independent examples and source limitations. Diagrams are static explanatory circuits, not simulations. Shared registry integration and all runtime/native acceptance belong to Main.

### Actually read source register

| Key | Source and locator | Evidence type and weight |
|---|---|---|
| T555 | [TI xx555 SLFS022K](https://www.ti.com/lit/ds/symlink/ne555.pdf), §6.3.2, Figure 6-5, equations 1–4; §5.5 footnote and §6.1 | Primary manufacturer device document. Direct circuit/timing support; formulas are ideal approximations, not timing-error guarantees. |
| RC | [TI SNOA998A](https://www.ti.com/lit/ab/snoa998a/snoa998a.pdf), pp. 1–2, design description and steps 1–7 | Primary manufacturer application circuit. Supports comparator positive-feedback threshold network and RC charge/discharge mechanism. Unequal-resistor equations below are independently derived. |
| TRI | [TI SBOA212B](https://www.ti.com/lit/pdf/sboa212), triangle-generator subcircuit and steps 3–4, pp. 1–2 | Primary manufacturer design example. Only its autonomous triangle subcircuit is used; the PWM/error-amplifier function is excluded. |
| WIEN | [TI SLOA060](https://www.ti.com/lit/an/sloa060/sloa060.pdf), §§3, 5, 6, 8.1–8.3, Figures 7–16 | Primary manufacturer circuit analysis. Direct Wien gain/phase support and important warning about treating unbuffered RC sections as independent. |
| PHASE | [GSU HyperPhysics phase-shift oscillator](http://hyperphysics.phy-astr.gsu.edu/hbase/Electronic/oscphas.html), [frequency equation image](http://hyperphysics.phy-astr.gsu.edu/hbase/Electronic/ietron/pso.gif) and [schematic image](http://hyperphysics.phy-astr.gsu.edu/hbase/Electronic/ietron/pso2.gif) | University instructional source, not a manufacturer guarantee. Explicit frequency and 1/29 feedback; its general schematic needs input-loading care. Exact implemented termination and nodal proof are stated below. |
| LC | [B. Razavi, The Cross-Coupled Pair—Part III](https://www.seas.ucla.edu/brweb/papers/Journals/BR_Magzine3.pdf), IEEE Solid-State Circuits Magazine, Winter 2015, “XCP as Negative Resistance,” Figure 2(b) | University-hosted author article. Direct support for the differential −2/gm resistance and LC use. Pole/growth equations below follow independently from tank KCL. |
| XTAL | ST AN2867 Rev16, August 2022, §§3.1–3.4, pp. 11–14, [actually read PDF mirror](https://oss.yzcstatic.com/online/xqq/20230106/3nnyWGGbbeGhRPJ7ZxFYsTEGWQhdTiXa.pdf), [manufacturer original URL](https://www.st.com/resource/en/application_note/cd00221665.pdf) | Primary manufacturer-authored document retrieved through a third-party mirror. Direct equal-capacitor, critical-gm and ratio-method support. The mirror is not independent corroboration. |

Evidence/access gaps: ST's direct English, content-path and regional PDF URLs timed out; an ST community link redirected to its home page; Silicon Labs AN0016.1 returned 403. A manuals.plus endpoint exposed raw PDF as text, then blocked direct fetches. The ST regional mirror aborted; an old StudyLib page exposed no document text. The accessible AN2867 PDF above was actually read instead. A University of Florida lab-manual hostname did not resolve; it is not cited as evidence. GSU transparent images were read after white-background compositing; HTTPS hostname verification failed, but its published HTTP image URLs worked. No claim relies on a search-result summary alone.


The source-access statements above record the research handoff, not new source retrieval during implementation. Full novelty and numerical conditioning notes: [research](../../research/expansion/oscillators.md).

## 1. `oscillators.timer_555_astable` — basic

### Purpose, novelty and source

Determine the settled frequency and output duty of an actual self-triggering 555. T555 §6.3.2 supplies the topology, threshold fractions and charge/discharge equations. The model uses exact `ln(2)` instead of the data sheet's rounded 0.693.

Nearest old models: `timing.timer_clock_divider` needs an existing reference clock; this circuit creates oscillation through internal threshold comparators and a discharge switch. `transients.rc_periodic_switching` takes externally imposed interval lengths; this circuit solves those lengths from thresholds and different current paths. The comparator oscillator below instead has a resistor-defined positive-feedback threshold network and no dedicated discharge transistor.

### Model, domains and limitations

RA runs from VCC to DISCH pin 7. RB runs from pin 7 to the capacitor node. Pins TRIG 2 and THRES 6 share that node; C connects node to ground. RESET 4 is high. Internal switching limits are VCC/3 and 2VCC/3. The capacitor charges toward VCC through RA+RB while output is high, and discharges toward zero through RB while output is low:

- `high_time = (RA+RB) C ln(2)`.
- `low_time = RB C ln(2)`.
- `period = high_time + low_time`.
- `frequency = 1/period`.
- `high_duty = (RA+RB)/(RA+2RB)`.

All components are strictly positive. The nominal period must be at least 10 µs, reflecting TI's 100 kHz bipolar-555 distortion guideline. This is not a maximum-frequency guarantee. Assume decoupled 5–15 V operation; check selected-part current, leakage and capacitor ratings externally. TI notes that maximum timing resistance depends on supply and input current (approximately RA+RB ≤3.4 MΩ at 5 V in its cited example). The model does not silently claim these limits are automatic. Discharge saturation, timing tolerance, propagation delay and first-cycle startup are omitted. Positive RA means duty strictly exceeds 0.5; diode-steered astables are a different topology and excluded.

### Complete independent example

Inputs: `charge_resistance=1000 ohm`, `discharge_resistance=10000 ohm`, `timing_capacitance=1e-7 F`.

`(RA+RB)C=0.0011 s`, `RB C=0.001 s`, and `ln(2)=0.6931471805599453094`.

| Output key | Derivation | Expected |
|---|---|---|
| `high_time` | 0.0011 ln(2) | 0.0007624618986159398 s |
| `low_time` | 0.001 ln(2) | 0.0006931471805599453 s |
| `period` | 0.0021 ln(2) | 0.0014556090791758851 s |
| `frequency` | 1/[0.0021 ln(2)] | 686.997638518554 Hz |
| `high_duty` | 11000/21000 | 0.5238095238095238 |

Tester cases: doubling C doubles both intervals and period, halves frequency and preserves duty. RA=RB gives duty 2/3. As RA/RB approaches zero from above, duty approaches 1/2 but does not pass below it. Reject zero/negative resistance or capacitance. With defaults except C=1e-12 F, reject the frequency-limit violation. Exercise the 100 kHz period boundary using representable values on both sides; do not assume a decimal-rounded equality is exact binary64 equality.

### Mandatory diagram specification

Draw a labeled 555 block left-center with correct named pins and pin numbers. VCC → RA → pin-7 junction → RB → timing junction → C → ground. Route the timing junction to both pins 2 and 6. Tie pin 4 to VCC and pin 1 to ground. Show pin 3 output toward the right; show pin 5 with a fixed, unbound “CONT bypass” capacitor to ground if space permits. Do not put the output in the capacitor charging path. Mark charging arrow through RA+RB and discharge arrow from timing node through RB into pin 7. Right inset shows illustrative capacitor motion between VCC/3 and 2VCC/3, aligned with output-high and output-low intervals.

Exact quantity bindings: RA `charge_resistance`; RB `discharge_resistance`; C `timing_capacitance`; high interval `high_time`; low interval `low_time`; period/frequency `period`/`frequency`; duty legend `high_duty`. VCC and fractional-threshold labels are unbound explanatory labels because supply is not an input.

Caption: “Standard 555 astable: C charges through RA+RB and discharges through RB. Ideal one-third/two-thirds thresholds; waveform illustrative, not simulated.”


## 2. `oscillators.comparator_rc_oscillator` — intermediate

### Purpose, novelty and source

Design an asymmetric comparator square-wave oscillator from its positive-feedback resistor network. RC describes the equal-resistor case; the following conductance derivation extends the same concrete circuit to unequal resistors. This is not a Schmitt input delay measured after an external edge. It determines two state-dependent thresholds and both intervals of an autonomous cycle. Unlike the 555, both timing directions use the same RT, and unequal duty comes from the threshold network rather than separate charge/discharge resistance paths.

### Model, domains and limitations

At the comparator noninverting input, RU connects to Vs, RL to ground and RF to output. The inverting input senses the timing capacitor, charged through RT from output. Output levels are exactly zero and Vs in this ideal model.

Let `G=1+RF/RU+RF/RL`. Nodal analysis gives `VL=Vs(RF/RU)/G` and `VH=Vs(1+RF/RU)/G`. Thus `VH−VL=Vs/G`. While output is high, the capacitor rises from VL to VH:

`tH=RT C ln[(Vs−VL)/(Vs−VH)]=RT C log1p(RL/RF)`.

While output is low, it falls from VH to VL:

`tL=RT C ln(VH/VL)=RT C log1p(RU/RF)`.

Frequency is `1/(tH+tL)` and output-high duty is `tH/(tH+tL)`. Positive finite supply, resistors and C guarantee `0<VL<VH<Vs`. No extra arbitrary ordering constraint is needed. Do not derive times by subtracting nearly equal displayed thresholds; the ratio algebra above is more stable.

A real comparator must have suitable input common-mode range, negligible internal hysteresis relative to the designed window, low output resistance relative to RT, and negligible delay relative to each interval. Excessively small C makes parasitics dominate. Low threshold-network impedances reduce bias-current/stray errors but increase output loading. Real finite rail swing or asymmetric propagation delay requires a different model; this report does not treat ideal output rails as guaranteed device specifications.

### Complete independent example

Inputs: `supply_voltage=5 V`, `upper_resistance=20000 ohm`, `lower_resistance=40000 ohm`, `feedback_resistance=20000 ohm`, `timing_resistance=10000 ohm`, `timing_capacitance=1e-7 F`.

`G=1+1+0.5=2.5`; `RT C=0.001 s`.

| Output key | Derivation | Expected |
|---|---|---|
| `upper_threshold` | 5(1+1)/2.5 | 4 V |
| `lower_threshold` | 5/2.5 | 2 V |
| `high_time` | 0.001 ln(3) | 0.0010986122886681097 s |
| `low_time` | 0.001 ln(2) | 0.0006931471805599453 s |
| `frequency` | 1000/ln(6) | 558.1106265512473 Hz |
| `high_duty` | ln(3)/ln(6) | 0.6131471927654584 |

Tester cases: setting all three threshold resistors to 20 kohm gives VL=5/3 V, VH=10/3 V, both intervals `0.001 ln(2) s`, frequency `721.3475204444817 Hz` and duty 0.5. Scaling all three threshold resistors equally leaves every output unchanged. Doubling supply doubles thresholds but leaves timing unchanged. Swap RU and RL: times exchange and duty complements; thresholds reflect around Vs/2. Large RF narrows the hysteresis window; `log1p` should retain nonzero small intervals. Reject any nonpositive supply/component. The ideal high-frequency limit is not a claim that finite-delay hardware still oscillates at that rate.

### Mandatory diagram specification

Place a push-pull comparator center-left. Its noninverting node has three visible branches: RU upward to Vs, RL down to ground, RF from output around the upper route. Its inverting node connects to RT from output around a separate lower route and C to ground. Keep both feedback paths visibly separate; crossings need no junction unless intentionally connected. Output goes to a right-side static square-wave inset; the capacitor inset shows a curved rise VL→VH while output is high, then curved decay VH→VL while output is low.

Exact bindings: supply `supply_voltage`; RU `upper_resistance`; RL `lower_resistance`; RF `feedback_resistance`; RT `timing_resistance`; C `timing_capacitance`; threshold labels `upper_threshold`/`lower_threshold`; output interval legend `high_time`/`low_time`; frequency/duty legend `frequency`/`high_duty`.

Caption: “Output-dependent positive feedback sets VH and VL; the same RT charges and discharges C. Ideal rail swing and zero delay; curves are illustrative.”


## 3. `oscillators.triangle_integrator_oscillator` — intermediate

### Purpose, novelty and source

Design the amplitude and period of an op-amp integrator closed around a hysteretic comparator. TRI's triangle-generator block and design steps 3–4 give the amplitude ratio and oscillation frequency. Its single-supply midpoint implementation is translated to symmetric voltages about a stiff reference; no PWM model is added.

Nearest old model: `opamps.integrator_constant_input` integrates a prescribed input from a prescribed initial state. This addition determines self-consistent switching amplitude and repetition frequency, with no prescribed interval. Unlike the RC comparator oscillator, an active integrator maintains a constant input-resistor voltage and produces linear ramps instead of exponential capacitor charging.

### Model, domains and limitations

Comparator inverting input is reference zero. Its noninverting input sums triangle output through RT and comparator square output through RF. At switching, `(Vtri/RT)+(Vsq/RF)=0`, hence the triangle threshold is `Vtri=−(RT/RF)Vsq`. When square output is +Vsq, the inverting integrator ramps downward; it switches at the negative triangle threshold, then ramps upward when square output becomes negative.

`triangle_peak=(RT/RF)square_peak`; `triangle_pp=2 triangle_peak`; `ramp_slew=square_peak/(RI C)`; half-cycle is `2 triangle_peak/ramp_slew`; full `period=4 RI C RT/RF`; `frequency=1/period`.

All inputs are positive. Symmetric square levels are essential to this formula. A real integrator's available output swing must exceed `triangle_peak`, and its slew rate must exceed `ramp_slew`; select supply rails and devices accordingly. GBW must support the integration approximation. Comparator delay, offsets, bias drift, capacitor leakage and saturation recovery are omitted. There is no arbitrary RT/RF<1 restriction because the integrator can legitimately have larger supply rails than the comparator. The default ±2 V triangle and ±5 V square are feasible with suitable split-supply devices; the cited 5 V single-supply part numbers are not asserted to support this exact split-supply example.

### Complete independent example

Inputs: `square_peak=5 V`, `triangle_resistance=8000 ohm`, `feedback_resistance=20000 ohm`, `integrator_resistance=10000 ohm`, `integrator_capacitance=1e-8 F`.

`RT/RF=0.4`; `RI C=1e-4 s`. Each ramp traverses 4 V at 50000 V/s, taking 80 µs.

| Output key | Derivation | Expected |
|---|---|---|
| `triangle_peak` | 0.4×5 | 2 V |
| `triangle_pp` | 2×2 | 4 V |
| `ramp_slew` | 5/1e-4 | 50000 V/s |
| `period` | 2×4/50000 | 0.00016 s |
| `frequency` | 1/0.00016 | 6250 Hz |

Tester cases: doubling square_peak doubles triangle_peak, triangle_pp and ramp_slew, while period and frequency remain unchanged. Doubling RT doubles triangle amplitude and period but leaves ramp_slew unchanged. Doubling RI doubles period and halves ramp_slew and frequency. With RT/RF approaching zero, predicted frequency diverges; hardware delay/slew limitations invalidate that asymptote, not the algebra. Reject zero/negative amplitude, resistance or capacitance. A high RT/RF is mathematically permitted but requires explicit larger integrator swing, not clipping disguised as a triangle result.

### Mandatory diagram specification

Draw the actual two-device loop, not two disconnected amplifier symbols. Comparator output → RI → integrator inverting input; C bridges integrator output to that input; integrator noninverting input goes to reference ground. Integrator triangle output → RT → comparator noninverting summing node; RF returns comparator output to that same node. Comparator inverting input goes to reference ground. Mark the square output ±Vsq and triangle output ±Vtri. Show a descending triangle segment under positive square output. DC supplies may be omitted with a caption requiring sufficient headroom.

Exact bindings: ±Vsq peak `square_peak`; RT `triangle_resistance`; RF `feedback_resistance`; RI `integrator_resistance`; C `integrator_capacitance`; triangle peak `triangle_peak`; peak-to-peak legend `triangle_pp`; slope magnitude `ramp_slew`; timing legend `period`/`frequency`. Do not bind a ± label to peak-to-peak voltage.

Caption: “The inverting integrator reverses ramp direction when the comparator changes state. Voltages are relative to a stiff reference; symmetric output levels and adequate linear swing are assumed.”

