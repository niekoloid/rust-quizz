extern crate gcc;

fn main(){
    gcc::Build::new()
                .file("src/c/server.c")
                .include("src")
                .compile("server.a");
}
