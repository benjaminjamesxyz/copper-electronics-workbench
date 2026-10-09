mod checked;
mod numerics;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap, HashSet};
use meval::{Context, Expr};
use serde::{Deserialize, Serialize};
include!(concat!(env!("OUT_DIR"), "/catalogs.rs"));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    pub key: String, pub label: String, pub unit: String, pub default: f64,
    pub min: Option<f64>, pub max: Option<f64>,
    #[serde(default)] pub exclusive_min: bool,
    #[serde(default)] pub exclusive_max: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output { pub key: String, pub label: String, pub unit: String, pub expression: String }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint { pub left: String, pub op: String, pub right: String, pub message: String }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reference { pub title: String, pub url: String }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Example {
    pub inputs: BTreeMap<String, f64>, pub expected: BTreeMap<String, f64>,
    pub relative_tolerance: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Calculation {
    pub id: String, pub title: String, pub category: String, pub level: String,
    pub description: String, pub formula: String, pub assumptions: Vec<String>,
    pub references: Vec<Reference>, pub inputs: Vec<Input>, pub outputs: Vec<Output>,
    #[serde(default)] pub constraints: Vec<Constraint>, pub example: Example,
}
struct Compiled { outputs: Vec<Expr>, constraints: Vec<(Expr, Expr)> }
pub struct Engine {
    definitions: Vec<Calculation>, compiled: Vec<Compiled>,
    by_id: HashMap<String, usize>, context: Context<'static>,
    stack: RefCell<Vec<f64>>,
}
fn identifier(key: &str) -> bool {
    let mut chars = key.chars();
    chars.next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_') && key != "pi" && key != "e"
}
impl Engine {
    pub fn new() -> Result<Self, String> {
        let mut definitions = Vec::new();
        for data in CATALOGS {
            let mut lane: Vec<Calculation> = serde_json::from_str(data).map_err(|e| e.to_string())?;
            definitions.append(&mut lane);
        }
        let mut compiled = Vec::with_capacity(definitions.len());
        let mut by_id = HashMap::with_capacity(definitions.len());
        for (index, calc) in definitions.iter().enumerate() {
            if by_id.insert(calc.id.clone(), index).is_some() { return Err(format!("Duplicate ID: {}", calc.id)); }
            if calc.inputs.is_empty() || calc.outputs.is_empty() { return Err(format!("Empty model: {}", calc.id)); }
            let mut keys = HashSet::new();
            for key in calc.inputs.iter().map(|i| &i.key).chain(calc.outputs.iter().map(|o| &o.key)) {
                if !identifier(key) || !keys.insert(key) { return Err(format!("Invalid/duplicate key {key} in {}", calc.id)); }
            }
            for c in &calc.constraints {
                if !matches!(c.op.as_str(), ">" | ">=" | "<" | "<=" | "==" | "!=") { return Err(format!("Invalid constraint in {}", calc.id)); }
            }
            let parse = |s: &str| s.parse::<Expr>().map_err(|e| format!("{}: {e}", calc.id));
            compiled.push(Compiled {
                outputs: calc.outputs.iter().map(|o| parse(&o.expression)).collect::<Result<_, _>>()?,
                constraints: calc.constraints.iter().map(|c| Ok((parse(&c.left)?, parse(&c.right)?))).collect::<Result<_, String>>()?,
            });
        }
        let mut context = Context::new();
        context.func("log10", f64::log10);
        context.func("expm1", f64::exp_m1);
        context.func("log1p", f64::ln_1p);
        context.func2("hypot", f64::hypot);
        context.func("exprel", numerics::exprel);
        context.func("log1prel", numerics::log1prel);
        context.func2("pf_tangent_difference", numerics::pf_tangent_difference);
        context.func("expm1mx_ratio", numerics::expm1mx_ratio);
        context.funcn("step_overdamped_scaled",
            |args| numerics::step_overdamped_scaled(args[0], args[1], args[2], args[3]), 4);
        context.func3("step_underdamped_scaled", numerics::step_underdamped_scaled);
        context.func3("parallel_heat", numerics::parallel_heat);
        let stack_capacity = compiled.iter().flat_map(|c|
            c.outputs.iter().chain(c.constraints.iter().flat_map(|(left, right)| [left, right])))
            .map(|expression| expression.len()).max().unwrap_or(0);
        let engine = Self { definitions, compiled, by_id, context,
            stack: RefCell::new(Vec::with_capacity(stack_capacity)) };
        for calc in &engine.definitions {
            let defaults = calc.inputs.iter().map(|i| (i.key.clone(), i.default)).collect();
            engine.calculate(&calc.id, &defaults).map_err(|e| format!("Invalid default {}: {e}", calc.id))?;
        }
        Ok(engine)
    }
    pub fn catalog(&self) -> &[Calculation] { &self.definitions }
    pub fn calculate(&self, id: &str, inputs: &BTreeMap<String, f64>) -> Result<BTreeMap<String, f64>, String> {
        let index = *self.by_id.get(id).ok_or_else(|| format!("Unknown calculator: {id}"))?;
        let calc = &self.definitions[index];
        let compiled = &self.compiled[index];
        if inputs.len() != calc.inputs.len() || inputs.keys().any(|key| !calc.inputs.iter().any(|i| &i.key == key)) {
            return Err("Supply exactly the named inputs in the catalog".into());
        }
        let mut values: HashMap<&str, f64> = HashMap::with_capacity(calc.inputs.len() + calc.outputs.len());
        for input in &calc.inputs {
            let value = *inputs.get(&input.key).ok_or_else(|| format!("Missing input: {}", input.label))?;
            if !value.is_finite() { return Err(format!("{} must be finite", input.label)); }
            if input.min.is_some_and(|min| if input.exclusive_min { value <= min } else { value < min })
                || input.max.is_some_and(|max| if input.exclusive_max { value >= max } else { value > max }) {
                return Err(format!("{} is outside its model range", input.label));
            }
            values.insert(&input.key, value);
        }
        let mut stack = self.stack.borrow_mut();
        let mut evaluate = |expr: &Expr, values: &HashMap<&str, f64>| {
            checked::evaluate(expr, values, &self.context, &mut stack)
        };
        for (constraint, (left, right)) in calc.constraints.iter().zip(&compiled.constraints) {
            let a = evaluate(left, &values)?; let b = evaluate(right, &values)?;
            let valid = match constraint.op.as_str() { ">" => a > b, ">=" => a >= b, "<" => a < b, "<=" => a <= b, "==" => a == b, "!=" => a != b, _ => false };
            if !valid { return Err(constraint.message.clone()); }
        }
        let mut results = BTreeMap::new();
        for (output, expr) in calc.outputs.iter().zip(&compiled.outputs) {
            let value = evaluate(expr, &values)?;
            values.insert(&output.key, value);
            results.insert(output.key.clone(), value);
        }
        Ok(results)
    }
}

#[derive(Deserialize)]
#[serde(tag = "command", rename_all = "snake_case")]
pub enum Request { Catalog, Calculate { id: String, inputs: BTreeMap<String, f64> } }
pub fn respond(engine: &Engine, request: &str) -> serde_json::Value {
    match serde_json::from_str::<Request>(request) {
        Ok(Request::Catalog) => serde_json::json!({"ok":true, "calculations":engine.catalog()}),
        Ok(Request::Calculate { id, inputs }) => match engine.calculate(&id, &inputs) {
            Ok(outputs) => serde_json::json!({"ok":true,"id":id,"outputs":outputs}),
            Err(error) => serde_json::json!({"ok":false,"error":error}),
        },
        Err(error) => serde_json::json!({"ok":false,"error":format!("Invalid request: {error}")}),
    }
}
