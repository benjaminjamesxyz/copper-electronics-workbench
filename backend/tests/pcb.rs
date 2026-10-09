mod common;

use common::{assert_close, calculate};

// Research oracles: research/pcb.md, TI SLUP224/TIDUCS7/SNAA103/SPRAAV2A,
// Eurocircuits annular-ring geometry, and the 35 um/oz manufacturing convention.
// Nonzero outputs use the researched 1e-6 relative tolerance with no absolute
// floor: a picosecond result must not pass merely because it is small. Exact
// zero products/differences use absolute=0. Decimal annular tangency alone uses
// 1e-19 m (sub-femtometre roundoff, not a manufacturing acceptance allowance).

#[test]
fn trace_complete_researched_example() {
    let out = calculate("pcb.trace_resistance_drop", &[
        ("length", 0.06), ("width", 0.002), ("thickness", 70e-6),
        ("resistivity", 1.7e-8), ("current", 4.0),
    ]).unwrap();
    assert_close(out["cross_section"], 1.4e-7, 1e-6, 0.0);
    assert_close(out["resistance"], 0.007285714285714286, 1e-6, 0.0);
    assert_close(out["voltage_drop"], 0.029142857142857144, 1e-6, 0.0);
}

#[test]
fn trace_alternate_geometry_and_zero_current() {
    // 100 mm x 1 mm x 100 um: A=0.1 mm^2; rho=20 nOhm m gives
    // R=20 mOhm. The conductor is resistive even with no applied current.
    for (current, drop) in [(2.0, 0.04), (0.0, 0.0)] {
        let out = calculate("pcb.trace_resistance_drop", &[
            ("length", 0.1), ("width", 0.001), ("thickness", 0.0001),
            ("resistivity", 2e-8), ("current", current),
        ]).unwrap();
        assert_close(out["cross_section"], 1e-7, 1e-6, 0.0);
        assert_close(out["resistance"], 0.02, 1e-6, 0.0);
        assert_close(out["voltage_drop"], drop, 1e-6, 0.0);
    }
}

#[test]
fn trace_rejects_scalar_domains_and_collapsed_section() {
    let valid = [
        ("length", 0.06), ("width", 0.002), ("thickness", 70e-6),
        ("resistivity", 1.7e-8), ("current", 4.0),
    ];
    for index in 0..4 {
        for value in [0.0, -1.0] {
            let mut inputs = valid;
            inputs[index].1 = value;
            assert!(calculate("pcb.trace_resistance_drop", &inputs).is_err(),
                "{}={value} must be rejected", inputs[index].0);
        }
    }
    let mut inputs = valid;
    inputs[4].1 = -0.001;
    assert!(calculate("pcb.trace_resistance_drop", &inputs).is_err());
    // Both dimensions are positive but their f64 product is zero: the
    // cross-input denominator guard must reject before dividing by area.
    inputs = valid;
    inputs[1].1 = 1e-200;
    inputs[2].1 = 1e-200;
    assert!(calculate("pcb.trace_resistance_drop", &inputs).is_err());
    inputs = valid;
    inputs[0].1 = f64::MAX;
    inputs[3].1 = 2.0;
    assert!(calculate("pcb.trace_resistance_drop", &inputs).is_err());
}

#[test]
fn ipc_complete_researched_example() {
    let out = calculate("pcb.ipc2221_current_estimate", &[
        ("width_mil", 40.0), ("thickness_mil", 1.4), ("temperature_rise", 20.0),
    ]).unwrap();
    assert_close(out["area_mil2"], 56.0, 1e-6, 0.0);
    assert_close(out["external_current"], 3.3199439759510987, 1e-6, 0.0);
    assert_close(out["internal_current"], 1.6599719879755493, 1e-6, 0.0);
}

