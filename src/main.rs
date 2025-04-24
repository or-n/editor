mod eat_term;
mod eval;
mod term;
mod util;
use std::collections::HashMap;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let data = eat_term::EatData { parens: false };
    let r = eat_term::eat(input.as_str(), data);
    println!("{:?}", r);
    let Ok((_, t)) = r else {
        return;
    };
    let mut ctx = HashMap::new();
    println!("{:?}", eval::eval(&mut ctx, t));
}
