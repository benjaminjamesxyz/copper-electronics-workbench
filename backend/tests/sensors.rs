mod common;

use common::{assert_close, calculate};
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, FRAC_PI_6, PI};

const NTC: &str = "sensors.ntc_steinhart_hart";
const RTD: &str = "sensors.platinum_callendar_van_dusen";
const TC: &str = "sensors.type_t_cold_junction_emf";
const HALL: &str = "sensors.hall_plate_angle";
const AMR: &str = "sensors.amr_angular_resistance";
const PH: &str = "sensors.hydrogen_activity_electrode";
const CAP: &str = "sensors.differential_gap_capacitor";

// Non-default calibration and geometry. No expectations come from catalog examples.
const N: [(&str, f64); 6] = [
    ("resistance", 7.38905609893065), ("coefficient_a", 0.002),
    ("coefficient_b", 0.0004), ("coefficient_c", 0.00005),
    ("fit_temperature_min", 250.0), ("fit_temperature_max", 400.0),
];
const R: [(&str, f64); 2] = [("zero_resistance", 250.0), ("temperature_celsius", -50.0)];
const T: [(&str, f64); 2] = [("hot_temperature", 250.0), ("cold_temperature", 40.0)];
const H: [(&str, f64); 7] = [
    ("field_magnitude", 0.08), ("field_angle", FRAC_PI_6),
    ("bias_current", -0.002), ("hall_coefficient", 0.005),
    ("plate_thickness", 0.00002), ("field_limit", 0.08), ("current_limit", 0.002),
];
const A: [(&str, f64); 5] = [
    ("perpendicular_resistance", 800.0), ("resistance_anisotropy", -40.0),
    ("magnetization_angle", FRAC_PI_4), ("bias_current", -0.003), ("current_limit", 0.003),
];
const P: [(&str, f64); 4] = [
    ("sample_activity", 1e-9), ("standard_activity", 1e-6),
    ("temperature", 310.0), ("offset_voltage", -0.012),
];
const C: [(&str, f64); 5] = [
    ("plate_width", 0.02), ("plate_length", 0.03), ("rest_gap", 0.0002),
    ("displacement", -0.0001), ("relative_permittivity", 3.0),
];

#[test]
fn ntc_complete_cubic_calibration_and_local_derivative() {
    let out = calculate(NTC, &N).unwrap();
    // ln(R)=2: reciprocal temperature is 0.0032; its log slope is 0.001.
    for (key, expected) in [("temperature_kelvin", 312.5), ("temperature_celsius", 39.35),
        ("temperature_sensitivity", -13.216336253575458)] {
        assert_close(out[key], expected, 1e-12, 0.0);
    }
    let mut low = N;
    let mut high = N;
    low[0].1 -= 1e-4;
    high[0].1 += 1e-4;
    let lo = calculate(NTC, &low).unwrap();
    let hi = calculate(NTC, &high).unwrap();
    assert!(hi["temperature_kelvin"] < lo["temperature_kelvin"]);
    assert_close((hi["temperature_kelvin"] - lo["temperature_kelvin"]) / 2e-4,
        out["temperature_sensitivity"], 2e-8, 0.0);
}

#[test]
fn ntc_two_coefficient_signed_a_and_inclusive_fit_endpoints() {
    for temperature in [250.0, 400.0] {
        let inputs = [("resistance", 1.0), ("coefficient_a", 1.0 / temperature),
            ("coefficient_b", 0.001), ("coefficient_c", 0.0),
            ("fit_temperature_min", 250.0), ("fit_temperature_max", 400.0)];
        let out = calculate(NTC, &inputs).unwrap();
        assert_close(out["temperature_kelvin"], temperature, 1e-14, 0.0);
        assert_close(out["temperature_celsius"], temperature - 273.15, 1e-14, 0.0);
        assert_close(out["temperature_sensitivity"], -0.001 * temperature * temperature, 1e-14, 0.0);
    }
    let out = calculate(NTC, &[("resistance", 4.0_f64.exp()), ("coefficient_a", -0.001),
        ("coefficient_b", 0.001), ("coefficient_c", 0.0),
        ("fit_temperature_min", 250.0), ("fit_temperature_max", 400.0)]).unwrap();
    assert_close(out["temperature_kelvin"], 1000.0 / 3.0, 1e-12, 0.0);
    assert_close(out["temperature_sensitivity"], -1000.0 / (9.0 * 4.0_f64.exp()), 1e-12, 0.0);
}

