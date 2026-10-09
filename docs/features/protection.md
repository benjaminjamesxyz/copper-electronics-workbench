# Protection calculators

These definitions run in the shared Rust engine. Flutter supplies static explanatory diagrams, not numerical models or live plots. Source-access statements below are inherited from the research handoff, not new source fetches. No implementation checks were run. Complete defaults and independent examples are retained at relative tolerance 1e-6; this is mathematical tolerance, not a physical accuracy guarantee.

## Sources and applicability

- **P1 — SCHURTER, Fuse-Link**, manufacturer knowledge base, sections 10 (pre-arcing versus clearing), 11 (breaking capacity), and 13 (pulse strength and thermal behavior): <https://www.schurter.com/en/knowledge/fuse-link>. Section 13 explicitly defines the square-current integral, near-constant pre-arcing integral for high-current melting times below 10 ms, pulse comparison against pre-arcing, and semiconductor comparison against total operating integral.
- **P2 — Texas Instruments, SLVAE37, “How to select a Surge Diode,” January 2019**, manufacturer application report, sections 2.3–2.4 and Figure 3: <https://www.ti.com/lit/an/slvae37/slvae37.pdf>. Explicitly gives `VCLAMP = VBR + ISURGE RDYN`, matching waveform requirements, pulse-current derating, and the distinction between standoff, breakdown and clamp voltage. Some prose examples in section 3 contain typographical/dimensional slips; none is used as a numerical oracle here.
- **P3 — Bruno van Beneden, Vishay BCcomponents, “Varistors: Ideal Solution to Surge Protection,” May 2003**, manufacturer-authored technical paper hosted by Vishay, “Varistor Operation” and “Selection Criteria”: <https://www.vishay.com/docs/49305/49305.pdf>. Gives `I = K V^alpha`, inverse `V = C I^beta`, protection-region limitations, `P = VI`, integrated pulse energy and waveform dependence. Its generalized reliability language is not adopted as a safety guarantee.
- **P4 — STMicroelectronics, AN4607 Rev 1, June 2017**, manufacturer application note, sections 1.1–1.2 and 2.12, Figures 2 and 15; actual PDF read through distributor mirror: <https://datasheet.octopart.com/TXN612RG-STMicroelectronics-datasheet-97040648.pdf>. Describes positive gate current, regenerative latching, distinct latching/holding currents and zener-triggered crowbar applications. The source defines device behavior; the added resistor-network equations below are independently derived KCL/KVL, not quotations from ST.
- **P5 — Liban Cables, Generalities, section 5.3**, cable-manufacturer engineering guide, copper-conductor page: <https://www.libancables.com/.rest/eservice/dam/v1/file/224143/Generalities_17.pdf>. Gives the copper temperature-dependent adiabatic expression with constants 226 and 234, and a 90–250 °C example yielding approximately 143.2 A sqrt(s)/mm². PDF text extraction loses some radical glyphs; the `1/sqrt(t)` dependence and the logarithm under the radical follow both dimensional consistency and its explicit 1 s, 0.5 s and 2 s examples. This reconstruction is disclosed rather than treating garbled extraction as a verbatim formula.
- **P6 — Schneider Electric, Electrical Installation Guide, cable short-circuit withstand**, manufacturer guide, “Thermal constraints”: <https://www.electrical-installation.org/enwiki/Verification_of_the_withstand_capabilities_of_cables_under_short-circuit_conditions>. Explicit `I²t = k²S²`, at most five seconds, copper XLPE 90–250 °C `k = 143`, and the need for separate electrodynamic checks.
- **P7 — onsemi, SR005AN/D, “Linear Regulator Protection Circuitry,” April 2001**, manufacturer application note, “Short Circuit Protection,” Figures 2–3 and following page: <https://www.onsemi.com/download/application-notes/pdf/sr005an-d.pdf>. Describes output-dependent foldback, reduced short-circuit dissipation and startup/load-dependent latch-up. The affine branch fit and its optimization below are explicitly a derived engineering model, not an assertion that every regulator has a straight foldback curve.

