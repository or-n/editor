mod eval;
mod term;
use std::collections::HashMap;

fn main() {
    let t = term::id("x".to_string());
    let mut ctx = HashMap::new();
    ctx.insert("x".to_string(), term::i(69));
    let eval_t = eval::eval(&mut ctx, t);
    println!("{:?}", eval_t);
}
