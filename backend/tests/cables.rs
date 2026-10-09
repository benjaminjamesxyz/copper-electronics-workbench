mod common;

use common::{assert_close, calculate};
use std::f64::consts::{LN_2, LN_10, PI, TAU};

// Independent source-law oracles: acosh(5/4)=ln(2), complex telegrapher roots,
// Gauss/Laplace radial fields, prescribed Hertz radius, a 3-4-5 thermal triangle,
// and the beam's second moment/root moment. No catalog examples are loaded.
const TWIN: [(&str, f64); 3] = [
    ("wire_diameter", 0.0008), ("center_spacing", 0.001),
    ("relative_permittivity", 4.0),
];
const TWIN_OUT: [(&str, f64); 4] = [
    ("geometry_factor", 0.6931471805599453),
    ("inductance_per_length", 2.772588722239781e-7),
    ("capacitance_per_length", 1.605214717239473e-10),
    ("differential_impedance", 41.56005941448021),
];
const RLGC: [(&str, f64); 6] = [
    ("series_resistance_per_length", 3.0), ("inductance_per_length", 4e-6),
    ("shunt_conductance_per_length", 0.012), ("capacitance_per_length", 5e-9),
    ("frequency", 1e6 / TAU), ("length", 4.0),
];
// z=3+j4, y=.012+j.005, gamma^2=.016+j.063; independent complex roots.
const RLGC_OUT: [(&str, f64); 11] = [
    ("angular_frequency", 1e6), ("series_impedance_magnitude", 5.0),
    ("shunt_admittance_magnitude", 0.013),
    ("normalized_propagation_square_real", 0.2461538461538462),
    ("normalized_propagation_square_imaginary", 0.9692307692307692),
    ("attenuation_constant", 0.20124611797498107),
    ("phase_constant", 0.15652475842498528),
    ("impedance_real", 18.92057519422899),
    ("impedance_imaginary", 5.16015687115336),
    ("forward_amplitude_ratio", 0.4470948690393243),
    ("attenuation_db", 6.9920062832788075),
];
const RADIAL: [(&str, f64); 4] = [
    ("conductor_radius", 0.002), ("shield_inner_radius", 0.006),
    ("peak_voltage", 1800.0), ("allowable_field", 2e6),
];
const RADIAL_OUT: [(&str, f64); 4] = [
    ("inner_surface_field", 819215.3039641536),
    ("outer_surface_field", 273071.7679880512),
    ("voltage_limit", 4394.449154672439),
    ("field_margin", 1180784.6960358464),
];
const HERTZ: [(&str, f64); 7] = [
    ("normal_force", 0.016), ("sphere_radius", 0.004),
    ("young_modulus", 96e9), ("poisson_ratio", 0.0),
    ("resistivity", 2e-8), ("electron_mean_free_path", 5e-8),
    ("elastic_pressure_limit", 2e8),
];
const HERTZ_OUT: [(&str, f64); 5] = [
    ("effective_modulus", 48e9), ("contact_radius", 1e-5),
    ("maximum_pressure", 76394372.68410976),
    ("constriction_resistance", 0.001), ("indentation", 2.5e-8),
];
const HOTSPOT: [(&str, f64); 4] = [
    ("contact_voltage", 0.09765625), ("bulk_temperature", 300.0),
    ("lorenz_number", 1.4901161193847656e-8), ("temperature_limit", 600.0),
];
const HOTSPOT_OUT: [(&str, f64); 4] = [
    ("hotspot_temperature", 500.0), ("temperature_rise", 200.0),
    ("contact_voltage_limit", 0.12685919000748613), ("temperature_margin", 100.0),
];
const BEAM: [(&str, f64); 6] = [
    ("beam_length", 0.016), ("beam_width", 0.003),
    ("beam_thickness", 0.0004), ("young_modulus", 120e9),
    ("tip_deflection", 0.00064), ("allowable_stress", 360e6),
];
const BEAM_OUT: [(&str, f64); 4] = [
    ("spring_rate", 1406.25), ("normal_force", 0.9),
    ("root_stress", 180e6), ("stress_margin", 180e6),
];
const CASES: &[(&str, &[(&str, f64)], &[(&str, f64)])] = &[
    ("cables.twin_lead_geometry", &TWIN, &TWIN_OUT),
    ("cables.lossy_rlgc_line", &RLGC, &RLGC_OUT),
    ("cables.radial_insulation_stress", &RADIAL, &RADIAL_OUT),
    ("cables.hertz_holm_contact", &HERTZ, &HERTZ_OUT),
    ("cables.contact_hotspot_temperature", &HOTSPOT, &HOTSPOT_OUT),
    ("cables.cantilever_contact_spring", &BEAM, &BEAM_OUT),
];

