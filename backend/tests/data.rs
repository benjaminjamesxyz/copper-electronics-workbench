mod common;

use common::{assert_close, calculate};

// Goldens come from research/data.md and independently selected dyadic/logarithmic
// identities. No expected value is read from the catalog or the engine's results.
// Absolute tolerance is zero unless a documented subtraction can round a true
// zero; discrete counts and exactly representable zero boundaries are exact.

#[test]
fn adc_lsb_complete_researched_example() {
    let out = calculate("data.adc_lsb", &[("bits", 12.0), ("full_scale_span", 3.3)]).unwrap();
    assert_eq!(out["code_count"], 4096.0);
    assert_close(out["lsb_v"], 0.0008056640625, 1e-6, 0.0);
}

#[test]
fn adc_lsb_binary_endpoints_and_span_scaling() {
    for (bits, span, count, step) in [
        (1.0, 2.0, 2.0, 1.0),
        (8.0, 2.0, 256.0, 0.0078125),
        (8.0, 4.0, 256.0, 0.015625),
        (32.0, 1e-9, 4294967296.0, 1e-9 / 4294967296.0),
        (32.0, 1e6, 4294967296.0, 1e6 / 4294967296.0),
    ] {
        let out = calculate("data.adc_lsb", &[("bits", bits), ("full_scale_span", span)]).unwrap();
        assert_eq!(out["code_count"], count);
        // The 32-bit minimum-span LSB is ~2.33e-19 V; a blanket voltage
        // absolute tolerance would incorrectly accept a zero result.
        assert_close(out["lsb_v"], step, 1e-6, 0.0);
    }
}

#[test]
fn adc_lsb_rejects_nonbinary_resolution_and_invalid_span() {
    for bits in [0.0, 33.0, 12.5, f64::NAN, f64::INFINITY] {
        assert!(calculate("data.adc_lsb", &[("bits", bits), ("full_scale_span", 3.3)]).is_err());
    }
    for span in [-1.0, 0.0, 5e-10, 1000001.0, f64::NAN, f64::INFINITY] {
        assert!(calculate("data.adc_lsb", &[("bits", 12.0), ("full_scale_span", span)]).is_err());
    }
}

#[test]
fn quantization_complete_researched_example() {
    let out = calculate("data.adc_quantization_error", &[
        ("bits", 10.0), ("v_low", -1.0), ("v_high", 1.0), ("v_in", 0.123),
    ]).unwrap();
    assert_close(out["quantization_step"], 0.001953125, 1e-6, 0.0);
    assert_eq!(out["code"], 574.0);
    assert_close(out["reconstructed_v"], 0.1220703125, 1e-6, 0.0);
    assert_close(out["error_v"], -0.0009296875, 1e-6, 0.0);
    assert_close(out["absolute_error_v"], 0.0009296875, 1e-6, 0.0);
}

#[test]
fn quantization_midrise_edges_midpoints_and_signed_errors() {
    // Four 1-V-wide bins over [-2,2). Boundaries belong to the higher bin;
    // midpoint reconstruction gives +0.5 V at each lower edge, never zero.
    for (input, code, reconstruction, error) in [
        (-2.0, 0.0, -1.5, 0.5),
        (-1.5, 0.0, -1.5, 0.0),
        (-1.25, 0.0, -1.5, -0.25),
        (-1.0, 1.0, -0.5, 0.5),
        (0.0, 2.0, 0.5, 0.5),
        (1.0, 3.0, 1.5, 0.5),
        (1.75, 3.0, 1.5, -0.25),
    ] {
        let out = calculate("data.adc_quantization_error", &[
            ("bits", 2.0), ("v_low", -2.0), ("v_high", 2.0), ("v_in", input),
        ]).unwrap();
        assert_eq!(out["quantization_step"], 1.0);
        assert_eq!(out["code"], code);
        assert_eq!(out["reconstructed_v"], reconstruction);
        assert_eq!(out["error_v"], error);
        assert_eq!(out["absolute_error_v"], error.abs());
    }
    // Either side of a binary transition tests floor, not rounding; offsets
    // remain distinguishable after subtracting the -2 V lower endpoint.
    for (input, code) in [(1.0 - 2.0_f64.powi(-50), 2.0), (1.0 + 2.0_f64.powi(-50), 3.0)] {
        let out = calculate("data.adc_quantization_error", &[
            ("bits", 2.0), ("v_low", -2.0), ("v_high", 2.0), ("v_in", input),
        ]).unwrap();
        assert_eq!(out["code"], code);
    }
}

