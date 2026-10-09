mod common;

use common::{assert_close, calculate};
use std::f64::consts::{PI, TAU};

// Independent non-default vectors: phase rotation, complex feedback algebra,
// lead transfer reconstruction, polynomial coefficients, event timing and Routh.
// No catalog examples or expression strings are used as numerical oracles.
const DELAY: &[(&str, f64)] = &[
    ("crossover_frequency", 125.0), ("initial_phase_margin", 90.0),
    ("target_phase_margin", 36.0), ("added_delay", 0.0004),
];
const SENSITIVITY: &[(&str, f64)] = &[
    ("loop_real", 2.0), ("loop_imaginary", -4.0),
    ("disturbance_amplitude", 0.025), ("noise_amplitude", 0.003),
];
const LEAD: &[(&str, f64)] = &[
    ("phase_boost", 36.86989764584402), ("crossover_frequency", 800.0),
    ("plant_magnitude", 0.25),
];
const PI_INPUTS: &[(&str, f64)] = &[
    ("plant_gain", 4.0), ("plant_time_constant", 0.125),
    ("natural_frequency", 16.0), ("damping_ratio", 0.625),
];
const SLOPE: &[(&str, f64)] = &[
    ("rising_slope", 20000.0), ("falling_slope", 60000.0),
    ("ramp_slope", 40000.0), ("switching_period", 0.00002),
    ("initial_perturbation", -0.006),
];
const ROUTH: &[(&str, f64)] = &[
    ("plant_gain", 5.0), ("plant_time_constant", 0.02),
    ("sensor_time_constant", 0.08), ("integral_gain", 5.0),
];

#[test]
fn six_independent_nondefault_complete_output_maps() {
    // L=2-j4 gives 1+L=3-j4, S=(3+j4)/25, T=(22-j4)/25.
    // Lead alpha=1/4 gives sin(phi)=3/5, corners 400/1600 Hz, gain 2.
    // PI denominator is .125s^2+2.5s+32, normalized to s^2+20s+256.
    // Current switching-time shift is +100 ns, hence next delta is +2 mV.
    // Cubic .0016s^3+.1s^2+s+25 has first-column reserve .6.
    let cases: &[(&str, &[(&str, f64)], &[(&str, f64)])] = &[
        ("control.delay_phase_budget", DELAY, &[
            ("added_phase_lag", 18.0), ("remaining_phase_margin", 72.0),
            ("maximum_delay", 0.0012), ("delay_headroom", 0.0008),
        ]),
        ("control.loop_sensitivity", SENSITIVITY, &[
            ("return_difference", 5.0), ("sensitivity_magnitude", 0.2),
            ("complementary_magnitude", 20.0_f64.sqrt() / 5.0),
            ("disturbance_output", 0.005), ("noise_output", 0.003 * 20.0_f64.sqrt() / 5.0),
        ]),
        ("control.lead_crossover_synthesis", LEAD, &[
            ("pole_zero_ratio", 0.25), ("zero_frequency", 400.0),
            ("pole_frequency", 1600.0), ("dc_gain", 2.0),
            ("high_frequency_gain", 8.0), ("zero_time_constant", 1.0 / (800.0 * PI)),
        ]),
        ("control.pi_pole_placement", PI_INPUTS, &[
            ("proportional_gain", 0.375), ("integral_gain", 8.0),
            ("integral_time", 0.046875), ("zero_frequency", 32.0 / (3.0 * PI)),
        ]),
        ("control.peak_current_slope_compensation", SLOPE, &[
            ("sample_multiplier", -1.0 / 3.0), ("next_perturbation", 0.002),
            ("minimum_ramp_slope", 20000.0), ("ramp_amplitude", 0.8),
            ("stability_reserve", 2.0 / 3.0), ("steady_duty", 0.75),
        ]),
        ("control.integral_two_lag_stability", ROUTH, &[
            ("critical_integral_gain", 12.5), ("gain_factor_to_boundary", 2.5),
            ("routh_reserve", 0.6), ("critical_frequency", 12.5 / PI),
        ]),
    ];
    for &(id, inputs, expected) in cases {
        let out = calculate(id, inputs).unwrap();
        assert_eq!(out.len(), expected.len(), "{id}");
        for &(key, value) in expected {
            assert_close(out[key], value, 1e-10, 0.0);
        }
    }
}

