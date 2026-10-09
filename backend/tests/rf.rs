mod common;

use common::{assert_close, calculate};
use std::f64::consts::PI;

const C: f64 = 299792458.0;
const PAD: &[(&str, f64)] = &[("reference_impedance", 75.0), ("attenuation_db", 9.542425094393248)];
const MATCH: &[(&str, f64)] = &[
    ("source_resistance", 20.0), ("load_resistance", 100.0), ("frequency", 2e6),
];
const LINK: &[(&str, f64)] = &[
    ("frequency", C), ("distance", 100.0), ("antenna_max_dimension", 1.0),
    ("transmit_power_dbm", 0.0), ("transmit_gain_dbi", -3.0), ("receive_gain_dbi", 5.0),
    ("total_loss_db", 2.0), ("sensitivity_dbm", -60.0),
];
const FRESNEL: &[(&str, f64)] = &[
    ("frequency", C), ("distance_from_tx", 200.0), ("distance_to_rx", 800.0),
    ("clearance_fraction", 0.5), ("available_clearance", -1.0),
];
const APERTURE: &[(&str, f64)] = &[
    ("diameter", 4.0), ("frequency", C), ("aperture_efficiency", 0.25),
];
const IM3: &[(&str, f64)] = &[
    ("lower_frequency", 40e6), ("upper_frequency", 45e6), ("input_tone_power_dbm", -30.0),
    ("gain_db", -5.0), ("input_intercept_dbm", -10.0), ("maximum_total_linear_output_dbm", -30.0),
];
const RADAR: &[(&str, f64)] = &[
    ("frequency", C), ("range", 100.0), ("antenna_max_dimension", 1.0),
    ("target_max_dimension", 2.0), ("transmit_power_dbm", 30.0), ("antenna_gain_dbi", 10.0),
    ("radar_cross_section", 4.0), ("total_loss_db", 0.0), ("receiver_threshold_dbm", -70.0),
];

#[test]
fn pad_nondefault_nodal_match_and_power() {
    // K=3 gives a 75-ohm matched network; solve its terminated nodes independently.
    let r = calculate("rf.matched_t_attenuator", PAD).unwrap();
    assert_eq!(r.len(), 3);
    assert_close(r["series_resistance"], 37.5, 1e-12, 0.0);
    assert_close(r["shunt_resistance"], 56.25, 1e-12, 0.0);
    assert_close(r["power_transmission"], 1.0 / 9.0, 1e-12, 0.0);
    let series = r["series_resistance"];
    let shunt = r["shunt_resistance"];
    let parallel = 1.0 / (1.0 / shunt + 1.0 / (series + 75.0));
    let zin = series + parallel;
    let vout = parallel / zin * 75.0 / (series + 75.0);
    assert_close(zin, 75.0, 1e-12, 0.0);
    assert_close(vout, 1.0 / 3.0, 1e-12, 0.0);
    assert_close(vout.powi(2) * zin / 75.0, r["power_transmission"], 1e-12, 0.0);
}

#[test]
fn pad_tiny_loss_retains_resistors() {
    let r = calculate("rf.matched_t_attenuator", &[
        ("reference_impedance", 75.0), ("attenuation_db", 1e-12),
    ]).unwrap();
    // tanh(a/2) ~ a/2 and 1/sinh(a) ~ 1/a; omitted relative terms are below 1e-26.
    let a: f64 = 1e-12 * 10.0_f64.ln() / 20.0;
    assert_close(r["series_resistance"], 75.0 * a / 2.0, 1e-12, 0.0);
    assert_close(r["shunt_resistance"], 75.0 / a, 1e-12, 0.0);
    assert_close(r["power_transmission"], 1.0 - 2.0 * a, 1e-15, 0.0);
}

#[test]
fn pad_impedance_scaling_and_high_loss() {
    for (z, loss, series, shunt, power) in [
        (150.0, PAD[1].1, 75.0, 112.5, 1.0 / 9.0),
        (75.0, 400.0, 75.0, 1.5e-18, 1e-40),
    ] {
        let r = calculate("rf.matched_t_attenuator", &[("reference_impedance", z), ("attenuation_db", loss)]).unwrap();
        assert_close(r["series_resistance"], series, 1e-12, 0.0);
        assert_close(r["shunt_resistance"], shunt, 1e-12, 0.0);
        assert_close(r["power_transmission"], power, 1e-12, 0.0);
    }
}

