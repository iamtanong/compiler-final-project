/// Lexer for Retro BASIC using nom parser combinator library
use nom::{
    branch::alt,
    character::complete::{char, digit1, one_of, space0},
    combinator::map,
    IResult,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LineNum(u32), // Line number (1-1000)
    Id(char),     // Variable identifier (A-Z)
    If,           // IF keyword
    Goto,         // GOTO keyword
    Print,        // PRINT keyword
    Stop,         // STOP keyword
    Plus,         // + operator
    Minus,        // - operator
    Less,         // < operator
    Assign,       // = operator/assignment
}

/// Tokenize entire input
pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut remaining = input;

    loop {
        // Skip leading whitespace
        let (rest, _) = space0::<_, nom::error::Error<_>>(remaining)
            .map_err(|_| "Failed to parse input".to_string())?;

        if rest.is_empty() {
            break;
        }

        if let Some(r) = rest.strip_prefix('\n') {
            remaining = r;
            continue;
        }

        // Try to parse next token
        match parse_token(rest) {
            Ok((rest, token)) => {
                tokens.push(token);
                remaining = rest;
            }
            Err(_) => {
                if let Some(first_char) = rest.chars().next() {
                    return Err(format!("Unexpected character: '{}'", first_char));
                } else {
                    break;
                }
            }
        }
    }

    Ok(tokens)
}

/// Parse a single token
fn parse_token(input: &str) -> IResult<&str, Token> {
    let (input, _) = space0(input)?;
    alt((parse_line_num, parse_id_or_keyword, parse_operator))(input)
}

/// Parse a line number token
fn parse_line_num(input: &str) -> IResult<&str, Token> {
    map(digit1, |s: &str| Token::LineNum(s.parse::<u32>().unwrap()))(input)
}

/// Parse an identifier or keyword
fn parse_id_or_keyword(input: &str) -> IResult<&str, Token> {
    let (input, first_char) = one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ")(input)?;

    match first_char {
        'I' => {
            if let Some('F') = input.chars().next() {
                // Detect "IF" keyword
                Ok((&input[1..], Token::If))
            } else {
                Ok((input, Token::Id('I')))
            }
        }
        'G' => {
            if let Some("OTO") = input.get(0..3) {
                // Detect "GOTO" keyword
                Ok((&input[3..], Token::Goto))
            } else {
                Ok((input, Token::Id('G')))
            }
        }
        'P' => {
            if let Some("RINT") = input.get(0..4) {
                // Detect "PRINT" keyword
                Ok((&input[4..], Token::Print))
            } else {
                Ok((input, Token::Id('P')))
            }
        }
        'S' => {
            if let Some("TOP") = input.get(0..3) {
                // Detect "STOP" keyword
                Ok((&input[3..], Token::Stop))
            } else {
                Ok((input, Token::Id('S')))
            }
        }
        c => Ok((input, Token::Id(c))),
    }
}

/// Parse an operator
fn parse_operator(input: &str) -> IResult<&str, Token> {
    alt((
        map(char('+'), |_| Token::Plus),
        map(char('-'), |_| Token::Minus),
        map(char('<'), |_| Token::Less),
        map(char('='), |_| Token::Assign),
    ))(input)
}
