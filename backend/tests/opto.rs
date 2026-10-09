mod common;

use common::{assert_close, calculate};
use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_6, LN_2, PI};

// Non-default oracles use exact SI constants, hemisphere integration, optical-depth
// fractions, Gaussian energy integrals and Poisson empty-cell probabilities.
// Constants were independently rounded from 70-digit Decimal arithmetic, not
// catalog examples. Nonzero quantities have no absolute tolerance floor.
const PHOTON: &[(&str, f64)] = &[
    ("wavelength", 5e-7), ("quantum_efficiency", 0.6), ("optical_power", 3e-6),
];
const PHOTON_EXPECTED: &[(&str, f64)] = &[
    ("photon_energy", 3.972891714297857e-19), ("photon_rate", 7551174851314.063),
    ("responsivity", 0.24196631812047634), ("photocurrent", 7.258989543614291e-7),
];
const LAMBERT: &[(&str, f64)] = &[
    ("radiant_flux", 0.36), ("distance", 0.75),
    ("emission_angle", FRAC_PI_3), ("incidence_angle", FRAC_PI_6),
];
const LAMBERT_EXPECTED: &[(&str, f64)] = &[
    ("axial_intensity", 0.11459155902616464),
    ("directional_intensity", 0.05729577951308232),
    ("target_irradiance", 0.08821262326748673),
];
const LAYER: &[(&str, f64)] = &[
    ("incident_power", 0.008), ("dead_absorption", 69314.71805599453),
    ("dead_thickness", 1e-5), ("active_absorption", 138629.43611198905),
    ("active_thickness", 1e-5),
];
const LAYER_EXPECTED: &[(&str, f64)] = &[
    ("dead_absorbed_power", 0.004), ("active_capture_fraction", 0.375),
    ("active_absorbed_power", 0.003), ("transmitted_power", 0.001),
];
const GAUSSIAN: &[(&str, f64)] = &[
    ("wavelength", 1e-6), ("waist_radius", 2e-4),
    ("axial_distance", 0.12566370614359174), ("aperture_radius", 2e-4),
    ("beam_power", 0.024),
];
const GAUSSIAN_EXPECTED: &[(&str, f64)] = &[
    ("rayleigh_range", 0.12566370614359174), ("beam_radius", 0.000282842712474619),
    ("captured_fraction", 0.6321205588285577), ("captured_power", 0.015170893411885385),
    ("axis_irradiance", 190985.9317102744),
];
const STEP: &[(&str, f64)] = &[
    ("core_index", 1.5625), ("cladding_index", 1.5546875),
    ("core_radius", 3e-6), ("wavelength", 1.3e-6),
];
const STEP_EXPECTED: &[(&str, f64)] = &[
    ("numerical_aperture", 0.15605456527686076), ("normalized_frequency", 2.2627378884596983),
    ("cutoff_wavelength", 1.2231015613295666e-6), ("single_mode_margin", 0.14226211154030166),
];
const DISPERSION: &[(&str, f64)] = &[
    ("initial_width", 2e-12), ("initial_chirp", 2.0), ("beta_two", -4e-27),
    ("fiber_length", 400.0), ("pulse_energy", 3e-14),
];
const DISPERSION_EXPECTED: &[(&str, f64)] = &[
    ("normalized_dispersion", -0.4), ("width_ratio", 0.4472135954999579),
    ("output_width", 8.944271909999159e-13), ("output_fwhm", 1.4893189644236137e-12),
    ("output_peak_power", 0.0189234939151512),
];
const SIPM: &[(&str, f64)] = &[
    ("photon_count", 1200.0), ("detection_efficiency", 0.25),
    ("cell_count", 300.0), ("avalanche_gain", 2e6),
];
const SIPM_EXPECTED: &[(&str, f64)] = &[
    ("seed_count", 300.0), ("fired_cells", 189.6361676485673),
    ("relative_response", 0.6321205588285577), ("output_charge", 6.076612735356825e-11),
];

