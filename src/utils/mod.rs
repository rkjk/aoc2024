#[allow(dead_code)]
#[allow(unused_imports)]
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::time;

pub fn read_input(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines()
        .map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

pub fn bench<F, R>(f: F, identifier: Option<&str>) -> R
where 
    F: FnOnce() -> R,
{
    let t0 = time::Instant::now();
    let ret = f();
    let time_elapsed = time::Instant::now().duration_since(t0);
    match identifier {
        Some(v) => println!("time used for {} -> {:?}", v, time_elapsed),
        None => println!("time used -> {:?}", time_elapsed)
    };
    ret
}