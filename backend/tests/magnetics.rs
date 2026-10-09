mod common;

use common::{assert_close, calculate};
use std::f64::consts::PI;

// Oracles come from research/magnetics.md's Faraday/Ampere/energy derivations,
// not catalog evaluation. Each vector specifies every input and checks every output.
// Nonzero outputs use the research arithmetic tolerance, with no absolute floor.
// Exact-zero boundaries below are multiplication by zero or exact equal operands:
// zero absolute tolerance is appropriate (there is no cancellation residual to hide).
const TURNS: &[(&str, f64)] = &[
    ("primary_turns", 1000.0), ("secondary_turns", 100.0),
    ("primary_voltage", 230.0), ("load_resistance", 24.0),
];
const SINE: &[(&str, f64)] = &[
    ("rms_voltage", 24.0), ("frequency", 50.0), ("turns", 1000.0),
    ("core_area", 0.0004), ("flux_limit", 0.3),
];
const TOROID: &[(&str, f64)] = &[
    ("inner_radius", 0.01), ("outer_radius", 0.02), ("height", 0.005),
    ("relative_permeability", 100.0), ("turns", 50.0),
    ("peak_current", 0.1), ("linear_flux_limit", 0.2),
];
const GAP: &[(&str, f64)] = &[
    ("core_length", 0.1), ("pole_width", 0.01), ("pole_height", 0.01),
    ("gap_length", 0.0005), ("relative_permeability", 2000.0),
    ("turns", 100.0), ("peak_current", 1.0), ("linear_flux_limit", 0.3),
];
const ENERGY: &[(&str, f64)] = &[
    ("flux_density", 0.2), ("relative_permeability", 1.0),
    ("volume", 5e-8), ("linear_flux_limit", 0.3),
];
const SKIN: &[(&str, f64)] = &[
    ("frequency", 100000.0), ("conductivity", 58000000.0),
    ("relative_permeability", 1.0), ("relative_permittivity", 1.0),
];
const VA: &[(&str, f64)] = &[
    ("secondary_one_voltage", 24.0), ("secondary_one_current", 2.0),
    ("secondary_two_voltage", 12.0), ("secondary_two_current", 0.5),
    ("headroom_factor", 1.25),
];
const COUPLED: &[(&str, f64)] = &[
    ("inductance_one", 0.0001), ("inductance_two", 0.0004), ("coupling", 0.9),
];

#[test]
fn turns_ratio_complete_researched_example() {
    let r = calculate("magnetics.turns_ratio", TURNS).unwrap();
    assert_close(r["voltage_ratio"], 0.1, 1e-6, 0.0);
    assert_close(r["secondary_voltage"], 23.0, 1e-6, 0.0);
    assert_close(r["reflected_resistance"], 2400.0, 1e-6, 0.0);
}

#[test]
fn turns_ratio_step_up_unity_zero_and_conditioned_reflection() {
    // Doubling secondary turns doubles voltage and divides reflected load by four.
    let r = calculate("magnetics.turns_ratio", &[
        ("primary_turns", 20.0), ("secondary_turns", 40.0),
        ("primary_voltage", 12.0), ("load_resistance", 100.0),
    ]).unwrap();
    assert_close(r["voltage_ratio"], 2.0, 1e-6, 0.0);
    assert_close(r["secondary_voltage"], 24.0, 1e-6, 0.0);
    assert_close(r["reflected_resistance"], 25.0, 1e-6, 0.0);
    let r = calculate("magnetics.turns_ratio", &[
        ("primary_turns", 1.0), ("secondary_turns", 1.0),
        ("primary_voltage", 0.0), ("load_resistance", 7.0),
    ]).unwrap();
    assert_close(r["voltage_ratio"], 1.0, 1e-6, 0.0);
    assert_close(r["secondary_voltage"], 0.0, 0.0, 0.0);
    assert_close(r["reflected_resistance"], 7.0, 1e-6, 0.0);
    // a=1e-200: Rload/a^2=1e100, despite a^2 underflowing binary64.
    let r = calculate("magnetics.turns_ratio", &[
        ("primary_turns", 1e200), ("secondary_turns", 1.0),
        ("primary_voltage", 1.0), ("load_resistance", 1e-300),
    ]).unwrap();
    assert_close(r["voltage_ratio"], 1e-200, 1e-6, 0.0);
    assert_close(r["secondary_voltage"], 1e-200, 1e-6, 0.0);
    assert_close(r["reflected_resistance"], 1e100, 1e-6, 0.0);
}

