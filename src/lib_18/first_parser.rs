use super::{
    expr::{Expr, ParseExprIntermediateResult, ParseExprResult},
    lexer::{get_next_token, LexOperator, LexToken},
};

pub fn parse_expr(tokens: &Vec<LexToken>) -> ParseExprResult {
    let (expr, pos2) = parse_e(tokens, 0)?;
    if pos2 != tokens.len() {
        Err(format!(
            "Unexpected tokens from position {}: {:?}",
            pos2,
            tokens.iter().skip(pos2).collect::<Vec<_>>()
        ))
    } else {
        Ok(expr)
    }
}

fn parse_e(tokens: &Vec<LexToken>, pos: usize) -> ParseExprIntermediateResult {
    parse_f(tokens, pos).and_then(|(left_expr, pos2)| parse_g(tokens, left_expr, pos2))
}

fn parse_f(tokens: &Vec<LexToken>, pos: usize) -> ParseExprIntermediateResult {
    match get_next_token(tokens, pos)? {
        LexToken::OpenParenthesis => {
            parse_e(tokens, pos + 1).and_then(|(expr, pos2)| match get_next_token(tokens, pos2)? {
                LexToken::CloseParenthesis => Ok((expr, pos2 + 1)),
                c => Err(format!("Unexpected token {:?} at {}", c, pos2)),
            })
        }
        &LexToken::Number(i) => Ok((Expr::Atom(i), pos + 1)),
        c => Err(format!("Unexpected token {:?} at {}", c, pos)),
    }
}

fn parse_g(tokens: &Vec<LexToken>, previous: Expr, pos: usize) -> ParseExprIntermediateResult {
    match get_next_token(tokens, pos) {
        Ok(LexToken::Operator(op)) => parse_f(tokens, pos + 1)
            .map(|(expr1, pos2)| match op {
                LexOperator::Plus => (Expr::Add(Box::new(previous), Box::new(expr1)), pos2),
                LexOperator::Multiply => (Expr::Mul(Box::new(previous), Box::new(expr1)), pos2),
            })
            .and_then(|(expr, pos3)| parse_g(tokens, expr, pos3)),
        _ => Ok((previous, pos)),
    }
}
