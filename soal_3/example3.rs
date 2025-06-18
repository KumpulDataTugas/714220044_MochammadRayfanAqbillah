struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(w: u32, h: u32) -> Self {
        Self { width: w, height: h }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let r = Rectangle::new(10, 20);
    println!("Area: {}", r.area());
}