#[test]
fn turns_ratio_rejects_fractional_windings_and_invalid_magnitudes() {
    for (key, bad) in [
        ("primary_turns", 0.0), ("primary_turns", -1.0), ("primary_turns", 1.5),
        ("secondary_turns", 0.0), ("secondary_turns", -1.0), ("secondary_turns", 1.5),
        ("primary_voltage", -1.0), ("load_resistance", 0.0), ("load_resistance", -1.0),
    ] {
        let mut inputs = TURNS.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = bad;
        assert!(calculate("magnetics.turns_ratio", &inputs).is_err(), "accepted {key}={bad}");
    }
}

#[test]
fn sinusoidal_flux_complete_researched_example() {
    let r = calculate("magnetics.sinusoidal_flux_density", SINE).unwrap();
    assert_close(r["peak_flux_density"], 0.2700948948471318, 1e-6, 0.0);
    assert_close(r["peak_flux"], 0.00010803795793885272, 1e-6, 0.0);
}

#[test]
fn sinusoidal_flux_faraday_scaling_zero_and_inclusive_limit() {
    // Twice f and half Ae preserve B; flux halves. Exact sine coefficient, not 4.44.
    let r = calculate("magnetics.sinusoidal_flux_density", &[
        ("rms_voltage", 24.0), ("frequency", 100.0), ("turns", 1000.0),
        ("core_area", 0.0002), ("flux_limit", 0.3),
    ]).unwrap();
    assert_close(r["peak_flux_density"], 0.2700948948471318, 1e-6, 0.0);
    assert_close(r["peak_flux"], 0.00005401897896942636, 1e-6, 0.0);
    let r = calculate("magnetics.sinusoidal_flux_density", &[
        ("rms_voltage", 0.0), ("frequency", 50.0), ("turns", 1.0),
        ("core_area", 0.0004), ("flux_limit", 0.3),
    ]).unwrap();
    assert_close(r["peak_flux_density"], 0.0, 0.0, 0.0);
    assert_close(r["peak_flux"], 0.0, 0.0, 0.0);
    // Faraday's law with V=f=N=A=1 gives B=sqrt(2)/(2*pi).
    let limit = 2.0_f64.sqrt() / (2.0 * PI);
    let r = calculate("magnetics.sinusoidal_flux_density", &[
        ("rms_voltage", 1.0), ("frequency", 1.0), ("turns", 1.0),
        ("core_area", 1.0), ("flux_limit", limit),
    ]).unwrap();
    assert_close(r["peak_flux_density"], 0.22507907903927654, 1e-6, 0.0);
    assert_close(r["peak_flux"], 0.22507907903927654, 1e-6, 0.0);
}

#[test]
fn sinusoidal_flux_rejects_field_limit_and_each_input_guard() {
    for (key, bad) in [
        ("rms_voltage", -1.0), ("rms_voltage", 30.0),
        ("frequency", 0.0), ("frequency", -1.0),
        ("turns", 0.0), ("turns", 1.5),
        ("core_area", 0.0), ("core_area", -1.0),
        ("flux_limit", 0.0), ("flux_limit", -1.0), ("flux_limit", 0.27),
    ] {
        let mut inputs = SINE.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = bad;
        assert!(calculate("magnetics.sinusoidal_flux_density", &inputs).is_err(), "accepted {key}={bad}");
    }
}

#[test]
fn rectangular_toroid_complete_researched_example() {
    let r = calculate("magnetics.rectangular_toroid_inductance", TOROID).unwrap();
    assert_close(r["inductance"], 0.00017328679513998633, 1e-6, 0.0);
    assert_close(r["inner_flux_density"], 0.01, 1e-6, 0.0);
}

