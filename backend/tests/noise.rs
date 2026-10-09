mod common;

use common::{assert_close, calculate};

// Oracles follow research/noise.md: one-sided spectra, exact SI k/q and 290 K NF.
// Nonzero quantities use relative-only tolerances, including subnormal inputs.
// Ideal zero-noise and unity-ratio identities are exact: absolute tolerance 0.

#[test]
fn johnson_complete_independent_example() {
    let out = calculate("noise.johnson_resistor", &[
        ("resistance", 10000.0), ("temperature", 300.0), ("bandwidth", 20000.0),
    ]).unwrap();
    assert_close(out["voltage_noise_density"], 1.2871591976131002e-8, 1e-6, 0.0);
    assert_close(out["voltage_noise_rms"], 1.8203179941977172e-6, 1e-6, 0.0);
}

#[test]
fn johnson_square_root_scaling_and_admissible_limits() {
    // Quarter R halves density; quadruple ENBW restores the original RMS.
    let out = calculate("noise.johnson_resistor", &[
        ("resistance", 2500.0), ("temperature", 300.0), ("bandwidth", 80000.0),
    ]).unwrap();
    assert_close(out["voltage_noise_density"], 6.435795988065501e-9, 1e-12, 0.0);
    assert_close(out["voltage_noise_rms"], 1.8203179941977172e-6, 1e-12, 0.0);

    // Numerical catalog endpoints, not claims about classical-model applicability.
    let out = calculate("noise.johnson_resistor", &[
        ("resistance", 1e12), ("temperature", 2000.0), ("bandwidth", 1e12),
    ]).unwrap();
    assert_close(out["voltage_noise_density"], (1.1045192e-7_f64).sqrt(), 1e-12, 0.0);
    assert_close(out["voltage_noise_rms"], (110451.92_f64).sqrt(), 1e-12, 0.0);
}

#[test]
fn johnson_representable_amplitude_must_not_underflow_with_variance() {
    let out = calculate("noise.johnson_resistor", &[
        ("resistance", 1e-310), ("temperature", 300.0), ("bandwidth", 1.0),
    ]).unwrap();
    // sqrt(4*k*300)*sqrt(R), whose amplitude remains representable.
    let expected = (1.6567788e-20_f64).sqrt() * 1e-155;
    assert_close(out["voltage_noise_density"], expected, 1e-12, 0.0);
    assert_close(out["voltage_noise_rms"], expected, 1e-12, 0.0);
}

#[test]
fn johnson_rejects_every_nonpositive_and_excessive_domain() {
    let valid = [("resistance", 10000.0), ("temperature", 300.0), ("bandwidth", 20000.0)];
    for (index, bad) in [(0, 0.0), (0, -1.0), (0, 1e12 + 1.0),
        (1, 0.0), (1, -1.0), (1, 2001.0), (2, 0.0), (2, -1.0), (2, 1e12 + 1.0)] {
        let mut inputs = valid;
        inputs[index].1 = bad;
        assert!(calculate("noise.johnson_resistor", &inputs).is_err(), "accepted {inputs:?}");
    }
}

#[test]
fn shot_complete_independent_example() {
    let out = calculate("noise.junction_shot", &[("dc_current", 1e-6), ("bandwidth", 10000.0)]).unwrap();
    assert_close(out["current_noise_density"], 5.660700723408719e-13, 1e-6, 0.0);
    assert_close(out["current_noise_rms"], 5.660700723408719e-11, 1e-6, 0.0);
}

#[test]
fn shot_current_bandwidth_scaling_zero_and_maximum() {
    let out = calculate("noise.junction_shot", &[("dc_current", 4e-6), ("bandwidth", 2500.0)]).unwrap();
    assert_close(out["current_noise_density"], 1.1321401446817438e-12, 1e-12, 0.0);
    assert_close(out["current_noise_rms"], 5.660700723408719e-11, 1e-12, 0.0);
    for bandwidth in [1.0, 1e12] {
        let out = calculate("noise.junction_shot", &[("dc_current", 0.0), ("bandwidth", bandwidth)]).unwrap();
        assert_close(out["current_noise_density"], 0.0, 0.0, 0.0);
        assert_close(out["current_noise_rms"], 0.0, 0.0, 0.0);
    }
    let out = calculate("noise.junction_shot", &[("dc_current", 1000.0), ("bandwidth", 1e12)]).unwrap();
    assert_close(out["current_noise_density"], (3.204353268e-16_f64).sqrt(), 1e-12, 0.0);
    assert_close(out["current_noise_rms"], (3.204353268e-4_f64).sqrt(), 1e-12, 0.0);
}

