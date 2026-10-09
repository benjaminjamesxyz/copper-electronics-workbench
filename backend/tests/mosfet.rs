mod common;

use common::{assert_close, calculate};

// SI vectors and independent oracles come from research/mosfet.md's worked
// substitutions, plus the charge, energy and waveform integrals noted below.
// Nonzero electrical quantities use relative tolerance only (no absolute floor).
// All zero oracles here are exact physical zeros: absent channel conduction,
// zero drain bias, or no added passive resistance. Their absolute tolerance is
// deliberately 0.0; no cancellation or measured-data uncertainty is involved.
const GATE: [(&str, f64); 3] = [("q_g", 40e-9), ("f_sw", 250_000.0), ("v_drive", 12.0)];
const CONDUCTION: [(&str, f64); 4] = [
    ("i_mean", 8.0), ("delta_i_pp", 6.0), ("duty", 0.6), ("r_ds_on", 0.015),
];
const OVERLAP: [(&str, f64); 5] = [
    ("v_off", 100.0), ("i_load", 5.0), ("t_on", 40e-9),
    ("t_off", 60e-9), ("f_sw", 200_000.0),
];
const DAMPING: [(&str, f64); 4] = [
    ("l_loop", 32e-9), ("c_iss", 2e-9), ("r_driver", 1.5), ("r_internal", 0.5),
];
const MILLER: [(&str, f64); 7] = [
    ("q_gd", 30e-9), ("v_drive", 12.0), ("v_plateau", 5.0),
    ("r_source", 2.0), ("r_sink", 1.0), ("r_external", 3.0), ("r_internal", 1.0),
];
const TRIODE: [(&str, f64); 4] = [
    ("beta", 0.004), ("v_gs", 2.5), ("v_th", 0.7), ("v_ds", 0.4),
];
const BOOTSTRAP: [(&str, f64); 9] = [
    ("q_g", 100e-9), ("i_hbs", 25e-6), ("i_hb", 500e-6),
    ("d_max", 0.8), ("f_sw", 50_000.0), ("v_dd", 15.0),
    ("v_diode", 1.0), ("v_min", 10.0), ("delta_v", 0.5),
];
const COSS: [(&str, f64); 3] = [
    ("c_o_er", 250e-12), ("v_ds", 200.0), ("f_sw", 500_000.0),
];

#[test]
fn gate_charge_complete_researched_example() {
    let out = calculate("mosfet.gate_charge_supply", &GATE).unwrap();
    // One 40 nC withdrawal every 4 us; charging AND discharging do not double it.
    for (key, expected) in [("i_supply", 0.01), ("e_drive", 480e-9), ("p_drive", 0.12)] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn gate_charge_cycle_budget_and_positive_boundary() {
    // 10 nC each 10 us, 5 V: 1 mA, 50 nJ/cycle, 5 mW.
    let out = calculate("mosfet.gate_charge_supply", &[
        ("q_g", 10e-9), ("f_sw", 100_000.0), ("v_drive", 5.0),
    ]).unwrap();
    for (key, expected) in [("i_supply", 1e-3), ("e_drive", 50e-9), ("p_drive", 0.005)] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
    // There is no inclusive zero endpoint; a small positive charge must not be
    // clamped to zero. These 1 fC / 1 Hz / 1 V values remain representable.
    let small = calculate("mosfet.gate_charge_supply", &[
        ("q_g", 1e-15), ("f_sw", 1.0), ("v_drive", 1.0),
    ]).unwrap();
    for key in ["i_supply", "e_drive", "p_drive"] {
        assert_close(small[key], 1e-15, 1e-6, 0.0);
    }
}

#[test]
fn gate_charge_rejects_nonpositive_and_nonfinite_inputs() {
    for index in 0..GATE.len() {
        for bad in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let mut inputs = GATE;
            inputs[index].1 = bad;
            assert!(calculate("mosfet.gate_charge_supply", &inputs).is_err(), "{inputs:?}");
        }
    }
}

