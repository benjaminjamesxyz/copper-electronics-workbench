mod common;

use common::{assert_close, calculate};
use std::f64::consts::{LN_2, PI};

// Non-default oracles were derived with 60-digit decimal arithmetic from threshold
// charge balance, summing-node KCL, the loaded RC ladder, and the LC characteristic.
// No catalog example is an expectation. Nonzero quantities have no absolute floor.
const TIMER: &[(&str, f64)] = &[
    ("charge_resistance", 6000.0),
    ("discharge_resistance", 3000.0),
    ("timing_capacitance", 2.2e-08),
];
const TIMER_EXPECTED: &[(&str, f64)] = &[
    ("high_time", 0.00013724314175086918),
    ("low_time", 4.574771391695639e-05),
    ("period", 0.00018299085566782556),
    ("frequency", 5464.753942761225),
    ("high_duty", 0.75),
];
const COMPARATOR: &[(&str, f64)] = &[
    ("supply_voltage", 12.0),
    ("upper_resistance", 30000.0),
    ("lower_resistance", 10000.0),
    ("feedback_resistance", 10000.0),
    ("timing_resistance", 2000.0),
    ("timing_capacitance", 5e-08),
];
const COMPARATOR_EXPECTED: &[(&str, f64)] = &[
    ("upper_threshold", 6.857142857142857),
    ("lower_threshold", 1.7142857142857142),
    ("high_time", 6.931471805599453e-05),
    ("low_time", 0.00013862943611198905),
    ("frequency", 4808.983469629878),
    ("high_duty", 0.3333333333333333),
];
const TRIANGLE: &[(&str, f64)] = &[
    ("square_peak", 3.0),
    ("triangle_resistance", 30000.0),
    ("feedback_resistance", 12000.0),
    ("integrator_resistance", 4000.0),
    ("integrator_capacitance", 2.5e-08),
];
const TRIANGLE_EXPECTED: &[(&str, f64)] = &[
    ("triangle_peak", 7.5),
    ("triangle_pp", 15.0),
    ("ramp_slew", 30000.0),
    ("period", 0.001),
    ("frequency", 1000.0),
];
const WIEN: &[(&str, f64)] = &[
    ("bridge_resistance", 2000.0),
    ("bridge_capacitance", 2.5e-08),
    ("gain_resistance", 4000.0),
    ("feedback_resistance", 6000.0),
];
const WIEN_EXPECTED: &[(&str, f64)] = &[
    ("frequency", 3183.0988618379065),
    ("feedback_fraction", 0.3333333333333333),
    ("critical_feedback_resistance", 8000.0),
    ("amplifier_gain", 2.5),
    ("loop_gain", 0.8333333333333334),
    ("loop_excess", -0.16666666666666666),
];
const PHASE: &[(&str, f64)] = &[
    ("section_resistance", 2000.0),
    ("section_capacitance", 2.5e-08),
    ("feedback_resistance", 56000.0),
];
const PHASE_EXPECTED: &[(&str, f64)] = &[
    ("frequency", 1299.4946687227934),
    ("feedback_magnitude", 0.034482758620689655),
    ("critical_feedback_resistance", 58000.0),
    ("amplifier_gain_magnitude", 28.0),
    ("loop_gain", 0.9655172413793104),
    ("loop_excess", -0.034482758620689655),
];
const LC: &[(&str, f64)] = &[
    ("tank_inductance", 0.25),
    ("tank_capacitance", 0.0625),
    ("parallel_loss_resistance", 4.0),
    ("transistor_transconductance", 2.0),
];
const LC_EXPECTED: &[(&str, f64)] = &[
    ("tank_frequency", 1.2732395447351628),
    ("negative_resistance", -1.0),
    ("critical_transconductance", 0.5),
    ("startup_ratio", 4.0),
    ("growth_rate", 6.0),
    ("startup_frequency", 0.8421687986955848),
];
const PIERCE: &[(&str, f64)] = &[
    ("crystal_frequency", 8000000.0),
    ("crystal_esr", 80.0),
    ("shunt_capacitance", 5e-12),
    ("load_capacitance", 2e-11),
    ("stray_capacitance", 4e-12),
    ("available_transconductance", 0.001),
    ("margin_target", 3.0),
];
const PIERCE_EXPECTED: &[(&str, f64)] = &[
    ("load_capacitor_each", 3.2e-11),
    ("critical_transconductance", 0.0005053237453357752),
    ("gain_margin", 1.9789293680144096),
    ("design_transconductance", 0.0015159712360073255),
    ("transconductance_headroom", -0.0005159712360073255),
];
const CASES: &[(&str, &[(&str, f64)], &[(&str, f64)])] = &[
    ("oscillators.timer_555_astable", TIMER, TIMER_EXPECTED),
    ("oscillators.comparator_rc_oscillator", COMPARATOR, COMPARATOR_EXPECTED),
    ("oscillators.triangle_integrator_oscillator", TRIANGLE, TRIANGLE_EXPECTED),
    ("oscillators.wien_bridge_balance", WIEN, WIEN_EXPECTED),
    ("oscillators.loaded_rc_phase_shift", PHASE, PHASE_EXPECTED),
    ("oscillators.cross_coupled_lc_startup", LC, LC_EXPECTED),
    ("oscillators.pierce_crystal_startup", PIERCE, PIERCE_EXPECTED),
];

