mod common;

use common::{assert_close, calculate};

// Oracles are rational circuit/KCL calculations from research/dc.md, not engine
// observations. Nonzero quantities use the declared 1e-6 relative tolerance and
// no absolute floor. Exact zeros below involve zero excitation or exactly equal
// dyadic ratios, so their physically justified absolute tolerance is zero.

#[test]
fn ohm_current_complete_example_polarity_zero_and_envelope() {
    let out = calculate("dc.ohm_current", &[("voltage", -3.3), ("resistance", 2200.0)]).unwrap();
    assert_eq!(out.len(), 1);
    assert_close(out["current"], -0.0015, 1e-6, 0.0);
    // 5 V / 2 kohm = 2.5 mA; reversing the passive reference reverses I.
    for (voltage, expected) in [(5.0, 0.0025), (-5.0, -0.0025), (0.0, 0.0)] {
        let out = calculate("dc.ohm_current", &[("voltage", voltage), ("resistance", 2000.0)]).unwrap();
        assert_close(out["current"], expected, 1e-6, 0.0);
    }
    for (voltage, resistance, expected) in [
        (1e6, 1e-9, 1e15), (-1e6, 1e12, -1e-6), (1e-12, 1e12, 1e-24),
    ] {
        let out = calculate("dc.ohm_current", &[("voltage", voltage), ("resistance", resistance)]).unwrap();
        assert_close(out["current"], expected, 1e-6, 0.0);
    }
}

#[test]
fn resistor_dissipation_complete_example_sign_invariance_zero_and_envelope() {
    let out = calculate("dc.resistor_dissipation", &[("current", 0.012), ("resistance", 680.0)]).unwrap();
    assert_eq!(out.len(), 1);
    assert_close(out["power"], 0.09792, 1e-6, 0.0);
    // +/-0.25 A through 16 ohm gives +/-4 V and positive 1 W = VI.
    for (current, expected) in [(0.25, 1.0), (-0.25, 1.0), (0.0, 0.0)] {
        let out = calculate("dc.resistor_dissipation", &[("current", current), ("resistance", 16.0)]).unwrap();
        assert_close(out["power"], expected, 1e-6, 0.0);
    }
    for (current, resistance, expected) in [
        (1e6, 1e-9, 1000.0), (-1e6, 1e12, 1e24), (1e-12, 1e-9, 1e-33),
    ] {
        let out = calculate("dc.resistor_dissipation", &[("current", current), ("resistance", resistance)]).unwrap();
        assert_close(out["power"], expected, 1e-6, 0.0);
    }
}

#[test]
fn series_parallel_complete_example_branch_swap_signed_kcl_and_zero() {
    let out = calculate("dc.series_parallel_branches", &[
        ("voltage", 12.0), ("r_1", 100.0), ("r_2", 200.0), ("r_3", 300.0), ("r_4", 300.0),
    ]).unwrap();
    let expected = [("r_branch_1", 300.0), ("r_branch_2", 600.0), ("r_equivalent", 200.0),
        ("i_total", 0.06), ("i_branch_1", 0.04), ("i_branch_2", 0.02)];
    assert_eq!(out.len(), expected.len());
    for (key, value) in expected { assert_close(out[key], value, 1e-6, 0.0); }
    assert_close(out["i_total"], out["i_branch_1"] + out["i_branch_2"], 1e-6, 0.0);
    // Swapping the branches preserves Req; reversing 12 V reverses all I.
    let out = calculate("dc.series_parallel_branches", &[
        ("voltage", -12.0), ("r_1", 300.0), ("r_2", 300.0), ("r_3", 100.0), ("r_4", 200.0),
    ]).unwrap();
    for (key, value) in [("r_branch_1", 600.0), ("r_branch_2", 300.0), ("r_equivalent", 200.0),
        ("i_total", -0.06), ("i_branch_1", -0.02), ("i_branch_2", -0.04)] {
        assert_close(out[key], value, 1e-6, 0.0);
    }
    let out = calculate("dc.series_parallel_branches", &[
        ("voltage", 0.0), ("r_1", 2.0), ("r_2", 6.0), ("r_3", 3.0), ("r_4", 5.0),
    ]).unwrap();
    for (key, value) in [("r_branch_1", 8.0), ("r_branch_2", 8.0), ("r_equivalent", 4.0),
        ("i_total", 0.0), ("i_branch_1", 0.0), ("i_branch_2", 0.0)] {
        assert_close(out[key], value, 1e-6, 0.0);
    }
}

