mod common;

use common::{assert_close, calculate};
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI, TAU};

// Research/ac.md supplies these independently worked SI/RMS vectors. Nothing is
// read from runtime defaults, runtime examples, or catalog expressions.
#[test]
fn all_eight_complete_researched_examples() {
    let cases: &[(&str, &[(&str, f64)], &[(&str, f64)])] = &[
        ("ac.inductor_reactance", &[("frequency", 1000.0), ("inductance", 0.01), ("voltage_rms", 5.0)],
         &[("reactance", 62.83185307179586), ("current_rms", 0.07957747154594767), ("impedance_phase", 1.5707963267948966)]),
        ("ac.capacitor_reactance", &[("frequency", 1000.0), ("capacitance", 1e-6), ("voltage_rms", 5.0)],
         &[("reactance_magnitude", 159.15494309189535), ("impedance_imaginary", -159.15494309189535), ("current_rms", 0.031415926535897934), ("impedance_phase", -1.5707963267948966)]),
        ("ac.series_rlc_impedance", &[("frequency", 1000.0), ("resistance", 100.0), ("inductance", 0.01), ("capacitance", 1e-6), ("voltage_rms", 5.0)],
         &[("impedance_real", 100.0), ("impedance_imaginary", -96.32309002009949), ("impedance_magnitude", 138.84573335547688), ("impedance_phase", -0.7666714810055317), ("current_rms", 0.03601118939102619), ("power_factor", 0.7202237878205238)]),
        ("ac.parallel_rlc_impedance", &[("frequency", 1000.0), ("resistance", 1000.0), ("inductance", 0.01), ("capacitance", 1e-6), ("voltage_rms", 5.0)],
         &[("conductance", 0.001), ("susceptance", -0.009632309002009948), ("impedance_real", 10.66309788872204), ("impedance_imaginary", 102.71025378285057), ("impedance_magnitude", 103.26227718156345), ("impedance_phase", 1.4673496474397956), ("current_rms", 0.04842039258158743)]),
        ("ac.series_resonator_bandwidth", &[("resistance", 10.0), ("inductance", 0.01), ("capacitance", 1e-6)],
         &[("resonance_frequency", 1591.5494309189535), ("quality_factor", 10.0), ("bandwidth", 159.15494309189535), ("upper_half_power", 1673.11509740738), ("lower_half_power", 1513.9601543154847)]),
        ("ac.lossy_coil_antiresonance", &[("coil_resistance", 30.0), ("inductance", 0.01), ("capacitance", 1e-6), ("voltage_rms", 5.0)],
         &[("resonance_frequency", 1518.2413931464207), ("ideal_lc_frequency", 1591.5494309189535), ("resonance_resistance", 333.3333333333333), ("source_current_rms", 0.015), ("coil_current_rms", 0.05), ("copper_loss", 0.075)]),
        ("ac.sinusoidal_power_triangle", &[("voltage_rms", 230.0), ("current_rms", 4.0), ("phase", 0.5235987755982988)],
         &[("apparent_power", 920.0), ("real_power", 796.7433714816837), ("reactive_power", 460.0), ("power_factor", 0.8660254037844387)]),
        ("ac.shunt_power_factor_correction", &[("real_power", 1000.0), ("voltage_rms", 230.0), ("frequency", 50.0), ("initial_pf", 0.8), ("target_pf", 0.95)],
         &[("capacitor_reactive_power", 421.3158948211366), ("required_capacitance", 2.5351420515678246e-5), ("initial_source_current", 5.434782608695652), ("corrected_source_current", 4.576659038901602), ("capacitor_current_rms", 1.8318082383527678)]),
    ];
    for &(id, inputs, expected) in cases {
        let out = calculate(id, inputs).unwrap_or_else(|e| panic!("{id}: {e}"));
        for &(key, value) in expected {
            assert_close(out[key], value, 1e-6, 0.0);
        }
        assert_eq!(out.len(), expected.len(), "unexpected outputs for {id}");
    }
}

