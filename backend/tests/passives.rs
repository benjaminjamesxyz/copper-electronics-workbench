mod common;

use common::{assert_close, calculate};
use std::f64::consts::{LN_10, LN_2};

// Oracles use charge conservation, exact polynomial quadrature, circuit KCL,
// and rational nodal elimination. No catalog examples are loaded as expectations.
// Relative-only tolerances preserve tiny nonzero quantities; exact zeros have no floor.
const AGING: &[(&str, f64)] = &[
    ("capacitance_one_hour", 22e-6), ("aging_percent_decade", 3.0), ("age", 3_600_000.0),
];
const ABSORPTION: &[(&str, f64)] = &[
    ("main_capacitance", 3e-6), ("absorption_capacitance", 1e-6),
    ("absorption_resistance", 2e6), ("charge_voltage", -12.0),
    ("charge_time", 2.0 * LN_2), ("short_time", 2.0 * LN_2),
    ("recovery_time", 1.5 * LN_2),
];
const BIAS: &[(&str, f64)] = &[
    ("zero_bias_capacitance", 12e-6), ("fit_capacitance", 3e-6),
    ("fit_voltage", 6.0), ("lower_voltage", 1.0), ("upper_voltage", 5.0),
];
const NOISE: &[(&str, f64)] = &[
    ("noise_index_db", 0.0), ("dc_voltage", -3.0), ("lower_frequency", 2.0),
    ("upper_frequency", 200.0), ("probe_frequency", 8.0),
];
const VCR: &[(&str, f64)] = &[
    ("supply_voltage", 600.0), ("high_resistance_zero", 1e6),
    ("bottom_resistance", 1e5), ("vcr_ppm_volt", -1000.0),
];
const LADDER: &[(&str, f64)] = &[
    ("ladder_resistance", 1000.0), ("lsb_switch_resistance", 2000.0),
    ("middle_switch_resistance", 0.0), ("msb_switch_resistance", 1000.0),
    ("reference_voltage", 8.0), ("code", 3.0), ("load_conductance", 0.0005),
];

#[test]
fn aging_complete_noncompounded_decade_oracles() {
    for (capacitance, rate, decades, retained) in [
        (22e-6, 3.0, 3, 20.02e-6),
        (10e-6, 2.0, 4, 9.2e-6),
        (10e-6, 2.0, 1, 9.8e-6),
    ] {
        let r = calculate("passives.mlcc_logarithmic_aging", &[
            ("capacitance_one_hour", capacitance), ("aging_percent_decade", rate),
            ("age", 3600.0 * 10.0_f64.powi(decades)),
        ]).unwrap();
        assert_eq!(r.len(), 3);
        assert_close(r["elapsed_decades"], decades as f64, 1e-12, 0.0);
        assert_close(r["fractional_loss"], rate * decades as f64 / 100.0, 1e-12, 0.0);
        assert_close(r["aged_capacitance"], retained, 1e-12, 0.0);
    }
}

#[test]
fn aging_reference_zero_rate_and_adjacent_age() {
    for (age, rate, decades) in [(3600.0, 3.0, 0.0), (3_600_000.0, 0.0, 3.0)] {
        let r = calculate("passives.mlcc_logarithmic_aging", &[
            ("capacitance_one_hour", 22e-6), ("aging_percent_decade", rate), ("age", age),
        ]).unwrap();
        assert_close(r["elapsed_decades"], decades, 1e-12, 0.0);
        assert_close(r["fractional_loss"], 0.0, 0.0, 0.0);
        assert_close(r["aged_capacitance"], 22e-6, 1e-12, 0.0);
    }
    let age = f64::from_bits(3600.0_f64.to_bits() + 1);
    let x: f64 = (age - 3600.0) / 3600.0;
    let decades = (x - x * x / 2.0) / LN_10;
    let r = calculate("passives.mlcc_logarithmic_aging", &[
        ("capacitance_one_hour", 22e-6), ("aging_percent_decade", 3.0), ("age", age),
    ]).unwrap();
    assert_close(r["elapsed_decades"], decades, 1e-12, 0.0);
    assert_close(r["fractional_loss"], 0.03 * decades, 1e-12, 0.0);
    assert_close(r["aged_capacitance"], 22e-6, 1e-15, 0.0);
}

