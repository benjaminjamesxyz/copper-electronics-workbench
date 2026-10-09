mod common;

use common::{assert_close, calculate};
use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2, FRAC_PI_4, PI};

// Nonzero research examples use their declared 1e-6 relative tolerance, with
// no absolute floor. DC identities are algebraically exact (absolute = 0).
// Independently rounded notch centers alone allow a cancellation-scale residual.
#[test]
fn all_eight_independent_research_examples_check_every_output() {
    let examples: &[(&str, &[(&str, f64)], &[(&str, f64)])] = &[
        (
            "filters.rc_lowpass_response",
            &[("r", 10000.0), ("c", 1e-8), ("f", 1000.0)],
            &[("cutoff_hz", 1591.5494309189535), ("frequency_ratio", 0.6283185307179586),
              ("voltage_gain", 0.8467330159648303), ("phase_rad", -0.5609821161086238),
              ("attenuation_db", 1.4450701162052881)],
        ),
        (
            "filters.rc_highpass_response",
            &[("r", 10000.0), ("c", 1e-8), ("f", 1000.0)],
            &[("cutoff_hz", 1591.5494309189535), ("frequency_ratio", 0.6283185307179586),
              ("voltage_gain", 0.532018044501408), ("phase_rad", 1.0098142106862729),
              ("attenuation_db", 5.481472749042987)],
        ),
        (
            "filters.second_order_highpass",
            &[("pole_frequency", 1000.0), ("q", 0.7071067811865476),
              ("passband_gain", 2.0), ("f", 500.0)],
            &[("frequency_ratio", 0.5), ("voltage_gain", 0.48507125007266594),
              ("gain_db", -6.283889300503116), ("half_power_hz", 1000.0)],
        ),
        (
            "filters.deliyannis_bandpass_design",
            &[("center_frequency", 1000.0), ("q", 2.0), ("c0", 1e-8), ("f", 1000.0)],
            &[("r0", 15915.494309189537), ("r3", 47746.482927568606),
              ("r2", 5305.164769729846), ("peak_gain", 2.0), ("bandwidth_hz", 500.0),
              ("lower_edge_hz", 780.7764064044152), ("upper_edge_hz", 1280.7764064044152),
              ("voltage_gain", 2.0)],
        ),
        (
            "filters.butterworth_spec_synthesis",
            &[("passband_edge", 1000.0), ("stopband_edge", 4000.0),
              ("passband_loss", 1.0), ("stopband_loss", 40.0)],
            &[("order_bound", 3.8092399086702753), ("order", 4.0),
              ("cutoff_min_hz", 1184.003988964071), ("cutoff_max_hz", 1264.9268763451062),
              ("actual_passband_db", 1.0), ("actual_stopband_db", 42.29680198990891)],
        ),
        (
            "filters.sallen_key_lowpass",
            &[("r1", 10000.0), ("r2", 10000.0), ("c1", 1e-8), ("c2", 1e-8),
              ("amplifier_gain", 2.0), ("f", 1000.0)],
            &[("dc_gain", 2.0), ("pole_hz", 1591.5494309189535), ("section_q", 1.0),
              ("frequency_ratio", 0.6283185307179586), ("voltage_gain", 2.2925435192800947),
              ("gain_db", 7.206351772370593), ("half_power_hz", 2024.482149301843)],
        ),
        (
            "filters.fliege_notch_response",
            &[("r0", 10000.0), ("c0", 1e-7), ("rq", 200000.0), ("f", 160.0)],
            &[("notch_hz", 159.15494309189535), ("section_q", 10.0),
              ("frequency_ratio", 1.0053096491487337), ("voltage_gain", 0.10532346435527205),
              ("bandwidth_hz", 15.915494309189535), ("lower_edge_hz", 151.3960154315485),
              ("upper_edge_hz", 167.31150974073802)],
        ),
        (
            "filters.buffered_rc_cascade",
            &[("r", 10000.0), ("c", 1e-8), ("stage_count", 3.0), ("f", 1000.0)],
            &[("stage_cutoff_hz", 1591.5494309189535), ("overall_order", 3.0),
              ("cascade_cutoff_hz", 811.4109382567457), ("frequency_ratio", 0.6283185307179586),
              ("voltage_gain", 0.6070709938555954), ("phase_rad", -1.6829463483258713),
              ("attenuation_db", 4.3352103486158615)],
        ),
    ];
    for &(id, inputs, expected) in examples {
        let output = calculate(id, inputs).unwrap_or_else(|error| panic!("{id}: {error}"));
        for &(key, value) in expected {
            assert_close(output[key], value, 1e-6, 0.0);
        }
    }
}