#[test]
fn inductive_loading_scales_with_frequency_and_inductance() {
    // omega=2, L=3 gives Z=j6 and I=12/6=2. Doubling either f or L halves I.
    for (frequency, inductance, reactance, current) in [
        (1.0 / PI, 3.0, 6.0, 2.0),
        (2.0 / PI, 3.0, 12.0, 1.0),
        (1.0 / PI, 6.0, 12.0, 1.0),
    ] {
        let out = calculate("ac.inductor_reactance", &[("frequency", frequency), ("inductance", inductance), ("voltage_rms", 12.0)]).unwrap();
        assert_close(out["reactance"], reactance, 1e-6, 0.0);
        assert_close(out["current_rms"], current, 1e-6, 0.0);
        assert_close(out["impedance_phase"], FRAC_PI_2, 1e-6, 0.0);
    }
}

#[test]
fn capacitive_loading_has_negative_impedance_and_inverse_scaling() {
    // omega=2, C=.25 -> Z=-j2; I=omega*C*12=6, independently from i=C dv/dt.
    for (frequency, capacitance, reactance, current) in [
        (1.0 / PI, 0.25, 2.0, 6.0),
        (2.0 / PI, 0.25, 1.0, 12.0),
        (1.0 / PI, 0.5, 1.0, 12.0),
    ] {
        let out = calculate("ac.capacitor_reactance", &[("frequency", frequency), ("capacitance", capacitance), ("voltage_rms", 12.0)]).unwrap();
        for (key, value) in [("reactance_magnitude", reactance), ("impedance_imaginary", -reactance), ("current_rms", current), ("impedance_phase", -FRAC_PI_2)] {
            assert_close(out[key], value, 1e-6, 0.0);
        }
    }
}

#[test]
fn zero_excitation_preserves_reactive_impedance_phase() {
    for id in ["ac.inductor_reactance", "ac.capacitor_reactance"] {
        let component = if id == "ac.inductor_reactance" { "inductance" } else { "capacitance" };
        let out = calculate(id, &[("frequency", 1.0 / TAU), (component, 1.0), ("voltage_rms", 0.0)]).unwrap();
        assert_close(out["current_rms"], 0.0, 0.0, 0.0); // Exactly zero excitation, no cancellation.
        if component == "inductance" {
            assert_close(out["reactance"], 1.0, 1e-6, 0.0);
            assert_close(out["impedance_phase"], FRAC_PI_2, 1e-6, 0.0);
        } else {
            assert_close(out["reactance_magnitude"], 1.0, 1e-6, 0.0);
            assert_close(out["impedance_imaginary"], -1.0, 1e-6, 0.0);
            assert_close(out["impedance_phase"], -FRAC_PI_2, 1e-6, 0.0);
        }
    }
}

#[test]
fn series_rlc_leading_and_lagging_three_four_five_triangles() {
    // R=2; omega=2 or .5, L=C=1 -> X=+/-1.5, |Z|=2.5, PF=.8.
    for (omega, sign) in [(2.0, 1.0), (0.5, -1.0)] {
        let out = calculate("ac.series_rlc_impedance", &[("frequency", omega / TAU), ("resistance", 2.0), ("inductance", 1.0), ("capacitance", 1.0), ("voltage_rms", 10.0)]).unwrap();
        for (key, value) in [("impedance_real", 2.0), ("impedance_imaginary", sign * 1.5), ("impedance_magnitude", 2.5), ("impedance_phase", sign * 1.5_f64.atan2(2.0)), ("current_rms", 4.0), ("power_factor", 0.8)] {
            assert_close(out[key], value, 1e-6, 0.0);
        }
        assert_close(out["power_factor"], out["impedance_phase"].cos(), 1e-6, 0.0);
    }
}

