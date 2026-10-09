mod common;

use common::{assert_close, calculate};
use std::f64::consts::{LN_10, PI};

// Independent non-default SI vectors. Expected maps come from node KCL,
// normalized transfer polynomials, exact SI constants, and logarithm identities.
const MIRROR: &[(&str, f64)] = &[
    ("reference_current", 0.00128), ("beta", 62.0),
    ("base_emitter_voltage", 0.7), ("collector_voltage", 1.9),
];
const MIRROR_EXPECTED: &[(&str, f64)] = &[
    ("collector_current", 0.00124), ("base_current_each", 0.00002),
    ("current_deficit", 0.00004), ("fractional_deficit", 0.03125),
    ("compliance_margin", 1.2),
];
const INSTRUMENT: &[(&str, f64)] = &[
    ("feedback_resistance", 12000.0), ("gain_resistance", 3000.0),
    ("difference_input_resistance", 20000.0), ("difference_feedback_resistance", 10000.0),
    ("common_voltage", -0.5), ("differential_voltage", -0.125),
    ("reference_voltage", 0.25), ("output_min", -3.0), ("output_max", 3.0),
    ("common_min", -2.0), ("common_max", 2.0),
];
const INSTRUMENT_EXPECTED: &[(&str, f64)] = &[
    ("first_stage_gain", 9.0), ("total_gain", 4.5),
    ("first_low_voltage", 0.0625), ("first_high_voltage", -1.0625),
    ("difference_pin_voltage", -0.1875), ("output_voltage", -0.3125),
];
// wt*Rf*Cs = 5; the normalized Butterworth solution has wt*Rf*Cf = 3.
const TIA: &[(&str, f64)] = &[
    ("feedback_resistance", 20000.0), ("source_capacitance", 3.978873577297384e-11),
    ("gain_bandwidth", 1000000.0), ("input_peak", 0.000025),
    ("output_peak_limit", 2.0),
];
const TIA_EXPECTED: &[(&str, f64)] = &[
    ("feedback_capacitance", 2.3873241463784302e-11),
    ("butterworth_bandwidth", 353553.39059327374),
    ("dc_transimpedance", -20000.0), ("output_peak", 0.5),
    ("noise_gain_zero", 125000.0),
];
const HOWLAND: &[(&str, f64)] = &[
    ("r_one", 2000.0), ("r_two", 4000.0), ("r_three", 1000.0), ("r_four", 1800.0),
    ("positive_drive", 0.6), ("negative_drive", -0.2), ("load_resistance", 1000.0),
    ("output_min", -4.0), ("output_max", 4.0), ("common_min", -2.0),
    ("common_max", 2.0), ("output_current_limit", 0.01),
];
const HOWLAND_EXPECTED: &[(&str, f64)] = &[
    ("norton_drive_current", 0.00039), ("output_conductance", 0.00005),
    ("load_voltage", 13.0 / 35.0), ("load_current", 13.0 / 35000.0),
    ("amplifier_voltage", 1.4), ("amplifier_current", 29.0 / 35000.0),
];
const BROKAW: &[(&str, f64)] = &[
    ("temperature", 320.0), ("area_ratio", 4.0), ("ptat_resistance", 8000.0),
    ("base_emitter_voltage", 0.61), ("base_emitter_slope", -0.0018),
];
// 70-digit Decimal evaluation using exact k and q, not catalog examples.
const BROKAW_EXPECTED: &[(&str, f64)] = &[
    ("thermal_voltage", 0.027575466438864566), ("delta_vbe", 0.03822771362944874),
    ("branch_current", 4.778464203681092e-6), ("resistor_ratio", 7.533801335639886),
    ("common_resistor", 60270.41068511909), ("reference_voltage", 1.186),
];
const LOG: &[(&str, f64)] = &[("signal_current", 2e-7), ("reference_resistance", 1250000.0)];
const LOG_EXPECTED: &[(&str, f64)] = &[
    ("reference_current", 2e-6), ("current_ratio", 0.1), ("log_decades", -1.0),
    ("log_output", -0.375), ("local_transimpedance", 814302.1535685972),
];

