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
