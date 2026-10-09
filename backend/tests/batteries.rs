mod common;

use common::{assert_close, calculate};
use std::f64::consts::{LN_2, PI};

// Independent charge budgets, Peukert invariants, KiBaM imbalance dynamics,
// free-energy derivatives and planar diffusion supply the oracles.
// Nonzero results have no absolute tolerance floor. Exact binary zero boundaries
// below need no absolute allowance; the derivative check states its truncation bound.
const R: f64 = 8.31446261815324;
const F: f64 = 96485.33212;
const SERIES: &[(&str, f64)] = &[
    ("capacity_one", 4.0), ("capacity_two", 2.0), ("capacity_three", 8.0),
    ("soc_one", 0.75), ("soc_two", 0.875), ("soc_three", 0.5), ("soc_floor", 0.25),
];
const CYCLE: &[(&str, f64)] = &[
    ("full_capacity", 8.0), ("initial_soc", 0.75), ("discharge_current", 3.0),
    ("discharge_time", 2400.0), ("charge_current", 4.0), ("charge_time", 1800.0),
    ("charge_efficiency", 0.75),
];
const PEUKERT: &[(&str, f64)] = &[
    ("rated_capacity", 72.0), ("rated_duration", 12.0),
    ("load_current", 24.0), ("peukert_exponent", 1.5),
];
const KINETIC: &[(&str, f64)] = &[
    ("maximum_charge", 200.0), ("available_fraction", 0.25),
    ("available_initial", 20.0), ("bound_initial", 120.0),
    ("equalization_rate", 0.5), ("pulse_current", 2.0), ("pulse_duration", 2.0),
];
const INSERTION: &[(&str, f64)] = &[
    ("midpoint_voltage", 0.2), ("temperature", 320.0), ("interaction_energy", -1200.0),
    ("filling_fraction", 0.25), ("full_insertion_charge", 2400.0),
];
const SAND: &[(&str, f64)] = &[
    ("salt_diffusivity", 2e-10), ("salt_concentration", 800.0),
    ("plating_current_density", 160.0), ("cation_transference", 0.25),
];

#[test]
fn series_complete_nondefault_charge_budget() {
    // Margins are 2, 1.25 and 2 Ah; equal series charge strands the other cells.
    let out = calculate("batteries.series_weakest_cell", SERIES).unwrap();
    for (key, expected) in [
        ("usable_charge", 1.25), ("end_soc_one", 0.4375),
        ("end_soc_two", 0.25), ("end_soc_three", 0.34375),
    ] {
        assert_close(out[key], expected, 1e-12, 0.0);
    }
    for (capacity, initial, key) in [
        (4.0, 0.75, "end_soc_one"), (2.0, 0.875, "end_soc_two"),
        (8.0, 0.5, "end_soc_three"),
    ] {
        assert_close(capacity * (initial - out[key]), 1.25, 1e-12, 0.0);
    }
}

#[test]
fn series_permutation_and_tiny_capacity_scaling() {
    for scale in [1.0, 1e-200] {
        // Move the limiting cell through every position; permute full cell tuples.
        for shift in 0..3 {
            let cells = [(4.0, 0.75, 0.4375), (2.0, 0.875, 0.25), (8.0, 0.5, 0.34375)];
            let mut inputs = SERIES.to_vec();
            for index in 0..3 {
                let cell = cells[(index + shift) % 3];
                inputs[index].1 = cell.0 * scale;
                inputs[index + 3].1 = cell.1;
            }
            let out = calculate("batteries.series_weakest_cell", &inputs).unwrap();
            assert_close(out["usable_charge"], 1.25 * scale, 1e-12, 0.0);
            for (index, key) in ["end_soc_one", "end_soc_two", "end_soc_three"].iter().enumerate() {
                assert_close(out[*key], cells[(index + shift) % 3].2, 1e-12, 0.0);
            }
        }
    }
}