#[test]
fn mirror_nondefault_all_outputs_and_two_base_kcl() {
    let r = calculate("precision.bjt_mirror_base_current", MIRROR).unwrap();
    assert_eq!(r.len(), MIRROR_EXPECTED.len());
    for &(key, expected) in MIRROR_EXPECTED {
        assert_close(r[key], expected, 2e-12, 0.0);
    }
    // Reference supplies 64 base-current units: 62 collector units and two bases.
    assert_close(r["collector_current"] + 2.0 * r["base_current_each"], 0.00128, 2e-12, 0.0);
    assert_close(r["collector_current"] / r["base_current_each"], 62.0, 2e-12, 0.0);
}

#[test]
fn mirror_scaling_tiny_currents_and_exact_compliance_boundary() {
    for scale in [0.5, 2.0, 1e-200] {
        let mut inputs = MIRROR.to_vec();
        inputs[0].1 *= scale;
        inputs[3].1 = 0.7;
        let r = calculate("precision.bjt_mirror_base_current", &inputs).unwrap();
        for (key, value) in [("collector_current", 0.00124), ("base_current_each", 0.00002), ("current_deficit", 0.00004)] {
            assert_close(r[key], value * scale, 2e-12, 0.0);
        }
        assert_close(r["fractional_deficit"], 0.03125, 2e-12, 0.0);
        assert_eq!(r["compliance_margin"], 0.0);
    }
    let mut inputs = MIRROR.to_vec();
    inputs[3].1 = f64::from_bits(0.7_f64.to_bits() - 1);
    assert!(calculate("precision.bjt_mirror_base_current", &inputs).is_err());
    inputs[3].1 = 0.7;
    let r = calculate("precision.bjt_mirror_base_current", &inputs).unwrap();
    assert_close(r["collector_current"], 0.00124, 2e-12, 0.0);
    assert_eq!(r["compliance_margin"], 0.0);
}

#[test]
fn mirror_large_beta_preserves_small_deficit() {
    let mut inputs = MIRROR.to_vec();
    inputs[0].1 = 0.001002;
    inputs[1].1 = 1000.0;
    let r = calculate("precision.bjt_mirror_base_current", &inputs).unwrap();
    for (key, expected) in [
        ("collector_current", 0.001), ("base_current_each", 1e-6),
        ("current_deficit", 2e-6), ("fractional_deficit", 1.0 / 501.0),
        ("compliance_margin", 1.2),
    ] {
        assert_close(r[key], expected, 2e-12, 0.0);
    }
}

#[test]
fn instrumentation_negative_differential_all_nodes_and_kcl() {
    let r = calculate("precision.instrumentation_internal_headroom", INSTRUMENT).unwrap();
    assert_eq!(r.len(), INSTRUMENT_EXPECTED.len());
    for &(key, expected) in INSTRUMENT_EXPECTED {
        assert_close(r[key], expected, 2e-12, 0.0);
    }
    assert!(r["first_low_voltage"] > r["first_high_voltage"]);
    let pin = r["difference_pin_voltage"];
    assert_close((r["first_low_voltage"] - pin) / 20000.0,
        (pin - r["output_voltage"]) / 10000.0, 2e-12, 0.0);
    assert_close((r["first_high_voltage"] - pin) / 20000.0,
        (pin - 0.25) / 10000.0, 2e-12, 0.0);
}

#[test]
fn instrumentation_zero_and_reversed_differential_preserve_reference() {
    for (vd, va, vb, pin, output) in [
        (0.0, -0.5, -0.5, 0.0, 0.25),
        (0.125, -1.0625, 0.0625, 0.1875, 0.8125),
    ] {
        let mut inputs = INSTRUMENT.to_vec();
        inputs[5].1 = vd;
        let r = calculate("precision.instrumentation_internal_headroom", &inputs).unwrap();
        for (key, expected) in [
            ("first_stage_gain", 9.0), ("total_gain", 4.5),
            ("first_low_voltage", va), ("first_high_voltage", vb),
            ("difference_pin_voltage", pin), ("output_voltage", output),
        ] {
            assert_close(r[key], expected, 2e-12, 0.0);
        }
    }
}

