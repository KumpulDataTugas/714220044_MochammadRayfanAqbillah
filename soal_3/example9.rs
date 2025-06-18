struct Scores {
    name: String,
    scores: Vec<u32>,
}

fn average(s: &Scores) -> f64 {
    let total: u32 = s.scores.iter().sum();
    total as f64 / s.scores.len() as f64
}

fn main() {
    let s = Scores {
        name: String::from("Andi"),
        scores: vec![80, 90, 100],
    };
    println!("Average score of {} is {:.2}", s.name, average(&s));
}
