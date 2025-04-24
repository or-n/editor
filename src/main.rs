mod eval;
mod term;
use std::collections::HashMap;

fn main() {
    let t = term::i(0);
    let mut ctx = HashMap::new();
    println!("{:?}", eval::eval(&mut ctx, t));
}
