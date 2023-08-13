mod p0_utility;
use crate::p0_utility::input_usize;

/// い: 石取りゲーム 1
fn main() {
    let mut input = String::new();
    println!(
        "ゲームで使用する石の数{:?}を入力して下さい: ",
        (2_usize..=10)
    );
    let mut quantity: usize = input_usize(2_usize..=10);

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
        get = input_usize(1_usize..=can_get);
        quantity -= get;
        println!("{}個の石を取って、残りは{}個です", get, quantity);

        if quantity == 0_usize {
            println!("\n\nあなたの勝利です");
            break;
        }
    }
}

// デバッグ用の型名表示関数
// https://qiita.com/garkimasera/items/729dbf11dd5738c70e11
fn print_typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}
