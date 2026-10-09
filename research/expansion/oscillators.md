# Oscillator expansion research — seven calculations

## Scope, novelty and evidence

Owned deliverables: this report and `research/expansion/oscillators.json`. No production code, registry, tests, existing catalogs or board entries change. The JSON array contains exactly seven items: two basic, three intermediate and two expert. All numerical inputs and outputs use SI units; duty, feedback and gain ratios are dimensionless.

The baseline's 128 IDs and formulas were read from `coordination/expansion/baseline.json`, alongside both contracts, the board and the expansion communication bus. No old oscillator exists. Closest exclusions are `timing.schmitt_rc_delay_bounds`, `timing.timer_clock_divider`, `transients.rc_periodic_switching`, `transients.capacitor_constant_current`, `opamps.integrator_constant_input`, `opamps.noninverting_stage`, `filters.buffered_rc_cascade`, `ac.series_resonator_bandwidth` and `ac.lossy_coil_antiresonance`. The seven additions solve autonomous active-loop design, not inverses or another output of those models. Crystal load sizing and startup assessment are deliberately one calculation, not two.

Search boundary: English manufacturer application notes/data sheets and university-hosted teaching/research material, accessed 2026-10-09. Searches covered 555 astables, comparator relaxation circuits, comparator/integrator triangle oscillators, Wien bridges, loaded RC phase-shift circuits, cross-coupled LC negative resistance and Pierce crystal startup. No date cutoff: established circuit equations remain applicable under their stated idealizations. Excluded bare resonance, driven passive delay, digital division, arbitrary waveform synthesis, noise/jitter and PLL control. This is a focused engineering literature search, not an exhaustive systematic review.

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

## Shared numerical and illustration requirements

Expected values were independently derived with Python standard-library `decimal` at 50-digit precision, including `ln` and `sqrt`; pi was explicitly supplied to 50 digits. No engine invocation, build, test, lint, formatter or desktop check was run. A missing optional `mpmath` package was bypassed without installation. JSON examples contain every output, not selected snapshots. Relative tolerance `1e-9` is for ordinary nominal values; exact-zero boundaries require explicit zero/absolute assertions. It does not claim physical accuracy to nine digits.

All formulas use the existing engine grammar. `log1p` preserves small relaxation hysteresis intervals. Signed loop excess is formed from resistor differences rather than subtracting rounded loop gains. LC frequency uses separate reciprocal square roots, and its oscillatory-pole factor uses `(1-u)(1+u)` rather than subtracting squared large frequencies. These avoid common cancellation/product errors without adding engine primitives. IEEE-754 extremes still need parent numerical review; no artificial ranges were added to hide them.

Every diagram uses the existing primitives on a 700×280 canvas, 16 px margins and labels at least 14 px. Use native component pins and explicit junction dots; break wires at every component. Do not use an op-amp's pin polarity interchangeably with its drawing position. A wire crossing is not a junction unless marked. Reserve the rightmost region for a small explanatory condition or schematic waveform where specified; quantity legends may carry secondary values instead of crowding the circuit. All waveform curves are static, explicitly not computed. Bind only exact keys listed below. Captions below are required explanatory content, not promises of hardware validation.

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

## 4. `oscillators.wien_bridge_balance` — basic

### Purpose, novelty and source

Select a Wien oscillator frequency and check the amplifier's small-signal loop balance. WIEN §8.1 derives the equal-R/equal-C feedback factor of 1/3; §§3 and 5 explain startup and amplitude stabilization. This is neither the existing noninverting amplifier by itself nor an ordinary filter-response calculation: the bridge's zero-phase frequency and loop gain jointly determine oscillator feasibility. No output amplitude or THD is invented.

### Model, domains and limitations

Noninverting amplifier: RF from output to inverting input; RG from that input to ground. Positive feedback: output → series R,C → noninverting input; R and C in parallel from noninverting input to ground. The bridge transfer is `beta(s)=sRC/[(sRC)^2+3sRC+1]`. At `omega RC=1`, phase is zero and beta=1/3. Therefore:

