//! # RPN-Reckoner
//! Reverse Polish Notation (: RPN) - Reckoner
//! # Example1
//! ```
//! let expression = String::from("1 2 +");
//! let solution = rpn_reckoner::eval(expression).unwrap();
//! println!("{}", solution); // -> 3
//! ```
//! # Example2
//! ```
//! let expression = String::from("5 2 - 3 +");
//! let solution = rpn_reckoner::eval(expression).unwrap();
//! println!("{}", solution); // -> 6
//! ```
//! # Example3
//! ```
//! let expression = String::from("5 2 3 ^ +");
//! let solution = rpn_reckoner::eval(expression).unwrap();
//! println!("{}", solution); // -> 13
//! ```
//! # Example4
//! ```
//! let expression = String::from("4 ! 2 +");
//! let solution = rpn_reckoner::eval(expression).unwrap();
//! println!("{}", solution); // -> 26
//! ```

mod operation;


/// The function computes RPN expressions
pub fn eval(expression: String) -> Result<f64, String> {
    // Separate tokens with white space
    let tokens = expression.split_whitespace();

    let mut stack: Vec<f64> = Vec::new();
    for token in tokens {
        let t = token.trim();
        if t == "" { continue; }

        // Push values onto the stack
        match t.parse::<f64>() {
            Ok(value) => { stack.push(value); continue; },
            Err(_) => 0.0,
        };

        // Push calculation results onto the stack
        let right = stack.pop().unwrap_or(0.0);
        let left = stack.pop().unwrap_or(0.0);
        match t {
            // Operator
            "+" => stack.push(left + right),
            "-" => stack.push(left - right),
            "*" => stack.push(left * right),
            "/" => stack.push(left / right),
            "%" => stack.push(left % right),

            // Power
            "^" => stack.push(left.powf(right)),
            "**" => stack.push(left.powf(right)),

            // Factorial
            "!" => {
                if left != 0.0 { stack.push(left); }
                stack.push(operation::factorial(right)); },

            // Error
            _ => return Err(format!("[Error] Invalid operator: {}", t)),
        }
    }

    if stack.len() == 0 { return Err(format!("[Error] No result.")); }
    if stack.len() > 1 { return Err(format!("[Error] Too many values in the stack.")); }

    Ok(stack.pop().unwrap_or(0.0))
}


mod test;