#[test]
fn l_match_nondefault_complex_input_impedance() {
    let r = calculate("rf.lowpass_l_match", MATCH).unwrap();
    assert_eq!(r.len(), 3);
    assert_close(r["loaded_q"], 2.0, 1e-12, 0.0);
    assert_close(r["series_inductance"], 1e-5 / PI, 1e-12, 0.0);
    assert_close(r["shunt_capacitance"], 5e-9 / PI, 1e-12, 0.0);
    let omega = 4e6 * PI;
    let g: f64 = 0.01;
    let b = omega * r["shunt_capacitance"];
    let real = g / (g * g + b * b);
    let imaginary = omega * r["series_inductance"] - b / (g * g + b * b);
    assert_close(real, 20.0, 1e-12, 0.0);
    // Cancellation of 40-ohm reactances allows 1e-12 ohm roundoff.
    assert_close(imaginary, 0.0, 0.0, 1e-12);
}

#[test]
fn l_match_frequency_scaling_and_adjacent_resistances() {
    let mut v = MATCH.to_vec();
    v[2].1 *= 2.0;
    let r = calculate("rf.lowpass_l_match", &v).unwrap();
    assert_close(r["loaded_q"], 2.0, 1e-12, 0.0);
    assert_close(r["series_inductance"], 5e-6 / PI, 1e-12, 0.0);
    assert_close(r["shunt_capacitance"], 2.5e-9 / PI, 1e-12, 0.0);
    v[0].1 = 64.0;
    v[1].1 = f64::from_bits(64.0_f64.to_bits() + 1);
    let q: f64 = 2.0_f64.powi(-26) ; // sqrt(one binary64 relative ulp).
    let r = calculate("rf.lowpass_l_match", &v).unwrap();
    assert_close(r["loaded_q"], q, 1e-12, 0.0);
    assert_close(r["series_inductance"], 64.0 * q / (8e6 * PI), 1e-12, 0.0);
    assert_close(r["shunt_capacitance"], q / (8e6 * PI) / v[1].1, 1e-12, 0.0);
    v[1].1 = 64.0;
    assert!(calculate("rf.lowpass_l_match", &v).is_err());
    v[1].1 = 63.0;
    assert!(calculate("rf.lowpass_l_match", &v).is_err());
    let r = calculate("rf.lowpass_l_match", MATCH).unwrap();
    assert_close(r["loaded_q"], 2.0, 1e-12, 0.0);
    assert_close(r["series_inductance"], 1e-5 / PI, 1e-12, 0.0);
    assert_close(r["shunt_capacitance"], 5e-9 / PI, 1e-12, 0.0);
}

#[test]
fn l_match_representable_capacitance_despite_large_denominator() {
    // Q=1. Both reactances and final components are representable; f*Rl is not.
    let r = calculate("rf.lowpass_l_match", &[
        ("source_resistance", 1e100), ("load_resistance", 2e100), ("frequency", 1e210),
    ]).expect("finite L and subnormal C must not fail from denominator overflow");
    assert_close(r["loaded_q"], 1.0, 1e-12, 0.0);
    assert_close(r["series_inductance"], 1e-110 / (2.0 * PI), 1e-12, 0.0);
    assert_close(r["shunt_capacitance"], 1e-310 / (4.0 * PI), 1e-10, 0.0);
}

#[test]
fn l_match_representable_q_despite_resistance_ratio_overflow() {
    // sqrt(1e400)=1e200 is finite; the matching reactance is exactly one ohm.
    let r = calculate("rf.lowpass_l_match", &[
        ("source_resistance", 1e-200), ("load_resistance", 1e200), ("frequency", 1e8),
    ]).expect("finite Q must not fail from squaring its representable magnitude");
    assert_close(r["loaded_q"], 1e200, 1e-12, 0.0);
    assert_close(r["series_inductance"], 1.0 / (2e8 * PI), 1e-12, 0.0);
    assert_close(r["shunt_capacitance"], 1.0 / (2e8 * PI), 1e-12, 0.0);
}

