/// え: エジプトの分数 - Egyptian fractions
/// 任意の分数を分子が1の分数の和で表現する
fn main() {
    let range_m = 0_usize..=100;
    println!("分子を入力してください。範囲 {:?}: ", range_m);
    let mut m = input_number(range_m);

    let range_n = 1_usize..=100;
    println!("分母を入力してください。範囲 {:?}: ", range_n);
    let mut n = input_number(range_n);

    let mut q: usize = 0_usize;

    println!("{}/{} = ", m, n);

    while n % m != 0_usize {
        q = n / m + 1;
        print!("1/{} + ", q);
        m = m * q - n;
        n *= q;
    }
    println!("1/{}", n / m);
}

// 入力値が引数の範囲内であれば入力値を返す関数。範囲外等の場合入力をやり直す。
fn input_number<R: std::ops::RangeBounds<usize> + std::iter::IntoIterator<Item = usize>>(
    range: R,
) -> usize {
    let mut input = String::new();
    loop {
        // 入力値の読み取り
        match std::io::stdin().read_line(&mut input) {
            // 読み取りの確認
            Ok(_) => {
                match input.trim().parse() {
                    // 数値化の確認
                    Ok(number) => {
                        // 入力値が企画内にあるかの確認
                        if range.contains(&number) {
                            return number;
                        } else {
                            println!("入力値が範囲外です。もう一度入力してください。");
                            input = String::new();
                        }
                    }
                    Err(_) => {
                        println!("数値化に失敗しました。もう一度入力してください。");
                        input = String::new();
                    }
                }
            }
            Err(_) => {
                println!("読み取りに失敗しました。もう一度入力してください。");
                input = String::new();
            }
        }
    }
}
