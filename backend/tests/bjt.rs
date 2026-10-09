mod common;

use common::{assert_close, calculate};

// MIT 6.101/6.012 bias and hybrid-pi equations, terminal KCL/KVL, and the
// exact SI k/q constants are the independent oracles (research/bjt.md).
// REL is numerical agreement, not a claim about real transistor accuracy.
// Nonzero outputs deliberately have no absolute floor, including tiny currents.
const REL: f64 = 1e-6;
const VT_300: f64 = 0.02585199978643553;
const DIVIDER: [(&str, f64); 7] = [
    ("vcc", 10.0), ("r_top", 40000.0), ("r_bottom", 10000.0),
    ("r_collector", 2000.0), ("r_emitter", 1000.0), ("beta", 99.0), ("vbe", 0.7),
];
const SWITCH: [(&str, f64); 6] = [
    ("i_collector", 0.1), ("beta_forced", 10.0), ("v_drive_min", 3.3),
    ("vbe_sat_max", 0.9), ("r_driver", 20.0), ("i_driver_max", 0.02),
];
const HYBRID: [(&str, f64); 3] = [
    ("i_collector", 0.002), ("beta_ac", 100.0), ("temperature", 300.0),
];
const DEGENERATED: [(&str, f64); 8] = [
    ("i_collector", 0.002), ("beta_ac", 100.0), ("temperature", 300.0),
    ("r_collector", 4000.0), ("r_load", 4000.0), ("r_emitter", 100.0),
    ("r_source", 600.0), ("r_intrinsic_output", 100000.0),
];
const POWER: [(&str, f64); 4] = [
    ("v_ce", 5.0), ("i_collector", 0.02), ("v_be", 0.7), ("i_base", 0.0002),
];
const EARLY: [(&str, f64); 4] = [
    ("v_early", 100.0), ("v_ce", 5.0), ("v_be", 0.7), ("i_collector", 0.001),
];
const PAIR: [(&str, f64); 4] = [
    ("i_tail", 0.002), ("beta", 100.0), ("temperature", 300.0), ("v_differential", 0.02),
];
const FOLLOWER: [(&str, f64); 7] = [
    ("i_collector", 0.002), ("beta_ac", 100.0), ("temperature", 300.0),
    ("r_emitter", 1000.0), ("r_load", 1000.0), ("r_source", 600.0),
    ("r_intrinsic_output", 100000.0),
];

#[test]
fn divider_complete_researched_example() {
    let out = calculate("bjt.divider_bias", &DIVIDER).unwrap();
    for (key, expected) in [
        ("v_thevenin", 2.0), ("r_thevenin", 8000.0),
        ("i_base", 1.2037037037037037e-5), ("i_collector", 0.0011916666666666666),
        ("i_emitter", 0.0012037037037037038), ("v_emitter", 1.2037037037037037),
        ("v_base", 1.9037037037037037), ("v_collector", 7.616666666666667),
        ("v_ce", 6.412962962962963),
    ] {
        assert_close(out[key], expected, REL, 0.0);
    }
}