#[test]
fn ntc_rejects_scalar_calibration_and_extrapolation_errors_then_recovers() {
    for (index, bad) in [(0, 0.0), (0, -1.0), (2, 0.0), (2, -0.001),
        (3, -1e-8), (4, 0.0), (5, 0.0), (4, 400.0), (4, 450.0),
        (1, 0.0), (1, 0.01), (1, -0.01)] {
        let mut inputs = N;
        inputs[index].1 = bad;
        assert!(calculate(NTC, &inputs).is_err(), "accepted index {index}={bad}");
        assert_close(calculate(NTC, &N).unwrap()["temperature_kelvin"], 312.5, 1e-12, 0.0);
    }
}

#[test]
fn platinum_complete_negative_positive_and_endpoint_oracles() {
    // Decimal arithmetic on the TI/IEC coefficient branches, R0=250 ohm.
    for (t, resistance, sensitivity, error) in [
        (-200.0, 46.3002, 1.080838, -8.2848),
        (-50.0, 200.7657046875, 0.9928196875, -0.3805453125),
        (0.0, 250.0, 0.977075, 0.0),
        (150.0, 393.3128125, 0.9337625, -3.2484375),
        (850.0, 976.2028125, 0.7316375, -104.3109375),
    ] {
        let out = calculate(RTD, &[("zero_resistance", 250.0), ("temperature_celsius", t)]).unwrap();
        assert_close(out["resistance"], resistance, 1e-12, 0.0);
        assert_close(out["resistance_sensitivity"], sensitivity, 1e-12, 0.0);
        assert_close(out["linear_model_error"], error, 1e-12, 0.0);
    }
}

#[test]
fn platinum_zero_branch_continuity_tiny_curvature_and_resistance_scaling() {
    let base = calculate(RTD, &R).unwrap();
    let scaled = calculate(RTD, &[("zero_resistance", 2500.0), R[1]]).unwrap();
    for key in ["resistance", "resistance_sensitivity", "linear_model_error"] {
        assert_close(scaled[key], 10.0 * base[key], 1e-12, 0.0);
    }
    for t in [-1e-8, 1e-8] {
        let out = calculate(RTD, &[("zero_resistance", 250.0), ("temperature_celsius", t)]).unwrap();
        // The omitted subzero cubic correction is <8e-12 relative at this t.
        assert_close(out["linear_model_error"], -1.44375e-20, 1e-10, 0.0);
        assert_close(out["resistance"], 250.0, 4e-11, 0.0);
        assert_close(out["resistance_sensitivity"], 0.977075, 4e-12, 0.0);
    }
}

#[test]
fn platinum_rejects_out_of_range_then_recovers() {
    for (index, bad) in [(0, 0.0), (0, -250.0), (1, -200.000001), (1, 850.000001)] {
        let mut inputs = R;
        inputs[index].1 = bad;
        assert!(calculate(RTD, &inputs).is_err());
        assert_close(calculate(RTD, &R).unwrap()["resistance"], 200.7657046875, 1e-12, 0.0);
    }
}

#[test]
fn thermocouple_complete_nist_nondefault_oracle_and_reversal() {
    // NIST SRD60 positive-branch coefficients, summed in 70-digit decimal arithmetic.
    let out = calculate(TC, &T).unwrap();
    for (key, value) in [("hot_emf", 0.012013410275125366),
        ("cold_emf", 0.0016117927674460546), ("terminal_emf", 0.010401617507679312),
        ("hot_sensitivity", 0.00005580016304841406)] {
        assert_close(out[key], value, 1e-11, 0.0);
    }
    let reverse = calculate(TC, &[("hot_temperature", 40.0), ("cold_temperature", 250.0)]).unwrap();
    assert_close(reverse["terminal_emf"], -out["terminal_emf"], 1e-12, 0.0);
    assert_close(reverse["hot_emf"], out["cold_emf"], 1e-12, 0.0);
    assert_close(reverse["cold_emf"], out["hot_emf"], 1e-12, 0.0);
    assert_close(reverse["hot_sensitivity"], 0.00004196406929227984, 1e-12, 0.0);
}

#[test]
fn thermocouple_equal_junctions_and_both_reference_endpoints() {
    for t in [0.0, 180.0, 400.0] {
        let out = calculate(TC, &[("hot_temperature", t), ("cold_temperature", t)]).unwrap();
        assert_eq!(out["terminal_emf"], 0.0);
        assert_eq!(out["hot_emf"], out["cold_emf"]);
    }
    let out = calculate(TC, &[("hot_temperature", 400.0), ("cold_temperature", 0.0)]).unwrap();
    assert_close(out["hot_emf"], 0.02087197005052672, 1e-11, 0.0);
    assert_eq!(out["cold_emf"], 0.0);
    assert_close(out["terminal_emf"], 0.02087197005052672, 1e-11, 0.0);
    assert_close(out["hot_sensitivity"], 0.0000618048838084544, 1e-11, 0.0);
}