#[test]
fn rc_corners_distinguish_output_nodes_and_preserve_rc_scaling() {
    // R is divided by five and C multiplied by five: the entire transfer
    // function is unchanged. At omega*RC=1 the two output nodes have opposite phase.
    for (id, phase) in [("filters.rc_lowpass_response", -FRAC_PI_4),
                        ("filters.rc_highpass_response", FRAC_PI_4)] {
        let output = calculate(id, &[("r", 2000.0), ("c", 5e-8),
                                     ("f", 1591.5494309189535)]).unwrap();
        for (key, expected) in [("cutoff_hz", 1591.5494309189535),
                                ("frequency_ratio", 1.0), ("voltage_gain", FRAC_1_SQRT_2),
                                ("phase_rad", phase), ("attenuation_db", 3.010299956639812)] {
            assert_close(output[key], expected, 1e-12, 0.0);
        }
    }
}

#[test]
fn rc_lowpass_dc_at_smallest_components_and_tiny_positive_loss() {
    let dc = calculate("filters.rc_lowpass_response",
        &[("r", 0.001), ("c", 1e-15), ("f", 0.0)]).unwrap();
    assert_close(dc["cutoff_hz"], 1.5915494309189533e17, 1e-12, 0.0);
    for (key, expected) in [("frequency_ratio", 0.0), ("voltage_gain", 1.0),
                            ("phase_rad", 0.0), ("attenuation_db", 0.0)] {
        assert_close(dc[key], expected, 0.0, 0.0);
    }
    // The complex divider has phi=-x+O(x^3) and loss=(10/ln10)x^2+O(x^4).
    // x=1e-12 makes the omitted relative corrections <1e-24, not a zero loss.
    let tiny = calculate("filters.rc_lowpass_response",
        &[("r", 1.0), ("c", 1e-6), ("f", 1.5915494309189535e-7)]).unwrap();
    for (key, expected) in [("cutoff_hz", 159154.94309189535),
                            ("frequency_ratio", 1e-12), ("voltage_gain", 1.0),
                            ("phase_rad", -1e-12), ("attenuation_db", 4.342944819032518e-24)] {
        assert_close(tiny[key], expected, 1e-12, 0.0);
    }
}

#[test]
fn rc_highpass_minimum_probe_and_high_frequency_do_not_erase_phase_or_loss() {
    let low = calculate("filters.rc_highpass_response",
        &[("r", 10000.0), ("c", 1e-8), ("f", 1e-12)]).unwrap();
    let x = 2.0 * PI * 1e-16;
    for (key, expected) in [("cutoff_hz", 1591.5494309189535),
                            ("frequency_ratio", x), ("voltage_gain", x),
                            ("phase_rad", FRAC_PI_2), ("attenuation_db", -20.0 * x.log10())] {
        // Small-x asymptotic corrections are below 1e-30 relative.
        assert_close(low[key], expected, 1e-12, 0.0);
    }
    let high = calculate("filters.rc_highpass_response",
        &[("r", 1.0), ("c", 1.0), ("f", 1e12)]).unwrap();
    let inverse_x = 1.0 / (2.0 * PI * 1e12);
    for (key, expected) in [("cutoff_hz", 1.0 / (2.0 * PI)),
                            ("frequency_ratio", 2.0 * PI * 1e12), ("voltage_gain", 1.0),
                            ("phase_rad", inverse_x),
                            ("attenuation_db", 10.0 / 10.0_f64.ln() * inverse_x * inverse_x)] {
        // Lead~1/x and loss~(10/ln10)/x^2; corrections <3e-26 relative.
        assert_close(high[key], expected, 1e-12, 0.0);
    }
    assert!(calculate("filters.rc_highpass_response",
        &[("r", 10000.0), ("c", 1e-8), ("f", 0.0)]).is_err());
}

#[test]
fn second_order_highpass_q_one_peak_and_true_half_power_are_distinct() {
    let pole = calculate("filters.second_order_highpass",
        &[("pole_frequency", 1000.0), ("q", 1.0), ("passband_gain", 3.0), ("f", 1000.0)]).unwrap();
    // At the pole, the real denominator vanishes and |H|=K*Q.
    // Solving t^2+t-1=0 gives the independently computed cutoff below f0.
    for (key, expected) in [("frequency_ratio", 1.0), ("voltage_gain", 3.0),
                            ("gain_db", 9.542425094393248),
                            ("half_power_hz", 786.1513777574233)] {
        assert_close(pole[key], expected, 1e-12, 0.0);
    }
    let half = calculate("filters.second_order_highpass",
        &[("pole_frequency", 1000.0), ("q", 1.0), ("passband_gain", 3.0),
          ("f", 786.1513777574233)]).unwrap();
    for (key, expected) in [("frequency_ratio", 0.7861513777574233),
                            ("voltage_gain", 3.0 * FRAC_1_SQRT_2),
                            ("gain_db", 6.532125137753436),
                            ("half_power_hz", 786.1513777574233)] {
        assert_close(half[key], expected, 1e-12, 0.0);
    }
}

