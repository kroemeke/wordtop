use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use clap::{Arg, App};

fn print_map(map: &HashMap<String, usize>, size: usize) {
    let mut count: usize = 0; 

    // convert map to vector and sort it by value
    let mut hash_vec: Vec<(&String, &usize)> = map.iter().collect();
    hash_vec.sort_by(|a, b| b.1.cmp(a.1));
    print!("\x1B[2J\x1B[1;1H");
    for (k, v) in hash_vec {
        println!("{} {}",k, v);
        count += 1;
        if count >= size { break; };
    }
}

fn main() {
    // argument parsing boilerplate
    let app = App::new("wordtop")
              .version("0.1")
              .about("top like word counting")
              .author("Marek Kroemeke");

    let topn = Arg::with_name("topn")
                .long("topn")
                .takes_value(true)
                .help("Display top N words")
                .required(true);

    let line_mode = Arg:with_name("lines")
                    .lond("lines")
                    .takes_value(false)
                    .help("Line mode - count unique lines not whitespace separated words.")
                    .requied(false);

    let app = app.arg(topn);
    let matches = app.get_matches();
    let i = matches.value_of("topn").expect("This can't be none").parse::<usize>().unwrap();


    let mut wordmap = HashMap::new();
    let mut count: usize = 0;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        count += 1;
        match line {
            Err(_) => {
                print_map(&wordmap, 1);
                break;
            },
            Ok(s) => {
                for word in s.split_ascii_whitespace() {
                  *wordmap.entry(word.to_string()).or_insert(0) += 1;
                }
            }
        }
        if count%100 == 0 {
            print_map(&wordmap, i);
        }
    }
    print_map(&wordmap, i);
}