#[test]
fn every_model_has_a_complete_independent_nondefault_oracle() {
    for &(id, inputs, expected) in CASES {
        let out = calculate(id, inputs).unwrap_or_else(|e| panic!("{id}: {e}"));
        assert_eq!(out.len(), expected.len(), "{id} output set");
        for &(key, value) in expected {
            assert_close(out[key], value, 1e-10, 0.0);
        }
    }
}

#[test]
fn twin_lead_scale_and_dielectric_invariants_preserve_loop_convention() {
    for scale in [0.125, 1.0, 16.0] {
        for er in [1.0, 4.0, 9.0] {
            let out = calculate("cables.twin_lead_geometry", &[
                ("wire_diameter", TWIN[0].1 * scale),
                ("center_spacing", TWIN[1].1 * scale), ("relative_permittivity", er),
            ]).unwrap();
            assert_close(out["geometry_factor"], LN_2, 1e-12, 0.0);
            assert_close(out["inductance_per_length"], TWIN_OUT[1].1, 1e-12, 0.0);
            assert_close(out["capacitance_per_length"], TWIN_OUT[2].1 * er / 4.0, 1e-12, 0.0);
            assert_close(out["differential_impedance"], TWIN_OUT[3].1 * 2.0 / er.sqrt(), 1e-12, 0.0);
            // TEM energy identity checks full loop L and between-wire C conventions.
            assert_close(out["inductance_per_length"] * out["capacitance_per_length"],
                4.0 * PI * 1e-7 * 8.8541878128e-12 * er, 1e-12, 0.0);
        }
    }
}

#[test]
fn twin_lead_one_ulp_gap_retains_positive_geometry_and_impedance() {
    let u = f64::EPSILON;
    let out = calculate("cables.twin_lead_geometry", &[
        ("wire_diameter", 1.0), ("center_spacing", 1.0 + u),
        ("relative_permittivity", 1.0),
    ]).unwrap();
    // acosh(1+u)=sqrt(2u)*(1-u/12+O(u^2)); neglected term below 1e-31.
    let q = (2.0 * u).sqrt() * (1.0 - u / 12.0);
    let inductance = 4e-7 * q;
    let capacitance = PI * 8.8541878128e-12 / q;
    for (key, expected) in [("geometry_factor", q), ("inductance_per_length", inductance),
        ("capacitance_per_length", capacitance),
        ("differential_impedance", (inductance / capacitance).sqrt())] {
        assert_close(out[key], expected, 1e-12, 0.0);
    }
}

#[test]
fn twin_lead_finite_ratio_survives_overflowing_dimension_sum() {
    // All outputs depend only on D/d. D+d overflows, but the geometry does not.
    for scale in [1.0, 1e308] {
        let out = calculate("cables.twin_lead_geometry", &[
            ("wire_diameter", scale), ("center_spacing", 1.5 * scale),
            ("relative_permittivity", 2.25),
        ]).unwrap();
        let q = 1.5_f64.acosh();
        let inductance = 4e-7 * q;
        let capacitance = PI * 8.8541878128e-12 * 2.25 / q;
        for (key, expected) in [("geometry_factor", q), ("inductance_per_length", inductance),
            ("capacitance_per_length", capacitance),
            ("differential_impedance", (inductance / capacitance).sqrt())] {
            assert_close(out[key], expected, 1e-12, 0.0);
        }
    }
}

