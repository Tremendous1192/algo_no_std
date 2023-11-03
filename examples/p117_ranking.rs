/// し: 順位付け - ranking
/// 降順で順位付けをする
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

    // 順位
    let rank = algo_no_std::ranking(&a);

    println!("ソート前: {:?}\n    順位: {:?}", a, rank);
}
