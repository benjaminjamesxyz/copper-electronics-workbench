mod common;

use common::{assert_close, calculate};

// Explicit research vectors, never defaults or application-generated oracles.
// TI SLVA689: released-bus 30--70% interval is RC ln(7/3).
const I2C: [(&str, f64); 5] = [
    ("v_supply", 3.3), ("v_ol", 0.4), ("i_sink", 0.003),
    ("c_bus", 2e-10), ("rise_limit", 3e-7),
];
const SCHMITT: [(&str, f64); 7] = [
    ("r", 2000.0), ("c", 1e-9), ("v_supply", 5.0),
    ("threshold_min", 2.5), ("threshold_max", 3.75),
    ("gate_delay_min", 1e-8), ("gate_delay_max", 2e-8),
];
const TIMER: [(&str, f64); 3] = [
    ("clock_hz", 16_000_000.0), ("prescaler", 8.0), ("target_hz", 3000.0),
];
const UART: [(&str, f64); 6] = [
    ("baud_tx", 9600.0), ("baud_rx", 9700.0), ("data_bits", 8.0),
    ("parity_bits", 0.0), ("oversample", 16.0), ("guard_time", 0.0),
];
const SETUP: [(&str, f64); 6] = [
    ("clock_period", 1e-8), ("clock_to_q", 5e-10), ("path_delay", 7e-9),
    ("setup_time", 3e-10), ("clock_skew", 2e-10), ("uncertainty", 1e-10),
];
const CROSSTALK: [(&str, f64); 6] = [
    ("r_victim", 2000.0), ("c_mutual", 2e-12), ("c_victim", 8e-12),
    ("voltage_swing", 5.0), ("edge_time", 2e-8), ("segment_delay", 1e-9),
];
const SPIKE: [(&str, f64); 5] = [
    ("loop_inductance", 4e-8), ("current_initial", 5.0), ("current_final", 1.0),
    ("fall_time", 1e-8), ("supply_voltage", 12.0),
];
const PLATE: [(&str, f64); 6] = [
    ("width", 0.02), ("length", 0.03), ("gap", 0.0005),
    ("epsilon_r", 4.0), ("voltage_change", 100.0), ("edge_time", 2e-8),
];

#[test]
fn i2c_complete_researched_example() {
    let out = calculate("timing.i2c_pullup_bounds", &I2C).unwrap();
    assert_close(out["r_min"], 966.6666666666666, 1e-6, 0.0);
    assert_close(out["r_max"], 1770.3337517157427, 1e-6, 0.0);
    assert_close(out["interval_width"], 803.667085049076, 1e-6, 0.0);
}

#[test]
fn i2c_infeasible_interval_is_a_signed_design_result() {
    // ln(7/3) = 0.8472978603872037; this rise budget allows exactly 1 kohm.
    let out = calculate("timing.i2c_pullup_bounds", &[
        ("v_supply", 5.0), ("v_ol", 0.0), ("i_sink", 0.001),
        ("c_bus", 1e-9), ("rise_limit", 8.472978603872037e-7),
    ]).unwrap();
    assert_close(out["r_min"], 5000.0, 1e-12, 0.0);
    assert_close(out["r_max"], 1000.0, 1e-12, 0.0);
    assert_close(out["interval_width"], -4000.0, 1e-12, 0.0);
}

#[test]
fn i2c_zero_low_voltage_and_zero_feasibility_width_are_admissible() {
    let out = calculate("timing.i2c_pullup_bounds", &[
        ("v_supply", 1.0), ("v_ol", 0.0), ("i_sink", 0.001),
        ("c_bus", 1e-9), ("rise_limit", 8.472978603872037e-7),
    ]).unwrap();
    assert_close(out["r_min"], 1000.0, 1e-12, 0.0);
    assert_close(out["r_max"], 1000.0, 1e-12, 0.0);
    // 2e-12 ohm covers a few ulps in subtraction of the two 1-kohm bounds.
    assert_close(out["interval_width"], 0.0, 0.0, 2e-12);
}

#[test]
fn i2c_rejects_sink_and_rise_domains_and_impossible_low_level() {
    for (index, value) in [
        (0, 0.0), (0, -1.0), (1, -0.1), (1, 3.3), (1, 3.4),
        (2, 0.0), (2, -0.001), (3, 0.0), (3, -1e-12),
        (4, 0.0), (4, -1e-9),
    ] {
        let mut inputs = I2C;
        inputs[index].1 = value;
        assert!(calculate("timing.i2c_pullup_bounds", &inputs).is_err(),
            "accepted invalid {}={value}", inputs[index].0);
    }
}

