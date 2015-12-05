#![crate_name="jsx"]
#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private, quote)]

extern crate syntax;
extern crate rustc;

use syntax::codemap::Span;
use syntax::ast::TokenTree;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult};
use rustc::plugin::Registry;

mod tag;
mod parse;
use parse::read_open_tag;

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

#[plugin_registrar]
pub fn plugin_registrar(registry: &mut Registry) {
    registry.register_macro("jsx", compile_jsx);
}
