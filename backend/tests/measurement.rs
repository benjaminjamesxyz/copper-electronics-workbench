mod common;

use common::{assert_close, calculate};
use std::f64::consts::{PI, SQRT_2};

const DMM: &[(&str, f64)] = &[
    ("reading", -1.25), ("range_value", 5.0),
    ("reading_percent", 0.04), ("range_percent", 0.006),
];
const DELTA: &[(&str, f64)] = &[
    ("test_current", 0.02), ("sample_interval", 0.25),
    ("voltage_first", 0.004875), ("voltage_second", -0.0052),
    ("voltage_third", 0.004725),
];
const PROBE: &[(&str, f64)] = &[
    ("source_voltage", 2.5), ("source_resistance", 2000.0),
    ("probe_resistance", 4e6), ("probe_capacitance", 8e-12),
    ("scope_resistance", 1e6), ("scope_capacitance", 64e-12),
    ("frequency", 1e5),
];
const APERTURE: &[(&str, f64)] = &[
    ("frequency", 75.0), ("aperture_time", 0.02), ("input_peak", 0.6),
];
const JITTER: &[(&str, f64)] = &[
    ("frequency", 3e6), ("timing_jitter", 7e-12), ("input_peak", 0.8),
];
const RATIO: &[(&str, f64)] = &[
    ("numerator_voltage", 12.0), ("denominator_voltage", 4.0),
    ("numerator_uncertainty", 0.036), ("denominator_uncertainty", 0.008),
    ("correlation", -1.0 / 3.0), ("coverage_factor", 2.5),
];

#[test]
fn dmm_signed_nondefault_complete_specification_bound() {
    // 400 ppm of 1.25 V plus 60 ppm of 5 V: arithmetic sum, not RSS.
    for sign in [-1.0, 1.0] {
        let mut inputs = DMM.to_vec();
        inputs[0].1 = sign * 1.25;
        let out = calculate("measurement.dmm_accuracy_bound", &inputs).unwrap();
        assert_eq!(out.len(), 5);
        for (key, expected) in [
            ("reading_error_bound", 0.0005), ("range_error_bound", 0.0003),
            ("total_error_bound", 0.0008),
            ("lower_bound", sign * 1.25 - 0.0008),
            ("upper_bound", sign * 1.25 + 0.0008),
        ] { assert_close(out[key], expected, 1e-12, 0.0); }
    }
}

#[test]
fn dmm_zero_reading_coefficients_and_signed_full_scale() {
    for (reading, a, b, er, eb) in [
        (0.0, 0.04, 0.006, 0.0, 0.0003),
        (-5.0, 0.0, 0.0, 0.0, 0.0), (5.0, 0.0, 0.0, 0.0, 0.0),
        (5.0, 0.04, 0.006, 0.002, 0.0003),
    ] {
        let out = calculate("measurement.dmm_accuracy_bound", &[
            ("reading", reading), ("range_value", 5.0),
            ("reading_percent", a), ("range_percent", b),
        ]).unwrap();
        for (key, expected) in [("reading_error_bound", er), ("range_error_bound", eb),
            ("total_error_bound", er + eb), ("lower_bound", reading - er - eb),
            ("upper_bound", reading + er + eb)] {
            assert_close(out[key], expected, 1e-12, 0.0);
        }
    }
}

#[test]
fn delta_forward_experiment_offset_drift_and_reversal() {
    // Generate meter readings from a 0.25-ohm DUT, rather than invert catalog algebra.
    for (emf, drift) in [(-0.0002, -0.0003), (0.0038, -0.0003),
        (-0.0002, 0.0003), (-0.0002, 0.0)] {
        let resistive = 0.02 * 0.25;
        let out = calculate("measurement.delta_resistance", &[
            ("test_current", 0.02), ("sample_interval", 0.25),
            ("voltage_first", resistive + emf - drift * 0.25),
            ("voltage_second", -resistive + emf),
            ("voltage_third", resistive + emf + drift * 0.25),
        ]).unwrap();
        assert_eq!(out.len(), 4);
        for (key, expected) in [("resistive_voltage", 0.005), ("resistance", 0.25),
            ("midpoint_emf", emf), ("emf_drift_rate", drift)] {
            assert_close(out[key], expected, 1e-11, 0.0);
        }
    }
}

