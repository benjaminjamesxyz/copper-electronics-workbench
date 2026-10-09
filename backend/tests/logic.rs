mod common;
use common::{assert_close, calculate};

// Independent SI vectors, not catalog defaults. Exact zero/integer boundaries use
// binary-exact inputs; tiny nonzero outputs never receive an absolute floor.
const REL: f64 = 1e-10;
const NOISE: [(&str, f64); 7] = [
    ("output_high_min", 3.25), ("output_low_max", 0.25),
    ("input_high_min", 2.5), ("input_low_max", 0.75),
    ("ground_offset", 0.125), ("high_noise", 0.125), ("low_noise", 0.25),
];
const FAN: [(&str, f64); 5] = [
    ("source_limit", 0.015625), ("sink_limit", 0.0234375),
    ("receiver_high_current", 0.001953125), ("receiver_low_current", 0.00390625),
    ("load_count", 7.0),
];
const POWER: [(&str, f64); 7] = [
    ("supply_voltage", 2.5), ("quiescent_current", 4e-6), ("active_sections", 3.0),
    ("power_capacitance", 8e-12), ("input_cycle_rate", 5e6),
    ("load_capacitance", 12e-12), ("output_cycle_rate", 2e6),
];
const META: [(&str, f64); 7] = [
    ("clock_frequency", 1048576.0), ("data_transition_rate", 1024.0),
    ("aperture_parameter", 9.313225746154785e-10), ("resolution_constant", 1e-10),
    ("resolution_time", 8e-10), ("chain_count", 16.0), ("target_system_mtbf", 64.0),
];
const EFFORT: [(&str, f64); 7] = [
    ("first_input_capacitance", 1e-15), ("final_load_capacitance", 24e-15),
    ("first_logical_effort", 2.0), ("second_logical_effort", 1.5),
    ("third_logical_effort", 3.0), ("path_parasitic_delay", 6.0),
    ("process_delay_unit", 8e-12),
];
const FIFO: [(&str, f64); 5] = [
    ("burst_words", 7.0), ("arrival_rate", 3.0), ("service_rate", 8.0),
    ("service_latency", 0.625), ("word_bits", 12.0),
];

#[test]
fn noise_complete_independent_voltage_budget() {
    let out = calculate("logic.guaranteed_noise_margins", &NOISE).unwrap();
    for (key, expected) in [("high_margin", 0.5), ("low_margin", 0.375), ("minimum_margin", 0.375)] {
        assert_close(out[key], expected, REL, 0.0);
    }
    // The common reference shift redistributes but cannot change total allowance.
    assert_close(out["high_margin"] + out["low_margin"], 0.875, REL, 0.0);
}

#[test]
fn noise_ground_sign_zero_and_negative_diagnostics() {
    for (offset, high, low, worst) in [
        (-0.125, 0.75, 0.125, 0.125), (0.625, 0.0, 0.875, 0.0),
        (0.75, -0.125, 1.0, -0.125), (-0.5, 1.125, -0.25, -0.25),
    ] {
        let mut inputs = NOISE;
        inputs[4].1 = offset;
        let out = calculate("logic.guaranteed_noise_margins", &inputs).unwrap();
        for (key, expected) in [("high_margin", high), ("low_margin", low), ("minimum_margin", worst)] {
            assert_close(out[key], expected, REL, 0.0);
        }
    }
    let mut inputs = NOISE;
    for index in [4, 5, 6] { inputs[index].1 = 0.0; }
    let out = calculate("logic.guaranteed_noise_margins", &inputs).unwrap();
    assert_close(out["high_margin"], 0.75, REL, 0.0);
    assert_close(out["low_margin"], 0.5, REL, 0.0);
    assert_close(out["minimum_margin"], 0.5, REL, 0.0);
}

#[test]
fn noise_rejects_bad_levels_and_recovers() {
    for (index, bad) in [(0, -1.0), (1, -1.0), (2, -1.0), (3, -1.0),
        (5, -0.1), (6, -0.1), (0, 0.25), (0, 0.125), (2, 0.75), (2, 0.5)] {
        let mut inputs = NOISE;
        inputs[index].1 = bad;
        assert!(calculate("logic.guaranteed_noise_margins", &inputs).is_err());
        noise_complete_independent_voltage_budget();
    }
}

