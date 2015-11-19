rustc --color always --crate-type=dylib src/lib.rs
#rustc -Z unstable-options --pretty expanded --extern jsx=libjsx.so src/main.rs
rustc --color always --extern jsx=libjsx.so src/main.rs
./main
