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

