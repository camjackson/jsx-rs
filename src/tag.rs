use syntax::codemap::{Span, Spanned};
use syntax::ast::{Ident, PathSegment, PathParameters, Path, Field};
use syntax::ext::base::{ExtCtxt, MacResult, MacEager};
use syntax::ext::build::AstBuilder;
use syntax::parse::token::{Lit, intern};

pub struct Tag {
    name: Ident,
    attribute: Ident,
    value: String,
}

impl Tag {
    pub fn new(name: Ident, attribute: Ident, value: String) -> Tag {
        Tag { name: name, attribute: attribute, value: value }
    }

    pub fn as_expr(&self, cx: &ExtCtxt, sp: &Span) -> Box<MacResult + 'static>{
        MacEager::expr(cx.expr_struct(*sp, self.path_to_struct(sp), self.fields(cx, sp)))
    }

    fn path_to_struct(&self, sp: &Span) -> Path {
        let cap_name = capitalise_identifier(self.name);

        let name_segment = PathSegment { identifier: cap_name, parameters: PathParameters::none() };
        Path { span: *sp, global: false, segments: vec![name_segment] }
    }

    fn fields(&self, cx: &ExtCtxt, sp: &Span) -> Vec<Field> {
        let identifier = Spanned { node: self.attribute, span: *sp };
        let value = &self.value;

        let expression = quote_expr!(cx, $value);
        vec![Field { ident: identifier, expr: expression, span: *sp }]
    }
}

fn capitalise_identifier(identifier: Ident) -> Ident {
    let name = identifier.name.as_str().to_string();
    let capitalised = name[0..1].to_uppercase().to_string() + &name[1..];
    Ident::with_empty_ctxt(intern(&capitalised))
}