#[test]
fn quantization_preserves_sub_ulp_local_error_at_large_offset() {
    // An EXACT dyadic span, 2^-29 V, is above the 1 nV minimum.
    // 32 bits give q=2^-61 V; the half-bin error is 2^-62 V, even though
    // the absolute midpoint near +/-999 V cannot be represented distinctly.
    for low in [-999.0, 999.0] {
        let out = calculate("data.adc_quantization_error", &[
            ("bits", 32.0), ("v_low", low),
            ("v_high", low + 2.0_f64.powi(-29)), ("v_in", low),
        ]).unwrap();
        assert_close(out["quantization_step"], 2.0_f64.powi(-61), 1e-6, 0.0);
        assert_eq!(out["code"], 0.0);
        assert_eq!(out["reconstructed_v"], low);
        assert_close(out["error_v"], 2.0_f64.powi(-62), 1e-6, 0.0);
        assert_close(out["absolute_error_v"], 2.0_f64.powi(-62), 1e-6, 0.0);
    }
    let out = calculate("data.adc_quantization_error", &[
        ("bits", 1.0), ("v_low", -1000.0), ("v_high", 1000.0), ("v_in", -1000.0),
    ]).unwrap();
    assert_eq!(out["quantization_step"], 1000.0);
    assert_eq!(out["code"], 0.0);
    assert_eq!(out["reconstructed_v"], -500.0);
    assert_eq!(out["error_v"], 500.0);
    assert_eq!(out["absolute_error_v"], 500.0);
}

#[test]
fn quantization_rejects_bad_spans_inputs_and_normalized_upper_edge() {
    let nominal = [("bits", 10.0), ("v_low", -1.0), ("v_high", 1.0), ("v_in", 0.123)];
    for (index, value) in [
        (0, 0.0), (0, 33.0), (0, 10.5),
        (1, -1001.0), (1, 1001.0),
        (2, -1001.0), (2, 1001.0),
        (3, -1001.0), (3, 1001.0), (3, -1.001), (3, 1.0), (3, 1.001),
    ] {
        let mut inputs = nominal;
        inputs[index].1 = value;
        assert!(calculate("data.adc_quantization_error", &inputs).is_err());
    }
    for (low, high, input) in [
        (0.0, 0.0, 0.0), (1.0, 0.0, 0.5), (0.0, 5e-10, 0.0),
        // The documented decimal span rounds to <1 nV in binary64.
        (999.0, 999.000000001, 999.0),
        // input < high, but subtracting -1000 rounds its normalized
        // fraction to exactly one. The explicit normalization guard applies.
        (-1000.0, 1.0, 1.0 - 2.0_f64.powi(-53)),
    ] {
        assert!(calculate("data.adc_quantization_error", &[
            ("bits", 32.0), ("v_low", low), ("v_high", high), ("v_in", input),
        ]).is_err());
    }
    for index in 0..nominal.len() {
        let mut inputs = nominal;
        inputs[index].1 = f64::NAN;
        assert!(calculate("data.adc_quantization_error", &inputs).is_err());
    }
}

