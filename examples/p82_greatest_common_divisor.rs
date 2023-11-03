mod p0_utility;
use crate::p0_utility::input_usize;

/// さ: 最大公約数 - greatest common divisor
/// 2つの自然数の最大公約数を計算します。
fn main() {
    let range_n = 1_usize..=50;
    println!("1番目の自然数を入力してください。 範囲: [{:?}]", range_n);
    let x = input_usize(range_n.clone());
    println!("2番目の自然数を入力してください。 範囲: [{:?}]", range_n);
    let y = input_usize(range_n);

    println!(
        "2つの自然数 ({}, {}) の最大公約数は {}",
        x,
        y,
        algo_no_std::greatest_common_divisor(x, y)
    );
}