#[test]
fn delay_zero_budget_boundary_and_signed_failure() {
    for (initial, target, delay, lag, remaining, maximum, headroom) in [
        (60.0, 45.0, 0.0, 0.0, 60.0, 1.0 / 24000.0, 1.0 / 24000.0),
        (60.0, 45.0, 1.0 / 24000.0, 15.0, 45.0, 1.0 / 24000.0, 0.0),
        (60.0, 45.0, 0.0002, 72.0, -12.0, 1.0 / 24000.0, -19.0 / 120000.0),
        (90.0, 90.0, 0.0001, 36.0, 54.0, 0.0, -0.0001),
        (180.0, 0.0, 0.0005, 180.0, 0.0, 0.0005, 0.0),
    ] {
        let out = calculate("control.delay_phase_budget", &[
            ("crossover_frequency", 1000.0), ("initial_phase_margin", initial),
            ("target_phase_margin", target), ("added_delay", delay),
        ]).unwrap();
        for (key, value) in [("added_phase_lag", lag), ("remaining_phase_margin", remaining),
            ("maximum_delay", maximum), ("delay_headroom", headroom)] {
            // Zero headroom subtracts two independently rounded time values.
            let absolute = if key == "delay_headroom" && value == 0.0 {
                8.0 * f64::EPSILON * maximum
            } else { 0.0 };
            assert_close(out[key], value, 1e-10, absolute);
        }
    }
}

#[test]
fn delay_frequency_scaling_retains_tiny_time_budget() {
    for scale in [1.0, 2.0, 1e100] {
        let out = calculate("control.delay_phase_budget", &[
            ("crossover_frequency", 125.0 * scale), ("initial_phase_margin", 90.0),
            ("target_phase_margin", 36.0), ("added_delay", 0.0004 / scale),
        ]).unwrap();
        assert_close(out["added_phase_lag"], 18.0, 1e-10, 0.0);
        assert_close(out["remaining_phase_margin"], 72.0, 1e-10, 0.0);
        assert_close(out["maximum_delay"], 0.0012 / scale, 1e-10, 0.0);
        assert_close(out["delay_headroom"], 0.0008 / scale, 1e-10, 0.0);
    }
}

#[test]
fn delay_representable_rotation_avoids_intermediate_frequency_overflow() {
    let out = calculate("control.delay_phase_budget", &[
        ("crossover_frequency", 1e307), ("initial_phase_margin", 180.0),
        ("target_phase_margin", 0.0), ("added_delay", 1e-307),
    ]).unwrap();
    // One full period is 360 degrees even when 360*f is unrepresentable.
    for (key, value) in [("added_phase_lag", 360.0), ("remaining_phase_margin", -180.0),
        ("maximum_delay", 5e-308), ("delay_headroom", -5e-308)] {
        assert_close(out[key], value, 1e-10, 0.0);
    }
}

#[test]
fn sensitivity_conjugation_and_complex_injection_identity() {
    for imaginary in [-4.0, 4.0] {
        let out = calculate("control.loop_sensitivity", &[
            ("loop_real", 2.0), ("loop_imaginary", imaginary),
            ("disturbance_amplitude", 0.025), ("noise_amplitude", 0.003),
        ]).unwrap();
        let s_real: f64 = 3.0 / 25.0;
        let s_imag: f64 = -imaginary / 25.0;
        let t_real: f64 = 22.0 / 25.0;
        let t_imag: f64 = imaginary / 25.0;
        assert_close(out["return_difference"], 5.0, 1e-10, 0.0);
        assert_close(out["sensitivity_magnitude"], s_real.hypot(s_imag), 1e-10, 0.0);
        assert_close(out["complementary_magnitude"], t_real.hypot(t_imag), 1e-10, 0.0);
        assert_close(out["disturbance_output"], 0.005, 1e-10, 0.0);
        assert_close(out["noise_output"], 0.003 * t_real.hypot(t_imag), 1e-10, 0.0);
        assert!(out["sensitivity_magnitude"] + out["complementary_magnitude"] > 1.0);
    }
}

