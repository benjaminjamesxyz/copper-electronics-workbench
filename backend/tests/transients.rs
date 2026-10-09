mod common;

use common::{assert_close, calculate};

// Explicit SI inputs; no defaults or engine-produced values are used as oracles.
const RC: &[(&str, f64)] = &[("r", 1000.0), ("c", 1e-6), ("v_source", 5.0), ("v_initial", 0.0), ("time", 0.001), ("error_fraction", 0.01)];
const RL: &[(&str, f64)] = &[("r", 10.0), ("l", 0.01), ("v_source", 12.0), ("i_initial", 0.0), ("time", 0.001), ("error_fraction", 0.01)];
const CURRENT: &[(&str, f64)] = &[("c", 1e-4), ("current", 0.01), ("v_initial", 1.0), ("time", 0.02)];
const SHARING: &[(&str, f64)] = &[("c_one", 1e-4), ("c_two", 2e-4), ("r", 10.0), ("v_one", 8.0), ("v_two", 2.0), ("time", 0.0006666666666666667)];
const PERIODIC: &[(&str, f64)] = &[("r", 1000.0), ("c", 1e-6), ("v_high", 5.0), ("v_low", 0.0), ("time_high", 0.001), ("time_low", 0.001)];
const RAMP: &[(&str, f64)] = &[("r", 1000.0), ("c", 1e-6), ("v_source_initial", 0.0), ("v_initial", 0.0), ("slope", 1000.0), ("time", 0.002)];
const LADDER: &[(&str, f64)] = &[("r_one", 1000.0), ("r_two", 1000.0), ("c_one", 1e-6), ("c_two", 1e-6), ("v_step", 5.0), ("time", 0.001)];
const RLC: &[(&str, f64)] = &[("r", 10.0), ("l", 0.001), ("c", 1e-5), ("v_step", 5.0), ("time", 0.0001), ("error_fraction", 0.01)];

// Complete independently worked research examples (MIT/GSU/Berkeley derivations).
// Absolute tolerance is zero: all these outputs are nonzero, including microjoules.
const EXAMPLES: &[(&str, &[(&str, f64)], &[(&str, f64)])] = &[
    ("transients.rc_step", RC, &[
        ("tau", 0.001), ("capacitor_voltage", 3.1606027941427883),
        ("capacitor_current", 0.0018393972058572117), ("settling_time", 0.004605170185988092),
        ("capacitor_energy", 4.9947050111716e-6), ("resistor_energy", 1.080830895954234e-5),
    ]),
    ("transients.rl_step", RL, &[
        ("tau", 0.001), ("final_current", 1.2), ("inductor_current", 0.7585446705942692),
        ("inductor_voltage", 4.414553294057308), ("settling_time", 0.004605170185988092),
    ]),
    ("transients.capacitor_constant_current", CURRENT, &[
        ("voltage_slope", 100.0), ("capacitor_voltage", 3.0), ("charge_change", 0.0002),
        ("stored_energy", 0.00045), ("energy_change", 0.0004),
    ]),
    ("transients.capacitor_charge_sharing", SHARING, &[
        ("equivalent_capacitance", 6.666666666666667e-5), ("tau", 0.0006666666666666668),
        ("final_voltage", 4.0), ("voltage_one", 5.471517764685769), ("voltage_two", 3.2642411176571153),
        ("current", 0.2207276647028654), ("transferred_charge", 0.0002528482235314231),
        ("initial_energy", 0.0036), ("final_energy", 0.0024), ("total_loss", 0.0012),
        ("dissipated_energy", 0.0010375976601160648),
    ]),
    ("transients.rc_periodic_switching", PERIODIC, &[
        ("tau", 0.001), ("high_decay", 0.36787944117144233), ("low_decay", 0.36787944117144233),
        ("voltage_min", 1.3447071068499756), ("voltage_max", 3.6552928931500244),
        ("ripple", 2.3105857863000487), ("average_voltage", 2.5),
        ("high_initial_current", 0.0036552928931500245), ("low_initial_current", -0.0036552928931500245),
    ]),
    ("transients.rc_ramp_source", RAMP, &[
        ("tau", 0.001), ("source_voltage", 2.0), ("capacitor_voltage", 1.1353352832366128),
        ("tracking_error", 0.8646647167633873), ("capacitor_current", 0.0008646647167633873),
    ]),
    ("transients.rc_ladder_step", LADDER, &[
        ("quadratic_coefficient", 1e-6), ("linear_coefficient", 0.003),
        ("tau_slow", 0.002618033988749895), ("tau_fast", 0.0003819660112501051),
        ("voltage_two", 1.0667720034831585), ("capacitor_two_current", 0.0013630446883126453),
        ("voltage_one", 2.4298166917958035),
    ]),
    ("transients.rlc_underdamped_step", RLC, &[
        ("alpha", 5000.0), ("omega_n", 10000.0), ("damping_ratio", 0.5), ("omega_d", 8660.254037844386),
        ("capacitor_voltage", 1.701499233041492), ("series_current", 0.2667535975573465),
        ("peak_time", 0.00036275987284684356), ("overshoot_fraction", 0.16303353482158048),
        ("peak_voltage", 5.815167674107903), ("envelope_settling_bound", 0.0009498022444427963),
    ]),
];