#[test]
fn series_rlc_resonance_and_zero_drive_are_finite() {
    for voltage in [0.0, 10.0] {
        let out = calculate("ac.series_rlc_impedance", &[("frequency", 1.0 / TAU), ("resistance", 2.0), ("inductance", 1.0), ("capacitance", 1.0), ("voltage_rms", voltage)]).unwrap();
        assert_close(out["impedance_real"], 2.0, 1e-6, 0.0);
        assert_close(out["impedance_magnitude"], 2.0, 1e-6, 0.0);
        // Cancellation of two 1-ohm reactances: 32 ulps of their physical scale.
        assert_close(out["impedance_imaginary"], 0.0, 0.0, 32.0 * f64::EPSILON);
        assert_close(out["impedance_phase"], 0.0, 0.0, 16.0 * f64::EPSILON);
        assert_close(out["current_rms"], voltage / 2.0, 1e-6, 0.0);
        assert_close(out["power_factor"], 1.0, 1e-6, 0.0);
    }
}

#[test]
fn parallel_rlc_conjugate_inversion_and_leading_lagging_signs() {
    // Y=.5 +/- j1.5 -> Z=.2 -/+ j.6, |Z|=sqrt(.4), source I=10sqrt(2.5).
    for (omega, sign) in [(2.0, 1.0), (0.5, -1.0)] {
        let out = calculate("ac.parallel_rlc_impedance", &[("frequency", omega / TAU), ("resistance", 2.0), ("inductance", 1.0), ("capacitance", 1.0), ("voltage_rms", 10.0)]).unwrap();
        for (key, value) in [("conductance", 0.5), ("susceptance", sign * 1.5), ("impedance_real", 0.2), ("impedance_imaginary", -sign * 0.6), ("impedance_magnitude", 0.4_f64.sqrt()), ("impedance_phase", -sign * 1.5_f64.atan2(0.5)), ("current_rms", 10.0 * 2.5_f64.sqrt())] {
            assert_close(out[key], value, 1e-6, 0.0);
        }
        assert_close(out["conductance"] * out["impedance_real"] - out["susceptance"] * out["impedance_imaginary"], 1.0, 1e-6, 0.0);
        // Imaginary component of Y*Z cancels; products each have magnitude .3.
        assert_close(out["conductance"] * out["impedance_imaginary"] + out["susceptance"] * out["impedance_real"], 0.0, 0.0, 32.0 * f64::EPSILON * 0.3);
    }
}

#[test]
fn parallel_resonance_is_shunt_resistance_not_infinite_impedance() {
    for voltage in [0.0, 10.0] {
        let out = calculate("ac.parallel_rlc_impedance", &[("frequency", 1.0 / TAU), ("resistance", 2.0), ("inductance", 1.0), ("capacitance", 1.0), ("voltage_rms", voltage)]).unwrap();
        assert_close(out["conductance"], 0.5, 1e-6, 0.0);
        // Two 1-S branch susceptances cancel; Z imaginary sensitivity is 1/G^2=4.
        assert_close(out["susceptance"], 0.0, 0.0, 32.0 * f64::EPSILON);
        assert_close(out["impedance_imaginary"], 0.0, 0.0, 128.0 * f64::EPSILON);
        assert_close(out["impedance_phase"], 0.0, 0.0, 64.0 * f64::EPSILON);
        assert_close(out["impedance_real"], 2.0, 1e-6, 0.0);
        assert_close(out["impedance_magnitude"], 2.0, 1e-6, 0.0);
        assert_close(out["current_rms"], voltage / 2.0, 1e-6, 0.0);
    }
}