#[test]
fn series_floor_and_tied_limiting_cells() {
    let mut floor = SERIES.to_vec();
    floor[4].1 = 0.25;
    let out = calculate("batteries.series_weakest_cell", &floor).unwrap();
    for (key, expected) in [
        ("usable_charge", 0.0), ("end_soc_one", 0.75),
        ("end_soc_two", 0.25), ("end_soc_three", 0.5),
    ] {
        assert_close(out[key], expected, 0.0, 0.0);
    }
    for (soc, floor, charge) in [(0.75, 0.25, 2.0), (1.0, 0.0, 4.0), (1.0, 1.0, 0.0)] {
        let out = calculate("batteries.series_weakest_cell", &[
            ("capacity_one", 4.0), ("capacity_two", 4.0), ("capacity_three", 4.0),
            ("soc_one", soc), ("soc_two", soc), ("soc_three", soc), ("soc_floor", floor),
        ]).unwrap();
        assert_close(out["usable_charge"], charge, 0.0, 0.0);
        for key in ["end_soc_one", "end_soc_two", "end_soc_three"] {
            assert_close(out[key], floor, 0.0, 0.0);
        }
    }
}

#[test]
fn series_invalid_domains_then_valid_recovery() {
    for (key, bad) in [
        ("capacity_one", 0.0), ("capacity_two", -1.0), ("capacity_three", 0.0),
        ("soc_one", -0.1), ("soc_two", 1.1), ("soc_three", 1.1),
        ("soc_one", 0.125), ("soc_two", 0.125), ("soc_three", 0.125),
        ("soc_floor", -0.1), ("soc_floor", 1.1),
    ] {
        let mut inputs = SERIES.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = bad;
        assert!(calculate("batteries.series_weakest_cell", &inputs).is_err(), "accepted {key}={bad}");
        let out = calculate("batteries.series_weakest_cell", SERIES).unwrap();
        assert_close(out["usable_charge"], 1.25, 1e-12, 0.0);
    }
}

#[test]
fn cycle_complete_nondefault_coulomb_budget() {
    // Initial 6 Ah, remove 2 Ah, pass 2 Ah through charger but retain only 1.5 Ah.
    let out = calculate("batteries.coulomb_state_cycle", CYCLE).unwrap();
    assert_close(out["discharged_soc"], 0.5, 1e-12, 0.0);
    assert_close(out["stored_charge"], 1.5, 1e-12, 0.0);
    assert_close(out["final_soc"], 0.6875, 1e-12, 0.0);
    assert_close(8.0 * out["final_soc"], 6.0 - 2.0 + 1.5, 1e-12, 0.0);
}

#[test]
fn cycle_zero_intervals_unity_efficiency_and_exact_endpoints() {
    for zero_currents in [false, true] {
        let mut inputs = CYCLE.to_vec();
        for index in if zero_currents { [2, 4] } else { [3, 5] } {
            inputs[index].1 = 0.0;
        }
        let out = calculate("batteries.coulomb_state_cycle", &inputs).unwrap();
        assert_close(out["discharged_soc"], 0.75, 0.0, 0.0);
        assert_close(out["stored_charge"], 0.0, 0.0, 0.0);
        assert_close(out["final_soc"], 0.75, 0.0, 0.0);
    }
    let mut ideal = CYCLE.to_vec();
    ideal[6].1 = 1.0;
    let out = calculate("batteries.coulomb_state_cycle", &ideal).unwrap();
    assert_close(out["discharged_soc"], 0.5, 1e-12, 0.0);
    assert_close(out["stored_charge"], 2.0, 1e-12, 0.0);
    assert_close(out["final_soc"], 0.75, 1e-12, 0.0);
    for charge in [0.0, 2.0] {
        let out = calculate("batteries.coulomb_state_cycle", &[
            ("full_capacity", 2.0), ("initial_soc", 0.5), ("discharge_current", 1.0),
            ("discharge_time", 3600.0), ("charge_current", charge), ("charge_time", 3600.0),
            ("charge_efficiency", 1.0),
        ]).unwrap();
        assert_close(out["discharged_soc"], 0.0, 0.0, 0.0);
        assert_close(out["stored_charge"], charge, 0.0, 0.0);
        assert_close(out["final_soc"], charge / 2.0, 0.0, 0.0);
    }
}

