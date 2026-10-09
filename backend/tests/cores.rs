mod common;

use common::{assert_close, calculate};
use std::f64::consts::PI;

// Independent oracles: inch-unit Wheeler, Mohan square-sheet coefficients,
// discrete window construction, calibrated power ratios, Faraday/Joule sheet
// integration, and the layer-summed thin-foil expansion. No catalog examples
// are loaded. Nonzero values have relative-only tolerances; zeros are exact.
const SOLENOID: &[(&str, f64)] = &[
    ("radius", 0.0127), ("length", 0.0254), ("turns", 25.0),
    ("wire_diameter", 0.000762),
];
const SOLENOID_OUT: &[(&str, f64)] = &[
    ("inductance", 1.0775862068965516e-5), ("turn_pitch", 0.001016),
];
const SPIRAL: &[(&str, f64)] = &[
    ("outer_side", 0.000256), ("turns", 3.0),
    ("trace_width", 0.000016), ("spacing", 0.000008),
];
const SPIRAL_OUT: &[(&str, f64)] = &[
    ("inner_side", 0.000128), ("average_side", 0.000192),
    ("fill_ratio", 1.0 / 3.0), ("inductance", 2.620711851467659e-9),
];
// Binary-exact dimensions isolate integer packing from decimal conversion.
const BOBBIN: &[(&str, f64)] = &[
    ("window_width", 0.015625), ("window_height", 0.0078125),
    ("end_margin", 0.001953125), ("wire_diameter", 0.00146484375),
    ("copper_diameter", 0.0009765625), ("tape_thickness", 0.000244140625),
    ("turns", 17.0),
];
const BOBBIN_OUT: &[(&str, f64)] = &[
    ("turns_per_layer", 8.0), ("layers_required", 3.0),
    ("radial_build", 0.0048828125), ("height_margin", 0.0029296875),
    ("copper_fill", 0.13908092476829814),
];
const STEINMETZ: &[(&str, f64)] = &[
    ("frequency", 40000.0), ("flux_peak", 0.2),
    ("reference_frequency", 160000.0), ("reference_flux_peak", 0.05),
    ("reference_loss_density", 36000.0), ("frequency_exponent", 1.5),
    ("flux_exponent", 2.5), ("core_volume", 0.000025),
    ("fit_frequency_min", 20000.0), ("fit_frequency_max", 200000.0),
    ("fit_flux_min", 0.025), ("fit_flux_max", 0.25),
];
const STEINMETZ_OUT: &[(&str, f64)] = &[
    ("loss_density", 144000.0), ("core_loss", 3.6), ("energy_per_cycle", 9e-5),
];
const LAMINATION: &[(&str, f64)] = &[
    ("frequency", 120.0), ("flux_peak", 0.6),
    ("lamination_thickness", 0.00015), ("resistivity", 8e-7),
    ("relative_permeability", 400.0), ("metal_volume", 0.0002),
    ("linear_flux_limit", 0.9),
];
const LAMINATION_OUT: &[(&str, f64)] = &[
    ("penetration_ratio", 0.07300401616752501),
    ("eddy_loss_density", 239.8313869464714), ("eddy_loss", 0.047966277389294286),
];
const FOIL: &[(&str, f64)] = &[
    ("frequency", 125000.0), ("foil_thickness", 0.00004),
    ("foil_width", 0.015), ("layers", 4.0), ("mean_turn_length", 0.09),
    ("resistivity", 2e-8), ("current_rms", 7.0),
];
const FOIL_OUT: &[(&str, f64)] = &[
    ("thickness_ratio", 0.19869176531592203), ("resistance_dc", 0.012),
    ("ac_excess_factor", 0.002736113134821757),
    ("resistance_ac", 0.01203283335761786),
    ("winding_loss", 0.5896088345232752), ("ac_excess_loss", 0.0016088345232751933),
];

