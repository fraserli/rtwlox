use anyhow::Result;

use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
pub struct Token<'a> {
    ttype: TokenType,
    value: &'a str,
    line: usize,
    pos: usize,
}

#[rustfmt::skip]
#[derive(Debug)]
pub enum TokenType {
    // Single character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two character tokens
    Bang, BangEqual, Equal, EqualEqual, Greater,
    GreaterEqual, Less, LessEqual,

    // Literals
    Identifier, String, Number,

    // Keywords
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,
}

pub fn lex(mut source: &str) -> Result<Vec<Token>> {
    use TokenType::*;

    let mut tokens = Vec::new();

    let mut chars = source.chars().peekable();
    let mut line = 0;
    let mut pos = 0;

    while let Some(c) = chars.next() {
        if c == '\n' {
            line += 1;
            pos = 0;
            source = &source[1..];
            continue;
        } else if c.is_whitespace() {
            pos += 1;
            source = &source[1..];
            continue;
        }

        let (ttype, len) = match c {
            '(' => (LeftParen, 1),
            ')' => (RightParen, 1),
            '{' => (LeftBrace, 1),
            '}' => (RightBrace, 1),
            ',' => (Comma, 1),
            '.' => (Dot, 1),
            '-' => (Minus, 1),
            '+' => (Plus, 1),
            ';' => (Semicolon, 1),
            '*' => (Star, 1),
            '!' => {
                if chars.next_if(|c| *c == '=').is_some() {
                    (BangEqual, 2)
                } else {
                    (Bang, 1)
                }
            }
            '=' => {
                if chars.next_if(|c| *c == '=').is_some() {
                    (EqualEqual, 2)
                } else {
                    (Equal, 1)
                }
            }
            '<' => {
                if chars.next_if(|c| *c == '=').is_some() {
                    (LessEqual, 2)
                } else {
                    (Less, 1)
                }
            }
            '>' => {
                if chars.next_if(|c| *c == '=').is_some() {
                    (GreaterEqual, 2)
                } else {
                    (Greater, 1)
                }
            }
            '/' => {
                if chars.next_if(|c| *c == '/').is_some() {
                    let len = 2 + count_bytes_while(&mut chars, |c| *c != '\n');
                    pos = 0;
                    source = &source[len..];
                    continue;
                } else {
                    (Slash, 1)
                }
            }
            '"' => {
                let mut len = 1;
                loop {
                    len += count_bytes_while(&mut chars, |c| *c != '"' && *c != '\n');
                    if let Some(c) = chars.next() {
                        len += 1;
                        if c == '"' {
                            break;
                        } else {
                            line += 1;
                        }
                    } else {
                        panic!("unterminated string literal");
                    }
                }

                (String, len)
            }
            _ => {
                if c.is_ascii_digit() {
                    let mut len = 1;
                    len += count_bytes_while(&mut chars, |c| c.is_ascii_digit());

                    // HACK
                    if chars.peek() == Some(&'.') {
                        if let Some(c) = source.chars().nth(pos + len + 1) {
                            if c.is_ascii_digit() {
                                chars.next();
                                len += 1 + count_bytes_while(&mut chars, |c| c.is_ascii_digit());
                            }
                        }
                    }

                    (Number, len)
                } else if c.is_ascii_alphabetic() {
                    let mut len = 1;
                    len += count_bytes_while(&mut chars, |c| {
                        c.is_ascii_alphabetic() || c.is_ascii_digit()
                    });
                    match &source[..len] {
                        "and" => (And, len),
                        "class" => (Class, len),
                        "else" => (Else, len),
                        "false" => (False, len),
                        "fun" => (Fun, len),
                        "for" => (For, len),
                        "if" => (If, len),
                        "nil" => (Nil, len),
                        "or" => (Or, len),
                        "print" => (Print, len),
                        "return" => (Return, len),
                        "super" => (Super, len),
                        "this" => (This, len),
                        "true" => (True, len),
                        "var" => (Var, len),
                        "while" => (While, len),
                        _ => (Identifier, len),
                    }
                } else {
                    panic!("{c}")
                }
            }
        };

        let value = &source[..len];

        tokens.push(Token {
            ttype,
            value,
            line,
            pos,
        });

        source = &source[len..];
        pos += len;
    }

    Ok(tokens)
}

fn count_bytes_while<F>(chars: &mut Peekable<Chars>, func: F) -> usize
where
    F: FnOnce(&char) -> bool + Copy,
{
    let mut bytes = 0;

    while let Some(c) = chars.next_if(func) {
        bytes += c.len_utf8();
    }

    bytes
}
