mod common;

use common::{assert_close, calculate};
use std::f64::consts::PI;

const SLOT: [(&str, f64); 4] = [
    ("frequency", 74_948_114.5), ("slot_length", 0.0625),
    ("slot_width", 0.005), ("shield_thickness", 0.001),
];
const RC: [(&str, f64); 7] = [
    ("resistance_one", 1000.0), ("resistance_two", 2000.0),
    ("capacitance_one", 1e-9), ("capacitance_two", 1e-9),
    ("differential_capacitance", 1e-9), ("frequency", 1e6 / (2.0 * PI)),
    ("common_mode_rms", 0.4),
];
const HARMONIC: [(&str, f64); 6] = [
    ("switching_frequency", 200_000.0), ("pulse_current", 0.012),
    ("duty_half_height", 0.25), ("edge_time", 0.625e-6),
    ("harmonic_order", 2.0), ("termination_resistance", 75.0),
];
const DITHER: [(&str, f64); 4] = [
    ("switching_frequency", 240_000.0), ("peak_deviation", 18_000.0),
    ("modulation_frequency", 3000.0), ("harmonic_order", 7.0),
];
const SNUBBER: [(&str, f64); 7] = [
    ("original_frequency", 20e6), ("loaded_frequency", 16e6),
    ("test_capacitance", 0.9e-9), ("damping_target", 0.5),
    ("capacitance_multiplier", 4.0), ("voltage_swing", 24.0),
    ("switching_frequency", 250_000.0),
];
const CABLE: [(&str, f64); 4] = [
    ("frequency", 149_896_229.0), ("cable_length", 0.125),
    ("common_mode_current_rms", 32e-6), ("distance", 8.0),
];

#[test]
fn slot_complete_binary_geometry_oracle() {
    let r = calculate("emc.thin_slot_shielding", &SLOT).unwrap();
    assert_close(r["wavelength"], 4.0, 1e-12, 0.0);
    assert_close(r["field_ratio"], 1.0 / 32.0, 1e-12, 0.0);
    assert_close(r["shielding_db"], 100.0 * 2.0_f64.log10(), 1e-12, 0.0);
}

#[test]
fn slot_small_aperture_thickness_and_length_limits() {
    for thickness in [0.0, 0.00625] {
        let mut p = SLOT;
        p[3].1 = thickness;
        let r = calculate("emc.thin_slot_shielding", &p).unwrap();
        assert_close(r["field_ratio"], 0.03125, 1e-12, 0.0);
        assert_close(r["wavelength"], 4.0, 1e-12, 0.0);
        assert_close(r["shielding_db"], 30.10299956639812, 1e-12, 0.0);
    }
    let mut p = SLOT;
    p[1].1 = 0.2;
    let r = calculate("emc.thin_slot_shielding", &p).unwrap();
    assert_close(r["field_ratio"], 0.1, 1e-12, 0.0);
    assert_close(r["shielding_db"], 20.0, 1e-12, 0.0);
    p[1].1 = 0.03125;
    let r = calculate("emc.thin_slot_shielding", &p).unwrap();
    assert_close(r["field_ratio"], 1.0 / 64.0, 1e-12, 0.0);
    assert_close(r["shielding_db"], 120.0 * 2.0_f64.log10(), 1e-12, 0.0);
}

#[test]
fn slot_invalid_geometry_recovers() {
    for (index, bad) in [(0, 0.0), (0, -1.0), (1, 0.0), (1, -0.1),
        (2, 0.0), (2, -0.1), (3, -1.0), (2, 0.0625), (2, 0.07),
        (3, 0.00625001), (1, 0.2000001)] {
        let mut p = SLOT;
        p[index].1 = bad;
        assert!(calculate("emc.thin_slot_shielding", &p).is_err());
        slot_complete_binary_geometry_oracle();
    }
}

#[test]
fn rc_complete_coupled_nodal_oracle() {
    // Normalized nodal matrix: [[1+2j,-j],[-2j,1+4j]].
    // Unit-drive solutions are (1+5j)/(-5+6j), (1+4j)/(-5+6j).
    let r = calculate("emc.rc_common_mode_conversion", &RC).unwrap();
    assert_close(r["mismatch_time"], 1e-6, 1e-12, 0.0);
    assert_close(r["conversion_gain"], 1.0 / 61.0_f64.sqrt(), 1e-12, 0.0);
    assert_close(r["differential_rms"], 0.4 / 61.0_f64.sqrt(), 1e-12, 0.0);
}

