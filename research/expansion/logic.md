# Expansion lane: digital logic and interfaces

Exactly six additional models: two basic, two intermediate and two expert. Definitions are in `research/expansion/logic.json`. No existing catalog, production code, registry or board is changed by this research.

## Scope and evidence

Read `coordination/EXPANSION_CONTRACT.md`, `coordination/CONTRACT.md`, the expansion allocation in `RESEARCH_BOARD.md`, the actual 128-item baseline formula inventory, and the expansion communication bus before selecting these models. Scope is guaranteed logic recognition, current loading, CMOS package switching power, metastability reliability, gate sizing and hardware FIFO capacity. No I2C pullup, UART mismatch, register setup, divider/timer, Schmitt RC delay, jitter, analog filter, passive RC transient or interface termination model is counted.

Primary materials actually retrieved and read:

1. Texas Instruments, [SZZA036C, Understanding and Interpreting Standard-Logic Data Sheets](https://www.ti.com/lit/an/szza036c/szza036c.pdf), manufacturer application report. Sections 3.3 and 3.5 distinguish absolute ratings and operating guarantees; sections 4.6.12–4.6.14 explain input current and its signs; section 5 and Figure 50 explain guaranteed logic compatibility and noise margins.
2. Texas Instruments, [SCAA029, Understanding Advanced Bus-Interface Products](https://www.ti.com/lit/pdf/scaa029), manufacturer application report. Figure 1 gives guaranteed logic ranges. The simultaneous-switching discussion and Figure 23 explain high and low noise margins and ground-reference disturbances.
3. Texas Instruments, [SCEA046B, Understanding Schmitt Triggers](https://www.ti.com/lit/pdf/scea046), manufacturer application brief, “Slow or Noisy Edges.” Read only as evidence that DC fanout does not establish acceptable input rise/fall time. No Schmitt calculation is proposed.
4. Texas Instruments, [SCAA035B, CMOS Power Consumption and Cpd Calculation](https://www.ti.com/lit/pdf/scaa035), manufacturer application report, printed pages 3–6, equations 3–10 and testing considerations. Distinguishes quiescent, internal switching and external load power; explains input/output frequency and Cpd characterization.
5. Stanford EE183, Ryan Donohue, [Synchronization in Digital Logic Circuits](https://web.stanford.edu/class/ee183/handouts_spr2003/synchronization_pres.pdf), university-hosted teaching material. Pages 1–4 present the exponential MTBF law and synchronizer limitations; pages 5–7 describe dual-port FIFO storage, pointers and conservative synchronization. The extracted equation reads MTBF = exp(tr/tau)/(T0*f*a). It is not a universal device characterization.
6. David Harris, [Logical Effort: Designing for Speed on the Back of an Envelope](https://bears.ece.ucsb.edu/class/ece224a/Harris_logical_effort.pdf), university-hosted lecture. Pages 6–9 define d = gh+p, logical/electrical effort and static CMOS effort values; pages 13–16 derive equal-stage-effort minimum delay and gate capacitance sizing.
7. Jean-Yves Le Boudec and Patrick Thiran, [Network Calculus](https://leboudec.github.io/netcal/latex/netCalBook.pdf), author-hosted university textbook, section 1.4.1, Theorems 1.4.1 and 1.4.2, Figure 1.10. Read the backlog and delay bounds and their token-bucket/rate-latency specialization: B = b+rT and d = T+b/R for r ≤ R.

Search boundaries and gaps: Intel AN42 at `https://cdrdv2-public.intel.com/653636/an042.pdf` returned HTTP 403; it is not claimed as read evidence. The Washington CSEP567 lecture redirected to institutional login; it is not cited as evidence. The old EPFL book-record URL returned HTTP 405; the author's book URL above was accessible. TI's SCAA126 HTML route returned binary PDF bytes; it was not used. An EPFL thesis and IEEE 802.1 workshop presentation corroborated network-calculus terminology, but the directly readable author book supplies the actual FIFO equations. Sources establish models, not current part-specific constants. Datasheet, characterized-cell and workload/service inputs remain the designer's responsibility.

Independent arithmetic used decimal hand substitutions and Python Decimal evaluation, not the Rust engine. JSON examples enumerate every input and every output. No builds, tests, lint, formatters, backend process runs or UI checks were performed.

## 1. `logic.guaranteed_noise_margins` — basic

### Purpose and novelty

Check whether a guaranteed output-high/low pair remains recognizable after ground offset and worst-direction line noise. Nearest baseline: `timing.schmitt_rc_delay_bounds`, which predicts threshold-crossing time through an RC network. This model instead compares static guaranteed voltage regions between two ICs; it has no capacitor, RC time or switching threshold prediction. Nearest new model: current fanout below; valid voltage margins do not establish available drive current and vice versa.

### Model and limits

From TI SZZA036C section 5/Figure 50 and SCAA029 Figure 23, the basic margins are VOH−VIH and VIL−VOL. Define ΔG = receiver ground minus driver ground. A driver-referenced voltage becomes V−ΔG at the receiver. Positive ΔG therefore hurts the high state and helps the low state:

- `high_margin = output_high_min - ground_offset - input_high_min - high_noise`.
- `low_margin = input_low_max - output_low_max + ground_offset - low_noise`.
- `minimum_margin = min(high_margin, low_margin)`.

All quantities are volts. Both noise inputs are nonnegative worst-direction magnitudes, not RMS random-noise values. `ground_offset` is signed. Output and input high/low pairs must each be ordered; negative calculated margins are accepted diagnostic results, not input errors. Zero means no remaining allowance.

Use guaranteed limits at compatible supply, temperature and load corners. A positive recognition margin does not establish safe overvoltage, undershoot, power-off behavior, fanout or timing. No generic family table substitutes for a real part's guarantees. This is positive single-ended logic; differential receivers require a different model.

### Complete independent example

Inputs: `output_high_min=2.4 V`, `output_low_max=0.4 V`, `input_high_min=2 V`, `input_low_max=0.8 V`, `ground_offset=0.1 V`, `high_noise=0.1 V`, `low_noise=0.1 V`.

- `high_margin = 2.4−0.1−2−0.1 = 0.2 V`.
- `low_margin = 0.8−0.4+0.1−0.1 = 0.4 V`.
- `minimum_margin = min(0.2,0.4) = 0.2 V`.

### Independent test cases

- Nominal: every result above, relative tolerance 1e-6.
- Equal references and no noise: set offset and both noise inputs to zero; both margins and minimum are 0.4 V.
- Ground-sign check: retain nominal noise and set offset to −0.1 V; high=0.4 V, low=0.2 V, minimum=0.2 V.
- Exact zero boundary with binary-exact values: VOH=2.5, VOL=0.5, VIH=2, VIL=1, offset=0.25, high_noise=0.25, low_noise=0.25 gives high=0, low=0.5, minimum=0 V.
- Incompatible but valid: nominal inputs with offset=0.4 V give high=−0.1 V, low=0.7 V and minimum=−0.1 V. Do not clamp or reject negative results.
- Invalid: negative noise magnitude, negative guaranteed level, VOH≤VOL, or VIH≤VIL. Reject nonfinite inputs through the existing engine boundary.

### Mandatory diagram specification

One 700×280 voltage-recognition diagram; use the shared primitives. Left block “Driver” and right block “Receiver” connect through one signal arrow. Under each block show its own ground reference, not a common-ground wire. Draw an offset arrow from driver ground to receiver ground, with + at receiver and − at driver; bind `ground_offset`.

At the center/right, draw two separated schematic voltage ladders, one for high recognition and one for low recognition. Bound labels: “VOH min” → `output_high_min`, “VIH min” → `input_high_min`, “VOL max” → `output_low_max`, “VIL max” → `input_low_max`; margin callouts “High headroom” → `high_margin`, “Low headroom” → `low_margin`. Show adverse high-noise arrow downward and low-noise arrow upward; label those arrows with `high_noise` and `low_noise`. These arrows represent reference shifts and noise budgets, not actual time-domain waveforms. Reserve label boxes outside ladder lines.

Caption: “Guaranteed recognition limits, not switching thresholds. Receiver ground offset subtracts from the received voltage. Static ladders are illustrative; negative reported headroom means an incompatible corner.”

## 2. `logic.dc_current_fanout` — basic

### Purpose and novelty

Determine the number of input pins one output can drive while preserving the output's characterized high/low voltage guarantees. Nearest baseline: `timing.i2c_pullup_bounds`, which solves an open-drain resistor/edge-time design window. Here a push-pull output must meet two static input-current sums; no pullup resistance or rise-time equation is used. This is not `dc.ohm_current`: there is no assumed resistive input model, and bipolar high/low input currents can differ greatly.

### Model and limits

TI SZZA036C sections 4.6.12–4.6.14 define II, IIH and IIL and distinguish CMOS leakage from bipolar input current. Its current-sign convention defines current entering the pin as positive; a low bipolar input normally sources current toward the driving sink. Enter positive magnitudes for both receiver currents and both driver capabilities.

Conservation of current gives N·IIH≤Isource and N·|IIL|≤Isink. Therefore:

- `high_fanout = floor(source_limit/receiver_high_current)`.
- `low_fanout = floor(sink_limit/receiver_low_current)`.
- `maximum_fanout = min(high_fanout,low_fanout)`.
- `source_headroom = source_limit - load_count*receiver_high_current`.
- `sink_headroom = sink_limit - load_count*receiver_low_current`.

Currents are amperes; counts are dimensionless whole numbers. `load_count` can be zero and may exceed the computed fanout: negative current headroom is useful. Driver limits can be zero. Input current magnitudes must be positive because ideal zero-current loading has no finite DC-current fanout.

Use operating drive guarantees at the required VOH/VOL, not destructive absolute-maximum ratings. Currents from external pulls and bus-hold circuits are not silently included. TI SCEA046B warns that heavy output loading can violate the next gate's rise/fall specification; this calculator deliberately does not claim AC fanout. Separately evaluate capacitance, package current and voltage margins.

### Complete independent example

Inputs: `source_limit=0.004 A`, `sink_limit=0.008 A`, `receiver_high_current=0.00002 A`, `receiver_low_current=0.0004 A`, `load_count=18`.

- `high_fanout = floor(0.004/0.00002) = 200`.
- `low_fanout = floor(0.008/0.0004) = 20`.
- `maximum_fanout = min(200,20) = 20`.
- `source_headroom = 0.004−18×0.00002 = 0.00364 A`.
- `sink_headroom = 0.008−18×0.0004 = 0.0008 A`.

### Independent test cases

- Nominal: all five outputs above.
- Fractional ratio must round downward: source=0.0039 A, receiver_high=0.0002 A, retain nominal sink/low input, loads=18; high=19, low=20, maximum=19, source headroom=0.0003 A, sink headroom=0.0008 A.
- Exactly exhausted low drive: nominal with loads=20; source headroom=0.0036 A, sink headroom=0; fanout outputs unchanged.
- Overloaded but valid: nominal with loads=21; source headroom=0.00358 A, sink headroom=−0.0004 A. Preserve the sign.
- Zero capabilities and zero loads: source=sink=0, positive receiver currents; all five outputs are zero.
- Invalid: either receiver current zero/negative, either driver current negative, fractional/negative load count, or nonfinite input. Tests near integer ratios should use binary-exact currents where exact floor placement matters.

### Mandatory diagram specification

One 700×280 current-distribution schematic. Place driver block at left, a branched net through explicit junction dots at center, and three representative receiver blocks at right. Label the repeated receiver group “N identical inputs” with quantity `load_count`; the three drawn blocks are representatives, not a literal count.

Show two separate arrow annotations above/below the same topology: high-state source current flows driver→receivers; low-state input current flows receivers→driver. Bind “Source limit” → `source_limit`, “Sink limit” → `sink_limit`, “Each IIH” → `receiver_high_current`, “Each |IIL|” → `receiver_low_current`. Put the output callout “DC Nmax” → `maximum_fanout` below the branch, clear of wires. Use no resistor icons on the receiver inputs: the current requirements are datasheet bounds, not ohmic loads.

Caption: “High and low states are separate cases. Currents are magnitudes; low-state bipolar inputs feed the output sink. DC current fanout does not guarantee acceptable edge rate.”

## 3. `logic.cmos_package_power` — intermediate

### Purpose and novelty

Budget the supply demand of CMOS logic using the manufacturer's internal Cpd characterization plus independently switched output loads. Nearest baseline: `mosfet.gate_charge_supply` predicts external gate-drive charge delivery for a power transistor; `mosfet.output_capacitance_hard_turn_on_loss` predicts dissipated Coss energy during hard turn-on. This model instead includes CMOS internal-node/through-current characterization, active logic-section multiplicity, whole-package leakage and different input/output rates. Neither prior model represents this package operating-power budget. It is not converter efficiency or a thermal solver.

### Model and limits

TI SCAA035B equations 3–10 directly supply:

- `static_power = supply_voltage * quiescent_current`.
- `internal_power = active_sections * power_capacitance * input_cycle_rate * supply_voltage^2`.
- `load_power = active_sections * load_capacitance * output_cycle_rate * supply_voltage^2`.
- `total_power = static_power + internal_power + load_power`.
- `average_current = total_power/supply_voltage`.

Voltage in V, current in A, capacitance in F, rates in cycles/s (Hz), power in W. There is no factor of 1/2 in full-cycle supply energy C·V²: half is dissipated while charging and the stored half is dissipated while discharging. Count low-to-high charging events, not both signal edges.

`power_capacitance` is Cpd **per active section**, not pin capacitance or a package Cpd copied without conversion. Add whole-package leakage only once. Cpd includes internal node charging and transistor through-current under its characterized input slew and pattern. The supplied numeric values illustrate a hypothetical eight-section device and do not claim a particular IC's rating. Slow input edges, intermediate logic levels, resistive loads and large frequency-dependent amplitude loss need additional modeling. Supply power is not always identical to package heat, especially with external series damping or termination.

Zero current, capacitance or activity is valid. Supply must remain positive; active section count is a positive integer. Actual voltage/slew/frequency limits come from the chosen device, not arbitrary catalog bounds.

### Complete independent example

Inputs: `supply_voltage=3.3 V`, `quiescent_current=20e-6 A`, `active_sections=8`, `power_capacitance=15e-12 F`, `input_cycle_rate=20e6 Hz`, `load_capacitance=25e-12 F`, `output_cycle_rate=10e6 Hz`.

Using V²=10.89:

- `static_power = 3.3×20e-6 = 0.000066 W`.
- `internal_power = 8×15e-12×20e6×10.89 = 0.026136 W`.
- `load_power = 8×25e-12×10e6×10.89 = 0.02178 W`.
- `total_power = 0.000066+0.026136+0.02178 = 0.047982 W`.
- `average_current = 0.047982/3.3 = 0.01454 A`.

### Independent test cases

- Nominal: every output above.
- Static limit: both rates zero; static=total=66e-6 W, internal=load=0, average current=20e-6 A.
- No output switching: set output rate zero only; total=0.026202 W, average current=0.00794 A, load=0; internal/static unchanged.
- No load and no quiescent current: CL=Iq=0; total=internal=0.026136 W, average current=0.00792 A; static/load=0.
- Doubling both rates doubles internal/load power, not leakage. Doubling supply at fixed characterized parameters multiplies dynamic terms by four and leakage power by two; this is an algebraic sensitivity test, not permission to exceed real device voltage limits.
- Invalid: zero/negative supply, negative rates/current/capacitance, zero or fractional active sections, nonfinite inputs.

### Mandatory diagram specification

One 700×280 energy-flow diagram. A top supply arrow enters a central block “N identical CMOS sections”. Bind “VCC” → `supply_voltage`, “N” → `active_sections`, “Input cycles” → `input_cycle_rate`. Inside, show a dashed explanatory equivalent-capacitance label “Internal Cpd” → `power_capacitance`; do not draw Cpd as a real externally wired capacitor. Draw one representative physical output capacitor from output node to ground; bind `load_capacitance`. Label the output path “Output cycles” → `output_cycle_rate`.

Below the central block, three separate accounting arrows lead to callouts: “Quiescent” → `static_power`, “Internal switching” → `internal_power`, “Load cycling” → `load_power`. Keep arrows as energy accounting, not three electrically paralleled voltage sources. A final right-side callout binds `total_power`. Place `average_current` on the supply arrow only if label spacing permits.

Caption: “Cpd is a measured internal power equivalent. Output capacitance is charged once per output cycle; quiescent package current is added once. Supply-power budget, not a thermal simulation.”

## 4. `logic.synchronizer_metastability` — expert

### Purpose and novelty

Quantify the exponential dependence of asynchronous sampling reliability on available regeneration time. Nearest baseline: `timing.synchronous_setup_slack` checks deterministic synchronous arrival timing, whereas this model assumes asynchronous aperture encounters and characterizes unresolved metastability statistically. `thermal.arrhenius_acceleration` is another exponential model but describes temperature-activated wear, not bistable regeneration. No clock jitter, sampling-aperture SNR or measurement model is claimed.

### Model and limits

The Stanford lecture gives MTBF = exp(tres/tau)/(T0·fclock·fdata). T0 and tau both have units seconds; the denominator has units 1/s. T0 is a characterized fit parameter, not automatically the setup-plus-hold interval.

The model reports logarithms in reference seconds:

- `log10_chain_mtbf = (resolution_time/resolution_constant - ln(aperture_parameter) - ln(clock_frequency) - ln(data_transition_rate))/ln(10)`.
- `log10_system_mtbf = log10_chain_mtbf - log10(chain_count)`.
- `required_resolution_time = max(0, resolution_constant * [ln(target_system_mtbf)+ln(chain_count)+ln(aperture_parameter)+ln(clock_frequency)+ln(data_transition_rate)])`.
- `resolution_margin = resolution_time - required_resolution_time`.

Separate logarithms avoid multiplying tiny T0 by large rates, and avoid exponentiating an enormous MTBF. The dimensionless outputs explicitly mean log10(MTBF/1 s), not seconds or decibels.

The identical-chain extension is derived here: independent rare Poisson failure rates sum, so lambda_system=N·lambda_chain and MTBF_system=MTBF_chain/N. This is not a quoted additional device law. Required time follows by solving the same inequality for tres, limited below by zero because available physical resolution time cannot be negative.

Use asynchronous transitions/s, not waveform periods/s. Characterization must cover process, voltage, temperature, fit time origin and the selected cell. Input `resolution_time` is the effective fitted regeneration interval from actual implementation, not an unqualified clock period. The model accepts zero time mathematically, but actual short-time extrapolation needs characterization. Require all logarithm operands positive, integral N≥1, tres≥0, and T0·fdata<1; practical rare-event use requires that last product much smaller than one. This mathematical guard is not proof of fit validity.

Repeated sampling does not guarantee pulse capture. Independently synchronizing each bus bit does not guarantee coherent data. Common-mode failure mechanisms violate the independent-chain assumption. Defaults below are illustrative fit constants, not a promised MTBF for any FPGA or ASIC.

### Complete independent example

Inputs: `clock_frequency=1e8 Hz`, `data_transition_rate=1e6 1/s`, `aperture_parameter=1e-10 s`, `resolution_constant=5e-11 s`, `resolution_time=2e-9 s`, `chain_count=100`, `target_system_mtbf=31557600000 s` (1000 Julian years).

The exponent is 2e-9/5e-11=40 and T0·fclock·fdata=10000 s⁻¹.

- `log10_chain_mtbf = 40/ln(10)−4 = 13.371779276130073`.
- `log10_system_mtbf = 13.371779276130073−2 = 11.371779276130073`.
- For the target, N·T0·fclock·fdata·target = 3.15576e16; `required_resolution_time = 5e-11*ln(3.15576e16) = 1.8995295421184684e-9 s`.
- `resolution_margin = 2e-9−1.8995295421184684e-9 = 1.0047045788153165e-10 s`.

For interpretation only, chain MTBF is exp(40)/10000 seconds. It is intentionally not an additional exponential catalog output.

### Independent test cases

- Nominal: all four outputs above.
- Shorter resolution: nominal with tres=1e-9 s gives log chain=4.6858896380650366, log system=2.6858896380650366, required time unchanged, margin=−8.995295421184684e-10 s. Negative margin must remain visible.
- N=1 raises system log MTBF by exactly two relative to nominal and lowers required time by tau·ln(100).
- Tiny target: nominal with target=1e-8 s gives required time=0 and margin=2e-9 s; MTBF outputs are unchanged.
- Zero resolution: default rates/fit/N give chain log=−4 and system log=−6; required time remains nominal and margin is its negative. This checks mathematical boundary behavior, not device accuracy there.
- Large exponent: tres=1e-6 s gives chain log=8681.8896380650366; it remains finite without exp(20000). The result is an extreme mathematical extrapolation only.
- Invalid: zero/negative rate, T0, tau or target; negative resolution; N<1 or fractional; T0=1e-6 s with fdata=1e6 1/s reaches the disallowed aperture-factor equality. Nonfinite input must fail before evaluation.

### Mandatory diagram specification

One 700×280 synchronizer/regeneration diagram. Draw asynchronous input arrow into first D/Q register block, then an isolated short interstage wire into a second D/Q register block, then a valid-domain output arrow. Connect the same destination clock to both clock pins through an explicit branched net with junction dots. Label asynchronous arrow `data_transition_rate`; destination clock `clock_frequency`. Mark the first Q node “May be metastable”; do not label it safe or draw multiple independent bus-bit paths.

Below the first Q and second sampling edge, draw an illustrative shrinking exponential envelope with a horizontal time arrow. Bind `resolution_time` to the available interval, `resolution_constant` to a separate “Regeneration tau” label, and `required_resolution_time` to a separate required-time callout. Put output `log10_system_mtbf` and input `chain_count` in a right-side reliability callout. `aperture_parameter` is labeled “Fit parameter T0”, not a literal drawn aperture width. The two-register chain is representative; the numeric available time is not automatically one full period.

Caption: “First stage may become metastable. Extra regeneration time reduces unresolved-event probability exponentially. Curves are illustrative; fit constants and routed timing determine the estimate. Independent chains are not a coherent bus synchronizer.”

## 5. `logic.three_stage_logical_effort` — expert

### Purpose and novelty

Choose two internal gate sizes of a fixed three-stage CMOS path under endpoint capacitance constraints. Nearest baseline: `opamps.dominant_pole_gain_bandwidth` models analog closed-loop bandwidth; `timing.synchronous_setup_slack` consumes a supplied path delay. Neither optimizes CMOS device sizing under gate-topology logical effort. Neighbor `logic.dc_current_fanout` deals with static input current, not capacitance ratios or propagation delay. Oscillator stage-period calculations, if present elsewhere, are not the endpoint-constrained gate-sizing problem here.

### Model and limits

Harris pages 6–9 define normalized stage delay d_i=g_i·h_i+p_i, with electrical effort h_i=Cout_i/Cin_i. Pages 13–16 show that a fixed N-stage path minimizes summed effort delay when each stage effort equals F^(1/N), where F is the product of efforts.

For an unbranched three-stage path, C1 and CL are fixed, while C2 and C3 are free:

- F=g1·g2·g3·CL/C1.
- `optimal_stage_effort = exp((ln(g1)+ln(g2)+ln(g3)+ln(CL)-ln(C1))/3)`.
- `second_input_capacitance = C1*optimal_stage_effort/g1`.
- `third_input_capacitance = CL*g3/optimal_stage_effort`.
- `normalized_path_delay = 3*optimal_stage_effort + path_parasitic_delay`.
- `path_delay = normalized_path_delay * process_delay_unit`.

The log-sum expression avoids unnecessary overflow in the full F product. The first and last sizing equations produce the same middle-stage effort by conservation of the endpoint capacitance ratio. Capacitances are F, efforts and normalized delays dimensionless, delay unit and final delay seconds. `path_parasitic_delay` is p1+p2+p3, not a dimensional time.

Exactly three stages, no side branches, no significant wire capacitance, continuously scalable gates, fixed topology and size-independent normalized parasitics. Ordinary static CMOS g≥1 is the explicit domain; exotic dynamic/asymmetric logical-effort variants are not included. Actual gate size floors and discrete libraries can prevent the computed optimum. The calculation cannot validate slew, polarity, functionality, standard-cell availability or PVT. It also does not use the approximate “stage effort four” rule to pretend to solve optimum stage count.

### Complete independent example

Inputs: `first_input_capacitance=3e-15 F`, `final_load_capacitance=144e-15 F`, `first_logical_effort=4/3` (JSON binary64 literal 1.3333333333333333), `second_logical_effort=1`, `third_logical_effort=1`, `path_parasitic_delay=4`, `process_delay_unit=20e-12 s`.

This corresponds to an ordinary NAND2→inverter→inverter path for one sensitized input, with textbook parasitic sum 2+1+1=4.

- F=(4/3)×1×1×144/3=64, hence `optimal_stage_effort=4`.
- `second_input_capacitance = 3e-15×4/(4/3) = 9e-15 F`.
- `third_input_capacitance = 144e-15×1/4 = 36e-15 F`.
- Middle effort: g2·C3/C2=36/9=4, independently confirming equal effort.
- `normalized_path_delay = 3×4+4 = 16`.
- `path_delay = 16×20e-12 = 3.2e-10 s`.

### Independent test cases

- Nominal: all five outputs above.
- All-inverter alternate: C1=1e-15 F, CL=64e-15 F, all g=1, P=3, tau=1e-11 s; f=4, C2=4e-15 F, C3=16e-15 F, D=15, delay=1.5e-10 s.
- Scale both endpoint capacitances by 1000: internal capacitances also scale by 1000, while effort and both delays remain nominal.
- Equal loads: C1=CL=1e-15 F, all g=1, P=3 and tau=1e-11 s gives f=1, C2=C3=1e-15 F, D=6 and delay=6e-11 s.
- Ideal parasitic boundary P=0 is accepted; nominal effort/sizes remain, D=12 and delay=2.4e-10 s.
- Independent minimization check: perturb either internal capacitance in the alternate all-inverter example and sum g_i·h_i+p_i directly. The sum cannot fall below 15. This tests the physical objective, not a snapshot of the same implementation expression.
- Invalid: endpoint C≤0, process unit≤0, negative P or any g<1. Nonfinite quantities fail at engine input validation.

### Mandatory diagram specification

One 700×280 three-stage path diagram. Draw three generic gate blocks in a left-to-right unbranched chain; label “Gate 1”, “Gate 2”, “Gate 3”, rather than hard-coding NAND symbols for all possible input g values. Bound labels above gates: `first_logical_effort`, `second_logical_effort`, `third_logical_effort`. Under each gate's input pin, place its input-capacitance callout: `first_input_capacitance`, `second_input_capacitance`, `third_input_capacitance`. Draw only the actual final load capacitor to reference ground and bind `final_load_capacitance`.

Below the chain, an arrow spanning its full length binds `path_delay`; a separate callout “Equal stage effort” binds `optimal_stage_effort`. Input capacitance callouts describe the respective gate pins and must not be drawn as extra parallel loads added on top of those gate inputs. Keep generic internal capacitance drawings out of the electrical path.

Caption: “Fixed three-stage, unbranched topology. Equal logical stage effort minimizes this linear delay estimate. Input capacitances are gate sizes, not extra discrete capacitors; layout and cell-library timing remain separate.”

## 6. `logic.fifo_envelope_depth` — intermediate

### Purpose and novelty

Size actual on-chip FIFO data storage for a digital interface with bounded burst traffic and delayed guaranteed read service. Nearest baseline: `timing.uart_sampling_margin` checks receiver bit-sampling alignment; timer division checks a generated rate. Neither predicts queue occupancy or memory capacity. New metastability reliability evaluates whether CDC sampling fails; this model instead budgets the backlog created by otherwise correct clock-domain and arbitration latency. It adds a hardware storage design problem, not a different protocol name for a timing-margin calculation.

### Model and limits

Le Boudec–Thiran section 1.4.1 defines backlog as the maximum vertical deviation between arrival and service curves. Figure 1.10 gives the specialization for:

- arrival envelope alpha(t)=b+r·t for every interval of positive duration;
- service curve beta(t)=R·max(0,t−T);
- r≤R.

Before T, alpha−beta rises to b+rT. After T its slope is r−R≤0. Thus the maximum is B=b+rT. The horizontal-deviation delay bound is T+b/R.

Catalog outputs:

- `backlog_bound = burst_words + arrival_rate*service_latency` in words;
- `usable_depth = ceil(backlog_bound)` in whole words;
- `storage_bits = usable_depth*word_bits` in data bits;
- `delay_bound = service_latency + burst_words/service_rate` in seconds.

The result is a conservative sufficient capacity, not a claim that a particular discrete clock schedule needs exactly this many entries. Burst and word width are positive whole numbers. Sustained rate may be zero. Guaranteed service is positive and may equal arrival rate. If R<r, perpetual worst-case backlog is unbounded, so reject rather than provide a misleading finite depth.

For this hardware specialization, the service latency must include at least one word-service interval 1/R. A one-word-at-a-time read interface with arbitrary phase cannot guarantee completed-word fluid service from t=0. Include that phase allowance plus worst-case CDC/empty-flag visibility, arbitration and service interruptions in the guaranteed curve. This does not require inventing a universal number of synchronizer stages. T is a service-curve latency valid across busy periods, not just a one-time boot stall or an average observed delay.

Stanford pages 5–7 establish the hardware context: dual-port storage, clock-domain pointers and conservative status. Capacity is **usable** data words. A FIFO implementation that reserves an entry, rounds to a memory primitive, stores ECC or imposes conservative full-flag thresholds must add those implementation costs. No Gray-pointer or synchronizer correctness is certified. A source violating b+rt or a reader with unbounded stalls invalidates the bound. “Average write clock less than read clock” alone is insufficient.

### Complete independent example

Inputs: `burst_words=64`, `arrival_rate=25e6 word/s`, `service_rate=50e6 word/s`, `service_latency=1e-6 s`, `word_bits=32 bit/word`.

R≥r and T≥1/R=20e-9 s.

- `backlog_bound = 64 + 25e6×1e-6 = 89 words`.
- `usable_depth = ceil(89) = 89 words`.
- `storage_bits = 89×32 = 2848 bits`.
- `delay_bound = 1e-6+64/50e6 = 2.28e-6 s`.

89 usable words are the envelope-based data capacity; a chosen 128-word memory primitive is an implementation choice, not another model output.

### Independent test cases

- Nominal: all four outputs above.
- Equal-rate limit: set arrival rate=service rate=50e6 word/s; backlog=depth=114 words, bits=3648, delay=2.28e-6 s. The backlog stays bounded; do not reject equality.
- One finite burst: set arrival rate=0; backlog=depth=64, bits=2048, delay=2.28e-6 s.
- Minimum admitted asynchronous word-phase latency: T=1/50e6=20e-9 s with other nominal inputs gives backlog=64.5, depth=65, bits=2080, delay=1.3e-6 s. The fractional bound rounds upward.
- Binary-exact rounding example: b=3 words, r=2 word/s, R=4 word/s, T=0.75 s, width=8; backlog=4.5, depth=5, bits=40, delay=1.5 s.
- Invalid: R<r; R≤0; T<1/R; fractional/zero/negative burst or word width; negative arrival rate; nonfinite inputs. An invalid envelope in real traffic is outside what numerical validation can detect.

### Mandatory diagram specification

One 700×280 hardware/storage and envelope diagram. Left third: source block “Burst b, sustained r” → FIFO block → reader block “Guaranteed R after T”; label arrows with `arrival_rate` and `service_rate`, and bind `burst_words` to source and `usable_depth` to FIFO. Bind `word_bits` beneath the memory block and `storage_bits` in a separate capacity callout.

Right half: illustrative cumulative-word axes. Show alpha as a line starting at a positive burst intercept, and beta flat until a marked T then rising with slope at least alpha's slope. Bind the T callout to `service_latency`. A vertical brace at T identifies `backlog_bound`; a separate horizontal brace shows the delay concept and binds `delay_bound`. Do not depict actual measured occupancy as alpha or beta. The diagram is a bound construction, not an executed discrete-event trace. If this layout is too dense, retain source/FIFO/reader across the upper half and use a single clear envelope plot below, with secondary capacity values in the existing quantity legend.

Caption: “Storage bound from an arrival envelope and guaranteed read-service curve, not average rates. Curves are illustrative; T includes whole-word phase and bounded interface stalls. Depth is usable data capacity before implementation overhead.”

## Shared illustration and implementation handoff

Every item requires one registered diagram using only `frontend/lib/diagrams/primitives.dart`, exported in the logic lane map. The specifications above name actual JSON keys; derived expressions in text are explanatory, not new quantity keys. Use a 700×280 canvas, at least 16 px outer margins, labels at least 14 px, explicit junctions, no wires through component bodies and no uncomputed live numerical curves. The common renderer/registry remains parent-owned.

No catalog schema extensions, expression-language additions, dependencies or production helpers are required. Expected values come from independent formulas above. Main should have implementers promote the six objects and diagrams, then have independent testers exercise complete vectors, physical boundaries, signed diagnostics and invalid-domain recovery. Main alone runs Rust/desktop verification after the concurrent wave.

Unresolved physical inputs are intentional: real logic guarantees, Cpd conditions, metastability fit constants and effective time origin, gate-delay characterization, and actual FIFO arrival/service guarantees. There is no unresolved cross-lane collision in the bus entries read. These are design estimates, not component qualification, CDC certification or a standards-compliance claim.