#[test]
fn rectangular_toroid_radial_integral_turns_scaling_and_zero_current() {
    // ro/ri=4 doubles ln(2); doubling N multiplies L by four and B by two.
    let r = calculate("magnetics.rectangular_toroid_inductance", &[
        ("inner_radius", 0.01), ("outer_radius", 0.04), ("height", 0.005),
        ("relative_permeability", 100.0), ("turns", 100.0),
        ("peak_current", 0.1), ("linear_flux_limit", 0.2),
    ]).unwrap();
    assert_close(r["inductance"], 0.0013862943611198906, 1e-6, 0.0);
    assert_close(r["inner_flux_density"], 0.02, 1e-6, 0.0);
    let r = calculate("magnetics.rectangular_toroid_inductance", &[
        ("inner_radius", 0.01), ("outer_radius", 0.02), ("height", 0.005),
        ("relative_permeability", 100.0), ("turns", 1.0),
        ("peak_current", 0.0), ("linear_flux_limit", 0.2),
    ]).unwrap();
    assert_close(r["inductance"], 6.931471805599453e-8, 1e-6, 0.0);
    assert_close(r["inner_flux_density"], 0.0, 0.0, 0.0);
}

#[test]
fn rectangular_toroid_rejects_empty_reversed_geometry_and_material_violation() {
    for (key, bad) in [
        ("inner_radius", 0.0), ("inner_radius", -1.0),
        ("outer_radius", 0.0), ("outer_radius", -1.0),
        ("outer_radius", 0.01), ("outer_radius", 0.009),
        ("height", 0.0), ("height", -1.0),
        ("relative_permeability", 0.0), ("relative_permeability", -1.0),
        ("turns", 0.0), ("turns", 50.5), ("peak_current", -1.0),
        ("peak_current", 2.1), ("linear_flux_limit", 0.0),
        ("linear_flux_limit", -1.0), ("linear_flux_limit", 0.009),
    ] {
        let mut inputs = TOROID.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = bad;
        assert!(calculate("magnetics.rectangular_toroid_inductance", &inputs).is_err(), "accepted {key}={bad}");
    }
}

#[test]
fn gapped_core_complete_researched_example() {
    let r = calculate("magnetics.gapped_core_inductance", GAP).unwrap();
    assert_close(r["core_reluctance"], 397887.35772973835, 1e-6, 0.0);
    assert_close(r["gap_reluctance"], 3978873.5772973835, 1e-6, 0.0);
    assert_close(r["inductance"], 0.0022847946571562133, 1e-6, 0.0);
    assert_close(r["peak_flux_density"], 0.22847946571562133, 1e-6, 0.0);
}

#[test]
fn gapped_core_equal_reluctances_and_independent_geometry_boundaries() {
    // lc/mur=g=0.0005: both reluctances equal; gap-only L would be twice too large.
    let r = calculate("magnetics.gapped_core_inductance", &[
        ("core_length", 0.1), ("pole_width", 0.01), ("pole_height", 0.01),
        ("gap_length", 0.0005), ("relative_permeability", 200.0),
        ("turns", 100.0), ("peak_current", 1.0), ("linear_flux_limit", 0.3),
    ]).unwrap();
    assert_close(r["core_reluctance"], 3978873.5772973835, 1e-6, 0.0);
    assert_close(r["gap_reluctance"], 3978873.5772973835, 1e-6, 0.0);
    assert_close(r["inductance"], 0.0012566370614359173, 1e-6, 0.0);
    assert_close(r["peak_flux_density"], 0.12566370614359173, 1e-6, 0.0);
    // Equality at both geometry screens, mur=1, N=1 and I=0 are admissible.
    let r = calculate("magnetics.gapped_core_inductance", &[
        ("core_length", 0.01), ("pole_width", 0.01), ("pole_height", 0.02),
        ("gap_length", 0.001), ("relative_permeability", 1.0),
        ("turns", 1.0), ("peak_current", 0.0), ("linear_flux_limit", 0.3),
    ]).unwrap();
    assert_close(r["core_reluctance"], 39788735.772973835, 1e-6, 0.0);
    assert_close(r["gap_reluctance"], 3978873.5772973835, 1e-6, 0.0);
    assert_close(r["inductance"], 2.2847946571562133e-8, 1e-6, 0.0);
    assert_close(r["peak_flux_density"], 0.0, 0.0, 0.0);
    // Swapping rectangular pole axes cannot affect any numerical result.
    let r = calculate("magnetics.gapped_core_inductance", &[
        ("core_length", 0.01), ("pole_width", 0.02), ("pole_height", 0.01),
        ("gap_length", 0.001), ("relative_permeability", 1.0),
        ("turns", 1.0), ("peak_current", 0.0), ("linear_flux_limit", 0.3),
    ]).unwrap();
    assert_close(r["core_reluctance"], 39788735.772973835, 1e-6, 0.0);
    assert_close(r["gap_reluctance"], 3978873.5772973835, 1e-6, 0.0);
    assert_close(r["inductance"], 2.2847946571562133e-8, 1e-6, 0.0);
    assert_close(r["peak_flux_density"], 0.0, 0.0, 0.0);
}