#[test]
fn shot_representable_amplitude_survives_tiny_carrier_current() {
    let out = calculate("noise.junction_shot", &[("dc_current", 1e-310), ("bandwidth", 1.0)]).unwrap();
    let expected = (3.204353268e-19_f64).sqrt() * 1e-155;
    assert_close(out["current_noise_density"], expected, 1e-12, 0.0);
    assert_close(out["current_noise_rms"], expected, 1e-12, 0.0);
}

#[test]
fn shot_rejects_signed_streams_and_invalid_bandwidth() {
    let valid = [("dc_current", 1e-6), ("bandwidth", 10000.0)];
    for (index, bad) in [(0, -1e-6), (0, 1001.0), (1, 0.0), (1, -1.0), (1, 1e12 + 1.0)] {
        let mut inputs = valid;
        inputs[index].1 = bad;
        assert!(calculate("noise.junction_shot", &inputs).is_err(), "accepted {inputs:?}");
    }
}

#[test]
fn rms_sum_complete_independent_example() {
    let out = calculate("noise.uncorrelated_rms_sum", &[
        ("noise_one", 3e-6), ("noise_two", 4e-6), ("noise_three", 12e-6),
    ]).unwrap();
    assert_close(out["total_noise_rms"], 13e-6, 1e-6, 0.0);
}

#[test]
fn rms_sum_permutation_isolated_sources_zero_and_upper_boundary() {
    for values in [[12e-6, 3e-6, 4e-6], [4e-6, 12e-6, 3e-6]] {
        let out = calculate("noise.uncorrelated_rms_sum", &[
            ("noise_one", values[0]), ("noise_two", values[1]), ("noise_three", values[2]),
        ]).unwrap();
        assert_close(out["total_noise_rms"], 13e-6, 1e-12, 0.0);
    }
    for (values, expected) in [([0.0, 0.0, 0.0], 0.0),
        ([1e6, 0.0, 0.0], 1e6), ([0.0, 1e6, 0.0], 1e6), ([0.0, 0.0, 1e6], 1e6),
        ([1e6, 1e6, 1e6], 1e6 * (3.0_f64).sqrt()),
        // Exact single-source identity must hold even when its square underflows.
        ([1e-200, 0.0, 0.0], 1e-200), ([0.0, 1e-200, 0.0], 1e-200), ([0.0, 0.0, 1e-200], 1e-200)] {
        let out = calculate("noise.uncorrelated_rms_sum", &[
            ("noise_one", values[0]), ("noise_two", values[1]), ("noise_three", values[2]),
        ]).unwrap();
        assert_close(out["total_noise_rms"], expected, 1e-12, 0.0);
    }
}

#[test]
fn rms_sum_rejects_negative_amplitudes_and_every_upper_bound() {
    let valid = [("noise_one", 3e-6), ("noise_two", 4e-6), ("noise_three", 12e-6)];
    for index in 0..valid.len() {
        for bad in [-1e-12, 1e6 + 1.0] {
            let mut inputs = valid;
            inputs[index].1 = bad;
            assert!(calculate("noise.uncorrelated_rms_sum", &inputs).is_err(), "accepted {inputs:?}");
        }
    }
}

