/// き: 逆写像ソート - inverse mapping sort
/// ベクトルを昇順に並び替える
fn main() {
    // 初期化
    // 元の配列
    let mut a: heapless::Vec<i8, 256_usize> = heapless::Vec::<i8, 256_usize>::new();
    let _ = a.push(-1_i8);
    let _ = a.push(0_i8);
    let _ = a.push(3_i8);
    let _ = a.push(-1_i8);
    let _ = a.push(5_i8);
    let _ = a.push(-2_i8);
    let _ = a.push(5_i8);

    let b = algo_no_std::inverse_mapping_sort(&a);

    println!("ソート前: {:?}\nソート後: {:?}", a, b);
}