#[test]
fn solenoid_inch_oracle_and_geometric_scaling() {
    // Half-inch radius, one-inch span: L = 0.25*625/14.5 microhenries.
    // Geometrically similar current sheets scale L linearly, not quadratically.
    for scale in [1.0, 2.0, 1e-120] {
        let mut inputs = SOLENOID.to_vec();
        for (key, value) in &mut inputs {
            if *key != "turns" { *value *= scale; }
        }
        let r = calculate("cores.single_layer_air_solenoid", &inputs).unwrap();
        for &(key, expected) in SOLENOID_OUT {
            assert_close(r[key], expected * scale, 1e-12, 0.0);
        }
    }
}

#[test]
fn solenoid_turn_density_and_inclusive_geometry_limits() {
    let r = calculate("cores.single_layer_air_solenoid", &[
        ("radius", 0.0127), ("length", 0.0254), ("turns", 50.0),
        ("wire_diameter", 0.000381),
    ]).unwrap();
    assert_close(r["inductance"], 4.0 * SOLENOID_OUT[0].1, 1e-12, 0.0);
    assert_close(r["turn_pitch"], 0.000508, 1e-12, 0.0);
    // Length=radius, pitch=radius/5 and diameter=pitch are all inclusive.
    let r = calculate("cores.single_layer_air_solenoid", &[
        ("radius", 0.01), ("length", 0.01), ("turns", 5.0),
        ("wire_diameter", 0.002),
    ]).unwrap();
    assert_close(r["inductance"], (0.01 / 0.0254) * 25.0 / 19.0 * 1e-6, 1e-12, 0.0);
    assert_close(r["turn_pitch"], 0.002, 1e-12, 0.0);
}

#[test]
fn solenoid_invalid_geometry_then_valid_recovery() {
    for (key, bad) in [
        ("radius", 0.0), ("radius", -0.01), ("length", 0.0), ("length", -0.01),
        ("wire_diameter", 0.0), ("wire_diameter", -0.001),
        ("turns", 0.0), ("turns", 1.5), ("length", 0.012),
        ("wire_diameter", 0.0011), ("turns", 9.0),
    ] {
        let mut inputs = SOLENOID.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = bad;
        assert!(calculate("cores.single_layer_air_solenoid", &inputs).is_err(), "accepted {key}={bad}");
        let r = calculate("cores.single_layer_air_solenoid", SOLENOID).unwrap();
        for &(key, expected) in SOLENOID_OUT { assert_close(r[key], expected, 1e-12, 0.0); }
    }
}

#[test]
fn solenoid_representable_inductance_survives_turn_square_overflow() {
    // Pure conditioning probe: L scales as radius*N^2. N^2 need not be representable.
    let r = calculate("cores.single_layer_air_solenoid", &[
        ("radius", 1e-100), ("length", 1e-100), ("turns", 1e160),
        ("wire_diameter", 1e-261),
    ]).expect("finite inductance must not overflow an intermediate turns square");
    assert_close(r["inductance"], (1e-6 / 0.0254 / 19.0) * 1e220, 1e-12, 0.0);
    assert_close(r["turn_pitch"], 1e-260, 1e-12, 0.0);
}

#[test]
fn spiral_complete_square_sheet_oracle_and_scaling() {
    // Three turns occupy 64 um radially: di=128 um, davg=192 um, fill=1/3.
    // The independently evaluated Mohan bracket is ln(6.21)+0.06+0.13/9.
    for scale in [1.0, 2.0, 1e-100] {
        let mut inputs = SPIRAL.to_vec();
        for (key, value) in &mut inputs {
            if *key != "turns" { *value *= scale; }
        }
        let r = calculate("cores.square_planar_spiral", &inputs).unwrap();
        for &(key, expected) in SPIRAL_OUT {
            assert_close(r[key], expected * if key == "fill_ratio" { 1.0 } else { scale }, 1e-12, 0.0);
        }
    }
}

#[test]
fn spiral_single_turn_has_no_interturn_gap_contribution() {
    let fill: f64 = 1.0 / 15.0;
    let inductance = 2.0 * PI * 1e-7 * 0.00024 * 1.27
        * ((2.07 / fill).ln() + 0.18 * fill + 0.13 * fill * fill);
    for spacing in [0.000008, 0.000032] {
        let r = calculate("cores.square_planar_spiral", &[
            ("outer_side", 0.000256), ("turns", 1.0),
            ("trace_width", 0.000016), ("spacing", spacing),
        ]).unwrap();
        assert_close(r["inner_side"], 0.000224, 1e-12, 0.0);
        assert_close(r["average_side"], 0.00024, 1e-12, 0.0);
        assert_close(r["fill_ratio"], fill, 1e-12, 0.0);
        assert_close(r["inductance"], inductance, 1e-12, 0.0);
    }
}

