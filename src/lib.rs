#![crate_name="jsx"]
#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private, quote)]

extern crate syntax;
extern crate rustc;

use syntax::codemap::{Span, Spanned};
use syntax::ast::{TokenTree, Ident, PathSegment, PathParameters, Path, Field};
use syntax::ext::base::{ExtCtxt, MacResult, MacEager, DummyResult};
use syntax::ext::build::AstBuilder;
use syntax::parse::token::{Token, Lit, intern};
use rustc::plugin::Registry;

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

struct Tag {
    name: Ident,
    attribute: Ident,
    value: Lit
}

impl Tag {
    fn path_to_struct(&self, sp: &Span) -> Path {
        let cap_name = capitalise_identifier(self.name);

        let name_segment = PathSegment { identifier: cap_name, parameters: PathParameters::none() };
        Path { span: *sp, global: false, segments: vec![name_segment] }
    }

    fn fields(&self, cx: &ExtCtxt, sp: &Span) -> Vec<Field> {
        let identifier = Spanned { node: self.attribute, span: *sp };

        let value = match self.value {
            Lit::Byte(val) => val.as_str(),
            Lit::Char(val) => val.as_str(),
            Lit::Integer(val) => val.as_str(),
            Lit::Float(val) => val.as_str(),
            Lit::Str_(val) => val.as_str(),
            Lit::StrRaw(val, _) => val.as_str(),
            Lit::ByteStr(val) => val.as_str(),
            Lit::ByteStrRaw(val, _) => val.as_str(),
        }.to_string();

        let expression = quote_expr!(cx, $value.to_string());
        vec![Field { ident: identifier, expr: expression, span: *sp }]
    }

    fn as_expr(&self, cx: &ExtCtxt, sp: &Span) -> Box<MacResult + 'static>{
        MacEager::expr(cx.expr_struct(*sp, self.path_to_struct(sp), self.fields(cx, sp)))
    }
}

fn read_open_tag(tokens: &[TokenTree]) -> Result<Tag, String> {
    try!(expect_token(&tokens[0], Token::Lt));
    let name = try!(expect_ident(&tokens[1]));
    let attribute = try!(expect_ident(&tokens[2]));
    try!(expect_token(&tokens[3], Token::Eq));
    let value = try!(expect_lit(&tokens[4]));
    Ok(Tag {
        name: name,
        attribute: attribute,
        value: value
    })
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

fn capitalise_identifier(identifier: Ident) -> Ident {
    let name = identifier.name.as_str().to_string();
    let capitalised = name[0..1].to_uppercase().to_string() + &name[1..];
    Ident::with_empty_ctxt(intern(&capitalised))
}

#[plugin_registrar]
pub fn plugin_registrar(registry: &mut Registry) {
    registry.register_macro("jsx", compile_jsx);
}