#[test]
fn quantization_accepts_exact_minimum_span_and_last_bin() {
    let out = calculate("data.adc_quantization_error", &[
        ("bits", 1.0), ("v_low", 0.0), ("v_high", 1e-9), ("v_in", 0.0),
    ]).unwrap();
    assert_close(out["quantization_step"], 5e-10, 1e-6, 0.0);
    assert_eq!(out["code"], 0.0);
    assert_close(out["reconstructed_v"], 2.5e-10, 1e-6, 0.0);
    assert_close(out["error_v"], 2.5e-10, 1e-6, 0.0);
    assert_close(out["absolute_error_v"], 2.5e-10, 1e-6, 0.0);
    let out = calculate("data.adc_quantization_error", &[
        ("bits", 32.0), ("v_low", 0.0), ("v_high", 1.0),
        ("v_in", 1.0 - 2.0_f64.powi(-53)),
    ]).unwrap();
    assert_eq!(out["quantization_step"], 2.0_f64.powi(-32));
    assert_eq!(out["code"], 4294967295.0);
    assert_eq!(out["reconstructed_v"], 1.0 - 2.0_f64.powi(-33));
    let error = -2.0_f64.powi(-33) + 2.0_f64.powi(-53);
    assert_close(out["error_v"], error, 1e-6, 0.0);
    assert_close(out["absolute_error_v"], -error, 1e-6, 0.0);
}

#[test]
fn bandlimited_snr_complete_researched_example() {
    let out = calculate("data.bandlimited_quantization_snr", &[
        ("bits", 12.0), ("sample_rate", 1e6), ("noise_bandwidth", 20000.0),
        ("amplitude_fraction", 0.8),
    ]).unwrap();
    assert_close(out["oversampling_ratio"], 25.0, 1e-6, 0.0);
    assert_close(out["snr_linear"], 20066.219972879797, 1e-6, 0.0);
    assert_close(out["process_gain_db"], 13.979400086720377, 1e-6, 0.0);
    assert_close(out["snr_db"], 86.04931137647155, 1e-6, 0.0);
}

#[test]
fn bandlimited_snr_white_error_integral_and_filtering_identities() {
    // Integrating uniform error gives variance q^2/12. A full-scale sine
    // gives signal power (2^N*q)^2/8; their ratio is 1.5*4^N.
    // Exact 16-LSB amplitude screen and exact Nyquist-band equality included.
    for (bits, rate, bandwidth, amplitude, osr, signal_span_lsb) in [
        (4.0, 32000.0, 16000.0, 1.0, 1.0, 16.0),
        (8.0, 32000.0, 16000.0, 1.0, 1.0, 256.0),
        (8.0, 64000.0, 16000.0, 1.0, 2.0, 256.0),
        (8.0, 64000.0, 16000.0, 0.5, 2.0, 128.0),
        (8.0, 32000.0, 16000.0, 0.0625, 1.0, 16.0),
        (32.0, 1e12, 1e-9, 1e-6, 5e20, 4294.967296),
    ] {
        let out = calculate("data.bandlimited_quantization_snr", &[
            ("bits", bits), ("sample_rate", rate), ("noise_bandwidth", bandwidth),
            ("amplitude_fraction", amplitude),
        ]).unwrap();
        let power_ratio: f64 = signal_span_lsb * signal_span_lsb * 1.5 * osr;
        assert_close(out["oversampling_ratio"], osr, 1e-6, 0.0);
        assert_close(out["snr_linear"], power_ratio.sqrt(), 1e-6, 0.0);
        // log10(1) is exact zero; no dB floor is permitted at OSR=1.
        assert_close(out["process_gain_db"], 10.0 * osr.log10(), 1e-6, 0.0);
        assert_close(out["snr_db"], 10.0 * power_ratio.log10(), 1e-6, 0.0);
    }
    let out = calculate("data.bandlimited_quantization_snr", &[
        ("bits", 4.0), ("sample_rate", 2e-9), ("noise_bandwidth", 1e-9),
        ("amplitude_fraction", 1.0),
    ]).unwrap();
    assert_eq!(out["oversampling_ratio"], 1.0);
    assert_close(out["snr_linear"], 384.0_f64.sqrt(), 1e-6, 0.0);
    assert_eq!(out["process_gain_db"], 0.0);
    assert_close(out["snr_db"], 10.0 * 384.0_f64.log10(), 1e-6, 0.0);
}

