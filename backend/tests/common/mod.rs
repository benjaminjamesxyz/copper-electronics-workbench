use std::collections::BTreeMap;
use pcb_calculator::Engine;

thread_local! {
    // Context callbacks are thread-local; each test worker parses the catalog only once.
    static ENGINE: Engine = Engine::new().expect("valid embedded calculator catalog");
}

pub fn calculate(id: &str, inputs: &[(&str, f64)]) -> Result<BTreeMap<String, f64>, String> {
    let inputs = inputs.iter().map(|(key, value)| ((*key).to_owned(), *value)).collect();
    ENGINE.with(|engine| engine.calculate(id, &inputs))
}

/// Relative tolerance is strict even for very small electrical quantities.
/// Callers must explicitly supply a physically justified absolute tolerance for zero boundaries.
pub fn assert_close(actual: f64, expected: f64, relative: f64, absolute: f64) {
    let limit = (expected.abs() * relative).max(absolute);
    assert!(actual.is_finite() && expected.is_finite() && (actual - expected).abs() <= limit,
        "expected {expected:.16e}, observed {actual:.16e}, limit {limit:.4e}");
}
