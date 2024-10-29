mod digit_text;
mod term;
mod term_text;
mod text;

use term::*;
use term_text::*;
use text::*;

use std::collections::HashMap;

fn main() {
    let source = include_str!("examples/blep");
    let (_, term) = BTerm::eat(source, Settings::default()).unwrap();
    let mut context = HashMap::new();
    context.insert("x".to_string(), integer(i(1)));
    println!("{:?}", term.run(&mut context));
}