#[test]
fn all_eight_independent_examples_assert_every_output() {
    for &(id, inputs, expected) in EXAMPLES {
        let actual = calculate(id, inputs).unwrap_or_else(|error| panic!("{id}: {error}"));
        let expected_keys: std::collections::BTreeSet<_> = expected.iter().map(|&(key, _)| key).collect();
        let actual_keys: std::collections::BTreeSet<_> = actual.keys().map(String::as_str).collect();
        assert_eq!(actual_keys, expected_keys, "complete output contract for {id}");
        for &(key, value) in expected {
            assert_close(actual[key], value, 1e-6, 0.0);
        }
    }
}

#[test]
fn rc_discharge_initial_continuity_energy_and_no_excitation() {
    // Half-voltage discharge: t=RC ln(2); lost energy complements stored energy.
    let out = calculate("transients.rc_step", &[
        ("r", 2000.0), ("c", 2e-6), ("v_source", 0.0), ("v_initial", -4.0),
        ("time", 0.0027725887222397813), ("error_fraction", 0.5),
    ]).unwrap();
    for (key, expected) in [("tau", 0.004), ("capacitor_voltage", -2.0), ("capacitor_current", 0.001),
        ("settling_time", 0.0027725887222397813), ("capacitor_energy", 4e-6), ("resistor_energy", 12e-6)] {
        assert_close(out[key], expected, 1e-6, 0.0);
    }
    assert_close(out["capacitor_energy"] + out["resistor_energy"], 16e-6, 1e-12, 0.0);
    let start = calculate("transients.rc_step", &[
        ("r", 1000.0), ("c", 1e-6), ("v_source", -5.0), ("v_initial", 2.0),
        ("time", 0.0), ("error_fraction", 1e-12),
    ]).unwrap();
    assert_close(start["capacitor_voltage"], 2.0, 0.0, 0.0);
    assert_close(start["capacitor_current"], -0.007, 1e-12, 0.0);
    assert_close(start["resistor_energy"], 0.0, 0.0, 0.0); // No time means exactly no dissipated J.
    assert_close(start["settling_time"], 0.02763102111592855, 1e-12, 0.0);
    for voltage in [0.0, -3.0] {
        let steady = calculate("transients.rc_step", &[
            ("r", 1000.0), ("c", 1e-6), ("v_source", voltage), ("v_initial", voltage),
            ("time", 0.001), ("error_fraction", 0.01),
        ]).unwrap();
        assert_close(steady["capacitor_voltage"], voltage, 1e-12, 0.0);
        for key in ["capacitor_current", "resistor_energy"] { assert_close(steady[key], 0.0, 0.0, 0.0); }
    }
    let late = calculate("transients.rc_step", &[
        ("r", 1000.0), ("c", 1e-6), ("v_source", 0.0), ("v_initial", 1.0),
        ("time", 0.04), ("error_fraction", 0.01),
    ]).unwrap();
    assert_close(late["capacitor_voltage"], 4.248354255291589e-18, 1e-12, 0.0);
}