#[test]
fn amplifier_complete_independent_six_source_example() {
    let out = calculate("noise.noninverting_amplifier_budget", &[
        ("source_resistance", 1000.0), ("feedback_resistance", 9000.0), ("ground_resistance", 1000.0),
        ("voltage_density", 5e-9), ("plus_current_density", 1e-12), ("minus_current_density", 2e-12),
        ("temperature", 300.0), ("bandwidth", 20000.0),
    ]).unwrap();
    assert_close(out["noise_gain"], 10.0, 1e-6, 0.0);
    assert_close(out["output_noise_density"], 7.79222671641425e-8, 1e-6, 0.0);
    assert_close(out["output_noise_rms"], 1.1019872703439002e-5, 1e-6, 0.0);
    assert_close(out["input_referred_noise_rms"], 1.1019872703439002e-6, 1e-6, 0.0);
}

#[test]
fn amplifier_independently_resolves_all_six_variance_transfers() {
    // Rf=Rg=Rs=1 kohm, GN=2. Source thermal variance=6.6271152e-17;
    // feedback and ground resistors each contribute 1.6567788e-17 V^2/Hz.
    // Turn on each amplifier density alone; its expected output variance is
    // respectively (2*en)^2, (2000*i+)^2 and (1000*i-)^2.
    for (en, plus, minus, variance) in [
        (0.0, 0.0, 0.0, 9.9406728e-17_f64),
        (1e-8, 0.0, 0.0, 4.99406728e-16),
        (0.0, 1e-11, 0.0, 4.99406728e-16),
        (0.0, 0.0, 1e-11, 1.99406728e-16),
    ] {
        let out = calculate("noise.noninverting_amplifier_budget", &[
            ("source_resistance", 1000.0), ("feedback_resistance", 1000.0), ("ground_resistance", 1000.0),
            ("voltage_density", en), ("plus_current_density", plus), ("minus_current_density", minus),
            ("temperature", 300.0), ("bandwidth", 4.0),
        ]).unwrap();
        let density = variance.sqrt();
        assert_close(out["noise_gain"], 2.0, 0.0, 0.0);
        assert_close(out["output_noise_density"], density, 1e-12, 0.0);
        assert_close(out["output_noise_rms"], 2.0 * density, 1e-12, 0.0);
        assert_close(out["input_referred_noise_rms"], density, 1e-12, 0.0);
    }
    // Rs=0 suppresses both plus-current and source-resistor noise, but NOT Rf/Rg noise.
    for plus in [0.0, 0.001] {
        let out = calculate("noise.noninverting_amplifier_budget", &[
            ("source_resistance", 0.0), ("feedback_resistance", 1000.0), ("ground_resistance", 1000.0),
            ("voltage_density", 0.0), ("plus_current_density", plus), ("minus_current_density", 0.0),
            ("temperature", 300.0), ("bandwidth", 1.0),
        ]).unwrap();
        let density = (3.3135576e-17_f64).sqrt();
        assert_close(out["noise_gain"], 2.0, 0.0, 0.0);
        assert_close(out["output_noise_density"], density, 1e-12, 0.0);
        assert_close(out["output_noise_rms"], density, 1e-12, 0.0);
        assert_close(out["input_referred_noise_rms"], density / 2.0, 1e-12, 0.0);
    }
}

#[test]
fn amplifier_ground_minimum_and_full_upper_input_boundaries() {
    let out = calculate("noise.noninverting_amplifier_budget", &[
        ("source_resistance", 0.0), ("feedback_resistance", 0.001), ("ground_resistance", 0.001),
        ("voltage_density", 0.0), ("plus_current_density", 0.0), ("minus_current_density", 0.0),
        ("temperature", 300.0), ("bandwidth", 1.0),
    ]).unwrap();
    let density = (3.3135576e-23_f64).sqrt();
    assert_close(out["noise_gain"], 2.0, 0.0, 0.0);
    assert_close(out["output_noise_density"], density, 1e-12, 0.0);
    assert_close(out["output_noise_rms"], density, 1e-12, 0.0);
    assert_close(out["input_referred_noise_rms"], density / 2.0, 1e-12, 0.0);

    let out = calculate("noise.noninverting_amplifier_budget", &[
        ("source_resistance", 1e9), ("feedback_resistance", 1e9), ("ground_resistance", 1e9),
        ("voltage_density", 0.001), ("plus_current_density", 0.001), ("minus_current_density", 0.001),
        ("temperature", 2000.0), ("bandwidth", 1e12),
    ]).unwrap();
    // Independent expanded six-source sum: 4e-6 +4e12 +4.4180768e-10
    // +1e12 +1.1045192e-10 +1.1045192e-10; small terms round below ULP.
    let density = (5e12_f64).sqrt();
    assert_close(out["noise_gain"], 2.0, 0.0, 0.0);
    assert_close(out["output_noise_density"], density, 1e-12, 0.0);
    assert_close(out["output_noise_rms"], density * 1e6, 1e-12, 0.0);
    assert_close(out["input_referred_noise_rms"], density * 5e5, 1e-12, 0.0);
}

