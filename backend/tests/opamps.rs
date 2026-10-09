mod common;

use common::{assert_close, calculate};

// Complete, explicit SI vectors from the independently derived TI/KCL examples
// in research/opamps.md. No expected value is obtained from the engine.
const INVERTING: [(&str, f64); 8] = [
    ("r_in", 10000.0), ("r_feedback", 47000.0), ("v_in", 0.2),
    ("v_reference", 0.0), ("v_out_min", -10.0), ("v_out_max", 10.0),
    ("v_cm_min", -10.0), ("v_cm_max", 10.0),
];
const NONINVERTING: [(&str, f64); 8] = [
    ("r_ground", 10000.0), ("r_feedback", 90000.0), ("v_in", 0.4),
    ("v_reference", 0.1), ("v_out_min", -10.0), ("v_out_max", 10.0),
    ("v_cm_min", -10.0), ("v_cm_max", 10.0),
];
const DIFFERENCE: [(&str, f64); 11] = [
    ("r_one", 10000.0), ("r_two", 100000.0), ("r_three", 10000.0),
    ("r_four", 101000.0), ("v_one", 1.0), ("v_two", 1.1),
    ("v_reference", 0.0), ("v_out_min", -10.0), ("v_out_max", 10.0),
    ("v_cm_min", -10.0), ("v_cm_max", 10.0),
];
const SUMMING: [(&str, f64); 12] = [
    ("r_one", 10000.0), ("r_two", 20000.0), ("r_three", 40000.0),
    ("r_feedback", 40000.0), ("v_one", 0.2), ("v_two", -0.3),
    ("v_three", 0.4), ("v_reference", 0.0),
    ("v_out_min", -10.0), ("v_out_max", 10.0),
    ("v_cm_min", -10.0), ("v_cm_max", 10.0),
];
const INTEGRATOR: [(&str, f64); 10] = [
    ("r_input", 10000.0), ("c_feedback", 1e-7), ("v_in", 0.25),
    ("v_reference", 0.0), ("v_initial", 1.0), ("duration", 0.002),
    ("v_out_min", -10.0), ("v_out_max", 10.0),
    ("v_cm_min", -10.0), ("v_cm_max", 10.0),
];
const DIFFERENTIATOR: [(&str, f64); 10] = [
    ("r_feedback", 10000.0), ("c_input", 1e-8), ("v_start", -0.5),
    ("v_end", 0.5), ("duration", 0.001), ("v_reference", 0.0),
    ("v_out_min", -10.0), ("v_out_max", 10.0),
    ("v_cm_min", -10.0), ("v_cm_max", 10.0),
];
const SLEW: [(&str, f64); 7] = [
    ("frequency", 100000.0), ("v_peak", 2.0), ("v_center", 0.0),
    ("slew_available", 5000000.0), ("design_margin", 2.0),
    ("v_out_min", -10.0), ("v_out_max", 10.0),
];
const BANDWIDTH: [(&str, f64); 5] = [
    ("r_ground", 10000.0), ("r_feedback", 90000.0),
    ("open_loop_gain", 100000.0), ("gain_bandwidth", 10000000.0),
    ("minimum_stable_noise_gain", 1.0),
];

#[test]
fn inverting_complete_researched_example() {
    // 20 uA input current requires a -0.94 V feedback drop.
    let out = calculate("opamps.inverting_stage", &INVERTING).unwrap();
    assert_close(out["gain"], -4.7, 1e-6, 0.0);
    assert_close(out["v_out"], -0.94, 1e-6, 0.0);
}

