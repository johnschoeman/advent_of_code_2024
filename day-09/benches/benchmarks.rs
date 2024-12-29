use day_09::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn bench_part1() {
    part1::process(divan::black_box(include_str!("../input.txt",))).unwrap();
}

#[divan::bench]
fn bench_part2() {
    part2::process(divan::black_box(include_str!("../input.txt",))).unwrap();
}