#[test]
fn twin_lead_rejects_touching_reversed_and_scalar_domains_then_recovers() {
    for (key, value) in [("center_spacing", 0.0008), ("center_spacing", 0.0007),
        ("center_spacing", 0.0), ("wire_diameter", 0.0), ("wire_diameter", -1.0),
        ("relative_permittivity", 0.999)] {
        let mut inputs = TWIN;
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = value;
        assert!(calculate("cables.twin_lead_geometry", &inputs).is_err(), "{key}={value}");
        let out = calculate("cables.twin_lead_geometry", &TWIN).unwrap();
        for (key, expected) in TWIN_OUT { assert_close(out[key], expected, 1e-10, 0.0); }
    }
}

#[test]
fn rlgc_complex_roots_obey_telegrapher_identities_and_both_impedance_signs() {
    for (r, l, g, c, sign) in [(3.0, 4e-6, 0.012, 5e-9, 1.0),
        (12.0, 5e-6, 0.003, 4e-9, -1.0)] {
        let out = calculate("cables.lossy_rlgc_line", &[
            ("series_resistance_per_length", r), ("inductance_per_length", l),
            ("shunt_conductance_per_length", g), ("capacitance_per_length", c),
            ("frequency", 1e6 / TAU), ("length", 4.0),
        ]).unwrap();
        let a = out["attenuation_constant"];
        let b = out["phase_constant"];
        let zr = out["impedance_real"];
        let zi = out["impedance_imaginary"];
        assert!(sign * zi > 0.0);
        assert_close(a, RLGC_OUT[5].1, 1e-12, 0.0);
        assert_close(b, RLGC_OUT[6].1, 1e-12, 0.0);
        assert_close(a * a - b * b, 0.016, 1e-12, 0.0);
        assert_close(2.0 * a * b, 0.063, 1e-12, 0.0);
        // gamma*Z0=z and gamma/Z0=y check the selected physical root jointly.
        assert_close(a * zr - b * zi, r, 1e-12, 0.0);
        assert_close(a * zi + b * zr, 1e6 * l, 1e-12, 0.0);
        assert_close((a * zr + b * zi) / (zr * zr + zi * zi), g, 1e-12, 0.0);
        assert_close((b * zr - a * zi) / (zr * zr + zi * zi), 1e6 * c, 1e-12, 0.0);
    }
}

#[test]
fn rlgc_lossless_limit_has_exact_zero_attenuation() {
    let out = calculate("cables.lossy_rlgc_line", &[
        ("series_resistance_per_length", 0.0), ("inductance_per_length", 2.5e-7),
        ("shunt_conductance_per_length", 0.0), ("capacitance_per_length", 1e-10),
        ("frequency", 1e6), ("length", 10.0),
    ]).unwrap();
    for (key, value) in [("angular_frequency", TAU * 1e6),
        ("series_impedance_magnitude", PI / 2.0),
        ("shunt_admittance_magnitude", TAU * 1e-4),
        ("normalized_propagation_square_real", -1.0),
        ("normalized_propagation_square_imaginary", 0.0),
        ("attenuation_constant", 0.0), ("phase_constant", PI / 100.0),
        ("impedance_real", 50.0), ("impedance_imaginary", 0.0),
        ("forward_amplitude_ratio", 1.0), ("attenuation_db", 0.0)] {
        assert_close(out[key], value, 1e-12, 0.0);
    }
}

#[test]
fn rlgc_zero_length_and_cascade_preserve_local_line_properties() {
    for length in [0.0, 8.0] {
        let mut inputs = RLGC;
        inputs[5].1 = length;
        let out = calculate("cables.lossy_rlgc_line", &inputs).unwrap();
        for (key, expected) in &RLGC_OUT[..9] { assert_close(out[*key], *expected, 1e-12, 0.0); }
        let ratio = if length == 0.0 { 1.0 } else { RLGC_OUT[9].1.powi(2) };
        assert_close(out["forward_amplitude_ratio"], ratio, 1e-12, 0.0);
        assert_close(out["attenuation_db"], RLGC_OUT[10].1 * length / 4.0, 1e-12, 0.0);
    }
}

