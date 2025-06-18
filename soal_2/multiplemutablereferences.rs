fn main() {
    let s = String::from("Rust");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2); // allowed: multiple immutable references
}
