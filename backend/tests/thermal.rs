mod common;

use common::{assert_close, calculate};

// Oracles are independent substitutions from research/thermal.md (TI thermal
// balance, university Fourier/lumped energy balance, NIST Arrhenius/CODATA,
// Vishay derating/TCR and the stated first-order expansion definition).
// Every vector specifies all inputs; no test relies on catalog defaults.
// Absolute tolerance is explicitly zero throughout: zero heat/drift/strain/
// exposure is an algebraic zero, not a measured near-zero engineering quantity.
// Nonzero outputs use relative tolerance, so tiny physical responses stay visible.
const JUNCTION: [(&str, f64); 7] = [
    ("ambient_temperature", 298.15), ("reference_temperature", 298.15),
    ("reference_power", 2.0), ("power_coefficient", 0.004),
    ("thermal_resistance", 20.0), ("fit_temperature_min", 233.15),
    ("fit_temperature_max", 423.15),
];
const SINK: [(&str, f64); 5] = [
    ("ambient_temperature", 313.15), ("junction_limit", 393.15),
    ("dissipated_power", 10.0), ("junction_case_resistance", 1.5),
    ("case_sink_resistance", 0.5),
];
const PATHS: [(&str, f64); 8] = [
    ("path_one_length", 0.005), ("path_one_area", 0.0001),
    ("path_one_conductivity", 200.0), ("path_two_length", 0.002),
    ("path_two_area", 0.0004), ("path_two_conductivity", 1.0),
    ("sink_temperature", 298.15), ("source_power", 21.0),
];
const LUMPED: [(&str, f64); 10] = [
    ("volume", 0.00001), ("density", 2700.0), ("specific_heat", 900.0),
    ("surface_area", 0.01), ("convection_coefficient", 10.0),
    ("thermal_conductivity", 200.0), ("ambient_temperature", 298.15),
    ("initial_temperature", 298.15), ("power", 2.0), ("elapsed_time", 243.0),
];
const ARRHENIUS: [(&str, f64); 4] = [
    ("activation_energy", 0.7), ("use_temperature", 328.15),
    ("test_temperature", 398.15), ("test_duration", 360000.0),
];
const DERATING: [(&str, f64); 5] = [
    ("rated_power", 0.25), ("rated_temperature", 343.15),
    ("zero_power_temperature", 428.15), ("ambient_temperature", 373.15),
    ("actual_power", 0.12),
];
const TCR: [(&str, f64); 6] = [
    ("reference_resistance", 10000.0), ("tcr_ppm", 100.0),
    ("reference_temperature", 298.15), ("operating_temperature", 358.15),
    ("fit_temperature_min", 218.15), ("fit_temperature_max", 443.15),
];
const EXPANSION: [(&str, f64); 6] = [
    ("initial_length", 0.1), ("expansion_coefficient", 0.000023),
    ("initial_temperature", 298.15), ("final_temperature", 378.15),
    ("fit_temperature_min", 273.15), ("fit_temperature_max", 423.15),
];

#[test]
fn junction_complete_researched_example() {
    let out = calculate("thermal.junction_feedback_equilibrium", &JUNCTION).unwrap();
    // Gain=4/25; rise=1000/21 K; equilibrium power=50/21 W.
    for (key, expected) in [("feedback_gain", 0.16),
        ("junction_temperature", 345.7690476190476),
        ("equilibrium_power", 2.380952380952381)] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
    assert_close(out["junction_temperature"] - 298.15,
        20.0 * out["equilibrium_power"], 1e-12, 0.0);
}

#[test]
fn heatsink_complete_researched_example() {
    let out = calculate("thermal.heatsink_resistance_budget", &SINK).unwrap();
    assert_close(out["total_resistance_budget"], 8.0, 1e-6, 0.0);
    assert_close(out["maximum_sink_resistance"], 6.0, 1e-6, 0.0);
    // The selected limiting sink gives 60+5+15=80 K total rise.
    assert_close(313.15 + 10.0 * (1.5 + 0.5 + out["maximum_sink_resistance"]),
        393.15, 1e-12, 0.0);
}