#[test]
fn loaded_divider_complete_example_independent_node_solution_and_zero() {
    let out = calculate("dc.loaded_divider", &[
        ("v_in", 12.0), ("r_top", 10000.0), ("r_bottom", 5000.0), ("r_load", 10000.0),
    ]).unwrap();
    let expected = [("v_open", 4.0), ("r_output", 10000.0 / 3.0), ("v_loaded", 3.0),
        ("i_load", 0.0003), ("loading_error", 0.25)];
    assert_eq!(out.len(), expected.len());
    for (key, value) in expected { assert_close(out[key], value, 1e-6, 0.0); }
    // Equal 2-ohm resistors: (Vin-V)/2 = V/2+V/2 -> V=Vin/3.
    // The unloaded divider gives Vin/2; attenuation is always 1/3.
    for (voltage, open, loaded, current) in [(-9.0, -4.5, -3.0, -1.5), (0.0, 0.0, 0.0, 0.0)] {
        let out = calculate("dc.loaded_divider", &[
            ("v_in", voltage), ("r_top", 2.0), ("r_bottom", 2.0), ("r_load", 2.0),
        ]).unwrap();
        for (key, value) in [("v_open", open), ("r_output", 1.0), ("v_loaded", loaded),
            ("i_load", current), ("loading_error", 1.0 / 3.0)] {
            assert_close(out[key], value, 1e-6, 0.0);
        }
        assert_close((voltage - out["v_loaded"]) / 2.0,
            out["v_loaded"] / 2.0 + out["i_load"], 1e-6, 0.0);
    }
}

#[test]
fn source_matching_complete_example_matched_power_and_energy_conservation() {
    let out = calculate("dc.resistive_source_matching", &[
        ("v_thevenin", 9.0), ("r_source", 3.0), ("r_load", 6.0),
    ]).unwrap();
    let expected = [("current", 1.0), ("v_load", 6.0), ("p_load", 6.0),
        ("p_source_resistance", 3.0), ("r_match", 3.0), ("p_maximum", 6.75)];
    assert_eq!(out.len(), expected.len());
    for (key, value) in expected { assert_close(out[key], value, 1e-6, 0.0); }
    // At a 3-ohm match, |-9/6|=1.5 A and each resistor dissipates 6.75 W.
    for (voltage, current, load_voltage, power) in [(-9.0, -1.5, -4.5, 6.75), (0.0, 0.0, 0.0, 0.0)] {
        let out = calculate("dc.resistive_source_matching", &[
            ("v_thevenin", voltage), ("r_source", 3.0), ("r_load", 3.0),
        ]).unwrap();
        for (key, value) in [("current", current), ("v_load", load_voltage), ("p_load", power),
            ("p_source_resistance", power), ("r_match", 3.0), ("p_maximum", power)] {
            assert_close(out[key], value, 1e-6, 0.0);
        }
        assert_close(out["p_load"] + out["p_source_resistance"], voltage * out["current"], 1e-6, 0.0);
    }
    // Reciprocal load ratios Rl/Rs=1/2 and 2 have the same below-maximum power.
    for (load, current, load_voltage, source_power) in [(1.5, 2.0, 3.0, 12.0), (6.0, 1.0, 6.0, 3.0)] {
        let out = calculate("dc.resistive_source_matching", &[
            ("v_thevenin", 9.0), ("r_source", 3.0), ("r_load", load),
        ]).unwrap();
        for (key, value) in [("current", current), ("v_load", load_voltage), ("p_load", 6.0),
            ("p_source_resistance", source_power), ("r_match", 3.0), ("p_maximum", 6.75)] {
            assert_close(out[key], value, 1e-6, 0.0);
        }
        assert!(out["p_load"] < out["p_maximum"]);
    }
}