#[test]
fn inverting_reference_bias_zero_and_both_inclusive_limits() {
    for sign in [-1.0, 1.0] {
        // 0.5 mA through 1 kohm is opposed by 2 kohm feedback.
        // Source voltage is outside the pin range; only Vref is the pin.
        let out = calculate("opamps.inverting_stage", &[
            ("r_in", 1000.0), ("r_feedback", 2000.0),
            ("v_in", 1.5 * sign), ("v_reference", sign),
            ("v_out_min", if sign > 0.0 { -1.0 } else { 0.0 }),
            ("v_out_max", if sign > 0.0 { 0.0 } else { 1.0 }),
            ("v_cm_min", if sign > 0.0 { 0.0 } else { -1.0 }),
            ("v_cm_max", if sign > 0.0 { 1.0 } else { 0.0 }),
        ]).unwrap();
        assert_close(out["gain"], -2.0, 1e-6, 0.0);
        // Cancellation at a 1 V scale: 10 fV allows only floating roundoff.
        assert_close(out["v_out"], 0.0, 0.0, 1e-14);
    }
}

#[test]
fn inverting_rejects_every_resistor_and_operating_range_guard() {
    for (key, value) in [
        ("r_in", 0.0), ("r_in", -1.0),
        ("r_feedback", 0.0), ("r_feedback", -1.0),
        ("v_out_min", 10.0), ("v_out_min", 11.0),
        ("v_out_max", -10.0), ("v_out_max", -11.0),
        ("v_cm_min", 10.0), ("v_cm_min", 11.0),
        ("v_cm_max", -10.0), ("v_cm_max", -11.0),
        ("v_cm_min", 9.0), ("v_cm_max", -9.0),
        ("v_out_min", 9.0), ("v_out_max", -9.0),
    ] {
        let mut inputs = INVERTING;
        inputs.iter_mut().find(|entry| entry.0 == key).unwrap().1 = value;
        assert!(calculate("opamps.inverting_stage", &inputs).is_err(), "accepted {key}={value}");
    }
}

#[test]
fn noninverting_complete_researched_example() {
    // 30 uA reference-divider current raises output 2.7 V above Vin.
    let out = calculate("opamps.noninverting_stage", &NONINVERTING).unwrap();
    assert_close(out["gain"], 10.0, 1e-6, 0.0);
    assert_close(out["v_out"], 3.1, 1e-6, 0.0);
}

#[test]
fn noninverting_unity_buffer_and_reference_shift() {
    for vin in [-1.0, 0.0, 1.0] {
        // Rf=0 shorts output to the pin; reference is not common-mode.
        let out = calculate("opamps.noninverting_stage", &[
            ("r_ground", 1000.0), ("r_feedback", 0.0), ("v_in", vin),
            ("v_reference", 100.0), ("v_out_min", -1.0), ("v_out_max", 1.0),
            ("v_cm_min", -1.0), ("v_cm_max", 1.0),
        ]).unwrap();
        assert_close(out["gain"], 1.0, 1e-6, 0.0);
        // Subtraction/addition at a 100 V reference scale: <1 pV roundoff.
        assert_close(out["v_out"], vin, 1e-6, 1e-12);
    }
    let out = calculate("opamps.noninverting_stage", &[
        ("r_ground", 1000.0), ("r_feedback", 2000.0), ("v_in", -0.5),
        ("v_reference", -1.0), ("v_out_min", -2.0), ("v_out_max", 2.0),
        ("v_cm_min", -0.5), ("v_cm_max", 0.5),
    ]).unwrap();
    assert_close(out["gain"], 3.0, 1e-6, 0.0);
    assert_close(out["v_out"], 0.5, 1e-6, 0.0);
}

#[test]
fn noninverting_rejects_every_resistor_and_operating_range_guard() {
    for (key, value) in [
        ("r_ground", 0.0), ("r_ground", -1.0), ("r_feedback", -1.0),
        ("v_out_min", 10.0), ("v_out_min", 11.0),
        ("v_out_max", -10.0), ("v_out_max", -11.0),
        ("v_cm_min", 10.0), ("v_cm_min", 11.0),
        ("v_cm_max", -10.0), ("v_cm_max", -11.0),
        ("v_cm_min", 9.0), ("v_cm_max", -9.0),
        ("v_out_min", 9.0), ("v_out_max", -9.0),
    ] {
        let mut inputs = NONINVERTING;
        inputs.iter_mut().find(|entry| entry.0 == key).unwrap().1 = value;
        assert!(calculate("opamps.noninverting_stage", &inputs).is_err(), "accepted {key}={value}");
    }
}