#[test]
fn rl_signed_decay_continuity_steady_state_and_long_time() {
    let decay = calculate("transients.rl_step", &[
        ("r", 20.0), ("l", 0.04), ("v_source", 0.0), ("i_initial", -2.0),
        ("time", 0.0013862943611198907), ("error_fraction", 0.5),
    ]).unwrap();
    for (key, expected) in [("tau", 0.002), ("final_current", 0.0), ("inductor_current", -1.0),
        ("inductor_voltage", 20.0), ("settling_time", 0.0013862943611198907)] {
        assert_close(decay[key], expected, 1e-6, 0.0);
    }
    let initial = calculate("transients.rl_step", &[
        ("r", 10.0), ("l", 0.01), ("v_source", -12.0), ("i_initial", 0.3),
        ("time", 0.0), ("error_fraction", 0.01),
    ]).unwrap();
    assert_close(initial["inductor_current"], 0.3, 0.0, 0.0);
    assert_close(initial["inductor_voltage"], -15.0, 0.0, 0.0);
    let steady = calculate("transients.rl_step", &[
        ("r", 10.0), ("l", 0.01), ("v_source", -12.0), ("i_initial", -1.2),
        ("time", 0.001), ("error_fraction", 0.01),
    ]).unwrap();
    assert_close(steady["inductor_current"], -1.2, 1e-12, 0.0);
    assert_close(steady["inductor_voltage"], 0.0, 0.0, 0.0);
    let late = calculate("transients.rl_step", &[
        ("r", 10.0), ("l", 0.01), ("v_source", -12.0), ("i_initial", 0.0),
        ("time", 1.0), ("error_fraction", 1e-12),
    ]).unwrap();
    assert_close(late["inductor_current"], -1.2, 1e-12, 0.0);
    assert_close(late["inductor_voltage"], 0.0, 0.0, 0.0); // exp(-1000) underflows below f64.
    let residual = calculate("transients.rl_step", &[
        ("r", 10.0), ("l", 0.01), ("v_source", 0.0), ("i_initial", 1.0),
        ("time", 0.04), ("error_fraction", 0.01),
    ]).unwrap();
    assert_close(residual["inductor_current"], 4.248354255291589e-18, 1e-12, 0.0);
}

#[test]
fn constant_current_discharge_zero_crossing_and_tiny_energy_increment() {
    // -10 mA for 20 ms removes 200 microC and 400 microJ from a 3-V capacitor.
    let discharge = calculate("transients.capacitor_constant_current", &[
        ("c", 1e-4), ("current", -0.01), ("v_initial", 3.0), ("time", 0.02),
    ]).unwrap();
    for (key, expected) in [("voltage_slope", -100.0), ("capacitor_voltage", 1.0),
        ("charge_change", -2e-4), ("stored_energy", 5e-5), ("energy_change", -4e-4)] {
        assert_close(discharge[key], expected, 1e-12, 0.0);
    }
    for (current, time, voltage, charge, energy_change) in [
        (0.0, 0.02, 1.0, 0.0, 0.0), (0.01, 0.0, 1.0, 0.0, 0.0),
        (-0.01, 0.01, 0.0, -1e-4, -5e-5), (-0.01, 0.02, -1.0, -2e-4, 0.0),
    ] {
        let out = calculate("transients.capacitor_constant_current", &[
            ("c", 1e-4), ("current", current), ("v_initial", 1.0), ("time", time),
        ]).unwrap();
        assert_close(out["voltage_slope"], current / 1e-4, 1e-12, 0.0);
        assert_close(out["capacitor_voltage"], voltage, 1e-12, 0.0);
        assert_close(out["charge_change"], charge, 1e-12, 0.0);
        assert_close(out["stored_energy"], 5e-5 * voltage * voltage, 1e-12, 0.0);
        assert_close(out["energy_change"], energy_change, 1e-12, 0.0);
    }
    let tiny = calculate("transients.capacitor_constant_current", &[
        ("c", 1.0), ("current", 1.0), ("v_initial", 1.0), ("time", 1e-20),
    ]).unwrap();
    // DeltaE = V0*I*t + (I*t)^2/(2C), even when the absolute voltage rounds to V0.
    assert_close(tiny["energy_change"], 1e-20, 1e-12, 0.0);
}