#[test]
fn rc_uncoupled_branch_swap_balance_dc_and_zero_drive() {
    let mut p = RC;
    p[4].1 = 0.0;
    let r = calculate("emc.rc_common_mode_conversion", &p).unwrap();
    // Independent branch subtraction: (0.5-0.5j)-(0.2-0.4j).
    assert_close(r["conversion_gain"], 0.1_f64.sqrt(), 1e-12, 0.0);
    assert_close(r["differential_rms"], 0.4 * 0.1_f64.sqrt(), 1e-12, 0.0);
    assert_close(r["mismatch_time"], 1e-6, 1e-12, 0.0);
    p = RC;
    p[0].1 = 2000.0;
    p[1].1 = 1000.0;
    let r = calculate("emc.rc_common_mode_conversion", &p).unwrap();
    assert_close(r["mismatch_time"], -1e-6, 1e-12, 0.0);
    assert_close(r["conversion_gain"], 1.0 / 61.0_f64.sqrt(), 1e-12, 0.0);
    assert_close(r["differential_rms"], 0.4 / 61.0_f64.sqrt(), 1e-12, 0.0);
    for cd in [0.0, 1e-9, 1e-6] {
        p = RC;
        p[3].1 = 0.5e-9;
        p[4].1 = cd;
        let r = calculate("emc.rc_common_mode_conversion", &p).unwrap();
        for key in ["mismatch_time", "conversion_gain", "differential_rms"] {
            assert_eq!(r[key], 0.0);
        }
    }
    p = RC;
    p[5].1 = 0.0;
    let r = calculate("emc.rc_common_mode_conversion", &p).unwrap();
    assert_close(r["mismatch_time"], 1e-6, 1e-12, 0.0);
    assert_eq!(r["conversion_gain"], 0.0);
    assert_eq!(r["differential_rms"], 0.0);
    p = RC;
    p[6].1 = 0.0;
    let r = calculate("emc.rc_common_mode_conversion", &p).unwrap();
    assert_close(r["conversion_gain"], 1.0 / 61.0_f64.sqrt(), 1e-12, 0.0);
    assert_eq!(r["differential_rms"], 0.0);
}

#[test]
fn rc_self_resonance_peak_is_driven_by_b_only() {
    // Cd=2.5e-10 nulls a=1-w^2 R1 R2[C1C2+Cd(C1+C2)] at w=5e5, the worst-case
    // common-to-differential conversion of the mismatched filter legs.
    let p = [
        ("resistance_one", 1000.0), ("resistance_two", 1000.0),
        ("capacitance_one", 1e-9), ("capacitance_two", 3e-9),
        ("differential_capacitance", 2.5e-10), ("frequency", 5e5 / (2.0 * PI)),
        ("common_mode_rms", 0.4),
    ];
    let r = calculate("emc.rc_common_mode_conversion", &p).unwrap();
    assert_close(r["mismatch_time"], 2e-6, 1e-12, 0.0);
    assert_close(r["conversion_gain"], 4.0 / 9.0, 1e-12, 0.0);
    assert_close(r["differential_rms"], 1.6 / 9.0, 1e-12, 0.0);
    let mut q = p;
    q[4].1 = -1e-10;
    assert!(calculate("emc.rc_common_mode_conversion", &q).is_err());
    q = p;
    q[4].1 = 2.5e-10 * (1.0 + 1e-3);
    let r = calculate("emc.rc_common_mode_conversion", &q).unwrap();
    // a=-2.5e-4: any detune of the bridging capacitor only lowers the peak below 4/9.
    assert!(r["conversion_gain"] < 4.0 / 9.0);
    rc_complete_coupled_nodal_oracle();
}

#[test]
fn rc_invalid_passives_and_overflow_recover() {
    for (index, bad) in [(0, 0.0), (0, -1.0), (1, 0.0), (1, -1.0),
        (2, 0.0), (2, -1e-9), (3, 0.0), (3, -1e-9),
        (4, -1e-9), (5, -1.0), (6, -1.0), (5, 1e200)] {
        let mut p = RC;
        p[index].1 = bad;
        assert!(calculate("emc.rc_common_mode_conversion", &p).is_err());
        rc_complete_coupled_nodal_oracle();
    }
}