#[test]
fn bandlimited_snr_rejects_every_range_and_model_guard() {
    let nominal = [("bits", 12.0), ("sample_rate", 1e6), ("noise_bandwidth", 20000.0), ("amplitude_fraction", 0.8)];
    for (index, value) in [
        (0, 3.0), (0, 33.0), (0, 12.5),
        (1, 0.0), (1, 5e-10), (1, 1e12 + 1.0), (1, 39999.0),
        (2, 0.0), (2, 5e-10), (2, 1e12 + 1.0), (2, 500001.0),
        (3, 0.0), (3, 5e-7), (3, 1.01), (3, 0.001),
    ] {
        let mut inputs = nominal;
        inputs[index].1 = value;
        assert!(calculate("data.bandlimited_quantization_snr", &inputs).is_err());
    }
    // The screen is >=16 LSB, not >16. Reject just below the exact screen.
    assert!(calculate("data.bandlimited_quantization_snr", &[
        ("bits", 8.0), ("sample_rate", 32000.0), ("noise_bandwidth", 16000.0),
        ("amplitude_fraction", 0.0625 - 2.0_f64.powi(-57)),
    ]).is_err());
    for index in 0..nominal.len() {
        let mut inputs = nominal;
        inputs[index].1 = f64::INFINITY;
        assert!(calculate("data.bandlimited_quantization_snr", &inputs).is_err());
    }
}

#[test]
fn enob_complete_researched_example() {
    let out = calculate("data.dynamic_enob", &[("sinad_db", 68.0)]).unwrap();
    assert_close(out["enob_bits"], 11.002074272256452, 1e-6, 0.0);
}

#[test]
fn enob_continuous_equivalent_bits_and_admitted_negative_boundary() {
    // Ideal full-scale signal/noise POWER ratio is 1.5*4^N.
    for bits in [0.0_f64, 1.0, 8.0, 12.5, 32.0] {
        let sinad = 10.0 * (1.5 * 4.0_f64.powf(bits)).log10();
        let out = calculate("data.dynamic_enob", &[("sinad_db", sinad)]).unwrap();
        // At N=0 the logarithmic subtraction can cancel two ~1.76-dB
        // quantities. 8 eps bits bounds only binary64 arithmetic, not noise.
        assert_close(out["enob_bits"], bits, 1e-6, 8.0 * f64::EPSILON);
    }
    let out = calculate("data.dynamic_enob", &[("sinad_db", 0.0)]).unwrap();
    assert_close(out["enob_bits"], -0.2924812503605781, 1e-6, 0.0);
    assert!(out["enob_bits"] < 0.0);
    let out = calculate("data.dynamic_enob", &[("sinad_db", 200.0)]).unwrap();
    let expected = (10.0_f64.powf(20.0) / 1.5).ln() / 4.0_f64.ln();
    assert_close(out["enob_bits"], expected, 1e-6, 0.0);
}

#[test]
fn enob_rejects_invalid_measured_sinad() {
    for sinad in [-0.01, 200.01, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(calculate("data.dynamic_enob", &[("sinad_db", sinad)]).is_err());
    }
}

#[test]
fn nyquist_complete_researched_example() {
    let out = calculate("data.nyquist_sampling_margin", &[
        ("sample_rate", 48000.0), ("highest_frequency", 20000.0),
    ]).unwrap();
    assert_close(out["nyquist_frequency"], 24000.0, 1e-6, 0.0);
    assert_close(out["frequency_margin"], 4000.0, 1e-6, 0.0);
    assert_close(out["rate_ratio"], 1.2, 1e-6, 0.0);
}

#[test]
fn nyquist_signed_failure_margin_equality_and_scaling() {
    for (rate, highest, nyquist, margin, ratio) in [
        (32000.0, 20000.0, 16000.0, -4000.0, 0.8),
        (40000.0, 20000.0, 20000.0, 0.0, 1.0),
        (96000.0, 40000.0, 48000.0, 8000.0, 1.2),
        (1e-9, 1e-9, 5e-10, -5e-10, 0.5),
        (1e12, 1e12, 5e11, -5e11, 0.5),
    ] {
        let out = calculate("data.nyquist_sampling_margin", &[
            ("sample_rate", rate), ("highest_frequency", highest),
        ]).unwrap();
        assert_close(out["nyquist_frequency"], nyquist, 1e-6, 0.0);
        // Zero is constructed exactly; diagnostic negative margins are valid.
        assert_close(out["frequency_margin"], margin, 1e-6, 0.0);
        assert_close(out["rate_ratio"], ratio, 1e-6, 0.0);
    }
}