#[test]
fn divider_low_beta_and_approach_to_cutoff() {
    // A one-ohm Thevenin source, beta=1 and Re=1 give Ib=(2-Vbe)/3.
    // Rc=6 remains active, whereas Rc=7 places collector exactly at base.
    let inputs = [
        ("vcc", 4.0), ("r_top", 2.0), ("r_bottom", 2.0),
        ("r_collector", 6.0), ("r_emitter", 1.0), ("beta", 1.0), ("vbe", 1.0),
    ];
    let out = calculate("bjt.divider_bias", &inputs).unwrap();
    for (key, expected) in [
        ("v_thevenin", 2.0), ("r_thevenin", 1.0), ("i_base", 1.0 / 3.0),
        ("i_collector", 1.0 / 3.0), ("i_emitter", 2.0 / 3.0),
        ("v_emitter", 2.0 / 3.0), ("v_base", 5.0 / 3.0),
        ("v_collector", 2.0), ("v_ce", 4.0 / 3.0),
    ] {
        assert_close(out[key], expected, REL, 0.0);
    }
    assert_close(out["i_emitter"] - out["i_collector"] - out["i_base"], 0.0, 0.0, 1e-15);
    // KCL residual tolerance is < 5 binary64 ulps of the one-ampere scale.
    let epsilon = 1.0 / 1048576.0;
    let mut near_cutoff = inputs;
    near_cutoff[6].1 = 2.0 - epsilon;
    let near = calculate("bjt.divider_bias", &near_cutoff).unwrap();
    for (key, expected) in [
        ("v_thevenin", 2.0), ("r_thevenin", 1.0), ("i_base", epsilon / 3.0),
        ("i_collector", epsilon / 3.0), ("i_emitter", 2.0 * epsilon / 3.0),
        ("v_emitter", 2.0 * epsilon / 3.0), ("v_base", 2.0 - epsilon / 3.0),
        ("v_collector", 4.0 - 2.0 * epsilon), ("v_ce", 4.0 - 8.0 * epsilon / 3.0),
    ] {
        assert_close(near[key], expected, REL, 0.0);
    }
}

#[test]
fn divider_rejects_cutoff_saturation_and_nonpositive_parameters() {
    for index in 0..DIVIDER.len() {
        for invalid in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            let mut inputs = DIVIDER;
            inputs[index].1 = invalid;
            assert!(calculate("bjt.divider_bias", &inputs).is_err(), "{}={invalid}", inputs[index].0);
        }
    }
    let mut inputs = [
        ("vcc", 4.0), ("r_top", 2.0), ("r_bottom", 2.0),
        ("r_collector", 6.0), ("r_emitter", 1.0), ("beta", 1.0), ("vbe", 1.0),
    ];
    for vbe in [2.0, 2.5] {
        inputs[6].1 = vbe;
        assert!(calculate("bjt.divider_bias", &inputs).is_err());
    }
    inputs[6].1 = 1.0;
    for rc in [7.0, 8.0] {
        inputs[3].1 = rc;
        assert!(calculate("bjt.divider_bias", &inputs).is_err());
    }
}

#[test]
fn switch_complete_researched_example() {
    let out = calculate("bjt.saturated_switch_drive", &SWITCH).unwrap();
    for (key, expected) in [
        ("i_base_required", 0.01), ("r_base_max", 220.0), ("p_base_resistor", 0.022),
    ] {
        assert_close(out[key], expected, REL, 0.0);
    }
}

#[test]
fn switch_zero_driver_resistance_current_limit_and_small_positive_resistor() {
    // Fractional forced beta is a ratio, not a discrete count. Driver capacity
    // may equal required Ib; zero source resistance is physically admissible.
    let out = calculate("bjt.saturated_switch_drive", &[
        ("i_collector", 0.25), ("beta_forced", 2.5), ("v_drive_min", 5.0),
        ("vbe_sat_max", 1.0), ("r_driver", 0.0), ("i_driver_max", 0.1),
    ]).unwrap();
    for (key, expected) in [
        ("i_base_required", 0.1), ("r_base_max", 40.0), ("p_base_resistor", 0.4),
    ] {
        assert_close(out[key], expected, REL, 0.0);
    }
    let tiny_r = 1.0 / 1048576.0;
    let near = calculate("bjt.saturated_switch_drive", &[
        ("i_collector", 0.25), ("beta_forced", 1.0), ("v_drive_min", 2.0),
        ("vbe_sat_max", 1.0), ("r_driver", 4.0 - tiny_r), ("i_driver_max", 0.25),
    ]).unwrap();
    for (key, expected) in [
        ("i_base_required", 0.25), ("r_base_max", tiny_r), ("p_base_resistor", tiny_r / 16.0),
    ] {
        assert_close(near[key], expected, REL, 0.0);
    }
}