#[test]
fn spiral_tiny_positive_opening_and_inclusive_spacing_screen() {
    let width: f64 = 1.0 / 65536.0;
    let opening: f64 = 2.0_f64.powi(-40);
    let average = 5.0 * width + opening;
    let fill = 5.0 * width / average;
    let r = calculate("cores.square_planar_spiral", &[
        ("outer_side", 10.0 * width + opening), ("turns", 2.0),
        ("trace_width", width), ("spacing", 3.0 * width),
    ]).unwrap();
    assert_close(r["inner_side"], opening, 1e-12, 0.0);
    assert_close(r["average_side"], average, 1e-12, 0.0);
    assert_close(r["fill_ratio"], fill, 1e-12, 0.0);
    let inductance = 2.0 * PI * 1e-7 * 4.0 * average * 1.27
        * ((2.07 / fill).ln() + 0.18 * fill + 0.13 * fill * fill);
    assert_close(r["inductance"], inductance, 1e-12, 0.0);
}

#[test]
fn spiral_invalid_opening_spacing_and_turns_then_valid_recovery() {
    for (key, bad) in [
        ("outer_side", 0.0), ("outer_side", -1.0),
        ("outer_side", 0.000128), ("outer_side", 0.00012),
        ("turns", 0.0), ("turns", 3.5), ("trace_width", 0.0),
        ("trace_width", -1.0), ("spacing", 0.0), ("spacing", -1.0),
        ("spacing", 0.000049),
    ] {
        let mut inputs = SPIRAL.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = bad;
        assert!(calculate("cores.square_planar_spiral", &inputs).is_err(), "accepted {key}={bad}");
        let r = calculate("cores.square_planar_spiral", SPIRAL).unwrap();
        for &(key, expected) in SPIRAL_OUT { assert_close(r[key], expected, 1e-12, 0.0); }
    }
}

#[test]
fn bobbin_complete_discrete_geometry_oracle_and_scaling() {
    // In units of 1/1024 m: usable width=12, finished diameter=1.5,
    // copper diameter=1, height=8. Seventeen turns need 8+8+1 positions.
    for scale in [1.0, 0.5, 2.0] {
        let mut inputs = BOBBIN.to_vec();
        for (key, value) in &mut inputs {
            if *key != "turns" { *value *= scale; }
        }
        let r = calculate("cores.bobbin_layer_fit", &inputs).unwrap();
        for &(key, expected) in BOBBIN_OUT {
            let factor = if key == "radial_build" || key == "height_margin" { scale } else { 1.0 };
            assert_close(r[key], expected * factor, 1e-12, 0.0);
        }
    }
}

#[test]
fn bobbin_partial_layer_jump_and_interlayer_only_tape() {
    for (turns, layers, build) in [
        (8.0, 1.0, 0.00146484375), (16.0, 2.0, 0.003173828125),
        (17.0, 3.0, 0.0048828125),
    ] {
        let mut inputs = BOBBIN.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == "turns").unwrap().1 = turns;
        let r = calculate("cores.bobbin_layer_fit", &inputs).unwrap();
        assert_close(r["turns_per_layer"], 8.0, 0.0, 0.0);
        assert_close(r["layers_required"], layers, 0.0, 0.0);
        assert_close(r["radial_build"], build, 0.0, 0.0);
        assert_close(r["height_margin"], 0.0078125 - build, 0.0, 0.0);
        assert_close(r["copper_fill"], turns * PI / 384.0, 1e-12, 0.0);
    }
}

