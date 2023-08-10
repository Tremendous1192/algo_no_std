use algo_no_std::swap;

fn main() {
    // u8
    println!("u8");
    let mut a = 1_u8;
    let mut b = 2_u8;
    println!("a = {}, b = {}", a, b);
    swap!(mut a, mut b);
    println!("a = {}, b = {}", a, b);

    // i8
    println!("\ni8");
    let mut a = -1_i8;
    let mut b = 2_i8;
    println!("a = {}, b = {}", a, b);
    swap!(mut a, mut b);
    println!("a = {}, b = {}", a, b);

    // f32
    println!("\nf32");
    let mut a = -1_f32;
    let mut b = 2_f32;
    println!("a = {}, b = {}", a, b);
    swap!(mut a, mut b);
    println!("a = {}, b = {}", a, b);

    // char
    println!("\nchar");
    let mut a = 'α';
    let mut b = 'β';
    println!("a = {}, b = {}", a, b);
    swap!(mut a, mut b);
    println!("a = {}, b = {}", a, b);

    // bool
    println!("\nbool");
    let mut a = true;
    let mut b = false;
    println!("a = {}, b = {}", a, b);
    swap!(mut a, mut b);
    println!("a = {}, b = {}", a, b);
}
