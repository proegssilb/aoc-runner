extern crate microbench;

use std::cmp::Ordering;

use aoc_runner_macros::{solver, generator, aoc};
// use criterion::{Criterion, criterion_group, criterion_main, black_box, PlottingBackend};
// use pprof::criterion::{PProfProfiler, Output};
// use pprof::flamegraph::Options as FGOptions;

// use microbench::{Options, retain, bench};

type DataLine = (u16, u16, u16, u16);
type DataLine2 = u64;

fn str_to_uint(s: &str) -> u16 {
    let byes = s.as_bytes();
    match s.len() {
        1 => u16::from_be_bytes([0, byes[0]]),
        2 => u16::from_be_bytes([byes[0], byes[1]]),
        _ => panic!("Inappropriately sized item found: {}", s),
    }
}

pub fn u16_to_u64_array_assign(xs: &[u16; 4]) -> u64 {
    let [a, b] = xs[0].to_be_bytes();
    let [c, d] = xs[1].to_be_bytes();
    let [e, f] = xs[2].to_be_bytes();
    let [g, h] = xs[3].to_be_bytes();
    u64::from_be_bytes([a, b, c, d, e, f, g, h])
}

#[aoc(day04, i32)]
pub mod solutions {
    use aoc_runner_macros::solution;

    use super::*;

    // Generators -------------------------------------------------------
    #[generator(parse)]
    pub fn input_generator_tuple_parse(input: &str) -> Vec<DataLine> {
        let mut results: Vec<DataLine> = Vec::new();
        for line in input.lines() {
            let (s1, rest) = line.split_once('-').unwrap();
            let (e1, rest) = rest.split_once(',').unwrap();
            let (s2, e2) = rest.split_once('-').unwrap();
            results.push((
                s1.parse().unwrap(),
                e1.parse().unwrap(),
                s2.parse().unwrap(),
                e2.parse().unwrap(),
            ))
        }
        results
    }

    #[generator(tuple_bits)]
    pub fn input_generator_tuple_bitbang(input: &str) -> Vec<DataLine> {
        let mut results: Vec<DataLine> = Vec::new();
        for line in input.lines() {
            let (s1, rest) = line.split_once('-').unwrap();
            let (e1, rest) = rest.split_once(',').unwrap();
            let (s2, e2) = rest.split_once('-').unwrap();
            results.push((
                str_to_uint(s1),
                str_to_uint(e1),
                str_to_uint(s2),
                str_to_uint(e2),
            ))
        }
        results
    }

    #[generator(uint_bits)]
    pub fn input_generator_u64_bitbang(input: &str) -> Vec<DataLine2> {
        let mut results: Vec<DataLine2> = Vec::new();
        for line in input.lines() {
            let (s1, rest) = line.split_once('-').unwrap();
            let (e1, rest) = rest.split_once(',').unwrap();
            let (s2, e2) = rest.split_once('-').unwrap();
            let s1 = str_to_uint(s1);
            let e1 = str_to_uint(e1);
            let s2 = str_to_uint(s2);
            let e2 = str_to_uint(e2);
            results.push(u16_to_u64_array_assign(&[s1, e1, s2, e2]));
        }
        results
    }

    // Solutions --------------------------------------------------------

    #[solver(part1, tuplecmp)]
    pub fn solve_part1_tuple_cmp(input: Vec<DataLine>) -> i32 {
        input.iter().map(|dl| {
            let (a_s, a_e, b_s, b_e) = dl;
            let cmp1 = a_s.cmp(b_s);
            let cmp2 = a_e.cmp(b_e);
            let res = cmp1 != cmp2 || cmp1 == Ordering::Equal;
            res as i32
        }).sum()
    }

    #[solver(part1, uintcmp)]
    pub fn solve_part1_uint_cmp(input: Vec<DataLine2>) -> i32 {
        input.iter().map(|dl| {
            let [a, b, c, d, e, f, g, h] = dl.to_be_bytes();
            let (a_s, a_e, b_s, b_e) = (
                u16::from_be_bytes([a, b]),
                u16::from_be_bytes([c, d]),
                u16::from_be_bytes([e, f]),
                u16::from_be_bytes([g, h]),
            );
            let cmp1 = a_s.cmp(&b_s);
            let cmp2 = a_e.cmp(&b_e);
            let res = cmp1 != cmp2 || cmp1 == Ordering::Equal;
            res as i32
        }).sum()
    }

    #[solver(part1, tuplebits)]
    pub fn solve_part1_tuple_bitbang(input: Vec<DataLine>) -> i32 {
        input.iter().map(|dl| {
            let (a_s, a_e, b_s, b_e) = dl;
            let cmp1 = ((*a_s as i32) - (*b_s as i32)).signum();
            let cmp2 = ((*a_e as i32) - (*b_e as i32)).signum();
            let res = cmp1 != cmp2 || cmp1 == 0;
        res as i32
        }).sum()
    }