#[test]
fn difference_complete_researched_example() {
    // Rational divider: p=101/111, Ad=10+1/222, Acm=1/111.
    let out = calculate("opamps.difference_ratio_error", &DIFFERENCE).unwrap();
    assert_close(out["differential_gain"], 10.004504504504505, 1e-6, 0.0);
    assert_close(out["common_mode_gain"], 0.009009009009009009, 1e-6, 0.0);
    assert_close(out["pin_voltage"], 1.000900900900901, 1e-6, 0.0);
    assert_close(out["v_out"], 1.0099099099099098, 1e-6, 0.0);
}

#[test]
fn difference_matched_noninteger_ratios_preserve_exact_common_mode_zero() {
    let out = calculate("opamps.difference_ratio_error", &[
        ("r_one", 10000.0), ("r_two", 11000.0), ("r_three", 20000.0),
        ("r_four", 22000.0), ("v_one", 1.0), ("v_two", 2.0),
        ("v_reference", -0.5), ("v_out_min", -2.0), ("v_out_max", 2.0),
        ("v_cm_min", 0.0), ("v_cm_max", 1.0),
    ]).unwrap();
    assert_close(out["differential_gain"], 1.1, 1e-6, 0.0);
    // Equal computed ratios must cancel exactly, not leave residual leakage.
    assert_close(out["common_mode_gain"], 0.0, 0.0, 0.0);
    assert_close(out["pin_voltage"], 17.0 / 21.0, 1e-6, 0.0);
    assert_close(out["v_out"], 0.6, 1e-6, 0.0);
}

#[test]
fn difference_signed_leakage_and_pin_not_source_common_mode() {
    for (r_four, ad, acm, pin, voltage) in [
        (3000.0, 1.25, 0.5, 0.75, 0.5),
        (1000.0 / 3.0, 0.75, -0.5, 0.25, -0.5),
    ] {
        // Equal sources remove differential excitation; output is Acm*1 V.
        let out = calculate("opamps.difference_ratio_error", &[
            ("r_one", 1000.0), ("r_two", 1000.0), ("r_three", 1000.0),
            ("r_four", r_four), ("v_one", 1.0), ("v_two", 1.0),
            ("v_reference", 0.0), ("v_out_min", -1.0), ("v_out_max", 1.0),
            ("v_cm_min", 0.2), ("v_cm_max", 0.8),
        ]).unwrap();
        assert_close(out["differential_gain"], ad, 1e-6, 0.0);
        assert_close(out["common_mode_gain"], acm, 1e-6, 0.0);
        assert_close(out["pin_voltage"], pin, 1e-6, 0.0);
        assert_close(out["v_out"], voltage, 1e-6, 0.0);
    }
    for sign in [-1.0, 1.0] {
        let out = calculate("opamps.difference_ratio_error", &[
            ("r_one", 1000.0), ("r_two", 1000.0), ("r_three", 1000.0),
            ("r_four", 1000.0), ("v_one", 0.0), ("v_two", 2.0 * sign),
            ("v_reference", 0.0), ("v_out_min", -2.0), ("v_out_max", 2.0),
            ("v_cm_min", -1.0), ("v_cm_max", 1.0),
        ]).unwrap();
        assert_close(out["differential_gain"], 1.0, 1e-6, 0.0);
        assert_close(out["common_mode_gain"], 0.0, 0.0, 0.0);
        assert_close(out["pin_voltage"], sign, 1e-6, 0.0);
        assert_close(out["v_out"], 2.0 * sign, 1e-6, 0.0);
    }
}

