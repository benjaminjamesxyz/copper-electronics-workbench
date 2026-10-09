mod common;

use common::{assert_close, calculate};

// Independent integrals, KCL/KVL, power-law interpolation and thermal-energy oracles.
// All nonzero quantities use relative-only tolerances, including tiny quantities.
const REL: f64 = 1e-10;
const FUSE: [(&str, f64); 8] = [
    ("peak_current", 30.0), ("rise_time", 0.0003), ("hold_time", 0.0005),
    ("fall_time", 0.0006), ("pre_arcing_i2t", 1.0), ("arcing_i2t", 0.25),
    ("pulse_allowance_fraction", 0.5), ("device_withstand_i2t", 1.0),
];
const TVS: [(&str, f64); 8] = [
    ("surge_voltage", 72.0), ("source_resistance", 3.0),
    ("breakdown_intercept", 24.0), ("dynamic_resistance", 1.0),
    ("fit_current_min", 2.0), ("fit_current_max", 16.0),
    ("rated_pulse_current", 10.0), ("protected_voltage_limit", 32.0),
];
const MOV: [(&str, f64); 7] = [
    ("current_one", 2.0), ("voltage_one", 100.0),
    ("current_two", 32.0), ("voltage_two", 200.0),
    ("pulse_current", 8.0), ("pulse_duration", 0.0005), ("energy_allowance", 0.5),
];
const SCR: [(&str, f64); 10] = [
    ("zener_voltage", 8.0), ("gate_voltage", 1.0),
    ("gate_trigger_current", 0.03125), ("gate_resistance", 12.0),
    ("gate_shunt_resistance", 32.0), ("fault_voltage", 13.0),
    ("source_resistance", 4.0), ("on_voltage", 1.0),
    ("latching_current", 4.0), ("holding_current", 0.25),
];
const COPPER: [(&str, f64); 5] = [
    ("area_mm2", 2.0), ("initial_temperature", 0.0),
    ("limit_temperature", 234.0), ("clearing_time", 4.0), ("fault_current", 300.0),
];
const FOLDBACK: [(&str, f64); 5] = [
    ("input_voltage", 10.0), ("knee_voltage", 6.0), ("knee_current", 2.0),
    ("short_current", 0.5), ("fault_resistance", 2.0),
];

#[test]
fn fuse_integrates_asymmetric_edges_and_keeps_failed_design_margins() {
    // Each ramp contributes one third of its duration to integral i^2 dt.
    let out = calculate("protection.fuse_pulse_coordination", &FUSE).unwrap();
    for (key, expected) in [
        ("pulse_i2t", 0.72), ("pulse_margin", -0.22), ("clearing_i2t", 1.25),
        ("clearing_margin", -0.25), ("pulse_utilization", 1.44), ("clearing_utilization", 1.25),
    ] {
        assert_close(out[key], expected, REL, 0.0);
    }
    assert_close(out["pulse_i2t"] + out["pulse_margin"], 0.5, REL, 0.0);
}

#[test]
fn fuse_rectangle_triangle_zero_and_unity_allowance() {
    for (peak, rise, hold, fall, integral) in [
        (30.0, 0.0, 0.0005, 0.0, 0.45),
        (30.0, 0.0003, 0.0, 0.0006, 0.27),
        (0.0, 0.0003, 0.0005, 0.0006, 0.0),
    ] {
        let mut inputs = FUSE;
        inputs[0].1 = peak;
        inputs[1].1 = rise;
        inputs[2].1 = hold;
        inputs[3].1 = fall;
        inputs[5].1 = 0.0;
        inputs[6].1 = 1.0;
        let out = calculate("protection.fuse_pulse_coordination", &inputs).unwrap();
        for (key, expected) in [
            ("pulse_i2t", integral), ("pulse_margin", 1.0 - integral),
            ("pulse_utilization", integral), ("clearing_i2t", 1.0),
            ("clearing_margin", 0.0), ("clearing_utilization", 1.0),
        ] {
            assert_close(out[key], expected, REL, 0.0);
        }
    }
}