Search boundary: manufacturers and their original documents, including manufacturer-authored material on distributor mirrors. Littelfuse asset and legacy URLs returned HTTP 403; Mouser mirrors returned access-denied pages; Eaton's high-speed guide timed out; direct ST and ST China PDFs timed out; onsemi AN004E redirected to navigation; Vishay `29079/varintro.pdf` failed PDF parsing; Hinode and an old TI 723 scan had incomplete extraction. These were **not** used as read evidence. Accessible P1–P7 replaced them. No paid standard was read; references to IEC in manufacturer guides are not claims to have inspected or implemented the complete standard.

All six are engineering screening models. Manufacturer data, waveform matching, temperature and tolerance corners, destructive-fault containment, applicable standards and hardware validation remain essential. No output is a certification or permission to exceed a component rating.


## 1. `protection.fuse_pulse_coordination` — basic

### Purpose and novelty

Select a fuse that survives a known inrush yet clears before a specified semiconductor thermal integral is exceeded. Closest old model: `pcb.trace_joule_heat_density`, which multiplies fixed resistivity by current density and time to calculate deposited heat. This model instead integrates an explicitly nonrectangular current waveform and distinguishes fuse pre-arcing from arcing/total clearing. It is not a second energy unit or inverse. Pulse survival and semiconductor coordination are deliberately one item, not two counts.

### Equations, units and domain

Let `i(t)` rise linearly from zero to `peak_current` during `rise_time`, remain constant during `hold_time`, then fall linearly to zero during `fall_time`. Each linear edge contributes `Ipk² t/3`, because `integral_0^1 x² dx = 1/3`.

- `pulse_i2t = peak_current² [hold_time + (rise_time+fall_time)/3]`, A² s.
- `pulse_margin = pulse_allowance_fraction pre_arcing_i2t - pulse_i2t`, A² s.
- `clearing_i2t = pre_arcing_i2t + arcing_i2t`, A² s.
- `clearing_margin = device_withstand_i2t - clearing_i2t`, A² s.
- `pulse_utilization = pulse_i2t/(pulse_allowance_fraction pre_arcing_i2t)`.
- `clearing_utilization = clearing_i2t/device_withstand_i2t`.

Positive margins mean only that the corresponding supplied integral comparison passes. A² s is not joules: conversion to actual heat requires resistance, which varies during fuse melting/arcing. Both current directions have the same square-current integral; this input specifies a nonnegative magnitude. Zero peak current is valid. Edge durations may individually vanish; total duration must be positive and below 10 ms. The square-root-scaled pulse expression avoids squaring current before multiplying by a short duration.

P1 supplies the physical distinction and short-pulse regime. An inrush waveform does not determine the fuse's fault arcing integral. Obtain the latter for the actual voltage, prospective fault current, AC/DC conditions and temperature. A manufacturer pulse-life fraction is an input, not a fixed empirical rule. Semiconductor I²t applicability must be explicit; most MOSFET/IGBT short-circuit designs need SOA and turn-off analysis instead.

### Complete independent example

Inputs: `peak_current=20 A`, `rise_time=0.001 s`, `hold_time=0.002 s`, `fall_time=0.001 s`, `pre_arcing_i2t=2 A² s`, `arcing_i2t=0.7 A² s`, `pulse_allowance_fraction=0.8`, `device_withstand_i2t=4 A² s`.

The 4 ms pulse has effective squared-current duration `0.002+0.002/3 = 0.002666666666666667 s`. Thus:

| Output key | Independent result |
|---|---:|
| `pulse_i2t` | `400 × 0.002666666666666667 = 1.0666666666666667 A² s` |
| `pulse_margin` | `1.6 − 1.0666666666666667 = 0.5333333333333333 A² s` |
| `clearing_i2t` | `2 + 0.7 = 2.7 A² s` |
| `clearing_margin` | `4 − 2.7 = 1.3 A² s` |
| `pulse_utilization` | `1.0666666666666667/1.6 = 0.6666666666666667` |
| `clearing_utilization` | `2.7/4 = 0.675` |

### Tester cases

- Nominal: all six values above, not merely finite outputs.
- Rectangle: set both edge times to zero and `hold_time=0.002`; `pulse_i2t=0.8`, `pulse_margin=0.8`, utilization `0.5`; clearing outputs stay unchanged.
- Triangle: set hold time to zero, retain 1 ms edges; `pulse_i2t=0.2666666666666667`.
- Zero current: pulse integral/utilization zero; pulse margin `1.6`; fault clearing outputs unchanged.
- Failed design, still valid model: `device_withstand_i2t=2` yields clearing margin `−0.7`, utilization `1.35`.
- Reject zero total duration, total duration exactly 10 ms, any negative duration/current, zero pre-arcing/withstand, or allowance outside `(0,1]`.

