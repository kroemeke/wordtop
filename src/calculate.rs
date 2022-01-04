use std::collections::HashMap;

/// Provided with two hashmaps where value is u64,
/// calculate rate per second.
// TODO change key/value into a tuple
// TODO change return to Option
#[inline]
pub fn calculate_rate(
    map: &HashMap<String, u64>,
    key: &str,
    value: &u64,
    seconds: u64,
) -> Option<u64> {
    let s = seconds as u64;
    match map.get(key) {
        Some(&p) => {
            let rate = (value - p) / s;
            Some(rate)
        }
        // No key in map, so we return Option->None
        _ => None,
    }
}

/// given two <String, u64> maps and time delta, return
/// a <String, u64> map where value is rate per second
pub fn calculate_rate_top(
    old_map: &HashMap<String, u64>,
    new_map: &HashMap<String, u64>,
    seconds: u64,
) -> HashMap<String, u64> {
    let mut result = HashMap::new();
    // iterate over new_map
    // for each item in the new map, check if we have an entry in old map
    // if yes - insert into result entry with (new_map[k] - old_map[k]) / seconds
    // if no - insert into result entry with new_map[k] / seconds
    // Once done for all elements - convert to vector, sort by value
    // and return top N results as either a map or vec
    for (k, v) in new_map.iter() {
        let rate = (v - old_map.get(k).unwrap_or(&0)) / seconds;
        result.insert(k.to_string(), rate);
    }
    result
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

    #[test]
    fn test_calculate_rate_top() {
        let mut old_map = HashMap::new();
        let mut new_map = HashMap::new();
        old_map.insert("dupa".to_string(), 10);
        new_map.insert("dupa".to_string(), 110);
        let result = calculate_rate_top(&old_map, &new_map, 10);
        for (k, v) in result.iter() {
            println!("result k:{} v:{}", k, v);
        }
        assert_eq!(result.get(&"dupa".to_string()).unwrap(), &10);
    }
}

// print::print_stats needs a sorted vector of (key, count, rate).
// Here we prepare this vector for display purposes.
pub fn prepare_vec(
    old_map: &HashMap<String, u64>,
    new_map: &HashMap<String, u64>,
    seconds: u64,
    sort: &String,
    elements: usize,
) -> Vec<(String, u64, u64)> {
    let mut ret_vec = Vec::new();
    // We are sorting by total count so we don't have to calculate rate for every single element.
    // Instead we calculate rate only for elements in the resulting top <elements> vector.
    if sort == "count" {
        let mut sorted_vec = Vec::new();
        sorted_vec = new_map.iter().collect(); // map to vector
        sorted_vec.sort_by(|a, b| b.1.cmp(a.1)); // sort by value
        sorted_vec.truncate(elements); // keep just top <elements>

        // iterate over top <elements> and individually calculate rate for each item.
        for (k, v) in sorted_vec {
            if let Some(rate) = calculate_rate(&old_map, k, v, seconds) {
                ret_vec.push((k.clone(), rate, v.clone()));
            } else {
                ret_vec.push((k.clone(), 0, v.clone()));
            }
        }
    } else {
        // In order to sort by current rate - we have to calculate rate for every single element
        // in our map. Once calculated - we convert to vector, sort, and return top <elements>.
        let rate_map = calculate_rate_top(old_map, new_map, seconds);
        let mut sorted_vec = Vec::new();
        sorted_vec = rate_map.iter().collect(); // map to vector
        sorted_vec.sort_by(|a, b| b.1.cmp(a.1)); // sort by value
        sorted_vec.truncate(elements); // keep just top <elements>

        for (k, v) in sorted_vec {
            ret_vec.push((k.clone(), v.clone(), new_map.get(k).unwrap().clone()));
        }

        ret_vec.sort_by(|a, b| b.1.cmp(&a.1));
        ret_vec.truncate(elements);
    }
    ret_vec
}
