mod digit_text;
mod term;
mod term_text;
mod term_token;
mod text;
mod token;

use term::*;
// use term_text::*;
// use term_token::*;
use text::EatMany;
use token::Eat;

use term_text::token::Token;

use std::collections::HashMap;

fn main() {
    let source = include_str!("examples/blep");
    let (_, tokens) = Token::eat_many(source, ());
    println!("{:?}", tokens);
    // let (_, term) = BTerm::eat(source, Settings::default()).unwrap();
    let (_, term) = BTerm::eat(&tokens, ()).unwrap();
    let mut context = HashMap::new();
    context.insert("x".to_string(), integer(i(1)));
    println!("{:?}", term.run(&mut context));
}