#[test]
fn all_seven_nondefault_complete_oracles() {
    for &(id, inputs, expected) in CASES {
        let out = calculate(id, inputs).unwrap();
        assert_eq!(out.len(), expected.len(), "{id}");
        for &(key, value) in expected {
            assert_close(out[key], value, 1e-12, 0.0);
        }
    }
}

#[test]
fn every_scalar_domain_rejects_and_recovers_with_complete_outputs() {
    for &(id, inputs, expected) in CASES {
        for index in 0..inputs.len() {
            let key = inputs[index].0;
            let zero_allowed = key == "shunt_capacitance" || key == "stray_capacitance";
            let invalid: &[f64] = if zero_allowed { &[-1.0] }
                else if key == "margin_target" { &[-1.0, 0.0, 0.9999999999999999] }
                else { &[-1.0, 0.0] };
            for &bad in invalid {
                let mut changed = inputs.to_vec();
                changed[index].1 = bad;
                assert!(calculate(id, &changed).is_err(), "accepted {id} {key}={bad}");
                let recovered = calculate(id, inputs).unwrap();
                for &(key, value) in expected {
                    assert_close(recovered[key], value, 1e-12, 0.0);
                }
            }
        }
    }
}

#[test]
fn every_nonfinite_input_rejects_and_recovers() {
    for &(id, inputs, expected) in CASES {
        for index in 0..inputs.len() {
            for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                let mut changed = inputs.to_vec();
                changed[index].1 = bad;
                assert!(calculate(id, &changed).is_err(), "accepted {id} {}={bad}", inputs[index].0);
                let recovered = calculate(id, inputs).unwrap();
                for &(key, value) in expected {
                    assert_close(recovered[key], value, 1e-12, 0.0);
                }
            }
        }
    }
}

#[test]
fn timer_capacitance_scaling_and_cycle_identity() {
    let mut inputs = TIMER.to_vec();
    inputs[2].1 *= 2.0;
    let out = calculate(CASES[0].0, &inputs).unwrap();
    for &(key, value) in TIMER_EXPECTED {
        let scale = match key { "frequency" => 0.5, "high_duty" => 1.0, _ => 2.0 };
        assert_close(out[key], scale * value, 1e-12, 0.0);
    }
    assert_close(out["period"] * out["frequency"], 1.0, 1e-12, 0.0);
    assert_close(out["high_time"] / out["period"], out["high_duty"], 1e-12, 0.0);
}

#[test]
fn timer_equal_resistors_and_small_charge_path_limit() {
    for (ra, duty) in [(3000.0, 2.0 / 3.0), (3e-6, 0.50000000025)] {
        let mut inputs = TIMER.to_vec();
        inputs[0].1 = ra;
        let out = calculate(CASES[0].0, &inputs).unwrap();
        assert_close(out["high_duty"], duty, 1e-12, 0.0);
        assert!(out["high_duty"] > 0.5);
        assert_close(out["low_time"], 4.574771391695639e-5, 1e-12, 0.0);
    }
}