#[test]
fn conduction_complete_researched_example() {
    let out = calculate("mosfet.channel_conduction_loss", &CONDUCTION).unwrap();
    // Endpoints 5 and 11 A: mean square (25 + 55 + 121)/3 = 67 A^2.
    for (key, expected) in [("i_rms", 6.340346993658943), ("p_cond", 0.603)] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn conduction_flat_waveform_zero_valley_and_zero_duty() {
    // Flat 4 A on half the cycle: full-cycle mean square 8 A^2, not 4 A^2.
    let flat = calculate("mosfet.channel_conduction_loss", &[
        ("i_mean", 4.0), ("delta_i_pp", 0.0), ("duty", 0.5), ("r_ds_on", 0.02),
    ]).unwrap();
    assert_close(flat["i_rms"], 2.8284271247461903, 1e-6, 0.0);
    assert_close(flat["p_cond"], 0.16, 1e-6, 0.0);
    // Ramp endpoints 0 and 6 A, full duty: average square 36/3 = 12 A^2.
    let valley = calculate("mosfet.channel_conduction_loss", &[
        ("i_mean", 3.0), ("delta_i_pp", 6.0), ("duty", 1.0), ("r_ds_on", 0.01),
    ]).unwrap();
    assert_close(valley["i_rms"], 3.4641016151377544, 1e-6, 0.0);
    assert_close(valley["p_cond"], 0.12, 1e-6, 0.0);
    for inputs in [
        [("i_mean", 3.0), ("delta_i_pp", 6.0), ("duty", 0.0), ("r_ds_on", 0.01)],
        [("i_mean", 0.0), ("delta_i_pp", 0.0), ("duty", 1.0), ("r_ds_on", 0.01)],
    ] {
        let out = calculate("mosfet.channel_conduction_loss", &inputs).unwrap();
        assert_close(out["i_rms"], 0.0, 1e-6, 0.0);
        assert_close(out["p_cond"], 0.0, 1e-6, 0.0);
    }
}

#[test]
fn conduction_rejects_negative_current_invalid_duty_and_negative_valley() {
    for (index, bad) in [(0, -1.0), (1, -1.0), (2, -0.01), (2, 1.01), (3, 0.0), (3, -0.01), (1, 16.01)] {
        let mut inputs = CONDUCTION;
        inputs[index].1 = bad;
        assert!(calculate("mosfet.channel_conduction_loss", &inputs).is_err(), "{inputs:?}");
    }
    // Even with duty zero, a waveform outside the model remains inadmissible.
    let outside = [("i_mean", 1.0), ("delta_i_pp", 3.0), ("duty", 0.0), ("r_ds_on", 0.01)];
    assert!(calculate("mosfet.channel_conduction_loss", &outside).is_err());
    for index in 0..CONDUCTION.len() {
        for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let mut inputs = CONDUCTION;
            inputs[index].1 = bad;
            assert!(calculate("mosfet.channel_conduction_loss", &inputs).is_err(), "{inputs:?}");
        }
    }
}

