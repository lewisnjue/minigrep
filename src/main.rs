use std::env;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = minigre::Config::build(&args).unwrap_or_else(|err| {
        eprintln!("failed with error : {}", err);

        process::exit(1);
    });
    if let Err(e) = minigre::run(config) {
        eprintln!("failed with error : {}", e);
        process::exit(1);
    }
}