#[test]
fn aging_fit_exhaustion_rejects_then_recovers() {
    let mut inputs = AGING.to_vec();
    inputs[1].1 = 100.0;
    inputs[2].1 = 36_000.0;
    assert!(calculate("passives.mlcc_logarithmic_aging", &inputs).is_err());
    inputs[1].1 = 100.001;
    assert!(calculate("passives.mlcc_logarithmic_aging", &inputs).is_err());
    inputs[1].1 = 99.0;
    let r = calculate("passives.mlcc_logarithmic_aging", &inputs).unwrap();
    assert_close(r["elapsed_decades"], 1.0, 1e-12, 0.0);
    assert_close(r["fractional_loss"], 0.99, 1e-12, 0.0);
    assert_close(r["aged_capacitance"], 2.2e-7, 1e-12, 0.0);
}

#[test]
fn absorption_complete_half_charge_half_short_half_recovery() {
    // Each preparation phase halves a state. Cm=3Ca then shares charge 1:4.
    let r = calculate("passives.dielectric_absorption_recovery", ABSORPTION).unwrap();
    assert_eq!(r.len(), 4);
    for (key, expected) in [("memory_voltage", -3.0), ("recovery_time_constant", 1.5),
        ("final_recovery_voltage", -0.75), ("recovered_voltage", -0.375)] {
        assert_close(r[key], expected, 1e-12, 0.0);
    }
    assert_close(r["final_recovery_voltage"] * 4e-6, r["memory_voltage"] * 1e-6, 1e-12, 0.0);
    let mut positive = ABSORPTION.to_vec();
    positive[3].1 = 12.0;
    let p = calculate("passives.dielectric_absorption_recovery", &positive).unwrap();
    for key in ["memory_voltage", "final_recovery_voltage", "recovered_voltage"] {
        assert_close(p[key], -r[key], 1e-12, 0.0);
    }
    assert_close(p["recovery_time_constant"], 1.5, 1e-12, 0.0);
}

#[test]
fn absorption_researched_timing_from_state_evolution() {
    let r = calculate("passives.dielectric_absorption_recovery", &[
        ("main_capacitance", 1e-6), ("absorption_capacitance", 1e-8),
        ("absorption_resistance", 1e9), ("charge_voltage", 10.0),
        ("charge_time", 20.0), ("short_time", 1.0), ("recovery_time", 10.0),
    ]).unwrap();
    let memory = 10.0 * ((-0.1_f64).exp() - (-2.1_f64).exp());
    let tau = 1000.0 / 101.0;
    let final_voltage = memory / 101.0;
    for (key, expected) in [("memory_voltage", memory), ("recovery_time_constant", tau),
        ("final_recovery_voltage", final_voltage),
        ("recovered_voltage", final_voltage * (1.0 - (-1.01_f64).exp()))] {
        assert_close(r[key], expected, 1e-12, 0.0);
    }
}

