#![feature(test)]
extern crate test;
use test::Bencher;

extern crate time;

extern crate num;
use num::bigint::BigInt;
use num::bigint::ToBigInt;
use num::One;
use std::thread;

extern crate num_cpus;


fn par_fact_thread(lower: num::BigInt, upper: num::BigInt) -> BigInt {
    let mut result = lower.clone();
    let mut iter = lower + BigInt::one();
    while iter <= upper {
        result = result * &iter;
        iter = iter + BigInt::one();
    }
    result
}

fn create_segments(upper_bound: u64, parts: u64) -> Vec<(u64, u64)> {
    let mut segments: Vec<(u64, u64)> = vec![];

    let part: u64 = upper_bound / parts;
    for i in 1..parts {
        let end = i * part;
        segments.push((end - part + 1, end));
    }

    let last_part: u64 = upper_bound % parts;
    if last_part == 0 {
        segments.push((upper_bound - part + 1, upper_bound));
    } else {
        segments.push(((parts - 1) * part + 1, upper_bound));
    }
    segments
}

fn par_fact(n: u64, threads: u64) -> BigInt {
    let segments: Vec<(u64, u64)> = create_segments(n, threads);

    let threads: Vec<_> = segments.into_iter()
        .map(|s| (s.0.to_bigint().unwrap(), s.1.to_bigint().unwrap()))
        .map(move |seg| thread::spawn(move || par_fact_thread(seg.0, seg.1)))
        .collect();
    let inter_results: Vec<BigInt> = threads.into_iter()
        .map(move |handle| handle.join().unwrap())
        .collect::<Vec<_>>();
    let mut result = BigInt::one();
    for i in inter_results.into_iter() {
        result = result * i;
    }
    return result;

}

fn fact(n: u64) -> BigInt {
    let mut result = n.to_bigint().unwrap();
    let mut i: BigInt = (n - 1).to_bigint().unwrap();
    let one = BigInt::one();
    let mut iters = n;
    while iters > 1 {
        result = result * &i;
        i = i - &one;
        iters -= 1;
    }
    result
}
fn time_fact(n: u64) {
    let start = time::precise_time_ns();
    let result = fact(n);
    let end = time::precise_time_ns();
    println!("Took {}ms\t to calculate {}! serially",
             (end - start) / 1000000,
             n);
}

fn time_par_fact(n: u64, threads: u64) {
    let start = time::precise_time_ns();
    let result = par_fact(n, threads);
    let end = time::precise_time_ns();
    println!("Took {}ms\t to calculate {}! concurrently with {} threads",
             (end - start) / 1000000,
             n,
             threads);
}

fn main() {
    let iters = 100000;
    time_fact(iters);
    time_par_fact(iters, 4);
}

#[bench]
fn bench_single(b: &mut Bencher) {
    b.iter(|| {
        test::black_box(fact(1000));
    });
}

#[bench]
fn bench_multiple(b: &mut Bencher) {
    b.iter(|| {
        test::black_box(par_fact(1000, num_cpus::get() as u64));

    });
}