#[test]
fn switch_rejects_driver_overload_headroom_and_invalid_inputs() {
    for index in 0..SWITCH.len() {
        for invalid in [-1.0, f64::NAN, f64::INFINITY] {
            let mut inputs = SWITCH;
            inputs[index].1 = invalid;
            assert!(calculate("bjt.saturated_switch_drive", &inputs).is_err());
        }
        if index != 4 {
            let mut inputs = SWITCH;
            inputs[index].1 = 0.0;
            assert!(calculate("bjt.saturated_switch_drive", &inputs).is_err());
        }
    }
    let mut inputs = [
        ("i_collector", 0.25), ("beta_forced", 1.0), ("v_drive_min", 2.0),
        ("vbe_sat_max", 1.0), ("r_driver", 4.0), ("i_driver_max", 0.25),
    ];
    for driver_r in [4.0, 5.0] {
        inputs[4].1 = driver_r;
        assert!(calculate("bjt.saturated_switch_drive", &inputs).is_err());
    }
    inputs[4].1 = 0.0;
    inputs[5].1 = 0.249;
    assert!(calculate("bjt.saturated_switch_drive", &inputs).is_err());
    inputs[5].1 = 0.25;
    for drive in [1.0, 0.5] {
        inputs[2].1 = drive;
        assert!(calculate("bjt.saturated_switch_drive", &inputs).is_err());
    }
}

#[test]
fn hybrid_pi_complete_researched_example() {
    let out = calculate("bjt.hybrid_pi_parameters", &HYBRID).unwrap();
    for (key, expected) in [
        ("v_thermal", VT_300), ("g_m", 0.07736345414366722), ("r_pi", 1292.5999893217765),
    ] {
        assert_close(out[key], expected, REL, 0.0);
    }
}

#[test]
fn hybrid_pi_current_scaling_and_inclusive_temperature_endpoints() {
    let doubled = calculate("bjt.hybrid_pi_parameters", &[
        ("i_collector", 0.004), ("beta_ac", 100.0), ("temperature", 300.0),
    ]).unwrap();
    for (key, expected) in [
        ("v_thermal", VT_300), ("g_m", 0.15472690828733444), ("r_pi", 646.2999946608883),
    ] {
        assert_close(doubled[key], expected, REL, 0.0);
    }
    for (temperature, vt) in [(200.0, 0.017234666524290355), (450.0, 0.03877799967965330)] {
        let out = calculate("bjt.hybrid_pi_parameters", &[
            ("i_collector", 0.001), ("beta_ac", 37.5), ("temperature", temperature),
        ]).unwrap();
        // Differentiating Ic=Is exp(Vbe/VT), then beta=gm*rpi, independently.
        for (key, expected) in [("v_thermal", vt), ("g_m", 0.001 / vt), ("r_pi", 37500.0 * vt)] {
            assert_close(out[key], expected, REL, 0.0);
        }
        assert_close(out["g_m"] * out["r_pi"], 37.5, REL, 0.0);
    }
}

#[test]
fn hybrid_pi_rejects_nonpositive_bias_beta_and_temperature_outside_envelope() {
    for index in 0..2 {
        for invalid in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            let mut inputs = HYBRID;
            inputs[index].1 = invalid;
            assert!(calculate("bjt.hybrid_pi_parameters", &inputs).is_err());
        }
    }
    for invalid in [199.999, 450.001, f64::NAN, f64::INFINITY] {
        let mut inputs = HYBRID;
        inputs[2].1 = invalid;
        assert!(calculate("bjt.hybrid_pi_parameters", &inputs).is_err());
    }
}

#[test]
fn degeneration_complete_researched_example() {
    let out = calculate("bjt.emitter_degeneration_gain", &DEGENERATED).unwrap();
    for (key, expected) in [
        ("r_collector_load", 2000.0), ("r_input", 11392.599989321776),
        ("gain_base", -17.55525518208828), ("gain_source", -16.6769508011674),
    ] {
        assert_close(out[key], expected, REL, 0.0);
    }
    assert!(out["gain_base"] < out["gain_source"] && out["gain_source"] < 0.0);
}

