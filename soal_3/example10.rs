struct Book {
    title: String,
    pages: u32,
}

fn create_book(t: &str, p: u32) -> Book {
    Book {
        title: t.to_string(),
        pages: p,
    }
}

fn main() {
    let b = create_book("Rust Programming", 300);
    println!("Book: {}, Pages: {}", b.title, b.pages);
}