#[test]
fn photon_complete_nondefault_quantum_oracle() {
    let r = calculate("opto.photodiode_quantum_conversion", PHOTON).unwrap();
    for &(key, expected) in PHOTON_EXPECTED {
        assert_close(r[key], expected, 1e-12, 0.0);
    }
    // Independent conservation: every collected carrier carries exactly q coulombs.
    assert_close(r["photon_rate"] * r["photon_energy"], 3e-6, 1e-12, 0.0);
    assert_close(r["photocurrent"] / r["photon_rate"], 0.6 * 1.602176634e-19, 1e-12, 0.0);
}

#[test]
fn photon_power_scaling_and_efficiency_endpoints() {
    for (power, efficiency) in [(6e-6, 0.6), (0.0, 0.6), (3e-6, 0.0), (3e-6, 1.0)] {
        let r = calculate("opto.photodiode_quantum_conversion", &[
            ("wavelength", 5e-7), ("quantum_efficiency", efficiency), ("optical_power", power),
        ]).unwrap();
        assert_close(r["photon_energy"], 3.972891714297857e-19, 1e-12, 0.0);
        assert_close(r["photon_rate"], 7551174851314.063 * (power / 3e-6), 1e-12, 0.0);
        assert_close(r["responsivity"], 0.24196631812047634 * (efficiency / 0.6), 1e-12, 0.0);
        assert_close(r["photocurrent"], 7.258989543614291e-7 * (power / 3e-6) * (efficiency / 0.6), 1e-12, 0.0);
    }
    let r = calculate("opto.photodiode_quantum_conversion", &[
        ("wavelength", 1e-6), ("quantum_efficiency", 0.6), ("optical_power", 3e-6),
    ]).unwrap();
    for &(key, expected) in PHOTON_EXPECTED {
        assert_close(r[key], expected * if key == "photon_energy" { 0.5 } else { 2.0 }, 1e-12, 0.0);
    }
}

#[test]
fn photon_rejects_domains_and_recovers() {
    for (key, bad) in [("wavelength", 0.0), ("wavelength", -1e-6),
        ("quantum_efficiency", -0.01), ("quantum_efficiency", 1.000001), ("optical_power", -1e-12)] {
        let mut inputs = PHOTON.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = bad;
        assert!(calculate("opto.photodiode_quantum_conversion", &inputs).is_err(), "accepted {key}={bad}");
        let r = calculate("opto.photodiode_quantum_conversion", PHOTON).unwrap();
        for &(key, expected) in PHOTON_EXPECTED { assert_close(r[key], expected, 1e-12, 0.0); }
    }
}

#[test]
fn lambert_complete_independent_projection_oracle() {
    // Hemisphere integration yields Phi/pi; cos(60)*cos(30) gives sqrt(3)/4.
    let r = calculate("opto.lambertian_target_irradiance", LAMBERT).unwrap();
    for &(key, expected) in LAMBERT_EXPECTED { assert_close(r[key], expected, 1e-12, 0.0); }
}

#[test]
fn lambert_inverse_square_two_angles_zero_and_grazing_limits() {
    let far = calculate("opto.lambertian_target_irradiance", &[
        ("radiant_flux", 0.36), ("distance", 1.5),
        ("emission_angle", FRAC_PI_3), ("incidence_angle", FRAC_PI_6),
    ]).unwrap();
    for &(key, expected) in LAMBERT_EXPECTED {
        assert_close(far[key], expected * if key == "target_irradiance" { 0.25 } else { 1.0 }, 1e-12, 0.0);
    }
    let swapped = calculate("opto.lambertian_target_irradiance", &[
        ("radiant_flux", 0.36), ("distance", 0.75),
        ("emission_angle", FRAC_PI_6), ("incidence_angle", FRAC_PI_3),
    ]).unwrap();
    assert_close(swapped["target_irradiance"], 0.08821262326748673, 1e-12, 0.0);
    assert_close(swapped["directional_intensity"], 0.18 * 3.0_f64.sqrt() / PI, 1e-12, 0.0);
    for (flux, emission, incidence) in [(0.0, FRAC_PI_3, FRAC_PI_6), (0.36, 0.0, 0.0),
        (0.36, FRAC_PI_2, 0.0), (0.36, 0.0, FRAC_PI_2)] {
        let r = calculate("opto.lambertian_target_irradiance", &[
            ("radiant_flux", flux), ("distance", 0.75),
            ("emission_angle", emission), ("incidence_angle", incidence),
        ]).unwrap();
        let axial = flux / PI;
        assert_close(r["axial_intensity"], axial, 1e-12, 0.0);
        // The represented pi/2 has a cosine residual bounded by one ulp in radians.
        let source_grazes = emission == FRAC_PI_2;
        let receiver_grazes = incidence == FRAC_PI_2;
        assert_close(r["directional_intensity"], if source_grazes { 0.0 } else { axial }, 1e-12,
            if source_grazes { axial * f64::EPSILON } else { 0.0 });
        assert_close(r["target_irradiance"], if source_grazes || receiver_grazes { 0.0 } else { axial / 0.5625 },
            1e-12, if source_grazes || receiver_grazes { axial / 0.5625 * f64::EPSILON } else { 0.0 });
    }
}