#[test]
fn l_match_representable_components_despite_angular_frequency_overflow() {
    let r = calculate("rf.lowpass_l_match", &[
        ("source_resistance", 50.0), ("load_resistance", 200.0), ("frequency", 1e308),
    ]).expect("finite components must not fail because 2*pi*f overflows");
    let q: f64 = 3.0_f64.sqrt();
    assert_close(r["loaded_q"], q, 1e-12, 0.0);
    assert_close(r["series_inductance"], (25.0 * q / PI) * 1e-308, 1e-12, 0.0);
    assert_close(r["shunt_capacitance"], (q / (400.0 * PI)) * 1e-308, 1e-10, 0.0);
}

#[test]
fn link_nondefault_linear_power_density() {
    let r = calculate("rf.free_space_link_budget", LINK).unwrap();
    assert_eq!(r.len(), 3);
    // One-metre wavelength: unit-gain receiving aperture is 1/(4*pi) m^2.
    let spreading = 4.0 * PI * 10000.0;
    let transmission = (1.0 / (4.0 * PI)) / spreading;
    let loss = -10.0 * transmission.log10();
    assert_close(r["free_space_loss_db"], loss, 1e-12, 0.0);
    assert_close(r["received_power_dbm"], 10.0 * transmission.log10(), 1e-12, 0.0);
    assert_close(r["link_margin_db"], 60.0 - loss, 1e-12, 0.0);
}

#[test]
fn link_inverse_square_gain_loss_and_signed_margins() {
    let loss = 10.0 * (160000.0 * PI * PI).log10();
    for (index, value, receive_shift, loss_shift) in [
        (1, 200.0, -10.0 * 4.0_f64.log10(), 10.0 * 4.0_f64.log10()),
        (4, 0.0, 3.0, 0.0), (5, 8.0, 3.0, 0.0), (6, 0.0, 2.0, 0.0),
    ] {
        let mut v = LINK.to_vec();
        v[index].1 = value;
        let r = calculate("rf.free_space_link_budget", &v).unwrap();
        assert_close(r["free_space_loss_db"], loss + loss_shift, 1e-12, 0.0);
        assert_close(r["received_power_dbm"], -loss + receive_shift, 1e-12, 0.0);
        assert_close(r["link_margin_db"], 60.0 - loss + receive_shift, 1e-11, 0.0);
    }
    let mut v = LINK.to_vec();
    v[7].1 = -loss;
    let r = calculate("rf.free_space_link_budget", &v).unwrap();
    // Independent linear-power/log evaluation can differ by tens of ulps in dB.
    assert_close(r["link_margin_db"], 0.0, 0.0, 1e-12);
    v[7].1 = -loss + 5.0;
    assert_close(calculate("rf.free_space_link_budget", &v).unwrap()["link_margin_db"], -5.0, 1e-12, 0.0);
}

#[test]
fn link_both_far_field_boundaries_and_recovery() {
    for (dimension, edge) in [(1.0, 10.0_f64), (4.0, 32.0)] {
        let mut v = LINK.to_vec();
        v[2].1 = dimension;
        v[1].1 = f64::from_bits(edge.to_bits() - 1);
        assert!(calculate("rf.free_space_link_budget", &v).is_err());
        v[1].1 = edge;
        let r = calculate("rf.free_space_link_budget", &v).unwrap();
        let loss = 10.0 * (4.0 * PI * edge).powi(2).log10();
        assert_close(r["free_space_loss_db"], loss, 1e-12, 0.0);
        assert_close(r["received_power_dbm"], -loss, 1e-12, 0.0);
        assert_close(r["link_margin_db"], 60.0 - loss, 1e-12, 0.0);
    }
}