#[test]
fn second_order_highpass_inclusive_q_limits_and_minimum_nonzero_probe() {
    // Roots below were independently evaluated at 75 decimal digits, not
    // obtained from the engine or from a previously returned cutoff.
    for (q, gain_db, cutoff) in [(0.05, -26.020599913279625, 19950.00031445996),
                                (50.0, 33.979400086720375, 643.6397657763485)] {
        let output = calculate("filters.second_order_highpass",
            &[("pole_frequency", 1000.0), ("q", q), ("passband_gain", 1.0), ("f", 1000.0)]).unwrap();
        for (key, expected) in [("frequency_ratio", 1.0), ("voltage_gain", q),
                                ("gain_db", gain_db), ("half_power_hz", cutoff)] {
            assert_close(output[key], expected, 1e-10, 0.0);
        }
    }
    let tiny = calculate("filters.second_order_highpass",
        &[("pole_frequency", 1e12), ("q", 1.0), ("passband_gain", 1.0), ("f", 1e-12)]).unwrap();
    for (key, expected) in [("frequency_ratio", 1e-24), ("voltage_gain", 1e-48),
                            ("gain_db", -960.0), ("half_power_hz", 7.861513777574233e11)] {
        assert_close(tiny[key], expected, 1e-12, 0.0);
    }
    assert!(calculate("filters.second_order_highpass",
        &[("pole_frequency", 1000.0), ("q", 1.0), ("passband_gain", 1.0), ("f", 0.0)]).is_err());
}

#[test]
fn deliyannis_independent_edges_dc_and_resistor_mapping() {
    for (f, gain) in [(0.0, 0.0), (780.7764064044152, 2.0 * FRAC_1_SQRT_2),
                      (1280.7764064044152, 2.0 * FRAC_1_SQRT_2)] {
        let output = calculate("filters.deliyannis_bandpass_design",
            &[("center_frequency", 1000.0), ("q", 2.0), ("c0", 1e-8), ("f", f)]).unwrap();
        for (key, expected) in [("r0", 15915.494309189537), ("r3", 47746.482927568606),
                                ("r2", 5305.164769729846), ("peak_gain", 2.0),
                                ("bandwidth_hz", 500.0), ("lower_edge_hz", 780.7764064044152),
                                ("upper_edge_hz", 1280.7764064044152), ("voltage_gain", gain)] {
            assert_close(output[key], expected, 1e-12, 0.0);
        }
        assert_close(output["r2"] * output["r3"], 253302959.10584444, 1e-12, 0.0);
        assert_close(output["upper_edge_hz"] - output["lower_edge_hz"], 500.0, 1e-12, 0.0);
        assert_close(output["upper_edge_hz"] * output["lower_edge_hz"], 1e6, 1e-12, 0.0);
    }
}

#[test]
fn deliyannis_exclusive_q_half_and_inclusive_q_fifty() {
    // Next binary64 value above 0.5 gives 2Q-1=2^-52: small R3 and large R2
    // are meaningful, finite component solutions, not grounds for a clamp.
    let q = 0.5000000000000001;
    let near = calculate("filters.deliyannis_bandpass_design",
        &[("center_frequency", 1000.0), ("q", q), ("c0", 1e-8), ("f", 1000.0)]).unwrap();
    let r0 = 15915.494309189537;
    let spacing = 2.0_f64.powi(-52);
    let lower = (2.0_f64.sqrt() - 1.0) * 1000.0;
    let upper = (2.0_f64.sqrt() + 1.0) * 1000.0;
    for (key, expected) in [("r0", r0), ("r3", r0 * spacing), ("r2", r0 / spacing),
                            ("peak_gain", q), ("bandwidth_hz", 2000.0),
                            ("lower_edge_hz", lower), ("upper_edge_hz", upper), ("voltage_gain", q)] {
        assert_close(near[key], expected, 1e-12, 0.0);
    }
    let high = calculate("filters.deliyannis_bandpass_design",
        &[("center_frequency", 1000.0), ("q", 50.0), ("c0", 1e-8), ("f", 1000.0)]).unwrap();
    let low_edge = 10.0 * (10001.0_f64.sqrt() - 1.0);
    let high_edge = 10.0 * (10001.0_f64.sqrt() + 1.0);
    for (key, expected) in [("r0", r0), ("r3", 99.0 * r0), ("r2", r0 / 99.0),
                            ("peak_gain", 50.0), ("bandwidth_hz", 20.0),
                            ("lower_edge_hz", low_edge), ("upper_edge_hz", high_edge), ("voltage_gain", 50.0)] {
        assert_close(high[key], expected, 1e-12, 0.0);
    }
    assert!(calculate("filters.deliyannis_bandpass_design",
        &[("center_frequency", 1000.0), ("q", 0.5), ("c0", 1e-8), ("f", 1000.0)]).is_err());
}