#[test]
fn harmonic_complete_quarter_duty_oracle() {
    // Fourier integral of the full-edge trapezoid at n=2 reduces to 2 Ipk/pi² RMS.
    let r = calculate("emc.trapezoid_conducted_harmonic", &HARMONIC).unwrap();
    assert_close(r["harmonic_frequency"], 400_000.0, 1e-12, 0.0);
    assert_close(r["dc_current"], 0.003, 1e-12, 0.0);
    assert_close(r["harmonic_current_rms"], 0.024 / PI.powi(2), 1e-12, 0.0);
    assert_close(r["termination_voltage_rms"], 1.8 / PI.powi(2), 1e-12, 0.0);
}

#[test]
fn harmonic_triangle_complement_null_zero_and_tiny_edge() {
    let mut p = HARMONIC;
    p[2].1 = 0.5;
    p[3].1 = 2.5e-6;
    p[4].1 = 1.0;
    let r = calculate("emc.trapezoid_conducted_harmonic", &p).unwrap();
    let triangle: f64 = 2.0_f64.sqrt() * 0.024 / PI.powi(2);
    assert_close(r["harmonic_current_rms"], triangle, 1e-12, 0.0);
    assert_close(r["termination_voltage_rms"], 75.0 * triangle, 1e-12, 0.0);
    assert_close(r["dc_current"], 0.006, 1e-12, 0.0);
    assert_close(r["harmonic_frequency"], 200_000.0, 1e-12, 0.0);
    p[4].1 = 2.0;
    let r = calculate("emc.trapezoid_conducted_harmonic", &p).unwrap();
    // Analytic null: tolerate only sine argument roundoff, scaled by current and load.
    assert_close(r["harmonic_current_rms"], 0.0, 0.0, 0.012 * 8.0 * f64::EPSILON);
    assert_close(r["termination_voltage_rms"], 0.0, 0.0, 0.9 * 8.0 * f64::EPSILON);
    p = HARMONIC;
    p[2].1 = 0.75;
    let r = calculate("emc.trapezoid_conducted_harmonic", &p).unwrap();
    assert_close(r["dc_current"], 0.009, 1e-12, 0.0);
    assert_close(r["harmonic_current_rms"], 0.024 / PI.powi(2), 1e-12, 0.0);
    p = HARMONIC;
    p[1].1 = 0.0;
    let r = calculate("emc.trapezoid_conducted_harmonic", &p).unwrap();
    for key in ["dc_current", "harmonic_current_rms", "termination_voltage_rms"] {
        assert_eq!(r[key], 0.0);
    }
    assert_eq!(r["harmonic_frequency"], 400_000.0);
    p = HARMONIC;
    p[3].1 = 1e-200;
    let r = calculate("emc.trapezoid_conducted_harmonic", &p).unwrap();
    let rectangular: f64 = 0.012 / (2.0_f64.sqrt() * PI);
    assert_close(r["harmonic_current_rms"], rectangular, 1e-12, 0.0);
    assert_close(r["termination_voltage_rms"], 75.0 * rectangular, 1e-12, 0.0);
    p[1].1 = 1e-200;
    let r = calculate("emc.trapezoid_conducted_harmonic", &p).unwrap();
    assert_close(r["harmonic_current_rms"], 1e-200 / (2.0_f64.sqrt() * PI), 1e-12, 0.0);
}

#[test]
fn harmonic_invalid_edges_orders_and_underflow_recover() {
    for (index, bad) in [(0, 0.0), (0, -1.0), (1, -1.0), (2, 0.0),
        (2, 1.0), (2, -0.1), (2, 1.1), (3, 0.0), (3, -1e-6),
        (3, 1.250001e-6), (4, 0.0), (4, 1.5), (5, 0.0), (5, -1.0)] {
        let mut p = HARMONIC;
        p[index].1 = bad;
        assert!(calculate("emc.trapezoid_conducted_harmonic", &p).is_err());
        harmonic_complete_quarter_duty_oracle();
    }
    let mut p = HARMONIC;
    p[0].1 = 1e-200;
    p[3].1 = 1e-200;
    assert!(calculate("emc.trapezoid_conducted_harmonic", &p).is_err());
    harmonic_complete_quarter_duty_oracle();
}

#[test]
fn dither_complete_interval_endpoint_oracle() {
    let r = calculate("emc.sinusoidal_dither_occupancy", &DITHER).unwrap();
    // Seventh band [1551000,1809000], eighth [1773000,2067000].
    assert_close(r["harmonic_modulation_index"], 42.0, 1e-12, 0.0);
    assert_close(r["occupied_bandwidth"], 1_809_000.0 - 1_551_000.0, 1e-12, 0.0);
    assert_close(r["next_harmonic_gap"], 1_773_000.0 - 1_809_000.0, 1e-12, 0.0);
}