#[test]
fn sensitivity_zero_loop_positive_loop_and_independent_zero_sources() {
    for (real, distance, s, t) in [(0.0, 1.0, 1.0, 0.0), (9.0, 10.0, 0.1, 0.9)] {
        for (disturbance, noise) in [(0.01, 0.001), (0.0, 0.001), (0.01, 0.0), (0.0, 0.0)] {
            let out = calculate("control.loop_sensitivity", &[
                ("loop_real", real), ("loop_imaginary", 0.0),
                ("disturbance_amplitude", disturbance), ("noise_amplitude", noise),
            ]).unwrap();
            for (key, value) in [("return_difference", distance), ("sensitivity_magnitude", s),
                ("complementary_magnitude", t), ("disturbance_output", disturbance * s),
                ("noise_output", noise * t)] {
                assert_close(out[key], value, 1e-10, 0.0);
            }
        }
    }
}

#[test]
fn sensitivity_near_singularity_preserves_amplification_and_tiny_sources() {
    for imaginary in [0.001, 1e-100] {
        let out = calculate("control.loop_sensitivity", &[
            ("loop_real", -1.0), ("loop_imaginary", imaginary),
            ("disturbance_amplitude", 1e-120), ("noise_amplitude", 2e-120),
        ]).unwrap();
        let t: f64 = 1.0_f64.hypot(imaginary) / imaginary;
        for (key, value) in [("return_difference", imaginary),
            ("sensitivity_magnitude", 1.0 / imaginary), ("complementary_magnitude", t),
            ("disturbance_output", 1e-120 / imaginary), ("noise_output", 2e-120 * t)] {
            assert_close(out[key], value, 1e-10, 0.0);
        }
    }
}

#[test]
fn sensitivity_hypot_handles_large_finite_components() {
    let out = calculate("control.loop_sensitivity", &[
        ("loop_real", 3e200), ("loop_imaginary", -4e200),
        ("disturbance_amplitude", 1.0), ("noise_amplitude", 0.002),
    ]).unwrap();
    // The unity term is below one ulp; scaled 3-4-5 geometry remains exact to tolerance.
    for (key, value) in [("return_difference", 5e200), ("sensitivity_magnitude", 2e-201),
        ("complementary_magnitude", 1.0), ("disturbance_output", 2e-201), ("noise_output", 0.002)] {
        assert_close(out[key], value, 1e-10, 0.0);
    }
}

#[test]
fn lead_reconstructs_transfer_at_crossover_and_scales_independently() {
    for (fc, magnitude, zero, pole, dc, high) in [
        (800.0, 0.25, 400.0, 1600.0, 2.0, 8.0),
        (1600.0, 0.25, 800.0, 3200.0, 2.0, 8.0),
        (800.0, 0.5, 400.0, 1600.0, 1.0, 4.0),
    ] {
        let out = calculate("control.lead_crossover_synthesis", &[
            ("phase_boost", 36.86989764584402), ("crossover_frequency", fc),
            ("plant_magnitude", magnitude),
        ]).unwrap();
        for (key, value) in [("pole_zero_ratio", 0.25), ("zero_frequency", zero),
            ("pole_frequency", pole), ("dc_gain", dc), ("high_frequency_gain", high),
            ("zero_time_constant", 1.0 / (TAU * zero))] {
            assert_close(out[key], value, 1e-10, 0.0);
        }
        let numerator_imag = fc / out["zero_frequency"];
        let denominator_imag = fc / out["pole_frequency"];
        let real = out["dc_gain"] * (1.0 + numerator_imag * denominator_imag)
            / (1.0 + denominator_imag * denominator_imag);
        let imag = out["dc_gain"] * (numerator_imag - denominator_imag)
            / (1.0 + denominator_imag * denominator_imag);
        assert_close(magnitude * real.hypot(imag), 1.0, 1e-10, 0.0);
        assert_close(imag.atan2(real).to_degrees(), 36.86989764584402, 1e-10, 0.0);
        assert_close(out["zero_frequency"] * out["pole_frequency"], fc * fc, 1e-10, 0.0);
    }
}