#[test]
fn gapped_core_rejects_both_min_branches_path_limit_and_field_limit() {
    for (key, bad) in [
        ("core_length", 0.0), ("core_length", -1.0),
        ("pole_width", 0.0), ("pole_width", -1.0),
        ("pole_height", 0.0), ("pole_height", -1.0),
        ("gap_length", 0.0), ("gap_length", -1.0),
        ("relative_permeability", 0.0), ("relative_permeability", 0.5),
        ("turns", 0.0), ("turns", 100.5), ("peak_current", -1.0),
        ("peak_current", 2.0), ("linear_flux_limit", 0.0),
        ("linear_flux_limit", -1.0), ("linear_flux_limit", 0.22),
        // Each geometry counterexample independently exceeds only its own screen.
        ("pole_width", 0.004), ("pole_height", 0.004),
        ("core_length", 0.004), ("gap_length", 0.00101),
    ] {
        let mut inputs = GAP.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = bad;
        assert!(calculate("magnetics.gapped_core_inductance", &inputs).is_err(), "accepted {key}={bad}");
    }
}

#[test]
fn field_energy_complete_researched_example() {
    let r = calculate("magnetics.field_energy_density", ENERGY).unwrap();
    assert_close(r["energy_density"], 15915.494309189535, 1e-6, 0.0);
    assert_close(r["field_energy"], 0.0007957747154594767, 1e-6, 0.0);
}

#[test]
fn field_energy_material_volume_scaling_zero_and_inclusive_field_limit() {
    // B doubles, mur quadruples: density unchanged; twice V doubles energy.
    let r = calculate("magnetics.field_energy_density", &[
        ("flux_density", 0.4), ("relative_permeability", 4.0),
        ("volume", 1e-7), ("linear_flux_limit", 0.4),
    ]).unwrap();
    assert_close(r["energy_density"], 15915.494309189535, 1e-6, 0.0);
    assert_close(r["field_energy"], 0.0015915494309189534, 1e-6, 0.0);
    let r = calculate("magnetics.field_energy_density", &[
        ("flux_density", 0.0), ("relative_permeability", 100.0),
        ("volume", 5e-8), ("linear_flux_limit", 0.3),
    ]).unwrap();
    assert_close(r["energy_density"], 0.0, 0.0, 0.0);
    assert_close(r["field_energy"], 0.0, 0.0, 0.0);
    // A small but nonzero field must not be swallowed by a blanket absolute tolerance.
    let r = calculate("magnetics.field_energy_density", &[
        ("flux_density", 2e-10), ("relative_permeability", 1.0),
        ("volume", 5e-8), ("linear_flux_limit", 0.3),
    ]).unwrap();
    assert_close(r["energy_density"], 1.5915494309189535e-14, 1e-6, 0.0);
    assert_close(r["field_energy"], 7.957747154594767e-22, 1e-6, 0.0);
}

