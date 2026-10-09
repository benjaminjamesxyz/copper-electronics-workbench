# MOSFET and gate-drive calculators

The executable catalog is [backend/catalog/mosfet.json](../../backend/catalog/mosfet.json). The completed lane contains eight distinct calculators. All catalog expressions are closed-form scalar expressions in the contract's supported language. SI inputs are used throughout: charge in C, time in s, capacitance in F, inductance in H, current in A, voltage in V, resistance in ohm, frequency in Hz, energy in J and power in W. Duty fraction and damping ratio are dimensionless. The long-channel process/geometry factor has units A/V².

## Research scope and evidence

Research on 2026-10-09 used web search restricted to primary manufacturer and university material, followed by reading the actual references below. Search subjects were gate-charge supply current, Miller plateau charge/timing, gate-loop critical damping, MOSFET RMS conduction loss, clamped-inductive overlap, bootstrap charge budgets and nonlinear output-capacitance energy. No date restriction was imposed; English references were used. Thermal temperature calculations, converter duty/inductor/capacitor ripple sizing, diode reverse recovery and general passive transient calculators were excluded. This is a targeted engineering-model review, not an exhaustive literature review or standards-compliance claim.

| Ref | Primary source and location | Evidence weight and limitations |
|---|---|---|
| S1 | Texas Instruments, Laszlo Balogh, *Fundamentals of MOSFET and IGBT Gate Driver Circuits*, SLUA618A (2018), sections 2.4–2.8, equations 9, 11, 13–15. <https://www.ti.com/lit/slua618> | High weight for the stated simplified gate-drive circuit equations. Manufacturer application report, not a universal measured accuracy guarantee. |
| S2 | Infineon, Green et al., *OptiMOS device selection for synchronous rectification*, AN_2212_PL15_2301_094414, v1.1 (2023), section 3.2, equations 5, 7 and 19. <https://www.infineon.com/assets/row/public/documents/24/42/infineon-mosfet-optimos-device-selection-for-synchronous-rectification-applicationnotes-en.pdf> | High weight for RMS channel dissipation and nonlinear stored-energy definitions; SR-specific switching-loss allocation is not generalized to all topologies. |
| S3 | Infineon, Stückler, Abdel-Rahman and Siu, *600 V CoolMOS C7 Design Guide* (2015), sections 3.1–3.4, Figures 9–12. <https://www.infineon.com/assets/row/public/documents/24/42/infineon-application-note-600v-coolmos-c7-applicationnotes-en.pdf> | High weight for energy-equivalent capacitance and hard-discharge versus ZVS-recycling mechanism. Includes manufacturer technology comparisons; no product-superiority claim is used here. |
| S4 | Texas Instruments, Mamadou Diallo, *Bootstrap Circuitry Selection for Half-Bridge Configurations*, SLUA887A (2023 revision), section 3.1, equation 3. <https://www.ti.com/lit/an/slua887a/slua887a.pdf> | High weight for the named TI charge-budget convention. Driver current definitions and practical recharging remain part-specific. |
| S5 | MIT EECS 6.012, Spring 1998, Lecture 8, *MOS Field Effect Transistors*, section II.D, pages 7–9. <https://web.mit.edu/course/6/6.012/SPR98/www/lectures/S98_Lecture8.pdf> | High weight for the textbook long-channel triode model and region boundary. Not a discrete power-device safe-operating-area model. |
| S6 | Texas Instruments, *DRV8334 Gate Drive Features and Benefits*, SBAA730 (2025), “How Can I Determine Which IDRIVE Setting to Use?”, equation 1. <https://www.ti.com/document-viewer/lit/html/SBAA730/GUID-5A1B27F5-016C-44FA-9999-FB0187F97B06> | Direct primary support for the Q_gd / slew-time relationship. The DRV8334 is current-programmed; our resistive-path current estimate instead uses S1. |

These six sources are not six independent experimental confirmations: S1/S4/S6 are from one manufacturer, and S2/S3 from another. The simple energy/charge identities are well established; confidence in prediction for a particular device is limited by supplied operating-point parameters and the assumptions below. No universal error bound was found or is asserted. An Infineon community FAQ on C_oss returned HTTP 403 during the direct read; it is not relied on as a sole reference because S2/S3 provide the necessary accessible primary documentation.

