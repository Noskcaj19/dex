extern crate ded;
extern crate pretty_env_logger;
use ded::Application;
use ded::Error;

fn main() {
    pretty_env_logger::init();
    let mut application = match Application::new() {
        Ok(a) => a,
        Err(e) => return handle_error(e),
    };

    // Run the main application loop.
    if let Err(e) = application.run() {
        handle_error(e)
    }
}

fn handle_error(err: Error) {
    eprintln!("An error occurred");
    eprintln!("{}", err.cause());
    std::process::exit(1);
}