#[test]
fn instrumentation_output_and_final_pin_endpoints_are_inclusive() {
    for sign in [-1.0, 1.0] {
        let mut inputs = INSTRUMENT.to_vec();
        inputs[4].1 = 0.0;
        inputs[5].1 = 0.0;
        inputs[6].1 = sign * 3.0;
        let r = calculate("precision.instrumentation_internal_headroom", &inputs).unwrap();
        assert_eq!(r["first_low_voltage"], 0.0);
        assert_eq!(r["first_high_voltage"], 0.0);
        assert_eq!(r["difference_pin_voltage"], sign * 2.0);
        assert_eq!(r["output_voltage"], sign * 3.0);
        inputs[6].1 = sign * f64::from_bits(3.0_f64.to_bits() + 1);
        assert!(calculate("precision.instrumentation_internal_headroom", &inputs).is_err());
    }
}

#[test]
fn instrumentation_rejects_both_hidden_first_stage_clips() {
    for sign in [-1.0, 1.0] {
        for vd in [-0.125, 0.125] {
            let mut inputs = INSTRUMENT.to_vec();
            inputs[4].1 = sign * 2.75;
            inputs[5].1 = vd;
            inputs[9].1 = -3.0;
            inputs[10].1 = 3.0;
            // Both signal pins fit +/-3 V and final output is -0.3125 or 0.8125 V.
            // One first-stage branch reaches +/-3.3125 V and must invalidate the result.
            assert!(calculate("precision.instrumentation_internal_headroom", &inputs).is_err());
            inputs[4].1 = sign * 2.0;
            let r = calculate("precision.instrumentation_internal_headroom", &inputs).unwrap();
            assert_close(r["output_voltage"], 0.25 + 4.5 * vd, 2e-12, 0.0);
        }
    }
}

#[test]
fn instrumentation_checks_signal_pins_and_final_pin_independently() {
    for sign in [-1.0, 1.0] {
        let mut inputs = INSTRUMENT.to_vec();
        inputs[4].1 = sign * 0.5;
        inputs[5].1 = 0.0;
        inputs[6].1 = -sign * 0.25;
        inputs[9].1 = -0.49;
        inputs[10].1 = 0.49;
        // Final pin is zero; only the two signal inputs violate common mode.
        assert!(calculate("precision.instrumentation_internal_headroom", &inputs).is_err());
        inputs[4].1 = 0.0;
        inputs[6].1 = sign * 1.0;
        // Signal pins and all outputs fit; final pin alone reaches +/-2/3 V.
        assert!(calculate("precision.instrumentation_internal_headroom", &inputs).is_err());
        inputs[6].1 = sign * 0.375;
        let r = calculate("precision.instrumentation_internal_headroom", &inputs).unwrap();
        assert_eq!(r["difference_pin_voltage"], sign * 0.25);
        assert_eq!(r["output_voltage"], sign * 0.375);
    }
}

#[test]
fn tia_nondefault_all_outputs_and_butterworth_frequency_response() {
    let r = calculate("precision.transimpedance_butterworth", TIA).unwrap();
    assert_eq!(r.len(), TIA_EXPECTED.len());
    for &(key, expected) in TIA_EXPECTED {
        assert_close(r[key], expected, 2e-12, 0.0);
    }
    let wt = 2.0 * PI * 1e6;
    let a = 1.0 / wt + 20000.0 * r["feedback_capacitance"];
    let b = 20000.0 * (TIA[1].1 + r["feedback_capacitance"]) / wt;
    assert_close(a * a, 2.0 * b, 2e-12, 0.0);
    let w = 2.0 * PI * r["butterworth_bandwidth"];
    let magnitude_squared = 1.0 / ((1.0 - b * w * w).powi(2) + (a * w).powi(2));
    assert_close(magnitude_squared, 0.5, 2e-12, 0.0);
    // Here Cs/Cf=5/3; a Cs>>Cf approximation is deliberately not acceptable.
    assert_close(r["feedback_capacitance"] / TIA[1].1, 0.6, 2e-12, 0.0);
}