#[test]
fn schmitt_complete_researched_example() {
    // Charging to 1/2 and 3/4 supply takes tau ln(2) and tau ln(4).
    let out = calculate("timing.schmitt_rc_delay_bounds", &SCHMITT).unwrap();
    assert_close(out["time_constant"], 2e-6, 1e-6, 0.0);
    assert_close(out["earliest_transition"], 1.3962943611198904e-6, 1e-6, 0.0);
    assert_close(out["latest_transition"], 2.7925887222397808e-6, 1e-6, 0.0);
    assert_close(out["timing_window"], 1.3962943611198904e-6, 1e-6, 0.0);
}

#[test]
fn schmitt_equal_thresholds_and_gate_delays_have_exact_zero_window() {
    let out = calculate("timing.schmitt_rc_delay_bounds", &[
        ("r", 1000.0), ("c", 1e-9), ("v_supply", 2.0),
        ("threshold_min", 1.0), ("threshold_max", 1.0),
        ("gate_delay_min", 0.0), ("gate_delay_max", 0.0),
    ]).unwrap();
    assert_close(out["time_constant"], 1e-6, 1e-12, 0.0);
    assert_close(out["earliest_transition"], 6.931471805599453e-7, 1e-12, 0.0);
    assert_close(out["latest_transition"], 6.931471805599453e-7, 1e-12, 0.0);
    // Identical evaluated expressions cancel exactly: no physical zero slack allowance.
    assert_close(out["timing_window"], 0.0, 0.0, 0.0);
}

#[test]
fn schmitt_tiny_thresholds_preserve_first_order_delay_via_log1p() {
    let out = calculate("timing.schmitt_rc_delay_bounds", &[
        ("r", 1.0), ("c", 1.0), ("v_supply", 1.0),
        ("threshold_min", 1e-16), ("threshold_max", 2e-16),
        ("gate_delay_min", 0.0), ("gate_delay_max", 0.0),
    ]).unwrap();
    // -ln(1-u) = u + u^2/2 + ...; next term is at most 2e-32 s.
    // No absolute tolerance may hide cancellation of these positive tiny delays.
    assert_close(out["time_constant"], 1.0, 1e-14, 0.0);
    assert_close(out["earliest_transition"], 1e-16, 1e-14, 0.0);
    assert_close(out["latest_transition"], 2e-16, 1e-14, 0.0);
    assert_close(out["timing_window"], 1e-16, 1e-14, 0.0);
}

#[test]
fn schmitt_rejects_every_ordering_log_domain_and_passive_input_guard() {
    for (index, value) in [
        (0, 0.0), (0, -1.0), (1, 0.0), (1, -1e-9),
        (2, 0.0), (2, -1.0), (3, 0.0), (3, -0.1), (3, 4.0),
        (4, 0.0), (4, -0.1), (4, 5.0), (4, 5.1),
        (5, -1e-9), (5, 3e-8), (6, -1e-9),
    ] {
        let mut inputs = SCHMITT;
        inputs[index].1 = value;
        assert!(calculate("timing.schmitt_rc_delay_bounds", &inputs).is_err(),
            "accepted invalid {}={value}", inputs[index].0);
    }
}

#[test]
fn timer_complete_researched_example() {
    let out = calculate("timing.timer_clock_divider", &TIMER).unwrap();
    assert_eq!(out["count"], 333.0);
    assert_eq!(out["compare_value"], 332.0);
    assert_close(out["actual_hz"], 3003.003003003003, 1e-6, 0.0);
    assert_close(out["error_percent"], 0.1001001001001001, 1e-6, 0.0);
    assert_close(out["output_period"], 0.000333, 1e-6, 0.0);
}

#[test]
fn timer_half_count_tie_rounds_period_up_not_frequency_error_down() {
    // Ideal count 2.5: choose 3, even though count 2 gives a different frequency error.
    let out = calculate("timing.timer_clock_divider", &[
        ("clock_hz", 10.0), ("prescaler", 1.0), ("target_hz", 2.0),
    ]).unwrap();
    assert_eq!(out["count"], 3.0);
    assert_eq!(out["compare_value"], 2.0);
    assert_close(out["actual_hz"], 5.0 / 3.0, 1e-12, 0.0);
    assert_close(out["error_percent"], -100.0 / 6.0, 1e-12, 0.0);
    assert_close(out["output_period"], 0.6, 1e-12, 0.0);
}

