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