#[test]
fn rlgc_heaviside_balance_is_real_and_frequency_independent() {
    for frequency in [1.0, 1e6, 1e10] {
        let out = calculate("cables.lossy_rlgc_line", &[
            ("series_resistance_per_length", 2.0), ("inductance_per_length", 2.5e-7),
            ("shunt_conductance_per_length", 0.0008), ("capacitance_per_length", 1e-10),
            ("frequency", frequency), ("length", 10.0),
        ]).unwrap();
        assert_close(out["attenuation_constant"], 0.04, 1e-12, 0.0);
        assert_close(out["phase_constant"], TAU * frequency * 5e-9, 1e-12, 0.0);
        assert_close(out["impedance_real"], 50.0, 1e-12, 0.0);
        // Difference of normalized products: allow 32 ulps on the 50-ohm scale.
        assert_close(out["impedance_imaginary"], 0.0, 0.0, 32.0 * f64::EPSILON * 50.0);
        assert_close(out["forward_amplitude_ratio"], (-0.4_f64).exp(), 1e-12, 0.0);
        assert_close(out["attenuation_db"], 8.0 / LN_10, 1e-12, 0.0);
    }
}

#[test]
fn rlgc_near_dc_keeps_tiny_phase_and_signed_impedance() {
    let out = calculate("cables.lossy_rlgc_line", &[
        ("series_resistance_per_length", 2.0), ("inductance_per_length", 2.5e-7),
        ("shunt_conductance_per_length", 2e-5), ("capacitance_per_length", 1e-10),
        ("frequency", 1e-12), ("length", 10.0),
    ]).unwrap();
    // Taylor expansion around DC; omega*max(L/R,C/G)<4e-17, error O(1e-33).
    let omega = TAU * 1e-12;
    let alpha = (4e-5_f64).sqrt();
    let impedance = (1e5_f64).sqrt();
    for (key, expected) in [("angular_frequency", omega),
        ("series_impedance_magnitude", 2.0), ("shunt_admittance_magnitude", 2e-5),
        ("normalized_propagation_square_real", 1.0),
        ("normalized_propagation_square_imaginary", omega * (1.25e-7 + 5e-6)),
        ("attenuation_constant", alpha),
        ("phase_constant", alpha * omega * (1.25e-7 + 5e-6) / 2.0),
        ("impedance_real", impedance),
        ("impedance_imaginary", impedance * omega * (1.25e-7 - 5e-6) / 2.0),
        ("forward_amplitude_ratio", (-10.0 * alpha).exp()),
        ("attenuation_db", 200.0 * alpha / LN_10)] {
        assert_close(out[key], expected, 1e-12, 0.0);
    }
}

#[test]
fn rlgc_arbitrarily_small_loss_is_not_rounded_to_lossless() {
    let out = calculate("cables.lossy_rlgc_line", &[
        ("series_resistance_per_length", 1e-24), ("inductance_per_length", 1.0),
        ("shunt_conductance_per_length", 0.0), ("capacitance_per_length", 1.0),
        ("frequency", 1.0 / TAU), ("length", 1.0),
    ]).unwrap();
    // sqrt(-1+jR)=R/2+j+O(R^2); sqrt(1-jR)=1-jR/2+O(R^2).
    assert_close(out["attenuation_constant"], 5e-25, 1e-12, 0.0);
    assert_close(out["phase_constant"], 1.0, 1e-12, 0.0);
    assert_close(out["impedance_real"], 1.0, 1e-12, 0.0);
    assert_close(out["impedance_imaginary"], -5e-25, 1e-12, 0.0);
    assert_close(out["attenuation_db"], 1e-23 / LN_10, 1e-12, 0.0);
}