#[test]
fn lambert_rejects_domains_and_recovers() {
    for (key, bad) in [("radiant_flux", -1.0), ("distance", 0.0), ("distance", -0.1),
        ("emission_angle", -1e-12), ("incidence_angle", -1e-12),
        ("emission_angle", f64::from_bits(FRAC_PI_2.to_bits() + 1)),
        ("incidence_angle", f64::from_bits(FRAC_PI_2.to_bits() + 1))] {
        let mut inputs = LAMBERT.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = bad;
        assert!(calculate("opto.lambertian_target_irradiance", &inputs).is_err(), "accepted {key}={bad}");
        let r = calculate("opto.lambertian_target_irradiance", LAMBERT).unwrap();
        for &(key, expected) in LAMBERT_EXPECTED { assert_close(r[key], expected, 1e-12, 0.0); }
    }
}

#[test]
fn layer_complete_half_then_quarter_transmission_oracle() {
    // ln(2) entrance depth transmits 1/2; ln(4) active depth transmits 1/4 of that.
    let r = calculate("opto.layer_absorption_capture", LAYER).unwrap();
    for &(key, expected) in LAYER_EXPECTED { assert_close(r[key], expected, 1e-12, 0.0); }
    assert_close(r["dead_absorbed_power"] + r["active_absorbed_power"] + r["transmitted_power"], 0.008, 1e-12, 0.0);
}

#[test]
fn layer_transparent_absent_opaque_and_zero_power_limits() {
    for (dead, active, power, expected) in [
        (0.0, 0.0, 0.008, [0.0, 0.0, 0.0, 0.008]),
        (0.0, 2.0 * LN_2, 0.008, [0.0, 0.75, 0.006, 0.002]),
        (LN_2, 0.0, 0.008, [0.004, 0.0, 0.0, 0.004]),
        (LN_2, 800.0, 0.008, [0.004, 0.5, 0.004, 0.0]),
        (LN_2, 2.0 * LN_2, 0.0, [0.0, 0.375, 0.0, 0.0]),
    ] {
        let r = calculate("opto.layer_absorption_capture", &[
            ("incident_power", power), ("dead_absorption", dead / 1e-5), ("dead_thickness", 1e-5),
            ("active_absorption", active / 1e-5), ("active_thickness", 1e-5),
        ]).unwrap();
        for ((key, _), value) in LAYER_EXPECTED.iter().zip(expected) { assert_close(r[*key], value, 1e-12, 0.0); }
        assert_close(r["dead_absorbed_power"] + r["active_absorbed_power"] + r["transmitted_power"], power, 1e-12, 0.0);
    }
    for (key, expected) in [
        ("dead_thickness", [0.0, 0.75, 0.006, 0.002]),
        ("active_thickness", [0.004, 0.0, 0.0, 0.004]),
    ] {
        let mut inputs = LAYER.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = 0.0;
        let r = calculate("opto.layer_absorption_capture", &inputs).unwrap();
        for ((key, _), value) in LAYER_EXPECTED.iter().zip(expected) { assert_close(r[*key], value, 1e-12, 0.0); }
    }
}