#[test]
fn charge_sharing_conserves_charge_and_energy_in_both_directions() {
    // Equal capacitors at opposite voltages: common final voltage is exactly zero.
    let out = calculate("transients.capacitor_charge_sharing", &[
        ("c_one", 2e-6), ("c_two", 2e-6), ("r", 1000.0), ("v_one", -2.0), ("v_two", 2.0),
        ("time", 0.0006931471805599453),
    ]).unwrap();
    for (key, expected) in [("equivalent_capacitance", 1e-6), ("tau", 0.001), ("final_voltage", 0.0),
        ("voltage_one", -1.0), ("voltage_two", 1.0), ("current", -0.002),
        ("transferred_charge", -2e-6), ("initial_energy", 8e-6), ("final_energy", 0.0),
        ("total_loss", 8e-6), ("dissipated_energy", 6e-6)] {
        assert_close(out[key], expected, 1e-12, 0.0);
    }
    assert_close(2e-6 * out["voltage_one"] + 2e-6 * out["voltage_two"], 0.0, 0.0, 0.0);
    assert_close(out["final_energy"] + out["total_loss"], 8e-6, 1e-12, 0.0);
    let nominal = calculate("transients.capacitor_charge_sharing", SHARING).unwrap();
    assert_close(1e-4 * nominal["voltage_one"] + 2e-4 * nominal["voltage_two"], 0.0012, 1e-12, 0.0);
    let remaining = (1e-4 * nominal["voltage_one"].powi(2) + 2e-4 * nominal["voltage_two"].powi(2)) / 2.0;
    assert_close(remaining + nominal["dissipated_energy"], 0.0036, 1e-12, 0.0);
    for (v_one, v_two, time) in [(8.0, 2.0, 0.0), (-3.0, -3.0, 0.001)] {
        let state = calculate("transients.capacitor_charge_sharing", &[
            ("c_one", 1e-4), ("c_two", 2e-4), ("r", 10.0),
            ("v_one", v_one), ("v_two", v_two), ("time", time),
        ]).unwrap();
        assert_close(state["voltage_one"], v_one, 1e-12, 0.0);
        assert_close(state["voltage_two"], v_two, 1e-12, 0.0);
        for key in ["transferred_charge", "dissipated_energy"] { assert_close(state[key], 0.0, 0.0, 0.0); }
        assert_close(state["current"], if time == 0.0 { 0.6 } else { 0.0 }, 1e-12, 0.0);
        if v_one == v_two { assert_close(state["total_loss"], 0.0, 0.0, 0.0); }
    }
    let tiny = calculate("transients.capacitor_charge_sharing", &[
        ("c_one", 1e-4), ("c_two", 2e-4), ("r", 10.0), ("v_one", 8.0), ("v_two", 2.0), ("time", 1e-20),
    ]).unwrap();
    assert_close(tiny["transferred_charge"], 6e-21, 1e-12, 0.0); // I(0)*t.
    assert_close(tiny["dissipated_energy"], 3.6e-20, 1e-12, 0.0); // I(0)^2*R*t.
}

#[test]
fn periodic_asymmetric_fixed_point_equal_levels_and_conditioning_boundary() {
    // Decays 1/2 and 1/4 at a 6-V high level imply Vmin=6/7, Vmax=24/7, average=2 V.
    let out = calculate("transients.rc_periodic_switching", &[
        ("r", 1000.0), ("c", 1e-6), ("v_high", 6.0), ("v_low", 0.0),
        ("time_high", 0.0006931471805599453), ("time_low", 0.0013862943611198907),
    ]).unwrap();
    for (key, expected) in [("tau", 0.001), ("high_decay", 0.5), ("low_decay", 0.25),
        ("voltage_min", 6.0 / 7.0), ("voltage_max", 24.0 / 7.0), ("ripple", 18.0 / 7.0),
        ("average_voltage", 2.0), ("high_initial_current", 36.0 / 7000.0), ("low_initial_current", -24.0 / 7000.0)] {
        assert_close(out[key], expected, 1e-12, 0.0);
    }
    assert_close(6.0 + (out["voltage_min"] - 6.0) * 0.5, out["voltage_max"], 1e-12, 0.0);
    assert_close(out["voltage_max"] * 0.25, out["voltage_min"], 1e-12, 0.0);
    let equal = calculate("transients.rc_periodic_switching", &[
        ("r", 1000.0), ("c", 1e-6), ("v_high", -2.0), ("v_low", -2.0),
        ("time_high", 0.001), ("time_low", 0.002),
    ]).unwrap();
    for key in ["voltage_min", "voltage_max", "average_voltage"] { assert_close(equal[key], -2.0, 1e-12, 0.0); }
    for key in ["ripple", "high_initial_current", "low_initial_current"] { assert_close(equal[key], 0.0, 0.0, 0.0); }
    // RC=1 s makes exactly 1e-7 s the inclusive conditioning boundary.
    let boundary = calculate("transients.rc_periodic_switching", &[
        ("r", 1.0), ("c", 1.0), ("v_high", 1.0), ("v_low", -1.0),
        ("time_high", 1e-7), ("time_low", 1e-7),
    ]).unwrap();
    assert_close(boundary["ripple"], 1e-7, 1e-12, 0.0); // 2*tanh(5e-8); relative correction <1e-15.
    assert_close(boundary["average_voltage"], 0.0, 0.0, 0.0);
    assert_close(boundary["high_initial_current"], 1.00000005, 1e-12, 0.0);
    assert_close(boundary["low_initial_current"], -1.00000005, 1e-12, 0.0);
    for (high, low, th, tl) in [(0.0, 1.0, 0.001, 0.001), (5.0, 0.0, 5e-11, 0.001), (5.0, 0.0, 0.001, 5e-11)] {
        assert!(calculate("transients.rc_periodic_switching", &[
            ("r", 1000.0), ("c", 1e-6), ("v_high", high), ("v_low", low), ("time_high", th), ("time_low", tl),
        ]).is_err());
    }
}

