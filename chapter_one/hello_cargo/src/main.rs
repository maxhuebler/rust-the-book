// https://doc.rust-lang.org/book/ch01-03-hello-cargo.html
// Cargo expects source files to live inside the src directory
// After running "cargo build", we can run the executable with "./target/debug/hello_cargo"
// However, we can simply use the "cargo run" command to combine both of these steps

// We can also use "cargo check" to make sure the code compiles without producing an executable.

// When your project is finally ready for release, you can use cargo build --release to compile it with optimizations.
// This makes an executable in target/release instead of target/debug
fn main() {
    println!("Hello, world!");
}