#[test]
fn layer_retains_tiny_absorption_without_cancellation() {
    let r = calculate("opto.layer_absorption_capture", &[
        ("incident_power", 0.008), ("dead_absorption", 0.0), ("dead_thickness", 0.0),
        ("active_absorption", 1e-11), ("active_thickness", 1e-5),
    ]).unwrap();
    // Integral exp(-u) du = u - u^2/2 + ...; the relative correction is 5e-17.
    assert_eq!(r["dead_absorbed_power"], 0.0);
    assert_close(r["active_capture_fraction"], 1e-16, 1e-12, 0.0);
    assert_close(r["active_absorbed_power"], 8e-19, 1e-12, 0.0);
    assert_close(r["transmitted_power"], 0.008, 1e-12, 0.0);
    let r = calculate("opto.layer_absorption_capture", &[
        ("incident_power", 0.008), ("dead_absorption", 1e-11), ("dead_thickness", 1e-5),
        ("active_absorption", 0.0), ("active_thickness", 0.0),
    ]).unwrap();
    assert_close(r["dead_absorbed_power"], 8e-19, 1e-12, 0.0);
    assert_eq!(r["active_capture_fraction"], 0.0);
    assert_eq!(r["active_absorbed_power"], 0.0);
    assert_close(r["transmitted_power"], 0.008, 1e-12, 0.0);
}

#[test]
fn layer_rejects_negative_inputs_and_recovers() {
    for index in 0..LAYER.len() {
        let mut inputs = LAYER.to_vec();
        inputs[index].1 = -1e-12;
        assert!(calculate("opto.layer_absorption_capture", &inputs).is_err(), "accepted {}", inputs[index].0);
        let r = calculate("opto.layer_absorption_capture", LAYER).unwrap();
        for &(key, expected) in LAYER_EXPECTED { assert_close(r[key], expected, 1e-12, 0.0); }
    }
}

#[test]
fn gaussian_complete_rayleigh_plane_integral_oracle() {
    // z=zR gives w=sqrt(2)*w0; a=w0 encloses 1-e^-1 of the radial Gaussian integral.
    let r = calculate("opto.gaussian_aperture_capture", GAUSSIAN).unwrap();
    for &(key, expected) in GAUSSIAN_EXPECTED { assert_close(r[key], expected, 1e-12, 0.0); }
    assert_close(r["axis_irradiance"] * PI * r["beam_radius"].powi(2) / 2.0, 0.024, 1e-12, 0.0);
}

#[test]
fn gaussian_signed_distance_waist_and_zero_limits() {
    let mut inputs = GAUSSIAN.to_vec();
    inputs[2].1 = -inputs[2].1;
    let r = calculate("opto.gaussian_aperture_capture", &inputs).unwrap();
    for &(key, expected) in GAUSSIAN_EXPECTED { assert_close(r[key], expected, 1e-12, 0.0); }
    inputs[2].1 = 0.0;
    let r = calculate("opto.gaussian_aperture_capture", &inputs).unwrap();
    assert_close(r["rayleigh_range"], 0.12566370614359174, 1e-12, 0.0);
    assert_close(r["beam_radius"], 2e-4, 1e-12, 0.0);
    assert_close(r["captured_fraction"], 0.8646647167633873, 1e-12, 0.0);
    assert_close(r["captured_power"], 0.024 * 0.8646647167633873, 1e-12, 0.0);
    assert_close(r["axis_irradiance"], 1200000.0 / PI, 1e-12, 0.0);
    for key in ["beam_power", "aperture_radius"] {
        let mut inputs = GAUSSIAN.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = 0.0;
        let r = calculate("opto.gaussian_aperture_capture", &inputs).unwrap();
        for &(output, nominal) in GAUSSIAN_EXPECTED {
            let zero = output == "captured_power" || (key == "beam_power" && output == "axis_irradiance")
                || (key == "aperture_radius" && output == "captured_fraction");
            assert_close(r[output], if zero { 0.0 } else { nominal }, 1e-12, 0.0);
        }
    }
}

