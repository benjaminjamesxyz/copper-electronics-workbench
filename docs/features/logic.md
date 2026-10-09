# Digital logic and interfaces — executable calculators

These models execute in the shared Rust engine. Flutter provides illustrations and catalog-driven inputs, not a second numerical implementation. Definitions are in [the logic catalog](../../backend/catalog/logic.json); diagrams use the shared primitives. Main owns diagram registration and execution acceptance.

The [research handoff](../../research/expansion/logic.md) retains primary-source access notes and independent derivations. Sources were read during research, not reverified by this implementation. Intel AN42 returned HTTP 403, the Washington lecture required login, and an old EPFL record returned HTTP 405; none supplies claimed evidence here. The accessible author-hosted Network Calculus book supplies the FIFO model.

Every example below enumerates all inputs and outputs with nominal relative tolerance 1e-6. Example inputs are defaults. The engine validates finite inputs, declared bounds and cross-input constraints before sequential outputs. Negative diagnostic margins remain valid. These are engineering estimates, not component qualification, CDC certification or guarantees of hardware reliability. No build, test, formatter, backend process or native UI check is claimed by this implementation.

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

### Illustration and accessible quantity bindings

One 700×280 voltage-recognition diagram; use the shared primitives. Left block “Driver” and right block “Receiver” connect through one signal arrow. Under each block show its own ground reference, not a common-ground wire. Draw an offset arrow from driver ground to receiver ground, with + at receiver and − at driver; bind `ground_offset`.

At the center/right, draw two separated schematic voltage ladders, one for high recognition and one for low recognition. Bound labels: “VOH min” → `output_high_min`, “VIH min” → `input_high_min`, “VOL max” → `output_low_max`, “VIL max” → `input_low_max`; margin callouts “High headroom” → `high_margin`, “Low headroom” → `low_margin`. Show adverse high-noise arrow downward and low-noise arrow upward; label those arrows with `high_noise` and `low_noise`. These arrows represent reference shifts and noise budgets, not actual time-domain waveforms. Reserve label boxes outside ladder lines.

Caption: “Guaranteed recognition limits, not switching thresholds. Receiver ground offset subtracts from the received voltage. Static ladders are illustrative; negative reported headroom means an incompatible corner.”

### Catalog input units and bounds

| Key | Unit | Default | Bounds |
|---|---|---|---|
| `output_high_min` | V | `2.4` | >= 0 |
| `output_low_max` | V | `0.4` | >= 0 |
| `input_high_min` | V | `2.0` | >= 0 |
| `input_low_max` | V | `0.8` | >= 0 |
| `ground_offset` | V | `0.1` | Finite, signed |
| `high_noise` | V | `0.1` | >= 0 |
| `low_noise` | V | `0.1` | >= 0 |

Cross-input constraints:
- `output_high_min > output_low_max` — Driver high guarantee must exceed its low guarantee.
- `input_high_min > input_low_max` — Receiver high threshold must exceed its low threshold.

### Sources