#[test]
fn dither_touching_bands_small_deviation_and_frequency_scaling() {
    for (frequency, gap) in [(275_999.0, -1.0), (276_000.0, 0.0), (276_001.0, 1.0)] {
        let mut p = DITHER;
        p[0].1 = frequency;
        let r = calculate("emc.sinusoidal_dither_occupancy", &p).unwrap();
        assert_eq!(r["next_harmonic_gap"], gap);
        assert_eq!(r["occupied_bandwidth"], 258_000.0);
        assert_eq!(r["harmonic_modulation_index"], 42.0);
    }
    let mut p = DITHER;
    p[1].1 = 1.0;
    let r = calculate("emc.sinusoidal_dither_occupancy", &p).unwrap();
    assert_close(r["harmonic_modulation_index"], 7.0 / 3000.0, 1e-12, 0.0);
    assert_eq!(r["occupied_bandwidth"], 6014.0);
    assert_eq!(r["next_harmonic_gap"], 233_985.0);
    p = DITHER;
    for value in &mut p[..3] { value.1 *= 0.125; }
    let r = calculate("emc.sinusoidal_dither_occupancy", &p).unwrap();
    assert_eq!(r["harmonic_modulation_index"], 42.0);
    assert_eq!(r["occupied_bandwidth"], 32_250.0);
    assert_eq!(r["next_harmonic_gap"], -4500.0);
    p[3].1 = 1.0;
    let r = calculate("emc.sinusoidal_dither_occupancy", &p).unwrap();
    assert_eq!(r["harmonic_modulation_index"], 6.0);
    assert_eq!(r["occupied_bandwidth"], 5250.0);
    assert_eq!(r["next_harmonic_gap"], 22_500.0);
}

#[test]
fn dither_invalid_modulation_and_order_recovers() {
    for (index, bad) in [(0, 0.0), (0, -1.0), (1, 0.0), (1, -1.0),
        (1, 240_000.0), (1, 240_001.0), (2, 0.0), (2, -1.0),
        (2, 240_000.0), (2, 240_001.0), (3, 0.0), (3, 2.5)] {
        let mut p = DITHER;
        p[index].1 = bad;
        assert!(calculate("emc.sinusoidal_dither_occupancy", &p).is_err());
        dither_complete_interval_endpoint_oracle();
    }
}

#[test]
fn snubber_complete_five_fourths_frequency_oracle() {
    // f0/f1=5/4, so adding 0.9 nF increases C by 9/16 of its original value.
    let r = calculate("emc.ringing_rc_snubber", &SNUBBER).unwrap();
    let inductance: f64 = 1.0 / (2_560_000.0 * PI.powi(2));
    assert_close(r["parasitic_capacitance"], 1.6e-9, 1e-12, 0.0);
    assert_close(r["parasitic_inductance"], inductance, 1e-12, 0.0);
    assert_close(r["snubber_resistance"], (inductance / 1.6e-9).sqrt(), 1e-12, 0.0);
    assert_close(r["snubber_capacitance"], 6.4e-9, 1e-12, 0.0);
    assert_close(r["snubber_power"], 0.9216, 1e-12, 0.0);
}

