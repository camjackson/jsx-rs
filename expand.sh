rustc --color always --crate-type=dylib src/lib.rs
rustc --color always --extern jsx=libjsx.so src/main.rs -Z unstable-options --pretty expanded