#[test]
fn fuse_preserves_tiny_integral_and_avoids_premature_current_square_overflow() {
    for (peak, hold, integral) in [(1e-100, 0.001, 1e-203), (1e160, 1e-20, 1e300)] {
        let mut inputs = FUSE;
        inputs[0].1 = peak;
        inputs[1].1 = 0.0;
        inputs[2].1 = hold;
        inputs[3].1 = 0.0;
        let out = calculate("protection.fuse_pulse_coordination", &inputs).unwrap();
        assert_close(out["pulse_i2t"], integral, REL, 0.0);
        assert_close(out["pulse_utilization"], 2.0 * integral, REL, 0.0);
    }
}

#[test]
fn fuse_rejects_domain_errors_and_recovers() {
    for (index, bad) in [(0, -1.0), (1, -1e-6), (2, -1e-6), (3, -1e-6),
        (4, 0.0), (5, -0.1), (6, 0.0), (6, 1.01), (7, 0.0)] {
        let mut inputs = FUSE;
        inputs[index].1 = bad;
        assert!(calculate("protection.fuse_pulse_coordination", &inputs).is_err());
        assert_close(calculate("protection.fuse_pulse_coordination", &FUSE).unwrap()["pulse_i2t"], 0.72, REL, 0.0);
    }
    for duration in [0.0, 0.01, 0.011] {
        let mut inputs = FUSE;
        inputs[1].1 = 0.0;
        inputs[2].1 = duration;
        inputs[3].1 = 0.0;
        assert!(calculate("protection.fuse_pulse_coordination", &inputs).is_err());
        inputs[2].1 = 0.009;
        assert_close(calculate("protection.fuse_pulse_coordination", &inputs).unwrap()["pulse_i2t"], 8.1, REL, 0.0);
    }
}

#[test]
fn tvs_load_line_all_outputs_and_source_power_balance() {
    let out = calculate("protection.tvs_surge_load_line", &TVS).unwrap();
    for (key, expected) in [("surge_current", 12.0), ("clamp_voltage", 36.0),
        ("peak_power", 432.0), ("voltage_margin", -4.0), ("current_margin", -2.0)] {
        assert_close(out[key], expected, REL, 0.0);
    }
    assert_close(3.0 * out["surge_current"] + out["clamp_voltage"], 72.0, REL, 0.0);
    assert_close(out["peak_power"] + 3.0 * out["surge_current"].powi(2), 864.0, REL, 0.0);
}

#[test]
fn tvs_includes_fit_endpoints_and_ideal_slope() {
    for (source, slope, current, clamp) in [(32.0, 1.0, 2.0, 26.0),
        (88.0, 1.0, 16.0, 40.0), (54.0, 0.0, 10.0, 24.0)] {
        let mut inputs = TVS;
        inputs[0].1 = source;
        inputs[3].1 = slope;
        let out = calculate("protection.tvs_surge_load_line", &inputs).unwrap();
        for (key, expected) in [("surge_current", current), ("clamp_voltage", clamp),
            ("peak_power", current * clamp), ("voltage_margin", 32.0 - clamp),
            ("current_margin", 10.0 - current)] {
            assert_close(out[key], expected, REL, 0.0);
        }
    }
}

#[test]
fn tvs_rejects_off_branch_and_invalid_fit_then_recovers() {
    for (index, bad) in [(0, 24.0), (0, 31.999), (0, 88.001), (1, 0.0),
        (2, 0.0), (3, -0.001), (4, 0.0), (4, 16.0), (4, 17.0),
        (5, 2.0), (5, 1.0), (6, 0.0), (7, 0.0)] {
        let mut inputs = TVS;
        inputs[index].1 = bad;
        assert!(calculate("protection.tvs_surge_load_line", &inputs).is_err());
        assert_close(calculate("protection.tvs_surge_load_line", &TVS).unwrap()["peak_power"], 432.0, REL, 0.0);
    }
}

