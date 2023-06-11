mod cli;
mod scraper;

use console::style;
use std::process::exit;

fn main() {
    if let Err(err) = cli::run() {
        println!("{} {}", style("error:").bold().red(), err.to_string());
        exit(1);
    }
}
