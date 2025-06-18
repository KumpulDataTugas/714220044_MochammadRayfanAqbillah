// This code will not compile
/*
fn dangle() -> &String {
    let s = String::from("hello");
    &s // Error: s will be dropped
}
*/

fn main() {
    println!("Dangling references are not allowed in Rust.");
}