#[test]
fn timer_supports_all_prescalers_and_inclusive_register_endpoints() {
    for prescaler in [1.0, 8.0, 64.0, 256.0, 1024.0] {
        // Maximum output frequency has count=1 and OCR=0, not count=0.
        let out = calculate("timing.timer_clock_divider", &[
            ("clock_hz", 2048.0), ("prescaler", prescaler),
            ("target_hz", 1024.0 / prescaler),
        ]).unwrap();
        assert_eq!(out["count"], 1.0);
        assert_eq!(out["compare_value"], 0.0);
        assert_close(out["actual_hz"], 1024.0 / prescaler, 1e-12, 0.0);
        assert_close(out["error_percent"], 0.0, 0.0, 0.0);
        assert_close(out["output_period"], prescaler / 1024.0, 1e-12, 0.0);
    }
    let out = calculate("timing.timer_clock_divider", &[
        ("clock_hz", 131072.0), ("prescaler", 1.0), ("target_hz", 1.0),
    ]).unwrap();
    assert_eq!(out["count"], 65536.0);
    assert_eq!(out["compare_value"], 65535.0);
    assert_close(out["actual_hz"], 1.0, 1e-12, 0.0);
    assert_close(out["error_percent"], 0.0, 0.0, 0.0);
    assert_close(out["output_period"], 1.0, 1e-12, 0.0);
}

#[test]
fn timer_rejects_invalid_prescalers_maximum_frequency_and_rounded_overflow() {
    for (index, value) in [
        (0, 0.0), (0, -1.0), (1, 0.0), (1, 1025.0),
        (1, 2.0), (1, 8.5), (2, 0.0), (2, -1.0), (2, 1_000_001.0),
        (2, 1.0),
    ] {
        let mut inputs = TIMER;
        inputs[index].1 = value;
        assert!(calculate("timing.timer_clock_divider", &inputs).is_err(),
            "accepted invalid {}={value}", inputs[index].0);
    }
    // Exact half-count tie at 65536.5 must overflow after upward rounding.
    assert!(calculate("timing.timer_clock_divider", &[
        ("clock_hz", 131073.0), ("prescaler", 1.0), ("target_hz", 1.0),
    ]).is_err());
}

#[test]
fn uart_complete_researched_example() {
    let out = calculate("timing.uart_sampling_margin", &UART).unwrap();
    assert_close(out["mismatch_percent"], 1.0416666666666667, 1e-6, 0.0);
    assert_close(out["stop_center_time"], 0.000979381443298969, 1e-6, 0.0);
    assert_close(out["sync_quantization"], 6.443298969072165e-6, 1e-6, 0.0);
    assert_close(out["early_margin"], 4.188144329896905e-5, 1e-6, 0.0);
    assert_close(out["late_margin"], 5.5841924398625466e-5, 1e-6, 0.0);
    assert_close(out["minimum_margin"], 4.188144329896905e-5, 1e-6, 0.0);
}

#[test]
fn uart_equal_baud_parity_and_guard_have_independent_tick_budget() {
    // Five data bits plus parity: center at 7.5 bits. 8x sync consumes 1/8 bit.
    let out = calculate("timing.uart_sampling_margin", &[
        ("baud_tx", 1000.0), ("baud_rx", 1000.0), ("data_bits", 5.0),
        ("parity_bits", 1.0), ("oversample", 8.0), ("guard_time", 0.000125),
    ]).unwrap();
    assert_close(out["mismatch_percent"], 0.0, 0.0, 0.0);
    assert_close(out["stop_center_time"], 0.0075, 1e-12, 0.0);
    assert_close(out["sync_quantization"], 0.000125, 1e-12, 0.0);
    assert_close(out["early_margin"], 0.000375, 1e-12, 0.0);
    assert_close(out["late_margin"], 0.00025, 1e-12, 0.0);
    assert_close(out["minimum_margin"], 0.00025, 1e-12, 0.0);
}

