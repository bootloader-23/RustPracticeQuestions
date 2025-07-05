// math_functions.rs

use std::f64::consts::PI;

fn main() {
    let x: f64 = 4.0;
    let y: f64 = 2.0;

    println!("--- Basic Arithmetic ---");
    // Standard arithmetic operations
    println!("x + y = {}", x + y);       // Addition
    println!("x - y = {}", x - y);       // Subtraction
    println!("x * y = {}", x * y);       // Multiplication
    println!("x / y = {}", x / y);       // Division
    println!("x % y = {}", x % y);       // Remainder (modulo)

    println!("\n--- Basic Math Methods ---");
    println!("abs(x) = {}", x.abs());       // Absolute value
    println!("floor(x) = {}", x.floor());   // Largest integer ≤ x
    println!("ceil(x) = {}", x.ceil());     // Smallest integer ≥ x
    println!("round(x) = {}", x.round());   // Round to nearest integer
    println!("trunc(x) = {}", x.trunc());   // Truncate decimal part (toward 0)
    println!("fract(x) = {}", x.fract());   // Fractional part (x - trunc(x))

    println!("\n--- Exponential and Log ---");
    println!("exp(x) = {}", x.exp());       // e^x
    println!("exp2(x) = {}", x.exp2());     // 2^x
    println!("ln(x) = {}", x.ln());         // Natural logarithm (base e)
    println!("log10(x) = {}", x.log10());   // Base-10 logarithm
    println!("log2(x) = {}", x.log2());     // Base-2 logarithm
    println!("log(x, y) = {}", x.log(y));   // Logarithm base y

    println!("\n--- Powers and Roots ---");
    println!("x.powf(3.0) = {}", x.powf(3.0)); // x raised to any floating-point power
    println!("sqrt(x) = {}", x.sqrt());        // Square root
    println!("cbrt(x) = {}", x.cbrt());        // Cube root
    println!("recip(x) = {}", x.recip());      // Reciprocal (1 / x)

    println!("\n--- Trigonometry ---");
    println!("sin(PI/2) = {}", (PI / 2.0).sin());   // Sine (radians)
    println!("cos(PI) = {}", PI.cos());             // Cosine
    println!("tan(PI/4) = {}", (PI / 4.0).tan());   // Tangent

    println!("asin(1.0) = {}", 1.0f64.asin());      // Inverse sine
    println!("acos(0.0) = {}", 0.0f64.acos());      // Inverse cosine
    println!("atan(1.0) = {}", 1.0f64.atan());      // Inverse tangent
    println!("atan2(y, x) = {}", y.atan2(x));       // Arctangent of y/x, returns angle in correct quadrant

    println!("\n--- Hyperbolic Functions ---");
    println!("sinh(x) = {}", x.sinh());         // Hyperbolic sine
    println!("cosh(x) = {}", x.cosh());         // Hyperbolic cosine
    println!("tanh(x) = {}", x.tanh());         // Hyperbolic tangent

    println!("asinh(x) = {}", x.asinh());       // Inverse hyperbolic sine
    println!("acosh(10.0) = {}", 10.0f64.acosh()); // Inverse hyperbolic cosine (must be ≥ 1)
    println!("atanh(0.5) = {}", 0.5f64.atanh());   // Inverse hyperbolic tangent (|x| < 1)

    println!("\n--- Sign and Utility ---");
    println!("signum(x) = {}", x.signum());             // 1.0 if positive, -1.0 if negative
    println!("is_sign_positive: {}", x.is_sign_positive()); // true if x is positive (including +0.0)
    println!("is_sign_negative: {}", x.is_sign_negative()); // true if x is negative (including -0.0)

    println!("min(x, y) = {}", x.min(y));               // Minimum of x and y
    println!("max(x, y) = {}", x.max(y));               // Maximum of x and y
    println!("clamp(x, 1.0, 3.0) = {}", x.clamp(1.0, 3.0)); // Restrict x to a range

    println!("is_nan = {}", x.is_nan());                // Check if x is Not a Number
    println!("is_infinite = {}", x.is_infinite());      // true if x is ±∞
    println!("is_finite = {}", x.is_finite());          // true if x is not NaN or infinite
    println!("is_normal = {}", x.is_normal());          // true if x is not subnormal or zero

    println!("\n--- Integer Math ---");
    let a: i32 = -42;
    let b: i32 = 17;

    println!("abs(a) = {}", a.abs());                       // Absolute value
    println!("a.checked_add(b) = {:?}", a.checked_add(b));  // Safe addition (returns Option)
    println!("a.saturating_sub(b) = {}", a.saturating_sub(b)); // Subtract with saturation (no overflow)
    println!("a.wrapping_mul(b) = {}", a.wrapping_mul(b));  // Multiply with wrapping on overflow
    println!("a.overflowing_div(b) = {:?}", a.overflowing_div(b)); // Division with overflow flag

    println!("a.count_ones() = {}", a.count_ones());        // Count number of 1s in binary
    println!("a.leading_zeros() = {}", a.leading_zeros());  // Number of leading zeros in binary
    println!("a.trailing_zeros() = {}", a.trailing_zeros()); // Number of trailing zeros in binary
}