#[test]
fn degeneration_low_beta_source_short_and_output_resistance_boundary() {
    // Ic=VT amperes sets gm=1 S; beta=1 sets rpi=1 ohm.
    // KVL gives Rin=1+2*2=5 ohm; collector parallel load=2 ohm.
    let out = calculate("bjt.emitter_degeneration_gain", &[
        ("i_collector", VT_300), ("beta_ac", 1.0), ("temperature", 300.0),
        ("r_collector", 6.0), ("r_load", 3.0), ("r_emitter", 2.0),
        ("r_source", 0.0), ("r_intrinsic_output", 20.0),
    ]).unwrap();
    for (key, expected) in [
        ("r_collector_load", 2.0), ("r_input", 5.0), ("gain_base", -0.4), ("gain_source", -0.4),
    ] {
        assert_close(out[key], expected, REL, 0.0);
    }
    for temperature in [200.0, 450.0] {
        let mut inputs = DEGENERATED;
        inputs[2].1 = temperature;
        inputs[6].1 = 0.0;
        inputs[7].1 = 20000.0;
        let out = calculate("bjt.emitter_degeneration_gain", &inputs).unwrap();
        let rin = 10100.0 + 1292.5999893217765 * temperature / 300.0;
        for (key, expected) in [
            ("r_collector_load", 2000.0), ("r_input", rin),
            ("gain_base", -200000.0 / rin), ("gain_source", -200000.0 / rin),
        ] {
            assert_close(out[key], expected, REL, 0.0);
        }
    }
}

#[test]
fn degeneration_rejects_inadequate_ro_invalid_resistances_and_temperature() {
    for index in [0, 1, 3, 4, 5, 7] {
        for invalid in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            let mut inputs = DEGENERATED;
            inputs[index].1 = invalid;
            assert!(calculate("bjt.emitter_degeneration_gain", &inputs).is_err());
        }
    }
    for invalid in [-0.001, f64::NAN, f64::INFINITY] {
        let mut inputs = DEGENERATED;
        inputs[6].1 = invalid;
        assert!(calculate("bjt.emitter_degeneration_gain", &inputs).is_err());
    }
    for invalid in [199.999, 450.001, f64::NAN, f64::INFINITY] {
        let mut inputs = DEGENERATED;
        inputs[2].1 = invalid;
        assert!(calculate("bjt.emitter_degeneration_gain", &inputs).is_err());
    }
    let mut inputs = DEGENERATED;
    inputs[7].1 = 19999.999;
    assert!(calculate("bjt.emitter_degeneration_gain", &inputs).is_err());
}

#[test]
fn terminal_power_complete_researched_example() {
    let out = calculate("bjt.terminal_dissipation", &POWER).unwrap();
    for (key, expected) in [("p_collector", 0.1), ("p_base", 0.00014), ("p_total", 0.10014)] {
        assert_close(out[key], expected, REL, 0.0);
    }
    // Independent reference at Ve=1 V: Pc+Pb-Pe, with Ie=Ic+Ib.
    assert_close(out["p_total"], 6.0 * 0.02 + 1.7 * 0.0002 - 0.0202, REL, 0.0);
}

#[test]
fn terminal_power_saturation_base_only_and_zero_boundaries() {
    let saturated = calculate("bjt.terminal_dissipation", &[
        ("v_ce", 0.25), ("i_collector", 0.125), ("v_be", 0.75), ("i_base", 0.0625),
    ]).unwrap();
    for (key, expected) in [("p_collector", 0.03125), ("p_base", 0.046875), ("p_total", 0.078125)] {
        assert_close(saturated[key], expected, REL, 0.0);
    }
    // Exact zero is justified: multiplication by zero and summing two zeros
    // carry no cancellation error. No watt-scale floor hides base-only power.
    for (vce, ic, vbe, ib, pc, pb, total) in [
        (5.0, 0.0, 0.7, 0.0, 0.0, 0.0, 0.0),
        (0.0, 0.02, 0.0, 0.0002, 0.0, 0.0, 0.0),
        (5.0, 0.0, 0.7, 0.0002, 0.0, 0.00014, 0.00014),
        (5.0, 0.02, 0.7, 0.0, 0.1, 0.0, 0.1),
        (0.0, 0.02, 0.7, 0.0002, 0.0, 0.00014, 0.00014),
        (5.0, 0.02, 0.0, 0.0002, 0.1, 0.0, 0.1),
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ] {
        let out = calculate("bjt.terminal_dissipation", &[
            ("v_ce", vce), ("i_collector", ic), ("v_be", vbe), ("i_base", ib),
        ]).unwrap();
        for (key, expected) in [("p_collector", pc), ("p_base", pb), ("p_total", total)] {
            assert_close(out[key], expected, REL, 0.0);
        }
    }
}

