mod day1;
mod day2;
mod day3;

fn main()-> std::io::Result<()> {
    day1::day1a()?;
    day1::day1b()?;
    day2::day2a()?;
    day2::day2b()?;
    day3::day3()?;
    Ok(())
}


