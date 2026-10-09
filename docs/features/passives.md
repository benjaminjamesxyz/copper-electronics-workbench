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


## 4. `passives.resistor_excess_noise` — intermediate

### Purpose and novelty

Predict DC-bias-driven resistance-fluctuation noise from manufacturer noise index. Closest baseline: `noise.johnson_resistor` is bias-independent white thermal noise. `noise.uncorrelated_rms_sum` merely combines amplitudes. Here a new physical source has 1/f power density, an unusual manufacturer normalization and logarithmic bandwidth dependence. No white-noise sum or general dB converter is being counted again.

### Definition and derivation

Vishay pp.14–15 defines the one-decade current-noise index; Table 2 fixes the µV/V scaling. Let `a=10^(noise_index_db/20)×1e−6 V/V` be the one-decade RMS ratio. Under the explicitly assumed 1/f excess-noise power law, choose `S_v(f)=V_DC² a²/[ln(10) f]` so integrating any decade gives `(V_DC a)²`.

Then:

- `band_decades=log10(f_hi/f_lo)`.
- `excess_noise_rms=|V_DC| a sqrt(band_decades)` V RMS.
- `excess_noise_density=|V_DC| a/[sqrt(ln(10)) sqrt(f_probe)]` V/√Hz.

The frequency integral requires f_lo>0. f_hi≥f_lo admits an empty band; the probe must stay inside the characterized interval. Use `log1p((f_hi−f_lo)/f_lo)` for a narrow band. A stated noise index alone does not prove a 1/f law over arbitrary frequencies: the spectrum and index must be characterized for the resistor technology, bias and temperature. Output loading, amplifier noise and Johnson noise are deliberately not included.

### Complete independent example

Inputs: `noise_index_db=−20 dB`, `dc_voltage=10 V`, `lower_frequency=1 Hz`, `upper_frequency=1000 Hz`, `probe_frequency=10 Hz`.

| Output key | Independent derivation | Expected |
|---|---|---:|
| `decade_voltage_ratio` | 10^(−20/20) µV/V | 1e−7 V/V |
| `band_decades` | log10(1000/1) | 3 |
| `excess_noise_rms` | 10×1e−7×sqrt(3) | 1.7320508075688771e−6 V |
| `excess_noise_density` | 1e−6/sqrt(10 ln(10)) | 2.0839733249330516e−7 V/√Hz |

### Independent tester cases

- Nominal complete table.
- A single decade gives exactly |V_DC|a; two decades increase RMS by sqrt(2), not 2.
- Zero DC bias gives zero excess-noise amplitudes, not zero thermal noise. Negative bias has the same amplitudes as positive bias.
- f_lo=f_hi=f_probe gives zero integrated noise and a finite spectral density.
- Noise index=0 means 1e−6 V/V. This guards against the factor-one-million normalization error.
- Reject f_lo≤0, f_hi<f_lo, and a probe outside the integration/characterization band.

### Mandatory diagram

Left side x=40…300: resistor connected across a DC source, plus sign at its upper terminal and a separate equivalent small noise voltage-source glyph in series in an explanatory inset. Label V_DC → `dc_voltage`, NI → `noise_index_db`, and v_ex,rms → `excess_noise_rms`. Right side x=380…650: conceptual log-frequency plot with descending noise-amplitude-density line labeled “e_ex ∝ 1/√f”, a highlighted integration band and probe. Bind band edges to `lower_frequency` and `upper_frequency`; density annotation to `excess_noise_density`. Caption: “Bias-driven excess noise only; a 1/f power spectrum gives equal variance per decade. Johnson noise is separate; plotted line is conceptual.”


## 5. `passives.negative_vcr_divider` — expert

### Purpose and novelty

A high-voltage resistor's value depends on its own terminal voltage, so its divider voltage cannot first be computed using a fixed resistance. Closest baselines are `dc.loaded_divider` and `thermal.resistor_tcr_drift`. Neither solves a constitutive parameter that depends on the unknown voltage itself. This is a nonlinear implicit circuit equilibrium and transfer-ratio error, not a renamed linear drift formula or an inverse divider calculation.

### Model, source boundary and stable solution

Vishay RIFAQ Q6 describes the physical voltage coefficient and constant-temperature measurement. The chosen first-order static fit is an explicit engineering approximation:

`R_h(V_h)=R_0(1+k V_h)`, with `k=vcr_ppm_volt×1e−6 V⁻¹≤0`.

R_0 is a zero-voltage extrapolated fit parameter, NOT resistance measured at a finite test voltage. The bottom resistor R_b is constant. Let r=R_b/R_0. Kirchhoff gives `V_s=V_h+V_h r/(1+kV_h)`, hence:

`k V_h² +(1+r−kV_s)V_h−V_s=0`.

The physical root is rationalized:

`V_h=2V_s/[1+r−kV_s+sqrt((1+r+kV_s)²−4r kV_s)]`.