#[test]
fn timer_frequency_screen_both_sides_and_recovery() {
    let boundary: f64 = 1e-5 / (12000.0 * LN_2);
    // Sixteen ulps clear arithmetic rounding; do not equate decimal 10 us with
    // an exactly representable component-product boundary.
    let mut inputs = TIMER.to_vec();
    inputs[2].1 = f64::from_bits(boundary.to_bits() - 16);
    assert!(calculate(CASES[0].0, &inputs).is_err());
    inputs[2].1 = f64::from_bits(boundary.to_bits() + 16);
    let out = calculate(CASES[0].0, &inputs).unwrap();
    assert_close(out["frequency"], 100000.0, 1e-12, 0.0);
    assert_close(out["high_duty"], 0.75, 1e-12, 0.0);
    assert!(out["frequency"] <= 100000.0);
    inputs[2].1 = 1e-12;
    assert!(calculate(CASES[0].0, &inputs).is_err());
    assert_close(calculate(CASES[0].0, TIMER).unwrap()["period"], TIMER_EXPECTED[2].1, 1e-12, 0.0);
}

#[test]
fn comparator_threshold_kcl_and_exponential_endpoints() {
    let out = calculate(CASES[1].0, COMPARATOR).unwrap();
    let vh = out["upper_threshold"];
    let vl = out["lower_threshold"];
    // Solve charge evolution independently from state-dependent threshold formulas.
    assert_close(12.0 - (12.0 - vl) * (-out["high_time"] / 1e-4).exp(), vh, 1e-12, 0.0);
    assert_close(vh * (-out["low_time"] / 1e-4).exp(), vl, 1e-12, 0.0);
    assert_close((12.0-vh)/30000.0 + (12.0-vh)/10000.0, vh/10000.0, 1e-12, 0.0);
    assert_close((12.0-vl)/30000.0, vl/10000.0 + vl/10000.0, 1e-12, 0.0);
}

#[test]
fn comparator_supply_network_scaling_and_resistor_swap() {
    for mode in 0..3 {
        let mut inputs = COMPARATOR.to_vec();
        if mode == 0 { inputs[0].1 *= 2.0; }
        if mode == 1 { for item in &mut inputs[1..4] { item.1 *= 7.0; } }
        if mode == 2 { inputs[1].1 = 10000.0; inputs[2].1 = 30000.0; }
        let out = calculate(CASES[1].0, &inputs).unwrap();
        for &(key, value) in COMPARATOR_EXPECTED {
            let expected = match (mode, key) {
                (0, "upper_threshold" | "lower_threshold") => 2.0 * value,
                (2, "upper_threshold") => 12.0 - COMPARATOR_EXPECTED[1].1,
                (2, "lower_threshold") => 12.0 - COMPARATOR_EXPECTED[0].1,
                (2, "high_time") => COMPARATOR_EXPECTED[3].1,
                (2, "low_time") => COMPARATOR_EXPECTED[2].1,
                (2, "high_duty") => 2.0 / 3.0,
                _ => value,
            };
            assert_close(out[key], expected, 1e-12, 0.0);
        }
    }
}

#[test]
fn comparator_symmetric_and_tiny_hysteresis_intervals() {
    let mut inputs = COMPARATOR.to_vec();
    inputs[1].1 = 10000.0;
    let equal = calculate(CASES[1].0, &inputs).unwrap();
    assert_close(equal["upper_threshold"], 8.0, 1e-12, 0.0);
    assert_close(equal["lower_threshold"], 4.0, 1e-12, 0.0);
    assert_close(equal["high_duty"], 0.5, 1e-12, 0.0);
    assert_close(equal["frequency"], 7213.475204444817, 1e-12, 0.0);
    inputs[3].1 = 1e20;
    let tiny = calculate(CASES[1].0, &inputs).unwrap();
    // ln(1+x)=x to binary64 accuracy at x=1e-16; plain ln(1+x) loses the interval.
    assert_close(tiny["high_time"], 1e-20, 1e-12, 0.0);
    assert_close(tiny["low_time"], 1e-20, 1e-12, 0.0);
    assert_close(tiny["frequency"], 5e19, 1e-12, 0.0);
    assert_close(tiny["high_duty"], 0.5, 1e-12, 0.0);
}