#[test]
fn absorption_zero_history_long_limits_and_tiny_time() {
    for index in [3, 4] {
        let mut inputs = ABSORPTION.to_vec();
        inputs[index].1 = 0.0;
        let r = calculate("passives.dielectric_absorption_recovery", &inputs).unwrap();
        for key in ["memory_voltage", "final_recovery_voltage", "recovered_voltage"] {
            assert_close(r[key], 0.0, 0.0, 0.0);
        }
        assert_close(r["recovery_time_constant"], 1.5, 1e-12, 0.0);
    }
    for (time, recovered) in [(0.0, 0.0), (1e-20, -5e-21), (1500.0, -0.75)] {
        let mut inputs = ABSORPTION.to_vec();
        inputs[6].1 = time;
        let r = calculate("passives.dielectric_absorption_recovery", &inputs).unwrap();
        // Initial current is Va0/Ra, so dVterminal/dt=Va0/(Ra Cm)=-0.5 V/s.
        assert_close(r["recovered_voltage"], recovered, 1e-12, 0.0);
        assert_close(r["memory_voltage"], -3.0, 1e-12, 0.0);
        assert_close(r["final_recovery_voltage"], -0.75, 1e-12, 0.0);
        assert_close(r["recovery_time_constant"], 1.5, 1e-12, 0.0);
    }
    let mut inputs = ABSORPTION.to_vec();
    inputs[4].1 = 2000.0;
    inputs[5].1 = 0.0;
    inputs[6].1 = 1500.0;
    let charged = calculate("passives.dielectric_absorption_recovery", &inputs).unwrap();
    assert_close(charged["memory_voltage"], -12.0, 1e-12, 0.0);
    assert_close(charged["final_recovery_voltage"], -3.0, 1e-12, 0.0);
    assert_close(charged["recovered_voltage"], -3.0, 1e-12, 0.0);
    inputs[5].1 = 2000.0;
    let erased = calculate("passives.dielectric_absorption_recovery", &inputs).unwrap();
    for key in ["memory_voltage", "final_recovery_voltage", "recovered_voltage"] {
        assert_close(erased[key], 0.0, 0.0, 0.0);
    }
}

#[test]
fn bias_complete_outputs_from_exact_simpson_integration() {
    for (c0, cf, vf, lo, hi) in [(12e-6, 3e-6, 6.0, 1.0, 5.0),
        (10e-6, 4e-6, 10.0, 2.0, 8.0), (10e-6, 4e-6, 10.0, 0.0, 10.0),
        (12e-6, 12e-6, 6.0, 0.0, 6.0)] {
        let r = calculate("passives.mlcc_bias_charge_energy", &[
            ("zero_bias_capacitance", c0), ("fit_capacitance", cf), ("fit_voltage", vf),
            ("lower_voltage", lo), ("upper_voltage", hi),
        ]).unwrap();
        let mid = (hi + lo) / 2.0;
        let capacitance = |v: f64| c0 * (1.0 - v / vf) + cf * (v / vf);
        // Trapezoid exactly integrates linear C; Simpson exactly integrates quadratic V*C.
        let charge = (hi - lo) * (capacitance(lo) + capacitance(hi)) / 2.0;
        let energy = (hi - lo) * (lo * capacitance(lo) + 4.0 * mid * capacitance(mid)
            + hi * capacitance(hi)) / 6.0;
        assert_eq!(r.len(), 5);
        for (key, expected) in [("upper_capacitance", capacitance(hi)), ("usable_charge", charge),
            ("usable_energy", energy), ("charge_equivalent_capacitance", charge / (hi - lo)),
            ("energy_equivalent_capacitance", 2.0 * energy / (hi * hi - lo * lo))] {
            assert_close(r[key], expected, 1e-12, 0.0);
        }
        assert!(r["energy_equivalent_capacitance"] <= r["charge_equivalent_capacitance"] * (1.0 + 1e-14));
        assert!(r["usable_energy"] >= lo * r["usable_charge"]);
        assert!(r["usable_energy"] <= hi * r["usable_charge"]);
    }
}

#[test]
fn bias_narrow_representable_window_retains_charge_and_energy() {
    let lo: f64 = 3.0;
    let hi = f64::from_bits(lo.to_bits() + 1);
    let width = hi - lo;
    let mut inputs = BIAS.to_vec();
    inputs[3].1 = lo;
    inputs[4].1 = hi;
    let r = calculate("passives.mlcc_bias_charge_energy", &inputs).unwrap();
    for key in ["upper_capacitance", "charge_equivalent_capacitance", "energy_equivalent_capacitance"] {
        assert_close(r[key], 7.5e-6, 1e-14, 0.0);
    }
    assert_close(r["usable_charge"], width * 7.5e-6, 1e-14, 0.0);
    assert_close(r["usable_energy"], width * 22.5e-6, 1e-14, 0.0);
}