#[test]
fn tia_zero_capacitance_solution_and_adjacent_unsolvable_input() {
    let mut inputs = TIA.to_vec();
    let boundary: f64 = 3.978873577297384e-12;
    inputs[1].1 = f64::from_bits(boundary.to_bits() - 1);
    assert!(calculate("precision.transimpedance_butterworth", &inputs).is_err());
    inputs[1].1 = boundary;
    let r = calculate("precision.transimpedance_butterworth", &inputs).unwrap();
    assert_eq!(r["feedback_capacitance"], 0.0);
    assert_close(r["butterworth_bandwidth"], 2.0_f64.sqrt() * 1e6, 2e-12, 0.0);
    assert_close(r["noise_gain_zero"], 2e6, 2e-12, 0.0);
    assert_eq!(r["dc_transimpedance"], -20000.0);
    assert_eq!(r["output_peak"], 0.5);
    inputs[1].1 = f64::from_bits(boundary.to_bits() + 1);
    let above = calculate("precision.transimpedance_butterworth", &inputs).unwrap();
    assert!(above["feedback_capacitance"] > 0.0);
    let wt = 2.0 * PI * 1e6;
    let a = 1.0 / wt + 20000.0 * above["feedback_capacitance"];
    let b = 20000.0 * (inputs[1].1 + above["feedback_capacitance"]) / wt;
    assert_close(a * a, 2.0 * b, 2e-12, 0.0);
}

#[test]
fn tia_zero_signal_and_inclusive_swing_with_recovery() {
    for peak in [0.0, 0.0001] {
        let mut inputs = TIA.to_vec();
        inputs[3].1 = peak;
        let r = calculate("precision.transimpedance_butterworth", &inputs).unwrap();
        for &(key, expected) in TIA_EXPECTED {
            assert_close(r[key], if key == "output_peak" { peak * 20000.0 } else { expected }, 2e-12, 0.0);
        }
    }
    let mut inputs = TIA.to_vec();
    inputs[3].1 = 0.00010001;
    assert!(calculate("precision.transimpedance_butterworth", &inputs).is_err());
    inputs[3].1 = 0.000025;
    inputs[2].1 = 1000.0;
    assert!(calculate("precision.transimpedance_butterworth", &inputs).is_err());
    let r = calculate("precision.transimpedance_butterworth", TIA).unwrap();
    for &(key, expected) in TIA_EXPECTED {
        assert_close(r[key], expected, 2e-12, 0.0);
    }
}

#[test]
fn howland_nondefault_all_outputs_from_two_node_kcl() {
    let r = calculate("precision.howland_mismatch_load", HOWLAND).unwrap();
    assert_eq!(r.len(), HOWLAND_EXPECTED.len());
    for &(key, expected) in HOWLAND_EXPECTED {
        assert_close(r[key], expected, 2e-12, 0.0);
    }
    let x = r["load_voltage"];
    let out = r["amplifier_voltage"];
    assert_close((0.6 - x) / 2000.0 + (out - x) / 4000.0, x / 1000.0, 2e-12, 0.0);
    assert_close((x + 0.2) / 1000.0, (out - x) / 1800.0, 2e-12, 0.0);
    assert_close(r["amplifier_current"], (out - x) / 4000.0 + (out - x) / 1800.0, 2e-12, 0.0);
    assert!(r["amplifier_current"] > r["load_current"]);
}

#[test]
fn howland_balanced_ratios_remove_load_dependence_not_output_drive() {
    for (load, voltage, output, current) in [(1000.0, 0.4, 1.6, 0.0009), (500.0, 0.2, 1.0, 0.0006)] {
        let mut inputs = HOWLAND.to_vec();
        inputs[3].1 = 2000.0;
        inputs[6].1 = load;
        let r = calculate("precision.howland_mismatch_load", &inputs).unwrap();
        for (key, expected) in [
            ("norton_drive_current", 0.0004), ("output_conductance", 0.0),
            ("load_voltage", voltage), ("load_current", 0.0004),
            ("amplifier_voltage", output), ("amplifier_current", current),
        ] {
            assert_close(r[key], expected, 2e-12, 0.0);
        }
    }
}

