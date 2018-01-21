#[derive(Debug, PartialEq)]
pub enum Term {
    Constant(usize),
    Symbol(usize),
}

impl Term {
    pub fn differentiate(mut self, symbol: usize) -> Self {
        match self {
            Term::Constant(_) => self = Term::Constant(0),
            Term::Symbol(other) => {
                if other == symbol {
                    self = Term::Constant(1);
                }
                else {
                    self = Term::Constant(0);
                }
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::Term;

    #[test]
    fn constant() {
        assert_eq!(Term::Constant(1).differentiate(0), Term::Constant(0));
        assert_eq!(Term::Constant(1).differentiate(1), Term::Constant(0));
    }

    #[test]
    fn symbol() {
        assert_eq!(Term::Symbol(0).differentiate(0), Term::Constant(1));
        assert_eq!(Term::Symbol(0).differentiate(1), Term::Constant(0));
    }
}
