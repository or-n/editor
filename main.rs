mod editor;
mod term;
mod term_from_text;
mod util;

use term::*;
use term_from_text::{settings::*, token::Token};
use util::text::EatMany;
use util::token::Eat;

use std::collections::HashMap;

fn main() {
    let source = include_str!("examples/iflet");
    let (_, tokens) = Token::eat_many(source, ());
    println!("{:?}", tokens);
    let (rest, term) = BTerm::eat(&tokens, Settings::all(true)).unwrap();
    println!("{:?}\n{:?}", rest, term);
    editor::Model {
        input: String::new(),
        terminate: false,
        w: std::io::stdout(),
    }
    .run(&term)
    .unwrap();

    let mut context = HashMap::new();
    println!("{:?}", term.clone().run(&mut context));

    let mut zipper = editor::zipper::Zipper {
        node: term,
        went: vec![],
    };
    zipper.down(1).unwrap();
    println!("{:?}", zipper);
    zipper.up().unwrap();
    println!("{:?}", zipper);
}
