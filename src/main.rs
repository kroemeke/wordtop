#![allow(unused_assignments)]
use clap::{App, Arg};
use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;

mod calculate; // functions related to rate calculation
mod print; // functions related to presenting results

fn main() {
    // argument parsing boilerplate
    let app = App::new("wordtop")
        .version("0.1")
        .about("top like word counting")
        .author("Marek Kroemeke");

    let top = Arg::with_name("top")
        .long("top")
        .short("t")
        .takes_value(true)
        .default_value("25")
        .help("Display top N words");

    let refresh = Arg::with_name("refresh")
        .long("refresh")
        .short("r")
        .takes_value(true)
        .default_value("2")
        .help("Refresh every <N> seconds.");

    let line = Arg::with_name("line")
        .long("line")
        .short("l")
        .takes_value(false)
        .help("Line mode - count same lines not words.");

    let sort = Arg::with_name("sort")
        .long("sort")
        .short("s")
        .takes_value(true)
        .possible_values(&["count", "rate"])
        .hide_possible_values(false)
        .default_value("count")
        .help("Sort by");

    let app = app.args(&[top, refresh, line, sort]);
    let matches = app.get_matches();

    let t = matches
        .value_of("top")
        .expect("top can't be none")
        .parse::<usize>()
        .unwrap();

    let r = matches
        .value_of("refresh")
        .expect("refresh can't be none")
        .parse::<u64>()
        .unwrap();

    let l = matches.is_present("line");

    let s = matches
        .value_of("sort")
        .expect("sort can't be none")
        .parse::<String>()
        .unwrap();

    let wordmap = Arc::new(Mutex::new(HashMap::new()));

    // end of argument parsing boilerplate
    //
    ctrlc::set_handler(|| {
        println!("ctrl-c");
        process::exit(0);
    })
    .unwrap();

    let arc_map = wordmap.clone();

    // Don't spawn a thread that prints top if we passed -r 0
    if r != 0 {
        thread::spawn(move || print::print_map_loop(arc_map, t, r, s));
    }

    let wordmap = wordmap;
    let stdin = io::stdin();
    if !l {
        for line in stdin.lock().lines() {
            match line {
                Err(_) => {}
                Ok(s) => {
                    for word in s.split_whitespace() {
                        let mut map = wordmap.lock().unwrap();
                        *map.entry(word.to_string()).or_insert(0) += 1;
                    }
                }
            }
        }
    } else {
        for line in stdin.lock().lines() {
            match line {
                Err(_) => {}
                Ok(s) => {
                    let mut map = wordmap.lock().unwrap();
                    *map.entry(s.to_string()).or_insert(0) += 1;
                }
            }
        }
    }
    print::print_map(wordmap, t);
}