#[test]
fn periodic_tiny_ripple_on_nonzero_dc_level_is_not_lost_to_subtraction() {
    // The source levels differ by exactly one binary64 spacing at 1 V.
    let delta = 2.0_f64.powi(-52);
    let out = calculate("transients.rc_periodic_switching", &[
        ("r", 1.0), ("c", 1.0), ("v_high", 1.0 + delta), ("v_low", 1.0),
        ("time_high", 1e-7), ("time_low", 1e-7),
    ]).unwrap();
    // DeltaV*tanh(5e-8) and DeltaV/(1+exp(-1e-7)), independently expanded.
    assert_close(out["ripple"], delta * 5e-8, 1e-12, 0.0);
    assert_close(out["high_initial_current"], delta * 0.500000025, 1e-12, 0.0);
    assert_close(out["low_initial_current"], -delta * 0.500000025, 1e-12, 0.0);
}

#[test]
fn ramp_signed_zero_slope_arbitrary_initial_state_and_positive_startup() {
    for (slope, time, source, voltage, error) in [
        (-1000.0, 0.002, -2.0, -1.1353352832366128, -0.8646647167633873),
        (0.0, 0.002, 0.0, 0.0, 0.0),
        (1000.0, 1e-20, 1e-17, 5e-35, 1e-17),
    ] {
        let out = calculate("transients.rc_ramp_source", &[
            ("r", 1000.0), ("c", 1e-6), ("v_source_initial", 0.0), ("v_initial", 0.0), ("slope", slope), ("time", time),
        ]).unwrap();
        for (key, expected) in [("tau", 0.001), ("source_voltage", source), ("capacitor_voltage", voltage),
            ("tracking_error", error), ("capacitor_current", error / 1000.0)] {
            assert_close(out[key], expected, 1e-12, 0.0);
        }
    }
    let start = calculate("transients.rc_ramp_source", &[
        ("r", 1000.0), ("c", 1e-6), ("v_source_initial", 2.0), ("v_initial", -3.0), ("slope", -1e9), ("time", 0.0),
    ]).unwrap();
    assert_close(start["capacitor_voltage"], -3.0, 0.0, 0.0);
    assert_close(start["tracking_error"], 5.0, 0.0, 0.0);
    assert_close(start["capacitor_current"], 0.005, 1e-12, 0.0);
    // A zero-slope ramp with arbitrary initial charge reduces to RC discharge.
    let step = calculate("transients.rc_ramp_source", &[
        ("r", 1000.0), ("c", 1e-6), ("v_source_initial", 0.0), ("v_initial", 4.0), ("slope", 0.0), ("time", 0.0006931471805599453),
    ]).unwrap();
    assert_close(step["capacitor_voltage"], 2.0, 1e-12, 0.0);
    assert_close(step["tracking_error"], -2.0, 1e-12, 0.0);
    assert_close(step["capacitor_current"], -0.002, 1e-12, 0.0);
}