#[test]
fn delta_zero_resistance_and_curved_drift_bias() {
    // Binary-exact values isolate the physical zero and q*dt^2/2 bias from rounding.
    for (resistance, curvature) in [(0.0, 0.0), (0.25, 0.0), (0.25, 0.125)] {
        let current = 0.125;
        let dt = 0.25;
        let emf = -0.0625;
        let drift = -0.03125;
        let bias = curvature * dt * dt / 2.0;
        let out = calculate("measurement.delta_resistance", &[
            ("test_current", current), ("sample_interval", dt),
            ("voltage_first", current * resistance + emf - drift * dt + curvature * dt * dt),
            ("voltage_second", -current * resistance + emf),
            ("voltage_third", current * resistance + emf + drift * dt + curvature * dt * dt),
        ]).unwrap();
        for (key, expected) in [("resistive_voltage", current * resistance + bias),
            ("resistance", resistance + bias / current), ("midpoint_emf", emf + bias),
            ("emf_drift_rate", drift)] {
            assert_close(out[key], expected, 1e-12, 0.0);
        }
    }
    let out = calculate("measurement.delta_resistance", &[
        ("test_current", 0.01), ("sample_interval", 0.1),
        ("voltage_first", 12e-6), ("voltage_second", 14e-6), ("voltage_third", 16e-6),
    ]).unwrap();
    // Four arithmetic operations on 16-uV readings permit a few ulps at zero.
    assert_close(out["resistive_voltage"], 0.0, 0.0, 1e-20);
    assert_close(out["resistance"], 0.0, 0.0, 1e-18);
    assert_close(out["midpoint_emf"], 14e-6, 1e-12, 0.0);
    assert_close(out["emf_drift_rate"], 20e-6, 1e-12, 0.0);
}

#[test]
fn probe_unmatched_complete_complex_impedance_oracle() {
    // Independent series-current solution: each arm is the reciprocal of G+jwC.
    for cp in [0.0, 8e-12, 16e-12, 32e-12] {
        let mut inputs = PROBE.to_vec();
        inputs[3].1 = cp;
        let out = calculate("measurement.compensated_probe_loading", &inputs).unwrap();
        let omega = 2.0 * PI * 1e5;
        let gp: f64 = 1.0 / 4e6;
        let gi: f64 = 1.0 / 1e6;
        let bp = omega * cp;
        let bi = omega * 64e-12;
        let zp_re = gp / (gp * gp + bp * bp);
        let zp_im = -bp / (gp * gp + bp * bp);
        let zi_re = gi / (gi * gi + bi * bi);
        let zi_im = -bi / (gi * gi + bi * bi);
        let gain = zi_re.hypot(zi_im) / (2000.0 + zp_re + zi_re).hypot(zp_im + zi_im);
        assert_eq!(out.len(), 4);
        for (key, expected) in [("matched_probe_capacitance", 16e-12),
            ("dc_gain", 1.0 / 5.002), ("ac_gain", gain), ("scope_voltage", 2.5 * gain)] {
            assert_close(out[key], expected, 1e-12, 0.0);
        }
    }
}

#[test]
fn probe_matched_arms_still_load_finite_source() {
    for frequency in [0.0, 100.0, 1e5, 1e7] {
        for resistance in [0.0, 2000.0] {
            let out = calculate("measurement.compensated_probe_loading", &[
                ("source_voltage", 2.5), ("source_resistance", resistance),
                ("probe_resistance", 4e6), ("probe_capacitance", 16e-12),
                ("scope_resistance", 1e6), ("scope_capacitance", 64e-12),
                ("frequency", frequency),
            ]).unwrap();
            // Matching cancels one pole: H=Ri/(Rs+Rp+Ri+j*w*Rs*tau).
            let dc = 1e6 / (5e6 + resistance);
            let gain = 1e6 / (5e6 + resistance).hypot(2.0 * PI * frequency * resistance * 64e-6);
            for (key, expected) in [("matched_probe_capacitance", 16e-12),
                ("dc_gain", dc), ("ac_gain", gain), ("scope_voltage", 2.5 * gain)] {
                assert_close(out[key], expected, 1e-12, 0.0);
            }
        }
    }
}