#[test]
fn fanout_complete_independent_current_conservation() {
    let out = calculate("logic.dc_current_fanout", &FAN).unwrap();
    for (key, expected) in [("high_fanout", 8.0), ("low_fanout", 6.0), ("maximum_fanout", 6.0),
        ("source_headroom", 0.001953125), ("sink_headroom", -0.00390625)] {
        assert_close(out[key], expected, REL, 0.0);
    }
}

#[test]
fn fanout_floor_neighbors_exhaustion_and_zero_capability() {
    let threshold: f64 = 0.015625;
    for (source, high) in [(f64::from_bits(threshold.to_bits() - 1), 7.0),
        (threshold, 8.0), (f64::from_bits(threshold.to_bits() + 1), 8.0)] {
        let mut inputs = FAN;
        inputs[0].1 = source;
        let out = calculate("logic.dc_current_fanout", &inputs).unwrap();
        assert_eq!(out["high_fanout"], high);
        assert_eq!(out["low_fanout"], 6.0);
        assert_eq!(out["maximum_fanout"], 6.0);
    }
    for (loads, source, sink) in [(0.0, 0.015625, 0.0234375), (6.0, 0.00390625, 0.0),
        (8.0, 0.0, -0.0078125), (9.0, -0.001953125, -0.01171875)] {
        let mut inputs = FAN;
        inputs[4].1 = loads;
        let out = calculate("logic.dc_current_fanout", &inputs).unwrap();
        assert_close(out["source_headroom"], source, REL, 0.0);
        assert_close(out["sink_headroom"], sink, REL, 0.0);
        assert_eq!(out["maximum_fanout"], 6.0);
    }
    let mut inputs = FAN;
    for index in [0, 1, 4] { inputs[index].1 = 0.0; }
    let out = calculate("logic.dc_current_fanout", &inputs).unwrap();
    for key in ["high_fanout", "low_fanout", "maximum_fanout", "source_headroom", "sink_headroom"] {
        assert_eq!(out[key], 0.0);
    }
    // High-state bottleneck, including a nonintegral current ratio.
    inputs = FAN;
    inputs[0].1 = 0.0107421875;
    let out = calculate("logic.dc_current_fanout", &inputs).unwrap();
    assert_eq!(out["high_fanout"], 5.0);
    assert_eq!(out["maximum_fanout"], 5.0);
}

#[test]
fn fanout_rejects_invalid_magnitudes_and_fractional_counts_with_recovery() {
    for (index, bad) in [(0, -1.0), (1, -1.0), (2, 0.0), (2, -1.0),
        (3, 0.0), (3, -1.0), (4, -1.0), (4, 6.5)] {
        let mut inputs = FAN;
        inputs[index].1 = bad;
        assert!(calculate("logic.dc_current_fanout", &inputs).is_err());
        fanout_complete_independent_current_conservation();
    }
}

#[test]
fn power_complete_independent_full_cycle_energy() {
    // Three sections consume 120 uA internally and 72 uA in loads at 1 V.
    // Scaling charge by 2.5 V gives 300+180 uA; package leakage adds 4 uA once.
    let out = calculate("logic.cmos_package_power", &POWER).unwrap();
    for (key, expected) in [("static_power", 1e-5), ("internal_power", 0.00075),
        ("load_power", 0.00045), ("total_power", 0.00121), ("average_current", 0.000484)] {
        assert_close(out[key], expected, REL, 0.0);
    }
}

#[test]
fn power_zero_activity_no_load_and_tiny_charge() {
    for (fi, fo, iq, cpd, cl, expected) in [
        (0.0, 0.0, 4e-6, 8e-12, 12e-12, [1e-5, 0.0, 0.0, 1e-5, 4e-6]),
        (5e6, 0.0, 4e-6, 8e-12, 12e-12, [1e-5, 0.00075, 0.0, 0.00076, 0.000304]),
        (5e6, 2e6, 0.0, 8e-12, 0.0, [0.0, 0.00075, 0.0, 0.00075, 0.0003]),
        (5e6, 2e6, 0.0, 0.0, 0.0, [0.0; 5]),
        (5e6, 2e6, 0.0, 8e-30, 0.0, [0.0, 7.5e-22, 0.0, 7.5e-22, 3e-22]),
    ] {
        let mut inputs = POWER;
        inputs[1].1 = iq; inputs[3].1 = cpd; inputs[4].1 = fi;
        inputs[5].1 = cl; inputs[6].1 = fo;
        let out = calculate("logic.cmos_package_power", &inputs).unwrap();
        for (key, value) in ["static_power", "internal_power", "load_power", "total_power", "average_current"].into_iter().zip(expected) {
            assert_close(out[key], value, REL, 0.0);
        }
    }
}