## Ownership and nonduplication

1. Average gate supply demand is a charge-per-cycle model, not a peak gate-current or switching-time inverse.
2. Channel conduction loss integrates a duty-gated current waveform and operating R_ds_on; no thermal solve is included.
3. Switching overlap integrates channel voltage times load current during commutation.
4. Gate resistor selection matches the damping of a gate-loop parasitic RLC network; it does not invert the Miller-time equation or calculate a general passive transient.
5. Miller timing estimates the drain-voltage slew interval from charge and resistive source/sink paths.
6. Triode current is an active-device channel charge model, not Ohm's law using a supplied fixed resistance.
7. Bootstrap capacitance is a floating driver supply charge budget with driver leakage/quiescent consumption and UVLO/gate-enhancement headroom.
8. Output-capacitance hard-turn-on loss accounts for an unloaded device's nonlinear stored-energy discharge; it is physically distinct from channel/load-current overlap.

Diode reverse recovery remains the diode lane's responsibility. In particular, neither overlap nor C_oss discharge includes V Q_rr f. Power, transients and thermal lane boundaries were reserved on COMMUNICATION.md. No calculator below computes junction temperature, semiconductor SOA, converter efficiency, converter ripple or general capacitor storage as a standalone problem.



## 1. Gate-charge average supply current and drive power

**ID:** `mosfet.gate_charge_supply` — **basic**. Sources: S1 equation 9; S2 equation 7.

A unipolar driver draws Q_g coulombs from its positive supply each complete switching cycle. With period T = 1/f_sw,

- I_supply = Q_g / T = Q_g f_sw.
- E_drive = V_drive Q_g.
- P_drive = E_drive / T = Q_g V_drive f_sw.

A complete cycle removes Q_g through the sink path too, but that does not draw another Q_g from the positive supply. Thus supply demand is Q_g f, not 2 Q_g f. For a linear equivalent capacitor, the charging and discharging resistive losses together are C V² = Q V; the gate-charge expression is the usual engineering estimate for a real gate with a Miller segment.

**Inputs and model domain:** Q_g > 0 C, f_sw > 0 Hz, V_drive > 0 V; all finite. Choose Q_g at the actual drive amplitude, drain swing and load current. Negative-bias, resonant/energy-recovery drive and driver quiescent consumption are outside the model. The calculated drive loss is distributed among the driver, external resistor and internal gate mesh; it is not entirely channel heat. C_iss measured at one bias is not a replacement for gate-charge data.

**Independent example:** 40 nC at 250 kHz and 12 V gives 40 × 250 × 10⁻⁶ A = **0.010 A**, E_drive = 40 nC × 12 V = **480 nJ**, and P_drive = 480 nJ × 250 kHz = **0.120 W**. These exact values are the JSON example expectations.

**Defaults:** 60 nC, 100 kHz, 10 V yield **6 mA**, **600 nJ/cycle**, **60 mW**.

### Executable interface and complete example vector

| Input key | Meaning | Unit | Default | Admissible range |
|---|---|---|---|---|
| `q_g` | Total gate charge at operating point | C | 6e-08 | > 0 |
| `f_sw` | Gate switching frequency | Hz | 100000 | > 0 |
| `v_drive` | Positive gate drive voltage | V | 10 | > 0 |

Outputs are evaluated in the listed order; earlier outputs are available to later expressions.

| Output key | Meaning | Unit | Expression | Independent example value |
|---|---|---|---|---|
| `i_supply` | Average gate-charge supply current | A | `q_g*f_sw` | 0.01 |
| `e_drive` | Gate-drive energy per cycle | J | `q_g*v_drive` | 4.8e-07 |
| `p_drive` | Gate-drive power | W | `e_drive*f_sw` | 0.12 |

Exact example inputs (SI):

```json
{
  "q_g": 4e-08,
  "f_sw": 250000,
  "v_drive": 12
}
```

Declared relative comparison tolerance: `1e-6`; this is an arithmetic criterion, not a physical model error guarantee.


