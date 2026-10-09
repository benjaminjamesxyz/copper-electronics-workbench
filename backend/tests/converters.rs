mod common;

use common::{assert_close, calculate};
use std::f64::consts::PI;

// Independent charge, volt-second, waveform-integral and impedance oracles.
// Nonzero results use relative tolerance only; algebraic zeros must be exact.
const PUMP: &str = "converters.inverting_charge_pump";
const SEPIC: &str = "converters.sepic_ccm_stress";
const FORWARD: &str = "converters.forward_reset_winding";
const FLYBACK: &str = "converters.flyback_dcm_demagnetization";
const LLC: &str = "converters.llc_half_bridge_fha";
const DAB: &str = "converters.dual_active_bridge_power";
const PUMP_INPUTS: &[(&str, f64)] = &[
    ("vin", 8.0), ("load_current", 0.4), ("switching_frequency", 20000.0),
    ("flying_capacitance", 25e-6), ("switch_resistance", 0.5),
    ("flying_esr", 0.125), ("output_esr", 0.25),
];
const SEPIC_INPUTS: &[(&str, f64)] = &[
    ("vin", 24.0), ("vout", 8.0), ("load_current", 3.0),
    ("switching_frequency", 100000.0), ("input_inductance", 120e-6),
    ("second_inductance", 80e-6),
];
const FORWARD_INPUTS: &[(&str, f64)] = &[
    ("vin", 80.0), ("vout", 12.0), ("primary_secondary_ratio", 2.0),
    ("reset_primary_ratio", 0.5), ("switching_frequency", 80000.0),
];
const FLYBACK_INPUTS: &[(&str, f64)] = &[
    ("vin", 24.0), ("vout", 8.0), ("load_current", 0.5),
    ("magnetizing_inductance", 20e-6), ("switching_frequency", 100000.0),
    ("primary_secondary_ratio", 1.5),
];
const LLC_INPUTS: &[(&str, f64)] = &[
    ("vin", 300.0), ("primary_secondary_ratio", 3.0),
    ("resonant_inductance", 40e-6), ("magnetizing_inductance", 120e-6),
    ("resonant_capacitance", 250e-9), ("switching_frequency", 50000.0),
    ("load_resistance", 6.0),
];
const DAB_INPUTS: &[(&str, f64)] = &[
    ("primary_voltage", 300.0), ("secondary_voltage", 80.0),
    ("primary_secondary_ratio", 2.5), ("switching_frequency", 80000.0),
    ("transfer_inductance", 75e-6), ("phase_shift", PI / 3.0),
];

#[test]
fn charge_pump_complete_nondefault_charge_transport() {
    let r = calculate(PUMP, PUMP_INPUTS).unwrap();
    // Each cycle moves I/fs = 20 uC; 25 uF therefore loses 0.8 V.
    for (key, value) in [("transport_resistance", 2.0), ("output_resistance", 3.25),
        ("voltage_droop", 1.3), ("output_voltage", -6.7)] {
        assert_close(r[key], value, 1e-12, 0.0);
    }
}

#[test]
fn charge_pump_zero_load_frequency_and_esr_floor() {
    let mut inputs = PUMP_INPUTS.to_vec();
    inputs[1].1 = 0.0;
    let r = calculate(PUMP, &inputs).unwrap();
    assert_close(r["output_voltage"], -8.0, 0.0, 0.0);
    assert_close(r["voltage_droop"], 0.0, 0.0, 0.0);
    assert_close(r["output_resistance"], 3.25, 1e-12, 0.0);
    inputs[1].1 = 0.4;
    for (frequency, transport) in [(40000.0, 1.0), (4e12, 1e-8)] {
        inputs[2].1 = frequency;
        let r = calculate(PUMP, &inputs).unwrap();
        assert_close(r["transport_resistance"], transport, 1e-12, 0.0);
        assert_close(r["output_resistance"], 1.25 + transport, 1e-12, 0.0);
        assert_close(r["voltage_droop"], 0.4 * (1.25 + transport), 1e-12, 0.0);
    }
}