- `frequency=1/(2 pi RC)`.
- `feedback_fraction=1/3`.
- `critical_feedback_resistance=2 RG`.
- `amplifier_gain=1+RF/RG`.
- `loop_gain=amplifier_gain/3`.
- `loop_excess=(RF−2RG)/(3RG)`.

All resistances and C are positive. Below-unity and exactly-unity loop gain are admitted as diagnostic results: below unity decays; unity alone does not start from zero; modest excess supports small-signal growth. A steady sine requires nonlinear stabilization (lamp, AGC or soft limiting), which is not modeled. A fixed-gain circuit may clip or behave poorly. Finite amplifier phase shift changes frequency and gain; TI §6 discusses bandwidth and slew limitations. No calculated output is a promise of oscillation, amplitude or purity on an arbitrary op amp.

### Complete independent example

Inputs: `bridge_resistance=10000 ohm`, `bridge_capacitance=1e-8 F`, `gain_resistance=10000 ohm`, `feedback_resistance=20200 ohm`.

| Output key | Derivation | Expected |
|---|---|---|
| `frequency` | 1/(2 pi×1e-4) | 1591.5494309189534 Hz |
| `feedback_fraction` | 1/3 | 0.3333333333333333 |
| `critical_feedback_resistance` | 2×10000 | 20000 ohm |
| `amplifier_gain` | 1+20200/10000 | 3.02 |
| `loop_gain` | 3.02/3 | 1.0066666666666666 |
| `loop_excess` | 200/(3×10000) | 0.006666666666666667 |

Tester cases: RF=20000 gives amplifier_gain=3, loop_gain=1 and loop_excess=0 exactly, with other outputs unchanged. RF=19000 gives 2.9, 29/30 and −1/30 respectively; this must not be rejected. Double either bridge component and frequency halves. Changing gain-setting resistors together preserves gain but scales critical RF. Reject nonpositive components. Probe a representable RF just above 2RG: loop_excess must retain the positive resistor difference even if displayed loop_gain rounds to one.

### Mandatory diagram specification

Draw one op amp with a complete noninverting gain network (RF upper feedback, RG to ground). Route output through a series R and C to the noninverting node; draw separate R and C shunt branches from that node to ground. Keep series and parallel R/C labels distinct but bind both resistors to the same R input and both capacitors to the same C input. Add an explicit positive-feedback loop arrow. A right-side note states “beta=1/3 at f0; startup needs A beta>1.” If a sine inset is present, mark it illustrative and amplitude unspecified; do not imply fixed-gain limiting is a clean sine generator.

Exact bindings: both R labels `bridge_resistance`; both C labels `bridge_capacitance`; RG `gain_resistance`; RF `feedback_resistance`; f0 `frequency`; beta `feedback_fraction`; A `amplifier_gain`; loop gain/excess `loop_gain`/`loop_excess`; critical-RF legend `critical_feedback_resistance`.

Caption: “The equal-component Wien bridge supplies zero phase and one-third feedback at f0. This checks small-signal balance only; real sine amplitude needs stabilization.”

## 5. `oscillators.loaded_rc_phase_shift` — intermediate

### Purpose, novelty and source

Design a three-section, unbuffered, high-pass RC oscillator without making the false independent-section assumption. PHASE gives `f0=1/(2 pi RC sqrt(6))` and feedback magnitude 1/29. WIEN §8.2 explicitly demonstrates the large error caused by neglecting loading. Its displayed low-pass circuit and the buffered gain-8 formula are not copied into this high-pass model.

Closest baseline: `filters.buffered_rc_cascade` uses independent buffered low-pass sections driven by an external source. Here sections load each other, the feedback is high-pass and an inverting amplifier closes the oscillation loop. This is not a filter-frequency rename or a second Wien calculation: its feedback polynomial, phase mechanism and required gain differ.

### Exact implemented topology and derivation

