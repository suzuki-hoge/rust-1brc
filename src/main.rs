use std::cmp::{max, min};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use chrono::Local;
use hashbrown::HashMap;
use itertools::Itertools;

type City = String;
type Total = i32;
type Count = usize;
type Min = i16;
type Max = i16;
type Data = (Total, Count, Min, Max);

fn main() -> Result<(), Box<dyn Error>> {
    let s = Local::now();

    let mut map: HashMap<City, Data> = HashMap::with_capacity(1024);

    let total = 10000000;
    let mut i = 0;

    for line in BufReader::new(File::open(format!("data/{total}.txt"))?).lines() {
        let line = line?;

        let (city, s2) = split(&line);
        let n = parse(s2);
        let e = map.entry_ref(city).or_insert((0, 0, 999, -999));
        let (a, b, c, d) = *e;
        *e = (a + n as i32, b + 1, min(c, n), max(d, n));

        i += 1;
        if i % (total / 10) == 0 {
            println!("{}%", i / (total / 10) * 10);
        }
    }

    let vec: Vec<(City, Data)> = map.into_iter().collect();

    vec.iter().sorted_by_key(|x| &x.0).for_each(|(city, (total, count, min, max))| {
        println!("{}={}/{:.1}/{}", city, *min as f64 / 10.0, *total as f64 / *count as f64 / 10.0, *max as f64 / 10.0)
    });

    let e = Local::now();

    println!("{:?}", e - s);

    Ok(())
}

fn split(s: &str) -> (&str, &str) {
    let i = s.rfind(';').unwrap();
    (&s[0..i], &s[i + 1..s.len()])
}

fn parse(s: &str) -> i16 {
    let mut f = 1;
    let n = s.as_bytes().iter().fold(0_i16, |acc, c| match c {
        b'-' => {
            f = -1;
            acc
        }
        b'.' => acc,
        _ => acc * 10 + ((*c - b'0') as i16),
    });
    f * n
}

#[cfg(test)]
mod tests {
    use crate::parse;

    #[test]
    fn test_parse() {
        assert_eq!(parse("12.3"), 123);
        assert_eq!(parse("-12.3"), -123);
    }
}
