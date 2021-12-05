use clap::{App, Arg};
use ctrlc;
use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::thread;
use std::time;

fn print_map(map: &HashMap<String, usize>, size: usize) {
    let mut count: usize = 0;

    // convert map to vector and sort it by value
    let mut hash_vec: Vec<(&String, &usize)> = map.iter().collect();
    // this is cpu intensive
    hash_vec.sort_by(|a, b| b.1.cmp(a.1));
    print!("\x1B[2J\x1B[1;1H");
    for (k, v) in hash_vec {
        println!("{} {}", k, v);
        count += 1;
        if count >= size {
            break;
        };
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
        .short("t")
        .takes_value(true)
        .help("Display top N words")
        .required(true);

    let refresh = Arg::with_name("refresh")
        .long("refresh")
        .short("r")
        .takes_value(true)
        .help("Refresh sorted top every N lines")
        .required(false);

    let app = app.args(&[topn, refresh]);
    let matches = app.get_matches();

    let i = matches
        .value_of("topn")
        .expect("This can't be none")
        .parse::<usize>()
        .unwrap();

    let mut r: usize = 100;
    if matches.is_present("refresh") {
        r = matches
            .value_of("refresh")
            .expect("Missing refresh?")
            .parse::<usize>()
            .unwrap();
    }

    let mut wordmap = HashMap::new();
    let mut count: usize = 0;

    // end of argument parsing boilerplate
    //
    ctrlc::set_handler(|| {
        println!("Final wordmap:");
        //print_map(&wordmap, i);
    })
    .unwrap();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        count += 1;
        match line {
            Err(_) => {
                print_map(&wordmap, 1);
                break;
            }
            Ok(s) => {
                for word in s.split_whitespace() {
                    *wordmap.entry(word.to_string()).or_insert(0) += 1;
                }
            }
        }
        if count % r == 0 {
            print_map(&wordmap, i);
        }
    }
    // TODO: remove sleep for 1 second before final update and exit - used for pv benchmark
    thread::sleep(time::Duration::from_secs(1));
    print_map(&wordmap, i);
}