#[test]
fn ipc_policy_endpoints_and_layer_coefficient_identity() {
    // Unit mil^2 section removes the area exponent. At DeltaT=1 the
    // currents equal the two coefficients; at 100 the independent decimal
    // log-domain oracle is k*exp(0.88*ln(10)). Both policy endpoints are valid.
    for (rise, external, internal) in [
        (1.0, 0.048, 0.024),
        (100.0, 0.3641172360140082, 0.1820586180070041),
    ] {
        let out = calculate("pcb.ipc2221_current_estimate", &[
            ("width_mil", 2.0), ("thickness_mil", 0.5), ("temperature_rise", rise),
        ]).unwrap();
        assert_close(out["area_mil2"], 1.0, 1e-6, 0.0);
        assert_close(out["external_current"], external, 1e-6, 0.0);
        assert_close(out["internal_current"], internal, 1e-6, 0.0);
        assert_close(out["external_current"] / out["internal_current"], 2.0, 1e-6, 0.0);
    }
}

#[test]
fn ipc_rejects_geometry_and_outside_screening_policy() {
    let valid = [("width_mil", 40.0), ("thickness_mil", 1.4), ("temperature_rise", 20.0)];
    for index in 0..2 {
        for value in [0.0, -0.5] {
            let mut inputs = valid;
            inputs[index].1 = value;
            assert!(calculate("pcb.ipc2221_current_estimate", &inputs).is_err());
        }
    }
    for value in [0.0, 0.999999, 100.000001] {
        let mut inputs = valid;
        inputs[2].1 = value;
        assert!(calculate("pcb.ipc2221_current_estimate", &inputs).is_err());
    }
    let mut inputs = valid;
    inputs[0].1 = 1e-200;
    inputs[1].1 = 1e-200;
    assert!(calculate("pcb.ipc2221_current_estimate", &inputs).is_err());
}

#[test]
fn via_complete_researched_example() {
    let out = calculate("pcb.via_barrel_resistance", &[
        ("barrel_length", 0.0016), ("finished_diameter", 0.0003),
        ("plating_thickness", 25e-6), ("plated_resistivity", 6e-8), ("via_count", 4.0),
    ]).unwrap();
    assert_close(out["barrel_copper_area"], 2.552544031041707e-8, 1e-6, 0.0);
    assert_close(out["single_resistance"], 0.003760953732140788, 1e-6, 0.0);
    assert_close(out["bank_resistance"], 0.0009402384330351971, 1e-6, 0.0);
}

#[test]
fn via_thick_annulus_and_single_via_boundary() {
    // Inner diameter=0.2 mm, outer diameter=0.6 mm: disk subtraction
    // gives A=(pi/4)*(0.6^2-0.2^2)=0.08*pi mm^2. This deliberately
    // makes the p^2 term substantial and detects a thin-wall substitute.
    for (count, bank) in [(1.0, 0.00007957747154594767), (2.0, 0.000039788735772973834)] {
        let out = calculate("pcb.via_barrel_resistance", &[
            ("barrel_length", 0.001), ("finished_diameter", 0.0002),
            ("plating_thickness", 0.0002), ("plated_resistivity", 2e-8), ("via_count", count),
        ]).unwrap();
        assert_close(out["barrel_copper_area"], 2.5132741228718346e-7, 1e-6, 0.0);
        assert_close(out["single_resistance"], 0.00007957747154594767, 1e-6, 0.0);
        assert_close(out["bank_resistance"], bank, 1e-6, 0.0);
    }
}

#[test]
fn via_rejects_invalid_material_geometry_and_discrete_count() {
    let valid = [
        ("barrel_length", 0.0016), ("finished_diameter", 0.0003),
        ("plating_thickness", 25e-6), ("plated_resistivity", 6e-8), ("via_count", 4.0),
    ];
    for index in 0..4 {
        for value in [0.0, -0.001] {
            let mut inputs = valid;
            inputs[index].1 = value;
            assert!(calculate("pcb.via_barrel_resistance", &inputs).is_err());
        }
    }
    for value in [-1.0, 0.0, 0.999999, 1.5, 4.25] {
        let mut inputs = valid;
        inputs[4].1 = value;
        assert!(calculate("pcb.via_barrel_resistance", &inputs).is_err());
    }
    let mut inputs = valid;
    inputs[1].1 = 1e-200;
    inputs[2].1 = 1e-200;
    assert!(calculate("pcb.via_barrel_resistance", &inputs).is_err());
}