Because k≤0 and r>0, the square-root argument is a sum of nonnegative squares; JSON uses `hypot(1+r+kV_s,2 sqrt(−r kV_s))`. This avoids subtracting nearly equal quadratic roots and works at k=0 without branching.

`R_h=R_0(1+kV_h)`; `V_o=V_h r/(1+kV_h)`; `V_ideal=V_s/(1+R_0/R_b)`. The fractional ratio error is algebraically `−kV_h/(1+r+kV_h)`, avoiding subtraction of near-equal outputs. At V_s=0 this gives the continuous small-signal limit zero, even though the literal quotient V_o/V_ideal is undefined.

Require V_s≥0, R_0>0, R_b>0, k≤0, and `1+kV_s>0`, which keeps the fit positive over the entire possible high-side voltage interval. These conditions give one physical root between zero and V_s. A datasheet two-point VCR may not support a linear fit down to zero. The user must characterize this interval; the calculator does not silently reinterpret a finite-voltage resistance as R_0. Positive VCR is outside this explicitly named model. Thermal feedback, meter loading, leakage and all working-voltage/power/creepage limits remain separate.

### Complete independent example

Inputs: `supply_voltage=1000 V`, `high_resistance_zero=9,900,000 Ω`, `bottom_resistance=100,000 Ω`, `vcr_ppm_volt=−10 ppm/V`. Thus r=1/99 and k=−1e−5 V⁻¹. A 60-digit Decimal quadratic solution gives:

| Output key | Derivation | Expected |
|---|---|---:|
| `high_voltage` | physical root of −1e−5 V_h² +(1+1/99+0.01)V_h−1000=0 | 989.901029888954 V |
| `high_resistance` | 9.9e6(1−1e−5 V_h) | 9801999.798040994 Ω |
| `output_voltage` | V_h×100000/R_h | 10.098970111046048 V |
| `ideal_output_voltage` | 1000×100000/10000000 | 10 V |
| `fractional_ratio_error` | (10.098970111046048/10)−1 | 0.009897011104604824 |

The supply is exactly the sum of the two voltage drops. Increasing supply changes the divider ratio; this is not a fixed gain correction.

### Independent tester cases

- Nominal table and current equality V_h/R_h=V_o/R_b.
- k=0 recovers V_h=990 V, R_h=9.9 MΩ, V_o=10 V and zero ratio error.
- V_s=0 gives zero voltages/error and R_h=R_0.
- For a small negative k, error approaches `−k V_s/(1+r)²`, positive. Use this to test a tiny nonzero coefficient without subtractive cancellation.
- R_b=R_0 with V_s=100 V and k=0 gives equal 50 V drops; negative k increases the bottom voltage.
- Reject positive VCR, negative supply, nonpositive resistances, and kV_s≤−1. Example default −10 ppm/V with 100,000 V is invalid at equality. Passing the guard never certifies physical voltage rating.

### Mandatory diagram

Draw an unloaded vertical divider at x=265: source rail at y=45; high resistor around y=95; output junction at y=155; bottom resistor around y=195; ground at y=245. The upper resistor is explicitly labeled `R_h(V_h)`, not an ideal fixed resistor. Place a positive voltage bracket across its upper-to-lower pins labeled V_h → `high_voltage`. Bind V_s → `supply_voltage`, R_0 → `high_resistance_zero`, k → `vcr_ppm_volt`, R_b → `bottom_resistance`, and V_o → `output_voltage`. To the right, show the symbolic dependency “V_h → R_h(V_h) → voltage division”; this is an explanatory relation, not an extra wire. Caption: “High-side resistance depends on its own voltage; solve the divider self-consistently. Unloaded output, fixed temperature, calibrated negative-VCR fit.”


## 6. `passives.nonideal_r2r_ladder` — expert

### Purpose and novelty

Resolve actual component-network bit weights, source impedance and loaded output when switch resistances are unequal. Closest baseline: `data.unipolar_dac_output` assumes an ideal code fraction with supplied gain; no component topology is represented. `dc.two_source_node` is a single node, not a terminated multi-rung ladder. This model solves the complete specified ladder; the ideal DAC formula appears only as a comparison oracle. It is not merely a new bit-width parameterization.

### Topology and algebra

Follow ADI §1.4/Figure 4b's voltage-mode topology. Let node N0 be the LSB end, N1 the middle and N2 the MSB/output end. R connects N0–N1 and N1–N2. N0 has a permanent 2R termination to ground. Each Ni has a branch of `2R+R_on,i` to an ideal source b_i V_ref selected by a switch; b_i is 0 or 1. Load conductance G_load connects N2 to ground. There is no extra horizontal resistor after N2.

For code 0…7, b0=code−2 floor(code/2), b1=floor(code/2)−2 floor(code/4), b2=floor(code/4). All are exact at these small integers. Each R_on input is independent; values represent a measured constant resistance in the selected switch state, not a transistor model.

Successive Thevenin reduction from the LSB end gives:

