# Research board — completed 128-feature inventory

All 16 research lanes completed eight calculators each: **128 actual unique IDs**, with **39 basic, 50 intermediate and 39 expert** models. Research is complete; implementation and product acceptance are not implied. The scrum-master reconciled physical problem boundaries and retained every ID: supporting outputs, inverse modes, charge/discharge and empirical layer alternatives are not extra calculators. See [sprint execution and overlap decisions](coordination/SPRINT.md) and [128 explicit acceptance/owner/example backlog rows](coordination/BACKLOG.md).

| Lane | Domain | Reserved boundary | Findings |
|---|---|---|---|
| dc | DC circuits and resistor networks | Ohm law, resistor power, loaded dividers, series/parallel resistors, source resistance, Wheatstone bridge; exclude temperature/noise | [Research](research/dc.md), [Definitions](research/dc.json) |
| ac | AC impedance and resonance | reactance, RLC impedance/magnitude/phase, series/parallel resonance, Q, bandwidth, power factor; exclude filter synthesis | [Research](research/ac.md), [Definitions](research/ac.json) |
| transients | Time-domain passive transients | RC/RL charge/discharge/time targets, stored energy, damped RLC transient; exclude filters and switch-mode converter sizing | [Research](research/transients.md), [Definitions](research/transients.json) |
| diodes | Diodes LEDs and rectification | LED resistor, diode Shockley current, bridge loss, rectifier ripple, zener sizing, clipping, LED parallel branch design; exclude BJT/MOSFET | [Research](research/diodes.md), [Definitions](research/diodes.json) |
| bjt | Bipolar transistor design | bias, base resistor, small signal gm/rpi, emitter degeneration, saturation switch, collector power, Early output resistance | [Research](research/bjt.md), [Definitions](research/bjt.json) |
| mosfet | MOSFET and gate drive | gate charge/current, conduction/switching loss, gate resistance, Miller time, linear-region current, bootstrap capacitance; exclude thermal temperature calculations | [Research](research/mosfet.md), [Definitions](research/mosfet.json) |
| opamps | Operational amplifier stages | inverting/noninverting/difference/summing gains, integrator/differentiator, slew rate, gain bandwidth; exclude noise and filters | [Research](research/opamps.md), [Definitions](research/opamps.json) |
| filters | Analog filter design | first/second order low/high/band pass, Butterworth cutoff, Sallen Key Q, notch, cascaded response; exclude bare AC resonance | [Research](research/filters.md), [Definitions](research/filters.json) |
| power | Power conversion and regulation | buck/boost duty ripple inductance capacitor, LDO dissipation, efficiency/input current; exclude diode-specific rectifier calculations | [Research](research/power.md), [Definitions](research/power.json) |
| magnetics | Magnetics and transformers | turns ratio, flux density, toroid inductance, air-gap inductance, magnetic energy, skin depth, transformer VA, coupled inductance | [Research](research/magnetics.md), [Definitions](research/magnetics.json) |
| signals | Signal integrity and transmission lines | microstrip/stripline impedance, delay, reflections/termination, coax impedance, rise-time bandwidth, wavelength; exclude PCB current and EMC coupling | [Research](research/signals.md), [Definitions](research/signals.json) |
| pcb | PCB interconnect and manufacturing | trace resistance/drop, IPC-2221 current estimate with caveat, via resistance, copper thickness conversion, differential skew, annular ring, trace Joule heating; exclude RF impedance | [Research](research/pcb.md), [Definitions](research/pcb.json) |
| thermal | Thermal and reliability design | junction temperature, heatsink resistance, thermal path, transient thermal lump, Arrhenius acceleration, derating, resistor temperature coefficient, thermal expansion | [Research](research/thermal.md), [Definitions](research/thermal.json) |
| data | ADC DAC and sampled systems | LSB, quantization error/SNR, ENOB, sampling/Nyquist, alias frequency, DAC output, acquisition settling; exclude analog noise budgets | [Research](research/data.md), [Definitions](research/data.json) |
| noise | Noise measurement and analog budgets | Johnson/shot noise, RMS noise sum, amplifier noise, dB/linear, dynamic range, noise figure/Friis; exclude ADC SNR | [Research](research/noise.md), [Definitions](research/noise.json) |
| timing | Digital timing interfaces and EMC | I2C pullup bounds, RC edge threshold timing, clock divider, UART error, setup slack, capacitive crosstalk, inductive spike, displacement current; exclude analog passive charge general cases | [Research](research/timing.md), [Definitions](research/timing.json) |

