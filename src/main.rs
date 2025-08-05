mod api;
mod cli;
mod lib;

use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "serve" {
        api::run().await;
    } else {
        cli::run(args);
    }
}
