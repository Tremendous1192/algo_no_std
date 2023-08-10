/// い: 石取りゲーム 1
fn main() {
    let mut input = String::new();
    println!(
        "ゲームで使用する石の数{:?}を入力して下さい: ",
        (2_usize..=10)
    );
    let mut quantity: usize = input_number(2_usize..=10);

    let max: usize = 3;
    println!("\n一度に取ることができる石の数です: {:?}", (1_usize..=max));
    let get_tange = (1_usize..=max);

    loop {
        // CPU
        println!("\n残りの石数: {}\nCPUの手番です", quantity);
        let mut get = if quantity <= max {
            quantity
        } else {
            std::cmp::max((quantity - 1) % (max + 1), 1_usize)
        };
        quantity -= get;
        println!("CPUが{}個の石を取って、残りは{}個です", get, quantity);

        if quantity == 0_usize {
            println!("\n\nCPUの勝利です");
            break;
        }

        // あなた
        let can_get: usize = if quantity <= max { quantity } else { max };
        println!("\n残りの石数: {}\nあなたの手番です", quantity);
        println!(
            "\n{:?}個の範囲で取り出す石の数を指定してください",
            (1_usize..=can_get)
        );
        get = input_number(1_usize..=can_get);
        quantity -= get;
        println!("{}個の石を取って、残りは{}個です", get, quantity);

        if quantity == 0_usize {
            println!("\n\nあなたの勝利です");
            break;
        }
    }
}

use std::io; // 入力が必要なのでstdが必要

// デバッグ用の型名表示関数
// https://qiita.com/garkimasera/items/729dbf11dd5738c70e11
fn print_typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

// 関数などの引数に range を受け取って普通にイテレーションするときの参考
// https://qiita.com/hystcs/items/8e064f8b9a79adb9cca7
use std::iter::IntoIterator;
use std::ops::RangeBounds;
// 入力値が引数の範囲内であれば入力値を返す関数。範囲外等の場合入力をやり直す。
fn input_number<R: RangeBounds<usize> + IntoIterator<Item = usize>>(range: R) -> usize {
    let mut input = String::new();
    loop {
        // 入力値の読み取り
        match io::stdin().read_line(&mut input) {
            Ok(read) => {
                // read - usize: 読み取った値をそのまま使用することはできないので数値化処理が必要
                //println!("\n入力値: {}", read);
                //print_typename(read);
                match input.trim().parse() {
                    Ok(number) => {
                        // number - usize: 数値化処理した値
                        //println!("\n数値化: {}", number);
                        //print_typename(number);

                        //println!("\n使用する石の数: {:?}", (1_usize..=10));
                        //print_typename((1_usize..=10));
                        if (1_usize..11).contains(&number) {
                            // (1_usize..=10) - core::ops::range::Range<usize>
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