#[test]
fn amplifier_feedback_thermal_amplitude_survives_tiny_positive_resistance() {
    let out = calculate("noise.noninverting_amplifier_budget", &[
        ("source_resistance", 0.0), ("feedback_resistance", 1e-310), ("ground_resistance", 1.0),
        ("voltage_density", 0.0), ("plus_current_density", 0.0), ("minus_current_density", 0.0),
        ("temperature", 300.0), ("bandwidth", 1.0),
    ]).unwrap();
    let expected = (1.6567788e-20_f64).sqrt() * 1e-155;
    assert_close(out["noise_gain"], 1.0, 0.0, 0.0);
    assert_close(out["output_noise_density"], expected, 1e-12, 0.0);
    assert_close(out["output_noise_rms"], expected, 1e-12, 0.0);
    assert_close(out["input_referred_noise_rms"], expected, 1e-12, 0.0);
}

#[test]
fn amplifier_rejects_every_resistance_density_temperature_and_bandwidth_guard() {
    let valid = [
        ("source_resistance", 1000.0), ("feedback_resistance", 9000.0), ("ground_resistance", 1000.0),
        ("voltage_density", 5e-9), ("plus_current_density", 1e-12), ("minus_current_density", 2e-12),
        ("temperature", 300.0), ("bandwidth", 20000.0),
    ];
    for (index, bad) in [
        (0, -1.0), (0, 1e9 + 1.0), (1, 0.0), (1, -1.0), (1, 1e9 + 1.0),
        (2, 0.0), (2, 0.000999), (2, 1e9 + 1.0),
        (3, -1e-12), (3, 0.001001), (4, -1e-12), (4, 0.001001), (5, -1e-12), (5, 0.001001),
        (6, 0.0), (6, -1.0), (6, 2001.0), (7, 0.0), (7, -1.0), (7, 1e12 + 1.0),
    ] {
        let mut inputs = valid;
        inputs[index].1 = bad;
        assert!(calculate("noise.noninverting_amplifier_budget", &inputs).is_err(), "accepted {inputs:?}");
    }
}

#[test]
fn decibel_complete_independent_example() {
    let out = calculate("noise.decibel_to_linear", &[("gain_db", -6.0)]).unwrap();
    assert_close(out["power_ratio"], 0.251188643150958, 1e-6, 0.0);
    assert_close(out["voltage_ratio"], 0.5011872336272722, 1e-6, 0.0);
}

#[test]
fn decibel_signed_gain_unity_and_both_catalog_endpoints() {
    for (gain, power, voltage) in [
        (0.0, 1.0, 1.0), (-20.0, 0.01, 0.1), (20.0, 100.0, 10.0),
        (-300.0, 1e-30, 1e-15), (300.0, 1e30, 1e15),
    ] {
        let out = calculate("noise.decibel_to_linear", &[("gain_db", gain)]).unwrap();
        assert_close(out["power_ratio"], power, 1e-12, 0.0);
        assert_close(out["voltage_ratio"], voltage, 1e-12, 0.0);
    }
    let out = calculate("noise.decibel_to_linear", &[("gain_db", 0.0)]).unwrap();
    assert_close(out["power_ratio"], 1.0, 0.0, 0.0);
    assert_close(out["voltage_ratio"], 1.0, 0.0, 0.0);
}

#[test]
fn decibel_rejects_both_out_of_range_signed_gains() {
    for gain in [-300.0001, 300.0001] {
        assert!(calculate("noise.decibel_to_linear", &[("gain_db", gain)]).is_err());
    }
}