#[test]
fn rlgc_rejects_each_scalar_domain_and_recovers_all_outputs() {
    for (key, bad) in [("series_resistance_per_length", -1.0),
        ("inductance_per_length", 0.0), ("shunt_conductance_per_length", -1.0),
        ("capacitance_per_length", 0.0), ("frequency", 0.0), ("length", -1.0)] {
        let mut inputs = RLGC;
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = bad;
        assert!(calculate("cables.lossy_rlgc_line", &inputs).is_err(), "{key}={bad}");
        let out = calculate("cables.lossy_rlgc_line", &RLGC).unwrap();
        for (key, expected) in RLGC_OUT { assert_close(out[key], expected, 1e-10, 0.0); }
    }
}

#[test]
fn radial_zero_voltage_negative_margin_and_geometric_scaling() {
    for scale in [1.0, 2.0] {
        for voltage in [0.0, 1800.0, 6000.0] {
            let out = calculate("cables.radial_insulation_stress", &[
                ("conductor_radius", 0.002 * scale), ("shield_inner_radius", 0.006 * scale),
                ("peak_voltage", voltage), ("allowable_field", 2e6),
            ]).unwrap();
            let inner = RADIAL_OUT[0].1 * voltage / 1800.0 / scale;
            assert_close(out["inner_surface_field"], inner, 1e-12, 0.0);
            assert_close(out["outer_surface_field"], inner / 3.0, 1e-12, 0.0);
            assert_close(out["voltage_limit"], RADIAL_OUT[2].1 * scale, 1e-12, 0.0);
            assert_close(out["field_margin"], 2e6 - inner, 1e-12, 0.0);
            if voltage == 6000.0 && scale == 1.0 { assert!(out["field_margin"] < 0.0); }
            // Integrating E(a)*a/r from a to b recovers the applied peak voltage.
            assert_close(out["inner_surface_field"] * 0.002 * scale * 3.0_f64.ln(), voltage, 1e-12, 0.0);
        }
    }
}

#[test]
fn radial_thin_gap_converges_to_planar_field_without_cancellation() {
    let gap = 2.0_f64.powi(-30);
    let out = calculate("cables.radial_insulation_stress", &[
        ("conductor_radius", 1.0), ("shield_inner_radius", 1.0 + gap),
        ("peak_voltage", gap), ("allowable_field", 2.0),
    ]).unwrap();
    // u/ln(1+u)=1+u/2-u^2/12+O(u^3); omitted terms below 1e-28.
    let inner = 1.0 + gap / 2.0 - gap * gap / 12.0;
    assert_close(out["inner_surface_field"], inner, 1e-12, 0.0);
    assert_close(out["outer_surface_field"], inner / (1.0 + gap), 1e-12, 0.0);
    assert_close(out["voltage_limit"], 2.0 * (gap - gap * gap / 2.0), 1e-12, 0.0);
    assert_close(out["field_margin"], 2.0 - inner, 1e-12, 0.0);
}

#[test]
fn radial_rejects_reversed_gap_and_invalid_inputs_then_recovers() {
    for (key, bad) in [("conductor_radius", 0.0), ("conductor_radius", -1.0),
        ("shield_inner_radius", 0.002), ("shield_inner_radius", 0.001),
        ("peak_voltage", -1.0), ("allowable_field", 0.0), ("allowable_field", -1.0)] {
        let mut inputs = RADIAL;
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = bad;
        assert!(calculate("cables.radial_insulation_stress", &inputs).is_err(), "{key}={bad}");
        let out = calculate("cables.radial_insulation_stress", &RADIAL).unwrap();
        for (key, expected) in RADIAL_OUT { assert_close(out[key], expected, 1e-10, 0.0); }
    }
}