#[test]
fn copper_complete_researched_example() {
    let out = calculate("pcb.nominal_copper_thickness", &[("copper_weight", 2.0)]).unwrap();
    assert_close(out["nominal_thickness"], 70e-6, 1e-6, 0.0);
    assert_close(out["nominal_thickness_mil"], 2.7559055118110236, 1e-6, 0.0);
}

#[test]
fn copper_fractional_designations_and_positive_near_boundary() {
    // Half-ounce is permitted, not rounded up to an integer designation.
    // There is no included zero endpoint; a small positive designation
    // tests approach to that open boundary without a fabricated minimum.
    for (weight, metres, mil) in [
        (0.5, 17.5e-6, 0.6889763779527559),
        (0.001, 35e-9, 0.0013779527559055118),
    ] {
        let out = calculate("pcb.nominal_copper_thickness", &[("copper_weight", weight)]).unwrap();
        assert_close(out["nominal_thickness"], metres, 1e-6, 0.0);
        assert_close(out["nominal_thickness_mil"], mil, 1e-6, 0.0);
    }
}

#[test]
fn copper_rejects_nonpositive_and_nonfinite_designations() {
    for weight in [0.0, -0.5, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(calculate("pcb.nominal_copper_thickness", &[("copper_weight", weight)]).is_err());
    }
}

#[test]
fn skew_complete_researched_example() {
    let out = calculate("pcb.differential_length_skew", &[
        ("positive_length", 0.08), ("negative_length", 0.0775), ("delay_per_length", 6.84e-9),
    ]).unwrap();
    assert_close(out["signed_length_mismatch"], 0.0025, 1e-6, 0.0);
    assert_close(out["signed_skew"], 1.71e-11, 1e-6, 0.0);
    assert_close(out["skew_magnitude"], 1.71e-11, 1e-6, 0.0);
}

#[test]
fn skew_signed_swap_equal_legs_and_zero_length_boundaries() {
    // Independent binary-exact lengths: mismatch +/-0.0625 m, giving
    // +/-0.5 ns at 8 ns/m. Common length cancels even when nonzero.
    for (positive, negative, mismatch, signed, magnitude) in [
        (0.125, 0.0625, 0.0625, 5e-10, 5e-10),
        (0.0625, 0.125, -0.0625, -5e-10, 5e-10),
        (0.125, 0.125, 0.0, 0.0, 0.0),
        (0.0, 0.0, 0.0, 0.0, 0.0),
        (0.0, 0.0625, -0.0625, -5e-10, 5e-10),
        (0.0625, 0.0, 0.0625, 5e-10, 5e-10),
        (0.0, 1e-9, -1e-9, -8e-18, 8e-18),
    ] {
        let out = calculate("pcb.differential_length_skew", &[
            ("positive_length", positive), ("negative_length", negative), ("delay_per_length", 8e-9),
        ]).unwrap();
        assert_close(out["signed_length_mismatch"], mismatch, 1e-6, 0.0);
        assert_close(out["signed_skew"], signed, 1e-6, 0.0);
        assert_close(out["skew_magnitude"], magnitude, 1e-6, 0.0);
    }
}

#[test]
fn skew_rejects_negative_lengths_and_nonpositive_delay_density() {
    let valid = [("positive_length", 0.08), ("negative_length", 0.0775), ("delay_per_length", 6.84e-9)];
    for index in 0..2 {
        let mut inputs = valid;
        inputs[index].1 = -1e-6;
        assert!(calculate("pcb.differential_length_skew", &inputs).is_err());
    }
    for value in [0.0, -1e-9] {
        let mut inputs = valid;
        inputs[2].1 = value;
        assert!(calculate("pcb.differential_length_skew", &inputs).is_err());
    }
}

#[test]
fn annular_complete_researched_example() {
    let out = calculate("pcb.annular_ring_registration", &[
        ("pad_diameter", 0.0008), ("tool_diameter", 0.0004), ("registration_offset", 0.00005),
    ]).unwrap();
    assert_close(out["nominal_ring"], 0.0002, 1e-6, 0.0);
    assert_close(out["minimum_ring"], 0.00015, 1e-6, 0.0);
}

