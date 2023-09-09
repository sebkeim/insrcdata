// minimalistic benchmark to verify behaviour with huge table

#[allow(dead_code)]
#[allow(unused_variables)]
mod insrcdata;

use std::cmp::max;
use std::env;
use std::time::Instant;

// run one cycle of benchmark
fn run() {
    let mut count = 1;

    for fic in insrcdata::Bench::array() {
        // println!("{}", fic.sentence());
        let x = fic.short();
        if x == 6151 {
            count += 1;
        }
    }

    for _i in 0..100 {
        let mut a = insrcdata::Bench::short_range(2, 69);
        count += a.next().is_some() as usize;

        let mut a = insrcdata::Bench::str_range("A", "K");
        count += a.next().is_some() as usize;
    }

    assert!(count > 0);
}

// parse configuration and run n cycles
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let cmd = &args[1];
        if cmd == "startup" {
            return;
        }
        let repeat = if cmd == "bench" {
            max(1, 10_000_000 / insrcdata::Bench::array().len())
        } else {
            cmd.parse::<usize>().expect("need number of repetitions")
        };
        let start = Instant::now();
        for _i in 0..repeat {
            run();
        }
        println!("{} : ms execution time", start.elapsed().as_millis());
    } else {
        // run only once, for test_all.py
        run();
    }
}