#[test]
fn power_rate_voltage_and_section_scaling() {
    for (voltage, sections, rate, static_power, internal, load, current) in [
        (2.5, 3.0, 2.0, 1e-5, 0.0015, 0.0009, 0.000964),
        (5.0, 3.0, 1.0, 2e-5, 0.003, 0.0018, 0.000964),
        (2.5, 6.0, 1.0, 1e-5, 0.0015, 0.0009, 0.000964),
    ] {
        let mut inputs = POWER;
        inputs[0].1 = voltage; inputs[2].1 = sections;
        inputs[4].1 *= rate; inputs[6].1 *= rate;
        let out = calculate("logic.cmos_package_power", &inputs).unwrap();
        for (key, value) in [("static_power", static_power), ("internal_power", internal),
            ("load_power", load), ("total_power", static_power + internal + load), ("average_current", current)] {
            assert_close(out[key], value, REL, 0.0);
        }
    }
}

#[test]
fn power_rejects_invalid_bounds_and_recovers() {
    for (index, bad) in [(0, 0.0), (0, -1.0), (1, -1.0), (2, 0.0), (2, 1.5),
        (2, -1.0), (3, -1.0), (4, -1.0), (5, -1.0), (6, -1.0)] {
        let mut inputs = POWER;
        inputs[index].1 = bad;
        assert!(calculate("logic.cmos_package_power", &inputs).is_err());
        power_complete_independent_full_cycle_energy();
    }
}

#[test]
fn metastability_complete_independent_failure_probability() {
    // Powers of two give one aperture encounter per second. An encounter survives
    // eight regeneration constants with probability exp(-8); sixteen rates add.
    let probability: f64 = (-8.0_f64).exp();
    let chain_seconds: f64 = 1.0 / probability;
    let required: f64 = 1e-10 * 1024.0_f64.ln();
    let out = calculate("logic.synchronizer_metastability", &META).unwrap();
    for (key, value) in [("log10_chain_mtbf", chain_seconds.log10()),
        ("log10_system_mtbf", (chain_seconds / 16.0).log10()),
        ("required_resolution_time", required), ("resolution_margin", 8e-10 - required)] {
        assert_close(out[key], value, REL, 0.0);
    }
}

#[test]
fn metastability_zero_short_long_and_target_floor() {
    // Use a 2/s encounter rate so zero-time logarithms are nonzero, avoiding a
    // tolerance floor on logarithmic cancellation around an exact zero.
    for time in [0.0, 2e-10, 1e-6] {
        let mut inputs = META;
        inputs[0].1 *= 2.0; inputs[4].1 = time;
        let required: f64 = 1e-10 * 2048.0_f64.ln();
        let chain: f64 = time / 1e-10 / std::f64::consts::LN_10 - 2.0_f64.log10();
        let out = calculate("logic.synchronizer_metastability", &inputs).unwrap();
        for (key, expected) in [("log10_chain_mtbf", chain),
            ("log10_system_mtbf", chain - 32.0_f64.log10() + 2.0_f64.log10()),
            ("required_resolution_time", required), ("resolution_margin", time - required)] {
            assert_close(out[key], expected, REL, 0.0);
        }
    }
    let mut inputs = META;
    inputs[6].1 = 0.001;
    let out = calculate("logic.synchronizer_metastability", &inputs).unwrap();
    assert_eq!(out["required_resolution_time"], 0.0);
    assert_eq!(out["resolution_margin"], 8e-10);
    assert_close(out["log10_chain_mtbf"], 8.0 / std::f64::consts::LN_10, REL, 0.0);
    assert_close(out["log10_system_mtbf"], 8.0 / std::f64::consts::LN_10 - 16.0_f64.log10(), REL, 0.0);
}

#[test]
fn metastability_independent_chain_rate_and_target_scaling() {
    for (count, target) in [(1.0, 64.0), (16.0, 640.0), (160.0, 64.0)] {
        let mut inputs = META;
        inputs[5].1 = count; inputs[6].1 = target;
        let system_seconds: f64 = 8.0_f64.exp() / count;
        let required: f64 = 1e-10 * (count * target).ln();
        let out = calculate("logic.synchronizer_metastability", &inputs).unwrap();
        assert_close(out["log10_chain_mtbf"], 8.0_f64.exp().log10(), REL, 0.0);
        assert_close(out["log10_system_mtbf"], system_seconds.log10(), REL, 0.0);
        assert_close(out["required_resolution_time"], required, REL, 0.0);
        assert_close(out["resolution_margin"], 8e-10 - required, REL, 0.0);
    }
}