#[test]
fn bobbin_exact_radial_fit_zero_margins_and_zero_tape() {
    let mut inputs = BOBBIN.to_vec();
    inputs.iter_mut().find(|(k, _)| *k == "window_height").unwrap().1 = 0.0048828125;
    let r = calculate("cores.bobbin_layer_fit", &inputs).unwrap();
    assert_close(r["turns_per_layer"], 8.0, 0.0, 0.0);
    assert_close(r["layers_required"], 3.0, 0.0, 0.0);
    assert_close(r["radial_build"], 0.0048828125, 0.0, 0.0);
    assert_close(r["height_margin"], 0.0, 0.0, 0.0);
    assert_close(r["copper_fill"], 17.0 * PI / 240.0, 1e-12, 0.0);
    // One bare wire exactly fills a square window: fill=pi/4, no tape or margins.
    let r = calculate("cores.bobbin_layer_fit", &[
        ("window_width", 0.001953125), ("window_height", 0.001953125),
        ("end_margin", 0.0), ("wire_diameter", 0.001953125),
        ("copper_diameter", 0.001953125), ("tape_thickness", 0.0), ("turns", 1.0),
    ]).unwrap();
    assert_close(r["turns_per_layer"], 1.0, 0.0, 0.0);
    assert_close(r["layers_required"], 1.0, 0.0, 0.0);
    assert_close(r["radial_build"], 0.001953125, 0.0, 0.0);
    assert_close(r["height_margin"], 0.0, 0.0, 0.0);
    assert_close(r["copper_fill"], PI / 4.0, 1e-12, 0.0);
}

#[test]
fn bobbin_invalid_fit_and_scalar_guards_then_valid_recovery() {
    for (key, bad) in [
        ("window_width", 0.0), ("window_width", -1.0),
        ("window_width", 0.00390625), ("window_width", 0.004),
        ("window_height", 0.0), ("window_height", -1.0), ("window_height", 0.0048),
        ("end_margin", -1.0), ("wire_diameter", 0.0), ("wire_diameter", -1.0),
        ("copper_diameter", 0.0), ("copper_diameter", -1.0), ("copper_diameter", 0.0015),
        ("tape_thickness", -1.0), ("turns", 0.0), ("turns", 17.5),
    ] {
        let mut inputs = BOBBIN.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = bad;
        assert!(calculate("cores.bobbin_layer_fit", &inputs).is_err(), "accepted {key}={bad}");
        let r = calculate("cores.bobbin_layer_fit", BOBBIN).unwrap();
        for &(key, expected) in BOBBIN_OUT { assert_close(r[key], expected, 1e-12, 0.0); }
    }
}

#[test]
fn steinmetz_fractional_exponents_and_volume_extensivity() {
    // (1/4)^(3/2)=1/8 and 4^(5/2)=32: measured 36 kW/m^3 becomes 144 kW/m^3.
    for (volume, heat) in [(0.000025, 3.6), (0.00005, 7.2), (1e-200, 1.44e-195)] {
        let mut inputs = STEINMETZ.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == "core_volume").unwrap().1 = volume;
        let r = calculate("cores.sinusoidal_steinmetz_loss", &inputs).unwrap();
        assert_close(r["loss_density"], 144000.0, 1e-12, 0.0);
        assert_close(r["core_loss"], heat, 1e-12, 0.0);
        assert_close(r["energy_per_cycle"], heat / 40000.0, 1e-12, 0.0);
    }
}

#[test]
fn steinmetz_reference_identity_and_all_inclusive_fit_endpoints() {
    for (frequency, flux, density) in [
        (160000.0, 0.05, 36000.0),
        (20000.0, 0.025, 281.25),
        (200000.0, 0.25, 2812500.0),
    ] {
        let mut inputs = STEINMETZ.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == "frequency").unwrap().1 = frequency;
        inputs.iter_mut().find(|(k, _)| *k == "flux_peak").unwrap().1 = flux;
        // The reference itself sits on both lower or upper bounds when equal.
        if frequency == 160000.0 {
            for key in ["fit_frequency_min", "fit_frequency_max"] {
                inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = frequency;
            }
            for key in ["fit_flux_min", "fit_flux_max"] {
                inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = flux;
            }
        }
        let r = calculate("cores.sinusoidal_steinmetz_loss", &inputs).unwrap();
        assert_close(r["loss_density"], density, 1e-12, 0.0);
        assert_close(r["core_loss"], density * 0.000025, 1e-12, 0.0);
        assert_close(r["energy_per_cycle"], density * 0.000025 / frequency, 1e-12, 0.0);
    }
}