Output drives three capacitors in series: node1, node2, node3 after each capacitor. Node1 and node2 each have R to ground. Node3 connects through the third R to the op-amp inverting virtual ground. The op-amp noninverting input is ground; RF connects output to inverting input. There is **no separate third shunt R plus another amplifier input resistor**. That would load the final section differently and invalidate the equal-load equation.

The GSU illustrative schematic shows a separate amplifier input resistor alongside the three shunts; its quoted ideal factor needs negligible additional loading or an effective-load adjustment. Our third-R-to-virtual-ground termination realizes exactly the three equal loads assumed by the quoted formula. This distinction is intentional and must survive implementation and drawing.

For `x=sRC`, normalized node equations are:

`(2x+1)v1 − x v2 = x vo`

`−x v1 + (2x+1)v2 − x v3 = 0`

`−x v2 + (x+1)v3 = 0`.

Elimination gives `beta=v3/vo=x^3/(x^3+6x^2+5x+1)`. At `x=j/sqrt(6)`, denominator real part vanishes and beta=−1/29. The amplifier gain is `−RF/R`, so unity positive loop gain needs RF=29R. Equivalently the closed-loop characteristic is `(1+RF/R)x^3+6x^2+5x+1=0`; substituting x=j/sqrt(6) gives RF/R=29.

All components are positive. Admit loop gains below, at and above one for diagnosis. The reported frequency is the ladder's 180-degree frequency and the marginal oscillator design frequency, not a nonlinear steady-state solution at arbitrary excess gain. Ideal amplifier bandwidth/input behavior, zero output impedance and no extra ladder loading are assumed. Output amplitude, saturation, distortion and startup time are excluded.

### Complete independent example

Inputs: `section_resistance=10000 ohm`, `section_capacitance=1e-8 F`, `feedback_resistance=300000 ohm`.

| Output key | Derivation | Expected |
|---|---|---|
| `frequency` | 1/(2 pi×1e-4×sqrt(6)) | 649.7473343613968 Hz |
| `feedback_magnitude` | 1/29 | 0.034482758620689655 |
| `critical_feedback_resistance` | 29×10000 | 290000 ohm |
| `amplifier_gain_magnitude` | 300000/10000 | 30 |
| `loop_gain` | 30/29 | 1.0344827586206897 |
| `loop_excess` | (300000−290000)/(29×10000) | 0.034482758620689655 |

Tester cases: RF=290000 produces gain 29, loop_gain 1 and loop_excess 0; RF=280000 produces gain 28, loop_gain 28/29 and loop_excess −1/29. Both are valid diagnostic inputs. Doubling C halves frequency without changing any gain output. Doubling R and RF together halves frequency, doubles critical_feedback_resistance and preserves all gain ratios. Reject nonpositive components. Independently evaluate the complex ladder at omega=1/(RC sqrt(6)): real beta must be −1/29 and imaginary beta zero within tolerance; do not test using the buffered response.

### Mandatory diagram specification

Use a long horizontal chain of three C symbols from op-amp output around the lower/right side back toward the amplifier. The first two intermediate nodes each have R to ground. The last node connects through a third R to the amplifier inverting input. RF runs from output to the same inverting node. Noninverting input connects to ground. Make the absence of a third extra shunt resistor visually unambiguous. Label the third R “R to virtual ground.” Add a loop annotation “RC network: −180°; amplifier: −180°,” and `|beta|=1/29 at f0` in a free region. Do not draw buffers between sections.

Exact bindings: three C labels `section_capacitance`; all three effective R labels `section_resistance`; RF `feedback_resistance`; f0 `frequency`; beta magnitude `feedback_magnitude`; gain magnitude `amplifier_gain_magnitude`; loop-gain legend `loop_gain`/`loop_excess`; critical-RF legend `critical_feedback_resistance`.

Caption: “Unbuffered sections load one another. The third R terminates at virtual ground; no extra third-node shunt is fitted. Gain 29 is marginal, not a steady-amplitude guarantee.”

## 6. `oscillators.cross_coupled_lc_startup` — expert

