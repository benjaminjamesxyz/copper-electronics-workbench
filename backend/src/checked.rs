use std::collections::HashMap;
use meval::{Context, ContextProvider, Expr};
use meval::tokenizer::{Operation, Token};

/// Reuse meval's parsed RPN and registered functions, but reject invalid
/// intermediate arithmetic before a later operation can conceal it.
pub(crate) fn evaluate(
    expression: &Expr,
    values: &HashMap<&str, f64>,
    context: &Context<'_>,
    stack: &mut Vec<f64>,
) -> Result<f64, String> {
    stack.clear();
    for token in expression.iter() {
        let value = match token {
            Token::Number(value) => *value,
            Token::Var(name) => values.get(name.as_str()).copied()
                .or_else(|| context.get_var(name))
                .ok_or_else(|| format!("Unknown variable: {name}"))?,
            Token::Binary(operation) => {
                let right = stack.pop().ok_or("Invalid expression stack")?;
                let left = stack.pop().ok_or("Invalid expression stack")?;
                match operation {
                    Operation::Plus => left + right,
                    Operation::Minus => left - right,
                    Operation::Times => left * right,
                    Operation::Div => left / right,
                    Operation::Rem => left % right,
                    Operation::Pow => left.powf(right),
                }
            }
            Token::Unary(operation) => {
                let value = stack.pop().ok_or("Invalid expression stack")?;
                match operation {
                    Operation::Plus => value,
                    Operation::Minus => -value,
                    _ => return Err("Unsupported unary operation".into()),
                }
            }
            Token::Func(name, Some(count)) => {
                let start = stack.len().checked_sub(*count).ok_or("Invalid function arguments")?;
                let value = context.eval_func(name, &stack[start..])
                    .map_err(|error| format!("{name}: {error:?}"))?;
                stack.truncate(start);
                value
            }
            _ => return Err("Unsupported expression token".into()),
        };
        if !value.is_finite() {
            return Err("Model produced a non-finite intermediate value; check its assumptions".into());
        }
        stack.push(value);
    }
    if stack.len() != 1 { return Err("Invalid expression result".into()); }
    Ok(stack[0])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn invalid_intermediates_cannot_be_hidden_and_next_evaluation_recovers() {
        let context = Context::new();
        let values = HashMap::new();
        let mut stack = Vec::with_capacity(32);
        for source in ["1/(1e308*1e308)", "sqrt(-1)^0", "exp(1000)^0", "1/(1/0)"] {
            let expression = source.parse::<Expr>().unwrap();
            assert!(evaluate(&expression, &values, &context, &mut stack).is_err());
            let expression = "-2^2+sqrt(9)+1e-3".parse::<Expr>().unwrap();
            assert!((evaluate(&expression, &values, &context, &mut stack).unwrap() + 0.999).abs() < 1e-15);
        }
    }
}