#[test]
fn howland_negative_conductance_zero_and_reversed_drives_are_valid() {
    for sign in [-1.0, 0.0, 1.0] {
        let mut inputs = HOWLAND.to_vec();
        inputs[3].1 = 2200.0;
        inputs[4].1 *= sign;
        inputs[5].1 *= sign;
        let r = calculate("precision.howland_mismatch_load", &inputs).unwrap();
        for (key, expected) in [
            ("norton_drive_current", sign * 0.00041), ("output_conductance", -0.00005),
            ("load_voltage", sign * 41.0 / 95.0), ("load_current", sign * 41.0 / 95000.0),
            ("amplifier_voltage", sign * 173.0 / 95.0), ("amplifier_current", sign * 93.0 / 95000.0),
        ] {
            assert_close(r[key], expected, 2e-12, 0.0);
        }
    }
}

#[test]
fn howland_both_polarities_accept_exact_limits_reject_each_excess() {
    for sign in [-1.0, 1.0] {
        let mut inputs = HOWLAND.to_vec();
        for item in &mut inputs[..4] { item.1 = 1024.0; }
        inputs[4].1 = sign * 0.5;
        inputs[5].1 = 0.0;
        inputs[6].1 = 512.0;
        inputs[7].1 = -0.5;
        inputs[8].1 = 0.5;
        inputs[9].1 = -0.25;
        inputs[10].1 = 0.25;
        inputs[11].1 = 1.0 / 2048.0;
        let r = calculate("precision.howland_mismatch_load", &inputs).unwrap();
        for (key, expected) in [
            ("norton_drive_current", sign / 2048.0), ("output_conductance", 0.0),
            ("load_voltage", sign * 0.25), ("load_current", sign / 2048.0),
            ("amplifier_voltage", sign * 0.5), ("amplifier_current", sign / 2048.0),
        ] {
            assert_close(r[key], expected, 2e-12, 0.0);
        }
        for index in [if sign < 0.0 { 7 } else { 8 }, if sign < 0.0 { 9 } else { 10 }, 11] {
            let mut invalid = inputs.clone();
            invalid[index].1 *= 0.99;
            assert!(calculate("precision.howland_mismatch_load", &invalid).is_err());
            let recovered = calculate("precision.howland_mismatch_load", &inputs).unwrap();
            assert_eq!(recovered["amplifier_current"], sign / 2048.0);
        }
    }
}

#[test]
fn howland_decimal_exact_singularity_rejects_even_with_zero_drive() {
    let mut inputs = HOWLAND.to_vec();
    for item in &mut inputs[..3] { item.1 = 10000.0; }
    inputs[3].1 = 30000.0;
    inputs[4].1 = 0.0;
    inputs[5].1 = 0.0;
    inputs[6].1 = 5000.0;
    // Exact conductances cancel. Zero drive does not make an indeterminate loop unique.
    // PRECISION-T1: reciprocal summation currently rounds this strict guard positive.
    assert!(calculate("precision.howland_mismatch_load", &inputs).is_err());
    inputs[3].1 = f64::from_bits(30000.0_f64.to_bits() + 1);
    assert!(calculate("precision.howland_mismatch_load", &inputs).is_err());
    inputs[3].1 = f64::from_bits(30000.0_f64.to_bits() - 1);
    let r = calculate("precision.howland_mismatch_load", &inputs).unwrap();
    for key in ["norton_drive_current", "load_voltage", "load_current", "amplifier_voltage", "amplifier_current"] {
        assert_eq!(r[key], 0.0);
    }
    assert_close(r["output_conductance"], -0.0002, 2e-12, 0.0);
}