#[test]
fn cycle_rejects_intermediate_exhaustion_and_overcharge_then_recovers() {
    // An eventual positive final SOC cannot legalize an exhausted discharge stage.
    for (discharge_time, charge_time) in [(8000.0, 7200.0), (2400.0, 7200.0)] {
        let mut inputs = CYCLE.to_vec();
        inputs[3].1 = discharge_time;
        inputs[5].1 = charge_time;
        assert!(calculate("batteries.coulomb_state_cycle", &inputs).is_err());
        let out = calculate("batteries.coulomb_state_cycle", CYCLE).unwrap();
        assert_close(out["discharged_soc"], 0.5, 1e-12, 0.0);
        assert_close(out["stored_charge"], 1.5, 1e-12, 0.0);
        assert_close(out["final_soc"], 0.6875, 1e-12, 0.0);
    }
}

#[test]
fn cycle_rejects_invalid_scalar_domains() {
    for (key, bad) in [
        ("full_capacity", 0.0), ("full_capacity", -1.0), ("initial_soc", -0.1),
        ("initial_soc", 1.1), ("discharge_current", -1.0), ("discharge_time", -1.0),
        ("charge_current", -1.0), ("charge_time", -1.0),
        ("charge_efficiency", 0.0), ("charge_efficiency", -0.1), ("charge_efficiency", 1.1),
    ] {
        let mut inputs = CYCLE.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = bad;
        assert!(calculate("batteries.coulomb_state_cycle", &inputs).is_err(), "accepted {key}={bad}");
    }
}

#[test]
fn peukert_complete_radical_oracle_and_load_scaling() {
    // Cp = 12*6^(3/2). At 24 A, t=12/8; quadrupling I divides t by eight.
    for (load, hours, capacity) in [(24.0, 1.5, 36.0), (96.0, 0.1875, 18.0)] {
        let mut inputs = PEUKERT.to_vec();
        inputs[2].1 = load;
        let out = calculate("batteries.peukert_runtime", &inputs).unwrap();
        assert_close(out["rated_current"], 6.0, 1e-12, 0.0);
        assert_close(out["runtime"], hours, 1e-12, 0.0);
        assert_close(out["delivered_capacity"], capacity, 1e-12, 0.0);
        assert_close(out["runtime"] * load * load.sqrt(), 72.0 * 6.0_f64.sqrt(), 1e-12, 0.0);
    }
}

#[test]
fn peukert_rated_point_and_ideal_capacity_are_invariants() {
    for exponent in [1.0, 1.7, 8.0] {
        let mut inputs = PEUKERT.to_vec();
        inputs[2].1 = 6.0;
        inputs[3].1 = exponent;
        let out = calculate("batteries.peukert_runtime", &inputs).unwrap();
        assert_close(out["rated_current"], 6.0, 0.0, 0.0);
        assert_close(out["runtime"], 12.0, 0.0, 0.0);
        assert_close(out["delivered_capacity"], 72.0, 0.0, 0.0);
    }
    let mut ideal = PEUKERT.to_vec();
    ideal[3].1 = 1.0;
    let out = calculate("batteries.peukert_runtime", &ideal).unwrap();
    assert_close(out["runtime"], 3.0, 1e-12, 0.0);
    assert_close(out["delivered_capacity"], 72.0, 1e-12, 0.0);
}

#[test]
fn peukert_ideal_runtime_survives_intermediate_ratio_underflow() {
    // Numerical-domain probe, not a calibrated hardware rating. At n=1, C/I is exact law.
    let out = calculate("batteries.peukert_runtime", &[
        ("rated_capacity", 1.0), ("rated_duration", 1e200),
        ("load_current", 1e200), ("peukert_exponent", 1.0),
    ]).unwrap();
    assert_close(out["rated_current"], 1e-200, 1e-12, 0.0);
    assert_close(out["runtime"], 1e-200, 1e-12, 0.0);
    assert_close(out["delivered_capacity"], 1.0, 1e-12, 0.0);
}

