use std::env;

use string_mult::evaluating::evaluate_list;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.contains(&"help".to_string()) {
        println!("Usage: string_mult [OPTIONS]");
        println!("Options:");
        println!("  help         Print this help message");
        println!("  authors      Print the authors of this program");
        println!("  <path>       Path to a file containing list of string multiplication commands");
        return;
    }

    if args.contains(&"authors".to_string()) {
        println!("Written by artemii.kolomiichuk@ukma.edu.ua");
        return;
    }

    let path = &args[1];
    let content = std::fs::read_to_string(path).expect("could not read file");
    let res = evaluate_list(&content);
    for r in res.unwrap() {
        match r {
            Ok(s) => println!("{}", s),
            Err(e) => println!("Error: {}", e),
        }
    }
}