#[test]
fn uart_both_mismatch_signs_keep_negative_framing_margins() {
    for (rx, mismatch, center, sync, early, late, minimum) in [
        (500.0, -50.0, 0.019, 0.000125, 0.010, -0.009125, -0.009125),
        (2000.0, 100.0, 0.00475, 0.00003125, -0.00425, 0.00521875, -0.00425),
    ] {
        let out = calculate("timing.uart_sampling_margin", &[
            ("baud_tx", 1000.0), ("baud_rx", rx), ("data_bits", 8.0),
            ("parity_bits", 0.0), ("oversample", 16.0), ("guard_time", 0.0),
        ]).unwrap();
        assert_close(out["mismatch_percent"], mismatch, 1e-12, 0.0);
        assert_close(out["stop_center_time"], center, 1e-12, 0.0);
        assert_close(out["sync_quantization"], sync, 1e-12, 0.0);
        assert_close(out["early_margin"], early, 1e-12, 0.0);
        assert_close(out["late_margin"], late, 1e-12, 0.0);
        assert_close(out["minimum_margin"], minimum, 1e-12, 0.0);
    }
}

#[test]
fn uart_zero_margin_at_maximum_frame_length_is_admissible() {
    // All values are binary fractions: guard=3/8 bit leaves the late margin zero.
    let out = calculate("timing.uart_sampling_margin", &[
        ("baud_tx", 1024.0), ("baud_rx", 1024.0), ("data_bits", 9.0),
        ("parity_bits", 1.0), ("oversample", 8.0), ("guard_time", 0.0003662109375),
    ]).unwrap();
    assert_close(out["mismatch_percent"], 0.0, 0.0, 0.0);
    assert_close(out["stop_center_time"], 0.01123046875, 1e-12, 0.0);
    assert_close(out["sync_quantization"], 0.0001220703125, 1e-12, 0.0);
    assert_close(out["early_margin"], 0.0001220703125, 1e-12, 0.0);
    assert_close(out["late_margin"], 0.0, 0.0, 0.0);
    assert_close(out["minimum_margin"], 0.0, 0.0, 0.0);
}

#[test]
fn uart_rejects_fractional_discrete_fields_and_all_input_boundaries() {
    for (index, value) in [
        (0, 0.0), (0, -1.0), (1, 0.0), (1, -1.0),
        (2, 4.0), (2, 10.0), (2, 7.5), (3, -1.0), (3, 2.0), (3, 0.5),
        (4, 7.0), (4, 17.0), (4, 12.0), (4, 8.5), (5, -1e-9),
    ] {
        let mut inputs = UART;
        inputs[index].1 = value;
        assert!(calculate("timing.uart_sampling_margin", &inputs).is_err(),
            "accepted invalid {}={value}", inputs[index].0);
    }
}

#[test]
fn setup_complete_researched_example() {
    let out = calculate("timing.synchronous_setup_slack", &SETUP).unwrap();
    assert_close(out["available_time"], 9.8e-9, 1e-6, 0.0);
    assert_close(out["arrival_time"], 7.5e-9, 1e-6, 0.0);
    assert_close(out["setup_slack"], 2.3e-9, 1e-6, 0.0);
}

#[test]
fn setup_negative_library_setup_skew_and_slack_remain_signed() {
    let out = calculate("timing.synchronous_setup_slack", &[
        ("clock_period", 8e-9), ("clock_to_q", 1e-9), ("path_delay", 8e-9),
        ("setup_time", -5e-10), ("clock_skew", -5e-10), ("uncertainty", 0.0),
    ]).unwrap();
    assert_close(out["available_time"], 8e-9, 1e-12, 0.0);
    assert_close(out["arrival_time"], 9e-9, 1e-12, 0.0);
    assert_close(out["setup_slack"], -1e-9, 1e-12, 0.0);
    // Positive capture-later skew helps setup by exactly the change in skew.
    let out = calculate("timing.synchronous_setup_slack", &[
        ("clock_period", 8e-9), ("clock_to_q", 1e-9), ("path_delay", 8e-9),
        ("setup_time", -5e-10), ("clock_skew", 5e-10), ("uncertainty", 0.0),
    ]).unwrap();
    assert_close(out["available_time"], 9e-9, 1e-12, 0.0);
    assert_close(out["arrival_time"], 9e-9, 1e-12, 0.0);
    // 1e-23 s is a few ulps at the 9-ns operand scale, not a timing margin.
    assert_close(out["setup_slack"], 0.0, 0.0, 1e-23);
}