#[test]
fn butterworth_minimality_and_feasible_cutoff_interval() {
    let output = calculate("filters.butterworth_spec_synthesis",
        &[("passband_edge", 1000.0), ("stopband_edge", 4000.0),
          ("passband_loss", 1.0), ("stopband_loss", 40.0)]).unwrap();
    assert!(output["cutoff_min_hz"] <= output["cutoff_max_hz"]);
    assert_close(output["actual_passband_db"], 1.0, 1e-12, 0.0);
    assert!(output["actual_stopband_db"] >= 40.0);
    // Any N=3 cutoff satisfying the passband must exceed this lower limit,
    // but every N=3 cutoff satisfying the stopband is below this upper limit.
    let pass_factor = 10.0_f64.powf(0.1) - 1.0;
    let third_order_min = 1000.0 / pass_factor.powf(1.0 / 6.0);
    let third_order_max = 4000.0 / 9999.0_f64.powf(1.0 / 6.0);
    assert!(third_order_min > third_order_max);
    assert_close(output["order"], 4.0, 0.0, 0.0);
}

#[test]
fn butterworth_nearest_integer_boundary_and_both_sides() {
    // With Ap=10 dB, fp=1, fs=2, an exact N=2 limit is As=10log10(145).
    // That real value is 21.613680022349748921...; nearest binary64 As below
    // it has N*=1.9999999999999997438079... and MUST still choose two poles.
    // The independent 75-digit oracles explicitly respect the actual f64 inputs.
    for (stop_loss, bound, order, cutoff_max, stop_actual) in [
        (21.613680022349747, 1.9999999999999998, 2.0, 0.5773502691896258, 21.61368002234975),
        (21.613680012349747, 1.999999998327501, 2.0, 0.5773502695242834, 21.61368002234975),
        (21.613680032349748, 2.0000000016724984, 3.0, 0.8735804643987219, 27.611758131557314),
    ] {
        let output = calculate("filters.butterworth_spec_synthesis",
            &[("passband_edge", 1.0), ("stopband_edge", 2.0),
              ("passband_loss", 10.0), ("stopband_loss", stop_loss)]).unwrap();
        let cutoff_min = if order == 2.0 { 0.5773502691896258 } else { 0.6933612743506347 };
        for (key, expected) in [("order_bound", bound), ("order", order),
                                ("cutoff_min_hz", cutoff_min), ("cutoff_max_hz", cutoff_max),
                                ("actual_passband_db", 10.0), ("actual_stopband_db", stop_actual)] {
            assert_close(output[key], expected, 1e-12, 0.0);
        }
        assert_close(output["order"], order, 0.0, 0.0);
    }
}

#[test]
fn butterworth_close_positive_losses_minimum_edge_ratio_and_order_cap() {
    // Adjacent positive loss values must not round their ratio to one and
    // produce zero order. Decimal derivations use exact binary64 input values.
    for (fp, fs, ap, ass, bound, order, cutoff_min, cutoff_max, actual_stop) in [
        (1000.0, 4000.0, 1.0, 1.0000000000000002,
         8.965939131579341e-17, 1.0, 1965.226728360272, 7860.906913441087, 7.112001916287117),
        (1.0, 1.001, 0.001, 0.001001,
         0.5000575956161906, 1.0, 65.8972293668006, 65.93016595305507, 0.0010020007691831707),
        (1.0, 2.0, 1.0, 185.0,
         31.70253064363295, 32.0, 1.021337188852464, 1.0279393601543026, 186.7909439811468),
    ] {
        let output = calculate("filters.butterworth_spec_synthesis",
            &[("passband_edge", fp), ("stopband_edge", fs),
              ("passband_loss", ap), ("stopband_loss", ass)]).unwrap();
        for (key, expected) in [("order_bound", bound), ("order", order),
                                ("cutoff_min_hz", cutoff_min), ("cutoff_max_hz", cutoff_max),
                                ("actual_passband_db", ap), ("actual_stopband_db", actual_stop)] {
            assert_close(output[key], expected, 1e-10, 0.0);
        }
        assert_close(output["order"], order, 0.0, 0.0);
        assert!(output["cutoff_min_hz"] <= output["cutoff_max_hz"]);
    }
    for (fp, fs, ap, ass) in [(1000.0, 1000.0, 1.0, 40.0),
                             (1000.0, 999.0, 1.0, 40.0),
                             (1000.0, 1000.5, 1.0, 1.00001),
                             (1000.0, 4000.0, 1.0, 1.0),
                             (1000.0, 4000.0, 2.0, 1.0),
                             (1.0, 2.0, 1.0, 190.0)] {
        assert!(calculate("filters.butterworth_spec_synthesis",
            &[("passband_edge", fp), ("stopband_edge", fs),
              ("passband_loss", ap), ("stopband_loss", ass)]).is_err());
    }
}