#[test]
fn annular_concentric_alternate_and_tangent_boundary() {
    // Radius difference=(1.0-0.5)/2 mm=0.25 mm. Concentric and
    // tangential holes are valid geometry, not manufacturing certification.
    for (offset, minimum) in [(0.0, 0.00025), (0.000125, 0.000125), (0.00025, 0.0)] {
        let out = calculate("pcb.annular_ring_registration", &[
            ("pad_diameter", 0.001), ("tool_diameter", 0.0005), ("registration_offset", offset),
        ]).unwrap();
        assert_close(out["nominal_ring"], 0.00025, 1e-6, 0.0);
        let absolute = if minimum == 0.0 { 1e-19 } else { 0.0 };
        assert_close(out["minimum_ring"], minimum, 1e-6, absolute);
    }
}

#[test]
fn annular_rejects_scalar_domains_reversed_geometry_and_breakout() {
    let valid = [("pad_diameter", 0.0008), ("tool_diameter", 0.0004), ("registration_offset", 0.00005)];
    for index in 0..2 {
        for value in [0.0, -0.0001] {
            let mut inputs = valid;
            inputs[index].1 = value;
            assert!(calculate("pcb.annular_ring_registration", &inputs).is_err());
        }
    }
    let mut inputs = valid;
    inputs[2].1 = -1e-6;
    assert!(calculate("pcb.annular_ring_registration", &inputs).is_err());
    for tool in [0.0008, 0.0009] {
        // Zero offset isolates the strictly larger pad condition.
        assert!(calculate("pcb.annular_ring_registration", &[
            ("pad_diameter", 0.0008), ("tool_diameter", tool), ("registration_offset", 0.0),
        ]).is_err());
    }
    for offset in [0.00020000001, 0.0003] {
        inputs = valid;
        inputs[2].1 = offset;
        assert!(calculate("pcb.annular_ring_registration", &inputs).is_err());
    }
}

#[test]
fn joule_complete_researched_example() {
    let out = calculate("pcb.trace_joule_heat_density", &[
        ("length", 0.05), ("width", 0.001), ("thickness", 35e-6),
        ("resistivity", 1.7e-8), ("pulse_current", 3.0), ("duration", 0.2),
    ]).unwrap();
    assert_close(out["current_density"], 85714285.71428572, 1e-6, 0.0);
    assert_close(out["volumetric_heat_rate"], 124897959.18367347, 1e-6, 0.0);
    assert_close(out["generated_power"], 0.21857142857142858, 1e-6, 0.0);
    assert_close(out["deposited_energy"], 0.04371428571428571, 1e-6, 0.0);
}

#[test]
fn joule_alternate_integral_current_square_and_zero_boundaries() {
    // A=0.1 mm^2, volume=10 mm^3 and R=0.02 ohm. Independent
    // total-power oracle I^2 R: 2 A -> 0.08 W; 4 A -> 0.32 W.
    // Zero duration removes energy only; zero current removes every output.
    for (current, duration, density, rate, power, energy) in [
        (2.0, 0.5, 2e7, 8e6, 0.08, 0.04),
        (4.0, 0.5, 4e7, 32e6, 0.32, 0.16),
        (2.0, 0.0, 2e7, 8e6, 0.08, 0.0),
        (0.0, 0.5, 0.0, 0.0, 0.0, 0.0),
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ] {
        let out = calculate("pcb.trace_joule_heat_density", &[
            ("length", 0.1), ("width", 0.001), ("thickness", 0.0001),
            ("resistivity", 2e-8), ("pulse_current", current), ("duration", duration),
        ]).unwrap();
        assert_close(out["current_density"], density, 1e-6, 0.0);
        assert_close(out["volumetric_heat_rate"], rate, 1e-6, 0.0);
        assert_close(out["generated_power"], power, 1e-6, 0.0);
        assert_close(out["deposited_energy"], energy, 1e-6, 0.0);
    }
}

