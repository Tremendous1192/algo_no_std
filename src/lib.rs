#![cfg_attr(not(feature = "std"), no_std)]

#[macro_export]
/// あ: 値の交換 - exchange of values
macro_rules! swap {
    (mut $a:expr, mut $b:expr) => {
        let temp = $a;
        $a = $b;
        $b = temp;
    };
}

/// あ: 誤り検出符号 - error detecting code
pub trait Luhn {
    /// クレジットカード番号の確認アルゴリズム
    fn luhn(&self) -> bool;
}

/// クレジットカード番号の桁数から u64 と u128 に実装する
impl Luhn for u128 {
    fn luhn(&self) -> bool {
        luhn_str(itoa::Buffer::new().format(*self))
    }
}

/// クレジットカード番号の桁数から u64 と u128 に実装する
impl Luhn for u64 {
    fn luhn(&self) -> bool {
        luhn_str(itoa::Buffer::new().format(*self))
    }
}

/// 共通処理
fn luhn_str(string: &str) -> bool {
    let mut w = 1;
    let mut t = 0;

    for c in string.chars().rev() {
        let digit = c.to_digit(10).unwrap();
        let mut d = w * digit;
        if d > 9 {
            d -= 9;
        }
        t += d;
        w = 3 - w;
    }

    t - (t / 10) * 10 == 0
}
// ここまであ 誤り検出符号 - error detecting code

/// あ: 暗号 - cryptosystem
/// 128文字以下の文字列スライスを暗号化する
pub fn crypt(sentence: &str, key: u32) -> heapless::String<128_usize> {
    let mut result = heapless::String::<128_usize>::new();

    // 1文字毎に暗号化する
    for c in sentence.chars() {
        // 数値化
        let c_num: u32 = c as u32;
        // 暗号化
        let c_num_convert: u32 = c_num ^ key;
        // 文字列化
        let c_convert: char = char::from_u32(c_num_convert).unwrap();
        // 文字列へ追加
        let _ = result.push(c_convert);
    }

    result
}

/// え: エジプトの分数 - Egyptian fractions
/// 任意の分数を分子が1の分数の和で表現する
pub fn egyptian_fractions(numerator: usize, denominator: usize) -> heapless::String<256_usize> {
    // 初期化
    let mut result = heapless::String::<256_usize>::new();
    let mut m: usize = numerator;
    let mut n: usize = denominator;
    let mut q: usize;

    let _ = result.push_str(itoa::Buffer::new().format(m));
    let _ = result.push_str("/");
    let _ = result.push_str(itoa::Buffer::new().format(n));
    let _ = result.push_str(" = ");

    // 反復試行
    while n % m != 0_usize {
        q = n / m + 1;
        let _ = result.push_str("1/");
        let _ = result.push_str(itoa::Buffer::new().format(q));
        let _ = result.push_str(" + ");
        m = m * q - n;
        n *= q;
    }
    let _ = result.push_str("1/");
    let _ = result.push_str(itoa::Buffer::new().format(n / m));

    result
}

/// き: 基数の変換 - radix conversion
pub fn radix_conversion(
    d_1: usize,
    x_1: heapless::Vec<usize, 16>,
    d_2: usize,
) -> (heapless::String<128_usize>, heapless::String<128_usize>) {
    // 初期化
    let mut result_1 = heapless::String::<128_usize>::new();
    let mut result_2 = heapless::String::<128_usize>::new();

    // 10進数に変換
    let mut x_1_10: usize = 0_usize;
    for i in 0_usize..x_1.len() {
        x_1_10 += x_1[i] * d_1.pow(i as u32);
    }

    // 元の進数を文字列に変換する
    let _ = result_1.push_str("(");
    let _ = result_1.push_str(itoa::Buffer::new().format(x_1_10));
    let _ = result_1.push_str(")_{");
    let _ = result_1.push_str(itoa::Buffer::new().format(d_1));
    let _ = result_1.push_str("} = ");
    for i in 0_usize..x_1.len() {
        let _ = result_1.push_str(" +");
        let _ = result_1.push_str(itoa::Buffer::new().format(x_1[i]));
        let _ = result_1.push_str("x");
        let _ = result_1.push_str(itoa::Buffer::new().format(d_1));
        let _ = result_1.push_str("^");
        let _ = result_1.push_str(itoa::Buffer::new().format(i));
    }

    // 変換後の係数
    let mut x_2: heapless::Vec<usize, 16> = heapless::Vec::<usize, 16>::new();
    let mut max: usize = 1_usize;
    while max <= x_1_10 {
        max *= d_2;
        let _ = x_2.push(0_usize);
    }

    // 変換
    for i in (0_usize..x_2.len()).rev() {
        let pow = d_2.pow(i as u32);
        x_2[i] = x_1_10 % pow;
        x_1_10 -= x_1_10 / pow;
    }

    // 変換後の進数を文字列に変換する
    let _ = result_2.push_str("(");
    let _ = result_2.push_str(itoa::Buffer::new().format(x_1_10));
    let _ = result_2.push_str(")_{");
    let _ = result_2.push_str(itoa::Buffer::new().format(d_2));
    let _ = result_2.push_str("} = ");
    for i in 0_usize..x_2.len() {
        let _ = result_2.push_str(" +");
        let _ = result_2.push_str(itoa::Buffer::new().format(x_2[i]));
        let _ = result_2.push_str("x");
        let _ = result_2.push_str(itoa::Buffer::new().format(d_2));
        let _ = result_2.push_str("^");
        let _ = result_2.push_str(itoa::Buffer::new().format(i));
    }

    (result_1, result_2)
}