## Actual level summary and implementation handoff

Exactly 16 implementation lanes each have one independent developer, tester and reviewer. Developers own `backend/catalog/<lane>.json` and `docs/features/<lane>.md` on isolated `feat/<lane>` worktrees; testers own `backend/tests/<lane>.rs` on `test/<lane>` worktrees; reviewers are read-only on implementation and write their lane report from root. Parent owns engine, Flutter, integration and actual desktop verification. All 128 are Must-have in this sprint; no later backlog deferrals.

| Lane | Basic | Intermediate | Expert | Total | Feature docs | Independent tests | Review report |
|---|---:|---:|---:|---:|---|---|---|
| dc | 3 | 3 | 2 | 8 | `docs/features/dc.md` | `backend/tests/dc.rs` | `coordination/reviews/dc.md` |
| ac | 3 | 4 | 1 | 8 | `docs/features/ac.md` | `backend/tests/ac.rs` | `coordination/reviews/ac.md` |
| transients | 3 | 3 | 2 | 8 | `docs/features/transients.md` | `backend/tests/transients.rs` | `coordination/reviews/transients.md` |
| diodes | 2 | 3 | 3 | 8 | `docs/features/diodes.md` | `backend/tests/diodes.rs` | `coordination/reviews/diodes.md` |
| bjt | 3 | 3 | 2 | 8 | `docs/features/bjt.md` | `backend/tests/bjt.rs` | `coordination/reviews/bjt.md` |
| mosfet | 2 | 3 | 3 | 8 | `docs/features/mosfet.md` | `backend/tests/mosfet.rs` | `coordination/reviews/mosfet.md` |
| opamps | 2 | 4 | 2 | 8 | `docs/features/opamps.md` | `backend/tests/opamps.rs` | `coordination/reviews/opamps.md` |
| filters | 2 | 3 | 3 | 8 | `docs/features/filters.md` | `backend/tests/filters.rs` | `coordination/reviews/filters.md` |
| power | 2 | 3 | 3 | 8 | `docs/features/power.md` | `backend/tests/power.rs` | `coordination/reviews/power.md` |
| magnetics | 2 | 3 | 3 | 8 | `docs/features/magnetics.md` | `backend/tests/magnetics.rs` | `coordination/reviews/magnetics.md` |
| signals | 2 | 3 | 3 | 8 | `docs/features/signals.md` | `backend/tests/signals.rs` | `coordination/reviews/signals.md` |
| pcb | 2 | 3 | 3 | 8 | `docs/features/pcb.md` | `backend/tests/pcb.rs` | `coordination/reviews/pcb.md` |
| thermal | 3 | 3 | 2 | 8 | `docs/features/thermal.md` | `backend/tests/thermal.rs` | `coordination/reviews/thermal.md` |
| data | 3 | 3 | 2 | 8 | `docs/features/data.md` | `backend/tests/data.rs` | `coordination/reviews/data.md` |
| noise | 3 | 3 | 2 | 8 | `docs/features/noise.md` | `backend/tests/noise.rs` | `coordination/reviews/noise.md` |
| timing | 2 | 3 | 3 | 8 | `docs/features/timing.md` | `backend/tests/timing.rs` | `coordination/reviews/timing.md` |
| **Total** | **39** | **50** | **39** | **128** | 16 developers | 16 testers | 16 reviewers |

Paths above identify assigned deliverables, not assertions that they already exist or have passed acceptance. Dev-timing has the parent-approved exception to update its research JSON/Markdown for stable `expm1` crosstalk. Main owns the shared engine/contract extension. Testing and review run independently in parallel against the fully integrated development snapshot, with at most 32 concurrent workers and no worker build/test runs.

