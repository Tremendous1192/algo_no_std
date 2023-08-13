mod p0_utility;
use crate::p0_utility::input_usize;

/// か: 階乗進法 - factorial representation
/// 非負の整数を会場進数に変換する
fn main() {
    let range = 0_usize..=5;
    println!("非負の整数を入力してください。範囲 {:?}: ", range);
    let digit = input_usize(range);
    println!(
        "0 から ({} + 1)! - 1 までの値を階乗進数で表します。\n",
        digit
    );

    // 初期化
    // 各桁の係数
    // 先頭は直感との乖離(0桁目はない)を防ぐバッファ
    // 最後尾は繰上げ処理時の無効な参照を防ぐバッファ
    let mut c = vec![0_usize; digit + 2_usize];
    // 10進数
    let mut i: usize = 0;
    // k桁目
    let mut k: usize;

    // 反復処理
    loop {
        // 変換する10進数
        print!("{:3} = ", i);

        // 各桁の係数を表示する
        for k in (1_usize..=digit).rev() {
            print!("  +{}!x{:2}", k, c[k]);
        }
        println!("");

        // 繰上げ処理
        k = 1_usize;
        while c[k] == k {
            c[k] = 0_usize;
            k += 1;
        }
        c[k] += 1;

        // 全ての係数が埋まったら終了
        if k > digit {
            break;
        }
        // 次の10進数に移る
        i += 1;
    }
}
