struct Marker;

impl Marker {
    fn print_message() {
        println!("This is a unit struct.");
    }
}

fn main() {
    Marker::print_message();
}
