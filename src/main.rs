use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(arg) => match arg.as_str() {
            "h" | "-help" => {
                println!("<filePath> [-s <searched word or regex>] [-i/-insensitive] [-from <line usize>] [-to <line usize>]  [-b <number of lines before>] [-a <number of lines after>]\n Help : help (h) \n Version: version (v)");
                process::exit(1);
            }
            "-v" | "-version" => {
                println!("V.0.0.1");
                process::exit(1);
            }
            _ => {
                let config = Config::new(&args).unwrap_or_else(|err| {
                    println!("Problem parsing arguments: {}", err);
                    process::exit(1);
                });
                if let Err(e) = minigrep::run(config) {
                    println!("Application error {}", e);
                    process::exit(1);
                }
            }
        },
        None => {
            println!("Not enough arguments \n -h for help");
            process::exit(1);
        }
    }
}