#[test]
fn charge_pump_strict_collapse_boundary_and_recovery() {
    let mut inputs = PUMP_INPUTS.to_vec();
    inputs[0].1 = 5.0;
    inputs[2].1 = 10000.0;
    inputs[3].1 = 100e-6;
    for input in &mut inputs[4..] { input.1 = 0.0; }
    for bad in [5.0, 5.001] {
        inputs[1].1 = bad;
        assert!(calculate(PUMP, &inputs).is_err());
        inputs[1].1 = 4.999;
        let r = calculate(PUMP, &inputs).unwrap();
        assert_close(r["output_voltage"], -0.001, 1e-10, 0.0);
        assert_close(r["voltage_droop"], 4.999, 1e-12, 0.0);
    }
    inputs[1].1 = 1.0;
    assert_close(calculate(PUMP, &inputs).unwrap()["output_voltage"], -4.0, 1e-12, 0.0);
}

#[test]
fn sepic_complete_buck_ratio_from_piecewise_current_integrals() {
    let r = calculate(SEPIC, SEPIC_INPUTS).unwrap();
    // Mean square of a linear ramp a..b is (a²+ab+b²)/3.
    let mean_square = |a: f64, b: f64| (a * a + a * b + b * b) / 3.0;
    let switch_rms = (0.25 * mean_square(3.375, 4.625)).sqrt();
    let capacitor_rms = (0.25 * mean_square(2.625, 3.375)
        + 0.75 * mean_square(0.75, 1.25)).sqrt();
    for (key, value) in [("duty", 0.25), ("input_current", 1.0),
        ("input_ripple", 0.5), ("second_ripple", 0.75),
        ("switch_peak_current", 4.625), ("switch_rms_current", switch_rms),
        ("coupling_cap_rms_current", capacitor_rms), ("blocking_voltage", 32.0)] {
        assert_close(r[key], value, 1e-12, 0.0);
    }
    assert_close(24.0 * r["input_current"], 8.0 * 3.0, 1e-12, 0.0);
    assert_close((1.0 - r["duty"]) * r["input_current"], 3.0 * r["duty"], 1e-12, 0.0);
}

#[test]
fn sepic_unity_boost_and_negligible_ripple_limits() {
    for (vin, vout, load, l1, l2, duty, current, ripple1, ripple2) in [
        (12.0, 12.0, 1.0, 100e-6, 100e-6, 0.5, 1.0, 0.3, 0.3),
        (12.0, 18.0, 1.0, 100e-6, 150e-6, 0.6, 1.5, 0.36, 0.24),
    ] {
        let r = calculate(SEPIC, &[("vin", vin), ("vout", vout), ("load_current", load),
            ("switching_frequency", 200000.0), ("input_inductance", l1), ("second_inductance", l2)]).unwrap();
        assert_close(r["duty"], duty, 1e-12, 0.0);
        assert_close(r["input_current"], current, 1e-12, 0.0);
        assert_close(r["input_ripple"], ripple1, 1e-12, 0.0);
        assert_close(r["second_ripple"], ripple2, 1e-12, 0.0);
    }
    let mut inputs = SEPIC_INPUTS.to_vec();
    inputs[4].1 *= 1e8;
    inputs[5].1 *= 1e8;
    let r = calculate(SEPIC, &inputs).unwrap();
    assert_close(r["switch_rms_current"], 2.0, 1e-12, 0.0);
    assert_close(r["coupling_cap_rms_current"], 3.0_f64.sqrt(), 1e-12, 0.0);
    assert_close(r["input_ripple"], 5e-9, 1e-12, 0.0);
    assert_close(r["second_ripple"], 7.5e-9, 1e-12, 0.0);
}

#[test]
fn sepic_each_branch_valley_boundary_and_recovery() {
    // At D=1/4 the applied volt-seconds are 60 uV s. L1crit=30 uH, L2crit=10 uH.
    for (index, boundary) in [(4, 30e-6), (5, 10e-6)] {
        for factor in [1.0, 0.999] {
            let mut inputs = SEPIC_INPUTS.to_vec();
            inputs[index].1 = boundary * factor;
            assert!(calculate(SEPIC, &inputs).is_err(), "accepted branch {index} at {factor}");
            inputs[index].1 = boundary * 1.001;
            let r = calculate(SEPIC, &inputs).unwrap();
            let (key, mean) = if index == 4 { ("input_ripple", 1.0) } else { ("second_ripple", 3.0) };
            assert_close(r[key], 2.0 * mean / 1.001, 1e-12, 0.0);
            assert!(mean - r[key] / 2.0 > 0.0);
        }
    }
}