- B0=2R+R_on,0; `T0=B0||2R`; `E0=b0 V_ref (2R)/(B0+2R)`.
- B1=2R+R_on,1; `T1=B1||(R+T0)`; `E1=[b1 V_ref(R+T0)+E0 B1]/(B1+R+T0)`.
- B2=2R+R_on,2; `R_out=B2||(R+T1)`; `V_oc=[b2 V_ref(R+T1)+E1 B2]/(B2+R+T1)`.
- `V_load=V_oc/(1+G_load R_out)`.
- `V_ideal=V_ref code/8`; `code_error_lsb=8 V_load/V_ref−code`.

The JSON uses ratio forms rather than unnecessary products of two resistances. All voltages reference ground. Positive V_ref is required because the result includes error in V_ref/8 LSB units. R>0, each R_on≥0, G_load≥0 and code integer 0…7. G_load=0 means open circuit. Fixed resistances imply code-independent R_out; they do not imply ideal binary weights. The error includes gain and loading and is expressly not endpoint-corrected INL. Settling, glitch, switch signal dependence, reference-source impedance and resistor tolerances are not modeled.

### Complete independent example

Inputs: `ladder_resistance=1000 Ω`, `lsb_switch_resistance=100 Ω`, `middle_switch_resistance=200 Ω`, `msb_switch_resistance=300 Ω`, `reference_voltage=5 V`, `code=5` (binary 101), `load_conductance=0.0001 S` (10 kΩ).

Here B0=2100 Ω, B1=2200 Ω, B2=2300 Ω. Independently solve the three-node conductance matrix:

- N0: `(1/2100+1/2000+1/1000)V0−V1/1000=5/2100`.
- N1: `−V0/1000+(1/2200+2/1000)V1−V2/1000=0`.
- N2: `−V1/1000+(1/2300+1/1000+1/10000)V2=5/2300`.

Exact rational elimination gives N0=2.2275279487651827 V, N1=2.02106713684548 V, and N2=`1142500/417997 V`=2.733273205310086 V. The actual loaded internal node voltages are NOT the isolated subladder Thevenin voltages below.

| Output key | Independent result | Expected |
|---|---|---:|
| `lsb_thevenin_resistance` | 2100||2000 | 1024.3902439024391 Ω |
| `lsb_thevenin_voltage` | 5×2000/(2100+2000) | 2.4390243902439024 V |
| `middle_thevenin_resistance` | 2200||(1000+1024.3902439024391) | 1054.2725173210163 Ω |
| `middle_thevenin_voltage` | E0×2200/(2200+1000+T0) | 1.2702078521939955 V |
| `output_resistance` | 2300||(1000+T1) | 1085.101304762915 Ω |
| `open_voltage` | unloaded N2 solution | 3.029861037445635 V |
| `loaded_voltage` | exact nodal rational 1142500/417997 | 2.733273205310086 V |
| `ideal_voltage` | 5×5/8 | 3.125 V |
| `code_error_lsb` | (loaded_voltage−3.125)/(5/8) | −0.6267628715038625 LSB |

### Independent tester cases

- Nominal table; verify loaded node equations independently, not by replaying catalog expressions.
- All switches zero and G_load=0: T0=T1=R_out=R and V_load=V_ref code/8 for every code 0…7.
- Same ideal switches with finite G: V_load=V_ref code/[8(1+GR)]. At R=1 kΩ, G=0.001 S and code=4, V_ref=5 V, result is 1.25 V.
- Code=0 gives all four voltage outputs zero, zero code error, and positive resistance outputs.
- For fixed switch/load values, compare codes 1,2,4 to isolate the three physical bit weights; their sum must equal code-7 output. Do not impose ideal 1:2:4 weights.
- With all other inputs unchanged, doubling every resistance and halving G leaves output voltages unchanged and doubles all Thevenin resistances.
- Reject code −1,8 or fractional 2.5; zero/negative R or V_ref; negative switch resistance or load conductance.

### Mandatory diagram

Use a horizontal ladder with N0 at (150,70), N1 at (335,70), N2 at (520,70). Draw R links between N0/N1 and N1/N2. A 2R termination descends from N0 toward ground through a separate left branch at x=65; do not accidentally put it in series with the LSB switched leg. At each node draw a vertical 2R resistor and a switch-resistance resistor below it, ending in a labeled ideal selected source `b0 V_ref`, `b1 V_ref`, `b2 V_ref`. Ground is common. A load resistor at x=625 connects N2 to ground, labeled `R_L=1/G_L; open if G_L=0`. Output positive polarity is N2 relative to ground.

Bind R → `ladder_resistance` only on an R link; label each 2R leg symbolically. Bind its corresponding switch-resistance labels to `lsb_switch_resistance`, `middle_switch_resistance`, `msb_switch_resistance`. Bind V_ref → `reference_voltage`, code → `code`, G_L → `load_conductance`, and V_out → `loaded_voltage`. The subladder E0/E1 results must NOT label the loaded N0/N1 nodes; they are disconnected-equivalent source voltages, not physical node readings. Caption: “Three voltage-mode rungs, permanent 2R LSB termination, and unequal series switch resistances. Settled DC output; code error includes loading and gain error.”

