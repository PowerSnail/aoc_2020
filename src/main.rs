use clap::clap_app;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

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
        _ => unreachable!(),
    };

    match answer {
        Some(answer) => println!("{}", answer),
        None => println!("ERROR: no answer found"),
    }
}