#[test]
fn fresnel_nondefault_half_wavelength_path_excess() {
    let r = calculate("rf.first_fresnel_clearance", FRESNEL).unwrap();
    assert_eq!(r.len(), 3);
    assert_close(r["zone_radius"], 160.0_f64.sqrt(), 1e-12, 0.0);
    assert_close(r["required_clearance"], 40.0_f64.sqrt(), 1e-12, 0.0);
    assert_close(r["clearance_margin"], -1.0 - 40.0_f64.sqrt(), 1e-12, 0.0);
    let excess = r["zone_radius"].powi(2) / 400.0 + r["zone_radius"].powi(2) / 1600.0;
    assert_close(excess, 0.5, 1e-12, 0.0);
}

#[test]
fn fresnel_symmetry_midpoint_frequency_and_obstacle() {
    for (d1, d2, frequency, clearance, radius) in [
        (800.0, 200.0, C, -1.0, 160.0_f64.sqrt()),
        (500.0, 500.0, C, -1.0, 250.0_f64.sqrt()),
        (200.0, 800.0, 4.0 * C, -1.0, 40.0_f64.sqrt()),
        (200.0, 800.0, C, 0.0, 160.0_f64.sqrt()),
        (200.0, 800.0, C, 40.0_f64.sqrt(), 160.0_f64.sqrt()),
    ] {
        let mut v = FRESNEL.to_vec();
        v[0].1 = frequency; v[1].1 = d1; v[2].1 = d2; v[4].1 = clearance;
        let r = calculate("rf.first_fresnel_clearance", &v).unwrap();
        assert_close(r["zone_radius"], radius, 1e-12, 0.0);
        assert_close(r["required_clearance"], radius / 2.0, 1e-12, 0.0);
        assert_close(r["clearance_margin"], clearance - radius / 2.0, 1e-12, 0.0);
    }
}

#[test]
fn fresnel_fraction_endpoints_screen_and_large_geometry() {
    for fraction in [0.0, 1.0] {
        let mut v = FRESNEL.to_vec(); v[3].1 = fraction;
        let r = calculate("rf.first_fresnel_clearance", &v).unwrap();
        let radius = 160.0_f64.sqrt();
        assert_close(r["zone_radius"], radius, 1e-12, 0.0);
        assert_close(r["required_clearance"], fraction * radius, 1e-12, 0.0);
        assert_close(r["clearance_margin"], -1.0 - fraction * radius, 1e-12, 0.0);
    }
    for index in [1, 2] {
        let mut v = FRESNEL.to_vec(); v[index].1 = f64::from_bits(100.0_f64.to_bits() - 1);
        assert!(calculate("rf.first_fresnel_clearance", &v).is_err());
        v[index].1 = 100.0;
        let radius = (v[1].1 * v[2].1 / (v[1].1 + v[2].1)).sqrt();
        let r = calculate("rf.first_fresnel_clearance", &v).unwrap();
        assert_close(r["zone_radius"], radius, 1e-12, 0.0);
        assert_close(r["required_clearance"], radius / 2.0, 1e-12, 0.0);
        assert_close(r["clearance_margin"], -1.0 - radius / 2.0, 1e-12, 0.0);
    }
    let mut v = FRESNEL.to_vec(); v[1].1 = 1e300; v[2].1 = 1e300;
    let r = calculate("rf.first_fresnel_clearance", &v).unwrap();
    let radius = 5e299_f64.sqrt();
    assert_close(r["zone_radius"], radius, 1e-12, 0.0);
    assert_close(r["required_clearance"], radius / 2.0, 1e-12, 0.0);
    // The -1 m obstruction is far below one ulp of radius/2, so the true -1-r/2
    // margin is exactly representable as -radius/2.
    assert_close(r["clearance_margin"], -radius / 2.0, 1e-12, 0.0);
}

#[test]
fn aperture_nondefault_area_reciprocity() {
    let r = calculate("rf.circular_aperture_gain", APERTURE).unwrap();
    assert_eq!(r.len(), 3);
    let area = 0.25 * PI * 2.0_f64.powi(2);
    let gain = 4.0 * PI * area;
    assert_close(r["effective_aperture"], area, 1e-12, 0.0);
    assert_close(r["gain_linear"], gain, 1e-12, 0.0);
    assert_close(r["gain_dbi"], 10.0 * gain.log10(), 1e-12, 0.0);
}

