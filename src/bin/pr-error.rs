fn main() {
    let res = pirate_share(100, 0);
    println!("{}", res);

    // let io_error = io::Error::new(
    //     io::ErrorKind::Other, "timed out");

    // )
}

fn pirate_share(total: u64, crew_size: usize) -> u64 {
    let half = total / 2;
    half / crew_size as u64
}

use std::io::{self, BufRead};
type GenError = Box<dyn std::error::Error>;
type GenResult<T> = Result<T, GenError>;

fn read_numbers(file: &mut dyn BufRead) -> GenResult<Vec<i64>> {
    let mut numbers = vec![];
    for line_r in file.lines() {
        let line = line_r?;
        numbers.push(line.parse()?);
    }
    Ok(numbers)
}