#[test]
fn overlap_complete_researched_example() {
    let out = calculate("mosfet.hard_switch_overlap_loss", &OVERLAP).unwrap();
    for (key, expected) in [("e_on", 10e-6), ("e_off", 15e-6), ("e_cycle", 25e-6), ("p_overlap", 5.0)] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn overlap_swapping_edges_and_zero_load_preserves_energy_accounting() {
    // 100 W triangle heights, exchanged 80/20 ns edges: 4/1 uJ.
    for (on, off, e_on, e_off) in [(80e-9, 20e-9, 4e-6, 1e-6), (20e-9, 80e-9, 1e-6, 4e-6)] {
        let out = calculate("mosfet.hard_switch_overlap_loss", &[
            ("v_off", 50.0), ("i_load", 2.0), ("t_on", on), ("t_off", off), ("f_sw", 100_000.0),
        ]).unwrap();
        for (key, expected) in [("e_on", e_on), ("e_off", e_off), ("e_cycle", 5e-6), ("p_overlap", 0.5)] {
            assert_close(out[key], expected, 1e-6, 0.0);
        }
    }
    // Exactly representable occupancy 3/4 is valid; load=0 makes every
    // overlap energy zero, not the separate output-capacitance loss.
    let zero = calculate("mosfet.hard_switch_overlap_loss", &[
        ("v_off", 50.0), ("i_load", 0.0), ("t_on", 0.25), ("t_off", 0.5), ("f_sw", 1.0),
    ]).unwrap();
    for key in ["e_on", "e_off", "e_cycle", "p_overlap"] {
        assert_close(zero[key], 0.0, 1e-6, 0.0);
    }
}

#[test]
fn overlap_rejects_period_occupancy_and_invalid_ranges() {
    // Binary-exact durations isolate strict equality and above-period guards.
    for frequency in [2.0, 3.0] {
        assert!(calculate("mosfet.hard_switch_overlap_loss", &[
            ("v_off", 50.0), ("i_load", 1.0), ("t_on", 0.25), ("t_off", 0.25), ("f_sw", frequency),
        ]).is_err());
    }
    for index in 0..OVERLAP.len() {
        let bad_values: &[f64] = if index == 1 {
            &[-1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY]
        } else {
            &[0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY]
        };
        for &bad in bad_values {
            let mut inputs = OVERLAP;
            inputs[index].1 = bad;
            assert!(calculate("mosfet.hard_switch_overlap_loss", &inputs).is_err(), "{inputs:?}");
        }
    }
}

#[test]
fn gate_damping_complete_researched_example() {
    let out = calculate("mosfet.gate_loop_damping_resistor", &DAMPING).unwrap();
    for (key, expected) in [("r_critical", 8.0), ("r_external", 6.0), ("zeta", 1.0)] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn gate_damping_handles_ideal_critical_and_overdamped_paths() {
    // Independently solving the series-RLC repeated-pole condition: L/C=4
    // ohm^2 implies total critical R=4 ohm, regardless of existing split.
    for (driver, internal, added, zeta) in [
        (0.0, 0.0, 4.0, 1.0), (3.0, 1.0, 0.0, 1.0), (5.0, 1.0, 0.0, 1.5),
    ] {
        let out = calculate("mosfet.gate_loop_damping_resistor", &[
            ("l_loop", 16e-9), ("c_iss", 4e-9), ("r_driver", driver), ("r_internal", internal),
        ]).unwrap();
        for (key, expected) in [("r_critical", 4.0), ("r_external", added), ("zeta", zeta)] {
            assert_close(out[key], expected, 1e-6, 0.0);
        }
    }
}

#[test]
fn gate_damping_rejects_nonphysical_lc_and_negative_resistance() {
    for index in 0..DAMPING.len() {
        let bad_values: &[f64] = if index < 2 {
            &[0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY]
        } else {
            &[-1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY]
        };
        for &bad in bad_values {
            let mut inputs = DAMPING;
            inputs[index].1 = bad;
            assert!(calculate("mosfet.gate_loop_damping_resistor", &inputs).is_err(), "{inputs:?}");
        }
    }
}

#[test]
fn miller_complete_researched_example() {
    let out = calculate("mosfet.miller_plateau_timing", &MILLER).unwrap();
    for (key, expected) in [
        ("i_gate_on", 1.1666666666666667), ("i_gate_off", 1.0),
        ("t_miller_on", 25.714285714285715e-9), ("t_miller_off", 30e-9),
    ] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn miller_balanced_headroom_and_permitted_zero_resistors() {
    // Mid-supply plateau gives equal 4 V headrooms. 20 nC / 2 A = 10 ns.
    let balanced = calculate("mosfet.miller_plateau_timing", &[
        ("q_gd", 20e-9), ("v_drive", 8.0), ("v_plateau", 4.0),
        ("r_source", 2.0), ("r_sink", 2.0), ("r_external", 0.0), ("r_internal", 0.0),
    ]).unwrap();
    for (key, expected) in [
        ("i_gate_on", 2.0), ("i_gate_off", 2.0), ("t_miller_on", 10e-9), ("t_miller_off", 10e-9),
    ] {
        assert_close(balanced[key], expected, 1e-6, 0.0);
    }
    // Source and sink may EACH be zero when an external or internal resistor
    // supplies the path resistance. Covers all four individual zero endpoints.
    for (external, internal) in [(2.0, 0.0), (0.0, 2.0)] {
        let out = calculate("mosfet.miller_plateau_timing", &[
            ("q_gd", 40e-9), ("v_drive", 10.0), ("v_plateau", 2.0),
            ("r_source", 0.0), ("r_sink", 0.0), ("r_external", external), ("r_internal", internal),
        ]).unwrap();
        for (key, expected) in [
            ("i_gate_on", 4.0), ("i_gate_off", 1.0), ("t_miller_on", 10e-9), ("t_miller_off", 40e-9),
        ] {
            assert_close(out[key], expected, 1e-6, 0.0);
        }
    }
}

#[test]
fn miller_rejects_insufficient_headroom_and_each_zero_total_path() {
    for drive in [5.0, 4.0] {
        let mut inputs = MILLER;
        inputs[1].1 = drive;
        assert!(calculate("mosfet.miller_plateau_timing", &inputs).is_err(), "{inputs:?}");
    }
    // Isolate the source and sink path guards; the other path stays positive.
    for (source, sink) in [(0.0, 1.0), (1.0, 0.0), (0.0, 0.0)] {
        assert!(calculate("mosfet.miller_plateau_timing", &[
            ("q_gd", 20e-9), ("v_drive", 8.0), ("v_plateau", 4.0),
            ("r_source", source), ("r_sink", sink), ("r_external", 0.0), ("r_internal", 0.0),
        ]).is_err());
    }
    for index in 0..MILLER.len() {
        let bad_values: &[f64] = if index < 3 {
            &[0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY]
        } else {
            &[-1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY]
        };
        for &bad in bad_values {
            let mut inputs = MILLER;
            inputs[index].1 = bad;
            assert!(calculate("mosfet.miller_plateau_timing", &inputs).is_err(), "{inputs:?}");
        }
    }
}

#[test]
fn triode_complete_researched_example() {
    let out = calculate("mosfet.long_channel_triode_current", &TRIODE).unwrap();
    for (key, expected) in [("v_overdrive", 1.8), ("i_d", 0.00256)] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn triode_channel_charge_integral_and_zero_bias_boundaries() {
    // Average inversion overdrive along 0..1 V is (2 + 1)/2=1.5 V;
    // beta=1 mA/V^2 yields 1.5 mA, with no extra half factor in beta.
    let out = calculate("mosfet.long_channel_triode_current", &[
        ("beta", 0.001), ("v_gs", 3.0), ("v_th", 1.0), ("v_ds", 1.0),
    ]).unwrap();
    assert_close(out["v_overdrive"], 2.0, 1e-6, 0.0);
    assert_close(out["i_d"], 0.0015, 1e-6, 0.0);
    // Threshold=0 is admitted, as is drain bias=0 (not negative drain bias).
    for (drain, current) in [(0.0, 0.0), (1.0, 0.0015), (1e-9, 1.9999999995e-12)] {
        let boundary = calculate("mosfet.long_channel_triode_current", &[
            ("beta", 0.001), ("v_gs", 2.0), ("v_th", 0.0), ("v_ds", drain),
        ]).unwrap();
        assert_close(boundary["v_overdrive"], 2.0, 1e-6, 0.0);
        assert_close(boundary["i_d"], current, 1e-6, 0.0);
    }
}

#[test]
fn triode_rejects_threshold_and_saturation_boundaries() {
    for gate in [1.0, 0.5] {
        assert!(calculate("mosfet.long_channel_triode_current", &[
            ("beta", 0.001), ("v_gs", gate), ("v_th", 1.0), ("v_ds", 0.0),
        ]).is_err());
    }
    for drain in [2.0, 2.5] {
        assert!(calculate("mosfet.long_channel_triode_current", &[
            ("beta", 0.001), ("v_gs", 3.0), ("v_th", 1.0), ("v_ds", drain),
        ]).is_err());
    }
    for index in 0..TRIODE.len() {
        let bad_values: &[f64] = if index < 2 {
            &[0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY]
        } else {
            &[-1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY]
        };
        for &bad in bad_values {
            let mut inputs = TRIODE;
            inputs[index].1 = bad;
            assert!(calculate("mosfet.long_channel_triode_current", &inputs).is_err(), "{inputs:?}");
        }
    }
}

#[test]
fn bootstrap_complete_researched_example() {
    let out = calculate("mosfet.bootstrap_charge_budget", &BOOTSTRAP).unwrap();
    // Gate 100 nC + leakage 0.4 nC + full-cycle quiescent 10 nC.
    for (key, expected) in [("q_total", 110.4e-9), ("v_headroom", 4.0), ("c_min", 220.8e-9)] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn bootstrap_distinguishes_on_time_leakage_from_full_cycle_quiescence() {
    // T=10 us: I_hbs=1 mA contributes 10*d nC; I_hb=2 mA
    // contributes 20 nC at BOTH duties, not 20*d nC.
    for (duty, charge, capacitance) in [(0.25, 62.5e-9, 31.25e-9), (0.75, 67.5e-9, 33.75e-9)] {
        let out = calculate("mosfet.bootstrap_charge_budget", &[
            ("q_g", 40e-9), ("i_hbs", 0.001), ("i_hb", 0.002),
            ("d_max", duty), ("f_sw", 100_000.0), ("v_dd", 12.0),
            ("v_diode", 1.0), ("v_min", 7.0), ("delta_v", 2.0),
        ]).unwrap();
        for (key, expected) in [("q_total", charge), ("v_headroom", 4.0), ("c_min", capacitance)] {
            assert_close(out[key], expected, 1e-6, 0.0);
        }
    }
    // No driver current/diode loss, allowed droop EXACTLY equals headroom.
    let boundary = calculate("mosfet.bootstrap_charge_budget", &[
        ("q_g", 40e-9), ("i_hbs", 0.0), ("i_hb", 0.0),
        ("d_max", 0.5), ("f_sw", 100_000.0), ("v_dd", 12.0),
        ("v_diode", 0.0), ("v_min", 8.0), ("delta_v", 4.0),
    ]).unwrap();
    for (key, expected) in [("q_total", 40e-9), ("v_headroom", 4.0), ("c_min", 10e-9)] {
        assert_close(boundary[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn bootstrap_rejects_unrefreshable_duty_and_invalid_voltage_budget() {
    for (index, bad) in [
        (0, 0.0), (0, -1.0), (1, -1.0), (2, -1.0),
        (3, 0.0), (3, 1.0), (3, -0.01), (3, 1.01),
        (4, 0.0), (4, -1.0), (5, 0.0), (5, -1.0), (6, -1.0),
        (7, 0.0), (7, -1.0), (8, 0.0), (8, -1.0),
        // Zero and negative headroom; strictly excessive permitted droop.
        (5, 11.0), (5, 10.0), (8, 4.01),
    ] {
        let mut inputs = BOOTSTRAP;
        inputs[index].1 = bad;
        assert!(calculate("mosfet.bootstrap_charge_budget", &inputs).is_err(), "{inputs:?}");
    }
    for index in 0..BOOTSTRAP.len() {
        for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let mut inputs = BOOTSTRAP;
            inputs[index].1 = bad;
            assert!(calculate("mosfet.bootstrap_charge_budget", &inputs).is_err(), "{inputs:?}");
        }
    }
}

#[test]
fn output_capacitance_complete_researched_example() {
    let out = calculate("mosfet.output_capacitance_hard_turn_on_loss", &COSS).unwrap();
    for (key, expected) in [("e_oss", 5e-6), ("p_oss", 2.5)] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn output_capacitance_voltage_squared_and_frequency_scaling() {
    // These are separate illustrative energy-equivalent C values stipulated
    // at EACH voltage; they do not assume a real device's Coss is constant.
    // Integral of v*C dv: 100 pF at 100 V stores 0.5 uJ; at 200 V 2 uJ.
    for (voltage, frequency, energy, power) in [
        (100.0, 100_000.0, 0.5e-6, 0.05),
        (200.0, 100_000.0, 2e-6, 0.2),
        (200.0, 200_000.0, 2e-6, 0.4),
    ] {
        let out = calculate("mosfet.output_capacitance_hard_turn_on_loss", &[
            ("c_o_er", 100e-12), ("v_ds", voltage), ("f_sw", frequency),
        ]).unwrap();
        assert_close(out["e_oss"], energy, 1e-6, 0.0);
        assert_close(out["p_oss"], power, 1e-6, 0.0);
    }
    // Zero voltage is excluded; a small positive endpoint has a representable
    // nonzero energy and must not be hidden by an absolute 1e-6 tolerance.
    let small = calculate("mosfet.output_capacitance_hard_turn_on_loss", &[
        ("c_o_er", 1e-12), ("v_ds", 1e-6), ("f_sw", 1.0),
    ]).unwrap();
    assert_close(small["e_oss"], 5e-25, 1e-6, 0.0);
    assert_close(small["p_oss"], 5e-25, 1e-6, 0.0);
}

#[test]
fn output_capacitance_rejects_nonpositive_and_nonfinite_inputs() {
    for index in 0..COSS.len() {
        for bad in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let mut inputs = COSS;
            inputs[index].1 = bad;
            assert!(calculate("mosfet.output_capacitance_hard_turn_on_loss", &inputs).is_err(), "{inputs:?}");
        }
    }
}
