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

## Runtime contract and verification handoff

Only the shared Rust engine evaluates these programs; there is no lane-local evaluator or Flutter math. Required finite inputs must match the catalog keys exactly. Input ranges and input-only relational constraints run before sequential outputs. Invalid arithmetic and nonfinite intermediate or final results are errors, never substituted zeros. Bounds protect positive divisors and roots; operating-region constraints are necessary model screens, not semiconductor safety certificates. Extreme finite inputs can still overflow or underflow binary64 and are subject to the shared engine arithmetic error policy, not fabricated physical upper limits.

The independent worked examples and defaults above are retained from the researched derivations; no build, test, lint, formatter or smoke command was run for this implementation. Main should exercise every complete example vector and default, plus each relational rejection and the admissible zero/overdamped boundaries named in the feature sections. No passing execution claim is made here.