#[test]
fn forward_complete_nondefault_reset_volt_seconds() {
    let r = calculate(FORWARD, FORWARD_INPUTS).unwrap();
    for (key, value) in [("duty", 0.3), ("reset_fraction", 0.15),
        ("duty_limit", 2.0 / 3.0), ("idle_fraction", 0.55),
        ("reset_time", 1.875e-6), ("switch_blocking_voltage", 240.0)] {
        assert_close(r[key], value, 1e-12, 0.0);
    }
    assert_close(80.0 * r["duty"], 160.0 * r["reset_fraction"], 1e-12, 0.0);
    assert_close(r["duty"] + r["reset_fraction"] + r["idle_fraction"], 1.0, 1e-12, 0.0);
}

#[test]
fn forward_reset_ratio_tradeoff_and_frequency_scaling() {
    let mut inputs = FORWARD_INPUTS.to_vec();
    inputs[3].1 = 1.0;
    let r = calculate(FORWARD, &inputs).unwrap();
    assert_close(r["duty"], 0.3, 1e-12, 0.0);
    assert_close(r["reset_fraction"], 0.3, 1e-12, 0.0);
    assert_close(r["duty_limit"], 0.5, 1e-12, 0.0);
    assert_close(r["idle_fraction"], 0.4, 1e-12, 0.0);
    assert_close(r["switch_blocking_voltage"], 160.0, 1e-12, 0.0);
    inputs[4].1 *= 2.0;
    let faster = calculate(FORWARD, &inputs).unwrap();
    for (key, value) in r {
        assert_close(faster[&key], if key == "reset_time" { value / 2.0 } else { value }, 1e-12, 0.0);
    }
}

#[test]
fn forward_strict_reset_boundary_and_tiny_positive_margin() {
    let mut inputs = FORWARD_INPUTS.to_vec();
    inputs[3].1 = 1.0;
    for bad in [20.0, 20.001] {
        inputs[1].1 = bad;
        assert!(calculate(FORWARD, &inputs).is_err());
        inputs[1].1 = 19.999;
        assert_close(calculate(FORWARD, &inputs).unwrap()["idle_fraction"], 0.00005, 1e-10, 0.0);
    }
    inputs[0].1 = 2.0;
    inputs[2].1 = 1.0;
    inputs[1].1 = 1.0 - 2.0_f64.powi(-40);
    assert_close(calculate(FORWARD, &inputs).unwrap()["idle_fraction"], 2.0_f64.powi(-40), 1e-12, 0.0);
}

#[test]
fn flyback_complete_nondefault_energy_and_triangle_integrals() {
    let r = calculate(FLYBACK, FLYBACK_INPUTS).unwrap();
    // 4 W / 100 kHz = 40 uJ, giving Ipk=2 A; slopes give 1/6 and 1/3 periods.
    for (key, value) in [("primary_peak_current", 2.0), ("duty", 1.0 / 6.0),
        ("demagnetization_fraction", 1.0 / 3.0), ("idle_fraction", 0.5),
        ("primary_rms_current", (2.0_f64 / 9.0).sqrt()), ("secondary_rms_current", 1.0),
        ("switch_blocking_voltage", 36.0), ("diode_reverse_voltage", 24.0)] {
        assert_close(r[key], value, 1e-12, 0.0);
    }
    assert_close(0.5 * 20e-6 * r["primary_peak_current"].powi(2) * 100000.0, 4.0, 1e-12, 0.0);
    assert_close(1.5 * r["primary_peak_current"] * r["demagnetization_fraction"] / 2.0, 0.5, 1e-12, 0.0);
    assert_close(24.0 * r["primary_peak_current"] * r["duty"] / 2.0, 4.0, 1e-12, 0.0);
}

