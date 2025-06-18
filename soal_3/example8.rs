struct Config {
    debug: Option<bool>,
}

fn print_debug(config: &Config) {
    match config.debug {
        Some(true) => println!("Debugging enabled."),
        Some(false) => println!("Debugging disabled."),
        None => println!("No debug info."),
    }
}

fn main() {
    let cfg = Config { debug: Some(true) };
    print_debug(&cfg);
}
