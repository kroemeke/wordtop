#![allow(unused_assignments)]
use clap::{App, Arg};
use ctrlc;
use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

fn print_map(map: Arc<Mutex<HashMap<String, usize>>>, size: usize) {
    let mut count: usize = 0;
    let xmap = map.lock().unwrap();
    let map = xmap.clone();
    drop(xmap);
    let mut hash_vec: Vec<(&String, &usize)> = map.iter().collect();
    // this is cpu intensive
    hash_vec.sort_by(|a, b| b.1.cmp(a.1));
    print!("\x1B[2J\x1B[1;1H");
    for (k, v) in hash_vec {
        {
            println!("{} {}", k, v);
            count += 1;
            if count >= size {
                count = 0;
                break;
            };
        }
    }
}

fn print_map_loop(map: Arc<Mutex<HashMap<String, usize>>>, size: usize, refresh: u64) {
    let mut count: usize = 0;
    thread::sleep(time::Duration::from_secs(5));
    loop {
        let xmap = map.lock().unwrap();
        let map = xmap.clone();
        drop(xmap);
        let mut hash_vec: Vec<(&String, &usize)> = map.iter().collect();
        // this is cpu intensive
        hash_vec.sort_by(|a, b| b.1.cmp(a.1));
        print!("\x1B[2J\x1B[1;1H");
        for (k, v) in hash_vec {
            {
                println!("{} {}", k, v);
                count += 1;
                if count >= size {
                    count = 0;
                    break;
                };
            }
        }
        thread::sleep(time::Duration::from_secs(refresh));
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
        .default_value("10")
        .help("Display top N words");

    let refresh = Arg::with_name("refresh")
        .long("refresh")
        .short("r")
        .takes_value(true)
        .default_value("5")
        .help("Refresh every <N> seconds.");

    let line = Arg::with_name("line")
        .long("line")
        .short("l")
        .takes_value(false)
        .help("Line mode - count same lines not words.");

    let app = app.args(&[topn, refresh, line]);
    let matches = app.get_matches();

    let i = matches
        .value_of("topn")
        .expect("topn can't be none")
        .parse::<usize>()
        .unwrap();

    let r = matches
        .value_of("refresh")
        .expect("refresh can't be none")
        .parse::<u64>()
        .unwrap();

    let l = matches.is_present("line");

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
        thread::spawn(move || print_map_loop(arc_map, i, r));
    }

    let wordmap = wordmap.clone();
    let stdin = io::stdin();
    if l == false {
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
    let arc_map = wordmap.clone();
    print_map(arc_map, i);
}