#[test]
fn metastability_rejects_bounds_aperture_equality_and_recovers() {
    for (index, bad) in [(0, 0.0), (0, -1.0), (1, 0.0), (1, -1.0),
        (2, 0.0), (2, -1.0), (3, 0.0), (3, -1.0), (4, -1e-10),
        (5, 0.0), (5, 1.5), (6, 0.0), (6, -1.0), (2, 0.0009765625), (2, 0.001953125)] {
        let mut inputs = META;
        inputs[index].1 = bad;
        assert!(calculate("logic.synchronizer_metastability", &inputs).is_err());
        metastability_complete_independent_failure_probability();
    }
}

#[test]
fn logical_effort_complete_unequal_gate_solution() {
    // Direct stage efforts: 2*(3/1), 1.5*(12/3), 3*(24/12), each six.
    let out = calculate("logic.three_stage_logical_effort", &EFFORT).unwrap();
    for (key, value) in [("optimal_stage_effort", 6.0), ("second_input_capacitance", 3e-15),
        ("third_input_capacitance", 12e-15), ("normalized_path_delay", 24.0), ("path_delay", 192e-12)] {
        assert_close(out[key], value, REL, 0.0);
    }
}

#[test]
fn logical_effort_minimum_from_direct_stage_delay_objective() {
    let out = calculate("logic.three_stage_logical_effort", &EFFORT).unwrap();
    let c2 = out["second_input_capacitance"];
    let c3 = out["third_input_capacitance"];
    for effort in [2.0 * c2 / 1e-15, 1.5 * c3 / c2, 3.0 * 24e-15 / c3] {
        assert_close(effort, 6.0, REL, 0.0);
    }
    for a in [0.5, 0.9, 1.1, 2.0] {
        for b in [0.5, 0.9, 1.1, 2.0] {
            let delay = 2.0 * (c2 * a) / 1e-15 + 1.5 * (c3 * b) / (c2 * a)
                + 3.0 * 24e-15 / (c3 * b) + 6.0;
            assert!(delay > out["normalized_path_delay"], "perturbation improved claimed optimum");
        }
    }
}

#[test]
fn logical_effort_scaling_and_zero_parasitic_delay() {
    for scale in [1e-12, 1e3, 1e12] {
        let mut inputs = EFFORT;
        inputs[0].1 *= scale; inputs[1].1 *= scale;
        let out = calculate("logic.three_stage_logical_effort", &inputs).unwrap();
        assert_close(out["optimal_stage_effort"], 6.0, REL, 0.0);
        assert_close(out["second_input_capacitance"], 3e-15 * scale, REL, 0.0);
        assert_close(out["third_input_capacitance"], 12e-15 * scale, REL, 0.0);
        assert_close(out["normalized_path_delay"], 24.0, REL, 0.0);
        assert_close(out["path_delay"], 192e-12, REL, 0.0);
    }
    let mut inputs = EFFORT;
    inputs[5].1 = 0.0;
    let out = calculate("logic.three_stage_logical_effort", &inputs).unwrap();
    assert_close(out["normalized_path_delay"], 18.0, REL, 0.0);
    assert_close(out["path_delay"], 144e-12, REL, 0.0);
    assert_close(out["optimal_stage_effort"], 6.0, REL, 0.0);
    assert_close(out["second_input_capacitance"], 3e-15, REL, 0.0);
    assert_close(out["third_input_capacitance"], 12e-15, REL, 0.0);
}

#[test]
fn logical_effort_inverters_equal_loads_and_downsizing() {
    for (load, effort, c2, c3, delay) in [(64e-15, 4.0, 4e-15, 16e-15, 15.0),
        (1e-15, 1.0, 1e-15, 1e-15, 6.0), (0.125e-15, 0.5, 0.5e-15, 0.25e-15, 4.5)] {
        let mut inputs = EFFORT;
        inputs[1].1 = load;
        for index in [2, 3, 4] { inputs[index].1 = 1.0; }
        inputs[5].1 = 3.0; inputs[6].1 = 1e-11;
        let out = calculate("logic.three_stage_logical_effort", &inputs).unwrap();
        for (key, value) in [("optimal_stage_effort", effort), ("second_input_capacitance", c2),
            ("third_input_capacitance", c3), ("normalized_path_delay", delay), ("path_delay", delay * 1e-11)] {
            assert_close(out[key], value, REL, 0.0);
        }
    }
}

