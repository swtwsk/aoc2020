use nom::{
    branch::alt,
    character::complete::{digit1, one_of},
    combinator::{map_res, recognize},
    multi::{fold_many1, many0},
    sequence::tuple,
    IResult,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LexOperator {
    Plus,
    Multiply,
}

#[derive(Clone, Debug)]
pub enum LexToken {
    Number(i64),
    Operator(LexOperator),
    OpenParenthesis,
    CloseParenthesis,
}

pub fn get_next_token(tokens: &Vec<LexToken>, pos: usize) -> Result<&LexToken, String> {
    tokens.get(pos).ok_or(format!(
        "Unexpected end of input at position {}, expected paren or number",
        pos
    ))
}

pub fn lex_input(s: &str) -> IResult<&str, Vec<LexToken>> {
    fold_many1(
        nom::combinator::map(
            tuple((
                skip_whitespace,
                alt((
                    nom::combinator::map(lex_operator, LexToken::Operator),
                    nom::combinator::map(lex_number, LexToken::Number),
                    nom::combinator::map(nom::character::complete::char('('), |_| {
                        LexToken::OpenParenthesis
                    }),
                    nom::combinator::map(nom::character::complete::char(')'), |_| {
                        LexToken::CloseParenthesis
                    }),
                )),
            )),
            |((), token)| token,
        ),
        Vec::new(),
        |mut acc, token| {
            acc.push(token);
            acc
        },
    )(s)
}

fn lex_operator(s: &str) -> IResult<&str, LexOperator> {
    let (s, c) = nom::bytes::complete::take(1 as usize)(s)?;
    let error = "Expected an operator";
    Ok((
        s,
        match c.chars().next().unwrap() {
            '+' => Ok(LexOperator::Plus),
            '*' => Ok(LexOperator::Multiply),
            operator => Err(nom::Err::Error(nom::error::ParseError::from_char(
                error, operator,
            ))),
        }?,
    ))
}
fn lex_number(s: &str) -> IResult<&str, i64> {
    map_res(recognize(digit1), str::parse)(s)
}
fn skip_whitespace(s: &str) -> IResult<&str, ()> {
    Ok((many0(one_of(" \t\x0c\n"))(s)?.0, ()))
}
