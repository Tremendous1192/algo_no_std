/// き: 基数の変換 - radix conversion
/// 今回の例で60文字のため、10進数で約100までの値に使用できると考えられる
fn main() {
    // 元の進数
    let d_1: usize = 3_usize;
    let x_1: heapless::Vec<usize, 16> =
        heapless::Vec::<usize, 16>::from_slice(&[0_usize, 1, 1, 0, 1]).unwrap();

    // 変換後の進数
    let d_2: usize = 2_usize;

    let (previous, converted) = algo_no_std::radix_conversion(d_1, &x_1, d_2);

    // 変換前後の進数の表示
    println!("{}\n{}", previous, converted);
}
