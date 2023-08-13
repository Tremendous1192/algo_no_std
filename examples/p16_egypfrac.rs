mod p0_utility;
use crate::p0_utility::input_usize;

/// え: エジプトの分数 - Egyptian fractions
/// 任意の分数を分子が1の分数の和で表現する
fn main() {
    let range_m = 0_usize..=50;
    println!("分子を入力してください。範囲 {:?}: ", range_m);
    let m = input_usize(range_m);

    let range_n = 1_usize..=100;
    println!("分母を入力してください。範囲 {:?}: ", range_n);
    let n = input_usize(range_n);

    // 結果
    println!("{}", algo_no_std::egyptian_fractions(m, n));
}
