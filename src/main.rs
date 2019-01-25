mod controller;
mod server;
mod counter;

fn main() {
    println!("Start up server");
    ::server::startup(::controller::execute_wrapper)
}