#[test]
fn snubber_endpoint_targets_scaling_zero_and_frequency_reconstruction() {
    let base = calculate("emc.ringing_rc_snubber", &SNUBBER).unwrap();
    let mut p = SNUBBER;
    p[3].1 = 1.0;
    p[4].1 = 3.0;
    let r = calculate("emc.ringing_rc_snubber", &p).unwrap();
    assert_close(r["snubber_resistance"], base["snubber_resistance"] / 2.0, 1e-12, 0.0);
    assert_close(r["snubber_capacitance"], 4.8e-9, 1e-12, 0.0);
    assert_close(r["snubber_power"], 0.6912, 1e-12, 0.0);
    p = SNUBBER;
    p[2].1 *= 2.0;
    let r = calculate("emc.ringing_rc_snubber", &p).unwrap();
    for key in ["parasitic_capacitance", "snubber_capacitance", "snubber_power"] {
        assert_close(r[key], base[key] * 2.0, 1e-12, 0.0);
    }
    for key in ["parasitic_inductance", "snubber_resistance"] {
        assert_close(r[key], base[key] / 2.0, 1e-12, 0.0);
    }
    let l = r["parasitic_inductance"];
    let c = r["parasitic_capacitance"];
    assert_close(1.0 / (2.0 * PI * (l * c).sqrt()), 20e6, 1e-12, 0.0);
    assert_close(1.0 / (2.0 * PI * (l * (c + p[2].1)).sqrt()), 16e6, 1e-12, 0.0);
    p = SNUBBER;
    p[5].1 = 0.0;
    let r = calculate("emc.ringing_rc_snubber", &p).unwrap();
    assert_eq!(r["snubber_power"], 0.0);
    for key in ["parasitic_capacitance", "parasitic_inductance", "snubber_resistance", "snubber_capacitance"] {
        assert_close(r[key], base[key], 1e-12, 0.0);
    }
    p = SNUBBER;
    p[5].1 = 48.0;
    p[6].1 = 500_000.0;
    let r = calculate("emc.ringing_rc_snubber", &p).unwrap();
    // P=Cs V^2 fs: quadratic in swing, linear in cycle rate.
    assert_close(r["snubber_power"], 0.9216 * 8.0, 1e-12, 0.0);
    assert_close(r["snubber_capacitance"], 6.4e-9, 1e-12, 0.0);
}

#[test]
fn snubber_near_equal_measured_frequencies_keep_small_difference() {
    let mut p = SNUBBER;
    p[0].1 = 1_000_000_000.0;
    p[1].1 = 999_999_999.0;
    p[2].1 = 2e-9;
    let r = calculate("emc.ringing_rc_snubber", &p).unwrap();
    // Exact integer difference of squares is 1,999,999,999, not a rounded ratio minus one.
    let cp: f64 = 2e-9 * (999_999_999.0_f64.powi(2) / 1_999_999_999.0);
    let lp: f64 = 1.0 / (4.0 * PI.powi(2) * 1e18 * cp);
    assert_close(r["parasitic_capacitance"], cp, 1e-12, 0.0);
    assert_close(r["parasitic_inductance"], lp, 1e-12, 0.0);
    assert_close(r["snubber_resistance"], (lp / cp).sqrt(), 1e-12, 0.0);
    assert_close(r["snubber_capacitance"], 4.0 * cp, 1e-12, 0.0);
    assert_close(r["snubber_power"], 576_000_000.0 * cp, 1e-12, 0.0);
}

#[test]
fn snubber_invalid_measurements_and_design_targets_recover() {
    for (index, bad) in [(0, 0.0), (0, -1.0), (1, 0.0), (1, -1.0),
        (1, 20e6), (1, 21e6), (2, 0.0), (2, -1e-9), (3, 0.4999),
        (3, 1.0001), (4, 2.9999), (4, 4.0001), (5, -1.0),
        (6, 0.0), (6, -1.0), (6, 16e6), (6, 17e6)] {
        let mut p = SNUBBER;
        p[index].1 = bad;
        assert!(calculate("emc.ringing_rc_snubber", &p).is_err());
        snubber_complete_five_fourths_frequency_oracle();
    }
}

#[test]
fn snubber_unrepresentable_positive_inductance_must_not_become_zero() {
    let p = [("original_frequency", 1e160), ("loaded_frequency", 5e159),
        ("test_capacitance", 3e10), ("damping_target", 0.5),
        ("capacitance_multiplier", 3.0), ("voltage_swing", 1.0),
        ("switching_frequency", 1.0)];
    // Cp=1e10 F; Lp≈2.533e-332 H cannot be represented. Reject, never design Rs=0.
    assert!(calculate("emc.ringing_rc_snubber", &p).is_err());
    snubber_complete_five_fourths_frequency_oracle();
}

#[test]
fn cable_complete_hertzian_current_moment_oracle() {
    let r = calculate("emc.short_cable_radiation", &CABLE).unwrap();
    let eta: f64 = 4.0 * PI * 1e-7 * 299_792_458.0;
    // lambda=2 m, k=pi rad/m; E=eta*k*(I*l)/(4*pi*r).
    assert_close(r["electrical_length"], 0.0625, 1e-12, 0.0);
    assert_close(r["far_field_parameter"], 8.0 * PI, 1e-12, 0.0);
    assert_close(r["electric_field_rms"], eta * 1.25e-7, 1e-12, 0.0);
}

