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