#[test]
fn steinmetz_rejects_uncalibrated_points_and_nonpositive_inputs_then_recovers() {
    for &(key, _) in STEINMETZ {
        for bad in [0.0, -1.0] {
            let mut inputs = STEINMETZ.to_vec();
            inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = bad;
            assert!(calculate("cores.sinusoidal_steinmetz_loss", &inputs).is_err(), "accepted {key}={bad}");
            let r = calculate("cores.sinusoidal_steinmetz_loss", STEINMETZ).unwrap();
            for &(key, expected) in STEINMETZ_OUT { assert_close(r[key], expected, 1e-12, 0.0); }
        }
    }
    for (key, bad) in [
        ("frequency", 19999.0), ("frequency", 200001.0),
        ("reference_frequency", 19999.0), ("reference_frequency", 200001.0),
        ("flux_peak", 0.024), ("flux_peak", 0.251),
        ("reference_flux_peak", 0.024), ("reference_flux_peak", 0.251),
        ("fit_frequency_min", 200001.0), ("fit_flux_min", 0.251),
    ] {
        let mut inputs = STEINMETZ.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = bad;
        assert!(calculate("cores.sinusoidal_steinmetz_loss", &inputs).is_err(), "accepted {key}={bad}");
        let r = calculate("cores.sinusoidal_steinmetz_loss", STEINMETZ).unwrap();
        for &(key, expected) in STEINMETZ_OUT { assert_close(r[key], expected, 1e-12, 0.0); }
    }
}

#[test]
fn steinmetz_balanced_power_ratios_keep_representable_results() {
    // Arithmetic conditioning, not a real material calibration: f^2*B^2=1.
    let r = calculate("cores.sinusoidal_steinmetz_loss", &[
        ("frequency", 1e160), ("flux_peak", 1e-160),
        ("reference_frequency", 1.0), ("reference_flux_peak", 1.0),
        ("reference_loss_density", 3.0), ("frequency_exponent", 2.0),
        ("flux_exponent", 2.0), ("core_volume", 2.0),
        ("fit_frequency_min", 1.0), ("fit_frequency_max", 1e160),
        ("fit_flux_min", 1e-160), ("fit_flux_max", 1.0),
    ]).expect("balanced powers must not overflow before multiplication");
    assert_close(r["loss_density"], 3.0, 1e-12, 0.0);
    assert_close(r["core_loss"], 6.0, 1e-12, 0.0);
    assert_close(r["energy_per_cycle"], 6e-160, 1e-12, 0.0);
}

#[test]
fn lamination_complete_faraday_joule_oracle() {
    // E_rms(y)=omega*Bpk*y/sqrt(2); average y^2=t^2/12.
    // Thus average Joule density is (omega*Bpk*t)^2/(24*rho), not the RMS-B answer.
    let omega: f64 = 240.0 * PI;
    let electric_field_second_moment = (omega * 0.6 * 0.00015).powi(2) / 24.0;
    let r = calculate("cores.lamination_eddy_loss", LAMINATION).unwrap();
    assert_close(r["penetration_ratio"], LAMINATION_OUT[0].1, 1e-12, 0.0);
    assert_close(r["eddy_loss_density"], electric_field_second_moment / 8e-7, 1e-12, 0.0);
    assert_close(r["eddy_loss"], electric_field_second_moment / 8e-7 * 0.0002, 1e-12, 0.0);
}

#[test]
fn lamination_thickness_frequency_flux_material_and_volume_scaling() {
    for (key, value, penetration_factor, density_factor, heat_factor) in [
        ("lamination_thickness", 0.000075, 0.5, 0.25, 0.25),
        ("frequency", 240.0, 2.0_f64.sqrt(), 4.0, 4.0),
        ("flux_peak", 0.3, 1.0, 0.25, 0.25),
        ("resistivity", 3.2e-6, 0.5, 0.25, 0.25),
        ("relative_permeability", 1600.0, 2.0, 1.0, 1.0),
        ("metal_volume", 0.0004, 1.0, 1.0, 2.0),
    ] {
        let mut inputs = LAMINATION.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = value;
        let r = calculate("cores.lamination_eddy_loss", &inputs).unwrap();
        assert_close(r["penetration_ratio"], LAMINATION_OUT[0].1 * penetration_factor, 1e-12, 0.0);
        assert_close(r["eddy_loss_density"], LAMINATION_OUT[1].1 * density_factor, 1e-12, 0.0);
        assert_close(r["eddy_loss"], LAMINATION_OUT[2].1 * heat_factor, 1e-12, 0.0);
    }
}