#[test]
fn logical_effort_rejects_invalid_domains_and_recovers() {
    for (index, bad) in [(0, 0.0), (0, -1.0), (1, 0.0), (1, -1.0),
        (2, 0.999), (3, 0.999), (4, 0.999), (5, -1.0), (6, 0.0), (6, -1.0)] {
        let mut inputs = EFFORT;
        inputs[index].1 = bad;
        assert!(calculate("logic.three_stage_logical_effort", &inputs).is_err());
        logical_effort_complete_unequal_gate_solution();
    }
}

#[test]
fn fifo_complete_independent_envelope_geometry() {
    let out = calculate("logic.fifo_envelope_depth", &FIFO).unwrap();
    for (key, value) in [("backlog_bound", 8.875), ("usable_depth", 9.0),
        ("storage_bits", 108.0), ("delay_bound", 1.5)] {
        assert_close(out[key], value, REL, 0.0);
    }
    // Vertical deviation peaks at T; horizontal deviation starts with burst/R.
    for time in [0.0_f64, 0.3125, 0.625, 1.0, 2.0] {
        let arrival = 7.0 + 3.0 * time;
        let service = 8.0 * (time - 0.625).max(0.0);
        assert!(arrival - service <= out["backlog_bound"]);
        assert!(0.625 + arrival / 8.0 - time <= out["delay_bound"]);
    }
    assert!(out["usable_depth"] >= out["backlog_bound"]);
    assert!(out["usable_depth"] - 1.0 < out["backlog_bound"]);
}

#[test]
fn fifo_equal_rate_burst_only_and_minimum_word_phase() {
    for (rate, latency, bound, depth, bits, delay) in [
        (8.0, 0.625, 12.0, 12.0, 144.0, 1.5),
        (0.0, 0.625, 7.0, 7.0, 84.0, 1.5),
        (3.0, 0.125, 7.375, 8.0, 96.0, 1.0),
    ] {
        let mut inputs = FIFO;
        inputs[1].1 = rate; inputs[3].1 = latency;
        let out = calculate("logic.fifo_envelope_depth", &inputs).unwrap();
        for (key, value) in [("backlog_bound", bound), ("usable_depth", depth),
            ("storage_bits", bits), ("delay_bound", delay)] {
            assert_close(out[key], value, REL, 0.0);
        }
    }
}

#[test]
fn fifo_integer_ceiling_neighbors() {
    let latency: f64 = 0.5;
    // Two ULPs below/above make the change survive addition to the burst.
    for (time, depth) in [(f64::from_bits(latency.to_bits() - 2), 2.0),
        (latency, 2.0), (f64::from_bits(latency.to_bits() + 2), 3.0)] {
        let inputs = [("burst_words", 1.0), ("arrival_rate", 2.0), ("service_rate", 4.0),
            ("service_latency", time), ("word_bits", 8.0)];
        let out = calculate("logic.fifo_envelope_depth", &inputs).unwrap();
        assert_eq!(out["usable_depth"], depth);
        assert_eq!(out["storage_bits"], 8.0 * depth);
    }
}

#[test]
fn fifo_positive_sub_ulp_arrivals_still_require_one_more_word() {
    // b is integral, so ceil(b+x)=b+ceil(x). Rounding b+x before ceil loses
    // a positive x below half an ULP, incorrectly understating usable capacity.
    let inputs = [("burst_words", 64.0), ("arrival_rate", 1e-9), ("service_rate", 1e6),
        ("service_latency", 1e-6), ("word_bits", 8.0)];
    let out = calculate("logic.fifo_envelope_depth", &inputs).unwrap();
    assert_close(out["backlog_bound"], 64.0, REL, 0.0);
    assert_eq!(out["usable_depth"], 65.0);
    assert_eq!(out["storage_bits"], 520.0);
    assert_close(out["delay_bound"], 65e-6, REL, 0.0);
}