#[test]
fn flyback_strict_dcm_boundary_and_recovery() {
    let mut inputs = FLYBACK_INPUTS.to_vec();
    for bad in [2.0, 2.001] {
        inputs[2].1 = bad;
        assert!(calculate(FLYBACK, &inputs).is_err());
        inputs[2].1 = 1.999;
        let r = calculate(FLYBACK, &inputs).unwrap();
        // Rationalized remainder avoids cancellation in the independent oracle.
        let root = (1.999_f64 / 2.0).sqrt();
        assert_close(r["idle_fraction"], 0.0005 / (1.0 + root), 1e-9, 0.0);
        assert_close(r["primary_peak_current"], (8.0_f64 * 1.999).sqrt(), 1e-12, 0.0);
    }
}

#[test]
fn flyback_tiny_loaded_limit_preserves_nonzero_currents() {
    let mut inputs = FLYBACK_INPUTS.to_vec();
    inputs[2].1 = 5e-201;
    let r = calculate(FLYBACK, &inputs).unwrap();
    for (key, value) in [("primary_peak_current", 2e-100), ("duty", 1e-100 / 6.0),
        ("demagnetization_fraction", 1e-100 / 3.0), ("idle_fraction", 1.0),
        ("primary_rms_current", (2.0_f64 / 9.0).sqrt() * 1e-150),
        ("secondary_rms_current", 1e-150), ("switch_blocking_voltage", 36.0),
        ("diode_reverse_voltage", 24.0)] {
        assert_close(r[key], value, 1e-12, 0.0);
    }
}

