use crate::term::*;
use std::collections::HashMap;

pub fn eval(ctx: &mut HashMap<String, T>, t: T) -> Result<T, ()> {
    use T::*;
    match t {
        Id(x) => ctx.get(&x).map(|x| x.clone()).ok_or(()),
        Abstract(a, b) => Ok(Abstract(a, b)),
        Let(a, b) => {
            let a_ = eval(ctx, *a)?;
            let b_ = eval(ctx, *b)?;
            match b_ {
                Abstract(x, cont) => {
                    ctx.insert(x, a_);
                    eval(ctx, *cont)
                }
                Array(xs) => {
                    let Pair(tag, value) = a_ else {
                        return Err(());
                    };
                    let I(tag_) = *tag else {
                        return Err(());
                    };
                    let Some(branch) = xs.into_iter().find(|x| {
                        let Pair(tag2, _) = x else {
                            return false;
                        };
                        let I(tag2_) = **tag2 else {
                            return false;
                        };
                        tag_ == tag2_
                    }) else {
                        return Err(());
                    };
                    let Pair(_, cont) = branch else {
                        return Err(());
                    };
                    eval(ctx, Let(value, cont))
                }
                _ => Err(()),
            }
        }
        I(x) => Ok(I(x)),
        Pair(a, b) => {
            let a_ = eval(ctx, *a)?;
            let b_ = eval(ctx, *b)?;
            Ok(pair(a_, b_))
        }
        Array(xs) => {
            let ys = xs.into_iter().map(|x| eval(ctx, x));
            let Ok(ys) = ys.collect() else {
                return Err(());
            };
            Ok(Array(ys))
        }
    }
}