    #[solution(p1, aio_bitbang)]
    pub fn solut_part1_bitbang(input: &str) -> i32 {
        let mut sum: i32 = 0;
        for line in input.lines() {
            let (s1, rest) = line.split_once('-').unwrap();
            let (e1, rest) = rest.split_once(',').unwrap();
            let (s2, e2) = rest.split_once('-').unwrap();
            let s1 = str_to_uint(s1);
            let e1 = str_to_uint(e1);
            let s2 = str_to_uint(s2);
            let e2 = str_to_uint(e2);
            let cmp1 = ((s1 as i32) - (s2 as i32)).signum();
            let cmp2 = ((e1 as i32) - (e2 as i32)).signum();
            let res = cmp1 != cmp2 || cmp1 == 0;
            sum += res as i32;
        }

        sum
    }

    #[solver(part2, tuplecmp)]
    pub fn solve_part2_tuple_cmp(input: Vec<DataLine>) -> i32 {
        input.iter().map(|dl| { 
            let (a_s, a_e, b_s, b_e) = dl;
            let res = !(a_e < b_s || b_e < a_s);
            res as i32
        }).sum()
    }

    #[solver(part2, uintcmp)]
    pub fn solve_part2_uint_cmp(input: Vec<DataLine2>) -> i32 {
        input.iter().map(|dl| { 
            let [a, b, c, d, e, f, g, h] = dl.to_be_bytes();
            let (a_s, a_e, b_s, b_e) = (
                u16::from_be_bytes([a, b]),
                u16::from_be_bytes([c, d]),
                u16::from_be_bytes([e, f]),
                u16::from_be_bytes([g, h]),
            );
            let res = !(a_e < b_s || b_e < a_s);
            res as i32
        }).sum()
    }
}

// Testing ----------------------------------------------------------

/* User-supplied Test:
#[aoc_test(Part1)]
fn aoc_test(p1: F)
where F: Fn(&str) -> impl Into<i32>
{
assert_eq!(2i32, p1(sample_in1));
}

#[aoc_test(Part2)]
fn aoc_test(p2: F)
where F: Fn(&str) -> impl Into<i32>
{
assert_eq!(4i32, p2(sample_in1));
}
*/

// User should supply this
#[allow(unused)]
const SAMPLE_IN1: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#;

// Auto-generated Result (ideally):
#[test]
fn aoc_test() {
    for p1 in _gen_lists::P1S {
        assert_eq!(2i32, p1(SAMPLE_IN1));
    }
    for p2 in _gen_lists::P2S {
        assert_eq!(4i32, p2(SAMPLE_IN1));
    }
}

// Auto-Genned Benches ----------------------------------------------

// Microbench

// fn run_benches() {
//     let options = Options::default().time(Duration::from_secs(5));
// 
//     bench(&options, "Part 1 - tuple_parse / tuple_cmp", || _gen_lists::p1_1(retain(AOC_RAW_INPUT)) );
//     bench(&options, "Part 1 - tuple_parse / tuple_bitbang", || _gen_lists::p1_2(retain(AOC_RAW_INPUT)) );
//     bench(&options, "Part 1 - tuple_bitbang / tuple_bitbang", || _gen_lists::p1_4(retain(AOC_RAW_INPUT)) );
//     bench(&options, "Part 1 - uint_bitbang / uint_cmp", || _gen_lists::p1_5(retain(AOC_RAW_INPUT)) );
//     bench(&options, "Part 2 - tuple_bitbang / tuple_cmp", || _gen_lists::p2_1(retain(AOC_RAW_INPUT)) );
//     bench(&options, "Part 2 - uint_bitbang / uint_cmp", || _gen_lists::p2_2(retain(AOC_RAW_INPUT)) );
// }

// Criterion

// fn bench(c: &mut Criterion) {
//     let mut group1 = c.benchmark_group("Part 1");
//     group1.bench_function("tuple_parse/tuple_cmp", |b| b.iter(|| p1_1(black_box(AOC_RAW_INPUT))));
//     group1.bench_function("tuple_parse/tuple_bitbang", |b| b.iter(|| p1_2(black_box(AOC_RAW_INPUT))));
//     group1.bench_function("tuple_bitbang/tuple_bitbang", |b| b.iter(|| p1_4(black_box(AOC_RAW_INPUT))));
//     group1.bench_function("uint_bitbang/uint_cmp", |b| b.iter(|| p1_5(black_box(AOC_RAW_INPUT))));
//     group1.finish();
//     let mut group2 = c.benchmark_group("Part 2");
//     group2.bench_function("tuple_bitbang / tuple_cmp", |b| b.iter(|| p2_1(black_box(AOC_RAW_INPUT))));
//     group2.bench_function("uint_bitbang / uint_cmp", |b| b.iter(|| p2_2(black_box(AOC_RAW_INPUT))));
//     group2.finish();
// }

// criterion_group! {
//     name = benches;
//     config = Criterion::default()
//         //.with_profiler(PProfProfiler::new(100, Output::Flamegraph(Some(FGOptions::default()))))
//         .with_output_color(true)
//         .with_plots()
//         .plotting_backend(PlottingBackend::Plotters);
//     targets = bench
// }

// criterion_main!(benches);

// Auto-Genned Main ------------------------------------------------