#[test]
fn sallen_key_unity_butterworth_requires_correct_capacitor_labels() {
    // Equal R, grounded C1=10 nF and feedback C2=20 nF give Q=1/sqrt(2)
    // at K=1. Reversing C labels produces another Q, so this is topology-sensitive.
    let output = calculate("filters.sallen_key_lowpass",
        &[("r1", 10000.0), ("r2", 10000.0), ("c1", 1e-8), ("c2", 2e-8),
          ("amplifier_gain", 1.0), ("f", 1125.3953951963827)]).unwrap();
    for (key, expected) in [("dc_gain", 1.0), ("pole_hz", 1125.3953951963827),
                            ("section_q", FRAC_1_SQRT_2), ("frequency_ratio", 1.0),
                            ("voltage_gain", FRAC_1_SQRT_2), ("gain_db", -3.010299956639812),
                            ("half_power_hz", 1125.3953951963827)] {
        assert_close(output[key], expected, 1e-12, 0.0);
    }
    let dc = calculate("filters.sallen_key_lowpass",
        &[("r1", 1.0), ("r2", 1.0), ("c1", 0.125), ("c2", 0.125),
          ("amplifier_gain", 1.0), ("f", 0.0)]).unwrap();
    for (key, expected) in [("dc_gain", 1.0), ("pole_hz", 4.0 / PI), ("section_q", 0.5),
                            ("frequency_ratio", 0.0), ("voltage_gain", 1.0), ("gain_db", 0.0),
                            ("half_power_hz", 4.0 / PI * (2.0_f64.sqrt() - 1.0).sqrt())] {
        assert_close(dc[key], expected, 1e-12, 0.0);
    }
}

#[test]
fn sallen_key_q_endpoints_and_all_damping_guards() {
    // K=1, C1=1, C2=0.01, equal R: sqrt(P)=0.1 s and D=2 s, hence Q=0.05.
    // Equal R,C with K=2.98 gives Q=50 (within binary64 rounding).
    for (c1, c2, k, q, pole, cutoff_factor) in [
        (1.0, 0.01, 1.0, 0.05, 5.0 / PI, 0.05012531249311259),
        (0.125, 0.125, 2.98, 50.0, 4.0 / PI, 1.5536641040098187),
    ] {
        let output = calculate("filters.sallen_key_lowpass",
            &[("r1", 1.0), ("r2", 1.0), ("c1", c1), ("c2", c2),
              ("amplifier_gain", k), ("f", 0.0)]).unwrap();
        for (key, expected) in [("dc_gain", k), ("pole_hz", pole), ("section_q", q),
                                ("frequency_ratio", 0.0), ("voltage_gain", k),
                                ("gain_db", 20.0 * k.log10()), ("half_power_hz", pole * cutoff_factor)] {
            assert_close(output[key], expected, 1e-10, 0.0);
        }
    }
    for (c1, c2, gain) in [(1.0, 0.009, 1.0), (0.125, 0.125, 2.99),
                          (0.125, 0.125, 3.0), (0.125, 0.125, 4.0)] {
        assert!(calculate("filters.sallen_key_lowpass",
            &[("r1", 1.0), ("r2", 1.0), ("c1", c1), ("c2", c2),
              ("amplifier_gain", gain), ("f", 0.0)]).is_err());
    }
}

#[test]
fn sallen_key_non_butterworth_half_power_is_relative_to_dc_not_peak() {
    let output = calculate("filters.sallen_key_lowpass",
        &[("r1", 10000.0), ("r2", 10000.0), ("c1", 1e-8), ("c2", 1e-8),
          ("amplifier_gain", 2.0), ("f", 2024.482149301843)]).unwrap();
    for (key, expected) in [("dc_gain", 2.0), ("pole_hz", 1591.5494309189535),
                            ("section_q", 1.0), ("frequency_ratio", 1.272019649514069),
                            ("voltage_gain", 2.0 * FRAC_1_SQRT_2), ("gain_db", 3.010299956639812),
                            ("half_power_hz", 2024.482149301843)] {
        assert_close(output[key], expected, 1e-12, 0.0);
    }
}