#[test]
fn cable_distance_zero_current_and_screening_endpoints() {
    let mut p = CABLE;
    p[3].1 = 16.0;
    let r = calculate("emc.short_cable_radiation", &p).unwrap();
    assert_close(r["electric_field_rms"], 2.0 * PI * 1e-7 * 299_792_458.0 * 1.25e-7, 1e-12, 0.0);
    assert_close(r["electrical_length"], 0.0625, 1e-12, 0.0);
    assert_close(r["far_field_parameter"], 16.0 * PI, 1e-12, 0.0);
    p = CABLE;
    p[2].1 = 0.0;
    let r = calculate("emc.short_cable_radiation", &p).unwrap();
    assert_eq!(r["electric_field_rms"], 0.0);
    assert_close(r["electrical_length"], 0.0625, 1e-12, 0.0);
    assert_close(r["far_field_parameter"], 8.0 * PI, 1e-12, 0.0);
    p = CABLE;
    p[1].1 = 0.2;
    let r = calculate("emc.short_cable_radiation", &p).unwrap();
    assert_close(r["electrical_length"], 0.1, 1e-12, 0.0);
    p = CABLE;
    p[3].1 = 10.0 / PI;
    let r = calculate("emc.short_cable_radiation", &p).unwrap();
    assert_close(r["far_field_parameter"], 10.0, 1e-12, 0.0);
}

#[test]
fn cable_invalid_near_field_and_length_recovers() {
    for (index, bad) in [(0, 0.0), (0, -1.0), (1, 0.0), (1, -1.0),
        (2, -1e-6), (3, 0.0), (3, -1.0), (1, 0.2000001),
        (3, (10.0 / PI) * (1.0 - 1e-10))] {
        let mut p = CABLE;
        p[index].1 = bad;
        assert!(calculate("emc.short_cable_radiation", &p).is_err());
        cable_complete_hertzian_current_moment_oracle();
    }
}

#[test]
fn cable_tiny_representable_field_survives_geometric_ratio_underflow() {
    let p = [("frequency", 1e100), ("cable_length", 1e-200),
        ("common_mode_current_rms", 1.0), ("distance", 1e200)];
    let r = calculate("emc.short_cable_radiation", &p).unwrap();
    assert_close(r["electrical_length"], 1e-100 / 299_792_458.0, 1e-12, 0.0);
    assert_close(r["far_field_parameter"], (2.0 * PI / 299_792_458.0) * 1e300, 1e-12, 0.0);
    assert_close(r["electric_field_rms"], (2.0 * PI * 1e-7) * 1e-300, 1e-12, 0.0);
}

#[test]
fn every_emc_input_rejects_nonfinite_missing_and_unknown_then_recovers() {
    for (id, inputs, check) in [
        ("emc.thin_slot_shielding", SLOT.as_slice(), slot_complete_binary_geometry_oracle as fn()),
        ("emc.rc_common_mode_conversion", RC.as_slice(), rc_complete_coupled_nodal_oracle as fn()),
        ("emc.trapezoid_conducted_harmonic", HARMONIC.as_slice(), harmonic_complete_quarter_duty_oracle as fn()),
        ("emc.sinusoidal_dither_occupancy", DITHER.as_slice(), dither_complete_interval_endpoint_oracle as fn()),
        ("emc.ringing_rc_snubber", SNUBBER.as_slice(), snubber_complete_five_fourths_frequency_oracle as fn()),
        ("emc.short_cable_radiation", CABLE.as_slice(), cable_complete_hertzian_current_moment_oracle as fn()),
    ] {
        for index in 0..inputs.len() {
            for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                let mut p = inputs.to_vec();
                p[index].1 = bad;
                assert!(calculate(id, &p).is_err(), "accepted {id} nonfinite input {}", p[index].0);
                check();
            }
            let mut p = inputs.to_vec();
            p.remove(index);
            assert!(calculate(id, &p).is_err());
            check();
        }
        let mut p = inputs.to_vec();
        p.push(("unknown", 1.0));
        assert!(calculate(id, &p).is_err());
        check();
    }
}

#[test]
fn slot_tiny_representable_ratio_does_not_overflow_logarithm_argument() {
    let p = [("frequency", 299_792_458.0), ("slot_length", 1e-310),
        ("slot_width", 1e-311), ("shield_thickness", 0.0)];
    let r = calculate("emc.thin_slot_shielding", &p).unwrap();
    assert_eq!(r["wavelength"], 1.0);
    assert_close(r["field_ratio"], 2e-310, 1e-12, 0.0);
    assert_close(r["shielding_db"], 20.0 * (310.0 - 2.0_f64.log10()), 1e-12, 0.0);
}