#[test]
fn difference_rejects_every_resistor_and_operating_range_guard() {
    for (key, value) in [
        ("r_one", 0.0), ("r_one", -1.0), ("r_two", 0.0), ("r_two", -1.0),
        ("r_three", 0.0), ("r_three", -1.0), ("r_four", 0.0), ("r_four", -1.0),
        ("v_out_min", 10.0), ("v_out_min", 11.0),
        ("v_out_max", -10.0), ("v_out_max", -11.0),
        ("v_cm_min", 10.0), ("v_cm_min", 11.0),
        ("v_cm_max", -10.0), ("v_cm_max", -11.0),
        ("v_cm_min", 9.0), ("v_cm_max", -9.0),
        ("v_out_min", 9.0), ("v_out_max", -9.0),
    ] {
        let mut inputs = DIFFERENCE;
        inputs.iter_mut().find(|entry| entry.0 == key).unwrap().1 = value;
        assert!(calculate("opamps.difference_ratio_error", &inputs).is_err(), "accepted {key}={value}");
    }
}

#[test]
fn summing_complete_researched_example() {
    // Independent source currents 20,-15,10 uA sum to 15 uA.
    let out = calculate("opamps.weighted_summing_stage", &SUMMING).unwrap();
    assert_close(out["gain_one"], -4.0, 1e-6, 0.0);
    assert_close(out["gain_two"], -2.0, 1e-6, 0.0);
    assert_close(out["gain_three"], -1.0, 1e-6, 0.0);
    assert_close(out["v_out"], -0.6, 1e-6, 0.0);
}

#[test]
fn summing_reference_superposition_cancellation_and_inclusive_ranges() {
    for (v_one, v_two, v_three, reference, expected) in [
        (1.5, 0.5, 0.0, 1.0, 1.0),
        (1.0, -2.0, 0.0, 0.0, 0.0),
        (1.0, 1.0, 1.0, 1.0, 1.0),
        (-1.0, -1.0, -1.0, -1.0, -1.0),
    ] {
        let out = calculate("opamps.weighted_summing_stage", &[
            ("r_one", 10000.0), ("r_two", 20000.0), ("r_three", 40000.0),
            ("r_feedback", 40000.0), ("v_one", v_one), ("v_two", v_two),
            ("v_three", v_three), ("v_reference", reference),
            ("v_out_min", -1.0), ("v_out_max", 1.0),
            ("v_cm_min", -1.0), ("v_cm_max", 1.0),
        ]).unwrap();
        assert_close(out["gain_one"], -4.0, 1e-6, 0.0);
        assert_close(out["gain_two"], -2.0, 1e-6, 0.0);
        assert_close(out["gain_three"], -1.0, 1e-6, 0.0);
        // Up to 4 V signed contributions: 100 fV covers rounding at cancellation.
        assert_close(out["v_out"], expected, 1e-6, 1e-13);
    }
}

#[test]
fn summing_rejects_every_resistor_and_operating_range_guard() {
    for (key, value) in [
        ("r_one", 0.0), ("r_one", -1.0), ("r_two", 0.0), ("r_two", -1.0),
        ("r_three", 0.0), ("r_three", -1.0),
        ("r_feedback", 0.0), ("r_feedback", -1.0),
        ("v_out_min", 10.0), ("v_out_min", 11.0),
        ("v_out_max", -10.0), ("v_out_max", -11.0),
        ("v_cm_min", 10.0), ("v_cm_min", 11.0),
        ("v_cm_max", -10.0), ("v_cm_max", -11.0),
        ("v_cm_min", 9.0), ("v_cm_max", -9.0),
        ("v_out_min", 9.0), ("v_out_max", -9.0),
    ] {
        let mut inputs = SUMMING;
        inputs.iter_mut().find(|entry| entry.0 == key).unwrap().1 = value;
        assert!(calculate("opamps.weighted_summing_stage", &inputs).is_err(), "accepted {key}={value}");
    }
}

#[test]
fn integrator_complete_researched_example() {
    // 50 nC accumulated on 100 nF gives a -0.5 V change.
    let out = calculate("opamps.integrator_constant_input", &INTEGRATOR).unwrap();
    assert_close(out["output_slope"], -250.0, 1e-6, 0.0);
    assert_close(out["v_final"], 0.5, 1e-6, 0.0);
}

