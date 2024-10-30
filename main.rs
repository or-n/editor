mod digit_text;
mod term;
mod term_text;
mod term_token;
mod text;
mod token;

use term::*;
use term_text::settings::*;
use text::EatMany;
use token::Eat;

use term_text::token::Token;

use std::collections::HashMap;

fn main() {
    let source = include_str!("examples/blep");
    let (_, tokens) = Token::eat_many(source, ());
    println!("{:?}", tokens);
    let (rest, term) = BTerm::eat(&tokens, Settings::all(true)).unwrap();
    println!("{:?} {:?}", rest, term);
    let mut context = HashMap::new();
    context.insert("x".to_string(), integer(i(1)));
    println!("{:?}", term.run(&mut context));
}