#[test]
fn wheatstone_complete_example_signed_loading_and_midpoint_kcl() {
    let out = calculate("dc.loaded_wheatstone_bridge", &[
        ("v_excitation", 10.0), ("r_1", 1000.0), ("r_2", 1000.0),
        ("r_3", 1000.0), ("r_4", 3000.0), ("r_detector", 1250.0),
    ]).unwrap();
    let expected = [("v_open", -2.5), ("r_output", 1250.0), ("i_detector", -0.001),
        ("v_loaded", -1.25), ("v_left", 5.5), ("v_right", 6.75), ("v_common", 6.125)];
    assert_eq!(out.len(), expected.len());
    for (key, value) in expected { assert_close(out[key], value, 1e-6, 0.0); }
    assert_close((10.0 - out["v_left"]) / 1000.0,
        out["v_left"] / 1000.0 + out["i_detector"], 1e-6, 0.0);
    assert_close((10.0 - out["v_right"]) / 1000.0 + out["i_detector"],
        out["v_right"] / 3000.0, 1e-6, 0.0);
    assert_close(out["v_left"] - out["v_right"], out["v_loaded"], 1e-6, 0.0);
    // Interchanging left and right reverses detector direction, not common mode.
    let out = calculate("dc.loaded_wheatstone_bridge", &[
        ("v_excitation", 10.0), ("r_1", 1000.0), ("r_2", 3000.0),
        ("r_3", 1000.0), ("r_4", 1000.0), ("r_detector", 1250.0),
    ]).unwrap();
    for (key, value) in [("v_open", 2.5), ("r_output", 1250.0), ("i_detector", 0.001),
        ("v_loaded", 1.25), ("v_left", 6.75), ("v_right", 5.5), ("v_common", 6.125)] {
        assert_close(out[key], value, 1e-6, 0.0);
    }
    let out = calculate("dc.loaded_wheatstone_bridge", &[
        ("v_excitation", -10.0), ("r_1", 1000.0), ("r_2", 1000.0),
        ("r_3", 1000.0), ("r_4", 3000.0), ("r_detector", 1250.0),
    ]).unwrap();
    for (key, value) in [("v_open", 2.5), ("r_output", 1250.0), ("i_detector", 0.001),
        ("v_loaded", 1.25), ("v_left", -5.5), ("v_right", -6.75), ("v_common", -6.125)] {
        assert_close(out[key], value, 1e-6, 0.0);
    }
}

#[test]
fn wheatstone_balanced_unequal_arms_and_zero_excitation_remain_finite() {
    // Both midpoints are Vs/4 despite unequal arm scales; Rout=750+1500.
    for (voltage, midpoint) in [(8.0, 2.0), (0.0, 0.0), (-8.0, -2.0)] {
        let out = calculate("dc.loaded_wheatstone_bridge", &[
            ("v_excitation", voltage), ("r_1", 3000.0), ("r_2", 1000.0),
            ("r_3", 6000.0), ("r_4", 2000.0), ("r_detector", 1e-9),
        ]).unwrap();
        for (key, value) in [("v_open", 0.0), ("r_output", 2250.0), ("i_detector", 0.0),
            ("v_loaded", 0.0), ("v_left", midpoint), ("v_right", midpoint), ("v_common", midpoint)] {
            assert_close(out[key], value, 1e-6, 0.0);
        }
    }
}

#[test]
fn delta_to_star_complete_example_terminal_equivalence_and_relabeling() {
    let out = calculate("dc.delta_to_star", &[("r_ab", 300.0), ("r_bc", 600.0), ("r_ca", 900.0)]).unwrap();
    let expected = [("r_a", 150.0), ("r_b", 100.0), ("r_c", 300.0)];
    assert_eq!(out.len(), expected.len());
    for (key, value) in expected { assert_close(out[key], value, 1e-6, 0.0); }
    // Third terminal is open: independently reduce each delta terminal pair.
    assert_close(out["r_a"] + out["r_b"], 250.0, 1e-6, 0.0);
    assert_close(out["r_b"] + out["r_c"], 400.0, 1e-6, 0.0);
    assert_close(out["r_c"] + out["r_a"], 450.0, 1e-6, 0.0);
    let out = calculate("dc.delta_to_star", &[("r_ab", 600.0), ("r_bc", 900.0), ("r_ca", 300.0)]).unwrap();
    for (key, value) in [("r_a", 100.0), ("r_b", 300.0), ("r_c", 150.0)] {
        assert_close(out[key], value, 1e-6, 0.0);
    }
    for (resistance, arm) in [(1e-9, 1e-9 / 3.0), (12.0, 4.0), (1e12, 1e12 / 3.0)] {
        let out = calculate("dc.delta_to_star", &[
            ("r_ab", resistance), ("r_bc", resistance), ("r_ca", resistance),
        ]).unwrap();
        for key in ["r_a", "r_b", "r_c"] { assert_close(out[key], arm, 1e-6, 0.0); }
        // R || 2R = 2R/3 for each terminal pair.
        assert_close(out["r_a"] + out["r_b"], 2.0 * arm, 1e-6, 0.0);
        assert_close(out["r_b"] + out["r_c"], 2.0 * arm, 1e-6, 0.0);
        assert_close(out["r_c"] + out["r_a"], 2.0 * arm, 1e-6, 0.0);
    }
}