#[test]
fn bias_fit_endpoint_preserves_small_positive_capacitance() {
    // At the measured fit endpoint, interpolation must return that endpoint exactly.
    // Regression: subtracting nearly equal capacitances can erase this admitted positive value.
    let r = calculate("passives.mlcc_bias_charge_energy", &[
        ("zero_bias_capacitance", 1e-6), ("fit_capacitance", 1e-24),
        ("fit_voltage", 1.0), ("lower_voltage", 0.0), ("upper_voltage", 1.0),
    ]).unwrap();
    for (key, expected) in [("upper_capacitance", 1e-24), ("usable_charge", 5e-7),
        ("usable_energy", 1e-6 / 6.0), ("charge_equivalent_capacitance", 5e-7),
        ("energy_equivalent_capacitance", 1e-6 / 3.0)] {
        assert_close(r[key], expected, 1e-12, 0.0);
    }
}

#[test]
fn bias_window_and_fit_rejections_recover() {
    for (index, bad) in [(1, 13e-6), (3, 5.0), (3, 5.1), (4, 6.1)] {
        let mut inputs = BIAS.to_vec();
        inputs[index].1 = bad;
        assert!(calculate("passives.mlcc_bias_charge_energy", &inputs).is_err());
        let r = calculate("passives.mlcc_bias_charge_energy", BIAS).unwrap();
        for (key, expected) in [("upper_capacitance", 4.5e-6), ("usable_charge", 30e-6),
            ("usable_energy", 82e-6), ("charge_equivalent_capacitance", 7.5e-6),
            ("energy_equivalent_capacitance", 41e-6 / 6.0)] {
            assert_close(r[key], expected, 1e-12, 0.0);
        }
    }
}

#[test]
fn noise_complete_decade_normalization_and_spectral_integral() {
    for (index_db, dc, lo, hi, probe, ratio, decades) in [
        (0.0, -3.0, 2.0, 200.0, 8.0, 1e-6, 2.0_f64),
        (-20.0, 10.0, 1.0, 1000.0, 10.0, 1e-7, 3.0_f64),
        (20.0, 2.0, 5.0, 50.0, 5.0, 1e-5, 1.0_f64),
    ] {
        let r = calculate("passives.resistor_excess_noise", &[
            ("noise_index_db", index_db), ("dc_voltage", dc), ("lower_frequency", lo),
            ("upper_frequency", hi), ("probe_frequency", probe),
        ]).unwrap();
        let density: f64 = (dc * ratio) * (dc * ratio) / (probe * LN_10);
        assert_eq!(r.len(), 4);
        assert_close(r["decade_voltage_ratio"], ratio, 1e-12, 0.0);
        assert_close(r["band_decades"], decades, 1e-12, 0.0);
        assert_close(r["excess_noise_rms"], (dc * ratio).abs() * decades.sqrt(), 1e-12, 0.0);
        assert_close(r["excess_noise_density"], density.sqrt(), 1e-12, 0.0);
        assert_close(r["excess_noise_rms"].powi(2), density * probe * decades * LN_10, 1e-12, 0.0);
    }
}

#[test]
fn noise_zero_bias_empty_band_polarity_and_bandwidth_scaling() {
    for dc in [-3.0_f64, 0.0, 3.0] {
        for (hi, decades) in [(2.0, 0.0_f64), (20.0, 1.0), (200.0, 2.0)] {
            let mut inputs = NOISE.to_vec();
            inputs[1].1 = dc;
            inputs[3].1 = hi;
            inputs[4].1 = 2.0;
            let r = calculate("passives.resistor_excess_noise", &inputs).unwrap();
            assert_close(r["decade_voltage_ratio"], 1e-6, 1e-12, 0.0);
            assert_close(r["band_decades"], decades, 1e-12, 0.0);
            assert_close(r["excess_noise_rms"], dc.abs() * 1e-6 * decades.sqrt(), 1e-12, 0.0);
            assert_close(r["excess_noise_density"], dc.abs() * 1e-6 / (2.0 * LN_10).sqrt(), 1e-12, 0.0);
        }
    }
}