#[test]
fn mov_geometric_midpoint_complete_oracle_and_incremental_resistance() {
    // Fourfold current with a fourth-root law gives sqrt(2) voltage gain.
    let voltage: f64 = 100.0 * 2.0_f64.sqrt();
    let out = calculate("protection.mov_power_law_pulse", &MOV).unwrap();
    for (key, expected) in [("voltage_exponent", 0.25), ("current_exponent", 4.0),
        ("clamp_voltage", voltage), ("differential_resistance", voltage / 32.0),
        ("absorbed_energy", voltage / 250.0), ("energy_margin", 0.5 - voltage / 250.0)] {
        assert_close(out[key], expected, REL, 0.0);
    }
    assert_close(out["clamp_voltage"].powi(2), 20000.0, REL, 0.0);
    assert_close(out["differential_resistance"] * 8.0 / out["clamp_voltage"], 0.25, REL, 0.0);
}

#[test]
fn mov_calibration_endpoints_and_duration_scaling() {
    for (current, voltage) in [(2.0, 100.0), (32.0, 200.0)] {
        for duration in [0.0005, 0.001] {
            let mut inputs = MOV;
            inputs[4].1 = current;
            inputs[5].1 = duration;
            let out = calculate("protection.mov_power_law_pulse", &inputs).unwrap();
            for (key, expected) in [("voltage_exponent", 0.25), ("current_exponent", 4.0),
                ("clamp_voltage", voltage), ("differential_resistance", voltage / (4.0 * current)),
                ("absorbed_energy", voltage * current * duration),
                ("energy_margin", 0.5 - voltage * current * duration)] {
                assert_close(out[key], expected, REL, 0.0);
            }
        }
    }
}

#[test]
fn mov_close_calibration_retains_positive_exponent() {
    // Binary-exact differences; the first-order log ratio error is below 2e-12.
    let delta: f64 = 2.0_f64.powi(-40);
    let inputs = [("current_one", 1.0), ("voltage_one", 1.0),
        ("current_two", 1.0 + 4.0 * delta), ("voltage_two", 1.0 + delta),
        ("pulse_current", 1.0), ("pulse_duration", 1e-100), ("energy_allowance", 2e-100)];
    let out = calculate("protection.mov_power_law_pulse", &inputs).unwrap();
    for (key, expected) in [("voltage_exponent", 0.25), ("current_exponent", 4.0),
        ("clamp_voltage", 1.0), ("differential_resistance", 0.25),
        ("absorbed_energy", 1e-100), ("energy_margin", 1e-100)] {
        assert_close(out[key], expected, REL, 0.0);
    }
}

#[test]
fn mov_rejects_nonprotective_fit_and_extrapolation_then_recovers() {
    for (index, bad) in [(0, 32.0), (0, 33.0), (1, 200.0), (1, 201.0),
        (3, 1600.0), (3, 1700.0), (4, 1.99), (4, 32.01)] {
        let mut inputs = MOV;
        inputs[index].1 = bad;
        assert!(calculate("protection.mov_power_law_pulse", &inputs).is_err());
        assert_close(calculate("protection.mov_power_law_pulse", &MOV).unwrap()["current_exponent"], 4.0, REL, 0.0);
    }
    for index in 0..MOV.len() {
        for bad in [0.0, -1.0] {
            let mut inputs = MOV;
            inputs[index].1 = bad;
            assert!(calculate("protection.mov_power_law_pulse", &inputs).is_err());
            assert_close(calculate("protection.mov_power_law_pulse", &MOV).unwrap()["current_exponent"], 4.0, REL, 0.0);
        }
    }
}