#[test]
fn lead_small_boost_and_near_ninety_use_half_angle_oracle() {
    for boost in [1e-7_f64, 89.999, 89.999999] {
        // sqrt(alpha)=tan((90-phi)/2), independently avoiding 1-sin(phi).
        let ratio: f64 = ((90.0 - boost).to_radians() / 2.0).tan();
        let out = calculate("control.lead_crossover_synthesis", &[
            ("phase_boost", boost), ("crossover_frequency", 800.0), ("plant_magnitude", 0.25),
        ]).unwrap();
        for (key, value) in [("pole_zero_ratio", ratio * ratio), ("zero_frequency", 800.0 * ratio),
            ("pole_frequency", 800.0 / ratio), ("dc_gain", 4.0 * ratio),
            ("high_frequency_gain", 4.0 / ratio), ("zero_time_constant", 1.0 / (1600.0 * PI * ratio))] {
            // Near 90 degrees, equivalent angle reductions differ by O(epsilon/(90-phi)).
            assert_close(out[key], value, 1e-7, 0.0);
        }
        assert!(out["zero_frequency"] < 800.0 && out["pole_frequency"] > 800.0);
        assert_close(out["zero_frequency"] * out["pole_frequency"], 640000.0, 1e-10, 0.0);
    }
}

#[test]
fn pi_gain_scaling_critical_and_overdamped_polynomials() {
    for (gain, damping, kp, ki, time) in [
        (4.0, 0.625, 0.375, 8.0, 0.046875),
        (8.0, 0.625, 0.1875, 4.0, 0.046875),
        (4.0, 1.0, 0.75, 8.0, 0.09375),
        (4.0, 2.0, 1.75, 8.0, 0.21875),
    ] {
        let out = calculate("control.pi_pole_placement", &[
            ("plant_gain", gain), ("plant_time_constant", 0.125),
            ("natural_frequency", 16.0), ("damping_ratio", damping),
        ]).unwrap();
        for (key, value) in [("proportional_gain", kp), ("integral_gain", ki),
            ("integral_time", time), ("zero_frequency", 1.0 / (TAU * time))] {
            assert_close(out[key], value, 1e-10, 0.0);
        }
        assert_close((1.0 + gain * out["proportional_gain"]) / 0.125, 32.0 * damping, 1e-10, 0.0);
        assert_close(gain * out["integral_gain"] / 0.125, 256.0, 1e-10, 0.0);
        if damping == 1.0 {
            // The normalized polynomial must vanish at the repeated pole s=-16.
            let residual = 256.0 - 16.0 * (1.0 + gain * out["proportional_gain"]) / 0.125
                + gain * out["integral_gain"] / 0.125;
            assert_close(residual, 0.0, 0.0, 32.0 * f64::EPSILON * 512.0);
        }
    }
}

#[test]
fn pi_one_ulp_above_positive_gain_boundary_is_not_clamped() {
    let out = calculate("control.pi_pole_placement", &[
        ("plant_gain", 1.0), ("plant_time_constant", 1.0),
        ("natural_frequency", 1.0), ("damping_ratio", 0.5000000000000001),
    ]).unwrap();
    for (key, value) in [("proportional_gain", f64::EPSILON), ("integral_gain", 1.0),
        ("integral_time", f64::EPSILON), ("zero_frequency", 1.0 / (TAU * f64::EPSILON))] {
        assert_close(out[key], value, 1e-10, 0.0);
    }
}