#[test]
fn howland_tiny_positive_loaded_conductance_retains_finite_nonzero_solution() {
    let mut inputs = HOWLAND.to_vec();
    for item in &mut inputs[..3] { item.1 = 1024.0; }
    inputs[3].1 = f64::from_bits(3072.0_f64.to_bits() - 1);
    inputs[4].1 = 2.0_f64.powi(-51);
    inputs[5].1 = 0.0;
    inputs[6].1 = 512.0;
    inputs[8].1 = 5.0;
    // R4/R3=3-2^-51. Both drive and loaded conductance are 2^-61, so Vx=1 V.
    let r = calculate("precision.howland_mismatch_load", &inputs).unwrap();
    for (key, expected) in [
        ("norton_drive_current", 2.0_f64.powi(-61)), ("output_conductance", -1.0 / 512.0 + 2.0_f64.powi(-61)),
        ("load_voltage", 1.0), ("load_current", 1.0 / 512.0),
        ("amplifier_voltage", 4.0 - 2.0_f64.powi(-51)), ("amplifier_current", 1.0 / 256.0),
    ] {
        assert_close(r[key], expected, 2e-12, 0.0);
    }
    inputs[3].1 = 3072.0;
    assert!(calculate("precision.howland_mismatch_load", &inputs).is_err());
    inputs[3].1 = 4096.0;
    assert!(calculate("precision.howland_mismatch_load", &inputs).is_err());
}

#[test]
fn brokaw_nondefault_all_outputs_and_two_emitter_current_drop() {
    let r = calculate("precision.brokaw_first_order_trim", BROKAW).unwrap();
    assert_eq!(r.len(), BROKAW_EXPECTED.len());
    for &(key, expected) in BROKAW_EXPECTED {
        assert_close(r[key], expected, 2e-12, 0.0);
    }
    assert_close(2.0 * r["branch_current"] * r["common_resistor"], 0.576, 2e-12, 0.0);
    assert_close(0.61 + 2.0 * r["branch_current"] * r["common_resistor"], r["reference_voltage"], 2e-12, 0.0);
    assert_close(2.0 * r["resistor_ratio"] * r["delta_vbe"] / 320.0, 0.0018, 2e-12, 0.0);
}

#[test]
fn brokaw_area_and_resistor_scaling_preserve_reference() {
    for (area, resistance, delta_scale, current_scale, ratio_scale, resistor_scale) in [
        (16.0, 8000.0, 2.0, 2.0, 0.5, 0.5),
        (4.0, 16000.0, 1.0, 0.5, 1.0, 2.0),
        (2.0, 8000.0, 0.5, 0.5, 2.0, 2.0),
    ] {
        let mut inputs = BROKAW.to_vec();
        inputs[1].1 = area;
        inputs[2].1 = resistance;
        let r = calculate("precision.brokaw_first_order_trim", &inputs).unwrap();
        for &(key, expected) in BROKAW_EXPECTED {
            let scale = match key {
                "delta_vbe" => delta_scale, "branch_current" => current_scale,
                "resistor_ratio" => ratio_scale, "common_resistor" => resistor_scale, _ => 1.0,
            };
            assert_close(r[key], expected * scale, 2e-12, 0.0);
        }
    }
}

#[test]
fn brokaw_area_ratio_next_to_one_and_fractional_geometry_are_admitted() {
    let mut inputs = BROKAW.to_vec();
    inputs[1].1 = 1.0;
    assert!(calculate("precision.brokaw_first_order_trim", &inputs).is_err());
    inputs[1].1 = f64::from_bits(1.0_f64.to_bits() + 1);
    let r = calculate("precision.brokaw_first_order_trim", &inputs).unwrap();
    let epsilon = f64::EPSILON;
    // ln(1+epsilon)=epsilon-epsilon^2/2; next omitted term is <2e-32 relative.
    let log_area = epsilon - epsilon * epsilon / 2.0;
    let delta = 0.027575466438864566 * log_area;
    assert_close(r["thermal_voltage"], 0.027575466438864566, 2e-12, 0.0);
    assert_close(r["delta_vbe"], delta, 2e-12, 0.0);
    assert_close(r["branch_current"], delta / 8000.0, 2e-12, 0.0);
    assert_close(r["resistor_ratio"], 0.576 / (2.0 * delta), 2e-12, 0.0);
    assert_close(r["common_resistor"], 0.576 * 8000.0 / (2.0 * delta), 2e-12, 0.0);
    assert_close(r["reference_voltage"], 1.186, 2e-12, 0.0);
    inputs[1].1 = 1.5;
    let fractional = calculate("precision.brokaw_first_order_trim", &inputs).unwrap();
    assert_close(fractional["delta_vbe"], 0.027575466438864566 * 1.5_f64.ln(), 2e-12, 0.0);
    assert_close(2.0 * fractional["branch_current"] * fractional["common_resistor"], 0.576, 2e-12, 0.0);
}