#[test]
fn mov_finite_energy_must_not_overflow_at_intermediate_power() {
    // All requested outputs are representable. Peak power is not a requested output.
    let inputs = [("current_one", 1e200), ("voltage_one", 1e120),
        ("current_two", 1e202), ("voltage_two", 1e121), ("pulse_current", 1e201),
        ("pulse_duration", 1e-300), ("energy_allowance", 1e23)];
    let root: f64 = 10.0_f64.sqrt();
    let out = calculate("protection.mov_power_law_pulse", &inputs)
        .expect("finite rectangular-pulse energy must survive intermediate VI overflow");
    for (key, expected) in [("voltage_exponent", 0.5), ("current_exponent", 2.0),
        ("clamp_voltage", root * 1e120), ("differential_resistance", root * 0.5e-81),
        ("absorbed_energy", root * 1e21), ("energy_margin", 1e23 - root * 1e21)] {
        assert_close(out[key], expected, REL, 0.0);
    }
}

#[test]
fn scr_source_loaded_gate_kcl_and_negative_latch_margin() {
    let out = calculate("protection.scr_crowbar_trigger_latch", &SCR).unwrap();
    for (key, expected) in [("trigger_source_voltage", 10.0),
        ("available_gate_current", 0.21875), ("available_anode_current", 3.0),
        ("latch_margin", -1.0), ("holding_release_voltage", 2.0)] {
        assert_close(out[key], expected, REL, 0.0);
    }
    // Zener branch carries gate current plus shunt current through BOTH resistances.
    assert_close(9.0 + 16.0 * (out["available_gate_current"] + 0.03125), 13.0, REL, 0.0);
}

#[test]
fn scr_exact_trigger_holding_equality_and_zero_on_drop() {
    let mut inputs = SCR;
    inputs[5].1 = 10.0;
    inputs[7].1 = 0.0;
    inputs[8].1 = 0.25;
    let out = calculate("protection.scr_crowbar_trigger_latch", &inputs).unwrap();
    for (key, expected) in [("trigger_source_voltage", 10.0),
        ("available_gate_current", 0.03125), ("available_anode_current", 2.5),
        ("latch_margin", 2.25), ("holding_release_voltage", 1.0)] {
        assert_close(out[key], expected, REL, 0.0);
    }
}

#[test]
fn scr_shunt_loading_changes_trigger_but_not_on_state() {
    let mut inputs = SCR;
    inputs[4].1 = 64.0;
    let out = calculate("protection.scr_crowbar_trigger_latch", &inputs).unwrap();
    for (key, expected) in [("trigger_source_voltage", 9.75),
        ("available_gate_current", 0.234375), ("available_anode_current", 3.0),
        ("latch_margin", -1.0), ("holding_release_voltage", 2.0)] {
        assert_close(out[key], expected, REL, 0.0);
    }
}

#[test]
fn scr_rejects_trigger_latch_and_off_state_violations_then_recovers() {
    for (index, bad) in [(5, 9.999), (7, 8.0), (7, 9.0), (7, -0.1), (9, 4.01)] {
        let mut inputs = SCR;
        inputs[index].1 = bad;
        assert!(calculate("protection.scr_crowbar_trigger_latch", &inputs).is_err());
        assert_close(calculate("protection.scr_crowbar_trigger_latch", &SCR).unwrap()["available_gate_current"], 0.21875, REL, 0.0);
    }
    for index in [0, 1, 2, 3, 4, 5, 6, 8, 9] {
        let mut inputs = SCR;
        inputs[index].1 = 0.0;
        assert!(calculate("protection.scr_crowbar_trigger_latch", &inputs).is_err());
        assert_close(calculate("protection.scr_crowbar_trigger_latch", &SCR).unwrap()["trigger_source_voltage"], 10.0, REL, 0.0);
    }
}

