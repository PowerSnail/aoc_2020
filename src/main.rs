use clap::clap_app;
mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

#[macro_use]
mod util;

fn main() {
    let matches = clap_app!(myapp =>
        (@setting SubcommandRequiredElseHelp)
        (@subcommand "day1-part1" =>)
        (@subcommand "day1-part2" =>)
        (@subcommand "day2-part1" =>)
        (@subcommand "day2-part2" =>)
        (@subcommand "day3-part1" =>)
        (@subcommand "day3-part2" =>)
        (@subcommand "day4-part1" =>)
        (@subcommand "day4-part2" =>)
        (@subcommand "day5-part1" =>)
        (@subcommand "day5-part2" =>)
        (@subcommand "day6-part1" =>)
        (@subcommand "day6-part2" =>)
        (@subcommand "day7-part1" =>)
        (@subcommand "day7-part2" =>)
        (@subcommand "day8-part1" =>)
        (@subcommand "day8-part2" =>)
        (@subcommand "day9-part1" =>)
        (@subcommand "day9-part2" =>)
        (@subcommand "day10-part1" =>)
        (@subcommand "day10-part2" =>)
        (@subcommand "day11-part1" =>)
        (@subcommand "day11-part2" =>)
        (@subcommand "day12-part1" =>)
        (@subcommand "day12-part2" =>)
        (@subcommand "day13-part1" =>)
        (@subcommand "day13-part2" =>)
        (@subcommand "day14-part1" =>)
        (@subcommand "day14-part2" =>)
        (@subcommand "day15-part1" =>)
        (@subcommand "day15-part2" =>)
        (@subcommand "day16-part1" =>)
        (@subcommand "day16-part2" =>)
        (@subcommand "day17-part1" =>)
        (@subcommand "day17-part2" =>)
        (@subcommand "day18-part1" =>)
        (@subcommand "day18-part2" =>)
        (@subcommand "day19-part1" =>)
        (@subcommand "day19-part2" =>)
        (@subcommand "day20-part1" =>)
        (@subcommand "day20-part2" =>)
        (@subcommand "day21-part1" =>)
        (@subcommand "day21-part2" =>)
        (@subcommand "day22-part1" =>)
        (@subcommand "day22-part2" =>)
        (@subcommand "day23-part1" =>)
        (@subcommand "day23-part2" =>)
        (@subcommand "day24-part1" =>)
        (@subcommand "day24-part2" =>)
        (@subcommand "day25-part1" =>)
        (@subcommand "day25-part2" =>)
    )
    .get_matches();

    let answer = match matches.subcommand() {
        Some(("day1-part1", _)) => day1::part1(),
        Some(("day1-part2", _)) => day1::part2(),
        Some(("day2-part1", _)) => day2::part1(),
        Some(("day2-part2", _)) => day2::part2(),
        Some(("day3-part1", _)) => day3::part1(),
        Some(("day3-part2", _)) => day3::part2(),
        Some(("day4-part1", _)) => day4::part1(),
        Some(("day4-part2", _)) => day4::part2(),
        Some(("day5-part1", _)) => day5::part1(),
        Some(("day5-part2", _)) => day5::part2(),
        Some(("day6-part1", _)) => day6::part1(),
        Some(("day6-part2", _)) => day6::part2(),
        Some(("day7-part1", _)) => day7::part1(),
        Some(("day7-part2", _)) => day7::part2(),
        Some(("day8-part1", _)) => day8::part1(),
        Some(("day8-part2", _)) => day8::part2(),
        Some(("day9-part1", _)) => day9::part1(),
        Some(("day9-part2", _)) => day9::part2(),
        Some(("day10-part1", _)) => day10::part1(),
        Some(("day10-part2", _)) => day10::part2(),
        Some(("day11-part1", _)) => day11::part1(),
        Some(("day11-part2", _)) => day11::part2(),
        Some(("day12-part1", _)) => day12::part1(),
        Some(("day12-part2", _)) => day12::part2(),
        Some(("day13-part1", _)) => day13::part1(),
        Some(("day13-part2", _)) => day13::part2(),
        Some(("day14-part1", _)) => day14::part1(),
        Some(("day14-part2", _)) => day14::part2(),
        Some(("day15-part1", _)) => day15::part1(),
        Some(("day15-part2", _)) => day15::part2(),
        Some(("day16-part1", _)) => day16::part1(),
        Some(("day16-part2", _)) => day16::part2(),
        Some(("day17-part1", _)) => day17::part1(),
        Some(("day17-part2", _)) => day17::part2(),
        Some(("day18-part1", _)) => day18::part1(),
        Some(("day18-part2", _)) => day18::part2(),
        Some(("day19-part1", _)) => day19::part1(),
        Some(("day19-part2", _)) => day19::part2(),
        Some(("day20-part1", _)) => day20::part1(),
        Some(("day20-part2", _)) => day20::part2(),
        Some(("day21-part1", _)) => day21::part1(),
        Some(("day21-part2", _)) => day21::part2(),
        Some(("day22-part1", _)) => day22::part1(),
        Some(("day22-part2", _)) => day22::part2(),
        Some(("day23-part1", _)) => day23::part1(),
        Some(("day23-part2", _)) => day23::part2(),
        Some(("day24-part1", _)) => day24::part1(),
        Some(("day24-part2", _)) => day24::part2(),
        Some(("day25-part1", _)) => day25::part1(),
        Some(("day25-part2", _)) => day25::part2(),
        _ => unreachable!(),
    };

    match answer {
        Some(answer) => println!("{}", answer),
        None => println!("ERROR: no answer found"),
    }
}