#[test]
fn aperture_diameter_frequency_efficiency_scaling() {
    for (index, value, area_factor, gain_factor) in [
        (0, 8.0, 4.0, 4.0), (1, 2.0 * C, 1.0, 4.0),
        (2, 0.125, 0.5, 0.5), (2, 1.0, 4.0, 4.0), (2, 1e-300, 4e-300, 4e-300),
    ] {
        let mut v = APERTURE.to_vec(); v[index].1 = value;
        let r = calculate("rf.circular_aperture_gain", &v).unwrap();
        let gain = 4.0 * PI * PI * gain_factor;
        assert_close(r["effective_aperture"], PI * area_factor, 1e-12, 0.0);
        assert_close(r["gain_linear"], gain, 1e-12, 0.0);
        assert_close(r["gain_dbi"], 10.0 * gain.log10(), 1e-12, 0.0);
    }
}

#[test]
fn aperture_strict_two_wavelength_boundary_and_recovery() {
    let mut v = APERTURE.to_vec(); v[0].1 = 2.0;
    assert!(calculate("rf.circular_aperture_gain", &v).is_err());
    v[0].1 = f64::from_bits(2.0_f64.to_bits() + 1);
    let r = calculate("rf.circular_aperture_gain", &v).unwrap();
    let area = PI * (v[0].1 / 2.0).powi(2) / 4.0;
    let gain = 4.0 * PI * area;
    assert_close(r["effective_aperture"], area, 1e-12, 0.0);
    assert_close(r["gain_linear"], gain, 1e-12, 0.0);
    assert_close(r["gain_dbi"], 10.0 * gain.log10(), 1e-12, 0.0);
}

#[test]
fn aperture_representable_gain_despite_diameter_frequency_overflow() {
    let r = calculate("rf.circular_aperture_gain", &[
        ("diameter", 1e100), ("frequency", 1e210), ("aperture_efficiency", 1e-300),
    ]).expect("finite effective area and gain must survive an avoidable D*f overflow");
    let area = PI / 4.0 * 1e-100;
    let gain = (PI / C).powi(2) * 1e300 * 1e20;
    assert_close(r["effective_aperture"], area, 1e-12, 0.0);
    assert_close(r["gain_linear"], gain, 1e-12, 0.0);
    assert_close(r["gain_dbi"], 10.0 * gain.log10(), 1e-12, 0.0);
}

#[test]
fn im3_nondefault_linear_power_and_mixing() {
    let r = calculate("rf.two_tone_im3", IM3).unwrap();
    assert_eq!(r.len(), 7);
    assert_close(r["lower_product_frequency"], 35e6, 0.0, 0.0);
    assert_close(r["upper_product_frequency"], 50e6, 0.0, 0.0);
    assert_close(r["output_tone_power_dbm"], -35.0, 1e-12, 0.0);
    assert_close(r["output_intercept_dbm"], -15.0, 1e-12, 0.0);
    // Pin/IIP3 = 0.01 in linear power; IM3/carrier is its square.
    let tone_mw = 10.0_f64.powf(-3.5);
    assert_close(r["product_power_dbm"], 10.0 * (tone_mw * 0.0001).log10(), 1e-12, 0.0);
    assert_close(r["carrier_to_product_db"], 40.0, 1e-12, 0.0);
    assert_close(r["total_fundamental_power_dbm"], 10.0 * (2.0 * tone_mw).log10(), 1e-12, 0.0);
}

#[test]
fn im3_cubic_slopes_intercept_and_low_power() {
    for (pin, intercept, tone, product, separation, oip3) in [
        (-29.0, -10.0, -34.0, -72.0, 38.0, -15.0),
        (-30.0, -7.0, -35.0, -81.0, 46.0, -12.0),
        (-1000.0, -10.0, -1005.0, -2985.0, 1980.0, -15.0),
    ] {
        let mut v = IM3.to_vec(); v[2].1 = pin; v[4].1 = intercept;
        let r = calculate("rf.two_tone_im3", &v).unwrap();
        for (key, expected) in [
            ("lower_product_frequency", 35e6), ("upper_product_frequency", 50e6),
            ("output_tone_power_dbm", tone), ("output_intercept_dbm", oip3),
            ("product_power_dbm", product), ("carrier_to_product_db", separation),
            ("total_fundamental_power_dbm", tone + 10.0 * 2.0_f64.log10()),
        ] { assert_close(r[key], expected, 1e-12, 0.0); }
    }
}

