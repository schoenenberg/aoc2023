/// Solves the daily challenge and prints the result. Requires a `solve` function in the module.
///
/// ```rust,ignore
/// # #[macro_use]
/// # extern crate aoc2023;
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     print_day!(1, day01);
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! print_day {
    ($day:literal, $module:ident) => {
        println!(
            "Day{:0>2}: {:?}",
            $day,
            $module::solve(format!("input/day{:0>2}.txt", $day).as_str())?
        );
    };
}
