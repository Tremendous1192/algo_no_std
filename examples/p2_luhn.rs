use algo_no_std::Luhn;
fn main() {
    // ECサイトの動作テストに使える、クレジットカードのテスト番号一覧 | Webクリエイターボックス
    // https://www.webcreatorbox.com/tech/creditcard-test-numbers

    println!("Master Cardの番号を検査する");
    let mut value = 5_555_555_555_554_444_u128;
    println!("{} -> {}", value, value.luhn());
    value = 5_105_105_105_105_100_u128;
    println!("{} -> {}", value, value.luhn());

    println!("\nVisaの番号を検査する");
    value = 4_111_111_111_111_111_u128;
    println!("{} -> {}", value, value.luhn());
    value = 4_012_888_888_881_881_u128;
    println!("{} -> {}", value, value.luhn());

    println!("\nJCBの番号を検査する");
    value = 3_530_111_333_300_000_u128;
    println!("{} -> {}", value, value.luhn());
    value = 3_566_002_020_360_505_u128;
    println!("{} -> {}", value, value.luhn());

    println!("\nDiners Clubの番号を検査する");
    value = 30_569_309_025_904_u128;
    println!("{} -> {}", value, value.luhn());
    value = 38_520_000_023_237_u128;
    println!("{} -> {}", value, value.luhn());

    println!("\nAmerican Expressの番号を検査する");
    value = 378_282_246_310_005_u128;
    println!("{} -> {}", value, value.luhn());
    value = 371_449_635_398_431_u128;
    println!("{} -> {}", value, value.luhn());
    value = 378_734_493_671_000_u128;
    println!("{} -> {}", value, value.luhn());

    println!("\nDiners Clubの番号を検査する");
    println!("u64 へのトレイトも確認する");
    let mut value_64 = 6_011_111_111_111_117_u64;
    println!("{} -> {}", value_64, value_64.luhn());
    value_64 = 6_011_000_990_139_424_u64;
    println!("{} -> {}", value_64, value_64.luhn());
}
