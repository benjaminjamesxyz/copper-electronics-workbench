# Diodes, LEDs and rectification

These eight catalog programs execute in the shared Rust engine; the desktop client consumes their metadata and results without implementing separate numerical math. All worked example inputs equal the catalog defaults and retain the independent researched expected values at relative tolerance **1e-6**. Mathematical tolerance does not assert physical model accuracy.

## Scope and evidence

Exactly eight physical design problems are defined in [the executable catalog](../../backend/catalog/diodes.json): series LED current limiting, forward-junction conduction, bridge conduction dissipation, line-rectifier reservoir ripple, zener shunt regulation, biased voltage clipping, current-sink parallel-LED matching, and reverse-recovery charge transport. Each includes finite bounded defaults, input-only domain constraints, sequential closed-form outputs and an independent numerical example covering **every** output. Levels: two basic, three intermediate, three expert. Different inversions are supporting outputs, not separate calculators.

Search boundary: English-language manufacturer application notes/datasheets and university circuit-teaching material, searched on 2026-10-09 without publication-date restriction. Queries addressed LED ballast/current limiting, Shockley ideality, bridge waveform-dependent dissipation, capacitor-input rectifier ripple, zener corner sizing, biased clippers and reverse-recovery charge. Generic blogs were excluded as formula authorities. This is a focused engineering model review, not an exhaustive standards review or device characterization. Semiconductor parameter fits and voltage/current envelopes still require the selected component's datasheet or measurements.

### Source register and evidence weight