#[test]
fn thermocouple_adjacent_temperatures_keep_signed_signal() {
    let t: f64 = 180.0;
    let next = f64::from_bits(t.to_bits() + 1);
    for (hot, cold, sign) in [(next, t, 1.0), (t, next, -1.0)] {
        let out = calculate(TC, &[("hot_temperature", hot), ("cold_temperature", cold)]).unwrap();
        // Decimal polynomial difference at exact binary inputs, not f64 EMF subtraction.
        assert_close(out["terminal_emf"], sign * 1.4776750170286517e-18, 1e-11, 0.0);
        assert_close(out["hot_emf"], 0.008236607696989244, 1e-12, 0.0);
        assert_close(out["cold_emf"], 0.008236607696989244, 1e-12, 0.0);
        assert_close(out["hot_sensitivity"], 0.00005199106762550724, 1e-12, 0.0);
    }
}

#[test]
fn thermocouple_rejects_each_out_of_branch_junction_then_recovers() {
    for index in 0..2 {
        for bad in [-1e-9, 400.000000001] {
            let mut inputs = T;
            inputs[index].1 = bad;
            assert!(calculate(TC, &inputs).is_err());
            assert_close(calculate(TC, &T).unwrap()["terminal_emf"], 0.010401617507679312, 1e-11, 0.0);
        }
    }
}

#[test]
fn hall_complete_signed_trigonometric_oracle_and_reversals() {
    let out = calculate(HALL, &H).unwrap();
    assert_close(out["normal_field"], 0.04 * 3.0_f64.sqrt(), 1e-12, 0.0);
    assert_close(out["hall_voltage"], -0.02 * 3.0_f64.sqrt(), 1e-12, 0.0);
    assert_close(out["angle_sensitivity"], 0.02, 1e-12, 0.0);
    for index in [2, 3] {
        let mut inputs = H;
        inputs[index].1 *= -1.0;
        let reverse = calculate(HALL, &inputs).unwrap();
        assert_close(reverse["normal_field"], out["normal_field"], 1e-12, 0.0);
        for key in ["hall_voltage", "angle_sensitivity"] {
            assert_close(reverse[key], -out[key], 1e-12, 0.0);
        }
    }
    let mut inputs = H;
    inputs[1].1 += PI;
    let reverse = calculate(HALL, &inputs).unwrap();
    for key in ["normal_field", "hall_voltage", "angle_sensitivity"] {
        assert_close(reverse[key], -out[key], 1e-12, 0.0);
    }
}

#[test]
fn hall_zero_excitation_and_angle_endpoints() {
    for index in [0, 2, 3] {
        let mut inputs = H;
        inputs[index].1 = 0.0;
        let out = calculate(HALL, &inputs).unwrap();
        assert_eq!(out["hall_voltage"], 0.0);
        assert_eq!(out["angle_sensitivity"], 0.0);
        assert_close(out["normal_field"], if index == 0 { 0.0 } else { 0.04 * 3.0_f64.sqrt() }, 1e-12, 0.0);
    }
    let mut inputs = H;
    inputs[1].1 = 0.0;
    let out = calculate(HALL, &inputs).unwrap();
    assert_close(out["normal_field"], 0.08, 1e-12, 0.0);
    assert_close(out["hall_voltage"], -0.04, 1e-12, 0.0);
    assert_eq!(out["angle_sensitivity"], 0.0);
    inputs[1].1 = FRAC_PI_2;
    let out = calculate(HALL, &inputs).unwrap();
    // Binary pi/2 carries a cosine residual; allow four angular ulps times amplitude.
    assert_close(out["normal_field"], 0.0, 0.0, 0.08 * 4.0 * f64::EPSILON);
    assert_close(out["hall_voltage"], 0.0, 0.0, 0.04 * 4.0 * f64::EPSILON);
    assert_close(out["angle_sensitivity"], 0.04, 1e-12, 0.0);
}

