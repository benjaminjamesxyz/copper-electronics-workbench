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
