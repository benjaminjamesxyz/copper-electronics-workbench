mod common;

use common::{assert_close, calculate};
use std::f64::consts::PI;

// Independent SI force/energy/circuit oracles; no catalog examples are loaded.
// Nonzero expectations use relative tolerance only; algebraic zeros are exact.
const DC: &[(&str, f64)] = &[
    ("voltage", 14.0), ("armature_resistance", 2.0), ("motor_constant", 0.5),
    ("viscous_drag", 0.025), ("load_torque", 0.5),
];
const STEP: &[(&str, f64)] = &[
    ("full_steps_per_turn", 100.0), ("microsteps_per_step", 3.0),
    ("holding_torque", 2.0), ("load_torque", 1.0),
];
const GEAR: &[(&str, f64)] = &[
    ("motor_inertia", 0.01), ("load_inertia", 0.16), ("gear_ratio", 4.0),
    ("gear_efficiency", 0.5), ("load_torque", 0.4), ("initial_speed", 2.0),
    ("final_speed", 6.0), ("ramp_time", 2.0), ("motor_constant", 0.2),
    ("armature_resistance", 3.0),
];
const BRAKE: &[(&str, f64)] = &[
    ("inertia", 0.5), ("armature_resistance", 1.0), ("brake_resistance", 3.0),
    ("motor_constant", 1.0), ("initial_speed", 8.0), ("time", 2.0),
];
const PMSM: &[(&str, f64)] = &[
    ("pole_pairs", 2.0), ("mechanical_speed", 50.0), ("stator_resistance", 0.5),
    ("d_inductance", 0.01), ("q_inductance", 0.02), ("magnet_flux", 0.1),
    ("d_current", -3.0), ("q_current", 4.0),
];
const INDUCTION: &[(&str, f64)] = &[
    ("frequency", 60.0), ("pole_pairs", 3.0), ("slip", 0.25),
    ("thevenin_voltage", 20.0), ("thevenin_resistance", 3.0),
    ("thevenin_reactance", 1.0), ("rotor_resistance", 1.0), ("rotor_reactance", 3.0),
];

#[test]
fn dc_complete_nondefault_equilibrium_and_power() {
    // Choose omega=20 and I=2: K I=0.5+0.025*20 and V=2*2+0.5*20.
    let r = calculate("motors.pm_dc_load_equilibrium", DC).unwrap();
    for (key, expected) in [("speed", 20.0), ("current", 2.0), ("back_emf", 10.0),
        ("shaft_power", 10.0), ("copper_loss", 8.0), ("friction_loss", 10.0)] {
        assert_close(r[key], expected, 1e-12, 0.0);
    }
    assert_close(2.0*r["current"] + r["back_emf"], 14.0, 1e-12, 0.0);
    assert_close(0.5*r["current"], 0.5 + 0.025*r["speed"], 1e-12, 0.0);
    assert_close(r["shaft_power"] + r["copper_loss"] + r["friction_loss"],
        14.0*r["current"], 1e-12, 0.0);
}

#[test]
fn dc_unloaded_stall_and_unenergized_limits() {
    for (voltage, drag, load, speed, current, emf, copper) in [
        (14.0, 0.0, 0.0, 28.0, 0.0, 14.0, 0.0),
        (14.0, 0.025, 3.5, 0.0, 7.0, 0.0, 98.0),
        (0.0, 0.025, 0.0, 0.0, 0.0, 0.0, 0.0),
    ] {
        let r = calculate("motors.pm_dc_load_equilibrium", &[
            ("voltage", voltage), ("armature_resistance", 2.0), ("motor_constant", 0.5),
            ("viscous_drag", drag), ("load_torque", load),
        ]).unwrap();
        for (key, expected) in [("speed", speed), ("current", current), ("back_emf", emf),
            ("shaft_power", 0.0), ("copper_loss", copper), ("friction_loss", 0.0)] {
            assert_close(r[key], expected, 1e-12, 0.0);
        }
    }
}

