mod common;

use common::{assert_close, calculate};

// Oracles come from volt-second balance, charge/waveform integration and power
// conservation in research/power.md, not from the runtime catalog. Nonzero
// quantities always use relative-only tolerance, including tiny A/H/F/W values.
// Structural zeros below are exact zero-current/zero-ESR/unity-efficiency
// identities: absolute tolerance 0 is intentional, not a blanket noise floor.

#[test]
fn buck_inductor_complete_researched_example() {
    let out = calculate("power.buck_inductor_design", &[
        ("vin", 12.0), ("vout", 5.0), ("iout", 2.0),
        ("fs", 500_000.0), ("ripple_ratio", 0.3),
    ]).unwrap();
    for (key, expected) in [
        ("duty", 0.4166666666666667), ("delta_i", 0.6),
        ("l_min", 9.722222222222223e-6), ("i_peak", 2.3), ("i_valley", 1.7),
    ] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn buck_inductor_independent_volt_seconds_and_ccm_boundary() {
    // 18 V on-state for one quarter-cycle gives 4.5 V average on area;
    // 1.2 A ripple at 100 kHz therefore requires 37.5 uH.
    let out = calculate("power.buck_inductor_design", &[
        ("vin", 24.0), ("vout", 6.0), ("iout", 3.0),
        ("fs", 100_000.0), ("ripple_ratio", 0.4),
    ]).unwrap();
    for (key, expected) in [
        ("duty", 0.25), ("delta_i", 1.2), ("l_min", 37.5e-6),
        ("i_peak", 3.6), ("i_valley", 2.4),
    ] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
    assert_close(18.0 * out["duty"] - 6.0 * (1.0 - out["duty"]), 0.0, 0.0, 0.0);
    assert_close((out["i_peak"] + out["i_valley"]) / 2.0, 3.0, 1e-6, 0.0);
    assert_close(18.0 * 0.25 / (100_000.0 * out["l_min"]), 1.2, 1e-6, 0.0);

    // r = 2-2^-52 is the admissible binary64 neighbor below the excluded
    // zero-valley endpoint. Its valley is 3*2^-53 A, not rounded-away zero.
    let edge = calculate("power.buck_inductor_design", &[
        ("vin", 24.0), ("vout", 6.0), ("iout", 3.0),
        ("fs", 100_000.0), ("ripple_ratio", 1.9999999999999998),
    ]).unwrap();
    for (key, expected) in [
        ("duty", 0.25), ("delta_i", 5.999999999999999),
        ("l_min", 7.500000000000001e-6), ("i_peak", 6.0),
        ("i_valley", 3.3306690738754696e-16),
    ] {
        assert_close(edge[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn buck_inductor_rejects_every_scalar_and_topology_guard() {
    let base = [("vin", 24.0), ("vout", 6.0), ("iout", 3.0),
        ("fs", 100_000.0), ("ripple_ratio", 0.4)];
    for index in 0..base.len() {
        for value in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            let mut inputs = base;
            inputs[index].1 = value;
            assert!(calculate("power.buck_inductor_design", &inputs).is_err(),
                "accepted {}={value}", inputs[index].0);
        }
    }
    for ratio in [2.0, 2.1] {
        let mut inputs = base;
        inputs[4].1 = ratio;
        assert!(calculate("power.buck_inductor_design", &inputs).is_err());
    }
    for vin in [6.0, 5.0] {
        let mut inputs = base;
        inputs[0].1 = vin;
        assert!(calculate("power.buck_inductor_design", &inputs).is_err());
    }
}

#[test]
fn boost_inductor_complete_researched_example() {
    let out = calculate("power.boost_inductor_design", &[
        ("vin", 5.0), ("vout", 12.0), ("iout", 0.5),
        ("fs", 500_000.0), ("ripple_ratio", 0.3),
    ]).unwrap();
    for (key, expected) in [
        ("duty", 0.5833333333333334), ("i_l_avg", 1.2), ("delta_i", 0.36),
        ("l_min", 16.203703703703704e-6), ("i_peak", 1.38), ("i_valley", 1.02),
    ] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn boost_inductor_independent_input_current_and_ccm_boundary() {
    // A 3:1 ideal boost supplying 1 A must draw mean 3 A, not 1 A.
    let out = calculate("power.boost_inductor_design", &[
        ("vin", 6.0), ("vout", 18.0), ("iout", 1.0),
        ("fs", 100_000.0), ("ripple_ratio", 0.5),
    ]).unwrap();
    for (key, expected) in [
        ("duty", 2.0 / 3.0), ("i_l_avg", 3.0), ("delta_i", 1.5),
        ("l_min", 26.666666666666667e-6), ("i_peak", 3.75), ("i_valley", 2.25),
    ] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
    assert_close(6.0 * out["i_l_avg"], 18.0, 1e-6, 0.0);
    assert_close(6.0 * (2.0 / 3.0) / (100_000.0 * out["l_min"]), 1.5, 1e-6, 0.0);

    let edge = calculate("power.boost_inductor_design", &[
        ("vin", 6.0), ("vout", 18.0), ("iout", 1.0),
        ("fs", 100_000.0), ("ripple_ratio", 1.9999999999999998),
    ]).unwrap();
    for (key, expected) in [
        ("duty", 2.0 / 3.0), ("i_l_avg", 3.0), ("delta_i", 5.999999999999999),
        ("l_min", 6.666666666666668e-6), ("i_peak", 6.0),
        ("i_valley", 3.3306690738754696e-16),
    ] {
        assert_close(edge[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn boost_inductor_preserves_large_and_near_unity_conversion_ratios() {
    // Displayed duty rounds to one, but the directly specified voltage ratio
    // still gives finite 1 A mean current and finite effective inductance.
    let high = calculate("power.boost_inductor_design", &[
        ("vin", 1.0), ("vout", 1e20), ("iout", 1e-20),
        ("fs", 100_000.0), ("ripple_ratio", 0.5),
    ]).unwrap();
    for (key, expected) in [
        ("duty", 1.0), ("i_l_avg", 1.0), ("delta_i", 0.5),
        ("l_min", 20e-6), ("i_peak", 1.25), ("i_valley", 0.75),
    ] {
        assert_close(high[key], expected, 1e-6, 0.0);
    }
    // Rail separation is exactly 2^-52 V. Volt-second balance gives a
    // representable tiny on-duty and L even with only one unit of fs.
    let near = calculate("power.boost_inductor_design", &[
        ("vin", 1.0), ("vout", 1.0000000000000002), ("iout", 1.0),
        ("fs", 1.0), ("ripple_ratio", 0.5),
    ]).unwrap();
    for (key, expected) in [
        ("duty", 2.2204460492503126e-16), ("i_l_avg", 1.0000000000000002),
        ("delta_i", 0.5000000000000001), ("l_min", 4.440892098500624e-16),
        ("i_peak", 1.2500000000000002), ("i_valley", 0.7500000000000002),
    ] {
        assert_close(near[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn boost_inductor_rejects_every_scalar_and_topology_guard() {
    let base = [("vin", 6.0), ("vout", 18.0), ("iout", 1.0),
        ("fs", 100_000.0), ("ripple_ratio", 0.5)];
    for index in 0..base.len() {
        for value in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            let mut inputs = base;
            inputs[index].1 = value;
            assert!(calculate("power.boost_inductor_design", &inputs).is_err(),
                "accepted {}={value}", inputs[index].0);
        }
    }
    for ratio in [2.0, 2.1] {
        let mut inputs = base;
        inputs[4].1 = ratio;
        assert!(calculate("power.boost_inductor_design", &inputs).is_err());
    }
    for vout in [6.0, 5.0] {
        let mut inputs = base;
        inputs[1].1 = vout;
        assert!(calculate("power.boost_inductor_design", &inputs).is_err());
    }
}

#[test]
fn buck_output_capacitor_complete_researched_example() {
    let out = calculate("power.buck_output_capacitor", &[
        ("delta_i", 0.6), ("iout", 2.0), ("vout", 5.0),
        ("fs", 500_000.0), ("ripple_budget", 0.02), ("esr", 0.01),
    ]).unwrap();
    for (key, expected) in [
        ("ripple_esr", 0.006), ("ripple_cap_budget", 0.014),
        ("c_min", 10.714285714285714e-6),
    ] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn buck_output_capacitor_independent_charge_esr_and_small_ripple_boundary() {
    // The positive current triangle transfers 1 uC per cycle; 40 mV ESR
    // leaves 60 mV, requiring 16 2/3 uF, irrespective of the 4 V DC level.
    let out = calculate("power.buck_output_capacitor", &[
        ("delta_i", 0.8), ("iout", 1.0), ("vout", 4.0),
        ("fs", 100_000.0), ("ripple_budget", 0.1), ("esr", 0.05),
    ]).unwrap();
    for (key, expected) in [
        ("ripple_esr", 0.04), ("ripple_cap_budget", 0.06),
        ("c_min", 16.666666666666667e-6),
    ] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
    assert_close(out["c_min"] * out["ripple_cap_budget"], 1e-6, 1e-6, 0.0);
    assert_close(0.8 / (8.0 * 100_000.0 * out["c_min"]) + out["ripple_esr"], 0.1, 1e-6, 0.0);

    let edge = calculate("power.buck_output_capacitor", &[
        ("delta_i", 0.8), ("iout", 1.0), ("vout", 4.0),
        ("fs", 100_000.0), ("ripple_budget", 0.4), ("esr", 0.0),
    ]).unwrap();
    assert_close(edge["ripple_esr"], 0.0, 0.0, 0.0);
    assert_close(edge["ripple_cap_budget"], 0.4, 1e-6, 0.0);
    assert_close(edge["c_min"], 2.5e-6, 1e-6, 0.0);
}

#[test]
fn buck_output_capacitor_accepts_positive_budget_neighbor() {
    // Budget 1/8+2^-55 V minus exact ESR excursion 1/8 V leaves 2^-55 V;
    // one-eighth coulomb per cycle requires exactly 2^52 F.
    let out = calculate("power.buck_output_capacitor", &[
        ("delta_i", 1.0), ("iout", 1.0), ("vout", 2.0),
        ("fs", 1.0), ("ripple_budget", 0.12500000000000003), ("esr", 0.125),
    ]).unwrap();
    assert_close(out["ripple_esr"], 0.125, 1e-6, 0.0);
    assert_close(out["ripple_cap_budget"], 2.7755575615628914e-17, 1e-6, 0.0);
    assert_close(out["c_min"], 4_503_599_627_370_496.0, 1e-6, 0.0);
}

#[test]
fn buck_output_capacitor_rejects_every_scalar_ccm_esr_and_ripple_guard() {
    let base = [("delta_i", 0.8), ("iout", 1.0), ("vout", 4.0),
        ("fs", 100_000.0), ("ripple_budget", 0.1), ("esr", 0.05)];
    for index in 0..base.len() {
        for value in [-1.0, f64::NAN, f64::INFINITY] {
            let mut inputs = base;
            inputs[index].1 = value;
            assert!(calculate("power.buck_output_capacitor", &inputs).is_err());
        }
        if index != 5 {
            let mut inputs = base;
            inputs[index].1 = 0.0;
            assert!(calculate("power.buck_output_capacitor", &inputs).is_err());
        }
    }
    for iout in [0.4, 0.3] {
        let mut inputs = base;
        inputs[1].1 = iout;
        assert!(calculate("power.buck_output_capacitor", &inputs).is_err());
    }
    // Exact dyadic ESR exhaustion, rather than relying on decimal rounding.
    for budget in [0.125, 0.12] {
        assert!(calculate("power.buck_output_capacitor", &[
            ("delta_i", 1.0), ("iout", 1.0), ("vout", 2.0),
            ("fs", 1.0), ("ripple_budget", budget), ("esr", 0.125),
        ]).is_err());
    }
    let mut inputs = base;
    inputs[4].1 = 0.4000000000000001;
    assert!(calculate("power.buck_output_capacitor", &inputs).is_err());
}

#[test]
fn boost_output_capacitor_complete_researched_example() {
    let out = calculate("power.boost_output_capacitor", &[
        ("vin", 5.0), ("vout", 12.0), ("iout", 0.5), ("delta_i", 0.36),
        ("fs", 500_000.0), ("ripple_budget", 0.05), ("esr", 0.01),
    ]).unwrap();
    for (key, expected) in [
        ("duty", 0.5833333333333334), ("i_peak", 1.38),
        ("ripple_esr", 0.0138), ("ripple_cap_budget", 0.0362),
        ("c_min", 16.114180478821363e-6),
    ] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn boost_output_capacitor_independent_on_charge_and_full_path_esr_step() {
    // Half-cycle interruption of 1 A at 100 kHz removes 5 uC. The full
    // 2.5 A rectifier peak, not the 1 A ripple, causes 50 mV ESR excursion.
    let out = calculate("power.boost_output_capacitor", &[
        ("vin", 6.0), ("vout", 12.0), ("iout", 1.0), ("delta_i", 1.0),
        ("fs", 100_000.0), ("ripple_budget", 0.1), ("esr", 0.02),
    ]).unwrap();
    for (key, expected) in [
        ("duty", 0.5), ("i_peak", 2.5), ("ripple_esr", 0.05),
        ("ripple_cap_budget", 0.05), ("c_min", 100e-6),
    ] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
    assert_close(out["c_min"] * out["ripple_cap_budget"], 5e-6, 1e-6, 0.0);
    assert_close(0.5 / (100_000.0 * out["c_min"]) + out["ripple_esr"], 0.1, 1e-6, 0.0);
}

#[test]
fn boost_output_capacitor_accepts_monotonic_equality_zero_esr_and_ripple_ceiling() {
    // Mean 2 A minus half of 2 A ripple equals the 1 A load: off-time
    // charging starts at zero and remains nonnegative. Both inclusive
    // monotonic-charge and 10%-ripple endpoints must remain admissible.
    let out = calculate("power.boost_output_capacitor", &[
        ("vin", 6.0), ("vout", 12.0), ("iout", 1.0), ("delta_i", 2.0),
        ("fs", 100_000.0), ("ripple_budget", 1.2), ("esr", 0.0),
    ]).unwrap();
    assert_close(out["duty"], 0.5, 1e-6, 0.0);
    assert_close(out["i_peak"], 3.0, 1e-6, 0.0);
    assert_close(out["ripple_esr"], 0.0, 0.0, 0.0);
    assert_close(out["ripple_cap_budget"], 1.2, 1e-6, 0.0);
    assert_close(out["c_min"], 4.166666666666667e-6, 1e-6, 0.0);
}

#[test]
fn boost_output_capacitor_preserves_finite_high_ratio_charge() {
    let out = calculate("power.boost_output_capacitor", &[
        ("vin", 1.0), ("vout", 1e20), ("iout", 1e-20), ("delta_i", 0.5),
        ("fs", 100_000.0), ("ripple_budget", 0.1), ("esr", 0.01),
    ]).unwrap();
    for (key, expected) in [
        ("duty", 1.0), ("i_peak", 1.25), ("ripple_esr", 0.0125),
        ("ripple_cap_budget", 0.0875), ("c_min", 1.1428571428571428e-24),
    ] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn boost_output_capacitor_rejects_every_scalar_topology_charge_esr_and_ripple_guard() {
    let base = [("vin", 6.0), ("vout", 12.0), ("iout", 1.0), ("delta_i", 1.0),
        ("fs", 100_000.0), ("ripple_budget", 0.1), ("esr", 0.02)];
    for index in 0..base.len() {
        for value in [-1.0, f64::NAN, f64::INFINITY] {
            let mut inputs = base;
            inputs[index].1 = value;
            assert!(calculate("power.boost_output_capacitor", &inputs).is_err());
        }
        if index != 6 {
            let mut inputs = base;
            inputs[index].1 = 0.0;
            assert!(calculate("power.boost_output_capacitor", &inputs).is_err());
        }
    }
    for vout in [6.0, 5.0] {
        let mut inputs = base;
        inputs[1].1 = vout;
        assert!(calculate("power.boost_output_capacitor", &inputs).is_err());
    }
    // 2.000001 A ripple still has positive inductor valley but briefly
    // discharges the capacitor off-time: testing CCM alone would miss this.
    for delta_i in [2.000001, 4.0, 5.0] {
        let mut inputs = base;
        inputs[3].1 = delta_i;
        assert!(calculate("power.boost_output_capacitor", &inputs).is_err());
    }
    for budget in [0.0625, 0.06] {
        assert!(calculate("power.boost_output_capacitor", &[
            ("vin", 6.0), ("vout", 12.0), ("iout", 1.0), ("delta_i", 1.0),
            ("fs", 100_000.0), ("ripple_budget", budget), ("esr", 0.025),
        ]).is_err());
    }
    let mut inputs = base;
    inputs[5].1 = 1.200001;
    assert!(calculate("power.boost_output_capacitor", &inputs).is_err());
}

#[test]
fn buck_input_rms_complete_researched_example() {
    let out = calculate("power.buck_input_capacitor_rms", &[
        ("vin", 12.0), ("vout", 5.0), ("iout", 2.0), ("delta_i", 0.6),
    ]).unwrap();
    assert_close(out["duty"], 0.4166666666666667, 1e-6, 0.0);
    assert_close(out["i_cin_rms"], 0.9923317097736131, 1e-6, 0.0);
}

#[test]
fn buck_input_rms_independent_waveform_integral_and_zero_ripple_boundary() {
    // D=1/2, mean=2 A, input mean square=8+1/6 A², hence AC
    // variance=25/6 A². Zero ripple removes only the triangle variance.
    let out = calculate("power.buck_input_capacitor_rms", &[
        ("vin", 10.0), ("vout", 5.0), ("iout", 4.0), ("delta_i", 2.0),
    ]).unwrap();
    assert_close(out["duty"], 0.5, 1e-6, 0.0);
    assert_close(out["i_cin_rms"], 2.041241452319315, 1e-6, 0.0);
    for (vout, duty, rms) in [(5.0, 0.5, 2.0),
        (2.5, 0.25, 1.7320508075688772), (7.5, 0.75, 1.7320508075688772)] {
        let zero = calculate("power.buck_input_capacitor_rms", &[
            ("vin", 10.0), ("vout", vout), ("iout", 4.0), ("delta_i", 0.0),
        ]).unwrap();
        assert_close(zero["duty"], duty, 1e-6, 0.0);
        assert_close(zero["i_cin_rms"], rms, 1e-6, 0.0);
    }
}

#[test]
fn buck_input_rms_preserves_near_equal_rail_off_time() {
    // eps=2^-52, D=1/(1+eps); RMS=sqrt(eps)/(1+eps) A.
    let out = calculate("power.buck_input_capacitor_rms", &[
        ("vin", 1.0000000000000002), ("vout", 1.0), ("iout", 1.0), ("delta_i", 0.0),
    ]).unwrap();
    assert_close(out["duty"], 0.9999999999999998, 1e-6, 0.0);
    assert_close(out["i_cin_rms"], 1.4901161193847653e-8, 1e-6, 0.0);
}

#[test]
fn buck_input_rms_has_representable_output_at_tiny_and_large_current_scales() {
    // POWER-R1: normal representable RMS must not vanish merely because
    // its squared intermediate underflows, nor fail from avoidable overflow.
    // Zero-ripple normalized variance is 35/144. For delta=0.6*Iout,
    // integrated triangular variance adds 1/80, giving 23/90 exactly.
    for (iout, delta_i, coefficient) in [
        (1e-200, 0.0, 35.0_f64.sqrt() / 12.0),
        (1e200, 0.0, 35.0_f64.sqrt() / 12.0),
        (1e-200, 6e-201, (23.0_f64 / 90.0).sqrt()),
        (1e200, 6e199, (23.0_f64 / 90.0).sqrt()),
    ] {
        let out = calculate("power.buck_input_capacitor_rms", &[
            ("vin", 12.0), ("vout", 5.0), ("iout", iout), ("delta_i", delta_i),
        ]).unwrap();
        assert_close(out["duty"], 5.0 / 12.0, 1e-6, 0.0);
        assert_close(out["i_cin_rms"], coefficient * iout, 1e-6, 0.0);
    }
}

#[test]
fn buck_input_rms_rejects_every_scalar_topology_and_ccm_guard() {
    let base = [("vin", 10.0), ("vout", 5.0), ("iout", 4.0), ("delta_i", 2.0)];
    for index in 0..base.len() {
        for value in [-1.0, f64::NAN, f64::INFINITY] {
            let mut inputs = base;
            inputs[index].1 = value;
            assert!(calculate("power.buck_input_capacitor_rms", &inputs).is_err());
        }
        if index != 3 {
            let mut inputs = base;
            inputs[index].1 = 0.0;
            assert!(calculate("power.buck_input_capacitor_rms", &inputs).is_err());
        }
    }
    for vin in [5.0, 4.0] {
        let mut inputs = base;
        inputs[0].1 = vin;
        assert!(calculate("power.buck_input_capacitor_rms", &inputs).is_err());
    }
    for delta_i in [8.0, 8.1] {
        let mut inputs = base;
        inputs[3].1 = delta_i;
        assert!(calculate("power.buck_input_capacitor_rms", &inputs).is_err());
    }
}

#[test]
fn ldo_complete_researched_example() {
    let out = calculate("power.ldo_dissipation", &[
        ("vin", 5.0), ("vout", 3.3), ("iout", 0.2), ("i_gnd", 0.00005), ("v_dropout", 0.2),
    ]).unwrap();
    for (key, expected) in [("p_pass", 0.34), ("p_ground", 0.00025), ("p_diss", 0.34025)] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn ldo_independent_kcl_power_balance_and_inclusive_dropout() {
    // Exactly 1 V headroom meets exactly 1 V dropout. Ground loss uses
    // input rail 6 V, not the 5 V output or their 1 V difference.
    let out = calculate("power.ldo_dissipation", &[
        ("vin", 6.0), ("vout", 5.0), ("iout", 0.5), ("i_gnd", 0.001), ("v_dropout", 1.0),
    ]).unwrap();
    for (key, expected) in [("p_pass", 0.5), ("p_ground", 0.006), ("p_diss", 0.506)] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
    assert_close(out["p_diss"] + 2.5, 6.0 * 0.501, 1e-6, 0.0);
}

#[test]
fn ldo_zero_load_zero_ground_and_equal_rails_remain_valid() {
    for (iout, i_gnd, p_pass, p_ground, p_diss) in [
        (0.0, 0.0, 0.0, 0.0, 0.0), (0.0, 0.001, 0.0, 0.006, 0.006),
        (0.5, 0.0, 0.5, 0.0, 0.5),
    ] {
        let out = calculate("power.ldo_dissipation", &[
            ("vin", 6.0), ("vout", 5.0), ("iout", iout), ("i_gnd", i_gnd), ("v_dropout", 1.0),
        ]).unwrap();
        assert_close(out["p_pass"], p_pass, 1e-6, 0.0);
        assert_close(out["p_ground"], p_ground, 1e-6, 0.0);
        assert_close(out["p_diss"], p_diss, 1e-6, 0.0);
    }
    let equal = calculate("power.ldo_dissipation", &[
        ("vin", 3.0), ("vout", 3.0), ("iout", 0.5), ("i_gnd", 0.0), ("v_dropout", 0.0),
    ]).unwrap();
    for key in ["p_pass", "p_ground", "p_diss"] {
        assert_close(equal[key], 0.0, 0.0, 0.0);
    }
}

#[test]
fn ldo_rejects_every_scalar_and_dropout_guard() {
    let base = [("vin", 6.0), ("vout", 5.0), ("iout", 0.5), ("i_gnd", 0.001), ("v_dropout", 1.0)];
    for index in 0..base.len() {
        for value in [-1.0, f64::NAN, f64::INFINITY] {
            let mut inputs = base;
            inputs[index].1 = value;
            assert!(calculate("power.ldo_dissipation", &inputs).is_err());
        }
        if index < 2 {
            let mut inputs = base;
            inputs[index].1 = 0.0;
            assert!(calculate("power.ldo_dissipation", &inputs).is_err());
        }
    }
    for vin in [5.999999999999999, 4.0] {
        let mut inputs = base;
        inputs[0].1 = vin;
        assert!(calculate("power.ldo_dissipation", &inputs).is_err());
    }
    let mut inputs = base;
    inputs[4].1 = 1.0000000000000002;
    assert!(calculate("power.ldo_dissipation", &inputs).is_err());
}

#[test]
fn converter_budget_complete_researched_example() {
    let out = calculate("power.converter_input_budget", &[
        ("vin", 12.0), ("vout", 5.0), ("iout", 2.0), ("efficiency", 0.9),
    ]).unwrap();
    for (key, expected) in [
        ("p_out", 10.0), ("p_in", 11.11111111111111),
        ("i_in", 0.9259259259259259), ("p_loss", 1.1111111111111112),
    ] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn converter_budget_independent_step_up_power_and_unity_efficiency_boundary() {
    // Six watts output at 4/5 efficiency demands 7.5 W input; this model
    // must allow Vout>Vin without a topology constraint or extra IQ loss.
    let out = calculate("power.converter_input_budget", &[
        ("vin", 5.0), ("vout", 12.0), ("iout", 0.5), ("efficiency", 0.8),
    ]).unwrap();
    for (key, expected) in [("p_out", 6.0), ("p_in", 7.5), ("i_in", 1.5), ("p_loss", 1.5)] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
    assert_close(out["p_out"] + out["p_loss"], 7.5, 1e-6, 0.0);
    assert_close(out["p_out"] / out["p_in"], 0.8, 1e-6, 0.0);
    let unity = calculate("power.converter_input_budget", &[
        ("vin", 5.0), ("vout", 12.0), ("iout", 0.5), ("efficiency", 1.0),
    ]).unwrap();
    assert_close(unity["p_out"], 6.0, 1e-6, 0.0);
    assert_close(unity["p_in"], 6.0, 1e-6, 0.0);
    assert_close(unity["i_in"], 1.2, 1e-6, 0.0);
    assert_close(unity["p_loss"], 0.0, 0.0, 0.0);
}

#[test]
fn converter_budget_preserves_near_unity_positive_loss() {
    // eta=1-2^-53. Ten watts times the exact fractional loss ratio gives
    // ~1.11e-15 W; subtracting independently rounded Pin/Pout is unsuitable.
    let out = calculate("power.converter_input_budget", &[
        ("vin", 12.0), ("vout", 5.0), ("iout", 2.0), ("efficiency", 0.9999999999999999),
    ]).unwrap();
    for (key, expected) in [
        ("p_out", 10.0), ("p_in", 10.000000000000002),
        ("i_in", 0.8333333333333335), ("p_loss", 1.1102230246251567e-15),
    ] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn converter_budget_rejects_every_scalar_guard() {
    let base = [("vin", 5.0), ("vout", 12.0), ("iout", 0.5), ("efficiency", 0.8)];
    for index in 0..base.len() {
        for value in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            let mut inputs = base;
            inputs[index].1 = value;
            assert!(calculate("power.converter_input_budget", &inputs).is_err());
        }
    }
    for efficiency in [1.0000000000000002, 1.1] {
        let mut inputs = base;
        inputs[3].1 = efficiency;
        assert!(calculate("power.converter_input_budget", &inputs).is_err());
    }
}

#[test]
fn boost_rhp_zero_complete_researched_example() {
    let out = calculate("power.boost_rhp_zero", &[
        ("vin", 5.0), ("vout", 12.0), ("iout", 0.5), ("inductance", 22e-6), ("fs", 500_000.0),
    ]).unwrap();
    for (key, expected) in [
        ("duty", 0.5833333333333334), ("r_load", 24.0),
        ("f_rhp_zero", 30142.9816461923), ("f_crossover_guideline", 3014.29816461923),
    ] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn boost_rhp_zero_independent_hertz_conversion_and_inductance_scaling() {
    // At D=1/2 and R=12 ohm, numerator is 3 ohm: fz=15000/pi Hz
    // for L=100 uH. Doubling L halves the zero without changing duty/load.
    for (inductance, frequency, guideline) in [
        (100e-6, 4774.64829275686, 477.464829275686),
        (200e-6, 2387.32414637843, 238.732414637843),
    ] {
        let out = calculate("power.boost_rhp_zero", &[
            ("vin", 6.0), ("vout", 12.0), ("iout", 1.0), ("inductance", inductance), ("fs", 100_000.0),
        ]).unwrap();
        assert_close(out["duty"], 0.5, 1e-6, 0.0);
        assert_close(out["r_load"], 12.0, 1e-6, 0.0);
        assert_close(out["f_rhp_zero"], frequency, 1e-6, 0.0);
        assert_close(out["f_crossover_guideline"], guideline, 1e-6, 0.0);
    }
}

#[test]
fn boost_rhp_zero_accepts_positive_valley_neighbor_and_high_ratio() {
    // With Vin=1,Vout=2,L=1,fs=1/2, half-ripple is exactly 1/2 A.
    // The load neighbor above 1/4 A gives positive valley 2^-53 A.
    let edge = calculate("power.boost_rhp_zero", &[
        ("vin", 1.0), ("vout", 2.0), ("iout", 0.25000000000000006), ("inductance", 1.0), ("fs", 0.5),
    ]).unwrap();
    assert_close(edge["duty"], 0.5, 1e-6, 0.0);
    assert_close(edge["r_load"], 7.999999999999998, 1e-6, 0.0);
    assert_close(edge["f_rhp_zero"], 0.3183098861837906, 1e-6, 0.0);
    assert_close(edge["f_crossover_guideline"], 0.03183098861837906, 1e-6, 0.0);

    // R=1e40 ohm and off-fraction=1e-20 yield R*(off)^2=1 ohm,
    // even though displayed D rounds to one. This is not a zero-frequency
    // plant or an infinite-current divider by reconstructed 1-D.
    let high = calculate("power.boost_rhp_zero", &[
        ("vin", 1.0), ("vout", 1e20), ("iout", 1e-20), ("inductance", 1.0), ("fs", 1.0),
    ]).unwrap();
    for (key, expected) in [
        ("duty", 1.0), ("r_load", 1e40), ("f_rhp_zero", 0.15915494309189535),
        ("f_crossover_guideline", 0.015915494309189534),
    ] {
        assert_close(high[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn boost_rhp_zero_rejects_every_scalar_topology_and_ccm_guard() {
    let base = [("vin", 6.0), ("vout", 12.0), ("iout", 1.0), ("inductance", 100e-6), ("fs", 100_000.0)];
    for index in 0..base.len() {
        for value in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            let mut inputs = base;
            inputs[index].1 = value;
            assert!(calculate("power.boost_rhp_zero", &inputs).is_err());
        }
    }
    for vout in [6.0, 5.0] {
        let mut inputs = base;
        inputs[1].1 = vout;
        assert!(calculate("power.boost_rhp_zero", &inputs).is_err());
    }
    // Exact zero-valley equality and strictly negative valley both reject.
    for iout in [0.25, 0.2] {
        assert!(calculate("power.boost_rhp_zero", &[
            ("vin", 1.0), ("vout", 2.0), ("iout", iout), ("inductance", 1.0), ("fs", 0.5),
        ]).is_err());
    }
    // Holding the other values fixed, lower switching frequency or lower L
    // can independently leave CCM: fs is a domain input, not ignored metadata.
    for (inductance, fs) in [(1e-8, 100_000.0), (100e-6, 1.0)] {
        assert!(calculate("power.boost_rhp_zero", &[
            ("vin", 6.0), ("vout", 12.0), ("iout", 1.0), ("inductance", inductance), ("fs", fs),
        ]).is_err());
    }
}