## 4. Gate-loop critical-damping resistor selection

**ID:** `mosfet.gate_loop_damping_resistor` — **expert**. Source: S1 Figure 7 and equation 15.

For a lumped series gate-loop RLC network, the characteristic equation is

s² + (R_total/L_loop)s + 1/(L_loop C_iss) = 0.

The repeated-pole critical-damping condition is R_total = 2 sqrt(L_loop/C_iss), with units sqrt(H/F) = ohm. Driver and internal gate resistances already contribute:

- R_critical = 2 sqrt(L_loop/C_iss).
- R_external = max(0, R_critical − R_driver − R_internal).
- zeta = (R_driver + R_internal + R_external) / R_critical.

The nonnegative maximum expresses an actual passive-component design condition: if the existing path is already overdamped, add no resistor. It is not an attempt to hide an invalid square root or denominator.

**Inputs and model domain:** L_loop > 0 H, C_iss > 0 F, R_driver ≥ 0 ohm, R_internal ≥ 0 ohm. Both the square root and R_critical denominator are strictly positive. Driver output resistance must correspond to the selected source or sink edge; a current-limited driver requires a different model. Use a locally appropriate initial input capacitance, not Miller-charge equivalent capacitance. Common-source feedback, capacitance nonlinearity and distributed parasitics limit prediction.

**Independent example:** 32 nH / 2 nF = 16 ohm², so R_critical = **8 ohm**. With driver 1.5 ohm and internal gate 0.5 ohm, add **6 ohm** and zeta = **1**. As a second independently evaluated physical case, keeping L and C but using R_driver = 9.5 ohm and R_internal = 0.5 ohm gives R_external = **0 ohm**, zeta = **1.25**; a negative resistor is not suggested.

**Defaults:** 18 nH, 2 nF, 1 ohm driver and 0.5 ohm internal resistance give **6 ohm critical**, **4.5 ohm external**, **zeta = 1**.

**Source interpretation:** The prose immediately after S1 equation 15 calls higher resistance “underdamp”; the series-RLC characteristic equation shows that higher total resistance instead produces **overdamping**. The calculator follows equation 15 and that derivation. Final resistor choice also involves switching-loss, EMI and measured waveform tradeoffs; no EMI or gate-voltage safety guarantee is made.

### Executable interface and complete example vector

| Input key | Meaning | Unit | Default | Admissible range |
|---|---|---|---|---|
| `l_loop` | Total gate-loop inductance | H | 1.8e-08 | > 0 |
| `c_iss` | Effective initial input capacitance | F | 2e-09 | > 0 |
| `r_driver` | Driver resistance for the selected edge | ohm | 1 | ≥ 0 |
| `r_internal` | MOSFET internal gate resistance | ohm | 0.5 | ≥ 0 |

Outputs are evaluated in the listed order; earlier outputs are available to later expressions.

| Output key | Meaning | Unit | Expression | Independent example value |
|---|---|---|---|---|
| `r_critical` | Critical total series resistance | ohm | `2*sqrt(l_loop/c_iss)` | 8 |
| `r_external` | Minimum added gate resistance | ohm | `max(0,r_critical-r_driver-r_internal)` | 6 |
| `zeta` | Resulting damping ratio | 1 | `(r_driver+r_internal+r_external)/r_critical` | 1 |

Exact example inputs (SI):

```json
{
  "l_loop": 3.2e-08,
  "c_iss": 2e-09,
  "r_driver": 1.5,
  "r_internal": 0.5
}
```

Declared relative comparison tolerance: `1e-6`; this is an arithmetic criterion, not a physical model error guarantee.


## 7. Bootstrap capacitance from gate and driver charge budget

**ID:** `mosfet.bootstrap_charge_budget` — **expert**. Source: S4 section 3.1 equation 3.

A charged floating supply capacitor supplies the MOSFET gate and high-side driver until the next recharge. S4's detailed convention assigns HB-to-VSS leakage to high-side on time, but conservatively budgets I_hb across a full cycle:

- Q_total = Q_g + I_hbs D_max/f_sw + I_hb/f_sw.
- V_headroom = V_dd − V_diode − V_min.
- C_min = Q_total/ΔV.