#[test]
fn triangle_ramp_charge_balance_and_amplitude_independent_period() {
    for square in [3.0, 6.0] {
        let mut inputs = TRIANGLE.to_vec();
        inputs[0].1 = square;
        let out = calculate(CASES[2].0, &inputs).unwrap();
        assert_close(out["triangle_peak"], 2.5*square, 1e-12, 0.0);
        assert_close(out["triangle_pp"], 5.0*square, 1e-12, 0.0);
        assert_close(out["ramp_slew"], 10000.0*square, 1e-12, 0.0);
        assert_close(out["period"], 0.001, 1e-12, 0.0);
        assert_close(out["frequency"], 1000.0, 1e-12, 0.0);
        assert_close(25e-9*out["triangle_pp"], square/4000.0*out["period"]/2.0, 1e-12, 0.0);
    }
}

#[test]
fn triangle_resistor_scaling_and_small_threshold_limit() {
    for (index, factor, peak, slew, period) in [
        (1, 2.0, 15.0, 30000.0, 0.002),
        (3, 2.0, 7.5, 15000.0, 0.002),
        (1, 1e-12, 7.5e-12, 30000.0, 1e-15),
    ] {
        let mut inputs = TRIANGLE.to_vec();
        inputs[index].1 *= factor;
        let out = calculate(CASES[2].0, &inputs).unwrap();
        assert_close(out["triangle_peak"], peak, 1e-12, 0.0);
        assert_close(out["triangle_pp"], 2.0*peak, 1e-12, 0.0);
        assert_close(out["ramp_slew"], slew, 1e-12, 0.0);
        assert_close(out["period"], period, 1e-12, 0.0);
        assert_close(out["frequency"], 1.0/period, 1e-12, 0.0);
    }
}

#[test]
fn wien_marginal_decay_growth_and_one_ulp_excess() {
    for (rf, gain, excess) in [(7600.0, 2.9, -1.0/30.0), (8000.0, 3.0, 0.0), (8400.0, 3.1, 1.0/30.0)] {
        let mut inputs = WIEN.to_vec();
        inputs[3].1 = rf;
        let out = calculate(CASES[3].0, &inputs).unwrap();
        assert_close(out["amplifier_gain"], gain, 1e-12, 0.0);
        assert_close(out["loop_gain"], gain/3.0, 1e-12, 0.0);
        assert_close(out["loop_excess"], excess, 1e-12, 0.0);
    }
    let mut inputs = WIEN.to_vec();
    inputs[2].1 = 1.0;
    inputs[3].1 = f64::from_bits(2.0_f64.to_bits()+1);
    let out = calculate(CASES[3].0, &inputs).unwrap();
    assert_close(out["loop_excess"], 2.0*f64::EPSILON/3.0, 1e-12, 0.0);
}

#[test]
fn wien_bridge_and_gain_network_scaling() {
    for mode in 0..3 {
        let mut inputs = WIEN.to_vec();
        if mode < 2 { inputs[mode].1 *= 2.0; }
        else { inputs[2].1 *= 2.0; inputs[3].1 *= 2.0; }
        let out = calculate(CASES[3].0, &inputs).unwrap();
        for &(key, value) in WIEN_EXPECTED {
            let scale = match (mode, key) {
                (0 | 1, "frequency") => 0.5,
                (2, "critical_feedback_resistance") => 2.0,
                _ => 1.0,
            };
            assert_close(out[key], value*scale, 1e-12, 0.0);
        }
    }
}

