#![crate_name="jsx"]
#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private, quote)]

extern crate syntax;
extern crate rustc;

use syntax::codemap::{Span};
use syntax::ast::{TokenTree, Ident};
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult};
use syntax::parse::token::{Token, BinOpToken, Lit};
use rustc::plugin::Registry;

mod tag;
use tag::Tag;

fn compile_jsx(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'static> {
    let tag = match read_open_tag(args) {
        Ok(tag) => tag,
        Err(err) => {
            cx.span_err(sp, &err);
            return DummyResult::expr(sp);
        }
    };

    tag.as_expr(cx, &sp)
}

fn read_open_tag(tokens: &[TokenTree]) -> Result<Tag, String> {
    try!(expect_token(&tokens[0], Token::Lt));
    let name = try!(expect_ident(&tokens[1]));

    let attribute = try!(expect_ident(&tokens[2]));
    try!(expect_token(&tokens[3], Token::Eq));
    let value = try!(expect_lit(&tokens[4]));

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

fn expect_lit(actual: &TokenTree) -> Result<Lit, String> {
    match *actual {
        TokenTree::Token(_, ref token) => {
            match *token {
                Token::Literal(ident, _) => Ok(ident),
                _ => Err(format!("Expected literal but found {:?}", token))
            }
        },
       _ => Err("I don't know how to parse that :(".to_string())
    }
}

#[plugin_registrar]
pub fn plugin_registrar(registry: &mut Registry) {
    registry.register_macro("jsx", compile_jsx);
}