#[test]
fn noise_adjacent_frequency_band_retains_nonzero_variance() {
    let hi = f64::from_bits(2.0_f64.to_bits() + 1);
    let x = (hi - 2.0) / 2.0;
    let decades = (x - x * x / 2.0) / LN_10;
    let mut inputs = NOISE.to_vec();
    inputs[3].1 = hi;
    inputs[4].1 = 2.0;
    let r = calculate("passives.resistor_excess_noise", &inputs).unwrap();
    assert_close(r["band_decades"], decades, 1e-12, 0.0);
    assert_close(r["excess_noise_rms"], 3e-6 * decades.sqrt(), 1e-12, 0.0);
    assert_close(r["excess_noise_density"], 3e-6 / (2.0 * LN_10).sqrt(), 1e-12, 0.0);
}

#[test]
fn noise_coupled_band_and_probe_rejections_recover() {
    for (index, bad) in [(3, 1.0), (4, 1.99), (4, 200.01)] {
        let mut inputs = NOISE.to_vec();
        inputs[index].1 = bad;
        assert!(calculate("passives.resistor_excess_noise", &inputs).is_err());
        let r = calculate("passives.resistor_excess_noise", NOISE).unwrap();
        assert_close(r["decade_voltage_ratio"], 1e-6, 1e-12, 0.0);
        assert_close(r["band_decades"], 2.0, 1e-12, 0.0);
        assert_close(r["excess_noise_rms"], 3e-6 * 2.0_f64.sqrt(), 1e-12, 0.0);
        assert_close(r["excess_noise_density"], 3e-6 / (8.0 * LN_10).sqrt(), 1e-12, 0.0);
    }
}

#[test]
fn vcr_complete_outputs_from_independent_kcl_root() {
    for (supply, r0, bottom, ppm) in [(600.0, 1e6, 1e5, -1000.0),
        (1000.0, 9.9e6, 1e5, -10.0)] {
        // Monotone current residual brackets Vh, independent of the catalog quadratic root.
        let (mut low, mut high) = (0.0, supply);
        for _ in 0..100 {
            let v = (low + high) / 2.0;
            if v / (r0 * (1.0 + ppm * 1e-6 * v)) > (supply - v) / bottom {
                high = v;
            } else {
                low = v;
            }
        }
        let vh = (low + high) / 2.0;
        let vo = supply - vh;
        let ideal = supply * bottom / (r0 + bottom);
        let r = calculate("passives.negative_vcr_divider", &[
            ("supply_voltage", supply), ("high_resistance_zero", r0),
            ("bottom_resistance", bottom), ("vcr_ppm_volt", ppm),
        ]).unwrap();
        assert_eq!(r.len(), 5);
        for (key, expected) in [("high_voltage", vh), ("high_resistance", r0 * (1.0 + ppm * 1e-6 * vh)),
            ("output_voltage", vo), ("ideal_output_voltage", ideal), ("fractional_ratio_error", vo / ideal - 1.0)] {
            assert_close(r[key], expected, 1e-10, 0.0);
        }
        assert_close(r["high_voltage"] + r["output_voltage"], supply, 1e-12, 0.0);
        assert_close(r["high_voltage"] / r["high_resistance"], r["output_voltage"] / bottom, 1e-12, 0.0);
        assert!(r["output_voltage"] > r["ideal_output_voltage"]);
    }
}