#[test]
fn loaded_phase_ladder_complex_response_not_buffered_response() {
    let out = calculate(CASES[4].0, PHASE).unwrap();
    let y = 2.0*PI*out["frequency"]*2000.0*25e-9;
    // Substitute x=j*wRC in the determinant from three coupled nodal equations.
    let numerator_im = -y*y*y;
    let denominator_re = 1.0-6.0*y*y;
    let denominator_im = 5.0*y-y*y*y;
    let norm = denominator_re*denominator_re+denominator_im*denominator_im;
    let beta_re = numerator_im*denominator_im/norm;
    let beta_im = numerator_im*denominator_re/norm;
    assert_close(beta_re, -out["feedback_magnitude"], 1e-12, 0.0);
    // Roundoff in cancellation of dimensionless 1-6y^2, not an electrical floor.
    assert_close(beta_im, 0.0, 0.0, 32.0*f64::EPSILON);
    assert_close(out["critical_feedback_resistance"]*(-beta_re), 2000.0, 1e-12, 0.0);
}

#[test]
fn loaded_phase_marginal_decay_and_component_scaling() {
    let mut marginal = PHASE.to_vec();
    marginal[2].1 = 58000.0;
    let out = calculate(CASES[4].0, &marginal).unwrap();
    assert_close(out["amplifier_gain_magnitude"], 29.0, 1e-12, 0.0);
    assert_eq!(out["loop_gain"], 1.0);
    assert_eq!(out["loop_excess"], 0.0);
    for mode in 0..2 {
        let mut inputs = PHASE.to_vec();
        if mode == 0 { inputs[1].1 *= 2.0; }
        else { inputs[0].1 *= 2.0; inputs[2].1 *= 2.0; }
        let out = calculate(CASES[4].0, &inputs).unwrap();
        for &(key, value) in PHASE_EXPECTED {
            let scale = match (mode, key) {
                (_, "frequency") => 0.5,
                (1, "critical_feedback_resistance") => 2.0,
                _ => 1.0,
            };
            assert_close(out[key], scale*value, 1e-12, 0.0);
        }
    }
}

#[test]
fn lc_differential_poles_satisfy_tank_characteristic() {
    let out = calculate(CASES[5].0, LC).unwrap();
    let alpha = out["growth_rate"];
    let omega = 2.0*PI*out["startup_frequency"];
    // C*s^2+(1/Rp-gm/2)*s+1/L=0; compare terms rather than accepting finite poles.
    assert_close(0.0625*(alpha*alpha-omega*omega)+4.0, 0.75*alpha, 1e-12, 0.0);
    assert_close(2.0*0.0625*alpha*omega, 0.75*omega, 1e-12, 0.0);
    assert_close(out["negative_resistance"], -1.0, 1e-12, 0.0);
}

#[test]
fn lc_marginal_and_decaying_diagnostics() {
    for (gm, alpha, ratio, negative) in [(0.5, 0.0, 1.0, -4.0), (0.25, -1.0, 0.5, -8.0)] {
        let mut inputs = LC.to_vec();
        inputs[3].1 = gm;
        let out = calculate(CASES[5].0, &inputs).unwrap();
        assert_close(out["growth_rate"], alpha, 1e-12, 0.0);
        assert_close(out["startup_ratio"], ratio, 1e-12, 0.0);
        assert_close(out["negative_resistance"], negative, 1e-12, 0.0);
        assert_close(out["startup_frequency"], (64.0_f64-alpha*alpha).sqrt()/(2.0*PI), 1e-12, 0.0);
    }
}

#[test]
fn lc_real_pole_boundary_rejection_then_oscillatory_recovery() {
    let mut inputs = [("tank_inductance", 1.0), ("tank_capacitance", 1.0),
        ("parallel_loss_resistance", 1.0), ("transistor_transconductance", 6.0)];
    for gm in [6.0, 7.0] {
        inputs[3].1 = gm;
        assert!(calculate(CASES[5].0, &inputs).is_err());
        inputs[3].1 = 5.0;
        let out = calculate(CASES[5].0, &inputs).unwrap();
        assert_close(out["growth_rate"], 0.75, 1e-12, 0.0);
        assert_close(out["startup_frequency"], 7.0_f64.sqrt()/(8.0*PI), 1e-12, 0.0);
    }
    // Exercise the negative-growth side of the same absolute-value constraint.
    inputs[2].1 = 0.25;
    inputs[3].1 = 4.0;
    assert!(calculate(CASES[5].0, &inputs).is_err());
    inputs[3].1 = 5.0;
    let out = calculate(CASES[5].0, &inputs).unwrap();
    assert_close(out["growth_rate"], -0.75, 1e-12, 0.0);
    assert_close(out["startup_frequency"], 7.0_f64.sqrt()/(8.0*PI), 1e-12, 0.0);
}

