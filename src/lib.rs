pub type Constant = i32;
pub type Sum = Vec<Term>;
pub type Symbol = i32;

#[derive(Debug, PartialEq)]
pub enum Term {
    Constant(Constant),
    Symbol(Symbol),
    Sum(Sum),
}

impl Term {
    pub fn differentiate(self, symbol: Symbol) -> Self {
        match self {
            Term::Constant(_) => Term::Constant(0),
            Term::Symbol(other) => diff_sym(other, symbol),
            Term::Sum(sum) => diff_sum(sum, symbol),
        }
    }
}

fn diff_sum(sum: Sum, symbol: Symbol) -> Term {
    let sum = sum.into_iter().map(|i| i.differentiate(symbol)).collect();
    Term::Sum(sum)
}

fn diff_sym(sym: Symbol, dsym: Symbol) -> Term {
    if sym == dsym {
        Term::Constant(1)
    } else {
        Term::Constant(0)
    }
}

#[cfg(test)]
mod tests {
    use super::Term;

    #[test]
    fn constant() {
        let s = 0;
        assert_eq!(Term::Constant(1).differentiate(s), Term::Constant(0));
        assert_eq!(Term::Constant(1).differentiate(s), Term::Constant(0));
    }

    #[test]
    fn symbol() {
        let x = 0;
        let y = 1;
        assert_eq!(Term::Symbol(x).differentiate(x), Term::Constant(1));
        assert_eq!(Term::Symbol(y).differentiate(x), Term::Constant(0));
    }

    #[test]
    fn sum() {
        let x = 0;
        let sum = Term::Sum(vec![Term::Constant(1), Term::Constant(2)]);
        let res = Term::Sum(vec![Term::Constant(0), Term::Constant(0)]);
        assert_eq!(sum.differentiate(x), res);
    }
}
