# Digital timing interfaces and EMC — executable calculators

These eight offline numerical models execute in the shared Rust engine; Flutter consumes catalog metadata and does not duplicate calculations. Units are SI unless stated as `%`, dimensionless `1`, or baud (symbols/s). Levels are two basic, three intermediate and three expert. Negative pullup interval width, UART margin and setup slack are intentional failed-design results, not input errors. These are engineering estimates, not simulation or standards-compliance certificates.

The derivations and independently computed nominal examples are retained from [timing research](../../research/timing.md); the executable definitions are [backend/catalog/timing.json](../../backend/catalog/timing.json). Each section lists every input and exact expected output with relative tolerance **1e-6**. Example inputs are also the defaults. Input-only constraints execute before sequential outputs; finite inputs, required keys, bounds and nonfinite intermediate/final arithmetic are enforced by the shared engine.

Sources are primary manufacturer/university material. Research directly read the cited TI, MIT and UMBC sources. Microchip CTC/UART references were available as indexed manufacturer content through web search; direct Microchip HTML retrieval returned HTTP 503 during research. No new direct-source verification or engine/build/test execution is claimed by this implementation.

Boundary distinctions: Schmitt thresholds and intrinsic gate delay distinguish digital RC timing from general analog charging. Timer quantization differs from asynchronous UART drift. Setup slack is not routed trace skew. Loaded-victim capacitive crosstalk differs from dielectric-overlap displacement current, and the specified-slope inductive spike does not predict an RL decay or gate-drive transition.

Numerical limits: physical positive denominators do not guarantee representable floating-point products at arbitrary finite magnitudes; the engine must explicitly reject nonfinite or invalid intermediates. Near-zero differences such as timing slack or threshold windows must be interpreted against hardware uncertainty. At extremely small threshold/supply ratios the Schmitt logarithmic subtraction can lose precision; the illustrative examples are well conditioned. Crosstalk uses native `expm1` to preserve fast-edge physics rather than adding an artificial minimum edge/time-constant ratio. A ratio that actually underflows to zero remains an explicit arithmetic-domain failure, not a physical-domain restriction.

## 3. 16-bit toggle timer clock divider — basic

**ID:** `timing.timer_clock_divider`

**Purpose:** Choose a realizable compare-register value for a supplied hardware prescaler and report actual output frequency and quantization error.

**Derivation:** A timer counts inclusively from 0 through compare value `K`, taking `K+1` prescaled clock ticks. Toggle output changes state once per interval, requiring two intervals for a complete square-wave cycle:

`f_actual = f_clock/[2 P(K+1)]`.

The ideal count is `f_clock/(2 P f_target)`. Choose nearest count, half-integer ties upward:

`count = floor(f_clock/(2 P f_target)+0.5)`; `compare_value = count-1`.

Then `error_percent = 100(f_actual/f_target - 1)` and `output_period = 1/f_actual`.

**Dimensions/domain:** Source/output frequency in Hz, count/compare/prescaler dimensionless, period in s, error in %. Positive source and target frequencies; target cannot exceed `f_clock/(2P)`, so rounded count cannot be zero. Rounded count must be at most 65536 for a 16-bit compare register. The polynomial equality `(P-1)(P-8)(P-64)(P-256)(P-1024)=0` restricts P to the five common AVR-style prescalers without conditional expressions. Verify the chosen timer's actual prescaler availability and toggle mode. This is nearest-period rounding, **not** a claim of mathematically minimum frequency error. Prescaler is supplied, not searched. Clock tolerance and safe register updating are separate design concerns.

**Independent example:** 16 MHz source, P = 8, target 3 kHz:

- Ideal count `= 16e6/(16 × 3000) = 333.333333333`.
- Rounded count `= 333`, compare `= 332`.
- Period `= 2 × 8 × 333 / 16e6 = 333 us`.
- Actual frequency `= 1/333e-6 = 3003.003003003 Hz`.
- Error `= (3003.003003003 - 3000)/3000 × 100 = +0.100100100100%`.