#[test]
fn field_energy_rejects_signed_magnitude_and_invalid_linear_region() {
    for (key, bad) in [
        ("flux_density", -0.1), ("flux_density", 0.300001),
        ("relative_permeability", 0.0), ("relative_permeability", -1.0),
        ("volume", 0.0), ("volume", -1.0),
        ("linear_flux_limit", 0.0), ("linear_flux_limit", -1.0), ("linear_flux_limit", 0.19),
    ] {
        let mut inputs = ENERGY.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = bad;
        assert!(calculate("magnetics.field_energy_density", &inputs).is_err(), "accepted {key}={bad}");
    }
}

#[test]
fn conductor_skin_depth_complete_researched_example() {
    let r = calculate("magnetics.conductor_skin_depth", SKIN).unwrap();
    assert_close(r["skin_depth"], 0.0002089806784938892, 1e-6, 0.0);
}

#[test]
fn conductor_skin_depth_inverse_square_root_and_conduction_boundary() {
    // Multiplying any of f, sigma, mur by four halves amplitude penetration depth.
    for (frequency, conductivity, permeability) in [
        (400000.0, 58000000.0, 1.0),
        (100000.0, 232000000.0, 1.0),
        (100000.0, 58000000.0, 4.0),
    ] {
        let r = calculate("magnetics.conductor_skin_depth", &[
            ("frequency", frequency), ("conductivity", conductivity),
            ("relative_permeability", permeability), ("relative_permittivity", 2.0),
        ]).unwrap();
        assert_close(r["skin_depth"], 0.0001044903392469446, 1e-6, 0.0);
    }
    // At sigma=100*omega*epsilon, displacement/conduction ratio is exactly 1%.
    // Substituting that boundary into diffusion gives delta=1/(omega*sqrt(50*mu*eps)).
    let sigma = 100.0 * 2.0 * PI * 100000.0 * 8.8541878128e-12 * 1.0;
    let omega = 2.0 * PI * 100000.0;
    let expected = 1.0 / (omega * (50.0 * (4.0 * PI * 1e-7) * 8.8541878128e-12).sqrt());
    let r = calculate("magnetics.conductor_skin_depth", &[
        ("frequency", 100000.0), ("conductivity", sigma),
        ("relative_permeability", 1.0), ("relative_permittivity", 1.0),
    ]).unwrap();
    assert_close(r["skin_depth"], expected, 1e-6, 0.0);
    assert!(calculate("magnetics.conductor_skin_depth", &[
        ("frequency", 100000.0), ("conductivity", sigma * 0.999),
        ("relative_permeability", 1.0), ("relative_permittivity", 1.0),
    ]).is_err());
}

#[test]
fn conductor_skin_depth_rejects_nonpositive_coefficients_and_displacement_dominance() {
    for (key, bad) in [
        ("frequency", 0.0), ("frequency", -1.0),
        ("conductivity", 0.0), ("conductivity", -1.0), ("conductivity", 0.0005),
        ("relative_permeability", 0.0), ("relative_permeability", -1.0),
        ("relative_permittivity", 0.0), ("relative_permittivity", -1.0),
        ("relative_permittivity", 2e11),
    ] {
        let mut inputs = SKIN.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = bad;
        assert!(calculate("magnetics.conductor_skin_depth", &inputs).is_err(), "accepted {key}={bad}");
    }
}

#[test]
fn transformer_va_complete_researched_example() {
    let r = calculate("magnetics.transformer_va_budget", VA).unwrap();
    assert_close(r["secondary_one_va"], 48.0, 1e-6, 0.0);
    assert_close(r["secondary_two_va"], 6.0, 1e-6, 0.0);
    assert_close(r["total_load_va"], 54.0, 1e-6, 0.0);
    assert_close(r["minimum_rating_va"], 67.5, 1e-6, 0.0);
}