#[test]
fn ladder_loaded_modes_signed_zero_startup_and_discriminant_guard() {
    // R1=1, R2=2, C1=1, C2=1/2 => a=1, b=5/2; modes 2 s and 1/2 s.
    let out = calculate("transients.rc_ladder_step", &[
        ("r_one", 1.0), ("r_two", 2.0), ("c_one", 1.0), ("c_two", 0.5), ("v_step", -1.0), ("time", 1.0),
    ]).unwrap();
    for (key, expected) in [("quadratic_coefficient", 1.0), ("linear_coefficient", 2.5),
        ("tau_slow", 2.0), ("tau_fast", 0.5), ("voltage_two", -0.23640421479535967),
        ("capacitor_two_current", -0.1570651254920069), ("voltage_one", -0.5505344657793735)] {
        assert_close(out[key], expected, 1e-12, 0.0);
    }
    assert_close((out["voltage_one"] - out["voltage_two"]) / 2.0, out["capacitor_two_current"], 1e-12, 0.0);
    for (step, time, v2, current, v1) in [(5.0, 0.0, 0.0, 0.0, 0.0),
        (0.0, 0.001, 0.0, 0.0, 0.0), (5.0, 1e-20, 2.5e-34, 5e-20, 5e-17)] {
        let state = calculate("transients.rc_ladder_step", &[
            ("r_one", 1000.0), ("r_two", 1000.0), ("c_one", 1e-6), ("c_two", 1e-6), ("v_step", step), ("time", time),
        ]).unwrap();
        for (key, expected) in [("voltage_two", v2), ("capacitor_two_current", current), ("voltage_one", v1)] {
            assert_close(state[key], expected, 1e-12, 0.0);
        }
    }
    // x=R1*C1=z=R2*C2=1; y=R1*C2 controls D/b^2 near the 1e-8 guard.
    for (c_two, r_two, admissible) in [(2e-8, 5e7, true), (5e-9, 2e8, false)] {
        let result = calculate("transients.rc_ladder_step", &[
            ("r_one", 1.0), ("r_two", r_two), ("c_one", 1.0), ("c_two", c_two), ("v_step", 1.0), ("time", 0.0),
        ]);
        if admissible {
            let state = result.unwrap();
            for key in ["voltage_one", "voltage_two", "capacitor_two_current"] { assert_close(state[key], 0.0, 0.0, 0.0); }
            assert!(state["tau_slow"] > state["tau_fast"]);
        } else { assert!(result.is_err()); }
    }
}

#[test]
fn rlc_peak_ringing_envelope_positive_startup_and_damping_domain() {
    for (time, voltage, current, absolute_current) in [
        (0.0, 0.0, 0.0, 0.0),
        (1e-20, 2.5e-32, 5e-17, 0.0),
        (0.00036275987284684356, 5.815167674107903, 0.0, 2e-15),
        (0.0007255197456936871, 4.867100332617903, 0.0, 2e-15),
    ] {
        let out = calculate("transients.rlc_underdamped_step", &[
            ("r", 10.0), ("l", 0.001), ("c", 1e-5), ("v_step", 5.0), ("time", time), ("error_fraction", 0.01),
        ]).unwrap();
        assert_close(out["capacitor_voltage"], voltage, 1e-6, 0.0);
        // At n*pi the true current is zero; 2e-15 A covers pi/trig argument rounding
        // on the 0.577-A undamped-current scale, not a generic small-current floor.
        assert_close(out["series_current"], current, 1e-12, absolute_current);
    }
    let negative_current = calculate("transients.rlc_underdamped_step", &[
        ("r", 10.0), ("l", 0.001), ("c", 1e-5), ("v_step", 5.0), ("time", 0.0005), ("error_fraction", 0.01),
    ]).unwrap();
    // Independent sine/exponential substitution; returning energy is a valid signed current.
    assert_close(negative_current["series_current"], -0.04397121036625643, 1e-12, 0.0);
    for time in [0.0009498022444427963, 0.001, 0.0013, 0.002] {
        let out = calculate("transients.rlc_underdamped_step", &[
            ("r", 10.0), ("l", 0.001), ("c", 1e-5), ("v_step", 5.0), ("time", time), ("error_fraction", 0.01),
        ]).unwrap();
        assert!((out["capacitor_voltage"] / 5.0 - 1.0).abs() <= 0.01);
        assert_close(out["envelope_settling_bound"], 0.0009498022444427963, 1e-12, 0.0);
    }
    for resistance in [20.0, 30.0, 20.0 * (1.0_f64 - 5e-9).sqrt()] {
        assert!(calculate("transients.rlc_underdamped_step", &[
            ("r", resistance), ("l", 0.001), ("c", 1e-5), ("v_step", 5.0), ("time", 0.0), ("error_fraction", 0.01),
        ]).is_err());
    }
    let near_critical = calculate("transients.rlc_underdamped_step", &[
        ("r", 20.0 * (1.0_f64 - 2e-8).sqrt()), ("l", 0.001), ("c", 1e-5), ("v_step", 5.0), ("time", 0.0), ("error_fraction", 1e-12),
    ]).unwrap();
    assert_close(near_critical["capacitor_voltage"], 0.0, 0.0, 0.0);
    assert_close(near_critical["series_current"], 0.0, 0.0, 0.0);
    assert!(near_critical["omega_d"] > 0.0);
}