#[test]
fn probe_zero_excitation_and_overcompensation_are_not_clamped() {
    for (cp, gain) in [(5e-12, 0.05263181693942273), (20e-12, 0.1818180179402501)] {
        for voltage in [0.0, 1.0] {
            let out = calculate("measurement.compensated_probe_loading", &[
                ("source_voltage", voltage), ("source_resistance", 0.0),
                ("probe_resistance", 9e6), ("probe_capacitance", cp),
                ("scope_resistance", 1e6), ("scope_capacitance", 90e-12),
                ("frequency", 1e6),
            ]).unwrap();
            for (key, expected) in [("matched_probe_capacitance", 10e-12),
                ("dc_gain", 0.1), ("ac_gain", gain), ("scope_voltage", voltage * gain)] {
                assert_close(out[key], expected, 1e-12, 0.0);
            }
        }
    }
}

#[test]
fn aperture_exact_integral_half_cycle_null_and_negative_lobe() {
    // Integrate a unit cosine symmetrically over 1/2, 1 and 3/2 cycles.
    for (frequency, gain) in [(25.0, 2.0 / PI), (50.0, 0.0), (75.0, -2.0 / (3.0 * PI))] {
        let out = calculate("measurement.rectangular_aperture_response", &[
            ("frequency", frequency), ("aperture_time", 0.02), ("input_peak", 0.6),
        ]).unwrap();
        assert_eq!(out.len(), 4);
        // sin(pi) has an argument-rounding residual; only mathematical nulls get a floor.
        let null_tolerance = if frequency == 50.0 { 1e-15 } else { 0.0 };
        assert_close(out["midpoint_gain"], gain, 1e-12, null_tolerance);
        assert_close(out["amplitude_ratio"], gain.abs(), 1e-12, null_tolerance);
        assert_close(out["measured_peak"], 0.6 * gain.abs(), 1e-12, 0.6 * null_tolerance);
        assert_close(out["effective_delay"], 0.01, 1e-12, 0.0);
    }
}

#[test]
fn aperture_dc_instantaneous_tiny_aperture_and_zero_amplitude() {
    for (frequency, time, amplitude) in [(0.0, 0.02, 0.6), (75.0, 0.0, 0.6),
        (0.0, 0.0, 0.6), (75.0, 1e-20, 0.6), (0.0, 0.02, 0.0)] {
        let out = calculate("measurement.rectangular_aperture_response", &[
            ("frequency", frequency), ("aperture_time", time), ("input_peak", amplitude),
        ]).unwrap();
        for (key, expected) in [("midpoint_gain", 1.0), ("amplitude_ratio", 1.0),
            ("measured_peak", amplitude), ("effective_delay", time / 2.0)] {
            assert_close(out[key], expected, 1e-12, 0.0);
        }
    }
    let mut inputs = APERTURE.to_vec();
    inputs[2].1 = 0.0;
    let out = calculate("measurement.rectangular_aperture_response", &inputs).unwrap();
    assert_close(out["midpoint_gain"], -2.0 / (3.0 * PI), 1e-12, 0.0);
    assert_close(out["amplitude_ratio"], 2.0 / (3.0 * PI), 1e-12, 0.0);
    assert_eq!(out["measured_peak"], 0.0);
    assert_eq!(out["effective_delay"], 0.01);
}

#[test]
fn jitter_rms_slope_and_independent_scaling() {
    for (f_scale, t_scale, a_scale) in [(1.0, 1.0, 1.0), (2.0, 1.0, 1.0),
        (1.0, 2.0, 1.0), (1.0, 1.0, 2.0)] {
        let out = calculate("measurement.sampling_jitter_snr", &[
            ("frequency", 3e6 * f_scale), ("timing_jitter", 7e-12 * t_scale),
            ("input_peak", 0.8 * a_scale),
        ]).unwrap();
        // RMS cosine slope times 7 ps; SNR comes from signal/error RMS ratio.
        let phase = 0.0001319468914507713 * f_scale * t_scale;
        let error = 7.464043336106054e-5 * f_scale * t_scale * a_scale;
        let signal = 0.8 * a_scale / SQRT_2;
        assert_eq!(out.len(), 3);
        assert_close(out["phase_jitter"], phase, 1e-12, 0.0);
        assert_close(out["error_rms"], error, 1e-12, 0.0);
        assert_close(out["jitter_snr"], 20.0 * (signal / error).log10(), 1e-12, 0.0);
    }
}

