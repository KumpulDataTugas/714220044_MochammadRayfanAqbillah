struct Address {
    city: String,
    zip: u32,
}

struct User {
    username: String,
    address: Address,
}

fn main() {
    let addr = Address {
        city: String::from("Jakarta"),
        zip: 12345,
    };

    let user = User {
        username: String::from("rayfan"),
        address: addr,
    };

    println!("{} lives in {}, {}", user.username, user.address.city, user.address.zip);
}
