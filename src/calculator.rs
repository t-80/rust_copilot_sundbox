
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(Calculator::add(2, 3), 5);
        assert_eq!(Calculator::add(-5, 10), 5);
        assert_eq!(Calculator::add(0, 0), 0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(Calculator::subtract(5, 3), 2);
        assert_eq!(Calculator::subtract(10, -5), 15);
        assert_eq!(Calculator::subtract(0, 0), 0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(Calculator::multiply(2, 3), 6);
        assert_eq!(Calculator::multiply(-5, 10), -50);
        assert_eq!(Calculator::multiply(0, 0), 0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(Calculator::divide(10, 2), Ok(5));
        assert_eq!(Calculator::divide(10, 0), Err("Cannot divide by zero"));
        assert_eq!(Calculator::divide(0, 5), Ok(0));
    }
}


pub struct Calculator {
}

/// A simple calculator struct.
impl Calculator {
    /// Adds two numbers and returns the result.
    ///
    /// # Arguments
    ///
    /// * `a` - The first number.
    /// * `b` - The second number.
    ///
    /// # Returns
    ///
    /// The sum of `a` and `b`.
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    /// Subtracts two numbers and returns the result.
    ///
    /// # Arguments
    ///
    /// * `a` - The first number.
    /// * `b` - The second number.
    ///
    /// # Returns
    ///
    /// The difference between `a` and `b`.
    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }

    /// Multiplies two numbers and returns the result.
    ///
    /// # Arguments
    ///
    /// * `a` - The first number.
    /// * `b` - The second number.
    ///
    /// # Returns
    ///
    /// The product of `a` and `b`.
    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    /// Divides two numbers and returns the result as a `Result`.
    ///
    /// # Arguments
    ///
    /// * `a` - The dividend.
    /// * `b` - The divisor.
    ///
    /// # Returns
    ///
    /// - `Ok(result)` if the division is successful, where `result` is the quotient of `a` divided by `b`.
    /// - `Err("Cannot divide by zero")` if `b` is zero.
    pub fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
        if b == 0 {
            Err("Cannot divide by zero")
        } else {
            Ok(a / b)
        }
    }
}// This file is intentionally left blank.