#[test]
fn im3_relational_guards_and_exact_total_limit_recovery() {
    let total = -35.0 + 10.0 * 2.0_f64.log10();
    for (index, bad) in [
        (1, 40e6), (1, 39e6), (1, 80e6), (1, 81e6),
        (2, -10.0), (2, -9.0), (5, -34.0),
        (5, f64::from_bits(total.to_bits() + 1)),
    ] {
        let mut v = IM3.to_vec(); v[index].1 = bad;
        assert!(calculate("rf.two_tone_im3", &v).is_err());
        let mut valid = IM3.to_vec(); valid[5].1 = total;
        let r = calculate("rf.two_tone_im3", &valid).unwrap();
        for (key, expected) in [
            ("lower_product_frequency", 35e6), ("upper_product_frequency", 50e6),
            ("output_tone_power_dbm", -35.0), ("output_intercept_dbm", -15.0),
            ("product_power_dbm", -75.0), ("carrier_to_product_db", 40.0),
            ("total_fundamental_power_dbm", total),
        ] { assert_close(r[key], expected, 1e-12, 0.0); }
    }
    let mut v = IM3.to_vec(); v[1].1 = f64::from_bits(80e6_f64.to_bits() - 1);
    let r = calculate("rf.two_tone_im3", &v).unwrap();
    assert_close(r["lower_product_frequency"], 2.0_f64.powi(-26), 0.0, 0.0);
    assert_close(r["upper_product_frequency"], 2.0 * v[1].1 - 40e6, 1e-15, 0.0);
}

#[test]
fn radar_nondefault_two_spreading_paths_and_aperture() {
    let r = calculate("rf.monostatic_radar_return", RADAR).unwrap();
    assert_eq!(r.len(), 2);
    let density_out = 10.0 / (4.0 * PI * 10000.0);
    let density_back = density_out * 4.0 / (4.0 * PI * 10000.0);
    let area = 10.0 / (4.0 * PI);
    let dbm = 10.0 * (density_back * area / 0.001).log10();
    assert_close(r["received_power_dbm"], dbm, 1e-12, 0.0);
    assert_close(r["echo_margin_db"], dbm + 70.0, 1e-12, 0.0);
}

#[test]
fn radar_fourth_power_range_gain_rcs_loss_and_margins() {
    let base = 10.0 * (400000.0 / ((4.0 * PI).powi(3) * 1e8)).log10();
    for (index, value, shift) in [
        (1, 200.0, -10.0 * 16.0_f64.log10()),
        (5, 11.0, 2.0), (5, -10.0, -40.0),
        (6, 8.0, 10.0 * 2.0_f64.log10()), (7, 1.0, -1.0),
        (6, 4e-300, -3000.0),
    ] {
        let mut v = RADAR.to_vec(); v[index].1 = value;
        let r = calculate("rf.monostatic_radar_return", &v).unwrap();
        assert_close(r["received_power_dbm"], base + shift, 1e-12, 0.0);
        assert_close(r["echo_margin_db"], base + shift + 70.0, 1e-12, 0.0);
    }
    let mut v = RADAR.to_vec(); v[8].1 = base;
    assert_close(calculate("rf.monostatic_radar_return", &v).unwrap()["echo_margin_db"], 0.0, 0.0, 1e-12);
    v[8].1 = base + 7.0;
    assert_close(calculate("rf.monostatic_radar_return", &v).unwrap()["echo_margin_db"], -7.0, 1e-12, 0.0);
}