#[test]
fn setup_zero_path_clock_to_q_uncertainty_and_signed_available_time() {
    let out = calculate("timing.synchronous_setup_slack", &[
        ("clock_period", 8e-9), ("clock_to_q", 0.0), ("path_delay", 0.0),
        ("setup_time", 0.0), ("clock_skew", -1e-8), ("uncertainty", 0.0),
    ]).unwrap();
    assert_close(out["available_time"], -2e-9, 1e-12, 0.0);
    assert_close(out["arrival_time"], 0.0, 0.0, 0.0);
    assert_close(out["setup_slack"], -2e-9, 1e-12, 0.0);
}

#[test]
fn setup_rejects_nonpositive_period_and_negative_unsigned_delays() {
    for (index, value) in [(0, 0.0), (0, -1e-9), (1, -1e-9), (2, -1e-9), (5, -1e-9)] {
        let mut inputs = SETUP;
        inputs[index].1 = value;
        assert!(calculate("timing.synchronous_setup_slack", &inputs).is_err(),
            "accepted invalid {}={value}", inputs[index].0);
    }
}

#[test]
fn crosstalk_complete_researched_example() {
    // Victim KCL gives 1 V times (1-exp(-1)) for edge_time=tau.
    let out = calculate("timing.capacitive_crosstalk_peak", &CROSSTALK).unwrap();
    assert_close(out["victim_time_constant"], 2e-8, 1e-6, 0.0);
    assert_close(out["peak_excursion"], 0.6321205588285577, 1e-6, 0.0);
    assert_close(out["fast_edge_limit"], 1.0, 1e-6, 0.0);
}

#[test]
fn crosstalk_two_time_constant_edge_and_lumped_delay_equality() {
    let out = calculate("timing.capacitive_crosstalk_peak", &[
        ("r_victim", 2000.0), ("c_mutual", 2e-12), ("c_victim", 8e-12),
        ("voltage_swing", 5.0), ("edge_time", 4e-8), ("segment_delay", 4e-9),
    ]).unwrap();
    // Integral of exponential response over a two-tau ramp: (1-e^-2)/2.
    assert_close(out["victim_time_constant"], 2e-8, 1e-12, 0.0);
    assert_close(out["peak_excursion"], 0.43233235838169365, 1e-12, 0.0);
    assert_close(out["fast_edge_limit"], 1.0, 1e-12, 0.0);
    assert!(out["peak_excursion"] <= out["fast_edge_limit"]);
}

#[test]
fn crosstalk_fast_edge_ratio_one_e_minus_sixteen_reaches_divider_limit() {
    for (swing, limit) in [(5.0, 1.0), (1e-300, 2e-301)] {
        let out = calculate("timing.capacitive_crosstalk_peak", &[
            ("r_victim", 2000.0), ("c_mutual", 2e-12), ("c_victim", 8e-12),
            ("voltage_swing", swing), ("edge_time", 2e-24), ("segment_delay", 0.0),
        ]).unwrap();
        // Attenuation=1-x/2+O(x^2); x=1e-16. The correction is below one ulp.
        // Tiny swing also detects premature underflow in divider*expm1 before /x.
        assert_close(out["victim_time_constant"], 2e-8, 4e-15, 0.0);
        assert_close(out["peak_excursion"], limit, 4e-15, 0.0);
        assert_close(out["fast_edge_limit"], limit, 4e-15, 0.0);
        assert!(out["peak_excursion"] <= out["fast_edge_limit"]);
    }
}

#[test]
fn crosstalk_zero_coupling_and_zero_swing_produce_exact_zero_excursion() {
    for (mutual, swing, tau) in [(0.0, 5.0, 1.6e-8), (2e-12, 0.0, 2e-8)] {
        let out = calculate("timing.capacitive_crosstalk_peak", &[
            ("r_victim", 2000.0), ("c_mutual", mutual), ("c_victim", 8e-12),
            ("voltage_swing", swing), ("edge_time", 2e-8), ("segment_delay", 0.0),
        ]).unwrap();
        assert_close(out["victim_time_constant"], tau, 1e-12, 0.0);
        assert_close(out["peak_excursion"], 0.0, 0.0, 0.0);
        assert_close(out["fast_edge_limit"], 0.0, 0.0, 0.0);
    }
}