#[test]
fn nyquist_rejects_nonpositive_and_out_of_range_frequencies() {
    for bad in [-1.0, 0.0, 5e-10, 1e12 + 1.0, f64::NAN, f64::INFINITY] {
        assert!(calculate("data.nyquist_sampling_margin", &[
            ("sample_rate", bad), ("highest_frequency", 20000.0),
        ]).is_err());
        assert!(calculate("data.nyquist_sampling_margin", &[
            ("sample_rate", 48000.0), ("highest_frequency", bad),
        ]).is_err());
    }
}

#[test]
fn alias_complete_researched_example() {
    let out = calculate("data.tone_alias_frequency", &[
        ("input_frequency", 173000.0), ("sample_rate", 48000.0),
    ]).unwrap();
    assert_eq!(out["nearest_sample_multiple"], 4.0);
    assert_close(out["alias_frequency"], 19000.0, 1e-6, 0.0);
}

#[test]
fn alias_fold_ties_dc_and_periodic_spectral_replication() {
    for (tone, rate, multiple, alias) in [
        (0.0, 48000.0, 0.0, 0.0),
        (96000.0, 48000.0, 2.0, 0.0),
        (24000.0, 48000.0, 1.0, 24000.0),
        (120000.0, 48000.0, 3.0, 24000.0),
        (119999.0, 48000.0, 2.0, 23999.0),
        (120001.0, 48000.0, 3.0, 23999.0),
        (19000.0, 48000.0, 0.0, 19000.0),
        (19000.0 + 7.0 * 48000.0, 48000.0, 7.0, 19000.0),
        (48000.0 - 19000.0, 48000.0, 1.0, 19000.0),
        (1e9, 1.0, 1e9, 0.0),
        (1e12, 1000.0, 1e9, 0.0),
        (0.0, 1e-9, 0.0, 0.0),
    ] {
        let out = calculate("data.tone_alias_frequency", &[
            ("input_frequency", tone), ("sample_rate", rate),
        ]).unwrap();
        assert_eq!(out["nearest_sample_multiple"], multiple);
        // These DC multiples are exactly representable, so no floor is needed.
        assert_close(out["alias_frequency"], alias, 1e-6, 0.0);
        assert!(out["alias_frequency"] >= 0.0 && out["alias_frequency"] <= rate / 2.0);
    }
    // Tiny but resolvable remainders around a sample-rate multiple: exact
    // binary spacing avoids masking a valid alias with a blanket Hz tolerance.
    for tone in [1.0 - 2.0_f64.powi(-52), 1.0 + 2.0_f64.powi(-52)] {
        let out = calculate("data.tone_alias_frequency", &[
            ("input_frequency", tone), ("sample_rate", 1.0),
        ]).unwrap();
        assert_eq!(out["nearest_sample_multiple"], 1.0);
        assert_close(out["alias_frequency"], 2.0_f64.powi(-52), 1e-6, 0.0);
    }
}

#[test]
fn alias_rejects_ratio_conditioning_and_invalid_frequencies() {
    for (tone, rate) in [
        (-1.0, 48000.0), (1e12 + 1.0, 48000.0),
        (173000.0, 0.0), (173000.0, -1.0), (0.0, 5e-10), (0.0, 1e12 + 1.0),
        (1e9 + 1.0, 1.0), (2.0, 1e-9),
        (f64::NAN, 48000.0), (173000.0, f64::INFINITY),
    ] {
        assert!(calculate("data.tone_alias_frequency", &[
            ("input_frequency", tone), ("sample_rate", rate),
        ]).is_err());
    }
}

#[test]
fn dac_complete_researched_example() {
    let out = calculate("data.unipolar_dac_output", &[
        ("bits", 12.0), ("code", 1234.0), ("reference_v", 2.5), ("gain", 2.0),
    ]).unwrap();
    assert_close(out["configured_span"], 5.0, 1e-6, 0.0);
    assert_close(out["output_v"], 1.50634765625, 1e-6, 0.0);
    assert_close(out["maximum_code_v"], 4.998779296875, 1e-6, 0.0);
}