#[test]
fn jitter_phase_boundary_and_tiny_positive_jitter() {
    let frequency = 1.0 / (2.0 * PI);
    for jitter in [0.1, 0.1 * (1.0 - 1e-12), 1e-300] {
        let out = calculate("measurement.sampling_jitter_snr", &[
            ("frequency", frequency), ("timing_jitter", jitter), ("input_peak", SQRT_2),
        ]).unwrap();
        assert_close(out["phase_jitter"], jitter, 1e-12, 0.0);
        assert_close(out["error_rms"], jitter, 1e-12, 0.0);
        assert_close(out["jitter_snr"], -20.0 * jitter.log10(), 1e-12, 0.0);
    }
    assert!(calculate("measurement.sampling_jitter_snr", &[
        ("frequency", frequency), ("timing_jitter", 0.1 * (1.0 + 1e-12)),
        ("input_peak", SQRT_2),
    ]).is_err());
    let out = calculate("measurement.sampling_jitter_snr", JITTER).unwrap();
    assert_close(out["phase_jitter"], 0.0001319468914507713, 1e-12, 0.0);
    assert_close(out["error_rms"], 7.464043336106054e-5, 1e-12, 0.0);
    assert_close(out["jitter_snr"], 77.59201673815932, 1e-12, 0.0);
}

#[test]
fn jitter_preserves_representable_voltage_when_phase_underflows() {
    // Dimensional RMS slope is sqrt(2)*pi V/s; multiply by 1e-200 s last.
    // The phase itself is below binary64, but the voltage error is representable.
    let out = calculate("measurement.sampling_jitter_snr", &[
        ("frequency", 1e-200), ("timing_jitter", 1e-200), ("input_peak", 1e200),
    ]).unwrap();
    assert_eq!(out["phase_jitter"], 0.0);
    assert_close(out["error_rms"], 4.442882938158366e-200, 1e-12, 0.0);
    assert_close(out["jitter_snr"], 7984.036402632838, 1e-12, 0.0);
}

#[test]
fn ratio_signed_covariance_complete_nondefault_oracle() {
    // Sensitivities are 1/4 and -3/4. Independent variance is 153e-6 at rho=-1/3.
    for (rho, variance) in [(-1.0 / 3.0, 153e-6_f64), (0.0, 117e-6),
        (-1.0, 225e-6), (1.0, 9e-6)] {
        let mut inputs = RATIO.to_vec();
        inputs[4].1 = rho;
        let out = calculate("measurement.correlated_voltage_ratio", &inputs).unwrap();
        let standard = variance.sqrt();
        assert_eq!(out.len(), 4);
        for (key, expected) in [("voltage_ratio", 3.0),
            ("relative_standard_uncertainty", standard / 3.0),
            ("ratio_standard_uncertainty", standard), ("expanded_uncertainty", 2.5 * standard)] {
            assert_close(out[key], expected, 1e-12, 0.0);
        }
    }
}

#[test]
fn ratio_common_mode_cancellation_and_one_ulp_correlation() {
    // a=b=2^-10 and 1-rho=2^-53: u_rel=2^-36, exactly representable.
    for (rho, relative) in [(1.0, 0.0), (f64::from_bits(1.0_f64.to_bits() - 1), 2.0_f64.powi(-36))] {
        let out = calculate("measurement.correlated_voltage_ratio", &[
            ("numerator_voltage", 8.0), ("denominator_voltage", 2.0),
            ("numerator_uncertainty", 0.0078125), ("denominator_uncertainty", 0.001953125),
            ("correlation", rho), ("coverage_factor", 3.0),
        ]).unwrap();
        for (key, expected) in [("voltage_ratio", 4.0),
            ("relative_standard_uncertainty", relative), ("ratio_standard_uncertainty", 4.0 * relative),
            ("expanded_uncertainty", 12.0 * relative)] {
            assert_close(out[key], expected, 1e-12, 0.0);
        }
    }
}