#[test]
fn exact_half_power_roots_are_geometrically_not_arithmetically_centered() {
    // R=2,L=C=1 -> omega0=1, omega endpoints sqrt(2)+/-1, Q=.5.
    let out = calculate("ac.series_resonator_bandwidth", &[("resistance", 2.0), ("inductance", 1.0), ("capacitance", 1.0)]).unwrap();
    for (key, value) in [("resonance_frequency", 1.0 / TAU), ("quality_factor", 0.5), ("bandwidth", 1.0 / PI), ("upper_half_power", (2.0_f64.sqrt() + 1.0) / TAU), ("lower_half_power", (2.0_f64.sqrt() - 1.0) / TAU)] {
        assert_close(out[key], value, 1e-6, 0.0);
    }
    assert_close(out["upper_half_power"] - out["lower_half_power"], 1.0 / PI, 1e-6, 0.0);
    assert_close(out["upper_half_power"] * out["lower_half_power"], 1.0 / (TAU * TAU), 1e-6, 0.0);
    assert!((out["upper_half_power"] + out["lower_half_power"]) / 2.0 > out["resonance_frequency"]);
    for key in ["upper_half_power", "lower_half_power"] {
        let omega = TAU * out[key];
        assert_close((omega - 1.0 / omega).abs(), 2.0, 1e-6, 0.0);
        assert_close(4.0 / (4.0 + (omega - 1.0 / omega).powi(2)), 0.5, 1e-6, 0.0);
    }
}

#[test]
fn bandwidth_inclusive_resistance_extremes_retain_low_root() {
    for resistance in [1e-9, 1e12] {
        let out = calculate("ac.series_resonator_bandwidth", &[("resistance", resistance), ("inductance", 1.0), ("capacitance", 1.0)]).unwrap();
        assert_close(out["resonance_frequency"], 1.0 / TAU, 1e-6, 0.0);
        assert_close(out["quality_factor"], 1.0 / resistance, 1e-6, 0.0);
        assert_close(out["bandwidth"], resistance / TAU, 1e-6, 0.0);
        if resistance == 1e12 {
            // Asymptotic quadratic roots omegaH=R+O(1/R), omegaL=1/R+O(1/R^3).
            // O(1e-24) relative remainder is far below the 1e-6 comparison tolerance.
            assert_close(out["upper_half_power"], 1e12 / TAU, 1e-6, 0.0);
            assert_close(out["lower_half_power"], 1e-12 / TAU, 1e-6, 0.0);
        } else {
            // omegaH/L=1+/-R/2+O(R^2), with O(1e-18) remainder.
            assert_close(out["upper_half_power"], (1.0 + 5e-10) / TAU, 1e-6, 0.0);
            assert_close(out["lower_half_power"], (1.0 - 5e-10) / TAU, 1e-6, 0.0);
            assert!(out["upper_half_power"] > out["lower_half_power"]);
            // Endpoint subtraction is conditioned by f0/BW=1e9; allow 16 ulps of f0.
            assert_close(out["upper_half_power"] - out["lower_half_power"], 1e-9 / TAU, 1e-6, 16.0 * f64::EPSILON / TAU);
        }
        assert_close(out["upper_half_power"] * out["lower_half_power"], 1.0 / (TAU * TAU), 1e-6, 0.0);
    }
}

#[test]
fn lossy_coil_resonance_cancels_branch_susceptance_and_conserves_power() {
    // r=1,L=2,C=1 -> omega=.5, Zcoil=1+j, Ycoil=.5-j.5, Ycapacitor=j.5.
    for voltage in [0.0, 4.0] {
        let out = calculate("ac.lossy_coil_antiresonance", &[("coil_resistance", 1.0), ("inductance", 2.0), ("capacitance", 1.0), ("voltage_rms", voltage)]).unwrap();
        for (key, value) in [("resonance_frequency", 1.0 / (4.0 * PI)), ("ideal_lc_frequency", 1.0 / (TAU * 2.0_f64.sqrt())), ("resonance_resistance", 2.0), ("source_current_rms", voltage / 2.0), ("coil_current_rms", voltage / 2.0_f64.sqrt()), ("copper_loss", voltage * voltage / 2.0)] {
            assert_close(out[key], value, 1e-6, 0.0);
        }
        let omega = TAU * out["resonance_frequency"];
        assert_close(omega - 2.0 * omega / (1.0 + (2.0 * omega).powi(2)), 0.0, 0.0, 32.0 * f64::EPSILON * 0.5);
        assert_close(voltage * out["source_current_rms"], out["copper_loss"], 1e-6, 0.0);
        assert!(out["resonance_frequency"] < out["ideal_lc_frequency"]);
    }
}