#[test]
fn terminal_power_rejects_negative_magnitudes_and_nonfinite_inputs() {
    for index in 0..POWER.len() {
        for invalid in [-0.001, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let mut inputs = POWER;
            inputs[index].1 = invalid;
            assert!(calculate("bjt.terminal_dissipation", &inputs).is_err());
        }
    }
}

#[test]
fn early_resistance_complete_researched_example() {
    let out = calculate("bjt.early_output_resistance", &EARLY).unwrap();
    for (key, expected) in [
        ("r_output", 105000.0), ("g_output", 9.523809523809523e-6),
        ("r_output_approx", 100000.0), ("approximation_relative_error", 0.047619047619047616),
    ] {
        assert_close(out[key], expected, REL, 0.0);
    }
    assert_close(out["r_output"] * out["g_output"], 1.0, REL, 0.0);
}

#[test]
fn early_resistance_large_correction_and_just_active_boundary() {
    // I0=0.0625 A and VA=1 V give differential slope 0.0625 S,
    // even though DC Vce/Ic is only 8 ohm, not the true 16 ohm.
    let inputs = [
        ("v_early", 1.0), ("v_ce", 1.0), ("v_be", 0.5), ("i_collector", 0.125),
    ];
    let out = calculate("bjt.early_output_resistance", &inputs).unwrap();
    for (key, expected) in [
        ("r_output", 16.0), ("g_output", 0.0625),
        ("r_output_approx", 8.0), ("approximation_relative_error", 0.5),
    ] {
        assert_close(out[key], expected, REL, 0.0);
    }
    let mut boundary = inputs;
    boundary[1].1 = f64::from_bits(0.5_f64.to_bits() + 1);
    let near = calculate("bjt.early_output_resistance", &boundary).unwrap();
    // One-ulp excess over Vbe is active. Limit values differ by far less
    // than REL, but rejection at equality is asserted separately below.
    for (key, expected) in [
        ("r_output", 12.0), ("g_output", 1.0 / 12.0),
        ("r_output_approx", 8.0), ("approximation_relative_error", 1.0 / 3.0),
    ] {
        assert_close(near[key], expected, REL, 0.0);
    }
}

#[test]
fn early_resistance_rejects_nonactive_junction_and_nonpositive_parameters() {
    for index in 0..EARLY.len() {
        for invalid in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            let mut inputs = EARLY;
            inputs[index].1 = invalid;
            assert!(calculate("bjt.early_output_resistance", &inputs).is_err());
        }
    }
    for vce in [0.7, 0.6] {
        let mut inputs = EARLY;
        inputs[1].1 = vce;
        assert!(calculate("bjt.early_output_resistance", &inputs).is_err());
    }
}

#[test]
fn pair_complete_researched_example() {
    let out = calculate("bjt.differential_pair_steering", &PAIR).unwrap();
    for (key, expected) in [
        ("v_thermal", VT_300), ("alpha", 0.9900990099009901),
        ("current_ratio", 2.1676302961648406), ("i_collector_one", 0.001355062560591518),
        ("i_collector_two", 0.0006251354592104623), ("i_difference", 0.0007299271013810556),
        ("g_differential", 0.03309487233880996),
    ] {
        assert_close(out[key], expected, REL, 0.0);
    }
    assert_close(out["i_collector_one"] + out["i_collector_two"], 0.0019801980198019802, REL, 0.0);
}

