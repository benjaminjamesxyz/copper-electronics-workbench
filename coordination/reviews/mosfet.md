# Independent review: MOSFET and gate drive

- Lane: `mosfet`; integrated development baseline: `7f12f45`.
- Decision: **clear**; no actionable patch-introduced correctness/security findings identified.
- Reviewed all eight models, independently of the source author and tester worktree. `tested=false`: no builds, tests, lint, formatters, smoke commands or numerical-engine runs were executed.
- Scope: `backend/catalog/mosfet.json` (all 842 lines), `research/mosfet.json`, `research/mosfet.md`, `docs/features/mosfet.md` (all 474 lines), lane acceptance rows in `coordination/BACKLOG.md`, `coordination/CONTRACT.md`, and `coordination/SPRINT.md`. Patch provenance inspected with `git diff 061209d^ 7f12f45 -- backend/catalog/mosfet.json docs/features/mosfet.md`; both production lane files were newly introduced by the reviewed development chain.

## Findings

None meeting the required provable-impact, actionable, unintentional, patch-introduced criteria. This is a static mathematical/model review, not a runtime certification or a guarantee of physical accuracy.

## Independently inspected primary evidence

These documents were actually fetched/read during this review, not merely inferred from citation metadata:

1. [TI SLUA618A](https://www.ti.com/lit/slua618), sections 2.7–2.8, equations 9, 11–15 and Figure 7: positive-supply demand `Qg*f`, gate-drive power, plateau source current, separate clamped-inductive current/voltage overlap ramps and gate-loop damping. Source extraction loses some mathematical formatting; the series-RLC characteristic equation independently resolves the damping formula. The source prose says higher resistance “underdamp”; the implementation correctly documents/follows overdamping rather than that prose error.
2. [Infineon OptiMOS device selection for synchronous rectification](https://www.infineon.com/assets/row/public/documents/24/42/infineon-mosfet-optimos-device-selection-for-synchronous-rectification-applicationnotes-en.pdf), section 3.2, equations 5 and 7, and equation 19 with surrounding loss-allocation discussion: channel loss uses MOSFET waveform RMS rather than converter mean; gate drive loss is `Qg*Vg*f`; nonlinear stored energy is the integral of `v*Coss(v)` or `v*dq`. Its SR-specific `V*Qoss-Eoss` cycle loss is correctly not generalized to the selected device's own hard discharge.
3. [Infineon 600 V CoolMOS C7 Design Guide](https://www.infineon.com/assets/row/public/documents/24/42/infineon-application-note-600v-coolmos-c7-applicationnotes-en.pdf), sections 3.1 and 3.4, Figures 9–12: nonlinear-capacitance snubbing limits overlap estimates; stored output-capacitance energy dissipates at hard turn-on and may recycle in ZVS; energy-related effective capacitance is distinct from a point Coss value.
4. [TI SLUA887A](https://www.ti.com/lit/an/slua887a/slua887a.pdf), section 3.1, equation 3 and recharge/component caveats: charge is `Qg+IHBS*D/f+IHB/f`, with the quiescent term deliberately full-cycle; capacitor sizing is charge divided by allowed voltage droop.
5. [MIT 6.012 Spring 1998 Lecture 8](https://web.mit.edu/course/6/6.012/SPR98/www/lectures/S98_Lecture8.pdf), section II.D, pages 7–9: full triode equation, beta convention and drain-voltage region boundary.
6. [TI SBAA730](https://www.ti.com/document-viewer/lit/html/SBAA730/GUID-5A1B27F5-016C-44FA-9999-FB0187F97B06), “How Can I Determine Which IDRIVE Setting to Use?”, equation 1: `IDRIVE=Qgd/TVDS_SLEW`; the implementation correctly uses resistive source/sink paths rather than claiming to model this current-programmed device.

The original research's HTTP-403 Infineon community-FAQ disclosure was retained; this reviewer did not inspect that FAQ and does not count it as source validation. Manufacturer reports establish model provenance, not an independent measured prediction tolerance.

## All-eight-model assessment and independent verification handoff

The following are algebraic checks and proposed parent execution scenarios, not observed runtime results. Complete catalog example output maps agree with independent hand substitutions and the documented units.

### `mosfet.gate_charge_supply` — catalog lines 3–83

Charge balance over one complete unipolar gate cycle gives positive-supply current `Qg/T`, not twice that amount; supply work is `Vdrive*Qg`. Example: 40 nC / 4 microseconds = 0.01 A, 12 V * 40 nC = 480 nJ, and 480 nJ / 4 microseconds = 0.12 W. All three inputs have exclusive positive minima. Current is average supply current, not peak source/sink gate current; power is distributed in the driver/resistors, not claimed to be channel heat. Parent scenario: `q_g=1e-8, f_sw=1e5, v_drive=10` should give `i_supply=0.001 A`, `e_drive=1e-7 J`, `p_drive=0.01 W`; doubling frequency doubles current/power without changing cycle energy. Reject zero or negative charge/frequency/drive.

### `mosfet.gate_loop_damping_resistor` — catalog lines 86–181

From `L*s^2+R*s+1/C=0`, repeated roots require `Rcrit=2*sqrt(L/C)`. The nonnegative external-resistor maximum is a deliberate passive selection rule, not invalid-input clamping. Example: 32 nH / 2 nF = 16 ohm-squared, hence 8 ohm critical; existing 2 ohm needs 6 ohm additional and zeta=1. Positive L/C protect root and denominator; source/sink edge-specific driver resistance and local initial Ciss are stated. Parent scenarios at those L/C: `r_driver=9.5, r_internal=0.5` gives external=0, zeta=1.25; `r_driver=7.5, r_internal=0.5` gives external=0, zeta=1. Reject nonpositive L/C and negative resistances. No exact critically damped response is promised when existing resistance is already excessive.

### `mosfet.bootstrap_charge_budget` — catalog lines 184–321

Direct integration over the specified TI convention gives `Qg+IHBS*D*T+IHB*T`. Example T=20 microseconds, D*T=16 microseconds: leakage 0.4 nC plus quiescent 10 nC plus gate 100 nC = 110.4 nC; 0.5 V droop requires 220.8 nF. Supply headroom is 15-1-10=4 V. Input-only guards enforce positive headroom and droop no greater than headroom; strict duty endpoints preserve the assumed once-per-cycle recharge topology. Parent scenario: `q_g=1e-7, i_hbs=0, i_hb=0, d_max=0.5, f_sw=1e5, v_dd=12, v_diode=1, v_min=8, delta_v=3` gives `q_total=1e-7 C`, `v_headroom=3 V`, `c_min=1e-7/3 F`; delta_v=3 is allowed, greater than 3 rejected. Reject duty 0/1 and exhausted headroom. Effective biased capacitance, full recharge, all other charge paths and designer-chosen minimum enhancement/UVLO voltage remain explicit caveats; no nominal-part or recharge-time guarantee is made.

### `mosfet.output_capacitance_hard_turn_on_loss` — catalog lines 324–405

Energy-equivalent capacitance is defined by `2*integral(v*dq)/V^2`, so the energy identity is valid at the characterization endpoint without replacing nonlinear Coss by a small-signal point. Example 250 pF at 200 V gives 5 microjoules and at 500 kHz gives 2.5 W. Inputs are strictly positive and no unsupported voltage-characterization match is asserted numerically. Parent scenario: `c_o_er=1e-10, v_ds=100, f_sw=1e5` gives 0.5 microjoules and 0.05 W. Doubling V gives four times these numbers only when the entered effective capacitance remains the appropriate supplied value; it is not a claim that real nonlinear C_o_er stays fixed with voltage. Reject nonpositive inputs. One selected-device hard discharge, no recovery, and no measured-Eon/Eoff double counting are correctly stated.

### `mosfet.channel_conduction_loss` — catalog lines 408–499

Uniform traversal between endpoint currents a and b has mean square `(a^2+a*b+b^2)/3`; multiplying by channel duty gives full-cycle RMS-squared. Example endpoints 5 and 11 A yield 67 A-squared on-state, 40.2 A-squared full-cycle, RMS 6.340346993658943 A and 0.603 W at 15 milliohm. Ripple is peak-to-peak and input mean is on-interval mean. Guard `mean>=ripple/2` correctly permits the zero valley without admitting an unaccounted negative/DCM interval. Parent scenario `i_mean=2, delta_i_pp=4, duty=0.75, r_ds_on=0.1` has endpoints 0/4 A; full-cycle RMS=2 A and loss=0.4 W. The same valid current vector with duty=0 yields both outputs zero; zero ripple with mean=2 and duty=0.25 yields RMS=1 A. Reject negative inputs, duty outside [0,1], ripple greater than twice mean, and nonpositive resistance. Rds_on temperature/gate conditioning is explicit; no thermal feedback/SOA inference.

### `mosfet.hard_switch_overlap_loss` — catalog lines 502–606

Independent integration of each linear power triangle yields one-half V*I*duration; durations deliberately include both separate current and voltage ramps, not propagation delay or only the Miller interval. Example triangle height 500 W gives 10/15 microjoules on/off, 25 microjoules per cycle and 5 W. Period-occupancy guard is a necessary screen, not a guarantee for arbitrary PWM duty. Parent scenario `v_off=100, i_load=2, t_on=1e-6, t_off=2e-6, f_sw=1e5` gives 100/200/300 microjoules and 30 W; replacing load by zero gives all four outputs zero. At `f_sw=1e6, t_on=t_off=5e-7`, equality to one period must reject. Recovery, Coss, avalanche and nonlinear snubbing exclusions and warning against adding to measured total switching energy are honest.

### `mosfet.miller_plateau_timing` — catalog lines 609–746

Ohmic headroom over separate source/sink resistance paths gives signed charging current and positive discharge-current magnitude, then charge divided by current gives only the drain-voltage plateau intervals. Example source total 6 ohm and headroom 7 V gives 7/6 A and 180/7 ns; sink total 5 ohm and plateau 5 V gives 1 A and 30 ns. All resistances may individually be zero, but each total must be positive; drive must exceed positive plateau. Parent scenario `q_gd=1e-8, v_drive=10, v_plateau=5, r_source=1, r_sink=2, r_external=0, r_internal=0` gives currents 5/2.5 A and intervals 2/4 ns. Reject equal/below-plateau drive and independently all-zero source/sink total. Same plateau/charge in both directions, absence of common-source inductance and current limiting, and non-equivalence to complete overlap time are explicitly qualified.

### `mosfet.long_channel_triode_current` — catalog lines 749–841

The MIT channel-charge equation equals `beta*Vds*(Voverdrive-Vds/2)`, with beta lacking an embedded half. Example overdrive 1.8 V, bracket 0.64 V-squared, beta 0.004 A/V-squared gives 0.00256 A. Domain `0<=Vds<overdrive` makes the bracket nonnegative with no near-boundary subtraction of equal terms; `Vds=0` is exactly admissible. Parent scenario `beta=0.002, v_gs=3, v_th=1, v_ds=1` gives overdrive 2 V and Id=0.003 A; same vector at Vds=0 gives zero. Reject Vgs<=Vth, negative Vds and Vds>=2 V. Strong inversion and body tied to source are honest qualitative assumptions rather than fabricated numeric guarantees; the IC long-channel model is correctly distinguished from discrete power-device linear-mode/SOA prediction.

## Numerical conditioning and cross-boundary routing

No model contains exponential/logarithmic small-argument cancellation. Triode domain guarantees the quadratic subtraction retains at least half the leading term; conduction uses a sum of nonnegative squares; bootstrap total charge is a sum of nonnegative terms. Drive/threshold/headroom subtractions represent actual physical voltage differences and guards require the required signs. The damping subtraction selects an absolute nonnegative additional resistance; resulting total damping is independently reported. Extreme binary64 overflow/underflow remains subject to the documented shared arithmetic policy, not a fabricated physical upper-bound restriction. This review does not claim arbitrary finite-extreme numerical coverage or underflow detection.

The consuming side was inspected: `backend/build.rs` embeds every JSON catalog from the directory without a lane whitelist; `Engine::new` in `backend/src/lib.rs` deserializes and compiles all definitions, registers IDs, and compiles each constraint/output; `Engine::calculate` checks named finite inputs/ranges, evaluates input-only constraints before outputs, and inserts each sequential output into both the expression context and returned results. The generic `Request::Calculate` branch in `respond` forwards the ID/input map and returns the complete output map; `backend/src/main.rs` processes every input line through that dispatcher. There is no unmatched MOSFET-specific variant or newly emitted payload being silently dropped. No MOSFET-specific function extensions are required by these eight expressions.

## Untested scope and required next owner

- Main owns all executed verification: run the integrated lane tests, every complete example/default, the above independent boundaries and rejection scenarios, and the shared protocol/UI regressions after all QA handoffs.
- Runtime parsing/operator evaluation, actual error strings, nonfinite/subnormal behavior, native process output, Flutter forms/result rendering and desktop packaging were not exercised by this reviewer.
- Actual hardware gate waveforms, temperature-dependent Rds_on, dynamic capacitance characterization, bootstrap recharge margins, EMI, SOA and standards compliance require device/application measurements and are not certified by the offline arithmetic models.
- No tester files were read or used as substitute for independent oracles. No production, research, documentation or test implementation changes were made.