#[test]
fn peukert_invalid_domains_then_valid_recovery() {
    for (key, bad) in [
        ("rated_capacity", 0.0), ("rated_capacity", -1.0),
        ("rated_duration", 0.0), ("rated_duration", -1.0),
        ("load_current", 0.0), ("load_current", -1.0), ("peukert_exponent", 0.99),
    ] {
        let mut inputs = PEUKERT.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = bad;
        assert!(calculate("batteries.peukert_runtime", &inputs).is_err(), "accepted {key}={bad}");
        assert_close(calculate("batteries.peukert_runtime", PEUKERT).unwrap()["runtime"], 1.5, 1e-12, 0.0);
    }
}

#[test]
fn kinetic_complete_independent_imbalance_solution() {
    // Q'=−I; delta=q1−cQ obeys delta'=−lambda*delta−(1−c)I.
    // Q(2)=136; delta(0)=−15; delta(infinity)=−3; hence q1(2)=31−12/e.
    let expected_available = 31.0 - 12.0 / std::f64::consts::E;
    let out = calculate("batteries.kinetic_charge_recovery", KINETIC).unwrap();
    assert_close(out["available_final"], expected_available, 1e-12, 0.0);
    assert_close(out["bound_final"], 136.0 - expected_available, 1e-12, 0.0);
    assert_close(out["total_final"], 136.0, 1e-12, 0.0);
    assert_close(out["available_final"] + out["bound_final"], out["total_final"], 1e-12, 0.0);
}

#[test]
fn kinetic_rest_disconnected_and_zero_time_limits() {
    for (key, value, available, bound, total) in [
        ("pulse_current", 0.0, 35.0 - 15.0 / std::f64::consts::E, 105.0 + 15.0 / std::f64::consts::E, 140.0),
        ("equalization_rate", 0.0, 16.0, 120.0, 136.0),
        ("pulse_duration", 0.0, 20.0, 120.0, 140.0),
    ] {
        let mut inputs = KINETIC.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = value;
        let out = calculate("batteries.kinetic_charge_recovery", &inputs).unwrap();
        assert_close(out["available_final"], available, 1e-12, 0.0);
        assert_close(out["bound_final"], bound, 1e-12, 0.0);
        assert_close(out["total_final"], total, 1e-12, 0.0);
    }
    for rate in [0.0, 0.5, 100.0] {
        let mut inputs = KINETIC.to_vec();
        inputs[2].1 = 40.0;
        inputs[4].1 = rate;
        inputs[5].1 = 0.0;
        let out = calculate("batteries.kinetic_charge_recovery", &inputs).unwrap();
        assert_close(out["available_final"], 40.0, 0.0, 0.0);
        assert_close(out["bound_final"], 120.0, 0.0, 0.0);
        assert_close(out["total_final"], 160.0, 0.0, 0.0);
    }
}

#[test]
fn kinetic_composition_and_rest_recover_access_not_total_charge() {
    let whole = calculate("batteries.kinetic_charge_recovery", KINETIC).unwrap();
    let mut half = KINETIC.to_vec();
    half[6].1 = 1.0;
    let first = calculate("batteries.kinetic_charge_recovery", &half).unwrap();
    half[2].1 = first["available_final"];
    half[3].1 = first["bound_final"];
    let second = calculate("batteries.kinetic_charge_recovery", &half).unwrap();
    for key in ["available_final", "bound_final", "total_final"] {
        assert_close(second[key], whole[key], 1e-12, 0.0);
    }
    half[2].1 = second["available_final"];
    half[3].1 = second["bound_final"];
    half[5].1 = 0.0;
    half[6].1 = 2.0;
    let rested = calculate("batteries.kinetic_charge_recovery", &half).unwrap();
    let available = 34.0 - 3.0 / std::f64::consts::E - 12.0 / std::f64::consts::E.powi(2);
    assert_close(rested["available_final"], available, 1e-12, 0.0);
    assert_close(rested["bound_final"], 136.0 - available, 1e-12, 0.0);
    assert_close(rested["total_final"], 136.0, 1e-12, 0.0);
    assert!(rested["available_final"] > second["available_final"]);
}