#[test]
fn dynamic_range_complete_independent_example() {
    let out = calculate("noise.analog_dynamic_range", &[
        ("maximum_sine_peak", 2.0), ("noise_rms", 1e-5), ("required_snr_db", 20.0),
    ]).unwrap();
    assert_close(out["maximum_signal_rms"], 1.414213562373095, 1e-6, 0.0);
    assert_close(out["minimum_signal_rms"], 1e-4, 1e-6, 0.0);
    assert_close(out["noise_limited_dynamic_range_db"], 103.01029995663981, 1e-6, 0.0);
    assert_close(out["usable_dynamic_range_db"], 83.01029995663981, 1e-6, 0.0);
}

#[test]
fn dynamic_range_alternate_margin_and_exact_detection_boundaries() {
    let out = calculate("noise.analog_dynamic_range", &[
        ("maximum_sine_peak", std::f64::consts::SQRT_2), ("noise_rms", 0.001), ("required_snr_db", 40.0),
    ]).unwrap();
    assert_close(out["maximum_signal_rms"], 1.0, 1e-12, 0.0);
    assert_close(out["minimum_signal_rms"], 0.1, 1e-12, 0.0);
    assert_close(out["noise_limited_dynamic_range_db"], 60.0, 1e-12, 0.0);
    assert_close(out["usable_dynamic_range_db"], 20.0, 1e-12, 0.0);
    for (noise, margin, noise_range) in [(1.0, 0.0, 0.0), (0.1, 20.0, 20.0)] {
        let out = calculate("noise.analog_dynamic_range", &[
            ("maximum_sine_peak", std::f64::consts::SQRT_2), ("noise_rms", noise), ("required_snr_db", margin),
        ]).unwrap();
        assert_close(out["maximum_signal_rms"], 1.0, 0.0, 0.0);
        assert_close(out["minimum_signal_rms"], 1.0, 0.0, 0.0);
        assert_close(out["noise_limited_dynamic_range_db"], noise_range, 1e-12, 0.0);
        assert_close(out["usable_dynamic_range_db"], 0.0, 0.0, 0.0);
    }
    // Smallest peak/floor and maximum margin remain meaningful and admissible.
    let out = calculate("noise.analog_dynamic_range", &[
        ("maximum_sine_peak", 1e-12), ("noise_rms", 1e-18), ("required_snr_db", 0.0),
    ]).unwrap();
    assert_close(out["maximum_signal_rms"], 1e-12 / std::f64::consts::SQRT_2, 1e-12, 0.0);
    assert_close(out["minimum_signal_rms"], 1e-18, 1e-12, 0.0);
    assert_close(out["noise_limited_dynamic_range_db"], 116.98970004336019, 1e-12, 0.0);
    assert_close(out["usable_dynamic_range_db"], 116.98970004336019, 1e-12, 0.0);
    let out = calculate("noise.analog_dynamic_range", &[
        ("maximum_sine_peak", 1e6), ("noise_rms", 1e-18), ("required_snr_db", 120.0),
    ]).unwrap();
    assert_close(out["maximum_signal_rms"], 1e6 / std::f64::consts::SQRT_2, 1e-12, 0.0);
    assert_close(out["minimum_signal_rms"], 1e-12, 1e-12, 0.0);
    assert_close(out["noise_limited_dynamic_range_db"], 476.9897000433602, 1e-12, 0.0);
    assert_close(out["usable_dynamic_range_db"], 356.9897000433602, 1e-12, 0.0);
}

#[test]
fn dynamic_range_rejects_amplitude_bounds_and_impossible_detection_margin() {
    let valid = [("maximum_sine_peak", 2.0), ("noise_rms", 1e-5), ("required_snr_db", 20.0)];
    for (index, bad) in [(0, 0.0), (0, 0.999e-12), (0, 1e6 + 1.0),
        (1, 0.0), (1, 0.999e-18), (1, 1e6 + 1.0), (2, -0.001), (2, 120.001)] {
        let mut inputs = valid;
        inputs[index].1 = bad;
        assert!(calculate("noise.analog_dynamic_range", &inputs).is_err(), "accepted {inputs:?}");
    }
    // Individually legal inputs can violate the actual signal/noise constraint.
    for (peak, noise, margin) in [(1.0, 1.0, 0.0), (std::f64::consts::SQRT_2, 0.1, 20.000001),
        (1e-12, 1e-18, 120.0), (1e6, 1e6, 0.0)] {
        assert!(calculate("noise.analog_dynamic_range", &[
            ("maximum_sine_peak", peak), ("noise_rms", noise), ("required_snr_db", margin),
        ]).is_err());
    }
}

