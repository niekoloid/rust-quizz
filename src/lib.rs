mod controller;
mod counter;
mod server;

pub fn main() {
    server::startup(controller::execute_wrapper);
}