#[test]
fn stepper_complete_nondefault_torque_angle() {
    // 25 electrical cycles/rev; half holding torque means 30 electrical degrees lag.
    let r = calculate("motors.stepper_static_microstep", STEP).unwrap();
    for (key, expected) in [("microstep_angle", PI/150.0), ("incremental_torque", 1.0),
        ("load_lag", PI/150.0), ("angular_stiffness", 25.0*3.0_f64.sqrt())] {
        assert_close(r[key], expected, 1e-12, 0.0);
    }
    assert_close(2.0*(25.0*r["load_lag"]).sin(), 1.0, 1e-12, 0.0);
}

#[test]
fn stepper_resolution_does_not_change_loaded_accuracy() {
    let base = calculate("motors.stepper_static_microstep", STEP).unwrap();
    let mut inputs = STEP.to_vec();
    inputs[1].1 = 30.0;
    let finer = calculate("motors.stepper_static_microstep", &inputs).unwrap();
    assert_close(finer["microstep_angle"], base["microstep_angle"]/10.0, 1e-12, 0.0);
    assert_close(finer["incremental_torque"], 2.0*(PI/60.0).sin(), 1e-12, 0.0);
    assert_eq!(finer["load_lag"], base["load_lag"]);
    assert_eq!(finer["angular_stiffness"], base["angular_stiffness"]);
    assert!(finer["incremental_torque"] < base["incremental_torque"]);
    inputs[0].1 = 4.0;
    inputs[1].1 = 1.0;
    inputs[3].1 = 0.0;
    let r = calculate("motors.stepper_static_microstep", &inputs).unwrap();
    for (key, expected) in [("microstep_angle", PI/2.0), ("incremental_torque", 2.0),
        ("load_lag", 0.0), ("angular_stiffness", 2.0)] {
        assert_close(r[key], expected, 1e-12, 0.0);
    }
}

#[test]
fn stepper_last_representable_stable_load_retains_stiffness() {
    let mut inputs = STEP.to_vec();
    let deficit: f64 = 2.0_f64.powi(-52);
    inputs[3].1 = 2.0 - deficit;
    let r = calculate("motors.stepper_static_microstep", &inputs).unwrap();
    // cos(lag_e)=sqrt(1-(1-deficit/2)^2); retained deficit avoids squared cancellation.
    let stiffness: f64 = 50.0*(deficit - deficit*deficit/4.0).sqrt();
    assert_close(r["angular_stiffness"], stiffness, 1e-12, 0.0);
    assert_close(r["load_lag"], (1.0-deficit/2.0).asin()/25.0, 1e-12, 0.0);
    assert!(r["load_lag"] < PI/50.0);
    assert!(r["angular_stiffness"] > 0.0);
}

#[test]
fn gear_complete_nondefault_two_inertia_balance() {
    // alpha_l=2, input inertia torque=.08; output demand=.72, gear input=.36.
    let r = calculate("motors.geared_axis_acceleration", GEAR).unwrap();
    for (key, expected) in [("reflected_inertia", 0.01), ("motor_acceleration", 8.0),
        ("motor_torque", 0.44), ("armature_current", 2.2), ("motor_final_speed", 24.0),
        ("required_final_voltage", 11.4)] {
        assert_close(r[key], expected, 1e-12, 0.0);
    }
    assert_close((r["motor_torque"]-0.01*r["motor_acceleration"])*0.5*4.0,
        0.16*2.0+0.4, 1e-12, 0.0);
    assert_close(0.5*r["reflected_inertia"]*24.0_f64.powi(2),
        0.5*0.16*6.0_f64.powi(2), 1e-12, 0.0);
}