- **S1 — Kingbright SA18-11SURKWA**, [datasheet](https://www.kingbrightusa.com/images/catalog/SPEC/SA18-11SURKWA.pdf), 2022. Manufacturer primary design guidance; page 5 recommends protective current-limiting resistors and a separate resistor per parallel LED. Pages 2–3 demonstrate forward-voltage and temperature dependence. Directly read. Strong authority for operating precautions, not proof of a generic fixed LED voltage.
- **S2 — Auburn ELEC2210 diode laboratory**, [Diodes](https://www.eng.auburn.edu/~niuguof/2210labdev/html/diode.html). University teaching source; §4.4.1 gives Shockley law and thermal voltage, explicitly excluding series resistance, capacitance and temperature dependence of saturation current. §4.4.3 describes full-wave bridge recharge and two forward drops. Directly read. Good circuit-model evidence; idealizations are not device guarantees.
- **S3 — Georgia Tech Physics Book**, [Semiconductor Devices, Diodes](https://physicsbook.gatech.edu/Semiconductor_Devices). Student-authored university teaching material, 2024–2025 contributors. Directly read; used only for the conventional ideality-factor extension of Shockley law. Lower evidentiary weight than manufacturer or faculty-authored notes; other unrelated statements on this page are not relied upon.
- **S4 — Vishay VS-KBPC1/VS-KBPC6**, [full-bridge datasheet](https://www.vishay.com/docs/93585/vs-kbpc1series.pdf), forward-conduction table and circuit configuration. Manufacturer primary source, directly read. Forward voltage is explicitly **per diode**; capacitive-load current ratings differ from resistive-load ratings.
- **S5 — Vishay**, [How to Calculate Power Losses in Gen 5 Diodes](https://www.vishay.com/docs/98280/howtocalculatepowerlossesingen5diodes.pdf). Manufacturer application note supporting affine forward-loss modeling. Search-supported; PDF text extraction failed, so not claimed as fully read. The bridge formula below is independently derived by time averaging the affine model and counting two conducting junctions.
- **S6 — W. C. Chew and Z. H. Chen, Purdue ECE255**, [September 5, 2017 lecture](https://engineering.purdue.edu/wcchew/ece255f17/ECE%20255%20f17%20latex%20pdf%20files/ece255LectureSep5.pdf). Faculty-authored circuit teaching notes, directly read. §2.3 derives reservoir ripple and eq. 2.5 gives IL/(fC); full-wave recharge occurs twice per source period. Strong authority for the stated small-ripple approximation; figures cite Sedra and Smith rather than constitute independent measurements.
- **S7 — TI SLVAG00**, [Zener Diodes Introduction](https://www.ti.com/lit/pdf/slvag00), January 2026, §2.2 and §6.1.1. Manufacturer primary application note, directly read. Equations 2–6 treat input/load/voltage corners and power; the nominal resistance bounds with resistor tolerance below are our explicit extension. The printed equation 2 labels a 22.8-V input as maximum despite the later 25.2-V maximum corner; our formulas use unambiguous minimum/maximum inputs, not that label.
- **S8 — Northwestern ECE270**, [Experiment 4](https://www.eecs.northwestern.edu/~plonus/270/270lab4.pdf). University primary teaching experiment, directly read; prelab constant-drop model and §3(a) biased clipper. The generalized two-sided formulas below are derived from the specified diode orientation and KVL, not quoted from the single experimental setup.
- **S9 — TI SLVA325, Mudassar Khatib**, [Ballast Resistor Calculation — Current Matching in Parallel LEDs](https://e2echina.ti.com/cfs-file/__key/telligent-evolution-components-attachments/13-107-00-00-00-01-00-92/slva325.pdf), April 2009. Manufacturer original application report hosted on TI's community domain, directly read. Pages 3–5 derive R = ΔV/ΔI; page 7 gives ballast power. The ordinary ti.com/lit aliases returned 404; the cited TI-hosted copy is readable.
- **S10 — STMicroelectronics AN5028**, [Calculation of turn-off power losses generated by an ultrafast diode](https://www.st.com/resource/en/application_note/dm00380483-calculation-of-turnoff-power-losses-generated-by-a-ultrafast-diode-stmicroelectronics.pdf). Manufacturer primary application note. Search-visible excerpts support charge integration, condition dependence and cell-level VR·Qrr contribution; direct reads of three ST URLs failed to connect. Not claimed as fully read. The more complete cell loss includes a forward-current/commutation term; VR·Qrr alone is not diode-only heat.
- **S11 — onsemi AND9068/D**, [Reading onsemi IGBT Datasheets](https://www.onsemi.com/pub/Collateral/AND9068-D.PDF), January 2022 revision. Manufacturer primary source, directly read; **only diode characteristics on pages 8–9 are used**. Reverse-recovery charge is the integral of reverse current; its trr is measured to 10% of peak, and the test table specifies IF, VR, diF/dt and temperature. That time definition is not a zero-to-zero triangular base.

No conflicting physical law was found; apparent discrepancies stem from model scope (constant versus affine forward drop, finite-conduction versus small-ripple treatment, diode heat versus cell recovery contribution). Confidence is high for algebra within each stated model; accuracy against real devices is limited by parameter fit, temperature and neglected dynamics. There is no evidence here for standards compliance, device safety or production thermal qualification.


## Runtime and numerical contract

Inputs must contain exactly the declared finite numeric keys. The engine enforces inclusive ranges unless explicitly marked exclusive, then evaluates the input-only cross-field constraints before sequential outputs. Invalid physical/model domains are errors rather than clamps. Bounds are conservative computational/application envelopes, not device ratings. Qualitative assumptions such as fitted device parameters, sinusoidal excitation and stiff references cannot be established by these scalar checks. Unsupported or nonfinite arithmetic must produce an engine error. Source access descriptions below are inherited research evidence, not claims of new fetches during implementation. No builds, tests, lint, formatter or smoke executions were performed by this lane.

## 1. `diodes.led_series_resistor` — basic

**Purpose/topology:** A voltage source feeds N forward LEDs in series with one resistor. This is not the current-sharing topology in calculator 7.

KVL gives Vs = N·Vf + I·R, hence R = (Vs − N·Vf)/I. Joule heating gives PR = I²R; total LED electrical input is N·Vf·I, not luminous output. At the greatest supply and lowest LED voltage, Imax = (Vs,max − N·Vf,min)/R and PR,max = (Vs,max − N·Vf,min)²/R.

**Units:** voltage V, current A, resistance ohm, power W; N is an integer count. **Domain:** positive current and forward voltages, N ≥ 1 integer, Vs > N·Vf, Vs,max ≥ Vs and Vf,min ≤ Vf. These inequalities guarantee positive headroom and a nonzero denominator. Input ranges bound finite arithmetic.

**Limits:** Fixed Vf must be evaluated at operating current and temperature. This returns an exact nominal calculated resistance, not a preferred-value recommendation. Increasing to a real resistor changes nominal current; independently check the worst corner and derated resistor/device rating. Use **minimum** Vf for maximum-current checks, not maximum Vf. Resistor tolerance and internal-resistor LEDs are excluded. Source: S1; equations derived from KVL.

**Independent example (also defaults):** Vs = 5 V, Vs,max = 5.5 V, Vf = 2 V, Vf,min = 1.8 V, N = 1, I = 0.020 A. Headroom is 3 V; 3/0.020 = **150 ohm**. PR = 0.020²·150 = **0.060 W**; LED electrical power = **0.040 W**. Worst headroom is 3.7 V; Imax = 3.7/150 = **0.024666666666666667 A**, PR,max = 13.69/150 = **0.09126666666666668 W**. All domains pass.

### Executable inputs and admissible ranges

| Key | Meaning | Unit | Default / worked input | Allowed range |
|---|---|---|---:|---|
| `supply` | Nominal supply voltage | V | 5 | (0, 1000] |
| `supply_max` | Maximum supply voltage | V | 5.5 | (0, 1000] |
| `forward_voltage` | Nominal forward voltage per LED | V | 2 | (0, 100] |
| `forward_min` | Minimum forward voltage per LED | V | 1.8 | (0, 100] |
| `led_count` | LEDs in series | count | 1 | [1, 1000] |
| `target_current` | Target string current | A | 0.02 | [1e-09, 100] |

### Sequential outputs and complete independent oracle

Expressions below are evaluated in row order; later rows can use earlier output keys. All expected values are the independent worked results above, not runtime-generated values.

| Key | Meaning | Unit | Executable expression | Expected |
|---|---|---|---|---:|
| `resistance` | Nominal calculated resistor | ohm | `(supply-led_count*forward_voltage)/target_current` | 150 |
| `resistor_power` | Nominal resistor dissipation | W | `target_current^2*resistance` | 0.06 |
| `led_power` | Nominal LED-string electrical power | W | `led_count*forward_voltage*target_current` | 0.04 |
| `current_max` | Maximum current corner | A | `(supply_max-led_count*forward_min)/resistance` | 0.024666666666666667 |
| `resistor_power_max` | Maximum resistor dissipation corner | W | `(supply_max-led_count*forward_min)^2/resistance` | 0.09126666666666668 |

### Constraint/error rationale and implementation limits

Nominal headroom must be strictly positive, making the calculated resistor a valid divisor. Maximum supply and minimum forward voltage ordering then imply positive worst-case headroom. Integer count is checked with floor; no preferred-value rounding occurs.

- `led_count == floor(led_count)` — LED count must be an integer.
- `supply > led_count*forward_voltage` — Supply must exceed the nominal LED-string voltage.
- `supply_max >= supply` — Maximum supply must not be below nominal supply.
- `forward_min <= forward_voltage` — Minimum LED voltage must not exceed nominal voltage.

## 2. `diodes.shockley_forward_current` — expert

**Purpose:** Predict junction current from a fitted forward model and obtain local incremental resistance, not generic resistor current or BJT transconductance.

Thermal voltage VT = kT/q. With k = 1.380649×10⁻²³ J/K and q = 1.602176634×10⁻¹⁹ C, ID = IS[exp(VD/(nVT)) − 1]. Differentiate at fixed T: dID/dVD = (ID + IS)/(nVT); invert to get rd = nVT/(ID + IS). Junction electrical power is VD·ID. The −1 term is retained near zero bias.

**Units:** T in K, VD and VT in V, IS and ID in A, rd in ohm and dissipation W. **Domain:** VD ≥ 0, 1 ≤ n ≤ 2, positive IS, temperature 200–500 K, exponent ≤ 40 and predicted current ≤ a user-supplied fitted-model ceiling. Zero bias is valid: ID = 0 and rd = nVT/IS is finite. Constraints expand VT and ID in terms of inputs because they run before outputs.

**Limits:** The temperature input is junction temperature, not ambient. IS must already be correct at that temperature. A fixed IS cannot model actual thermal drift. No series resistance, high injection, reverse breakdown or self-heating. The exponent guard is a bounded model range, not a semiconductor fundamental limit; the model ceiling must come from the characterized fit. Sources: S2 and S3.

**Independent example (also defaults):** VD = 0.6 V, IS = 10⁻¹² A, n = 1, T = 300 K, model ceiling = 0.1 A. Constants give **VT = 0.025851999786435535 V**. The exponent is about 23.209, below 40. Direct exponential arithmetic yields **ID = 0.01201036955312849 A**, below the ceiling; **junction power = 0.007206221731877094 W**, and **rd = 2.1524732998368954 ohm**. Values were computed independently of the catalog evaluator.

### Executable inputs and admissible ranges

| Key | Meaning | Unit | Default / worked input | Allowed range |
|---|---|---|---:|---|
| `diode_voltage` | Forward junction voltage | V | 0.6 | [0, 5] |
| `saturation_current` | Saturation current at entered temperature | A | 1e-12 | [1e-20, 0.001] |
| `ideality` | Ideality factor | 1 | 1 | [1, 2] |
| `temperature` | Junction temperature | K | 300 | [200, 500] |
| `model_current_max` | Maximum current of fitted model | A | 0.1 | [1e-12, 1000] |

### Sequential outputs and complete independent oracle

Expressions below are evaluated in row order; later rows can use earlier output keys. All expected values are the independent worked results above, not runtime-generated values.

| Key | Meaning | Unit | Executable expression | Expected |
|---|---|---|---|---:|
| `thermal_voltage` | Thermal voltage | V | `1.380649e-23*temperature/1.602176634e-19` | 0.025851999786435535 |
| `diode_current` | Forward diode current | A | `saturation_current*expm1(diode_voltage/(ideality*thermal_voltage))` | 0.01201036955312849 |
| `junction_power` | Junction electrical dissipation | W | `diode_voltage*diode_current` | 0.007206221731877094 |
| `incremental_resistance` | Incremental junction resistance | ohm | `ideality*thermal_voltage/(diode_current+saturation_current)` | 2.1524732998368954 |

### Constraint/error rationale and implementation limits

Implementation uses `expm1(x)` for both forward-current output and the input-only fitted-current guard, with x = VD/(n VT). This is algebraically the full Shockley law, but avoids rounding exp(x) to 1 at very small positive x. At zero bias the current and power are exactly zero; incremental resistance remains n VT/IS. For x much smaller than one, ID ≈ IS·x. The input ranges give positive thermal voltage and a nonzero incremental-resistance denominator, while the exponent/model-current constraints bound the fitted model.

- `diode_voltage/(ideality*(1.380649e-23*temperature/1.602176634e-19)) <= 40` — Forward exponent exceeds the selected model range.
- `saturation_current*expm1(diode_voltage/(ideality*(1.380649e-23*temperature/1.602176634e-19))) <= model_current_max` — Predicted current exceeds the fitted model-current ceiling.