#[test]
fn copper_doubled_resistivity_thermal_integral_complete_oracle() {
    // Final (234+T) is twice initial: integrated heat uses ln(2), not delta T.
    let k: f64 = 226.0 * std::f64::consts::LN_2.sqrt();
    let out = calculate("protection.copper_adiabatic_fault_limit", &COPPER).unwrap();
    for (key, expected) in [("material_k", k), ("allowable_fault_current", k),
        ("allowable_i2t", 4.0 * k * k), ("minimum_area_mm2", 600.0 / k),
        ("area_margin_mm2", 2.0 - 600.0 / k)] {
        assert_close(out[key], expected, REL, 0.0);
    }
    assert_close(4.0 * out["allowable_fault_current"].powi(2), out["allowable_i2t"], REL, 0.0);
    assert!(out["area_margin_mm2"] < 0.0);
}

#[test]
fn copper_time_scaling_zero_current_and_five_second_limit() {
    let k: f64 = 226.0 * std::f64::consts::LN_2.sqrt();
    for time in [1.0_f64, 4.0, 5.0] {
        let mut inputs = COPPER;
        inputs[3].1 = time;
        inputs[4].1 = 0.0;
        let out = calculate("protection.copper_adiabatic_fault_limit", &inputs).unwrap();
        for (key, expected) in [("material_k", k), ("allowable_fault_current", 2.0 * k / time.sqrt()),
            ("allowable_i2t", 4.0 * k * k), ("minimum_area_mm2", 0.0), ("area_margin_mm2", 2.0)] {
            assert_close(out[key], expected, REL, 0.0);
        }
    }
    let mut inputs = COPPER;
    inputs[2].1 = 250.0;
    let out = calculate("protection.copper_adiabatic_fault_limit", &inputs).unwrap();
    let expected_k: f64 = 226.0 * (484.0_f64 / 234.0).ln().sqrt();
    assert_close(out["material_k"], expected_k, REL, 0.0);
}

#[test]
fn copper_tiny_positive_temperature_headroom_does_not_cancel() {
    let mut inputs = COPPER;
    inputs[2].1 = 1e-20;
    inputs[3].1 = 1.0;
    inputs[4].1 = 1e-10;
    // ln(1+x)=x within relative 3e-23 here; adding x to 1 would lose all heat budget.
    let k: f64 = 226.0 * (1e-20_f64 / 234.0).sqrt();
    let out = calculate("protection.copper_adiabatic_fault_limit", &inputs).unwrap();
    for (key, expected) in [("material_k", k), ("allowable_fault_current", 2.0 * k),
        ("allowable_i2t", 4.0 * k * k), ("minimum_area_mm2", 1e-10 / k),
        ("area_margin_mm2", 2.0 - 1e-10 / k)] {
        assert_close(out[key], expected, REL, 0.0);
    }
}

#[test]
fn copper_rejects_unphysical_temperature_time_area_then_recovers() {
    for (index, bad) in [(0, 0.0), (0, -1.0), (1, -0.01), (1, 234.0),
        (1, 235.0), (2, 0.0), (2, 250.001), (3, 0.0), (3, -1.0),
        (3, 5.001), (4, -1.0)] {
        let mut inputs = COPPER;
        inputs[index].1 = bad;
        assert!(calculate("protection.copper_adiabatic_fault_limit", &inputs).is_err());
        let out = calculate("protection.copper_adiabatic_fault_limit", &COPPER).unwrap();
        assert_close(out["allowable_i2t"], 204304.0 * std::f64::consts::LN_2, REL, 0.0);
    }
}