#[test]
fn ratio_zero_uncertainties_coverage_and_one_percent_boundaries() {
    for (ua, ub, rho, relative) in [(0.0, 0.0, -1.0, 0.0), (0.0, 0.0, 1.0, 0.0),
        (0.01, 0.0, -1.0, 0.01), (0.0, 0.01, 1.0, 0.01),
        (0.01, 0.01, -1.0, 0.02), (0.01, 0.01, 1.0, 0.0)] {
        for coverage in [1.0, 3.0] {
            let out = calculate("measurement.correlated_voltage_ratio", &[
                ("numerator_voltage", 1.0), ("denominator_voltage", 1.0),
                ("numerator_uncertainty", ua), ("denominator_uncertainty", ub),
                ("correlation", rho), ("coverage_factor", coverage),
            ]).unwrap();
            for (key, expected) in [("voltage_ratio", 1.0),
                ("relative_standard_uncertainty", relative), ("ratio_standard_uncertainty", relative),
                ("expanded_uncertainty", coverage * relative)] {
                assert_close(out[key], expected, 1e-12, 0.0);
            }
        }
    }
}

#[test]
fn scalar_and_coupled_domain_errors_recover_for_every_model() {
    for (id, valid, invalid, expected) in [
        ("measurement.dmm_accuracy_bound", DMM, vec![(0, -5.00001), (0, 5.00001),
            (1, 0.0), (1, -1.0), (2, -0.001), (3, -0.001)],
            vec![("reading_error_bound", 0.0005), ("range_error_bound", 0.0003),
                ("total_error_bound", 0.0008), ("lower_bound", -1.2508), ("upper_bound", -1.2492)]),
        ("measurement.delta_resistance", DELTA, vec![(0, 0.0), (0, -0.1), (1, 0.0),
            (1, -0.1), (3, 0.02)],
            vec![("resistive_voltage", 0.005), ("resistance", 0.25),
                ("midpoint_emf", -0.0002), ("emf_drift_rate", -0.0003)]),
        ("measurement.compensated_probe_loading", PROBE, vec![(0, -1.0), (1, -1.0),
            (2, 0.0), (2, -1.0), (3, -1e-12), (4, 0.0), (4, -1.0),
            (5, 0.0), (5, -1e-12), (6, -1.0)],
            vec![("matched_probe_capacitance", 16e-12), ("dc_gain", 0.19992003198720512),
                ("ac_gain", 0.1111549158333101), ("scope_voltage", 0.27788728958327524)]),
        ("measurement.rectangular_aperture_response", APERTURE,
            vec![(0, -1.0), (1, -1e-6), (2, -1.0)],
            vec![("midpoint_gain", -2.0 / (3.0 * PI)), ("amplitude_ratio", 2.0 / (3.0 * PI)),
                ("measured_peak", 0.4 / PI), ("effective_delay", 0.01)]),
        ("measurement.sampling_jitter_snr", JITTER,
            vec![(0, 0.0), (0, -1.0), (1, 0.0), (1, -1e-12), (2, 0.0), (2, -1.0), (1, 1e-6)],
            vec![("phase_jitter", 0.0001319468914507713), ("error_rms", 7.464043336106054e-5),
                ("jitter_snr", 77.59201673815932)]),
        ("measurement.correlated_voltage_ratio", RATIO,
            vec![(0, 0.0), (0, -1.0), (1, 0.0), (1, -1.0), (2, -1e-3),
                (3, -1e-3), (2, 0.1200001), (3, 0.0400001), (4, -1.000001),
                (4, 1.000001), (5, 0.0), (5, -1.0)],
            vec![("voltage_ratio", 3.0), ("relative_standard_uncertainty", 0.00412310562561766),
                ("ratio_standard_uncertainty", 0.01236931687685298), ("expanded_uncertainty", 0.030923292192132454)]),
    ] {
        for (index, value) in invalid {
            let mut inputs = valid.to_vec();
            inputs[index].1 = value;
            assert!(calculate(id, &inputs).is_err(), "accepted {id}: {}={value}", inputs[index].0);
            let out = calculate(id, valid).unwrap();
            assert_eq!(out.len(), expected.len());
            for &(key, value) in &expected { assert_close(out[key], value, 1e-11, 0.0); }
        }
    }
}