#[test]
fn hall_calibration_limits_and_scalar_errors_recover() {
    for (index, bad) in [(0, -1e-6), (0, 0.080000001), (2, -0.002000001),
        (2, 0.002000001), (4, 0.0), (4, -1e-5), (5, 0.0), (6, 0.0)] {
        let mut inputs = H;
        inputs[index].1 = bad;
        assert!(calculate(HALL, &inputs).is_err());
        assert_close(calculate(HALL, &H).unwrap()["angle_sensitivity"], 0.02, 1e-12, 0.0);
    }
}

#[test]
fn amr_complete_negative_anisotropy_periodicity_and_current_sign() {
    let out = calculate(AMR, &A).unwrap();
    assert_close(out["resistance"], 780.0, 1e-12, 0.0);
    assert_close(out["voltage_drop"], -2.34, 1e-12, 0.0);
    assert_close(out["angular_resistance_sensitivity"], 40.0, 1e-12, 0.0);
    let mut inputs = A;
    inputs[2].1 += PI;
    let periodic = calculate(AMR, &inputs).unwrap();
    for key in ["resistance", "voltage_drop", "angular_resistance_sensitivity"] {
        assert_close(periodic[key], out[key], 1e-12, 0.0);
    }
    inputs = A;
    inputs[3].1 *= -1.0;
    let reverse = calculate(AMR, &inputs).unwrap();
    assert_close(reverse["voltage_drop"], 2.34, 1e-12, 0.0);
    assert_close(reverse["resistance"], 780.0, 1e-12, 0.0);
    assert_close(reverse["angular_resistance_sensitivity"], 40.0, 1e-12, 0.0);
}

#[test]
fn amr_parallel_perpendicular_isotropic_and_zero_bias_limits() {
    for (angle, resistance) in [(0.0, 760.0), (FRAC_PI_2, 800.0)] {
        let mut inputs = A;
        inputs[2].1 = angle;
        let out = calculate(AMR, &inputs).unwrap();
        assert_close(out["resistance"], resistance, 1e-12, 0.0);
        assert_close(out["voltage_drop"], -0.003 * resistance, 1e-12, 0.0);
        // sin(pi) residual is bounded by four ulps times the 80-ohm derivative amplitude.
        assert_close(out["angular_resistance_sensitivity"], 0.0, 0.0, 80.0 * 4.0 * f64::EPSILON);
    }
    let mut inputs = A;
    inputs[1].1 = 0.0;
    let out = calculate(AMR, &inputs).unwrap();
    assert_eq!(out["resistance"], 800.0);
    assert_close(out["voltage_drop"], -2.4, 1e-12, 0.0);
    assert_eq!(out["angular_resistance_sensitivity"], 0.0);
    inputs = A;
    inputs[3].1 = 0.0;
    let out = calculate(AMR, &inputs).unwrap();
    assert_eq!(out["voltage_drop"], 0.0);
    assert_close(out["resistance"], 780.0, 1e-12, 0.0);
    assert_close(out["angular_resistance_sensitivity"], 40.0, 1e-12, 0.0);
    inputs = A;
    inputs[1].1 = 40.0;
    let out = calculate(AMR, &inputs).unwrap();
    assert_close(out["resistance"], 820.0, 1e-12, 0.0);
    assert_close(out["voltage_drop"], -2.46, 1e-12, 0.0);
    assert_close(out["angular_resistance_sensitivity"], -40.0, 1e-12, 0.0);
}

#[test]
fn amr_rejects_nonpositive_endpoint_even_if_selected_angle_is_positive() {
    for (index, bad) in [(0, 0.0), (0, -1.0), (1, -800.0), (1, -801.0),
        (3, -0.003000001), (3, 0.003000001), (4, 0.0)] {
        let mut inputs = A;
        inputs[index].1 = bad;
        assert!(calculate(AMR, &inputs).is_err());
        assert_close(calculate(AMR, &A).unwrap()["resistance"], 780.0, 1e-12, 0.0);
    }
}

#[test]
fn electrode_complete_decade_oracle_equal_activity_and_reversal() {
    let out = calculate(PH, &P).unwrap();
    for (key, value) in [("sample_ph", 9.0), ("nernst_slope", 0.06151064364340943),
        ("electrode_voltage", -0.1965319309302283), ("activity_sensitivity", 26713733.11265005)] {
        assert_close(out[key], value, 1e-12, 0.0);
    }
    let mut inputs = P;
    inputs[0].1 = 1e-6;
    let equal = calculate(PH, &inputs).unwrap();
    assert_eq!(equal["electrode_voltage"], -0.012);
    assert_close(equal["sample_ph"], 6.0, 1e-12, 0.0);
    assert_close(equal["activity_sensitivity"], 26713.73311265005, 1e-12, 0.0);
    inputs[1].1 = 1e-9;
    let reverse = calculate(PH, &inputs).unwrap();
    assert_close(reverse["electrode_voltage"] + 0.012, -(out["electrode_voltage"] + 0.012), 1e-12, 0.0);
    assert_close(reverse["activity_sensitivity"], out["activity_sensitivity"] / 1000.0, 1e-12, 0.0);
}