### Mandatory diagram

Canvas 700×280. Left panel x=24–360: horizontal time axis y=208 and positive current axis x=50; trapezoid starts at zero, rises, has a plateau, and falls to zero. Bind `peak_current` to the plateau height label; put nonoverlapping duration brackets below the three segments using `rise_time`, `hold_time`, `fall_time`. Shade/outline the conceptual integral region with an annotation `integral i² dt` bound to `pulse_i2t`, explicitly not geometric area under `i(t)` itself. Right panel x=390–674: a series chain of labeled source, fuse and semiconductor blocks, current arrow pointing source→fuse→device. Label fuse pre-arcing `pre_arcing_i2t`, total clearing `clearing_i2t`, and device `device_withstand_i2t`. Do not bind an area under `i(t)` directly to I²t. Caption: “Illustrative trapezoidal inrush; square-current integration checks pulse survival. Fault clearing uses separate manufacturer pre-arcing plus arcing data.”

## 2. `protection.tvs_surge_load_line` — intermediate

### Purpose and novelty

Find actual clamp stress when source impedance and finite TVS avalanche slope interact. Closest baseline models are `diodes.biased_two_sided_clipper` and `diodes.zener_shunt_resistor_window`. Those use fixed clipping/shunt levels. Here the nonlinear-device avalanche branch is locally affine, so surge current changes clamp voltage and must be solved simultaneously with source impedance. This is a transient-rating/load-line design problem, not another zener resistor inverse. Neighbor MOV uses a multiplicative power law and imposed pulse current, not this affine source intersection.

### Derivation and limits

P2 gives `Vclamp=Vb+rd I`. KVL gives `Vs=Rs I+Vclamp`. Subtraction yields:

`I=(Vs−Vb)/(Rs+rd)`; `Vclamp=Vb+rd I`; `Ppeak=Vclamp I`.

Voltage margin is `protected_voltage_limit−Vclamp`; current margin is `rated_pulse_current−I`. SI units: V, A, ohm, W. The source top node and protected node are positive relative to return; TVS avalanche current flows from its cathode at the protected node to its anode at return. This is the positive-surge branch only.

`Rs>0`, `rd>=0`, `Vb>0`, and the solved current must lie in the supplied positive characterized interval. The independent pulse-current rating may be lower than the characterized upper fit point, so a negative rating margin remains reportable. The fit and ratings must match waveform and temperature. Loading, lead inductance, time-dependent heating, negative-polarity operation and normal standoff selection are not solved.

### Complete independent example

Inputs: `surge_voltage=100 V`, `source_resistance=2 ohm`, `breakdown_intercept=30 V`, `dynamic_resistance=0.5 ohm`, `fit_current_min=1 A`, `fit_current_max=100 A`, `rated_pulse_current=35 A`, `protected_voltage_limit=60 V`.

`I=(100−30)/2.5=28 A` is within `[1,100] A`.

| Output key | Independent result |
|---|---:|
| `surge_current` | `28 A` |
| `clamp_voltage` | `30+0.5×28=44 V` |
| `peak_power` | `44×28=1232 W` |
| `voltage_margin` | `60−44=16 V` |
| `current_margin` | `35−28=7 A` |

### Tester cases

- Nominal above; verify source KVL `100 = 2×28+44` independently.
- Ideal-slope limit `dynamic_resistance=0`: `I=35 A`, `V=30 V`, `P=1050 W`, voltage margin `30 V`, current margin zero.
- Fit endpoints: source `32.5 V` gives `1 A` and `30.5 V`; source `280 V` gives `100 A` and `80 V`. Both valid; the latter returns negative protection margins.
- Lower `protected_voltage_limit=40` gives `voltage_margin=−4`, not a domain error.
- Reject zero source resistance, negative slope, reversed/equal fit limits, or source levels producing current outside the fit interval. `surge_voltage=30` is off-avalanche and must reject rather than produce a fabricated clamp result.

### Mandatory diagram