- [Texas Instruments SZZA036C, section 5 and Figure 50: logic compatibility](https://www.ti.com/lit/an/szza036c/szza036c.pdf)
- [Texas Instruments SCAA029, Figure 23: DC noise margin](https://www.ti.com/lit/pdf/scaa029)

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

### Illustration and accessible quantity bindings

One 700×280 current-distribution schematic. Place driver block at left, a branched net through explicit junction dots at center, and three representative receiver blocks at right. Label the repeated receiver group “N identical inputs” with quantity `load_count`; the three drawn blocks are representatives, not a literal count.

Show two separate arrow annotations above/below the same topology: high-state source current flows driver→receivers; low-state input current flows receivers→driver. Bind “Source limit” → `source_limit`, “Sink limit” → `sink_limit`, “Each IIH” → `receiver_high_current`, “Each |IIL|” → `receiver_low_current`. Put the output callout “DC Nmax” → `maximum_fanout` below the branch, clear of wires. Use no resistor icons on the receiver inputs: the current requirements are datasheet bounds, not ohmic loads.

Caption: “High and low states are separate cases. Currents are magnitudes; low-state bipolar inputs feed the output sink. DC current fanout does not guarantee acceptable edge rate.”

### Catalog input units and bounds

| Key | Unit | Default | Bounds |
|---|---|---|---|
| `source_limit` | A | `0.004` | >= 0 |
| `sink_limit` | A | `0.008` | >= 0 |
| `receiver_high_current` | A | `2e-05` | > 0 |
| `receiver_low_current` | A | `0.0004` | > 0 |
| `load_count` | count | `18` | >= 0 |

Cross-input constraints:
- `load_count == floor(load_count)` — Receiver count must be a whole number.

### Sources

- [Texas Instruments SZZA036C, sections 3.3, 4.6.12-4.6.14 and 5: terminal signs, input currents and loading](https://www.ti.com/lit/an/szza036c/szza036c.pdf)
- [Texas Instruments SCEA046B, Slow or Noisy Edges: heavily loaded output edge-rate limitation](https://www.ti.com/lit/pdf/scea046)

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

### Illustration and accessible quantity bindings

One 700×280 energy-flow diagram. A top supply arrow enters a central block “N identical CMOS sections”. Bind “VCC” → `supply_voltage`, “N” → `active_sections`, “Input cycles” → `input_cycle_rate`. Inside, show a dashed explanatory equivalent-capacitance label “Internal Cpd” → `power_capacitance`; do not draw Cpd as a real externally wired capacitor. Draw one representative physical output capacitor from output node to ground; bind `load_capacitance`. Label the output path “Output cycles” → `output_cycle_rate`.

Below the central block, three separate accounting arrows lead to callouts: “Quiescent” → `static_power`, “Internal switching” → `internal_power`, “Load cycling” → `load_power`. Keep arrows as energy accounting, not three electrically paralleled voltage sources. A final right-side callout binds `total_power`. Place `average_current` on the supply arrow only if label spacing permits.

Caption: “Cpd is a measured internal power equivalent. Output capacitance is charged once per output cycle; quiescent package current is added once. Supply-power budget, not a thermal simulation.”

### Catalog input units and bounds

| Key | Unit | Default | Bounds |
|---|---|---|---|
| `supply_voltage` | V | `3.3` | > 0 |
| `quiescent_current` | A | `2e-05` | >= 0 |
| `active_sections` | count | `8` | >= 1 |
| `power_capacitance` | F | `1.5e-11` | >= 0 |
| `input_cycle_rate` | Hz | `20000000` | >= 0 |
| `load_capacitance` | F | `2.5e-11` | >= 0 |
| `output_cycle_rate` | Hz | `10000000` | >= 0 |

Cross-input constraints:
- `active_sections == floor(active_sections)` — Active section count must be a whole number.

### Sources

- [Texas Instruments SCAA035B, equations 3-10 and Cpd testing considerations](https://www.ti.com/lit/pdf/scaa035)

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

### Illustration and accessible quantity bindings

One 700×280 synchronizer/regeneration diagram. Draw asynchronous input arrow into first D/Q register block, then an isolated short interstage wire into a second D/Q register block, then a valid-domain output arrow. Connect the same destination clock to both clock pins through an explicit branched net with junction dots. Label asynchronous arrow `data_transition_rate`; destination clock `clock_frequency`. Mark the first Q node “May be metastable”; do not label it safe or draw multiple independent bus-bit paths.

Below the first Q and second sampling edge, draw an illustrative shrinking exponential envelope with a horizontal time arrow. Bind `resolution_time` to the available interval, `resolution_constant` to a separate “Regeneration tau” label, and `required_resolution_time` to a separate required-time callout. Put output `log10_system_mtbf` and input `chain_count` in a right-side reliability callout. `aperture_parameter` is labeled “Fit parameter T0”, not a literal drawn aperture width. The two-register chain is representative; the numeric available time is not automatically one full period.

Caption: “First stage may become metastable. Extra regeneration time reduces unresolved-event probability exponentially. Curves are illustrative; fit constants and routed timing determine the estimate. Independent chains are not a coherent bus synchronizer.”

### Catalog input units and bounds

| Key | Unit | Default | Bounds |
|---|---|---|---|
| `clock_frequency` | Hz | `100000000` | > 0 |
| `data_transition_rate` | 1/s | `1000000` | > 0 |
| `aperture_parameter` | s | `1e-10` | > 0 |
| `resolution_constant` | s | `5e-11` | > 0 |
| `resolution_time` | s | `2e-09` | >= 0 |
| `chain_count` | count | `100` | >= 1 |
| `target_system_mtbf` | s | `31557600000` | > 0 |

Cross-input constraints:
- `chain_count == floor(chain_count)` — Synchronizer chain count must be a whole number.
- `aperture_parameter < 1/data_transition_rate` — The aperture-times-transition-rate factor must be below one; rare-event characterization must also be valid.

### Sources

- [Stanford EE183, Ryan Donohue, Synchronization in Digital Logic Circuits: MTBF and synchronizer requirements, pages 1-4](https://web.stanford.edu/class/ee183/handouts_spr2003/synchronization_pres.pdf)

