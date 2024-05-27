use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        len if len < 3 => {
            println!("Not enoght arguments!");
            process::exit(1);
        }
        len if len > 3 => {
            println!("To many arguments!");
            process::exit(1);
        }
        _ => {}
    }

    let query = &args[1];
    let filename = &args[2];
    let contents = fs::read_to_string(filename).unwrap_or_else(|err| {
        println!("Application error -> \"{err}\"");
        process::exit(1);
    });

    println!("Searching for {}", query);
    println!("In file {}", filename);
    // println!("With text:\n{}", contents);

    for line in search(&query, &contents) {
        println!("{}", line);
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line);
        }
    }
    if results.len() == 0 {
        println!("\"{}\" Not found!", query);
        process::exit(0);
    }
    results
}