#[test]
fn gear_lossless_constant_speed_unity_and_single_inertias() {
    for (jm, jl, ratio, eta, initial, load, inertia, acceleration, torque, current, speed, voltage) in [
        (0.01, 0.16, 4.0, 1.0, 2.0, 0.4, 0.01, 8.0, 0.26, 1.3, 24.0, 8.7),
        (0.01, 0.16, 4.0, 0.5, 6.0, 0.4, 0.01, 0.0, 0.2, 1.0, 24.0, 7.8),
        (0.01, 0.16, 1.0, 1.0, 2.0, 0.4, 0.16, 2.0, 0.74, 3.7, 6.0, 12.3),
        (0.0, 0.16, 4.0, 0.5, 2.0, 0.0, 0.01, 8.0, 0.16, 0.8, 24.0, 7.2),
        (0.01, 0.0, 4.0, 0.5, 2.0, 0.0, 0.0, 8.0, 0.08, 0.4, 24.0, 6.0),
    ] {
        let mut inputs = GEAR.to_vec();
        inputs[0].1 = jm; inputs[1].1 = jl; inputs[2].1 = ratio;
        inputs[3].1 = eta; inputs[4].1 = load; inputs[5].1 = initial;
        let r = calculate("motors.geared_axis_acceleration", &inputs).unwrap();
        for (key, expected) in [("reflected_inertia", inertia), ("motor_acceleration", acceleration),
            ("motor_torque", torque), ("armature_current", current), ("motor_final_speed", speed),
            ("required_final_voltage", voltage)] {
            assert_close(r[key], expected, 1e-12, 0.0);
        }
    }
}

#[test]
fn brake_complete_nondefault_generator_energy() {
    // tau=2; speed falls by e, initial kinetic energy=16 J, heat ratio=3:1.
    let decay: f64 = (-1.0_f64).exp();
    let removed: f64 = 16.0*(1.0-decay*decay);
    let r = calculate("motors.resistive_dynamic_braking", BRAKE).unwrap();
    for (key, expected) in [("mechanical_tau", 2.0), ("speed", 8.0*decay),
        ("braking_current", 2.0*decay), ("electromagnetic_torque", -2.0*decay),
        ("angular_travel", 16.0*(1.0-decay)), ("brake_energy", removed*0.75),
        ("winding_energy", removed*0.25)] {
        assert_close(r[key], expected, 1e-12, 0.0);
    }
    assert_close(r["brake_energy"]+r["winding_energy"]+0.25*r["speed"].powi(2), 16.0, 1e-12, 0.0);
    assert_close(-r["electromagnetic_torque"]*r["speed"], 4.0*r["braking_current"].powi(2), 1e-12, 0.0);
}

#[test]
fn brake_initial_tiny_and_long_time_limits() {
    for time in [0.0, 1e-20, 100.0] {
        let mut inputs = BRAKE.to_vec(); inputs[5].1 = time;
        let r = calculate("motors.resistive_dynamic_braking", &inputs).unwrap();
        let (travel, brake_heat, winding_heat): (f64, f64, f64) = if time == 0.0 {
            (0.0, 0.0, 0.0)
        } else if time < 1e-10 {
            // Initial rates: omega=8, I=2, Rb I²=12 W, Ra I²=4 W.
            (8e-20, 12e-20, 4e-20)
        } else { (16.0, 12.0, 4.0) };
        let decay: f64 = (-time/2.0).exp();
        for (key, expected) in [("mechanical_tau", 2.0), ("speed", 8.0*decay),
            ("braking_current", 2.0*decay), ("electromagnetic_torque", -2.0*decay),
            ("angular_travel", travel), ("brake_energy", brake_heat), ("winding_energy", winding_heat)] {
            assert_close(r[key], expected, 1e-12, 0.0);
        }
    }
}

#[test]
fn brake_shorted_and_stationary_limits() {
    let mut inputs = BRAKE.to_vec(); inputs[2].1 = 0.0;
    let r = calculate("motors.resistive_dynamic_braking", &inputs).unwrap();
    let decay: f64 = (-4.0_f64).exp();
    for (key, expected) in [("mechanical_tau", 0.5), ("speed", 8.0*decay),
        ("braking_current", 8.0*decay), ("electromagnetic_torque", -8.0*decay),
        ("angular_travel", 4.0*(1.0-decay)), ("brake_energy", 0.0),
        ("winding_energy", 16.0*(1.0-decay*decay))] {
        assert_close(r[key], expected, 1e-12, 0.0);
    }
    inputs[4].1 = 0.0;
    let r = calculate("motors.resistive_dynamic_braking", &inputs).unwrap();
    assert_close(r["mechanical_tau"], 0.5, 1e-12, 0.0);
    for key in ["speed", "braking_current", "electromagnetic_torque", "angular_travel", "brake_energy", "winding_energy"] {
        assert_eq!(r[key], 0.0);
    }
}

