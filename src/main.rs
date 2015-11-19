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

struct Img {
    pub src: String,
}

impl Img {
    pub fn render(&self) -> String {
        format!("<img src='{}'/>", &self.src)
    }
}

fn main() {
    let div = jsx!(<div class_name="world"></div>);
    //let img = jsx!(<Img src="pic.jpg"></div>);

    println!("{}", div.render());
    //println!("{}", img.render());
}
