extern crate syntax;
extern crate rustc;

use syntax::ast::{TokenTree, Ident};
use syntax::parse::token::{Token, BinOpToken, Lit};

use tag::Tag;

pub fn read_open_tag(tokens: &[TokenTree]) -> Result<Tag, String> {
    try!(expect_token(&tokens[0], Token::Lt));
    let name = try!(expect_ident(&tokens[1]));

    let attribute = try!(expect_ident(&tokens[2]));
    try!(expect_token(&tokens[3], Token::Eq));
    let value = try!(expect_string_lit(&tokens[4]));

    try!(expect_token(&tokens[5], Token::Gt));

    try!(expect_token(&tokens[6], Token::Lt));
    try!(expect_binop(&tokens[7], BinOpToken::Slash));
    let closing_name = try!(expect_ident(&tokens[8]));
    try!(expect_token(&tokens[9], Token::Gt));

    if name == closing_name {
        Ok(Tag::new(name, attribute, value))
    } else {
        Err(format!("Expected closing tag {:?} but found {:?}", name, closing_name))
    }
}

fn expect_token(actual: &TokenTree, expected: Token) -> Result<(), String> {
    match *actual {
        TokenTree::Token(_, ref token) => {
            if *token == expected {
                Ok(())
            } else {
                Err(format!("Expected token {:?} but found {:?}", expected, token))
            }
       },
       _ => Err("I don't know how to parse that :(".to_string())
    }
}

fn expect_binop(actual: &TokenTree, expected: BinOpToken) -> Result<(), String> {
    match *actual {
        TokenTree::Token(_, ref token) => {
            match *token {
                Token::BinOp(binop) => {
                    if binop == expected {
                        Ok(())
                    } else {
                        Err(format!("Expected binop {:?} but found binop {:?}", expected, binop))
                    }
                },
                _ => Err(format!("Expected binop {:?} but found {:?}", expected, token))
            }
        },
       _ => Err("I don't know how to parse that :(".to_string())
    }
}

fn expect_ident(actual: &TokenTree) -> Result<Ident, String> {
    match *actual {
        TokenTree::Token(_, ref token) => {
            match *token {
                Token::Ident(ident, _) => Ok(ident),
                _ => Err(format!("Expected identifier but found {:?}", token))
            }
        },
       _ => Err("I don't know how to parse that :(".to_string())
    }
}

fn expect_string_lit(actual: &TokenTree) -> Result<String, String> {
    match *actual {
        TokenTree::Token(_, ref token) => {
            match *token {
                Token::Literal(lit, _) => {
                    match lit {
                        Lit::Str_(string) => Ok(string.as_str().to_string()),
                        _ => Err(format!("Expected string literal for attribute value, but found {:?}", lit))
                    }
                }
                _ => Err(format!("Expected literal but found {:?}", token))
            }
        },
       _ => Err("I don't know how to parse that :(".to_string())
    }
}