#[test]
fn noise_temperature_complete_independent_example() {
    let out = calculate("noise.figure_equivalent_temperature", &[("noise_figure_db", 3.0)]).unwrap();
    assert_close(out["noise_factor"], 1.9952623149688795, 1e-6, 0.0);
    assert_close(out["equivalent_noise_temperature"], 288.62607134097505, 1e-6, 0.0);
}

#[test]
fn noise_temperature_fixed_reference_ideal_and_maximum_factors() {
    for (nf, factor, temperature) in [(0.0, 1.0, 0.0), (10.0, 10.0, 2610.0),
        (100.0, 1e10, 2899999999710.0)] {
        let out = calculate("noise.figure_equivalent_temperature", &[("noise_figure_db", nf)]).unwrap();
        assert_close(out["noise_factor"], factor, 1e-12, 0.0);
        assert_close(out["equivalent_noise_temperature"], temperature, 1e-12, 0.0);
    }
    let out = calculate("noise.figure_equivalent_temperature", &[("noise_figure_db", 0.0)]).unwrap();
    assert_close(out["noise_factor"], 1.0, 0.0, 0.0);
    assert_close(out["equivalent_noise_temperature"], 0.0, 0.0, 0.0);
}

#[test]
fn noise_temperature_preserves_sub_epsilon_positive_added_noise() {
    let out = calculate("noise.figure_equivalent_temperature", &[("noise_figure_db", 1e-16)]).unwrap();
    // x=ln(10)*1e-17; exp(x)-1=x+O(1e-34). The linear factor rounds to 1,
    // but Te is independently representable and must never be rounded to zero.
    assert_close(out["noise_factor"], 1.0, 0.0, 0.0);
    assert_close(out["equivalent_noise_temperature"], 6.677496769682733e-15, 1e-12, 0.0);
}

#[test]
fn noise_temperature_rejects_negative_conventional_nf_and_maximum_excess() {
    for nf in [-1e-16, 100.0001] {
        assert!(calculate("noise.figure_equivalent_temperature", &[("noise_figure_db", nf)]).is_err());
    }
}

