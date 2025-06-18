struct Person {
    name: String,
    age: u8,
}

fn greet(person: &Person) {
    println!("Hello, my name is {} and I am {} years old.", person.name, person.age);
}

fn main() {
    let p = Person {
        name: String::from("Alice"),
        age: 25,
    };
    greet(&p);
}