#[test]
fn pmsm_complete_nondefault_saliency_and_generation() {
    // Peak current is a 3-4-5 triangle; PM torque=1.2, reluctance torque=.36 N m.
    for (iq, vd, vq, torque, power) in [(4.0, -9.5, 9.0, 1.56, 78.0), (-4.0, 6.5, 5.0, -1.56, -78.0)] {
        let mut inputs = PMSM.to_vec(); inputs[7].1 = iq;
        let r = calculate("motors.salient_pmsm_dq_operating_point", &inputs).unwrap();
        let voltage: f64 = vd*vd+vq*vq;
        for (key, expected) in [("electrical_speed", 100.0), ("d_voltage", vd), ("q_voltage", vq),
            ("phase_voltage_peak", voltage.sqrt()), ("phase_current_peak", 5.0),
            ("electromagnetic_torque", torque), ("converted_power", power), ("stator_copper_loss", 18.75)] {
            assert_close(r[key], expected, 1e-12, 0.0);
        }
        assert_close(1.5*(-3.0*r["d_voltage"]+iq*r["q_voltage"]),
            r["converted_power"]+r["stator_copper_loss"], 1e-12, 0.0);
    }
}

#[test]
fn pmsm_surface_standstill_open_current_and_lossless_reverse() {
    // Each row exercises a distinct admitted physical limit, including positive d current.
    for (speed, resistance, ld, id, iq, we, vd, vq, torque, power, copper) in [
        (50.0, 0.5, 0.02, -3.0, 4.0, 100.0, -9.5, 6.0, 1.2, 60.0, 18.75),
        (0.0, 0.5, 0.01, -3.0, 4.0, 0.0, -1.5, 2.0, 1.56, 0.0, 18.75),
        (50.0, 0.5, 0.01, 0.0, 0.0, 100.0, 0.0, 10.0, 0.0, 0.0, 0.0),
        (-50.0, 0.0, 0.01, 3.0, 4.0, -100.0, 8.0, -13.0, 0.84, -42.0, 0.0),
    ] {
        let mut inputs = PMSM.to_vec();
        inputs[1].1 = speed; inputs[2].1 = resistance; inputs[3].1 = ld;
        inputs[6].1 = id; inputs[7].1 = iq;
        let r = calculate("motors.salient_pmsm_dq_operating_point", &inputs).unwrap();
        let voltage_squared: f64 = vd*vd+vq*vq;
        let current_squared: f64 = id*id+iq*iq;
        for (key, expected) in [("electrical_speed", we), ("d_voltage", vd), ("q_voltage", vq),
            ("phase_voltage_peak", voltage_squared.sqrt()), ("phase_current_peak", current_squared.sqrt()),
            ("electromagnetic_torque", torque), ("converted_power", power), ("stator_copper_loss", copper)] {
            assert_close(r[key], expected, 1e-12, 0.0);
        }
    }
}

#[test]
fn induction_complete_nondefault_original_rotor_circuit() {
    // Original circuit R2/s=4, total impedance 7+j4; three RMS phases, not peak dq.
    let current: f64 = 20.0/65.0_f64.sqrt();
    let gap: f64 = 4800.0/65.0;
    let r = calculate("motors.induction_slip_power", INDUCTION).unwrap();
    for (key, expected) in [("synchronous_speed", 40.0*PI), ("rotor_speed", 30.0*PI),
        ("rotor_current", current), ("air_gap_power", gap), ("electromagnetic_torque", gap/(40.0*PI)),
        ("rotor_copper_loss", 1200.0/65.0), ("converted_power", 3600.0/65.0),
        ("breakdown_slip", 0.2), ("breakdown_torque", 1.875/PI)] {
        assert_close(r[key], expected, 1e-12, 0.0);
    }
    assert_close(r["converted_power"], r["electromagnetic_torque"]*r["rotor_speed"], 1e-12, 0.0);
    assert_close(r["air_gap_power"], r["rotor_copper_loss"]+r["converted_power"], 1e-12, 0.0);
    assert_close(r["rotor_copper_loss"], 3.0*r["rotor_current"].powi(2), 1e-12, 0.0);
}

