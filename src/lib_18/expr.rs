#[derive(Debug)]
pub enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Atom(i64),
}

pub type ParseExprResult = Result<Expr, String>;
pub type ParseExprIntermediateResult = Result<(Expr, usize), String>;

impl Expr {
    pub fn compute_value(&self) -> i64 {
        match self {
            Expr::Add(e1, e2) => e1.compute_value() + e2.compute_value(),
            Expr::Mul(e1, e2) => e1.compute_value() * e2.compute_value(),
            &Expr::Atom(i) => i,
        }
    }
}