#[test]
fn pierce_zero_parasitics_and_esr_frequency_scaling() {
    for mode in 0..4 {
        let mut inputs = PIERCE.to_vec();
        let (cap, factor) = match mode {
            0 => { inputs[4].1=0.0; (40e-12, 1.0) },
            1 => { inputs[2].1=0.0; (32e-12, 0.64) },
            2 => { inputs[1].1*=2.0; (32e-12, 2.0) },
            _ => { inputs[0].1*=2.0; (32e-12, 4.0) },
        };
        let out = calculate(CASES[6].0, &inputs).unwrap();
        let critical = PIERCE_EXPECTED[1].1*factor;
        assert_close(out["load_capacitor_each"], cap, 1e-12, 0.0);
        assert_close(out["critical_transconductance"], critical, 1e-12, 0.0);
        assert_close(out["gain_margin"], 0.001/critical, 1e-12, 0.0);
        assert_close(out["design_transconductance"], 3.0*critical, 1e-12, 0.0);
        assert_close(out["transconductance_headroom"], 0.001-3.0*critical, 1e-12, 0.0);
    }
}

#[test]
fn pierce_target_boundary_and_tiny_positive_external_capacitors() {
    let mut inputs = PIERCE.to_vec();
    inputs[5].1 = PIERCE_EXPECTED[3].1;
    let out = calculate(CASES[6].0, &inputs).unwrap();
    // A few ulps of the ~1.5 mS design gm can remain after independent pi arithmetic.
    assert_close(out["transconductance_headroom"], 0.0, 0.0, 16.0*f64::EPSILON*inputs[5].1);
    assert_close(out["gain_margin"], 3.0, 1e-12, 0.0);
    inputs[6].1 = 1.0;
    inputs[4].1 = f64::from_bits(inputs[3].1.to_bits()-1);
    let out = calculate(CASES[6].0, &inputs).unwrap();
    assert_close(out["load_capacitor_each"], 2.0*(inputs[3].1-inputs[4].1), 1e-12, 0.0);
    assert_close(out["design_transconductance"], PIERCE_EXPECTED[1].1, 1e-12, 0.0);
}

#[test]
fn pierce_stray_equality_and_excess_reject_then_recover() {
    for stray in [20e-12, 21e-12] {
        let mut inputs = PIERCE.to_vec();
        inputs[4].1 = stray;
        assert!(calculate(CASES[6].0, &inputs).is_err());
        let recovered = calculate(CASES[6].0, PIERCE).unwrap();
        for &(key, value) in PIERCE_EXPECTED {
            assert_close(recovered[key], value, 1e-12, 0.0);
        }
        assert!(recovered["transconductance_headroom"] < 0.0);
    }
}

// The following accepted-domain regressions deliberately expose intermediate
// overflow/underflow. They are mathematical conditioning tests, not hardware designs.
#[test]
fn timer_large_resistors_with_finite_time_constants() {
    let out = calculate(CASES[0].0, &[("charge_resistance", 1e308),
        ("discharge_resistance", 1e308), ("timing_capacitance", 1e-308)]).unwrap();
    for (key, value) in [("high_time", 2.0*LN_2), ("low_time", LN_2),
        ("period", 3.0*LN_2), ("frequency", 1.0/(3.0*LN_2)), ("high_duty", 2.0/3.0)] {
        assert_close(out[key], value, 1e-12, 0.0);
    }
}

