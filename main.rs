mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

fn main() {
    day01::part_one(include_str!("./day01/input"));
    day01::part_two(include_str!("./day01/input"));

    day02::part_one(include_str!("./day02/input"));
    day02::part_two(include_str!("./day02/input"));

    day03::part_one(include_str!("./day03/input"), 3, 1);
    day03::part_two(include_str!("./day03/input"));

    day04::part_one(include_str!("./day04/input"));
    day04::part_two(include_str!("./day04/input"));

    day05::part_one(include_str!("./day05/input"));
    day05::part_two(include_str!("./day05/input"));

    day06::part_one(include_str!("./day06/input"));
    day06::part_two(include_str!("./day06/input"));

    day07::part_one(include_str!("./day07/input"));
    day07::part_two(include_str!("./day07/input"));

    day08::part_one(include_str!("./day08/input"));
    day08::part_two(include_str!("./day08/input"));

    day09::part_one(include_str!("./day09/input"), 25);
    day09::part_two(include_str!("./day09/input"), 25);

    day10::part_one(include_str!("./day10/input"));
    day10::part_two(include_str!("./day10/input"));

    day11::part_one(include_str!("./day11/input"));
    day11::part_two(include_str!("./day11/input"));
}