#[test]
fn crosstalk_rejects_invalid_loading_edges_and_distributed_segments() {
    for (index, value) in [
        (0, 0.0), (0, -1.0), (1, -1e-12), (2, 0.0), (2, -1e-12),
        (3, -1.0), (4, 0.0), (4, -1e-9), (5, -1e-9), (5, 2.01e-9),
    ] {
        let mut inputs = CROSSTALK;
        inputs[index].1 = value;
        assert!(calculate("timing.capacitive_crosstalk_peak", &inputs).is_err(),
            "accepted invalid {}={value}", inputs[index].0);
    }
    // Physically positive but x underflows to zero: explicit arithmetic failure,
    // rather than silently treating 0/0 as a zero excursion.
    assert!(calculate("timing.capacitive_crosstalk_peak", &[
        ("r_victim", 1e100), ("c_mutual", 2e-12), ("c_victim", 8e-12),
        ("voltage_swing", 5.0), ("edge_time", 1e-300), ("segment_delay", 0.0),
    ]).is_err());
}

#[test]
fn crosstalk_rejects_overflowed_edge_ratio_instead_of_masking_it_as_zero() {
    // tau=2e-300 s is finite, but edge/tau=5e309 overflows binary64.
    // expm1(-infinity)=-1 can mask that invalid intermediate as peak=0.
    // The analytic slow-ramp peak is R*Cm*DeltaV/t_edge ~= 1e-310 V.
    // Per the documented finite-intermediate contract, this realization must
    // explicitly reject the exhausted ratio rather than report a valid zero.
    assert!(calculate("timing.capacitive_crosstalk_peak", &[
        ("r_victim", 1e-200), ("c_mutual", 1e-100), ("c_victim", 1e-100),
        ("voltage_swing", 1.0), ("edge_time", 1e10), ("segment_delay", 0.0),
    ]).is_err());
}

#[test]
fn spike_complete_researched_example() {
    let out = calculate("timing.inductive_turnoff_spike", &SPIKE).unwrap();
    assert_close(out["current_fall_rate"], 400_000_000.0, 1e-6, 0.0);
    assert_close(out["spike_voltage"], 16.0, 1e-6, 0.0);
    assert_close(out["switch_voltage"], 28.0, 1e-6, 0.0);
}

#[test]
fn spike_full_current_turnoff_zero_supply_has_positive_low_side_polarity() {
    let out = calculate("timing.inductive_turnoff_spike", &[
        ("loop_inductance", 1e-6), ("current_initial", 2.0), ("current_final", 0.0),
        ("fall_time", 1e-6), ("supply_voltage", 0.0),
    ]).unwrap();
    assert_close(out["current_fall_rate"], 2e6, 1e-12, 0.0);
    assert_close(out["spike_voltage"], 2.0, 1e-12, 0.0);
    assert_close(out["switch_voltage"], 2.0, 1e-12, 0.0);
}

#[test]
fn spike_equal_currents_and_zero_inductance_are_admissible() {
    for current in [0.0, 3.0] {
        let out = calculate("timing.inductive_turnoff_spike", &[
            ("loop_inductance", 4e-8), ("current_initial", current),
            ("current_final", current), ("fall_time", 1e-8), ("supply_voltage", 12.0),
        ]).unwrap();
        assert_close(out["current_fall_rate"], 0.0, 0.0, 0.0);
        assert_close(out["spike_voltage"], 0.0, 0.0, 0.0);
        assert_close(out["switch_voltage"], 12.0, 1e-12, 0.0);
    }
    let out = calculate("timing.inductive_turnoff_spike", &[
        ("loop_inductance", 0.0), ("current_initial", 5.0), ("current_final", 1.0),
        ("fall_time", 1e-8), ("supply_voltage", 12.0),
    ]).unwrap();
    assert_close(out["current_fall_rate"], 4e8, 1e-12, 0.0);
    assert_close(out["spike_voltage"], 0.0, 0.0, 0.0);
    assert_close(out["switch_voltage"], 12.0, 1e-12, 0.0);
}