#[test]
fn hertz_force_cube_root_scaling_and_auxetic_poisson_symmetry() {
    for force_factor in [1.0_f64, 8.0] {
        let mut inputs = HERTZ;
        inputs[0].1 *= force_factor;
        let out = calculate("cables.hertz_holm_contact", &inputs).unwrap();
        let scale = force_factor.cbrt();
        assert_close(out["effective_modulus"], 48e9, 1e-12, 0.0);
        assert_close(out["contact_radius"], 1e-5 * scale, 1e-12, 0.0);
        assert_close(out["maximum_pressure"], HERTZ_OUT[2].1 * scale, 1e-12, 0.0);
        assert_close(out["constriction_resistance"], 0.001 / scale, 1e-12, 0.0);
        assert_close(out["indentation"], 2.5e-8 * scale * scale, 1e-12, 0.0);
        // Integrating Hertz's pressure profile p0*sqrt(1-r^2/a^2) gives F.
        assert_close(2.0 * PI * out["contact_radius"].powi(2) * out["maximum_pressure"] / 3.0,
            inputs[0].1, 1e-12, 0.0);
    }
    for nu in [-0.25_f64, 0.25] {
        let mut inputs = HERTZ;
        inputs[3].1 = nu;
        let out = calculate("cables.hertz_holm_contact", &inputs).unwrap();
        let scale = (15.0_f64 / 16.0).cbrt();
        assert_close(out["effective_modulus"], 51.2e9, 1e-12, 0.0);
        assert_close(out["contact_radius"], 1e-5 * scale, 1e-12, 0.0);
        assert_close(out["constriction_resistance"], 0.001 / scale, 1e-12, 0.0);
    }
}

#[test]
fn hertz_inclusive_screens_are_distinct_and_allow_equality() {
    // Deliberately exact cube=1: all three screens meet equality independently.
    let out = calculate("cables.hertz_holm_contact", &[
        ("normal_force", 100.0), ("sphere_radius", 10.0),
        ("young_modulus", 1500.0), ("poisson_ratio", 0.0),
        ("resistivity", 2e-8), ("electron_mean_free_path", 0.1),
        ("elastic_pressure_limit", 150.0 / PI),
    ]).unwrap();
    for (key, expected) in [("effective_modulus", 750.0), ("contact_radius", 1.0),
        ("maximum_pressure", 150.0 / PI), ("constriction_resistance", 1e-8),
        ("indentation", 0.1)] {
        assert_close(out[key], expected, 1e-12, 0.0);
    }
}

#[test]
fn hertz_small_contact_equality_must_not_fail_from_fractional_power_rounding() {
    // a^3=1e-9 exactly in the physical model; pow(a^3,1/3) rounds above 0.001.
    let out = calculate("cables.hertz_holm_contact", &[
        ("normal_force", 10000.0), ("sphere_radius", 0.01),
        ("young_modulus", 150e9), ("poisson_ratio", 0.0),
        ("resistivity", 2e-8), ("electron_mean_free_path", 1e-8),
        ("elastic_pressure_limit", 1e10),
    ]).unwrap();
    for (key, expected) in [("effective_modulus", 75e9), ("contact_radius", 0.001),
        ("maximum_pressure", 1.5e10 / PI), ("constriction_resistance", 1e-5),
        ("indentation", 1e-4)] {
        assert_close(out[key], expected, 1e-12, 0.0);
    }
}

#[test]
fn hertz_rejects_each_physical_screen_and_scalar_domain_then_recovers() {
    for (key, bad) in [("normal_force", 0.0), ("sphere_radius", 0.0),
        ("young_modulus", 0.0), ("poisson_ratio", -1.0), ("poisson_ratio", 0.5),
        ("resistivity", 0.0), ("electron_mean_free_path", 0.0),
        ("elastic_pressure_limit", 0.0), ("elastic_pressure_limit", 7e7),
        ("electron_mean_free_path", 2e-6), ("sphere_radius", 1e-5)] {
        let mut inputs = HERTZ;
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = bad;
        // Isolate the large-patch screen from the pressure and diffusive screens.
        if key == "sphere_radius" && bad > 0.0 {
            inputs[5].1 = 1e-10;
            inputs[6].1 = 1e20;
        }
        assert!(calculate("cables.hertz_holm_contact", &inputs).is_err(), "{key}={bad}");
        let out = calculate("cables.hertz_holm_contact", &HERTZ).unwrap();
        for (key, expected) in HERTZ_OUT { assert_close(out[key], expected, 1e-10, 0.0); }
    }
}

