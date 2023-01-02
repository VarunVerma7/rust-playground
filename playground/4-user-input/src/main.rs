
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    
    let mut file = fs::read_to_string(config.file).expect("File path to exist");
    let arr = file.split(" ");
    let mut counter = 0;
    for item in arr {
        if item == config.word {
            counter += 1;
        }
    }
    println!("Counter is {counter}");
}

struct Config {
    file: String,
    word: String
}

impl Config {
    fn new(args: &[String]) -> Config {

        if args.len() < 3 {
            panic!("Not enough arguments")
        }

        let file = args[1].clone();
        let word = args[2].clone();
        Config {
            file,
            word
        }
    }
}



