# Power conversion and regulation calculators

The executable definitions in [the power catalog](../../backend/catalog/power.json) run through the shared Rust expression engine; Flutter displays inputs and results and performs no domain mathematics. Each calculation consumes independent inputs. Supporting duty/current/budget outputs do not create extra calculators.

## Units and model conventions

Enter SI units: V, A, Hz, H, F, ohm and W. Duty, efficiency and ripple ratios use unit `1`; enter 0.9 rather than 90 for 90% efficiency. Ripple is peak-to-peak unless explicitly labeled RMS. Effective L and C mean their values after operating-bias, temperature and tolerance effects. All inputs must be finite and match the calculator's keys. The engine applies scalar ranges and input-only model constraints before evaluating outputs in listed order; invalid domains and nonfinite arithmetic produce errors, not fabricated component selections.

Buck/boost models are ideal, single-phase, fixed-frequency CCM steady-state models. Duty comes from volt-second balance, not efficiency-adjusted heuristics: a scalar efficiency does not identify winding or semiconductor voltage drops. Output-ripple estimates neglect ESL/spikes and conservatively sum capacitor and ESR peak-to-peak excursions even though their extrema can differ. The 10% output-voltage ripple ceiling is an explicit catalog small-ripple guard, not a universal manufacturer limit.

Real design sign-off still needs controller duty/current limits and compensation, tolerances, nonlinear derating, component ratings, parasitics, load steps and measurement. These calculations do not certify stability, standards compliance, reliability, saturation margin or temperature. Missing real-device/material information remains a visible assumption rather than a fake fallback.

## Source register

The research handoff [power research](../../research/power.md) records manufacturer-source access on 2026-10-09. The sources below were directly read during research; this implementation preserves that provenance and does not claim new source verification or experimental evidence. All seven references are from TI, not seven independent experimental groups.

