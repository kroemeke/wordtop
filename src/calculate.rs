use std::collections::HashMap;

/// Provided with two hashmaps where value is usize,
/// calculate rate per second.
// TODO change key/value into a tuple
// TODO change return to Option
#[inline]
pub fn calculate_rate(
    map: &HashMap<String, usize>,
    key: &str,
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

/// given two <String, usize> maps and time delta, return
/// a <String, usize> map where value is rate per second
pub fn calculate_rate_top(
    old_map: &HashMap<String, usize>,
    new_map: &HashMap<String, usize>,
    seconds: usize,
) -> HashMap<String, usize> {
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
