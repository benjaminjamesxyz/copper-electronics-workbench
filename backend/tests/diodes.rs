mod common;

use common::{assert_close, calculate};

// Every vector supplies all inputs: no expectation relies on catalog defaults.
// Worked oracles come from research/diodes.md (KVL, KCL, charge integration and
// waveform averaging), not from the evaluator. Nonzero quantities use relative
// tolerance only. Exact zero boundaries use absolute=0: these topologies and
// zero excitations entail exact zero arithmetic, not a measurement noise floor.
const LED: &[(&str, f64)] = &[
    ("supply", 5.0), ("supply_max", 5.5), ("forward_voltage", 2.0),
    ("forward_min", 1.8), ("led_count", 1.0), ("target_current", 0.020),
];
const SHOCKLEY: &[(&str, f64)] = &[
    ("diode_voltage", 0.6), ("saturation_current", 1e-12),
    ("ideality", 1.0), ("temperature", 300.0), ("model_current_max", 0.1),
];
const BRIDGE: &[(&str, f64)] = &[
    ("threshold_voltage", 0.7), ("slope_resistance", 0.08),
    ("path_current_avg", 1.0), ("path_current_rms", 1.5), ("path_current_peak", 3.0),
];
const RIPPLE: &[(&str, f64)] = &[
    ("source_rms", 12.0), ("line_frequency", 50.0), ("forward_drop", 0.8),
    ("load_current", 0.1), ("capacitance", 0.0022),
];
const ZENER: &[(&str, f64)] = &[
    ("input_min", 12.0), ("input_max", 15.0), ("zener_min", 4.9),
    ("zener_max", 5.1), ("load_min", 0.0), ("load_max", 0.010),
    ("zener_current_min", 0.005), ("zener_current_limit", 0.050),
    ("resistor_tolerance", 0.05), ("resistance", 390.0),
];
const CLIPPER: &[(&str, f64)] = &[
    ("input_voltage", 8.0), ("upper_reference", 3.3), ("lower_reference", -1.0),
    ("upper_drop", 0.65), ("lower_drop", 0.65), ("series_resistance", 1000.0),
];
const BALLAST: &[(&str, f64)] = &[
    ("total_current", 0.040), ("forward_one", 3.0), ("forward_two", 3.2),
    ("allowed_difference", 0.004), ("ballast_resistance", 68.0),
    ("supply", 5.0), ("sink_headroom_min", 0.3),
];
const RECOVERY: &[(&str, f64)] = &[
    ("reverse_peak_current", 4.0), ("triangle_duration", 1e-7),
    ("reverse_voltage", 400.0), ("commutation_frequency", 100000.0),
];

