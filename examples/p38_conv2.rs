/// き: 基数の変換 - radix conversion
/// 今回の例で60文字のため、100進数で約100までの値に使用できると考えられる
fn main() {
    // 元の進数
    let d_1: usize = 3_usize;
    let x_1: heapless::Vec<usize, 16> =
        heapless::Vec::<usize, 16>::from_slice(&[0_usize, 1, 1, 0, 1]).unwrap();

    // 変換後の進数
    let d_2: usize = 2_usize;

    let (previous, converted) = algo_no_std::radix_conversion(d_1, x_1, d_2);

    // 変換前後の進数の表示
    println!("{}\n{}", previous, converted);

    /*
    // 元の進数
        let d_1: usize = 3_usize;
        // 係数
        let x_1: [usize; 5_usize] = [0_usize, 1, 1, 0, 1];
        // 10進数に変換
        let mut x_1_10: usize = 0_usize;
        for i in 0_usize..x_1.len() {
            x_1_10 += x_1[i] * d_1.pow(i as u32);
        }
        print!("元の{}進数 ({})_{} =", d_1, x_1_10, d_1);
        for i in 0_usize..x_1.len() {
            print!(" +{:2}x{}^{}", x_1[i], d_1, i);
        }
        println!("");

        // 変換後の進数
        let d_2: usize = 2_usize;
        //係数
        let mut x_2: Vec<usize> = Vec::<usize>::new();
        let mut max: usize = 1_usize;
        while max <= x_1_10 {
            max *= d_2;
            x_2.push(0_usize);
        }

        // 変換
        for i in (0_usize..x_2.len()).rev() {
            let pow = d_2.pow(i as u32);
            x_2[i] = x_1_10 % pow;
            x_1_10 -= x_1_10 / pow;
        }

        print!("変換後の{}進数 =", d_2);
        for i in 0_usize..x_2.len() {
            print!(" +{:2}x{}^{}", x_2[i], d_2, i);
        }
        println!("");
    */
}
