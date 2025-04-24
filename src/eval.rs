use crate::term::*;
use crate::term_new::*;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Error {
    Id(String),
    CallIfValuePair,
    CallIfValuePairFirstI,
    CallIfBranchMissing,
    CallIfBranchPair,
    CallWrong,
}

type Ctx = HashMap<String, T>;

pub fn eval(ctx: &mut Ctx, t: T) -> Result<T, Error> {
    use T::*;
    match t {
        Id(x) => ctx.get(&x).map(|x| x.clone()).ok_or(Error::Id(x.clone())),
        Abstract(a, b) => Ok(Abstract(a, b)),
        Let(value, cont) => {
            let cont_ = eval(ctx, *cont)?;
            match cont_ {
                Abstract(x, cont) => {
                    ctx.insert(x, *value);
                    eval(ctx, *cont)
                }
                If(xs) => {
                    let Pair(tag, value_) = *value else {
                        return Err(Error::CallIfValuePair);
                    };
                    let I(tag_) = *tag else {
                        return Err(Error::CallIfValuePairFirstI);
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
                        return Err(Error::CallIfBranchMissing);
                    };
                    let Pair(_, cont) = branch else {
                        return Err(Error::CallIfBranchPair);
                    };
                    eval(ctx, Let(value_, cont))
                }
                _ => Err(Error::CallWrong),
            }
        }
        I(x) => Ok(I(x)),
        Pair(a, b) => {
            let a_ = eval(ctx, *a)?;
            let b_ = eval(ctx, *b)?;
            Ok(pair(a_, b_))
        }
        If(xs) => {
            let iter = xs.into_iter().map(|x| eval(ctx, x));
            let xs_: Result<_, _> = iter.collect();
            Ok(If(xs_?))
        }
    }
}
