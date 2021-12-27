use crate::calculate;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

pub fn print_map(map: Arc<Mutex<HashMap<String, usize>>>, size: usize) {
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

/// Given a Vec of tuples (word, count, rate), iterate over the vector and pretty-print the
/// the result. Result is printed as-is without any changes to to the order. Caller is responsible
/// for sorting Vec in advance if required.
pub fn print_stats(hash_vec: &Vec<(String, usize, usize)>, kwidth: usize, twidth: usize) -> (usize, usize) {
    let mut kwidth = kwidth; // key (word/line) width fill
    let mut twidth = twidth; // value count width
    for (word, count, rate) in hash_vec {
        // We try to automatically adjust pritnln spacing based on key lengths seen
        if word.len() > kwidth {
            kwidth = word.len();
        }
        // We try do automatically adjust println spacing based on count value len
        if rate.to_string().len() + 3 > twidth {
            twidth = rate.to_string().len() + 4;
        }
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
      let mut x: Vec<(String, usize, usize)> = Vec::new(); 
      x.push(("dupa".to_string(),100,10));
      x.push(("dupadupadupadupa".to_string(),100,10));
      x.push(("dupa2".to_string(),100,13213120));
      let (mut k, mut t) = print_stats(&x, 10,10);
      let (mut k, mut t) = print_stats(&x, 10,10);
  }
}


// TODO add sorting by rate
// TODO dynamically adjust format {: <20} if keys are too long
// TODO maybe add tui-rs option for prettier printing
pub fn print_map_loop(
    map: Arc<Mutex<HashMap<String, usize>>>,
    size: usize,
    refresh: u64,
    sort: String,
) {
    let mut count: usize = 0; // used to show top N entries
    let mut old_map = HashMap::new();
    let mut kwidth = 10; // key (word/line) width fill
    let mut twidth = 10; // value count width
    loop {
        let xmap = map.lock().unwrap();
        let map = xmap.clone();
        drop(xmap);
        let mut hash_vec: Vec<(&String, &usize)>;
        let rate_map;
        // this is cpu intensive
        if sort == "count" {
            // if we sort by count, then we convert current map into a sorted by value vec
            hash_vec = map.iter().collect();
            hash_vec.sort_by(|a, b| b.1.cmp(a.1));
        } else {
            // if we sort by rate, we need to calculate using our calculate_rate_top function
            // which returns a map where value is rate instead of count
            rate_map =
                calculate::calculate_rate_top(&old_map, &map, refresh.try_into().unwrap(), 10);
            hash_vec = rate_map.iter().collect();
            hash_vec.sort_by(|a, b| b.1.cmp(a.1));
        }
        print!("\x1B[2J\x1B[1;1H");
        for (k, v) in hash_vec {
            if sort == "count" {
                if let Some(rate) = calculate::calculate_rate(&old_map, k, v, refresh) {
                    // We try to automatically adjust pritnln spacing based on key lengths seen
                    if k.len() > kwidth {
                        kwidth = k.len();
                    }
                    // We try do automatically adjust println spacing based on count value len
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
                    // We couldn't calculate rate so we need to display things differently here
                    println!("{: <kwidth$}  {}", k, v, kwidth = kwidth + 10);
                }
                // Break out of the loop after N displayed entries (-t)
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