#[test]
fn gaussian_tiny_aperture_and_inclusive_paraxial_endpoint() {
    let r = calculate("opto.gaussian_aperture_capture", &[
        ("wavelength", 1e-6), ("waist_radius", 2e-4), ("axial_distance", 0.0),
        ("aperture_radius", 2e-13), ("beam_power", 0.024),
    ]).unwrap();
    // Nearly constant intensity over a tiny disk: captured power = Iaxis*pi*a^2.
    assert_close(r["captured_fraction"], 2e-18, 1e-12, 0.0);
    assert_close(r["captured_power"], 4.8e-20, 1e-12, 0.0);
    assert_close(r["captured_power"], r["axis_irradiance"] * PI * (2e-13_f64).powi(2), 1e-12, 0.0);
    let r = calculate("opto.gaussian_aperture_capture", &[
        ("wavelength", 0.1 * PI * 1e-3), ("waist_radius", 1e-3), ("axial_distance", 0.0),
        ("aperture_radius", 1e-3), ("beam_power", 0.02),
    ]).unwrap();
    assert_close(r["rayleigh_range"], 0.01, 1e-12, 0.0);
    assert_close(r["beam_radius"], 1e-3, 1e-12, 0.0);
    assert_close(r["captured_fraction"], 0.8646647167633873, 1e-12, 0.0);
    assert_close(r["captured_power"], 0.02 * 0.8646647167633873, 1e-12, 0.0);
    assert_close(r["axis_irradiance"], 40000.0 / PI, 1e-12, 0.0);
}

#[test]
fn gaussian_rejects_domains_and_recovers() {
    for (key, bad) in [("wavelength", 0.0), ("wavelength", -1e-6), ("waist_radius", 0.0),
        ("waist_radius", -1e-4), ("aperture_radius", -1e-12), ("beam_power", -1e-12),
        ("wavelength", 0.101 * PI * 2e-4)] {
        let mut inputs = GAUSSIAN.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = bad;
        assert!(calculate("opto.gaussian_aperture_capture", &inputs).is_err(), "accepted {key}={bad}");
        let r = calculate("opto.gaussian_aperture_capture", GAUSSIAN).unwrap();
        for &(key, expected) in GAUSSIAN_EXPECTED { assert_close(r[key], expected, 1e-12, 0.0); }
    }
}

#[test]
fn step_index_complete_binary_exact_contrast_oracle() {
    // n1=200/128, n2=199/128, so NA=sqrt(399)/128 without decimal cancellation.
    let r = calculate("opto.step_index_mode_cutoff", STEP).unwrap();
    for &(key, expected) in STEP_EXPECTED { assert_close(r[key], expected, 1e-12, 0.0); }
}

#[test]
fn step_index_radius_scaling_signed_margin_and_cutoff() {
    let mut inputs = STEP.to_vec();
    inputs[2].1 = 6e-6;
    let r = calculate("opto.step_index_mode_cutoff", &inputs).unwrap();
    assert_close(r["numerical_aperture"], 0.15605456527686076, 1e-12, 0.0);
    assert_close(r["normalized_frequency"], 2.0 * 2.2627378884596983, 1e-12, 0.0);
    assert_close(r["cutoff_wavelength"], 2.0 * 1.2231015613295666e-6, 1e-12, 0.0);
    assert_close(r["single_mode_margin"], -2.1204757769193965, 1e-12, 0.0);
    inputs[2].1 = 3e-6;
    inputs[3].1 = 1.2231015613295666e-6;
    let r = calculate("opto.step_index_mode_cutoff", &inputs).unwrap();
    assert_close(r["normalized_frequency"], 2.405, 1e-12, 0.0);
    assert_close(r["cutoff_wavelength"], inputs[3].1, 1e-12, 0.0);
    // Margin subtracts two O(2.405) values; allow eight roundoff ulps of that scale.
    assert_close(r["single_mode_margin"], 0.0, 0.0, 8.0 * f64::EPSILON * 2.405);
}