## Full actual feature index

Each ID selects its exact object in the linked lane JSON and its physical derivation/source/example in the linked lane findings above. Every source/formula/model/default/example must survive catalog promotion. Full independent output oracles, model-specific boundaries and developer/tester/reviewer ownership are in BACKLOG.md.

| ID | Actual researched feature | Level |
|---|---|---|
| `dc.ohm_current` | Signed Ohm-law current | basic |
| `dc.resistor_dissipation` | Steady resistor dissipation | basic |
| `dc.series_parallel_branches` | Two series branches in parallel | basic |
| `dc.loaded_divider` | Finite-load voltage divider | intermediate |
| `dc.resistive_source_matching` | Resistive source loading and maximum-power match | intermediate |
| `dc.loaded_wheatstone_bridge` | Loaded Wheatstone bridge and common-mode voltage | expert |
| `dc.delta_to_star` | Delta-to-star resistor equivalence | intermediate |
| `dc.two_source_node` | Two-source resistive node with load | expert |
| `ac.inductor_reactance` | Ideal inductor AC reactance and current | basic |
| `ac.capacitor_reactance` | Ideal capacitor AC reactance and current | basic |
| `ac.series_rlc_impedance` | Series RLC impedance, phase and current | intermediate |
| `ac.parallel_rlc_impedance` | Parallel RLC admittance and impedance | intermediate |
| `ac.series_resonator_bandwidth` | Series resonator frequency, Q and exact bandwidth | intermediate |
| `ac.lossy_coil_antiresonance` | Lossy-coil parallel phase resonance | expert |
| `ac.sinusoidal_power_triangle` | Single-phase sinusoidal power triangle | basic |
| `ac.shunt_power_factor_correction` | Single-phase shunt capacitor power-factor correction | intermediate |
| `transients.rc_step` | RC charge or discharge after a voltage step | basic |
| `transients.rl_step` | RL current buildup or decay after a voltage step | basic |
| `transients.capacitor_constant_current` | Constant-current capacitor ramp and energy | basic |
| `transients.capacitor_charge_sharing` | Two-capacitor charge redistribution through resistance | intermediate |
| `transients.rc_periodic_switching` | Periodic RC charging extrema under two-level excitation | intermediate |
| `transients.rc_ramp_source` | RC tracking of a finite-slope voltage ramp | intermediate |
| `transients.rc_ladder_step` | Unbuffered two-stage RC ladder startup | expert |
| `transients.rlc_underdamped_step` | Underdamped series-RLC step ringing and peak stress | expert |
| `diodes.led_series_resistor` | LED series resistor and tolerance corners | basic |
| `diodes.shockley_forward_current` | Shockley diode current and incremental resistance | expert |
| `diodes.bridge_conduction_loss` | Bridge rectifier waveform-dependent conduction loss | intermediate |
| `diodes.full_wave_capacitor_ripple` | Full-wave capacitor-input rectifier ripple | intermediate |
| `diodes.zener_shunt_resistor_window` | Zener shunt-regulator resistor design window | expert |
| `diodes.biased_two_sided_clipper` | Biased two-sided diode clipper | basic |
| `diodes.parallel_led_ballast` | Two-branch LED ballast and current sharing | intermediate |
| `diodes.reverse_recovery_charge_budget` | Diode reverse-recovery charge and commutation budget | expert |
| `bjt.divider_bias` | Loaded-divider BJT DC operating point | intermediate |
| `bjt.saturated_switch_drive` | Saturated BJT switch base-drive sizing | basic |
| `bjt.hybrid_pi_parameters` | BJT hybrid-pi transconductance and input resistance | basic |
| `bjt.emitter_degeneration_gain` | Loaded emitter-degenerated common-emitter gain | intermediate |
| `bjt.terminal_dissipation` | BJT DC terminal power dissipation | basic |
| `bjt.early_output_resistance` | Early-effect incremental output resistance | expert |
| `bjt.differential_pair_steering` | Matched BJT differential-pair current steering | expert |
| `bjt.emitter_follower_loading` | Loaded emitter-follower gain and input resistance | intermediate |
| `mosfet.gate_charge_supply` | Gate-charge average supply current and drive power | basic |
| `mosfet.channel_conduction_loss` | MOSFET conduction loss with on-state current ripple | basic |
| `mosfet.hard_switch_overlap_loss` | Clamped-inductive switching overlap loss | intermediate |
| `mosfet.gate_loop_damping_resistor` | Gate-loop critical-damping resistor selection | expert |
| `mosfet.miller_plateau_timing` | Miller plateau source/sink timing | intermediate |
| `mosfet.long_channel_triode_current` | Long-channel NMOS triode-region current | intermediate |
| `mosfet.bootstrap_charge_budget` | Bootstrap capacitance from gate and driver charge budget | expert |
| `mosfet.output_capacitance_hard_turn_on_loss` | MOSFET output-capacitance hard turn-on loss | expert |
| `opamps.inverting_stage` | Inverting amplifier with reference bias | basic |
| `opamps.noninverting_stage` | Noninverting amplifier with reference bias | basic |
| `opamps.difference_ratio_error` | Difference amplifier resistor-ratio error | expert |
| `opamps.weighted_summing_stage` | Three-input weighted summing amplifier | intermediate |
| `opamps.integrator_constant_input` | Integrator constant-input ramp | intermediate |
| `opamps.differentiator_ramp` | Differentiator response to an input ramp | intermediate |
| `opamps.sinusoidal_slew_requirement` | Sinusoidal output slew-rate requirement | intermediate |
| `opamps.dominant_pole_gain_bandwidth` | Finite-gain dominant-pole amplifier bandwidth | expert |
| `filters.rc_lowpass_response` | First-order RC low-pass response | basic |
| `filters.rc_highpass_response` | First-order RC high-pass response | basic |
| `filters.second_order_highpass` | Second-order active high-pass response | intermediate |
| `filters.deliyannis_bandpass_design` | Modified Deliyannis band-pass component design | expert |
| `filters.butterworth_spec_synthesis` | Butterworth order and cutoff from attenuation specifications | expert |
| `filters.sallen_key_lowpass` | General Sallen-Key low-pass pole, Q and response | expert |
| `filters.fliege_notch_response` | Fliege notch component and frequency response | intermediate |
| `filters.buffered_rc_cascade` | Buffered identical RC low-pass cascade | intermediate |
| `power.buck_inductor_design` | Buck CCM inductor sizing | intermediate |
| `power.boost_inductor_design` | Boost CCM inductor sizing | intermediate |
| `power.buck_output_capacitor` | Buck output capacitor with ESR budget | intermediate |
| `power.boost_output_capacitor` | Boost output capacitor with ESR budget | expert |
| `power.buck_input_capacitor_rms` | Buck input capacitor RMS current | expert |
| `power.ldo_dissipation` | LDO dissipation including ground current | basic |
| `power.converter_input_budget` | Converter input current and loss budget | basic |
| `power.boost_rhp_zero` | Boost right-half-plane zero | expert |
| `magnetics.turns_ratio` | Ideal transformer turns ratio and reflected load | basic |
| `magnetics.sinusoidal_flux_density` | Sinusoidal transformer peak flux density | intermediate |
| `magnetics.rectangular_toroid_inductance` | Rectangular-section toroid inductance | expert |
| `magnetics.gapped_core_inductance` | Finite-permeability gapped-core inductance | expert |
| `magnetics.field_energy_density` | Magnetic field energy in a material volume | intermediate |
| `magnetics.conductor_skin_depth` | Good-conductor skin depth | expert |
| `magnetics.transformer_va_budget` | Two-secondary transformer VA budget | basic |
| `magnetics.coupled_coil_inductance` | Mutual and series-connected coupled-coil inductance | intermediate |
| `signals.microstrip_impedance` | Thin-conductor microstrip impedance | expert |
| `signals.stripline_impedance` | Symmetric finite-thickness stripline impedance | expert |
| `signals.coax_impedance` | Homogeneous coaxial line impedance | intermediate |
| `signals.line_propagation` | Transmission-line delay and wavelength | basic |
| `signals.load_reflection` | Resistive-load reflection and first arrival | intermediate |
| `signals.source_series_termination` | Source-series termination for an open receiver | intermediate |
| `signals.rise_time_bandwidth` | Single-pole rise-time equivalent bandwidth | basic |
| `signals.quarter_wave_transformer` | Quarter-wave impedance transformer | expert |
| `pcb.trace_resistance_drop` | PCB trace DC resistance and voltage drop | basic |
| `pcb.ipc2221_current_estimate` | Legacy IPC-2221 trace current screening estimate | expert |
| `pcb.via_barrel_resistance` | Plated via-barrel DC resistance | expert |
| `pcb.nominal_copper_thickness` | Nominal PCB copper weight to thickness | basic |
| `pcb.differential_length_skew` | Differential-pair routed length-mismatch skew | intermediate |
| `pcb.annular_ring_registration` | Annular ring with hole-registration allowance | expert |
| `pcb.trace_joule_heat_density` | Trace volumetric Joule heating and deposited energy | intermediate |
| `pcb.stencil_aperture_area_ratio` | Rectangular solder-stencil aperture area ratio | intermediate |
| `thermal.junction_feedback_equilibrium` | Junction temperature with electrothermal feedback | expert |
| `thermal.heatsink_resistance_budget` | Maximum heatsink thermal resistance | intermediate |
| `thermal.parallel_conduction_paths` | Two parallel solid-conduction heat paths | intermediate |
| `thermal.lumped_thermal_step` | Lumped body temperature after a power step | intermediate |
| `thermal.arrhenius_acceleration` | Arrhenius accelerated-life exposure | expert |
| `thermal.resistor_power_derating` | Linear resistor power derating | basic |
| `thermal.resistor_tcr_drift` | Resistor resistance drift from temperature | basic |
| `thermal.free_linear_expansion` | Free linear thermal expansion | basic |
| `data.adc_lsb` | Ideal ADC LSB voltage | basic |
| `data.adc_quantization_error` | Ideal ADC code and quantization error | intermediate |
| `data.bandlimited_quantization_snr` | Bandlimited ideal quantization SNR | expert |
| `data.dynamic_enob` | Dynamic ENOB from measured SINAD | intermediate |
| `data.nyquist_sampling_margin` | Baseband Nyquist sampling margin | basic |
| `data.tone_alias_frequency` | Sampled tone alias frequency | intermediate |
| `data.unipolar_dac_output` | Ideal unipolar DAC output voltage | basic |
| `data.sar_acquisition_settling` | SAR ADC half-LSB acquisition settling | expert |
| `noise.johnson_resistor` | Resistor Johnson noise | basic |
| `noise.junction_shot` | Junction shot noise | intermediate |
| `noise.uncorrelated_rms_sum` | Three-source RMS noise sum | basic |
| `noise.noninverting_amplifier_budget` | Non-inverting amplifier white-noise budget | expert |
| `noise.decibel_to_linear` | Decibel gain to linear ratios | basic |
| `noise.analog_dynamic_range` | Analog dynamic range and detection margin | intermediate |
| `noise.figure_equivalent_temperature` | Noise figure to equivalent input temperature | intermediate |
| `noise.friis_three_stage` | Three-stage Friis noise cascade | expert |
| `timing.i2c_pullup_bounds` | I2C pullup resistance bounds | intermediate |
| `timing.schmitt_rc_delay_bounds` | RC Schmitt-input digital delay bounds | intermediate |
| `timing.timer_clock_divider` | 16-bit toggle timer clock divider | basic |
| `timing.uart_sampling_margin` | UART baud mismatch and stop-sample margin | expert |
| `timing.synchronous_setup_slack` | Synchronous register setup slack | intermediate |
| `timing.capacitive_crosstalk_peak` | Finite-edge lumped capacitive crosstalk | expert |
| `timing.inductive_turnoff_spike` | Inductive turn-off voltage spike | basic |
| `timing.plate_displacement_current` | Parallel-plate displacement current | expert |

