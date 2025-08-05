use crate::lib::{expand, shorten};

pub fn run(args: Vec<String>) {
    if args.len() < 3 {
        eprintln!("Usage: cargo run -- [shorten|expand] <value>");
        return;
    }

    match args[1].as_str() {
        "shorten" => {
            let url = &args[2];
            let short = shorten(url);
            println!("Shortened -> {}", short);
        }
        "expand" => {
            let code = &args[2];
            match expand(code) {
                Some(url) => println!("Original -> {}", url),
                None => println!("Not found"),
            }
        }
        _ => eprintln!("Unknown command"),
    }
}