This follows charge balance ΔQ = C ΔV. The separate high-side quiescent term is deliberately I_hb/f_sw, **not** silently changed to I_hb D_max/f_sw. Driver-family definitions differ and must be read before choosing the input currents.

**Inputs and model domain:** Q_g > 0 C; I_hbs and I_hb ≥ 0 A; f_sw > 0 Hz; 0 < D_max < 1; V_dd > 0 V; V_diode ≥ 0 V; V_min > 0 V; ΔV > 0 V. Constraints require V_headroom > 0 and ΔV ≤ V_headroom, expressed only in input keys. V_min should cover adequate MOSFET gate enhancement as well as UVLO and margin; using UVLO alone does not establish a sufficiently low R_ds_on.

Initial charge is assumed V_dd − V_diode with switch node near ground. The model presumes full recharge once each period. Finite low-side on time, bootstrap resistance, startup, skipped pulses and switch-node undershoot need separate analysis. It is a theoretical **effective** minimum capacitance, not a nominal part recommendation. Account for ceramic DC-bias derating, tolerance, temperature, aging, ESR/ESL and all leakage/level-shifter charges not represented by the selected driver currents. S4's 10× gate-capacitance heuristic is not automatically imposed because this calculator uses the explicit charge and requested-droop budget; no universal safety multiplier is asserted.

**Independent example:** Q_g = 100 nC, I_hbs = 25 µA, I_hb = 500 µA, D_max = 0.8, f_sw = 50 kHz. Period is 20 µs and high-side on time 16 µs. Leakage removes **0.4 nC** and quiescent demand **10 nC**, for Q_total = **110.4 nC**. With V_dd = 15 V, diode = 1 V, V_min = 10 V, available headroom is **4 V**. A requested **0.5 V** droop needs **220.8 nF** effective capacitance, within the headroom condition.

**Defaults:** 50 nC, 10 µA leakage, 200 µA quiescent current, D_max = 0.9, 100 kHz, V_dd = 12 V, diode = 0.7 V, V_min = 8 V, droop = 1 V give **52.09 nC**, **3.3 V headroom**, **52.09 nF** minimum effective capacitance. D_max < 1 supplies a possible refresh interval but does not guarantee adequate recharge through a specific circuit.

### Executable interface and complete example vector

| Input key | Meaning | Unit | Default | Admissible range |
|---|---|---|---|---|
| `q_g` | High-side MOSFET total gate charge | C | 5e-08 | > 0 |
| `i_hbs` | HB-to-VSS leakage current | A | 1e-05 | ≥ 0 |
| `i_hb` | High-side quiescent current | A | 0.0002 | ≥ 0 |
| `d_max` | Maximum high-side duty fraction | 1 | 0.9 | > 0, < 1 |
| `f_sw` | Switching and recharge frequency | Hz | 100000 | > 0 |
| `v_dd` | Gate-driver bias supply voltage | V | 12 | > 0 |
| `v_diode` | Bootstrap diode forward drop | V | 0.7 | ≥ 0 |
| `v_min` | Minimum acceptable floating supply voltage | V | 8 | > 0 |
| `delta_v` | Maximum permitted bootstrap droop | V | 1 | > 0 |

Outputs are evaluated in the listed order; earlier outputs are available to later expressions.

| Output key | Meaning | Unit | Expression | Independent example value |
|---|---|---|---|---|
| `q_total` | Charge consumed per recharge cycle | C | `q_g+i_hbs*d_max/f_sw+i_hb/f_sw` | 1.104e-07 |
| `v_headroom` | Available supply-voltage headroom | V | `v_dd-v_diode-v_min` | 4 |
| `c_min` | Minimum effective bootstrap capacitance | F | `q_total/delta_v` | 2.208e-07 |

Exact example inputs (SI):

```json
{
  "q_g": 1e-07,
  "i_hbs": 2.5e-05,
  "i_hb": 0.0005,
  "d_max": 0.8,
  "f_sw": 50000,
  "v_dd": 15,
  "v_diode": 1,
  "v_min": 10,
  "delta_v": 0.5
}
```