#[test]
fn wheatstone_near_balance_and_finite_detector_limits() {
    // Original-network KCL, not a midpoint difference oracle:
    // 3Va-Vb=8, -Va+(2+1/(1+d))*Vb=8, d=2^-20.
    // Solving gives Va=(32+24d)/(8+5d), Vb=(32+32d)/(8+5d).
    let d = 1.0 / 1048576.0;
    let out = calculate("dc.loaded_wheatstone_bridge", &[
        ("v_excitation", 8.0), ("r_1", 1.0), ("r_2", 1.0),
        ("r_3", 1.0), ("r_4", 1.0000009536743164), ("r_detector", 1.0),
    ]).unwrap();
    for (key, value) in [
        ("v_open", -4.0 * d / (2.0 + d)),
        ("r_output", (4.0 + 3.0 * d) / (4.0 + 2.0 * d)),
        ("i_detector", -8.0 * d / (8.0 + 5.0 * d)),
        ("v_loaded", -8.0 * d / (8.0 + 5.0 * d)),
        ("v_left", (32.0 + 24.0 * d) / (8.0 + 5.0 * d)),
        ("v_right", (32.0 + 32.0 * d) / (8.0 + 5.0 * d)),
        ("v_common", (32.0 + 28.0 * d) / (8.0 + 5.0 * d)),
    ] {
        assert_close(out[key], value, 1e-6, 0.0);
    }
    // A finite detector at either accepted endpoint must remain finite:
    // the research network has a -2.5 V source with 1250-ohm series R.
    for detector in [1e-9, 1e12] {
        let expected_current = -2.5 / (1250.0 + detector);
        let out = calculate("dc.loaded_wheatstone_bridge", &[
            ("v_excitation", 10.0), ("r_1", 1000.0), ("r_2", 1000.0),
            ("r_3", 1000.0), ("r_4", 3000.0), ("r_detector", detector),
        ]).unwrap();
        let left = 5.0 - 500.0 * expected_current;
        let right = 7.5 + 750.0 * expected_current;
        for (key, value) in [
            ("v_open", -2.5), ("r_output", 1250.0), ("i_detector", expected_current),
            ("v_loaded", detector * expected_current), ("v_left", left),
            ("v_right", right), ("v_common", (left + right) / 2.0),
        ] {
            assert_close(out[key], value, 1e-6, 0.0);
        }
    }
}

#[test]
fn two_source_complete_example_backfeed_source_swap_and_signed_kcl() {
    let out = calculate("dc.two_source_node", &[
        ("v_1", 12.0), ("r_1", 1000.0), ("v_2", 3.0), ("r_2", 2000.0), ("r_load", 2000.0),
    ]).unwrap();
    let expected = [("g_total", 0.002), ("v_node", 6.75), ("i_source_1", 0.00525),
        ("i_source_2", -0.001875), ("i_load", 0.003375)];
    assert_eq!(out.len(), expected.len());
    for (key, value) in expected { assert_close(out[key], value, 1e-6, 0.0); }
    assert_close(out["i_source_1"] + out["i_source_2"], out["i_load"], 1e-6, 0.0);
    let out = calculate("dc.two_source_node", &[
        ("v_1", 3.0), ("r_1", 2000.0), ("v_2", 12.0), ("r_2", 1000.0), ("r_load", 2000.0),
    ]).unwrap();
    for (key, value) in [("g_total", 0.002), ("v_node", 6.75), ("i_source_1", -0.001875),
        ("i_source_2", 0.00525), ("i_load", 0.003375)] {
        assert_close(out[key], value, 1e-6, 0.0);
    }
    let out = calculate("dc.two_source_node", &[
        ("v_1", -12.0), ("r_1", 1000.0), ("v_2", -3.0), ("r_2", 2000.0), ("r_load", 2000.0),
    ]).unwrap();
    for (key, value) in [("g_total", 0.002), ("v_node", -6.75), ("i_source_1", -0.00525),
        ("i_source_2", 0.001875), ("i_load", -0.003375)] {
        assert_close(out[key], value, 1e-6, 0.0);
    }
}