// Solve jXm || Rac in rectangular coordinates, then take |Zparallel/Ztotal|.
// This does not reuse the catalog's normalized closed-form gain equation.
fn llc_impedance_oracle(vin: f64, n: f64, lr: f64, lm: f64, cr: f64, fs: f64, ro: f64) -> [(&'static str, f64); 8] {
    let omega = 2.0 * PI * fs;
    let rac = (n * ro.sqrt()).powi(2) * 8.0 / PI.powi(2);
    let xm = omega * lm;
    let conductance = 1.0 / rac;
    let susceptance = -1.0 / xm;
    let admittance_square = conductance * conductance + susceptance * susceptance;
    let real = conductance / admittance_square;
    let imaginary = -susceptance / admittance_square;
    let xin = omega * lr - 1.0 / (omega * cr) + imaginary;
    let gain = real.hypot(imaginary) / real.hypot(xin);
    let fr = 1.0 / (2.0 * PI * (lr * cr).sqrt());
    [("resonant_frequency", fr), ("normalized_frequency", fs / fr),
        ("ac_load_resistance", rac), ("quality_factor", (lr / cr).sqrt() / rac),
        ("inductance_ratio", lm / lr), ("tank_gain", gain),
        ("output_voltage", (vin / n) * gain / 2.0), ("input_reactance", xin)]
}

#[test]
fn llc_complete_nondefault_complex_impedance_and_capacitive_region() {
    for fs in [50000.0, 10000.0] {
        let mut inputs = LLC_INPUTS.to_vec();
        inputs[5].1 = fs;
        let r = calculate(LLC, &inputs).unwrap();
        for (key, value) in llc_impedance_oracle(300.0, 3.0, 40e-6, 120e-6, 250e-9, fs, 6.0) {
            assert_close(r[key], value, 1e-11, 0.0);
        }
        assert_eq!(r["input_reactance"] > 0.0, fs == 50000.0);
    }
}

#[test]
fn llc_series_resonance_is_load_independent_even_with_small_lm() {
    let fr = 1.0 / (2.0 * PI * (40e-6_f64 * 250e-9).sqrt());
    for ro in [0.1, 6.0, 1e6] {
        for lm in [20e-6, 120e-6] {
            let mut inputs = LLC_INPUTS.to_vec();
            inputs[3].1 = lm;
            inputs[5].1 = fr;
            inputs[6].1 = ro;
            let r = calculate(LLC, &inputs).unwrap();
            assert_close(r["tank_gain"], 1.0, 1e-12, 0.0);
            assert_close(r["output_voltage"], 50.0, 1e-12, 0.0);
            assert_close(r["normalized_frequency"], 1.0, 1e-12, 0.0);
            assert_close(r["inductance_ratio"], lm / 40e-6, 1e-12, 0.0);
        }
    }
}

#[test]
fn llc_lower_resonance_large_gain_is_not_clamped() {
    let mut inputs = LLC_INPUTS.to_vec();
    inputs[5].1 = 1.0 / (2.0 * PI * (160e-6_f64 * 250e-9).sqrt());
    inputs[6].1 = 1e5;
    let r = calculate(LLC, &inputs).unwrap();
    // x=1/2, k=3: reactive denominator cancels, leaving gain=Rac/(1.5 Z0).
    let gain = (72.0 * 1e5 / PI.powi(2)) / (1.5 * 160.0_f64.sqrt());
    assert_close(r["tank_gain"], gain, 1e-10, 0.0);
    assert_close(r["output_voltage"], 50.0 * gain, 1e-10, 0.0);
    assert!(r["tank_gain"] > 10000.0);
}

#[test]
fn llc_representable_reflected_load_must_not_overflow_ratio_square() {
    let mut inputs = LLC_INPUTS.to_vec();
    inputs[0].1 = 3e155;
    inputs[1].1 = 1e155;
    inputs[6].1 = 1e-300;
    let r = calculate(LLC, &inputs).expect("finite reflected load despite n squared overflowing");
    for (key, value) in llc_impedance_oracle(3e155, 1e155, 40e-6, 120e-6, 250e-9, 50000.0, 1e-300) {
        assert_close(r[key], value, 1e-11, 0.0);
    }
}

#[test]
fn dab_complete_nondefault_power_from_two_current_segments() {
    let r = calculate(DAB, DAB_INPUTS).unwrap();
    // Half-period=6.25 us, lag=half/3. Integrate primary v*i using antisymmetry.
    let half: f64 = 1.0 / 160000.0;
    let lag = half / 3.0;
    let rising = 500.0 / 75e-6;
    let remaining = 100.0 / 75e-6;
    let initial = -(rising * lag + remaining * (half - lag)) / 2.0;
    let corner = initial + rising * lag;
    let end = -initial;
    let average = ((initial + corner) * lag + (corner + end) * (half - lag)) / (2.0 * half);
    let power = 300.0 * average;
    for (key, value) in [("voltage_ratio", 2.0 / 3.0), ("transferred_power", power),
        ("maximum_power", 1250.0), ("primary_bus_current", average),
        ("secondary_bus_current", power / 80.0), ("phase_delay", lag)] {
        assert_close(r[key], value, 1e-12, 0.0);
    }
}

#[test]
fn dab_reverse_maximum_falling_branch_and_closed_endpoints() {
    for (phase, power) in [(0.0, 0.0), (PI / 3.0, 10000.0 / 9.0),
        (-PI / 3.0, -10000.0 / 9.0), (PI / 2.0, 1250.0),
        (-PI / 2.0, -1250.0), (2.0 * PI / 3.0, 10000.0 / 9.0),
        (-2.0 * PI / 3.0, -10000.0 / 9.0), (PI, 0.0), (-PI, 0.0)] {
        let mut inputs = DAB_INPUTS.to_vec();
        inputs[5].1 = phase;
        let r = calculate(DAB, &inputs).unwrap();
        for (key, value) in [("voltage_ratio", 2.0 / 3.0), ("maximum_power", 1250.0),
            ("transferred_power", power), ("primary_bus_current", power / 300.0),
            ("secondary_bus_current", power / 80.0), ("phase_delay", phase / (160000.0 * PI))] {
            assert_close(r[key], value, 1e-12, 0.0);
        }
        assert_close(300.0 * r["primary_bus_current"], 80.0 * r["secondary_bus_current"], 1e-12, 0.0);
    }
}

#[test]
fn dab_tiny_signed_phase_and_frequency_inductance_scaling() {
    let mut inputs = DAB_INPUTS.to_vec();
    inputs[5].1 = -1e-200;
    let r = calculate(DAB, &inputs).unwrap();
    assert_close(r["transferred_power"], -5000.0 / PI * 1e-200, 1e-12, 0.0);
    assert_close(r["phase_delay"], -1e-200 / (160000.0 * PI), 1e-12, 0.0);
    for index in [3, 4] {
        let mut inputs = DAB_INPUTS.to_vec();
        inputs[index].1 *= 2.0;
        let r = calculate(DAB, &inputs).unwrap();
        assert_close(r["transferred_power"], 5000.0 / 9.0, 1e-12, 0.0);
        assert_close(r["maximum_power"], 625.0, 1e-12, 0.0);
        let delay = if index == 3 { 1.0 / 960000.0 } else { 1.0 / 480000.0 };
        assert_close(r["phase_delay"], delay, 1e-12, 0.0);
    }
}

#[test]
fn dab_outside_phase_domain_rejects_then_reverse_power_recovers() {
    for bad in [PI + 1e-9, -PI - 1e-9] {
        let mut inputs = DAB_INPUTS.to_vec();
        inputs[5].1 = bad;
        assert!(calculate(DAB, &inputs).is_err());
        inputs[5].1 = -PI / 2.0;
        let r = calculate(DAB, &inputs).unwrap();
        assert_close(r["transferred_power"], -1250.0, 1e-12, 0.0);
        assert_close(r["phase_delay"], -3.125e-6, 1e-12, 0.0);
    }
}

#[test]
fn all_converter_scalar_domains_nonfinite_missing_and_valid_recovery() {
    for (id, valid, sentinel, expected) in [
        (PUMP, PUMP_INPUTS, "output_voltage", -6.7),
        (SEPIC, SEPIC_INPUTS, "switch_peak_current", 4.625),
        (FORWARD, FORWARD_INPUTS, "switch_blocking_voltage", 240.0),
        (FLYBACK, FLYBACK_INPUTS, "primary_peak_current", 2.0),
        (LLC, LLC_INPUTS, "inductance_ratio", 3.0),
        (DAB, DAB_INPUTS, "maximum_power", 1250.0),
    ] {
        for index in 0..valid.len() {
            let key = valid[index].0;
            let nonnegative = id == PUMP && matches!(key, "load_current" | "switch_resistance" | "flying_esr" | "output_esr");
            for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -1.0, 0.0] {
                if id == DAB && key == "phase_shift" && bad.is_finite() { continue; }
                if nonnegative && bad == 0.0 { continue; }
                let mut inputs = valid.to_vec();
                inputs[index].1 = bad;
                assert!(calculate(id, &inputs).is_err(), "{id} accepted {key}={bad}");
                assert_close(calculate(id, valid).unwrap()[sentinel], expected, 1e-12, 0.0);
            }
            let mut missing = valid.to_vec();
            missing.remove(index);
            assert!(calculate(id, &missing).is_err(), "{id} accepted missing {key}");
            assert_close(calculate(id, valid).unwrap()[sentinel], expected, 1e-12, 0.0);
        }
    }
}

