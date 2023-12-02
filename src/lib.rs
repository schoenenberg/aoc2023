mod macros;

pub mod day01;
pub mod day02;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    print_day!(1, day01);
    print_day!(2, day02);

    Ok(())
}