#[test]
fn conduction_complete_researched_example() {
    let out = calculate("thermal.parallel_conduction_paths", &PATHS).unwrap();
    for (key, expected) in [("path_one_resistance", 0.25),
        ("path_two_resistance", 5.0), ("equivalent_resistance", 0.2380952380952381),
        ("hot_temperature", 303.15), ("path_one_heat", 20.0), ("path_two_heat", 1.0)] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
    assert_close(out["path_one_heat"] + out["path_two_heat"], 21.0, 1e-12, 0.0);
}

#[test]
fn lumped_complete_researched_example() {
    let out = calculate("thermal.lumped_thermal_step", &LUMPED).unwrap();
    // 0.027 kg * 900 J/(kg K), one time constant and 1-exp(-1).
    for (key, expected) in [("thermal_capacitance", 24.3),
        ("convection_resistance", 10.0), ("time_constant", 243.0),
        ("biot_number", 0.00005), ("steady_temperature", 318.15),
        ("body_temperature", 310.7924111765712)] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn arrhenius_complete_researched_example() {
    let out = calculate("thermal.arrhenius_acceleration", &ARRHENIUS).unwrap();
    // Independent NIST exponent 4.352152076475486, not a ten-degree heuristic.
    assert_close(out["acceleration_factor"], 77.64538204961666, 1e-6, 0.0);
    assert_close(out["equivalent_use_duration"], 27952337.537862, 1e-6, 0.0);
}

#[test]
fn derating_complete_researched_example() {
    let out = calculate("thermal.resistor_power_derating", &DERATING).unwrap();
    // 55/85=11/17 remaining fraction, multiplied by 1/4 W.
    for (key, expected) in [("rating_fraction", 0.6470588235294118),
        ("allowed_power", 0.16176470588235295), ("power_margin", 0.04176470588235294)] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn tcr_complete_researched_example() {
    let out = calculate("thermal.resistor_tcr_drift", &TCR).unwrap();
    for (key, expected) in [("fractional_drift", 0.006),
        ("resistance_change", 60.0), ("operating_resistance", 10060.0)] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn expansion_complete_researched_example() {
    let out = calculate("thermal.free_linear_expansion", &EXPANSION).unwrap();
    for (key, expected) in [("thermal_strain", 0.00184),
        ("length_change", 0.000184), ("final_length", 0.100184)] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn junction_zero_feedback_fit_endpoints_and_shifted_reference() {
    // Zero beta recovers constant-power balance, with ambient at lower fit bound
    // and junction exactly at upper bound. Reference may be either endpoint.
    for reference in [300.0, 320.0] {
        let inputs = [("ambient_temperature", 300.0), ("reference_temperature", reference),
            ("reference_power", 2.0), ("power_coefficient", 0.0),
            ("thermal_resistance", 10.0), ("fit_temperature_min", 300.0),
            ("fit_temperature_max", 320.0)];
        let out = calculate("thermal.junction_feedback_equilibrium", &inputs).unwrap();
        assert_close(out["feedback_gain"], 0.0, 1e-12, 0.0);
        assert_close(out["junction_temperature"], 320.0, 1e-12, 0.0);
        assert_close(out["equilibrium_power"], 2.0, 1e-12, 0.0);
    }
    // Ambient dissipation 1.6 W, gain .2 => P_eq=2 W and Tj=T_ref=320 K.
    let inputs = [("ambient_temperature", 300.0), ("reference_temperature", 320.0),
        ("reference_power", 2.0), ("power_coefficient", 0.01),
        ("thermal_resistance", 10.0), ("fit_temperature_min", 280.0),
        ("fit_temperature_max", 350.0)];
    let out = calculate("thermal.junction_feedback_equilibrium", &inputs).unwrap();
    assert_close(out["feedback_gain"], 0.2, 1e-12, 0.0);
    assert_close(out["junction_temperature"], 320.0, 1e-12, 0.0);
    assert_close(out["equilibrium_power"], 2.0, 1e-12, 0.0);
}

#[test]
fn heatsink_zero_interface_and_power_scaling() {
    // 30 K rise with 2 K/W package and ideal interface: 10 W leaves 1 K/W;
    // half the power doubles total resistance headroom, leaving 4 K/W sink.
    for (power, total, sink) in [(10.0, 3.0, 1.0), (5.0, 6.0, 4.0)] {
        let inputs = [("ambient_temperature", 300.0), ("junction_limit", 330.0),
            ("dissipated_power", power), ("junction_case_resistance", 2.0),
            ("case_sink_resistance", 0.0)];
        let out = calculate("thermal.heatsink_resistance_budget", &inputs).unwrap();
        assert_close(out["total_resistance_budget"], total, 1e-12, 0.0);
        assert_close(out["maximum_sink_resistance"], sink, 1e-12, 0.0);
    }
}

#[test]
fn conduction_equal_paths_zero_heat_and_path_exchange() {
    // Distinct geometry gives R1=R2=2 K/W, Req=1 K/W and equal sharing.
    for power in [6.0, 0.0] {
        let inputs = [("path_one_length", 0.004), ("path_one_area", 0.0001),
            ("path_one_conductivity", 20.0), ("path_two_length", 0.008),
            ("path_two_area", 0.0001), ("path_two_conductivity", 40.0),
            ("sink_temperature", 300.0), ("source_power", power)];
        let out = calculate("thermal.parallel_conduction_paths", &inputs).unwrap();
        for (key, expected) in [("path_one_resistance", 2.0),
            ("path_two_resistance", 2.0), ("equivalent_resistance", 1.0),
            ("hot_temperature", 300.0 + power),
            ("path_one_heat", power / 2.0), ("path_two_heat", power / 2.0)] {
            assert_close(out[key], expected, 1e-12, 0.0);
        }
    }
    let inputs = [("path_one_length", 0.002), ("path_one_area", 0.0004),
        ("path_one_conductivity", 1.0), ("path_two_length", 0.005),
        ("path_two_area", 0.0001), ("path_two_conductivity", 200.0),
        ("sink_temperature", 298.15), ("source_power", 21.0)];
    let out = calculate("thermal.parallel_conduction_paths", &inputs).unwrap();
    for (key, expected) in [("path_one_resistance", 5.0),
        ("path_two_resistance", 0.25), ("equivalent_resistance", 0.2380952380952381),
        ("hot_temperature", 303.15), ("path_one_heat", 1.0), ("path_two_heat", 20.0)] {
        assert_close(out[key], expected, 1e-12, 0.0);
    }
    // A sub-ULP temperature rise must not erase nonzero heat partition.
    let mut inputs = PATHS;
    inputs[7].1 = 21e-20;
    let out = calculate("thermal.parallel_conduction_paths", &inputs).unwrap();
    assert_close(out["path_one_resistance"], 0.25, 1e-12, 0.0);
    assert_close(out["path_two_resistance"], 5.0, 1e-12, 0.0);
    assert_close(out["equivalent_resistance"], 0.2380952380952381, 1e-12, 0.0);
    assert_close(out["hot_temperature"], 298.15, 1e-12, 0.0);
    assert_close(out["path_one_heat"], 20e-20, 1e-12, 0.0);
    assert_close(out["path_two_heat"], 1e-20, 1e-12, 0.0);
}

#[test]
fn lumped_initial_condition_cooling_warming_and_long_time() {
    // Cooling/warming with zero heat: 300 +/- 50 exp(-1).
    for (initial, power, time, steady, body) in [
        (350.0, 2.0, 0.0, 320.0, 350.0),
        (350.0, 0.0, 243.0, 300.0, 318.3939720585721),
        (250.0, 0.0, 243.0, 300.0, 281.6060279414279),
        (250.0, 2.0, 243000.0, 320.0, 320.0),
    ] {
        let mut inputs = LUMPED;
        inputs[6].1 = 300.0;
        inputs[7].1 = initial;
        inputs[8].1 = power;
        inputs[9].1 = time;
        let out = calculate("thermal.lumped_thermal_step", &inputs).unwrap();
        for (key, expected) in [("thermal_capacitance", 24.3),
            ("convection_resistance", 10.0), ("time_constant", 243.0),
            ("biot_number", 0.00005), ("steady_temperature", steady),
            ("body_temperature", body)] {
            assert_close(out[key], expected, 1e-12, 0.0);
        }
    }
    // Just inside Bi<0.1 remains admissible (hV/(Ak)=0.099).
    let mut inputs = LUMPED;
    inputs[5].1 = 10.0;
    inputs[0].1 = 0.00099;
    inputs[9].1 = 0.0;
    let out = calculate("thermal.lumped_thermal_step", &inputs).unwrap();
    for (key, expected) in [("thermal_capacitance", 2405.7),
        ("convection_resistance", 10.0), ("time_constant", 24057.0),
        ("biot_number", 0.099), ("steady_temperature", 318.15),
        ("body_temperature", 298.15)] {
        assert_close(out[key], expected, 1e-12, 0.0);
    }
}

#[test]
fn lumped_short_time_retains_small_heating_weight() {
    // C=R=tau=1 and Bi=.001; t/tau=1e-16. Taylor remainder in
    // (1-exp(-x)) is <5e-33. Large steady temperature must not cancel T0.
    let inputs = [("volume", 0.001), ("density", 1000.0), ("specific_heat", 1.0),
        ("surface_area", 1.0), ("convection_coefficient", 1.0),
        ("thermal_conductivity", 1.0), ("ambient_temperature", 1.0),
        ("initial_temperature", 1e-20), ("power", 9999.0), ("elapsed_time", 1e-16)];
    let out = calculate("thermal.lumped_thermal_step", &inputs).unwrap();
    for (key, expected) in [("thermal_capacitance", 1.0),
        ("convection_resistance", 1.0), ("time_constant", 1.0),
        ("biot_number", 0.001), ("steady_temperature", 10000.0),
        ("body_temperature", 1.00000001e-12)] {
        assert_close(out[key], expected, 1e-12, 0.0);
    }
}

#[test]
fn arrhenius_unity_zero_exposure_and_independent_exponent() {
    // Equal temperatures are admissible; upper energy/duration bounds inclusive.
    for duration in [0.0, 120.0, 1e100] {
        let inputs = [("activation_energy", 10.0), ("use_temperature", 300.0),
            ("test_temperature", 300.0), ("test_duration", duration)];
        let out = calculate("thermal.arrhenius_acceleration", &inputs).unwrap();
        assert_close(out["acceleration_factor"], 1.0, 1e-12, 0.0);
        assert_close(out["equivalent_use_duration"], duration, 1e-12, 0.0);
    }
    // Ea=300*k_B, temperatures 300 and 600 K => exponent exactly 1/2.
    for duration in [0.0, 100.0] {
        let inputs = [("activation_energy", 0.025851999786435),
            ("use_temperature", 300.0), ("test_temperature", 600.0),
            ("test_duration", duration)];
        let out = calculate("thermal.arrhenius_acceleration", &inputs).unwrap();
        assert_close(out["acceleration_factor"], 1.6487212707001282, 1e-12, 0.0);
        assert_close(out["equivalent_use_duration"],
            if duration == 0.0 { 0.0 } else { 164.87212707001282 }, 1e-12, 0.0);
    }
}

#[test]
fn derating_plateau_breakpoint_midpoint_endpoint_and_overload() {
    for (ambient, actual, fraction, allowed, margin) in [
        (250.0, 0.0, 1.0, 2.0, 2.0),
        (300.0, 2.0, 1.0, 2.0, 0.0),
        (350.0, 1.5, 0.5, 1.0, -0.5),
        (400.0, 0.0, 0.0, 0.0, 0.0),
        (400.0, 0.25, 0.0, 0.0, -0.25),
    ] {
        let inputs = [("rated_power", 2.0), ("rated_temperature", 300.0),
            ("zero_power_temperature", 400.0), ("ambient_temperature", ambient),
            ("actual_power", actual)];
        let out = calculate("thermal.resistor_power_derating", &inputs).unwrap();
        assert_close(out["rating_fraction"], fraction, 1e-12, 0.0);
        assert_close(out["allowed_power"], allowed, 1e-12, 0.0);
        assert_close(out["power_margin"], margin, 1e-12, 0.0);
    }
}

#[test]
fn tcr_signed_slope_cooling_zero_slope_and_fit_endpoints() {
    // A 200 ppm/K slope across 100 K gives exactly 2 percent magnitude.
    for (slope, reference, operating, drift, change, resistance) in [
        (-200.0, 250.0, 350.0, -0.02, -20.0, 980.0),
        (200.0, 350.0, 250.0, -0.02, -20.0, 980.0),
        (-200.0, 350.0, 250.0, 0.02, 20.0, 1020.0),
        (-200.0, 300.0, 300.0, 0.0, 0.0, 1000.0),
        (0.0, 250.0, 350.0, 0.0, 0.0, 1000.0),
    ] {
        let inputs = [("reference_resistance", 1000.0), ("tcr_ppm", slope),
            ("reference_temperature", reference), ("operating_temperature", operating),
            ("fit_temperature_min", 250.0), ("fit_temperature_max", 350.0)];
        let out = calculate("thermal.resistor_tcr_drift", &inputs).unwrap();
        assert_close(out["fractional_drift"], drift, 1e-12, 0.0);
        assert_close(out["resistance_change"], change, 1e-12, 0.0);
        assert_close(out["operating_resistance"], resistance, 1e-12, 0.0);
    }
}

#[test]
fn expansion_signed_cte_cooling_zero_and_both_strain_limits() {
    // Fit endpoints are included; +/-1% strains are included, not clamped.
    for (alpha, initial, final_temp, strain, change, length) in [
        (0.0001, 250.0, 350.0, 0.01, 0.002, 0.202),
        (-0.0001, 250.0, 350.0, -0.01, -0.002, 0.198),
        (0.0001, 350.0, 250.0, -0.01, -0.002, 0.198),
        (-0.0001, 350.0, 250.0, 0.01, 0.002, 0.202),
        (0.0001, 300.0, 300.0, 0.0, 0.0, 0.2),
        (0.0, 250.0, 350.0, 0.0, 0.0, 0.2),
    ] {
        let inputs = [("initial_length", 0.2), ("expansion_coefficient", alpha),
            ("initial_temperature", initial), ("final_temperature", final_temp),
            ("fit_temperature_min", 250.0), ("fit_temperature_max", 350.0)];
        let out = calculate("thermal.free_linear_expansion", &inputs).unwrap();
        assert_close(out["thermal_strain"], strain, 1e-12, 0.0);
        assert_close(out["length_change"], change, 1e-12, 0.0);
        assert_close(out["final_length"], length, 1e-12, 0.0);
    }
}

#[test]
fn junction_rejects_all_cross_input_model_guards() {
    // Each change starts from the complete independent researched vector.
    for (index, value) in [
        (5, 423.15), (5, 424.0), // zero/reversed fit width
        (0, 233.0), (0, 424.0), // ambient outside either fit endpoint
        (1, 233.0), (1, 424.0), // reference outside either fit endpoint
        (3, 0.025), (3, 0.03), // gain exactly one / above one
        (6, 345.0), // stable root above fit maximum
    ] {
        let mut inputs = JUNCTION;
        inputs[index].1 = value;
        assert!(calculate("thermal.junction_feedback_equilibrium", &inputs).is_err(),
            "accepted invalid {}={value}", inputs[index].0);
    }
    // Both temperatures are in fit; gain .4 is stable, but ambient power is
    // zero/negative because beta*(Ta-Tref) <= -1.
    for reference in [350.0, 360.0] {
        let inputs = [("ambient_temperature", 300.0), ("reference_temperature", reference),
            ("reference_power", 2.0), ("power_coefficient", 0.02),
            ("thermal_resistance", 10.0), ("fit_temperature_min", 250.0),
            ("fit_temperature_max", 500.0)];
        assert!(calculate("thermal.junction_feedback_equilibrium", &inputs).is_err());
    }
}

#[test]
fn heatsink_rejects_temperature_and_exhausted_resistance_budget() {
    for (limit, interface) in [(300.0, 0.0), (299.0, 0.0),
        (330.0, 1.0), (330.0, 1.1)] {
        // 30 K /10 W -2 K/W package -1 K/W interface is exactly zero.
        let inputs = [("ambient_temperature", 300.0), ("junction_limit", limit),
            ("dissipated_power", 10.0), ("junction_case_resistance", 2.0),
            ("case_sink_resistance", interface)];
        assert!(calculate("thermal.heatsink_resistance_budget", &inputs).is_err());
    }
}

#[test]
fn lumped_rejects_biot_limit_and_nonuniform_body_region() {
    for volume in [0.1, 0.11] {
        // h=A=k=1 => Bi=V; exact .1 must be rejected as well as greater values.
        let inputs = [("volume", volume), ("density", 1000.0), ("specific_heat", 1.0),
            ("surface_area", 1.0), ("convection_coefficient", 1.0),
            ("thermal_conductivity", 1.0), ("ambient_temperature", 300.0),
            ("initial_temperature", 300.0), ("power", 1.0), ("elapsed_time", 1.0)];
        assert!(calculate("thermal.lumped_thermal_step", &inputs).is_err());
    }
}

#[test]
fn arrhenius_rejects_reversed_stress_and_factor_or_exposure_overflow() {
    let mut reversed = ARRHENIUS;
    reversed[2].1 = 328.0;
    assert!(calculate("thermal.arrhenius_acceleration", &reversed).is_err());
    // Ea=10 eV, use=100 K, test=1000 K => exponent ~1044 > ln(1e300).
    for duration in [0.0, 1.0] {
        let inputs = [("activation_energy", 10.0), ("use_temperature", 100.0),
            ("test_temperature", 1000.0), ("test_duration", duration)];
        assert!(calculate("thermal.arrhenius_acceleration", &inputs).is_err());
    }
    // Exponent=500 is individually finite and below ln(1e300)~690.8,
    // but exposure=exp(500)*1e100 exceeds the declared 1e300 ceiling.
    let inputs = [("activation_energy", 8.617333262145), ("use_temperature", 100.0),
        ("test_temperature", 200.0), ("test_duration", 1e100)];
    assert!(calculate("thermal.arrhenius_acceleration", &inputs).is_err());
    let mut finite = inputs;
    finite[3].1 = 1.0;
    let out = calculate("thermal.arrhenius_acceleration", &finite).unwrap();
    assert_close(out["acceleration_factor"], 1.4035922178528375e217, 1e-12, 0.0);
    assert_close(out["equivalent_use_duration"], 1.4035922178528375e217, 1e-12, 0.0);
}

#[test]
fn derating_rejects_invalid_curve_and_above_endpoint() {
    for (index, value) in [(2, 343.15), (2, 340.0), (3, 428.16)] {
        let mut inputs = DERATING;
        inputs[index].1 = value;
        assert!(calculate("thermal.resistor_power_derating", &inputs).is_err());
    }
}

#[test]
fn tcr_rejects_each_fit_guard_and_nonpositive_resistance() {
    for (index, value) in [(4, 443.15), (4, 444.0),
        (2, 218.0), (2, 444.0), (3, 218.0), (3, 444.0)] {
        let mut inputs = TCR;
        inputs[index].1 = value;
        assert!(calculate("thermal.resistor_tcr_drift", &inputs).is_err());
    }
    // Delta T=100 K and -10000 ppm/K extrapolate to zero resistance.
    for slope in [-10000.0, -10001.0] {
        let inputs = [("reference_resistance", 1000.0), ("tcr_ppm", slope),
            ("reference_temperature", 250.0), ("operating_temperature", 350.0),
            ("fit_temperature_min", 250.0), ("fit_temperature_max", 350.0)];
        assert!(calculate("thermal.resistor_tcr_drift", &inputs).is_err());
    }
}

#[test]
fn expansion_rejects_each_fit_guard_and_excess_strain_of_either_sign() {
    for (index, value) in [(4, 423.15), (4, 424.0),
        (2, 273.0), (2, 424.0), (3, 273.0), (3, 424.0)] {
        let mut inputs = EXPANSION;
        inputs[index].1 = value;
        assert!(calculate("thermal.free_linear_expansion", &inputs).is_err());
    }
    for alpha in [0.000101, -0.000101] {
        let inputs = [("initial_length", 0.2), ("expansion_coefficient", alpha),
            ("initial_temperature", 250.0), ("final_temperature", 350.0),
            ("fit_temperature_min", 250.0), ("fit_temperature_max", 350.0)];
        assert!(calculate("thermal.free_linear_expansion", &inputs).is_err());
    }
}

#[test]
fn every_thermal_scalar_range_and_finiteness_guard() {
    // Table identifies physically positive inputs. Every other bounded input is
    // nonnegative, except signed material coefficients which are unbounded.
    let cases: [(&str, &[(&str, f64)], &[usize], &[usize]); 8] = [
        ("thermal.junction_feedback_equilibrium", &JUNCTION, &[0, 1, 2, 4, 5, 6], &[3]),
        ("thermal.heatsink_resistance_budget", &SINK, &[0, 1, 2, 3], &[4]),
        ("thermal.parallel_conduction_paths", &PATHS, &[0, 1, 2, 3, 4, 5, 6], &[7]),
        ("thermal.lumped_thermal_step", &LUMPED, &[0, 1, 2, 3, 4, 5, 6, 7], &[8, 9]),
        ("thermal.arrhenius_acceleration", &ARRHENIUS, &[0, 1, 2], &[3]),
        ("thermal.resistor_power_derating", &DERATING, &[0, 1, 2, 3], &[4]),
        ("thermal.resistor_tcr_drift", &TCR, &[0, 2, 3, 4, 5], &[]),
        ("thermal.free_linear_expansion", &EXPANSION, &[0, 2, 3, 4, 5], &[]),
    ];
    for (id, vector, positive, nonnegative) in cases {
        for &index in positive {
            for invalid in [0.0, -1.0] {
                let mut inputs = vector.to_vec();
                inputs[index].1 = invalid;
                assert!(calculate(id, &inputs).is_err(), "{id}: accepted {}={invalid}", inputs[index].0);
            }
        }
        for &index in nonnegative {
            let mut inputs = vector.to_vec();
            inputs[index].1 = -1.0;
            assert!(calculate(id, &inputs).is_err(), "{id}: accepted negative {}", inputs[index].0);
        }
        for index in 0..vector.len() {
            for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                let mut inputs = vector.to_vec();
                inputs[index].1 = invalid;
                assert!(calculate(id, &inputs).is_err(), "{id}: accepted nonfinite {}", inputs[index].0);
            }
        }
    }
    for (index, value) in [(0, 10.0001), (3, 1.0001e100)] {
        let mut inputs = ARRHENIUS;
        inputs[index].1 = value;
        assert!(calculate("thermal.arrhenius_acceleration", &inputs).is_err());
    }
}