#[test]
fn pi_representable_coefficients_avoid_intermediate_double_frequency_overflow() {
    let out = calculate("control.pi_pole_placement", &[
        ("plant_gain", 1e308), ("plant_time_constant", 1e-308),
        ("natural_frequency", 1e308), ("damping_ratio", 1.0),
    ]).unwrap();
    // wn*tau=1, so K*Kp=1 and Ki=1; every requested output is representable.
    for (key, value) in [("proportional_gain", 1e-308), ("integral_gain", 1.0),
        ("integral_time", 1e-308), ("zero_frequency", 1e308 / TAU)] {
        assert_close(out[key], value, 1e-10, 0.0);
    }
}

#[test]
fn peak_current_unstable_marginal_deadbeat_and_positive_contraction() {
    for (ramp, multiplier, next, reserve, amplitude) in [
        (0.0, -3.0, 0.018, -2.0, 0.0),
        (20000.0, -1.0, 0.006, 0.0, 0.4),
        (60000.0, 0.0, 0.0, 1.0, 1.2),
        (140000.0, 0.5, -0.003, 0.5, 2.8),
    ] {
        let out = calculate("control.peak_current_slope_compensation", &[
            ("rising_slope", 20000.0), ("falling_slope", 60000.0), ("ramp_slope", ramp),
            ("switching_period", 0.00002), ("initial_perturbation", -0.006),
        ]).unwrap();
        for (key, value) in [("sample_multiplier", multiplier), ("next_perturbation", next),
            ("minimum_ramp_slope", 20000.0), ("ramp_amplitude", amplitude),
            ("stability_reserve", reserve), ("steady_duty", 0.75)] {
            assert_close(out[key], value, 1e-10, 0.0);
        }
        // Comparator crossing geometry, before simplifying to the multiplier formula.
        let time_shift = 0.006 / (20000.0 + ramp);
        let event_next = -0.006 + 80000.0 * time_shift;
        assert_close(out["next_perturbation"], event_next, 1e-10, 16.0 * f64::EPSILON * 0.006);
    }
}

#[test]
fn peak_current_zero_ramp_distinguishes_subhalf_and_half_duty() {
    for (rise, fall, multiplier, next, reserve, duty) in [
        (60000.0, 20000.0, -1.0 / 3.0, -0.002, 2.0 / 3.0, 0.25),
        (20000.0, 20000.0, -1.0, -0.006, 0.0, 0.5),
    ] {
        let out = calculate("control.peak_current_slope_compensation", &[
            ("rising_slope", rise), ("falling_slope", fall), ("ramp_slope", 0.0),
            ("switching_period", 0.00002), ("initial_perturbation", 0.006),
        ]).unwrap();
        for (key, value) in [("sample_multiplier", multiplier), ("next_perturbation", next),
            ("minimum_ramp_slope", 0.0), ("ramp_amplitude", 0.0),
            ("stability_reserve", reserve), ("steady_duty", duty)] {
            assert_close(out[key], value, 1e-10, 0.0);
        }
        assert_close(rise * out["steady_duty"], fall * (1.0 - out["steady_duty"]), 1e-10, 0.0);
    }
}

#[test]
fn peak_current_signed_zero_tiny_perturbations_and_period_scaling() {
    for perturbation in [-0.006, 0.0, 0.006, 3e-200] {
        for (period, amplitude) in [(0.00002, 0.8), (0.00004, 1.6)] {
            let out = calculate("control.peak_current_slope_compensation", &[
                ("rising_slope", 20000.0), ("falling_slope", 60000.0), ("ramp_slope", 40000.0),
                ("switching_period", period), ("initial_perturbation", perturbation),
            ]).unwrap();
            for (key, value) in [("sample_multiplier", -1.0 / 3.0), ("next_perturbation", -perturbation / 3.0),
                ("minimum_ramp_slope", 20000.0), ("ramp_amplitude", amplitude),
                ("stability_reserve", 2.0 / 3.0), ("steady_duty", 0.75)] {
                assert_close(out[key], value, 1e-10, 0.0);
            }
        }
    }
}