#[test]
fn pair_sign_swap_zero_balance_and_sub_ulp_differential_signal() {
    let mut negative = PAIR;
    negative[3].1 = -0.02;
    let out = calculate("bjt.differential_pair_steering", &negative).unwrap();
    for (key, expected) in [
        ("v_thermal", VT_300), ("alpha", 0.9900990099009901),
        ("current_ratio", 1.0 / 2.1676302961648406),
        ("i_collector_one", 0.0006251354592104623), ("i_collector_two", 0.001355062560591518),
        ("i_difference", -0.0007299271013810556), ("g_differential", 0.03309487233880996),
    ] {
        assert_close(out[key], expected, REL, 0.0);
    }
    let balanced = calculate("bjt.differential_pair_steering", &[
        ("i_tail", 0.004), ("beta", 1.0), ("temperature", 300.0), ("v_differential", 0.0),
    ]).unwrap();
    for (key, expected) in [
        ("v_thermal", VT_300), ("alpha", 0.5), ("current_ratio", 1.0),
        ("i_collector_one", 0.001), ("i_collector_two", 0.001),
        ("i_difference", 0.0), ("g_differential", 0.001 / VT_300),
    ] {
        // Idiff=0 exactly at perfect balance; no absolute current floor.
        assert_close(balanced[key], expected, REL, 0.0);
    }
    for vd in [-1e-20, 1e-20] {
        let mut inputs = PAIR;
        inputs[3].1 = vd;
        let near = calculate("bjt.differential_pair_steering", &inputs).unwrap();
        // Taylor remainder is O((Vdiff/VT)^3), utterly below binary64.
        // This must preserve a nonzero 3.83e-22 A signed imbalance even
        // when each collector current rounds to the same value.
        for (key, expected) in [
            ("v_thermal", VT_300), ("alpha", 0.9900990099009901), ("current_ratio", 1.0),
            ("i_collector_one", 0.0009900990099009901), ("i_collector_two", 0.0009900990099009901),
            ("i_difference", 0.03829873967508278 * vd), ("g_differential", 0.03829873967508278),
        ] {
            assert_close(near[key], expected, REL, 0.0);
        }
        assert_eq!(near["i_difference"].is_sign_positive(), vd.is_sign_positive());
    }
}

#[test]
fn pair_forty_thermal_voltage_edges_and_temperature_endpoints() {
    // Independent 65-digit SI/exp oracle, rounded inward at the binary64
    // voltage boundary. Small branch currents need relative, not absolute,
    // checks: they remain about 8.4e-21 A, never a fabricated cutoff zero.
    for (vd, ratio, one, two, difference) in [
        (1.0340799914574213, 2.3538526683701999e17, 0.0019801980198019802,
            8.41258268374572e-21, 0.0019801980198019802),
        (-1.0340799914574213, 4.248354255291589e-18, 8.41258268374572e-21,
            0.0019801980198019802, -0.0019801980198019802),
    ] {
        let mut inputs = PAIR;
        inputs[3].1 = vd;
        let out = calculate("bjt.differential_pair_steering", &inputs).unwrap();
        for (key, expected) in [
            ("v_thermal", VT_300), ("alpha", 0.9900990099009901), ("current_ratio", ratio),
            ("i_collector_one", one), ("i_collector_two", two), ("i_difference", difference),
            ("g_differential", 6.50826454683771e-19),
        ] {
            assert_close(out[key], expected, REL, 0.0);
        }
    }
    for (temperature, vt) in [(200.0, 0.017234666524290355), (450.0, 0.03877799967965330)] {
        let out = calculate("bjt.differential_pair_steering", &[
            ("i_tail", 0.004), ("beta", 1.0), ("temperature", temperature), ("v_differential", 0.0),
        ]).unwrap();
        for (key, expected) in [
            ("v_thermal", vt), ("alpha", 0.5), ("current_ratio", 1.0),
            ("i_collector_one", 0.001), ("i_collector_two", 0.001),
            ("i_difference", 0.0), ("g_differential", 0.001 / vt),
        ] {
            assert_close(out[key], expected, REL, 0.0);
        }
    }
}