#[test]
fn radar_far_field_uses_larger_target_or_antenna_and_recovers() {
    for (antenna, target, edge) in [(1.0, 2.0, 10.0_f64), (1.0, 4.0, 32.0), (4.0, 1.0, 32.0)] {
        let mut v = RADAR.to_vec(); v[2].1 = antenna; v[3].1 = target;
        v[1].1 = f64::from_bits(edge.to_bits() - 1);
        assert!(calculate("rf.monostatic_radar_return", &v).is_err());
        v[1].1 = edge;
        let r = calculate("rf.monostatic_radar_return", &v).unwrap();
        let density = 10.0 / (4.0 * PI * edge * edge);
        let received_w = density * 4.0 / (4.0 * PI * edge * edge) * 10.0 / (4.0 * PI);
        let dbm = 10.0 * (received_w / 0.001).log10();
        assert_close(r["received_power_dbm"], dbm, 1e-12, 0.0);
        assert_close(r["echo_margin_db"], dbm + 70.0, 1e-12, 0.0);
    }
}

#[test]
fn all_scalar_domains_nonfinite_inputs_and_complete_recovery() {
    let link_loss = 10.0 * (160000.0 * PI * PI).log10();
    let radar_dbm = 10.0 * (400000.0 / ((4.0 * PI).powi(3) * 1e8)).log10();
    let cases: &[(&str, &[(&str, f64)], &[&str], &[(&str, f64)])] = &[
        ("rf.matched_t_attenuator", PAD, &["reference_impedance", "attenuation_db"], &[
            ("series_resistance", 37.5), ("shunt_resistance", 56.25), ("power_transmission", 1.0 / 9.0),
        ]),
        ("rf.lowpass_l_match", MATCH, &["source_resistance", "load_resistance", "frequency"], &[
            ("loaded_q", 2.0), ("series_inductance", 1e-5 / PI), ("shunt_capacitance", 5e-9 / PI),
        ]),
        ("rf.free_space_link_budget", LINK, &["frequency", "distance", "antenna_max_dimension"], &[
            ("free_space_loss_db", link_loss), ("received_power_dbm", -link_loss), ("link_margin_db", 60.0 - link_loss),
        ]),
        ("rf.first_fresnel_clearance", FRESNEL, &["frequency", "distance_from_tx", "distance_to_rx"], &[
            ("zone_radius", 160.0_f64.sqrt()), ("required_clearance", 40.0_f64.sqrt()),
            ("clearance_margin", -1.0 - 40.0_f64.sqrt()),
        ]),
        ("rf.circular_aperture_gain", APERTURE, &["diameter", "frequency", "aperture_efficiency"], &[
            ("effective_aperture", PI), ("gain_linear", 4.0 * PI * PI), ("gain_dbi", 10.0 * (4.0 * PI * PI).log10()),
        ]),
        ("rf.two_tone_im3", IM3, &["lower_frequency", "upper_frequency"], &[
            ("lower_product_frequency", 35e6), ("upper_product_frequency", 50e6),
            ("output_tone_power_dbm", -35.0), ("output_intercept_dbm", -15.0),
            ("product_power_dbm", -75.0), ("carrier_to_product_db", 40.0),
            ("total_fundamental_power_dbm", -35.0 + 10.0 * 2.0_f64.log10()),
        ]),
        ("rf.monostatic_radar_return", RADAR, &["frequency", "range", "antenna_max_dimension", "target_max_dimension", "radar_cross_section"], &[
            ("received_power_dbm", radar_dbm), ("echo_margin_db", radar_dbm + 70.0),
        ]),
    ];
    for &(id, valid, positive, expected) in cases {
        for (index, &(key, _)) in valid.iter().enumerate() {
            let mut bad_values = vec![f64::NAN, f64::INFINITY, f64::NEG_INFINITY];
            if positive.contains(&key) { bad_values.extend([0.0, -1.0]); }
            if key == "total_loss_db" { bad_values.push(-1.0); }
            if key == "clearance_fraction" { bad_values.extend([-0.01, 1.01]); }
            if key == "aperture_efficiency" { bad_values.push(f64::from_bits(1.0_f64.to_bits() + 1)); }
            for bad in bad_values {
                let mut v = valid.to_vec(); v[index].1 = bad;
                assert!(calculate(id, &v).is_err(), "{id}: {key}={bad} must fail");
                let r = calculate(id, valid).unwrap();
                assert_eq!(r.len(), expected.len());
                for &(output, value) in expected { assert_close(r[output], value, 1e-11, 0.0); }
            }
        }
    }
}