#[test]
fn peak_current_common_slope_scaling_avoids_overflowing_sum() {
    for scale in [1.0, 1e308] {
        let out = calculate("control.peak_current_slope_compensation", &[
            ("rising_slope", scale), ("falling_slope", 0.5 * scale), ("ramp_slope", scale),
            ("switching_period", 1.0 / scale), ("initial_perturbation", 0.001),
        ]).unwrap();
        // Common scaling cannot turn a quarter-cycle contraction into deadbeat control.
        for (key, value) in [("sample_multiplier", 0.25), ("next_perturbation", 0.00025),
            ("minimum_ramp_slope", 0.0), ("ramp_amplitude", 1.0),
            ("stability_reserve", 0.75), ("steady_duty", 1.0 / 3.0)] {
            assert_close(out[key], value, 1e-10, 0.0);
        }
    }
}

#[test]
fn two_lag_routh_column_crosses_strict_boundary_without_rejection() {
    for (ki, factor, reserve) in [(5.0, 2.5, 0.6), (12.5, 1.0, 0.0), (25.0, 0.5, -1.0)] {
        let out = calculate("control.integral_two_lag_stability", &[
            ("plant_gain", 5.0), ("plant_time_constant", 0.02),
            ("sensor_time_constant", 0.08), ("integral_gain", ki),
        ]).unwrap();
        for (key, value) in [("critical_integral_gain", 12.5), ("gain_factor_to_boundary", factor),
            ("routh_reserve", reserve), ("critical_frequency", 12.5 / PI)] {
            assert_close(out[key], value, 1e-10, 0.0);
        }
        // First Routh column of .0016s^3+.1s^2+s+5Ki.
        assert_close(out["routh_reserve"], (0.1 - 0.0016 * 5.0 * ki) / 0.1,
            1e-10, 16.0 * f64::EPSILON);
        let omega = TAU * out["critical_frequency"];
        assert_close(omega, 25.0, 1e-10, 0.0);
        assert_close(0.0016 * omega * omega, 1.0, 1e-10, 0.0);
    }
}

#[test]
fn two_lag_exchange_equal_poles_and_fast_sensor_limit() {
    for (plant_time, sensor_time, critical, frequency) in [
        (0.02, 0.08, 12.5, 12.5 / PI),
        (0.08, 0.02, 12.5, 12.5 / PI),
        (0.02, 0.02, 20.0, 25.0 / PI),
        (0.02, 0.0002, 1010.0, 250.0 / PI),
    ] {
        let out = calculate("control.integral_two_lag_stability", &[
            ("plant_gain", 5.0), ("plant_time_constant", plant_time),
            ("sensor_time_constant", sensor_time), ("integral_gain", 5.0),
        ]).unwrap();
        for (key, value) in [("critical_integral_gain", critical), ("gain_factor_to_boundary", critical / 5.0),
            ("routh_reserve", (critical - 5.0) / critical), ("critical_frequency", frequency)] {
            assert_close(out[key], value, 1e-10, 0.0);
        }
    }
}

#[test]
fn two_lag_near_boundary_retains_signed_small_reserve() {
    for (ki, reserve) in [(1.0 - f64::EPSILON / 2.0, f64::EPSILON / 2.0),
        (1.0 + f64::EPSILON, -f64::EPSILON)] {
        let out = calculate("control.integral_two_lag_stability", &[
            ("plant_gain", 2.0), ("plant_time_constant", 1.0),
            ("sensor_time_constant", 1.0), ("integral_gain", ki),
        ]).unwrap();
        assert_close(out["critical_integral_gain"], 1.0, 1e-10, 0.0);
        assert_close(out["gain_factor_to_boundary"], 1.0 / ki, 1e-10, 0.0);
        assert_close(out["routh_reserve"], reserve, 1e-10, 0.0);
        assert_close(out["critical_frequency"], 1.0 / TAU, 1e-10, 0.0);
    }
}

