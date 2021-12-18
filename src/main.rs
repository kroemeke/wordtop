#![allow(unused_assignments)]
use clap::{App, Arg};
use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

/// Provided with two hashmaps where value is usize,
/// calculate rate per second.
// TODO change key/value into a tuple
// TODO change return to Option
fn calculate_rate(
    map: &HashMap<String, usize>,
    key: &String,
    value: &usize,
    seconds: u64,
) -> Option<usize> {
    let s = seconds as usize;
    match map.get(key) {
        Some(&p) => {
            let rate = (value - p) / s;
            Some(rate)
        }
        // No key in map, so we return Option->None
        _ => None,
    }
}

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
            println!("{: <20} {}", k, v);
            count += 1;
            if count >= size {
                count = 0;
                break;
            };
        }
    }
}

// TODO add sorting by rate
// TODO dynamically adjust format {: <20} if keys are too long
// TODO maybe add tui-rs option for prettier printing
fn print_map_loop(map: Arc<Mutex<HashMap<String, usize>>>, size: usize, refresh: u64) {
    let mut count: usize = 0;
    let mut old_map = HashMap::new();
    let mut kwidth = 10; // key (word/line) width fill
    let mut twidth = 10; // value count width
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
                if let Some(rate) = calculate_rate(&old_map, k, v, refresh) {
                    if k.len() > kwidth {
                        kwidth = k.len();
                    }
                    if rate.to_string().len() + 3 > twidth {
                        twidth = rate.to_string().len() + 4;
                    }
                    println!(
                        "{: <kwidth$} {: <twidth$} {}",
                        k,
                        format!("[{}/s]", rate),
                        v,
                        kwidth = kwidth,
                        twidth = twidth,
                    );
                } else {
                    println!("{: <kwidth$}  {}", k, v, kwidth = kwidth + 10);
                }
                count += 1;
                if count >= size {
                    count = 0;
                    break;
                };
            }
        }
        old_map = map.clone();
        thread::sleep(time::Duration::from_secs(refresh));
    }
}

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

    let app = app.args(&[top, refresh, line]);
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
        thread::spawn(move || print_map_loop(arc_map, t, r));
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
    print_map(wordmap, t);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_calculate_rate() {
        let mut test_map = HashMap::new();
        test_map.insert("dupa".to_string(), 20);
        let rate = calculate_rate(&test_map, &"dupa".to_string(), &300, 5);
        assert_eq!(rate.unwrap(), 56);
    }
}