**Primary sources:** [Microchip AVR CTC mode documentation](https://onlinedocs.microchip.com/oxy/GUID-80B1922D-872B-40C8-A8A5-0CBE009FD908-en-US-3/GUID-6CCB4482-FFD4-4AD9-9D5C-DFC302038433.html); [Microchip ATmega640/1280/1281/2560/2561 datasheet](https://ww1.microchip.com/downloads/en/DeviceDoc/ATmega640-1280-1281-2560-2561-Datasheet-DS40002211A.pdf), timer CTC frequency relation. The register-rounding policy is this calculator's explicit design choice.


### Catalog inputs and complete worked outputs

| Input key | Label | Unit | Example/default | Admissible range |
|---|---|---|---|---|
| `clock_hz` | Timer source clock | Hz | `16000000` | > 0 |
| `prescaler` | Hardware prescaler | 1 | `8` | >= 1; <= 1024 |
| `target_hz` | Target square-wave frequency | Hz | `3000` | > 0 |

Cross-input/model constraints (evaluated before outputs):

- `(prescaler-1)*(prescaler-8)*(prescaler-64)*(prescaler-256)*(prescaler-1024) == 0` — Prescaler must be one of 1, 8, 64, 256 or 1024.
- `target_hz <= clock_hz/(2*prescaler)` — Target exceeds the maximum toggle-output frequency.
- `floor(clock_hz/(2*prescaler*target_hz)+0.5) <= 65536` — Rounded count exceeds the 16-bit compare-register range.

| Sequential output key | Label | Unit | Exact independent expected value | Executable expression |
|---|---|---|---|---|
| `count` | Timer clocks per toggle | 1 | `333` | `floor(clock_hz/(2*prescaler*target_hz)+0.5)` |
| `compare_value` | Compare register value | 1 | `332` | `count-1` |
| `actual_hz` | Actual output frequency | Hz | `3003.003003003003` | `clock_hz/(2*prescaler*count)` |
| `error_percent` | Signed frequency error | % | `0.1001001001001001` | `100*(actual_hz/target_hz-1)` |
| `output_period` | Actual output period | s | `0.000333` | `1/actual_hz` |

## 6. Finite-edge lumped capacitive crosstalk — expert

**ID:** `timing.capacitive_crosstalk_peak`

**Purpose:** Estimate the voltage excursion on a resistively held victim node from an adjacent linear voltage ramp. Unlike the bare `R C dv/dt` estimate, this includes victim charge storage and its finite fast-edge limit.

**Topology and derivation:** An ideal aggressor connects to the victim through mutual capacitance C_m. The victim connects to AC ground through resistance R_v and shunt capacitance C_v. Initially the victim excursion is zero. Kirchhoff current balance gives:

`(C_v+C_m) dv_v/dt + v_v/R_v = C_m dv_a/dt`.

For a full aggressor ramp of DeltaV over t_edge, dv_a/dt is constant. Solving the first-order forced equation with zero initial excursion gives:

`v_v(t) = R_v C_m DeltaV/t_edge [1-exp(-t/[R_v(C_v+C_m)])]` during the ramp.

The response grows monotonically, peaks at its end, then decays with `tau = R_v(C_v+C_m)`. Thus:

`peak_excursion = R_v C_m DeltaV/t_edge [1-exp(-t_edge/tau)]`.

For t_edge much shorter than tau, the limiting peak is the capacitive divider `C_m DeltaV/(C_m+C_v)`. Including this comparison output helps identify why a pure resistive estimate overpredicts short-edge noise.

**Stable executable form:** Let `x = edge_time/tau` and `V_divider = C_m DeltaV/(C_m+C_v)`. Algebraically the peak is `V_divider [-expm1(-x)/x]`. Native `expm1` computes `exp(z)-1` without subtracting two nearly equal rounded numbers. For positive x approaching zero, `-expm1(-x)/x` approaches 1, preserving the capacitive-divider limit even for x around 1e-16. The attenuation quotient is computed before multiplication by the divider voltage to avoid premature underflow of their small product. No artificial minimum x is imposed, and zero mutual capacitance or voltage swing remains a valid zero excursion. Existing output keys/order are unchanged: the peak expression inlines the divider limit, followed by the independently visible `fast_edge_limit`.

At finite floating-point extremes, tau or x may underflow or overflow despite physically positive inputs; the common engine must report invalid/nonfinite arithmetic, including x that actually rounds to zero. Native expm1 fixes cancellation for representable positive x, not arbitrary range exhaustion. The nominal x = 1 example and all independent expected values below remain unchanged.

**Dimensions/domain:** R_v positive ohm; C_m nonnegative F; C_v strictly positive F; swing nonnegative V; full ramp duration strictly positive s. Segment propagation delay is nonnegative and limited to at most edge_time/10. This conservative criterion deliberately restricts the lumped model; TI discusses distributed effects becoming significant as propagation delay approaches the edge duration. It is not a claimed accuracy bound. Use **full ramp duration**, not a 10%–90% specification without conversion. No distributed near/far-end decomposition, mutual inductance, input clamps, nonlinear loading or multiple aggressors. Formula is derived here from MIT's capacitor law/KCL; it is not attributed verbatim to TI's distributed crosstalk discussion.

**Independent example:** R_v = 2 kohm, C_m = 2 pF, C_v = 8 pF, swing 5 V, full ramp 20 ns and segment delay 1 ns:

- `tau = 2000 × 10e-12 = 20 ns`.
- Forcing asymptote `= 2000 × 2e-12 × 5 / 20e-9 = 1 V`.
- Peak `= 1 × [1-exp(-1)] = 0.632120558829 V`.
- Fast-edge limit `= 2/(2+8) × 5 = 1 V`.
- Delay 1 ns is below the required 2 ns limit.

**Primary sources:** [TI/National SNOA382, Understanding and Eliminating EMI in Microcontroller Applications](https://www.ti.com/lit/pdf/snoa382), sections 3.6, 4.5 and 4.6 for capacitive coupling, edge-rate scaling and distributed limits; [MIT 6.071 Capacitors and inductors](https://ocw.mit.edu/courses/6-071j-introduction-to-electronics-signals-and-measurement-spring-2006/eac1995aca830cfd0de6bab60a8abf10_capactr_inductr.pdf), equation 1.5 and capacitor KCL foundations.


### Catalog inputs and complete worked outputs

| Input key | Label | Unit | Example/default | Admissible range |
|---|---|---|---|---|
| `r_victim` | Victim resistance to AC ground | ohm | `2000` | > 0 |
| `c_mutual` | Mutual aggressor-victim capacitance | F | `2e-12` | >= 0 |
| `c_victim` | Victim shunt capacitance | F | `8e-12` | > 0 |
| `voltage_swing` | Aggressor full voltage swing magnitude | V | `5` | >= 0 |
| `edge_time` | Full linear-ramp duration (not 10%-90%) | s | `2e-08` | > 0 |
| `segment_delay` | Coupled segment one-way propagation delay | s | `1e-09` | >= 0 |

Cross-input/model constraints (evaluated before outputs):

- `segment_delay <= edge_time/10` — Propagation delay must be at most one tenth of edge time for this lumped estimate.

| Sequential output key | Label | Unit | Exact independent expected value | Executable expression |
|---|---|---|---|---|
| `victim_time_constant` | Victim recovery time constant | s | `2e-08` | `r_victim*(c_mutual+c_victim)` |
| `peak_excursion` | Peak victim voltage excursion | V | `0.6321205588285577` | `c_mutual/(c_mutual+c_victim)*voltage_swing*(-expm1(-edge_time/victim_time_constant)/(edge_time/victim_time_constant))` |
| `fast_edge_limit` | Capacitive-divider fast-edge limit | V | `1` | `c_mutual/(c_mutual+c_victim)*voltage_swing` |