#[test]
fn every_input_rejects_nonfinite_values_and_recovers() {
    for (id, valid) in [
        ("measurement.dmm_accuracy_bound", DMM), ("measurement.delta_resistance", DELTA),
        ("measurement.compensated_probe_loading", PROBE),
        ("measurement.rectangular_aperture_response", APERTURE),
        ("measurement.sampling_jitter_snr", JITTER), ("measurement.correlated_voltage_ratio", RATIO),
    ] {
        for index in 0..valid.len() {
            for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                let mut inputs = valid.to_vec();
                inputs[index].1 = bad;
                assert!(calculate(id, &inputs).is_err(), "accepted {id}: {}={bad}", inputs[index].0);
                // Same model must recover after each rejection; independent maps are tested above.
                let recovered = calculate(id, valid).unwrap();
                let key_value = match id {
                    "measurement.dmm_accuracy_bound" => ("total_error_bound", 0.0008),
                    "measurement.delta_resistance" => ("resistance", 0.25),
                    "measurement.compensated_probe_loading" => ("scope_voltage", 0.27788728958327524),
                    "measurement.rectangular_aperture_response" => ("measured_peak", 0.4 / PI),
                    "measurement.sampling_jitter_snr" => ("error_rms", 7.464043336106054e-5),
                    _ => ("expanded_uncertainty", 0.030923292192132454),
                };
                assert_close(recovered[key_value.0], key_value.1, 1e-11, 0.0);
            }
        }
    }
}

#[test]
fn aperture_zero_time_survives_large_finite_frequency() {
    let out = calculate("measurement.rectangular_aperture_response", &[
        ("frequency", 1e308), ("aperture_time", 0.0), ("input_peak", 0.2),
    ]).unwrap();
    for (key, expected) in [("midpoint_gain", 1.0), ("amplitude_ratio", 1.0),
        ("measured_peak", 0.2), ("effective_delay", 0.0)] {
        assert_close(out[key], expected, 1e-12, 0.0);
    }
}

#[test]
fn jitter_large_frequency_small_time_has_finite_small_phase() {
    let out = calculate("measurement.sampling_jitter_snr", &[
        ("frequency", 1e308), ("timing_jitter", 1e-312), ("input_peak", 1.0),
    ]).unwrap();
    // 1e-312 is subnormal: its relative input quantization is about 1.5e-12.
    assert_close(out["phase_jitter"], 0.0006283185307179586, 1e-10, 0.0);
    assert_close(out["error_rms"], 0.0004442882938158366, 1e-10, 0.0);
    assert_close(out["jitter_snr"], 64.0364026328377, 1e-10, 0.0);
}

#[test]
fn delta_decimal_linear_emf_zero_resistance_is_admitted() {
    let out = calculate("measurement.delta_resistance", &[
        ("test_current", 0.01), ("sample_interval", 100.0),
        ("voltage_first", 0.00009765625), ("voltage_second", 0.0001953125),
        ("voltage_third", 0.00029296875),
    ]).unwrap();
    // Decimal-linear EMF leaves a few ulps of second-difference roundoff, not negative resistance.
    assert_close(out["resistive_voltage"], 0.0, 0.0, 1e-19);
    assert_close(out["resistance"], 0.0, 0.0, 1e-17);
    assert_close(out["midpoint_emf"], 0.0001953125, 1e-12, 0.0);
    assert_close(out["emf_drift_rate"], 9.765625e-7, 1e-12, 0.0);
}

