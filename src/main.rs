use std::cmp::{max, min};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use chrono::Local;
use hashbrown::HashMap;
use itertools::Itertools;

type City = String;
type Total = i64;
type Count = usize;
type Min = i64;
type Max = i64;
type Data = (Total, Count, Min, Max);

fn main() -> Result<(), Box<dyn Error>> {
    let s = Local::now();

    let mut map: HashMap<City, Data> = HashMap::with_capacity(1024);

    let total = 10000000;
    let mut i = 0;

    for line in BufReader::new(File::open(format!("data/{total}.txt"))?).lines() {
        let line = line?;

        let sp: Vec<&str> = line.split(';').collect();
        let city = sp[0];
        let n = (sp[1].parse::<f64>().unwrap() * 10.0) as i64;
        let e = map.entry_ref(city).or_insert((0, 0, 999, -999));
        let (a, b, c, d) = *e;
        *e = (a + n, b + 1, min(c, n), max(d, n));

        i += 1;
        if i % (total / 10) == 0 {
            println!("{}%", i / (total / 10) * 10);
        }
    }

    let vec: Vec<(City, Data)> = map.into_iter().collect();

    vec.iter().sorted_by_key(|x| &x.0).for_each(|(city, (total, count, min, max))| {
        println!("{}={}/{:.1}/{}", city, *min as f64 / 10.0, *total as f64 / *count as f64, *max as f64 / 10.0)
    });

    let e = Local::now();

    println!("{:?}", e - s);

    Ok(())
}