#[test]
fn hotspot_zero_drop_and_equal_temperature_limit_are_valid() {
    for limit in [300.0_f64, 600.0] {
        let mut inputs = HOTSPOT;
        inputs[0].1 = 0.0;
        inputs[3].1 = limit;
        let out = calculate("cables.contact_hotspot_temperature", &inputs).unwrap();
        assert_close(out["hotspot_temperature"], 300.0, 1e-12, 0.0);
        assert_close(out["temperature_rise"], 0.0, 0.0, 0.0);
        assert_close(out["contact_voltage_limit"], (limit * limit - 90000.0).sqrt() / 4096.0, 1e-12, 0.0);
        assert_close(out["temperature_margin"], limit - 300.0, 1e-12, 0.0);
    }
}

#[test]
fn hotspot_sub_ulp_heating_retains_positive_quadratic_temperature_rise() {
    for voltage in [1e-6_f64, 1e-10] {
        let out = calculate("cables.contact_hotspot_temperature", &[
            ("contact_voltage", voltage), ("bulk_temperature", 300.0),
            ("lorenz_number", 2.45e-8), ("temperature_limit", 600.0),
        ]).unwrap();
        // Binomial series in x=Vc^2/(4*Lorenz*Tbulk^2), not Tm-Tbulk subtraction.
        let x = voltage * voltage / (4.0 * 2.45e-8 * 300.0 * 300.0);
        let rise = 300.0 * (x / 2.0 - x * x / 8.0 + x * x * x / 16.0);
        assert_close(out["temperature_rise"], rise, 1e-12, 0.0);
        assert_close(out["hotspot_temperature"], 300.0 + rise, 1e-12, 0.0);
        assert_close(out["temperature_margin"], 300.0 - rise, 1e-12, 0.0);
        assert_close(out["contact_voltage_limit"], 0.16266530054071152, 1e-12, 0.0);
    }
}

#[test]
fn hotspot_exact_thermal_limit_and_lorenz_scaling() {
    for scale in [1.0, 4.0] {
        let mut inputs = HOTSPOT;
        inputs[0].1 *= scale;
        inputs[2].1 *= scale * scale;
        inputs[3].1 = 500.0;
        let out = calculate("cables.contact_hotspot_temperature", &inputs).unwrap();
        assert_close(out["hotspot_temperature"], 500.0, 1e-12, 0.0);
        assert_close(out["temperature_rise"], 200.0, 1e-12, 0.0);
        assert_close(out["contact_voltage_limit"], 0.09765625 * scale, 1e-12, 0.0);
        assert_close(out["temperature_margin"], 0.0, 0.0, 0.0);
    }
}

#[test]
fn hotspot_rejects_scalar_and_coupled_limits_then_recovers() {
    for (key, bad) in [("contact_voltage", -1.0), ("contact_voltage", 0.2),
        ("bulk_temperature", 0.0), ("lorenz_number", 0.0),
        ("temperature_limit", 0.0), ("temperature_limit", 299.0),
        ("temperature_limit", 499.0)] {
        let mut inputs = HOTSPOT;
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = bad;
        assert!(calculate("cables.contact_hotspot_temperature", &inputs).is_err(), "{key}={bad}");
        let out = calculate("cables.contact_hotspot_temperature", &HOTSPOT).unwrap();
        for (key, expected) in HOTSPOT_OUT { assert_close(out[key], expected, 1e-10, 0.0); }
    }
}

#[test]
fn hotspot_equal_bulk_and_limit_reject_any_positive_contact_drop() {
    // V2CablesRun counterexample: rounded hypot=300 cannot authorize positive heat.
    let mut inputs = [
        ("contact_voltage", 1e-10), ("bulk_temperature", 300.0),
        ("lorenz_number", 2.45e-8), ("temperature_limit", 300.0),
    ];
    assert!(calculate("cables.contact_hotspot_temperature", &inputs).is_err());
    inputs[0].1 = 0.0;
    let out = calculate("cables.contact_hotspot_temperature", &inputs).unwrap();
    for (key, expected) in [("hotspot_temperature", 300.0), ("temperature_rise", 0.0),
        ("contact_voltage_limit", 0.0), ("temperature_margin", 0.0)] {
        assert_close(out[key], expected, 1e-12, 0.0);
    }
}

