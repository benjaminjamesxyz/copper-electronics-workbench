/// exp(x) - 1 - x, without cancellation near zero.
pub(crate) fn expm1mx(x: f64) -> f64 {
    if x.abs() > 0.1 { return x.exp_m1() - x; }
    let mut term = x * x / 2.0;
    let mut sum = term;
    for n in 3..=24 {
        term *= x / f64::from(n);
        let next = sum + term;
        if next == sum { break; }
        sum = next;
    }
    sum
}

/// Unit step response of two distinct positive real pole rates.
/// The short-time expression removes the analytically cancelling linear terms.
pub(crate) fn step_overdamped(time: f64, slow: f64, fast: f64) -> f64 {
    if time < 0.0 || slow <= 0.0 || fast <= slow { return f64::NAN; }
    if time == 0.0 { return 0.0; }
    let ratio = slow / fast;
    let x = slow * time;
    let y = fast * time;
    if y <= 0.1 {
        (ratio * expm1mx(-y) - expm1mx(-x)) / (1.0 - ratio)
    } else {
        (-(-x).exp_m1() + ratio * (-y).exp_m1()) / (1.0 - ratio)
    }
}

/// Unit second-order underdamped step; x = natural_frequency * time.
/// For x <= 0.1, coefficients follow y'' + 2*zeta*y' + y = 1,
/// y(0)=y'(0)=0. Nineteen terms put truncation below double precision.
pub(crate) fn step_underdamped(x: f64, zeta: f64) -> f64 {
    if x < 0.0 || zeta <= 0.0 || zeta >= 1.0 { return f64::NAN; }
    if x == 0.0 { return 0.0; }
    if x <= 0.1 {
        let mut previous = 0.0;
        let mut coefficient = 0.5;
        let mut power = x * x;
        let mut sum = coefficient * power;
        for n in 2..=18 {
            let next = -(2.0 * zeta * f64::from(n) * coefficient + previous)
                / f64::from(n * (n + 1));
            power *= x;
            sum += next * power;
            previous = coefficient;
            coefficient = next;
        }
        sum
    } else {
        let beta = ((1.0 - zeta) * (1.0 + zeta)).sqrt();
        let angle = beta * x;
        let decay = (-zeta * x).exp();
        -(-zeta * x).exp_m1()
            + decay * (2.0 * (angle / 2.0).sin().powi(2) - zeta / beta * angle.sin())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn second_order_startup_survives_subtraction_scale() {
        let x = 1e-16;
        assert!((expm1mx(-x) / (0.5 * x * x) - 1.0).abs() < 1e-14);
        assert!((step_underdamped(x, 0.5) / (0.5 * x * x) - 1.0).abs() < 1e-14);
        assert!((step_overdamped(1e-20, 1000.0, 2000.0) / 1e-34 - 1.0).abs() < 1e-14);
        assert_eq!(step_underdamped(0.0, 0.5), 0.0);
        assert_eq!(step_overdamped(0.0, 1000.0, 2000.0), 0.0);
    }
    #[test]
    fn stable_step_branches_agree_with_analytic_response() {
        for x in [0.099, 0.1, 0.101, 1.0, 10.0] {
            let expected = 1.0 - (-0.5_f64 * x).exp()
                * ((0.75_f64.sqrt() * x).cos() + 0.5 / 0.75_f64.sqrt() * (0.75_f64.sqrt() * x).sin());
            assert!((step_underdamped(x, 0.5) - expected).abs() < 2e-15);
            let expected = 1.0 - 2.0 * (-x).exp() + (-2.0 * x).exp();
            assert!((step_overdamped(x, 1.0, 2.0) - expected).abs() < 2e-15);
        }
    }
}