#[test]
fn two_source_opposing_cancellation_and_zero_sources() {
    // 12/1000 + (-24)/2000 = 0 exactly; circulation is not zero current.
    for (v_one, v_two, i_one, i_two) in [(12.0, -24.0, 0.012, -0.012), (0.0, 0.0, 0.0, 0.0)] {
        let out = calculate("dc.two_source_node", &[
            ("v_1", v_one), ("r_1", 1000.0), ("v_2", v_two), ("r_2", 2000.0), ("r_load", 2000.0),
        ]).unwrap();
        for (key, value) in [("g_total", 0.002), ("v_node", 0.0), ("i_source_1", i_one),
            ("i_source_2", i_two), ("i_load", 0.0)] {
            assert_close(out[key], value, 1e-6, 0.0);
        }
    }
}

#[test]
fn two_source_strong_source_current_does_not_disappear_in_rounded_node() {
    // Exact powers of two make the independent oracle explicit. Let
    // d=1+2^-55. Then G=2^29*d, V=2^19/d, Istrong=2^-7/d,
    // Iweak=-2^-8/d and Iload=2^-8/d. Binary64 d rounds to 1, but
    // the currents remain milliamp-scale: subtracting rounded V from Vs is
    // not an admissible way to discard them. No zero absolute tolerance floor.
    for sign in [1.0, -1.0] {
        for swapped in [false, true] {
            let (v_one, r_one, v_two, r_two, i_one, i_two) = if swapped {
                (0.0, 134217728.0, sign * 524288.0, 1.862645149230957e-9,
                    sign * -0.00390625, sign * 0.0078125)
            } else {
                (sign * 524288.0, 1.862645149230957e-9, 0.0, 134217728.0,
                    sign * 0.0078125, sign * -0.00390625)
            };
            let out = calculate("dc.two_source_node", &[
                ("v_1", v_one), ("r_1", r_one), ("v_2", v_two), ("r_2", r_two), ("r_load", 134217728.0),
            ]).unwrap();
            for (key, value) in [("g_total", 536870912.0), ("v_node", sign * 524288.0),
                ("i_source_1", i_one), ("i_source_2", i_two), ("i_load", sign * 0.00390625)] {
                assert_close(out[key], value, 1e-6, 0.0);
            }
            assert_close(out["i_source_1"] + out["i_source_2"], sign * 0.00390625, 1e-6, 0.0);
        }
    }
}

