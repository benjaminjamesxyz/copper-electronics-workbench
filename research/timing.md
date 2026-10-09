# Digital timing interfaces and EMC — eight researched calculations

This lane defines exactly eight calculations in [timing.json](timing.json). Units are SI unless marked `%`, dimensionless `1`, or baud (symbols/s). Levels: two basic, three intermediate, three expert. Results are closed-form engineering estimates, not simulation or standards-compliance certificates. A negative feasibility width, UART margin or setup slack is a meaningful failed-design result and is not suppressed.

Sources are primary manufacturer/university material. Numerical examples below were independently computed from the derived equations and elementary arithmetic/logarithms, not from the application engine. Every JSON example includes every output, including supporting quantities. All constraint expressions use inputs/constants only; sequential output expressions can use earlier outputs. All defaults equal the worked-example inputs and lie inside their stated model domains. No build, lint, test or formatter runs were performed.

## Ownership and distinctness

- Digital RC threshold-to-output timing includes receiver switching-threshold bounds and intrinsic gate delay. General analog RC charge/discharge remains with `transients`.
- Clock divider selection concerns timer-register quantization; UART concerns accumulated asynchronous sampling displacement, not another divider rearrangement.
- Setup slack concerns synchronous endpoint timing, not PCB trace skew or transmission-line delay.
- Capacitive crosstalk solves the coupled victim voltage response, including victim loading. Displacement current solves dielectric overlap geometry and electric-field current to a reference surface. Although both use the capacitor current law, neither is an inverse rearrangement of the other's physical problem.
- Inductive spike uses a specified current fall and series parasitic inductance. It does not duplicate magnetic energy, transformer coupling, RL decay or MOSFET gate-drive switching-loss calculations.

## 1. I2C pullup resistance bounds — intermediate

**ID:** `timing.i2c_pullup_bounds`

**Purpose:** Select the passive pullup interval that simultaneously permits an open-drain device to sink the bus low and meets the bus rise-time budget.

**Derivation:** At the guaranteed low-voltage limit, resistor current is `(V_supply - V_OL)/R`; limiting it to guaranteed `I_sink` gives:

`R_min = (V_supply - V_OL)/I_sink`.

For a lumped released bus, `V(t) = V_supply[1-exp(-t/(R C_bus))]`. The 30% and 70% crossing times differ by:

`t_30_70 = R C_bus[ln(0.7)-ln(0.3)] = R C_bus ln(7/3)`.

Thus `R_max = rise_limit/[C_bus ln(7/3)]`. The exact logarithm is used rather than TI's rounded factor 0.8473. `interval_width = R_max - R_min`; a negative width indicates no passive solution.

**Dimensions/domain:** Voltage in V, sink capability in A, bus capacitance in F, rise limit in s, all resistance outputs in ohm. Supply/current/capacitance/time must be positive, and `0 <= V_OL < V_supply`. Infeasible electrical specifications remain evaluable so the user can see the interval failure. Use the weakest sink on the bus and the applicable mode's actual rise limit. No active pullup, accelerator, leakage, resistor tolerance, or distributed bus model.

**Independent example:** 3.3 V supply, 0.4 V maximum low, 3 mA sink, 200 pF bus, 300 ns rise limit:

- `R_min = 2.9/0.003 = 966.666666667 ohm`.
- `ln(7/3) = 0.847297860387`; `R_max = 300e-9/(200e-12 × 0.847297860387) = 1770.333751716 ohm`.
- Width `= 803.667085049 ohm`.

This recomputes, rather than transcribes, the TI example and deliberately preserves the exact logarithmic factor. The chosen resistor needs tolerance margin inside the interval; this calculator makes no I2C compliance claim.

