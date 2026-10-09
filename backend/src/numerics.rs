/// (exp(x) - 1 - x) / x², including its analytic limit at zero.
/// The ratio keeps dimensional scaling outside a potentially underflowing square.
pub(crate) fn expm1mx_ratio(x: f64) -> f64 {
    if x.abs() > 0.1 { return (x.exp_m1() / x - 1.0) / x; }
    let mut term = 0.5;
    let mut sum = term;
    for n in 3..=24 {
        term *= x / f64::from(n);
        let next = sum + term;
        if next == sum { break; }
        sum = next;
    }
    sum
}

pub(crate) fn exprel(x: f64) -> f64 {
    if x == 0.0 { 1.0 } else { x.exp_m1() / x }
}

pub(crate) fn log1prel(x: f64) -> f64 {
    if x == 0.0 { 1.0 } else { x.ln_1p() / x }
}

/// tan(acos(initial)) - tan(acos(target)), without subtracting near-equals.
pub(crate) fn pf_tangent_difference(initial: f64, target: f64) -> f64 {
    if initial == target { return 0.0; }
    let initial_sine = ((1.0 - initial) * (1.0 + initial)).sqrt();
    let target_sine = ((1.0 - target) * (1.0 + target)).sqrt();
    (target - initial) * (target + initial)
        / (initial * target * (target * initial_sine + initial * target_sine))
}

/// Power split through positive parallel paths, including extreme resistance ratios.
pub(crate) fn parallel_heat(power: f64, branch: f64, other: f64) -> f64 {
    if branch <= 0.0 || other <= 0.0 || power < 0.0 { return f64::NAN; }
    if other >= branch { return power / (1.0 + branch / other); }
    let ratio = other / branch;
    if ratio >= f64::MIN_POSITIVE {
        power * (ratio / (1.0 + ratio))
    } else {
        let root_ratio = other.sqrt() / branch.sqrt();
        ((power * root_ratio) * root_ratio) / (1.0 + ratio)
    }
}

/// Dimensional step of two distinct positive real pole rates.
pub(crate) fn step_overdamped_scaled(time: f64, slow: f64, fast: f64, amplitude: f64) -> f64 {
    if time < 0.0 || slow <= 0.0 || fast <= slow { return f64::NAN; }
    if time == 0.0 { return 0.0; }
    let ratio = slow / fast;
    let x = slow * time;
    let y = fast * time;
    if y <= 0.1 {
        // Divided differences of powers become geometric sums, including
        // nearly coincident rates. Factor amplitude before the tiny product.
        let mut term = 0.5;
        let mut geometric = 1.0;
        let mut sum = term;
        for k in 1..=22 {
            term *= -y / f64::from(k + 2);
            geometric = 1.0 + ratio * geometric;
            sum += term * geometric;
        }
        ((amplitude * y) * x) * sum
    } else {
        amplitude * ((-(-x).exp_m1() + ratio * (-y).exp_m1()) / (1.0 - ratio))
    }
}

/// Dimensional second-order underdamped step; x = natural_frequency * time.
pub(crate) fn step_underdamped_scaled(x: f64, zeta: f64, amplitude: f64) -> f64 {
    if x < 0.0 || zeta <= 0.0 || zeta >= 1.0 { return f64::NAN; }
    if x == 0.0 { return 0.0; }
    if x <= 0.1 {
        let mut previous = 0.0;
        let mut coefficient = 0.5;
        let mut power = 1.0;
        let mut sum = coefficient;
        for n in 2..=18 {
            let next = -(2.0 * zeta * f64::from(n) * coefficient + previous)
                / f64::from(n * (n + 1));
            power *= x;
            sum += next * power;
            previous = coefficient;
            coefficient = next;
        }
        ((amplitude * x) * x) * sum
    } else {
        let beta = ((1.0 - zeta) * (1.0 + zeta)).sqrt();
        let angle = beta * x;
        let decay = (-zeta * x).exp();
        amplitude * (-(-zeta * x).exp_m1()
            + decay * (2.0 * (angle / 2.0).sin().powi(2) - zeta / beta * angle.sin()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn startup_and_dimensional_scaling_survive_underflow() {
        let x = 1e-16;
        assert!((expm1mx_ratio(-x) / 0.5 - 1.0).abs() < 1e-14);
        assert!((step_underdamped_scaled(x, 0.5, 1.0) / (0.5 * x * x) - 1.0).abs() < 1e-14);
        assert!((step_overdamped_scaled(1e-20, 1000.0, 2000.0, 1.0) / 1e-34 - 1.0).abs() < 1e-14);
        assert_eq!(step_underdamped_scaled(1e-164, 0.5, 1e6), 5e-323);
        assert_eq!(step_overdamped_scaled(1e-155, 1e-9, 2e-9, 1e6), 1e-322);
        assert_eq!(step_underdamped_scaled(0.0, 0.5, 1.0), 0.0);
        assert_eq!(step_overdamped_scaled(0.0, 1000.0, 2000.0, 1.0), 0.0);
    }
    #[test]
    fn stable_step_branches_agree_with_analytic_response() {
        for x in [0.099, 0.1, 0.101, 1.0, 10.0] {
            let expected = 1.0 - (-0.5_f64 * x).exp()
                * ((0.75_f64.sqrt() * x).cos() + 0.5 / 0.75_f64.sqrt() * (0.75_f64.sqrt() * x).sin());
            assert!((step_underdamped_scaled(x, 0.5, 1.0) - expected).abs() < 2e-15);
            let expected = 1.0 - 2.0 * (-x).exp() + (-2.0 * x).exp();
            assert!((step_overdamped_scaled(x, 1.0, 2.0, 1.0) - expected).abs() < 2e-15);
        }
    }
}