#[test]
fn spike_rejects_increasing_current_nonpositive_fall_time_and_negative_magnitudes() {
    for (index, value) in [
        (0, -1e-9), (1, -1.0), (1, 0.5), (2, -1.0), (2, 6.0),
        (3, 0.0), (3, -1e-9), (4, -1.0),
    ] {
        let mut inputs = SPIKE;
        inputs[index].1 = value;
        assert!(calculate("timing.inductive_turnoff_spike", &inputs).is_err(),
            "accepted invalid {}={value}", inputs[index].0);
    }
}

#[test]
fn plate_complete_researched_example() {
    // Maxwell J=epsilon/gap*dV/dt integrated over the 0.0006 m^2 overlap.
    let out = calculate("timing.plate_displacement_current", &PLATE).unwrap();
    assert_close(out["overlap_capacitance"], 4.250010150144e-11, 1e-6, 0.0);
    assert_close(out["voltage_slew"], 5e9, 1e-6, 0.0);
    assert_close(out["displacement_current"], 0.2125005075072, 1e-6, 0.0);
    assert_close(out["current_density"], 354.167512512, 1e-6, 0.0);
}

#[test]
fn plate_falling_edge_and_swapped_dimensions_preserve_density_and_current_sign() {
    let out = calculate("timing.plate_displacement_current", &[
        ("width", 0.03), ("length", 0.02), ("gap", 0.0005),
        ("epsilon_r", 4.0), ("voltage_change", -100.0), ("edge_time", 4e-8),
    ]).unwrap();
    assert_close(out["overlap_capacitance"], 4.250010150144e-11, 1e-12, 0.0);
    assert_close(out["voltage_slew"], -2.5e9, 1e-12, 0.0);
    assert_close(out["displacement_current"], -0.1062502537536, 1e-12, 0.0);
    assert_close(out["current_density"], -177.083756256, 1e-12, 0.0);
}

#[test]
fn plate_vacuum_geometry_endpoint_and_zero_voltage_change_are_admissible() {
    for (change, slew, current, density) in [
        (1.0, 1e8, 8.8541878128e-5, 0.88541878128),
        (0.0, 0.0, 0.0, 0.0),
    ] {
        let out = calculate("timing.plate_displacement_current", &[
            ("width", 0.01), ("length", 0.01), ("gap", 0.001),
            ("epsilon_r", 1.0), ("voltage_change", change), ("edge_time", 1e-8),
        ]).unwrap();
        assert_close(out["overlap_capacitance"], 8.8541878128e-13, 1e-12, 0.0);
        // Zero current is an exact constitutive-law zero; no absolute allowance.
        assert_close(out["voltage_slew"], slew, 1e-12, 0.0);
        assert_close(out["displacement_current"], current, 1e-12, 0.0);
        assert_close(out["current_density"], density, 1e-12, 0.0);
    }
}

#[test]
fn plate_quasistatic_size_equality_is_admissible() {
    // Independent model-selection endpoint c*t/10 in vacuum.
    let side = 299_792_458.0 * 1e-8 / 10.0;
    let out = calculate("timing.plate_displacement_current", &[
        ("width", side), ("length", side), ("gap", 0.001),
        ("epsilon_r", 1.0), ("voltage_change", 1.0), ("edge_time", 1e-8),
    ]).unwrap();
    // Oracles from E=V/gap, D=epsilon E, J=dD/dt; current is J times area.
    let density = 0.88541878128;
    let current = density * side * side;
    assert_close(out["overlap_capacitance"], current * 1e-8, 1e-12, 0.0);
    assert_close(out["voltage_slew"], 1e8, 1e-12, 0.0);
    assert_close(out["displacement_current"], current, 1e-12, 0.0);
    assert_close(out["current_density"], density, 1e-12, 0.0);
}

#[test]
fn plate_rejects_nonphysical_geometry_dielectric_and_both_model_guards() {
    for (index, value) in [
        (0, 0.0), (0, -0.01), (1, 0.0), (1, -0.01),
        (2, 0.0), (2, -0.001), (2, 0.00201),
        (3, 0.99), (5, 0.0), (5, -1e-9), (5, 1e-10),
        // Test each dimension independently for fringing and electrical size.
        (0, 0.004), (1, 0.004), (0, 0.31), (1, 0.31),
    ] {
        let mut inputs = PLATE;
        inputs[index].1 = value;
        assert!(calculate("timing.plate_displacement_current", &inputs).is_err(),
            "accepted invalid {}={value}", inputs[index].0);
    }
}