#[test]
fn flyback_representable_intervals_must_not_overflow_inductance_frequency_product() {
    let r = calculate(FLYBACK, &[
        ("vin", 1e100), ("vout", 1e100), ("load_current", 1e-304),
        ("magnetizing_inductance", 1e200), ("switching_frequency", 1e200),
        ("primary_secondary_ratio", 1.0),
    ]).expect("finite energy, currents and intervals despite Lm*fs overflowing");
    let peak = 2.0_f64.sqrt() * 1e-302;
    let fraction = 2.0_f64.sqrt() * 0.01;
    for (key, value) in [("primary_peak_current", peak), ("duty", fraction),
        ("demagnetization_fraction", fraction), ("idle_fraction", 1.0 - 2.0 * fraction),
        ("primary_rms_current", peak * (fraction / 3.0).sqrt()),
        ("secondary_rms_current", peak * (fraction / 3.0).sqrt()),
        ("switch_blocking_voltage", 2e100), ("diode_reverse_voltage", 2e100)] {
        assert_close(r[key], value, 1e-11, 0.0);
    }
}

#[test]
fn dab_representable_power_must_apply_phase_before_overflowing_bus_product() {
    let r = calculate(DAB, &[
        ("primary_voltage", 2e154), ("secondary_voltage", 2e154),
        ("primary_secondary_ratio", 1.0), ("switching_frequency", 1.0),
        ("transfer_inductance", 1.0), ("phase_shift", PI / 2.0),
    ]).expect("finite maximum transfer despite the unscaled bus product overflowing");
    for (key, value) in [("voltage_ratio", 1.0), ("transferred_power", 5e307),
        ("maximum_power", 5e307), ("primary_bus_current", 2.5e153),
        ("secondary_bus_current", 2.5e153), ("phase_delay", 0.25)] {
        assert_close(r[key], value, 1e-12, 0.0);
    }
}