#[test]
fn two_lag_equal_extreme_times_avoid_product_underflow_and_overflow() {
    for time in [1e-200, 1e200] {
        let out = calculate("control.integral_two_lag_stability", &[
            ("plant_gain", 2.0), ("plant_time_constant", time),
            ("sensor_time_constant", time), ("integral_gain", 0.5 / time),
        ]).unwrap();
        for (key, value) in [("critical_integral_gain", 1.0 / time), ("gain_factor_to_boundary", 2.0),
            ("routh_reserve", 0.5), ("critical_frequency", (1.0 / time) / TAU)] {
            assert_close(out[key], value, 1e-10, 0.0);
        }
    }
}

#[test]
fn two_lag_gain_scaling_avoids_overflowing_reciprocal_sum() {
    let out = calculate("control.integral_two_lag_stability", &[
        ("plant_gain", 4.0), ("plant_time_constant", 1e-308),
        ("sensor_time_constant", 1e-308), ("integral_gain", 1e307),
    ]).unwrap();
    // Equal poles: critical gain is 2/(K*tau), not an unrepresentable reciprocal sum.
    for (key, value) in [("critical_integral_gain", 5e307), ("gain_factor_to_boundary", 5.0),
        ("routh_reserve", 0.8), ("critical_frequency", 1e308 / TAU)] {
        assert_close(out[key], value, 1e-10, 0.0);
    }
}

#[test]
fn all_scalar_domain_errors_recover_on_same_engine() {
    let cases: &[(&str, &[(&str, f64)], &[(&str, f64)], &str, f64)] = &[
        ("control.delay_phase_budget", DELAY, &[
            ("crossover_frequency", 0.0), ("crossover_frequency", -1.0),
            ("initial_phase_margin", 0.0), ("initial_phase_margin", -1.0),
            ("initial_phase_margin", 180.0001), ("target_phase_margin", -1.0),
            ("target_phase_margin", 181.0), ("added_delay", -1e-9),
        ], "added_phase_lag", 18.0),
        ("control.loop_sensitivity", SENSITIVITY, &[
            ("disturbance_amplitude", -1e-12), ("noise_amplitude", -1e-12),
        ], "return_difference", 5.0),
        ("control.lead_crossover_synthesis", LEAD, &[
            ("phase_boost", 0.0), ("phase_boost", -1.0), ("phase_boost", 90.0), ("phase_boost", 91.0),
            ("crossover_frequency", 0.0), ("crossover_frequency", -1.0),
            ("plant_magnitude", 0.0), ("plant_magnitude", -1.0),
        ], "pole_zero_ratio", 0.25),
        ("control.pi_pole_placement", PI_INPUTS, &[
            ("plant_gain", 0.0), ("plant_gain", -1.0), ("plant_time_constant", 0.0),
            ("plant_time_constant", -1.0), ("natural_frequency", 0.0), ("natural_frequency", -1.0),
            ("damping_ratio", 0.0), ("damping_ratio", -1.0),
        ], "proportional_gain", 0.375),
        ("control.peak_current_slope_compensation", SLOPE, &[
            ("rising_slope", 0.0), ("rising_slope", -1.0), ("falling_slope", 0.0),
            ("falling_slope", -1.0), ("ramp_slope", -1.0),
            ("switching_period", 0.0), ("switching_period", -1.0),
        ], "next_perturbation", 0.002),
        ("control.integral_two_lag_stability", ROUTH, &[
            ("plant_gain", 0.0), ("plant_gain", -1.0), ("plant_time_constant", 0.0),
            ("plant_time_constant", -1.0), ("sensor_time_constant", 0.0),
            ("sensor_time_constant", -1.0), ("integral_gain", 0.0), ("integral_gain", -1.0),
        ], "routh_reserve", 0.6),
    ];
    for &(id, valid, invalid, recovery_key, expected) in cases {
        for &(key, value) in invalid {
            let mut inputs = valid.to_vec();
            inputs.iter_mut().find(|(name, _)| *name == key).unwrap().1 = value;
            assert!(calculate(id, &inputs).is_err(), "accepted {id} {key}={value}");
            let recovered = calculate(id, valid).unwrap();
            assert_close(recovered[recovery_key], expected, 1e-10, 0.0);
        }
    }
}