### Purpose, novelty and source

Compare a cross-coupled pair's active negative conductance with differential tank losses and predict the startup pole pair. LC Figure 2(b) establishes −2/gm between the two differential terminals; the article describes its oscillator use. The following pole expression is independently derived from KCL, not claimed as a quoted source equation.

Closest old models: `ac.series_resonator_bandwidth` and `ac.lossy_coil_antiresonance` describe passive externally driven resonators. This addition includes the active pair, an oscillation threshold, signed envelope growth and the frequency of growing/decaying natural oscillation. Its supporting tank frequency is not separately counted.

### Model, signs, validity and derivation

Define differential tank voltage `v=Vplus−Vminus`, with passive current positive from plus to minus. L, C and Rp are complete two-terminal differential values. Each matched transistor has gm; the pair's differential resistance is `negative_resistance=−2/gm`. Its negative conductance is −gm/2. Include transistor output conductance and other positive losses in Rp.

Differentiated KCL is `C v''+(1/Rp−gm/2)v'+v/L=0`. Thus poles are `alpha ± j omega_start`, with:

`alpha=(gm/2−1/Rp)/(2C)`, `omega0=1/sqrt(LC)`, `omega_start=sqrt(omega0^2−alpha^2)`.

Outputs: `tank_frequency=omega0/(2pi)`; `critical_transconductance=2/Rp`; `startup_ratio=gm Rp/2`; `growth_rate=alpha`; `startup_frequency=omega_start/(2pi)`. Positive growth is exponential startup, zero is marginal, negative is decay. The formula does not say that negative resistance of larger absolute magnitude is better: **conductance** magnitude must exceed tank loss, equivalently `abs(Rneg)<Rp`.

Strictly positive L,C,Rp,gm are required; gm=0 would make the requested negative-resistance output infinite. The complex-pole condition `abs(gm/2−1/Rp)<2sqrt(C/L)` is enforced. Equality gives real repeated poles and is outside this oscillatory-frequency model; larger magnitude gives real poles. This is a physical pole-domain restriction, not a numerical convenience. Admit decaying underdamped cases rather than hiding failed startup designs.

Bias networks are AC open in differential mode, transistor response is quasistatic, and parasitic C is included in total C. This linear model is local around the bias point: nonlinear gm compression, final amplitude, phase noise, seed level and actual startup time are not supplied. A real oscillator must be biased appropriately and operate below significant device transit-time effects.

### Complete independent example

Inputs: `tank_inductance=1e-8 H`, `tank_capacitance=1e-11 F`, `parallel_loss_resistance=2000 ohm`, `transistor_transconductance=0.002 S`.

`omega0=sqrt(1e19)=3162277660.16837933 rad/s`; tank loss conductance=0.0005 S; negative-conductance magnitude=0.001 S. Therefore `alpha=0.0005/(2e-11)=25000000 1/s`, and `omega_start=sqrt(1e19−6.25e14)`.

| Output key | Derivation | Expected |
|---|---|---|
| `tank_frequency` | sqrt(1e19)/(2 pi) | 503292121.0448704 Hz |
| `negative_resistance` | −2/0.002 | −1000 ohm |
| `critical_transconductance` | 2/2000 | 0.001 S |
| `startup_ratio` | 0.002×2000/2 | 2 |
| `growth_rate` | 0.0005/(2e-11) | 25000000 1/s |
| `startup_frequency` | sqrt(1e19−6.25e14)/(2 pi) | 503276392.9203319 Hz |

Tester cases: gm=0.001 S gives negative_resistance=−2000 ohm, startup_ratio=1, growth_rate=0, and startup_frequency=tank_frequency. gm=0.0005 S gives −4000 ohm, ratio=0.5 and growth_rate=−12500000 1/s; it is valid but decays. With all inputs `{L:1,C:1,Rp:1,gm:6}`, the complex-pole boundary is exactly reached and must reject; gm=5 is inside it, and gm=7 outside. Use the actual catalog keys when submitting these vectors. Reject nonpositive components or gm. Do not confuse half-circuit gm Rp criteria with this explicitly differential convention.