#[test]
fn electrode_full_activity_envelope_and_temperature_scaling() {
    let k: f64 = 1.380649e-23 / 1.602176634e-19;
    for temperature in [273.15, 373.15] {
        for (sample, standard, ph, sign) in [(1e-14, 1.0, 14.0, -1.0), (1.0, 1e-14, 0.0, 1.0)] {
            let out = calculate(PH, &[("sample_activity", sample), ("standard_activity", standard),
                ("temperature", temperature), ("offset_voltage", 0.0)]).unwrap();
            let slope = k * temperature * 10.0_f64.ln();
            assert_close(out["sample_ph"], ph, 1e-12, 0.0);
            assert_close(out["nernst_slope"], slope, 1e-12, 0.0);
            assert_close(out["electrode_voltage"], sign * 14.0 * slope, 1e-12, 0.0);
            assert_close(out["activity_sensitivity"], k * temperature / sample, 1e-12, 0.0);
        }
    }
}

#[test]
fn electrode_adjacent_activity_keeps_signed_voltage() {
    let activity: f64 = 1e-6;
    let next = f64::from_bits(activity.to_bits() + 1);
    let kt: f64 = 1.380649e-23 / 1.602176634e-19 * 310.0;
    for (sample, standard, sign) in [(next, activity, 1.0), (activity, next, -1.0)] {
        let out = calculate(PH, &[("sample_activity", sample), ("standard_activity", standard),
            ("temperature", 310.0), ("offset_voltage", 0.0)]).unwrap();
        // First-order logarithm remainder is <2e-16 relative for adjacent activities.
        assert_close(out["electrode_voltage"], sign * kt * ((next - activity) / activity), 1e-12, 0.0);
        assert_close(out["activity_sensitivity"], kt / sample, 1e-12, 0.0);
    }
}

#[test]
fn electrode_scalar_envelope_errors_recover() {
    for (index, bad) in [(0, 0.0), (0, -1e-9), (0, 0.999e-14), (0, 1.000001),
        (1, 0.0), (1, -1e-9), (1, 0.999e-14), (1, 1.000001),
        (2, 273.149999), (2, 373.150001)] {
        let mut inputs = P;
        inputs[index].1 = bad;
        assert!(calculate(PH, &inputs).is_err());
        assert_close(calculate(PH, &P).unwrap()["electrode_voltage"], -0.1965319309302283, 1e-12, 0.0);
    }
}

#[test]
fn capacitor_complete_negative_displacement_and_signed_symmetry() {
    let out = calculate(CAP, &C).unwrap();
    for (key, value) in [("near_capacitance", 5.31251268768e-11),
        ("far_capacitance", 1.593753806304e-10), ("differential_capacitance", -1.062502537536e-10),
        ("normalized_difference", -0.5), ("differential_sensitivity", 1.77083756256e-6)] {
        assert_close(out[key], value, 1e-12, 0.0);
    }
    let mut inputs = C;
    inputs[3].1 *= -1.0;
    let reverse = calculate(CAP, &inputs).unwrap();
    assert_close(reverse["near_capacitance"], out["far_capacitance"], 1e-12, 0.0);
    assert_close(reverse["far_capacitance"], out["near_capacitance"], 1e-12, 0.0);
    for key in ["differential_capacitance", "normalized_difference"] {
        assert_close(reverse[key], -out[key], 1e-12, 0.0);
    }
    assert_close(reverse["differential_sensitivity"], out["differential_sensitivity"], 1e-12, 0.0);
}

