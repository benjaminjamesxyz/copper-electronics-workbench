# Power conversion and regulation research

Exactly eight distinct calculations, defined in [power.json](power.json). Levels: two basic, three intermediate, three expert. Duty cycles and current endpoints are supporting outputs, not separately counted inverse rearrangements.

## Research scope and evidence

Research question: which closed-form power-conversion calculations can support practical component sizing and electrical power budgeting without pretending to simulate a controller? Searched the public web for TI buck/boost power-stage calculation notes, input-capacitor RMS-current derivations, boost right-half-plane zeros, and LDO ground-current dissipation. Included manufacturer primary application reports/articles with explicitly stated equations and operating modes; excluded forum claims as evidence, diode-specific rectifier sizing, MOSFET loss models, magnetic geometry and thermal-temperature calculations. English-language sources were read on 2026-10-09; no publication-date restriction was imposed because the underlying circuit laws are not date-sensitive. Six primary TI documents were used. This is a targeted engineering reference review, not an exhaustive literature survey or an independent empirical validation.

| Source | Actual URL | Evidence weight and use |
|---|---|---|
| S1: Everett Rogers, TI SLVA057, *Understanding Buck Power Stages in Switchmode Power Supplies*, March 1999 | https://www.ti.com/lit/an/slva057/slva057.pdf | Primary manufacturer analytic report; high weight for ideal CCM voltage/current and volt-second-balance derivation, section 2.1. |
| S2: Brigitte Hauke, TI SLVA477, *Basic Calculation of a Buck Converter's Power Stage* | https://www.ti.com/lit/an/slva477b/slva477b.pdf | Primary manufacturer design note; high weight within its assumptions for inductance and output-capacitor equations, sections 3 and 7. The URL currently redirects to revision C; document identifier and section are more robust than revision-dependent equation numbering. |
| S3: Brigitte Hauke, TI SLVA372D, *Basic Calculation of a Boost Converter's Power Stage*, November 2022 | https://www.ti.com/lit/an/slva372d/slva372d.pdf | Primary manufacturer analytic design note; high weight for ideal inductance, current stress and output-capacitor sizing, sections 2, 3 and 7. |
| S4: Manjing Xie, TI SSZTBI6, *Power Tips: How to Select Ceramic Capacitors to Meet Ripple-Current Requirements*, March 2016 | https://www.ti.com/lit/ta/ssztbi6/ssztbi6.pdf | Primary manufacturer technical article; high weight for RMS waveform integration, equation 2. Its capacitor-sharing simulation is not treated as proof for arbitrary layouts. |
| S5: Markus Zehendner, TI SLVAFJ5, *How to Approach a Power-Supply Design – Part 3*, March 2023 | https://www.ti.com/lit/ab/slvafj5/slvafj5.pdf | Primary manufacturer application brief; high weight for topology and RHP-zero equation 7, moderate weight for the heuristic crossover guideline, which is not a stability test. |
| S6: Wilson Fwu, TI SLYY206, *Understanding the Foundations of Quiescent Current in Linear Power Systems*, March 2021 | https://www.ti.com/lit/wp/slyy206/slyy206.pdf | Primary manufacturer engineering white paper; high weight for conservation-of-power equations 1–3 and distinction between no-load IQ and loaded ground current. |

