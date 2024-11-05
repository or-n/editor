use super::{zipper::Node, SyntaxItem};
use enum_as_inner::EnumAsInner;

use crate::term::{BTerm, Integer as Int};
use std::convert::{Into, TryFrom};

pub type Name = String;

#[derive(Clone, Debug, EnumAsInner)]
pub enum Kid {
    Basic(Option<BTerm>),
    Name(Name),
    I(isize),
}

fn basic(t: BTerm) -> Kid {
    Kid::Basic(Some(t))
}

impl Into<Node<SyntaxItem, Kid>> for BTerm {
    fn into(self) -> Node<SyntaxItem, Kid> {
        use crate::term::Term::*;
        match *self {
            Parameter(n) => Node {
                item: SyntaxItem::Parameter,
                kids: vec![Kid::Name(n)],
            },
            Let(n, a, b) => Node {
                item: SyntaxItem::Let,
                kids: vec![Kid::Name(n), basic(a), basic(b)],
            },
            Integer(i) => match i {
                Int::I(n) => Node {
                    item: SyntaxItem::I,
                    kids: vec![Kid::I(n)],
                },
                Int::Add => Node {
                    item: SyntaxItem::Add,
                    kids: vec![],
                },
                Int::Mul => Node {
                    item: SyntaxItem::Mul,
                    kids: vec![],
                },
            },
            If(a, branches) => {
                let mut kids = vec![basic(a)];
                for (n, branch) in branches {
                    kids.push(Kid::I(n));
                    kids.push(Kid::Name(branch.name));
                    kids.push(basic(branch.block));
                }
                Node {
                    item: SyntaxItem::If,
                    kids,
                }
            }
            IfLet(a, branches, (default_n, default_branch)) => {
                let mut kids = vec![
                    Kid::I(default_n),
                    Kid::Name(default_branch.name),
                    basic(a),
                ];
                for (n, branch) in branches {
                    kids.push(Kid::I(n));
                    kids.push(Kid::Name(branch.name));
                    kids.push(basic(branch.block));
                }
                kids.push(basic(default_branch.block));
                Node {
                    item: SyntaxItem::IfLet,
                    kids,
                }
            }
            Apply(a, b) => Node {
                item: SyntaxItem::Apply,
                kids: vec![basic(a), basic(b)],
            },
            InfixL(a, f, b) => Node {
                item: SyntaxItem::InfixL,
                kids: vec![basic(a), basic(f), basic(b)],
            },
            InfixR(a, f, b) => Node {
                item: SyntaxItem::InfixR,
                kids: vec![basic(a), basic(f), basic(b)],
            },
            Pair(a, b) => Node {
                item: SyntaxItem::Pair,
                kids: vec![basic(a), basic(b)],
            },
        }
    }
}

impl TryInto<Node<SyntaxItem, Kid>> for Kid {
    type Error = ();

    fn try_into(self) -> Result<Node<SyntaxItem, Kid>, Self::Error> {
        match self {
            Kid::Basic(basic) => {
                let t = basic.ok_or(())?;
                Ok(t.into())
            }
            Kid::Name(_) => Err(()),
            Kid::I(_) => Err(()),
        }
    }
}

fn as_basic(t: &Kid) -> Option<BTerm> {
    t.as_basic()?.clone()
}

impl TryFrom<Node<SyntaxItem, Kid>> for BTerm {
    type Error = ();

    fn try_from(value: Node<SyntaxItem, Kid>) -> Result<Self, Self::Error> {
        use crate::term as T;
        use SyntaxItem::*;
        match (value.item, &value.kids[..]) {
            (Parameter, [n]) => {
                let n = n.as_name().ok_or(())?.to_string();
                Ok(T::parameter(n))
            }
            (I, [n]) => {
                let n = *n.as_i().ok_or(())?;
                Ok(T::integer(T::Integer::I(n)))
            }
            (Add, []) => Ok(T::integer(T::Integer::Add)),
            (Mul, []) => Ok(T::integer(T::Integer::Mul)),
            (If, [a, rest @ ..]) => {
                let a = as_basic(a).ok_or(())?;
                let mut branches = Vec::new();
                for chunk in rest.chunks(3) {
                    match chunk {
                        [tag, n, block] => {
                            let tag = *tag.as_i().ok_or(())?;
                            let name = n.as_name().ok_or(())?.to_string();
                            let block = as_basic(block).ok_or(())?;
                            branches.push((tag, T::Branch { name, block }));
                        }
                        [] => (),
                        _ => return Err(()),
                    }
                }
                Ok(T::r#if(a, branches))
            }
            (IfLet, [default_tag, default_name, a, rest @ ..]) => {
                let default_tag = *default_tag.as_i().ok_or(())?;
                let a = a.as_basic().ok_or(())?.clone().ok_or(())?;
                let mut branches = Vec::new();
                let mut process = || {
                    for chunk in rest.chunks(3) {
                        match chunk {
                            [tag, n, block] => {
                                let tag = *tag.as_i().ok_or(())?;
                                let name = n.as_name().ok_or(())?.to_string();
                                let block = as_basic(block).ok_or(())?;
                                branches.push((tag, T::Branch { name, block }));
                            }
                            [block] => return Ok(as_basic(block).ok_or(())?),
                            _ => return Err(()),
                        }
                    }
                    unreachable!()
                };
                let default_branch = T::Branch {
                    name: default_name.as_name().ok_or(())?.to_string(),
                    block: process()?,
                };
                Ok(T::r#iflet(a, branches, (default_tag, default_branch)))
            }
            (Let, [n, a, b]) => {
                let n = n.as_name().ok_or(())?.to_string();
                let a = as_basic(a).ok_or(())?;
                let b = as_basic(b).ok_or(())?;
                Ok(T::r#let(n, a, b))
            }
            (Apply, [a, b]) => {
                let a = as_basic(a).ok_or(())?;
                let b = as_basic(b).ok_or(())?;
                Ok(T::apply(a, b))
            }
            (InfixL, [a, f, b]) => {
                let a = as_basic(a).ok_or(())?;
                let f = as_basic(f).ok_or(())?;
                let b = as_basic(b).ok_or(())?;
                Ok(T::infixl(a, f, b))
            }
            (InfixR, [a, f, b]) => {
                let a = as_basic(a).ok_or(())?;
                let f = as_basic(f).ok_or(())?;
                let b = as_basic(b).ok_or(())?;
                Ok(T::infixr(a, f, b))
            }
            (Pair, [a, b]) => {
                let a = as_basic(a).ok_or(())?;
                let b = as_basic(b).ok_or(())?;
                Ok(T::pair(a, b))
            }
            _ => Err(()),
        }
    }
}

impl TryFrom<Node<SyntaxItem, Kid>> for Kid {
    type Error = ();

    fn try_from(value: Node<SyntaxItem, Kid>) -> Result<Self, Self::Error> {
        let t = BTerm::try_from(value)?;
        Ok(basic(t))
    }
}