#[test]
fn integrator_signed_ramps_zero_time_zero_current_and_endpoint_boundaries() {
    for (vin, reference, initial, duration, slope, final_voltage) in [
        (-0.5, 0.5, -1.0, 2.0, 1.0, 1.0),
        (0.5, -0.5, 1.0, 2.0, -1.0, -1.0),
        (0.5, -0.5, 1.0, 0.0, -1.0, 1.0),
        (0.5, 0.5, -0.25, 100.0, 0.0, -0.25),
    ] {
        // RC=1 s: one volt differential yields one V/s output slope.
        let out = calculate("opamps.integrator_constant_input", &[
            ("r_input", 1000.0), ("c_feedback", 0.001), ("v_in", vin),
            ("v_reference", reference), ("v_initial", initial), ("duration", duration),
            ("v_out_min", -1.0), ("v_out_max", 1.0),
            ("v_cm_min", -0.5), ("v_cm_max", 0.5),
        ]).unwrap();
        // Identical input/reference subtraction gives exact zero current/slope.
        assert_close(out["output_slope"], slope, 1e-6, 0.0);
        assert_close(out["v_final"], final_voltage, 1e-6, 0.0);
    }
}

#[test]
fn integrator_rejects_every_scalar_pin_initial_and_final_guard() {
    for (key, value) in [
        ("r_input", 0.0), ("r_input", -1.0),
        ("c_feedback", 0.0), ("c_feedback", -1e-7), ("duration", -0.001),
        ("v_out_min", 10.0), ("v_out_min", 11.0),
        ("v_out_max", -10.0), ("v_out_max", -11.0),
        ("v_cm_min", 10.0), ("v_cm_min", 11.0),
        ("v_cm_max", -10.0), ("v_cm_max", -11.0),
        ("v_cm_min", 9.0), ("v_cm_max", -9.0),
        ("v_initial", -11.0), ("v_initial", 11.0),
        ("duration", 1.0), ("v_in", -10.0),
    ] {
        let mut inputs = INTEGRATOR;
        inputs.iter_mut().find(|entry| entry.0 == key).unwrap().1 = value;
        // duration=1 -> -249 V final; Vin=-10 -> +21 V final;
        // their initial 1 V state remains admissible, isolating final guards.
        assert!(calculate("opamps.integrator_constant_input", &inputs).is_err(), "accepted {key}={value}");
    }
}

#[test]
fn differentiator_complete_researched_example() {
    // 10 nF * 1000 V/s = 10 uA, opposed by 10 kohm feedback.
    let out = calculate("opamps.differentiator_ramp", &DIFFERENTIATOR).unwrap();
    assert_close(out["input_slope"], 1000.0, 1e-6, 0.0);
    assert_close(out["v_out"], -0.1, 1e-6, 0.0);
}

#[test]
fn differentiator_descending_flat_and_both_pin_output_boundaries() {
    for (start, end, duration, reference, slope, voltage) in [
        (2.0, -2.0, 2.0, -0.5, -2.0, 1.5),
        (0.0, 1.0, 1.0, 0.5, 1.0, -0.5),
        (100.0, 100.0, 1.0, 0.5, 0.0, 0.5),
        (100.0, 100.0, 1.0, 0.0, 0.0, 0.0),
    ] {
        // Rf*C=1 s; source endpoints are not common-mode pin voltages.
        let out = calculate("opamps.differentiator_ramp", &[
            ("r_feedback", 1000.0), ("c_input", 0.001),
            ("v_start", start), ("v_end", end), ("duration", duration),
            ("v_reference", reference), ("v_out_min", -0.5), ("v_out_max", 1.5),
            ("v_cm_min", -0.5), ("v_cm_max", 0.5),
        ]).unwrap();
        // Flat ramps subtract identical endpoints: exact zero is appropriate.
        assert_close(out["input_slope"], slope, 1e-6, 0.0);
        assert_close(out["v_out"], voltage, 1e-6, 0.0);
    }
}