#[test]
fn vcr_zero_supply_zero_coefficient_and_tiny_ratio_error() {
    for (supply, ppm, vh, vo, rh, ideal, error) in [
        (0.0, -1000.0, 0.0, 0.0, 1e6, 0.0, 0.0),
        (600.0, 0.0, 6000.0 / 11.0, 600.0 / 11.0, 1e6, 600.0 / 11.0, 0.0),
        (600.0, -1e-18, 6000.0 / 11.0, 600.0 / 11.0, 1e6, 600.0 / 11.0, 600e-24 / 1.21),
    ] {
        let mut inputs = VCR.to_vec();
        inputs[0].1 = supply;
        inputs[3].1 = ppm;
        let r = calculate("passives.negative_vcr_divider", &inputs).unwrap();
        for (key, expected) in [("high_voltage", vh), ("output_voltage", vo), ("high_resistance", rh),
            ("ideal_output_voltage", ideal), ("fractional_ratio_error", error)] {
            assert_close(r[key], expected, 1e-12, 0.0);
        }
    }
    let mut equal = VCR.to_vec();
    equal[0].1 = 100.0;
    equal[2].1 = 1e6;
    equal[3].1 = 0.0;
    let r = calculate("passives.negative_vcr_divider", &equal).unwrap();
    assert_close(r["high_voltage"], 50.0, 1e-12, 0.0);
    assert_close(r["output_voltage"], 50.0, 1e-12, 0.0);
    equal[3].1 = -1000.0;
    assert!(calculate("passives.negative_vcr_divider", &equal).unwrap()["output_voltage"] > 50.0);
}

#[test]
fn vcr_whole_interval_positivity_guard_and_recovery() {
    for supply in [1000.0, 1001.0] {
        let mut inputs = VCR.to_vec();
        inputs[0].1 = supply;
        assert!(calculate("passives.negative_vcr_divider", &inputs).is_err());
        let r = calculate("passives.negative_vcr_divider", VCR).unwrap();
        for (key, expected) in [("high_voltage", 500.0), ("high_resistance", 500000.0),
            ("output_voltage", 100.0), ("ideal_output_voltage", 600.0 / 11.0),
            ("fractional_ratio_error", 5.0 / 6.0)] {
            assert_close(r[key], expected, 1e-12, 0.0);
        }
    }
}

#[test]
fn ladder_complete_rational_nodal_oracle_and_kcl() {
    let r = calculate("passives.nonideal_r2r_ladder", LADDER).unwrap();
    assert_eq!(r.len(), 9);
    // Units: multiply KCL by 1 kohm. Matrix rows are
    // [7/4,-1,0], [-1,5/2,-1], [0,-1,11/6]; source vector [2,4,0].
    // Exact elimination gives N0=232/71, N1=264/71, N2=144/71 volts.
    for (key, expected) in [("lsb_thevenin_resistance", 4000.0 / 3.0),
        ("lsb_thevenin_voltage", 8.0 / 3.0), ("middle_thevenin_resistance", 14000.0 / 13.0),
        ("middle_thevenin_voltage", 72.0 / 13.0), ("output_resistance", 13500.0 / 11.0),
        ("open_voltage", 36.0 / 11.0), ("loaded_voltage", 144.0 / 71.0),
        ("ideal_voltage", 3.0), ("code_error_lsb", -69.0 / 71.0)] {
        assert_close(r[key], expected, 1e-12, 0.0);
    }
    let n0 = 232.0 / 71.0;
    let n1 = 264.0 / 71.0;
    let n2 = r["loaded_voltage"];
    assert_close((8.0 - n0) / 4000.0 + (n1 - n0) / 1000.0, n0 / 2000.0, 1e-12, 0.0);
    assert_close((8.0 - n1) / 2000.0 + (n0 - n1) / 1000.0, (n1 - n2) / 1000.0, 1e-12, 0.0);
    assert_close((n1 - n2) / 1000.0, n2 / 3000.0 + 0.0005 * n2, 1e-12, 0.0);
    assert!(r["lsb_thevenin_voltage"] != n0);
    assert!(r["middle_thevenin_voltage"] != n1);
}

