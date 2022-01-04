use crate::calculate;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

pub fn print_map(map: Arc<Mutex<HashMap<String, u64>>>, size: usize) {
    let mut count: usize = 0;
    let xmap = map.lock().unwrap();
    let map = xmap.clone();
    drop(xmap);
    let mut hash_vec: Vec<(&String, &u64)> = map.iter().collect();
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

/// Given a Vec of tuples (word, count, rate), iterate over the vector and pretty-print the
/// the result. Result is printed as-is without any changes to to the order. Caller is responsible
/// for sorting Vec in advance if required.
pub fn print_stats(
    hash_vec: &Vec<(String, u64, u64)>,
    kwidth: usize,
    twidth: usize,
) -> (usize, usize) {
    let mut kwidth = kwidth; // key (word/line) width fill
    let mut twidth = twidth; // value count width

    // First, scan whole vector and bump widths used for pretty printing
    // so that we don't have to adjust it half way through.
    //for (word, _count, rate) in hash_vec {
    for (word, rate, _count) in hash_vec {
        // We try to automatically adjust pritnln spacing based on key lengths seen
        if word.len() > kwidth {
            kwidth = word.len();
        }
        // We try do automatically adjust println spacing based on count value len
        if rate.to_string().len() + 3 > twidth {
            twidth = rate.to_string().len() + 4;
        }
    }

    print!("\x1B[2J\x1B[1;1H"); // clear screen
    for (word, rate, count) in hash_vec {
        println!(
            "{: <kwidth$} {: <twidth$} {}",
            word,
            format!("[{}/s]", rate),
            count,
            kwidth = kwidth,
            twidth = twidth,
        );
    }
    (kwidth, twidth)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_print_stats() {
        let mut x: Vec<(String, u64, u64)> = Vec::new();
        x.push(("dupa".to_string(), 100, 10));
        x.push(("dupadupadupadupa".to_string(), 100, 10));
        x.push(("dupa2".to_string(), 100, 13213120));
        let (mut k, mut t) = print_stats(&x, 10, 10);
    }
}

// TODO add sorting by rate
// TODO dynamically adjust format {: <20} if keys are too long
// TODO maybe add tui-rs option for prettier printing
pub fn print_map_loop(
    map: Arc<Mutex<HashMap<String, u64>>>,
    size: usize,
    refresh: u64,
    sort: String,
) {
    let mut old_map = HashMap::new();
    let kwidth = 10; // key (word/line) width fill
    let twidth = 10; // value count width
    loop {
        let xmap = map.lock().unwrap();
        let map = xmap.clone();
        drop(xmap);
        print_stats(
            &calculate::prepare_vec(&old_map, &map, refresh, &sort, size),
            kwidth,
            twidth,
        );
        old_map = map.clone();
        thread::sleep(time::Duration::from_secs(refresh));
    }
}