#[test]
fn differentiator_rejects_every_scalar_and_operating_range_guard() {
    for (key, value) in [
        ("r_feedback", 0.0), ("r_feedback", -1.0),
        ("c_input", 0.0), ("c_input", -1e-8), ("duration", 0.0), ("duration", -0.001),
        ("v_out_min", 10.0), ("v_out_min", 11.0),
        ("v_out_max", -10.0), ("v_out_max", -11.0),
        ("v_cm_min", 10.0), ("v_cm_min", 11.0),
        ("v_cm_max", -10.0), ("v_cm_max", -11.0),
        ("v_cm_min", 9.0), ("v_cm_max", -9.0),
        ("v_out_min", 9.0), ("v_out_max", -9.0),
    ] {
        let mut inputs = DIFFERENTIATOR;
        inputs.iter_mut().find(|entry| entry.0 == key).unwrap().1 = value;
        assert!(calculate("opamps.differentiator_ramp", &inputs).is_err(), "accepted {key}={value}");
    }
}

#[test]
fn slew_complete_researched_example() {
    let out = calculate("opamps.sinusoidal_slew_requirement", &SLEW).unwrap();
    assert_close(out["required_slew"], 1256637.0614359172, 1e-6, 0.0);
    assert_close(out["design_slew"], 2513274.1228718343, 1e-6, 0.0);
    assert_close(out["slew_margin"], 3.978873577297384, 1e-6, 0.0);
}

#[test]
fn slew_keeps_insufficient_margin_and_both_exact_peak_boundaries() {
    for center in [-2.0, 2.0] {
        // One-Hz, one-V-peak sine has max slope 2*pi V/s, not RMS/pp.
        // Available pi V/s gives a meaningful diagnostic margin 0.5.
        let out = calculate("opamps.sinusoidal_slew_requirement", &[
            ("frequency", 1.0), ("v_peak", 1.0), ("v_center", center),
            ("slew_available", std::f64::consts::PI), ("design_margin", 1.0),
            ("v_out_min", center - 1.0), ("v_out_max", center + 1.0),
        ]).unwrap();
        assert_close(out["required_slew"], 6.283185307179586, 1e-6, 0.0);
        assert_close(out["design_slew"], 6.283185307179586, 1e-6, 0.0);
        assert_close(out["slew_margin"], 0.5, 1e-6, 0.0);
    }
    // Reciprocal frequency/amplitude scaling preserves the slope requirement.
    let out = calculate("opamps.sinusoidal_slew_requirement", &[
        ("frequency", 2.0), ("v_peak", 0.5), ("v_center", 0.0),
        ("slew_available", 10.0), ("design_margin", 3.0),
        ("v_out_min", -0.5), ("v_out_max", 0.5),
    ]).unwrap();
    assert_close(out["required_slew"], 6.283185307179586, 1e-6, 0.0);
    assert_close(out["design_slew"], 18.84955592153876, 1e-6, 0.0);
    assert_close(out["slew_margin"], 1.5915494309189535, 1e-6, 0.0);
}

#[test]
fn slew_small_positive_frequency_is_not_treated_as_zero() {
    let out = calculate("opamps.sinusoidal_slew_requirement", &[
        ("frequency", 1e-9), ("v_peak", 1.0), ("v_center", 0.0),
        ("slew_available", 1e-9), ("design_margin", 1.0),
        ("v_out_min", -1.0), ("v_out_max", 1.0),
    ]).unwrap();
    assert_close(out["required_slew"], 6.283185307179586e-9, 1e-6, 0.0);
    assert_close(out["design_slew"], 6.283185307179586e-9, 1e-6, 0.0);
    assert_close(out["slew_margin"], 0.15915494309189535, 1e-6, 0.0);
}