#[test]
fn lossy_coil_one_ulp_above_existence_threshold_is_admissible() {
    let inductance = 1.0000000000000002; // 1 + 2^-52, exactly representable.
    let delta = f64::EPSILON;
    let out = calculate("ac.lossy_coil_antiresonance", &[("coil_resistance", 1.0), ("inductance", inductance), ("capacitance", 1.0), ("voltage_rms", 1.0)]).unwrap();
    for (key, value) in [("resonance_frequency", delta.sqrt() / (TAU * inductance)), ("ideal_lc_frequency", 1.0 / (TAU * inductance.sqrt())), ("resonance_resistance", inductance), ("source_current_rms", 1.0 / inductance), ("coil_current_rms", 1.0 / inductance.sqrt()), ("copper_loss", 1.0 / inductance)] {
        assert_close(out[key], value, 1e-6, 0.0);
    }
    for invalid_l in [1.0, 0.5] {
        assert!(calculate("ac.lossy_coil_antiresonance", &[("coil_resistance", 1.0), ("inductance", invalid_l), ("capacitance", 1.0), ("voltage_rms", 1.0)]).is_err());
    }
}

#[test]
fn power_triangle_leading_sign_and_pythagorean_identity() {
    for sign in [-1.0, 1.0] {
        let out = calculate("ac.sinusoidal_power_triangle", &[("voltage_rms", 10.0), ("current_rms", 2.0), ("phase", sign * FRAC_PI_4)]).unwrap();
        for (key, value) in [("apparent_power", 20.0), ("real_power", 10.0 * 2.0_f64.sqrt()), ("reactive_power", sign * 10.0 * 2.0_f64.sqrt()), ("power_factor", 1.0 / 2.0_f64.sqrt())] {
            assert_close(out[key], value, 1e-6, 0.0);
        }
        assert_close(out["real_power"].powi(2) + out["reactive_power"].powi(2), 400.0, 1e-6, 0.0);
    }
}

#[test]
fn power_triangle_zero_phase_and_purely_reactive_endpoints() {
    for phase in [-FRAC_PI_2, 0.0, FRAC_PI_2] {
        let out = calculate("ac.sinusoidal_power_triangle", &[("voltage_rms", 10.0), ("current_rms", 2.0), ("phase", phase)]).unwrap();
        assert_close(out["apparent_power"], 20.0, 1e-6, 0.0);
        if phase == 0.0 {
            assert_close(out["real_power"], 20.0, 1e-6, 0.0);
            assert_close(out["reactive_power"], 0.0, 0.0, 0.0);
            assert_close(out["power_factor"], 1.0, 1e-6, 0.0);
        } else {
            // cos of the representable pi/2 has a residual: two ulps of S and PF.
            assert_close(out["real_power"], 0.0, 0.0, 2.0 * f64::EPSILON * 20.0);
            assert_close(out["power_factor"], 0.0, 0.0, 2.0 * f64::EPSILON);
            assert_close(out["reactive_power"], phase.signum() * 20.0, 1e-6, 0.0);
        }
    }
    for phase in [-(FRAC_PI_2 + 1e-10), FRAC_PI_2 + 1e-10, -PI, PI] {
        assert!(calculate("ac.sinusoidal_power_triangle", &[("voltage_rms", 10.0), ("current_rms", 2.0), ("phase", phase)]).is_err());
    }
    let out = calculate("ac.sinusoidal_power_triangle", &[("voltage_rms", 1e-9), ("current_rms", 1e-12), ("phase", 0.0)]).unwrap();
    assert_close(out["apparent_power"], 1e-21, 1e-6, 0.0);
    assert_close(out["real_power"], 1e-21, 1e-6, 0.0);
    assert_close(out["reactive_power"], 0.0, 0.0, 0.0);
    assert_close(out["power_factor"], 1.0, 1e-6, 0.0);
}

