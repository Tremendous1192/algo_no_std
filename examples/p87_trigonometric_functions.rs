mod p0_utility;
use crate::p0_utility::input_usize;

/// さ: 三角関数 - p87_trigonometric functions
/// し: 指数関数 - p108 exponential function
fn main() {
    println!("三角関数等の計算は micromath crate を使用します");
    let mut x: f32 = core::f32::consts::PI * 0.5_f32;
    println!("\nx = π/2 = {}", x);
    println!("sin(x) = {}", x.sin());
    println!("cos(x) = {}", x.cos());
    x = core::f32::consts::PI * 0.25_f32;
    println!("\nx = π/4 = {}", x);
    println!("tan(x) = {}", x.tan());

    println!("\nexp(x) = {}", x.exp());
    println!("ln(x) = {}", x.ln());
}