#[test]
fn sallen_key_asymmetric_resistor_order_controls_feedback_damping() {
    // P is unchanged by exchanging the resistors, but D weights R1 in the
    // gain-dependent feedback term. With C1=C2=1/8 F and K=2, these two
    // circuits have Q=0.5 and Q=2, respectively; this is not a symmetric swap.
    for (r1, r2, q, magnitude, gain_db, cutoff_factor) in [
        (1.0, 4.0, 0.5, 1.0, 0.0, (2.0_f64.sqrt() - 1.0).sqrt()),
        (4.0, 1.0, 2.0, 4.0, 12.041199826559248,
         ((7.0 + 113.0_f64.sqrt()) / 8.0).sqrt()),
    ] {
        let output = calculate("filters.sallen_key_lowpass",
            &[("r1", r1), ("r2", r2), ("c1", 0.125), ("c2", 0.125),
              ("amplifier_gain", 2.0), ("f", 2.0 / PI)]).unwrap();
        for (key, expected) in [("dc_gain", 2.0), ("pole_hz", 2.0 / PI), ("section_q", q),
                                ("frequency_ratio", 1.0), ("voltage_gain", magnitude),
                                ("gain_db", gain_db), ("half_power_hz", 2.0 / PI * cutoff_factor)] {
            assert_close(output[key], expected, 1e-12, 0.0);
        }
    }
}

#[test]
fn fliege_near_notch_retains_small_positive_transmission() {
    // x-1=1e-8: first-order expansion of the complex numerator gives
    // |H|=2Q*|x-1| to relative O(1e-8), independent of exact-center cancellation.
    let output = calculate("filters.fliege_notch_response",
        &[("r0", 10000.0), ("c0", 1e-7), ("rq", 200000.0),
          ("f", 159.1549446834448)]).unwrap();
    for (key, expected) in [("notch_hz", 159.15494309189535), ("section_q", 10.0),
                            ("frequency_ratio", 1.00000001), ("bandwidth_hz", 15.915494309189535),
                            ("lower_edge_hz", 151.3960154315485), ("upper_edge_hz", 167.31150974073802)] {
        assert_close(output[key], expected, 1e-12, 0.0);
    }
    assert_close(output["voltage_gain"], 2e-7, 1e-6, 0.0);
}

#[test]
fn fliege_dc_independent_notch_zero_and_exact_half_power_edges() {
    for (f, ratio, gain) in [(0.0, 0.0, 1.0),
                            (159.15494309189535, 1.0, 0.0),
                            (151.3960154315485, 0.9512492197250393, FRAC_1_SQRT_2),
                            (167.31150974073802, 1.0512492197250394, FRAC_1_SQRT_2)] {
        let output = calculate("filters.fliege_notch_response",
            &[("r0", 10000.0), ("c0", 1e-7), ("rq", 200000.0), ("f", f)]).unwrap();
        for (key, expected) in [("notch_hz", 159.15494309189535), ("section_q", 10.0),
                                ("frequency_ratio", ratio), ("bandwidth_hz", 15.915494309189535),
                                ("lower_edge_hz", 151.3960154315485), ("upper_edge_hz", 167.31150974073802)] {
            assert_close(output[key], expected, 1e-12, 0.0);
        }
        // A rounded independently supplied f0 may differ by a few ulps.
        // Near x=1, |H| approximately 2Q*|delta x|: 64*Q*epsilon permits
        // only roundoff-scale cancellation, never a fabricated finite floor.
        let zero_tolerance = if gain == 0.0 { 64.0 * 10.0 * f64::EPSILON } else { 0.0 };
        assert_close(output["voltage_gain"], gain, 1e-12, zero_tolerance);
        assert_close(output["upper_edge_hz"] * output["lower_edge_hz"],
                     (1000.0 / (2.0 * PI)).powi(2), 1e-12, 0.0);
        assert_close(output["upper_edge_hz"] - output["lower_edge_hz"],
                     100.0 / (2.0 * PI), 1e-12, 0.0);
        // Returned center reuse is an identity input, NOT an expected oracle:
        // frequency/itself=1 and the notch numerator must then be exactly zero.
        let center = calculate("filters.fliege_notch_response",
            &[("r0", 10000.0), ("c0", 1e-7), ("rq", 200000.0),
              ("f", output["notch_hz"])]).unwrap();
        assert_close(center["frequency_ratio"], 1.0, 0.0, 0.0);
        assert_close(center["voltage_gain"], 0.0, 0.0, 0.0);
    }
}