#[test]
fn lamination_dc_zero_flux_and_inclusive_material_and_penetration_limits() {
    for (key, penetration) in [("frequency", 0.0), ("flux_peak", LAMINATION_OUT[0].1)] {
        let mut inputs = LAMINATION.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = 0.0;
        let r = calculate("cores.lamination_eddy_loss", &inputs).unwrap();
        assert_close(r["penetration_ratio"], penetration, 1e-12, 0.0);
        assert_close(r["eddy_loss_density"], 0.0, 0.0, 0.0);
        assert_close(r["eddy_loss"], 0.0, 0.0, 0.0);
    }
    let thickness: f64 = 0.00015;
    let frequency = (0.3 / thickness).powi(2) * 8e-7 / (4.0 * PI * PI * 1e-7 * 400.0);
    let r = calculate("cores.lamination_eddy_loss", &[
        ("frequency", frequency), ("flux_peak", 0.9),
        ("lamination_thickness", thickness), ("resistivity", 8e-7),
        ("relative_permeability", 400.0), ("metal_volume", 0.0002),
        ("linear_flux_limit", 0.9),
    ]).unwrap();
    let density = (2.0 * PI * frequency * 0.9 * thickness).powi(2) / (24.0 * 8e-7);
    assert_close(r["penetration_ratio"], 0.3, 1e-12, 0.0);
    assert_close(r["eddy_loss_density"], density, 1e-12, 0.0);
    assert_close(r["eddy_loss"], density * 0.0002, 1e-12, 0.0);
    let mut inputs = LAMINATION.to_vec();
    inputs.iter_mut().find(|(k, _)| *k == "relative_permeability").unwrap().1 = 1.0;
    let r = calculate("cores.lamination_eddy_loss", &inputs).unwrap();
    assert_close(r["penetration_ratio"], LAMINATION_OUT[0].1 / 20.0, 1e-12, 0.0);
    assert_close(r["eddy_loss_density"], LAMINATION_OUT[1].1, 1e-12, 0.0);
    assert_close(r["eddy_loss"], LAMINATION_OUT[2].1, 1e-12, 0.0);
}

#[test]
fn lamination_invalid_material_thickness_and_flux_then_valid_recovery() {
    for (key, bad) in [
        ("frequency", -1.0), ("frequency", 2100.0), ("flux_peak", -1.0),
        ("flux_peak", 0.901), ("lamination_thickness", 0.0),
        ("lamination_thickness", -1.0), ("lamination_thickness", 0.001),
        ("resistivity", 0.0), ("resistivity", -1.0),
        ("relative_permeability", 0.999), ("metal_volume", 0.0),
        ("metal_volume", -1.0), ("linear_flux_limit", 0.0), ("linear_flux_limit", -1.0),
    ] {
        let mut inputs = LAMINATION.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = bad;
        assert!(calculate("cores.lamination_eddy_loss", &inputs).is_err(), "accepted {key}={bad}");
        let r = calculate("cores.lamination_eddy_loss", LAMINATION).unwrap();
        for &(key, expected) in LAMINATION_OUT { assert_close(r[key], expected, 1e-12, 0.0); }
    }
}

#[test]
fn lamination_dimensional_heat_survives_density_underflow() {
    // Pure conditioning probe: density rounds to zero, but density*volume is representable.
    let r = calculate("cores.lamination_eddy_loss", &[
        ("frequency", 120.0), ("flux_peak", 1e-170),
        ("lamination_thickness", 0.00015), ("resistivity", 8e-7),
        ("relative_permeability", 400.0), ("metal_volume", 1e100),
        ("linear_flux_limit", 0.9),
    ]).unwrap();
    assert_close(r["penetration_ratio"], LAMINATION_OUT[0].1, 1e-12, 0.0);
    assert_close(r["eddy_loss_density"], 0.0, 0.0, 0.0);
    assert_close(r["eddy_loss"], 6.661982970735316e-238, 1e-12, 0.0);
}

