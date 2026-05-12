#[derive(Clone)]
enum Term {
    App(Box<Term>, Box<Term>), // 01xy  (Application)
    Lam(Box<Term>),            // 00x   (Lambda)
    Var(u16),                  // x0    (Variable index)
}

fn main() {
    // Get the contents of the provided file
    let input_path = match std::env::args().nth(1) {
        Some(p) => p,
        None => {
            println!("Please provide an input filename.");
            return;
        }
    };

    let input_str = match std::fs::read(input_path) {
        Ok(s) => s,
        Err(_) => {
            println!("Could not read from provided file path.");
            return;
        }
    };

    // Filter anything that isn't a 1 or 0 from the input contents
    let filtered_program_bits = &mut input_str.into_iter()
        .filter(|c| [b'0', b'1'].contains(c));

    // Attempt to parse the input
    match Term::from_binary(filtered_program_bits) {
        // Print parsed expression, reduce it, then print the result.
        Some(mut t) => {
            println!("before: {t}");
            println!("      : {t:?}");
            while t.normal() {}
            println!(" after: {t}");
            println!("      : {t:?}");
        }
        None =>
            println!("There was an error wile reducing the expression.")
    }
}

impl Term {
    fn from_binary<I>(src: &mut I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        let first = src.next()?;

        // Term is either a lambda or an application
        if first == b'0' {
            match src.next() {
                // wrap the next term in a lambda
                Some(b'0') => Some(Term::Lam(Box::new(Term::from_binary(src)?))),

                // wrap the next two terms in an Application term
                Some(b'1') => Some(Term::App(
                    Box::new(Term::from_binary(src)?),
                    Box::new(Term::from_binary(src)?),
                )),

                Some(c) => panic!("Unexpected character {} in from_binary call's input.", c as char),
                None => panic!("File ended in the middle of a symbol."),
            }
        } 
        // Term is a variable index
        else if first == b'1' {
            // Count the total number of consecutive 1's
            let mut index = 1;
            loop {
                match src.next() {
                    Some(b'1') => index += 1,
                    Some(b'0') => break,
                    Some(c) => panic!("Unexpected character {} in from_binary call's input.", c as char),
                    None => panic!("File ended in the middle of a variable index symbol")
                }
            }
            Some(Term::Var(index))
        } 
        else {
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
