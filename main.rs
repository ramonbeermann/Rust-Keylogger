use std::io::prelude::*;
use std::fs::File;
use std::time::Instant;

fn main() {
    let mut file = File::create("tasten.txt").unwrap();

    let mut start = Instant::now();
    loop {
        let timestamp = start.elapsed();
        let mut key = String::new();
        std::io::stdin().read_line(&mut key).unwrap();

        if key.starts_with("Ctrl") || key.starts_with("Alt") {
            continue;
        }

        file.write_all(format!("{:?},{}\n", timestamp, key).as_bytes()).unwrap();
    }
}
