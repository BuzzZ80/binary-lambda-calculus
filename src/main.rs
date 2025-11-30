#[derive(Clone)]
enum Term {
    App(Box<Term>, Box<Term>), // 01xy
    Lam(Box<Term>),            // 00x
    Var(u16),                  // x0
}

fn main() {
    let mut input_str = Vec::<u8>::new();
    std::io::stdin().read_to_end(&mut input_str).unwrap();

    if let Some(mut t) =
        Term::from_binary(&mut input_str.into_iter().filter(|c| [b'0', b'1'].contains(c)))
    {
        println!("before: {t}");
        println!("      : {t:?}");
        while t.normal() {}
        println!(" after: {t}");
        println!("      : {t:?}");
    } else {
        println!("invalid input lol");
    }
}

impl Term {
    fn from_binary<I>(src: &mut I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        let first = src.next()?;

        if first == b'0' {
            let second = src.next()?;
            if second == b'0' {
                Some(Term::Lam(Box::new(Term::from_binary(src)?)))
            } else if second == b'1' {
                Some(Term::App(
                    Box::new(Term::from_binary(src)?),
                    Box::new(Term::from_binary(src)?),
                ))
            } else {
                None
            }
        } else if first == b'1' {
            let mut index = 1;
            while let Some(b'1') = src.next() {
                index += 1;
            }
            Some(Term::Var(index))
        } else {
            None
        }
    }

    // returns true if a reduction was done
    fn normal(&mut self) -> bool {
        match self {
            Term::App(x, y) => match *x.clone() {
                Term::Lam(mut t) => {
                    t.find_and_replace(1, y);
                    t.adjust_indices(0, -1);
                    *self = *t;
                    true
                }
                _ => x.normal() || y.normal(),
            },
            Term::Lam(x) => x.normal(),
            Term::Var(_) => false,
        }
    }

    fn find_and_replace(&mut self, f: u16, r: &Term) {
        match self {
            Term::App(x, y) => {
                x.find_and_replace(f, r);
                y.find_and_replace(f, r);
            }
            Term::Lam(x) => {
                x.find_and_replace(f + 1, r);
            }
            Term::Var(v) => {
                if *v == f {
                    let mut t = r.clone();
                    t.adjust_indices(0, f as i16);
                    *self = t;
                }
            }
        }
    }

    fn adjust_indices(&mut self, depth: u16, amt: i16) {
        match self {
            Term::App(x, y) => {
                x.adjust_indices(depth, amt);
                y.adjust_indices(depth, amt);
            }
            Term::Lam(x) => {
                x.adjust_indices(depth + 1, amt);
            }
            Term::Var(v) => {
                if *v > depth {
                    *self = Term::Var((*v as i16 + amt) as u16);
                }
            }
        }
    }
}

use std::fmt::Display;
impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::App(a, b) => write!(f, "({} {})", *a, *b)?,
            Term::Lam(a) => write!(f, "(\\{})", a)?,
            Term::Var(i) => write!(f, "{i}")?,
        }
        Ok(())
    }
}

use std::fmt::Debug;
use std::io::Read;
impl Debug for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::App(a, b) => write!(f, "01{:?}{:?}", *a, *b)?,
            Term::Lam(a) => write!(f, "00{:?}", a)?,
            Term::Var(i) => write!(f, "{}0", (0..*i).map(|_| '1').collect::<String>())?,
        }
        Ok(())
    }
}