#[test]
fn friis_complete_independent_example() {
    let out = calculate("noise.friis_three_stage", &[
        ("first_noise_figure_db", 2.0), ("second_noise_figure_db", 6.0), ("third_noise_figure_db", 10.0),
        ("first_gain_db", 20.0), ("second_gain_db", 10.0),
    ]).unwrap();
    for (key, expected) in [
        ("first_noise_factor", 1.5848931924611136), ("second_noise_factor", 3.9810717055349722),
        ("third_noise_factor", 10.0), ("first_power_gain", 100.0), ("second_power_gain", 10.0),
        ("cascade_noise_factor", 1.6237039095164632), ("cascade_noise_figure_db", 2.1050683636637153),
        ("cascade_noise_temperature", 180.8741337597743),
    ] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn friis_ideal_factor_boundaries_and_signed_power_gains() {
    for (first, second, g1, g2) in [(0.0, 0.0, 1.0, 1.0),
        (-120.0, 120.0, 1e-12, 1e12), (120.0, -120.0, 1e12, 1e-12)] {
        let out = calculate("noise.friis_three_stage", &[
            ("first_noise_figure_db", 0.0), ("second_noise_figure_db", 0.0), ("third_noise_figure_db", 0.0),
            ("first_gain_db", first), ("second_gain_db", second),
        ]).unwrap();
        for key in ["first_noise_factor", "second_noise_factor", "third_noise_factor", "cascade_noise_factor"] {
            assert_close(out[key], 1.0, 0.0, 0.0);
        }
        assert_close(out["first_power_gain"], g1, 1e-12, 0.0);
        assert_close(out["second_power_gain"], g2, 1e-12, 0.0);
        assert_close(out["cascade_noise_figure_db"], 0.0, 0.0, 0.0);
        assert_close(out["cascade_noise_temperature"], 0.0, 0.0, 0.0);
    }
    // Three 10 dB passive losses at T0: F=10+9/0.1+9/0.01=1000.
    let out = calculate("noise.friis_three_stage", &[
        ("first_noise_figure_db", 10.0), ("second_noise_figure_db", 10.0), ("third_noise_figure_db", 10.0),
        ("first_gain_db", -10.0), ("second_gain_db", -10.0),
    ]).unwrap();
    for (key, expected) in [("first_noise_factor", 10.0), ("second_noise_factor", 10.0),
        ("third_noise_factor", 10.0), ("first_power_gain", 0.1), ("second_power_gain", 0.1),
        ("cascade_noise_factor", 1000.0), ("cascade_noise_figure_db", 30.0),
        ("cascade_noise_temperature", 289710.0)] {
        assert_close(out[key], expected, 1e-12, 0.0);
    }
}

#[test]
fn friis_preserves_tiny_excess_from_each_stage_before_weighting() {
    // Independently expand ln(1+X) at X<3e-17. O(X^2) is below double precision.
    for (nfs, expected_nf, expected_temperature) in [
        ([1e-16, 0.0, 0.0], 1e-16, 6.677496769682733e-15),
        ([0.0, 1e-16, 0.0], 1e-18, 6.677496769682733e-17),
        ([0.0, 0.0, 1e-16], 1e-19, 6.677496769682733e-18),
    ] {
        let out = calculate("noise.friis_three_stage", &[
            ("first_noise_figure_db", nfs[0]), ("second_noise_figure_db", nfs[1]), ("third_noise_figure_db", nfs[2]),
            ("first_gain_db", 20.0), ("second_gain_db", 10.0),
        ]).unwrap();
        for key in ["first_noise_factor", "second_noise_factor", "third_noise_factor", "cascade_noise_factor"] {
            assert_close(out[key], 1.0, 0.0, 0.0);
        }
        assert_close(out["first_power_gain"], 100.0, 1e-12, 0.0);
        assert_close(out["second_power_gain"], 10.0, 1e-12, 0.0);
        assert_close(out["cascade_noise_figure_db"], expected_nf, 1e-12, 0.0);
        assert_close(out["cascade_noise_temperature"], expected_temperature, 1e-12, 0.0);
    }
    // A tiny second-stage NF after -120 dB gain has X=ln(10)*1e-5,
    // despite F2 rounding to unity. Independent four-term Mercator expansion
    // has relative remainder <6e-20 here, rather than using the runtime oracle.
    let out = calculate("noise.friis_three_stage", &[
        ("first_noise_figure_db", 0.0), ("second_noise_figure_db", 1e-16), ("third_noise_figure_db", 0.0),
        ("first_gain_db", -120.0), ("second_gain_db", 0.0),
    ]).unwrap();
    let x = std::f64::consts::LN_10 * 1e-5;
    let logarithm = x - x * x / 2.0 + x * x * x / 3.0 - x * x * x * x / 4.0;
    for key in ["first_noise_factor", "second_noise_factor", "third_noise_factor"] {
        assert_close(out[key], 1.0, 0.0, 0.0);
    }
    assert_close(out["first_power_gain"], 1e-12, 1e-12, 0.0);
    assert_close(out["second_power_gain"], 1.0, 0.0, 0.0);
    assert_close(out["cascade_noise_factor"], 1.0 + x, 1e-12, 0.0);
    assert_close(out["cascade_noise_figure_db"], 10.0 * logarithm / std::f64::consts::LN_10, 1e-12, 0.0);
    assert_close(out["cascade_noise_temperature"], 290.0 * x, 1e-12, 0.0);
}

#[test]
fn friis_maximum_nf_and_minimum_gain_remain_finite_and_correct() {
    let out = calculate("noise.friis_three_stage", &[
        ("first_noise_figure_db", 100.0), ("second_noise_figure_db", 100.0), ("third_noise_figure_db", 100.0),
        ("first_gain_db", -120.0), ("second_gain_db", -120.0),
    ]).unwrap();
    let excess = 9999999999.0 + 9999999999.0 * 1e12 + 9999999999.0 * 1e24;
    for key in ["first_noise_factor", "second_noise_factor", "third_noise_factor"] {
        assert_close(out[key], 1e10, 1e-12, 0.0);
    }
    assert_close(out["first_power_gain"], 1e-12, 1e-12, 0.0);
    assert_close(out["second_power_gain"], 1e-12, 1e-12, 0.0);
    assert_close(out["cascade_noise_factor"], 1.0 + excess, 1e-12, 0.0);
    assert_close(out["cascade_noise_figure_db"], 10.0 * (1.0_f64 + excess).log10(), 1e-12, 0.0);
    assert_close(out["cascade_noise_temperature"], 290.0 * excess, 1e-12, 0.0);
}

#[test]
fn friis_rejects_every_stage_nf_and_signed_gain_guard() {
    let valid = [("first_noise_figure_db", 2.0), ("second_noise_figure_db", 6.0),
        ("third_noise_figure_db", 10.0), ("first_gain_db", 20.0), ("second_gain_db", 10.0)];
    for index in 0..3 {
        for bad in [-1e-16, 100.0001] {
            let mut inputs = valid;
            inputs[index].1 = bad;
            assert!(calculate("noise.friis_three_stage", &inputs).is_err(), "accepted {inputs:?}");
        }
    }
    for index in 3..5 {
        for bad in [-120.0001, 120.0001] {
            let mut inputs = valid;
            inputs[index].1 = bad;
            assert!(calculate("noise.friis_three_stage", &inputs).is_err(), "accepted {inputs:?}");
        }
    }
}

#[test]
fn all_noise_models_reject_nonfinite_missing_and_extra_inputs() {
    let cases: &[(&str, &[(&str, f64)])] = &[
        ("noise.johnson_resistor", &[("resistance", 10000.0), ("temperature", 300.0), ("bandwidth", 20000.0)]),
        ("noise.junction_shot", &[("dc_current", 1e-6), ("bandwidth", 10000.0)]),
        ("noise.uncorrelated_rms_sum", &[("noise_one", 3e-6), ("noise_two", 4e-6), ("noise_three", 12e-6)]),
        ("noise.noninverting_amplifier_budget", &[
            ("source_resistance", 1000.0), ("feedback_resistance", 9000.0), ("ground_resistance", 1000.0),
            ("voltage_density", 5e-9), ("plus_current_density", 1e-12), ("minus_current_density", 2e-12),
            ("temperature", 300.0), ("bandwidth", 20000.0),
        ]),
        ("noise.decibel_to_linear", &[("gain_db", -6.0)]),
        ("noise.analog_dynamic_range", &[("maximum_sine_peak", 2.0), ("noise_rms", 1e-5), ("required_snr_db", 20.0)]),
        ("noise.figure_equivalent_temperature", &[("noise_figure_db", 3.0)]),
        ("noise.friis_three_stage", &[("first_noise_figure_db", 2.0), ("second_noise_figure_db", 6.0),
            ("third_noise_figure_db", 10.0), ("first_gain_db", 20.0), ("second_gain_db", 10.0)]),
    ];
    for &(id, valid) in cases {
        for index in 0..valid.len() {
            for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                let mut inputs = valid.to_vec();
                inputs[index].1 = bad;
                assert!(calculate(id, &inputs).is_err(), "{id} accepted nonfinite {}", inputs[index].0);
            }
            let mut inputs = valid.to_vec();
            inputs.remove(index);
            assert!(calculate(id, &inputs).is_err(), "{id} accepted missing {}", valid[index].0);
        }
        let mut inputs = valid.to_vec();
        inputs.push(("unexpected_input", 1.0));
        assert!(calculate(id, &inputs).is_err(), "{id} accepted an undeclared input");
    }
}
