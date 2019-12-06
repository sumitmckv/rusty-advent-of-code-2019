mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod lib;

fn main()-> std::io::Result<()> {
    day1::day1a()?;
    day1::day1b()?;
    day2::day2a()?;
    day2::day2b()?;
    day3::day3()?;
    day4::day4a();
    day4::day4b();
    day5::day5a()?;
    day5::day5b()?;
    Ok(())
}


