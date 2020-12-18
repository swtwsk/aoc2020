pub mod expr;

mod first_parser;
mod lexer;
mod second_parser;

pub fn first_parse(line: &str) -> Result<expr::Expr, String> {
    let (_, tokens) = lexer::lex_input(line).unwrap();
    first_parser::parse_expr(&tokens)
}

pub fn second_parse(line: &str) -> Result<expr::Expr, String> {
    let (_, tokens) = lexer::lex_input(line).unwrap();
    second_parser::parse_expr(&tokens)
}
