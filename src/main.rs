mod editor;
mod eval;
mod term;
mod term_new;

use term_new::*;

fn main() {
    let t = id("x".to_string());
    let mut ctx = eval::Ctx::new();
    let mut stdout = std::io::stdout();
    let mut m = editor::Model {
        input: "".to_string(),
        output: "".to_string(),
        mode: editor::Mode::Migrate,
        command: None,
    };
    let r = m.run(&mut stdout);
    println!("{:?}", r);
    // ctx.insert("x".to_string(), i(69));
    // let eval_t = eval::eval(&mut ctx, t);
    // println!("{:?}", eval_t);
}