#[test]
fn led_complete_researched_example() {
    let out = calculate("diodes.led_series_resistor", LED).unwrap();
    for (key, expected) in [
        ("resistance", 150.0), ("resistor_power", 0.060), ("led_power", 0.040),
        ("current_max", 0.024666666666666667),
        ("resistor_power_max", 0.09126666666666668),
    ] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn shockley_complete_researched_example() {
    let out = calculate("diodes.shockley_forward_current", SHOCKLEY).unwrap();
    for (key, expected) in [
        ("thermal_voltage", 0.025851999786435535),
        ("diode_current", 0.01201036955312849),
        ("junction_power", 0.007206221731877094),
        ("incremental_resistance", 2.1524732998368954),
    ] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn bridge_complete_researched_example() {
    let out = calculate("diodes.bridge_conduction_loss", BRIDGE).unwrap();
    for (key, expected) in [
        ("bridge_power", 1.76), ("power_per_diode", 0.44), ("peak_path_drop", 1.88),
    ] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn ripple_complete_researched_example() {
    let out = calculate("diodes.full_wave_capacitor_ripple", RIPPLE).unwrap();
    for (key, expected) in [
        ("ripple_frequency", 100.0), ("capacitor_peak", 15.370562748477143),
        ("ripple_pp", 0.4545454545454546), ("capacitor_trough", 14.916017293931688),
        ("output_dc", 15.143290021204416), ("ripple_rms", 0.13121597027036952),
    ] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn zener_complete_researched_example() {
    let out = calculate("diodes.zener_shunt_resistor_window", ZENER).unwrap();
    for (key, expected) in [
        ("resistance_min", 212.63157894736844),
        ("resistance_max", 438.0952380952381),
        ("zener_current_low", 0.006849816849816852),
        ("zener_current_high", 0.027260458839406206),
        ("zener_power_bound", 0.13902834008097165),
        ("resistor_power_bound", 0.2753306342780027),
    ] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn clipper_complete_researched_example() {
    let out = calculate("diodes.biased_two_sided_clipper", CLIPPER).unwrap();
    for (key, expected) in [
        ("upper_level", 3.95), ("lower_level", -1.65), ("output_voltage", 3.95),
        ("upper_current", 0.00405), ("lower_current", 0.0), ("resistor_power", 0.0164025),
    ] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn ballast_complete_researched_example() {
    let out = calculate("diodes.parallel_led_ballast", BALLAST).unwrap();
    for (key, expected) in [
        ("ballast_min", 50.0), ("current_one", 0.021470588235294116),
        ("current_two", 0.018529411764705885),
        ("current_difference", 0.002941176470588232), ("branch_voltage", 4.46),
        ("ballast_power_total", 0.054694117647058824), ("sink_headroom", 0.54),
    ] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn recovery_complete_researched_example() {
    let out = calculate("diodes.reverse_recovery_charge_budget", RECOVERY).unwrap();
    for (key, expected) in [
        ("recovery_charge", 2e-7), ("charge_energy_budget", 8e-5),
        ("charge_power_budget", 8.0), ("reverse_charge_current_avg", 0.020),
    ] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn led_series_count_and_equal_tolerance_corners() {
    // Three 3 V junctions leave 3 V across the resistor. Equal supply/Vf
    // corners must give precisely the nominal current and heating.
    let out = calculate("diodes.led_series_resistor", &[
        ("supply", 12.0), ("supply_max", 12.0), ("forward_voltage", 3.0),
        ("forward_min", 3.0), ("led_count", 3.0), ("target_current", 0.020),
    ]).unwrap();
    for (key, expected) in [
        ("resistance", 150.0), ("resistor_power", 0.060), ("led_power", 0.180),
        ("current_max", 0.020), ("resistor_power_max", 0.060),
    ] {
        assert_close(out[key], expected, 1e-12, 0.0);
    }
    assert_close(out["resistor_power"] + out["led_power"], 12.0 * 0.020, 1e-12, 0.0);

    // Inclusive maximum integer count remains a physical series string.
    let out = calculate("diodes.led_series_resistor", &[
        ("supply", 1000.0), ("supply_max", 1000.0), ("forward_voltage", 0.5),
        ("forward_min", 0.5), ("led_count", 1000.0), ("target_current", 1.0),
    ]).unwrap();
    for (key, expected) in [
        ("resistance", 500.0), ("resistor_power", 500.0), ("led_power", 500.0),
        ("current_max", 1.0), ("resistor_power_max", 500.0),
    ] {
        assert_close(out[key], expected, 1e-12, 0.0);
    }
}

#[test]
fn led_rejects_fractional_count_headroom_and_reversed_corners() {
    for (key, value) in [
        ("led_count", 1.5), ("supply", 2.0), ("supply", 1.9),
        ("supply_max", 4.9), ("forward_min", 2.1),
    ] {
        let mut inputs = LED.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = value;
        assert!(calculate("diodes.led_series_resistor", &inputs).is_err(), "{key}={value}");
    }
}

#[test]
fn shockley_ln_two_identity_and_zero_bias_temperature_scaling() {
    // Set VD/(n VT)=ln(2): ID=IS and rd=n VT/(2 IS), an analytic
    // exponential identity rather than a duplicate catalog-generated oracle.
    let voltage = 2.0 * 0.025851999786435535 * std::f64::consts::LN_2;
    let out = calculate("diodes.shockley_forward_current", &[
        ("diode_voltage", voltage), ("saturation_current", 1e-6),
        ("ideality", 2.0), ("temperature", 300.0), ("model_current_max", 2e-6),
    ]).unwrap();
    for (key, expected) in [
        ("thermal_voltage", 0.025851999786435535), ("diode_current", 1e-6),
        ("junction_power", voltage * 1e-6), ("incremental_resistance", 25851.999786435535),
    ] {
        assert_close(out[key], expected, 1e-12, 0.0);
    }

    // Zero bias at the upper temperature/ideality endpoints: zero power/current
    // but finite resistance. IS is entered explicitly, not temperature corrected.
    let out = calculate("diodes.shockley_forward_current", &[
        ("diode_voltage", 0.0), ("saturation_current", 1e-9),
        ("ideality", 2.0), ("temperature", 500.0), ("model_current_max", 1e-12),
    ]).unwrap();
    for (key, expected) in [
        ("thermal_voltage", 0.04308666631072589), ("diode_current", 0.0),
        ("junction_power", 0.0), ("incremental_resistance", 86173332.62145178),
    ] {
        assert_close(out[key], expected, 1e-12, 0.0);
    }
}

#[test]
fn shockley_retains_tiny_positive_forward_current() {
    // x≈3.87e-19. The omitted x^2/2 term is <2e-19 relative, so the
    // independently derived first-order Shockley limit is ample at 1e-12.
    let out = calculate("diodes.shockley_forward_current", &[
        ("diode_voltage", 1e-20), ("saturation_current", 1e-12),
        ("ideality", 1.0), ("temperature", 300.0), ("model_current_max", 0.1),
    ]).unwrap();
    for (key, expected) in [
        ("thermal_voltage", 0.025851999786435535),
        ("diode_current", 3.86817270718336e-31),
        ("junction_power", 3.86817270718336e-51),
        ("incremental_resistance", 25851999786.435535),
    ] {
        assert_close(out[key], expected, 1e-12, 0.0);
    }
}

#[test]
fn shockley_exponent_endpoint_and_independent_model_guards() {
    let out = calculate("diodes.shockley_forward_current", &[
        ("diode_voltage", 40.0 * 0.025851999786435535),
        ("saturation_current", 1e-20), ("ideality", 1.0),
        ("temperature", 300.0), ("model_current_max", 0.1),
    ]).unwrap();
    assert_close(out["diode_current"], 0.0023538526683702, 1e-12, 0.0);
    for (key, value) in [("diode_voltage", 1.1), ("model_current_max", 0.001)] {
        let mut inputs = SHOCKLEY.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = value;
        assert!(calculate("diodes.shockley_forward_current", &inputs).is_err(), "{key}={value}");
    }
}

#[test]
fn bridge_dc_square_pulse_zero_current_and_ideal_junction_boundaries() {
    // DC: iavg=irms=ipeak. Square pulses of 4 A at 25% duty:
    // iavg=1 A, irms=2 A, equality in irms^2=ipeak*iavg.
    for (avg, rms, peak, power, per_diode, drop) in [
        (2.0, 2.0, 2.0, 4.0, 1.0, 2.0),
        (1.0, 2.0, 4.0, 3.0, 0.75, 3.0),
        (0.0, 0.0, 0.0, 0.0, 0.0, 1.0),
    ] {
        let out = calculate("diodes.bridge_conduction_loss", &[
            ("threshold_voltage", 0.5), ("slope_resistance", 0.25),
            ("path_current_avg", avg), ("path_current_rms", rms), ("path_current_peak", peak),
        ]).unwrap();
        for (key, expected) in [
            ("bridge_power", power), ("power_per_diode", per_diode), ("peak_path_drop", drop),
        ] {
            assert_close(out[key], expected, 1e-12, 0.0);
        }
        assert_close(4.0 * out["power_per_diode"], power, 1e-12, 0.0);
    }
    let out = calculate("diodes.bridge_conduction_loss", &[
        ("threshold_voltage", 0.0), ("slope_resistance", 0.0),
        ("path_current_avg", 1.0), ("path_current_rms", 1.5), ("path_current_peak", 3.0),
    ]).unwrap();
    for key in ["bridge_power", "power_per_diode", "peak_path_drop"] {
        assert_close(out[key], 0.0, 0.0, 0.0);
    }
}

#[test]
fn bridge_rejects_each_impossible_current_statistic() {
    for (avg, rms, peak) in [
        (2.0, 1.0, 3.0), // RMS below average
        (1.0, 1.5, 1.4), // peak below RMS
        (1.0, 2.0, 3.0), // ordering holds, but second moment impossible
        (0.0, 1.0, 2.0), // no nonnegative waveform has positive RMS and zero mean
    ] {
        assert!(calculate("diodes.bridge_conduction_loss", &[
            ("threshold_voltage", 0.7), ("slope_resistance", 0.08),
            ("path_current_avg", avg), ("path_current_rms", rms), ("path_current_peak", peak),
        ]).is_err());
    }
}

#[test]
fn ripple_unloaded_and_ten_percent_limit() {
    let out = calculate("diodes.full_wave_capacitor_ripple", &[
        ("source_rms", 12.0), ("line_frequency", 60.0), ("forward_drop", 0.8),
        ("load_current", 0.0), ("capacitance", 0.0022),
    ]).unwrap();
    for (key, expected) in [
        ("ripple_frequency", 120.0), ("capacitor_peak", 15.370562748477143),
        ("ripple_pp", 0.0), ("capacitor_trough", 15.370562748477143),
        ("output_dc", 15.370562748477143), ("ripple_rms", 0.0),
    ] {
        assert_close(out[key], expected, 1e-12, 0.0);
    }
    // A 20 V peak, 100 recharges/s and 10 mF lose 2 V at 2 A.
    // This is the inclusive 10% small-ripple boundary; ideal bridge drop=0.
    let out = calculate("diodes.full_wave_capacitor_ripple", &[
        ("source_rms", 14.142135623730951), ("line_frequency", 50.0),
        ("forward_drop", 0.0), ("load_current", 2.0), ("capacitance", 0.010),
    ]).unwrap();
    for (key, expected) in [
        ("ripple_frequency", 100.0), ("capacitor_peak", 20.0), ("ripple_pp", 2.0),
        ("capacitor_trough", 18.0), ("output_dc", 19.0), ("ripple_rms", 0.5773502691896258),
    ] {
        assert_close(out[key], expected, 1e-12, 0.0);
    }
}

#[test]
fn ripple_rejects_unavailable_peak_and_excessive_discharge() {
    for (source, drop, load) in [(1.0, 1.0, 0.0), (12.0, 0.8, 1.0)] {
        assert!(calculate("diodes.full_wave_capacitor_ripple", &[
            ("source_rms", source), ("line_frequency", 50.0), ("forward_drop", drop),
            ("load_current", load), ("capacitance", 0.0022),
        ]).is_err());
    }
}

#[test]
fn zener_zero_tolerance_inclusive_resistor_endpoints() {
    // Fixed 15->5 V regulation, 10 mA load, IZ=10..40 mA:
    // window 10/0.05=200 through 10/0.02=500 ohm.
    for (resistance, current, zener_power, resistor_power) in [
        (200.0, 0.040, 0.200, 0.500),
        (250.0, 0.030, 0.150, 0.400),
        (500.0, 0.010, 0.050, 0.200),
    ] {
        let out = calculate("diodes.zener_shunt_resistor_window", &[
            ("input_min", 15.0), ("input_max", 15.0), ("zener_min", 5.0),
            ("zener_max", 5.0), ("load_min", 0.010), ("load_max", 0.010),
            ("zener_current_min", 0.010), ("zener_current_limit", 0.040),
            ("resistor_tolerance", 0.0), ("resistance", resistance),
        ]).unwrap();
        for (key, expected) in [
            ("resistance_min", 200.0), ("resistance_max", 500.0),
            ("zener_current_low", current), ("zener_current_high", current),
            ("zener_power_bound", zener_power), ("resistor_power_bound", resistor_power),
        ] {
            assert_close(out[key], expected, 1e-12, 0.0);
        }
    }
}

#[test]
fn zener_singleton_window_and_maximum_resistor_tolerance() {
    // Equality of all corners and IZ bounds gives a singleton window.
    let out = calculate("diodes.zener_shunt_resistor_window", &[
        ("input_min", 15.0), ("input_max", 15.0), ("zener_min", 5.0),
        ("zener_max", 5.0), ("load_min", 0.0), ("load_max", 0.0),
        ("zener_current_min", 0.020), ("zener_current_limit", 0.020),
        ("resistor_tolerance", 0.0), ("resistance", 500.0),
    ]).unwrap();
    for (key, expected) in [
        ("resistance_min", 500.0), ("resistance_max", 500.0),
        ("zener_current_low", 0.020), ("zener_current_high", 0.020),
        ("zener_power_bound", 0.100), ("resistor_power_bound", 0.200),
    ] {
        assert_close(out[key], expected, 1e-12, 0.0);
    }
    // 400 ohm ±50% gives 200..600 actual ohm and 10/600..10/200 A.
    let out = calculate("diodes.zener_shunt_resistor_window", &[
        ("input_min", 15.0), ("input_max", 15.0), ("zener_min", 5.0),
        ("zener_max", 5.0), ("load_min", 0.0), ("load_max", 0.0),
        ("zener_current_min", 0.010), ("zener_current_limit", 0.100),
        ("resistor_tolerance", 0.5), ("resistance", 400.0),
    ]).unwrap();
    for (key, expected) in [
        ("resistance_min", 200.0), ("resistance_max", 666.6666666666666),
        ("zener_current_low", 0.016666666666666666), ("zener_current_high", 0.050),
        ("zener_power_bound", 0.250), ("resistor_power_bound", 0.500),
    ] {
        assert_close(out[key], expected, 1e-12, 0.0);
    }
}

#[test]
fn zener_rejects_all_reversed_infeasible_and_unregulated_corners() {
    for (key, value) in [
        ("input_max", 11.0), ("zener_max", 4.8), ("input_min", 5.1),
        ("load_min", 0.020), ("zener_current_limit", 0.004),
        ("load_max", 0.050), // empty window despite ordered corners
        ("resistance", 200.0), // excessive IZ
        ("resistance", 500.0), // lost regulation
    ] {
        let mut inputs = ZENER.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = value;
        assert!(calculate("diodes.zener_shunt_resistor_window", &inputs).is_err(), "{key}={value}");
    }
}

#[test]
fn clipper_lower_interior_and_exact_transition_samples() {
    // Deliberately unequal diode drops prove the two orientations are distinct.
    for (input, voltage, upper_current, lower_current, power) in [
        (-5.0, -2.25, 0.0, 0.0275, 0.075625),
        (-1.0, -1.0, 0.0, 0.0, 0.0),
        (-2.25, -2.25, 0.0, 0.0, 0.0),
        (2.5, 2.5, 0.0, 0.0, 0.0),
        (5.0, 2.5, 0.025, 0.0, 0.0625),
    ] {
        let out = calculate("diodes.biased_two_sided_clipper", &[
            ("input_voltage", input), ("upper_reference", 2.0), ("lower_reference", -2.0),
            ("upper_drop", 0.5), ("lower_drop", 0.25), ("series_resistance", 100.0),
        ]).unwrap();
        for (key, expected) in [
            ("upper_level", 2.5), ("lower_level", -2.25), ("output_voltage", voltage),
            ("upper_current", upper_current), ("lower_current", lower_current),
            ("resistor_power", power),
        ] {
            assert_close(out[key], expected, 1e-12, 0.0);
        }
    }
}

#[test]
fn clipper_equal_negative_references_ideal_diodes_and_invalid_order() {
    let out = calculate("diodes.biased_two_sided_clipper", &[
        ("input_voltage", -3.0), ("upper_reference", -2.0), ("lower_reference", -2.0),
        ("upper_drop", 0.0), ("lower_drop", 0.0), ("series_resistance", 2.0),
    ]).unwrap();
    for (key, expected) in [
        ("upper_level", -2.0), ("lower_level", -2.0), ("output_voltage", -2.0),
        ("upper_current", 0.0), ("lower_current", 0.5), ("resistor_power", 0.5),
    ] {
        assert_close(out[key], expected, 1e-12, 0.0);
    }
    assert!(calculate("diodes.biased_two_sided_clipper", &[
        ("input_voltage", 0.0), ("upper_reference", -1.0), ("lower_reference", 1.0),
        ("upper_drop", 2.0), ("lower_drop", 2.0), ("series_resistance", 1000.0),
    ]).is_err()); // even ordered resultant levels do not waive reference order
}

#[test]
fn ballast_equal_forward_voltages_and_exact_compliance() {
    let out = calculate("diodes.parallel_led_ballast", &[
        ("total_current", 0.040), ("forward_one", 3.0), ("forward_two", 3.0),
        ("allowed_difference", 0.004), ("ballast_resistance", 50.0),
        ("supply", 4.3), ("sink_headroom_min", 0.3),
    ]).unwrap();
    for (key, expected) in [
        ("ballast_min", 0.0), ("current_one", 0.020), ("current_two", 0.020),
        ("current_difference", 0.0), ("branch_voltage", 4.0),
        ("ballast_power_total", 0.040), ("sink_headroom", 0.3),
    ] {
        assert_close(out[key], expected, 1e-12, 0.0);
    }
}

#[test]
fn ballast_matching_endpoint_led_exchange_and_kirchhoff_identities() {
    for (forward_one, forward_two, current_one, current_two) in [
        (2.0, 3.0, 0.625, 0.375), (3.0, 2.0, 0.375, 0.625),
    ] {
        let out = calculate("diodes.parallel_led_ballast", &[
            ("total_current", 1.0), ("forward_one", forward_one), ("forward_two", forward_two),
            ("allowed_difference", 0.25), ("ballast_resistance", 4.0),
            ("supply", 5.5), ("sink_headroom_min", 1.0),
        ]).unwrap();
        for (key, expected) in [
            ("ballast_min", 4.0), ("current_one", current_one), ("current_two", current_two),
            ("current_difference", 0.25), ("branch_voltage", 4.5),
            ("ballast_power_total", 2.125), ("sink_headroom", 1.0),
        ] {
            assert_close(out[key], expected, 1e-12, 0.0);
        }
        assert_close(out["current_one"] + out["current_two"], 1.0, 1e-12, 0.0);
        assert_close(forward_one + 4.0 * out["current_one"], 4.5, 1e-12, 0.0);
        assert_close(forward_two + 4.0 * out["current_two"], 4.5, 1e-12, 0.0);
    }
}

#[test]
fn ballast_near_extinction_retains_each_positive_branch_under_led_exchange() {
    // The exact representable total is 0.5+2^-53 A. KCL and KVL require
    // the small branch to be 2^-54 A, not the 2^-53 A that can result from
    // subtracting an already-rounded large branch. No absolute tolerance
    // may erase this positive current or hide a factor-of-two error.
    let small = 5.551115123125783e-17;
    for (forward_one, forward_two, current_one, current_two) in [
        (0.5, 1.0, 0.5, small), (1.0, 0.5, small, 0.5),
    ] {
        let out = calculate("diodes.parallel_led_ballast", &[
            ("total_current", 0.5000000000000001),
            ("forward_one", forward_one), ("forward_two", forward_two),
            ("allowed_difference", 0.5), ("ballast_resistance", 1.0),
            ("supply", 5.0), ("sink_headroom_min", 0.3),
        ]).unwrap();
        for (key, expected) in [
            ("ballast_min", 1.0), ("current_one", current_one), ("current_two", current_two),
            ("current_difference", 0.5), ("branch_voltage", 1.0),
            ("ballast_power_total", 0.25), ("sink_headroom", 4.0),
        ] {
            assert_close(out[key], expected, 1e-12, 0.0);
        }
    }
}

#[test]
fn ballast_rejects_unachievable_matching_nonconducting_branch_and_compliance() {
    for (key, value) in [
        ("allowed_difference", 0.040), ("allowed_difference", 0.050),
        ("ballast_resistance", 49.0), ("supply", 4.7),
        // At R=5, mismatch/R=total: one branch is extinguished. The matching
        // guard also rejects it; positivity is implied by the two stricter
        // inequalities allowed_difference<total and R>=Rmin in exact arithmetic.
        ("ballast_resistance", 5.0),
    ] {
        let mut inputs = BALLAST.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = value;
        assert!(calculate("diodes.parallel_led_ballast", &inputs).is_err(), "{key}={value}");
    }
}

#[test]
fn recovery_triangle_area_and_inclusive_occupancy_endpoint() {
    // 2 A triangle over 1 us encloses 1 uC. At 100 V, each event
    // transports a 100 uJ cell budget; 100 kHz gives 10 W and 0.1 A.
    let out = calculate("diodes.reverse_recovery_charge_budget", &[
        ("reverse_peak_current", 2.0), ("triangle_duration", 1e-6),
        ("reverse_voltage", 100.0), ("commutation_frequency", 100000.0),
    ]).unwrap();
    for (key, expected) in [
        ("recovery_charge", 1e-6), ("charge_energy_budget", 1e-4),
        ("charge_power_budget", 10.0), ("reverse_charge_current_avg", 0.1),
    ] {
        assert_close(out[key], expected, 1e-12, 0.0);
    }
    assert_close(out["charge_power_budget"], 100.0 * out["reverse_charge_current_avg"], 1e-12, 0.0);
}

#[test]
fn recovery_zero_charge_zero_voltage_and_overlapping_events() {
    let out = calculate("diodes.reverse_recovery_charge_budget", &[
        ("reverse_peak_current", 0.0), ("triangle_duration", 1e-7),
        ("reverse_voltage", 400.0), ("commutation_frequency", 100000.0),
    ]).unwrap();
    for key in ["recovery_charge", "charge_energy_budget", "charge_power_budget", "reverse_charge_current_avg"] {
        assert_close(out[key], 0.0, 0.0, 0.0);
    }
    let out = calculate("diodes.reverse_recovery_charge_budget", &[
        ("reverse_peak_current", 4.0), ("triangle_duration", 1e-7),
        ("reverse_voltage", 0.0), ("commutation_frequency", 100000.0),
    ]).unwrap();
    for (key, expected) in [
        ("recovery_charge", 2e-7), ("charge_energy_budget", 0.0),
        ("charge_power_budget", 0.0), ("reverse_charge_current_avg", 0.020),
    ] {
        assert_close(out[key], expected, 1e-12, 0.0);
    }
    assert!(calculate("diodes.reverse_recovery_charge_budget", &[
        ("reverse_peak_current", 4.0), ("triangle_duration", 1e-7),
        ("reverse_voltage", 400.0), ("commutation_frequency", 2000000.0),
    ]).is_err());
}

#[test]
fn every_declared_input_rejects_outside_bounds_and_nonfinite_values() {
    // Each pair lies outside its documented inclusive/exclusive interval.
    // One-key mutation keeps all other physical inputs explicit and valid.
    let cases: &[(&str, &[(&str, f64)], &[(&str, f64, f64)])] = &[
        ("diodes.led_series_resistor", LED, &[
            ("supply", 0.0, 1001.0), ("supply_max", 0.0, 1001.0),
            ("forward_voltage", 0.0, 101.0), ("forward_min", 0.0, 101.0),
            ("led_count", 0.0, 1001.0), ("target_current", 0.0, 101.0),
        ]),
        ("diodes.shockley_forward_current", SHOCKLEY, &[
            ("diode_voltage", -1.0, 5.1), ("saturation_current", 0.0, 0.002),
            ("ideality", 0.9, 2.1), ("temperature", 199.0, 501.0),
            ("model_current_max", 0.0, 1001.0),
        ]),
        ("diodes.bridge_conduction_loss", BRIDGE, &[
            ("threshold_voltage", -1.0, 11.0), ("slope_resistance", -1.0, 1001.0),
            ("path_current_avg", -1.0, 10001.0), ("path_current_rms", -1.0, 10001.0),
            ("path_current_peak", -1.0, 10001.0),
        ]),
        ("diodes.full_wave_capacitor_ripple", RIPPLE, &[
            ("source_rms", 0.0, 10001.0), ("line_frequency", 0.0, 100001.0),
            ("forward_drop", -1.0, 11.0), ("load_current", -1.0, 10001.0),
            ("capacitance", 0.0, 101.0),
        ]),
        ("diodes.zener_shunt_resistor_window", ZENER, &[
            ("input_min", 0.0, 10001.0), ("input_max", 0.0, 10001.0),
            ("zener_min", 0.0, 1001.0), ("zener_max", 0.0, 1001.0),
            ("load_min", -1.0, 1001.0), ("load_max", -1.0, 1001.0),
            ("zener_current_min", 0.0, 1001.0), ("zener_current_limit", 0.0, 1001.0),
            ("resistor_tolerance", -0.01, 0.51), ("resistance", 0.0, 1.1e12),
        ]),
        ("diodes.biased_two_sided_clipper", CLIPPER, &[
            ("input_voltage", -10001.0, 10001.0),
            ("upper_reference", -10001.0, 10001.0), ("lower_reference", -10001.0, 10001.0),
            ("upper_drop", -1.0, 11.0), ("lower_drop", -1.0, 11.0),
            ("series_resistance", 0.0, 1.1e12),
        ]),
        ("diodes.parallel_led_ballast", BALLAST, &[
            ("total_current", 0.0, 101.0), ("forward_one", 0.0, 101.0),
            ("forward_two", 0.0, 101.0), ("allowed_difference", 0.0, 101.0),
            ("ballast_resistance", 0.0, 1.1e12), ("supply", 0.0, 10001.0),
            ("sink_headroom_min", -1.0, 1001.0),
        ]),
        ("diodes.reverse_recovery_charge_budget", RECOVERY, &[
            ("reverse_peak_current", -1.0, 10001.0), ("triangle_duration", 0.0, 1.1),
            ("reverse_voltage", -1.0, 10001.0), ("commutation_frequency", 0.0, 1.1e9),
        ]),
    ];
    for &(id, nominal, bounds) in cases {
        for &(key, below, above) in bounds {
            for value in [below, above, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                let mut inputs = nominal.to_vec();
                inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = value;
                assert!(calculate(id, &inputs).is_err(), "{id}: {key}={value}");
            }
        }
        // Rejections may not poison the reused thread-local engine.
        assert!(calculate(id, nominal).is_ok(), "recovery after invalid {id} inputs");
    }
}