#[test]
fn transformer_va_swapped_loads_disabled_winding_and_no_headroom() {
    let r = calculate("magnetics.transformer_va_budget", &[
        ("secondary_one_voltage", 12.0), ("secondary_one_current", 0.5),
        ("secondary_two_voltage", 24.0), ("secondary_two_current", 2.0),
        ("headroom_factor", 1.25),
    ]).unwrap();
    assert_close(r["secondary_one_va"], 6.0, 1e-6, 0.0);
    assert_close(r["secondary_two_va"], 48.0, 1e-6, 0.0);
    assert_close(r["total_load_va"], 54.0, 1e-6, 0.0);
    assert_close(r["minimum_rating_va"], 67.5, 1e-6, 0.0);
    let r = calculate("magnetics.transformer_va_budget", &[
        ("secondary_one_voltage", 24.0), ("secondary_one_current", 2.0),
        ("secondary_two_voltage", 12.0), ("secondary_two_current", 0.0),
        ("headroom_factor", 1.0),
    ]).unwrap();
    assert_close(r["secondary_one_va"], 48.0, 1e-6, 0.0);
    assert_close(r["secondary_two_va"], 0.0, 0.0, 0.0);
    assert_close(r["total_load_va"], 48.0, 1e-6, 0.0);
    assert_close(r["minimum_rating_va"], 48.0, 1e-6, 0.0);
    let r = calculate("magnetics.transformer_va_budget", &[
        ("secondary_one_voltage", 24.0), ("secondary_one_current", 0.0),
        ("secondary_two_voltage", 12.0), ("secondary_two_current", 0.0),
        ("headroom_factor", 1.25),
    ]).unwrap();
    for key in ["secondary_one_va", "secondary_two_va", "total_load_va", "minimum_rating_va"] {
        assert_close(r[key], 0.0, 0.0, 0.0);
    }
}

#[test]
fn transformer_va_rejects_negative_rms_currents_and_subunity_budget() {
    for (key, bad) in [
        ("secondary_one_voltage", 0.0), ("secondary_one_voltage", -1.0),
        ("secondary_two_voltage", 0.0), ("secondary_two_voltage", -1.0),
        ("secondary_one_current", -1.0), ("secondary_two_current", -1.0),
        ("headroom_factor", 0.0), ("headroom_factor", 0.999),
    ] {
        let mut inputs = VA.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = bad;
        assert!(calculate("magnetics.transformer_va_budget", &inputs).is_err(), "accepted {key}={bad}");
    }
}

#[test]
fn coupled_coils_complete_researched_example() {
    let r = calculate("magnetics.coupled_coil_inductance", COUPLED).unwrap();
    assert_close(r["mutual_inductance"], 0.00018, 1e-6, 0.0);
    assert_close(r["series_aiding"], 0.00086, 1e-6, 0.0);
    assert_close(r["series_opposing"], 0.00014, 1e-6, 0.0);
}

#[test]
fn coupled_coils_reciprocity_uncoupled_and_perfect_coupling_endpoints() {
    let r = calculate("magnetics.coupled_coil_inductance", &[
        ("inductance_one", 0.0004), ("inductance_two", 0.0001), ("coupling", 0.9),
    ]).unwrap();
    assert_close(r["mutual_inductance"], 0.00018, 1e-6, 0.0);
    assert_close(r["series_aiding"], 0.00086, 1e-6, 0.0);
    assert_close(r["series_opposing"], 0.00014, 1e-6, 0.0);
    let r = calculate("magnetics.coupled_coil_inductance", &[
        ("inductance_one", 0.0001), ("inductance_two", 0.0004), ("coupling", 0.0),
    ]).unwrap();
    assert_close(r["mutual_inductance"], 0.0, 0.0, 0.0);
    assert_close(r["series_aiding"], 0.0005, 1e-6, 0.0);
    assert_close(r["series_opposing"], 0.0005, 1e-6, 0.0);
    let r = calculate("magnetics.coupled_coil_inductance", &[
        ("inductance_one", 0.0001), ("inductance_two", 0.0001), ("coupling", 1.0),
    ]).unwrap();
    assert_close(r["mutual_inductance"], 0.0001, 1e-6, 0.0);
    assert_close(r["series_aiding"], 0.0004, 1e-6, 0.0);
    assert_close(r["series_opposing"], 0.0, 0.0, 0.0);
    let r = calculate("magnetics.coupled_coil_inductance", &[
        ("inductance_one", 0.0001), ("inductance_two", 0.0004), ("coupling", 1.0),
    ]).unwrap();
    assert_close(r["mutual_inductance"], 0.0002, 1e-6, 0.0);
    assert_close(r["series_aiding"], 0.0009, 1e-6, 0.0);
    assert_close(r["series_opposing"], 0.0001, 1e-6, 0.0);
}

