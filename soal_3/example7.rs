struct Counter {
    value: i32,
}

impl Counter {
    fn increment(&mut self) {
        self.value += 1;
    }
}

fn main() {
    let mut c = Counter { value: 0 };
    c.increment();
    println!("Counter: {}", c.value);
}