#[test]
fn induction_synchronous_locked_and_zero_excitation_limits() {
    for (slip, voltage, speed, current, gap, copper, power, peak) in [
        (0.0, 20.0, 40.0*PI, 0.0, 0.0, 0.0, 0.0, 1.875/PI),
        (1.0, 20.0, 0.0, 20.0/32.0_f64.sqrt(), 37.5, 37.5, 0.0, 1.875/PI),
        (0.25, 0.0, 30.0*PI, 0.0, 0.0, 0.0, 0.0, 0.0),
    ] {
        let mut inputs = INDUCTION.to_vec(); inputs[2].1 = slip; inputs[3].1 = voltage;
        let r = calculate("motors.induction_slip_power", &inputs).unwrap();
        for (key, expected) in [("synchronous_speed", 40.0*PI), ("rotor_speed", speed),
            ("rotor_current", current), ("air_gap_power", gap), ("rotor_copper_loss", copper),
            ("converted_power", power), ("electromagnetic_torque", gap/(40.0*PI)),
            ("breakdown_slip", 0.2), ("breakdown_torque", peak)] {
            assert_close(r[key], expected, 1e-12, 0.0);
        }
    }
}

#[test]
fn induction_peak_neighbors_rotor_resistance_and_lossless_stator() {
    let mut inputs = INDUCTION.to_vec(); inputs[2].1 = 0.2;
    let peak = calculate("motors.induction_slip_power", &inputs).unwrap();
    assert_close(peak["electromagnetic_torque"], 1.875/PI, 1e-12, 0.0);
    for slip in [0.19, 0.21] {
        inputs[2].1 = slip;
        let r = calculate("motors.induction_slip_power", &inputs).unwrap();
        assert!(r["electromagnetic_torque"] < peak["electromagnetic_torque"]);
    }
    inputs[6].1 = 10.0;
    let r = calculate("motors.induction_slip_power", &inputs).unwrap();
    assert_close(r["breakdown_slip"], 2.0, 1e-12, 0.0);
    assert_eq!(r["breakdown_torque"], peak["breakdown_torque"]);
    inputs[4].1 = 0.0;
    let r = calculate("motors.induction_slip_power", &inputs).unwrap();
    assert_close(r["breakdown_slip"], 2.5, 1e-12, 0.0);
    assert_close(r["breakdown_torque"], 3.75/PI, 1e-12, 0.0);
    for (xth, xrotor) in [(0.0, 4.0), (4.0, 0.0)] {
        inputs[5].1 = xth; inputs[7].1 = xrotor;
        let split = calculate("motors.induction_slip_power", &inputs).unwrap();
        for (key, expected) in &r { assert_close(split[key], *expected, 1e-12, 0.0); }
    }
}