- **S1** — Everett Rogers, [TI SLVA057: Understanding Buck Power Stages in Switchmode Power Supplies](https://www.ti.com/lit/an/slva057/slva057.pdf), section 2.1: ideal CCM buck volt-second balance.
- **S2** — Brigitte Hauke, [TI SLVA477: Basic Calculation of a Buck Converter's Power Stage](https://www.ti.com/lit/an/slva477b/slva477b.pdf), sections 3 and 7: inductance and output-capacitor ripple. Research notes the URL redirects to revision C; section/topic is more robust than revision-dependent equation numbers.
- **S3** — Brigitte Hauke, [TI SLVA372D: Basic Calculation of a Boost Converter's Power Stage](https://www.ti.com/lit/an/slva372d/slva372d.pdf), sections 2, 3 and 7: boost current stress and capacitor sizing.
- **S4** — Manjing Xie, [TI SSZTBI6: Power Tips: How to Select Ceramic Capacitors to Meet Ripple-Current Requirements](https://www.ti.com/lit/ta/ssztbi6/ssztbi6.pdf), equation 2: buck input-capacitor RMS current. Its sharing simulation does not establish arbitrary-layout current sharing.
- **S5** — Markus Zehendner, [TI SLVAFJ5: How to Approach a Power-Supply Design – Part 3](https://www.ti.com/lit/ab/slvafj5/slvafj5.pdf), boost topology, equation 7 and bandwidth discussion: ideal duty/RHP zero and heuristic crossover restriction.
- **S6** — Wilson Fwu, [TI SLYY206: Understanding the Foundations of Quiescent Current in Linear Power Systems](https://www.ti.com/lit/wp/slyy206/slyy206.pdf), equations 1–3 and dropout: operating ground current and power conservation.
- **S7** — [TI SLVA118A: Linear Regulator Design Guide For LDOs](https://www.ti.com/lit/an/slva118a/slva118a.pdf), section 3, equations 1–4: supplementary power-conservation derivation.

## `power.ldo_dissipation` — basic

**Purpose:** estimate electrical heat generated in an enabled, regulated single-supply LDO, including loaded ground current; this is not a thermal-temperature solve. Source: S6 equation 1 and operating-current/dropout discussion.

KCL gives `Iin = Iout + Ignd`, so `Pdiss = Vin(Iout+Ignd)-Vout Iout`. Sequential outputs are `p_pass = (vin-vout)*iout`, `p_ground = vin*i_gnd`, and `p_diss = p_pass+p_ground`, all W (V × A).

**Inputs and defaults:** `vin=5 V`, `vout=3.3 V`, `iout=0.2 A`, `i_gnd=0.00005 A`, `v_dropout=0.2 V`. Voltages must be positive; currents and dropout may be zero. Ground current is its actual operating value, not automatically no-load IQ. Supply separate bias, reverse-current, shutdown and external-divider losses outside this model.

**Model-domain errors:** reject `vin-vout < v_dropout`; insufficient headroom invalidates the assumption of regulation and can otherwise create negative pass-path loss. Equality is allowed. Current ratings, absolute maximum limits and thermal feasibility cannot be inferred from this headroom guard. Zero load and zero ground current legitimately give zero dissipation.

**Independent worked outputs (default input vector; relative tolerance 1e-6):**

| Output | Derivation | Expected |
|---|---|---|
| `p_pass` | `(5-3.3)*0.2` | `0.34 W` |
| `p_ground` | `5*0.00005` | `0.00025 W` |
| `p_diss` | `0.34+0.00025` | `0.34025 W` |

Headroom is 1.7 V ≥ 0.2 V. Independent conservation identity: `Pin=5*0.20005=1.00025 W`; `Pout=3.3*0.2=0.66 W`; their difference is 0.34025 W. These are independently derived expected values, not engine execution results.

## `power.boost_output_capacitor` — expert

**Purpose:** size boost output capacitance for the interval when the switch disconnects delivery to the load. Sources: S3 section 7, equations 12–13, and S5 topology/duty; the additional monotonic-charge restriction below is independently derived.

During switch on-time capacitor current is `-Iout`, with charge deficit `Iout D/fs`. During off-time current is `IL-Iout`. Requiring valley `IL >= Iout` makes charging monotonic, so the deficit equals the full capacitor charge excursion. The ESR excursion is the full peak output-path current step, **not** merely inductor ripple.

Sequential outputs: `duty=(vout-vin)/vout`; `i_peak=iout*vout/vin+delta_i/2`; `ripple_esr=esr*i_peak`; `ripple_cap_budget=ripple_budget-ripple_esr`; `c_min=iout*duty/(fs*ripple_cap_budget)`. Units are `1`, A, V, V, F; `A/(Hz V)=F`. Difference-before-division duty avoids rounding a nearly unity `vin/vout` to one. Peak current uses the direct voltage ratio, not division by cancellation-prone `1-duty` at high conversion ratio; these are algebraically the same ideal model.

**Inputs/defaults:** `vin=5 V`, `vout=12 V`, `iout=0.5 A`, `delta_i=0.36 A`, `fs=500000 Hz`, `ripple_budget=0.05 V`, `esr=0.01 ohm`. All except ESR must be positive; ESR may be zero.

**Model-domain errors:** reject `vout<=vin`; reject `iout*vout/vin-delta_i/2 < iout`, since off-time discharge invalidates the simple charge-deficit formula even in some CCM cases. Equality remains valid. Reject `ripple_budget <= esr*(iout*vout/vin+delta_i/2)`: no positive capacitive budget remains and a finite C cannot solve it. Reject budgets above `0.1*vout` to preserve the catalog small-ripple domain. Constant load, triangular CCM, effective C, constant ESR and negligible ESL are assumptions; actual ratings/compensation/load steps remain separate.

**Independent worked outputs (default vector; relative tolerance 1e-6):**

| Output | Derivation | Expected |
|---|---|---|
| `duty` | `7/12` | `0.5833333333333334` |
| `i_peak` | `0.5*12/5+0.36/2` | `1.38 A` |
| `ripple_esr` | `0.01*1.38` | `0.0138 V` |
| `ripple_cap_budget` | `0.05-0.0138` | `0.0362 V` |
| `c_min` | `(0.5*7/12)/(500000*0.0362)` | `0.000016114180478821363 F` |

The charge deficit is 583.3333333333 nC. Valley is 1.02 A ≥ 0.5 A; 50 mV ≤ 1.2 V. The summed ripple bound at Cmin is exactly the requested 50 mV analytically; it is not a prediction of coincident extrema.

## `power.buck_input_capacitor_rms` — expert

**Purpose:** estimate switching-current stress in the entire local buck input-capacitor bank, not output capacitance or capacitor temperature. Source: S4 equation 2.

The input switch current is the triangular inductor current during duty D and zero otherwise. Its mean is `D Iout`, mean square is `D(Iout^2+delta_i^2/12)`, and the AC variance is the mean square minus squared mean. Thus `ICin_rms^2=Iout^2 D(1-D)+D delta_i^2/12`. The `1/12` coefficient follows the variance of zero-mean triangular ripple.

Sequential outputs are `duty=vout/vin` (unit `1`) and `i_cin_rms=sqrt(iout^2*duty*((vin-vout)/vin)+duty*delta_i^2/12)` (A). The direct off-fraction `(vin-vout)/vin` is algebraically `1-D` but avoids loss of a small valid off-time through ratio/subtraction cancellation. Terms inside sqrt have units A².

**Inputs/defaults:** `vin=12 V`, `vout=5 V`, `iout=2 A`, `delta_i=0.6 A`. Voltages/load must be positive; zero ripple is admissible. Frequency is unnecessary once ripple is supplied.

**Model-domain errors:** reject `vin<=vout` and `iout<=delta_i/2`; they invalidate the buck/positive-valley CCM assumptions. These guards make the square-root argument nonnegative. The source must supply only DC, with all switching AC in the local bank; source impedance/filter resonance, interleaving and individual-capacitor sharing remain outside the model.

**Independent worked outputs (default vector; relative tolerance 1e-6):**

| Output | Derivation | Expected |
|---|---|---|
| `duty` | `5/12` | `0.4166666666666667` |
| `i_cin_rms` | `sqrt(4*(5/12)*(7/12)+(5/12)*0.36/12)` | `0.9923317097736131 A` |

Mean-square AC current is `35/36+1/80=0.9847222222222222 A²`; the independent DC mean is 5/6 A. At zero ripple, the formula reduces to `Iout*sqrt(D(1-D))` and peaks at D=1/2; no single-capacitor rating or temperature claim follows.

## `power.boost_rhp_zero` — expert

**Purpose:** estimate the nonminimum-phase dynamic limitation of an averaged boost CCM duty-to-output plant, not passive resonance or a stable compensation design. Source: S5 equation 7/bandwidth discussion; S3 ripple equation 2 supplies the CCM check.

Increasing duty initially removes output-delivery time: voltage initially falls even though its final steady-state value rises. The model zero is `fRHPZ=Vout(1-D)^2/(2*pi*L*Iout)`. For a resistive incremental load, sequential outputs are `duty=(vout-vin)/vout`, `r_load=vout/iout`, `f_rhp_zero=r_load*(vin/vout)^2/(2*pi*inductance)` and `f_crossover_guideline=f_rhp_zero/10`. Units: `1`, ohm, Hz, Hz; ohm/H is s⁻¹ and `2*pi` converts angular frequency to Hz.

The direct off-fraction `vin/vout` avoids reconstructing a very small `1-D` by subtracting an almost-one duty; difference-before-division duty preserves near-unity-conversion on-time. No physical model or expected value changes.

**Inputs/defaults:** `vin=5 V`, `vout=12 V`, `iout=0.5 A`, `inductance=0.000022 H`, `fs=500000 Hz`. Every input must be positive.

**Model-domain errors:** reject `vout<=vin` and reject `iout*vout/vin <= vin*((vout-vin)/vout)/(2*fs*inductance)`. The latter compares average inductor current with half predicted ripple, requiring strictly positive valley for CCM. Frequency is needed for this guard although absent from fRHPZ itself. The incremental load must be resistive: constant-power loads, DCM, nonlinear operation, interleaving and material loss/drop departures invalidate this ideal plant estimate.

The one-tenth guideline is the conservative end of TI's one-fifth-to-one-tenth range, not computed closed-loop crossover. You must still design full loop gain/phase and keep crossover well below switching frequency; no stability certification or silent switching-frequency clamp is applied.

**Independent worked outputs (default vector; relative tolerance 1e-6):**

| Output | Derivation | Expected |
|---|---|---|
| `duty` | `7/12` | `0.5833333333333334` |
| `r_load` | `12/0.5` | `24 ohm` |
| `f_rhp_zero` | `24*(5/12)^2/(2*pi*22e-6)=25/(12*pi*22e-6)` | `30142.9816461923 Hz` |
| `f_crossover_guideline` | `30142.9816461923/10` | `3014.29816461923 Hz` |

Independent CCM check: ripple `5*(7/12)/(500000*22e-6)=0.2651515151515152 A`; average 1.2 A; valley 1.0674242424242424 A > 0. Holding voltages/load fixed, doubling L halves fRHPZ; no steady-state sizing feature is duplicated.

## `power.converter_input_budget` — basic

**Purpose:** estimate DC supply demand and aggregate conversion loss from specified operating-point total power efficiency. It applies to step-up or step-down conversion without inferring topology duty or device losses. Sources: S6 equation 2 and S7 section 3; their LDO examples establish conservation identities, not a universal switching-loss model.

From `eta=Pout/Pin`, sequential outputs are `p_out=vout*iout`, `p_in=p_out/efficiency`, `i_in=p_in/vin`, and `p_loss=p_out*((1-efficiency)/efficiency)`. Power outputs use W; current uses A. The loss expression is algebraically `Pin-Pout` but avoids subtracting separately rounded near-equal powers near unity efficiency. At eta=1 loss is exactly zero; this is not a clamp.

**Inputs/defaults:** `vin=12 V`, `vout=5 V`, `iout=2 A`, `efficiency=0.9` (dimensionless). Voltage/current must be positive and `0<efficiency<=1`. No buck/boost voltage-order constraint applies.

**Model-domain errors:** nonpositive voltage/current invalidates this positive-load power-efficiency model; nonpositive efficiency divides by zero or implies nonphysical budgeting, and efficiency above one implies negative passive conversion loss. Zero-load efficiency is deliberately excluded because it requires an explicit quiescent-power model. Total efficiency already includes control/ground losses, so do not add them again. Startup surge, switching ripple, transients and multi-rail bias power are excluded.

**Independent worked outputs (default vector; relative tolerance 1e-6):**

| Output | Derivation | Expected |
|---|---|---|
| `p_out` | `5*2` | `10 W` |
| `p_in` | `10/(9/10)=100/9` | `11.11111111111111 W` |
| `i_in` | `(100/9)/12=25/27` | `0.9259259259259259 A` |
| `p_loss` | `10*(1/10)/(9/10)=10/9` | `1.1111111111111112 W` |

Conservation identities: `Pin=Pout+Ploss` and `Pout/Pin=0.9`. At unity efficiency the same vector gives `Pin=10 W`, `Iin=5/6 A`, `Ploss=0 W`; changing Vin alone inversely scales DC source current.

## `power.buck_inductor_design` — intermediate

**Purpose:** size the energy-transfer inductor of an ideal buck at a chosen load/ripple design point. Sources: S1 section 2.1 and S2 section 3. Neither duty inversion nor stored energy is a separate feature.

Volt-second balance `(Vin-Vout)D - Vout(1-D)=0` gives `D=Vout/Vin`. During on-time `D/fs`, `di/dt=(Vin-Vout)/L`. With target `delta_I=r Iout`, the minimum effective L is `(Vin-Vout)D/(fs*delta_I)`. Capacitor charge balance gives `ILavg=Iout`.

Sequential outputs are `duty=vout/vin`, `delta_i=ripple_ratio*iout`, `l_min=(vin-vout)*duty/(fs*delta_i)`, `i_peak=iout*(1+ripple_ratio/2)` and `i_valley=iout*(1-ripple_ratio/2)`. Units are `1`, A, H, A, A; `V/(Hz A)=H`. Factored endpoint expressions are equivalent to `Iout +/- delta_I/2` but avoid separately rounded large-current subtraction near the CCM boundary.

**Inputs/defaults:** `vin=12 V`, `vout=5 V`, `iout=2 A`, `fs=500000 Hz`, `ripple_ratio=0.3` (unit `1`). All dimensional inputs are positive; `0<ripple_ratio<2`.

**Model-domain errors:** reject `vin<=vout` (not buck step-down), nonpositive fs/load/ripple (undefined sizing denominator/design point), and ratio ≥ 2 (zero/negative valley, not CCM). Ratio is peak-to-peak ripple divided by output mean, not RMS. Effective Lmin/endpoints describe the target design; larger L reduces ripple. Switch drops, winding loss, duty/current limits, saturation, tolerance and load-range guarantees are excluded.

**Independent worked outputs (default vector; relative tolerance 1e-6):**

| Output | Derivation | Expected |
|---|---|---|
| `duty` | `5/12` | `0.4166666666666667` |
| `delta_i` | `0.3*2` | `0.6 A` |
| `l_min` | `7*(5/12)/(500000*0.6)` | `0.000009722222222222223 H` |
| `i_peak` | `2*(1+0.3/2)` | `2.3 A` |
| `i_valley` | `2*(1-0.3/2)` | `1.7 A` |

On-time is 0.8333333333 μs; `(Vin-Vout)*ton/Lmin=0.6 A`. Endpoint mean equals 2 A and difference equals 0.6 A. Positive valley and `7D-5(1-D)=0` independently establish the stated ideal operating point.

## `power.boost_inductor_design` — intermediate

**Purpose:** size a boost input-energy inductor and its input-current stress. Unlike buck, load current is not the inductor mean. Sources: S3 sections 2–3 and S5 duty/topology.

Boost on-state voltage is Vin and off-state voltage is Vin−Vout. Volt-second balance gives `D=1-Vin/Vout`. The output path conducts only off-time, so `Iout=(1-D)ILavg`. With `delta_I=r ILavg`, `Lmin=Vin D/(fs delta_I)`.

Sequential outputs: `duty=(vout-vin)/vout`; `i_l_avg=iout*vout/vin`; `delta_i=ripple_ratio*i_l_avg`; `l_min=vin*duty/(fs*delta_i)`; `i_peak=i_l_avg*(1+ripple_ratio/2)`; `i_valley=i_l_avg*(1-ripple_ratio/2)`. Units: `1`, A, A, H, A, A. `V/(Hz A)=H`. Direct mean current avoids division by `1-duty` rounding to zero at high boost ratio, and factored endpoint currents preserve small positive valley near ratio=2. These are equivalent ideal equations, not loss corrections.

**Inputs/defaults:** `vin=5 V`, `vout=12 V`, `iout=0.5 A`, `fs=500000 Hz`, `ripple_ratio=0.3` (unit `1`). Dimensional inputs must be positive; `0<ripple_ratio<2`.

**Model-domain errors:** reject `vout<=vin`, nonpositive fs/load/ripple, and ratio ≥ 2. The ratio denominator is average **input-inductor** current, never output load current; the upper strict bound enforces CCM. Real losses raise current demand. Effective L, saturation rating, tolerances, controller duty/current limits and compensation must be checked separately; no selected IC performance is predicted.

**Independent worked outputs (default vector; relative tolerance 1e-6):**

| Output | Derivation | Expected |
|---|---|---|
| `duty` | `7/12` | `0.5833333333333334` |
| `i_l_avg` | `0.5*12/5` | `1.2 A` |
| `delta_i` | `0.3*1.2` | `0.36 A` |
| `l_min` | `5*(7/12)/(500000*0.36)` | `0.000016203703703703704 H` |
| `i_peak` | `1.2*(1+0.3/2)` | `1.38 A` |
| `i_valley` | `1.2*(1-0.3/2)` | `1.02 A` |

Independent ideal-power identity: `Vin*ILavg=5*1.2=6 W=12*0.5=Vout*Iout`. Volt-second identity `5*(7/12)+(5-12)*(5/12)=0` and positive valley validate the mathematical default domain, not real converter sign-off.

## `power.buck_output_capacitor` — intermediate

**Purpose:** size effective buck output capacitance by allocating a peak-to-peak ripple budget after ESR. This is the capacitor charge problem, not an inverse inductor-size feature. Source: S2 section 7; section/topic is used because equation numbering varies with revision.

Capacitor current is the inductor AC ripple. Its positive triangle has base half a cycle and height `delta_I/2`, so charge excursion is `delta_Q=delta_I/(8 fs)`. Capacitive ripple is `delta_I/(8 fs C)`; ESR ripple is `ESR delta_I`. Their summed peak-to-peak bound is conservative because extrema need not coincide.

Sequential outputs: `ripple_esr=esr*delta_i`, `ripple_cap_budget=ripple_budget-ripple_esr`, `c_min=delta_i/(8*fs*ripple_cap_budget)`. Units: V, V, F; `A/(Hz V)=F`.

**Inputs/defaults:** `delta_i=0.6 A`, `iout=2 A`, `vout=5 V`, `fs=500000 Hz`, `ripple_budget=0.02 V`, `esr=0.01 ohm`. All except ESR must be positive; ESR may be zero.

**Model-domain errors:** reject `iout<=delta_i/2` because positive-valley CCM is required; reject `ripple_budget<=esr*delta_i` because no positive capacitive budget remains; reject `ripple_budget>0.1*vout` because the catalog assumes small ripple. A zero remaining budget does not receive a fabricated finite C. Constant load, triangular current, resistive ESR at switching frequency and negligible ESL are qualitative assumptions. Bias/temperature/tolerance derating, loop compensation and load-step response can require a larger real capacitor bank.

**Independent worked outputs (default vector; relative tolerance 1e-6):**

| Output | Derivation | Expected |
|---|---|---|
| `ripple_esr` | `0.01*0.6` | `0.006 V` |
| `ripple_cap_budget` | `0.02-0.006` | `0.014 V` |
| `c_min` | `0.6/(8*500000*0.014)` | `0.000010714285714285714 F` |

Charge excursion is `0.6/(8*500000)=150 nC`; `150 nC/14 mV=10.714285714285714 μF`. Valley is 1.7 A > 0 and budget 20 mV ≤ 500 mV. At Cmin the analytical conservative ripple bound is 6 mV + 14 mV = 20 mV; at zero ESR the complete budget is available for capacitance.

## Numerical and integration handoff

All eight original IDs, input/output keys, nominal default vectors and complete independent expected maps are preserved. Examples use relative tolerance `1e-6`; tables and identities are mathematical expectations, not observations from the Rust engine. Constraints reference inputs/constants only, while output expressions use inputs and earlier outputs. No second evaluator or frontend math is introduced.

Algebraic conditioning changes preserve the physical domains: direct boost voltage ratios rather than recovering off-fraction from rounded duty; voltage-difference-before-division duty/off-time; factored inductor endpoints near the CCM limit; efficiency loss as `Pout*(1-eta)/eta` rather than near-equal power subtraction. Under extreme magnitudes floating-point overflow/underflow remains possible; the shared engine reports arithmetic errors, and no undocumented physical range restriction or output clamp is added. A duty rounded to one at enormous boost ratio does not authorize reconstructing off-fraction from that displayed value.

No builds, lint, tests, formatters or engine/smoke runs were performed by this lane. Main should exercise every full example, defaults, topology/range/CCM guards, boost-capacitor monotonic-charge equality/rejection, exhausted ESR budgets, 10% boundaries, zero ESR/ripple where admissible, zero LDO load, dropout equality/rejection and unity efficiency. Independent conditioning scenarios should include a high boost ratio, nearly equal voltage rails, ripple ratio just below two, and efficiency just below one; real desktop/protocol acceptance belongs to integration.