#[test]
fn foil_complete_layer_summed_oracle() {
    // Four layer-average squared fields are 1/3, 7/3, 19/3, 37/3.
    // Their average 16/3 enters the proximity term; the asymptotic coefficient
    // is 16/9 - 1/45 = 79/45. This is not the single-layer coefficient 4/45.
    let layer_average = (1.0 + 7.0 + 19.0 + 37.0) / 12.0;
    let coefficient = layer_average / 3.0 - 1.0 / 45.0;
    let x_squared = 0.004 * PI * PI;
    let excess = coefficient * x_squared * x_squared;
    let r = calculate("cores.thin_foil_winding_loss", FOIL).unwrap();
    assert_close(r["thickness_ratio"], x_squared.sqrt(), 1e-12, 0.0);
    assert_close(r["resistance_dc"], 0.012, 1e-12, 0.0);
    assert_close(r["ac_excess_factor"], excess, 1e-12, 0.0);
    assert_close(r["resistance_ac"], 0.012 * (1.0 + excess), 1e-12, 0.0);
    assert_close(r["winding_loss"], 0.588 * (1.0 + excess), 1e-12, 0.0);
    assert_close(r["ac_excess_loss"], 0.588 * excess, 1e-12, 0.0);
}

#[test]
fn foil_dc_and_zero_current_preserve_resistances() {
    for (key, x, excess, rac, heat) in [
        ("frequency", 0.0, 0.0, 0.012, 0.588),
        ("current_rms", FOIL_OUT[0].1, FOIL_OUT[2].1, FOIL_OUT[3].1, 0.0),
    ] {
        let mut inputs = FOIL.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = 0.0;
        let r = calculate("cores.thin_foil_winding_loss", &inputs).unwrap();
        assert_close(r["thickness_ratio"], x, 1e-12, 0.0);
        assert_close(r["resistance_dc"], 0.012, 1e-12, 0.0);
        assert_close(r["ac_excess_factor"], excess, 1e-12, 0.0);
        assert_close(r["resistance_ac"], rac, 1e-12, 0.0);
        assert_close(r["winding_loss"], heat, 1e-12, 0.0);
        assert_close(r["ac_excess_loss"], 0.0, 0.0, 0.0);
    }
}

#[test]
fn foil_frequency_current_and_series_layer_scaling() {
    for (key, value, x_factor, dc_factor, excess_factor, current_squared_factor) in [
        ("frequency", 62500.0, 1.0 / 2.0_f64.sqrt(), 1.0, 0.25, 1.0),
        ("current_rms", 14.0, 1.0, 1.0, 1.0, 4.0),
        ("layers", 1.0, 1.0, 0.25, 4.0 / 79.0, 1.0),
        ("layers", 8.0, 1.0, 2.0, 319.0 / 79.0, 1.0),
    ] {
        let mut inputs = FOIL.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = value;
        let r = calculate("cores.thin_foil_winding_loss", &inputs).unwrap();
        let dc = 0.012 * dc_factor;
        let excess = FOIL_OUT[2].1 * excess_factor;
        assert_close(r["thickness_ratio"], FOIL_OUT[0].1 * x_factor, 1e-12, 0.0);
        assert_close(r["resistance_dc"], dc, 1e-12, 0.0);
        assert_close(r["ac_excess_factor"], excess, 1e-12, 0.0);
        assert_close(r["resistance_ac"], dc * (1.0 + excess), 1e-12, 0.0);
        assert_close(r["winding_loss"], 49.0 * current_squared_factor * dc * (1.0 + excess), 1e-12, 0.0);
        assert_close(r["ac_excess_loss"], 49.0 * current_squared_factor * dc * excess, 1e-12, 0.0);
    }
}