Left x=24–390: source `surge_voltage`, series `source_resistance`, protected-node dot, and shunt block “TVS avalanche” to common return. Mark cathode K at positive node, anode A at return; do not draw the ordinary forward diode in the wrong direction. Draw downward arrow `surge_current`; bracket protected-node-to-return `clamp_voltage`. Right x=430–674: conceptual V-horizontal/I-vertical graph with descending source line and ascending avalanche line; label its voltage intercept `breakdown_intercept` and slope relation `dV/dI = rd` bound to `dynamic_resistance`. Intersection is explanatory, not a live computed plot. Caption identifies negligible load and inductance, positive avalanche operation and waveform-specific affine fit.

## 3. `protection.mov_power_law_pulse` — intermediate

### Purpose and novelty

Estimate a metal-oxide varistor's nonlinear protection-region clamp, incremental resistance and absorption using two measured/datasheet points. Unlike the TVS affine intercept-plus-slope model, grain-boundary conduction uses `I=K V^alpha` and has a current-dependent incremental resistance. Unlike `diodes.shockley_forward_current`, this is a bulk MOV empirical power law, not a forward PN-junction exponential or thermal-voltage calculation. Pulse absorption is a supporting output, not a separately counted energy calculator.

### Derivation and limits

P3 gives `V=C I^beta`, `beta=1/alpha`. Divide the two calibration equations and take logarithms:

`beta=ln(V2/V1)/ln(I2/I1)`; `alpha=1/beta`.

Interpolation gives `V=V1 exp[beta ln(I/I1)]`. Differentiation gives `rd=beta V/I`, which is not the static ratio `V/I`. An imposed rectangular pulse gives `E=V I duration`; margin is allowance minus E.

Require `I2>I1>0`, `V2>V1>0`, `0<beta<1`, and `I1<=Ipulse<=I2`. These bounds keep the fit in the declared protection region. The stable `log1p((x2−x1)/x1)` form preserves close-point differences. No low-current extrapolation to zero is allowed. Current and voltage inputs are positive magnitudes; the model may describe either MOV polarity only when the chosen measured branch supports it.

Pulse energy applies only to the flat current interval with ideal zero-time edges. Actual repetitive life, heating-induced fit changes, pulse shape, thermal disconnection and failure modes are not inferred from an energy margin.

### Complete independent example

Inputs: `current_one=1 A`, `voltage_one=320 V`, `current_two=100 A`, `voltage_two=400 V`, `pulse_current=10 A`, `pulse_duration=0.0001 s`, `energy_allowance=1 J`.

`beta=ln(1.25)/ln(100)=0.0484550065040282`. The pulse current is the geometric mean of the two endpoints, so the voltage is independently `sqrt(320×400)=357.77087639996637 V`.

| Output key | Independent result |
|---|---:|
| `voltage_exponent` | `0.0484550065040282` |
| `current_exponent` | `20.637702317032343` |
| `clamp_voltage` | `357.77087639996637 V` |
| `differential_resistance` | `beta×357.77087639996637/10 = 1.7335790142912242 ohm` |
| `absorbed_energy` | `357.77087639996637×10×0.0001 = 0.35777087639996635 J` |
| `energy_margin` | `0.6422291236000337 J` |

### Tester cases

- Nominal above, including the geometric-mean identity.
- At `pulse_current=1`, voltage equals exactly the first fitted value `320 V`; at `100 A`, equals `400 V`.
- Double pulse duration: exponent, clamp and differential resistance unchanged; energy doubles.
- Set allowance below absorbed energy: return a negative margin, not a fabricated safe result.
- Reject equal/reversed currents, equal/decreasing voltages, `beta>=1`, pulse current outside either endpoint, zero pulse duration or nonpositive allowance.
- Close-point oracle: choose `I2=2`, `V2=320×2^0.05`; fitted beta is `0.05`. The tester should calculate this independently at high precision rather than snapshot implementation output.

### Mandatory diagram

Left x=24–340: current-pulse source feeding a two-terminal block “MOV”, positive top node and return below. Bind imposed pulse arrow to `pulse_current`, pulse-width bracket to `pulse_duration`, and MOV voltage bracket to `clamp_voltage`. Right x=390–674: logarithmic-current/logarithmic-voltage conceptual graph, straight fit segment between `(current_one,voltage_one)` and `(current_two,voltage_two)`. Bind endpoint labels to all four exact keys; identify slope as `voltage_exponent` on log axes. Place an energy annotation `integral v i dt` bound to `absorbed_energy` outside the geometry. Caption: “Power-law interpolation only between characterized MOV points; energy assumes an imposed rectangular current pulse. Static log-log sketch, not a live trace.”