### Mandatory diagram specification

Draw two identical NMOS devices with joined sources feeding a tail current-source symbol to a lower supply. Drain nodes are Vplus and Vminus; cross-connect each gate to the opposite drain using clearly nonjoining crossings. Draw a center-tapped inductor between the drains, center tap to VDD, with total differential L explicitly labeled; draw C and Rp across the same two drain nodes. A small right inset may replace the active pair with a −2/gm two-terminal element across the differential tank to teach the loss-cancellation criterion. Label differential voltage plus at Vplus, minus at Vminus. Do not use one half-inductor as the full `tank_inductance` binding.

Exact bindings: total L `tank_inductance`; differential C `tank_capacitance`; total Rp `parallel_loss_resistance`; each-device gm `transistor_transconductance`; active-element label `negative_resistance`; criterion legend `critical_transconductance`/`startup_ratio`; envelope-rate legend `growth_rate`; frequency legend `tank_frequency`/`startup_frequency`.

Caption: “L, C and Rp are differential tank values; gm belongs to each transistor. The pair supplies −gm/2 conductance. Positive alpha predicts only small-signal growth, not final amplitude.”

## 7. `oscillators.pierce_crystal_startup` — expert

### Purpose, novelty and source

Choose equal external load capacitors and assess whether the MCU/inverter can overcome the specified crystal loss with the requested startup margin. XTAL §§3.3–3.4 directly gives load capacitance and critical transconductance for equal capacitors, and distinguishes actual gm from maximum supported critical crystal transconductance.

Closest old model: `ac.lossy_coil_antiresonance` concerns a lossy passive coil, not a piezoelectric crystal, shunt capacitance and an inverter loop. Adjacent LC startup concerns a cross-coupled differential negative shunt conductance; this Pierce model concerns an inverter, capacitive loading and crystal ESR. Load sizing and startup ratio are one design problem and one item. No standalone crystal resonance or inverse capacitor calculator is counted.

### Model, domains and limitations

Pierce topology: biased inverter between OSC_IN and OSC_OUT; high-valued feedback resistor across it; crystal bridging the same two nodes; one load capacitor from each node to ground. Under equal capacitors, terminal load is `CL=Ceach/2+Cs`, so `load_capacitor_each=2(CL−Cs)`.

Use specified crystal load CL, shunt capacitance C0, nominal frequency f and worst-case ESR. For this AN2867 model:

`gmcrit=4 ESR [2 pi f(C0+CL)]^2`.

`gain_margin=gm_available/gmcrit`; `design_transconductance=margin_target gmcrit`; signed `transconductance_headroom=gm_available−design_transconductance`.

Require f,ESR,CL,gm>0; C0 and Cs may equal zero; margin_target≥1; CL>Cs ensures physically positive external capacitors. Below-target gm is allowed and reported with negative headroom. Margin_target=1 is the ideal marginal design boundary, not robust hardware guidance; the nominal value 5 follows AN2867's actual-gm method. A product's own oscillator guidance takes precedence.

**Do not substitute `Gm_crit_max` for actual gm.** Some MCU data sheets call their maximum supported critical crystal transconductance `Gm`; that parameter is compared directly with gmcrit instead, and already embodies its own design criterion. This catalog deliberately accepts actual available inverter gm only.

Cs is equivalent terminal stray capacitance. Two pin-to-ground capacitances must be reduced with the full network; do not simply add them and also include them in Ceach. ESR means the crystal data-sheet parameter applicable to this startup model, not an arbitrary measured DC resistance. This version excludes a series drive-limiting resistor; fitting one requires the extended model and further device-specific checks. Fundamental-mode operation, symmetric load branches and correct crystal load tuning are assumed. Drive level, crystal nonlinearities, process/voltage/temperature, frequency-range compatibility and PCB coupling remain hardware checks. A large computed gm margin is not proof of safe crystal power or guaranteed startup.

### Complete independent example