#[test]
fn every_scalar_and_coupled_domain_rejects_then_recovers() {
    for (id, nominal, invalid) in [
        ("motors.pm_dc_load_equilibrium", DC, &[(0, -1.0), (1, 0.0), (1, -1.0), (2, 0.0), (2, -1.0), (3, -1.0), (4, -1.0), (4, 3.5000000000000004)][..]),
        ("motors.stepper_static_microstep", STEP, &[(0, 0.0), (0, 3.0), (0, 102.0), (0, 100.5), (1, 0.0), (1, 1.5), (2, 0.0), (2, -1.0), (3, -1.0), (3, 2.0), (3, 3.0)][..]),
        ("motors.geared_axis_acceleration", GEAR, &[(0, -1.0), (1, -1.0), (2, 0.0), (2, -1.0), (3, 0.0), (3, -1.0), (3, 1.01), (4, -1.0), (5, -1.0), (6, -1.0), (6, 1.0), (7, 0.0), (7, -1.0), (8, 0.0), (8, -1.0), (9, 0.0), (9, -1.0)][..]),
        ("motors.resistive_dynamic_braking", BRAKE, &[(0, 0.0), (0, -1.0), (1, 0.0), (1, -1.0), (2, -1.0), (3, 0.0), (3, -1.0), (4, -1.0), (5, -1.0)][..]),
        ("motors.salient_pmsm_dq_operating_point", PMSM, &[(0, 0.0), (0, -1.0), (0, 1.5), (2, -1.0), (3, 0.0), (3, -1.0), (4, 0.0), (4, -1.0), (5, 0.0), (5, -1.0)][..]),
        ("motors.induction_slip_power", INDUCTION, &[(0, 0.0), (0, -1.0), (1, 0.0), (1, -1.0), (1, 1.5), (2, -1e-20), (2, 1.0000000000000002), (3, -1.0), (4, -1.0), (5, -1.0), (6, 0.0), (6, -1.0), (7, -1.0)][..]),
    ] {
        let expected = calculate(id, nominal).unwrap();
        for &(index, bad) in invalid {
            let mut inputs = nominal.to_vec(); inputs[index].1 = bad;
            assert!(calculate(id, &inputs).is_err(), "accepted {id} {}={bad}", inputs[index].0);
            let recovered = calculate(id, nominal).unwrap();
            for (key, value) in &expected { assert_eq!(recovered[key], *value); }
        }
    }
    let mut gear = GEAR.to_vec(); gear[0].1 = 0.0; gear[1].1 = 0.0;
    assert!(calculate("motors.geared_axis_acceleration", &gear).is_err());
    gear_complete_nondefault_two_inertia_balance();
    let mut induction = INDUCTION.to_vec(); induction[5].1 = 0.0; induction[7].1 = 0.0;
    assert!(calculate("motors.induction_slip_power", &induction).is_err());
    induction_complete_nondefault_original_rotor_circuit();
    dc_complete_nondefault_equilibrium_and_power();
    stepper_complete_nondefault_torque_angle();
    brake_complete_nondefault_generator_energy();
    pmsm_complete_nondefault_saliency_and_generation();
}

#[test]
fn every_input_rejects_nonfinite_missing_and_unknown_keys_with_recovery() {
    for (id, nominal) in [
        ("motors.pm_dc_load_equilibrium", DC), ("motors.stepper_static_microstep", STEP),
        ("motors.geared_axis_acceleration", GEAR), ("motors.resistive_dynamic_braking", BRAKE),
        ("motors.salient_pmsm_dq_operating_point", PMSM), ("motors.induction_slip_power", INDUCTION),
    ] {
        let expected = calculate(id, nominal).unwrap();
        for index in 0..nominal.len() {
            for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                let mut inputs = nominal.to_vec(); inputs[index].1 = bad;
                assert!(calculate(id, &inputs).is_err(), "accepted {id} {}={bad}", inputs[index].0);
                assert_eq!(calculate(id, nominal).unwrap(), expected);
            }
            let mut missing = nominal.to_vec(); missing.remove(index);
            assert!(calculate(id, &missing).is_err(), "accepted missing {id} {}", nominal[index].0);
            assert_eq!(calculate(id, nominal).unwrap(), expected);
        }
        let mut unknown = nominal.to_vec(); unknown.push(("unexpected_input", 1.0));
        assert!(calculate(id, &unknown).is_err());
        assert_eq!(calculate(id, nominal).unwrap(), expected);
    }
}

#[test]
fn dc_tiny_constants_preserve_representable_unloaded_speed() {
    // V/K=1; squaring K first destroys both numerator and denominator.
    let r = calculate("motors.pm_dc_load_equilibrium", &[
        ("voltage", 1e-200), ("armature_resistance", 1.0), ("motor_constant", 1e-200),
        ("viscous_drag", 0.0), ("load_torque", 0.0),
    ]).unwrap();
    for (key, expected) in [("speed", 1.0), ("current", 0.0), ("back_emf", 1e-200),
        ("shaft_power", 0.0), ("copper_loss", 0.0), ("friction_loss", 0.0)] {
        assert_close(r[key], expected, 1e-12, 0.0);
    }
}