#[test]
fn joule_rejects_material_geometry_negative_pulse_and_collapsed_section() {
    let valid = [
        ("length", 0.05), ("width", 0.001), ("thickness", 35e-6),
        ("resistivity", 1.7e-8), ("pulse_current", 3.0), ("duration", 0.2),
    ];
    for index in 0..4 {
        for value in [0.0, -0.001] {
            let mut inputs = valid;
            inputs[index].1 = value;
            assert!(calculate("pcb.trace_joule_heat_density", &inputs).is_err());
        }
    }
    for index in 4..6 {
        let mut inputs = valid;
        inputs[index].1 = -0.001;
        assert!(calculate("pcb.trace_joule_heat_density", &inputs).is_err());
    }
    let mut inputs = valid;
    inputs[1].1 = 1e-200;
    inputs[2].1 = 1e-200;
    assert!(calculate("pcb.trace_joule_heat_density", &inputs).is_err());
    inputs = valid;
    inputs[4].1 = 1e200;
    assert!(calculate("pcb.trace_joule_heat_density", &inputs).is_err());
}

#[test]
fn stencil_complete_researched_example() {
    let out = calculate("pcb.stencil_aperture_area_ratio", &[
        ("aperture_length", 0.0005), ("aperture_width", 0.0003),
        ("stencil_thickness", 0.0001), ("target_area_ratio", 0.66),
    ]).unwrap();
    assert_close(out["opening_area"], 1.5e-7, 1e-6, 0.0);
    assert_close(out["wall_area"], 1.6e-7, 1e-6, 0.0);
    assert_close(out["area_ratio"], 0.9375, 1e-6, 0.0);
    assert_close(out["ratio_margin"], 0.2775, 1e-6, 0.0);
}

#[test]
fn stencil_square_identity_and_signed_target_boundary() {
    // Square opening: AR=side/(4*thickness). Binary-exact SI inputs
    // make the zero margin exact; positive/negative margins must survive.
    for (target, margin) in [(0.25, 0.25), (0.5, 0.0), (0.75, -0.25)] {
        let out = calculate("pcb.stencil_aperture_area_ratio", &[
            ("aperture_length", 1.0 / 1024.0), ("aperture_width", 1.0 / 1024.0),
            ("stencil_thickness", 1.0 / 2048.0), ("target_area_ratio", target),
        ]).unwrap();
        assert_close(out["opening_area"], 1.0 / 1048576.0, 1e-6, 0.0);
        assert_close(out["wall_area"], 1.0 / 524288.0, 1e-6, 0.0);
        assert_close(out["area_ratio"], 0.5, 1e-6, 0.0);
        assert_close(out["ratio_margin"], margin, 1e-6, 0.0);
    }
}

#[test]
fn stencil_orientation_invariance() {
    // Rotating the rectangular aperture changes neither its opening nor
    // the sum of its four wall rectangles; no length/width special case.
    let out = calculate("pcb.stencil_aperture_area_ratio", &[
        ("aperture_length", 0.0003), ("aperture_width", 0.0005),
        ("stencil_thickness", 0.0001), ("target_area_ratio", 1.0),
    ]).unwrap();
    assert_close(out["opening_area"], 1.5e-7, 1e-6, 0.0);
    assert_close(out["wall_area"], 1.6e-7, 1e-6, 0.0);
    assert_close(out["area_ratio"], 0.9375, 1e-6, 0.0);
    assert_close(out["ratio_margin"], -0.0625, 1e-6, 0.0);
}

#[test]
fn stencil_rejects_nonpositive_dimensions_target_and_collapsed_walls() {
    let valid = [
        ("aperture_length", 0.0005), ("aperture_width", 0.0003),
        ("stencil_thickness", 0.0001), ("target_area_ratio", 0.66),
    ];
    for index in 0..4 {
        for value in [0.0, -0.001] {
            let mut inputs = valid;
            inputs[index].1 = value;
            assert!(calculate("pcb.stencil_aperture_area_ratio", &inputs).is_err());
        }
    }
    let inputs = [
        ("aperture_length", 1e-200), ("aperture_width", 1e-200),
        ("stencil_thickness", 1e-200), ("target_area_ratio", 0.66),
    ];
    assert!(calculate("pcb.stencil_aperture_area_ratio", &inputs).is_err());
}