Inputs: `crystal_frequency=16000000 Hz`, `crystal_esr=40 ohm`, `shunt_capacitance=3e-12 F`, `load_capacitance=1.2e-11 F`, `stray_capacitance=2e-12 F`, `available_transconductance=0.005 S`, `margin_target=5`.

`C0+CL=15e-12 F`; `2 pi f(C0+CL)=0.00150796447372310075 S`; multiply its square by `4 ESR=160 ohm`.

| Output key | Derivation | Expected |
|---|---|---|
| `load_capacitor_each` | 2(12−2) pF | 2e-11 F |
| `critical_transconductance` | 160[2 pi×16e6×15e-12]^2 | 0.0003638330966417581 S |
| `gain_margin` | 0.005/0.0003638330966417581161 | 13.742565055655622 |
| `design_transconductance` | 5×0.0003638330966417581161 | 0.0018191654832087906 S |
| `transconductance_headroom` | 0.005−0.001819165483208790581 | 0.0031808345167912094 S |

Tester cases: Cs=0 gives each external C=24 pF with every gm output unchanged, because nominal CL stays fixed. C0=0 is valid and changes gmcrit by factor `(12/15)^2=0.64`. Doubling ESR doubles gmcrit and required gm, and halves gain_margin. With available gm=0.001 S, gain_margin≈2.7485130111311244 and headroom≈−0.0008191654832087906 S: valid failed-margin result. Set available gm equal to independently computed design gm and check near-zero headroom with a justified absolute tolerance. Reject Cs=CL, Cs>CL, negative C0/Cs, nonpositive f/ESR/CL/gm and margin_target<1. When Cs approaches CL from below, external C approaches zero; parasitic uncertainty then dominates physical accuracy.

### Mandatory diagram specification

Draw an inverter symbol with OSC_IN and OSC_OUT nodes, connected by a high-valued bias resistor above the inverter and by a crystal symbol on a separate lower branch. Draw C1 from OSC_IN to ground and C2 from OSC_OUT to ground; both bind to the same computed `load_capacitor_each`. Do not wire crystal or resistor in series with the inverter signal path. Put a small crystal equivalent-model inset or legend near the crystal identifying ESR and C0 without falsely depicting a simple standalone capacitor as the full crystal. Label Cs as “equivalent terminal stray,” optionally a dashed capacitor across the two nodes, not two copies to ground. Right annotation shows available gm versus M×gmcrit.

Exact bindings: crystal f `crystal_frequency`; ESR `crystal_esr`; C0 `shunt_capacitance`; required CL `load_capacitance`; equivalent Cs `stray_capacitance`; both C1/C2 `load_capacitor_each`; inverter gm `available_transconductance`; target M `margin_target`; result legend `critical_transconductance`, `gain_margin`, `design_transconductance`, `transconductance_headroom`. Bias-resistor value is an unbound “RF bias” label, since no resistance is calculated here.

Caption: “Equal Pierce load capacitors realize the specified CL after equivalent stray capacitance. Compare actual inverter gm with crystal critical gm; startup margin does not check crystal drive power.”

## Handoff and unverified scope

All seven proposed models include exact schema quantities, complete independent nominal vectors, physically meaningful limiting/invalid cases, source equations/limitations and concrete circuit diagrams. No current expansion bus claim overlaps these seven active-loop models. Main should verify the exact count and novelty, then promote all seven together with their diagrams during the later implementation wave.

Parent checks to run after implementation: every output of every complete example; signed failed-startup/failed-margin diagnostic results; exact-zero and relational-domain cases; independent RC ladder complex response and LC pole identities; malformed/invalid-then-valid process recovery; every exact diagram quantity binding; actual native images, polarity/connectivity, captions and narrow/large-text layout.

Skipped by assignment: production code, engine execution, builds, tests, lint, formatting, desktop runs and Git commits. Remaining risk: all are stated closed-form design estimates; real-device tolerances, amplitude regulation, startup robustness and hardware ratings require device-specific validation.