#[test]
fn brokaw_local_temperature_trajectory_cancels_first_order_slope() {
    for temperature in [319.0, 321.0] {
        let mut inputs = BROKAW.to_vec();
        inputs[0].1 = temperature;
        inputs[3].1 = 0.61 - 0.0018 * (temperature - 320.0);
        let r = calculate("precision.brokaw_first_order_trim", &inputs).unwrap();
        assert_close(r["reference_voltage"], 1.186, 2e-12, 0.0);
        assert_close(r["branch_current"], 4.778464203681092e-6 * temperature / 320.0, 2e-12, 0.0);
        assert_close(r["common_resistor"], 60270.41068511909, 2e-12, 0.0);
    }
}

#[test]
fn log114_nondefault_negative_decade_all_outputs() {
    let r = calculate("precision.log114_current_ratio", LOG).unwrap();
    assert_eq!(r.len(), LOG_EXPECTED.len());
    for &(key, expected) in LOG_EXPECTED {
        assert_close(r[key], expected, 2e-12, 0.0);
    }
    assert_close(r["local_transimpedance"] * 2e-7, 0.375 / LN_10, 2e-12, 0.0);
}

#[test]
fn log114_equal_and_adjacent_currents_retain_signed_tiny_voltage() {
    let reference: f64 = 2e-6;
    let mut inputs = LOG.to_vec();
    for current in [f64::from_bits(reference.to_bits() - 1), reference, f64::from_bits(reference.to_bits() + 1)] {
        inputs[0].1 = current;
        let r = calculate("precision.log114_current_ratio", &inputs).unwrap();
        let delta = (current - reference) / reference;
        let decades = (delta - delta * delta / 2.0) / LN_10;
        assert_close(r["reference_current"], reference, 2e-12, 0.0);
        assert_close(r["current_ratio"], 1.0 + delta, 2e-12, 0.0);
        assert_close(r["log_decades"], decades, 2e-12, 0.0);
        assert_close(r["log_output"], 0.375 * decades, 2e-12, 0.0);
        assert_close(r["local_transimpedance"], 0.375 / LN_10 / current, 2e-12, 0.0);
        if current != reference { assert_eq!(r["log_output"].is_sign_positive(), current > reference); }
    }
}

#[test]
fn log114_one_decade_and_reference_scaling_follow_device_slope() {
    for (signal, resistance, reference, ratio, decades) in [
        (2e-8, 1250000.0, 2e-6, 0.01, -2.0),
        (2e-6, 1250000.0, 2e-6, 1.0, 0.0),
        (2e-7, 12500000.0, 2e-7, 1.0, 0.0),
        (2e-7, 125000.0, 2e-5, 0.01, -2.0),
    ] {
        let r = calculate("precision.log114_current_ratio", &[("signal_current", signal), ("reference_resistance", resistance)]).unwrap();
        for (key, expected) in [
            ("reference_current", reference), ("current_ratio", ratio), ("log_decades", decades),
            ("log_output", 0.375 * decades), ("local_transimpedance", 0.375 / LN_10 / signal),
        ] {
            assert_close(r[key], expected, 2e-12, 0.0);
        }
    }
}

#[test]
fn log114_all_device_range_corners_keep_full_log_ratio_precision() {
    for (signal, resistance, reference, ratio, decades) in [
        (1e-10, 1250.0, 0.002, 5e-8, -7.301029995663981),
        (0.01, 1250.0, 0.002, 5.0, 0.6989700043360189),
        (1e-10, 25000000000.0, 1e-10, 1.0, 0.0),
        (0.01, 25000000000.0, 1e-10, 1e8, 8.0),
    ] {
        let r = calculate("precision.log114_current_ratio", &[("signal_current", signal), ("reference_resistance", resistance)]).unwrap();
        // PRECISION-T2: log1p is accurate near unity, but subtraction near -1 loses low-ratio digits.
        for (key, expected) in [
            ("reference_current", reference), ("current_ratio", ratio), ("log_decades", decades),
            ("log_output", 0.375 * decades), ("local_transimpedance", 0.375 / LN_10 / signal),
        ] {
            assert_close(r[key], expected, 2e-12, 0.0);
        }
    }
}

