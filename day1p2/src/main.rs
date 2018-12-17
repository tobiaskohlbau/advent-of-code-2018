use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::collections::HashMap;

fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn main() -> Result<(), Error> {
    let vec = read(File::open("input.txt")?)?;
    let mut frequencies: HashMap<i64, i64> = HashMap::new();
    let mut frequency: i64 = 0;

    let mut i = 0;
    loop {
        frequency += vec[i];
        if frequencies.contains_key(&frequency) {
            break;
        }
        frequencies.insert(frequency, frequency);
        i += 1;
        if i == vec.len() {
            i = 0;
        }
    }

    println!("{}", frequency);
    Ok(())
}