#[test]
fn dac_zero_highest_code_resolution_and_reference_gain_scaling() {
    for (bits, code, reference, gain, span, voltage, maximum) in [
        (1.0, 0.0, 2.0, 1.0, 2.0, 0.0, 1.0),
        (1.0, 1.0, 2.0, 1.0, 2.0, 1.0, 1.0),
        (3.0, 3.0, 2.0, 1.0, 2.0, 0.75, 1.75),
        (3.0, 3.0, 4.0, 1.0, 4.0, 1.5, 3.5),
        (3.0, 3.0, 2.0, 2.0, 4.0, 1.5, 3.5),
        (32.0, 4294967295.0, 1.0, 1.0, 1.0, 1.0 - 2.0_f64.powi(-32), 1.0 - 2.0_f64.powi(-32)),
        (32.0, 1.0, 1e-9, 1e-6, 1e-15, 1e-15 / 4294967296.0, 1e-15 * (1.0 - 2.0_f64.powi(-32))),
        (1.0, 1.0, 1e6, 1000.0, 1e9, 5e8, 5e8),
    ] {
        let out = calculate("data.unipolar_dac_output", &[
            ("bits", bits), ("code", code), ("reference_v", reference), ("gain", gain),
        ]).unwrap();
        assert_close(out["configured_span"], span, 1e-6, 0.0);
        assert_close(out["output_v"], voltage, 1e-6, 0.0);
        assert_close(out["maximum_code_v"], maximum, 1e-6, 0.0);
        assert!(out["maximum_code_v"] < out["configured_span"]);
        if bits == 32.0 && reference == 1.0 {
            // Relative-only comparison at ~1 V could hide the one-LSB gap;
            // compare the small gap independently to enforce the 2^N divisor.
            assert_close(out["configured_span"] - out["output_v"], 2.0_f64.powi(-32), 1e-6, 0.0);
        }
    }
}

#[test]
fn dac_rejects_every_scalar_and_discrete_code_guard() {
    let nominal = [("bits", 12.0), ("code", 1234.0), ("reference_v", 2.5), ("gain", 2.0)];
    for (index, value) in [
        (0, 0.0), (0, 33.0), (0, 12.5),
        (1, -1.0), (1, 4294967296.0), (1, 1234.5), (1, 4096.0),
        (2, -1.0), (2, 0.0), (2, 5e-10), (2, 1000001.0),
        (3, -1.0), (3, 0.0), (3, 5e-7), (3, 1001.0),
    ] {
        let mut inputs = nominal;
        inputs[index].1 = value;
        assert!(calculate("data.unipolar_dac_output", &inputs).is_err());
    }
    for index in 0..nominal.len() {
        let mut inputs = nominal;
        inputs[index].1 = f64::NAN;
        assert!(calculate("data.unipolar_dac_output", &inputs).is_err());
    }
}

#[test]
fn sar_complete_researched_example() {
    let out = calculate("data.sar_acquisition_settling", &[
        ("bits", 12.0), ("source_resistance", 1000.0), ("switch_resistance", 20.0),
        ("sample_capacitance", 1.5e-11), ("step_fraction", 1.0),
    ]).unwrap();
    assert_close(out["time_constant_s"], 1.53e-8, 1e-6, 0.0);
    assert_close(out["required_time_constants"], 9.010913347279288, 1e-6, 0.0);
    assert_close(out["minimum_acquisition_s"], 1.3786697421337312e-7, 1e-6, 0.0);
}

