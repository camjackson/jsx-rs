rustc --crate-type=dylib src/lib.rs
#rustc -Z unstable-options --pretty expanded --extern jsx=libjsx.so src/main.rs
rustc --extern jsx=libjsx.so src/main.rs
./main