#[test]
fn fliege_reciprocal_frequency_symmetry_and_q_endpoints() {
    for (f, x) in [(79.57747154594767, 0.5), (318.3098861837907, 2.0)] {
        let output = calculate("filters.fliege_notch_response",
            &[("r0", 10000.0), ("c0", 1e-7), ("rq", 40000.0), ("f", f)]).unwrap();
        for (key, expected) in [("notch_hz", 159.15494309189535), ("section_q", 2.0),
                                ("frequency_ratio", x), ("voltage_gain", 0.9486832980505138),
                                ("bandwidth_hz", 79.57747154594767),
                                ("lower_edge_hz", 159.15494309189535 * (17.0_f64.sqrt() - 1.0) / 4.0),
                                ("upper_edge_hz", 159.15494309189535 * (17.0_f64.sqrt() + 1.0) / 4.0)] {
            assert_close(output[key], expected, 1e-12, 0.0);
        }
    }
    for (rq, q) in [(1000.0, 0.05), (1000000.0, 50.0)] {
        let output = calculate("filters.fliege_notch_response",
            &[("r0", 10000.0), ("c0", 1e-7), ("rq", rq), ("f", 0.0)]).unwrap();
        let root = (1.0_f64 + 4.0 * q * q).sqrt();
        let lower = 159.15494309189535 * (root - 1.0) / (2.0 * q);
        let upper = 159.15494309189535 * (root + 1.0) / (2.0 * q);
        for (key, expected) in [("notch_hz", 159.15494309189535), ("section_q", q),
                                ("frequency_ratio", 0.0), ("voltage_gain", 1.0),
                                ("bandwidth_hz", 159.15494309189535 / q),
                                ("lower_edge_hz", lower), ("upper_edge_hz", upper)] {
            assert_close(output[key], expected, 1e-10, 0.0);
        }
    }
    for rq in [999.0, 1000001.0] {
        assert!(calculate("filters.fliege_notch_response",
            &[("r0", 10000.0), ("c0", 1e-7), ("rq", rq), ("f", 0.0)]).is_err());
    }
}

#[test]
fn cascade_total_cutoff_and_inclusive_discrete_orders() {
    for n in [2.0_f64, 3.0, 32.0] {
        // Solve the power equation (1+x^2)^N=2 using the input order.
        let x = (2.0_f64.powf(1.0 / n) - 1.0).sqrt();
        let cutoff = 1591.5494309189535 * x;
        let output = calculate("filters.buffered_rc_cascade",
            &[("r", 10000.0), ("c", 1e-8), ("stage_count", n), ("f", cutoff)]).unwrap();
        for (key, expected) in [("stage_cutoff_hz", 1591.5494309189535), ("overall_order", n),
                                ("cascade_cutoff_hz", cutoff), ("frequency_ratio", x),
                                ("voltage_gain", FRAC_1_SQRT_2), ("phase_rad", -n * x.atan()),
                                ("attenuation_db", 3.010299956639812)] {
            assert_close(output[key], expected, 1e-12, 0.0);
        }
    }
    for n in [1.0, 2.5, 31.999999999999996, 33.0] {
        assert!(calculate("filters.buffered_rc_cascade",
            &[("r", 10000.0), ("c", 1e-8), ("stage_count", n), ("f", 1000.0)]).is_err());
    }
}

#[test]
fn cascade_dc_unwrapped_phase_tiny_loss_and_underflowed_linear_gain() {
    let dc = calculate("filters.buffered_rc_cascade",
        &[("r", 10000.0), ("c", 1e-8), ("stage_count", 32.0), ("f", 0.0)]).unwrap();
    let cutoff_factor = (2.0_f64.powf(1.0 / 32.0) - 1.0).sqrt();
    for (key, expected) in [("stage_cutoff_hz", 1591.5494309189535), ("overall_order", 32.0),
                            ("cascade_cutoff_hz", 1591.5494309189535 * cutoff_factor),
                            ("frequency_ratio", 0.0), ("voltage_gain", 1.0),
                            ("phase_rad", 0.0), ("attenuation_db", 0.0)] {
        assert_close(dc[key], expected, 1e-12, 0.0);
    }
    let pole = calculate("filters.buffered_rc_cascade",
        &[("r", 10000.0), ("c", 1e-8), ("stage_count", 8.0), ("f", 1591.5494309189535)]).unwrap();
    for (key, expected) in [("stage_cutoff_hz", 1591.5494309189535), ("overall_order", 8.0),
                            ("cascade_cutoff_hz", 1591.5494309189535 * (2.0_f64.powf(0.125) - 1.0).sqrt()),
                            ("frequency_ratio", 1.0), ("voltage_gain", 0.0625),
                            ("phase_rad", -2.0 * PI), ("attenuation_db", 24.082399653118497)] {
        assert_close(pole[key], expected, 1e-12, 0.0);
    }
    let tiny = calculate("filters.buffered_rc_cascade",
        &[("r", 1.0), ("c", 1e-6), ("stage_count", 32.0), ("f", 1.5915494309189535e-7)]).unwrap();
    for (key, expected) in [("stage_cutoff_hz", 159154.94309189535), ("overall_order", 32.0),
                            ("cascade_cutoff_hz", 159154.94309189535 * cutoff_factor),
                            ("frequency_ratio", 1e-12), ("voltage_gain", 1.0),
                            ("phase_rad", -32e-12), ("attenuation_db", 32.0 * 4.342944819032518e-24)] {
        assert_close(tiny[key], expected, 1e-12, 0.0);
    }
    let extreme = calculate("filters.buffered_rc_cascade",
        &[("r", 1e9), ("c", 1.0), ("stage_count", 32.0), ("f", 1e12)]).unwrap();
    // x=2pi*1e21; x^-32 is far below the smallest binary64 subnormal.
    // This legitimate rounded zero must not make the independently computed
    // positive insertion loss nonfinite. Phase remains unwrapped at -16pi.
    for (key, expected) in [("stage_cutoff_hz", 1.0 / (2.0 * PI * 1e9)), ("overall_order", 32.0),
                            ("cascade_cutoff_hz", cutoff_factor / (2.0 * PI * 1e9)),
                            ("frequency_ratio", 2.0 * PI * 1e21), ("voltage_gain", 0.0),
                            ("phase_rad", -16.0 * PI),
                            ("attenuation_db", 640.0 * (2.0 * PI * 1e21).log10())] {
        assert_close(extreme[key], expected, 1e-12, 0.0);
    }
}