#[test]
fn every_precision_scalar_domain_rejects_and_recovers_with_complete_map() {
    let cases: &[(&str, &[(&str, f64)], &[(&str, f64)], &[(usize, f64)])] = &[
        ("precision.bjt_mirror_base_current", MIRROR, MIRROR_EXPECTED,
            &[(0, 0.0), (0, -1.0), (1, 0.0), (1, -1.0), (2, 0.0), (2, -1.0), (3, -1.0)]),
        ("precision.instrumentation_internal_headroom", INSTRUMENT, INSTRUMENT_EXPECTED,
            &[(0, 0.0), (0, -1.0), (1, 0.0), (1, -1.0), (2, 0.0), (2, -1.0), (3, 0.0), (3, -1.0),
              (7, 3.0), (7, 4.0), (9, 2.0), (9, 3.0), (6, -4.0), (6, 4.0)]),
        ("precision.transimpedance_butterworth", TIA, TIA_EXPECTED,
            &[(0, 0.0), (0, -1.0), (1, 0.0), (1, -1e-12), (2, 0.0), (2, -1.0), (3, -1e-6), (4, 0.0), (4, -1.0)]),
        ("precision.howland_mismatch_load", HOWLAND, HOWLAND_EXPECTED,
            &[(0, 0.0), (0, -1.0), (1, 0.0), (1, -1.0), (2, 0.0), (2, -1.0), (3, 0.0), (3, -1.0),
              (6, 0.0), (6, -1.0), (7, 4.0), (7, 5.0), (9, 2.0), (9, 3.0), (11, 0.0), (11, -1.0)]),
        ("precision.brokaw_first_order_trim", BROKAW, BROKAW_EXPECTED,
            &[(0, 0.0), (0, -1.0), (1, 0.0), (1, 1.0), (1, -1.0), (2, 0.0), (2, -1.0), (3, 0.0), (3, -1.0), (4, 0.0), (4, 0.001)]),
        ("precision.log114_current_ratio", LOG, LOG_EXPECTED,
            &[(0, 0.0), (0, -1.0), (0, 0.5e-10), (0, 0.010000001), (1, 1249.0), (1, 25000000001.0), (1, 0.0), (1, -1.0)]),
    ];
    for &(id, nominal, expected, invalids) in cases {
        for &(index, bad) in invalids {
            let mut inputs = nominal.to_vec();
            inputs[index].1 = bad;
            assert!(calculate(id, &inputs).is_err(), "accepted {id} {}={bad}", inputs[index].0);
            let recovered = calculate(id, nominal).unwrap();
            assert_eq!(recovered.len(), expected.len());
            for &(key, value) in expected { assert_close(recovered[key], value, 2e-12, 0.0); }
        }
    }
}

#[test]
fn every_precision_input_rejects_nonfinite_values_without_poisoning_engine() {
    for (id, nominal, expected) in [
        ("precision.bjt_mirror_base_current", MIRROR, MIRROR_EXPECTED),
        ("precision.instrumentation_internal_headroom", INSTRUMENT, INSTRUMENT_EXPECTED),
        ("precision.transimpedance_butterworth", TIA, TIA_EXPECTED),
        ("precision.howland_mismatch_load", HOWLAND, HOWLAND_EXPECTED),
        ("precision.brokaw_first_order_trim", BROKAW, BROKAW_EXPECTED),
        ("precision.log114_current_ratio", LOG, LOG_EXPECTED),
    ] {
        for index in 0..nominal.len() {
            for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                let mut inputs = nominal.to_vec();
                inputs[index].1 = bad;
                assert!(calculate(id, &inputs).is_err(), "accepted {id} {}={bad}", inputs[index].0);
                let recovered = calculate(id, nominal).unwrap();
                assert_eq!(recovered.len(), expected.len());
                for &(key, value) in expected { assert_close(recovered[key], value, 2e-12, 0.0); }
            }
        }
    }
}