#[test]
fn fifo_rejects_unstable_rate_short_phase_and_bad_integers_with_recovery() {
    for (index, bad) in [(0, 0.0), (0, -1.0), (0, 1.5), (1, -1.0),
        (1, 8.125), (2, 0.0), (2, -1.0), (3, 0.0), (3, -1.0),
        (3, 0.124), (4, 0.0), (4, -1.0), (4, 8.5)] {
        let mut inputs = FIFO;
        inputs[index].1 = bad;
        assert!(calculate("logic.fifo_envelope_depth", &inputs).is_err());
        fifo_complete_independent_envelope_geometry();
    }
}

#[test]
fn every_logic_input_rejects_nonfinite_missing_and_extra_then_recovers() {
    for (id, valid, recovery) in [
        ("logic.guaranteed_noise_margins", NOISE.as_slice(), noise_complete_independent_voltage_budget as fn()),
        ("logic.dc_current_fanout", FAN.as_slice(), fanout_complete_independent_current_conservation as fn()),
        ("logic.cmos_package_power", POWER.as_slice(), power_complete_independent_full_cycle_energy as fn()),
        ("logic.synchronizer_metastability", META.as_slice(), metastability_complete_independent_failure_probability as fn()),
        ("logic.three_stage_logical_effort", EFFORT.as_slice(), logical_effort_complete_unequal_gate_solution as fn()),
        ("logic.fifo_envelope_depth", FIFO.as_slice(), fifo_complete_independent_envelope_geometry as fn()),
    ] {
        for index in 0..valid.len() {
            for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                let mut inputs = valid.to_vec();
                inputs[index].1 = bad;
                assert!(calculate(id, &inputs).is_err(), "{id} accepted nonfinite {}", valid[index].0);
                recovery();
            }
            let mut inputs = valid.to_vec();
            inputs.remove(index);
            assert!(calculate(id, &inputs).is_err(), "{id} accepted missing {}", valid[index].0);
            recovery();
        }
        let mut inputs = valid.to_vec();
        inputs.push(("undeclared_input", 1.0));
        assert!(calculate(id, &inputs).is_err(), "{id} accepted undeclared input");
        recovery();
    }
}

#[test]
fn metastability_subnormal_transition_rate_keeps_finite_logarithms() {
    let mut inputs = META;
    inputs[1].1 = 1e-310;
    let out = calculate("logic.synchronizer_metastability", &inputs).unwrap();
    // Encounter rate = 2^-10 * 1e-310; do not form its reciprocal.
    let chain: f64 = 8.0 / std::f64::consts::LN_10 + 10.0 * 2.0_f64.log10()
        - 1e-310_f64.log10();
    assert_close(out["log10_chain_mtbf"], chain, REL, 0.0);
    assert_close(out["log10_system_mtbf"], chain - 16.0_f64.log10(), REL, 0.0);
    assert_eq!(out["required_resolution_time"], 0.0);
    assert_eq!(out["resolution_margin"], 8e-10);
}

#[test]
fn metastability_representable_log_mtbf_survives_raw_exponent_overflow() {
    let mut inputs = META;
    inputs[3].1 = 5e-11;
    inputs[4].1 = 1e298;
    let out = calculate("logic.synchronizer_metastability", &inputs).unwrap();
    // 2e308 / ln(10) is representable although the raw exponent is not.
    let chain: f64 = 1e308 * (2.0 / std::f64::consts::LN_10);
    let required: f64 = 5e-11 * 1024.0_f64.ln();
    assert_close(out["log10_chain_mtbf"], chain, REL, 0.0);
    assert_close(out["log10_system_mtbf"], chain, REL, 0.0);
    assert_close(out["required_resolution_time"], required, REL, 0.0);
    assert_close(out["resolution_margin"], 1e298, REL, 0.0);
}

#[test]
fn metastability_equal_tiny_resolution_and_tau_preserve_unit_exponent() {
    let mut inputs = META;
    inputs[3].1 = 1e-310;
    inputs[4].1 = 1e-310;
    inputs[6].1 = 0.001;
    let out = calculate("logic.synchronizer_metastability", &inputs).unwrap();
    assert_close(out["log10_chain_mtbf"], std::f64::consts::LOG10_E, REL, 0.0);
    assert_close(out["log10_system_mtbf"], std::f64::consts::LOG10_E - 16.0_f64.log10(), REL, 0.0);
    assert_eq!(out["required_resolution_time"], 0.0);
    assert_eq!(out["resolution_margin"], 1e-310);
}
