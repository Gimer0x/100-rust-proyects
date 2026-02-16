// Day 76: Project: Unit & Integration Testing Suite
// Today you'll build a project that showcases unit testing, integration testing, 
// and best practices using cargo test. You’ll learn how to use #[test], organize 
// tests in modules, and create an tests/ directory for integration.
// Key Concepts:
// + #[cfg(test)] — Compiles test-only code
// + cargo test auto-discovers all unit & integration tests
// + tests/ folder = black-box integration testing
// + assert_eq!, assert!, .unwrap() for checks
// You now have a full Rust testing suite with clear structure and test coverage, 
// essential for all professional Rust projects.

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".into())
    } else {
        Ok(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let result = add(2, 3);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_divide_success() {
        assert_eq!(divide(10.0, 2.0), Ok(5.0));
    }

    #[test]
    fn test_divide_by_zero() {
        assert!(divide(8.0, 0.0,).is_err());
    }
}