The generic input-budget entry also cites TI SLVA118A, *Linear Regulator Design Guide For LDOs*, section 3 (https://www.ti.com/lit/an/slva118a/slva118a.pdf), as a corroborating conservation-of-energy derivation. Thus there are six core references plus one supplementary primary reference. All come from one manufacturer, not seven independent experimental teams. Confidence is high in the analytic identities within the stated domains, not in suitability for an unspecified real converter.

### Model conventions and exclusions

All numbers in JSON use SI units: voltage V, current A, frequency Hz, inductance H, capacitance F, resistance ohm, power W; ratios and duty are dimensionless (`1`). Ripple quantities are explicitly **peak-to-peak**, never RMS unless named RMS. Components use **effective** inductance/capacitance at operating bias, not unde-rated nominal markings.

Inductor and waveform entries use ideal duty from volt-second balance. S2/S3 also provide efficiency-adjusted duty heuristics for loss estimates; those are **not** inserted into the ideal equations here, because a scalar efficiency alone does not identify semiconductor/winding voltage drops. S1 provides the exact ideal buck relation; S5's duty equations reduce to the ideal buck/boost relations with zero forward drop. The independent generic input-power budget accepts total efficiency separately.

Output-capacitor models sum capacitive and ESR peak-to-peak contributions as a conservative bound. Their phases can differ. The 10%-of-output ripple ceiling is our explicit small-ripple admissibility guard, not a manufacturer's universal guarantee. No entry certifies standards compliance, component reliability or loop stability. Missing information for real component sign-off remains controller requirements, load-transient response, nonlinear bias derating, device ratings, parasitics and bench validation.

## 1. `power.buck_inductor_design` — intermediate

**Physical problem:** size an energy-transfer inductor in an ideal CCM buck. It is not an isolated duty-cycle or stored-energy calculator.

From S1, average inductor voltage is zero:

`(Vin − Vout) D + (−Vout)(1 − D) = 0`, hence `D = Vout/Vin`.

During on-time `D/fs`, `di/dt = (Vin − Vout)/L`. Choose peak-to-peak ripple `delta_I = ripple_ratio Iout`, then

`Lmin = (Vin − Vout) D/(fs delta_I)`.

Capacitor charge balance gives `ILavg = Iout`, so current endpoints at Lmin are `Ipeak = Iout + delta_I/2` and `Ivalley = Iout − delta_I/2`. This is S2's section-3 inductance equation derived rather than simply rearranged from an unrelated calculator. Dimensional check: `V/(Hz A) = H`.

**Domain:** Vin > Vout > 0; fs > 0; Iout > 0; 0 < ripple_ratio < 2. The strict ratio upper bound ensures CCM at this load. No loss voltage, saturation, duty limit or load-range guarantee is inferred. Increasing actual inductance reduces ripple; values reported describe the target/minimum design point.

**Independent worked example/defaults:** Vin = 12 V, Vout = 5 V, Iout = 2 A, fs = 500000 Hz, ripple_ratio = 0.3. Direct arithmetic gives D = 5/12 = 0.4166666666666667; delta_I = 0.3 × 2 = 0.6 A; on-time = 0.8333333333 microseconds; Lmin = 7 × (5/12)/(500000 × 0.6) = 9.722222222222223 microhenries. Ipeak = 2.3 A; Ivalley = 1.7 A > 0. Every JSON output is included in the example.

**Sources:** S1 section 2.1; S2 section 3.

## 2. `power.boost_inductor_design` — intermediate

**Physical problem:** size the boost input-energy inductor and its larger input-current stress. Although the same element law is used, the topology and average-current relationship differ from a buck.

For boost on-state voltage Vin and off-state voltage Vin − Vout, volt-second balance gives `D = 1 − Vin/Vout`. The output path conducts only during off-time; charge balance gives `Iout = (1 − D) ILavg`. Thus

`ILavg = Iout/(1 − D)`, `delta_I = ripple_ratio ILavg`,

`Lmin = Vin D/(fs delta_I)`,

`Ipeak,valley = ILavg +/- delta_I/2`.

Dimensional check again yields H. The ripple fraction is referenced to **average input-inductor current**, not load current. S3 section 3 uses the equivalent ideal ratio Vout/Vin.

**Domain:** Vout > Vin > 0; Iout, fs positive; 0 < ripple_ratio < 2. This guarantees positive valley, but real losses raise input-current demand and ratings must include margin. Ideal voltage conversion and current stress are not predictions of a selected real IC.

**Independent worked example/defaults:** Vin = 5 V, Vout = 12 V, Iout = 0.5 A, fs = 500000 Hz, ripple_ratio = 0.3. D = 7/12 = 0.5833333333333334; ILavg = 0.5/(5/12) = 1.2 A; delta_I = 0.36 A; Lmin = 5 × (7/12)/(500000 × 0.36) = 16.203703703703704 microhenries. Ipeak = 1.38 A; Ivalley = 1.02 A. Vin × ILavg = 6 W equals Vout × Iout as an independent ideal-power consistency identity.

**Sources:** S3 sections 2–3; S5 boost duty/topology discussion.

## 3. `power.buck_output_capacitor` — intermediate

**Physical problem:** size buck output capacitance from triangular capacitor charge and an ESR ripple budget; not the inductor-sizing inverse.

Output-capacitor current is the AC part of inductor current. The positive triangle has base half a cycle and height delta_I/2; its area is `delta_Q = delta_I/(8 fs)`. Hence ideal capacitor ripple is `delta_I/(8 fs C)`. ESR contributes `ESR delta_I` peak-to-peak. Allocate the ripple budget conservatively:

`ripple_esr = esr delta_i`,

`ripple_cap_budget = ripple_budget − ripple_esr`,

`Cmin = delta_i/(8 fs ripple_cap_budget)`.

Dimensional check `A/(Hz V) = F`. Summing two peak-to-peak excursions bounds the total; it does not assert that the extrema occur simultaneously.

**Domain:** delta_i, fs, iout and vout positive; ESR >= 0; Iout > delta_i/2 for CCM; ripple_budget > ESR delta_i; ripple_budget <= 0.1 Vout for the small-ripple model. A zero remaining budget is rejected, not replaced with a fabricated capacitor. Switching spikes/ESL and load steps are excluded.

**Independent worked example/defaults:** delta_i = 0.6 A, iout = 2 A, vout = 5 V, fs = 500000 Hz, ripple_budget = 0.02 V, esr = 0.01 ohm. ESR ripple = 0.006 V; remaining capacitive budget = 0.014 V. Charge excursion = 0.6/(8 × 500000) = 150 nanocoulombs. Cmin = 150 nC/14 mV = 10.714285714285714 microfarads. Valley = 1.7 A; 20 mV <= 500 mV, so all model constraints pass mathematically.

**Source:** S2 section 7, equations for minimum output C and ESR ripple.

## 4. `power.boost_output_capacitor` — expert

**Physical problem:** size boost output capacitance for the output-current interruption during switch on-time. Unlike buck, the capacitor must temporarily supply the **entire load**, so its charge law and ESR current step are different.

During on-time, output-path current is zero and capacitor current is −Iout. The charge deficit is `Iout D/fs`. During off-time, capacitor current is `IL − Iout`. If the inductor valley remains at least Iout, the capacitor charges monotonically throughout off-time; the on-time deficit is then the full capacitive peak-to-peak charge excursion. We enforce this extra guard rather than silently extrapolating the common approximation near the CCM boundary.

Peak output-path current is `Ipeak = Iout/(1 − D) + delta_i/2`; the switching transition changes capacitor current by this amount, so the ESR bound is `esr Ipeak`, **not** `esr delta_i`.

`D = 1 − Vin/Vout`,

`ripple_esr = esr Ipeak`,

`ripple_cap_budget = ripple_budget − ripple_esr`,

`Cmin = Iout D/(fs ripple_cap_budget)`.

**Domain:** Vout > Vin > 0; Iout, fs and delta_i positive; ESR nonnegative; `Iout Vout/Vin − delta_i/2 >= Iout`. The last condition implies CCM and excludes off-interval voltage reversal caused by too much ripple. Also require positive residual ripple budget and budget <= 10% Vout. Capacitive and ESR excursions are conservatively added; ESR and capacitance must reflect actual operating bias/frequency. This is not a diode-loss or bridge-rectifier calculation.

**Independent worked example/defaults:** Vin = 5 V, Vout = 12 V, Iout = 0.5 A, delta_i = 0.36 A, fs = 500000 Hz, ripple_budget = 0.05 V, esr = 0.01 ohm. D = 7/12 = 0.5833333333333334; Ipeak = 1.2 + 0.18 = 1.38 A; ESR ripple = 0.0138 V; capacitive budget = 0.0362 V. On-time charge deficit = 0.5 × (7/12)/500000 = 583.3333333333 nC. Cmin = 583.3333333333 nC/36.2 mV = 16.114180478821363 microfarads. Valley = 1.02 A >= 0.5 A, so the monotonic-charging guard holds; 50 mV <= 1.2 V.

**Sources:** S3 section 7 (C equation 12, ESR equation 13); S5 boost topology/duty. The additional valley >= load guard is derived directly above from capacitor current sign.

## 5. `power.buck_input_capacitor_rms` — expert

**Physical problem:** rate the input capacitor bank for switching-current stress. This is separate from capacitance/output-ripple sizing and does not compute capacitor temperature.

The switch input current equals the triangular inductor current during duty D and zero otherwise. The DC average is D Iout. Its mean square is `D (Iout^2 + delta_i^2/12)`, because a zero-mean triangular ripple has variance delta_i^2/12. Subtracting the squared DC average yields

`ICin_rms^2 = Iout^2 D(1 − D) + D delta_i^2/12`,

`D = Vout/Vin`.

The source supplies the DC average and the local capacitor supplies the AC remainder. Units inside sqrt are A squared.

**Domain:** Vin > Vout > 0; Iout > 0; delta_i >= 0; Iout > delta_i/2. These guards ensure 0 < D < 1, CCM and a nonnegative square-root argument. Assumes the entire AC current flows through the local bank; source inductance/filter resonance, interleaving and individual capacitor sharing are not represented. No fs input is needed once delta_i is given.

**Independent worked example/defaults:** Vin = 12 V, Vout = 5 V, Iout = 2 A, delta_i = 0.6 A. D = 5/12 = 0.4166666666666667. Mean-square AC current = `4 × (5/12)(7/12) + (5/12) × 0.36/12` = `35/36 + 1/80` = 0.9847222222222222 A squared. Taking its square root gives ICin_rms = 0.9923317097736131 A. Ideal DC input current 5/6 A was used only for the derivation, not counted as another calculator output.

**Source:** S4 equation 2.

## 6. `power.ldo_dissipation` — basic

**Physical problem:** quantify heat electrically generated by an LDO, including loaded ground current; not a temperature/thermal-resistance calculation.

KCL gives `Iin = Iout + Ignd`; input power minus load power gives

`Pdiss = Vin(Iout + Ignd) − Vout Iout`

`= (Vin − Vout) Iout + Vin Ignd`.

Report pass-path loss, ground-current loss and total loss separately. V × A = W.

**Domain:** Vin, Vout positive; Iout, Ignd, dropout >= 0; `Vin − Vout >= required_dropout`. Dropout is a user-supplied requirement for the actual load/temperature from the selected device's data sheet. Ground current must be its loaded operating value, not blindly substituted no-load IQ. Separate bias rails, reverse conduction, external feedback-dividers and disabled operation are outside this single-supply regulator model. At zero load and zero ground current zero dissipation is valid.

**Independent worked example/defaults:** Vin = 5 V, Vout = 3.3 V, Iout = 0.2 A, Ignd = 50 microamps, required_dropout = 0.2 V. Ppass = 1.7 × 0.2 = 0.34 W; Pgnd = 5 × 0.00005 = 0.00025 W; Pdiss = 0.34025 W. Headroom 1.7 V >= 0.2 V. Independent power balance: Pin = 5 × 0.20005 = 1.00025 W; Pout = 3.3 × 0.2 = 0.66 W; difference = 0.34025 W.

**Source:** S6 equation 1, operating-current definitions and dropout discussion.

## 7. `power.converter_input_budget` — basic

**Physical problem:** determine DC source-current demand and aggregate conversion loss from a specified efficiency. Unlike the LDO entry, this does not infer loss from the pass-voltage mechanism and applies to either step-up or step-down conversion.

The definition of power efficiency is `eta = Pout/Pin`. With `Pout = Vout Iout`, obtain

`Pin = Pout/eta`, `Iin = Pin/Vin`, `Ploss = Pin − Pout`.

Total efficiency is an operating-point assumption/measurement, not calculated from duty or selected MOSFET parameters. Power and current remain W and A.

**Domain:** Vin, Vout, Iout positive; 0 < eta <= 1. Zero-load efficiency is deliberately excluded because it is generally undefined without an explicit quiescent-power model. Efficiency must already include control and ground-current consumption. No further quiescent term is added, preventing double counting. This is DC budgeting only, not a startup or ripple-current rating.

**Independent worked example/defaults:** Vin = 12 V, Vout = 5 V, Iout = 2 A, eta = 0.9. Pout = 10 W; Pin = 10/(9/10) = 100/9 = 11.11111111111111 W; Iin = (100/9)/12 = 25/27 = 0.9259259259259259 A; Ploss = 10/9 = 1.1111111111111112 W. Efficiency is 10/(100/9) = 0.9 and loss is nonnegative.

**Sources:** S6 equation 2 defines power efficiency; supplementary SLVA118A section 3 gives input/output power and conservation-of-energy loss equations. The algebra applies generally to DC power, even though those references illustrate it with an LDO.

## 8. `power.boost_rhp_zero` — expert

**Physical problem:** estimate the dynamic nonminimum-phase limitation of a boost control plant, not its steady-state inductor size or a passive RLC resonance.

In the averaged CCM model, increasing duty initially lengthens the interval with no output delivery. Output voltage initially falls even though its final steady-state value rises. S5 gives the right-half-plane zero

`fRHPZ = Vout (1 − D)^2/(2 pi L Iout)`.

Use `Rload = Vout/Iout` and ideal `D = 1 − Vin/Vout` to express it as `Rload(1 − D)^2/(2 pi L)`. Since ohm/H = per second, division by 2 pi converts the angular frequency to Hz. Report `f_crossover_guideline = fRHPZ/10`, the conservative end of S5's usual one-fifth-to-one-tenth bandwidth guideline. This output is **not** a computed stable crossover or compensation design.

**Domain:** Vout > Vin > 0; Iout, L, fs positive; resistive incremental load; CCM guard

`Iout Vout/Vin > Vin(1 − Vin/Vout)/(2 fs L)`.

The left side is average inductor current; the right side is half the predicted switching ripple. Constraints are expanded in input keys because the implementation evaluates constraints before outputs. Constant-power loads, DCM, multi-phase behavior, significant winding resistance and nonlinear control invalidate this plant estimate. Switching frequency remains necessary for the CCM check even though it does not appear directly in fRHPZ. Actual loop crossover must also remain well below switching frequency and satisfy a complete loop-gain/phase design; the calculator does not certify this.

**Independent worked example/defaults:** Vin = 5 V, Vout = 12 V, Iout = 0.5 A, L = 22 microhenries, fs = 500000 Hz. D = 7/12 = 0.5833333333333334; Rload = 24 ohm. Thus `fRHPZ = 24(5/12)^2/(2 pi × 22e-6) = 25/(12 pi × 22e-6)` = 30142.9816461923 Hz. Guideline = 3014.29816461923 Hz. Ripple = `5 × (7/12)/(500000 × 22e-6)` = 0.2651515151515152 A; average = 1.2 A; valley = 1.0674242424242424 A > 0, so CCM holds.

**Sources:** S5 boost equation 7 and bandwidth discussion; S3 ripple equation 2 supplies the CCM domain check.

## Implementation and handoff notes

- JSON contains exactly the eight IDs above with no copied inverse calculation used to inflate the count.
- Constraints reference input keys/constants only; sequential outputs reference only inputs and earlier outputs. Every example lists every input and expected output.
- Strictly positive divisors are guarded by input ranges or topology/ESR constraints. All provided default/example domains hold by the arithmetic shown above. Independent values were obtained from rational arithmetic and high-precision decimal square root/pi calculations, not the application engine.
- Expressions require only arithmetic, exponentiation, `sqrt` and `pi`; no conditional or numerical simulation is needed.
- No build, engine tests, formatter or linter was run. Parent should validate catalog schema/constraints, execute these full examples, and exercise invalid topology, nonpositive divisors, CCM boundaries, exhausted ESR budget, dropout violation and invalid efficiency when integrating.
