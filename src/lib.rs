#![crate_name="jsx"]
#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private, quote)]

extern crate syntax;
extern crate rustc;

use syntax::codemap::{Span, Spanned};
use syntax::ast::{TokenTree, Ident, PathSegment, PathParameters, Path, Field};
use syntax::ext::base::{ExtCtxt, MacResult, MacEager, DummyResult};
use syntax::ext::build::AstBuilder;
use syntax::parse::token::{Token, intern};
use rustc::plugin::Registry;

fn compile_jsx(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'static> {
    let tag = match read_open_tag(args) {
        Ok(tag) => tag,
        Err(err) => {
            cx.span_err(sp, &err);
            return DummyResult::expr(sp);
        }
    };

    let tag = capitalise_identifier(tag);

    let path_to_struct = path_to_struct(tag, &sp);
    let fields = vec![field("class_name", "hello", cx, &sp)];

    MacEager::expr(cx.expr_struct(sp, path_to_struct, fields))
}

fn read_open_tag(tokens: &[TokenTree]) -> Result<Ident, String> {
    try!(expect_token(&tokens[0], Token::Lt));
    let tag = try!(expect_ident(&tokens[1]));
    Ok(tag)
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

fn capitalise_identifier(identifier: Ident) -> Ident {
    let name = identifier.name.as_str().to_string();
    let capitalised = name[0..1].to_uppercase().to_string() + &name[1..];
    Ident::with_empty_ctxt(intern(&capitalised))
}

fn path_to_struct(identifier: Ident, sp: &Span) -> Path {
    //let identifier = Ident::with_empty_ctxt(intern(name));
    let name_segment = PathSegment { identifier: identifier, parameters: PathParameters::none() };
    Path { span: *sp, global: false, segments: vec![name_segment] }
}

fn field(name: &str, val: &str, cx: &ExtCtxt, sp: &Span) -> Field {
    let identifier = Spanned { node: Ident::with_empty_ctxt(intern(name)), span: *sp };
    let expression = quote_expr!(cx, $val.to_string());
    Field { ident: identifier, expr: expression, span: *sp }
}

#[plugin_registrar]
pub fn plugin_registrar(registry: &mut Registry) {
    registry.register_macro("jsx", compile_jsx);
}