#[test]
fn researched_nominals_follow_independent_physical_derivations() {
    // Complete source examples supplement, rather than replace, non-default coverage above.
    let out = calculate("measurement.dmm_accuracy_bound", &[
        ("reading", 0.5), ("range_value", 2.0), ("reading_percent", 0.003),
        ("range_percent", 0.001),
    ]).unwrap();
    // 30 ppm of reading and 10 ppm of range, summed as worst-case errors.
    for (key, expected) in [("reading_error_bound", 15e-6), ("range_error_bound", 20e-6),
        ("total_error_bound", 35e-6), ("lower_bound", 0.499965), ("upper_bound", 0.500035)] {
        assert_close(out[key], expected, 1e-12, 0.0);
    }
    for (v1, v2, v3, emf, drift) in [
        (0.001012, -0.000986, 0.001016, 14e-6, 20e-6),
        (0.001010, -0.000990, 0.001010, 10e-6, 0.0),
    ] {
        let out = calculate("measurement.delta_resistance", &[
            ("test_current", 0.01), ("sample_interval", 0.1),
            ("voltage_first", v1), ("voltage_second", v2), ("voltage_third", v3),
        ]).unwrap();
        for (key, expected) in [("resistive_voltage", 0.001), ("resistance", 0.1),
            ("midpoint_emf", emf), ("emf_drift_rate", drift)] {
            assert_close(out[key], expected, 1e-11, 0.0);
        }
    }
    for (frequency, cp) in [(1e6, 10e-12), (0.0, 10e-12), (1e6, 0.0)] {
        let out = calculate("measurement.compensated_probe_loading", &[
            ("source_voltage", 1.0), ("source_resistance", 1000.0),
            ("probe_resistance", 9e6), ("probe_capacitance", cp),
            ("scope_resistance", 1e6), ("scope_capacitance", 90e-12),
            ("frequency", frequency),
        ]).unwrap();
        // Matched reduction; Cp=0 instead reduces to one shunt capacitance.
        let gain = if cp == 0.0 {
            1e6 / 10001000.0_f64.hypot(2.0 * PI * frequency * 9001000.0 * 90e-6)
        } else {
            1e6 / 10001000.0_f64.hypot(2.0 * PI * frequency * 1000.0 * 90e-6)
        };
        for (key, expected) in [("matched_probe_capacitance", 10e-12),
            ("dc_gain", 1e6 / 10001000.0), ("ac_gain", gain), ("scope_voltage", gain)] {
            assert_close(out[key], expected, 1e-12, 0.0);
        }
    }
    let out = calculate("measurement.rectangular_aperture_response", &[
        ("frequency", 25.0), ("aperture_time", 0.01), ("input_peak", 0.2),
    ]).unwrap();
    let gain = 2.0 * SQRT_2 / PI;
    for (key, expected) in [("midpoint_gain", gain), ("amplitude_ratio", gain),
        ("measured_peak", 0.2 * gain), ("effective_delay", 0.005)] {
        assert_close(out[key], expected, 1e-12, 0.0);
    }
    let out = calculate("measurement.sampling_jitter_snr", &[
        ("frequency", 1e7), ("timing_jitter", 2e-12), ("input_peak", 1.0),
    ]).unwrap();
    let error = 44428829.38158366 * 2e-12;
    assert_close(out["phase_jitter"], error * SQRT_2, 1e-12, 0.0);
    assert_close(out["error_rms"], error, 1e-12, 0.0);
    assert_close(out["jitter_snr"], 20.0 * ((1.0 / SQRT_2) / error).log10(), 1e-12, 0.0);
    let out = calculate("measurement.correlated_voltage_ratio", &[
        ("numerator_voltage", 5.0), ("denominator_voltage", 2.5),
        ("numerator_uncertainty", 0.002), ("denominator_uncertainty", 0.0015),
        ("correlation", 0.8), ("coverage_factor", 2.0),
    ]).unwrap();
    // Sensitivity covariance gives Var(r)=5.44e-7, independently of stable hypot form.
    let standard = 5.44e-7_f64.sqrt();
    for (key, expected) in [("voltage_ratio", 2.0),
        ("relative_standard_uncertainty", standard / 2.0),
        ("ratio_standard_uncertainty", standard), ("expanded_uncertainty", 2.0 * standard)] {
        assert_close(out[key], expected, 1e-12, 0.0);
    }
}
