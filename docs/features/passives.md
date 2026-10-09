# Nonideal passive components

Models use the shared Rust evaluator. Diagrams are static explanatory illustrations, not live plots. Source-access evidence and independent numerical examples below come from the researched handoff; this implementation did not repeat source retrieval or run checks. Calibration and physical ratings remain the designer’s responsibility.



1. [KYOCERA AVX / American Technical Ceramics, Circuit Designer's Notebook: Dielectric Aging Phenomena](https://rfs.kyocera-avx.com/userFiles/uploads/pdfs/dielectric_aging.pdf), single-page manufacturer note, especially “Calculating Aging Rate”, Curie-temperature discussion and the percent-per-decade table.
2. [Texas Instruments SBOA597, Measurement and Calibration Techniques for Ultra-low Current Measurement Systems](https://www.ti.com/lit/pdf/sboa597), November 2024, §4.5 and Figure 4-15. The text explicitly describes parallel RC polarization time constants, retained voltage memory, and the charge/short/recovery protocol. The source discusses many absorption branches; the proposed calculator deliberately fits one branch.
3. [Analog Devices RAQ192, How to Use LTspice Simulations to Account for the Effect of Voltage Dependence](https://www.analog.com/en/resources/analog-dialogue/raqs/raq-issue-192.html), Reiner Bidenbach, August 2021. Read successfully in Chromium after static-page and PDF retrieval timed out. Its first-order charge equation is `Q=x*c0V-0.5*x**2*(c0V-cVmax)/Vmax`. The source warns that linear fits can overestimate capacitance and excludes several other MLCC nonidealities.
4. [Vishay, Shunts, Current Shunts, and Current-Sensing Resistors](https://www.vishay.com/doc?49159=), PL0005-1801, pp.14–15, “Noise”, Table 2 and the one-decade-bandwidth definition. The table explicitly gives 0 dB = 1 µV/V and −20 dB = 0.10 µV/V.
5. [Vishay Beyschlag, Basics of Linear Fixed Resistors](https://www.vishay.com/docs/28771/basics.pdf), November 2008, p.4 “Noise” and “Non-Linearity”. Distinguishes bias-driven current noise from white Johnson noise.
6. [Vishay RIFAQ, Basic Principles Q6](https://www.vishay.com/en/landingpage/rifaq/index.html#bp6). Defines voltage coefficient in ppm/V, measurements at maximum voltage and one-tenth maximum voltage, and constant-temperature measurement to exclude TCR. This establishes the physical effect, not a universal fitted law.
7. [Analog Devices University, Current Steering Digital-to-Analog Converters](https://wiki.analog.com/university/courses/tutorials/cmos-dac-chapter), Douglas Mercer, §1.4 and Figure 4b. Establishes the voltage-mode R–2R topology, switched reference/ground rungs, end-of-ladder voltage output and need for a low-impedance reference. §1 defines INL as gain/offset-corrected, which is why the supplied error is not called INL.

Access gaps are not disguised as evidence: Murata FAQ returned only metadata; ADI MT-015 static retrieval timed out; KYOCERA AVX tantalum-model PDF returned HTTP 406; an All About Circuits RAQ mirror returned 403. None is needed for the final models. Search-generated summaries were not accepted as source equations; one summary even misstated the noise-index normalization. The read Vishay table resolves that ambiguity.

Independent numbers below were derived with Python arithmetic outside the Rust engine: rational polynomial integration, a 60-digit Decimal quadratic, direct transient equations and an exact rational nodal matrix. No build, tests, lint, formatter, backend or UI execution occurred. No new expression helper is required.


## 1. `passives.mlcc_logarithmic_aging` — basic

### Purpose and novelty

Estimate ferroelectric dielectric-permittivity loss with elapsed time after deaging. Closest baseline: `thermal.arrhenius_acceleration`, which scales thermally activated life exposure, not capacitance. `thermal.resistor_tcr_drift` is an instantaneous temperature law, not a logarithmic material-relaxation clock. No baseline includes capacitor aging.

### Physical model and units

The KYOCERA AVX note gives `C_A=C_1[1−(A/100)log10(t)]` when `C_1` is the one-hour reference and `t` is in hours. The calculator fixes that reference explicitly, avoiding ambiguity between one-hour and 24-hour datasheet conventions:

- `elapsed_decades = log10(age / 3600)`; `age` is seconds since the most recent deaging event.
- `fractional_loss = aging_percent_decade * elapsed_decades / 100`.
- `aged_capacitance = capacitance_one_hour * (1−fractional_loss)` in F.

The rate applies to the one-hour reference; do not compound a 2% decrement at each decade. `log1p((age−3600)/3600)/ln(10)` retains near-reference small differences. The fit is only meaningful within the characterized aging interval, at fixed temperature and bias, without reheating above Curie temperature. The source gives typical rather than universal dielectric rates; the default 2%/decade is illustrative. Positive retained capacitance is required, but that algebraic guard does not validate extrapolation over arbitrary times.

### Complete independent example

Inputs: `capacitance_one_hour=10e−6 F`, `aging_percent_decade=2 %/decade`, `age=36,000,000 s` = 10,000 h.

| Output key | Independent derivation | Expected |
|---|---|---:|
| `elapsed_decades` | log10(10,000 h / 1 h) | 4 decade |
| `fractional_loss` | 2×4/100 | 0.08 |
| `aged_capacitance` | 10 µF×0.92 | 9.2e−6 F |

### Independent tester cases

- Nominal: all three values above.
- Reference boundary: age=3600 s gives exactly zero decades/loss and the reference capacitance.
- No-aging boundary: aging rate=0 keeps C constant at any otherwise valid age.
- At age=36,000 s and rate=2, C=9.8 µF; this is one decade, not ten.
- Reject age below 3600 s, zero/negative capacitance, negative aging rate, and any fit yielding loss≥1. Example: rate=25 and age=36,000,000 s is exactly zero capacitance and invalid.
- A zero-aging result does not imply zero DC-bias or temperature effects.

### Mandatory diagram

Use a semilog material-aging plot. Plot area x=70…520, y=55…205; horizontal axis “time since deaging, logarithmic”, vertical axis “C”. Draw a descending straight conceptual line in semilog coordinates; mark the one-hour anchor on the left and a generic later observation on the right. At x=545 place a small capacitor symbol and “ferroelectric dielectric”. Bound labels: initial `C₁h` → `capacitance_one_hour`; clock label `t` → `age`; rate `A` → `aging_percent_decade`; result `C(t)` → `aged_capacitance`. Do not bind a marker coordinate to an actual numerical age. Caption: “Conceptual semilog aging law; heating above the Curie point resets the clock. Fixed temperature and DC bias.”


## 2. `passives.dielectric_absorption_recovery` — expert

### Purpose and novelty

A capacitor can recover terminal voltage after being shorted because slow polarization retains charge. Closest baseline: `transients.capacitor_charge_sharing`. That baseline starts from two externally specified capacitor voltages and models their connection through a resistor. Here the hidden state is established by a finite-duration charge, depleted during a short, then released after short removal. The same conservation laws apply, but the design problem is a nonideal single component's history-dependent dielectric memory, not another ideal RC delay or a renamed redistribution output. The finite charge/short preparation is essential; do not replace it with an arbitrary initial hidden voltage.

### Topology, derivation and validity

Across terminal A and reference B, connect fast `main_capacitance=C_m` directly in parallel with the SERIES branch `absorption_resistance=R_a` followed by `absorption_capacitance=C_a`. The internal node between R_a and C_a is the polarization node. Start with both capacitors uncharged.

1. Clamp A to `charge_voltage=V_ch` for `charge_time=t_ch`. Internal voltage becomes `V_ch[1−exp(−t_ch/(R_a C_a))]`.
2. Clamp A to B for `short_time=t_s`. C_m becomes zero immediately; the internal voltage decays to `memory_voltage=V_a0` through R_a.
3. Disconnect the source and short. The A terminal is open except for the modeled capacitor. Charge conservation gives `V_inf=V_a0 C_a/(C_m+C_a)`. The difference between internal and terminal voltages decays with `tau=R_a C_a C_m/(C_m+C_a)`.
4. At `recovery_time=t`, `recovered_voltage=V_inf[1−exp(−t/tau)]`.

All capacitances are F, resistance Ω, times s, voltages V. A positive charge voltage means A and the internal node become positive relative to B; signed voltage is supported for nonpolar dielectric models. TI SBOA597 §4.5 provides the polarization-branch model and physical sequence. Reducing many time constants to one is a fit, not a material identity. Infinite input resistance is essential; finite meter leakage ultimately changes recovery. ESR and the impulsive current of ideal clamps are not calculated. Zero short time denotes the limiting instantaneous reset of C_m with no time for C_a discharge; zero charge time gives no stored polarization.

Use `-expm1(-x)` for charging and recovery. Do not infer that the asymptotic voltage equals the specified absorption percentage for an arbitrary timing protocol. The TI text's standardized timings are context only, not a compliance claim.

### Complete independent example

Inputs: `main_capacitance=1e−6 F`, `absorption_capacitance=1e−8 F`, `absorption_resistance=1e9 Ω`, `charge_voltage=10 V`, `charge_time=20 s`, `short_time=1 s`, `recovery_time=10 s`.

First, R_a C_a=10 s. The internal voltage after charging is `10(1−exp(−2))`; after shorting it is multiplied by `exp(−0.1)`.

| Output key | Independent derivation | Expected |
|---|---|---:|
| `memory_voltage` | 10(1−exp(−2))exp(−0.1) | 7.823809897829776 V |
| `recovery_time_constant` | 1e9/(1/1e−8+1/1e−6) | 9.900990099009901 s |
| `final_recovery_voltage` | memory_voltage/101 | 0.07746346433494829 V |
| `recovered_voltage` | final_recovery_voltage×(1−exp(−1.01)) | 0.04924980040079833 V |

### Independent tester cases

- Nominal: complete table.
- At recovery_time=0, recovered_voltage=0 while the other outputs are unchanged.
- At charge_time=0 or charge_voltage=0, all voltage outputs are zero, but tau remains positive.
- Negating charge_voltage negates all three voltages and leaves tau unchanged.
- Long charge with short_time=0: V_inf tends to V_ch C_a/(C_m+C_a). Long short forces both recovery voltages toward zero. Long recovery approaches V_inf, never above it for positive excitation.
- Small positive recovery time has slope `V_a0/(R_a C_m)`; nominal values at 1e−12 s give approximately 7.82380989783e−15 V, not zero from subtractive cancellation.
- Reject nonpositive capacitances/resistance and negative times. A finite instrument input resistance is outside this model, not an extra input silently ignored.

### Mandatory diagram

Left two-thirds: terminals A at (120,65) and B at (120,220). Draw C_m vertically on the x=230 branch. On x=400 draw R_a above C_a, with explicit internal junction between them. Top and bottom rails connect the branches; the internal node must not connect to the C_m branch. Show a detached “charge → short → open” three-phase timing strip at x=35…650, y=245; no switch is electrically closed in the recovery picture. Label the A terminal positive and B reference. Bound labels: C_m → `main_capacitance`, R_a → `absorption_resistance`, C_a → `absorption_capacitance`, internal V_a0 → `memory_voltage`, terminal V_rec → `recovered_voltage`. Put timing labels in the legend if cramped. Caption: “After finite charging and shorting, the open terminal recovers voltage from retained polarization. One fitted absorption branch; no leakage or meter loading.”


## 3. `passives.mlcc_bias_charge_energy` — intermediate

### Purpose and novelty

Calculate usable charge and energy when a ceramic capacitor's differential capacitance changes across the operating window. Closest baseline: `transients.capacitor_constant_current` assumes a fixed C and computes ideal stored energy. `mosfet.output_capacitance_hard_turn_on_loss` accepts an already energy-equivalent C; it does not integrate a C(V) characteristic. This model introduces a nonlinear constitutive relation, not extra fixed-capacitance outputs.

### Equations, conventions and domain

ADI RAQ192 explicitly integrates its first-order differential-capacitance curve:

`k=(C_0−C_fit)/V_fit`, `C(V)=C_0−kV=dQ/dV`.

For `0≤V_lo<V_hi≤V_fit`:

- `upper_capacitance=C_0−k V_hi`.
- `usable_charge=∫[V_lo,V_hi] C(V)dV = ΔV[C_0−k(V_hi+V_lo)/2]`.
- `usable_energy=∫[V_lo,V_hi] V C(V)dV = ΔV[C_0(V_hi+V_lo)/2−k(V_hi²+V_hi V_lo+V_lo²)/3]`.
- `charge_equivalent_capacitance=usable_charge/ΔV`.
- `energy_equivalent_capacitance=2 usable_energy/(V_hi²−V_lo²)`.

Energy follows directly from dW=V dQ. The last expression is evaluated algebraically without subtracting nearly equal squares: `C_0−(2k/3)[V_hi+V_lo²/(V_hi+V_lo)]`. Charge and energy factor ΔV rather than subtracting two large antiderivatives. Units: capacitances F, voltages V, charge C, energy J, k F/V. Positive charge and energy describe what is released in discharge from V_hi to V_lo; charge delivered during charging has equal magnitude only in this reversible model.

C_fit must be positive and no greater than C_0; V_fit>0. The fit is interpolation-only. It does not imply that measured small-signal capacitance at arbitrary frequency equals an energy-storage constitutive law. Use a quasistatic/reversible characterization relevant to the application. Hysteresis, ESR dissipation and rate dependence require a different model.

### Complete independent example

Inputs: `zero_bias_capacitance=10e−6 F`, `fit_capacitance=4e−6 F`, `fit_voltage=10 V`, `lower_voltage=2 V`, `upper_voltage=8 V`. Thus k=0.6e−6 F/V, ΔV=6 V, Δ(V²)=60 V², Δ(V³)=504 V³.

| Output key | Independent polynomial integration | Expected |
|---|---|---:|
| `upper_capacitance` | 10e−6−0.6e−6×8 | 5.2e−6 F |
| `usable_charge` | 10e−6×6−(0.6e−6/2)×60 | 42e−6 C |
| `usable_energy` | (10e−6/2)×60−(0.6e−6/3)×504 | 199.2e−6 J |
| `charge_equivalent_capacitance` | 42e−6/6 | 7e−6 F |
| `energy_equivalent_capacitance` | 2×199.2e−6/60 | 6.64e−6 F |

### Independent tester cases

- Nominal table and independent charge/energy integrals.
- Constant-C boundary C_fit=C_0 gives C_Q=C_E=C_0, Q=C_0 ΔV and W=C_0 Δ(V²)/2.
- With nominal fit and window 0…10 V: C_hi=4 µF, Q=70 µC, W=300 µJ, C_Q=7 µF, C_E=6 µF.
- As window width approaches zero around a positive voltage, both equivalent capacitances approach local differential C; both dimensional amounts approach zero. Test a narrow representable window without cancellation from antiderivative subtraction.
- Reject equal/reversed voltage endpoints, upper voltage above V_fit, negative lower voltage, nonpositive capacitances/V_fit, and C_fit>C_0.

### Mandatory diagram

Draw a C_diff versus V plot at x=60…425, y=45…210. The descending straight conceptual curve starts at C_0 and ends at C_fit at V_fit. Show two generic vertical window markers labeled V_lo and V_hi and shade/hatch the intervening area as “∫C(V)dV = ΔQ”. At x=490…655 draw a capacitor with a positive upper terminal and an energy annotation “ΔW = ∫V C(V)dV”. Bound keys: `zero_bias_capacitance`, `fit_capacitance`, `fit_voltage`, `lower_voltage`, `upper_voltage`, `usable_charge`, `usable_energy`. Curves and marker positions are explanatory, not live-scaled. Caption: “Differential capacitance must be integrated; energy-equivalent C differs from endpoint C. Linear reversible fit, only inside the characterized voltage range.”