#[test]
fn comparator_extreme_feedback_has_finite_thresholds_and_intervals() {
    let out = calculate(CASES[1].0, &[("supply_voltage", 2.0), ("upper_resistance", 1.0),
        ("lower_resistance", 1.0), ("feedback_resistance", 1e308),
        ("timing_resistance", 1e150), ("timing_capacitance", 1e150)]).unwrap();
    for (key, value) in [("upper_threshold", 1.0), ("lower_threshold", 1.0),
        ("high_time", 1e-8), ("low_time", 1e-8), ("frequency", 5e7), ("high_duty", 0.5)] {
        assert_close(out[key], value, 1e-12, 0.0);
    }
}

#[test]
fn triangle_large_resistance_small_capacitance_preserves_finite_period() {
    let out = calculate(CASES[2].0, &[("square_peak", 1.0), ("triangle_resistance", 1.0),
        ("feedback_resistance", 1.0), ("integrator_resistance", 1e308),
        ("integrator_capacitance", 1e-308)]).unwrap();
    for (key, value) in [("triangle_peak", 1.0), ("triangle_pp", 2.0),
        ("ramp_slew", 1.0), ("period", 4.0), ("frequency", 0.25)] {
        assert_close(out[key], value, 1e-12, 0.0);
    }
}

#[test]
fn lc_startup_ratio_remains_representable_after_factor_two() {
    let out = calculate(CASES[5].0, &[("tank_inductance", 1.0), ("tank_capacitance", 1.0),
        ("parallel_loss_resistance", 1e308), ("transistor_transconductance", 2.0)]).unwrap();
    for (key, value) in [("tank_frequency", 1.0/(2.0*PI)), ("negative_resistance", -1.0),
        ("critical_transconductance", 2e-308), ("startup_ratio", 1e308),
        ("growth_rate", 0.5), ("startup_frequency", 3.0_f64.sqrt()/(4.0*PI))] {
        assert_close(out[key], value, 1e-12, 0.0);
    }
}

#[test]
fn pierce_small_admittance_squared_retains_representable_critical_gm() {
    let out = calculate(CASES[6].0, &[("crystal_frequency", 1e-100), ("crystal_esr", 1e200),
        ("shunt_capacitance", 0.0), ("load_capacitance", 1e-100), ("stray_capacitance", 0.0),
        ("available_transconductance", 1e-198), ("margin_target", 1.0)]).unwrap();
    let critical: f64 = 16.0*PI*PI*1e-200;
    for (key, value) in [("load_capacitor_each", 2e-100), ("critical_transconductance", critical),
        ("gain_margin", 100.0/(16.0*PI*PI)), ("design_transconductance", critical),
        ("transconductance_headroom", 1e-198-critical)] {
        assert_close(out[key], value, 1e-12, 0.0);
    }
}

#[test]
fn wien_tiny_resistance_large_capacitance_preserves_frequency() {
    let out = calculate(CASES[3].0, &[("bridge_resistance", 1e-310),
        ("bridge_capacitance", 1e308), ("gain_resistance", 10000.0),
        ("feedback_resistance", 20200.0)]).unwrap();
    for (key, value) in [("frequency", 50.0/PI), ("feedback_fraction", 1.0/3.0),
        ("critical_feedback_resistance", 20000.0), ("amplifier_gain", 3.02),
        ("loop_gain", 151.0/150.0), ("loop_excess", 1.0/150.0)] {
        assert_close(out[key], value, 1e-12, 0.0);
    }
}

#[test]
fn loaded_phase_tiny_resistance_large_capacitance_preserves_frequency() {
    let out = calculate(CASES[4].0, &[("section_resistance", 1e-310),
        ("section_capacitance", 1e308), ("feedback_resistance", 3e-309)]).unwrap();
    for (key, value) in [("frequency", 50.0/(PI*6.0_f64.sqrt())),
        ("feedback_magnitude", 1.0/29.0), ("critical_feedback_resistance", 2.9e-309),
        ("amplifier_gain_magnitude", 30.0), ("loop_gain", 30.0/29.0),
        ("loop_excess", 1.0/29.0)] {
        assert_close(out[key], value, 1e-12, 0.0);
    }
}