#[test]
fn brake_dimensional_energy_survives_speed_square_underflow() {
    // Kinetic energy=(sqrt(J)*omega)^2/2=5e-141 J, not zero.
    let r = calculate("motors.resistive_dynamic_braking", &[
        ("inertia", 1e200), ("armature_resistance", 1.0), ("brake_resistance", 1.0),
        ("motor_constant", 1e100), ("initial_speed", 1e-170), ("time", 2.0),
    ]).unwrap();
    let decay: f64 = (-1.0_f64).exp();
    let heat: f64 = 2.5e-141*(1.0-decay*decay);
    for (key, expected) in [("mechanical_tau", 2.0), ("speed", 1e-170*decay),
        ("braking_current", 5e-71*decay), ("electromagnetic_torque", -5e29*decay),
        ("angular_travel", 2e-170*(1.0-decay)), ("brake_energy", heat), ("winding_energy", heat)] {
        assert_close(r[key], expected, 1e-12, 0.0);
    }
}

#[test]
fn pmsm_dimensional_copper_loss_survives_current_square_underflow() {
    let r = calculate("motors.salient_pmsm_dq_operating_point", &[
        ("pole_pairs", 1.0), ("mechanical_speed", 0.0), ("stator_resistance", 1e200),
        ("d_inductance", 1.0), ("q_inductance", 1.0), ("magnet_flux", 1.0),
        ("d_current", 1e-200), ("q_current", 0.0),
    ]).unwrap();
    // Direct terminal power is 3/2 * vd * id = 1.5e-200 W at standstill.
    for (key, expected) in [("electrical_speed", 0.0), ("d_voltage", 1.0), ("q_voltage", 0.0),
        ("phase_voltage_peak", 1.0), ("phase_current_peak", 1e-200),
        ("electromagnetic_torque", 0.0), ("converted_power", 0.0), ("stator_copper_loss", 1.5e-200)] {
        assert_close(r[key], expected, 1e-12, 0.0);
    }
}

#[test]
fn induction_peak_torque_survives_voltage_square_underflow() {
    let r = calculate("motors.induction_slip_power", &[
        ("frequency", 1.0), ("pole_pairs", 1.0), ("slip", 1.0),
        ("thevenin_voltage", 1e-170), ("thevenin_resistance", 0.0),
        ("thevenin_reactance", 1e-200), ("rotor_resistance", 1e-200), ("rotor_reactance", 0.0),
    ]).unwrap();
    // R2=X and s=1: maximum-power match, |I|=V/(sqrt(2) R2), Pag=1.5e-140 W.
    for (key, expected) in [("synchronous_speed", 2.0*PI), ("rotor_speed", 0.0),
        ("rotor_current", 1e30/2.0_f64.sqrt()), ("air_gap_power", 1.5e-140),
        ("electromagnetic_torque", 7.5e-141/PI), ("rotor_copper_loss", 1.5e-140),
        ("converted_power", 0.0), ("breakdown_slip", 1.0), ("breakdown_torque", 7.5e-141/PI)] {
        assert_close(r[key], expected, 1e-12, 0.0);
    }
}

#[test]
fn stepper_finite_stiffness_survives_torque_sum_overflow() {
    let r = calculate("motors.stepper_static_microstep", &[
        ("full_steps_per_turn", 4.0), ("microsteps_per_step", 1.0),
        ("holding_torque", 1e308), ("load_torque", 9e307),
    ]).unwrap();
    for (key, expected) in [("microstep_angle", PI/2.0), ("incremental_torque", 1e308),
        ("load_lag", 0.9_f64.asin()), ("angular_stiffness", 0.19_f64.sqrt()*1e308)] {
        assert_close(r[key], expected, 1e-12, 0.0);
    }
}