#[test]
fn coupled_boundaries_reject_then_valid_inputs_recover() {
    let mut delay = DELAY.to_vec();
    delay[2].1 = 90.0001;
    assert!(calculate("control.delay_phase_budget", &delay).is_err());
    assert_close(calculate("control.delay_phase_budget", DELAY).unwrap()["delay_headroom"], 0.0008, 1e-10, 0.0);
    for imaginary in [0.0, -0.0] {
        let mut inputs = SENSITIVITY.to_vec();
        inputs[0].1 = -1.0;
        inputs[1].1 = imaginary;
        assert!(calculate("control.loop_sensitivity", &inputs).is_err());
        assert_close(calculate("control.loop_sensitivity", SENSITIVITY).unwrap()["disturbance_output"], 0.005, 1e-10, 0.0);
    }
    for damping in [0.25, 0.125] {
        let mut inputs = PI_INPUTS.to_vec();
        inputs[3].1 = damping;
        assert!(calculate("control.pi_pole_placement", &inputs).is_err());
        assert_close(calculate("control.pi_pole_placement", PI_INPUTS).unwrap()["integral_time"], 0.046875, 1e-10, 0.0);
    }
}

#[test]
fn every_nonfinite_input_rejects_and_recovers() {
    let cases: &[(&str, &[(&str, f64)], &str, f64)] = &[
        ("control.delay_phase_budget", DELAY, "remaining_phase_margin", 72.0),
        ("control.loop_sensitivity", SENSITIVITY, "return_difference", 5.0),
        ("control.lead_crossover_synthesis", LEAD, "high_frequency_gain", 8.0),
        ("control.pi_pole_placement", PI_INPUTS, "integral_gain", 8.0),
        ("control.peak_current_slope_compensation", SLOPE, "next_perturbation", 0.002),
        ("control.integral_two_lag_stability", ROUTH, "critical_integral_gain", 12.5),
    ];
    for &(id, valid, key, expected) in cases {
        for index in 0..valid.len() {
            for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                let mut inputs = valid.to_vec();
                inputs[index].1 = bad;
                assert!(calculate(id, &inputs).is_err(), "accepted {id} {}={bad}", inputs[index].0);
                assert_close(calculate(id, valid).unwrap()[key], expected, 1e-10, 0.0);
            }
        }
    }
}

#[test]
fn peak_current_tiny_representable_duty_avoids_overflowing_slope_ratio() {
    let out = calculate("control.peak_current_slope_compensation", &[
        ("rising_slope", 1e200), ("falling_slope", 1e-109), ("ramp_slope", 0.0),
        ("switching_period", 1e-5), ("initial_perturbation", 1.0),
    ]).unwrap();
    // D/(1-D)=m2/m1=1e-309; its O(1e-618) correction cannot affect binary64.
    for (key, value) in [("sample_multiplier", -1e-309), ("next_perturbation", -1e-309),
        ("minimum_ramp_slope", 0.0), ("ramp_amplitude", 0.0),
        ("stability_reserve", 1.0), ("steady_duty", 1e-309)] {
        assert_close(out[key], value, 1e-10, 0.0);
    }
}