#[test]
fn kinetic_tiny_recovery_retains_nonzero_transfer() {
    // q1(0)=0 exposes transfer directly: q1=20*(x−x²/2+...), x=1e−18.
    let out = calculate("batteries.kinetic_charge_recovery", &[
        ("maximum_charge", 200.0), ("available_fraction", 0.25),
        ("available_initial", 0.0), ("bound_initial", 80.0),
        ("equalization_rate", 1e-18), ("pulse_current", 0.0), ("pulse_duration", 1.0),
    ]).unwrap();
    assert_close(out["available_final"], 2e-17, 1e-12, 0.0);
    assert_close(out["bound_final"], 80.0, 1e-12, 0.0);
    assert_close(out["total_final"], 80.0, 1e-12, 0.0);
}

#[test]
fn kinetic_available_exhaustion_is_inclusive_but_bound_charge_is_not_accessible() {
    let mut inputs = KINETIC.to_vec();
    inputs[4].1 = 0.0;
    inputs[5].1 = 10.0;
    let out = calculate("batteries.kinetic_charge_recovery", &inputs).unwrap();
    assert_close(out["available_final"], 0.0, 0.0, 0.0);
    assert_close(out["bound_final"], 120.0, 0.0, 0.0);
    assert_close(out["total_final"], 120.0, 0.0, 0.0);
    inputs[6].1 = 2.0001;
    assert!(calculate("batteries.kinetic_charge_recovery", &inputs).is_err());
    inputs[6].1 = 2.0;
    assert_close(calculate("batteries.kinetic_charge_recovery", &inputs).unwrap()["available_final"], 0.0, 0.0, 0.0);
}

#[test]
fn kinetic_invalid_well_domains_then_valid_recovery() {
    for (key, bad) in [
        ("maximum_charge", 0.0), ("maximum_charge", -1.0),
        ("available_fraction", 0.0), ("available_fraction", 1.0),
        ("available_initial", -1.0), ("available_initial", 51.0),
        ("bound_initial", -1.0), ("bound_initial", 151.0),
        ("bound_initial", 30.0), ("equalization_rate", -1.0),
        ("pulse_current", -1.0), ("pulse_current", 100.0), ("pulse_duration", -1.0),
    ] {
        let mut inputs = KINETIC.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = bad;
        assert!(calculate("batteries.kinetic_charge_recovery", &inputs).is_err(), "accepted {key}={bad}");
        assert_close(calculate("batteries.kinetic_charge_recovery", KINETIC).unwrap()["available_final"], 31.0 - 12.0 / std::f64::consts::E, 1e-12, 0.0);
    }
}

#[test]
fn intercalation_complete_negative_interaction_oracle() {
    // At x=1/4, entropy derivative is −RT ln(3), interaction derivative is −600 J/mol.
    // Free-energy curvature is RT*(4+4/3)+2400 J/mol, independently differentiated.
    let curvature = R * 320.0 * (4.0 + 4.0 / 3.0) + 2400.0;
    let out = calculate("batteries.intercalation_regular_solution", INSERTION).unwrap();
    assert_close(out["equilibrium_voltage"], 0.2 + (R * 320.0 * 3.0_f64.ln() + 600.0) / F, 1e-12, 0.0);
    assert_close(out["voltage_slope"], -curvature / F, 1e-12, 0.0);
    assert_close(out["chemical_capacitance"], 2400.0 / (curvature / F), 1e-12, 0.0);
}

