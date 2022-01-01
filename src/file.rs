use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::{Arc, Mutex};

/// Saves result hashmap into given file.
pub fn save_result(
    map: Arc<Mutex<HashMap<String, usize>>>,
    filename: String,
) -> Result<(), std::io::Error> {
    let map = map.lock().expect("Unable to lock mutex.");
    let file = File::create(filename)?;
    let mut file = BufWriter::new(file);

    let mut sorted: Vec<(&String, &usize)> = map.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));

    for (k, v) in sorted {
        file.write(format!("{} {}\n", k, v).as_bytes())?;
    }

    file.flush()?;
    Ok(())
}
