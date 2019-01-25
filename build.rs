extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/c/server.c")
        .compile("server.a");
}
