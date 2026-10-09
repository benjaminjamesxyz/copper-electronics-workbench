mod common;
use common::{assert_close, calculate};

#[test]
fn thin_toroid_retains_adjacent_radius_difference() {
    let result = calculate("magnetics.rectangular_toroid_inductance", &[
        ("inner_radius", 3.0), ("outer_radius", 3.0000000000000004),
        ("height", 0.005), ("relative_permeability", 100.0), ("turns", 50.0),
        ("peak_current", 0.0), ("linear_flux_limit", 0.2),
    ]).unwrap();
    assert_close(result["inductance"], 3.700743415417188e-20, 1e-12, 0.0);
}

#[test]
fn skin_depth_does_not_overflow_before_square_root() {
    let result = calculate("magnetics.conductor_skin_depth", &[
        ("frequency", 1e-310), ("conductivity", 1.0),
        ("relative_permeability", 1.0), ("relative_permittivity", 1.0),
    ]).unwrap();
    assert_close(result["skin_depth"], 5.032921210448704e157, 1e-12, 0.0);
}

#[test]
fn magnetic_energy_is_not_derived_from_underflowed_density() {
    let result = calculate("magnetics.field_energy_density", &[
        ("flux_density", 1e-162), ("relative_permeability", 1.0),
        ("volume", 1e300), ("linear_flux_limit", 0.3),
    ]).unwrap();
    // One subnormal ULP for density; energy independently retains normal precision.
    assert_close(result["energy_density"], 3.978873577297384e-319, 0.0, f64::from_bits(1));
    assert_close(result["field_energy"], 3.978873577297384e-19, 1e-12, 0.0);
}

#[test]
fn mutual_inductance_is_invariant_to_extreme_winding_order() {
    for (one, two) in [(1e-200, 1e200), (1e200, 1e-200)] {
        let result = calculate("magnetics.coupled_coil_inductance", &[
            ("inductance_one", one), ("inductance_two", two), ("coupling", 1e-300),
        ]).unwrap();
        assert_close(result["mutual_inductance"], 1e-300, 1e-12, 0.0);
    }
}

#[test]
fn parallel_heat_paths_preserve_finite_subnormal_resistance() {
    for (one, two) in [(1e-309, 1.0), (1.0, 1e-309)] {
        let result = calculate("thermal.parallel_conduction_paths", &[
            ("path_one_length", one), ("path_one_area", 1.0), ("path_one_conductivity", 1.0),
            ("path_two_length", two), ("path_two_area", 1.0), ("path_two_conductivity", 1.0),
            ("sink_temperature", 300.0), ("source_power", 1.0),
        ]).unwrap();
        assert_close(result["equivalent_resistance"], 1e-309, 1e-12, f64::from_bits(1));
        assert_close(result["path_one_heat"], if one < two { 1.0 } else { 1e-309 }, 1e-12, f64::from_bits(1));
        assert_close(result["path_two_heat"], if one < two { 1e-309 } else { 1.0 }, 1e-12, f64::from_bits(1));
        assert_close(result["path_one_heat"] + result["path_two_heat"], 1.0, 1e-12, 0.0);
    }
}

#[test]
fn subnormal_noise_figure_survives_temperature_conversion_and_friis() {
    let tiny = 1e-323;
    // Independently rounded 290*ln(10)/10 times the exact binary64 input.
    let expected_temperature = 6.6e-322;
    let direct = calculate("noise.figure_equivalent_temperature", &[("noise_figure_db", tiny)]).unwrap();
    let cascade = calculate("noise.friis_three_stage", &[
        ("first_noise_figure_db", tiny), ("second_noise_figure_db", 0.0),
        ("third_noise_figure_db", 0.0), ("first_gain_db", 0.0), ("second_gain_db", 0.0),
    ]).unwrap();
    assert_eq!(cascade["cascade_noise_figure_db"], tiny);
    for value in [direct["equivalent_noise_temperature"], cascade["cascade_noise_temperature"]] {
        assert!(value > 0.0);
        assert_close(value, expected_temperature, 0.0, f64::from_bits(1));
    }
}

#[test]
fn large_heat_power_recovers_small_parallel_path_fraction() {
    for (one, two) in [(1e-200, 1e200), (1e200, 1e-200)] {
        let result = calculate("thermal.parallel_conduction_paths", &[
            ("path_one_length", one), ("path_one_area", 1.0), ("path_one_conductivity", 1.0),
            ("path_two_length", two), ("path_two_area", 1.0), ("path_two_conductivity", 1.0),
            ("sink_temperature", 300.0), ("source_power", 1e200),
        ]).unwrap();
        assert_close(result["hot_temperature"], 301.0, 1e-12, 0.0);
        assert_close(result["path_one_heat"], if one < two { 1e200 } else { 1e-200 }, 1e-12, 0.0);
        assert_close(result["path_two_heat"], if one < two { 1e-200 } else { 1e200 }, 1e-12, 0.0);
    }
}

#[test]
fn nearly_coincident_ladder_poles_preserve_subnormal_excitation() {
    let result = calculate("transients.rc_ladder_step", &[
        ("r_one", 0.001), ("r_two", 10000.0), ("c_one", 1.0),
        ("c_two", 1e-7), ("v_step", 1e-321), ("time", 0.0002),
    ]).unwrap();
    // High-precision analytic two-pole response rounds to four subnormal ULPs.
    assert!(result["voltage_two"] > 0.0);
    assert_close(result["voltage_two"], 2e-323, 0.0, f64::from_bits(1));
}

#[test]
fn third_stage_temperature_scales_before_gain_attenuation() {
    let result = calculate("noise.friis_three_stage", &[
        ("first_noise_figure_db", 0.0), ("second_noise_figure_db", 0.0),
        ("third_noise_figure_db", 1e-301), ("first_gain_db", 120.0),
        ("second_gain_db", 120.0),
    ]).unwrap();
    // NF is below representability; dimensional temperature rounds to one ULP.
    assert_eq!(result["cascade_noise_figure_db"], 0.0);
    assert_eq!(result["cascade_noise_factor"], 1.0);
    assert_eq!(result["cascade_noise_temperature"], f64::from_bits(1));
}

#[test]
fn small_magnetic_volume_is_scaled_after_permeability_normalization() {
    let result = calculate("magnetics.field_energy_density", &[
        ("flux_density", 1e-170), ("relative_permeability", 5e-324),
        ("volume", 1e-310), ("linear_flux_limit", 0.3),
    ]).unwrap();
    assert_close(result["energy_density"], 8.053329776699101e-12, 1e-12, 0.0);
    assert!(result["field_energy"] > 0.0);
    assert_close(result["field_energy"], 8.05e-322, 0.0, f64::from_bits(1));
}
