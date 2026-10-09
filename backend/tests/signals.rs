mod common;

use common::{assert_close, calculate};

// Explicit source-research vectors, not defaults read from the runtime catalog.
const MICROSTRIP: &[(&str, f64)] = &[
    ("width", 0.0008), ("height", 0.0004), ("thickness", 0.000002),
    ("relative_permittivity", 3.66), ("frequency", 1e9),
];
const STRIPLINE: &[(&str, f64)] = &[
    ("width", 0.0006), ("plane_spacing", 0.001), ("thickness", 0.000018),
    ("relative_permittivity", 4.0),
];
const COAX: &[(&str, f64)] = &[
    ("inner_radius", 0.0005), ("shield_radius", 0.0015), ("relative_permittivity", 2.25),
];
const PROPAGATION: &[(&str, f64)] = &[
    ("length", 0.15), ("effective_permittivity", 3.0), ("frequency", 1e9),
];
const REFLECTION: &[(&str, f64)] = &[
    ("line_impedance", 50.0), ("load_resistance", 75.0), ("incident_voltage", 1.2),
];
const TERMINATION: &[(&str, f64)] = &[
    ("line_impedance", 60.0), ("driver_resistance", 18.0), ("step_voltage", 3.3),
];
const QUARTER_WAVE: &[(&str, f64)] = &[
    ("source_resistance", 50.0), ("load_resistance", 100.0),
    ("frequency", 2.4e9), ("effective_permittivity", 2.25),
];