#[test]
fn ladder_ideal_all_codes_open_and_loaded() {
    for code in 0..=7 {
        for load in [0.0, 0.001] {
            let r = calculate("passives.nonideal_r2r_ladder", &[
                ("ladder_resistance", 1000.0), ("lsb_switch_resistance", 0.0),
                ("middle_switch_resistance", 0.0), ("msb_switch_resistance", 0.0),
                ("reference_voltage", 8.0), ("code", code as f64), ("load_conductance", load),
            ]).unwrap();
            let ideal = code as f64;
            let loaded = if load == 0.0 { ideal } else { ideal / 2.0 };
            for key in ["lsb_thevenin_resistance", "middle_thevenin_resistance", "output_resistance"] {
                assert_close(r[key], 1000.0, 1e-12, 0.0);
            }
            for (key, expected) in [("lsb_thevenin_voltage", (code & 1) as f64 * 4.0),
                ("middle_thevenin_voltage", (code & 1) as f64 * 2.0 + ((code >> 1) & 1) as f64 * 4.0),
                ("open_voltage", ideal), ("loaded_voltage", loaded), ("ideal_voltage", ideal),
                ("code_error_lsb", loaded - ideal)] {
                assert_close(r[key], expected, 1e-12, 0.0);
            }
        }
    }
}

#[test]
fn ladder_unequal_bit_weights_signed_error_superposition_and_scaling() {
    let mut sum = 0.0;
    for (code, open_voltage) in [(0.0, 0.0), (1.0, 8.0 / 11.0), (2.0, 28.0 / 11.0),
        (4.0, 36.0 / 11.0), (7.0, 72.0 / 11.0)] {
        for scale in [1.0, 2.0] {
            let mut inputs = LADDER.to_vec();
            for input in &mut inputs[..4] { input.1 *= scale; }
            inputs[5].1 = code;
            inputs[6].1 = 0.0;
            let r = calculate("passives.nonideal_r2r_ladder", &inputs).unwrap();
            assert_close(r["open_voltage"], open_voltage, 1e-12, 0.0);
            assert_close(r["loaded_voltage"], open_voltage, 1e-12, 0.0);
            assert_close(r["code_error_lsb"], open_voltage - code, 1e-12, 0.0);
            assert_close(r["output_resistance"], scale * 13500.0 / 11.0, 1e-12, 0.0);
            inputs[6].1 = 0.0005 / scale;
            let loaded = calculate("passives.nonideal_r2r_ladder", &inputs).unwrap();
            assert_close(loaded["loaded_voltage"], open_voltage * 44.0 / 71.0, 1e-12, 0.0);
        }
        if code == 1.0 || code == 2.0 || code == 4.0 { sum += open_voltage; }
        if code == 7.0 { assert_close(sum, open_voltage, 1e-12, 0.0); }
    }
}

#[test]
fn ladder_fractional_out_of_range_codes_reject_and_recover() {
    for code in [-1.0, 8.0, 2.5, f64::from_bits(3.0_f64.to_bits() + 1)] {
        let mut inputs = LADDER.to_vec();
        inputs[5].1 = code;
        assert!(calculate("passives.nonideal_r2r_ladder", &inputs).is_err());
        let r = calculate("passives.nonideal_r2r_ladder", LADDER).unwrap();
        assert_close(r["loaded_voltage"], 144.0 / 71.0, 1e-12, 0.0);
        assert_close(r["code_error_lsb"], -69.0 / 71.0, 1e-12, 0.0);
    }
}

