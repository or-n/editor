mod digit_text;
mod term;
mod term_text;
mod term_token;
mod text;
mod token;

use term::*;
use term_text::*;
use text::*;

use std::collections::HashMap;

fn main() {
    let source = include_str!("examples/blep");
    let tokens = term_text::token::Token::eat_many(source, ());
    println!("{:?}", tokens);
    let (_, term) = BTerm::eat(source, Settings::default()).unwrap();
    let mut context = HashMap::new();
    context.insert("x".to_string(), integer(i(1)));
    println!("{:?}", term.run(&mut context));
}