#[test]
fn slew_rejects_every_scalar_peak_and_range_guard() {
    for (key, value) in [
        ("frequency", 0.0), ("frequency", -1.0),
        ("v_peak", 0.0), ("v_peak", -1.0),
        ("slew_available", 0.0), ("slew_available", -1.0),
        ("design_margin", 0.999),
        ("v_out_min", 10.0), ("v_out_min", 11.0),
        ("v_out_max", -10.0), ("v_out_max", -11.0),
        ("v_center", -9.0), ("v_center", 9.0),
    ] {
        let mut inputs = SLEW;
        inputs.iter_mut().find(|entry| entry.0 == key).unwrap().1 = value;
        assert!(calculate("opamps.sinusoidal_slew_requirement", &inputs).is_err(), "accepted {key}={value}");
    }
}

#[test]
fn bandwidth_complete_researched_example() {
    let out = calculate("opamps.dominant_pole_gain_bandwidth", &BANDWIDTH).unwrap();
    assert_close(out["noise_gain"], 10.0, 1e-6, 0.0);
    assert_close(out["open_loop_pole"], 100.0, 1e-6, 0.0);
    assert_close(out["dc_gain"], 9.99900009999, 1e-6, 0.0);
    assert_close(out["closed_loop_bandwidth"], 1000100.0, 1e-6, 0.0);
    assert_close(out["approximate_bandwidth"], 1000000.0, 1e-6, 0.0);
    assert_close(out["relative_gain_error"], 0.00009999000099990002, 1e-6, 0.0);
    assert_close(out["dc_gain"] * out["closed_loop_bandwidth"], 10000000.0, 1e-6, 0.0);
}

#[test]
fn bandwidth_finite_loop_gain_noise_gain_and_stability_equality() {
    // beta=1/2, A0=2 => loop gain 1. Exact gain=1 and pole=100 Hz.
    // GBW is 100 Hz asymptote, NOT the unity crossing 50*sqrt(3) Hz.
    let out = calculate("opamps.dominant_pole_gain_bandwidth", &[
        ("r_ground", 1000.0), ("r_feedback", 1000.0), ("open_loop_gain", 2.0),
        ("gain_bandwidth", 100.0), ("minimum_stable_noise_gain", 2.0),
    ]).unwrap();
    for (key, expected) in [
        ("noise_gain", 2.0), ("open_loop_pole", 50.0), ("dc_gain", 1.0),
        ("closed_loop_bandwidth", 100.0), ("approximate_bandwidth", 50.0),
        ("relative_gain_error", 0.5),
    ] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
    assert_close(out["dc_gain"] * out["closed_loop_bandwidth"], 100.0, 1e-6, 0.0);
    // Inverting signal |gain| would be 1, but bandwidth uses noise gain 2.
    assert_close(out["closed_loop_bandwidth"] - out["approximate_bandwidth"], 50.0, 1e-6, 0.0);
}

#[test]
fn bandwidth_unity_feedback_boundary_and_high_loop_gain_error_conditioning() {
    for (feedback, a0, gbw, noise, pole, gain, bandwidth, approximation, error) in [
        (0.0, 2.0, 100.0, 1.0, 50.0, 2.0 / 3.0, 150.0, 100.0, 1.0 / 3.0),
        (1000.0, 1e16, 1e6, 2.0, 1e-10, 2.0, 500000.0, 500000.0, 2e-16),
    ] {
        let out = calculate("opamps.dominant_pole_gain_bandwidth", &[
            ("r_ground", 1000.0), ("r_feedback", feedback), ("open_loop_gain", a0),
            ("gain_bandwidth", gbw), ("minimum_stable_noise_gain", 1.0),
        ]).unwrap();
        for (key, expected) in [
            ("noise_gain", noise), ("open_loop_pole", pole), ("dc_gain", gain),
            ("closed_loop_bandwidth", bandwidth), ("approximate_bandwidth", approximation),
            ("relative_gain_error", error),
        ] {
            // No absolute floor: retain the physically positive 2e-16 deficit.
            assert_close(out[key], expected, 1e-6, 0.0);
        }
        assert_close(out["dc_gain"] * out["closed_loop_bandwidth"], gbw, 1e-6, 0.0);
    }
}

