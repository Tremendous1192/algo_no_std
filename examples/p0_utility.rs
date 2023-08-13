/// 入力値が引数の範囲内であれば入力値を返す。
/// 範囲外等の場合入力をやり直す。
/// 参考: 関数などの引数に range を受け取って普通にイテレーションする場合
/// https://qiita.com/hystcs/items/8e064f8b9a79adb9cca7
pub fn input_usize<R: std::ops::RangeBounds<usize> + std::iter::IntoIterator<Item = usize>>(
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