#[test]
fn all_passives_scalar_domain_errors_and_valid_recovery() {
    let cases: &[(&str, &[(&str, f64)], &[(usize, f64)], &str, f64)] = &[
        ("passives.mlcc_logarithmic_aging", AGING, &[(0, 0.0), (0, -1e-6), (1, -0.01), (2, 3599.0)], "aged_capacitance", 20.02e-6),
        ("passives.dielectric_absorption_recovery", ABSORPTION, &[(0, 0.0), (0, -1e-6), (1, 0.0), (1, -1e-6), (2, 0.0), (2, -1.0), (4, -1.0), (5, -1.0), (6, -1.0)], "recovered_voltage", -0.375),
        ("passives.mlcc_bias_charge_energy", BIAS, &[(0, 0.0), (0, -1e-6), (1, 0.0), (1, -1e-6), (2, 0.0), (2, -1.0), (3, -1.0), (4, 0.0), (4, -1.0)], "usable_energy", 82e-6),
        ("passives.resistor_excess_noise", NOISE, &[(2, 0.0), (2, -1.0), (3, 0.0), (3, -1.0), (4, 0.0), (4, -1.0)], "excess_noise_rms", 4.242640687119286e-6),
        ("passives.negative_vcr_divider", VCR, &[(0, -1.0), (1, 0.0), (1, -1.0), (2, 0.0), (2, -1.0), (3, 0.01)], "output_voltage", 100.0),
        ("passives.nonideal_r2r_ladder", LADDER, &[(0, 0.0), (0, -1.0), (1, -1.0), (2, -1.0), (3, -1.0), (4, 0.0), (4, -1.0), (6, -0.001)], "loaded_voltage", 144.0 / 71.0),
    ];
    for &(id, valid, invalid, key, expected) in cases {
        for &(index, bad) in invalid {
            let mut inputs = valid.to_vec();
            inputs[index].1 = bad;
            assert!(calculate(id, &inputs).is_err(), "{id} accepted {}={bad}", inputs[index].0);
            assert_close(calculate(id, valid).unwrap()[key], expected, 1e-12, 0.0);
        }
    }
}

#[test]
fn all_passives_nonfinite_missing_extra_inputs_reject_without_poisoning_engine() {
    for (id, valid, key, expected) in [
        ("passives.mlcc_logarithmic_aging", AGING, "aged_capacitance", 20.02e-6),
        ("passives.dielectric_absorption_recovery", ABSORPTION, "recovered_voltage", -0.375),
        ("passives.mlcc_bias_charge_energy", BIAS, "usable_energy", 82e-6),
        ("passives.resistor_excess_noise", NOISE, "excess_noise_rms", 4.242640687119286e-6),
        ("passives.negative_vcr_divider", VCR, "output_voltage", 100.0),
        ("passives.nonideal_r2r_ladder", LADDER, "loaded_voltage", 144.0 / 71.0),
    ] {
        for index in 0..valid.len() {
            for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                let mut inputs = valid.to_vec();
                inputs[index].1 = bad;
                assert!(calculate(id, &inputs).is_err(), "{id} accepted nonfinite {}", inputs[index].0);
                assert_close(calculate(id, valid).unwrap()[key], expected, 1e-12, 0.0);
            }
            let mut missing = valid.to_vec();
            missing.remove(index);
            assert!(calculate(id, &missing).is_err(), "{id} accepted missing {}", valid[index].0);
            assert_close(calculate(id, valid).unwrap()[key], expected, 1e-12, 0.0);
        }
        let mut extra = valid.to_vec();
        extra.push(("undeclared_input", 1.0));
        assert!(calculate(id, &extra).is_err(), "{id} accepted undeclared input");
        assert_close(calculate(id, valid).unwrap()[key], expected, 1e-12, 0.0);
    }
}

#[test]
fn ladder_researched_vector_from_rational_conductance_matrices() {
    let r = calculate("passives.nonideal_r2r_ladder", &[
        ("ladder_resistance", 1000.0), ("lsb_switch_resistance", 100.0),
        ("middle_switch_resistance", 200.0), ("msb_switch_resistance", 300.0),
        ("reference_voltage", 5.0), ("code", 5.0), ("load_conductance", 0.0001),
    ]).unwrap();
    // Exact rational Gaussian elimination of the one-, two-, and three-node
    // conductance matrices. A 1 A output injection with sources grounded gives Rout.
    for (key, expected) in [("lsb_thevenin_resistance", 42000.0 / 41.0),
        ("lsb_thevenin_voltage", 100.0 / 41.0), ("middle_thevenin_resistance", 456500.0 / 433.0),
        ("middle_thevenin_voltage", 550.0 / 433.0), ("output_resistance", 10229250.0 / 9427.0),
        ("open_voltage", 57125.0 / 18854.0), ("loaded_voltage", 1142500.0 / 417997.0),
        ("ideal_voltage", 25.0 / 8.0), ("code_error_lsb", -261985.0 / 417997.0)] {
        assert_close(r[key], expected, 1e-12, 0.0);
    }
}