**Primary source:** [TI SLVA689, I2C Bus Pullup Resistor Calculation](https://www.ti.com/lit/pdf/slva689), equations 1–6 and section 4.

## 2. RC Schmitt-input digital delay bounds — intermediate

**ID:** `timing.schmitt_rc_delay_bounds`

**Purpose:** Compute a rising digital input's earliest/latest output transition, including receiver threshold variation rather than assuming a universal 50% threshold.

**Derivation:** Starting from zero, `V_input(t) = V_supply[1-exp(-t/(RC))]`. A rising Schmitt threshold `V_T` is crossed at `t_T = -RC ln(1 - V_T/V_supply)`. Since this is monotone in threshold, minimum threshold yields earliest crossing and maximum threshold yields latest crossing. Add the applicable intrinsic propagation-delay corners:

- `time_constant = RC`.
- `earliest_transition = -RC ln(1-threshold_min/V_supply) + gate_delay_min`.
- `latest_transition = -RC ln(1-threshold_max/V_supply) + gate_delay_max`.
- `timing_window = latest_transition - earliest_transition`.

**Dimensions/domain:** R in ohm, total input C in F, thresholds/supply in V, delays/results in s. Require R,C,supply positive; `0 < threshold_min <= threshold_max < supply`; `0 <= gate_delay_min <= gate_delay_max`. These constraints enforce the log domain and ordered timing corners. Initial input is zero; charging resistance is constant; supply step is ideal. Use a Schmitt input that permits the slow ramp, not an ordinary CMOS input. Driver resistance, leakage and R/C tolerances are not inferred. Repeat at component tolerance corners and use device threshold/propagation bounds for voltage, load and temperature. The independent defaults are illustrative threshold values, not a claim that they are SN74AHC14 specifications.

**Independent example:** R = 2 kohm, C = 1 nF gives `RC = 2 us`; supply 5 V, threshold bounds 2.5 V and 3.75 V, intrinsic delays 10 ns and 20 ns:

- Earliest `= 2 us × ln(2) + 0.010 us = 1.396294361120 us`.
- Latest `= 2 us × ln(4) + 0.020 us = 2.792588722240 us`.
- Window `= 1.396294361120 us`.

**Primary sources:** [TI SN74AHC14 datasheet](https://www.ti.com/lit/ds/symlink/sn74ahc14.pdf), section 5.5 guaranteed thresholds and section 8.2 RC delay application; [TI SCEA046, Schmitt-Trigger Inputs](https://www.ti.com/document-viewer/lit/html/scea046), slow-input suitability. The datasheet's application explains the digital RC delay and threshold-dependent bounds.

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

## 4. UART baud mismatch and stop-sample margin — expert

**ID:** `timing.uart_sampling_margin`

**Purpose:** Expose accumulated transmitter/receiver baud mismatch over a frame and estimate whether the first stop-bit center sample can leave the ideal stop-bit interval.

**Derivation:** With one start bit, D data bits, P parity bits and one stop bit, the transmitter stop bit begins at `(1+D+P)/B_tx` and ends at `(2+D+P)/B_tx`. Set `n = 1+D+P+0.5`. The nominal receiver stop sample is `n/B_rx`. If the receiver recognizes a falling start edge on the next oversampling tick, detection delay lies in `[0, 1/(S B_rx)]`. Therefore the earliest sample is `n/B_rx` and latest is `n/B_rx + 1/(S B_rx)`.

Subtract guard G at each boundary:

- `early_margin = n/B_rx - (n-0.5)/B_tx - G`.
- `late_margin = (n+0.5)/B_tx - n/B_rx - 1/(S B_rx) - G`.
- `minimum_margin = min(early_margin, late_margin)`.
- `mismatch_percent = 100(B_rx/B_tx - 1)`.

The first-stop sample is farther from the start than all data/parity center samples; with constant baud mismatch the accumulated drift is largest there. This does not model waveform distortion or a hardware voting algorithm.

**Dimensions/domain:** Actual B_tx/B_rx in baud and strictly positive; D is an integer 5–9; P is 0 or 1; S is 8 or 16; guard nonnegative seconds. All time outputs in s and mismatch in %. Mismatch is signed receiver-relative-to-transmitter, not generator error versus a desired nominal baud. Negative margins intentionally remain valid results.

**Limits:** This is a derived ideal-center sampling budget informed by oversampling receiver descriptions, **not** a manufacturer acceptance-limit formula. Some hardware votes on three central samples, filters start detection, resynchronizes, or has additional fixed offsets. Consult the actual peripheral's tolerance tables. Positive margin does not certify reliable communication; start detection, analog settling/noise, oscillator PVT drift and jitter still matter. For hardware where start phase can be early or exceeds one tick, this particular model does not apply.

**Independent example:** B_tx = 9600 baud, B_rx = 9700 baud, D = 8, P = 0, S = 16, G = 0:

- Mismatch `= 100 × 100/9600 = +1.041666666667%`.
- Nominal stop center `= 9.5/9700 = 979.381443299 us`.
- Detection quantization `= 1/155200 = 6.443298969 us`.
- Transmitter stop begins at `9/9600 = 937.5 us`, ends at `10/9600 = 1041.666666667 us`.
- Early margin `= 41.881443299 us`; late margin `= 55.841924399 us`.
- Minimum margin `= 41.881443299 us`.

**Primary sources:** [Microchip USART Asynchronous Receiver](https://onlinedocs.microchip.com/oxy/GUID-4D282FC5-82FC-4934-8BAD-D4A5D8422E6C-en-US-7/GUID-D742E4B3-493C-4FD7-8A7C-DE55893B1017.html), 16x oversampling, mid-bit start recognition and stop-bit sampling; [Microchip AVR Asynchronous Data Reception](https://onlinedocs.microchip.com/oxy/GUID-0EC909F9-8FB7-46B2-BF4B-05290662B5C3-en-US-12.1.1/GUID-694A5C41-8200-43BB-94E7-D49D1E6F1E43.html), majority-vote behavior that motivates the hardware-specific limitation. Manufacturer indexed content was available through web search; direct Microchip HTML retrieval returned HTTP 503 during research.

## 5. Synchronous register setup slack — intermediate

**ID:** `timing.synchronous_setup_slack`

**Purpose:** Compute setup headroom for a one-cycle register-to-register data path with explicit skew sign.

**Derivation:** Relative to the launch clock at time zero, data arrives at `clock_to_q + path_delay`. Capture occurs at `clock_period + clock_skew`, where skew is **capture minus launch clock arrival**. Data must arrive setup time before capture, further reduced by worst-case uncertainty:

`available_time = clock_period + clock_skew - setup_time - uncertainty`;
`arrival_time = clock_to_q + path_delay`;
`setup_slack = available_time - arrival_time`.

Positive skew helps setup. Positive slack is headroom; negative slack is a violation. A cell library can legitimately specify negative setup time, so setup_time is intentionally signed.

**Dimensions/domain:** Every input/output is seconds. Clock period positive, clock-to-Q/path delay/uncertainty nonnegative, skew and setup signed. Uncertainty must be an appropriate worst-case setup penalty, not an RMS jitter number. Model assumes related clocks, one-cycle capture and characterized maximum delays. It does not replace STA or perform hold, multicycle, CDC, recovery/removal, OCV or common-clock-path calculations.

**Independent example:** T = 10 ns, CQ = 0.5 ns, path = 7 ns, setup = 0.3 ns, skew = +0.2 ns, uncertainty = 0.1 ns:

- Latest arrival allowed `= 10+0.2-0.3-0.1 = 9.8 ns`.
- Actual worst arrival `= 0.5+7 = 7.5 ns`.
- Slack `= 9.8-7.5 = +2.3 ns`.

**Primary source:** [UMBC, Ryan Robucci, Physical Design Flow and Timing Analysis](https://eclipse.umbc.edu/robucci/cmpeRSD/lectures/Physical_Design_Flow_and_Timing_Analysis/Physical_Design_Flow_and_Timing_Analysis.html), sections 20–23 explicitly define positive capture-later skew and setup slack with uncertainty.

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

## 7. Inductive turn-off voltage spike — basic

**ID:** `timing.inductive_turnoff_spike`

**Purpose:** Estimate unclamped low-side switch overvoltage from a known current fall through series parasitic inductance.

**Derivation:** The passive-sign inductor voltage is `v_L = L di/dt`. During a positive-current fall, di/dt is negative. With low-side switching orientation, the positive voltage rise across the switch has magnitude:

`current_fall_rate = (I_initial - I_final)/fall_time`;
`spike_voltage = L current_fall_rate`;
`switch_voltage = V_supply + spike_voltage`.

**Dimensions/domain:** L nonnegative H, current in nonnegative A with initial >= final, interval strictly positive s, supply nonnegative V. Current-fall rate is A/s and voltages V. Supply-plus-spike polarity is valid only for the stated low-side orientation. Input is the **full current-change interval**, not gate transition time or a voltage rise-time value. Constant L and linear current fall are assumed. The model does not determine the actual current fall, avalanche/clamp voltage, parasitic-capacitance ringing or device safe operating area. No RL exponential or magnetic stored-energy output is duplicated.

**Independent example:** L = 40 nH, current falls from 5 A to 1 A in 10 ns, supply 12 V:

- Slew magnitude `= 4/10e-9 = 4e8 A/s`.
- Spike `= 40e-9 × 4e8 = 16 V`.
- Estimated switch voltage `= 12+16 = 28 V`.

**Primary sources:** [MIT 6.071 Capacitors and inductors](https://ocw.mit.edu/courses/6-071j-introduction-to-electronics-signals-and-measurement-spring-2006/eac1995aca830cfd0de6bab60a8abf10_capactr_inductr.pdf), equation 1.27; [TI/National SNOA382](https://www.ti.com/lit/pdf/snoa382), section 4.7 applies `L di/dt` to package/lead parasitic inductance and voltage transients. Supply addition follows KVL for the stated switch topology, not a general polarity rule.

## 8. Parallel-plate displacement current — expert

**ID:** `timing.plate_displacement_current`

**Purpose:** Estimate electric-field current during an edge through a dielectric overlap between a switching surface and chassis/reference plane, without a DC conductive connection.

**Derivation:** For homogeneous dielectric between parallel plates, `E = V/gap`, `D = epsilon E` and Maxwell's displacement-current density is `J_d = dD/dt = epsilon/gap × dV/dt`. Integrating over rectangular overlap area A = width × length gives:

- `C_overlap = epsilon_0 epsilon_r width length/gap`.
- `voltage_slew = voltage_change/edge_time`.
- `displacement_current = C_overlap voltage_slew`.
- `current_density = displacement_current/(width length)`.

Use `epsilon_0 = 8.8541878128e-12 F/m` as the fixed numerical engineering constant and `c = 299792458 m/s`. Signed voltage change gives signed current in the chosen positive plate-normal direction.

**Dimensions/domain:** Width/length/gap in positive m; epsilon_r dimensionless >= 1; signed change in V; positive full ramp time in s; capacitance F, slew V/s, current A, current density A/m². Require `gap <= min(width,length)/10` to select the no-fringing approximation. Also require `max(width,length) <= c edge_time/[10 sqrt(epsilon_r)]`, so dielectric propagation time is at most one tenth of edge duration for nonmagnetic material. Both are conservative model-selection conditions, not guaranteed percentage-error bounds. Dielectric must be linear, homogeneous and effectively insulating; use edge-spectrum permittivity. This computes edge current, not AC reactance, breakdown rating, leakage current certification or emitted field strength. Chassis return impedance and external common-mode network are excluded.

**Independent example:** Width 20 mm, length 30 mm, gap 0.5 mm, epsilon_r = 4, change +100 V over full ramp 20 ns:

- Area `= 0.02 × 0.03 = 0.0006 m²`.
- Capacitance `= 8.8541878128e-12 × 4 × 0.0006/0.0005 = 4.250010150144e-11 F = 42.50010150144 pF`.
- Slew `= 100/20e-9 = 5e9 V/s`.
- Displacement current `= 42.50010150144e-12 × 5e9 = +0.2125005075072 A`.
- Density `= 0.2125005075072/0.0006 = +354.167512512 A/m²`.
- Geometry condition: gap 0.0005 m <= 0.002 m. Quasistatic size bound is `299792458 × 20e-9/(10 × 2) = 0.299792458 m`, above the 0.03 m longest side.

**Primary source:** [MIT 6.013 Electromagnetics and Applications, chapter 3: Electromagnetic Fields in Simple Devices and Circuits](https://ocw.mit.edu/courses/6-013-electromagnetics-and-applications-spring-2009/be09fe40701d8ba037ddc18fea301449_MIT6_013S09_chap03.pdf), section 3.1.3 equations 3.1.10–3.1.12 for parallel-plate capacitance/current and equation 3.2.1 for the displacement-current term in Ampere's law. The introductory discussion explicitly distinguishes quasistatic from dynamic structures.

## Numerical and integration notes

Every expression uses only the contract's arithmetic, `ln`, `exp`, `expm1`, `floor`, `min`, `max` and `sqrt`. Logs and all divisions have positive validated physical denominators in the model domain. UART discrete fields use floor/polynomial equalities; timer range checks inline the rounded count because constraints precede outputs. Failure cases remain numeric where they have physical meaning. Floating-point overflow/nonfinite intermediate results must still be rejected by the common engine; finite inputs alone do not mathematically guarantee finite products at arbitrary magnitudes. Capacitive crosstalk uses the stable divider-limit form `V_divider [-expm1(-x)/x]`, with native expm1 preserving the physical fast-edge domain and quotient-first ordering avoiding premature underflow. A ratio that actually underflows to zero is an explicit arithmetic-domain failure, not a new physical-domain restriction. Near-zero margins/slack should be interpreted against measurement uncertainty, not merely a displayed decimal sign.

Parent integration checks: parse the catalog, enforce schema/unique keys, evaluate defaults and all eight independent examples, exercise invalid denominator/log domains and discrete fields, and retain negative interval/slack/margin behavior. Those checks are intentionally deferred to the integration owner.