#[test]
fn first_order_step_startup_retains_small_positive_charge_and_current() {
    let rc = calculate("transients.rc_step", &[
        ("r", 1000.0), ("c", 1e-6), ("v_source", 5.0), ("v_initial", 0.0),
        ("time", 1e-20), ("error_fraction", 0.01),
    ]).unwrap();
    // v=Vs*t/(RC), ER=Vs^2*t/R to leading order; corrections are <1e-16 relative.
    assert_close(rc["capacitor_voltage"], 5e-17, 1e-12, 0.0);
    assert_close(rc["capacitor_current"], 0.005, 1e-12, 0.0);
    assert_close(rc["capacitor_energy"], 1.25e-39, 1e-12, 0.0);
    assert_close(rc["resistor_energy"], 2.5e-22, 1e-12, 0.0);
    let rl = calculate("transients.rl_step", &[
        ("r", 10.0), ("l", 0.01), ("v_source", 12.0), ("i_initial", 0.0),
        ("time", 1e-20), ("error_fraction", 0.01),
    ]).unwrap();
    assert_close(rl["inductor_current"], 1.2e-17, 1e-12, 0.0); // di/dt=Vs/L.
    assert_close(rl["inductor_voltage"], 12.0, 1e-12, 0.0);
}

#[test]
fn stable_primitives_match_independent_goldens_on_both_sides_of_branch_transitions() {
    // Independently evaluated at 70-digit decimal precision. Ramp uses exp(-x)-1+x;
    // ladder uses poles 1/2 and 2; RLC goldens solve y''+y'+y=1 by ODE coefficients.
    for (x, voltage, error) in [
        (0.09999999, 0.0048374170843337988, 0.0951625729156662),
        (0.1, 0.004837418035959573, 0.09516258196404043),
        (0.10000001, 0.004837418987585438, 0.09516259101241456),
    ] {
        let out = calculate("transients.rc_ramp_source", &[
            ("r", 1.0), ("c", 1.0), ("v_source_initial", 0.0), ("v_initial", 0.0), ("slope", 1.0), ("time", x),
        ]).unwrap();
        assert_close(out["tau"], 1.0, 0.0, 0.0);
        assert_close(out["source_voltage"], x, 0.0, 0.0);
        assert_close(out["capacitor_voltage"], voltage, 1e-11, 0.0);
        assert_close(out["tracking_error"], error, 1e-12, 0.0);
        assert_close(out["capacitor_current"], error, 1e-12, 0.0);
    }
    for (time, v2, current, v1) in [
        (0.049999995, 0.0011992564059679973, 0.02349082912742455, 0.0481809146608171),
        (0.05, 0.0011992566408762996, 0.023490831330791032, 0.04818091930245836),
        (0.050000005, 0.0011992568757846239, 0.023490833534157485, 0.04818092394409959),
    ] {
        let out = calculate("transients.rc_ladder_step", &[
            ("r_one", 1.0), ("r_two", 2.0), ("c_one", 1.0), ("c_two", 0.5), ("v_step", 1.0), ("time", time),
        ]).unwrap();
        for (key, expected) in [("quadratic_coefficient", 1.0), ("linear_coefficient", 2.5),
            ("tau_slow", 2.0), ("tau_fast", 0.5), ("voltage_two", v2), ("capacitor_two_current", current), ("voltage_one", v1)] {
            assert_close(out[key], expected, 1e-11, 0.0);
        }
    }
    for (time, voltage, current) in [
        (0.000009999999, 0.02416707163991125, 0.04750203717565078),
        (0.00001, 0.02416707639011519, 0.04750204167646331),
        (0.000010000001, 0.024167081140319584, 0.04750204617727579),
    ] {
        let out = calculate("transients.rlc_underdamped_step", &[
            ("r", 10.0), ("l", 0.001), ("c", 1e-5), ("v_step", 5.0), ("time", time), ("error_fraction", 0.01),
        ]).unwrap();
        assert_close(out["capacitor_voltage"], voltage, 1e-11, 0.0);
        assert_close(out["series_current"], current, 1e-12, 0.0);
    }
}