#[test]
fn foil_inclusive_thickness_and_width_screens() {
    let thickness: f64 = 0.00004;
    let frequency = (0.5 / thickness).powi(2) * 2e-8 / (4.0 * PI * PI * 1e-7);
    let r = calculate("cores.thin_foil_winding_loss", &[
        ("frequency", frequency), ("foil_thickness", thickness),
        ("foil_width", 20.0 * thickness), ("layers", 4.0),
        ("mean_turn_length", 0.09), ("resistivity", 2e-8), ("current_rms", 7.0),
    ]).unwrap();
    let excess = 79.0 / 720.0;
    assert_close(r["thickness_ratio"], 0.5, 1e-12, 0.0);
    assert_close(r["resistance_dc"], 0.225, 1e-12, 0.0);
    assert_close(r["ac_excess_factor"], excess, 1e-12, 0.0);
    assert_close(r["resistance_ac"], 0.225 * (1.0 + excess), 1e-12, 0.0);
    assert_close(r["winding_loss"], 11.025 * (1.0 + excess), 1e-12, 0.0);
    assert_close(r["ac_excess_loss"], 11.025 * excess, 1e-12, 0.0);
}

#[test]
fn foil_tiny_dimensionless_excess_does_not_erase_dimensional_heat() {
    let r = calculate("cores.thin_foil_winding_loss", &[
        ("frequency", 1.25e-165), ("foil_thickness", 0.00004),
        ("foil_width", 0.015), ("layers", 4.0), ("mean_turn_length", 0.09),
        ("resistivity", 2e-8), ("current_rms", 1e100),
    ]).unwrap();
    // Frequency ratio=1e-170: x scales 1e-85, excess scales 1e-340.
    // 80-digit decimal arithmetic gives finite I^2*Rdc*excess even though excess=0 in f64.
    assert_close(r["thickness_ratio"], 1.9869176531592203e-86, 1e-12, 0.0);
    assert_close(r["resistance_dc"], 0.012, 1e-12, 0.0);
    assert_close(r["ac_excess_factor"], 0.0, 0.0, 0.0);
    assert_close(r["resistance_ac"], 0.012, 1e-12, 0.0);
    assert_close(r["winding_loss"], 1.2e198, 1e-12, 0.0);
    assert_close(r["ac_excess_loss"], 3.283335761786109e-145, 1e-12, 0.0);
}

#[test]
fn foil_invalid_current_layers_geometry_and_frequency_then_valid_recovery() {
    for (key, bad) in [
        ("frequency", -1.0), ("frequency", 800000.0),
        ("foil_thickness", 0.0), ("foil_thickness", -1.0),
        ("foil_width", 0.0), ("foil_width", -1.0), ("foil_width", 0.00079),
        ("layers", 0.0), ("layers", 4.5), ("mean_turn_length", 0.0),
        ("mean_turn_length", -1.0), ("resistivity", 0.0),
        ("resistivity", -1.0), ("current_rms", -1.0),
    ] {
        let mut inputs = FOIL.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = bad;
        assert!(calculate("cores.thin_foil_winding_loss", &inputs).is_err(), "accepted {key}={bad}");
        let r = calculate("cores.thin_foil_winding_loss", FOIL).unwrap();
        for &(key, expected) in FOIL_OUT { assert_close(r[key], expected, 1e-12, 0.0); }
    }
}

#[test]
fn every_model_rejects_every_nonfinite_input_then_recovers_all_outputs() {
    for (id, inputs, expected) in [
        ("cores.single_layer_air_solenoid", SOLENOID, SOLENOID_OUT),
        ("cores.square_planar_spiral", SPIRAL, SPIRAL_OUT),
        ("cores.bobbin_layer_fit", BOBBIN, BOBBIN_OUT),
        ("cores.sinusoidal_steinmetz_loss", STEINMETZ, STEINMETZ_OUT),
        ("cores.lamination_eddy_loss", LAMINATION, LAMINATION_OUT),
        ("cores.thin_foil_winding_loss", FOIL, FOIL_OUT),
    ] {
        for index in 0..inputs.len() {
            for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                let mut malformed = inputs.to_vec();
                malformed[index].1 = bad;
                assert!(calculate(id, &malformed).is_err(), "{id} accepted {}={bad}", malformed[index].0);
                let r = calculate(id, inputs).unwrap();
                for &(key, value) in expected { assert_close(r[key], value, 1e-12, 0.0); }
            }
        }
    }
}