#[test]
fn pair_rejects_outside_signed_envelope_invalid_bias_and_temperature() {
    for index in [0, 1] {
        for invalid in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            let mut inputs = PAIR;
            inputs[index].1 = invalid;
            assert!(calculate("bjt.differential_pair_steering", &inputs).is_err());
        }
    }
    for invalid in [199.999, 450.001, f64::NAN, f64::INFINITY] {
        let mut inputs = PAIR;
        inputs[2].1 = invalid;
        assert!(calculate("bjt.differential_pair_steering", &inputs).is_err());
    }
    for invalid in [-1.0340799914574215, 1.0340799914574215, -2.0, 2.0, f64::NAN, f64::INFINITY] {
        let mut inputs = PAIR;
        inputs[3].1 = invalid;
        assert!(calculate("bjt.differential_pair_steering", &inputs).is_err());
    }
}

#[test]
fn follower_complete_researched_example() {
    let out = calculate("bjt.emitter_follower_loading", &FOLLOWER).unwrap();
    for (key, expected) in [
        ("r_emitter_effective", 500.0), ("r_input", 51792.599989321774),
        ("gain_base", 0.9750427669283204), ("gain_source", 0.9638765781864708),
    ] {
        assert_close(out[key], expected, REL, 0.0);
    }
    assert!(0.0 < out["gain_source"] && out["gain_source"] < out["gain_base"] && out["gain_base"] < 1.0);
}

#[test]
fn follower_low_beta_source_short_parallel_load_and_ro_boundary() {
    // rpi=1, Re||RL=2, reflected load=4, hence Rin=5 and gain=4/5.
    // Exchange unequal emitter/load resistors without changing AC loading.
    for (re, load) in [(6.0, 3.0), (3.0, 6.0)] {
        let out = calculate("bjt.emitter_follower_loading", &[
            ("i_collector", VT_300), ("beta_ac", 1.0), ("temperature", 300.0),
            ("r_emitter", re), ("r_load", load), ("r_source", 0.0), ("r_intrinsic_output", 20.0),
        ]).unwrap();
        for (key, expected) in [
            ("r_emitter_effective", 2.0), ("r_input", 5.0), ("gain_base", 0.8), ("gain_source", 0.8),
        ] {
            assert_close(out[key], expected, REL, 0.0);
        }
    }
    for temperature in [200.0, 450.0] {
        let mut inputs = FOLLOWER;
        inputs[2].1 = temperature;
        inputs[5].1 = 0.0;
        inputs[6].1 = 5000.0;
        let out = calculate("bjt.emitter_follower_loading", &inputs).unwrap();
        let rin = 50500.0 + 1292.5999893217765 * temperature / 300.0;
        for (key, expected) in [
            ("r_emitter_effective", 500.0), ("r_input", rin),
            ("gain_base", 50500.0 / rin), ("gain_source", 50500.0 / rin),
        ] {
            assert_close(out[key], expected, REL, 0.0);
        }
    }
}

#[test]
fn follower_rejects_inadequate_ro_invalid_resistances_and_temperature() {
    for index in [0, 1, 3, 4, 6] {
        for invalid in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            let mut inputs = FOLLOWER;
            inputs[index].1 = invalid;
            assert!(calculate("bjt.emitter_follower_loading", &inputs).is_err());
        }
    }
    for invalid in [-0.001, f64::NAN, f64::INFINITY] {
        let mut inputs = FOLLOWER;
        inputs[5].1 = invalid;
        assert!(calculate("bjt.emitter_follower_loading", &inputs).is_err());
    }
    for invalid in [199.999, 450.001, f64::NAN, f64::INFINITY] {
        let mut inputs = FOLLOWER;
        inputs[2].1 = invalid;
        assert!(calculate("bjt.emitter_follower_loading", &inputs).is_err());
    }
    let mut inputs = FOLLOWER;
    inputs[6].1 = 4999.999;
    assert!(calculate("bjt.emitter_follower_loading", &inputs).is_err());
}
