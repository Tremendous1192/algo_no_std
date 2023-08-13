mod p0_utility;
use crate::p0_utility::input_usize;

/// く: 組み合わせの数 - number of combination
fn main() {
    let range_n = 1_usize..=16;
    println!("総データ数を入力してください。 範囲: [{:?}]", range_n);
    let n = input_usize(range_n);

    // 全ての組み合わせの数を計算する
    for k in 0_usize..=n {
        println!("{}_C_{} = {}", n, k, algo_no_std::p60_combination(n, k));
    }
}