#[test]
fn foldback_complete_intersection_and_interior_power_vertex() {
    // I=1/2+V/4; R=2 intersects at (2 V,1 A). P=5+2V-V^2/4=9-(V-4)^2/4.
    let out = calculate("protection.foldback_fault_envelope", &FOLDBACK).unwrap();
    for (key, expected) in [("foldback_slope", 0.25), ("knee_load_resistance", 3.0),
        ("fault_output_voltage", 2.0), ("fault_output_current", 1.0),
        ("fault_pass_power", 8.0), ("short_pass_power", 5.0),
        ("peak_stress_voltage", 4.0), ("peak_stress_current", 1.5), ("maximum_pass_power", 9.0)] {
        assert_close(out[key], expected, REL, 0.0);
    }
    assert_close(out["fault_output_voltage"], 2.0 * out["fault_output_current"], REL, 0.0);
    assert_close(0.5 + out["fault_output_voltage"] / 4.0, out["fault_output_current"], REL, 0.0);
    assert!(out["maximum_pass_power"] > out["short_pass_power"]);
    assert!(out["maximum_pass_power"] > (10.0 - 6.0) * 2.0);
}

#[test]
fn foldback_short_knee_and_all_three_peak_locations() {
    // Peak at zero for nearly flat foldback, at knee for high Vin, interior otherwise.
    for (vin, short, peak_v, peak_i, peak_p) in [
        (10.0, 0.5, 4.0, 1.5, 9.0), (16.0, 0.5, 6.0, 2.0, 20.0),
        (10.0, 1.5, 0.0, 1.5, 15.0),
    ] {
        for resistance in [0.0, 3.0] {
            let mut inputs = FOLDBACK;
            inputs[0].1 = vin;
            inputs[3].1 = short;
            inputs[4].1 = resistance;
            let out = calculate("protection.foldback_fault_envelope", &inputs).unwrap();
            let (v, i) = if resistance == 0.0 { (0.0, short) } else { (6.0, 2.0) };
            for (key, expected) in [("foldback_slope", (2.0 - short) / 6.0),
                ("knee_load_resistance", 3.0), ("fault_output_voltage", v),
                ("fault_output_current", i), ("fault_pass_power", (vin - v) * i),
                ("short_pass_power", vin * short), ("peak_stress_voltage", peak_v),
                ("peak_stress_current", peak_i), ("maximum_pass_power", peak_p)] {
                assert_close(out[key], expected, REL, 0.0);
            }
        }
    }
}

#[test]
fn foldback_tiny_short_current_still_reaches_knee() {
    let mut inputs = FOLDBACK;
    inputs[3].1 = 1e-20;
    inputs[4].1 = 3.0;
    let out = calculate("protection.foldback_fault_envelope", &inputs).unwrap();
    for (key, expected) in [("foldback_slope", 1.0 / 3.0), ("knee_load_resistance", 3.0),
        ("fault_output_voltage", 6.0), ("fault_output_current", 2.0),
        ("fault_pass_power", 8.0), ("short_pass_power", 1e-19),
        ("peak_stress_voltage", 5.0), ("peak_stress_current", 5.0 / 3.0),
        ("maximum_pass_power", 25.0 / 3.0)] {
        assert_close(out[key], expected, REL, 0.0);
    }
}

#[test]
fn foldback_rejects_nonfolding_or_outside_load_then_recovers() {
    for (index, bad) in [(0, 6.0), (0, 5.0), (1, 0.0), (2, 0.5),
        (2, 0.4), (3, 0.0), (3, -0.1), (4, -0.001), (4, 3.001)] {
        let mut inputs = FOLDBACK;
        inputs[index].1 = bad;
        assert!(calculate("protection.foldback_fault_envelope", &inputs).is_err());
        assert_close(calculate("protection.foldback_fault_envelope", &FOLDBACK).unwrap()["maximum_pass_power"], 9.0, REL, 0.0);
    }
}