#[test]
fn capacitor_center_dielectric_scaling_and_thin_gap_equality() {
    let mut inputs = C;
    inputs[3].1 = 0.0;
    let center = calculate(CAP, &inputs).unwrap();
    assert_close(center["near_capacitance"], 7.96876903152e-11, 1e-12, 0.0);
    assert_close(center["far_capacitance"], 7.96876903152e-11, 1e-12, 0.0);
    assert_eq!(center["differential_capacitance"], 0.0);
    assert_eq!(center["normalized_difference"], 0.0);
    assert_close(center["differential_sensitivity"], 7.96876903152e-7, 1e-12, 0.0);
    let base = calculate(CAP, &C).unwrap();
    inputs = C;
    inputs[4].1 = 6.0;
    let doubled = calculate(CAP, &inputs).unwrap();
    for key in ["near_capacitance", "far_capacitance", "differential_capacitance", "differential_sensitivity"] {
        assert_close(doubled[key], 2.0 * base[key], 1e-12, 0.0);
    }
    assert_eq!(doubled["normalized_difference"], base["normalized_difference"]);
    // Binary-exact dimensions make the declared 10:1 screen equality unambiguous.
    for (width, length) in [(0.15625, 0.25), (0.25, 0.15625)] {
        let out = calculate(CAP, &[("plate_width", width), ("plate_length", length),
            ("rest_gap", 0.015625), ("displacement", 0.0), ("relative_permittivity", 1.0)]).unwrap();
        assert_close(out["near_capacitance"], 8.8541878128e-12 * 2.5, 1e-12, 0.0);
        assert_eq!(out["differential_capacitance"], 0.0);
    }
}

#[test]
fn capacitor_tiny_displacement_retains_normal_and_subnormal_signals() {
    for x in [1e-24_f64, -1e-24, 1e-313, -1e-313] {
        let out = calculate(CAP, &[("plate_width", 0.01), ("plate_length", 0.01),
            ("rest_gap", 0.0001), ("displacement", x), ("relative_permittivity", 1.0)]).unwrap();
        let expected = 1.77083756256e-7 * x;
        // One minimum subnormal permits final rounding, never loss of the signed signal.
        let absolute = if x.abs() < 1e-300 { f64::from_bits(1) } else { 0.0 };
        assert!(out["differential_capacitance"] * x.signum() > 0.0,
            "representable signed differential lost for displacement={x}");
        assert_close(out["differential_capacitance"], expected, 1e-12, absolute);
        assert_close(out["normalized_difference"], x / 1e-4, 1e-10, 0.0);
        assert_close(out["near_capacitance"], 8.8541878128e-12, 1e-12, 0.0);
        assert_close(out["far_capacitance"], 8.8541878128e-12, 1e-12, 0.0);
        assert_close(out["differential_sensitivity"], 1.77083756256e-7, 1e-12, 0.0);
    }
}

#[test]
fn capacitor_near_contact_and_invalid_geometry_recovery() {
    let gap: f64 = C[2].1;
    let x = f64::from_bits(gap.to_bits() - 1);
    let mut inputs = C;
    inputs[3].1 = x;
    let out = calculate(CAP, &inputs).unwrap();
    let charge_per_volt_length: f64 = 8.8541878128e-12 * 3.0 * 0.02 * 0.03;
    assert_close(out["near_capacitance"], charge_per_volt_length / (gap - x), 1e-12, 0.0);
    assert_close(out["differential_sensitivity"], charge_per_volt_length / (gap - x).powi(2), 1e-12, 0.0);
    for (index, bad) in [(0, 0.0), (1, 0.0), (2, 0.0), (0, -0.01), (1, -0.01),
        (2, -0.0002), (3, gap), (3, -gap), (3, 2.0 * gap),
        (4, 0.999), (0, 0.002), (1, 0.002)] {
        inputs = C;
        inputs[index].1 = bad;
        assert!(calculate(CAP, &inputs).is_err());
        assert_close(calculate(CAP, &C).unwrap()["normalized_difference"], -0.5, 1e-12, 0.0);
    }
}

#[test]
fn every_sensor_rejects_each_nonfinite_input_and_recovers_on_same_thread() {
    for (id, nominal, key, expected) in [
        (NTC, N.as_slice(), "temperature_kelvin", 312.5),
        (RTD, R.as_slice(), "resistance", 200.7657046875),
        (TC, T.as_slice(), "terminal_emf", 0.010401617507679312),
        (HALL, H.as_slice(), "angle_sensitivity", 0.02),
        (AMR, A.as_slice(), "resistance", 780.0),
        (PH, P.as_slice(), "electrode_voltage", -0.1965319309302283),
        (CAP, C.as_slice(), "normalized_difference", -0.5),
    ] {
        for index in 0..nominal.len() {
            for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                let mut inputs = nominal.to_vec();
                inputs[index].1 = bad;
                assert!(calculate(id, &inputs).is_err(), "accepted {id} input {index}={bad}");
                assert_close(calculate(id, nominal).unwrap()[key], expected, 1e-11, 0.0);
            }
        }
    }
}