#[test]
fn step_index_adjacent_indices_and_one_percent_endpoint() {
    let n1: f64 = 1.5625;
    let n2 = f64::from_bits(n1.to_bits() - 1);
    let r = calculate("opto.step_index_mode_cutoff", &[
        ("core_index", n1), ("cladding_index", n2), ("core_radius", 3e-6), ("wavelength", 1.3e-6),
    ]).unwrap();
    // One-ulp spacing is 2^-52; squared-index difference is exactly delta*(2*n1-delta).
    let delta = 2.0_f64.powi(-52);
    let na = (delta * (3.125 - delta)).sqrt();
    let v = (6e-6 * PI / 1.3e-6) * na;
    assert_close(r["numerical_aperture"], na, 1e-12, 0.0);
    assert_close(r["normalized_frequency"], v, 1e-12, 0.0);
    assert_close(r["cutoff_wavelength"], 6e-6 * PI * na / 2.405, 1e-12, 0.0);
    assert_close(r["single_mode_margin"], 2.405 - v, 1e-12, 0.0);
    assert!(r["numerical_aperture"] < 0.15605456527686076);
    let boundary: f64 = 1.546875;
    let mut inputs = [("core_index", n1), ("cladding_index", boundary),
        ("core_radius", 3e-6), ("wavelength", 1.3e-6)];
    let r = calculate("opto.step_index_mode_cutoff", &inputs).unwrap();
    let na = 199.0_f64.sqrt() / 64.0;
    let v = 6e-6 * PI * na / 1.3e-6;
    assert_close(r["numerical_aperture"], na, 1e-12, 0.0);
    assert_close(r["normalized_frequency"], v, 1e-12, 0.0);
    assert_close(r["cutoff_wavelength"], 6e-6 * PI * na / 2.405, 1e-12, 0.0);
    assert_close(r["single_mode_margin"], 2.405 - v, 1e-12, 0.0);
    inputs[1].1 = f64::from_bits(boundary.to_bits() - 1);
    assert!(calculate("opto.step_index_mode_cutoff", &inputs).is_err());
    let r = calculate("opto.step_index_mode_cutoff", STEP).unwrap();
    for &(key, expected) in STEP_EXPECTED { assert_close(r[key], expected, 1e-12, 0.0); }
}

#[test]
fn step_index_rejects_domains_and_recovers() {
    for (key, bad) in [("core_index", 0.99), ("cladding_index", 0.99),
        ("cladding_index", 1.5625), ("cladding_index", 1.57), ("cladding_index", 1.54),
        ("core_radius", 0.0), ("core_radius", -1e-6), ("wavelength", 0.0), ("wavelength", -1e-6)] {
        let mut inputs = STEP.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = bad;
        assert!(calculate("opto.step_index_mode_cutoff", &inputs).is_err(), "accepted {key}={bad}");
        let r = calculate("opto.step_index_mode_cutoff", STEP).unwrap();
        for &(key, expected) in STEP_EXPECTED { assert_close(r[key], expected, 1e-12, 0.0); }
    }
}

#[test]
fn dispersion_complete_minimum_compression_energy_oracle() {
    // Completing the square: b^2=5*(x+2/5)^2+1/5 for C=2.
    let r = calculate("opto.fiber_gaussian_dispersion", DISPERSION).unwrap();
    for &(key, expected) in DISPERSION_EXPECTED { assert_close(r[key], expected, 1e-12, 0.0); }
    assert_close(PI.sqrt() * r["output_peak_power"] * r["output_width"], 3e-14, 1e-12, 0.0);
    assert_close(r["output_fwhm"] / r["output_width"], 2.0 * LN_2.sqrt(), 1e-12, 0.0);
}

#[test]
fn dispersion_signed_chirp_minimum_unchirped_and_zero_limits() {
    for (chirp, beta, length, energy, x, ratio) in [
        (-2.0, 4e-27, 400.0, 3e-14, 0.4, 1.0 / 5.0_f64.sqrt()),
        (2.0, -4e-27, 0.0, 3e-14, 0.0, 1.0),
        (2.0, 0.0, 400.0, 3e-14, 0.0, 1.0),
        (0.0, -4e-27, 1000.0, 3e-14, -1.0, 2.0_f64.sqrt()),
        (2.0, -4e-27, 200.0, 3e-14, -0.2, 0.4_f64.sqrt()),
        (2.0, -4e-27, 600.0, 3e-14, -0.6, 0.4_f64.sqrt()),
        (2.0, -4e-27, 400.0, 0.0, -0.4, 1.0 / 5.0_f64.sqrt()),
    ] {
        let r = calculate("opto.fiber_gaussian_dispersion", &[
            ("initial_width", 2e-12), ("initial_chirp", chirp), ("beta_two", beta),
            ("fiber_length", length), ("pulse_energy", energy),
        ]).unwrap();
        assert_close(r["normalized_dispersion"], x, 1e-12, 0.0);
        assert_close(r["width_ratio"], ratio, 1e-12, 0.0);
        assert_close(r["output_width"], 2e-12 * ratio, 1e-12, 0.0);
        assert_close(r["output_fwhm"], 4e-12 * ratio * LN_2.sqrt(), 1e-12, 0.0);
        assert_close(r["output_peak_power"], energy / (PI.sqrt() * 2e-12 * ratio), 1e-12, 0.0);
    }
}