#[test]
fn each_declared_input_range_and_nonfinite_domain_is_rejected() {
    for &(id, inputs, _) in EXAMPLES {
        for &(key, _) in inputs {
            let (min, max, exclusive_min, exclusive_max): (f64, f64, bool, bool) = match key {
                "r" | "r_one" | "r_two" => (0.001, 1e9, false, false),
                "c" | "c_one" | "c_two" => (1e-15, 1.0, false, false),
                "l" => (1e-12, 1e6, false, false),
                "time" => (0.0, 1e9, false, false),
                "time_high" | "time_low" => (1e-15, 1e9, false, false),
                "error_fraction" => (1e-12, 1.0, false, true),
                "slope" => (-1e9, 1e9, false, false),
                "v_step" if id == "transients.rlc_underdamped_step" => (0.0, 1e6, true, false),
                _ => (-1e6, 1e6, false, false),
            };
            let below = if min > 0.0 { min / 2.0 } else { min - 1.0 };
            let above = max + max.abs().max(1.0);
            let mut invalid = vec![below, above, f64::NAN, f64::INFINITY, f64::NEG_INFINITY];
            if min > 0.0 { invalid.extend([0.0, -min]); }
            if exclusive_min { invalid.push(min); }
            if exclusive_max { invalid.push(max); }
            for value in invalid {
                let mut rejected_inputs = inputs.to_vec();
                rejected_inputs.iter_mut().find(|(name, _)| *name == key).unwrap().1 = value;
                assert!(calculate(id, &rejected_inputs).is_err(), "{id} should reject {key}={value}");
            }
        }
    }
}

#[test]
fn scaled_startup_preserves_representable_subnormal_voltage() {
    // Independent ODE leading terms: ramp m*t^2/(2*RC), ladder Vs*t^2/(2*a),
    // RLC Vs*t^2/(2*LC). Higher-order corrections are utterly below one ULP here.
    // Binary64 subnormal spacing is exactly 2^-1074 V, so relative-only comparison
    // is unattainable; two ULPs cover final multiplication quantization, never zero.
    let ulp = f64::from_bits(1);
    for sign in [-1.0, 1.0] {
        let ramp = calculate("transients.rc_ramp_source", &[
            ("r", 1000.0), ("c", 1e-6), ("v_source_initial", 0.0), ("v_initial", 0.0),
            ("slope", sign * 1e9), ("time", 1e-165),
        ]).unwrap();
        assert_close(ramp["capacitor_voltage"], sign * 5e-319, 0.0, 2.0 * ulp);
        assert!(sign * ramp["capacitor_voltage"] > 0.0);
        let ladder = calculate("transients.rc_ladder_step", &[
            ("r_one", 1000.0), ("r_two", 1000.0), ("c_one", 1e-6), ("c_two", 1e-6),
            ("v_step", sign * 1e6), ("time", 1e-165),
        ]).unwrap();
        assert_close(ladder["voltage_two"], sign * 5e-319, 0.0, 2.0 * ulp);
        assert!(sign * ladder["voltage_two"] > 0.0);
    }
    let rlc = calculate("transients.rlc_underdamped_step", &[
        ("r", 10.0), ("l", 0.001), ("c", 1e-5), ("v_step", 1e6),
        ("time", 1e-168), ("error_fraction", 0.01),
    ]).unwrap();
    // This voltage is only ten subnormal spacings; allow one final rounding ULP.
    assert_close(rlc["capacitor_voltage"], 5e-323, 0.0, ulp);
    assert!(rlc["capacitor_voltage"] > 0.0);
}
