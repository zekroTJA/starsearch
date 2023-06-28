use console::{style, Term};
use std::io::{self, Write};

#[allow(unused_must_use)]
pub fn print_status(v: &str) {
    Term::stdout().clear_line();
    print!("{}", style(v).dim().italic());
    io::stdout().flush();
}

#[allow(unused_must_use)]
pub fn print_success(v: &str) {
    Term::stdout().clear_line();
    println!("{}", style(v).green());
}