#[test]
fn dispersion_rejects_domains_and_recovers() {
    for (key, bad) in [("initial_width", 0.0), ("initial_width", -1e-12),
        ("fiber_length", -1.0), ("pulse_energy", -1e-20)] {
        let mut inputs = DISPERSION.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = bad;
        assert!(calculate("opto.fiber_gaussian_dispersion", &inputs).is_err(), "accepted {key}={bad}");
        let r = calculate("opto.fiber_gaussian_dispersion", DISPERSION).unwrap();
        for &(key, expected) in DISPERSION_EXPECTED { assert_close(r[key], expected, 1e-12, 0.0); }
    }
}

#[test]
fn sipm_complete_one_seed_per_cell_poisson_oracle() {
    // Mean one seed/cell leaves e^-1 cells empty; multiply occupied cells fire only once.
    let r = calculate("opto.sipm_finite_cell_response", SIPM).unwrap();
    for &(key, expected) in SIPM_EXPECTED { assert_close(r[key], expected, 1e-12, 0.0); }
    assert_close(r["fired_cells"] / r["seed_count"], r["relative_response"], 1e-12, 0.0);
    assert_close(r["output_charge"] / r["fired_cells"], 2e6 * 1.602176634e-19, 1e-12, 0.0);
}

#[test]
fn sipm_zero_exposure_gain_scaling_and_single_cell() {
    for (photons, pde, cells, gain, seeds, fired, response) in [
        (0.0, 0.25, 300.0, 2e6, 0.0, 0.0, 1.0),
        (1200.0, 0.0, 300.0, 2e6, 0.0, 0.0, 1.0),
        (1200.0, 0.25, 300.0, 4e6, 300.0, 189.6361676485673, 0.6321205588285577),
        (1.0, 1.0, 1.0, 2e6, 1.0, 0.6321205588285577, 0.6321205588285577),
    ] {
        let r = calculate("opto.sipm_finite_cell_response", &[
            ("photon_count", photons), ("detection_efficiency", pde), ("cell_count", cells), ("avalanche_gain", gain),
        ]).unwrap();
        assert_close(r["seed_count"], seeds, 1e-12, 0.0);
        assert_close(r["fired_cells"], fired, 1e-12, 0.0);
        assert_close(r["relative_response"], response, 1e-12, 0.0);
        assert_close(r["output_charge"], 1.602176634e-19 * gain * fired, 1e-12, 0.0);
    }
}

#[test]
fn sipm_tiny_fractional_photons_and_saturation_monotonicity() {
    let r = calculate("opto.sipm_finite_cell_response", &[
        ("photon_count", 4e-17), ("detection_efficiency", 0.25),
        ("cell_count", 300.0), ("avalanche_gain", 2e6),
    ]).unwrap();
    assert_close(r["seed_count"], 1e-17, 1e-12, 0.0);
    assert_close(r["fired_cells"], 1e-17, 1e-12, 0.0);
    assert_eq!(r["relative_response"], 1.0);
    assert_close(r["output_charge"], 3.204353268e-30, 1e-12, 0.0);
    let mut previous_fired = 0.0;
    let mut previous_response = 1.0;
    for (photons, fired, response) in [
        (1200.0, 189.6361676485673, 0.6321205588285577),
        (2400.0, 259.3994150290162, 0.43233235838169365),
        (60000.0, 300.0, 0.02),
    ] {
        let r = calculate("opto.sipm_finite_cell_response", &[
            ("photon_count", photons), ("detection_efficiency", 0.25),
            ("cell_count", 300.0), ("avalanche_gain", 2e6),
        ]).unwrap();
        assert_close(r["seed_count"], photons * 0.25, 1e-12, 0.0);
        assert_close(r["fired_cells"], fired, 1e-12, 0.0);
        assert_close(r["relative_response"], response, 1e-12, 0.0);
        assert_close(r["output_charge"], fired * 3.204353268e-13, 1e-12, 0.0);
        assert!(r["fired_cells"] > previous_fired && r["fired_cells"] <= 300.0);
        assert!(r["relative_response"] < previous_response);
        previous_fired = r["fired_cells"];
        previous_response = r["relative_response"];
    }
}

