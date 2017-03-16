fn main() {
    let x: i32 = std::i32::MAX;
    let y: i64 = std::i64::MAX;
    let z:isize = std::isize::MAX;
    let another_size: u32 = std::u32::MAX;
    let yani: u64 = std::u64::MAX;

    let my_unknown: usize = std::usize::MAX;

    println!("{}:?", x);
    println!("{}:?", y);
    println!("{}:?", z);
    println!("{}:?", another_size);
    println!("{}:?", yani);
    println!("{}:?", my_unknown);


}