#[test]
fn every_scalar_envelope_rejects_below_above_and_nonfinite_inputs() {
    // Bounds are independently transcribed from the SI/model-domain contracts.
    // Each valid vector is explicit; changing one field cannot rely on defaults.
    let cases: &[(&str, &[(&str, f64)], &[(f64, f64)])] = &[
        ("filters.rc_lowpass_response", &[("r", 10000.0), ("c", 1e-8), ("f", 1000.0)],
         &[(0.001, 1e9), (1e-15, 1.0), (0.0, 1e12)]),
        ("filters.rc_highpass_response", &[("r", 10000.0), ("c", 1e-8), ("f", 1000.0)],
         &[(0.001, 1e9), (1e-15, 1.0), (1e-12, 1e12)]),
        ("filters.second_order_highpass",
         &[("pole_frequency", 1000.0), ("q", 1.0), ("passband_gain", 2.0), ("f", 500.0)],
         &[(1e-6, 1e12), (0.05, 50.0), (0.001, 1000.0), (1e-12, 1e12)]),
        ("filters.deliyannis_bandpass_design",
         &[("center_frequency", 1000.0), ("q", 2.0), ("c0", 1e-8), ("f", 1000.0)],
         &[(1e-6, 1e12), (0.5, 50.0), (1e-15, 1.0), (0.0, 1e12)]),
        ("filters.butterworth_spec_synthesis",
         &[("passband_edge", 1000.0), ("stopband_edge", 4000.0),
           ("passband_loss", 1.0), ("stopband_loss", 40.0)],
         &[(1e-6, 1e12), (1e-6, 1e12), (0.001, 100.0), (0.001, 200.0)]),
        ("filters.sallen_key_lowpass",
         &[("r1", 10000.0), ("r2", 10000.0), ("c1", 1e-8), ("c2", 1e-8),
           ("amplifier_gain", 2.0), ("f", 1000.0)],
         &[(0.001, 1e9), (0.001, 1e9), (1e-15, 1.0), (1e-15, 1.0), (1.0, 100.0), (0.0, 1e12)]),
        ("filters.fliege_notch_response",
         &[("r0", 10000.0), ("c0", 1e-7), ("rq", 200000.0), ("f", 160.0)],
         &[(0.001, 1e9), (1e-15, 1.0), (0.001, 1e9), (0.0, 1e12)]),
        ("filters.buffered_rc_cascade",
         &[("r", 10000.0), ("c", 1e-8), ("stage_count", 3.0), ("f", 1000.0)],
         &[(0.001, 1e9), (1e-15, 1.0), (2.0, 32.0), (0.0, 1e12)]),
    ];
    for &(id, inputs, bounds) in cases {
        for (index, &(minimum, maximum)) in bounds.iter().enumerate() {
            let below = if minimum == 0.0 { -1.0 } else { minimum / 2.0 };
            let zero_or_negative = if minimum > 0.0 { 0.0 } else { -1.0 };
            for invalid in [below, zero_or_negative, -1.0, maximum * 2.0,
                            f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                let mut storage = [("", 0.0); 6];
                let changed = &mut storage[..inputs.len()];
                changed.copy_from_slice(inputs);
                changed[index].1 = invalid;
                assert!(calculate(id, changed).is_err(), "{id} accepted {}={invalid}", inputs[index].0);
            }
        }
        // A domain error must not corrupt subsequent calls on the cached worker engine.
        assert!(calculate(id, inputs).is_ok(), "{id} did not recover after rejected inputs");
    }
}