#[test]
fn intercalation_complement_symmetry_midpoint_and_ideal_lattice() {
    let base = calculate("batteries.intercalation_regular_solution", INSERTION).unwrap();
    let mut inputs = INSERTION.to_vec();
    inputs[3].1 = 0.75;
    let complement = calculate("batteries.intercalation_regular_solution", &inputs).unwrap();
    assert_close(base["equilibrium_voltage"] + complement["equilibrium_voltage"], 0.4, 1e-12, 0.0);
    for key in ["voltage_slope", "chemical_capacitance"] {
        assert_close(complement[key], base[key], 1e-12, 0.0);
    }
    inputs[3].1 = 0.5;
    for interaction in [-1200.0, 0.0, 1000.0] {
        inputs[2].1 = interaction;
        let out = calculate("batteries.intercalation_regular_solution", &inputs).unwrap();
        let curvature = 4.0 * R * 320.0 - 2.0 * interaction;
        assert_close(out["equilibrium_voltage"], 0.2, 0.0, 0.0);
        assert_close(out["voltage_slope"], -curvature / F, 1e-12, 0.0);
        assert_close(out["chemical_capacitance"], 2400.0 * F / curvature, 1e-12, 0.0);
    }
    inputs[2].1 = 0.0;
    inputs[3].1 = 0.25;
    let ideal = calculate("batteries.intercalation_regular_solution", &inputs).unwrap();
    assert_close(ideal["equilibrium_voltage"], 0.2 + R * 320.0 / F * 3.0_f64.ln(), 1e-12, 0.0);
    assert!(ideal["chemical_capacitance"] > base["chemical_capacitance"]);
}

#[test]
fn intercalation_derivative_matches_voltage_and_charge_response() {
    let center = calculate("batteries.intercalation_regular_solution", INSERTION).unwrap();
    let h: f64 = 1e-5;
    let mut inputs = INSERTION.to_vec();
    inputs[3].1 = 0.25 - h;
    let low = calculate("batteries.intercalation_regular_solution", &inputs).unwrap();
    inputs[3].1 = 0.25 + h;
    let high = calculate("batteries.intercalation_regular_solution", &inputs).unwrap();
    let derivative = (high["equilibrium_voltage"] - low["equilibrium_voltage"]) / (2.0 * h);
    // Centered derivative error is O(h²); 1e−8 relative exceeds the analytic ~5e−10 truncation.
    assert_close(derivative, center["voltage_slope"], 1e-8, 0.0);
    assert_close(-center["chemical_capacitance"] * center["voltage_slope"], 2400.0, 1e-12, 0.0);
}

#[test]
fn intercalation_near_critical_and_near_full_limits() {
    let mut inputs = INSERTION.to_vec();
    inputs[2].1 = 2.0 * R * 320.0 - 0.0625;
    inputs[3].1 = 0.5;
    let critical = calculate("batteries.intercalation_regular_solution", &inputs).unwrap();
    assert_close(critical["equilibrium_voltage"], 0.2, 0.0, 0.0);
    assert_close(critical["voltage_slope"], -0.125 / F, 1e-10, 0.0);
    assert_close(critical["chemical_capacitance"], 2400.0 * F / 0.125, 1e-10, 0.0);
    let vacancy: f64 = 2.0_f64.powi(-40);
    let filling = 1.0 - vacancy;
    inputs[2].1 = 0.0;
    inputs[3].1 = filling;
    let full = calculate("batteries.intercalation_regular_solution", &inputs).unwrap();
    let thermal = R * 320.0 / F;
    assert_close(full["equilibrium_voltage"], 0.2 - thermal * (filling.ln() + 40.0 * LN_2), 1e-12, 0.0);
    assert_close(full["voltage_slope"], -thermal / (filling * vacancy), 1e-12, 0.0);
    assert_close(full["chemical_capacitance"], 2400.0 * filling * vacancy / thermal, 1e-12, 0.0);
}

#[test]
fn intercalation_tiny_filling_has_representable_voltage_slope_and_capacitance() {
    // Admitted endpoint-conditioning probe, not a claim of continuum validity at one atom.
    // RT/x overflows, but (RT/F)/x and all requested physical outputs are representable.
    let out = calculate("batteries.intercalation_regular_solution", &[
        ("midpoint_voltage", 0.0), ("temperature", 300.0), ("interaction_energy", 0.0),
        ("filling_fraction", 1e-307), ("full_insertion_charge", 1.0),
    ]).unwrap();
    let thermal = R * 300.0 / F;
    assert_close(out["equilibrium_voltage"], thermal * 307.0 * 10.0_f64.ln(), 1e-12, 0.0);
    assert_close(out["voltage_slope"], -thermal * 1e307, 1e-12, 0.0);
    assert_close(out["chemical_capacitance"], 1e-307 / thermal, 1e-12, 0.0);
}