#[test]
fn unity_target_compensation_obeys_rms_phasor_current_identity() {
    // .6 PF -> tan(phi)=4/3; P=300 gives Q=400 var. V=100 gives I=5 ->3 A.
    let out = calculate("ac.shunt_power_factor_correction", &[("real_power", 300.0), ("voltage_rms", 100.0), ("frequency", 50.0), ("initial_pf", 0.6), ("target_pf", 1.0)]).unwrap();
    for (key, value) in [("capacitor_reactive_power", 400.0), ("required_capacitance", 400.0 / (100.0 * PI * 10000.0)), ("initial_source_current", 5.0), ("corrected_source_current", 3.0), ("capacitor_current_rms", 4.0)] {
        assert_close(out[key], value, 1e-6, 0.0);
    }
    assert_close(out["corrected_source_current"].hypot(out["capacitor_current_rms"]), 5.0, 1e-6, 0.0);
}

#[test]
fn equal_power_factors_including_unity_require_exactly_no_capacitor() {
    for pf in [1e-9, 0.5, 0.8, 1.0] {
        let out = calculate("ac.shunt_power_factor_correction", &[("real_power", 300.0), ("voltage_rms", 100.0), ("frequency", 50.0), ("initial_pf", pf), ("target_pf", pf)]).unwrap();
        // Identical physical initial and target states: exact zero, not a small positive floor.
        for key in ["capacitor_reactive_power", "required_capacitance", "capacitor_current_rms"] {
            assert_close(out[key], 0.0, 0.0, 0.0);
        }
        for key in ["initial_source_current", "corrected_source_current"] {
            assert_close(out[key], 3.0 / pf, 1e-6, 0.0);
        }
    }
    assert!(calculate("ac.shunt_power_factor_correction", &[("real_power", 300.0), ("voltage_rms", 100.0), ("frequency", 50.0), ("initial_pf", 0.8), ("target_pf", 0.6)]).is_err());
}

#[test]
fn adjacent_power_factors_preserve_small_nonzero_compensation() {
    // Independent Taylor oracle: t'(1/2)=-8/sqrt(3). With delta=2^-52,
    // omitted quadratic term is O(delta) relative (<2e-15), not engine-derived.
    // A separate ordinary increment ensures conditioning is not single-vector-specific.
    for (target, delta) in [(0.5000000000000002, f64::EPSILON), (0.5000000001, 0.5000000001 - 0.5)] {
        let qc = 1e12 * 8.0 / 3.0_f64.sqrt() * delta;
        // At delta~1e-10 the Taylor relative remainder is <1e-9, below 1e-6.
        let out = calculate("ac.shunt_power_factor_correction", &[("real_power", 1e12), ("voltage_rms", 230.0), ("frequency", 50.0), ("initial_pf", 0.5), ("target_pf", target)]).unwrap();
        for (key, value) in [("capacitor_reactive_power", qc), ("required_capacitance", qc / (100.0 * PI * 52900.0)), ("initial_source_current", 1e12 / 115.0), ("corrected_source_current", 1e12 / (230.0 * target)), ("capacitor_current_rms", qc / 230.0)] {
            assert_close(out[key], value, 1e-6, 0.0);
        }
    }
}

