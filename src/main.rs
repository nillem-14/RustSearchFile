use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "h" || args[1] == "help" {
        println!("<filePath> [-s <searched word or regex>] [-i/-insensitive] [-from <line usize>] [-to <line usize>]  \n Help : help (h) \n Version: version (v)");
        process::exit(1);
    } else if args[1] == "v" || args[1] == "version" {
        println!("V.0.0.1");
        process::exit(1);
    } else {
        let config = Config::new(&args).unwrap_or_else(|err| {
            println!("Problem parsing arguments: {}", err);
            process::exit(1);
        });
        if let Err(e) = minigrep::run(config) {
            println!("Application error {}", e);
            process::exit(1);
        }
    }
}