#[test]
fn every_protection_input_rejects_nonfinite_values_and_recovers() {
    for (id, valid, key, expected) in [
        ("protection.fuse_pulse_coordination", FUSE.as_slice(), "pulse_i2t", 0.72),
        ("protection.tvs_surge_load_line", TVS.as_slice(), "peak_power", 432.0),
        ("protection.mov_power_law_pulse", MOV.as_slice(), "current_exponent", 4.0),
        ("protection.scr_crowbar_trigger_latch", SCR.as_slice(), "trigger_source_voltage", 10.0),
        ("protection.copper_adiabatic_fault_limit", COPPER.as_slice(), "allowable_i2t", 204304.0 * std::f64::consts::LN_2),
        ("protection.foldback_fault_envelope", FOLDBACK.as_slice(), "maximum_pass_power", 9.0),
    ] {
        for index in 0..valid.len() {
            for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                let mut inputs = valid.to_vec();
                inputs[index].1 = bad;
                assert!(calculate(id, &inputs).is_err(), "accepted {id} {}={bad}", inputs[index].0);
                assert_close(calculate(id, valid).unwrap()[key], expected, REL, 0.0);
            }
        }
    }
}

#[test]
fn fuse_subnormal_ramp_keeps_representable_integral() {
    let mut inputs = FUSE;
    inputs[0].1 = 1e162;
    inputs[1].1 = f64::from_bits(1);
    inputs[2].1 = 0.0;
    inputs[3].1 = 0.0;
    // Independent scaling integrates u^2 over [0,1] without dividing tiny time first.
    let integral: f64 = (f64::from_bits(1) * 1e162) * (1e162 / 3.0);
    let out = calculate("protection.fuse_pulse_coordination", &inputs).unwrap();
    assert_close(out["pulse_i2t"], integral, REL, 0.0);
    assert_close(out["pulse_margin"], 0.5 - integral, REL, 0.0);
    assert_close(out["pulse_utilization"], 2.0 * integral, REL, 0.0);
}

#[test]
fn tvs_large_series_resistances_preserve_small_avalanche_current() {
    let inputs = [("surge_voltage", 100.0), ("source_resistance", 1e308),
        ("breakdown_intercept", 30.0), ("dynamic_resistance", 1e308),
        ("fit_current_min", 1e-308), ("fit_current_max", 1e-306),
        ("rated_pulse_current", 1e-306), ("protected_voltage_limit", 100.0)];
    // Equal resistances each drop 35 V from the 70 V above breakdown.
    let out = calculate("protection.tvs_surge_load_line", &inputs)
        .expect("finite avalanche solution must not fail from Rs+rd overflow");
    for (key, expected) in [("surge_current", 3.5e-307), ("clamp_voltage", 65.0),
        ("peak_power", 2.275e-305), ("voltage_margin", 35.0), ("current_margin", 6.5e-307)] {
        assert_close(out[key], expected, REL, 0.0);
    }
}

#[test]
fn copper_large_fault_keeps_finite_required_area() {
    let mut inputs = COPPER;
    inputs[4].1 = 1e308;
    let k: f64 = 226.0 * std::f64::consts::LN_2.sqrt();
    let required: f64 = (1e308 / k) * 2.0;
    let out = calculate("protection.copper_adiabatic_fault_limit", &inputs)
        .expect("finite required area must not fail from fault_current*sqrt(time) overflow");
    assert_close(out["minimum_area_mm2"], required, REL, 0.0);
    assert_close(out["area_margin_mm2"], -required, REL, 0.0);
}

#[test]
fn foldback_knee_survives_underflowing_short_to_knee_ratio() {
    let inputs = [("input_voltage", 9.0), ("knee_voltage", 5.0),
        ("knee_current", 1e200), ("short_current", 1e-200), ("fault_resistance", 5e-200)];
    let out = calculate("protection.foldback_fault_envelope", &inputs)
        .expect("the knee remains (Vk,Ik) when Isc/Ik underflows");
    for (key, expected) in [("foldback_slope", 2e199), ("knee_load_resistance", 5e-200),
        ("fault_output_voltage", 5.0), ("fault_output_current", 1e200),
        ("fault_pass_power", 4e200), ("short_pass_power", 9e-200),
        ("peak_stress_voltage", 4.5), ("peak_stress_current", 9e199),
        ("maximum_pass_power", 4.05e200)] {
        assert_close(out[key], expected, REL, 0.0);
    }
}