#[test]
fn reactance_inclusive_numerical_envelopes_remain_finite_and_correct() {
    for (frequency, inductance, capacitance, voltage) in [(1e-6, 1e-15, 1e-18, 1e9), (1e12, 1e3, 1e3, 1e9)] {
        let inductive = calculate("ac.inductor_reactance", &[("frequency", frequency), ("inductance", inductance), ("voltage_rms", voltage)]).unwrap();
        let capacitive = calculate("ac.capacitor_reactance", &[("frequency", frequency), ("capacitance", capacitance), ("voltage_rms", voltage)]).unwrap();
        let xl = TAU * frequency * inductance;
        let xc = 1.0 / (TAU * frequency * capacitance);
        assert_close(inductive["reactance"], xl, 1e-6, 0.0);
        assert_close(inductive["current_rms"], voltage / xl, 1e-6, 0.0);
        assert_close(inductive["impedance_phase"], FRAC_PI_2, 1e-6, 0.0);
        assert_close(capacitive["reactance_magnitude"], xc, 1e-6, 0.0);
        assert_close(capacitive["impedance_imaginary"], -xc, 1e-6, 0.0);
        assert_close(capacitive["current_rms"], voltage * TAU * frequency * capacitance, 1e-6, 0.0);
        assert_close(capacitive["impedance_phase"], -FRAC_PI_2, 1e-6, 0.0);
    }
}

#[test]
fn every_scalar_guard_rejects_nonfinite_and_out_of_envelope_inputs() {
    let cases: &[(&str, &[(&str, f64)])] = &[
        ("ac.inductor_reactance", &[("frequency", 1000.0), ("inductance", 0.01), ("voltage_rms", 5.0)]),
        ("ac.capacitor_reactance", &[("frequency", 1000.0), ("capacitance", 1e-6), ("voltage_rms", 5.0)]),
        ("ac.series_rlc_impedance", &[("frequency", 1000.0), ("resistance", 100.0), ("inductance", 0.01), ("capacitance", 1e-6), ("voltage_rms", 5.0)]),
        ("ac.parallel_rlc_impedance", &[("frequency", 1000.0), ("resistance", 1000.0), ("inductance", 0.01), ("capacitance", 1e-6), ("voltage_rms", 5.0)]),
        ("ac.series_resonator_bandwidth", &[("resistance", 10.0), ("inductance", 0.01), ("capacitance", 1e-6)]),
        ("ac.lossy_coil_antiresonance", &[("coil_resistance", 30.0), ("inductance", 0.01), ("capacitance", 1e-6), ("voltage_rms", 5.0)]),
        ("ac.sinusoidal_power_triangle", &[("voltage_rms", 230.0), ("current_rms", 4.0), ("phase", PI / 6.0)]),
        ("ac.shunt_power_factor_correction", &[("real_power", 1000.0), ("voltage_rms", 230.0), ("frequency", 50.0), ("initial_pf", 0.8), ("target_pf", 0.95)]),
    ];
    for &(id, base) in cases {
        for index in 0..base.len() {
            let key = base[index].0;
            for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                let mut input = base.to_vec();
                input[index].1 = bad;
                assert!(calculate(id, &input).is_err(), "{id}: accepted nonfinite {key}");
            }
            let (min, max) = match key {
                "frequency" => (1e-6, 1e12),
                "inductance" => (1e-15, 1e3),
                "capacitance" => (1e-18, 1e3),
                "resistance" | "coil_resistance" => (1e-9, 1e12),
                "voltage_rms" if id == "ac.sinusoidal_power_triangle" || id == "ac.shunt_power_factor_correction" => (1e-9, 1e9),
                "voltage_rms" => (0.0, 1e9),
                "current_rms" => (1e-12, 1e9),
                "real_power" => (1e-9, 1e12),
                "initial_pf" | "target_pf" => (1e-9, 1.0),
                "phase" => continue, // Signed passive-load relational guard tested separately.
                _ => panic!("unaccounted input {key}"),
            };
            for bad in [if min == 0.0 { -1e-12 } else { min / 2.0 }, max * 2.0, -1.0] {
                let mut input = base.to_vec();
                input[index].1 = bad;
                assert!(calculate(id, &input).is_err(), "{id}: accepted {key}={bad}");
            }
            if min > 0.0 {
                let mut input = base.to_vec();
                input[index].1 = 0.0;
                assert!(calculate(id, &input).is_err(), "{id}: accepted zero {key}");
            }
        }
        // Exact-key contract and isolation: each rejection must leave a valid calculation usable.
        assert!(calculate(id, &base[1..]).is_err());
        let mut extra = base.to_vec();
        extra.push(("unexpected", 1.0));
        assert!(calculate(id, &extra).is_err());
        let out = calculate(id, base).unwrap();
        assert!(out.values().all(|value| value.is_finite()));
    }
}

