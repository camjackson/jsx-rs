#![feature(plugin)]
#![plugin(jsx)]

struct Div {
    pub class_name: &'static str,
}

impl Div {
    pub fn render(&self) -> String {
        format!("<div class='{}'></div>", &self.class_name)
    }
}

struct Img {
    pub src: &'static str,
}

impl Img {
    pub fn render(&self) -> String {
        format!("<img src='{}'/>", &self.src)
    }
}

fn main() {
    let div = jsx!(<div class_name="world"></div>);
    let img = jsx!(<img src="pic.jpg"></img>); //TODO: Make self-terminating

    println!("{}", div.render());
    println!("{}", img.render());
}