## 4. `protection.scr_crowbar_trigger_latch` — intermediate

### Purpose and novelty

Design a zener-triggered SCR crowbar with the correct gate-shunt/source loading and distinguish triggering, latching and holding. Closest baseline `bjt.saturated_switch_drive` computes sustained BJT base drive but has no regenerative latch or distinct holding boundary. Closest old zener sizing does not model a state transition that collapses its own trigger voltage. This model is not an SCR current-gain substitution for a BJT formula.

### Topology and derivation

The positive Thevenin fault source feeds node A through `source_resistance`. SCR A is this node, SCR K is return. A second branch runs A→`gate_resistance`→zener cathode, zener anode→G. `gate_shunt_resistance` runs G→K. The protected load is high impedance and omitted.

Before turn-on, gate voltage is Vg, zener current is `Iz=Ig+Vg/Rgk`, and source resistance carries this same Iz. KVL therefore gives:

- `trigger_source_voltage=Vz+Vg+(Rs+Rg)(Igt+Vg/Rgk)`.
- `available_gate_current=(Vfault−Vz−Vg)/(Rs+Rg)−Vg/Rgk`.

After turn-on, require `Von<Vz`; the trigger branch is off. Available on-state current is `(Vfault−Von)/Rs`. Subtract `latching_current` for `latch_margin`. A falling source reaches the quasi-static holding boundary at `Von+Rs holding_current`.

P4 distinguishes these device states; the resistor network is derived here. A sufficient gate pulse must persist until anode current exceeds IL. Constant drops, negligible loop inductance and adequate turn-on duration are assumptions, not a prediction of the event's switching dynamics. The source must be capable of the required gate current; this is enforced. A negative latch margin is allowed to expose inadequate anode current. The current/voltage parameters must be appropriate corners for a real part; the illustrative values are not a part recommendation.

### Complete independent example

Inputs: `zener_voltage=12 V`, `gate_voltage=0.8 V`, `gate_trigger_current=0.005 A`, `gate_resistance=100 ohm`, `gate_shunt_resistance=1000 ohm`, `fault_voltage=18 V`, `source_resistance=10 ohm`, `on_voltage=1.2 V`, `latching_current=0.05 A`, `holding_current=0.02 A`.

Required zener current is `0.005+0.8/1000=0.0058 A`.

| Output key | Independent result |
|---|---:|
| `trigger_source_voltage` | `12+0.8+110×0.0058 = 13.438 V` |
| `available_gate_current` | `5.2/110−0.0008 = 0.04647272727272727 A` |
| `available_anode_current` | `(18−1.2)/10 = 1.68 A` |
| `latch_margin` | `1.68−0.05 = 1.63 A` |
| `holding_release_voltage` | `1.2+10×0.02 = 1.4 V` |

### Tester cases

- Nominal above; source resistance must contribute to trigger loading, not only anode current.
- Set fault source exactly `13.438 V`: gate current is `0.005 A`, available anode current `1.2238 A`, latch margin `1.1738 A`, release boundary unchanged.
- Set `latching_current=2 A`, keeping holding at `0.02 A`: trigger remains valid, latch margin `−0.32 A`; no assertion that it latches.
- Holding equal to latching is accepted as the model's limiting case. Setting holding above latching rejects.
- Reject source below the computed trigger requirement, zero source/gate/shunt resistance, nonpositive gate requirement, or `on_voltage>=zener_voltage`.
- Larger Rgk reduces wasted shunt current; the finite value must remain part of the gate-current equation.

### Mandatory diagram

Use x=30–660 for an electrically complete circuit, not a decorative switch icon. Source at left, Rs in top rail, explicit node A after Rs. At x≈530 put a three-terminal outlined block “SCR” with A top, K bottom, G left; use named pins and ordinary wires, because the shared primitives have no native SCR glyph. K joins return. The gate branch runs from node A through Rg and a block “Vz, reverse avalanche” into G; clearly mark zener cathode toward Rg/A, anode toward G. Shunt Rgk connects G to K with explicit junctions. Bind labels to `fault_voltage`, `source_resistance`, `gate_resistance`, `zener_voltage`, `gate_shunt_resistance`, `available_gate_current`, and `available_anode_current`. Put a compact separate state legend using `latching_current` and `holding_current`. Caption explains positive A→K current, gate current vanishing after clamp, and the required gate pulse until IL. Never depict a two-terminal diode as a complete SCR or connect the gate directly to the cathode wire.

