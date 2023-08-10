use algo_no_std::crypt;
fn main() {
    // 元の文章
    let original: &str = "Hello, world";
    // 暗号キー
    let key: u32 = 12_u32;
    // 暗号化した文章
    let convertedd: &str = &crypt(&original, key);
    // 復号化した文章
    let reconverted: &str = &crypt(&convertedd, key);
    // 確認
    println!("{}\n{}\n{}", original, convertedd, reconverted);
}