#[test]
fn symmetric_networks_accept_inclusive_resistance_and_voltage_extremes() {
    // Independent equal-resistor reductions exercise every arm at both bounds.
    for resistance in [1e-9, 1e12] {
        for voltage in [-1e6, 1e6] {
            let out = calculate("dc.series_parallel_branches", &[
                ("voltage", voltage), ("r_1", resistance), ("r_2", resistance),
                ("r_3", resistance), ("r_4", resistance),
            ]).unwrap();
            for (key, value) in [("r_branch_1", 2.0 * resistance), ("r_branch_2", 2.0 * resistance),
                ("r_equivalent", resistance), ("i_total", voltage / resistance),
                ("i_branch_1", voltage / (2.0 * resistance)), ("i_branch_2", voltage / (2.0 * resistance))] {
                assert_close(out[key], value, 1e-6, 0.0);
            }
            let out = calculate("dc.loaded_divider", &[
                ("v_in", voltage), ("r_top", resistance), ("r_bottom", resistance), ("r_load", resistance),
            ]).unwrap();
            for (key, value) in [("v_open", voltage / 2.0), ("r_output", resistance / 2.0),
                ("v_loaded", voltage / 3.0), ("i_load", voltage / (3.0 * resistance)), ("loading_error", 1.0 / 3.0)] {
                assert_close(out[key], value, 1e-6, 0.0);
            }
            let out = calculate("dc.resistive_source_matching", &[
                ("v_thevenin", voltage), ("r_source", resistance), ("r_load", resistance),
            ]).unwrap();
            for (key, value) in [("current", voltage / (2.0 * resistance)), ("v_load", voltage / 2.0),
                ("p_load", 1e12 / (4.0 * resistance)), ("p_source_resistance", 1e12 / (4.0 * resistance)),
                ("r_match", resistance), ("p_maximum", 1e12 / (4.0 * resistance))] {
                assert_close(out[key], value, 1e-6, 0.0);
            }
            let out = calculate("dc.loaded_wheatstone_bridge", &[
                ("v_excitation", voltage), ("r_1", resistance), ("r_2", resistance),
                ("r_3", resistance), ("r_4", resistance), ("r_detector", resistance),
            ]).unwrap();
            for (key, value) in [("v_open", 0.0), ("r_output", resistance), ("i_detector", 0.0),
                ("v_loaded", 0.0), ("v_left", voltage / 2.0), ("v_right", voltage / 2.0), ("v_common", voltage / 2.0)] {
                assert_close(out[key], value, 1e-6, 0.0);
            }
            // Three equal grounded branches: Vnode=2Vs/3, each source I=Vs/3R.
            let out = calculate("dc.two_source_node", &[
                ("v_1", voltage), ("r_1", resistance), ("v_2", voltage), ("r_2", resistance), ("r_load", resistance),
            ]).unwrap();
            for (key, value) in [("g_total", 3.0 / resistance), ("v_node", 2.0 * voltage / 3.0),
                ("i_source_1", voltage / (3.0 * resistance)), ("i_source_2", voltage / (3.0 * resistance)),
                ("i_load", 2.0 * voltage / (3.0 * resistance))] {
                assert_close(out[key], value, 1e-6, 0.0);
            }
        }
    }
}

#[test]
fn every_feature_rejects_each_invalid_resistance_signed_range_and_nonfinite_input() {
    // Each case is a complete explicit valid vector before changing exactly one
    // field. Positivity constraints have no additional hidden cross-input guard.
    let cases: &[(&str, &[(&str, f64)])] = &[
        ("dc.ohm_current", &[("voltage", -3.3), ("resistance", 2200.0)]),
        ("dc.resistor_dissipation", &[("current", 0.012), ("resistance", 680.0)]),
        ("dc.series_parallel_branches", &[("voltage", 12.0), ("r_1", 100.0), ("r_2", 200.0), ("r_3", 300.0), ("r_4", 300.0)]),
        ("dc.loaded_divider", &[("v_in", 12.0), ("r_top", 10000.0), ("r_bottom", 5000.0), ("r_load", 10000.0)]),
        ("dc.resistive_source_matching", &[("v_thevenin", 9.0), ("r_source", 3.0), ("r_load", 6.0)]),
        ("dc.loaded_wheatstone_bridge", &[("v_excitation", 10.0), ("r_1", 1000.0), ("r_2", 1000.0), ("r_3", 1000.0), ("r_4", 3000.0), ("r_detector", 1250.0)]),
        ("dc.delta_to_star", &[("r_ab", 300.0), ("r_bc", 600.0), ("r_ca", 900.0)]),
        ("dc.two_source_node", &[("v_1", 12.0), ("r_1", 1000.0), ("v_2", 3.0), ("r_2", 2000.0), ("r_load", 2000.0)]),
    ];
    for &(id, valid) in cases {
        for (index, &(key, _)) in valid.iter().enumerate() {
            let invalid: &[f64] = if key == "resistance" || key.starts_with("r_") {
                &[0.0, -1.0, 5e-10, 1000000000001.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY]
            } else {
                &[-1000001.0, 1000001.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY]
            };
            for &value in invalid {
                let mut inputs = valid.to_vec();
                inputs[index].1 = value;
                assert!(calculate(id, &inputs).is_err(), "{id} must reject {key}={value:?}");
            }
        }
        // Input schema must not fall back to example defaults or accept extras.
        assert!(calculate(id, &valid[1..]).is_err(), "{id}: missing required input");
        let mut extra = valid.to_vec();
        extra.push(("unexpected_field", 1.0));
        assert!(calculate(id, &extra).is_err(), "{id}: extra input");
        // Failed requests must not contaminate subsequent independent calls.
        assert!(calculate(id, valid).is_ok(), "{id}: recovery after domain errors");
    }
}