#[test]
fn sar_initial_condition_log_factors_zero_branch_resistance_and_scaling() {
    for (bits, source, switch, capacitance, step, tau, log2_ratio) in [
        (12.0, 1000.0, 20.0, 1.5e-11, 0.5, 1.53e-8, 12.0),
        (12.0, 2000.0, 40.0, 1.5e-11, 1.0, 3.06e-8, 13.0),
        (12.0, 1000.0, 20.0, 3e-11, 1.0, 3.06e-8, 13.0),
        (1.0, 0.0, 100.0, 1e-12, 0.5, 1e-10, 1.0),
        (1.0, 100.0, 0.0, 1e-12, 1.0, 1e-10, 2.0),
        (32.0, 0.0, 1.0, 1e-15, 2.0_f64.powi(-32), 1e-15, 1.0),
        (32.0, 1e9, 1e9, 1e-3, 1.0, 2e6, 33.0),
    ] {
        let out = calculate("data.sar_acquisition_settling", &[
            ("bits", bits), ("source_resistance", source), ("switch_resistance", switch),
            ("sample_capacitance", capacitance), ("step_fraction", step),
        ]).unwrap();
        let factor = log2_ratio * std::f64::consts::LN_2;
        assert_close(out["time_constant_s"], tau, 1e-6, 0.0);
        assert_close(out["required_time_constants"], factor, 1e-6, 0.0);
        assert_close(out["minimum_acquisition_s"], tau * factor, 1e-6, 0.0);
        // Independently enforce the physical half-LSB residual, not just time.
        let residual = step * (-out["minimum_acquisition_s"] / tau).exp();
        assert_close(residual, 2.0_f64.powf(-bits - 1.0), 1e-6, 0.0);
    }
    // One binary64 increment above the initial-error guard is admissible.
    // ln(1+eps) ~= eps (the omitted correction is ~eps^2/2).
    let out = calculate("data.sar_acquisition_settling", &[
        ("bits", 1.0), ("source_resistance", 1.0), ("switch_resistance", 0.0),
        ("sample_capacitance", 1e-15), ("step_fraction", 0.25 + 2.0_f64.powi(-54)),
    ]).unwrap();
    assert_close(out["time_constant_s"], 1e-15, 1e-6, 0.0);
    assert_close(out["required_time_constants"], f64::EPSILON, 1e-6, 0.0);
    assert_close(out["minimum_acquisition_s"], 1e-15 * f64::EPSILON, 1e-6, 0.0);
}

#[test]
fn sar_rejects_every_scalar_initial_error_and_zero_total_resistance_guard() {
    let nominal = [
        ("bits", 12.0), ("source_resistance", 1000.0), ("switch_resistance", 20.0),
        ("sample_capacitance", 1.5e-11), ("step_fraction", 1.0),
    ];
    for (index, value) in [
        (0, 0.0), (0, 33.0), (0, 12.5),
        (1, -1.0), (1, 1e9 + 1.0), (2, -1.0), (2, 1e9 + 1.0),
        (3, 0.0), (3, 5e-16), (3, 0.001001),
        (4, 0.0), (4, 5e-13), (4, 1.01),
        (4, 1.0 / 8192.0), (4, 1.0 / 16384.0),
    ] {
        let mut inputs = nominal;
        inputs[index].1 = value;
        assert!(calculate("data.sar_acquisition_settling", &inputs).is_err());
    }
    assert!(calculate("data.sar_acquisition_settling", &[
        ("bits", 12.0), ("source_resistance", 0.0), ("switch_resistance", 0.0),
        ("sample_capacitance", 1.5e-11), ("step_fraction", 1.0),
    ]).is_err());
    for index in 0..nominal.len() {
        let mut inputs = nominal;
        inputs[index].1 = f64::INFINITY;
        assert!(calculate("data.sar_acquisition_settling", &inputs).is_err());
    }
}

#[test]
fn data_exact_input_keys_and_error_recovery() {
    assert!(calculate("data.adc_lsb", &[("bits", 12.0)]).is_err());
    assert!(calculate("data.adc_lsb", &[
        ("bits", 12.0), ("full_scale_span", 3.3), ("unexpected", 0.0),
    ]).is_err());
    assert!(calculate("data.adc_lsb", &[("bits", 12.5), ("full_scale_span", 3.3)]).is_err());
    let out = calculate("data.adc_lsb", &[("bits", 12.0), ("full_scale_span", 3.3)]).unwrap();
    assert_eq!(out["code_count"], 4096.0);
    assert_close(out["lsb_v"], 0.0008056640625, 1e-6, 0.0);
}
