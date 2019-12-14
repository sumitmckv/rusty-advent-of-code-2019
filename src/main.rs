mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod lib;

fn main() -> std::io::Result<()> {
    day1::day1a()?;
    day1::day1b()?;
    day2::day2a()?;
    day2::day2b()?;
    day3::day3()?;
    day4::day4a();
    day4::day4b();
    day5::day5a()?;
    day5::day5b()?;
    day6::day6();
    day7::day7a();
    day7::day7b();
    day8::day8a();
    day8::day8b();
    day9::day9()?;
    Ok(())
}