#[test]
fn microstrip_complete_researched_example() {
    let out = calculate("signals.microstrip_impedance", MICROSTRIP).unwrap();
    for (key, expected) in [
        ("width_ratio", 2.0), ("fit_a", 0.9995302713283848),
        ("fit_b", 0.5382736881490864), ("air_factor", 6.000115105754384),
        ("effective_permittivity", 2.8372126159506297),
        ("air_impedance", 89.09056365708382), ("impedance", 52.89148559435723),
    ] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn microstrip_vacuum_and_inclusive_thickness_electrical_boundaries() {
    // u=1, er=1: all fields are in air, so ee=1 and Z0=Zair.
    // Both conductor ratios and h*f/c attain their inclusive 1% limits.
    let out = calculate("signals.microstrip_impedance", &[
        ("width", 1.0), ("height", 1.0), ("thickness", 0.01),
        ("relative_permittivity", 1.0), ("frequency", 2997924.58),
    ]).unwrap();
    for (key, expected) in [
        ("width_ratio", 1.0), ("fit_a", 0.992688562101591),
        ("fit_b", 0.463841843886721), ("air_factor", 6.000000547112273),
        ("effective_permittivity", 1.0), ("air_impedance", 126.51138655581163),
        ("impedance", 126.51138655581163),
    ] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn microstrip_width_fit_endpoints_and_zero_thickness() {
    // Goldens independently substituted in the research Hammerstad-Jensen fit.
    for (width, er, expected) in [
        (0.01, 1.0, [0.01, 0.6727721157225198, 0.463841843886721,
            6.0, 1.0, 401.0768911586108, 401.0768911586108]),
        (100.0, 128.0, [100.0, 1.2745287157696419, 0.5630972929326404,
            6.187799023095436, 123.80166281318296, 3.61364030550529, 0.3247743203861406]),
    ] {
        let out = calculate("signals.microstrip_impedance", &[
            ("width", width), ("height", 1.0), ("thickness", 0.0),
            ("relative_permittivity", er), ("frequency", 1.0),
        ]).unwrap();
        for (key, value) in ["width_ratio", "fit_a", "fit_b", "air_factor",
            "effective_permittivity", "air_impedance", "impedance"].into_iter().zip(expected) {
            assert_close(out[key], value, 1e-6, 0.0);
        }
    }
}

#[test]
fn microstrip_scale_invariance_and_thickness_is_applicability_only() {
    // Doubling every dimension and halving frequency leaves both the static
    // fit and electrical thickness unchanged. Zero thickness has the same fit.
    for thickness in [0.0, 0.000004] {
        let out = calculate("signals.microstrip_impedance", &[
            ("width", 0.0016), ("height", 0.0008), ("thickness", thickness),
            ("relative_permittivity", 3.66), ("frequency", 5e8),
        ]).unwrap();
        for (key, expected) in [
            ("width_ratio", 2.0), ("fit_a", 0.9995302713283848),
            ("fit_b", 0.5382736881490864), ("air_factor", 6.000115105754384),
            ("effective_permittivity", 2.8372126159506297),
            ("air_impedance", 89.09056365708382), ("impedance", 52.89148559435723),
        ] {
            assert_close(out[key], expected, 1e-6, 0.0);
        }
    }
}

#[test]
fn microstrip_rejects_each_scalar_and_model_guard() {
    for (key, bad) in [
        ("width", 0.0), ("width", -0.0008), ("height", 0.0), ("height", -0.0004),
        ("thickness", -1e-9), ("relative_permittivity", 0.99),
        ("relative_permittivity", 128.01), ("frequency", 0.0), ("frequency", -1.0),
    ] {
        let mut inputs = MICROSTRIP.to_vec();
        inputs.iter_mut().find(|(name, _)| *name == key).unwrap().1 = bad;
        assert!(calculate("signals.microstrip_impedance", &inputs).is_err(), "{key}={bad}");
    }
    // Independently isolate u limits, t/h, t/w and electrical-thickness guard.
    for (width, height, thickness, er, frequency) in [
        (0.0099, 1.0, 0.0, 1.0, 1.0),
        (100.01, 1.0, 0.0, 1.0, 1.0),
        (2.0, 1.0, 0.0101, 1.0, 1.0),
        (0.1, 1.0, 0.00101, 1.0, 1.0),
        (1.0, 1.0, 0.0, 1.0, 3e6),
        // Bulk er=4, not an effective er, sets the applicability wavelength.
        (1.0, 1.0, 0.0, 4.0, 1.8e6),
    ] {
        assert!(calculate("signals.microstrip_impedance", &[
            ("width", width), ("height", height), ("thickness", thickness),
            ("relative_permittivity", er), ("frequency", frequency),
        ]).is_err());
    }
}

#[test]
fn stripline_complete_researched_example() {
    let out = calculate("signals.stripline_impedance", STRIPLINE).unwrap();
    for (key, expected) in [
        ("width_ratio", 0.6), ("thickness_ratio", 0.018),
        ("effective_width_ratio", 0.6), ("fringe_factor", 0.4659693534707028),
        ("impedance", 43.41181070235101),
    ] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn stripline_narrow_correction_and_wide_branch_join() {
    // Independently calculated using q=0.01 and total plane separation b=1.
    for (width, er, effective_width, impedance) in [
        (0.2, 1.0, 0.1799107142857143, 146.52366276805267),
        (0.35, 4.0, 0.35, 57.818381243270096),
    ] {
        let out = calculate("signals.stripline_impedance", &[
            ("width", width), ("plane_spacing", 1.0), ("thickness", 0.01),
            ("relative_permittivity", er),
        ]).unwrap();
        for (key, expected) in [
            ("width_ratio", width), ("thickness_ratio", 0.01),
            ("effective_width_ratio", effective_width),
            ("fringe_factor", 0.4568826885608919), ("impedance", impedance),
        ] {
            assert_close(out[key], expected, 1e-6, 0.0);
        }
    }
}

#[test]
fn stripline_admissible_edges_and_tiny_finite_thickness() {
    for (width, thickness, er, effective_width, fringe, impedance) in [
        // Immediately inside the strict width/clearance accuracy limit.
        (0.04950000000000001, 0.01, 1.0, -0.0311252232142857,
            0.4568826885608919, 219.15129952133108),
        // Immediately below the strict q=0.025 upper limit.
        (0.6, 0.024999999999999998, 1.0, 0.6,
            0.47291754931113716, 85.64645547693772),
        // q(2-q)/(1-q)^2 must retain a positive logarithm argument as q -> 0.
        (0.6, 1e-16, 4.0, 0.6, 0.4412712003053045, 45.25611559220114),
    ] {
        let out = calculate("signals.stripline_impedance", &[
            ("width", width), ("plane_spacing", 1.0), ("thickness", thickness),
            ("relative_permittivity", er),
        ]).unwrap();
        for (key, expected) in [
            ("width_ratio", width), ("thickness_ratio", thickness),
            ("effective_width_ratio", effective_width), ("fringe_factor", fringe),
            ("impedance", impedance),
        ] {
            assert_close(out[key], expected, 1e-6, 0.0);
        }
    }
}

#[test]
fn stripline_rejects_scalar_geometry_and_exclusive_fit_limits() {
    for (key, bad) in [
        ("width", 0.0), ("width", -1.0), ("plane_spacing", 0.0),
        ("plane_spacing", -1.0), ("thickness", 0.0), ("thickness", -1.0),
        ("relative_permittivity", 0.99),
    ] {
        let mut inputs = STRIPLINE.to_vec();
        inputs.iter_mut().find(|(name, _)| *name == key).unwrap().1 = bad;
        assert!(calculate("signals.stripline_impedance", &inputs).is_err(), "{key}={bad}");
    }
    for (width, spacing, thickness) in [
        (0.6, 1.0, 0.025), (0.6, 1.0, 0.03),
        (0.6, 1.0, 1.0), (0.6, 1.0, 1.1),
        (0.0495, 1.0, 0.01), (0.0494, 1.0, 0.01),
    ] {
        assert!(calculate("signals.stripline_impedance", &[
            ("width", width), ("plane_spacing", spacing), ("thickness", thickness),
            ("relative_permittivity", 4.0),
        ]).is_err());
    }
}

#[test]
fn coax_complete_researched_example() {
    let out = calculate("signals.coax_impedance", COAX).unwrap();
    assert_close(out["impedance"], 43.94449154672439, 1e-6, 0.0);
}

#[test]
fn coax_vacuum_geometry_scaling_and_dielectric_square_root() {
    // Radius ratio=2, so the vacuum result is 60*ln(2), independently 41.5888 ohm.
    for (inner, shield, er, expected) in [
        (1.0, 2.0, 1.0, 41.58883083359672),
        (0.001, 0.002, 1.0, 41.58883083359672),
        (1.0, 2.0, 4.0, 20.79441541679836),
        // Adjacent binary64 radii: ln(1+2^-52)≈2^-52, nonzero impedance.
        (1.0, 1.0000000000000002, 1.0, 1.3322676295501877e-14),
    ] {
        let out = calculate("signals.coax_impedance", &[
            ("inner_radius", inner), ("shield_radius", shield), ("relative_permittivity", er),
        ]).unwrap();
        assert_close(out["impedance"], expected, 1e-6, 0.0);
    }
}

#[test]
fn coax_rejects_touching_reversed_and_invalid_geometry() {
    for (key, bad) in [
        ("inner_radius", 0.0), ("inner_radius", -1.0),
        ("shield_radius", 0.0), ("shield_radius", -1.0),
        ("relative_permittivity", 0.99),
        ("shield_radius", 0.0005), ("shield_radius", 0.0004),
    ] {
        let mut inputs = COAX.to_vec();
        inputs.iter_mut().find(|(name, _)| *name == key).unwrap().1 = bad;
        assert!(calculate("signals.coax_impedance", &inputs).is_err(), "{key}={bad}");
    }
}

#[test]
fn propagation_complete_researched_example() {
    let out = calculate("signals.line_propagation", PROPAGATION).unwrap();
    for (key, expected) in [
        ("velocity", 173085256.3273196), ("delay", 8.666249406959115e-10),
        ("round_trip_delay", 1.733249881391823e-9),
        ("wavelength", 0.17308525632731958), ("electrical_length", 5.445165094215932),
    ] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn propagation_vacuum_full_wave_unwrapped_phase_and_zero_length() {
    // At f=c Hz the vacuum wavelength is exactly 1 m; phase must not wrap.
    for (length, delay, round_trip, phase) in [
        (2.0, 6.671281903963041e-9, 1.3342563807926082e-8, 12.566370614359172),
        (0.0, 0.0, 0.0, 0.0),
    ] {
        let out = calculate("signals.line_propagation", &[
            ("length", length), ("effective_permittivity", 1.0), ("frequency", 299792458.0),
        ]).unwrap();
        // Exact zero length is an algebraic zero: require exact zero, no
        // permissive time/radian absolute floor hiding spurious propagation.
        for (key, expected) in [
            ("velocity", 299792458.0), ("delay", delay),
            ("round_trip_delay", round_trip), ("wavelength", 1.0),
            ("electrical_length", phase),
        ] {
            assert_close(out[key], expected, 1e-6, 0.0);
        }
    }
}

#[test]
fn propagation_uses_effective_dielectric_and_preserves_picosecond_scale() {
    // er_eff=4 halves c. Length=1 mm, f=c/2 -> wavelength=1 m.
    let out = calculate("signals.line_propagation", &[
        ("length", 0.001), ("effective_permittivity", 4.0), ("frequency", 149896229.0),
    ]).unwrap();
    for (key, expected) in [
        ("velocity", 149896229.0), ("delay", 6.671281903963041e-12),
        ("round_trip_delay", 1.3342563807926082e-11), ("wavelength", 1.0),
        ("electrical_length", 0.006283185307179586),
    ] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn propagation_rejects_negative_length_invalid_dielectric_and_frequency() {
    for (key, bad) in [
        ("length", -1e-12), ("effective_permittivity", 0.99),
        ("frequency", 0.0), ("frequency", -1.0),
    ] {
        let mut inputs = PROPAGATION.to_vec();
        inputs.iter_mut().find(|(name, _)| *name == key).unwrap().1 = bad;
        assert!(calculate("signals.line_propagation", &inputs).is_err(), "{key}={bad}");
    }
}

#[test]
fn reflection_complete_researched_example() {
    let out = calculate("signals.load_reflection", REFLECTION).unwrap();
    for (key, expected) in [
        ("reflection_coefficient", 0.2), ("reflected_voltage", 0.24),
        ("load_voltage", 1.44), ("reflected_fraction", 0.04), ("transmitted_fraction", 0.96),
    ] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn reflection_short_match_signed_wave_and_zero_excitation() {
    for (load, incident, gamma, reflected, voltage, rejected, accepted) in [
        (0.0, 2.0, -1.0, -2.0, 0.0, 1.0, 0.0),
        (0.0, -2.0, -1.0, 2.0, 0.0, 1.0, 0.0),
        (50.0, -2.0, 0.0, 0.0, -2.0, 0.0, 1.0),
        (150.0, -2.0, 0.5, -1.0, -3.0, 0.25, 0.75),
        (150.0, 0.0, 0.5, 0.0, 0.0, 0.25, 0.75),
    ] {
        let out = calculate("signals.load_reflection", &[
            ("line_impedance", 50.0), ("load_resistance", load), ("incident_voltage", incident),
        ]).unwrap();
        // Short/match/zero excitation are exact algebraic zeros in volts or
        // dimensionless fractions. Absolute tolerance 0 requires exact zero.
        for (key, expected) in [
            ("reflection_coefficient", gamma), ("reflected_voltage", reflected),
            ("load_voltage", voltage), ("reflected_fraction", rejected),
            ("transmitted_fraction", accepted),
        ] {
            assert_close(out[key], expected, 1e-6, 0.0);
        }
        assert_close(out["reflected_fraction"] + out["transmitted_fraction"], 1.0, 1e-6, 0.0);
    }
}

#[test]
fn reflection_adjacent_binary64_match_preserves_small_reflected_wave() {
    // Independent 80-digit decimal arithmetic on exact binary64 inputs:
    // d=2^-47 ohm, Gamma=±d/(100+d), Gamma²≈5.048709793414475e-33.
    // An absolute epsilon would hide a 20% error in this real nonzero wave.
    for (line, load, sign) in [
        (50.0, 50.00000000000001, 1.0),
        (50.00000000000001, 50.0, -1.0),
    ] {
        let out = calculate("signals.load_reflection", &[
            ("line_impedance", line), ("load_resistance", load), ("incident_voltage", 1.0),
        ]).unwrap();
        for (key, expected) in [
            ("reflection_coefficient", sign * 7.105427357601001e-17),
            ("reflected_voltage", sign * 7.105427357601001e-17),
            ("load_voltage", 1.0), ("reflected_fraction", 5.048709793414475e-33),
            ("transmitted_fraction", 1.0),
        ] {
            assert_close(out[key], expected, 1e-6, 0.0);
        }
    }
}

#[test]
fn reflection_near_short_and_open_preserves_tiny_accepted_power() {
    // For r=RL/Z0=1e-20: VL/Vi≈2r, accepted power≈4r;
    // for r=1e20: VL/Vi≈2, accepted power≈4/r. Corrections are <1e-19.
    for (line, load, gamma, voltage) in [
        (1.0, 1e-20, -1.0, 2e-20),
        (1e-20, 1.0, 1.0, 2.0),
    ] {
        let out = calculate("signals.load_reflection", &[
            ("line_impedance", line), ("load_resistance", load), ("incident_voltage", 1.0),
        ]).unwrap();
        for (key, expected) in [
            ("reflection_coefficient", gamma), ("reflected_voltage", gamma),
            ("load_voltage", voltage), ("reflected_fraction", 1.0),
            ("transmitted_fraction", 4e-20),
        ] {
            assert_close(out[key], expected, 1e-6, 0.0);
        }
    }
}

#[test]
fn reflection_impedance_sum_may_exceed_f64_without_invalid_physics() {
    // Scaling both impedances cannot change the reflection; 1e308+5e307
    // remains finite but their original products in power would overflow.
    // Equal 1e308 impedances have an overflowing sum yet are exactly matched.
    for (line, load, gamma, voltage, fraction, accepted) in [
        (1e308, 1e308, 0.0, -1.0, 0.0, 1.0),
        (1e308, 5e307, -1.0 / 3.0, -2.0 / 3.0, 1.0 / 9.0, 8.0 / 9.0),
    ] {
        let out = calculate("signals.load_reflection", &[
            ("line_impedance", line), ("load_resistance", load), ("incident_voltage", -1.0),
        ]).unwrap();
        for (key, expected) in [
            ("reflection_coefficient", gamma), ("reflected_voltage", -gamma),
            ("load_voltage", voltage), ("reflected_fraction", fraction),
            ("transmitted_fraction", accepted),
        ] {
            assert_close(out[key], expected, 1e-6, 0.0);
        }
    }
}

#[test]
fn reflection_rejects_invalid_impedance_negative_load_and_nonfinite_open() {
    for (key, bad) in [
        ("line_impedance", 0.0), ("line_impedance", -1.0),
        ("load_resistance", -1e-12), ("load_resistance", f64::INFINITY),
        ("incident_voltage", f64::NAN),
    ] {
        let mut inputs = REFLECTION.to_vec();
        inputs.iter_mut().find(|(name, _)| *name == key).unwrap().1 = bad;
        assert!(calculate("signals.load_reflection", &inputs).is_err(), "{key}={bad}");
    }
}

#[test]
fn source_termination_complete_researched_example() {
    let out = calculate("signals.source_series_termination", TERMINATION).unwrap();
    for (key, expected) in [
        ("series_resistance", 42.0), ("launch_voltage", 1.65), ("receiver_voltage", 3.3),
    ] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
}

#[test]
fn source_termination_passive_endpoints_signed_and_zero_step() {
    for (driver, step, series, launch, receiver) in [
        (50.0, -4.0, 0.0, -2.0, -4.0),
        (0.0, 4.0, 50.0, 2.0, 4.0),
        (10.0, 0.0, 40.0, 0.0, 0.0),
    ] {
        let out = calculate("signals.source_series_termination", &[
            ("line_impedance", 50.0), ("driver_resistance", driver), ("step_voltage", step),
        ]).unwrap();
        // Rdriver=Z0 requires exactly zero external resistor; a zero driver
        // step requires exactly zero traveling/receiver voltage.
        for (key, expected) in [
            ("series_resistance", series), ("launch_voltage", launch), ("receiver_voltage", receiver),
        ] {
            assert_close(out[key], expected, 1e-6, 0.0);
        }
    }
}

#[test]
fn source_termination_preserves_tiny_receiver_and_avoids_large_divider() {
    // The matched-source launch is Vstep/2 and open receiver is Vstep.
    let large = calculate("signals.source_series_termination", &[
        ("line_impedance", 1e308), ("driver_resistance", 5e307), ("step_voltage", -4.0),
    ]).unwrap();
    assert_close(large["series_resistance"], 5e307, 1e-6, 0.0);
    assert_close(large["launch_voltage"], -2.0, 1e-6, 0.0);
    assert_close(large["receiver_voltage"], -4.0, 1e-6, 0.0);
    for step in [f64::from_bits(1), -f64::from_bits(1)] {
        let out = calculate("signals.source_series_termination", &[
            ("line_impedance", 50.0), ("driver_resistance", 50.0), ("step_voltage", step),
        ]).unwrap();
        assert_close(out["series_resistance"], 0.0, 0.0, 0.0);
        // Half the smallest subnormal rounds to zero; the receiver must still
        // preserve the representable original step, rather than double zero.
        assert_close(out["launch_voltage"], 0.0, 0.0, 0.0);
        assert_close(out["receiver_voltage"], step, 0.0, 0.0);
    }
}

#[test]
fn source_termination_rejects_negative_or_excess_driver_instead_of_clamping() {
    for (key, bad) in [
        ("line_impedance", 0.0), ("line_impedance", -1.0),
        ("driver_resistance", -1e-12), ("driver_resistance", 60.00000000000001),
        ("step_voltage", f64::INFINITY),
    ] {
        let mut inputs = TERMINATION.to_vec();
        inputs.iter_mut().find(|(name, _)| *name == key).unwrap().1 = bad;
        assert!(calculate("signals.source_series_termination", &inputs).is_err(), "{key}={bad}");
    }
}

#[test]
fn rise_time_complete_researched_example() {
    let out = calculate("signals.rise_time_bandwidth", &[("rise_time", 2e-9)]).unwrap();
    assert_close(out["bandwidth"], 174849576.2830299, 1e-6, 0.0);
}

#[test]
fn rise_time_single_pole_identity_scaling_and_positive_boundary() {
    // A single pole with tau=1/(2*pi) has BW=1 Hz and tr=tau*ln(9).
    // This discriminates exact ln(9)/(2*pi) from the rounded 0.35 factor.
    for (rise, expected) in [
        (0.3496991525660598, 1.0), (4e-9, 87424788.14151495),
        (1e-15, 3.496991525660598e14), (1.0, 0.3496991525660598),
    ] {
        let out = calculate("signals.rise_time_bandwidth", &[("rise_time", rise)]).unwrap();
        assert_close(out["bandwidth"], expected, 1e-6, 0.0);
    }
}

#[test]
fn rise_time_rejects_zero_negative_and_nonrepresentable_bandwidth() {
    for rise in [0.0, -1e-9, f64::INFINITY, f64::NAN, f64::from_bits(1)] {
        // Smallest positive f64 has a genuinely unrepresentable bandwidth;
        // reject nonfinite output rather than silently returning infinity.
        assert!(calculate("signals.rise_time_bandwidth", &[("rise_time", rise)]).is_err());
    }
}

#[test]
fn quarter_wave_complete_researched_example() {
    let out = calculate("signals.quarter_wave_transformer", QUARTER_WAVE).unwrap();
    assert_close(out["transformer_impedance"], 70.71067811865476, 1e-6, 0.0);
    assert_close(out["length"], 0.020818920694444444, 1e-6, 0.0);
}

#[test]
fn quarter_wave_impedance_inversion_reciprocity_and_vacuum_boundary() {
    // Rs=25, RL=100 -> Zt=50 and Zin=Zt²/RL=25 ohm.
    // With f=c/4 and vacuum section dielectric, lambda/4 is exactly 1 m.
    for (source, load, er, impedance, length) in [
        (25.0, 100.0, 1.0, 50.0, 1.0),
        (100.0, 25.0, 1.0, 50.0, 1.0),
        (75.0, 75.0, 4.0, 75.0, 0.5),
    ] {
        let out = calculate("signals.quarter_wave_transformer", &[
            ("source_resistance", source), ("load_resistance", load),
            ("frequency", 74948114.5), ("effective_permittivity", er),
        ]).unwrap();
        assert_close(out["transformer_impedance"], impedance, 1e-6, 0.0);
        assert_close(out["length"], length, 1e-6, 0.0);
        assert_close(out["transformer_impedance"].powi(2) / load, source, 1e-6, 0.0);
    }
}

#[test]
fn quarter_wave_geometric_mean_and_length_avoid_intermediate_overflow_underflow() {
    for (source, load, frequency, impedance, length) in [
        (1e200, 1e200, 1e308, 1e200, 7.49481145e-301),
        (1e-200, 1e-200, 74948114.5, 1e-200, 1.0),
        (1e300, 1e-300, 74948114.5, 1.0, 1.0),
    ] {
        let out = calculate("signals.quarter_wave_transformer", &[
            ("source_resistance", source), ("load_resistance", load),
            ("frequency", frequency), ("effective_permittivity", 1.0),
        ]).unwrap();
        assert_close(out["transformer_impedance"], impedance, 1e-6, 0.0);
        assert_close(out["length"], length, 1e-6, 0.0);
    }
}

#[test]
fn quarter_wave_rejects_nonpositive_resistances_frequency_and_invalid_dielectric() {
    for (key, bad) in [
        ("source_resistance", 0.0), ("source_resistance", -1.0),
        ("load_resistance", 0.0), ("load_resistance", -1.0),
        ("frequency", 0.0), ("frequency", -1.0), ("effective_permittivity", 0.99),
    ] {
        let mut inputs = QUARTER_WAVE.to_vec();
        inputs.iter_mut().find(|(name, _)| *name == key).unwrap().1 = bad;
        assert!(calculate("signals.quarter_wave_transformer", &inputs).is_err(), "{key}={bad}");
    }
}

#[test]
fn all_signals_features_require_exact_finite_inputs_and_recover_independently() {
    for (id, nominal) in [
        ("signals.microstrip_impedance", MICROSTRIP),
        ("signals.stripline_impedance", STRIPLINE),
        ("signals.coax_impedance", COAX),
        ("signals.line_propagation", PROPAGATION),
        ("signals.load_reflection", REFLECTION),
        ("signals.source_series_termination", TERMINATION),
        ("signals.rise_time_bandwidth", &[("rise_time", 2e-9)][..]),
        ("signals.quarter_wave_transformer", QUARTER_WAVE),
    ] {
        let mut extra = nominal.to_vec();
        extra.push(("unknown_signal_parameter", 1.0));
        assert!(calculate(id, &extra).is_err(), "{id}: extra input");
        for position in 0..nominal.len() {
            let mut missing = nominal.to_vec();
            missing.remove(position);
            assert!(calculate(id, &missing).is_err(), "{id}: missing input {}", nominal[position].0);
            for nonfinite in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                let mut invalid = nominal.to_vec();
                invalid[position].1 = nonfinite;
                assert!(calculate(id, &invalid).is_err(), "{id}: input {}", invalid[position].0);
            }
        }
        // Stateful requests must not poison the shared worker engine after errors.
        assert!(calculate(id, nominal).is_ok(), "{id} recovery");
    }
}
