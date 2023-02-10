use regex::Regex;
use std::error::Error;
use std::{fs, process, usize};
use ansi_term::Colour::Cyan;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    match config.query {
        Some(query) => {
            let (results, numl) = search(
                &query,
                &contents,
                config.from.as_ref(),
                config.to.as_ref(),
                config.before,
                config.after,
                config.case_sensitive,
            );
            if results.is_empty() {
                println!("No results found");
            } else {
                for (num, line) in results.iter().enumerate() {
                    println!("{}: {}", numl[num], line);
                }
            }
        }
        None => {
            let (results, numl) = run_shows(&contents, config.from, config.to);

            for (num, _line) in results.iter().enumerate() {
                println!("{}: {}", numl[num], results[num]);
            }
        }
    }
    Ok(())
}

pub struct Config {
    pub filename: String,
    pub query: Option<String>,
    pub case_sensitive: bool,
    pub from: Option<usize>,
    pub to: Option<usize>,
    pub before: usize,
    pub after: usize,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments \n -h for help");
        }
        let filename = args[1].clone();
        let mut case_sensitive = false;
        let mut query = None;
        let mut from = None;
        let mut to = None;
        let mut after = 0;
        let mut before = 0;
        let mut i = 2;

        while i < args.len() {
            if let Err(err) = len_args(args, i) {
                println!("{}", err);
                process::exit(1);
            } else {
                match args[i].as_ref() {
                    "-s" | "-search" => {
                        query = Some(args[i + 1].clone());
                        i += 1;
                    }
                    "-i" | "-insensitive" => case_sensitive = true,
                    "-from" => {
                        from = Some(args[i + 1].parse().unwrap());
                        i += 1;
                    }
                    "-to" => {
                        to = Some(args[i + 1].parse().unwrap());
                        i += 1;
                    }
                    "-a" => {
                        after = args[i + 1].parse().unwrap();
                        i += 1;
                    }
                    "-b" => {
                        before = args[i + 1].parse().unwrap();
                        i += 1;
                    }
                    _ => return Err("invalid argument format \n -h for help"),
                }
                i += 1;
            }
        }
        Ok(Config {
            filename,
            query,
            case_sensitive,
            from,
            to,
            before,
            after,
        })
    }
}

pub fn len_args(args: &[String], i: usize) -> Result<(), &'static str> {
    if i + 1 >= args.len() {
        return Err("not enough arguments \n -h for help");
    }
    Ok(())
}

pub fn run_shows(
    contents: &str,
    from: Option<usize>,
    to: Option<usize>,
) -> (Vec<&str>, Vec<usize>) {
    let mut results = Vec::new();
    let mut num_lignes: Vec<usize> = Vec::new();

    for (i, line) in contents.lines().enumerate() {
        if from.map_or(true, |f| i >= f) && to.map_or(true, |t| i <= t && !results.contains(&line))
        {
            num_lignes.push(i);
            results.push(line);
        }
    }
    (results, num_lignes)
}

pub fn search<'a>(
    query: &str,
    contents: &'a str,
    from: Option<&usize>,
    to: Option<&usize>,
    before: usize,
    after: usize,
    case_sensitive: bool,
) -> (Vec<String>, Vec<usize>) {
    let mut results = Vec::new();
    let mut num_lignes: Vec<usize> = Vec::new();
    let re = if case_sensitive {
        Regex::new(&format!("(?i){}", query)).unwrap()
    } else {
        Regex::new(query).unwrap()
    };

    for (i, line) in contents.lines().enumerate() {
        if from.map_or(true, |f| i >= *f) && to.map_or(true, |t| i <= *t) && re.is_match(line){
            let mut edge: usize = 0;
            let mut edge_final: usize = contents.lines().count()-1;
            if before < i {
                edge = i - before;
            }
            if i + after < contents.lines().count() {
                edge_final = i + after;
            }
            for j in edge..edge_final + 1 {
                if !num_lignes.contains(&j) {
                    num_lignes.push(j);
                    let line = contents.lines().nth(j).unwrap();
                    if re.is_match(&line){
                        let colored = format!("{}", Cyan.paint(line));
                        results.push(colored);
                    }
                    else{
                        results.push(line.into());
                    }
                }
            }
        }
    }
    (results, num_lignes)
}
