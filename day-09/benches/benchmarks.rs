use day_09::part1;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process(divan::black_box(include_str!("../input.txt",))).unwrap();
}

// #[divan::bench]
// fn part2() {
//     part2::process(divan::black_box(include_str!(
//         "../input2.txt",
//     )))
//     .unwrap();
// }