#[test]
fn near_unity_initial_power_factor_preserves_representable_capacitor() {
    // a=1-2^-52. Independent root expansion sqrt(1-a^2)=sqrt(2*delta)
    // times (1+O(delta)); omitted relative term is below 1e-15.
    let initial = 0.9999999999999998;
    let qc = 1000.0 * (2.0 * f64::EPSILON).sqrt() / initial;
    let out = calculate("ac.shunt_power_factor_correction", &[("real_power", 1000.0), ("voltage_rms", 230.0), ("frequency", 50.0), ("initial_pf", initial), ("target_pf", 1.0)]).unwrap();
    for (key, value) in [("capacitor_reactive_power", qc), ("required_capacitance", qc / (100.0 * PI * 52900.0)), ("initial_source_current", 1000.0 / (230.0 * initial)), ("corrected_source_current", 1000.0 / 230.0), ("capacitor_current_rms", qc / 230.0)] {
        assert_close(out[key], value, 1e-6, 0.0);
    }
}

#[test]
fn series_and_parallel_envelope_corners_match_independent_phasors() {
    for (frequency, resistance, inductance, capacitance) in [
        (1e-6, 1e-9, 1e-15, 1e-18),
        (1e12, 1e12, 1e3, 1e3),
    ] {
        let inputs = [("frequency", frequency), ("resistance", resistance), ("inductance", inductance), ("capacitance", capacitance), ("voltage_rms", 1e9)];
        let omega = TAU * frequency;
        // Independently sum complex branch impedances, then use hypot/atan2.
        let imaginary = omega * inductance - (omega * capacitance).recip();
        let modulus = resistance.hypot(imaginary);
        let series = calculate("ac.series_rlc_impedance", &inputs).unwrap();
        for (key, value) in [("impedance_real", resistance), ("impedance_imaginary", imaginary), ("impedance_magnitude", modulus), ("impedance_phase", imaginary.atan2(resistance)), ("current_rms", 1e9 / modulus), ("power_factor", resistance / modulus)] {
            assert_close(series[key], value, 1e-6, 0.0);
        }
        // Admittance has G=1/R, B=omega*C-1/(omega*L). Convert to polar
        // and invert there, rather than reusing the catalog's Cartesian denominators.
        let conductance = resistance.recip();
        let susceptance = omega * capacitance - (omega * inductance).recip();
        let admittance_modulus = conductance.hypot(susceptance);
        let normalized_real = conductance / admittance_modulus;
        let normalized_imaginary = -susceptance / admittance_modulus;
        let parallel = calculate("ac.parallel_rlc_impedance", &inputs).unwrap();
        for (key, value) in [("conductance", conductance), ("susceptance", susceptance), ("impedance_real", normalized_real / admittance_modulus), ("impedance_imaginary", normalized_imaginary / admittance_modulus), ("impedance_magnitude", admittance_modulus.recip()), ("impedance_phase", (-susceptance).atan2(conductance)), ("current_rms", 1e9 * admittance_modulus)] {
            assert_close(parallel[key], value, 1e-6, 0.0);
        }
    }
}