#[test]
fn intercalation_global_phase_guard_and_invalid_inputs_then_recovery() {
    for (key, bad) in [
        ("temperature", 0.0), ("temperature", -1.0),
        ("filling_fraction", 0.0), ("filling_fraction", 1.0),
        ("filling_fraction", -0.1), ("filling_fraction", 1.1),
        ("full_insertion_charge", 0.0), ("full_insertion_charge", -1.0),
        ("interaction_energy", 2.0 * R * 320.0), ("interaction_energy", 6000.0),
    ] {
        let mut inputs = INSERTION.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = bad;
        assert!(calculate("batteries.intercalation_regular_solution", &inputs).is_err(), "accepted {key}={bad}");
        assert_close(calculate("batteries.intercalation_regular_solution", INSERTION).unwrap()["equilibrium_voltage"], 0.2365133080905016, 1e-12, 0.0);
    }
    let mut metastable = INSERTION.to_vec();
    metastable[2].1 = 6000.0;
    metastable[3].1 = 0.01;
    // Local curvature is positive here, but the globally single-phase requirement still fails.
    assert!(calculate("batteries.intercalation_regular_solution", &metastable).is_err());
}

#[test]
fn sand_complete_nondefault_diffusion_oracle() {
    // Constant boundary molar flux J=j(1−t+)/F gives c_surface=c0−2J sqrt(t/(pi D)).
    // Set c_surface=0 independently of the catalog's square-root factorization.
    let flux: f64 = 120.0 / F;
    let time = PI * 2e-10 * (400.0 / flux).powi(2);
    let out = calculate("batteries.sand_depletion_limit", SAND).unwrap();
    assert_close(out["sand_time"], time, 1e-12, 0.0);
    assert_close(out["sand_areal_charge"], 160.0 * time, 1e-12, 0.0);
    assert_close(out["diffusion_length"], (2e-10 * time).sqrt(), 1e-12, 0.0);
    assert_close(2.0 * flux * (out["sand_time"] / (PI * 2e-10)).sqrt(), 800.0, 1e-12, 0.0);
}

#[test]
fn sand_current_concentration_diffusivity_and_transference_scaling() {
    let base = calculate("batteries.sand_depletion_limit", SAND).unwrap();
    for (key, value, time_scale, charge_scale, length_scale) in [
        ("plating_current_density", 320.0, 0.25, 0.5, 0.5),
        ("salt_concentration", 1600.0, 4.0, 4.0, 2.0),
        ("salt_diffusivity", 8e-10, 4.0, 4.0, 4.0),
        ("cation_transference", 0.0, 0.5625, 0.5625, 0.75),
        ("cation_transference", 0.9990234375, 589824.0, 589824.0, 768.0),
    ] {
        let mut inputs = SAND.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = value;
        let out = calculate("batteries.sand_depletion_limit", &inputs).unwrap();
        assert_close(out["sand_time"], base["sand_time"] * time_scale, 1e-12, 0.0);
        assert_close(out["sand_areal_charge"], base["sand_areal_charge"] * charge_scale, 1e-12, 0.0);
        assert_close(out["diffusion_length"], base["diffusion_length"] * length_scale, 1e-12, 0.0);
    }
}

#[test]
fn sand_tiny_diffusivity_preserves_representable_length() {
    let mut inputs = SAND.to_vec();
    inputs[0].1 = 1e-300;
    let out = calculate("batteries.sand_depletion_limit", &inputs).unwrap();
    // Diffusion length is linear in D; forming D*t first would underflow.
    assert_close(out["sand_time"], 64.99200739336196 * 5e-291, 1e-12, 0.0);
    assert_close(out["sand_areal_charge"], 10398.721182937914 * 5e-291, 1e-12, 0.0);
    assert_close(out["diffusion_length"], 0.00011401053231466114 * 5e-291, 1e-12, 0.0);
}