## 5. `protection.copper_adiabatic_fault_limit` — expert

### Purpose and novelty

Derive a conductor's fault withstand from temperature-dependent copper heating and a permitted thermal endpoint. Closest baseline `pcb.trace_joule_heat_density` uses constant resistivity to report heat, not a material-temperature integral or insulation endpoint. Closest fuse item consumes supplied device I²t values, whereas this model derives the conductor limit from material behavior and initial/final temperatures. Cables confirmed no overlap.

### Physical model and domain

For uniform current density J, adiabatic volumetric heating satisfies `Cv dT/dt = rho(T) J²`. Copper resistivity varies approximately linearly with the absolute-offset temperature `234+T` when T is in °C. Integrating this relation gives the logarithmic thermal budget in P5:

`k=226 sqrt(ln[(234+Tf)/(234+Ti)])` in A sqrt(s)/mm².

Then `Iallow=k S/sqrt(t)`, `allowable_i2t=(k S)²`, `Smin=Ifault sqrt(t)/k`, and area margin `S−Smin`. `log1p((Tf−Ti)/(234+Ti))` preserves small positive temperature headroom. P6 independently corroborates the resulting k≈143 for copper XLPE 90–250 °C and the five-second adiabatic screening horizon.

The implemented scope is explicitly **warm copper cable**, `0<=Ti<Tf<=250 °C`, not cryogenic conductors, other metals or material melting. The 0 °C application boundary deliberately avoids extrapolating this cable approximation to low-temperature copper; it was not chosen to mask numerical behavior. `0<t<=5 s`, positive installed area, and nonnegative RMS fault current. Use a lower final limit when insulation/terminations require it. No PVC assumption is hidden behind the 250 °C example. Source constants are rounded engineering constants; do not replace the resulting 143.174414… with rounded table k=143 in numerical tests.

### Complete independent example

Inputs: `area_mm2=4 mm²`, `initial_temperature=90 °C`, `limit_temperature=250 °C`, `clearing_time=0.5 s`, `fault_current=500 A`.

`k=226 sqrt[ln(484/324)] = 143.1744142046674 A sqrt(s)/mm²`.

| Output key | Independent result |
|---|---:|
| `material_k` | `143.1744142046674 A sqrt(s)/mm²` |
| `allowable_fault_current` | `4k/sqrt(0.5) = 809.9167934122551 A` |
| `allowable_i2t` | `(4k)² = 327982.6061255947 A² s` |
| `minimum_area_mm2` | `500 sqrt(0.5)/k = 2.4693894684833158 mm²` |
| `area_margin_mm2` | `1.5306105315166842 mm²` |

### Tester cases

- Nominal complete vector above; verify `Iallow² t=(kS)²` as an independent physical invariant.
- Change clearing time to `2 s`: allowable current halves, required area doubles, k and allowable integral stay unchanged.
- Zero fault current: minimum area zero, area margin equals installed `4 mm²`; withstand outputs remain nonzero.
- Initial and final temperatures approaching each other from below: k and withstand tend to zero, required area grows. A tiny positive temperature difference must not cancel in the logarithm.
- The 5 s endpoint and 0 °C initial endpoint are admitted. Reject longer intervals, zero/negative time, zero area, equal/inverted temperatures, subzero initial temperature or final temperature above 250 °C.
- A high specified fault current may give a negative area margin; keep that result rather than reject a physically meaningful failed design.

### Mandatory diagram

Use a conductor cross-section inside a short cable segment, x=50–320, with positive current arrow along the conductor bound to `fault_current`. Cross-sectional area bracket binds `area_mm2`. At x=380–660 show a thermal progression box from `initial_temperature` to `limit_temperature`, separated by a time arrow bound to `clearing_time`; crossed outward heat arrows are labeled “adiabatic: no heat escape,” not unbound computed zero watt labels. Under the segment identify `minimum_area_mm2` as a required-area result distinct from actual geometry. A small equation annotation `k(Ti,Tf)` binds `material_k`. Caption: “Uniform warm-copper adiabatic fault heating, at most 5 s; actual insulation, joints and electrodynamic limits remain separate.”