#[test]
fn beam_width_thickness_length_scaling_follows_bending_energy() {
    for (width_scale, thickness_scale, length_scale) in [(2.0, 1.0, 1.0),
        (1.0, 2.0, 1.0), (1.0, 1.0, 2.0)] {
        let mut inputs = BEAM;
        inputs[0].1 *= length_scale;
        inputs[1].1 *= width_scale;
        inputs[2].1 *= thickness_scale;
        inputs[5].1 = 1e9;
        let out = calculate("cables.cantilever_contact_spring", &inputs).unwrap();
        let (length, width, thickness) = (inputs[0].1, inputs[1].1, inputs[2].1);
        let second_moment = width * thickness.powi(3) / 12.0;
        let stiffness = 3.0 * 120e9 * second_moment / length.powi(3);
        let force = stiffness * 0.00064;
        let stress = force * length * thickness / (2.0 * second_moment);
        assert_close(out["spring_rate"], stiffness, 1e-12, 0.0);
        assert_close(out["normal_force"], force, 1e-12, 0.0);
        assert_close(out["root_stress"], stress, 1e-12, 0.0);
        assert_close(out["stress_margin"], 1e9 - stress, 1e-12, 0.0);
    }
}

#[test]
fn beam_zero_travel_and_each_inclusive_physical_boundary() {
    for (thickness, travel, limit, stiffness, force, stress) in [
        (0.0004, 0.0, 360e6, 1406.25, 0.0, 0.0),
        (0.0016, 0.0, 360e6, 90000.0, 0.0, 0.0),
        (0.0004, 0.0016, 500e6, 1406.25, 2.25, 450e6),
        (0.0004, 0.00064, 180e6, 1406.25, 0.9, 180e6),
    ] {
        let mut inputs = BEAM;
        inputs[2].1 = thickness;
        inputs[4].1 = travel;
        inputs[5].1 = limit;
        let out = calculate("cables.cantilever_contact_spring", &inputs).unwrap();
        for (key, expected) in [("spring_rate", stiffness), ("normal_force", force),
            ("root_stress", stress), ("stress_margin", limit - stress)] {
            assert_close(out[key], expected, 1e-12, 0.0);
        }
    }
}

#[test]
fn beam_rejects_geometry_travel_stress_and_scalar_domains_then_recovers() {
    for (key, bad) in [("beam_length", 0.0), ("beam_width", 0.0),
        ("beam_thickness", 0.0), ("young_modulus", 0.0),
        ("tip_deflection", -1e-6), ("allowable_stress", 0.0),
        ("beam_thickness", 0.0016001), ("tip_deflection", 0.0016001),
        ("allowable_stress", 179e6)] {
        let mut inputs = BEAM;
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = bad;
        if key == "beam_thickness" && bad > 0.0 { inputs[4].1 = 0.0; }
        if key == "tip_deflection" && bad > 0.0 { inputs[5].1 = 1e9; }
        assert!(calculate("cables.cantilever_contact_spring", &inputs).is_err(), "{key}={bad}");
        let out = calculate("cables.cantilever_contact_spring", &BEAM).unwrap();
        for (key, expected) in BEAM_OUT { assert_close(out[key], expected, 1e-10, 0.0); }
    }
}

#[test]
fn every_input_rejects_nonfinite_values_and_recovers_complete_outputs() {
    for &(id, valid, expected) in CASES {
        for index in 0..valid.len() {
            for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                let mut inputs = valid.to_vec();
                inputs[index].1 = bad;
                assert!(calculate(id, &inputs).is_err(), "{id}: {}={bad}", inputs[index].0);
                let out = calculate(id, valid).unwrap();
                assert_eq!(out.len(), expected.len());
                for &(key, value) in expected { assert_close(out[key], value, 1e-10, 0.0); }
            }
        }
    }
}