#[test]
fn sand_invalid_domains_then_valid_recovery() {
    for (key, bad) in [
        ("salt_diffusivity", 0.0), ("salt_diffusivity", -1.0),
        ("salt_concentration", 0.0), ("salt_concentration", -1.0),
        ("plating_current_density", 0.0), ("plating_current_density", -1.0),
        ("cation_transference", -0.1), ("cation_transference", 1.0), ("cation_transference", 1.1),
    ] {
        let mut inputs = SAND.to_vec();
        inputs.iter_mut().find(|(k, _)| *k == key).unwrap().1 = bad;
        assert!(calculate("batteries.sand_depletion_limit", &inputs).is_err(), "accepted {key}={bad}");
        let out = calculate("batteries.sand_depletion_limit", SAND).unwrap();
        assert_close(out["sand_time"], 64.99200739336196, 1e-12, 0.0);
        assert_close(out["sand_areal_charge"], 10398.721182937914, 1e-12, 0.0);
        assert_close(out["diffusion_length"], 0.00011401053231466114, 1e-12, 0.0);
    }
}

#[test]
fn all_battery_inputs_reject_nonfinite_values_without_poisoning_engine() {
    for (id, valid, output_key, expected) in [
        ("batteries.series_weakest_cell", SERIES, "usable_charge", 1.25),
        ("batteries.coulomb_state_cycle", CYCLE, "final_soc", 0.6875),
        ("batteries.peukert_runtime", PEUKERT, "runtime", 1.5),
        ("batteries.kinetic_charge_recovery", KINETIC, "total_final", 136.0),
        ("batteries.intercalation_regular_solution", INSERTION, "equilibrium_voltage", 0.2365133080905016),
        ("batteries.sand_depletion_limit", SAND, "sand_time", 64.99200739336196),
    ] {
        for index in 0..valid.len() {
            for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                let mut inputs = valid.to_vec();
                inputs[index].1 = bad;
                assert!(calculate(id, &inputs).is_err(), "accepted {id} {}={bad}", valid[index].0);
                assert_close(calculate(id, valid).unwrap()[output_key], expected, 1e-12, 0.0);
            }
        }
    }
}

#[test]
fn peukert_runtime_preserves_representable_scaled_power() {
    // V2Batteries found the n=2 variant: Cp=Ir²*Hr=1e−200, despite Ir² underflow.
    let out = calculate("batteries.peukert_runtime", &[
        ("rated_capacity", 1.0), ("rated_duration", 1e200),
        ("load_current", 1.0), ("peukert_exponent", 2.0),
    ]).unwrap();
    for key in ["rated_current", "runtime", "delivered_capacity"] {
        assert_close(out[key], 1e-200, 1e-12, 0.0);
    }
}

#[test]
fn kinetic_fast_exchange_preserves_charge_without_intermediate_overflow() {
    // V2Batteries scaling probe: total falls 4e200→3e200; rapid exchange shares
    // that remaining charge equally. The finite 0.5 C imbalance is below one ulp.
    let out = calculate("batteries.kinetic_charge_recovery", &[
        ("maximum_charge", 1e201), ("available_fraction", 0.5),
        ("available_initial", 2e200), ("bound_initial", 2e200),
        ("equalization_rate", 1e200), ("pulse_current", 1e200), ("pulse_duration", 1.0),
    ]).unwrap();
    assert_close(out["available_final"], 1.5e200, 1e-12, 0.0);
    assert_close(out["bound_final"], 1.5e200, 1e-12, 0.0);
    assert_close(out["total_final"], 3e200, 1e-12, 0.0);
    assert_close(out["available_final"] + out["bound_final"], 3e200, 1e-12, 0.0);
}

#[test]
fn intercalation_capacitance_scales_without_molar_charge_product_overflow() {
    // V2Batteries found Qmax*F overflow. Use our nondefault electrode to isolate it.
    let mut inputs = INSERTION.to_vec();
    inputs[4].1 = 1e305;
    let out = calculate("batteries.intercalation_regular_solution", &inputs).unwrap();
    let curvature = R * 320.0 * (4.0 + 4.0 / 3.0) + 2400.0;
    assert_close(out["equilibrium_voltage"], 0.2365133080905016, 1e-12, 0.0);
    assert_close(out["voltage_slope"], -curvature / F, 1e-12, 0.0);
    assert_close(out["chemical_capacitance"], 1e305 / (curvature / F), 1e-12, 0.0);
}
