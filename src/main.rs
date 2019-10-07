use ignore::Walk;
use std::fs::metadata;
use trigram::find_words_iter;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 1+1 {
        eprintln!("Usage: ff pattern
This will search for pattern in files in the current directory tree not excluded
by .gitignore.
");
        std::process::exit(1);
    }
    let pattern = &args[1];
    for result in Walk::new("./") {
        match result {
            Ok(entry) => {
                let path: &str = entry.path().to_str().unwrap();
                let md = metadata(path).unwrap();
                if md.is_file() {
                    search_file(&pattern, path)
                }
            }
            Err(err) => eprintln!("ERROR: {}", err),
        }
    }
}

fn search_file(pattern: &str, path: &str) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut line_num = 0;
    for line in reader.lines() {
        line_num += 1;
        match line {
            Ok(line) => {
                for w in find_words_iter(pattern, &line, 0.45) {
                    println!("{}:{}: {}", path, line_num, w.as_str());
                }
            },
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}