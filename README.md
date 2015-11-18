# jsx-rs
JSX syntax extension for Rust. Very early days, this doesn't do much yet.

When it's done, it will turn this:
```rust
jsx!(<div className="hello"><img src="pic.jpg"/></div>
```

into this:
```rust
react.Div { 
  className: "hello",
  children: vec![react.Img { src: "pic.jpg" }]
}
```

Those structs will come from [react-rs](/camjackson/react-rs)