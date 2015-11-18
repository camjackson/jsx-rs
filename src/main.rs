#![feature(plugin)]
#![plugin(jsx)]

struct Div {
    pub class_name: String,
}

impl Div {
    pub fn render(&self) -> String {
        format!("<div class='{}'></div>", &self.class_name)
    }
}

fn main() {
    let div = jsx!(<Div class_name="world"></div>);

    println!("{}", div.render());
}
