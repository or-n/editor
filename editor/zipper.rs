use crate::term::*;

#[derive(Debug)]
pub struct Zipper {
    pub node: BTerm,
    pub went: Vec<Went>,
}

impl Zipper {
    pub fn new(node: BTerm) -> Self {
        Zipper { node, went: vec![] }
    }
}

#[derive(Debug)]
pub enum Went {
    LetA {
        name: Name,
        b: BTerm,
    },
    LetB {
        name: Name,
        a: BTerm,
    },
    PairA {
        b: BTerm,
    },
    PairB {
        a: BTerm,
    },
    ApplyA {
        b: BTerm,
    },
    ApplyB {
        a: BTerm,
    },
    InfixLA {
        f: BTerm,
        b: BTerm,
    },
    InfixLF {
        a: BTerm,
        b: BTerm,
    },
    InfixLB {
        a: BTerm,
        f: BTerm,
    },
    InfixRA {
        f: BTerm,
        b: BTerm,
    },
    InfixRF {
        a: BTerm,
        b: BTerm,
    },
    InfixRB {
        a: BTerm,
        f: BTerm,
    },
    IfValue {
        branches: Vec<Branch>,
    },
    IfBranch {
        value: BTerm,
        before: Vec<Branch>,
        after: Vec<Branch>,
        tag: isize,
        name: Name,
    },
    IfLetValue {
        branches: Vec<Branch>,
        default: Branch,
    },
    IfLetBranch {
        value: BTerm,
        before: Vec<Branch>,
        after: Vec<Branch>,
        default: Branch,
        tag: isize,
        name: Name,
    },
    IfLetDefault {
        value: BTerm,
        branches: Vec<Branch>,
        tag: isize,
        name: Name,
    },
}

impl Zipper {
    pub fn kids(&self) -> usize {
        use Term::*;
        match *self.node.clone() {
            Let(_, _, _) => 2,
            Parameter(_) => 0,
            Integer(_) => 0,
            Pair(_, _) => 2,
            Apply(_, _) => 2,
            InfixL(_, _, _) => 3,
            InfixR(_, _, _) => 3,
            If(_, branches) => branches.len() + 1,
            IfLet(_, branches, _) => branches.len() + 2,
        }
    }

    pub fn up(&mut self) -> Option<usize> {
        let last = self.went.pop()?;
        use Went::*;
        match last {
            LetA { name, b } => {
                let a = self.node.clone();
                self.node = r#let(name, a, b);
                Some(0)
            }
            LetB { name, a } => {
                let b = self.node.clone();
                self.node = r#let(name, a, b);
                Some(1)
            }
            PairA { b } => {
                let a = self.node.clone();
                self.node = pair(a, b);
                Some(0)
            }
            PairB { a } => {
                let b = self.node.clone();
                self.node = pair(a, b);
                Some(1)
            }
            ApplyA { b } => {
                let a = self.node.clone();
                self.node = apply(a, b);
                Some(0)
            }
            ApplyB { a } => {
                let b = self.node.clone();
                self.node = apply(a, b);
                Some(1)
            }
            InfixLA { b, f } => {
                let a = self.node.clone();
                self.node = infixl(a, f, b);
                Some(0)
            }
            InfixLF { a, b } => {
                let f = self.node.clone();
                self.node = infixl(a, f, b);
                Some(1)
            }
            InfixLB { a, f } => {
                let b = self.node.clone();
                self.node = infixl(a, f, b);
                Some(2)
            }
            InfixRA { b, f } => {
                let a = self.node.clone();
                self.node = infixr(a, f, b);
                Some(0)
            }
            InfixRF { a, b } => {
                let f = self.node.clone();
                self.node = infixr(a, f, b);
                Some(1)
            }
            InfixRB { a, f } => {
                let b = self.node.clone();
                self.node = infixr(a, f, b);
                Some(2)
            }
            IfValue { branches } => {
                let value = self.node.clone();
                self.node = r#if(value, branches);
                Some(0)
            }
            IfBranch {
                value,
                before,
                after,
                tag,
                name,
            } => {
                let block = self.node.clone();
                let index = before.len();
                let mut branches = before;
                branches.push(Branch { tag, name, block });
                branches.extend(after);
                self.node = r#if(value, branches);
                Some(index + 1)
            }
            IfLetValue { branches, default } => {
                let value = self.node.clone();
                self.node = iflet(value, branches, default);
                Some(0)
            }
            IfLetBranch {
                value,
                before,
                after,
                tag,
                name,
                default,
            } => {
                let block = self.node.clone();
                let index = before.len();
                let mut branches = before;
                branches.push(Branch { tag, name, block });
                branches.extend(after);
                self.node = iflet(value, branches, default);
                Some(index + 2)
            }
            IfLetDefault {
                value,
                branches,
                tag,
                name,
            } => {
                let block = self.node.clone();
                let default = Branch { tag, name, block };
                self.node = iflet(value, branches, default);
                Some(1)
            }
        }
    }