#[test]
fn coupled_coils_preserve_tiny_opposing_mismatch_and_geometric_mean_extremes() {
    // Equal coils reduce to M=k*L, Laiding=2L(1+k), Lopposing=2L(1-k).
    for (inductance, mutual, aiding, opposing) in [
        (1e200, 5e199, 3e200, 1e200),
        (1e-200, 5e-201, 3e-200, 1e-200),
    ] {
        let r = calculate("magnetics.coupled_coil_inductance", &[
            ("inductance_one", inductance), ("inductance_two", inductance), ("coupling", 0.5),
        ]).unwrap();
        assert_close(r["mutual_inductance"], mutual, 1e-6, 0.0);
        assert_close(r["series_aiding"], aiding, 1e-6, 0.0);
        assert_close(r["series_opposing"], opposing, 1e-6, 0.0);
    }
    // Adjacent binary64 L values: sqrt(1+2^-52)-1 ~ 2^-53, so opposing ~2^-106.
    // Relative correction is <3e-16; compare the small quantity without an absolute floor.
    let r = calculate("magnetics.coupled_coil_inductance", &[
        ("inductance_one", 1.0), ("inductance_two", 1.0000000000000002), ("coupling", 1.0),
    ]).unwrap();
    assert_close(r["mutual_inductance"], 1.0, 1e-6, 0.0);
    assert_close(r["series_aiding"], 4.0, 1e-6, 0.0);
    assert_close(r["series_opposing"], 1.232595164407831e-32, 1e-6, 0.0);
    // Exact binary k=1-2^-40 and L1=L2=1 gives opposing=2^-39.
    let r = calculate("magnetics.coupled_coil_inductance", &[
        ("inductance_one", 1.0), ("inductance_two", 1.0), ("coupling", 0.9999999999990905),
    ]).unwrap();
    assert_close(r["mutual_inductance"], 0.9999999999990905, 1e-6, 0.0);
    assert_close(r["series_aiding"], 3.999999999998181, 1e-6, 0.0);
    assert_close(r["series_opposing"], 1.8189894035458565e-12, 1e-6, 0.0);
}

#[test]
fn coupled_coils_reject_nonpassive_coupling_and_invalid_self_inductances() {
    for (key, bad) in [
        ("inductance_one", 0.0), ("inductance_one", -1.0),
        ("inductance_two", 0.0), ("inductance_two", -1.0),
        ("coupling", -0.000001), ("coupling", 1.000001),
    ] {
        let mut inputs = COUPLED.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = bad;
        assert!(calculate("magnetics.coupled_coil_inductance", &inputs).is_err(), "accepted {key}={bad}");
    }
}

#[test]
fn all_magnetic_models_reject_nonfinite_inputs_without_poisoning_worker() {
    for (id, nominal) in [
        ("magnetics.turns_ratio", TURNS),
        ("magnetics.sinusoidal_flux_density", SINE),
        ("magnetics.rectangular_toroid_inductance", TOROID),
        ("magnetics.gapped_core_inductance", GAP),
        ("magnetics.field_energy_density", ENERGY),
        ("magnetics.conductor_skin_depth", SKIN),
        ("magnetics.transformer_va_budget", VA),
        ("magnetics.coupled_coil_inductance", COUPLED),
    ] {
        for (index, (key, _)) in nominal.iter().enumerate() {
            for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                let mut inputs = nominal.to_vec();
                inputs[index].1 = bad;
                assert!(calculate(id, &inputs).is_err(), "accepted {id}: {key}={bad}");
            }
        }
    }
    // A valid, fully asserted model after the invalid requests checks helper recovery.
    let r = calculate("magnetics.turns_ratio", TURNS).unwrap();
    assert_close(r["voltage_ratio"], 0.1, 1e-6, 0.0);
    assert_close(r["secondary_voltage"], 23.0, 1e-6, 0.0);
    assert_close(r["reflected_resistance"], 2400.0, 1e-6, 0.0);
}