#[test]
fn sipm_rejects_domains_and_fractional_cells_then_recovers() {
    for (key, bad) in [("photon_count", -1e-12), ("detection_efficiency", -1e-12),
        ("detection_efficiency", 1.000001), ("cell_count", 0.0), ("cell_count", -1.0),
        ("cell_count", 1.5), ("cell_count", f64::from_bits(300.0_f64.to_bits() + 1)),
        ("avalanche_gain", 0.0), ("avalanche_gain", -1.0)] {
        let mut inputs = SIPM.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = bad;
        assert!(calculate("opto.sipm_finite_cell_response", &inputs).is_err(), "accepted {key}={bad}");
        let r = calculate("opto.sipm_finite_cell_response", SIPM).unwrap();
        for &(key, expected) in SIPM_EXPECTED { assert_close(r[key], expected, 1e-12, 0.0); }
    }
}

#[test]
fn every_opto_input_rejects_nonfinite_values_then_recovers_all_outputs() {
    for (id, inputs, expected) in [
        ("opto.photodiode_quantum_conversion", PHOTON, PHOTON_EXPECTED),
        ("opto.lambertian_target_irradiance", LAMBERT, LAMBERT_EXPECTED),
        ("opto.layer_absorption_capture", LAYER, LAYER_EXPECTED),
        ("opto.gaussian_aperture_capture", GAUSSIAN, GAUSSIAN_EXPECTED),
        ("opto.step_index_mode_cutoff", STEP, STEP_EXPECTED),
        ("opto.fiber_gaussian_dispersion", DISPERSION, DISPERSION_EXPECTED),
        ("opto.sipm_finite_cell_response", SIPM, SIPM_EXPECTED),
    ] {
        for index in 0..inputs.len() {
            for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                let mut invalid = inputs.to_vec();
                invalid[index].1 = bad;
                assert!(calculate(id, &invalid).is_err(), "{id} accepted {}={bad}", invalid[index].0);
                let r = calculate(id, inputs).unwrap();
                for &(key, value) in expected { assert_close(r[key], value, 1e-12, 0.0); }
            }
        }
    }
}

#[test]
fn layer_preserves_dimensional_power_when_transmission_fraction_underflows() {
    // exp(-800) itself is below binary64; multiplying by P0 first leaves real power.
    // These admitted numerical probes are not physically realizable power ratings.
    for (active, captured, transmitted) in [
        (0.0, 0.0, 3.667874584177687e-48),
        (1.0, 2.3185389318634633e-48, 1.3493356523142239e-48),
    ] {
        let r = calculate("opto.layer_absorption_capture", &[
            ("incident_power", 1e300), ("dead_absorption", 800000.0),
            ("dead_thickness", 0.001), ("active_absorption", active), ("active_thickness", 1.0),
        ]).unwrap();
        assert_close(r["dead_absorbed_power"], 1e300, 1e-12, 0.0);
        assert_eq!(r["active_capture_fraction"], 0.0);
        assert_close(r["active_absorbed_power"], captured, 1e-12, 0.0);
        assert_close(r["transmitted_power"], transmitted, 1e-12, 0.0);
    }
}

#[test]
fn gaussian_preserves_dimensional_power_when_capture_fraction_underflows() {
    let r = calculate("opto.gaussian_aperture_capture", &[
        ("wavelength", 6.33e-7), ("waist_radius", 1e-4), ("axial_distance", 0.0),
        ("aperture_radius", 1e-174), ("beam_power", 1e100),
    ]).unwrap();
    // Disk integral is Iaxis*pi*a^2 to relative error O(1e-340).
    // Its fraction rounds to zero, but its dimensional power must not.
    assert_close(r["rayleigh_range"], 0.04963021569652122, 1e-12, 0.0);
    assert_close(r["beam_radius"], 1e-4, 1e-12, 0.0);
    assert_eq!(r["captured_fraction"], 0.0);
    assert_close(r["captured_power"], 2e-240, 1e-12, 0.0);
    assert_close(r["axis_irradiance"], 6.366197723675813e107, 1e-12, 0.0);
}
