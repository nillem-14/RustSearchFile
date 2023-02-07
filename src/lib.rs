use regex::Regex;
use std::error::Error;
use std::{fs, usize};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let mut results: Vec<&str> = Vec::new();
    let mut numl: Vec<usize> = Vec::new();

    match config.query {
        Some(query) => {
            if Regex::new(&query).is_err() {
                results = if config.case_sensitive.unwrap_or(true) {
                    search(&query, &contents, config.from.as_ref(), config.to.as_ref()).0
                } else {
                    search_case_insensitive(
                        &query,
                        &contents,
                        config.from.as_ref(),
                        config.to.as_ref(),
                    )
                    .0
                };
                numl = if config.case_sensitive.unwrap_or(true) {
                    search(&query, &contents, config.from.as_ref(), config.to.as_ref()).1
                } else {
                    search_case_insensitive(
                        &query,
                        &contents,
                        config.from.as_ref(),
                        config.to.as_ref(),
                    )
                    .1
                };
            } else {
                results = if config.case_sensitive.unwrap_or(true) {
                    search_re(&query, &contents, config.from.as_ref(), config.to.as_ref()).0
                } else {
                    search_case_insensitive_re(
                        &query,
                        &contents,
                        config.from.as_ref(),
                        config.to.as_ref(),
                    )
                    .0
                };

                numl = if config.case_sensitive.unwrap_or(true) {
                    search_re(&query, &contents, config.from.as_ref(), config.to.as_ref()).1
                } else {
                    search_case_insensitive_re(
                        &query,
                        &contents,
                        config.from.as_ref(),
                        config.to.as_ref(),
                    )
                    .1
                };
            }
            for (num, _line) in results.iter().enumerate() {
                println!("{}. {}", numl[num], results[num]);
            }
        }
        None => {
            numl = run_shows(&contents, config.from, config.to).1;
            results = run_shows(&contents, config.from, config.to).0;

            for (num, _line) in results.iter().enumerate() {
                println!("{}, {}", numl[num], results[num]);
            }
        }
    }
    Ok(())
}

pub struct Config {
    pub filename: String,
    pub query: Option<String>,
    pub case_sensitive: Option<bool>,
    pub from: Option<usize>,
    pub to: Option<usize>,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments \n -h for help");
        }
        let filename = args[1].clone();

        let mut case_sensitive = None;
        let mut query = None;
        let mut from = None;
        let mut to = None;

        let mut i = 2;

        while i < args.len() {
            match args[i].as_ref() {
                "-s" | "--search" => {
                    if i + 1 >= args.len() {
                        return Err("Not enough arguments \n h for help");
                    }
                    query = Some(args[i + 1].clone());
                    i += 1;
                }
                "-i" | "--insensitive" => case_sensitive = Some(false),
                "--from" => {
                    if i + 1 >= args.len() {
                        return Err("not enough arguments \n h for help");
                    }
                    from = Some(args[i + 1].parse().unwrap());
                    i += 1;
                }
                "--to" => {
                    if i + 1 >= args.len() {
                        return Err("not enough arguments \n h for help");
                    }
                    to = Some(args[i + 1].parse().unwrap());
                    i += 1;
                }
                _ => return Err("invalid argument format \n h for help"),
            }
            i += 1;
        }
        Ok(Config {
            filename,
            query,
            case_sensitive,
            from,
            to,
        })
    }
}

pub fn run_shows(
    contents: &str,
    from: Option<usize>,
    to: Option<usize>,
) -> (Vec<&str>, Vec<usize>) {
    let mut results = Vec::new();
    let mut num_lignes: Vec<usize> = Vec::new();

    for (i, line) in contents.lines().enumerate() {
        if from.map_or(true, |f| i >= f) && to.map_or(true, |t| i <= t) {
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
) -> (Vec<&'a str>, Vec<usize>) {
    let mut results = Vec::new();
    let mut num_lignes: Vec<usize> = Vec::new();

    for (i, line) in contents.lines().enumerate() {
        if from.map_or(true, |f| i >= *f) && to.map_or(true, |t| i <= *t) && line.contains(query) {
            num_lignes.push(i);
            results.push(line);
        }
    }
    (results, num_lignes)
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
    from: Option<&usize>,
    to: Option<&usize>,
) -> (Vec<&'a str>, Vec<usize>) {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    let mut num_lignes: Vec<usize> = Vec::new();

    for (i, line) in contents.lines().enumerate() {
        if from.map_or(true, |f| i >= *f)
            && to.map_or(true, |t| i <= *t)
            && line.to_lowercase().contains(&query)
        {
            num_lignes.push(i);
            results.push(line);
        }
    }
    (results, num_lignes)
}

pub fn search_re<'a>(
    query: &str,
    contents: &'a str,
    from: Option<&usize>,
    to: Option<&usize>,
) -> (Vec<&'a str>, Vec<usize>) {
    let mut results = Vec::new();
    let mut num_lignes: Vec<usize> = Vec::new();
    let re = Regex::new(query).unwrap();

    for (i, line) in contents.lines().enumerate() {
        if from.map_or(true, |f| i >= *f) && to.map_or(true, |t| i <= *t) && re.is_match(line) {
            num_lignes.push(i);
            results.push(line);
        }
    }
    (results, num_lignes)
}

pub fn search_case_insensitive_re<'a>(
    query: &str,
    contents: &'a str,
    from: Option<&usize>,
    to: Option<&usize>,
) -> (Vec<&'a str>, Vec<usize>) {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    let mut num_lignes: Vec<usize> = Vec::new();
    let re = Regex::new(&query).unwrap();

    for (i, line) in contents.lines().enumerate() {
        if from.map_or(true, |f| i >= *f)
            && to.map_or(true, |t| i <= *t)
            && re.is_match(&line.to_lowercase())
        {
            num_lignes.push(i);
            results.push(line);
        }
    }
    (results, num_lignes)
}
