// Source: https://repl.it/@Scoder12/aoc-rust-template

// Days
pub mod day01;
pub mod day02;
pub mod day05;
pub mod day07;
pub mod day08;
pub mod day09;

pub fn noop(_inp: String) {}

pub type DayFn = fn(String);

pub fn get_day(day: u32) -> (DayFn, DayFn) {
    return match day {
        1 => (day01::part1, day01::part2), // driver: yogan, navigator: t-animal, Russell
        2 => (day02::part1, day02::part2), // driver: yogan, navigator: dnnr, Russell
        5 => (day05::part1, day05::part2), // driver: yogan, navigator: dnnr
        7 => (day07::part1, day07::part2), // yogan
        8 => (day08::part1, day08::part2), // driver: yogan, navigator: dnnr
        9 => (day09::part1, day09::part2), // driver: dnnr,  navigator: yogan, Russell
        _ => {
            println!("Unknown day: {}", day);
            return (noop, noop);
        }
    };
}