#[test]
fn bandwidth_rejects_every_scalar_and_minimum_stability_guard() {
    for (key, value) in [
        ("r_ground", 0.0), ("r_ground", -1.0), ("r_feedback", -1.0),
        ("open_loop_gain", 1.0), ("open_loop_gain", 0.5),
        ("gain_bandwidth", 0.0), ("gain_bandwidth", -1.0),
        ("minimum_stable_noise_gain", 0.999),
        ("minimum_stable_noise_gain", 10.001),
    ] {
        let mut inputs = BANDWIDTH;
        inputs.iter_mut().find(|entry| entry.0 == key).unwrap().1 = value;
        assert!(calculate("opamps.dominant_pole_gain_bandwidth", &inputs).is_err(), "accepted {key}={value}");
    }
}

#[test]
fn noninverting_tiny_signed_unity_signal_survives_nonzero_reference() {
    // Exact follower identity: Rf=0 means Vo=Vin irrespective of Vref.
    // These amplitudes are representable, not physically zero; no abs floor.
    for vin in [-1e-17, 1e-17, 1e-12] {
        let out = calculate("opamps.noninverting_stage", &[
            ("r_ground", 10000.0), ("r_feedback", 0.0), ("v_in", vin),
            ("v_reference", 1.0), ("v_out_min", -10.0), ("v_out_max", 10.0),
            ("v_cm_min", -10.0), ("v_cm_max", 10.0),
        ]).unwrap();
        assert_close(out["gain"], 1.0, 1e-6, 0.0);
        assert_close(out["v_out"], vin, 1e-6, 0.0);
    }
}

#[test]
fn difference_tiny_signed_reference_survives_balanced_sources() {
    // Matched k=1 gives Vo=Vref+(V2-V1). Equal 1 V sources cancel,
    // but the real, representable reference contribution must remain.
    for reference in [-1e-17, 0.0, 1e-17] {
        let out = calculate("opamps.difference_ratio_error", &[
            ("r_one", 10000.0), ("r_two", 10000.0), ("r_three", 10000.0),
            ("r_four", 10000.0), ("v_one", 1.0), ("v_two", 1.0),
            ("v_reference", reference), ("v_out_min", -10.0), ("v_out_max", 10.0),
            ("v_cm_min", -10.0), ("v_cm_max", 10.0),
        ]).unwrap();
        assert_close(out["differential_gain"], 1.0, 1e-6, 0.0);
        assert_close(out["common_mode_gain"], 0.0, 0.0, 0.0);
        // 0.5 +/- 5e-18 V rounds to 0.5 in binary64; output must be
        // computed without subtracting those rounded absolute pin levels.
        assert_close(out["pin_voltage"], 0.5, 1e-6, 0.0);
        assert_close(out["v_out"], reference, 1e-6, 0.0);
    }
}

#[test]
fn all_opamp_models_reject_nonfinite_inputs_without_poisoning_next_request() {
    for (id, nominal) in [
        ("opamps.inverting_stage", INVERTING.as_slice()),
        ("opamps.noninverting_stage", NONINVERTING.as_slice()),
        ("opamps.difference_ratio_error", DIFFERENCE.as_slice()),
        ("opamps.weighted_summing_stage", SUMMING.as_slice()),
        ("opamps.integrator_constant_input", INTEGRATOR.as_slice()),
        ("opamps.differentiator_ramp", DIFFERENTIATOR.as_slice()),
        ("opamps.sinusoidal_slew_requirement", SLEW.as_slice()),
        ("opamps.dominant_pole_gain_bandwidth", BANDWIDTH.as_slice()),
    ] {
        for index in 0..nominal.len() {
            for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                let mut inputs = nominal.to_vec();
                inputs[index].1 = invalid;
                assert!(calculate(id, &inputs).is_err(),
                    "accepted nonfinite input {} for {id}", nominal[index].0);
            }
        }
        // No shared mutable model/input state may survive rejected calls.
        assert!(calculate(id, nominal).is_ok(), "invalid-input recovery failed for {id}");
    }
}