#[test]
fn stepper_large_integer_resolution_retains_subnormal_increment() {
    let r = calculate("motors.stepper_static_microstep", &[
        ("full_steps_per_turn", 4.0), ("microsteps_per_step", 1e308),
        ("holding_torque", 0.5), ("load_torque", 0.0),
    ]).unwrap();
    for (key, expected) in [("microstep_angle", (PI/2.0)/1e308),
        ("incremental_torque", (PI/4.0)/1e308), ("load_lag", 0.0), ("angular_stiffness", 0.5)] {
        assert_close(r[key], expected, 1e-12, 0.0);
    }
}

#[test]
fn gear_inertia_domain_does_not_require_representable_sum() {
    let r = calculate("motors.geared_axis_acceleration", &[
        ("motor_inertia", 1e308), ("load_inertia", 1e308), ("gear_ratio", 1.0),
        ("gear_efficiency", 1.0), ("load_torque", 0.0), ("initial_speed", 0.0),
        ("final_speed", 0.0), ("ramp_time", 1.0), ("motor_constant", 1.0),
        ("armature_resistance", 1.0),
    ]).unwrap();
    for (key, expected) in [("reflected_inertia", 1e308), ("motor_acceleration", 0.0),
        ("motor_torque", 0.0), ("armature_current", 0.0), ("motor_final_speed", 0.0),
        ("required_final_voltage", 0.0)] {
        assert_close(r[key], expected, 1e-12, 0.0);
    }
}

#[test]
fn brake_long_time_travel_stays_at_finite_integral_limit() {
    let r = calculate("motors.resistive_dynamic_braking", &[
        ("inertia", 0.002), ("armature_resistance", 1.0), ("brake_resistance", 3.0),
        ("motor_constant", 0.1), ("initial_speed", 100.0), ("time", 1e307),
    ]).unwrap();
    for (key, expected) in [("mechanical_tau", 0.8), ("speed", 0.0),
        ("braking_current", 0.0), ("electromagnetic_torque", 0.0),
        ("angular_travel", 80.0), ("brake_energy", 7.5), ("winding_energy", 2.5)] {
        assert_close(r[key], expected, 1e-12, 0.0);
    }
}

#[test]
fn pmsm_dimensional_copper_loss_survives_current_square_overflow() {
    let r = calculate("motors.salient_pmsm_dq_operating_point", &[
        ("pole_pairs", 1.0), ("mechanical_speed", 0.0), ("stator_resistance", 1e-200),
        ("d_inductance", 1.0), ("q_inductance", 1.0), ("magnet_flux", 1.0),
        ("d_current", 1e155), ("q_current", 0.0),
    ]).unwrap();
    for (key, expected) in [("electrical_speed", 0.0), ("d_voltage", 1e-45), ("q_voltage", 0.0),
        ("phase_voltage_peak", 1e-45), ("phase_current_peak", 1e155),
        ("electromagnetic_torque", 0.0), ("converted_power", 0.0), ("stator_copper_loss", 1.5e110)] {
        assert_close(r[key], expected, 1e-12, 0.0);
    }
}

#[test]
fn induction_small_rotor_resistance_preserves_zero_and_tiny_slip() {
    for (slip, current, gap, copper) in [(0.0, 0.0, 0.0, 0.0), (1e-200, 88.0, 23232.0, 2.3232e-196)] {
        let r = calculate("motors.induction_slip_power", &[
            ("frequency", 50.0), ("pole_pairs", 2.0), ("slip", slip),
            ("thevenin_voltage", 220.0), ("thevenin_resistance", 0.5),
            ("thevenin_reactance", 1.0), ("rotor_resistance", 1e-200), ("rotor_reactance", 1.0),
        ]).unwrap();
        // For s=1e-200 the original circuit is 1.5+j2 ohm, so I2=88 A.
        for (key, expected) in [("synchronous_speed", 50.0*PI), ("rotor_speed", 50.0*PI),
            ("rotor_current", current), ("air_gap_power", gap), ("rotor_copper_loss", copper),
            ("converted_power", gap), ("electromagnetic_torque", gap/(50.0*PI)),
            ("breakdown_slip", 1e-200/4.25_f64.sqrt()),
            ("breakdown_torque", 145200.0/(100.0*PI*(0.5+4.25_f64.sqrt())))] {
            assert_close(r[key], expected, 1e-12, 0.0);
        }
    }
}