    pub fn down(&mut self, i: usize) -> Option<()> {
        use Term::*;
        match *self.node.clone() {
            Let(name, a, b) => match i {
                0 => {
                    self.went.push(Went::LetA { name, b });
                    self.node = a;
                    Some(())
                }
                1 => {
                    self.went.push(Went::LetB { name, a });
                    self.node = b;
                    Some(())
                }
                _ => None,
            },
            Parameter(_) => None,
            Integer(_) => None,
            Pair(a, b) => match i {
                0 => {
                    self.went.push(Went::PairA { b });
                    self.node = a;
                    Some(())
                }
                1 => {
                    self.went.push(Went::PairB { a });
                    self.node = b;
                    Some(())
                }
                _ => None,
            },
            Apply(a, b) => match i {
                0 => {
                    self.went.push(Went::ApplyA { b });
                    self.node = a;
                    Some(())
                }
                1 => {
                    self.went.push(Went::ApplyB { a });
                    self.node = b;
                    Some(())
                }
                _ => None,
            },
            InfixL(a, f, b) => match i {
                0 => {
                    self.went.push(Went::InfixLA { f, b });
                    self.node = a;
                    Some(())
                }
                1 => {
                    self.went.push(Went::InfixLF { a, b });
                    self.node = f;
                    Some(())
                }
                2 => {
                    self.went.push(Went::InfixLB { a, f });
                    self.node = b;
                    Some(())
                }
                _ => None,
            },
            InfixR(a, f, b) => match i {
                0 => {
                    self.went.push(Went::InfixRA { f, b });
                    self.node = a;
                    Some(())
                }
                1 => {
                    self.went.push(Went::InfixRF { a, b });
                    self.node = f;
                    Some(())
                }
                2 => {
                    self.went.push(Went::InfixRB { a, f });
                    self.node = b;
                    Some(())
                }
                _ => None,
            },
            If(value, branches) => match i {
                0 => {
                    self.went.push(Went::IfValue { branches });
                    self.node = value;
                    Some(())
                }
                _ => {
                    let index = i - 1;
                    let branch = branches.get(index)?.clone();
                    self.went.push(Went::IfBranch {
                        value,
                        before: branches[..index].to_vec(),
                        after: branches[index + 1..].to_vec(),
                        tag: branch.tag,
                        name: branch.name,
                    });
                    self.node = branch.block;
                    Some(())
                }
            },
            IfLet(value, branches, default) => match i {
                0 => {
                    self.went.push(Went::IfLetValue { branches, default });
                    self.node = value;
                    Some(())
                }
                1 => {
                    self.went.push(Went::IfLetDefault {
                        value,
                        branches,
                        tag: default.tag,
                        name: default.name,
                    });
                    self.node = default.block;
                    Some(())
                }
                _ => {
                    let index = i - 2;
                    let branch = branches.get(index)?.clone();
                    self.went.push(Went::IfLetBranch {
                        value,
                        before: branches[..index].to_vec(),
                        after: branches[index + 1..].to_vec(),
                        tag: branch.tag,
                        name: branch.name,
                        default,
                    });
                    self.node = branch.block;
                    Some(())
                }
            },
        }
    }
}