Declared relative comparison tolerance: `1e-6`; this is an arithmetic criterion, not a physical model error guarantee.


## 8. Output-capacitance hard-turn-on loss

**ID:** `mosfet.output_capacitance_hard_turn_on_loss` — **expert**. Sources: S3 section 3.4, Figures 9–12; S2 equation 19.

A real MOSFET's output capacitance is nonlinear. Its stored energy for drain swing 0 to V_ds is

E_oss(V_ds) = ∫[0 to V_ds] v C_oss(v) dv = ∫ v dq.

Energy-equivalent capacitance is defined at that same endpoint by C_o_er = 2 E_oss(V_ds)/V_ds², so the implementable calculation is

- E_oss = C_o_er V_ds²/2.
- P_oss = E_oss f_sw.

S3 explicitly describes energy stored at turn-off and then dissipated in the MOSFET channel at the next hard turn-on. This can remain nonzero even when load-current overlap tends to zero, which is why this calculator is a distinct physical loss mechanism.

**Inputs and model domain:** C_o_er > 0 F, V_ds > 0 V, f_sw > 0 Hz, all finite. C_o_er must be characterized for the actual entered 0-to-V_ds swing. This cannot be enforced numerically from the three inputs; the label and model assumptions require a datasheet curve or characterized value at that voltage. A single small-signal C_oss point and time-equivalent C_o_tr are not substitutes. The energy-equivalent definition makes the energy identity exact at its supplied voltage, but the claimed loss requires one complete non-recovered hard discharge per cycle.

The calculator covers the chosen device's own stored-energy discharge, not total bridge-leg commutation energy, external capacitors, reverse recovery, supply charging work, or the topology-specific synchronous-rectifier V Q_oss − E_oss loss model in S2. In ZVS/resonant operation much of this energy can be recycled. Do not add this term to switching-energy measurements already containing it.

**Independent example:** C_o_er = 250 pF at a 200 V swing and f_sw = 500 kHz. E_oss = (250 × 10⁻¹²)(200²)/2 = **5 µJ**. One hard discharge each 2 µs gives **2.5 W**. These values are illustrative model parameters, not a claim about a particular MOSFET.

**Defaults:** 150 pF at 400 V and 100 kHz give **12 µJ** and **1.2 W**.

### Executable interface and complete example vector

| Input key | Meaning | Unit | Default | Admissible range |
|---|---|---|---|---|
| `c_o_er` | Energy-equivalent output capacitance at entered voltage | F | 1.5e-10 | > 0 |
| `v_ds` | Off-state voltage matching C_o_er characterization | V | 400 | > 0 |
| `f_sw` | Hard-discharge switching frequency | Hz | 100000 | > 0 |

Outputs are evaluated in the listed order; earlier outputs are available to later expressions.

| Output key | Meaning | Unit | Expression | Independent example value |
|---|---|---|---|---|
| `e_oss` | Output-capacitance energy dissipated per cycle | J | `c_o_er*v_ds^2/2` | 5e-06 |
| `p_oss` | Hard turn-on output-capacitance loss | W | `e_oss*f_sw` | 2.5 |

Exact example inputs (SI):

```json
{
  "c_o_er": 2.5e-10,
  "v_ds": 200,
  "f_sw": 500000
}
```

Declared relative comparison tolerance: `1e-6`; this is an arithmetic criterion, not a physical model error guarantee.

## Runtime contract and verification handoff

Only the shared Rust engine evaluates these programs; there is no lane-local evaluator or Flutter math. Required finite inputs must match the catalog keys exactly. Input ranges and input-only relational constraints run before sequential outputs. Invalid arithmetic and nonfinite intermediate or final results are errors, never substituted zeros. Bounds protect positive divisors and roots; operating-region constraints are necessary model screens, not semiconductor safety certificates. Extreme finite inputs can still overflow or underflow binary64 and are subject to the shared engine arithmetic error policy, not fabricated physical upper limits.

The independent worked examples and defaults above are retained from the researched derivations; no build, test, lint, formatter or smoke command was run for this implementation. Main should exercise every complete example vector and default, plus each relational rejection and the admissible zero/overdamped boundaries named in the feature sections. No passing execution claim is made here.
